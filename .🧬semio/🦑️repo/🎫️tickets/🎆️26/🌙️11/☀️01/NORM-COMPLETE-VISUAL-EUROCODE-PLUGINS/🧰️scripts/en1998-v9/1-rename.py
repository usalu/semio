"""en1998 v9 step 1: renames wall_storey -> wall_storey_id and liquefiable_thickness_m -> liquefiable_layer in file contents and in file/dir names of the en1998 artifact and the mutation-leaf taxonomy. Run once, python -I."""
import os, re, sys
NORM = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ROOTS = [NORM + "/🗿️artifacts/🫨️en1998", NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1"]
RULES = [
    (re.compile(r"wall_storey(?!_id)"), "wall_storey_id"), (re.compile(r"wallStorey(?!Id)"), "wallStoreyId"),
    (re.compile(r"WallStorey(?!Id)"), "WallStoreyId"), (re.compile(r"wall-storey(?!-id)"), "wall-storey-id"),
    (re.compile(r"liquefiable_thickness_m"), "liquefiable_layer"), (re.compile(r"liquefiableThicknessM"), "liquefiableLayer"),
    (re.compile(r"LiquefiableThicknessM"), "LiquefiableLayer"), (re.compile(r"liquefiable-thickness-m"), "liquefiable-layer"),
]
def tr(s):
    for r, n in RULES: s = r.sub(n, s)
    return s
files = changed = 0
paths = []
for root in ROOTS:
    for dp, dn, fs in os.walk(root):
        dn[:] = [d for d in dn if d not in ("node_modules", "target")]
        for f in fs:
            p = os.path.join(dp, f); paths.append(p)
            try: t = open(p, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError): continue
            n = tr(t); files += 1
            if n != t: open(p, "w", encoding="utf-8").write(n); changed += 1
renamed = 0
for root in ROOTS:
    for dp, dn, fs in os.walk(root, topdown=False):
        if "node_modules" in dp or "/target" in dp: continue
        for name in dn + fs:
            n = tr(name)
            if n != name:
                dst = os.path.join(dp, n)
                assert not os.path.exists(dst), dst
                os.rename(os.path.join(dp, name), dst); renamed += 1
for dp, _, fs in os.walk(ROOTS[0]):
    for f in fs:
        q = os.path.join(dp, f)[len("/home/deck/semio/"):]
        assert len(q.encode()) < 256, q
print("scanned", files, "changed", changed, "renamed", renamed)
