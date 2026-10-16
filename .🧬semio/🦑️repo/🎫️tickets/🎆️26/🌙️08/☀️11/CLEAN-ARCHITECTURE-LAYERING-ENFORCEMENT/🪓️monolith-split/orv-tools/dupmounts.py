"""Duplicate-mount scan: every #[path]/include target mounted by the staged/installed root and children is mounted exactly once, every mount target of the original is still mounted, and no other tracked source in the repo mentions a new child path.
usage: dupmounts.py [stage|real]"""
import collections, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from spec import SPEC
from split import src_path
from verify import files_of

which = sys.argv[1] if len(sys.argv) > 1 else "stage"
RX = re.compile(r'#\[path\s*=\s*"([^"]+)"\]|include!\("([^"]+)"\)')
bad = 0
for name, s in SPEC.items():
    t = TARGETS[name]
    base, root, kids = files_of(name, which)
    real_pk = f"{ROOT}/{t['pkgdir']}"
    def mounts_of(text, vdir):
        out = []
        for m in RX.finditer(text):
            lit = m.group(1) or m.group(2)
            if lit == ".": continue
            out.append(os.path.normpath(os.path.join(vdir, lit)))
        return out
    new = collections.defaultdict(list)
    for m in mounts_of(read(root), real_pk): new[m].append("<root>")
    for c in s["children"]:
        f = c["folder"]
        for m in mounts_of(read(f"{base}/{f}/🦀️.rs"), f"{real_pk}/{f}"): new[m].append(f)
    old = collections.Counter(mounts_of(read(src_path(name)), real_pk))
    child_targets = {os.path.normpath(f"{real_pk}/{c['folder']}/🦀️.rs") for c in s["children"]}
    multi = {k: v for k, v in new.items() if len(v) > 1}
    lost = [k for k in old if k not in new]
    unexpected = [k for k in new if k not in old and k not in child_targets]
    kid_missing = [k for k in child_targets if k not in new]
    pats = []
    for c in s["children"]:
        pats += ["-e", f"{os.path.basename(t['pkgdir'])}/{c['folder']}/🦀️.rs"]
    r = subprocess.run(["git", "-c", "core.quotepath=false", "grep", "-l", "-F", *pats, "--", f"{M}"], cwd=ROOT, capture_output=True, text=True)
    ext = [h for h in r.stdout.split("\n") if h and not h.startswith(t["pkgdir"].replace(M + "/", "") ) and not h.startswith(t["pkgdir"] + "/")]
    miss = [k for k in new if not os.path.isfile(k) and not (which == "stage" and k in child_targets)]
    print(f"{name}: mount literals in crate files {sum(len(v) for v in new.values())} (original {sum(old.values())} + {len(child_targets)} new children); targets mounted from >1 place {len(multi)}; original targets no longer mounted {len(lost)}; unexpected new targets {len(unexpected)}; children not mounted {len(kid_missing)}; missing target files {len(miss)}; external tracked mentions of new paths {len(ext)}")
    for k in list(multi)[:5]: print("   MULTI", os.path.relpath(k, ROOT), multi[k])
    for k in lost[:5]: print("   LOST", os.path.relpath(k, ROOT))
    for k in unexpected[:5]: print("   UNEXPECTED", os.path.relpath(k, ROOT))
    for k in miss[:5]: print("   MISSING", os.path.relpath(k, ROOT))
    for k in ext[:5]: print("   EXT", k)
    if multi or lost or unexpected or kid_missing or miss or ext: bad += 1
print("DUPLICATE-MOUNT SCAN", "CLEAN" if bad == 0 else "PROBLEMS")
