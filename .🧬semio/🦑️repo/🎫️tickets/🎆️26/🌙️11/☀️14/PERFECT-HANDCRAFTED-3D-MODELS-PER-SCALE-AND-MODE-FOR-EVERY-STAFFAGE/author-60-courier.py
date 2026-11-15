"""Author 60: humans/courier-with-hand-truck props at all four scales (hand truck as tube frame with toe plate and two wheels, parcels as boxes with edges),
breath and plume emitters, prop structure, prop decisions. Run after `figure-build.ts humans/courier-with-hand-truck --write`: python3 author-60-courier.py [50 100 200 500]
The human lists come from the figure kit and are not touched; every prop list of a scale is rebuilt from this script (frames: hand-truck local x up the rails, y towards the courier, z lateral)."""
import json, sys, re, math

F = "volumes/humans/courier-with-hand-truck.json"
PROPS = ("hand-truck", "parcels", "truck-lift")
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
STEEL, STEEL_DARK, RUBBER, CARD = ("steel-tube", 0.35), ("steel-tube", 0.1), ("tyre-rubber", 0.85), ("cardboard", 0.52)


def mk(id, bone, mat, kind, role="body", **kw):
    m, tone = mat
    return {"id": id, "bone": bone, "material": m, "role": role, "tone": tone, "kind": kind, **kw}


def cap(id, bone, mat, p0, p1, r, role="body", **kw):
    return mk(id, bone, mat, "capsule", role, p0=p0, p1=p1, r0=r, r1=r, **kw)


def box(id, bone, mat, size, at, radius=None, role="body", **kw):
    s = mk(id, bone, mat, "box", role, size=size, transform={"translation": at}, **kw)
    if radius:
        s["cornerRadius"] = radius
    return s


def plate(id, th, lip, z):
    prof = [[0.06 - th, 0], [0.06 - th, -0.3], [0.06 - th + lip, -0.3], [0.06 - th + lip, -0.3 + th], [0.06, -0.3 + th], [0.06, 0]] if lip > 0 else [[0.06 - th, 0], [0.06 - th, -0.3], [0.06, -0.3], [0.06, 0]]
    return mk(id, "hand-truck", STEEL, "extrude", profile=prof, plane="xy", **{"from": -z, "to": z})


def tyre(r, hw, side, cap_ring=True):
    prof = [[-hw, 0], [-hw, r * 0.77], [-hw * 0.8, r * 0.946], [-hw * 0.4, r * 0.996], [0, r], [hw * 0.4, r * 0.996], [hw * 0.8, r * 0.946], [hw, r * 0.77], [hw, 0]]
    return prof


