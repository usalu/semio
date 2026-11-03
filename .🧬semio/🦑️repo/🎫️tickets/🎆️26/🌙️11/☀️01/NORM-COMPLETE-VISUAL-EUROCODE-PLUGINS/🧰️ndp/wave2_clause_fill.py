import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import ndplib
SPEC = {
    "ndp.en1993.v-255.1": "EN 1993-1-3 6.1.5 (V-255 shear)",
    "ndp.en1993.v-256.1": "EN 1993-1-3 6.1.7 (V-256 web crippling)",
    "ndp.en1993.v-257.1": "EN 1993-1-3 6.1.8 (V-257 combined M and web crippling)",
    "ndp.en1993.v-305.1": "EN 1993-1-4 Annex (V-305 stainless steel)",
    "ndp.en1993.v-800.1": "EN 1993-1-6 section 8 (V-800 limit states)",
    "ndp.en1993.v-851.1": "EN 1993-3-2 (V-851 chimneys)",
    "ndp.en1993.v-851.2": "EN 1993-3-2 (V-851 chimneys)",
    "ndp.en1993.v-905.1": "EN 1993-4-3 (V-905 pipelines)",
    "ndp.en1993.v-905.2": "EN 1993-4-3 (V-905 pipelines)",
    "ndp.en1993.v-905.3": "EN 1993-4-3 (V-905 pipelines)",
}
d = ndplib.load()
n = 0
for e in d.values():
    if e["clause"] != "?":
        continue
    clause = SPEC.get(e["id"]) or e["source"]["locator"]
    assert clause != "?", e["id"]
    e["clause"] = clause
    e["source"]["locator"] = clause
    n += 1
ndplib.save(d)
print("filled", n)
