"""Usage: python3 author-25-addshapes.py <slug> --scale 500 --from 200 --json file.json [--apply]
Appends the shapes of a json file ([{id, bone, pts}]) to the elevation list of a scale (text patch): tone, z, region(s) copied from the finer scale's shape of that bone, closed filled poly."""
import glob, json, re, sys
slug = sys.argv[1]
arg = lambda k, d=None: sys.argv[sys.argv.index(k) + 1] if k in sys.argv else d
scale, src, jf = arg("--scale"), arg("--from"), arg("--json")
path = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/%s/🔣️.json" % slug)[0]
text = open(path, encoding="utf-8").read()
asset = json.loads(text)["asset"]
new = json.load(open(jf))
fine = {s["bone"]: s for s in asset["representations"][src]["geometry"]["elevation"] if s["role"] == "body"}
objs = []
for n in new:
    t = fine[n["bone"]]
    o = {"id": n["id"], "bone": n["bone"], "role": "body", "z": t["z"], "filled": True, "tone": t["tone"]}
    for k in ("region", "regions"):
        if k in t:
            o[k] = t[k]
    o.update({"kind": "poly", "points": n["pts"], "closed": True})
    objs.append(o)
dec = json.JSONDecoder()
rep0 = text.index('"representations"')
for m in re.finditer(r'"geometry": \{', text[rep0:]):
    gs = rep0 + m.end() - 1
    scl = re.findall(r'"denominator": (\d+)', text[rep0:gs])[-1]
    if scl != scale:
        continue
    geo, ge = dec.raw_decode(text, gs)
    vm = re.compile(r'"elevation": \[').search(text, gs, ge)
    ls = vm.end() - 1
    lst, le = dec.raw_decode(text, ls)
    close = le - 2
    while text[close] in " \n":
        close -= 1
    first = text.index("{", ls)
    ind = " " * (first - text.rfind("\n", 0, first) - 1)
    existing = {sh["id"]: sh for sh in lst}
    reps = []
    pp = ls + 1
    for sh in lst:
        while text[pp] in " \n,":
            pp += 1
        ob, ee = dec.raw_decode(text, pp)
        if sh["id"] in {o["id"] for o in objs}:
            n = next(o for o in objs if o["id"] == sh["id"])
            for k in ("penMm", "d", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "solid"):
                ob.pop(k, None)
            ob.update({"filled": True, "kind": "poly", "points": n["points"], "closed": True})
            ind2 = " " * (pp - text.rfind("\n", 0, pp) - 1)
            reps.append((pp, ee, json.dumps(ob, ensure_ascii=False, indent=2).replace("\n", "\n" + ind2)))
        pp = ee
    objs = [o for o in objs if o["id"] not in existing]
    add = (",\n" + ",\n".join(ind + json.dumps(o, ensure_ascii=False, indent=2).replace("\n", "\n" + ind) for o in objs)) if objs else ""
    for r in reps:
        print("replace shape at", r[0])
    for o in objs:
        print("add", o["id"], o["bone"], o.get("region") or o.get("regions"))
    if "--apply" in sys.argv:
        fresh = open(path, encoding="utf-8").read()
        assert fresh == text
        out = text[: close + 1] + add + text[close + 1 :]
        for a0, b0, nt in sorted(reps, reverse=True):
            out = out[:a0] + nt + out[b0:]
        json.loads(out)
        open(path, "w", encoding="utf-8").write(out)
        print("patched")
    break
