# 🔬️ Union differential of the integration pass: two plugin trees (A after, B before) each get every flipped crate mounted as a module `pl_<crate>` of ONE crate and are compared by the patched full-resolution oracle.
# usage: uo2.py <tag> --a <dir with PLUG_REL> --b <dir> [--aflip k1,k2,..] [--bflip ..] [--configs lib,test,wasip2,lib-features,test-features] [--only-a] [--seed rel|old|new]
import collections, json, os, re, shutil, subprocess, sys, importlib.util, time
from common import *
from rs import blank
sys.path.insert(0, SPLIT + "/pwp0-tools")
spec = importlib.util.spec_from_file_location("pwp0oracle", SPLIT + "/pwp0-tools/oracle.py")
O = importlib.util.module_from_spec(spec)
spec.loader.exec_module(O)
O.GEN = WS
LIBKEY = {PFX + k.replace("-", "_"): k for k in CRATES}
PL = lambda k: "pl_" + k.replace("-", "_")
MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
PATHATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def guard(sb, path):
    real = os.path.realpath(path)
    assert real.startswith(os.path.realpath(sb) + "/"), (sb, path)


def tree_files(root):
    seen, order = set(), []
    def walk(p):
        p = os.path.normpath(p)
        if p in seen or not os.path.exists(p):
            return
        seen.add(p); order.append(p)
        text = open(p, encoding="utf8", errors="ignore").read(); clean = blank(text); here = os.path.dirname(p)
        for m in MOUNT.finditer(clean):
            attrs = text[m.start(1):m.end(1)]
            pm = PATHATTR.search(attrs)
            if pm:
                walk(os.path.join(here, pm.group(1)))
            else:
                stem = os.path.splitext(os.path.basename(p))[0]
                for cand in (os.path.join(here, stem, m.group(2) + ".rs"), os.path.join(here, stem, m.group(2), "mod.rs")):
                    if os.path.exists(cand):
                        walk(cand); break
        for m in INCLUDE.finditer(text):
            if clean[m.start():m.start() + 8] == "include!":
                walk(os.path.join(here, m.group(1)))
    walk(root)
    return order


def sub_outside(text, rx, repl):
    clean = blank(text); out, last = [], 0
    for m in rx.finditer(clean):
        out.append(text[last:m.start()]); out.append(repl(m)); last = m.end()
    out.append(text[last:])
    return "".join(out)


def mapping(text, k):
    t = sub_outside(text, re.compile(r"(?<![A-Za-z0-9_$:])crate::"), lambda m: "crate::" + PL(k) + "::")
    return sub_outside(t, re.compile(r"\bpub\(crate\)"), lambda m: "pub(in crate::" + PL(k) + ")")


def crossmap(text):
    return sub_outside(text, re.compile(r"(?<![A-Za-z0-9_$:])(semio_framework_plugin_[a-z0-9_]+)(?=::)"), lambda m: ("crate::" + PL(LIBKEY[m.group(1)])) if m.group(1) in LIBKEY else m.group(0))


def externs_to_use(text):
    rx = re.compile(r"(pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(semio_framework_plugin_[a-z0-9_]+)\s+as\s+(\w+)\s*;")
    return sub_outside(text, rx, lambda m: f"{m.group(1) or ''}use crate::{PL(LIBKEY[m.group(2)])} as {m.group(3)};" if m.group(2) in LIBKEY else m.group(0))


