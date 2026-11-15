import glob, json, re
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json")[0]
s = open(f, encoding="utf-8").read()
out = s
n = 0
for m in reversed(list(re.finditer(r'"front": \[\n', s))):
    i = out.find('"id": "base-block"', m.end())
    k = out.rfind("{", 0, i)
    e = out.find("}", i)
    blk = out[k:e + 1]
    new = re.sub(r'"x": -0\.1995|"x": -0\.2\b', '"x": -0.15', blk, count=1)
    new = new.replace('"w": 0.4,', '"w": 0.3,')
    assert new != blk
    out = out[:k] + new + out[e + 1:]
    n += 1
seam = '''"points": [
              [
                -0.2,
                0.15
              ],
              [
                0.2,
                0.15
              ]
            ]'''
print("front base-block fixed", n, "seam occurrences", out.count('"id": "base-seam"'))
json.loads(out)
open(f, "w", encoding="utf-8").write(out)

# seam edit: second base-seam (front, 1:50) points -0.2/0.2 -> -0.15/0.15 (done by an inline edit, same session)
