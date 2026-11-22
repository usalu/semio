"""🌲 Scots pine, second round pass 1: the two small limbs (north, south) that carry the depth-facing needle plates at 1:500 in body, thermal and structure, their 2D strokes at 1:500, and the body decisions of 1:100, 1:200 and 1:500.

Usage: python3 rework-scots-pine-coarse-limbs.py [--catalog] [--plan-order]
Without --catalog only the source `volumes/vegetation/scots-pine.json` is written; with it the 1:500 2D shapes of the catalog asset are written too (file re-read right before the write).
"""
import json
import math
import re
import sys

SOURCE = "volumes/vegetation/scots-pine.json"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/scots-pine/🔣️.json"

BODY_RADII = {"limb-n": [0.062, 0.052, 0.046], "limb-s": [0.060, 0.052, 0.046]}
BARK_M = 0.004
RING = 8

DECISIONS = {
    "100": {
        "en": "Body at 1:100 (smallest feature 15 mm): the root flare as one mass without the single surface roots, the lower bole in plated bark and the orange upper bole with their whorl swellings, the two dead stubs and one snag, six crooked main limbs and the two small limbs to the north and south at their true taper, under every plate the continuing branch with two side branches, and the nine needle plates as flat clouds of fifteen lobes (twelve at the top, ten on each small limb); the knots, the forked stub tip, two snags and four of the six side branches under every plate are below the feature and left out.",
        "de": "Körper bei 1:100 (kleinstes Merkmal 15 mm): der Wurzelanlauf als eine Masse ohne die einzelnen Wurzelanläufe, der untere Schaft in Plattenborke und der orange obere Schaft mit ihren Quirlverdickungen, die zwei toten Aststummel und ein Totast, sechs krumme Starkäste und die zwei kleinen Äste nach Norden und Süden mit ihrer wahren Verjüngung, unter jeder Platte der weiterlaufende Ast mit zwei Seitenästen und die neun Nadelplatten als flache Wolken aus fünfzehn Lappen (zwölf am Gipfel, zehn an jedem kleinen Ast); die Astnarben, die gegabelte Stummelspitze, zwei Totäste und vier der sechs Seitenäste unter jeder Platte liegen unter dem Merkmal und entfallen.",
    },
    "200": {
        "en": "Body at 1:200 (smallest feature 30 mm): the bole in three tapered parts with the root flare, the two stubs as slender rods, six main limbs and the two small limbs as tubes of three sections that run on into their plates, the leader with the branch under the top plate, and the nine needle plates with the same fifteen, twelve and ten lobes as at 1:100, each lobe (1.2 to 2.5 m across, 6 to 12 mm on paper) with fewer faces; the side branches under the plates, the snags and the whorl swellings are below the feature and left out.",
        "de": "Körper bei 1:200 (kleinstes Merkmal 30 mm): der Schaft in drei verjüngten Teilen mit dem Wurzelanlauf, die zwei Stummel als schlanke Stäbe, sechs Starkäste und die zwei kleinen Äste als Rohre aus drei Querschnitten, die in ihre Platten hineinlaufen, der Gipfeltrieb mit dem Ast unter der Gipfelplatte und die neun Nadelplatten mit denselben fünfzehn, zwölf und zehn Lappen wie bei 1:100, jeder Lappen (1,2 bis 2,5 m breit, 6 bis 12 mm auf dem Papier) mit weniger Flächen; die Seitenäste unter den Platten, die Totäste und die Quirlverdickungen liegen unter dem Merkmal und entfallen.",
    },
    "500": {
        "en": "Body at 1:500 (smallest feature 75 mm): the bole in three parts, the lowest with the root flare drawn into it, the two stubs as slender rods, six main limbs that run through to the centres of their plates, the two small limbs to the north and south thickened at their tips from 44 mm to 92 mm so that they stay above the feature and carry the two depth-facing plates, the leader and the branch under the top plate, and the nine needle plates as lobed clouds (each lobe 2.4 to 5 mm on paper) with the fewest faces; nothing floats: every plate sits on its limb.",
        "de": "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Schaft in drei Teilen, der unterste mit eingezogenem Wurzelanlauf, die zwei Stummel als schlanke Stäbe, sechs Starkäste, die bis in die Mitte ihrer Platten durchlaufen, die zwei kleinen Äste nach Norden und Süden, an der Spitze von 44 mm auf 92 mm verstärkt, damit sie über dem Merkmal bleiben und die zwei in die Tiefe gerichteten Platten tragen, der Gipfeltrieb und der Ast unter der Gipfelplatte sowie die neun Nadelplatten als gelappte Wolken (jeder Lappen 2,4 bis 5 mm auf dem Papier) mit den wenigsten Flächen; nichts schwebt: jede Platte sitzt auf ihrem Ast.",
    },
}
STRUCTURE_500 = {
    "en": "Structure at 1:500: the wood of the three bole parts, the leader, the six main limbs with the branches under their plates and the two small limbs to the north and south, all joined at the bole, the two stubs as slender members, the round root plate as the fixed support and the four bole masses.",
    "de": "Tragwerk bei 1:500: das Holz der drei Schaftteile, des Gipfeltriebs, der sechs Starkäste mit den Ästen unter ihren Platten und der zwei kleinen Äste nach Norden und Süden, alle am Schaft angeschlossen, die zwei Stummel als schlanke Stäbe, der runde Wurzelteller als Einspannung und die vier Schaftmassen.",
}


