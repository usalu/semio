# Fix Subtitle Read Time

## Root cause
Most Heating + Cooling parts 2/3/4/6 timed `hold_for` from English speech WPS (2.5) while showing longer German captions. Reading-only (silent) full renders felt rushed. Audit found 268 clauses with EN budget short of DE reading need at 1.6 wps.

## Fix
1. `manim_visuals.hold_for`: budget = max(measured TTS, WPS estimate, German `subtitle_read_seconds` at 1.6 wps, min 1.5s).
2. `set_vo_language("de")` on every captioned Heating/Cooling scene + final_calculation.

## Verification
391 clauses checked; 0 with budget < read need. See `audit.txt`.
