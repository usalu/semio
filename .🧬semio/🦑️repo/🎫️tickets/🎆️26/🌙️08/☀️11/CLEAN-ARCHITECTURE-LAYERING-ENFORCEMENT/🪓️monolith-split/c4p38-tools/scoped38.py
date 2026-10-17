"""🔬 c4p38 scoped patched differential (the `📓️RESOLUTION-ORACLE.md` method; model: `c4p23-tools/scoped.py`): the crate lib root runs through `rustc --emit=metadata -o /dev/null`, once with the ORIGINAL text restored (tree `orig`) and once with the split text (tree `new`, from the stage dir or the live tree).
Both trees are sandbox copies of the owning directory under the stable work path; nothing is written to the repo. Patches applied identically to both: every `extern crate` line is removed, every glob import from a non-local crate becomes `const _: () = ();`, inner attributes move to the top of the root.
Errors and warnings (`-W unused-imports`) are compared as multisets with path, line and column stripped.
usage: scoped38.py stage|live <group> [cfg,cfg,...]    groups: see GROUPS; cfgs: lib test libfeat testfeat p2 p2test"""
import collections, os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p38lib import *
from plans38 import PLANS, ORDER, H

SBX = WORK + "/scoped"
TOOLCHAIN = "+nightly-2026-07-20"
GROUPS = {
    "cas": dict(copy=[f"{H}/🗿️artifact-authority"], root=f"{H}/🗿️artifact-authority/🦀️.rs", keys=["cas"], crate="semio_hub_artifact_authority", cfgs="lib,test,libfeat,testfeat"),
    "boot": dict(copy=[f"{H}/🚀️local-bootstrap"], root=f"{H}/🚀️local-bootstrap/🦀️.rs", keys=["boot"], crate="semio_hub_local_bootstrap", cfgs="lib,test"),
    "space": dict(copy=[f"{H}/🧩️compositions/🪐️space/⚙️engine/🪐️space"], root=f"{H}/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs", keys=["space"], crate="semio_hub_space_app", cfgs="lib,test,libfeat,p2"),
    "hostile": dict(copy=[f"{H}/🧪️tests"], root=f"{H}/🧪️tests/🔬️bin-unit/🎲️hostile-input/🏛️laws/🦀️.rs", keys=["hostile"], crate="semio_hub_hostile_input_laws", cfgs="test,testfeat"),
    "session": dict(copy=[f"{H}/🧪️tests"], root=f"{H}/🧪️tests/🔬️bin-unit/🔐️session/🏛️laws/🦀️.rs", keys=["session"], crate="semio_hub_session_route_laws", cfgs="test,testfeat"),
    "openplan": dict(copy=[f"{H}/🧪️tests"], root=f"{H}/🧪️tests/🔬️bin-unit/🧭️open-plan/🏛️laws/🦀️.rs", keys=["openplan"], crate="semio_hub_open_plan_laws", cfgs="test,testfeat"),
    "testkit": dict(copy=[f"{H}/🧪️tests"], root=f"{H}/🧪️tests/🔬️bin-unit/🧰️testkit/🦀️.rs", keys=["testkit"], crate="semio_hub_route_testkit", cfgs="lib,test"),
}
FEAT = ["--cfg", 'feature="testkit"', "--cfg", 'feature="integration-fixtures"']
CONFIGS = {
    "lib": [], "test": ["--test"], "libfeat": FEAT, "testfeat": ["--test"] + FEAT,
    "p2": ["--target", "wasm32-wasip2"], "p2test": ["--test", "--target", "wasm32-wasip2"],
}
IGNORE = shutil.ignore_patterns("target", "node_modules", ".git", "🗑️generated")
MODS = {}


def under(path):
    rp = os.path.realpath(path)
    rr = os.path.realpath(SBX)
    assert rp == rr or rp.startswith(rr + os.sep), ("write outside the sandbox", path)


def wr(path, text):
    under(os.path.dirname(path))
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)


def neutralize_text(t, mods):
    keep = {"crate", "super", "self"} | mods
    rx_ext = re.compile(r"^\s*extern\s+crate\s+\w+\s+as\s+\w+;[ \t]*$", re.M)
    rx_glob = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    n = [0, 0]
    t2 = rx_ext.sub(lambda m: (n.__setitem__(0, n[0] + 1), "")[1], t)

    def sub(m):
        if m.group(3) in keep:
            return m.group(0)
        n[1] += 1
        return m.group(1) + "const _: () = ();"
    return rx_glob.sub(sub, t2), n


def neutralize(tree):
    files, mods, texts = [], set(), {}
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith(".rs"):
                files.append(os.path.join(dp, f))
    for p in files:
        t = open(p, encoding="utf-8", errors="ignore").read()
        texts[p] = t
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", t, re.M):
            mods.add(m.group(1))
    MODS[tree] = mods
    tot = [0, 0]
    for p, t in texts.items():
        t2, n = neutralize_text(t, mods)
        tot[0] += n[0]
        tot[1] += n[1]
        if t2 != t:
            wr(p, t2)
    return tot


