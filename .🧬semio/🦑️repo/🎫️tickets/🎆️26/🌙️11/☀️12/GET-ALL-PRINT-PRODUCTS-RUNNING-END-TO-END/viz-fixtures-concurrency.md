# Bounded concurrency for test-viz-fixtures (revision 2, not yet proven by a full run)

## Diagnosis of the cancelled run (fx-par.log)
1. `--output=static` withholds all task output until the task ends. `repo run --help` lists `--output=<stream|stream-without-prefixes|static>`. The log has 0 `"owner":"print…"` progress lines even from the completed prerequisite tasks, and the SIGINT discarded the task's buffer. So the lack of `gallery` lines proves nothing about whether the loop ran. Use `--output=stream` to see per-compile progress live.
2. The pool in revision 1 was effectively 1. It was sized with `compileUnitsForHostV1`, which limits to `max(grantedJobs, grantedMB/unitMB)`. A lone Nx task is granted 1 job and about 1638 MB, which gives 1 unit. The gate sizes `preferredJobs` from the task count, so a single target cannot ask for more jobs, and the 1-job grant is the real reason.
3. The 4.8 GB observed against 1638 MB reserved is the sequential path (tectonic plus a PDF.js measurement worker per variant). It is undercounted by the gate and not caused by this change. I did not measure the split.

## Changes (revision 2)
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/⏱️schedule/🟦️.ts` (new):
  - `printGalleryCapacityRequestV1(host, items)` plans with the host's grant facts nulled and keeps cores, free memory, pressure and the item count. It requests `available` capacity with minimum 1, so it starts as soon as one core is free.
  - `measurePrintItemsInOrderV1` runs items on at most `workers` workers, returns results in item order, calls `completed` per item and honours the abort signal.
- `⏱️schedule/🧪️tests/🟦️.ts` (new): 3 light tests pass (`repo tools exec --light bun -- test …`): the capacity request ignores the lone-task grant, concurrency is bounded, ordering is deterministic and cancellation is honoured.
- `…/📏️measurement/🟦️.ts`: `writePrintGalleryEvidence` acquires the cores through the existing mechanism `withCompilerCapacityScopeV1` (lease directory `<workspace>/.cache/b/l/m/data`, the same one other heavy builds use) and runs `measurePrintItemsInOrderV1` on `facts.parallelism`. Progress goes to `console.log` (`[print] gallery i/320 section/theme/language Ns`) and to `control.publish`. The evidence is assembled in matrix order, so it matches the sequential result.
- `📦️artifacts/🟦️.ts`: `runPrintWorkersV1` accepts an optional `workers` count, still capped by the authored maximum of 4. `📦️artifacts/🧪️tests/🟦️.ts` has a test for it (8 pass).
- `command-boundaries.json`: unchanged. The new files are not bundled by a listed command entry.

## Not done
- Resumability: skipped. A resumed row needs an input identity per variant (source, bundle, fonts), and the only durable write path is whole-directory artifact publication, which I cannot exercise without a real run. Proposal: per-variant evidence keyed by the sha256 of the section source and toolchain identity, staged in batches.
- No real run since revision 2, as instructed. A direct `repo ./📜️script.ts test viz fixtures viz-1` is refused ("Original parent Script process invocation required"), so a proof needs the Nx target. Cheapest proof: `repo run @semio-tech/print:test-viz-fixtures --output=stream --skip-remote-cache -- viz-1` (4 compiles; check the "granted compiler core(s)" line and the 1/4..4/4 lines).
- On the current memory-bound host the memory fact alone may still yield 1 core (about 5 GB free, minus the 2 GB reserve, over the 2048 MB unit). Parallelism will only appear with free memory.
- The `build-viz` completion loop (header reads only) and `verifyPrintPdfs` (shared `painted` state, `painted.slice(-leaves.length)`) were left sequential; the latter would race.
