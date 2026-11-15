import json, copy, os, sys
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
FIX = f"{S}/🧫️fixtures/🧊️volume-scales/🔣️.json"
src = {"volume": {"solids": json.load(open(FIX))["volumes"]["50"]["body"]}}
old = {s["id"]: s for s in src["volume"]["solids"]}

def T(en, de): return {"en": en, "de": de}

def plain(s):
    s = copy.deepcopy(s)
    for k in ("region", "regions", "maxDenominator"): s.pop(k, None)
    return s

def solid(id, kind, bone, role, tone, material="cotton-clothing", **kw):
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **kw}

def ref(id, of, region, layer=None):
    r = {"id": id, "of": of, "region": region}
    if layer: r["layer"] = layer
    return r

def side(prefix, z):
    return z

def structure(scale):
    out = []
    members = {
        "pelvis": ("box", dict(size=[0.12, 0.2, 0.3], cornerRadius=0.02, transform={"translation": [-0.02, 0, 0]}), "mass"),
    }
    out.append(solid("m-pelvis", "ellipsoid", "pelvis", "body", 0.3, radii=[0.1, 0.11, 0.15], transform={"translation": [-0.02, 0, 0]}, member="pelvis", part="mass"))
    r = {"50": 1.0, "100": 1.0, "200": 1.25, "500": 1.7}[scale]
    def cap(id, bone, p0, p1, r0, r1, part="member"):
        return solid(id, "capsule", bone, "body", 0.4, p0=p0, p1=p1, r0=round(r0 * r, 4), r1=round(r1 * r, 4), member=bone, part=part)
    out.append(cap("m-torso", "torso", [0, 0, 0], [0.45, 0, 0], 0.07, 0.075))
    out.append(cap("m-head", "head", [-0.06, 0, 0], [0.1, 0, 0], 0.05, 0.08))
    for s, z in (("l", 1), ("r", -1)):
        out.append(cap(f"m-arm-{s}", f"arm-{s}", [0, 0, 0.225 * z], [0.6, 0, 0.225 * z], 0.04, 0.032))
        out.append(cap(f"m-thigh-{s}", f"thigh-{s}", [0, 0, 0.09 * z], [0.45, 0, 0.09 * z], 0.075, 0.058))
        out.append(cap(f"m-shank-{s}", f"shank-{s}", [0, 0, 0.09 * z], [0.43, 0, 0.09 * z], 0.052, 0.036))
        out.append(solid(f"m-foot-{s}", "box", f"foot-{s}", "body", 0.3, size=[0.26, 0.03, 0.09], transform={"translation": [0.07, -0.05, 0.09 * z]}, member=f"foot-{s}", part="member"))
    if scale in ("50", "100"):
        for s, z in (("l", 1), ("r", -1)):
            out.append(solid(f"j-hip-{s}", "ellipsoid", f"thigh-{s}", "detail", 0.2, radii=[0.04, 0.04, 0.04], transform={"translation": [0, 0, 0.09 * z]}, member=f"thigh-{s}", part="joint"))
            out.append(solid(f"j-knee-{s}", "ellipsoid", f"shank-{s}", "detail", 0.2, radii=[0.035, 0.035, 0.035], transform={"translation": [0, 0, 0.09 * z]}, member=f"shank-{s}", part="joint"))
    if scale == "50":
        for s, z in (("l", 1), ("r", -1)):
            out.append(solid(f"j-shoulder-{s}", "ellipsoid", f"arm-{s}", "detail", 0.2, radii=[0.03, 0.03, 0.03], transform={"translation": [0, 0, 0.225 * z]}, member=f"arm-{s}", part="joint"))
            out.append(solid(f"j-ankle-{s}", "ellipsoid", f"foot-{s}", "detail", 0.2, radii=[0.03, 0.03, 0.03], transform={"translation": [0, 0, 0.09 * z]}, member=f"foot-{s}", part="joint"))
    for s, z in (("l", 1), ("r", -1)):
        out.append(solid(f"s-{s}", "box", f"foot-{s}", "body", 0.1, material="leather-shoe", size=[0.27, 0.015, 0.09 if scale in ("50", "100") else 0.1], transform={"translation": [0.075, -0.0725, 0.09 * z]}, member=f"foot-{s}", part="support"))
    return out

