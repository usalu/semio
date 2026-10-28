p = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/scene-extrema.md"
s = open(p, encoding="utf-8").read()
start = s.index("- Re-capture en and de with the toolkit change")
end = s.index("### Remaining risks", start)
new = """- Re-capture of both languages after the toolkit's B1 fix and all scene changes (29 frames each): `capture.py` strict reports **en 0 findings, de 0 findings**. Every PNG was read; the changed frames (response, sweep, sweep_2, sweep_3, sweep_4, local_2, global, negate_2, constraint, compare_2, scope, scope_2, and the German counterparts) were read again after the final capture. The strict check still tests boxes, not lines (see Round 2 correction), so the eye check stays the reference.
- Caption wrapping (`T/extrema-r3-captions.py`): the one-word orphans are gone (response, global de, scope_2 en and de, global_3 de now fit one line); two-word endings remain in a few captions ("curve flips.", "east façade?", "in kWh/a.").
- Numbers: `T/check-extrema-numbers.py` and `T/check-extrema-r2-tables.py` have 0 failures; the latter now rebuilds heating and cooling demand from the printed terms at 90°, 180°, 282°, 315°, 360° (equality to 1e-6) and the lamp product at 90°, 180°, 360° (0.432 kW × 1,500 h/a × 22.7 / 20.8 / 25.3 % = 147 / 135 / 164 kWh/a).
- Tests: `C/🧪️tests/🐍️.py` with `SEMIO_TEACHING_OUTPUT_ROOT=T/🗑️generated/output`: see `T/🗑️generated/extrema-r2/tests.log` (result noted in the reply).

"""
open(p, "w", encoding="utf-8").write(s[:start] + new + s[end:])
