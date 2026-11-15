"""🛠️ Draws the pivot sleeves and end pins of the scissor lift where they bridge the gap between the inner and the outer arm strip in the front view (1:50, 1:100, 1:200). Idempotent."""
import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scissor-lift/🔣️.json"
FR = json.load(open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/🗑️generated/author-35/scissor.frames.json"))
s = open(P, encoding="utf8").read()
if '"id": "sleeve-pivot-1-l"' in s:
    raise SystemExit("already done")
ARM = {50: (0.34, 0.25, 0.06), 100: (0.34, 0.22, 0.08), 200: (0.33, 0.15, 0.12)}
PR = {50: 0.015, 100: 0.025, 200: None}
CR = {50: 0.03, 100: 0.045, 200: 0.055}


def wy(bone, u):
    f = FR[bone]
    return f["o"][1] + u * f["u"][1]


def shape(id, lo, hi, yc, r):
    out = []
    for sg, x in (("l", lo), ("r", -hi)):
        out.append('''
      {
       "id": "%s-%s",
       "bone": "chassis",
       "role": "detail",
       "z": 3.3,
       "filled": true,
       "tone": 0.1,
       "kind": "rect",
       "x": %s,
       "y": %s,
       "w": %s,
       "h": %s,
       "region": "arms",
       "material": "scissor-steel"
      }''' % (id, sg, round(x, 5), round(yc - r, 5), round(hi - lo, 5), round(2 * r, 5)))
    return out


pos = 0
for scale in (50, 100, 200):
    ca, cb, aw = ARM[scale]
    lo, hi = cb + aw / 2, ca - aw / 2
    items = []
    for i in range(1, 6):
        items += shape(f"sleeve-pivot-{i}", lo, hi, wy(f"a{i}", 0.852), CR[scale])
    if PR[scale]:
        for i in range(1, 6):
            items += shape(f"sleeve-pin-left-{i}", lo, hi, wy(f"a{i}", 0.0), PR[scale])
            items += shape(f"sleeve-pin-right-{i}", lo, hi, wy(f"b{i}", 0.0), PR[scale])
        items += shape("sleeve-pin-top-left", lo, hi, wy("b5", 1.7), PR[scale])
        items += shape("sleeve-pin-top-right", lo, hi, wy("a5", 1.7), PR[scale])
    c = s.index('"id": "lift-cylinder-eye"', pos)
    e = s.index("\n      }", c) + len("\n      }")
    ins = "," + ",".join(items)
    s = s[:e] + ins + s[e:]
    pos = e + len(ins)
open(P, "w", encoding="utf8").write(s)
print("done")