def prepare(sb, flipped):
    plug = sb + "/" + O.PLUG
    owners = {}
    roots = {}
    for k in sorted(flipped):
        root = plug + "/" + CRATES[k]["root"] + "/🦀️.rs"
        if not os.path.exists(root):
            continue
        roots[k] = root
        for f in tree_files(root):
            owners.setdefault(f, k)
    for f, k in owners.items():
        guard(sb, f)
        t = open(f, encoding="utf8").read()
        t = mapping(t, k)
        if f == roots[k]:
            t = re.sub(r"(?m)^#!\[(?:feature|allow)\([^\]]*\)\]\s*$", "", t)
        open(f, "w", encoding="utf8").write(t)
    for dp, dn, fn in os.walk(plug):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for fl in fn:
            if fl.endswith(".rs"):
                p = os.path.join(dp, fl)
                t = open(p, encoding="utf8", errors="ignore").read()
                if "semio_framework_plugin_" in t:
                    n = crossmap(externs_to_use(t))
                    if n != t:
                        guard(sb, p); open(p, "w", encoding="utf8").write(n)
    mounts = "".join(f'#[path = "{os.path.relpath(r, plug)}"]\npub mod {PL(k)};\n' for k, r in sorted(roots.items()))
    cur = open(plug + "/🦀️.rs", encoding="utf8").read()
    open(plug + "/🦀️.rs", "w", encoding="utf8").write(cur.rstrip("\n") + "\n" + mounts)
    made = stub_missing(plug)
    if made:
        print("stubbed missing mount targets:", len(made), flush=True)
    O.neutralize(plug)
    os.makedirs(sb + "/outdir", exist_ok=True)
    return roots


def stub_missing(tree):
    made = []
    for base, dirs, files in os.walk(tree):
        dirs[:] = [d for d in dirs if d != "node_modules"]
        for f in files:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(base, f)
            text = open(p, encoding="utf8", errors="ignore").read(); clean = blank(text)
            for m in MOUNT.finditer(clean):
                pm = PATHATTR.search(text[m.start(1):m.end(1)])
                if pm:
                    target = os.path.normpath(os.path.join(base, pm.group(1)))
                    if not os.path.exists(target):
                        os.makedirs(os.path.dirname(target), exist_ok=True); open(target, "w").write(""); made.append(target)
    return made


def crates_root(sb, roots):
    pkd = sb + "/" + O.PK
    mounts = "".join(f'#[path = "{os.path.relpath(r, pkd)}"]\npub mod {PL(k)};\n' for k, r in sorted(roots.items()))
    head = "#![allow(unused)]\n#![feature(associated_type_defaults)]\n#![allow(async_fn_in_trait)]\n"
    open(pkd + "/🦀️.rs", "w", encoding="utf8").write(head + mounts)


def build(src_dir, sb, flipped, mode="full"):
    shutil.rmtree(sb, ignore_errors=True)
    os.makedirs(sb)
    ex = []
    for e in O.EXCLUDE:
        ex += ["--exclude", e]
    os.makedirs(sb + "/" + O.PLUG)
    subprocess.check_call(["rsync", "-a", *ex, src_dir + "/" + O.PLUG + "/", sb + "/" + O.PLUG + "/"])
    os.makedirs(sb + "/" + O.PK, exist_ok=True)
    for f in ("🦀️.rs", "Cargo.toml"):
        shutil.copy(REPO + "/" + O.PK + "/" + f, sb + "/" + O.PK + "/" + f)
    roots = prepare(sb, flipped)
    if mode == "crates":
        crates_root(sb, roots)
    return roots


