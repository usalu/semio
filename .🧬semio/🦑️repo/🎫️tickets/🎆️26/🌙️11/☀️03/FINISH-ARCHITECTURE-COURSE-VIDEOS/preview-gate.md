# English Course Preview Gate

Read-only audit of `join-en.ps1` in this ticket and of `manifest.py`, `course_preview.py`, and `lib.py` in `HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️r3-media`. No render was started.

## Joiner

`join-en.ps1` polls every 60 seconds.

- `generated/render-bim.log` containing `bim_consistency gave up` exits 1.
- It continues only when that log contains `bim_consistency ok` and `videos/en/.media-lock` is absent. The lock path is `.🧬semio/🎓️teaching/⚡️cache/videos/en/.media-lock`.
- It then runs `manifest.py en speaker` and `course_preview.py en`.
- It does not pass `--allow-missing`, `--video`, or any other flag.

`course_preview.py` walks every lesson in curriculum order. A lesson whose manifest master has `fresh` true is concatenated. Every other lesson is missing. Any missing lesson raises `SystemExit` with `stale or missing lesson masters: ...` and writes no preview.

## What a lesson master must satisfy

`manifest.py` sets `videos[<id>].master.fresh` from `lib.master_fresh`. All of the following have to hold. The encoded 480p15 file by itself does not.

1. The curriculum entry supplies `output` (otherwise the id is used). For English speaker the master path is:

   `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered/<output>_speaker_en_480p15.mp4`

   `join-en.ps1` sets `SEMIO_TEACHING_CACHE_ROOT` to `.🧬semio/🎓️teaching/⚡️cache`, which is the cache `lib.base("en", "speaker")` uses. The label `480p15` is fixed in `lib.py` (`QUALITY = "-ql"`, `FPS = 15.0`).

2. The sibling `<output>_speaker_en_480p15.render.json` exists. Its shape is the one `publish_presentation` writes: `content.chapters[].segments`. A sidecar without that shape raises inside `manifest.py` and the joiner stops before the preview.

3. Every segment in `segment_plan()` for that lesson is fresh under `reusable_clip`:

   - Clip: `videos/en/speaker/s/<source_ordinal>/v/<scene>.mp4`
   - Size greater than 1000 bytes
   - Sibling `<scene>.render.json` exists and its JSON equals the current `segment_state` exactly
   - `source_ordinal` is the first curriculum occurrence of `(script, scene)`. Clause ranges do not get their own ordinal. Older copies of the same scene name under a different `s/<n>/` directory are not read.

4. The flattened `content.chapters[].segments` list in the master sidecar equals those current segment states, in plan order.

The state used for both checks is `segment_state({script, scene}, settings)` with settings `{language: en, quality: -ql, resolution: null, fps: 15.0, audience: speaker}`. `clauses` are omitted. The state embeds that settings object, SHA-256 of every authored dependency, the `{script, scene}` identity, and the staffage cast keys.

`[lesson] Ready: <path>` is printed by `compose_videos` after `publish_presentation`. Neither `manifest.py` nor `course_preview.py` reads that line.

When a segment is fresh, `manifest.py` also calls `staffage_provenance` for each cast. A cast index whose viewpoint or provenance disagrees with the staffage manifest raises `ValueError` and aborts the manifest for every lesson.

## Files and directories read

Wait gate:

- `<ticket>/generated/render-bim.log`
- `.🧬semio/🎓️teaching/⚡️cache/videos/en/.media-lock`

Curriculum, via `load_manifest` / `segment_plan`:

- `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json`
- Each lesson `script` and each segment `script` (must exist). A segment with `clauses` is parsed for a literal or `_figures([...])` `NARRATION`.

Per lesson, under `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/`:

- `media-manifest.json` (previous document, then rewritten)
- `rendered/<output>_speaker_en_480p15.mp4`
- `rendered/<output>_speaker_en_480p15.render.json`
- `s/<source_ordinal>/v/<scene>.mp4`
- `s/<source_ordinal>/v/<scene>.render.json`

The segment state hash also reads the scene script, imported `🎓️teaching` modules, sibling assets (`.json`, `.pdf`, `.svg`, `.png`, `.jpg`, `.jpeg`, `.webp`, `.ttf`, `.otf`, `.mp3`, `.wav`, `.m4a`, `.flac`, `.ogg`, tests excluded), per-language `timing.json`, narration recordings, theme sources, audience sources, and cast keys from the staffage module. A fresh segment then reads each cast's rendered index.

`course_preview.py` reads the new `media-manifest.json`, the same curriculum order, and `lib.ROOT / master.file` for each fresh master. It does not open segment clips again.

## Preview output

`.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/Architecture_course_speaker_en_480p15.mp4`

