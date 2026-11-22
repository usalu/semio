"""✒️ Hand-placed detail and texture strokes of the three views per scale (seams, grain, slots, bolt holes, soot, labels)."""
import importlib.util
import pathlib

_g = importlib.util.spec_from_file_location("geo", pathlib.Path(__file__).with_name("author-geometry.py"))
G = importlib.util.module_from_spec(_g)
_g.loader.exec_module(G)
r6 = G.r6
XB, YB = G.XB, G.YB


def line(id, bone, pts, role="detail", z=8.0, tone=0.25, region=None, closed=False):
    """➖ An unfilled stroke."""
    d = {"id": id, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed}
    if region:
        d["region"] = region
    return d


def hatches(scale):
    """▦ Hatches of the representation (none: timber grain and soot are strokes)."""
    return []


def details(scale):
    """🖋️ Detail strokes for elevation, front and plan of one scale."""
    el, fr, pl = [], [], []
    if scale == 200:
        el.append(line("crate-rim", "crate", [[0, G.WALL_TOP - 0.03], [G.CRATE_L, G.WALL_TOP - 0.03]], region="crate"))
        pl.append(line("crate-inner-edge", "crate", [[0.03, -0.18], [0.59, -0.18], [0.59, 0.18], [0.03, 0.18]], region="crate", closed=True))
        return el, fr, pl
    grain = [0.075, 0.105, 0.145] if scale == 50 else [0.11]
    for i, y in enumerate(grain):
        el.append(line(f"wall-grain-{i + 1}", "crate", [[0.03, y], [0.25, y + 0.004], [0.42, y - 0.003], [0.59, y]], role="texture", tone=0.35, region="crate"))
    for x in ((0.009, 0.611) if scale == 50 else ()):
        for y in (0.07, 0.11, 0.15):
            el.append(line(f"nail-{int(x * 1000)}-{int(y * 1000)}", "crate", [[x - 0.004, y], [x + 0.004, y]], role="texture", tone=0.15, region="crate"))
    el.append(line("wall-end-joint-rear", "crate", [[0.018, G.SKID_H], [0.018, G.WALL_TOP]], region="crate"))
    el.append(line("wall-end-joint-front", "crate", [[0.602, G.SKID_H], [0.602, G.WALL_TOP]], region="crate"))
    el.append(line("housing-seam", "burner", [[-0.06, -0.13], [-0.06, 0.11]], region="housing"))
    el.append(line("scroll-edge", "burner", G.arc(-0.2, -0.04, 0.06, 200, 330, 6), role="texture", region="housing", tone=0.4))
    if scale == 50:
        el.append(line("type-plate-text-1", "burner", [[-0.105, -0.055], [-0.055, -0.055]], role="texture", region="housing", tone=0.3))
        el.append(line("type-plate-text-2", "burner", [[-0.105, -0.07], [-0.065, -0.07]], role="texture", region="housing", tone=0.3))
        el.append(line("soot-front", "burner", [[0.12, 0.0445], [0.15, 0.0445]], role="texture", region="blast-tube", tone=0.05))
        el.append(line("gasket-joint", "burner", [[0.0, -0.085], [0.0, 0.085]], region="gasket"))
        el.append(line("shutter-ribs", "air-damper", [[0.02, -0.015], [0.02, 0.0]], role="texture", region="housing"))
        for i, y in enumerate((0.135, 0.15)):
            el.append(line(f"control-label-{i + 1}", "burner", [[-0.25, y], [-0.17, y]], role="texture", region="controls", tone=0.6))
    fx = G.CRATE_L
    fr.append(line("hand-slot-front", "crate", [[-0.05, 0.13], [0.05, 0.13], [0.05, 0.155], [-0.05, 0.155]], region="crate", closed=True))
    fr.append(line("tube-mouth-ring", "burner", G.circle(0, YB, 0.0425, 24), region="blast-tube", closed=True))
    if scale == 50:
        for i, (cx, cy) in enumerate([(0.053, 0.053), (-0.053, 0.053), (-0.053, -0.053), (0.053, -0.053)]):
            fr.append(line(f"bolt-hole-{i + 1}", "burner", G.circle(cx, YB + cy, 0.0055, 10), region="gasket", closed=True))
        fr.append(line("swirl-vanes", "burner", [[-0.03, YB], [0.03, YB]], role="texture", region="combustion-head", tone=0.5))
        fr.append(line("nozzle-bore", "burner", G.circle(0, YB, 0.009, 10), role="texture", region="combustion-head", tone=0.5, closed=True))
    for i, z in enumerate((-0.048, 0.048) if scale == 100 else (-0.096, 0.0, 0.096)):
        pl.append(line(f"floor-gap-{i + 1}", "crate", [[0.018, z], [0.602, z]], role="texture", region="crate", tone=0.3))
    pl.append(line("crate-inner-edge", "crate", [[0.018, -0.192], [0.602, -0.192], [0.602, 0.192], [0.018, 0.192]], region="crate", closed=True))
    pl.append(line("transformer-leads", "burner", [[XB - 0.06, -0.02], [XB - 0.035, -0.02]], role="texture", region="controls"))
    return el, fr, pl
