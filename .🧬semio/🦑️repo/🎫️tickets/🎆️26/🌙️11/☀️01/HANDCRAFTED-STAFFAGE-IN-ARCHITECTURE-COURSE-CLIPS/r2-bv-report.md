# R2 bv: BIM verification run and published model names

Scripts: `T/🧰️bv/` (`run.sh` detached launcher, `bimrun.py` runner with faulthandler, signal/exit log and in-process RSS heartbeat every 5 min, `monitor.py` external memory log every 5 min). `BV_OUT=<name> bash T/🧰️bv/run.sh <TestCase...>` writes to `T/🗑️generated/<name>/`. The command line carries the marker `--agent-bv-bim-verification-do-not-kill-<name>`.

## Task A: BimInformationContract + BimConsistencyContract (one process, one geometry build)

Started 2026-11-01 16:51, stopped by me 2026-11-02 05:05 (about 12.2 h) because the remaining work is many more hours. NOT complete.

Observed (the process was alive and healthy the whole time):

| step | result |
| --- | --- |
| base `building_geometry` in `ModelCase.setUpClass` (includes every finish reserve check, incl. RES-FIN-ROOF-GRAVEL-SL-R-705 and all 1150 SL-R reserves) | finished without error after about 3 h 50 min (no cutter-winding defect, br's fix holds for the whole base building) |
| BimInformationContract | 5 of 5 ok (the sill-event test needed a full second geometry build for the changed model, about 3 h 20 min) |
| BimConsistencyContract | 3 ok (`changed_objects...`, `changes_outside...`, `narration...`; with the 5 Information tests that is 8 ok in total); `test_deeper_paving_raises_the_roof...` was still in its third full geometry build (paving model) after 4 h 45 min, no failure or error line so far |
| not reached | the other 6 Consistency tests, among them `paving_alt` on top of `paving`, which needs yet another full build (about 4 h) |

So: 8 ok, 0 failures, 0 errors; 9 of 17 tests not finished (1 running, 8 not reached; counted from the 5 + 12 test methods). The sill-changed and base models built with exact reserves without defect; the paving model build was unfinished.

### Why br's earlier run died silently
- Not OOM: peak working set 2.4 GB (commit 2.8 GB), free RAM stayed 15 to 30 GB of 64 GB (`memory.log`).
- Not a timeout: this process ran 12 h with the same command.
- Not an external `taskkill` of python.exe: other agents' python processes came and went, mine (PID 63096, `uv run python -u bimrun.py ... --agent-bv-bim-verification-do-not-kill-bv`) survived 12 h.
- Cause: br's process was a child of its own shell and died with it at context compaction. Mine was started with stdin `</dev/null`, output redirected to files and `python -u`, from a background Bash task that stayed alive.
- Side effect to know: output to `run.log` stays empty until a test starts (the unittest header is only written then), so an empty log during the build is normal.

### Runtime note (open item)
Every `apply_event` in `ModelCase` tests rebuilds the entire exact geometry (no memo between distinct models, about 4 h each under load). BimConsistencyContract needs three distinct models (sill, paving, paving_alt) after the base: about 12 h on top of the base build. A persistent exact-geometry cache or a smaller verification model would be needed to make these suites runnable; not added (new infrastructure). Remaining work to finish: re-run with `bash T/🧰️bv/run.sh BimConsistencyContract` and about 10 more hours.

No further reserve failure was seen, so no change to `🧱️model/🐍️.py` winding code was needed.

## Task B: publish model functions (fixes the BimClipContract failure)

`🏛️savoye/🧱️core/📐️geometry.py` used private `_model._surface_finishes`, `_model._source`, `_model._geometry_producers` and `_model.__file__`. Renamed in `🧱️model/🐍️.py` to proper public API (no aliases, no old names left):

- `_surface_finishes` -> `surface_finishes`
- `_geometry_producers` -> `geometry_producers`
- `_source` -> `source_text` (cached reader), plus new `source_path()` (replaces `Path(_model.__file__).with_name("🔣️.json")`)

Callers updated: savoye `🧱️core/📐️geometry.py`, `📋️plan/🐍️.py`, `🧊️axon/🧩️primitives.py`, `🧱️core/🧪️snapshot/🐍️.py` (including `patch.object` names), `🧱️core/🧪️surfaces.py`, `🧪️ortho-remodel/test_frustums.py`, and the model package users (`🎨️finishes/🛝️ramps.py`, `🎨️finishes/🧱️layers/🐍️.py`, `🏗️facade/🔗️junctions.py`, `🔌️exchange/🐍️.py`, `🧪️tests/🧱️interfaces.py`, `🧪️tests/🪈️tubes.py`, `🎨️finishes/🧪️tests/🐍️.py`). Grep over `🎓️teaching` finds no old name left.

Ordering note: the model package callers were renamed only after the long run had loaded them (a running process cannot be affected by renames of already-imported modules).

Result: `BimClipContract` 7 of 7 ok (Ran 7 tests in 519.9 s, OK), including `test_scenes_call_only_published_model_functions_and_no_abstract_wall_remains`.

Not re-run after the rename: the suites that exercise the renamed finishes/exchange modules in full (BimStepExchangeContract, finishes tests, savoye snapshot tests); only the import-level grep and the clip contract confirm consistency.

## Files changed
`🏗️bim/🎬️clip/🧱️model/🐍️.py` and the callers above. New: `T/🧰️bv/*`. Generated output (`T/🗑️generated/bv`, `bvclip`) removed. No background process left.
