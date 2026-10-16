"""Scoped patched full-resolution differential (RESOLUTION-ORACLE.md): baseline snapshot vs live tree.
usage: oracle.py <label> [target[,target]] [cfg[,cfg]]
Writes only inside osm-work sandboxes; rustc writes metadata to /dev/null."""
import os, re, shutil, subprocess, sys, collections, json
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *

TOOLCHAIN = "+nightly-2026-07-20"
CFG = {
    "lib": [],
    "test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2": ["--target", "wasm32-wasip2"],
    "p2test": ["--test", "--target", "wasm32-wasip2"],
}
KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}
HEAD = re.compile(r"^(error(?:\[E\d+\])?|warning): (.*)$")
LINT = os.environ.get("OSM_LINTS") == "1"


def sandbox_under(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


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
        sandbox_under(p)
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
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", *([] if LINT else ["--cap-lints", "allow"]), *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    return p.stderr


def parse(err):
    L = err.split("\n")
    keys = []
    for i, line in enumerate(L):
        m = HEAD.match(line)
        if not m or line.startswith("error: aborting") or "previous error" in line or (m.group(1) == "warning" and not LINT):
            continue
        loc, label = "", ""
        for j in range(i + 1, min(i + 14, len(L))):
            if HEAD.match(L[j]):
                break
            mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
            if mm and not loc:
                loc = mm.group(1)
            ml = re.search(r"\^+ ?(.*)$", L[j])
            if ml and not label and "|" in L[j]:
                label = ml.group(1)
        msg = m.group(2)
        if msg.startswith("couldn't read"):
            msg = "couldn't read <include>"
        keys.append((m.group(1), msg, label, loc))
    return keys


EXTERNAL = re.compile(r"unresolved module or unlinked crate|cannot find (?:module or )?crate|unresolved import `[a-z_0-9]+`$|can't find crate|unlinked crate")


def split_external(keys):
    ext = [k for k in keys if EXTERNAL.search(k[1]) or EXTERNAL.search(k[2])]
    loc = [k for k in keys if not (EXTERNAL.search(k[1]) or EXTERNAL.search(k[2]))]
    return loc, ext


def fresh(snapkey, which, crate, tree):
    root = f"{W}/oracle-{snapkey}-{which}"
    sandbox_under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    if which == "A":
        shutil.copytree(f"{W}/snap-{snapkey}", root, symlinks=True)
    elif os.environ.get("OSM_STAGE") == "1":
        shutil.copytree(f"{W}/stage-{snapkey}", root, symlinks=True)
    else:
        copy_tree(tree, root)
    return root, patch_tree(root, crate)


def main():
    label = sys.argv[1]
    names = sys.argv[2].split(",") if len(sys.argv) > 2 and sys.argv[2] != "all" else list(TARGETS)
    only = sys.argv[3].split(",") if len(sys.argv) > 3 else None
    bad = 0
    summary = {}
    for name in names:
        t = TARGETS[name]
        snapkey = "services" if name == "services" else "mcp"
        rootA, pa = fresh(snapkey, "A", t["crate"], t["tree"])
        rootB, pb = fresh(snapkey, "B", t["crate"], t["tree"])
        print(f"[{name}] patched A(globs,externs)={pa} B={pb}")
        for c in (only or t["cfgs"]):
            errA = run_rustc(rootA, t["lib"], t["crate"], CFG[c]).replace(rootA, "<S>")
            errB = run_rustc(rootB, t["lib"], t["crate"], CFG[c]).replace(rootB, "<S>")
            write(f"{F}/🗑️generated/osm-oracle-{label}-{name}-{c}-A.err", errA)
            write(f"{F}/🗑️generated/osm-oracle-{label}-{name}-{c}-B.err", errB)
            ka_all, kb_all = parse(errA), parse(errB)
            ka, ea = split_external(ka_all)
            kb, eb = split_external(kb_all)
            ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
            cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
            onlyA, onlyB = ca - cb, cb - ca
            status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
            if status != "IDENTICAL":
                bad += 1
            print(f"== {name}/{c}: baseline={len(ka)} current={len(kb)} (external-crate errors cancelled: base {len(ea)} cur {len(eb)}) {status}")
            for (h, m, l), n in sorted(onlyB.items())[:80]:
                print(f"   +current {n}x {h}: {m[:150]} | {l[:90]}")
            for (h, m, l), n in sorted(onlyA.items())[:80]:
                print(f"   -current {n}x {h}: {m[:150]} | {l[:90]}")
            summary[f"{name}/{c}"] = dict(base=len(ka), cur=len(kb), ext_base=len(ea), ext_cur=len(eb), status=status)
    write(f"{F}/🗑️generated/osm-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
