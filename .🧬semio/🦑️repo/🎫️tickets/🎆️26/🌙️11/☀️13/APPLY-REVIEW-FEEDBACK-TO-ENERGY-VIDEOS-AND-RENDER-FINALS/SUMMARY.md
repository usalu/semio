# Summary: review feedback applied, five German finals rendered

## Result
All five energy videos are rendered as German finals (1080p30), each in a presenter (`speaker`) and an
audience (`spectator`) version. They sit in one folder per audience:

`.🧬semio/🎓️teaching/⚡️cache/videos/final/de/<speaker|spectator>/rendered/`

| Video | File stem |
| --- | --- |
| Physical fundamentals | `Full_Physical_Fundamentals_<audience>_de_1080p30` |
| Ventilation | `Full_Ventilation_<audience>_de_1080p30` |
| Heating demand | `Full_Heating_Demand_<audience>_de_1080p30` |
| Cooling demand | `Full_Cooling_Demand_<audience>_de_1080p30` |
| Energy balance | `Full_EnergyBalance_<audience>_de_1080p30` |

Each `.mp4` has its `.render.json` and `.export.json` beside it. The working roots `⚡️cache/vb`, `vc`, `vd`,
`ve` hold the same files plus their segment caches and can be removed.

## What was done
- Reviewer feedback F00–F29 (`FEEDBACK.md`) applied; mapping to scenes in `MAPPING.md`, shared notation rules
  in `CONVENTIONS.md`, per-pass details in the `REPORT.md` of each sub-folder.
- The overview and introduction show real frames of the videos instead of placeholders
  (`UI-INTRO-REPORT.md`); posters recaptured for physics and energy balance.
- Every video opens with the roadmap; references are findable (`references/REPORT.md`).
- Two render-and-audit rounds on full finals. Round 1 found held-frame defects in heating, energy balance
  and ventilation; they are fixed (`fix-heat2`, `fix-eb2`, `fix-vent2`) and confirmed by eye in the round 2
  finals. A last overlap in the heating thermal-bridge result (corner drawing against the result box) was
  fixed afterwards and heating rendered a third time.
- Morph engine: `split` accepts `copy: true`.
- Presentation renderer: a press is replayed as a single pointer-down.

- Round 4, after the per-item verification: three ventilation leftovers (roadmap chip "Kühltechnik",
  "Lüftungsterm Q̇_V der Heizlast", "ungenutzt" label off the frame edge) and the clock spacing in physics
  ("22:00 Uhr"); ventilation and physics rendered again and the four spots confirmed by eye.

## Verification
Every item F00–F29 and the introduction frames are checked against the finals: index with status,
timestamp and my rulings in `VERIFICATION.md`, frame-level evidence in `verify/*.md`.

## Tests
All 66 test modules exit 0 on the final sources (`tests-final/RESULTS-FINAL.md`). They run one by one
through `tests-final/run-all.sh`, because the combined `script.ts test` command hangs; five modules need a
different launch (the `repo` build with `--light` on PATH, or profile `preview`) and are green with it. The
modules touched by the round 4 fixes (roadmap, ventilation, physics) were run again afterwards.

## Not done
- The voice track is silent: there is no speech key (`🎓️teaching/.env` is missing) and no cached audio.
- English finals are not rendered.

## Refactorings outside the feedback (no effect on the picture)
None of these is reviewer feedback or changes a frame; each would invalidate the render fingerprint of
several finals, so they are not part of this ticket.
- Move the width-aware reference chip into `🏗️authoring`; heating, cooling and ventilation chips are
  single-line.
- `stage.transition` should remove a source group's parts even when they were added one by one; the
  `_regroup` helpers in `📊️demand` could then go.
- The heating clip hard-codes 11500 kWh/a and repeats `ETA_UTILISATION`; a shared running-example constant
  would remove both.
- Posters for BIM, Grasshopper, Excel and Optimization are stale (other lanes).
- The `repo` binary on PATH lacks `tools exec --light`; renders used the build under
  `AppData/Local/Semio/repo/aa03a414…`.

## Rendering notes
- `final-render.sh <video-id> <root-name>` renders one German final and retries when another lane edits
  inputs mid-render. Any edit to energy sources aborts every running lane, so batch edits before rendering.
- `fix-heat/frame.sh` extracts frames (`VID`, `OUT`, times in seconds).