def resection(solid, radii):
    """📐 The tube of the finer scale with every ring set to a hand-chosen radius about its own centre."""
    vertices = [list(vertex) for vertex in solid["vertices"]]
    for index, radius in enumerate(radii):
        ring = vertices[index * RING:(index + 1) * RING]
        centre = [sum(vertex[axis] for vertex in ring) / RING for axis in range(3)]
        mean = sum(math.dist(vertex, centre) for vertex in ring) / RING
        for vertex in ring:
            for axis in range(3):
                vertex[axis] = round(centre[axis] + (vertex[axis] - centre[axis]) * radius / mean, 5)
    return vertices


def main():
    document = json.load(open(SOURCE))
    volumes = document["volumes"]
    fine, coarse = volumes["200"], volumes["500"]
    for part in ("body", "thermal", "structure"):
        coarse[part] = [solid for solid in coarse[part] if solid["id"] not in ("limb-n", "limb-s", "th-limb-n", "th-limb-s", "member-limb-n", "member-limb-s")]
    for name, radii in BODY_RADII.items():
        source = next(solid for solid in fine["body"] if solid["id"] == name)
        assert len(source["vertices"]) == 3 * RING + 2
        body = {**source, "vertices": resection(source, radii)}
        at = next(index for index, solid in enumerate(coarse["body"]) if solid["id"] == "trunk-3") + 1
        coarse["body"].insert(at, body)
        member_source = next(solid for solid in fine["structure"] if solid["id"] == "member-" + name)
        member = {**member_source, "vertices": resection(member_source, [radius - BARK_M for radius in radii])}
        at = next(index for index, solid in enumerate(coarse["structure"]) if solid["id"] == "member-trunk-3") + 1
        coarse["structure"].insert(at, member)
        at = next(index for index, solid in enumerate(coarse["thermal"]) if solid.get("of") == "trunk-3") + 1
        coarse["thermal"].insert(at, {"id": "th-" + name, "of": name, "region": "limb-bark"})
    for scale, text in DECISIONS.items():
        volumes[scale]["decisions"][0] = text
    coarse["decisions"][2] = STRUCTURE_500
    open(SOURCE, "w").write(json.dumps(document, ensure_ascii=False, separators=(",", ":")))
    print("source written:", {scale: {part: len(volumes[scale][part]) for part in ("body", "thermal", "structure")} for scale in volumes})
    if "--catalog" in sys.argv:
        catalog()


def block(text, view, name):
    """🔎 The text span of the shape object `name` (written compactly, `id` first) in the view array that starts at `view`."""
    start = text.index('{"id":"' + name + '"', view)
    depth, index = 0, start
    while True:
        depth += {"{": 1, "}": -1}.get(text[index], 0)
        index += 1
        if depth == 0:
            return start, index


def view_start(text, scale, view):
    """🔎 Where the array of one drawing of one scale starts in the compact `representations` member of the asset text."""
    representations = text.index('\n  "representations": {')
    return text.index('"' + view + '":[', text.index('"' + scale + '":{"status"', representations))


def catalog():
    """✏️ The two small limbs as filled strokes in the three 1:500 drawings, directly below the upper bole in the list, as text splices so that every other byte of the asset file stays."""
    text = open(CATALOG).read()
    before = len(text)
    for view in ("elevation", "front", "plan"):
        coarse = view_start(text, "500", view)
        if '{"id":"limb-n"' in text[coarse:text.index("]", coarse)]:
            continue
        shapes = []
        for name in ("limb-n", "limb-s"):
            start, stop = block(text, view_start(text, "200", view), name)
            shapes.append(text[start:stop])
        at, _ = block(text, coarse, "bole-top")
        text = text[:at] + "".join(shape + "," for shape in shapes) + text[at:]
    json.loads(text)
    open(CATALOG, "w").write(text)
    print("catalog 2D written", before, "->", len(text))




def plan_order():
    """✏️ 1:500 plan: the limb to the top-east plate lies under the needle clouds, as the model shows from above (it was drawn over them)."""
    text = open(CATALOG).read()
    plan = view_start(text, "500", "plan")
    old, new = '{"role":"body","z":3.0843,', '{"role":"body","z":2.999,'
    at = text.find(old, plan)
    if at < 0 or '"id":"limb-e2"' not in text[at:at + 200]:
        print("plan order already set")
        return
    open(CATALOG, "w").write(text[:at] + new + text[at + len(old):])
    print("1:500 plan: limb-e2 under the needle clouds")


if __name__ == "__main__":
    main()
    if "--plan-order" in sys.argv:
        plan_order()
