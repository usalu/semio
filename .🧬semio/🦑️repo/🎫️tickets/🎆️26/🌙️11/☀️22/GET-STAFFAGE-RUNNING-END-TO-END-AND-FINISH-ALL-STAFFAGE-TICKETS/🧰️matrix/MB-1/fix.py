"""🪡️ The multiview repairs of batch MB-1, one function per asset, each the last step of the asset's chain (after the silhouette scripts of SAM, SC and SF) and idempotent: python3 fix.py <id>… | all

The rule of the redraws: a band draws in its front and plan the parts its own side elevation and volume carry (also those another part covers in the rest pose), so a finer band shows more than a coarser one; nothing is taken over from another view by turning, mirroring or flattening."""
import copy, math, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import mb

GEOMETRY = ("kind", "x", "y", "w", "h", "points", "closed", "cx", "cy", "rx", "ry", "d", "solid", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "rotationDeg", "from", "to", "section", "hatch")


def find(doc, scale, view, test):
    """🔎️ The first drawn shape of a view that passes a test, or None."""
    return next((s for s in mb.shapes(doc, scale, view) if test(s)), None)


def like(template, **over):
    """🧬️ A new shape with the paint of a template (bone, role, tone, region, material) and its own geometry."""
    shape = {k: v for k, v in copy.deepcopy(template).items() if k not in GEOMETRY}
    shape.update(over)
    return shape


def rect(template, sid, x0, y0, x1, y1, **over):
    """▭️ A filled rect between two corners."""
    return like(template, id=sid, filled=True, kind="rect", x=mb.r6(x0), y=mb.r6(y0), w=mb.r6(x1 - x0), h=mb.r6(y1 - y0), **over)


def line(template, sid, points, **over):
    """📏️ An open unfilled polyline."""
    return like(template, id=sid, filled=False, kind="poly", points=[[mb.r6(x), mb.r6(y)] for x, y in points], closed=False, **over)


def disc(template, sid, cx, cy, r, **over):
    """⚪️ A filled circle."""
    return like(template, id=sid, filled=True, kind="ellipse", cx=mb.r6(cx), cy=mb.r6(cy), rx=mb.r6(r), ry=mb.r6(r), **over)


def split_anchor(doc, old, new):
    """⚓️ Replaces one lumped contact by the contacts of its physical places (D45): the anchor list, every list of anchor ids and every list of `{anchor}` records follow; a support solid that repeated once per place becomes one solid per contact at its place."""
    asset = doc["asset"]
    if not any(a["id"] == old for a in asset["anchors"]):
        return
    at = next(n for n, a in enumerate(asset["anchors"]) if a["id"] == old)
    base = asset["anchors"][at]
    asset["anchors"][at:at + 1] = [{**copy.deepcopy(base), **entry} for entry in new]
    ids = [entry["id"] for entry in new]

    def placed(item, k, anchor):
        record = {**copy.deepcopy(item), "anchor": anchor}
        step = record.get("repeat")
        if step is None:
            return record
        if step["count"] != len(ids):
            raise SystemExit(f"{asset['id']}: {item['id']} repeats {step['count']} times for {len(ids)} contacts")
        del record["repeat"]
        record["id"] = record["id"].replace(old, anchor)
        at = record["transform"]["translation"] if "transform" in record else record["origin"]
        at[:] = [mb.r6(v + k * d) for v, d in zip(at, step["translation"])]
        return record

    def walk(node):
        if isinstance(node, dict):
            if old in node:
                raise SystemExit(f"{asset['id']}: {old} is a key, split by hand")
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            n = 0
            while n < len(node):
                item = node[n]
                if item == old:
                    node[n:n + 1] = ids
                    n += len(ids)
                elif isinstance(item, dict) and item.get("anchor") == old:
                    node[n:n + 1] = [placed(item, k, i) for k, i in enumerate(ids)]
                    n += len(ids)
                else:
                    walk(item)
                    n += 1

    walk({k: v for k, v in asset.items() if k != "anchors"})


