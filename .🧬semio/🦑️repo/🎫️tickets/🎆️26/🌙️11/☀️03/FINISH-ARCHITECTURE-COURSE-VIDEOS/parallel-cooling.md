# Parallel Cooling And Energy Balance

Cooling: yes. Energy balance: yes. Neither lesson's source directories intersect a directory anatomy, optimization, or BIM is using.

## Ordinals that would be written

Source ordinal is the first `(script, scene)` occurrence in `segment_plan` order over the full curriculum in `teaching/architecture/energy/clip` manifest. Later repeats keep that ordinal. A lesson render addresses `videos/en/speaker/s/<source_ordinal>`.

Cooling (`cooling`), 21 source directories:

6, 53, 54, 56, 66, 67, 68, 69, 70, 71, 72, 73, 74, 77, 78, 82, 83, 84, 85, 86, 87

Energy balance (`energy_balance`), 26 source directories:

65, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113

Occurrence numbers that are not directories: 75 and 76 repeat 6, 79 repeats 53, 80 repeats 56, 81 repeats 54, 100 repeats 65.

## Why this does not share a writer

Live `--media_dir` values at check time were only:

- `s/114` BIM information
- `s/115` BIM objects
- `s/116` BIM representations
- `s/117` BIM models
- `s/120` grasshopper anatomy (Manim PID 79444, `Beat1_Anatomy`)

No process command line referenced cooling, energy balance, optimization, `bim_exchange`, `bim_consistency`, or ordinals 6, 53, 54, 56, or 65.

Manifest ownership of the other in-flight lessons:

- BIM information through consistency: 114, 115, 116, 117, 118, 119
- Grasshopper anatomy: 120
- Optimization extrema through algorithms: 129, 130, 131, 132, 133, 134

Those sets are disjoint from both lists above. Optimization's Manim phase is finished: `s/129` through `s/134` each have a canonical mp4 with render and cursor sidecars, about 12 to 17 minutes old, and no live process. `s/118` and `s/119` are not in a live command line.

Lesson index directories are also disjoint. Cooling is video 4 and energy balance is video 5. BIM is 6 through 11. Anatomy is 12.

## Shared identities

The only repeated `(script, scene)` identities in the 134-segment curriculum are these five. The previously named ordinals all match. There are no others.

| Ordinal | Scene | Lessons, first occurrence then repeats |
| --- | --- | --- |
| 6 | Beat6_ThermischeMasse | physical fundamentals (6), heating (57, 58), cooling (75, 76) |
| 53 | Beat2_BestrahlungUndFlaeche | heating (53), cooling (79) |
| 54 | Beat3_GWert | heating (54), cooling (81) |
| 56 | Beat5_Verschattung | heating (56), cooling (80) |
| 65 | AnlagenVerluste | heating (65), energy balance (100) |

## Fresh mp4s in the shared directories

Cache: `videos/en/speaker/s` under the teaching cache. Each shared directory contains a canonical mp4 larger than 1000 bytes with render, cursor, and trace sidecars. Settings in those render sidecars are English, speaker, `-ql`, 15 fps. No live process has these directories open.

| Ordinal | Canonical | Clause excerpt already on disk |
| --- | --- | --- |
| 6 | Beat6_ThermischeMasse.mp4, about 7.0 h old | day-lag `026e720f3a` and summer-summer `312d34bf84`, about 5.0 h, plus the heating excerpts |
| 53 | Beat2_BestrahlungUndFlaeche.mp4, about 5.6 h | a-ff `1219281f6e` |
| 54 | Beat3_GWert.mp4, about 5.6 h | split-gval `7cba3bfa47` |
| 56 | Beat5_Verschattung.mp4, about 5.6 h | unshaded-reduced `4b60a85592` |
| 65 | AnlagenVerluste.mp4, about 5.4 h | chain-sum `a6a0a4e0a7`, about 4.5 h |

Those excerpt names are the clause selections cooling and energy balance declare. A non-force render still rewrites a source if the sidecar fingerprint does not match. Sidecar presence alone does not prove the fingerprint. These five directories are not open by anatomy, optimization, or BIM either way.

Two cooling sources and two energy-balance sources have an mp4 and a cursor sidecar but no render sidecar, so a new render would write them: 66 and 67 (about 4.4 days old), and 105 and 107 (about 4.4 days old). The other cooling and energy-balance ordinals already have render and cursor sidecars from the same English speaker low-quality pass, about 4.5 to 5.2 hours old.
