"""Patched full-resolution differential (RESOLUTION-ORACLE.md): baseline snapshot (A) vs current tree (B), per cfg.
Writes only inside tgm-work sandboxes; rustc output goes to a scratch .rmeta that is deleted at once (--emit=metadata)."""
import collections, json, os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *

TOOLCHAIN = "+nightly-2026-07-20"
CFG = {
    "lib": [],
    "test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "p2": ["--target", "wasm32-wasip2"],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2test": ["--test", "--target", "wasm32-wasip2"],
}
KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}
OUTDIR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/tgm-out"


def patch_tree(tree_root, crate):
    files, mods = [], set()
    for dp, dn, fn in os.walk(tree_root):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                files.append(p)
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", read(p), re.M):
                    mods.add(m.group(1))
    keep = KEEP_EXT | mods | {crate}
    rx = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    ex = re.compile(r"^\s*extern\s+crate\s+(\w+)(?:\s+as\s+\w+)?;[ \t]*$", re.M)
    n = [0, 0]
    for p in files:
        under(p, tree_root)
        t = read(p)

        def sub(m):
            if m.group(2) in keep:
                return m.group(0)
            n[0] += 1
            return m.group(1) + "const _: () = ();"

        def subex(m):
            if m.group(1) in keep:
                return m.group(0)
            n[1] += 1
            return ""

        t2 = ex.sub(subex, rx.sub(sub, t))
        if t2 != t:
            write(p, t2)
    return n


def run_rustc(root, lib_rel, crate, extra):
    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, f"{os.getpid()}.rmeta")
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", out, "--cap-lints", "allow", *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    if os.path.exists(out):
        os.remove(out)
    return p.stderr


HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def norm(s):
    return PATHRX.sub("", s)


def parse(err):
    L = err.split("\n")
    keys = []
    for i, line in enumerate(L):
        m = HEAD.match(line)
        if not m or line.startswith("error: aborting") or "previous error" in line:
            continue
        loc, label = "", ""
        for j in range(i + 1, min(i + 14, len(L))):
            if HEAD.match(L[j]) or L[j].startswith("warning"):
                break
            mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
            if mm and not loc:
                loc = mm.group(1)
            ml = re.search(r"\^+ ?(.*)$", L[j])
            if ml and not label and "|" in L[j]:
                label = ml.group(1)
        keys.append((m.group(1), norm(m.group(2)), norm(label), loc))
    return keys


def fresh(name, which, crate):
    root = f"{W}/{name}/oracle-{which}"
    under(root, W)
    if os.path.exists(root):
        shutil.rmtree(root)
    t = TARGETS[name]
    if which == "A":
        shutil.copytree(snap_dir(name), root, symlinks=True)
    else:
        copy_tree(t["tree"], root)
    return root, patch_tree(root, crate)


def main():
    name, label = sys.argv[1], sys.argv[2]
    only = [a for a in sys.argv[3:] if a in CFG]
    t = TARGETS[name]
    crate = t["crate"]
    rootA, pa = fresh(name, "A", crate)
    rootB, pb = fresh(name, "B", crate)
    print(f"[{name}] patched A(globs,externs)={pa} B={pb}")
    for i, a_ in enumerate(sys.argv):
        if a_ == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(rootB, rel)
            under(fp, rootB)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print(f"   mutated {rel}: {old!r} -> {new!r}")
    runs = [(c, CFG[c]) for c in (only or t["cfgs"])]
    for ft in t["features"]:
        runs.append((f"lib+{ft}", ["--cfg", f'feature="{ft}"']))
        runs.append((f"test+{ft}", ["--test", "--cfg", f'feature="{ft}"']))
    summary, bad = {}, 0
    for c, extra in runs:
        errA = run_rustc(rootA, t["lib"], crate, extra).replace(rootA, "<S>")
        errB = run_rustc(rootB, t["lib"], crate, extra).replace(rootB, "<S>")
        write(f"{F}/🗑️generated/tgm-{name}-oracle-{label}-{c}-A.err", errA)
        write(f"{F}/🗑️generated/tgm-{name}-oracle-{label}-{c}-B.err", errB)
        ka, kb = parse(errA), parse(errB)
        ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
        onlyA, onlyB = ca - cb, cb - ca
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        if status != "IDENTICAL":
            bad += 1
        print(f"== {c}: baseline errors={len(ka)} current errors={len(kb)} onlyNew={sum(onlyB.values())} onlyOld={sum(onlyA.values())} {status}")
        for (h, m, l), n in sorted(onlyB.items())[:40]:
            print(f"   +current {n}x {h}: {m[:140]} | {l[:80]}")
        for (h, m, l), n in sorted(onlyA.items())[:40]:
            print(f"   -current {n}x {h}: {m[:140]} | {l[:80]}")
        summary[c] = dict(base=len(ka), cur=len(kb), onlyNew=sum(onlyB.values()), onlyOld=sum(onlyA.values()), status=status)
    write(f"{F}/🗑️generated/tgm-{name}-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