def bookshelf(doc):
    """📚️ Plan: the back panel in its groove and the recessed plinth at 1:50, the plinth at 1:100 and 1:200 (as the fronts of these bands draw them), the bare block at 1:500."""
    for scale, parts in ((50, ("back", "plinth")), (100, ("plinth",)), (200, ("plinth",))):
        for part in parts:
            front = mb.shape(doc, scale, "front", part)
            mb.put(mb.shapes(doc, scale, "plan"), rect(front, part, 0.018, front["x"], 0.782, front["x"] + front["w"], z=2))


def dining_table(doc):
    """🍽️ Plan: the four legs under the two halves at 1:50, 1:100 and 1:200 and the aprons at 1:50; the three slabs alone at 1:500."""
    for scale in (50, 100, 200):
        plan = mb.shapes(doc, scale, "plan")
        for bone, tag, x0 in (("top-left", "l", 0.105), ("top-right", "r", 1.435)):
            paint = find(doc, scale, "elevation", lambda s: s["bone"] == bone and "leg" in s["id"]) or find(doc, scale, "elevation", lambda s: s["bone"] == bone)
            for side, z0 in (("near", -0.405), ("far", 0.345)):
                mb.put(plan, rect(paint, f"leg-{tag}-{side}", x0, z0, x0 + 0.06, z0 + 0.06, z=2))
            if scale == 50:
                a0, a1, e0 = (0.145, 0.8, 0.11) if tag == "l" else (0.8, 1.455, 1.46)
                for side, z0 in (("near", -0.4), ("far", 0.37)):
                    mb.put(plan, rect(paint, f"apron-{tag}-{side}", a0, z0, a1, z0 + 0.03, z=1.8))
                mb.put(plan, rect(paint, f"apron-{tag}-end", e0, -0.365, e0 + 0.03, 0.365, z=1.8))


def park_bench(doc):
    """🪑️ Plan at 1:200: the front and the rear foot pad of each cast end on its foot strip (the pads of 1:100; 70 mm print at this scale); contacts: one per pad at its real z (D45)."""
    plan = mb.shapes(doc, 200, "plan")
    for end in ("l", "r"):
        strip = mb.shape(doc, 200, "plan", f"{end}-feet")
        for pad in (f"{end}-pads", f"{end}-pads-rear"):
            source = mb.shape(doc, 100, "plan", pad)
            y0 = max(source["y"], strip["y"])
            mb.put(plan, rect(strip, pad, source["x"], y0, source["x"] + source["w"], source["y"] + source["h"], z=2.1), after=f"{end}-feet")
    for side, en, de in (("left", "Left", "Linker"), ("right", "Right", "Rechter")):
        split_anchor(doc, f"contact-{side}", [
            {"id": f"contact-{side}-front", "z": -0.268, "footprintWidthM": 0.07, "title": {"en": f"{en} foot, front pad", "de": f"{de} Fuß, vordere Platte"}},
            {"id": f"contact-{side}-rear", "z": 0.245, "footprintWidthM": 0.07, "title": {"en": f"{en} foot, rear pad", "de": f"{de} Fuß, hintere Platte"}},
        ])
    for text in doc["asset"]["structure"]["assumptions"]:
        text["en"] = text["en"].replace("on two foot contacts", "on four foot-pad contacts")
        text["de"] = text["de"].replace("auf zwei Fußkontakten", "auf vier Fußplattenkontakten")


def single_bed(doc):
    """🛏️ The centre leg in every plan and in the fronts that lacked it, the cross beam it carries in the plan at 1:50; the head posts as two contacts at their real z (D45)."""
    for scale in (50, 100, 200, 500):
        side = mb.shape(doc, scale, "elevation", "leg-mid")
        mb.put(mb.shapes(doc, scale, "plan"), rect(side, "leg-mid", side["x"], -0.03, side["x"] + side["w"], 0.03, z=1.4))
        if find(doc, scale, "front", lambda s: s["id"] == "leg-mid") is None:
            mb.put(mb.shapes(doc, scale, "front"), rect(side, "leg-mid", -0.03, side["y"], 0.03, side["y"] + side["h"], z=1.4))
    side = mb.shape(doc, 50, "elevation", "leg-mid")
    mb.put(mb.shapes(doc, 50, "plan"), rect(side, "cross-beam", 1.04, -0.455, 1.08, 0.455, z=1.45))
    split_anchor(doc, "foot-head", [
        {"id": "foot-head-l", "z": 0.475, "footprintWidthM": 0.06, "title": {"en": "Head post, left", "de": "Kopfpfosten links"}},
        {"id": "foot-head-r", "z": -0.475, "footprintWidthM": 0.06, "title": {"en": "Head post, right", "de": "Kopfpfosten rechts"}},
    ])


