import json
p = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/solar-thermal-collector/🔣️.json"
text = open(p, encoding="utf8").read()
assert '"id": "tray-side-wall"' not in text
pts = [[-0.79047, 0.18], [0.84783, 1.32715], [0.79047, 1.40907], [-0.84783, 0.26192]]
rows = ",\n".join("        [\n         %s,\n         %s\n        ]" % (json.dumps(x), json.dumps(y)) for x, y in pts)
block = '''      {
       "id": "tray-side-wall",
       "bone": "housing",
       "role": "body",
       "z": 3.1,
       "filled": true,
       "tone": 0.35,
       "kind": "poly",
       "points": [
%s
       ],
       "closed": true,
       "region": "housing",
       "material": "anodised-aluminium"
      },
''' % rows
marker = '      {\n       "id": "tray",\n'
assert text.count(marker) == 2, text.count(marker)
text = text.replace(marker, block + marker)
json.loads(text)
open(p, "w", encoding="utf8").write(text)
print("inserted")
