import json, re, glob
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json")[0]
s = open(f, encoding="utf-8").read()
assert '"id": "wall-far"' not in s
X = {"50": 0.08, "100": 0.08, "200": 0.078}
pos = [m.end() for m in re.finditer(r'"elevation": \[\n', s)]
assert len(pos) == 3, pos
out = s
for p, sc in reversed(list(zip(pos, ("50", "100", "200")))):
    x = X[sc]
    block = '''            {
              "id": "wall-far",
              "bone": "pot",
              "role": "detail",
              "z": 3.1,
              "filled": true,
              "tone": 0.55,
              "kind": "rect",
              "x": %s,
              "y": 0.055,
              "w": %s,
              "h": 0.045,
              "region": "wall",
              "material": "stainless"
            },
''' % (-x, round(2 * x, 4))
    out = out[:p] + block + out[p:]
json.loads(out)
open(f, "w", encoding="utf-8").write(out)
print("ok")
