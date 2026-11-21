"""🔥 Round 5 charcoal kettle grill: the round-3 volumes with the round-5 corrections stated by hand, written as the volumes source.

Change: at 1:500 the two wheel legs run 14 mm below the axle centre to their real end (as at 1:50, bottom 0.062 m), so the
25 mm slender tubes are 20.2 times as long as thick (they stopped at the axle centre, 19.7, `volumes-slender` failed).
Usage: `python3 r5-charcoal-grill-build.py [--source]` (candidate under `🗑️generated/r5-charcoal-grill/`).

@see T/r5-brief.md
@see T/r3-objects-charcoal-grill.md
"""
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("r3_charcoal_grill_build", os.path.join(T, "r3-charcoal-grill-build.py"))
B = importlib.util.module_from_spec(spec)
spec.loader.exec_module(B)
OUT = os.path.join(T, "🗑️generated", "r5-charcoal-grill")
WHEEL_LEG_BOTTOM = 0.062
r3_module = B.module


def module(name):
    """🦵 The round-3 module; the 1:500 one with its wheel legs ending at their real bottom."""
    m = r3_module(name)
    if name == "s500":
        K = m.K

        def legs():
            out = []
            for side in ("front", "left", "right"):
                top = K.leg_point(side, K.socket_top_y())
                if side == "front":
                    out.append(K.flush_loft("leg-front", "bowl", "steel-black", "body", 0.25, K.leg_point(side, 0.0), top, 0.0125, n0=(0.0, -1.0, 0.0), n1=K.bowl_normal(top), inset=0.0004, slender=True))
                else:
                    out.append(K.flush_loft("leg-%s" % side, "bowl", "steel-black", "body", 0.25, K.leg_point(side, WHEEL_LEG_BOTTOM), top, 0.0125, n1=K.bowl_normal(top), slender=True))
            return out
        m.legs = legs
    return m


B.module = module

if __name__ == "__main__":
    document = B.build(json.loads(open(B.CATALOG, encoding="utf-8").read()))
    os.makedirs(OUT, exist_ok=True)
    with open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(document, f, ensure_ascii=False, indent=2)
    print("[DEBUG] candidate written")
    if "--source" in sys.argv:
        with open(B.SOURCE, "w", encoding="utf-8") as f:
            json.dump({"volumes": document["asset"]["volumes"]}, f, ensure_ascii=False, indent=2)
            f.write("\n")
        print("[DEBUG] volumes source written")
