"""Patched full-resolution differential (RESOLUTION-ORACLE.md): baseline snapshot vs current tree, per cfg.
Writes only inside orv-work sandboxes; rustc output goes to /dev/null (--emit=metadata -o /dev/null)."""
import collections, json, os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from snapshot import copy_files, snap_dir, closure_files
from spec import SPEC

TOOLCHAIN = "+nightly-2026-07-20"
CFG = {
    "lib": [],
    "test": ["--test"],
    "wasmu": ["--target", "wasm32-unknown-unknown"],
    "wasmu-test": ["--test", "--target", "wasm32-unknown-unknown"],
    "wasip2": ["--target", "wasm32-wasip2"],
    "wasip2-test": ["--test", "--target", "wasm32-wasip2"],
}
EXTRA_CFG = {"pack-value": ["--cfg", 'feature="deflate"'], "neural-model": []}
KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}


def sandbox_under(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


SERDE_ATTR = re.compile(r"#\[serde\((?:[^\[\]]|\[[^\]]*\])*\)\]")
DERIVE = re.compile(r"#\[derive\(([^)]*)\)\]")


EXT_USE = re.compile(r"(?:^[ \t]*#\[[^\n]*\][ \t]*\n)*^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+(?:::)?([A-Za-z_]\w*)(?:::\w+)*(?:::\{[^;]*\}|::\w+)?;[ \t]*$", re.M)


def deep_patch(t):
    """Sandbox-only: drop serde helper attributes and serde derives so rustc reaches typeck on crates that derive Serialize."""
    t = SERDE_ATTR.sub("", t)

    def dv(m):
        names = [n.strip() for n in m.group(1).split(",") if n.strip() and n.strip() not in ("Serialize", "Deserialize")]
        return "#[derive(" + ", ".join(names) + ")]" if names else ""

    t = DERIVE.sub(dv, t)
    if os.environ.get("ORV_DEEP2") == "1":
        t = EXT_USE.sub(lambda m: m.group(0) if m.group(1) in KEEP_EXT else "", t)
    return t


def patch_tree(tree_root, crate):
    files = []
    mods = set()
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
        if os.environ.get("ORV_DEEP") == "1":
            t2 = deep_patch(t2)
        if t2 != t:
            write(p, t2)
    return n


def run_rustc(root, lib_rel, crate, extra, name=None):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", *EXTRA_CFG.get(name, []), *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    return p.stderr


HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")


def parse(err):
    L = err.split("\n")
    keys = []
    i = 0
    while i < len(L):
        m = HEAD.match(L[i])
        if m and not L[i].startswith("error: aborting") and "previous error" not in L[i]:
            msg = m.group(2)
            loc = ""
            label = ""
            for j in range(i + 1, min(i + 14, len(L))):
                if HEAD.match(L[j]) or L[j].startswith("warning"):
                    break
                mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
                if mm and not loc:
                    loc = mm.group(1)
                ml = re.search(r"\^+ ?(.*)$", L[j])
                if ml and not label and "|" in L[j]:
                    label = ml.group(1)
            keys.append((m.group(1), msg, label, loc))
        i += 1
    return keys


def fresh(name, which):
    t = TARGETS[name]
    root = f"{W}/{name}/oracle-{which}"
    sandbox_under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    if which == "A":
        shutil.copytree(snap_dir(name), root, symlinks=True)
    elif os.environ.get("ORV_STAGE") == "1":
        shutil.copytree(snap_dir(name), root, symlinks=True)
        stage = f"{W}/{name}/stage"
        for dp, dn, fn in os.walk(stage):
            for f in fn:
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, stage)
                dst = os.path.join(root, rel)
                sandbox_under(dst)
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copyfile(p, dst)
    else:
        order, side, children = closure_files(f"{ROOT}/{t['root']}")
        copy_files(order + side, root)
    patched = patch_tree(root, t["crate"])
    return root, patched


def main():
    names = sys.argv[1].split(",") if sys.argv[1] != "all" else list(TARGETS)
    label = sys.argv[2]
    cfgs = sys.argv[3].split(",") if len(sys.argv) > 3 else list(CFG)
    allbad = 0
    for name in names:
        t = TARGETS[name]
        rootA, pa = fresh(name, "A")
        rootB, pb = fresh(name, "B")
        print(f"[{name}] patched A(globs,externs)={pa} B={pb}")
        summary = {}
        for c in cfgs:
            extra = CFG[c]
            errA = run_rustc(rootA, t["root"], t["crate"], extra, name).replace(rootA, "<S>")
            errB = run_rustc(rootB, t["root"], t["crate"], extra, name).replace(rootB, "<S>")
            write(f"{F}/🗑️generated/orv-{name}-oracle-{label}-{c}-A.err", errA)
            write(f"{F}/🗑️generated/orv-{name}-oracle-{label}-{c}-B.err", errB)
            ka, kb = parse(errA), parse(errB)
            ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
            cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
            onlyA, onlyB = ca - cb, cb - ca
            status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
            if status != "IDENTICAL":
                allbad += 1
            print(f"== {name} {c}: baseline errors={len(ka)} current errors={len(kb)} {status}")
            for (h, m, l), n in sorted(onlyB.items())[:80]:
                print(f"   +current {n}x {h}: {m[:150]} | {l[:80]}")
            for (h, m, l), n in sorted(onlyA.items())[:80]:
                print(f"   -current {n}x {h}: {m[:150]} | {l[:80]}")
            summary[c] = dict(base=len(ka), cur=len(kb), status=status)
        write(f"{F}/🗑️generated/orv-{name}-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if allbad == 0 else f"{allbad} DIFFERENT")


if __name__ == "__main__":
    main()