def props(sc):
    S = {}
    z = 0.21
    if sc == 50:
        S["frame"] = [
            cap("rail-l", "hand-truck", STEEL, [-0.08, 0, z], [1.0, 0, z], 0.016),
            cap("rail-r", "hand-truck", STEEL, [-0.08, 0, -z], [1.0, 0, -z], 0.016),
            cap("handle", "hand-truck", STEEL_DARK, [1.045, 0.095, -z], [1.045, 0.095, z], 0.02),
            cap("handle-arm-l", "hand-truck", STEEL_DARK, [0.985, 0.012, z], [1.04, 0.12, z], 0.011),
            cap("handle-arm-r", "hand-truck", STEEL_DARK, [0.985, 0.012, -z], [1.04, 0.12, -z], 0.011),
            cap("brace-0", "hand-truck", STEEL, [0.45, 0, -z], [0.45, 0, z], 0.01),
            cap("brace-1", "hand-truck", STEEL, [0.8, 0, -z], [0.8, 0, z], 0.01),
            cap("stay-l-1", "hand-truck", STEEL, [0.02, 0, 0.19], [0.02, -0.12, 0.19], 0.0075),
            cap("stay-l-2", "hand-truck", STEEL, [0.02, -0.12, 0.19], [0.06, -0.29, 0.19], 0.0075),
            cap("stay-r-1", "hand-truck", STEEL, [0.02, 0, -0.19], [0.02, -0.12, -0.19], 0.0075),
            cap("stay-r-2", "hand-truck", STEEL, [0.02, -0.12, -0.19], [0.06, -0.29, -0.19], 0.0075),
            plate("plate", 0.01, 0.026, 0.19),
            cap("axle-stay", "hand-truck", STEEL, [-0.016, 0.0, -0.21], [-0.016, 0.0, 0.21], 0.015),
        ]
        S["wheels"] = [
            mk("tyre", "truck-lift", RUBBER, "revolve", profile=tyre(0.13, 0.025, 0), axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.275]}, repeat={"count": 2, "translation": [0, 0, 0.55]}),
            mk("hub", "truck-lift", (RUBBER[0], 0.5), "revolve", "detail", profile=[[-0.03, 0], [-0.03, 0.055], [0.03, 0.055], [0.03, 0]], axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.275]}, repeat={"count": 2, "translation": [0, 0, 0.55]}),
            cap("axle", "truck-lift", STEEL_DARK, [0, 0, -0.285], [0, 0, 0.285], 0.012, "detail"),
        ]
        S["parcels"] = [
            box("parcel-a", "parcels", CARD, [0.4, 0.4, 0.46], [0.26, -0.22, 0], 0.01),
            box("parcel-b", "parcels", CARD, [0.3, 0.36, 0.4], [0.61, -0.2, 0], 0.01),
            box("parcel-c", "parcels", CARD, [0.2337, 0.2838, 0.36], [0.8769, -0.1581, 0], 0.01),
            box("tape-a", "parcels", (CARD[0], 0.2), [0.4, 0.004, 0.05], [0.26, -0.4185, 0], role="detail", texture=True),
            box("tape-b", "parcels", (CARD[0], 0.2), [0.3, 0.004, 0.05], [0.61, -0.3785, 0], role="detail", texture=True),
            box("tape-c", "parcels", (CARD[0], 0.2), [0.2337, 0.004, 0.05], [0.8769, -0.2985, 0], role="detail", texture=True),
        ]
    elif sc == 100:
        S["frame"] = [
            cap("rail-l", "hand-truck", STEEL, [-0.08, 0, z], [1.0, 0, z], 0.016),
            cap("rail-r", "hand-truck", STEEL, [-0.08, 0, -z], [1.0, 0, -z], 0.016),
            cap("handle", "hand-truck", STEEL_DARK, [1.045, 0.095, -z], [1.045, 0.095, z], 0.02),
            cap("handle-arm-l", "hand-truck", STEEL_DARK, [0.985, 0.012, z], [1.04, 0.12, z], 0.011),
            cap("handle-arm-r", "hand-truck", STEEL_DARK, [0.985, 0.012, -z], [1.04, 0.12, -z], 0.011),
            cap("brace-0", "hand-truck", STEEL, [0.45, 0, -z], [0.45, 0, z], 0.0105),
            cap("brace-1", "hand-truck", STEEL, [0.8, 0, -z], [0.8, 0, z], 0.0105),
            cap("stay-l", "hand-truck", STEEL, [0.02, 0, 0.19], [0.06, -0.29, 0.19], 0.0105),
            cap("stay-r", "hand-truck", STEEL, [0.02, 0, -0.19], [0.06, -0.29, -0.19], 0.0105),
            plate("plate", 0.015, 0.03, 0.19),
            cap("axle-stay", "hand-truck", STEEL, [-0.016, 0.0, -0.21], [-0.016, 0.0, 0.21], 0.015),
        ]
        S["wheels"] = [
            mk("tyre", "truck-lift", RUBBER, "revolve", profile=tyre(0.13, 0.025, 0), axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.275]}, repeat={"count": 2, "translation": [0, 0, 0.55]}),
            mk("hub", "truck-lift", (RUBBER[0], 0.5), "revolve", "detail", profile=[[-0.03, 0], [-0.03, 0.055], [0.03, 0.055], [0.03, 0]], axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.275]}, repeat={"count": 2, "translation": [0, 0, 0.55]}),
            cap("axle", "truck-lift", STEEL_DARK, [0, 0, -0.285], [0, 0, 0.285], 0.0125, "detail"),
        ]
        S["parcels"] = [
            box("parcel-a", "parcels", CARD, [0.4, 0.4, 0.46], [0.26, -0.22, 0], 0.012),
            box("parcel-b", "parcels", CARD, [0.3, 0.36, 0.4], [0.61, -0.2, 0], 0.012),
            box("parcel-c", "parcels", CARD, [0.2337, 0.2838, 0.36], [0.8769, -0.1581, 0], 0.012),
        ]
    elif sc == 200:
        S["frame"] = [
            cap("rail-l", "hand-truck", STEEL, [-0.08, 0, z], [1.0, 0, z], 0.016),
            cap("rail-r", "hand-truck", STEEL, [-0.08, 0, -z], [1.0, 0, -z], 0.016),
            cap("handle", "hand-truck", STEEL_DARK, [1.045, 0.095, -z], [1.045, 0.095, z], 0.02),
            cap("handle-arm-l", "hand-truck", STEEL_DARK, [0.985, 0.012, z], [1.04, 0.12, z], 0.015),
            cap("handle-arm-r", "hand-truck", STEEL_DARK, [0.985, 0.012, -z], [1.04, 0.12, -z], 0.015),
            cap("stay-l", "hand-truck", STEEL, [0.0, 0, 0.19], [0.045, -0.29, 0.19], 0.015),
            cap("stay-r", "hand-truck", STEEL, [0.0, 0, -0.19], [0.045, -0.29, -0.19], 0.015),
            plate("plate", 0.03, 0.0, 0.19),
        ]
        S["wheels"] = [
            mk("tyre", "truck-lift", RUBBER, "revolve", profile=tyre(0.13, 0.025, 0), axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.275]}, repeat={"count": 2, "translation": [0, 0, 0.55]}),
            cap("axle", "truck-lift", STEEL_DARK, [0, 0, -0.285], [0, 0, 0.285], 0.0155, "detail"),
        ]
        S["parcels"] = [
            box("parcel-a", "parcels", CARD, [0.4, 0.4, 0.46], [0.26, -0.22, 0]),
            box("parcel-b", "parcels", CARD, [0.3, 0.36, 0.4], [0.61, -0.2, 0]),
            box("parcel-c", "parcels", CARD, [0.22, 0.28, 0.36], [0.87, -0.16, 0]),
        ]
    else:
        S["frame"] = [
            cap("rail-l", "hand-truck", STEEL, [0.0, 0, 0.21], [1.03, 0, 0.21], 0.05),
            cap("rail-r", "hand-truck", STEEL, [0.0, 0, -0.21], [1.03, 0, -0.21], 0.05),
            cap("handle", "hand-truck", STEEL_DARK, [1.05, 0.0, -0.21], [1.05, 0.0, 0.21], 0.05),
            plate("plate", 0.075, 0.0, 0.19),
        ]
        S["wheels"] = [
            mk("tyre", "truck-lift", RUBBER, "revolve", profile=tyre(0.13, 0.05, 0), axis=[0, 0, 1], origin=[0, 0, 0], transform={"translation": [0, 0, -0.25]}, repeat={"count": 2, "translation": [0, 0, 0.5]}),
        ]
        S["parcels"] = [
            box("parcel-a", "parcels", CARD, [0.4, 0.4, 0.46], [0.26, -0.22, 0]),
            box("parcel-b", "parcels", CARD, [0.3, 0.36, 0.46], [0.61, -0.2, 0]),
            box("parcel-c", "parcels", CARD, [0.22, 0.28, 0.46], [0.87, -0.16, 0]),
        ]
    return S


