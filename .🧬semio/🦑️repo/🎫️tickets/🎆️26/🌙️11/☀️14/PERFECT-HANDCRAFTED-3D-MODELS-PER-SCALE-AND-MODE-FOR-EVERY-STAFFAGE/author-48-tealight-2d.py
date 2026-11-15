import json, glob, re
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json")[0]
s = open(f, encoding="utf-8").read()
pat = re.compile(r'("id": "wax-surface",\n\s+"bone": "wax",.*?"points": \[\n)(.*?)(\n\s+\],\n\s+"closed": false)', re.S)
n = 0


def fix(m):
    global n
    n += 1
    body = re.sub(r"(\n\s+)(0\.0138|0\.0144|0\.0152|0\.0158)(\n)", lambda k: k.group(1) + {"0.0138": "0.0126", "0.0144": "0.0132", "0.0152": "0.014", "0.0158": "0.0146"}[k.group(2)] + k.group(3), m.group(2))
    return m.group(1) + body + m.group(3)


out = pat.sub(fix, s)
assert n == 2, n
json.loads(out)
open(f, "w", encoding="utf-8").write(out)
print("ok", n)
