# Render Admission

Admitted 2026-11-08 03:22 local. Capacity reserved 16384 MB (35822 MB available, 4096 MB floor). Canonical output: no `SEMIO_TEACHING_OUTPUT_ROOT`. Log: `🗑️generated/gh-excel-480p15.log`.

Serial English speaker 480p15, jobs 1, quality l, no play:

- grasshopper_capsule
- grasshopper_casting
- excel_references
- excel_program
- excel_energy

First segment observed: `Rendering Beat2_Capsule`.

The canonical authoring suite (started 2026-11-07 23:58, cpython still live) holds `🗑️generated/full-slab-v4`. The six BIM 480p15 masters stay queued until that suite releases the cache. `bim_information` must be rendered from the current source because the period-car scene changed after the suite imported it.

## BIM information retry

The first admission at 03:27 local reserved 8192 MB and then exited 1. The slab copy used a literal wildcard, so the destination had zero JSON files. That run is discarded.

Retried at 03:28 local against `🗑️generated/full-slab-render` (31 JSON files, 254492945 bytes, matching `full-slab-v4`). Capacity reserved 8192 MB. Log: `🗑️generated/bim-information-480p15.log`. First segment: `Rendering Beat1_DataToInformation`. No `SEMIO_TEACHING_OUTPUT_ROOT`.

## Queued BIM cohort

A waiter is holding until the BIM information shell exits. On exit 0 it renders `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, and `bim_consistency` serially at 8192 MB against `🗑️generated/full-slab-render`. Log: `🗑️generated/bim-rest-480p15.log`.

## Checkpoint 03:33 local

Grasshopper capsule (`Beat2_Capsule`, scene 121) and BIM information (`Beat1_DataToInformation`, scene 114) are still inside scene construction, before Manim's first animation line. Both processes are accumulating CPU. The BIM render has touched slab-cache locks in `🗑️generated/full-slab-render` and has not started a new slab compute (no temporary record). `present_information` builds `building_geometry` before the first `step`; `present_capsule` solves the capsule graph before the first `step`. The five-lesson BIM waiter is still parked on the information shell. No preview join.
