"""Patched full-resolution differential (RESOLUTION-ORACLE.md): baseline snapshot vs current tree, per cfg.
Writes only inside frs-work sandboxes; rustc output goes to /dev/null (--emit=metadata -o /dev/null)."""
import os, re, shutil, subprocess, sys, collections, json
sys.path.insert(0, os.path.dirname(__file__))
from frslib import *
from snapshot import copy_tree, SKIP

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


def sandbox_under(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


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
    ex = re.compile(r"^(?:[ \t]*#\[cfg\([^\n]*\)\][ \t]*\n)?[ \t]*extern\s+crate\s+(\w+)(?:\s+as\s+\w+)?;[ \t]*$", re.M)
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


OUTDIR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/wbr-out"


def run_rustc(root, lib_rel, crate, extra, caplints=True):
    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, f"{os.getpid()}.rmeta")
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", out, *(["--cap-lints", "allow"] if caplints else []), *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root, env={**os.environ, "CARGO_MANIFEST_DIR": os.path.join(root, os.path.dirname(lib_rel))})
    if os.path.exists(out):
        os.remove(out)
    return p.stderr


HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")


PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def norm(s):
    """Module-placement-insensitive message text: drop lowercase module path prefixes."""
    return PATHRX.sub("", s)


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
            keys.append((m.group(1), norm(msg), norm(label), loc))
        i += 1
    return keys


def fresh(name, which, crate):
    root = f"{W}/{name}/oracle-{which}"
    sandbox_under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    t = TARGETS[name]
    if which == "A":
        shutil.copytree(snap_dir(name), root, symlinks=True)
    elif "--stage" in sys.argv:
        shutil.copytree(snap_dir(name), root, symlinks=True)
        sd = stage_dir(name)
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                p = os.path.join(dp, f)
                dst = os.path.join(root, os.path.relpath(p, sd))
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copyfile(p, dst)
    else:
        copy_tree(t["tree"], root)
    patched = patch_tree(root, crate)
    return root, patched


def stub_both(rootA, rootB, name):
    t = TARGETS[name]
    if not t.get("ext") or "--nostubs" in sys.argv:
        return
    import stubgen
    if t.get("alias"):
        alt = "|".join(t["alias"])
        rx = re.compile(r"(?<![A-Za-z0-9_:])crate::(" + alt + r")(?=\s*::)")
        for r in (rootA, rootB):
            for p in stubgen.rs_files(r):
                sandbox_under(p)
                tx = read(p)
                tx2 = rx.sub(lambda m: m.group(1), tx)
                tx2 = re.sub(r"(?m)^(\s*(?:pub(?:\([^)]*\))?\s+)?use\s+[\w:]+::)\{\s*(\w+)\s*\};", r"\1\2;", tx2)
                if tx2 != tx:
                    write(p, tx2)
    stubgen.apply([rootA, rootB], t["lib"], t["ext"], t.get("extra_stub", ""), t.get("traits", ()))


def main():
    name = sys.argv[1]
    label = sys.argv[2]
    only = sys.argv[3].split(",") if len(sys.argv) > 3 and not sys.argv[3].startswith("--") else None
    t = TARGETS[name]
    crate = t["crate"]
    rootA, pa = fresh(name, "A", crate)
    rootB, pb = fresh(name, "B", crate)
    print(f"[{name}] patched A(globs,externs)={pa} B={pb}")
    stub_both(rootA, rootB, name)
    for i, a_ in enumerate(sys.argv):
        if a_ == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(rootB, rel)
            sandbox_under(fp)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print(f"   mutated {rel}: {old!r} -> {new!r}")
    runs = []
    for c in (only or t["cfgs"]):
        runs.append((c, CFG[c]))
    for ft in t["features"]:
        runs.append((f"lib+{ft}", ["--cfg", f'feature="{ft}"']))
        runs.append((f"test+{ft}", ["--test", "--cfg", f'feature="{ft}"']))
    if only is None and t["features"]:
        feats = sum([["--cfg", f'feature="{ft}"'] for ft in t["features"]], [])
        runs.append(("lib+all", feats))
        runs.append(("test+all", ["--test", *feats]))
    summary = {}
    bad = 0
    if "--warn" in sys.argv:
        def unused(err, root):
            L = err.replace(root, "<S>").split("\n")
            out = []
            for i, l in enumerate(L):
                if l.startswith("warning: unused import") or l.startswith("warning: unused qualif") or "more private than" in l or l.startswith("warning: private ") or l.startswith("warning: unreachable pub"):
                    loc = next((x.strip() for x in L[i + 1:i + 3] if x.strip().startswith("-->")), "")
                    names = re.findall(r"`([^`]+)`", l)
                    segs = [s for s in re.sub(r":\d+:\d+$", "", loc.replace("--> <S>/", "")).split("/") if s not in ("..", ".")]
                    out.append(("/".join(segs[-2:]), l[len("warning: "):], tuple(names)))
            return out
        agg = collections.defaultdict(set)
        for c, extra in runs:
            wa = unused(run_rustc(rootA, t["lib"], crate, extra, False), rootA)
            wb = unused(run_rustc(rootB, t["lib"], crate, extra, False), rootB)
            ca, cb = collections.Counter(w[1] for w in wa), collections.Counter(w[1] for w in wb)
            new = cb - ca
            gone = ca - cb
            print(f"== warnings {c}: baseline={len(wa)} current={len(wb)} new={sum(new.values())} gone={sum(gone.values())}")
            for k in sorted(new):
                where = ",".join(sorted({w[0] for w in wb if w[1] == k}))
                agg[(k, where)].add(c)
        print("-- new warnings by message (a message that only moved between files is not new; prune only when it fires in ALL cfgs)")
        for k in sorted(agg):
            print(f"   {k[1]} | {k[0][:600]} | {sorted(agg[k])}")
        return
    for c, extra in runs:
        errA = run_rustc(rootA, t["lib"], crate, extra)
        errB = run_rustc(rootB, t["lib"], crate, extra)
        errA = errA.replace(rootA, "<S>")
        errB = errB.replace(rootB, "<S>")
        write(f"{F}/🗑️generated/wbr-{name}-oracle-{label}-{c}-A.err", errA)
        write(f"{F}/🗑️generated/wbr-{name}-oracle-{label}-{c}-B.err", errB)
        ka, kb = parse(errA), parse(errB)
        nl = "--nolabel" in sys.argv
        ca = collections.Counter((k[0], k[1], "" if nl else k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], "" if nl else k[2]) for k in kb)
        onlyA, onlyB = ca - cb, cb - ca
        ea = len(ka)
        eb = len(kb)
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        if status != "IDENTICAL":
            bad += 1
        print(f"== {c}: baseline errors={ea} current errors={eb} {status}")
        for (h, m, l), n in sorted(onlyB.items())[:60]:
            print(f"   +current {n}x {h}: {m[:140]} | {l[:80]}")
        for (h, m, l), n in sorted(onlyA.items())[:60]:
            print(f"   -current {n}x {h}: {m[:140]} | {l[:80]}")
        summary[c] = dict(base=ea, cur=eb, status=status)
    write(f"{F}/🗑️generated/wbr-{name}-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