def thermal_props(sc, S):
    out = []
    for s in S["frame"]:
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "hand-truck"})
    for s in S["wheels"]:
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "hand-truck" if s["id"] == "axle" else "tyres"})
    for s in S["parcels"]:
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": "parcels"})
    return out


def emitters(sc, body_head):
    r = {50: 0.0045, 100: 0.00825, 200: 0.0165, 500: 0.04125}[sc]
    mat = body_head
    base = {"bone": "head", "material": mat, "role": "body", "tone": 0.5, "kind": "ellipsoid"}
    return [
        {"id": "th-nose-breath", **base, "radii": [r, r, r], "transform": {"translation": [0.03, -0.104, 0]}, "region": "face-skin", "tags": ["emitter", "nose-breath"]},
        {"id": "th-mouth-breath", **base, "radii": [r, r, r], "transform": {"translation": [-0.005, -0.0801, 0]}, "region": "face-skin", "tags": ["emitter", "mouth-breath"]},
        {"id": "th-body-plume", **base, "radii": [r, 0.125, 0.125], "transform": {"translation": [0.175, 0, 0]}, "region": "jacket", "tags": ["emitter", "body-plume"]},
    ]


def structure_props(sc, S):
    out = []
    for s in S["frame"]:
        if s["id"] in ("axle-stay",) and sc != 50:
            continue
        out.append({"id": f"member-{s['id']}", "of": s["id"], "member": "hand-truck", "part": "member"})
    for s in S["wheels"]:
        if s["id"] == "tyre":
            out.append({"id": "member-tyre", "of": "tyre", "member": "truck-lift", "part": "member"})
        elif s["id"] == "axle":
            out.append({"id": "member-axle", "of": "axle", "member": "truck-lift", "part": "member"})
    first = S["parcels"][0]["id"]
    out.append({"id": "member-parcels", "of": first, "member": "parcels", "part": "member"})
    rr = {50: 0.13, 100: 0.13, 200: 0.14, 500: 0.16}[sc]
    out.append({"id": "mass-parcels", "bone": "parcels", "material": CARD[0], "role": "detail", "tone": 0.6, "kind": "ellipsoid", "radii": [rr, rr, rr], "transform": {"translation": [0.26, -0.22, 0] if sc != 500 else [0.3, -0.22, 0]}, "member": "parcels", "part": "mass"})
    return out


