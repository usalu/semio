import json
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/glulam-beam/🔣️.json"
text = open(P).read()
doc = json.loads(text)
g = doc["asset"]["representations"]["200"]["geometry"]["front"]
s = [x for x in g if x["id"] == "bearing-end"][0]
if s["points"] == [[-0.08, 0], [0.08, 0]]:
    i = text.index('"id": "bearing-end"')
    j = text.index('"closed": false', i)
    seg = text[i:j]
    seg = seg.replace("-0.08", "-0.062", 1).replace("0.08,", "0.062,", 1) if False else seg
    import re
    seg2 = re.sub(r"(\[\s*)-0\.08(,\s*0\s*\])", r"\g<1>-0.062\2", seg, count=1)
    seg2 = re.sub(r"(\[\s*)0\.08(,\s*0\s*\])", r"\g<1>0.062\2", seg2, count=1)
    assert seg2 != seg
    new = text[:i] + seg2 + text[j:]
    json.loads(new)
    open(P, "w").write(new)
    print("patched")
else:
    print("already", s["points"])
