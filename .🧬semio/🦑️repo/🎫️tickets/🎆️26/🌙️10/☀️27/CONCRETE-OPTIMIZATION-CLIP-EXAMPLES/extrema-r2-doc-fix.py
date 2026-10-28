import sys
p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/scene-extrema.md"
s = open(p, encoding="utf-8").read()
old = s[s.index("- `T/capture.py` en and de: every PNG"):s.index("### Remaining risks")]
new = """- `T/capture.py` en and de: every PNG of both languages viewed (29 frames each). Final strict reports: **en 0 findings in 29 frames, de 0 findings in 29 frames** (`T/🗑️generated/frames/Beat1_LocalGlobalExtrema/…-layout.md`); the classic checker's remaining lines are bounding-box noise (a ring-shaped sector polygon under the plan's centre label).
- Tests: `C/🧪️tests/🐍️.py` with `SEMIO_TEACHING_OUTPUT_ROOT=T/🗑️generated/output`: 63 tests OK (narration, captions fit in two lines in both languages and audiences, curriculum, entry points).
- Real playback: `T/render-smoke-extrema.py en` plays the whole scene (no skipped animations, 320 × 180) without error: 278 s render time, 293 s scene time (4:53; was 4:14: the three added beats and the longer table beats).
- Numbers: `T/check-extrema-numbers.py` (narration vs model) and `T/check-extrema-r2-tables.py` (formula parts at 90/180/225/282/315/360/450°, derivations demand ÷ COP/EER and 648 kWh/a × lamps-on share, neighbour tables, north-to-south and east-to-south tables, saving 449.8 kWh/a × 0.25 = 112.5 CHF/a, courtyard price 201 kWh/a = 50 CHF/a = 29.5 %) all `ok`.

"""
open(p, "w", encoding="utf-8").write(s.replace(old, new))
print("ok")
