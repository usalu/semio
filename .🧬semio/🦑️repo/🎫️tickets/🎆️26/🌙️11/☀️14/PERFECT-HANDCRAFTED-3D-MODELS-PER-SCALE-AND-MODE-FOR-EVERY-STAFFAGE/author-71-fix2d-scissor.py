"""🛠️ Adds the hydraulic lift cylinder and its cross tube to the front views of the scissor lift (1:50, 1:100, 1:200): seen end-on through the open corridor between the inner arm pairs. Idempotent."""
import re
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scissor-lift/🔣️.json"
s = open(P, encoding="utf8").read()
if '"id": "lift-cylinder"' in s:
    raise SystemExit("already done")
SH = ''',
      {
       "id": "lift-cylinder",
       "bone": "chassis",
       "role": "detail",
       "z": 2.6,
       "filled": true,
       "tone": 0.3,
       "kind": "rect",
       "x": -0.05,
       "y": 0.455,
       "w": 0.1,
       "h": 0.1,
       "region": "arms",
       "material": "chassis-paint"
      },
      {
       "id": "lift-cylinder-eye",
       "bone": "b1",
       "role": "detail",
       "z": 2.7,
       "filled": true,
       "tone": 0.3,
       "kind": "rect",
       "x": -0.22,
       "y": 0.478,
       "w": 0.44,
       "h": 0.07,
       "region": "arms",
       "material": "chassis-paint"
      }'''
pos = 0
n = 0
for _ in range(3):
    f = s.index('"front": [', pos)
    c = s.index('"id": "chassis-body"', f)
    e = s.index("\n      }", c) + len("\n      }")
    s = s[:e] + SH + s[e:]
    pos = e + len(SH)
    n += 1
open(P, "w", encoding="utf8").write(s)
print("inserted", n)
