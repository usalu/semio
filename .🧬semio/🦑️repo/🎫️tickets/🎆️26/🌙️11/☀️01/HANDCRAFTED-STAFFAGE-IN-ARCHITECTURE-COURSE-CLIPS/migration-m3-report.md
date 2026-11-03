# Migration M3 report: cooling internal-gains clip

Scope: `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/**`.

## Done

- Removed the activity SVG folder `🏃️activity` (SVGs, JSON, schema, old tests), `_ACTIVITY`, `_ACTIVITY_ROOT`, `SVGMobject` and all `person_glyph`, `seated_person_glyph`, `object_silhouette`, `lamp_glyph` uses in `👥️internal-gains/🐍️.py`.
- Everything is now drawn through `staffage` / `staffage_motion` via two local helpers (`_figure`, `_spot`) plus `_lamp`, `_seated`, `_workstation`, `_activity_figure`. Heat anchor points (chest, head, mouth, laptop, bulb) live in the clip's `_HEAT_M` table, in metres from the cast anchor.
- 13 casts `cooling-internal-gains-*` added to the shared manifest `V/🧍️staffage/🔣️.json` (all thermal with `office-neutral` and `theme: dark`, except desk and chair, which are `dark-line`):
  - Humans and clips: `standing` (idle-breathing), `walker` (walk-cycle), `jogger` (run-cycle), `sleeper` (sleeping-person, breathing), `seated-worker` (seated-office-worker, typing).
  - Appliances: `laptop`, `monitor`, `monitor-off`, `lamp`, `lamp-dimmed`.
  - Furniture: `desk`, `office-chair`.
- Mapping of the activity ladder (metabolic watts 80/100/200/800 kept): sleep = sleeper clip, sit = seated-worker clip, walk = walker clip, sprint = jogger clip. Ripples from the chest stay, so the W-per-activity story reads as heat on top of the thermal figures.
- Behaviour changes:
  - Usage panels: monitor icons switch from `monitor-off` to `monitor` (Transform) instead of recolouring.
  - Dimming in Beat 3: lamp crossfades to `lamp-dimmed`.
  - Dimming in Beat 5: lamp crossfades to `lamp-dimmed`.
  - Morph of the activity figures into the scale dots now uses `FadeTransform`. `ReplacementTransform` fails on the motion groups (point count mismatch); the hall's single person into the grid uses `FadeTransform` too.
- Tests:
  - New `👥️internal-gains/🧪️tests/🐍️.py` plus `🔣️.cases.json`. It checks no ad-hoc figure code remains, every used cast is listed with theme dark and thermal for heat figures, and the metabolic watts `[80, 100, 200, 800]` are preserved.
  - `📜️script.ts` test module entry repointed from `…internal-gains.🏃️activity.🧪️tests.🐍️` to `…internal-gains.🧪️tests.🐍️`.
  - Removed `test_activity_art_has_brush_contours_without_speed_lines` from `V/🧪️tests/🐍️silhouettes.py`, which referenced `_ACTIVITY`; the now-unused `activities` / `activity_minimum_cubic_fraction` keys in `🔣️silhouettes.json` are left for the silhouette-removal owner.
  - Passing: the new internal-gains test, `❄️cooling/🎬️clip/🧪️tests`, and `V/🧪️tests/🐍️silhouettes`.

## Rendering check

Rendered Beat 1, 2, 3, 4 and 5 at quality l, 15 fps (manim, pyav frame extraction, since `repo tools exec ffmpeg` reports ffmpeg not provisioned: "Tool ffmpeg not found ... winget install Gyan.FFmpeg") and viewed frames.

- Beats 1, 2, 4 and 5 look right: workstations, seated worker with breathing and heat paths, the activity ladder, the 50-person hall, cards and the room with lamp dimming.
- Beat 3 was viewed before the last tweaks (usage-panel icon rows moved to y 0.72 / 0.12, rack of 2 monitors); those tweaks were not re-rendered.

## Notes and judgement calls

- Assets already contain their furniture: `seated-office-worker` includes chair, desk, monitor and laptop, and `height-adjustable-desk` includes a monitor. Beat 2 and Beat 5 therefore use the worker alone. In Beat 5 the laptop label points at the worker's laptop, and only the lamp dims (the laptop cannot dim separately).
- The "server rack" has no staffage asset. It is drawn as a vertical stack of two `desktop-monitor` casts, and the Beat 3 narration is unchanged. A proper rack asset would be the clean follow-up.
- `sleeping-person` carries its own mattress, so the old drawn bed is gone.
- Rendering is slow: Beat 2 takes about 25 minutes at ql / 15 fps with the 3 s-wide motion groups, so use a timeout of 3000 s or more.