TXT = {
    50: ("1:50 props: the hand truck is a tube frame (two 32 mm rails, handle bar with two bent arms, two cross braces, two side stays to the toe plate, an axle stay), the toe plate is a 10 mm sheet with an upturned 26 mm lip, two pneumatic tyres 0.26 m by 50 mm on hubs with a 0.57 m axle; the three parcels are cardboard boxes with 10 mm rounded edges stacked on the plate, their tapes are texture solids. Thermal: every prop part references its body solid. Structure: every frame part, tyre, axle and the bottom parcel as members, a 38 kg mass ellipsoid in the parcels.",
         "1:50-Requisiten: Die Sackkarre ist ein Rohrrahmen (zwei 32-mm-Holme, Griffstange mit zwei gebogenen Armen, zwei Querstreben, zwei Seitenstreben zur Schaufel, eine Achsstrebe), die Schaufel ein 10 mm dickes Blech mit aufgebogener 26-mm-Lippe, zwei Luftreifen 0,26 m mal 50 mm auf Naben mit 0,57 m Achse; die drei Pakete sind Pappkartons mit 10 mm gerundeten Kanten, auf der Schaufel gestapelt, ihre Klebebänder sind Textur-Körper. Thermik: jedes Requisitenteil verweist auf seinen Körper-Solid. Tragwerk: jedes Rahmenteil, die Reifen, die Achse und das unterste Paket als Glieder, ein 38-kg-Massenellipsoid in den Paketen."),
    100: ("1:100 props modelled anew: rails, handle with arms, two cross braces, one side stay per side (the two stay segments merged), toe plate 15 mm with a 30 mm lip, axle stay, two tyres on hubs; parcels with 12 mm edges, tapes (4 mm) left out. Thermal references and structure members as at 1:50.",
          "1:100-Requisiten neu modelliert: Holme, Griff mit Armen, zwei Querstreben, je Seite eine Seitenstrebe (die zwei Stababschnitte verschmolzen), Schaufel 15 mm mit 30-mm-Lippe, Achsstrebe, zwei Reifen auf Naben; Pakete mit 12-mm-Kanten, Klebebänder (4 mm) entfallen. Thermik-Verweise und Tragwerksglieder wie bei 1:50."),
    200: ("1:200 props modelled anew: rails, handle with two arms, one side stay per side, 30 mm toe plate, two tyres on one axle (cross braces, hubs, stay and tapes are below 30 mm and left out); three plain boxes. Thermal references and structure members as at 1:50.",
          "1:200-Requisiten neu modelliert: Holme, Griff mit zwei Armen, je Seite eine Seitenstrebe, 30-mm-Schaufel, zwei Reifen auf einer Achse (Querstreben, Naben, Strebe und Klebebänder liegen unter 30 mm und entfallen); drei schlichte Kisten. Thermik-Verweise und Tragwerksglieder wie bei 1:50."),
    500: ("1:500 props modelled anew as merged masses: two 100 mm rails, a handle bar, the 75 mm toe plate, two 100 mm wheels and one box for the whole parcel stack (as drawn). Thermal references and structure members as at 1:50.",
          "1:500-Requisiten neu als zusammengefasste Massen modelliert: zwei 100-mm-Holme, eine Griffstange, die 75-mm-Schaufel, zwei 100-mm-Räder und drei schlichte Kisten des Stapels, alle 0,46 m breit wie in Vorder- und Draufsicht gezeichnet. Thermik-Verweise und Tragwerksglieder wie bei 1:50."),
}


def main():
    src = json.load(open(F))
    for sc in [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]:
        v = src["volumes"][str(sc)]
        head = next((s["material"] for s in v["body"] if s["bone"] == "head"), "skin")
        S = props(sc)
        v["body"] = [s for s in v["body"] if s["bone"] not in PROPS] + S["frame"] + S["wheels"] + S["parcels"]
        v["thermal"] = [t for t in v["thermal"] if t.get("region") not in ("hand-truck", "tyres", "parcels") and "tags" not in t] + thermal_props(sc, S) + emitters(sc, head)
        v["structure"] = [s for s in v["structure"] if s.get("bone") not in PROPS and s.get("member") not in PROPS] + structure_props(sc, S)
        n = {"body": len(v["body"]), "thermal": len(v["thermal"]), "structure": len(v["structure"])}
        d = [x for x in v["decisions"] if not x["en"].startswith(f"1:{sc} props")][:3]
        for x, key, lab in zip(d, ("body", "thermal", "structure"), (("body", "Körper"), ("Thermal", "Thermik"), ("Structure", "Tragwerk"))):
            x["en"] = re.sub(r"\b\d+ solids", f"{n[key]} solids", x["en"], count=1)
            x["de"] = re.sub(r"\b\d+ Solids", f"{n[key]} Solids", x["de"], count=1)
        d.append({"en": TXT[sc][0], "de": TXT[sc][1]})
        v["decisions"] = d
        print(sc, n)
    json.dump(src, open(F, "w"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
