"""🔧 Round 5 root fix of the excavator: the tail swing radius goes from 2.95 to the 2.83 m of the Cat 320 data sheet, so the engine house, its hood, grilles and stack move 0.12 m forward with the counterweight (exact tokens on the named lines only), the frame cross member follows, and the transport pose folds the stick and curls the bucket fully (machine height 3.16 m of the data sheet). Idempotent: refuses to run twice."""
import re, sys
D = 0.12
XS = {"-2.38", "-2.28", "-1.30", "-1.79", "-1.825", "-2.20", "-1.45", "-2.25", "-2.17", "-1.48", "-1.775", "-1.80", "-2.30"}
def shift(path, lines):
    src = open(path, encoding="utf8").read().split("\n")
    for i in lines:
        s = src[i - 1]
        assert "F.REAR" not in s, (path, i)
        src[i - 1] = re.sub(r"(?<![\w.])-\d\.\d+", lambda m: ("%.3f" % (float(m.group(0)) + D)).rstrip("0") if m.group(0) in XS else m.group(0), s)
        print(path, i, src[i - 1].strip()[:150])
    open(path, "w", encoding="utf8").write("\n".join(src))
shift("r3-excavator-s50.py", [140, 142, 143, 147, 148, 149, 151, 152, 153, 154, 155, 156, 157, 160, 161, 290])
shift("r3-excavator-coarse.py", [111, 114, 115, 121, 122, 125, 126, 127, 128, 131, 132, 133])
shift("r3-excavator-s100.py", [101])
shift("r3-excavator-s200.py", [73])
