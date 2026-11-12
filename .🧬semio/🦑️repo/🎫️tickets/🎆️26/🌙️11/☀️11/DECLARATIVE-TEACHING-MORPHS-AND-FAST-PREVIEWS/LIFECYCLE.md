# Lifecycle: leak-cycle results

Harness `$T/lifecycle/cycle.py`, one process, 4 warm-up + 12 measured cycles, Manim 32x18 at 10 fps, Python 3.12.11, win32. Each cycle: open stage -> preview play (real `scene.play`) -> seek (8 random alphas + 0 and 1 on `PreviewSwap.interpolate`) -> profile switch to final (Job keys differ, real full morph) -> export-like capture (3 frames rendered and PNG-encoded) -> cancel mid-play (`KeyboardInterrupt` inside `scene.play`) -> `Stage.close()` twice. Per-cycle sampling happens after two `gc.collect()` calls. The cancel step mirrors `manim.Manager.render`, which calls `file_writer.abort_encode_jobs()` on any `BaseException` raised mid-play; a bare `scene.play` interrupted without that call leaks one unsealed encoder worker and about 13 native encoder threads per cycle (measured with this harness before the abort was added), which is Manim's contract, not a semio defect.

## Plateau verdict

| check | observed | budget | result |
| --- | --- | --- | --- |
| cache entries growth after warm-up | 0 | <= 0 | PASS |
| cache bytes growth after warm-up | 0 | <= 0 | PASS |
| Mobject instances growth (gc census) | 0 | <= 0 | PASS |
| live updaters growth | 0 | <= 0 | PASS |
| suspended updating flags after close | 0 | <= 0 | PASS |
| open handles growth | 0 | <= 2 | PASS |
| child processes growth | 0 | <= 0 | PASS |
| heap (tracemalloc current) growth | -319758 | <= 524288 | PASS |
| heap slope per cycle (least squares) | -476 | <= 65536 | PASS |

## Per cycle

| cycle | s | cache entries | cache bytes | Mobjects | updaters | suspended | heap current B | heap peak B | handles | children | threads |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | 1.781 | 4099 | 128 | 4 | 0 | 0 | 8795799 | 9700634 | 348 | 0 | 38 |
| 1 | 1.309 | 4101 | 256 | 4 | 0 | 0 | 8933939 | 9835508 | 348 | 0 | 38 |
| 2 | 1.202 | 4103 | 384 | 4 | 0 | 0 | 8988865 | 9890193 | 348 | 0 | 38 |
| 3 | 0.945 | 4105 | 512 | 4 | 0 | 0 | 8913597 | 9815206 | 348 | 0 | 38 |
| 4 | 1.26 | 4105 | 512 | 4 | 0 | 0 | 8735426 | 9636670 | 348 | 0 | 38 |
| 5 | 0.916 | 4105 | 512 | 4 | 0 | 0 | 8558764 | 9460282 | 348 | 0 | 38 |
| 6 | 0.761 | 4105 | 512 | 4 | 0 | 0 | 8487677 | 9389179 | 348 | 0 | 38 |
| 7 | 0.739 | 4105 | 512 | 4 | 0 | 0 | 8568239 | 9469529 | 348 | 0 | 38 |
| 8 | 0.837 | 4105 | 512 | 4 | 0 | 0 | 8647060 | 9548370 | 348 | 0 | 38 |
| 9 | 0.79 | 4105 | 512 | 4 | 0 | 0 | 8631809 | 9533098 | 348 | 0 | 38 |
| 10 | 0.794 | 4105 | 512 | 4 | 0 | 0 | 8612308 | 9513700 | 348 | 0 | 38 |
| 11 | 0.855 | 4105 | 512 | 4 | 0 | 0 | 8601182 | 9502554 | 348 | 0 | 38 |
| 12 | 0.9 | 4105 | 512 | 4 | 0 | 0 | 8603246 | 9504721 | 348 | 0 | 38 |
| 13 | 0.882 | 4105 | 512 | 4 | 0 | 0 | 8622674 | 9523874 | 348 | 0 | 38 |
| 14 | 0.762 | 4105 | 512 | 4 | 0 | 0 | 8609687 | 9511117 | 348 | 0 | 38 |
| 15 | 0.793 | 4105 | 512 | 4 | 0 | 0 | 8593839 | 9495098 | 348 | 0 | 38 |

Caches seen at the last cycle (entries, bytes): `bounded:harness.job` (8, 512), `dict:instrument._COUNTS` (1, 0), `dict:stage._REPORTED` (0, 0), `lru:optimization._beam_samples` (1024, 0), `lru:optimization._evaluate` (2048, 0), `lru:optimization._sun_point` (0, 0), `lru:optimization._sun_samples` (1024, 0).

Work counters of the last cycle: preview `{"preview.swap": 1}`, final `{"cache.create[harness.job]": 1, "cache.evict[harness.job]": 1, "full.evaluate": 10, "full.prepare": 4}`.

Top allocation growth sites after warm-up (tracemalloc, bytes): 🐍️.py:135 +53584.

## Shared resources and last-owner release

| property | holds |
| --- | --- |
| definitions valid after first owner closed | yes |
| second owner still plays final | yes |
| definitions unmutated after last owner closed | yes |
| stage owned target released after close | yes |
| definition alive while owner holds it | yes |
| definition releasable after last owner | yes |
| evicted entry valid while held | yes |
| evicted entry releasable after last owner | yes |

## Instrumentation gaps (honest list)

- GPU allocations, browser object URLs, DOM listeners and web workers are not observable here: this harness covers the Python render side only (player/browser lifecycle belongs to the playback owner).
- `tracemalloc` sees Python allocations (including NumPy buffers) but not native Cairo/FFmpeg memory; handles come from `psutil` (`num_handles` on Windows, `num_fds` elsewhere).
- The Mobject census counts gc-tracked instances only; a leak held in a C extension would show in the process working set, which is not sampled per cycle.
- Disk caches (nakagin orders, BIM slabs, structure solves, staffage casts) are bounded and pruned by `🗃️cache/💾️disk` and covered by unit tests, not exercised in this loop.
- Staffage lock waits and the exporter process tree need the exporter command; they are covered by unit tests with fake commands (see the report), not by this loop.
