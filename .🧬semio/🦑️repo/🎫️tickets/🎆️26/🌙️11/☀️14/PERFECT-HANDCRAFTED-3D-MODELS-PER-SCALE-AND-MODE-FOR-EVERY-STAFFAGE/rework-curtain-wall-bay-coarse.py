"""🏢 Curtain wall bay, coarse scales modelled for their own smallest feature (15 / 30 / 75 mm): the stick system of tube, isolator, pressure plate, cap and gaskets (1:50) becomes one cap-web-tube profile per member (1:100), one plain bar per member (1:200) and two infill blocks between the mullion backs (1:500).

Usage: python3 rework-curtain-wall-bay-coarse.py   (rewrites the three coarse scales of `volumes/components/curtain-wall-bay.json`; at 1:50 only the base spigots change: 42 mm wide, a sliding fit with 1 mm play in the 44 mm tube instead of 3 mm of air that left them unconnected)
Frame of bone `frame`: u across the bay (0 to 1.4 m), v up (0 to 3.3 m), w through the wall from the cap face (-0.1) to the mullion back (0.1); transoms end at 0.05. Bone `igu` starts at (0.05, 0.9), bone `spandrel` at (0.05, 0.05), bone `cradle` at (0.05, 0.875).
Members 50 mm wide: cap and pressure plate -0.1 to -0.088, glazing plane -0.084 to -0.056, tube from -0.046. Glass 6/16/6 mm, 1.3 x 2.35 m visible; spandrel 1.3 x 0.8 m of enamel glass, air, 100 mm wool and steel pan to 0.0435.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "components", "curtain-wall-bay.json")

WIDTH, HEIGHT, MEMBER = 1.4, 3.3, 0.05
FACE, CAP_BACK, GLASS_BACK, TRANSOM_BACK, MULLION_BACK = -0.1, -0.084, -0.056, 0.05, 0.1
REBATE = 0.015
SPANDREL_BACK = 0.0435
TRANSOMS = {"base": 0.0, "mid": 0.85, "head": 3.25}
BLOCK_BACK = 0.02
BACK_WIDTH = 0.075
SPIGOT_WIDTH = 0.042
FEATURE = {"100": 0.015, "200": 0.03, "500": 0.075}

DECISIONS = {
    "100": [
        {"en": "Smallest feature 15 mm: tube, isolator, pressure plate, cap and gaskets of every member are one profile of cap (16 mm), 20 mm web and tube, with the two glazing rebates between them; panes, cavity and edge seal are one 28 mm glazing strip that ends in the rebates; enamel glass, air, wool and pan of the spandrel are one 127.5 mm strip between the tubes.", "de": "Kleinstes Merkmal 15 mm: Rohr, Isolator, Pressleiste, Deckschale und Dichtungen jedes Profils sind ein Profil aus Deckleiste (16 mm), 20-mm-Steg und Rohr mit den zwei Glasfalzen dazwischen; Scheiben, Zwischenraum und Randverbund sind ein 28 mm starker Glasstreifen, der in den Falzen endet; Emailglas, Luft, Dämmung und Wanne der Brüstung sind ein 127,5-mm-Streifen zwischen den Rohren."},
        {"en": "Left out: chambers, screws, gaskets and stub mullions. Slab cleats and hanging bolts stay, the base spigots lie in the solid mullion profile and stay as supports of the structure list; the setting blocks and the cradle hook of the structure list are pads one feature thick.", "de": "Entfallen: Kammern, Schrauben, Dichtungen und Profilstummel. Deckenlaschen und Hängebolzen bleiben, die Fußzapfen liegen im vollen Pfostenprofil und bleiben als Auflager der Tragwerksliste; die Klotzung und der Gondelhaken der Tragwerksliste sind Polster von einem Merkmal Stärke."},
    ],
    "200": [
        {"en": "Smallest feature 30 mm: cap, rebate and tube of every member are one plain bar of its real 50 mm width and depth (mullions 200 mm, transoms 150 mm), as the drawing shows them; the glazing is one strip of its true 28 mm between the bars and the spandrel one 127.5 mm strip.", "de": "Kleinstes Merkmal 30 mm: Deckleiste, Falz und Rohr jedes Profils sind ein glatter Stab in seiner wahren Breite von 50 mm und Tiefe (Pfosten 200 mm, Riegel 150 mm), wie die Zeichnung sie zeigt; die Verglasung ist ein Streifen in wahrer Stärke von 28 mm zwischen den Stäben und die Brüstung ein 127,5-mm-Streifen."},
        {"en": "Left out: slab cleats, hanging bolts, spigots and setting blocks; the structure list keeps the four supports as solids of their own and the transom connectors.", "de": "Entfallen: Deckenlaschen, Hängebolzen, Fußzapfen und Klotzung; die Tragwerksliste behält die vier Auflager als eigene Körper und die Riegelverbinder."},
    ],
    "500": [
        {"en": "Smallest feature 75 mm: mullions (0.1 mm on paper), transoms and glass are one vision block of 2.45 m and one spandrel block of 0.85 m, both 120 mm deep, between the two mullion backs that give the plan its step, as the drawing shows the bay.", "de": "Kleinstes Merkmal 75 mm: Pfosten (0,1 mm auf dem Papier), Riegel und Glas sind ein Sichtfeldblock von 2,45 m und ein Brüstungsblock von 0,85 m, beide 120 mm tief, zwischen den zwei Pfostenrücken, die dem Grundriss seine Stufe geben, wie die Zeichnung das Feld zeigt."},
        {"en": "Left out: individual members, glass layers, spandrel build-up and every fixing; the thermal list keeps panes, cavity, breaks and caps as layer shells.", "de": "Entfallen: einzelne Profile, Glasschichten, Brüstungsaufbau und alle Befestigungen; die Thermikliste behält Scheiben, Zwischenraum, Trennstege und Deckschalen als Schichtschalen."},
    ],
}


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def box(identifier, bone, material, tone, size, centre, **more):
    return {"id": identifier, "kind": "box", "bone": bone, "material": material, "role": "body", "tone": tone, "size": [round(value, 6) for value in size], "transform": {"translation": [round(value, 6) for value in centre]}, **more}


def panel(identifier, bone, material, role, tone, width, height, front, back, edge=0.0):
    profile = [[-edge, -edge], [width + edge, -edge], [width + edge, height + edge], [-edge, height + edge]]
    return {"id": identifier, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": [[round(x, 6), round(y, 6)] for x, y in profile], "plane": "xy", "from": front, "to": back}


def mullion_profile(tone):
    a, b = REBATE, MEMBER - REBATE
    outline = [[FACE, 0.0], [FACE, MEMBER], [CAP_BACK, MEMBER], [CAP_BACK, b], [GLASS_BACK, b], [GLASS_BACK, MEMBER], [MULLION_BACK, MEMBER], [MULLION_BACK, 0.0], [GLASS_BACK, 0.0], [GLASS_BACK, a], [CAP_BACK, a], [CAP_BACK, 0.0]]
    return {"id": "mullion", "kind": "extrude", "bone": "frame", "material": "aluminium", "role": "body", "tone": tone, "profile": outline, "plane": "zx", "from": 0.0, "to": HEIGHT, "repeat": {"count": 2, "translation": [WIDTH - MEMBER, 0.0, 0.0]}}


def transom_profile(name, tone):
    y = TRANSOMS[name]
    a, b, top = round(y + REBATE, 6), round(y + MEMBER - REBATE, 6), round(y + MEMBER, 6)
    outline = [[y, FACE], [top, FACE], [top, CAP_BACK], [b, CAP_BACK], [b, GLASS_BACK], [top, GLASS_BACK], [top, TRANSOM_BACK], [y, TRANSOM_BACK], [y, GLASS_BACK], [a, GLASS_BACK], [a, CAP_BACK], [y, CAP_BACK]]
    return {"id": f"transom-{name}", "kind": "extrude", "bone": "frame", "material": "aluminium", "role": "body", "tone": tone, "profile": outline, "plane": "yz", "from": MEMBER, "to": WIDTH - MEMBER}


def mullion_bar(tone):
    return box("mullion", "frame", "aluminium", tone, [MEMBER, HEIGHT, MULLION_BACK - FACE], [MEMBER / 2, HEIGHT / 2, 0.0], repeat={"count": 2, "translation": [WIDTH - MEMBER, 0.0, 0.0]})


def transom_bar(name, tone):
    return box(f"transom-{name}", "frame", "aluminium", tone, [WIDTH - 2 * MEMBER, MEMBER, TRANSOM_BACK - FACE], [WIDTH / 2, TRANSOMS[name] + MEMBER / 2, (FACE + TRANSOM_BACK) / 2])


def caps():
    shells = [box("th-mullion-cap", "frame", "aluminium", 0.5, [MEMBER, HEIGHT, 0.012], [MEMBER / 2, HEIGHT / 2, FACE + 0.006], repeat={"count": 2, "translation": [WIDTH - MEMBER, 0.0, 0.0]}, region="mullion", layer="cap", sectionFaces=["-v", "+v", "+w"])]
    for name, y in TRANSOMS.items():
        shells.append(box(f"th-transom-{name}-cap", "frame", "aluminium", 0.5, [WIDTH - 2 * MEMBER, MEMBER, 0.012], [WIDTH / 2, y + MEMBER / 2, FACE + 0.006], region="transom", layer="cap", sectionFaces=["+w"]))
    return shells


def thermal(volume):
    kept = [solid for solid in volume["thermal"] if "of" not in solid and not solid["id"].endswith("-plate")]
    breaks = [solid for solid in kept if solid["id"].endswith("-break")]
    others = [solid for solid in kept if not solid["id"].endswith("-break")]
    shells = by_id(caps())
    frame = []
    for solid in breaks:
        frame.append(solid)
        frame.append(shells[solid["id"].replace("-break", "-cap")])
    return others + frame


def member(identifier, of, bone):
    return {"id": identifier, "of": of, "member": bone, "part": "member"}


def masses(volume):
    return [solid for solid in volume["structure"] if solid["part"] == "mass"]


def hook(scale):
    edge = max(0.04, FEATURE[scale]) if scale == "500" else 0.04
    return box("joint-cradle-hook", "cradle", "steel", 0.3, [edge, edge, FEATURE[scale]], [0.0, 0.0, FACE + FEATURE[scale] / 2], member="cradle", part="joint")


def setting(identifier, bone, scale):
    thickness = FEATURE[scale]
    return box(identifier, bone, "epdm", 0.2, [0.1, thickness, GLASS_BACK - CAP_BACK], [0.15, -REBATE - thickness / 2, (CAP_BACK + GLASS_BACK) / 2], repeat={"count": 2, "translation": [1.0, 0.0, 0.0]}, member=bone, part="joint")


def spigot(identifier, anchor, centre, tone):
    return box(identifier, "frame", "aluminium", tone, [SPIGOT_WIDTH, 0.2, 0.134], [centre, 0.1, 0.027], member="frame", part="support", anchor=anchor)


def scale_50(volume):
    for solid in volume["body"]:
        if solid["id"] in ("spigot-left", "spigot-right"):
            solid["size"][0] = SPIGOT_WIDTH


def scale_100(volume):
    body = by_id(volume["body"])
    structure = by_id(volume["structure"])
    aluminium, glass, enamel = body["mullion-tube"]["tone"], body["igu-pane-out"]["tone"], body["spandrel-enamel"]["tone"]
    members = [mullion_profile(aluminium)] + [transom_profile(name, aluminium) for name in TRANSOMS]
    volume["thermal"] = thermal(volume)
    volume["body"] = members + [
        panel("igu", "igu", "glass", "glazing", glass, 1.3, 2.35, CAP_BACK, GLASS_BACK, REBATE),
        panel("spandrel", "spandrel", "spandrel-glass", "body", enamel, 1.3, 0.8, CAP_BACK, SPANDREL_BACK),
        body["slab-cleat"], body["hanging-bolt-left"], body["hanging-bolt-right"],
    ]
    volume["structure"] = [
        member("member-mullion", "mullion", "frame"),
        *[member(f"member-transom-{name}", f"transom-{name}", "frame") for name in TRANSOMS],
        member("member-igu", "igu", "igu"),
        member("member-spandrel", "spandrel", "spandrel"),
        structure["joint-connector-base"], structure["joint-connector-mid"], structure["joint-connector-head"],
        setting("joint-igu-setting-blocks", "igu", "100"),
        setting("joint-spandrel-setting-blocks", "spandrel", "100"),
        hook("100"),
        structure["support-head-left"], spigot("support-base-left", "base-guide-left", 0.025, aluminium),
        structure["support-head-right"], spigot("support-base-right", "base-guide-right", 1.375, aluminium),
        *masses(volume), structure["m-slab-cleat"],
    ]


def scale_200(volume):
    body = by_id(volume["body"])
    structure = by_id(volume["structure"])
    aluminium, glass, enamel = body["mullion-tube"]["tone"], body["igu-pane-out"]["tone"], body["spandrel-enamel"]["tone"]

    def bolt(identifier, anchor, start):
        return {"id": identifier, "kind": "revolve", "bone": "frame", "material": "steel", "role": "body", "tone": 0.3, "profile": [[0.0, 0.015], [0.069, 0.015]], "axis": [1, 0, 0], "origin": [start, 3.275, 0.06], "member": "frame", "part": "support", "anchor": anchor}

    volume["thermal"] = thermal(volume)
    volume["body"] = [mullion_bar(aluminium)] + [transom_bar(name, aluminium) for name in TRANSOMS] + [
        panel("igu", "igu", "glass", "glazing", glass, 1.3, 2.35, CAP_BACK, GLASS_BACK),
        panel("spandrel", "spandrel", "spandrel-glass", "body", enamel, 1.3, 0.8, CAP_BACK, SPANDREL_BACK),
    ]
    volume["structure"] = [
        member("member-mullion", "mullion", "frame"),
        *[member(f"member-transom-{name}", f"transom-{name}", "frame") for name in TRANSOMS],
        member("member-igu", "igu", "igu"),
        member("member-spandrel", "spandrel", "spandrel"),
        structure["joint-connector-base"], structure["joint-connector-mid"], structure["joint-connector-head"],
        hook("200"),
        bolt("support-head-left", "head-anchor-left", 0.0), spigot("support-base-left", "base-guide-left", 0.025, aluminium),
        bolt("support-head-right", "head-anchor-right", 1.331), spigot("support-base-right", "base-guide-right", 1.375, aluminium),
        *masses(volume),
    ]


def scale_500(volume):
    body = by_id(volume["body"])
    structure = by_id(volume["structure"])
    aluminium, glass, enamel = body["mullion-tube"]["tone"], body["igu-pane-out"]["tone"], body["spandrel-enamel"]["tone"]
    depth, middle = BLOCK_BACK - FACE, (FACE + BLOCK_BACK) / 2
    volume["thermal"] = thermal(volume)
    volume["body"] = [
        box("mullion-back", "frame", "aluminium", aluminium, [BACK_WIDTH, HEIGHT, MULLION_BACK - BLOCK_BACK], [BACK_WIDTH / 2, HEIGHT / 2, (BLOCK_BACK + MULLION_BACK) / 2], repeat={"count": 2, "translation": [WIDTH - BACK_WIDTH, 0.0, 0.0]}),
        box("vision", "igu", "glass", glass, [WIDTH, 2.45, depth], [0.65, 1.175, middle]),
        box("spandrel", "spandrel", "spandrel-glass", enamel, [WIDTH, 0.85, depth], [0.65, 0.375, middle]),
    ]
    volume["structure"] = [
        member("member-frame", "mullion-back", "frame"),
        member("member-igu", "vision", "igu"),
        member("member-spandrel", "spandrel", "spandrel"),
        hook("500"),
        structure["support-head-left"], structure["support-base-left"], structure["support-head-right"], structure["support-base-right"],
        *masses(volume),
    ]


def main():
    document = json.load(open(SOURCE, encoding="utf-8"))
    volumes = document["volumes"]
    if "mullion-tube" not in by_id(volumes["100"]["body"]):
        raise SystemExit("already applied: the coarse scales no longer carry the 1:50 parts list")
    scale_50(volumes["50"])
    for scale, build in (("100", scale_100), ("200", scale_200), ("500", scale_500)):
        build(volumes[scale])
        volumes[scale]["decisions"] = DECISIONS[scale]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    for scale in ("100", "200", "500"):
        print(scale, {name: len(volumes[scale][name]) for name in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