def duct_fan(doc):
    """🌀️ The rotor seen through the housing: shaft and one blade in the front and the plan at 1:50, the blade in the plan at 1:100; the blade that crosses the horizontal axis plane (plan) lies further out than the one that crosses the vertical one (front), so the two views of the square plate and round ring are no quarter turn of each other."""
    shaft_front, blade_front, shaft_plan = (mb.shape(doc, 100, v, i) for v, i in (("front", "shaft-section"), ("front", "blade-section"), ("plan", "shaft-section")))
    lo, hi = min(p[1] for p in blade_front["points"]), max(p[1] for p in blade_front["points"])
    blade_plan = like(blade_front, kind="poly", closed=True, points=[[mb.r6(0.0332 + (r - lo) / (hi - lo) * 0.0437), d] for d, r in blade_front["points"]])
    mb.put(mb.shapes(doc, 100, "plan"), copy.deepcopy(blade_plan))
    for view, parts in (("front", (shaft_front, blade_front)), ("plan", (shaft_plan, blade_plan))):
        for part in parts:
            mb.put(mb.shapes(doc, 50, view), copy.deepcopy(part))


def kitchen_extractor_hood(doc):
    """🍳️ Plan at 1:50: the spindle of the backdraught damper seen down the open duct. The section of the side elevation is named A-A in both languages: its label was a sentence that differs per language, so the cut-line marker of the plan had other geometry in German (`language-geometry`)."""
    paint = mb.shape(doc, 50, "plan", "chimney-lower")
    mb.put(mb.shapes(doc, 50, "plan"), line(paint, "damper-spindle", [(0.012, 0), (0.158, 0)], role="detail", z=3.7))
    for scale in (50, 100):
        doc["asset"]["representations"][str(scale)]["sections"]["elevation"]["label"] = {"en": "A-A", "de": "A-A"}


def used_crt_television(doc):
    """📺️ Plan: the round swivel stand under the cabinet in every band (it stands 20 mm proud of the bezel, which the side elevation shows and the plan did not), and the picture tube behind the bezel at 1:200."""
    for scale in (50, 100, 200, 500):
        side = find(doc, scale, "elevation", lambda s: s["bone"] == "base")
        mb.put(mb.shapes(doc, scale, "plan"), disc(side, side["id"], 0, 0, 0.17, z=2), after="blanket")
    tube = mb.shape(doc, 200, "front", "screen")
    mb.put(mb.shapes(doc, 200, "plan"), rect(tube, "screen", 0.035, -0.28, 0.138, 0.28, z=3.5))


def hot_water_cylinder(doc):
    """🛢️ Plan at 1:50, 1:100 and 1:200: the base ring the cylinder stands on, under the jacket; the cap with its stubs alone at 1:500."""
    for scale in (50, 100, 200):
        side = find(doc, scale, "elevation", lambda s: s["id"] in ("base-ring", "feet"))
        mb.drop(mb.shapes(doc, scale, "plan"), "feet" if side["id"] != "feet" else "")
        mb.put(mb.shapes(doc, scale, "plan"), disc(side, side["id"], 0, 0, side["w"] / 2, z=2.5))