def pull_missing(tree, err):
    pulled = 0
    for m in re.finditer(r"couldn't find file `([^`]+)`", err):
        path = os.path.normpath(m.group(1))
        prefix = os.path.realpath(tree)
        if not (path.startswith(tree + os.sep) or path.startswith(prefix + os.sep)):
            continue
        rel = path[len(tree) + 1:] if path.startswith(tree + os.sep) else path[len(prefix) + 1:]
        src = os.path.join(ROOT, rel)
        if os.path.isfile(src) and not os.path.exists(path):
            wr(path, open(src, encoding="utf-8", errors="ignore").read())
            if path.endswith(".rs"):
                t = open(path, encoding="utf-8", errors="ignore").read()
                t2, _ = neutralize_text(t, MODS.get(tree, set()))
                if t2 != t:
                    wr(path, t2)
            pulled += 1
    return pulled


def fix_inner_attrs(path, prelude=""):
    t = open(path, encoding="utf-8").read()
    lines = t.split("\n")
    inner = [l for l in lines if re.match(r"^\s*#!\[", l)]
    rest = [l for l in lines if not re.match(r"^\s*#!\[", l)]
    wr(path, "\n".join(inner + ([prelude] if prelude else []) + rest))


def copy_dirs(g, dst):
    under(dst)
    shutil.rmtree(dst, ignore_errors=True)
    for d in g["copy"]:
        shutil.copytree(os.path.join(ROOT, d), os.path.join(dst, d), symlinks=True, ignore=IGNORE)


def restore_orig(g, dst):
    for key in g["keys"]:
        spec = PLANS[key]
        p = os.path.join(dst, spec["rel"])
        wr(p, orig_text(spec["n"]))
        for c in spec["children"]:
            d = os.path.join(os.path.dirname(p), c["folder"])
            under(d)
            shutil.rmtree(d, ignore_errors=True)


def overlay_stage(g, dst):
    for key in g["keys"]:
        sd = f"{STAGE}/{key}"
        for dp, dn, fn in os.walk(sd):
            for f in fn:
                s = os.path.join(dp, f)
                rel = os.path.relpath(s, sd)
                wr(os.path.join(dst, rel), read(s))


def run(g, gname, tree, cfg, label):
    root = os.path.join(tree, g["root"])
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", g["crate"], "--emit=metadata", "-o", "/dev/null", "--error-format=short", "-W", "unused-imports"] + CONFIGS[cfg] + [root]
    out = WORK + "/orc"
    os.makedirs(out, exist_ok=True)
    for attempt in range(80):
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(root))
        if "couldn't find file" in p.stderr and pull_missing(tree, p.stderr):
            continue
        break
    log(f"scoped-{gname}-{label}-{cfg}.err", p.stderr)
    return p.stderr


def parse(err, tree):
    err = err.replace(os.path.realpath(tree), "<T>").replace(tree, "<T>")
    err = re.sub(r"(couldn't read `)([^`]+)(`)", lambda m: m.group(1) + os.path.normpath(m.group(2)) + m.group(3), err)
    errs, warns = collections.Counter(), collections.Counter()
    for line in err.split("\n"):
        m = re.match(r"^(?:[^\s:][^:]*:\d+:\d+: )?(error(?:\[E\d+\])?|warning): (.*)$", line)
        if not m:
            continue
        kind, msg = m.group(1), m.group(2)
        if msg.startswith("aborting due to") or re.search(r"generated \d+ warning", msg):
            continue
        msg = re.sub(r": not found in this scope$", "", msg)
        (errs if kind.startswith("error") else warns)[kind + ": " + msg] += 1
    return errs, warns


def main():
    mode, gname = sys.argv[1], sys.argv[2]
    g = GROUPS[gname]
    cfgs = sys.argv[3].split(",") if len(sys.argv) > 3 else g["cfgs"].split(",")
    A, B = f"{SBX}/{gname}/orig", f"{SBX}/{gname}/new"
    os.makedirs(f"{SBX}/{gname}", exist_ok=True)
    if not os.environ.get("C4P38_REUSE"):
        copy_dirs(g, A)
        restore_orig(g, A)
        copy_dirs(g, B)
        if mode == "stage":
            overlay_stage(g, B)
        for tree in (A, B):
            n = neutralize(tree)
            fix_inner_attrs(os.path.join(tree, g["root"]), g.get("prelude", ""))
        print("neutralized extern/glob", n)
    bad = 0
    for cfg in cfgs:
        ea, eb = run(g, gname, A, cfg, "orig"), run(g, gname, B, cfg, "new")
        a_e, a_w = parse(ea, A)
        b_e, b_w = parse(eb, B)
        oa, ob, wa, wb = a_e - b_e, b_e - a_e, a_w - b_w, b_w - a_w
        print("== %s %s: errors orig=%d new=%d only-orig=%d only-new=%d | warnings orig=%d new=%d only-orig=%d only-new=%d" % (gname, cfg, sum(a_e.values()), sum(b_e.values()), sum(oa.values()), sum(ob.values()), sum(a_w.values()), sum(b_w.values()), sum(wa.values()), sum(wb.values())))
        for k, v in sorted(oa.items())[:40]:
            print("  -orig %dx %s" % (v, k[:230]))
        for k, v in sorted(ob.items())[:40]:
            print("  +new  %dx %s" % (v, k[:230]))
        for k, v in sorted(wb.items())[:20]:
            print("  +warn %dx %s" % (v, k[:200]))
        for k, v in sorted(wa.items())[:20]:
            print("  -warn %dx %s" % (v, k[:200]))
        if ob or wb or oa:
            bad += 1
    print("SCOPED", gname, "CLEAN" if not bad else "DIFFERENCES")
    sys.exit(1 if bad else 0)


main()
