"""Usage: python3 author-59-patch.py <slug> <ops.json> [--apply]
ops.json = [{"scale": "200", "view": "plan", "id": "hindbody", "set": {...}, "drop": [...]}]  (set/drop keys of one existing shape; add: {"add": {...shape}} appends to the view list).
Text patch of the shape objects of one 2D view (the rest of the asset stays byte-identical); prints every change."""
import glob, json, re, sys

slug, opsf = sys.argv[1], sys.argv[2]
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
ops = json.load(open(opsf))
dec = json.JSONDecoder()
rep0 = text.index('"representations"')
patches = []
for m in re.finditer(r'"geometry": \{', text[rep0:]):
    gs = rep0 + m.end() - 1
    scl = re.findall(r'"denominator": (\d+)', text[rep0:gs])[-1]
    mine = [o for o in ops if o["scale"] == scl]
    if not mine:
        continue
    geo, ge = dec.raw_decode(text, gs)
    for view in {o["view"] for o in mine}:
        vm = re.compile(r'"%s": \[' % view).search(text, gs, ge)
        ls = vm.end() - 1
        lst, le = dec.raw_decode(text, ls)
        p = ls + 1
        seen = set()
        for sh in lst:
            while text[p] in " \n,":
                p += 1
            obj, e = dec.raw_decode(text, p)
            for o in mine:
                if o["view"] == view and o.get("id") == sh["id"]:
                    seen.add(sh["id"])
                    for k in o.get("drop", []):
                        obj.pop(k, None)
                    obj.update(o.get("set", {}))
                    ind = " " * (p - text.rfind("\n", 0, p) - 1)
                    patches.append((p, e, json.dumps(obj, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)))
                    print("patch", scl, view, sh["id"])
            p = e
        for o in mine:
            if o["view"] == view and "id" in o and o["id"] not in seen and "add" not in o:
                print("MISSING", scl, view, o["id"])
        adds = [o["add"] for o in mine if o["view"] == view and "add" in o]
        if adds:
            close = le - 1
            while text[close] in " \n]":
                close -= 1
            close += 1
            first = text.index("{", ls)
            ind = " " * (first - text.rfind("\n", 0, first) - 1)
            patches.append((close, close, "".join(",\n" + ind + json.dumps(a, ensure_ascii=False, indent=2).replace("\n", "\n" + ind) for a in adds)))
            for a in adds:
                print("add", scl, view, a["id"])
if "--apply" in sys.argv:
    fresh = open(path, encoding="utf-8").read()
    assert fresh == text, "file changed under me"
    out = text
    for p, e, nt in sorted(patches, key=lambda x: -x[0]):
        out = out[:p] + nt + out[e:]
    json.loads(out)
    open(path, "w", encoding="utf-8").write(out)
    print("patched", len(patches))
