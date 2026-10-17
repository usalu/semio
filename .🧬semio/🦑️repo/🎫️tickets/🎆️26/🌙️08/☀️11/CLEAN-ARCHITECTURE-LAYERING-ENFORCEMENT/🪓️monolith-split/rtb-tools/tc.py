#!/usr/bin/env python3
"""🔬️ Stub-layer typeck differential of the rtb pass (sandbox only; the repo is never written).
A = the live tree: the emptied top crate + every real crate (bins and companions included) mounted as modules of the library root.
B = the pre-edit snapshot tree: the monolith top crate with its tests mounted + the crates that already existed.
Both go through the frs stub layer (frs-tools/stubgen.py) for the crates outside the renderer, so rustc reaches the typeck phase where field and method privacy (E0616/E0624/E0451) surface.
A new error row in A that B does not have is a regression of the move; rows both have are stub noise.
usage: tc.py <tag> [--configs test,wasm-test] [--reuse-sandbox]"""
import collections, importlib.util, json, os, re, shutil, subprocess, sys, time
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/../frs-tools")
spec = importlib.util.spec_from_file_location("rtbuo", HERE + "/uo.py")
U = importlib.util.module_from_spec(spec)
spec.loader.exec_module(U)
import stubgen
from common import *

NEW_IN_A = ["fixtures", "frame-tests-1"]
CFGS = {"test": ["--test"], "wasm-test": ["--test", "--target", "wasm32-unknown-unknown"], "lib": [], "wasm": ["--target", "wasm32-unknown-unknown"]}
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def ext_crates(crates, table):
    """Crate names outside the renderer that the sources name: manifest dependency keys and extern-crate aliases."""
    names = set()
    for c in crates:
        mf = read(T + "/" + table[c] + "/📦️packages/🦀️rust/Cargo.toml")
        sec = None
        for line in mf.splitlines():
            m = re.match(r"^\[(.+)\]\s*$", line)
            if m:
                sec = m.group(1)
                continue
            if sec and re.fullmatch(r"(target\..+\.)?(dev-)?dependencies", sec) and "=" in line and not line.lstrip().startswith("#"):
                key = line.split("=", 1)[0].strip().split(".")[0]
                if re.match(r"^[A-Za-z0-9_\-]+$", key) and not key.startswith(PKG_PREFIX):
                    names.add(key.replace("-", "_"))
    return names


def aliases(root, own_mods):
    out = set()
    rx = re.compile(r"extern\s+crate\s+(\w+)\s+as\s+(\w+)\s*;")
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d != "node_modules"]
        for f in fn:
            if f.endswith(".rs"):
                for m in rx.finditer(open(os.path.join(dp, f), encoding="utf8", errors="ignore").read()):
                    if not m.group(1).startswith(PREFIX) and m.group(2) not in own_mods:
                        out.add(m.group(2))
    return out


def build(tag, label, snap):
    sb = f"{RS}/tc-{tag}/{label}"
    table = crate_table()
    real = json.load(open(HERE + "/real.json"))
    crates = [c for c in real if not (snap and c in NEW_IN_A)]
    if snap:
        U.chain_snap(sb)
    else:
        U.O.chain(sb)
    U.prepare(sb, crates, table, "full")
    eng = sb + "/" + ENG_REL
    U.O.neutralize(eng)
    os.makedirs(sb + "/outdir", exist_ok=True)
    mods = set()
    for dp, dn, fn in os.walk(eng):
        dn[:] = [d for d in dn if d != "node_modules"]
        for f in fn:
            if f.endswith(".rs"):
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type)\s+(\w+)", open(os.path.join(dp, f), encoding="utf8", errors="ignore").read(), re.M):
                    mods.add(m.group(1))
    ext = sorted((ext_crates(crates, table) | aliases(eng, mods)) - mods - {"std", "core", "alloc", "crate", "self", "super"})
    stubgen.apply([eng], LIB_REL[len(ENG_REL) + 1:], ext)
    print(f"[tc] {label}: {len(crates)} crates mounted, {len(ext)} external crate roots stubbed", flush=True)
    return sb


def norm(s):
    return PATHRX.sub("", s)


def keys(err, sb):
    c = collections.Counter()
    where = collections.defaultdict(collections.Counter)
    for line in err.split("\n"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error" or str(d.get("message", "")).startswith(("aborting due to", "couldn't create a temp dir")) or "previous error" in str(d.get("message", "")):
            continue
        code = (d.get("code") or {}).get("code") or ""
        sp = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        label = (sp.get("label") or "") if sp else ""
        f = os.path.normpath((sp.get("file_name", "") if sp else "").replace(sb + "/", "").replace(REPO + "/", ""))
        k = (code, norm(str(d.get("message", ""))), norm(label))
        c[k] += 1
        where[k][U.short(f)] += 1
    return c, where


def main():
    tag = sys.argv[1]
    configs = ["test"]
    if "--configs" in sys.argv:
        configs = sys.argv[sys.argv.index("--configs") + 1].split(",")
    A = build(tag, "A", False)
    if "--only-a" in sys.argv:
        for name in configs:
            ea, ta = U.run(A, "tcA-" + name, CFGS[name], tag)
            a, wa = keys(ea, A)
            by = collections.Counter()
            for k, v in a.items():
                by[k[0]] += v
            print(f"== A only {name}: errors={sum(a.values())} sec={round(ta)}", dict(by.most_common(25)), flush=True)
        return
    B = build(tag, "B", True)
    for name in configs:
        started = time.time()
        ea, ta = U.run(A, "tcA-" + name, CFGS[name], tag)
        eb, tb = U.run(B, "tcB-" + name, CFGS[name], tag)
        a, wa = keys(ea, A)
        b, wb = keys(eb, B)
        new, old = a - b, b - a
        print(f"== {name}: A(current) errors={sum(a.values())} B(baseline) errors={sum(b.values())} onlyA={sum(new.values())} onlyB={sum(old.values())} sec={round(ta)}/{round(tb)}", flush=True)
        by = collections.Counter()
        for k, v in new.items():
            by[k[0]] += v
        print("   onlyA by code:", dict(by.most_common(20)))
        for k, v in sorted(new.items(), key=lambda kv: (kv[0][0] not in ("E0616", "E0624", "E0451", "E0603"), -kv[1], kv[0])):
            print("  +A", v, k[0], k[1][:170], "|", k[2][:50], "|", dict(wa[k].most_common(2)), flush=True)
        by = collections.Counter()
        for k, v in old.items():
            by[k[0]] += v
        print("   onlyB by code:", dict(by.most_common(12)))


if __name__ == "__main__":
    main()
