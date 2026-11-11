# Master Inventory

Counted 2026-11-08 03:38 local from `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/rendered`. This is file presence, not `master_fresh`.

Twenty English speaker 480p15 masters are on disk. Fifteen are dated 2026-11-07: the five energy films, Grasshopper anatomy, tower and trees, Excel anatomy, and all six optimization films. Five are dated 2026-10-29 and are in the running serial render: Grasshopper capsule (now constructing `Beat2_Capsule`), Grasshopper casting, Excel references, Excel program, Excel energy.

The six BIM lessons exist only as 1080p30 masters from 2026-10-28. No `BIM_*_speaker_en_480p15.mp4` is present. `BIM_information` is constructing `Beat1_DataToInformation` against `🗑️generated/full-slab-render`. The other five BIM lessons are queued behind that process.

At this count the capsule process had about 623 s of CPU and 3.9 GB resident, and the information process about 376 s of CPU and 1.3 GB resident. Neither Manim log had reached `Animation 0`.

## Source diff at 03:40 local

Working tree versus HEAD changes three lesson sources: BIM information, Excel energy, and Excel references. All three are already in the running or queued 480p15 renders, which started after those edits. The staffage bridge and the visuals package match HEAD, so the 7 November masters are not invalidated by the 03:20 file touch. Yesterday's capsule attempt wrote its first partial about 18 minutes after the scene started. This attempt is still inside that window.

## First frames 03:43 local

`Beat2_Capsule` wrote `uncached_00000` through `uncached_00003` from 03:43:36 and reached Animation 4. BIM information had no new partial at that time; its process was still in scene construction.
