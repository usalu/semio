# Test Run Report: Grasshopper, Excel and Nakagin Clips

Run date: 2026-10-28 to 2026-10-29 (local time, Windows 11). Tests were run only; no source files were edited.
Logs and exit codes: `🗑️generated/tests/` in this ticket. Temp dirs: `🗑️generated/tests/tmp/<suite>`.
Interpreter: `.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe` with `PYTHONIOENCODING=utf-8`, `SEMIO_REPO_BINARY` set to the repo-shim.cmd of ticket CONCRETE-OPTIMIZATION-CLIP-EXAMPLES.

| Suite | Command (relative to `🎓️teaching/🏛️architecture`) | Tests run | Result | Exit | Log |
|---|---|---|---|---|---|
| Nakagin, Python | `🏢️nakagin/🧪️tests/🐍️.py` | 29 | PASS | 0 | `nakagin-py.log` |
| Nakagin, Node | `node 🏢️nakagin/🧪️tests/🟨️.cjs` | n/a (script oracle, no test count printed) | PASS | 0 | `nakagin-cjs.log` |
| Grasshopper clip, Python | `🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py` | 57 | PASS | 0 | `g-py.log` |
| Grasshopper clip, Node | `node .../🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs` | n/a (script oracle, no test count printed) | PASS | 0 | `g-cjs.log` |
| Grasshopper registry | `.../🧪️tests/📇️registry/🐍️.py` | 5 | PASS | 0 | `g-registry.log` |
| Grasshopper presentation | `.../🧪️tests/🎭️presentation/🐍️.py` | 12 | PASS | 0 | `g-presentation.log` |
| Excel clip, Python | `📊️excel/🎬️clip/🧪️tests/🐍️.py` | 83 | PASS | 0 | `x-py.log` |
| Excel clip, Node | `node .../📊️excel/🎬️clip/🧪️tests/🟨️.cjs` | n/a (script oracle, no test count printed) | PASS | 0 | `x-cjs.log` |
| Excel registry | `.../🧪️tests/📇️registry/🐍️.py` | 5 | PASS | 0 | `x-registry.log` |
| Excel presentation | `.../🧪️tests/🎭️presentation/🐍️.py` | see below | see below | see below | `x-presentation.log` |

## Failures

None among the nine completed suites. No tracebacks.

## Open item: Excel presentation suite

The Excel presentation suite (pid 46006, started 23:08 on 2026-10-28) printed nothing after 23:09 and stalled at `test_native_lifecycle`, which renders each scene with Cairo in both languages. Its exit code is 143 (SIGTERM), so the process was terminated externally at about 01:2x on 2026-10-29. This was not done by this run, and the result is not a pass or a fail. The Grasshopper presentation suite took about 50 minutes for 12 tests, so the stall was well beyond normal. Scene files were not modified during this run.

Next step: rerun `📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py` alone, then update this report.

## Summary

1. Nine of ten suites passed with exit code 0: Nakagin (Python 29, Node), Grasshopper (Python 57, Node, registry 5, presentation 12), Excel (Python 83, Node, registry 5).
2. The Excel presentation suite had not finished when this report was written, so its result is unknown.
3. No failures or tracebacks in the completed suites.
4. Excel presentation's log and exit file should be checked once `x-presentation.exit` appears in `🗑️generated/tests/`.
5. Other python processes on the machine (for example pids 39810, 40982, 41927) belong to other users and were not touched.

## Re-run after Panels removal

Both presentation suites were re-run after the Panels removal. Both passed. No failures, so no scene mtimes are relevant.

| Suite | Tests | Result | Exit | Log |
|---|---|---|---|---|
| Excel presentation (`📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py`) | 15 | OK, 4387.9 s | 0 | `🗑️generated/tests/excel-presentation.log` |
| Grasshopper presentation (`🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py`) | 12 | OK, 2393.8 s | 0 | `🗑️generated/tests/grasshopper-presentation.log` |

Excel: SceneContract (imports resolve, literal dependency closure, narration title and steps, native lifecycle, series folders registered) and WorkbookToolkit (addresses and ranges, engine hooks, formula glyphs, localized strings, long-formula wrap, punctuation baseline, reference tokens and colours, selection tint, Nakagin workbook window, window in both languages): all ok.

Grasshopper: CanvasWidgets (Bezier S-curve, every engine view builds, states and panels from the solution, tree notation, widget zoo) and GrasshopperPresentationContract (declared chapters, dispatch and step keys, imports resolve, explicit language, bilingual unique narration, native lifecycle, provenance files exist): all ok.

Environment: run with PYTHONUTF8=1, TMP and TEMP in `🗑️generated/tests/tmp`, and SEMIO_REPO_BINARY pointing at `repo-shim.cmd` in the CONCRETE-OPTIMIZATION-CLIP-EXAMPLES ticket. The Grasshopper run was started a second time because the first launch never produced a process. Both runs were slowed by other agents' render processes on the same machine.

## Final run (all scenes final)

Logs: `🗑️generated/tests/final/` (one `.log`, `.exit`, `.dur` per suite). All 10 suites exited 0.

| Suite | Tests | Result | Duration |
|---|---|---|---|
| nakagin python (`🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py`) | 29 | pass | 0.3 s (2 s wall) |
| nakagin node (`🟨️.cjs`) | checks printed, exit 0 | pass | 1 s |
| grasshopper python (`🧪️tests/🐍️.py`) | 57 | pass | 0.9 s |
| grasshopper node (`🟨️.cjs`) | checks printed, exit 0 | pass | 1 s |
| grasshopper registry (`📇️registry/🐍️.py`) | 5 | pass | 2.2 s |
| grasshopper presentation (`🎭️presentation/🐍️.py -v`) | 12 | pass | 1149 s |
| excel python (`🧪️tests/🐍️.py`) | 84 | pass | 34 s |
| excel node (`🟨️.cjs`) | checks printed, exit 0 | pass | 1 s |
| excel registry (`📇️registry/🐍️.py`) | 5 | pass | 3.6 s |
| excel presentation (`🎭️presentation/🐍️.py -v`) | 16 | pass | 2340 s |

No failures, so no tracebacks to report. Run environment: `PYTHONUTF8=1`, `TMP`/`TEMP` in `🗑️generated/tests/tmp`, `SEMIO_REPO_BINARY` set to the `repo-shim.cmd` of ticket `CONCRETE-OPTIMIZATION-CLIP-EXAMPLES`. Max 3 suites concurrently, each under `timeout 6000`.
