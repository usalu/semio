"""🚲 Round 5 handcraft of objects/bicycle-stand on top of the round-3 model (`r3-bicycle-stand-model.py`).

Change: the 1:500 thermal hoop (true CHS 48.3 on the real centre line) carries a round section every 18 degrees in the
bends (chord sagitta 2.2 mm) instead of every 22.5 degrees, so the bends read as arcs, not as four-sided polylines (finer steps
would exceed the 1:200 thermal triangle count, rule volumes-monotonic). The 1:500 body pen is 0.14 mm instead of 0.13 mm:
the 63 mm front and plan bars measured 0.193 mm at 1:1000, the end of the declared range, under the 0.2 mm feature width
of the office-laser profile (PRINT range-end-feature); the lateral extent rule caps the bars at 63.3 mm.
Usage: `python3 r5-bicycle-stand-model.py build|apply`.

@see T/r5-brief.md
@see T/r3-objects-bicycle-stand.md
"""
import importlib.util
import math
from pathlib import Path

T = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("r3_bicycle_stand", T / "r3-bicycle-stand-model.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
m.OUT = T / "🗑️generated" / "r5-bicycle-stand"


def hoop_loft():
    """➰ The true centre line of the bent tube in the left leg's frame, round 48.3 mm sections every 18 degrees in the bends."""
    pts = [((-m.XL, 0.0), (0.0, 1.0)), ((-m.XL, m.YB), (0.0, 1.0))]
    for k in range(1, 5):
        r_ = math.radians(180 - 18 * k)
        pts.append(((-(m.XL - m.CLR) + m.CLR * math.cos(r_), m.YB + m.CLR * math.sin(r_)), (math.sin(r_), -math.cos(r_))))
    pts += [((-(m.XL - m.CLR), m.YT), (1.0, 0.0)), ((m.XL - m.CLR, m.YT), (1.0, 0.0))]
    for k in range(1, 5):
        r_ = math.radians(90 - 18 * k)
        pts.append(((m.XL - m.CLR + m.CLR * math.cos(r_), m.YB + m.CLR * math.sin(r_)), (math.sin(r_), -math.cos(r_))))
    pts += [((m.XL, m.YB), (0.0, -1.0)), ((m.XL, 0.0), (0.0, -1.0))]
    s = 0.707107
    secs = [{"at": [m.f(wy), m.f(-(wx + m.XL)), 0.0], "rx": m.R, "ry": m.R, "rotation": [m.f(-(-tx_) * s), m.f(ty * s), 0.0, s]} for (wx, wy), (tx_, ty) in pts]
    return {"id": "th-hoop", "kind": "loft", "bone": "leg-left", "material": "galv-steel", "role": "body", "tone": 0.45, "sections": secs, "capStart": True, "capEnd": True, "slender": True, "region": "hoop-galv"}


TEXTS = [
    ("the true 48.3 mm tube along its real centre line, one slender loft", "the true 48.3 mm tube along its real centre line, one slender loft with a section every 18 degrees in the bends"),
    ("eine schlanke Loft im", "eine schlanke Loft mit einem Schnitt alle 18 Grad in den Bögen im"),
    ("Solid band tone 0.5, outline 0.13 mm", "Solid band tone 0.5, outline 0.14 mm (the 63 mm bars of front and plan then still print 0.2 mm at 1:1000, the end of the range)"),
    ("Volles Band Ton 0,5, Kontur 0,13 mm", "Volles Band Ton 0,5, Kontur 0,14 mm (die Balken von 63 mm in Ansicht und Grundriss drucken so auch bei 1:1000, dem Bereichsende, noch 0,2 mm)"),
]


def retext(node):
    """🔤 Rewrites the 1:500 sentences of round 3 to the 85 mm band, every replacement stated above."""
    if isinstance(node, dict):
        return {k: retext(v) for k, v in node.items()}
    if isinstance(node, list):
        return [retext(v) for v in node]
    if isinstance(node, str):
        for a, b in TEXTS:
            node = node.replace(a, b)
    return node


m.hoop_loft = hoop_loft
m.VOLUME_DECISIONS = retext(m.VOLUME_DECISIONS)
r3_rep_decisions = m.rep_decisions
m.rep_decisions = lambda scale: retext(r3_rep_decisions(scale))
r3_roots = m.roots


def roots(asset):
    """🖊 The round-3 roots plus the 1:500 body pen 0.14 mm: the 63 mm front and plan bars measured 0.193 mm at 1:1000 with 0.13 mm."""
    r3_roots(asset)
    asset["representations"]["500"]["pens"]["body"] = 0.14
    return asset


m.roots = roots

if __name__ == "__main__":
    m.main()
