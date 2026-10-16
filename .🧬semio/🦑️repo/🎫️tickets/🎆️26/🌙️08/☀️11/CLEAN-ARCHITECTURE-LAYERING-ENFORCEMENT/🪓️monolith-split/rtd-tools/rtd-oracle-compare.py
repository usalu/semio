"""📊️ Re-compares the saved A/B stub-layer error files of rtd-oracle.py (use while a long run is still going). Usage: python3 rtd-oracle-compare.py"""
import os, sys, glob, collections, re
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
sys.path.insert(0, F + "/frs-tools")
import oracle
RES = {"E0412", "E0425", "E0422", "E0423", "E0432", "E0433", "E0603", "E0616", "E0624", "E0252", "E0255", "E0428", "E0404", "E0405", "E0407", "E0408", "E0416", "E0426", "E0430", "E0431", "E0434", "E0435", "E0437", "E0438", "E0451", "E0574", "E0575", "E0576", "E0583", "E0532", "E0531", "E0530", "E0423", "E0424", "E0401", "E0411", "E0413", "E0414", "E0415", "E0429", "E0445", "E0446"}


def resolution_class(head):
    m = re.match(r"error\[(E\d+)\]", head)
    return True if not m else m.group(1) in RES


bad = 0
badres = 0
n = 0
for a in sorted(glob.glob(F + "/🗑️generated/rtd-oracle-semio-*-A.err")):
    b = a[:-6] + "-B.err"
    if not os.path.exists(b):
        continue
    ka, kb = oracle.parse(open(a, encoding="utf8").read()), oracle.parse(open(b, encoding="utf8").read())
    ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
    cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
    onlyB, onlyA = cb - ca, ca - cb
    name = os.path.basename(a)[len("rtd-oracle-semio-framework-os-renderer-"):-6]
    n += 1
    st = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
    newres = {k: v for k, v in onlyB.items() if resolution_class(k[0])}
    goneres = {k: v for k, v in onlyA.items() if resolution_class(k[0])}
    rst = "RESOLUTION-IDENTICAL" if not newres and not goneres else "RESOLUTION-DIFFERENT"
    if st != "IDENTICAL":
        bad += 1
    if rst != "RESOLUTION-IDENTICAL":
        badres += 1
    print(f"{name}: baseline={len(ka)} current={len(kb)} {st} {rst}")
    for (h, m, l), c in sorted(newres.items())[:8]:
        print(f"   !!new-resolution {c}x {h}: {m[:150]} | {l[:60]}")
    for (h, m, l), c in sorted(goneres.items())[:8]:
        print(f"   !!gone-resolution {c}x {h}: {m[:150]} | {l[:60]}")
    for (h, m, l), c in sorted(onlyB.items())[:8]:
        print(f"   +current {c}x {h}: {m[:150]} | {l[:60]}")
    for (h, m, l), c in sorted(onlyA.items())[:8]:
        print(f"   -current {c}x {h}: {m[:150]} | {l[:60]}")
print(n, "pairs;", bad, "different;", badres, "resolution-class different")
