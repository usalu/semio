"""🧭 C4P49 include resolution proof on the LIVE tree: every include_str!/include_bytes!/include! literal of the snapshot (resolved against the original file's folder) must equal, as a multiset of absolute paths, the literals of the children (resolved against each child's folder) plus the root's remaining ones, minus the root's own child mounts; every resolved target must exist. usage: incl.py"""
import collections, glob, json, os, re, sys
REPO = "/Users/ueli/Documents/semio/"
RX = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
ok = True
for path in sorted(glob.glob(os.path.dirname(os.path.abspath(__file__)) + "/specs/*.json")):
    s = json.load(open(path, encoding="utf-8"))
    base = REPO + os.path.dirname(s["file"])
    orig = collections.Counter(os.path.normpath(os.path.join(base, m)) for m in RX.findall(open(s["original"], encoding="utf-8").read()))
    new = collections.Counter()
    files = [(base, REPO + s["file"])] + [(f"{base}/{c['folder']}", f"{base}/{c['folder']}/🦀️.rs") for c in s["chunks"]]
    mounts = {f"{c['folder']}/🦀️.rs" for c in s["chunks"]}
    for d, f in files:
        for m in RX.findall(open(f, encoding="utf-8").read()):
            if f == REPO + s["file"] and m in mounts:
                continue
            new[os.path.normpath(os.path.join(d, m))] += 1
    missing = [p for p in new if not os.path.exists(p)]
    miss_orig = [p for p in orig if not os.path.exists(p)]
    same = orig == new
    print(f"{os.path.basename(path)} literals orig={sum(orig.values())} new={sum(new.values())} same-targets={same} missing-targets={len(missing)} (orig missing {len(miss_orig)})")
    if not same or missing:
        ok = False
        for p in list((orig - new).keys())[:3]: print("   ORIG-ONLY", p[-100:])
        for p in list((new - orig).keys())[:3]: print("   NEW-ONLY", p[-100:])
print("INCLUDES", "OK" if ok else "FAILED")
sys.exit(0 if ok else 1)
