"""🩹 Repairs the excavator front view at the root: the house was the whole side profile swept over the engine half, so the front drew the engine half at cab height (1.97 m). The engine half is 1.25 m high; the cab block (already drawn separately) carries the 1.97 m. Run once; the plan keeps its path."""
import re, glob
P = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/excavator/*.json")[0]
t = open(P, encoding="utf8").read()
D = {"0": "M -1.75,0 L 1.6,0 L 1.6,0.3 L 1.58,0.95 C 1.56,1.4 1.5,1.75 1.42,1.88 C 1.36,1.96 1.28,1.97 1.2,1.97 L 0.4,1.97 C 0.22,1.97 0.12,1.88 0.12,1.7 L 0.12,1.25 L -1.4,1.25 C -1.62,1.25 -1.75,1.15 -1.75,0.95 Z"}
D["-0.16"] = D["0"].replace("M -1.75,0 L 1.6,0", "M -1.75,-0.16 L 1.6,-0.16")
rect = re.compile(r'( *)"kind": "rect",\n\s+"x": -1\.75,\n\s+"y": (0|-0\.16),\n\s+"w": 3\.35,\n\s+"h": [0-9.]+,\n(\s+"id": "house",\n(?:(?!"id").)*?"z1": (1\.25|0\.1)\n)', re.S)
n = {"plan": 0, "front": 0}
def rep(m):
    if m.group(4) == "1.25":
        n["plan"] += 1
        return f'{m.group(1)}"kind": "path",\n{m.group(1)}"d": "{D[m.group(2)]}",\n{m.group(3)}'
    return m.group(0)
t2 = rect.sub(rep, t)
print(n)
open(P, "w", encoding="utf8").write(t2)
