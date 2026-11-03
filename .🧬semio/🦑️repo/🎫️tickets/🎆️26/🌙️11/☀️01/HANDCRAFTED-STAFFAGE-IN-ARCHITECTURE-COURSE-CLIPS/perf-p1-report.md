# P1 report: staffage performance (time and memory)

## Causes found

| Problem | Cause |
|---|---|
| Renders over 20 min, 42 GB host starvation | Clips with long timelines exported one SVG per 1/15 s: toaster 4501 frames, laptop 9001, vacuum 22501 (1500 s). The loader parsed every frame with `SVGMobject` (about 0.1 to 0.2 s each) and kept every frame as a full VMobject family (about 0.8 to 1.2 MB each, so 10 to 26 GB per process, times `--jobs`). `_TEMPLATES` also kept a pristine copy of every frame forever. |
| `--disable_caching` | Manim hashes the whole `__dict__` of a mobject recursively; `_Motion.frames` held thousands of mobjects, so hashing was huge and slow. |
| Export queue over 600 s | `repo tools exec` reserved 2048 MB and queued behind other agents, although the exporter is light. The exporter also rendered all casts first and held all scenes and SVG strings in memory before writing. |

## Fixes

- Exporter (`S/🔨️modules/📤️export/🎭️cast/🟦️.ts`): casts render and publish one at a time, scenes are reduced to SVG text at once, schedules over 600 frames are thinned to 600 samples (fractional `fps`), identical frames are written once and repeated by name, progress is throttled, final `[peak]` line prints resident memory. Tests: the cast-export test accepts repeated names and has a thinning test.
- Loader (`V/🧍️staffage/🐍️.py`): own dependency-free path parser (`M L H V C Z`) to world-metre Bezier arrays and styles; bounded LRU of parsed frames (256, keyed by path, mtime, size); `_Motion` is lazy: frames are parsed when shown and written into a fixed set of slots (point arrays and styles swapped, the family never changes); `_Source` (no `__dict__`) holds the folder so the Manim play-hash is small and deterministic (content key, frame names, slot state); the exporter runs with `--light`.
- Manim validation: the parser equals Manim's `SVGMobject` import (points to 1e-9, styles, widths) on 193 frames of all 50 cached casts; stroke opacity is exact instead of quantised to 8 bit.
- Tests (24, all green): parser versus `SVGMobject` (lines, curves, subpaths, mirror, fill none), fixed family under frame swapping, bounded cache with exact revisits, shared parse of repeated frame names, small deterministic play-hash.
- Migration: `⚖️heating-comparison` toaster (`rate` 12) and laptop (`rate` 24) now `staffage_motion`; `🌤️summer-strategy` already had no static devices.

## Measurements (Windows, quality l, 15 fps)

| | Before | After |
|---|---|---|
| Load occupant (75 frames) | 16.7 s, 326 MB | 0.0 s, 110 MB |
| Load radiator (600 frames) | 49.8 s, 573 MB | 0.1 s, 109 MB |
| Toaster / laptop / vacuum load | extrapolated 6 min / 12 min / 31 min, about 3.5 / 7 / 26 GB | 0.2 s, about 105 MB each (600 frames) |
| Export of the 5 worst casts | thousands of frames, GB | 14 s, 368 MB peak |
| Benchmark scene (occupant + radiator + toaster + laptop, 10 s) | over 20 min, over 11 GB (extrapolated) | 13.2 s, 214 MB peak; cached re-render 5.8 s, 122 MB |
| Real segment (`Beat1_Bilanzgrenze`, EnergyBalance_01, 48 animations, caching off) | not measured | 128 s, 305 MB |
| `heating-comparison` Beat1 with animated toaster and laptop | not measured | 97 s, 358 MB |

The vacuum cast alone was about 26 GB per process, so three jobs of `physical_fundamentals` match the 42 GB observed.

## Open points

- The TS vitest run (`repo run @teaching/staffage:test`) sat in the build queue for minutes behind other agents; the exporter changes are verified by a manual export of five casts and the Python tests, not by vitest. Please run it when the queue is calm.
- The segment runtime still passes `--disable_caching` to every scene; staffage no longer needs it.
- A thinned long clip played at `rate=1` shows coarse steps (physics vacuum uses the default rate); set `rate` for those.
- Scripts and benchmark are in this folder: `p1_bench.py`, `p1_run.py`, `p1_load.py`, `p1_equiv.py`, `p1_rss.py`, `p1_frames.py`.
