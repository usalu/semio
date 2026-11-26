# XLSX end-to-end status (2026-11-26 14:45)

## Progress
| Area | State |
|---|---|
| Oracles, fixtures, base mutation tests, claims test, natural decode, Strict fixture | done and verified (see claude-status-2026-11-25.md) |
| German decimal fix | native tests pass; browser confirmation pending |
| Pane journey (6273) with the last good wasm (23:48 yesterday) | 8 of 9 steps pass |
| Stdio component over wasmparser's 1,000,000-function limit | runtime-host split done; it builds at 978k (claude-stdio-component-split.md) |
| Runtime cleanup pump faulting every XLSX open (regression from another agent's in-flight retirement change) | fixed in two rounds; 16 + 3 laws pass (claude-cleanup-pump-stall.md) |
| Wasm rebuild with both fixes | stdio built 14:38; stdio-office compiling, ETA about 14:55 |

## Expected finish
- **About 15:10:** pane journey in the browser (en, de, Transitional, Strict, German decimal), if the open succeeds with the new wasm.
- **About 16:00–16:30:** full S lane on 6070 (en, de) and its verdict table (claude-full-s-host.md).
- **About 17:00–18:00:** native reruns (base_io_laws, paged routes, paged backtrace), Home viewer failures, then cleanup and ticket close.

Each unexpected foreign compile break costs one rebuild, about 40 min.

## Problems faced and fixed today
- **Stdio function limit:** stdio was over 1M functions. Split: a definition-only core, with stdio-semio and stdio-bim as runtime hosts.
- **Cleanup-pump false stalls:**
  - Round 1: the stall verdict now needs real time, not just a count.
  - Round 2: a pump reading taken after the turn's budget is spent no longer counts as a stall.
- **Foreign compile breaks that blocked the wasm build, each fixed minimally:**
  - `value-ordered` missing the `Entry`/`Node` import;
  - `cause()` was gated to tests only;
  - store test-support had a `1 << 32` overflow on wasm32 and an HRTB lifetime error.
- **Disk at the guard's admission line:** freed about 7 GB of stale build intermediates, and cancelled a lower-priority attribution build to keep the critical path moving.

## Anticipated
- The new wasm may expose the next defect in the open path. The round-2 cause is inferred from code; there is no native xlsx import harness.
- Other agents keep landing in-flight edits, so more compile breaks are likely.
- Stdio has only 22k functions of headroom. A move of the native codecs/text apps to get under 700k is pending, after the browser lanes.
- Stdio and stdio-semio descriptors are stale; the native emission route fails. They need regenerating from the wasm.
- Kernel depends on store test-support at runtime; a fix plan is in claude-test-support-wasm.md.
- Disk hovers at 26–30 GiB free.

## Resource use
- **CPU and memory:** the host is deliberately limited to one heavy job at a time (claude-resource-guard.sh), because parallel builds caused the kernel panic on 2026-11-25. The wasm link is serial by nature: load is about 7 and memory is 80% free, so this is not saturation by design. Crash safety beats speed.
- **Agents:**
  - The split agent builds the wasm and owns the 700k move.
  - The browser agent runs the journeys and the S lane.
  - The cleanup-pump agent and the test-support agent work on source fixes.
- **Parallel work:** source fixes, analysis and tests are written while the build runs; only heavy jobs are serialized.
