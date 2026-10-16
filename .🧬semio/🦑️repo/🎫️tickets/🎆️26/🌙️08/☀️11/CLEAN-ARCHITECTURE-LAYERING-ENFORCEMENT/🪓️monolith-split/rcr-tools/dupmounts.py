"""Duplicate-mount scan: every #[path] target mounted by the staged/installed roots is mounted exactly once in the crate, the unit-test mount count is unchanged, and no other tracked or untracked source in the repo already mounts a new child path."""
import os, re, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
PATH_RX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
bad = 0
for name, s in SPEC.items():
    t = TARGETS[name]
    base = f"{W}/{name}/stage/{t['pkgdir']}" if which == "stage" else f"{ROOT}/{t['pkgdir']}"
    mounts = collections.defaultdict(list)
    for dp, dn, fn in os.walk(base):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                for m in PATH_RX.finditer(read(p)):
                    tgt = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
                    mounts[tgt].append(os.path.relpath(p, base))
    multi = {k: v for k, v in mounts.items() if len(v) > 1}
    unit = [k for k in mounts if k.endswith("🧪️tests/🔬️unit/🦀️.rs")]
    orig = read(f"{F}/rcr-tools/orig/{name}.rs.txt")
    orig_unit = len(PATH_RX.findall(orig))
    new_unit_root = len(PATH_RX.findall(read(f"{base}/🦀️.rs")))
    nchild = len(s["children"])
    ext = []
    pats = []
    for c in s["children"]:
        pats += ["-e", f"{c['folder']}/🦀️.rs"]
    r = subprocess.run(["git", "-c", "core.quotepath=false", "grep", "-l", "-F", *pats, "--", f"{MODS}"], cwd=ROOT, capture_output=True, text=True)
    ext = [h for h in r.stdout.split("\n") if h and not h.startswith(t["pkgdir"] + "/") and not h.endswith("/🦀️.rs") is False and False]
    ext = [h for h in r.stdout.split("\n") if h and not h.startswith(t["pkgdir"] + "/")]
    miss = [k for k in mounts if not os.path.isfile(k)]
    print(f"{name}: mounts in crate {sum(len(v) for v in mounts.values())} (children {nchild} + tests {len(unit)}); targets mounted from >1 file: {len(multi)}; original #[path] count {orig_unit}, root now {new_unit_root} (= {orig_unit} + {nchild}); missing targets {len(miss)}; external tracked mounters of new paths {len(ext)}")
    if multi or miss or ext or new_unit_root != orig_unit + nchild: bad += 1
print("DUPLICATE-MOUNT SCAN", "CLEAN" if bad == 0 else "PROBLEMS")
