"""Scoped patched full-resolution differential (📓️RESOLUTION-ORACLE.md) of the svm slice.

usage: oracle.py <plan.py> <label> [cfg,cfg...] [--installed]

A = sandbox copy of the crate trees with the pristine monolith restored and the new child folders removed.
B = the same trees with the staged (or, with --installed, the real) split overlaid.
Both get the sanctioned sandbox patches (missing `extern crate` removed, external glob imports replaced by `const _: () = ();`).
rustc runs `--emit=metadata -o /dev/null`; only the sandboxes under 🗑️generated/svm-work are ever written."""
import collections, json, os, re, shutil, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from split import load_plan, read, write, ROOT, F, WORK

TOOLCHAIN = "+nightly-2026-07-20"
CFG = {
    "lib": [],
    "test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"],
    "p2": ["--target", "wasm32-wasip2"],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2test": ["--test", "--target", "wasm32-wasip2"],
}
SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist"}
KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}
HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")


def under(path, root):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(root) + os.sep), f"write outside sandbox: {rp}"


def copy_tree(rel, dst_root, skip_dirs=()):
    base = os.path.join(ROOT, rel)
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in SKIP and os.path.join(dp, d) not in skip_dirs]
        for f in fn:
            p = os.path.join(dp, f)
            if os.path.islink(p):
                continue
            if not f.endswith(".rs") and os.path.getsize(p) > 300_000:
                continue
            dst = os.path.join(dst_root, os.path.relpath(p, ROOT))
            under(dst, WORK)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(p, dst)


def patch_tree(root, crate):
    files, mods = [], set()
    for dp, dn, fn in os.walk(root):
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
        under(p, WORK)
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


def run_rustc(root, lib_rel, crate, extra, feats, crate_type="lib"):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", crate_type, "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--cap-lints", "allow", *extra, *feats, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    return p.stderr


EXTERNAL_MSG = [re.compile(r"unresolved import `(\w+)"), re.compile(r"unlinked crate `(\w+)`"), re.compile(r"cannot find (?:macro|attribute|derive macro) `\w+`"), re.compile(r"could not find `\w+` in `(\w+)`")]
LOCAL = set()


def is_external(msg):
    for rx in EXTERNAL_MSG:
        m = rx.search(msg)
        if m:
            g = m.group(1) if m.groups() else None
            if g is None or (g not in KEEP_EXT and g not in LOCAL):
                return True
    return False


def parse(err):
    L = err.split("\n")
    keys = []
    for i, line in enumerate(L):
        m = HEAD.match(line)
        if m and not line.startswith("error: aborting") and "previous error" not in line and not is_external(m.group(2)):
            label = ""
            loc = ""
            for j in range(i + 1, min(i + 14, len(L))):
                if HEAD.match(L[j]) or L[j].startswith("warning"):
                    break
                mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
                if mm and not loc:
                    loc = mm.group(1)
                ml = re.search(r"\^+ ?(.*)$", L[j])
                if ml and not label and "|" in L[j]:
                    label = ml.group(1)
            keys.append((m.group(1), re.sub(r'/dev/rmeta\w+', '/dev/rmeta*', m.group(2)), label, loc))
    return keys


def main():
    plan = load_plan(sys.argv[1])
    label = sys.argv[2]
    only = sys.argv[3].split(",") if len(sys.argv) > 3 and not sys.argv[3].startswith("--") else None
    installed = "--installed" in sys.argv
    o = plan["oracle"]
    name = plan["name"]
    real_dir = os.path.dirname(ROOT + "/" + plan["real"])
    new_dirs = [os.path.join(real_dir, c["folder"]) for c in plan["children"]] + [os.path.join(ROOT, d) for d in plan.get("extra_new_dirs", [])]
    wa, wb = f"{WORK}/{name}/A", f"{WORK}/{name}/B"
    for w in (wa, wb):
        under(w, WORK)
        if os.path.exists(w):
            shutil.rmtree(w)
    for tree in o["trees"]:
        copy_tree(tree, wa, skip_dirs=new_dirs)
        copy_tree(tree, wb, skip_dirs=[] if installed else new_dirs)
    orig = plan["orig"] if os.path.isabs(plan["orig"]) else F + "/" + plan["orig"]
    shutil.copyfile(orig, os.path.join(wa, plan["real"]))
    for extra in plan.get("extra_origs", []):
        shutil.copyfile(F + "/" + extra[0], os.path.join(wa, extra[1]))
    if not installed:
        stage = f"{WORK}/{name}/stage"
        for dp, dn, fn in os.walk(stage):
            for f in fn:
                p = os.path.join(dp, f)
                dst = os.path.join(wb, os.path.relpath(p, stage))
                under(dst, WORK)
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copyfile(p, dst)
    for i, a_ in enumerate(sys.argv):
        if a_ == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(wb, rel)
            under(fp, WORK)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print(f"   mutated {rel}: {old!r} -> {new!r}")
    if "synthetic_lib" in plan:
        for w in (wa, wb):
            fp = os.path.join(w, plan["synthetic_lib"]["path"])
            under(fp, WORK)
            write(fp, plan["synthetic_lib"]["text"])
    pa, pb = patch_tree(wa, o["crate"]), patch_tree(wb, o["crate"])
    for root_ in (wa, wb):
        for dp, dn, fn in os.walk(root_):
            for f in fn:
                if f.endswith(".rs"):
                    for mm in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", read(os.path.join(dp, f)), re.M):
                        LOCAL.add(mm.group(1))
    print(f"[{name}] patched A={pa} B={pb}")
    runs = [(c, CFG[c], []) for c in (only or o["cfgs"])]
    for ft in o.get("features", []):
        runs.append((f"lib+{ft}", [], ["--cfg", f'feature="{ft}"']))
        runs.append((f"test+{ft}", ["--test"], ["--cfg", f'feature="{ft}"']))
    bad = 0
    summary = {}
    for c, extra, feats in runs:
        ea = run_rustc(wa, plan["real_lib"], o["crate"], extra, feats, o.get("crate_type", "lib")).replace(wa, "<S>")
        eb = run_rustc(wb, plan["real_lib"], o["crate"], extra, feats, o.get("crate_type", "lib")).replace(wb, "<S>")
        write(f"{F}/🗑️generated/svm-{name}-oracle-{label}-{c}-A.err", ea)
        write(f"{F}/🗑️generated/svm-{name}-oracle-{label}-{c}-B.err", eb)
        ka, kb = parse(ea), parse(eb)
        ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
        oa, ob = ca - cb, cb - ca
        status = "IDENTICAL" if not oa and not ob else "DIFFERENT"
        bad += status != "IDENTICAL"
        print(f"== {c}: baseline errors={len(ka)} split errors={len(kb)} {status}")
        for (h, m, l), n in sorted(ob.items())[:60]:
            print(f"   +split {n}x {h}: {m[:150]} | {l[:80]}")
        for (h, m, l), n in sorted(oa.items())[:60]:
            print(f"   -split {n}x {h}: {m[:150]} | {l[:80]}")
        summary[c] = dict(base=len(ka), split=len(kb), status=status)
    write(f"{F}/🗑️generated/svm-{name}-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
