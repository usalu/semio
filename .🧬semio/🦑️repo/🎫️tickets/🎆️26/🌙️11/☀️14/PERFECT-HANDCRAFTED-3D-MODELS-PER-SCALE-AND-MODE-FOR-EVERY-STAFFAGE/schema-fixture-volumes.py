import json, sys, copy
FEATURE = 0.15
import math
def smallest(s):
    k = s["kind"]
    span = lambda v: (max(v) - min(v)) if v else 0
    if k == "box": e = s["size"]
    elif k == "ellipsoid": e = [2 * r for r in s["radii"]]
    elif k == "capsule":
        w = 2 * max(s["r0"], s["r1"]); e = [math.dist(s["p0"], s["p1"]) + s["r0"] + s["r1"], w, w * s.get("lateralScale", 1)]
    elif k == "revolve":
        w = 2 * max([0] + [q[1] for q in s["profile"]]); e = [span([q[0] for q in s["profile"]]), w, w * s.get("lateralScale", 1)]
    elif k == "extrude":
        t = max(s.get("taperTo", [1, 1])); e = [s["to"] - s["from"], span([q[0] for q in s["profile"]]) * t, span([q[1] for q in s["profile"]]) * t]
    else:
        e = [math.dist(s["sections"][0]["at"], s["sections"][-1]["at"]), 2 * max(c["rx"] for c in s["sections"]), 2 * max(c["ry"] for c in s["sections"])]
    f = min(s.get("transform", {}).get("scale", [1]))
    return min(e) * f
def convert(a):
    sol = a["volume"]["solids"]
    height = a["dimensions"]["heightM"]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    eligible = [b["id"] for b in a["rig"]["bones"] if "section" in b or b["lengthM"] >= 0.05 * height]
    volumes = {}
    for key in ("50", "100", "200", "500"):
        d = int(key)
        rep = a["representations"][key]
        if rep["status"] != "authored":
            volumes[key] = {"status": "unavailable", "denominator": d}
            continue
        feature = FEATURE * d / 1000
        body = []
        for s in sol:
            if s.get("maxDenominator") is not None and d > s["maxDenominator"]:
                continue
            if s["role"] not in ("glazing", "texture", "ground") and smallest(s) < feature:
                continue
            n = {k: v for k, v in s.items() if k not in ("region", "regions", "maxDenominator")}
            body.append(n)
        thermal = []
        for r in a["thermal"]["regions"]:
            refs = [s for s in body if r["id"] in ([next((x.get("region") for x in sol if x["id"] == s["id"]), None)] + (next((x.get("regions") or [] for x in sol if x["id"] == s["id"]), [])))]
            if not refs:
                refs = [s for s in body][:1]
            for s in refs:
                if any(t.get("id") == f"{s['id']}-{r['id']}" for t in thermal):
                    continue
                thermal.append({"id": f"{s['id']}-{r['id']}", "of": s["id"], "region": r["id"]})
        anchors = {x["id"]: x for x in a["anchors"]}
        for e in a["thermal"]["emitters"]:
            an = anchors[e["anchor"]]
            region = e.get("region") or (a["thermal"]["regions"][0]["id"])
            thermal.append({"id": f"emit-{e['id']}", "kind": "ellipsoid", "bone": an["bone"], "material": body[0]["material"], "role": "detail", "tone": 0.5, "radii": [max(feature, e["diameterM"] / 2)] * 3, "transform": {"translation": [an["x"], an["y"], an.get("z", 0)]}, "region": region, "tags": [e["id"]]})
        structure = []
        for bid in eligible:
            L = bones[bid]["lengthM"]
            r = max(0.04, feature)
            structure.append({"id": f"member-{bid}", "kind": "capsule", "bone": bid, "material": body[0]["material"], "role": "body", "tone": 0.4, "p0": [0.0, 0.0, 0.0], "p1": [L, 0.0, 0.0], "r0": r, "r1": r, "member": bid, "part": "member"})
        for sid in a["structure"]["supports"]:
            an = anchors[sid]
            structure.append({"id": f"support-{sid}", "kind": "box", "bone": an["bone"], "material": body[0]["material"], "role": "ground", "tone": 0.3, "size": [max(feature, 0.05)] * 3, "transform": {"translation": [an["x"], an["y"], an.get("z", 0)]}, "member": an["bone"], "part": "support", "anchor": sid})
        decisions = [{"en": f"Fixture model at 1:{d}, derived mechanically from the former single volume for tests only.", "de": f"Fixture-Modell bei 1:{d}, nur für Tests mechanisch aus dem früheren einzelnen Volumen abgeleitet."}]
        if d != 50:
            decisions.append({"en": "Minimal: the fixture keeps the structure of the finer scale.", "de": "Minimal: die Fixture behält die Struktur des feineren Maßstabs."})
        volumes[key] = {"status": "authored", "denominator": d, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}
    return volumes
for path in sys.argv[1:]:
    doc = json.load(open(path, encoding="utf8"))
    wrapped = "asset" in doc
    a = doc["asset"] if wrapped else doc
    if "volume" not in a:
        continue
    vols = convert(a)
    out = {}
    for k, v in a.items():
        if k == "volume":
            out["volumes"] = vols
        else:
            out[k] = v
    if wrapped:
        doc["asset"] = out
    else:
        doc = out
    raw = open(path, encoding="utf8").read()
    indent = 2 if raw.startswith("{\n  ") else None
    import re
    text = json.dumps(doc, ensure_ascii=False, indent=indent)
    text = re.sub(r"\[\s+([-0-9.eE+, \n]+?)\s+\]", lambda m: "[" + re.sub(r"\s+", " ", m.group(1)) + "]", text)
    open(path, "w", encoding="utf8").write(text + "\n")
    print("converted", path)
