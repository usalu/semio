"""🌬️ Round 5 MVHR unit: keeps the round-3 hand-stated lists of 1:50, 1:100 and 1:200 (`r3-mvhr-unit-volumes.py`, re-read and graded in round 5), gives the 1:200 extract fan its real drum (it was a 144 mm cube) and authors the digital 1:500 anew with its own decisions (the inherited 1:500 had one generic sentence and a 2 mm hollow glass and 0.7 mm spigot walls copied from 1:200); writes `volumes/appliances/mvhr-unit.json`.

Usage: python3 r5-mvhr-unit-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "appliances", "mvhr-unit.json")
TEXTS = os.path.join(HERE, "r5-mvhr-unit-texts.json")


def _module(name, file):
    """📦 Imports a sibling script as a module."""
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


V = _module("r3mvhr", "r3-mvhr-unit-volumes.py")
g = V.g
DRUM = [(0, 0), (0, 0.072), (0.105, 0.072), (0.105, 0)]


def thermal200():
    """🌡️ 1:200 thermal: the round-3 list with the extract fan as its 144 mm scroll drum on the casing axis, as at 1:100 and 1:500."""
    return [dict(V.rev("th-extract-fan", "casing", "pp-impeller", 0.5, DRUM, V.ZAX, [0.5, 0.165, 0]), region="extract-fan") if s["id"] == "th-extract-fan" else s for s in V.thermal200()]


def body500():
    """🎨 1:500 body (75 mm feature): the casing with the cover as one crisp 0.6 x 0.6 x 0.3 m block from the cover face to the wall (bends, cover joint, bezel, light and grip are far below the feature), the 254 mm window flush in its face, the impeller disc behind it, and the four DN100 spigots as solid 90 mm drums (their 0.7 mm wall is below the feature; the cavity is shown by the outlet patches only)."""
    return [
        V.box("casing", "casing", "painted-steel", 0.82, [0.6, 0.6, 0.298], [0.3, 0.0, 0.001]),
        V.rev("window", "casing", "polycarbonate", 0.3, [(-0.151, 0), (-0.151, 0.127), (-0.147, 0.127), (-0.147, 0)], V.ZAX, V.WIN_ORIGIN, role="glazing"),
        V.rev("impeller", "impeller", "pp-impeller", 0.8, [(-0.0165, 0), (-0.0165, 0.095), (0.0135, 0.095), (0.0135, 0)], V.ZAX, [0, 0, 0]),
        V.rev("spigot-a", "spigots", "galvanised-steel", 0.66, [(-0.003, 0), (-0.003, 0.045), (0.062, 0.045), (0.062, 0)], V.UAX, V.SPIG_A, repeat=V.ROW),
        V.rev("spigot-b", "spigots", "galvanised-steel", 0.66, [(-0.003, 0), (-0.003, 0.045), (0.062, 0.045), (0.062, 0)], V.UAX, V.SPIG_B, repeat=V.ROW),
    ]


def thermal500():
    """🌡️ 1:500 thermal: the casing block, the window (front face only), the impeller, the four spigots as closed duct drums (both ends are duct sections), the hidden plate pack, the extract-fan drum and the filter slab inside, and the supply and exhaust outlet patches flush in the spigot tops."""
    return [
        V.ref("th-casing", "casing", region="casing"),
        dict(V.rev("th-window", "casing", "polycarbonate", 0.3, [(-0.1515, 0), (-0.1515, 0.112), (-0.1495, 0.112), (-0.1495, 0)], V.ZAX, V.WIN_ORIGIN, role="glazing", sectionFaces=["+w"]), region="window"),
        V.ref("th-impeller", "impeller", region="impeller"),
        V.ref("th-duct-a", "spigot-a", region="ducts", sectionFaces=["-u", "+u"]),
        V.ref("th-duct-b", "spigot-b", region="ducts", sectionFaces=["-u", "+u"]),
        {k: v for k, v in V.core(-0.08, 0.11).items() if k != "layer"},
        dict(V.rev("th-extract-fan", "casing", "pp-impeller", 0.5, DRUM, V.ZAX, [0.5, 0.165, 0]), region="extract-fan"),
        dict(V.box("th-filters", "casing", "filter-media", 0.9, [0.035, 0.445, 0.12], [0.4725, -0.0525, -0.065]), region="filters"),
    ] + V.emitters(0.0565)


def structure500():
    """🏗️ 1:500 structure: the steel wrapper as one sheet ring and the back tray as one plate (the load path to the wall), the four spigots as their tubes on 30 mm collars, the impeller on its shaft, the hook and rest at their anchors, and the mass at the centre of the casing."""
    tube = V.loop([(0, 0.0443), (0.062, 0.0443), (0.062, 0.045), (0, 0.045)])
    return [
        dict(V.ext("member-wrapper", "casing", "painted-steel", 0.88, V.RR_OUT, g.Z_COVER, g.Z_WALL, holes=[V.RR_SHEET[::-1]]), member="casing", part="member"),
        dict(V.box("member-back", "casing", "painted-steel", 0.88, [0.565, 0.57, 0.003], [0.3, 0.0, 0.146]), member="casing", part="member"),
        dict(V.rev("member-spigot-a", "spigots", "galvanised-steel", 0.66, tube, V.UAX, V.SPIG_A, repeat=V.ROW), member="spigots", part="member"),
        dict(V.rev("member-spigot-b", "spigots", "galvanised-steel", 0.66, tube, V.UAX, V.SPIG_B, repeat=V.ROW), member="spigots", part="member"),
        V.ref("member-impeller", "impeller", member="impeller", part="member"),
        dict(V.rev("joint-impeller-shaft", "impeller", "painted-steel", 0.4, [(-0.010, 0), (-0.010, 0.015), (0.020, 0.015), (0.020, 0)], V.ZAX, [0, 0, 0], role="detail"), member="impeller", part="joint"),
        V.collar("joint-spigot-a", V.SPIG_A, 0.03),
        V.collar("joint-spigot-b", V.SPIG_B, 0.03),
        dict(V.box("support-wall-hook", "casing", "galvanised-steel", 0.6, [0.03, 0.3, 0.03], [0.585, 0, 0.1345]), member="casing", part="support", anchor="wall-hook"),
        dict(V.box("support-wall-rest", "casing", "epdm-seal", 0.25, [0.03, 0.3, 0.03], [0.03, 0, 0.1345]), member="casing", part="support", anchor="wall-rest"),
        V.ball("mass-casing", "casing", 0.04, [0.3, 0, 0], "casing"),
    ]


def main():
    """💾 Writes the volumes source."""
    texts = json.load(open(TEXTS, encoding="utf-8"))
    vols = V.volumes()
    vols["200"]["thermal"] = V.clean(thermal200())
    vols["200"]["decisions"] = texts["200"]
    vols["500"] = {"status": "authored", "denominator": 500, "body": V.clean(body500()), "thermal": V.clean(thermal500()), "structure": V.clean(structure500()), "decisions": texts["500"]}
    for s, v in vols.items():
        for lst in ("body", "thermal", "structure"):
            ids = [x["id"] for x in v[lst]]
            if len(ids) != len(set(ids)):
                raise SystemExit(f"duplicate ids in {s} {lst}")
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", SOURCE, {s: {k: len(v[k]) for k in ("body", "thermal", "structure")} for s, v in vols.items()})


if __name__ == "__main__":
    main()