def main():
    args = sys.argv[1:]
    tag = args[0]
    def arg(name, default=None):
        return args[args.index(name) + 1] if name in args else default
    a_dir, b_dir = arg("--a"), arg("--b")
    aflip = set(filter(None, (arg("--aflip", "") or "").split(",")))
    bflip = set(filter(None, (arg("--bflip", "") or "").split(",")))
    configs = (arg("--configs") or "lib,test,wasip2,lib-features,test-features").split(",")
    base = WS + "/sb-" + tag
    A, B = base + "/A", base + "/B"
    mode = arg("--mode", "full")
    build(a_dir, A, aflip, mode)
    if "--only-a" not in args:
        build(b_dir, B, bflip, mode)
    for s in [x for x in args if x.startswith("seed:")]:
        rel, old, new = s[5:].split("|||")
        p = A + "/" + O.PLUG + "/" + rel
        t = open(p, encoding="utf8").read()
        assert t.count(old) >= 1, (rel, old)
        guard(A, p); open(p, "w", encoding="utf8").write(t.replace(old, new, 1)); print("seeded", rel, flush=True)
    summary = {}
    for name in configs:
        ea, ta = O.run(A, "A-" + name, O.CFGS[name], "pi1" + tag)
        a = O.parse(ea, A)
        if "--only-a" in args:
            kept = {k: v for k, v in a.items() if not (mode == "crates" and is_noise(k, aflip))}
            print(f"== {name}: A errors={sum(a.values())} non-noise={sum(kept.values())} sec={round(ta)}", flush=True)
            for k, v in sorted(kept.items(), key=lambda kv: (kv[0][4], kv[0][0], kv[0][1]))[:int(arg('--top', '80'))]:
                print("  ", v, k[0], k[1][:170], "|", k[2][:50], "|", k[4][-70:])
            continue
        eb, tb = O.run(B, "B-" + name, O.CFGS[name], "pi1" + tag)
        b = O.parse(eb, B)
        fa, fb = collections.Counter(), collections.Counter()
        where = {}
        for k, v in a.items():
            fa[norm(k)] += v; where.setdefault(norm(k), collections.Counter())[k[4][-50:]] += v
        for k, v in b.items():
            fb[norm(k)] += v
        new, old = fa - fb, fb - fa
        summary[name] = dict(a=sum(a.values()), b=sum(b.values()), onlyNew=sum(new.values()), onlyOld=sum(old.values()))
        print(f"== {name}: A errors={sum(a.values())} B errors={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())} sec={round(ta)}/{round(tb)}", flush=True)
        for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:int(arg('--top', '80'))]:
            print("  +new", v, k[0], k[1][:160], "|", k[2][:50], "|", dict(where[k].most_common(2)))
        for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:int(arg('--top', '40'))]:
            print("  -old", v, k[0], k[1][:160], "|", k[2][:50])
    json.dump(summary, open(f"{GEN}/pi1-oracle-{tag}-summary.json", "w"), indent=1)


_NOISE = None


def is_noise(key, stitched):
    global _NOISE
    if _NOISE is None:
        keep = "|".join(re.escape(PFX + s.replace("-", "_")) for s in stitched) or "NOPE"
        e = r"(?:store|protocol|dsl|vcs|pack|serde|serde_json|wit_bindgen|replication|base64_codec|semio_framework(?!_plugin_(?:" + "|".join(re.escape(s.replace("-", "_")) for s in stitched) + r")\b)[A-Za-z0-9_]*)"
        _NOISE = [
            re.compile(r"cannot find module or crate `" + e + r"` in this scope"),
            re.compile(r"cannot find [a-z ,]*`[A-Za-z_0-9]+` in (?:module )?`" + e + r"`"),
            re.compile(r"cannot find [a-z ]*`[A-Za-z_0-9]+` in the crate root"),
            re.compile(r"unresolved imports? `" + e + r"(?:::[^`]*)?`(?:, `" + e + r"(?:::[^`]*)?`)*$"),
            re.compile(r"unresolved imports? `" + e + r"::\{[^`]*\}`$"),
            re.compile(r"^cannot find (?:attribute|derive macro|macro)"),
            re.compile(r"^failed to resolve: use of unresolved module or unlinked crate `" + e + r"`"),
            re.compile(r"^failed to resolve: use of unresolved module or unlinked crate `[a-z_0-9]+`"),
        ]
    return any(r.search(key[1]) for r in _NOISE)


def norm(k):
    code, msg, text, label = k[:4]
    f = lambda s: re.sub(r"crate::pl_[a-z0-9_]+::", "crate::", s).replace("crate::pl_", "crate::")
    return (code, f(msg), f(text), "")


if __name__ == "__main__":
    main()
