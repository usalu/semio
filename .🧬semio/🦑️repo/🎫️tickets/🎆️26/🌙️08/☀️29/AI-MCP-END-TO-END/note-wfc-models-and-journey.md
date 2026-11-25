# Note and wfc models and client journey (2026-11-25), status: blocked on image storage

## Built
Nothing. No component model was staged: `buildComponentClosureV1` (via `client-journey-driver.ts`) was refused by the native-build storage gate before compiling its first unit.

## Run 1 (08:51, `🗑️generated/journey-run1.log`)
- Descriptor emitter rebuilt, then planning: 1 pass, 2 components (note 926 units: 888 library, 29 build script, 8 proc-macro, 1 component; wfc 969 units).
- Batch `prototype-00-b1` (wfc, shares the note closure) predicted 5664 MiB, admitted. Batch `prototype-00-b0` (note) predicted 17690 MiB + 2048 MiB reserve = 19738 MiB, refused: 6986 MiB remained after 5664 MiB charged to the running pass; reclaim ran and the image has no grow port. 0 units compiled. Journey steps not reached.

## Prediction audit (storage-gate, `componentBatchesV1`)
- 17696 MiB = 888 x 16 + 29 x 32 + 8 x 64 + 2048, exactly the declared per-kind upper bounds (`Policy.unitFootprintBoundsBytes`).
- Reason: `components/build-telemetry/units.jsonl` is empty and no `storage-growth.jsonl` ledger exists, so every unit is unmeasured. The gate learns after builds (ledger needs 2 rows, quantile 0.9), but the first build must pass on the bounds.
- No ground truth for real per-unit output exists on this machine (earlier deps reclaimed), so no estimator change was made. After the first admitted build the ledger and units history replace the bounds.
- Image grow: `growStorage` is never passed to the component driver (only gate tests do), hence "no image grow is available here". The declared repair is `repo doctor --fix` (grow images below the declared need; held while any opener runs). `repo doctor --only=storage`: healthy, no findings. Not run with `--fix`.
- `repo cache gc --dry-run`: would free 20 KiB (empty scratch, a retired root). A collector (pid 98581) already runs continuously. The image's 24 GiB in `u/l` lanes is live: `q0c9bae` (semio-os-mcp) 18 GiB, `qda3ae2` (semio-plugin-describe) 3.5 GiB, `qa7b96d` 1.5 GiB.

## What the gate needs
At least 19738 MiB free on the image (`/Users/ueli/Documents/semio/.cache/b/r/m`, 40 GiB, 66% used, 14 GiB free at 08:55, 9 GiB at last poll) after charges, for note alone; then 5664 MiB more for wfc. Ways: other sessions finish their `cargo check` runs and lanes go idle for the collector, or the coordinator frees a lane, or a sanctioned grow of the image.

## Retry
`SEMIO_OS_MCP_BIN=$PWD/.cache/mcp-fallback-bin/semio-os-mcp <bun> client-journey-driver.ts --note-only`, then `--skip-build`. Note: the old binary overflows the stack on every `action_prepare` (coordinator has a fix pending), so expected failures start there.

## Fixes
None to source.

## Run 2 and 3 (09:14 and 09:19, `--note-only`, binary `semio-os-mcp-new`)
- Run 2: source-closure reader refused `process.source.unresolved-module {"module":"dbg"}` in `🧳️original/🦀️.rs` (another machine's gitignored debug instrumentation; coordinator then stubbed the generated file on this machine).
- Run 3: source admission passed (509 units). The storage gate refused `prototype-00` (note alone): 17696 MiB predicted (17693 MiB still to write after a 3 MiB credit) plus the 2048 MiB reserve; image `.cache/b/r/m` had 12 GiB free (40 GiB, 70% used) and no image grow is available. 0 units compiled.
