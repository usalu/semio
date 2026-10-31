# English speaker previews of the 12 BIM and optimization videos

Output folder: `.🧬semio/🎓️teaching/⚡️cache/videos/en/speaker/preview/` (`<Output>_speaker_en_480p15.mp4` plus `.json`). All 854x480 at 15 fps.

| Video | Result |
|---|---|
| Optimization_Extrema, Sampling, Experiments, Index, Pareto, Algorithms | rendered from current code with `render/preview_any.py` (one import snapshot, no patch), exit 0, 287 s video for Extrema, about 3 to 13 min wall each |
| BIM_information, objects, representations, models, exchange, consistency | **fallback: older delivery** (1080p30 delivery scaled with the `imageio_ffmpeg` ffmpeg by `render/fallback_delivery.py`) |

## Why the BIM videos fell back

- All six BIM scenes call `building_geometry()` of the shared `🏗️bim/🎬️clip/🧱️model/🐍️.py` before the first frame. It builds exact rational (`Fraction`) geometry and is memoised only in process.
- With 4 scenes running, none wrote a frame in over 25 minutes (about 21 CPU minutes each). Stack sampling (`render/dump_any.py`, `render/sample_geometry.py`) put about 90 percent of the time in the roof slab `SL-R`: `_slab_geometry_uncached` -> `_finish_reservations` -> `_reference_material_parts` / `_clip_stock_kernel` while wall `SC-1` waits for its cutters.
- A preview-only patch (`render/bim_fast.py`, skips `_finish_reservations`) was timed with `render/time_geometry.py`: `building_geometry` still took 6027 s under cProfile (65 walls; `_solid_geometry` 4570 s, `_wall_owned_parts` 2235 s), so the cost is spread over the whole building, not one hotspot. Six processes would each repeat it, since there is no cross-process cache.
- No shared BIM source was edited. Not a crash: the current code is only extremely slow. A real fix needs a disk cache or a coarser geometry path in the shared BIM code.

## Files in the ticket

`render/preview_any.py` (`<script-path> <Scene> <Output> <lang> [patch-module]`), `render/previews-bim-opt.sh`, `render/previews-list.sh`, `render/bim-opt.list`, `render/opt.list`, `render/gen_list.py`, `render/status_bim_opt.py`, `render/dump_any.py`, `render/dump_geometry.py`, `render/sample_geometry.py`, `render/time_geometry.py`, `render/bim_fast.py`, `render/fallback_delivery.py`. Logs under `🗑️generated` (delete at ticket close).

## Follow-up: rendering the BIM previews from current code

- `render/bim_fast.py` now caches `building_geometry` in `🗑️generated/bim-geometry.pkl` (key: sha256 of every imported project module plus sha256 of the model json), and `render/preview_many.py` renders the scenes sequentially in one process and replaces a fallback only after a scene completes.
- Result: **the current shared code fails.** The first scene (BIM_information) ran the exact geometry for about 3.5 h (start about 21:30, end 00:56) and then raised `ValueError: Finish reserve RES-FIN-ROOF-GRAVEL-SL-R-705 is not fully occupied by its declared physical owners: exact missing volume Fraction(...) m3` from `_finish_reservations` (`🧱️model/🐍️.py` line 2268, roof slab SL-R). The model file was unchanged since 2026-10-30 11:12 and no BIM source changed during the run, so this is not an edit race.
- Nothing was cached, and no fallback was overwritten. All six BIM previews stay "fallback: older delivery". The earlier patched timing run only passed because `bim_fast` then skipped `_finish_reservations`.
- A fix belongs in the shared BIM code (the declared roof gravel owners of SL-R no longer cover its finish reserve) or in the roof model data. Once that is fixed, rerun `python render/preview_many.py bim_fast en render/bim.list`.
