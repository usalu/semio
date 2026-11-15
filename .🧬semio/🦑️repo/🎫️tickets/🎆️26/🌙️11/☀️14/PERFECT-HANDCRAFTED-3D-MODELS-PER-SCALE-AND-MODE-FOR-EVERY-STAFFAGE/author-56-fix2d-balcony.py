import json, re
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/cantilever-balcony/🔣️.json"
text = open(P).read()
doc = json.loads(text)
plan = doc["asset"]["representations"]["200"]["geometry"]["plan"]
s = [x for x in plan if x["id"] == "railing-post"][0]
if s["points"] == [[1.45, -0.5], [1.45, 0.5]]:
    i = text.index('"id": "railing-post"', text.index('"plan": [', text.index('"denominator": 200')))
    j = text.index('"closed": false', i)
    seg = text[i:j]
    seg2 = re.sub(r"(\[\s*1\.45,\s*)-0\.5(\s*\])", r"\g<1>-0.475\2", seg, count=1)
    seg2 = re.sub(r"(\[\s*1\.45,\s*)0\.5(\s*\])", r"\g<1>0.475\2", seg2, count=1)
    assert seg2 != seg
    new = text[:i] + seg2 + text[j:]
    json.loads(new)
    open(P, "w").write(new)
    print("patched")
else:
    print("already", s["points"])
