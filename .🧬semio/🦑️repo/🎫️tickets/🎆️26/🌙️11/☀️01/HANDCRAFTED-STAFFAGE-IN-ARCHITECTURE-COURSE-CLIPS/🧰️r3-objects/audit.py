import json, glob, sys

A = "C:/git/semio/🎓️teaching/🏛️architecture"
S = A + "/🧍️staffage/🗂️catalog"
used = json.load(open(A + "/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf8"))["casts"]
usedAssets = {}
for v in used.values():
    usedAssets.setdefault(v["asset"], set()).add((v.get("pose") or v.get("clip"), v.get("viewpoint")))
cats = {"🔌️appliances", "🏗️machinery", "🪑️objects", "🛋️furniture", "🚜️vehicles"}
rows = []
for p in sorted(glob.glob(S + "/*/*/🔣️.json")):
    q = p.replace(chr(92), "/").split("/")
    cat, aid = q[-3], q[-2]
    if cat not in cats or (len(sys.argv) > 1 and sys.argv[1] == "used" and aid not in usedAssets):
        continue
    a = json.load(open(p, encoding="utf8"))["asset"]
    d = a["dimensions"]
    bones = a.get("rig", {}).get("bones", [])
    msum = sum(b.get("mass", {}).get("kg", 0) for b in bones)
    reps = a["representations"]
    views = {}
    for sc, r in reps.items():
        views[sc] = "/".join(k[:2] for k, v in r.get("geometry", {"MISSING": 1}).items() if v)
    em = a["thermal"]["emitters"]
    emdesc = ",".join(f"{e['id']}:{e['kind'][:3]}:{e.get('anchor', '-')}:{e.get('operatingKey', '-')}" for e in em)
    ancE = [x["id"] for x in a.get("anchors", []) if x.get("kind") == "emitter"]
    ops = [o["id"] for o in a["thermal"].get("operating", [])]
    print(f"## {cat}/{aid} used={sorted(map(str, usedAssets.get(aid, [])))}")
    print(f"  dims h{d['heightM']} w{d['widthM']} d{d['depthM']} mass{d.get('massKg')} bonesum{round(msum, 3)}")
    print(f"  views {views}")
    print(f"  poses {[x['id'] for x in a['poses']]} clips {[x['id'] for x in a['clips']]} ops {ops}")
    print(f"  emitters {emdesc} emitterAnchors {ancE}")