def curtain_wall_bay(doc):
    """🪟️ At 1:100 the plan section cuts the mullions with their glazing rebates and the glass runs 15 mm into them (the profile of the 1:100 volume); the front section shows the slab cleat and the head of the hanging bolt beyond the cut."""
    notch = [[0, -0.1], [0.05, -0.1], [0.05, -0.084], [0.035, -0.084], [0.035, -0.056], [0.05, -0.056], [0.05, 0.1], [0, 0.1], [0, -0.056], [0.015, -0.056], [0.015, -0.084], [0, -0.084]]
    for sid, dx in (("mullion-left", 0), ("mullion-right", 1.35)):
        mb.shape(doc, 100, "plan", sid)["points"] = [[mb.r6(x + dx), z] for x, z in notch]
    glass = mb.shape(doc, 100, "plan", "glass")
    glass["x"], glass["w"] = 0.035, 1.33
    beyond = mb.shape(doc, 100, "front", "mullion-side")
    mb.put(mb.shapes(doc, 100, "front"), rect(beyond, "slab-cleat", 0.02, 3.19, 0.1, 3.3, role="detail", z=1.6))
    mb.put(mb.shapes(doc, 100, "front"), disc(beyond, "hanging-bolt", 0.06, 3.275, 0.0125, role="detail", z=1.7))


def wall(doc):
    """🧱️ Side elevation (the section through the wall): the bed joints of the twelve courses at 1:100, the six bond bands and the joint between the two insulation courses at 1:200, as the volumes of these bands build the wall; the plan of the top of the wall keeps its plain layers and is no longer the same drawing."""
    joint = mb.shape(doc, 50, "elevation", "joint-1")
    for scale, count in ((100, 12), (200, 6)):
        brick = mb.shape(doc, scale, "elevation", "brick")
        for n in range(1, count):
            mb.put(mb.shapes(doc, scale, "elevation"), line(joint, f"joint-{n}", [(brick["x"], n / count), (brick["x"] + brick["w"], n / count)]))
    board = find(doc, 200, "elevation", lambda s: s["id"] == "insulation")
    if board is not None:
        mb.put(mb.shapes(doc, 200, "elevation"), line(joint, "insulation-joint", [(board["x"], 0.4), (board["x"] + board["w"], 0.4)]))


def external_venetian_blind(doc):
    """🪟️ Plan: the two guide rails under the box in the three coarse bands, the window under the wall head with its glass (its frame is drawn wider at 1:500), and the sill board at 1:100."""
    for scale in (100, 200, 500):
        plan = mb.shapes(doc, scale, "plan")
        guide = mb.shape(doc, scale, "elevation", "guide")
        mb.put(plan, rect(guide, "guide-left", 0.075, 0.57, 0.095, 0.6, z=2))
        mb.put(plan, rect(guide, "guide-right", 0.075, -0.6, 0.095, -0.57, z=2))
        frame = find(doc, scale, "elevation", lambda s: s["id"] in ("window", "frame-jamb"))
        mb.put(plan, rect(frame, frame["id"], -0.227, -0.6, -0.157, 0.6, z=0.5))
        pane = mb.shape(doc, scale, "elevation", "glass")
        half = 0.525 if scale == 500 else 0.55
        mb.put(plan, line(pane, "glass", [(-0.188, -half), (-0.188, half)], z=0.6))
    board = mb.shape(doc, 100, "elevation", "sill-board")
    mb.put(mb.shapes(doc, 100, "plan"), rect(board, "sill-board", -0.157, -0.6, 0.03, 0.6, z=0.4))


def glulam_beam(doc):
    """🪵️ Plan: the steel bearing plates under the beam ends at 1:50, 1:100 and 1:200, the beam axis at 1:200 as before, the two finger joints of the top lamella at 1:50 and 1:100 and its grain at 1:50."""
    for scale in (50, 100, 200):
        for side in ("left", "right"):
            plate = find(doc, scale, "elevation", lambda s: s["id"] in (f"bearing-plate-{side}", f"bearing-{side}"))
            mb.put(mb.shapes(doc, scale, "plan"), rect(plate, plate["id"], plate["x"], -0.08, plate["x"] + plate["w"], 0.08, z=0.5))
    mb.drop(mb.shapes(doc, 500, "plan"), "beam-axis")
    for sid in ("finger-joint-top-0", "finger-joint-top-1"):
        mb.put(mb.shapes(doc, 100, "plan"), copy.deepcopy(mb.shape(doc, 50, "plan", sid)))
    grain = mb.shape(doc, 50, "elevation", "glue-line-01")
    for n, (x0, x1, z) in enumerate(((0.4, 2.45, -0.03), (2.75, 5.7, 0.035), (6.05, 7.7, -0.01))):
        mb.put(mb.shapes(doc, 50, "plan"), line(grain, f"grain-top-{n}", [(x0, z), (x1, z)], z=1.55))


