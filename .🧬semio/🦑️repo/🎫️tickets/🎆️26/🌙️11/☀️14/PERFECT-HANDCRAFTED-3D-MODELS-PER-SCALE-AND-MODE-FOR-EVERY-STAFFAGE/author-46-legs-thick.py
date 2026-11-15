"""Usage: python3 author-46-legs-thick.py <slug> --scale 500 [--w 0.075] [--apply] — turns the open hairline leg strokes (front) and hairline capsules (plan) of a coarse scale into closed filled legs of width w (the smallest feature), text patch of those shape objects."""
import glob, json, re, sys
slug = sys.argv[1]
arg = lambda k, d=None: sys.argv[sys.argv.index(k) + 1] if k in sys.argv else d
scale, w = arg("--scale"), float(arg("--w", "0.075"))
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
dec = json.JSONDecoder()
rep0 = text.index('"representations"')
patches = []
for m in re.finditer(r'"geometry": \{', text[rep0:]):
    gs = rep0 + m.end() - 1
    if re.findall(r'"denominator": (\d+)', text[rep0:gs])[-1] != scale:
        continue
    geo, ge = dec.raw_decode(text, gs)
    for view in ("front", "plan"):
        ls = re.compile(r'"%s": \[' % view).search(text, gs, ge).end() - 1
        lst, le = dec.raw_decode(text, ls)
        p = ls + 1
        for sh in lst:
            while text[p] in " \n,":
                p += 1
            ob, e = dec.raw_decode(text, p)
            new = None
            if ob["id"].startswith("leg-") and ob["kind"] == "poly" and ob.get("closed") is False and len(ob["points"]) == 2:
                (x0, y0), (x1, y1) = ob["points"]
                ob["points"] = [[round(x0 - w / 2, 5), y0], [round(x0 + w / 2, 5), y0], [round(x1 + w / 2, 5), y1], [round(x1 - w / 2, 5), y1]]
                ob["closed"] = True
                ob["filled"] = True
                new = ob
            elif ob["id"].startswith("leg-") and ob["kind"] == "capsule" and ob["r0"] < 0.01:
                ob["r0"] = ob["r1"] = w / 2
                ob["filled"] = True
                new = ob
            if new is not None:
                ind = " " * (p - text.rfind("\n", 0, p) - 1)
                patches.append((p, e, json.dumps(new, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)))
            p = e
    break
print("patch", len(patches))
if "--apply" in sys.argv:
    out = text
    for a, b, nt in sorted(patches, reverse=True):
        out = out[:a] + nt + out[b:]
    json.loads(out)
    assert open(path, encoding="utf-8").read() == text
    open(path, "w", encoding="utf-8").write(out)
    print("patched")