def body(scale):
    if scale == "50":
        return [plain(s) for s in src["volume"]["solids"]]
    if scale == "100":
        keep = [i for i in old if i not in ("nose", "buttons", "sole-l", "sole-r")]
        return [plain(old[i]) for i in keep]
    if scale == "200":
        o = [plain(old[i]) for i in ("head", "torso", "pelvis", "arm-l", "thigh-l", "shank-l", "shoe-l", "arm-r", "thigh-r", "shank-r", "shoe-r")]
        for s in o:
            if s["id"] == "head": s["radii"] = [0.12, 0.1, 0.09]
            if s["id"] == "torso":
                s.update(sections=[x for i, x in enumerate(s["sections"]) if i != 1])
            if s["id"].startswith(("arm", "thigh", "shank")):
                s["r0"] = max(s["r0"], 0.04); s["r1"] = max(s["r1"], 0.04)
        return o
    out = [plain(old[i]) for i in ("head",)]
    out[0]["radii"] = [0.125, 0.105, 0.095]
    torso = plain(old["torso"])
    torso["sections"] = [torso["sections"][0], torso["sections"][2]]
    out.append(torso)
    out.append(plain(old["pelvis"]))
    for s, z in (("l", 1), ("r", -1)):
        out.append(solid(f"arm-{s}", "box", f"arm-{s}", "body", 0.5, size=[0.6, 0.085, 0.085], transform={"translation": [0.3, 0, 0.225 * z]}))
        out.append(solid(f"thigh-{s}", "box", f"thigh-{s}", "body", 0.3, size=[0.45, 0.145, 0.145], transform={"translation": [0.225, 0, 0.09 * z]}))
        out.append(solid(f"shank-{s}", "box", f"shank-{s}", "body", 0.3, size=[0.43, 0.095, 0.095], transform={"translation": [0.215, 0, 0.09 * z]}))
        out.append(solid(f"shoe-{s}", "box", f"foot-{s}", "body", 0.15, material="leather-shoe", size=[0.27, 0.075, 0.1], transform={"translation": [0.07, -0.03, 0.09 * z]}))
    return out

def thermal(scale, b):
    ids = {s["id"] for s in b}
    pairs = [("head", "head"), ("hair", "head"), ("neck", "head"), ("nose", "head"), ("torso", "torso"), ("pelvis", "torso"), ("arm-l", "arms"), ("arm-r", "arms"), ("thigh-l", "legs"), ("shank-l", "legs"), ("thigh-r", "legs"), ("shank-r", "legs"), ("shoe-l", "feet"), ("shoe-r", "feet")]
    layer = {"torso": "garment", "arms": "garment", "legs": "garment"}
    out = []
    if scale in ("50", "100"):
        for of, region in pairs:
            if of in ids: out.append(ref(f"t-{of}", of, region, layer.get(region)))
        if scale == "100":
            out = [x for x in out if x["of"] not in ("hair",)]
            out.append(solid("t-hair-cap", "ellipsoid", "head", "body", 0.2, material="hair", radii=[0.12, 0.1, 0.09], transform={"translation": [0.11, 0, 0]}, region="head"))
        return out
    if scale == "200":
        for of, region in pairs:
            if of in ids: out.append(ref(f"t-{of}", of, region, layer.get(region)))
        return out
    for of, region in pairs:
        if of in ids: out.append(ref(f"t-{of}", of, region, layer.get(region)))
    return out

DEC = {
 "50": T("Smallest feature 7.5 mm: every solid of the test figure is modelled, including nose, buttons and soles; the thermal list reuses the body surfaces of the head, coat, limbs and shoes (shared geometry by construction); the structure list has capsule members, joint spheres at hip, knee, shoulder and ankle, soles as supports and the pelvis as mass.", "Kleinstes Merkmal 7,5 mm: jeder Körper der Testfigur ist modelliert, auch Nase, Knöpfe und Sohlen; die thermische Liste verwendet die Körperflächen von Kopf, Mantel, Gliedmassen und Schuhen wieder (geteilte Geometrie per Konstruktion); die Tragwerksliste hat Kapselstäbe, Gelenkkugeln an Hüfte, Knie, Schulter und Knöchel, die Sohlen als Auflager und das Becken als Masse."),
 "100": T("Smallest feature 15 mm: nose, buttons and soles are merged into their parents; the hair stays as a cap of its own thermal solid; joint spheres remain only at hip and knee.", "Kleinstes Merkmal 15 mm: Nase, Knöpfe und Sohlen gehen in ihren Eltern auf; das Haar bleibt als eigener thermischer Kappenkörper; Gelenkkugeln bleiben nur an Hüfte und Knie."),
 "200": T("Smallest feature 30 mm: the head is one ellipsoid, the torso a two-section loft, the limbs are thickened to the smallest feature and the shoes are generalised; members are thickened and joints are absorbed into the members.", "Kleinstes Merkmal 30 mm: der Kopf ist ein Ellipsoid, der Rumpf ein Loft aus zwei Schnitten, die Gliedmassen sind auf das kleinste Merkmal verdickt und die Schuhe verallgemeinert; Stäbe sind verdickt und Gelenke in den Stäben aufgegangen."),
 "500": T("Smallest feature 75 mm: head and pelvis are ellipsoids, the torso a two-section loft, limbs and shoes plain boxes; the structure keeps one thickened member per bone, the soles as supports and the pelvis as mass.", "Kleinstes Merkmal 75 mm: Kopf und Becken sind Ellipsoide, der Rumpf ein Loft aus zwei Schnitten, Gliedmassen und Schuhe einfache Quader; das Tragwerk behält einen verdickten Stab je Knochen, die Sohlen als Auflager und das Becken als Masse."),
}
vol = {}
for sc in ("50", "100", "200", "500"):
    b = body(sc)
    vol[sc] = {"status": "authored", "denominator": int(sc), "body": b, "thermal": thermal(sc, b), "structure": structure(sc), "decisions": [DEC[sc]]}
out = {"volumes": vol}
json.dump(out, open(f"{S}/🧫️fixtures/🧊️volume-scales/🔣️.json", "w"), ensure_ascii=False, indent=1)
print({k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vol.items()})
