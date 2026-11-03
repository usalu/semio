# r2-pr: plume check of the scenes using the five transport casts

Scripts in `T/🧰️pr/` (`render.sh` own media dir so it does not collide with `r2_render.sh` users, `all.sh`, `frames.py`, `strip.py`, `sheet.py`, `crop.py`, `pop.py` grey-pixel pop detector, `guard.py` staffage_only runner).

## Scenes rendered (ql, 15 fps, de, one at a time, --jobs 1, memory far below 4 GB)
Users of the five casts: mechanical Beat3; natural Beat1, Beat4, Beat6; volume Beat2 (use glyphs, three casts) and Beat3 (4 standing persons); need Beat1 was bp's.
| scene | result |
|---|---|
| mechanical Beat3_Anlagenvergleich | plume vertical over head, forward breath jet at 45 deg; head at correct position; no overlap with labels |
| natural Beat1_Fensterregeln | vertical plume, no overlap with the "innen" label |
| natural Beat4_Auftrieb | vertical plume plus jet; the scene's own noise rings sit behind it, acceptable |
| natural Beat6_GrenzenDerFreienLueftung | fine |
| volume Beat3_Volumenstrom (persons 48-66 s) | four plumes vertical, jets end before next person; dimmed state light |
| volume Beat2_Luftwechselrate (glyphs) | operating-room person fine; classroom students: plumes as tall as the seated body and touching the lamp ellipses (clutter) |

Loop restart: grey-pixel count in the head region never drops to zero after the person appears in mechanical Beat3, natural Beat1, Beat4, Beat6 (min > 0, no frame below 40 except fade-in/out in volume Beat3 where the persons fade by design), so no empty-air pop at 5 s loop wraps.

## Fix
- `V/🧍️staffage/🔣️.json:41` `ventilation-volume-student`: `transportAgeS` 3 to 2, plumes shorter, no longer reach the lamps (re-rendered volume Beat2, verified).

## Open
- Plumes are light grey hair-like strokes; jets of standing persons look slightly arm-like in the first frame of volume Beat3 (visual taste, not changed).
- Unrelated: volume Beat2 failed once with `requireLanguage is not defined` (another agent's in-progress exporter change); passed on retry.
- staffage_only guard: see the result line appended below.
- staffage_only guard after the student change: 20 tests, OK.
