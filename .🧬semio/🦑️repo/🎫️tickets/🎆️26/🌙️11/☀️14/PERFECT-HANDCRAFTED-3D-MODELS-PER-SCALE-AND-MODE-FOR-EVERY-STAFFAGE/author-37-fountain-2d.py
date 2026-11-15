import glob, json
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/drinking-fountain/🔣️.json")[0]
s = open(f, encoding="utf-8").read()
old = '''              "id": "rim-bead",
              "bone": "basin",
              "role": "detail",
              "z": 3.6,
              "filled": false,
              "tone": 0.5,
              "kind": "ellipse",
              "cx": 0,
              "cy": 0,
              "rx": 0.2,
              "ry": 0.2,'''
assert s.count(old) == 1
new = old.replace('"rx": 0.2,', '"rx": 0.205,').replace('"ry": 0.2,', '"ry": 0.205,')
s = s.replace(old, new)
json.loads(s)
open(f, "w", encoding="utf-8").write(s)
print("ok")
