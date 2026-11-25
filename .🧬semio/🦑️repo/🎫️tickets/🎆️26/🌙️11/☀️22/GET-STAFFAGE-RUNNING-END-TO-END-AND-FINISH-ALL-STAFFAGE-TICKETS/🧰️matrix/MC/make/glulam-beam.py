"""🪵️ Ops writer of `glulam-beam` (a leftover of batch MB-1, 7 `derived-view-in-band` cells). At 1:500 the side elevation is one rectangle of 8.00 x 0.62 m, the 0.60 m beam with its two 20 mm bearing plates merged into the bar, and the plan one rectangle of 8.00 x 0.16 m; the measure normalises by the 8 m span and finds them 230 mm apart where 240 mm are asked. Batch MC had drawn the bearing plates as steps of 120 x 20 mm under the ends of the outline (0.24 x 0.04 mm on paper, below the 0.13 mm outline pen), which only the measure saw; the coordinator refused geometry below what the scale prints (2026-11-25), so this writer puts the merged bar and its thin-element text back and the two views are declared the same drawing at 1:500 by an allowlist entry with its physical reason (`🧰️matrix/MX/allowlist.json`, decision D51). Writes `ops/glulam-beam.json`; a second run writes the same ops."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("glulam-beam")
beam = shape(a, 500, "elevation", "beam")
bar = like(beam, drop=("points", "closed", "kind", "x", "y", "w", "h"), kind="rect", x=0, y=-0.32, w=8, h=0.62)
thin = {
    "en": "There are no thin parts; the 20 mm plates (0.04 mm) are merged into the bar bottom, a deviation of 0.04 mm.",
    "de": "Es gibt keine dünnen Teile; die 20-mm-Platten (0,04 mm) gehen im Balkenfuß auf, eine Abweichung von 0,04 mm.",
}
write("glulam-beam", [op(500, "elevation", "beam", bar), put(["representations", "500", "decisions", "thinElements"], thin)])