def reclaimed_steel_column(doc):
    """🏗️ Plan at 1:50: the web between the two flanges, the solid the front already places."""
    mb.put(mb.shapes(doc, 50, "plan"), copy.deepcopy(mb.shape(doc, 50, "front", "web")), before="flange-top")


def steel_column(doc):
    """🏛️ Plan at 1:500: one block, as the side and the front of this band draw the column, without the cap plate."""
    mb.drop(mb.shapes(doc, 500, "plan"), "cap-plate")


def stair_flight(doc):
    """🪜️ Plan at 1:50: the base plate and the head plate of the stringer, turned out of the stringer frame into the plan (lateral 260 mm)."""
    bone = next(b for b in doc["asset"]["rig"]["bones"] if b["id"] == "stringer")
    turn = math.radians(bone["angleDeg"])
    for sid in ("base-plate", "head-plate"):
        side = mb.shape(doc, 50, "elevation", sid)
        xs = [bone["x"] + x * math.cos(turn) - y * math.sin(turn) for x, y in side["points"]]
        mb.put(mb.shapes(doc, 50, "plan"), rect(side, sid, min(xs), -0.13, max(xs), 0.13, z=1.5))


def used_trench_convector(doc):
    """♨️ Front (end view): the levelling brackets beside the trough and the end of the fin pack at 1:100, the fin pack and the pipe tails at 1:200; the bare trough at 1:500."""
    for sid in ("levelling-bracket-0", "levelling-bracket-1"):
        mb.put(mb.shapes(doc, 100, "front"), copy.deepcopy(mb.shape(doc, 50, "front", sid)), before="trough-body")
    fins = mb.shape(doc, 100, "elevation", "fin-pack")
    mb.put(mb.shapes(doc, 100, "front"), rect(fins, "fin-pack", -0.11, 0.1, 0.03, 0.16, z=6.95), before="trough-body")
    fins = mb.shape(doc, 200, "elevation", "fin-pack")
    mb.put(mb.shapes(doc, 200, "front"), rect(fins, "fin-pack", -0.11, 0.08125, 0.03, 0.16, z=6.95), before="trough-body")
    tails = mb.shape(doc, 200, "elevation", "tails")
    mb.put(mb.shapes(doc, 200, "front"), rect(tails, "tails", -0.09, 0.115, 0.01, 0.145, z=6.96), before="trough-body")


def spiral_stair(doc):
    """🌀️ Plan at 1:100: the inner edge of the stiffening flange under the landing tread, which only the 1:100 volume carries."""
    flange = mb.solid(doc, 100, "tread-15-flange")
    tread = mb.shape(doc, 100, "plan", "tread-15")
    ring = like(tread, id="tread-15-flange", role="detail", filled=False, kind="poly", closed=True, points=[[mb.r6(x), mb.r6(z)] for z, x in flange["holes"][0]], z=mb.r6(tread["z"] - 0.005))
    mb.put(mb.shapes(doc, 100, "plan"), ring, before="tread-15")


REPAIRS = {
    "bookshelf": bookshelf, "dining-table": dining_table, "park-bench": park_bench, "single-bed": single_bed, "duct-fan": duct_fan,
    "kitchen-extractor-hood": kitchen_extractor_hood, "used-crt-television": used_crt_television, "hot-water-cylinder": hot_water_cylinder,
    "curtain-wall-bay": curtain_wall_bay, "etics-brick-wall-12": wall, "etics-brick-wall-30": wall, "solid-brick-wall-1960": wall,
    "external-venetian-blind": external_venetian_blind, "glulam-beam": glulam_beam, "reclaimed-steel-column": reclaimed_steel_column,
    "steel-column": steel_column, "stair-flight": stair_flight, "spiral-stair": spiral_stair, "used-trench-convector": used_trench_convector,
}

if __name__ == "__main__":
    names = list(REPAIRS) if sys.argv[1:] == ["all"] else sys.argv[1:]
    for name in names:
        text, doc = mb.load(name)
        REPAIRS[name](doc)
        mb.save(name, text, doc)