Beside it: `Architecture_course_speaker_en_480p15.json`, `Architecture_course_speaker_en_480p15.poster.png`, and `Architecture_course_speaker_en_480p15.thumb.png`.

Included masters are scaled and padded to 854×480 at 15 fps, audio stripped, then concatenated with chapter titles. That normalize step is not the inclusion test.

## The 26 lesson outputs

Curriculum order. Each file is `<output>_speaker_en_480p15.mp4` in `videos/en/speaker/rendered/`, and each needs the `.render.json` sibling.

| # | id | encoded master |
| --- | --- | --- |
| 1 | physical_fundamentals | Full_Physical_Fundamentals_speaker_en_480p15.mp4 |
| 2 | ventilation | Full_Ventilation_speaker_en_480p15.mp4 |
| 3 | heating | Full_Heating_Demand_speaker_en_480p15.mp4 |
| 4 | cooling | Full_Cooling_Demand_speaker_en_480p15.mp4 |
| 5 | energy_balance | Full_EnergyBalance_speaker_en_480p15.mp4 |
| 6 | bim_information | BIM_information_speaker_en_480p15.mp4 |
| 7 | bim_objects | BIM_objects_speaker_en_480p15.mp4 |
| 8 | bim_representations | BIM_representations_speaker_en_480p15.mp4 |
| 9 | bim_models | BIM_models_speaker_en_480p15.mp4 |
| 10 | bim_exchange | BIM_exchange_speaker_en_480p15.mp4 |
| 11 | bim_consistency | BIM_consistency_speaker_en_480p15.mp4 |
| 12 | grasshopper_anatomy | Grasshopper_anatomy_speaker_en_480p15.mp4 |
| 13 | grasshopper_capsule | Grasshopper_capsule_speaker_en_480p15.mp4 |
| 14 | grasshopper_tower | Grasshopper_tower_speaker_en_480p15.mp4 |
| 15 | grasshopper_casting | Grasshopper_casting_speaker_en_480p15.mp4 |
| 16 | grasshopper_trees | Grasshopper_trees_speaker_en_480p15.mp4 |
| 17 | excel_anatomy | Excel_anatomy_speaker_en_480p15.mp4 |
| 18 | excel_references | Excel_references_speaker_en_480p15.mp4 |
| 19 | excel_program | Excel_program_speaker_en_480p15.mp4 |
| 20 | excel_energy | Excel_energy_speaker_en_480p15.mp4 |
| 21 | optimization_extrema | Optimization_Extrema_speaker_en_480p15.mp4 |
| 22 | optimization_sampling | Optimization_Sampling_speaker_en_480p15.mp4 |
| 23 | optimization_experiments | Optimization_Experiments_speaker_en_480p15.mp4 |
| 24 | optimization_index | Optimization_Index_speaker_en_480p15.mp4 |
| 25 | optimization_pareto | Optimization_Pareto_speaker_en_480p15.mp4 |
| 26 | optimization_algorithms | Optimization_Algorithms_speaker_en_480p15.mp4 |

The rendered directory currently has `.render.json` sidecars for the five energy lessons, five Grasshopper lessons, four Excel lessons, and six optimization lessons. The six `BIM_*_speaker_en_480p15.render.json` files are absent, so those six masters are missing and the unguarded join aborts until they are fresh.

## Heating's existing master

`Full_Heating_Demand_speaker_en_480p15.mp4` and its `.render.json` are present. The sidecar is the published shape (`content.video` is `heating`).

`Heating_05_Solar` uses `Beat6_ThermischeMasse` twice, both from `🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`, with clause ranges `store`–`room` and `winter`–`winter`. The master sidecar stores those ranges inside the two segment states. `video_status` builds both states from `(script, scene)` only, so the two current states are equal to each other and unequal to the recorded states.

`master_fresh` therefore returns false for this file even when the source clip is reusable and the mp4 exists. `course_preview.py` then treats `heating` as missing and aborts the whole preview.

The same comparison rejects the existing cooling and energy-balance masters. Their sidecars also embed `clauses` (cooling has five, energy balance has the `AnlagenVerluste` range `chain`–`sum`), while the checker drops `clauses`.

## Optimization masters that already printed Ready

All six parallel logs contain a Ready line, for example:

`[optimization_extrema] Ready: ...\videos\en\speaker\rendered\Optimization_Extrema_speaker_en_480p15.mp4`

The same encoded path is logged for Sampling, Experiments, Index, Pareto, and Algorithms. Those six curriculum segments have no `clauses`, and their sidecars contain no `clauses`, so the heating comparison does not reject them.

Ready is still not acceptance. Each lesson is included only when its single source clip at the live `s/<source_ordinal>/v/<scene>.mp4` is reusable against the current `segment_state` and the one recorded chapter segment equals that state. Scene files left under earlier ordinals (the same scene names also exist under `s/120`–`s/125`) are not the files this check reads.
