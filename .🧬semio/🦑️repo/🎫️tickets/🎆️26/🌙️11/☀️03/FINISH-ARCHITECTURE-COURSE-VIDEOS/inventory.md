# Course video inventory

Goal: finish every architecture course video, then rerender the preview that includes all of them.

Ticket: `2026/11/03/FINISH-ARCHITECTURE-COURSE-VIDEOS`. Goal: `🎯updatedtutorials`.

Manifest: `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json` (26 videos).

Canonical join: `HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media/course_preview.py`. It aborts unless every lesson master in `media-manifest.json` is fresh. No `media-manifest.json` exists yet.

Cache root: `.🧬semio/🎓️teaching/⚡️cache/videos`.

## English speaker masters (`en/speaker/rendered/*_speaker_en_480p15.mp4`)

Present: the five energy lessons (`Full_Physical_Fundamentals`, `Full_Ventilation`, `Full_Heating_Demand`, `Full_Cooling_Demand`, `Full_EnergyBalance`), four Excel lessons, five Grasshopper lessons.

Absent: six BIM lessons, six optimization lessons.

An older joined preview exists: `en/speaker/preview/Architecture_course_speaker_en_480p15.mp4` (about 290 MB). BIM and optimization appear there only as standalone preview copies, not as canonical rendered masters. Those copies have no cast-key provenance, so the gated join will not accept them.

## German speaker masters

Present: Excel and Grasshopper only. No energy, BIM, optimization, and no `Architecture_course_speaker_de_480p15.mp4`.

## Freshness (`status.py en --stale-only`, 2026-11-03)

Energy lessons are stale on every segment (masters exist, none fresh): physical_fundamentals 0/8, ventilation 0/22, heating 0/35, cooling 0/22, energy_balance 0/26.

BIM lessons have no master: bim_information, bim_objects, bim_representations, bim_models, bim_exchange, bim_consistency. Each segment file is stale. A fresh BIM render still dies in `_finish_reservations` on `RES-FIN-ROOF-GRAVEL-SL-R-705` (about 0.00297 m3 unoccupied).

Status then raised `ValueError: malformed node` in `source_narration` on `grasshopper_anatomy`, because `Beat1_Anatomy.NARRATION` is `_figures([...])` rather than a literal. `narration_rows` now imports that class when `ast.literal_eval` cannot read the assignment. Excel, the other Grasshopper lessons, and optimization were not reached.

## Render

`render-en.ps1` renders the 20 non-BIM English speaker lessons at 480p15, one at a time, three attempts, jobs 1. The course preview is joined only after those masters and the six BIM masters are fresh.

Published this run (`[DEBUG] <id> ok attempt=1`): physical_fundamentals (00:06, 43.5 MB), ventilation (00:57, 108.2 MB), heating (01:44, 120.9 MB, 6 chapter windows, 26028 frames).

Cooling attempt 1 rendered the later scenes, then failed chapter assembly because `Beat1_WinterGains` and `Beat2_SummerOverheat` ask for cast `cooling-heating-comparison-table`, which is not in the staffage manifest. The winter and summer dining-table casts are. `_internal_gains` now uses `cooling-heating-comparison-table-winter` or `cooling-heating-comparison-table-summer` with the rest of the season cast. Attempt 2 had already loaded the old scene, so attempt 3 is the one that picks up the fix.

`RES-FIN-ROOF-GRAVEL-SL-R-705` now completes inside the full roof void list. The regression test is `test_roof_gravel_window_705_is_exactly_occupied`. `render-bim.ps1` waits for the English media lock, then renders the six BIM lessons.
