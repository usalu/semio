"""🪟 Round 5 glass balustrade: the round-3 volumes (`r3-glass-balustrade-volumes.py`) with the hollow stand-ins restored now that the area measure floods closed cavities (C31): the handrail region is the real slotted tube with its U gasket and closed dished caps at 1:50 and the slotted tube with its end plugs at 1:100, instead of a solid round skin.

Writes `volumes/components/glass-balustrade.json`.

Usage: python3 r5-glass-balustrade-volumes.py

@see ./r5-brief.md
@see ./r3-components-glass-balustrade.md
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3vol", os.path.join(HERE, "r3-glass-balustrade-volumes.py"))
V = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(V)
k, ref, solid, text = V.k, V.ref, V.solid, V.text
SOURCE = os.path.join(HERE, "volumes", "components", "glass-balustrade.json")


def cap(id, side):
    """🧢 A dished end cap closing the handrail tube: 3 mm rim, 5 mm crown, solid to the axis so the tube is sealed."""
    return solid(
        id,
        "revolve",
        "steel",
        V.RAIL,
        role="detail",
        profile=[[0.0, 0.0], [0.0, k.RAIL_R], [0.003, k.RAIL_R], [0.0055, 0.0185], [0.0072, 0.0125], [k.CAP, 0.005], [k.CAP, 0.0]],
        axis=[0, 0, side],
        origin=[k.RAIL_U, 0, side * k.RAIL_HALF_LENGTH],
    )


def thermal50():
    """🌡️ 1:50: every body surface in its region; the handrail region is the slotted tube, its U gasket and the two caps, which together with the glass seal the tube's air."""
    clamp = ["base-plate", "cheek-fixed", "cheek-clamp", "weld-outer", "weld-inner", "pad-fixed", "pad-clamp", "setting-block", "bolt-sleeve", "bolt-shank", "bolt-head", "bolt-nut", "anchor-nut-fixed", "anchor-nut-clamp", "anchor-stud-fixed", "anchor-stud-clamp"]
    glass = ["pane-fixed", "interlayer", "pane-clamp"]
    rail = ["handrail-tube", "handrail-gasket", "handrail-cap-left", "handrail-cap-right"]
    return [ref(f"th-{s}", s, region="clamp-steel") for s in clamp] + [ref(f"th-{s}", s, region="panel-glass") for s in glass] + [ref(f"th-{s}", s, region="handrail-steel") for s in rail]


def thermal100():
    """🌡️ 1:100: the merged body surfaces in their regions; the handrail region is the slotted tube and its two end plugs, sealed by the glass in the slot."""
    clamp = ["base-plate", "clamp-bar-fixed", "clamp-bar-clamp", "clamp-bolt", "anchor-fixed", "anchor-clamp"]
    rail = ["handrail-tube", "handrail-plug-left", "handrail-plug-right"]
    return [ref(f"th-{s}", s, region="clamp-steel") for s in clamp] + [ref("th-glass", "glass", region="panel-glass")] + [ref(f"th-{s}", s, region="handrail-steel") for s in rail]


THERMAL50 = text(
    "Thermal: every region is its body surfaces: the clamp region is base plate, clamp plates, welds, elastomer strips, setting blocks, sleeves, bolts and anchors (the strips and sleeves between glass and steel show in the cut), the glass region is both panes with the film, the handrail region is the slotted tube itself with its U gasket and the two closed dished caps: tube, gasket, caps and glass seal the air inside the tube, so only the outer skin, the caps and the gasket faces count.",
    "Thermisch: jede Region besteht aus ihren Körperflächen: die Klemmregion aus Fußplatte, Klemmblechen, Nähten, Elastomerstreifen, Klotzungen, Hülsen, Schrauben und Ankern (Streifen und Hülsen zwischen Glas und Stahl zeigen sich im Schnitt), die Glasregion aus beiden Scheiben mit der Folie, die Handlaufregion ist das geschlitzte Rohr selbst mit seiner U-Dichtung und den beiden geschlossenen gewölbten Kappen: Rohr, Dichtung, Kappen und Glas schließen die Luft im Rohr ein, daher zählen nur die Außenhaut, die Kappen und die Flächen der Dichtung.",
)
THERMAL100 = text(
    "Thermal: the merged body surfaces in their three regions; the handrail region is the slotted tube with its two end plugs, closed by the glass in the slot, so its inner bore is no outer surface. Structure: the glass body and the handrail tube are the members, the two clamp bars with the seven bolts the fixed joint, the base plate with its ten anchor studs the support, the mass disc sits in the glass at 0.55 m.",
    "Thermisch: die zusammengefassten Körperflächen in ihren drei Regionen; die Handlaufregion ist das geschlitzte Rohr mit seinen beiden Endstopfen, vom Glas im Schlitz geschlossen, so dass seine Bohrung keine Außenfläche ist. Tragwerk: Glaskörper und Handlaufrohr sind die Stäbe, die beiden Klemmleisten mit den sieben Schrauben die Einspannung, die Fußplatte mit ihren zehn Ankerbolzen das Auflager, die Massenscheibe sitzt im Glas bei 0,55 m.",
)


def build():
    """🧮 All four scales with the round-5 changes."""
    V.cap = cap
    V.thermal50 = thermal50
    V.thermal100 = thermal100
    out = V.build()
    vol = out.get("volumes", out)
    for key, new in (("50", THERMAL50), ("100", THERMAL100)):
        dec = vol[key]["decisions"]
        i = next(n for n, d in enumerate(dec) if d["en"].startswith("Thermal:"))
        dec[i] = new
    import runpy
    runpy.run_path(os.path.join(HERE,"equipment-match.py"))["rail_skin"](vol)
    return {"volumes": vol}


if __name__ == "__main__":
    v = build()
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump(v, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print("written", SOURCE, {s: {l: len(v["volumes"][s].get(l, [])) for l in ("body", "thermal", "structure")} for s in v["volumes"]})
