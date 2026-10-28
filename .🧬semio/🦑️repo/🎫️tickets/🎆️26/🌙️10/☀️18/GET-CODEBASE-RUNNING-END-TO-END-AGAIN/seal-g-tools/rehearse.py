#!/usr/bin/env python3
"""🏛️ Landing rehearsal (run INSIDE a mirror, cwd = mirror root): overlays whole staged batches that touch the sealed module or the front door
(oracle2-g, conv-g, obs-g fleet) as file copies, each file only when its base hash equals the mirror file (new files: when absent), and reports every outcome.
Writes only below the working directory. usage: rehearse.py <ticket-dir>"""
import collections, hashlib, json, os, shutil, sys
C = sys.argv[1]
ROOT = os.getcwd()
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.isfile(p) else None
count = collections.Counter()


def land(lane, rel, source, base_sha):
    target = os.path.join(ROOT, rel)
    current, new = sha(target), sha(source)
    if current == new:
        state = "DONE"
    elif current == base_sha:
        state = "OK"
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(source, target)
    else:
        state = "NEW-BUT-EXISTS" if base_sha is None else ("MISSING-TARGET" if current is None else "BASE-MISMATCH")
    count[(lane, state)] += 1
    if state not in ("OK", "DONE"):
        print(lane, state, rel[-120:])


for lane, stage in (("oracle2-g", C + "/oracle2-g-tools/staged"), ("conv-g", C + "/conv-g-tools/staged")):
    for dirpath, _, names in os.walk(stage + "/files"):
        for name in names:
            rel = os.path.relpath(os.path.join(dirpath, name), stage + "/files")
            base = stage + "/base/" + rel
            land(lane, rel, os.path.join(dirpath, name), sha(base) if os.path.isfile(base) else None)
fleet = C + "/obs-g-tools/staged/fleet"
for rel, row in json.load(open(fleet + "/manifest.json", encoding="utf-8")).items():
    land("obs-g", rel, fleet + "/files/" + row["file"], row.get("base"))
print(dict(count))
