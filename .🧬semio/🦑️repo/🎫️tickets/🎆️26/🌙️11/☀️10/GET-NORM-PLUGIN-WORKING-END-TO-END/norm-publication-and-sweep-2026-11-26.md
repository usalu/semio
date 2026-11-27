# Norm publication and strict sweep, 2026-11-26 (blocked on host capacity)

## Result

No component was freshly published and the strict 15-route sweep did not run. No source change was needed or made: every stop was host capacity or warden, not a compile or route failure.

## Order (coordinator)

Base, energy, eurocode, eurocode-structural, then materials last, after `en1995-unit-ceiling-2026-11-26.md` shows the EN 1995 carve done. Base was attempted first.

## Evidence for Base (`🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust/Cargo.toml`)

1. Descriptor emitter refresh (about 764 units) completed under the 2,048 MiB controller grant.
2. Component cargo pass was SIGTERM-cancelled three times about 50 s in with 0 units compiled (`cargo was ended by SIGTERM (exit 143)`; reports `.cache/b/r/m/components/runs/wiFqaDA`, `wx9Jwks`, `wiYXkbL`). The warden status line names the cause: `cancelled cargo pid ... memory pressure is critical`, swap 7.2-7.8 GiB.
3. A later run survived planning, then refused: `ParentComponentMemoryRequiredErrorV1: Component build needs 5807 MiB for the local pass, but the parent granted 2048 MiB and no fleet placement is admissible` (fleet does not answer; run `wB6GoFQ`). The plain `native-exec.ts` route therefore cannot publish any component on this host. The controller must be wrapped in an outer grant.
4. Retry loop `publish-component.sh` now wraps the same command in `repo capacity run --memory=5900 --cpu=1` (retained grant, same as the Energy recipe in the admission note). It has waited more than 830 s: available memory stays at 3.3-4.9 GiB, while 5,900 MiB plus the 2,048 MiB floor needs about 8 GiB. Other agents hold 2,663 MiB EN 1990 replay grants and graph-plan grants continuously, so the host is not a single heavy lane.

## State left running

`publish-component.sh base ... 5900` (up to 20 attempts) is still queued. Log: `🗑️generated/pub-base.log`, attempts in `🗑️generated/pub-base.attempts`. It does not bypass the gate or the warden. Nothing was killed: no orphaned process was identified (fseventsd at 3.2 GiB is a system daemon, the fleet entry, Claude app and the 2,663 MiB grants belong to live work).

## Blocker

Host-wide admission: a 5.9 GiB retained grant cannot be offered while other agents' EN 1990 replay, graph-plan and norm-suites lanes run. Needs those lanes drained (or the host RAM headroom to reach about 8 GiB). Same applies to the other four components (forecasts 4.1-4.8 GiB final plus passes) and the 6,144 MiB browser sweep.

## Script

`publish-component.sh <name> <manifest> [MiB]` in this ticket folder.

## Follow-up: where the 5,807 MiB comes from (coordinator asked for a policy fix)

Computed in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧵️job-slots/🟦️.ts` (`runPassesInJobSlotsV1.attempt`, `ParentComponentMemoryV1.required`), fed by `componentPassPeakMegabytesV1`, called from `component-pass/🟦️.ts:1285` with `units = tokens = 1`.

- 4,766 MiB: the forecast of the ONE final wasm component link, `predictPeakBytesV1` over the recorded per-crate link peaks (headroom times p95; the eight Base samples). Not a sum and not a whole-tree estimate; the other cargo units are admitted one by one by the per-unit broker and are not in this number.
- 512 MiB: fixed local footprint (`componentFleetChargeV1`).
- 529 MiB: controller baseline (process RSS).

So the requirement is already "largest single unit plus fixed controller overhead", and the refusal is already "one unit alone cannot fit the grant". A 2,048 MiB grant cannot hold a 4.7 GiB link, however units are serialised. Letting it run in the 2,048 MiB grant would spend about 2.7 GiB that the ledger never reserved, which is exactly what the warden and gate exist to prevent, so I did not change the policy.

The existing escape hatch is grant growth: the pass `declare`s its need (`declareHostCapacityNeedV1`) and the holder grows the grant. It waited about 1,169 s, but growth needs host available memory of at least 2,048 MiB floor plus 3,759 MiB, about 7.8 GiB. Observed available memory is 3.3-4.9 GiB.

Fleet is not an alternative right now: all 12 workers report `refusing` (`no-headroom`, load 21.9 on 12 cores; others busy), and offloaded slots for a 6,322 MiB final read zero.

Options needing a decision (none applied):
1. Drain the other agents' 2,663 MiB replay, graph-plan and norm-suites lanes and restart fseventsd (root) so about 8 GiB is available.
2. Lower the real link peak (smaller Base component, or a leaner link profile). This changes the measured forecast legitimately.
3. Lower the 2,048 MiB desktop floor for this lane. This is a safety decision for the owner, not an agent change.

## Follow-up 2: lowering the real final-link peak

Evidence (`.cache/b/r/m/components/build-telemetry/units.jsonl`, wasm32-wasip2 prototype `cdylib` rows of the five norm components):
- Rows split by CPU/wall ratio. 36 rows run at 0.57-0.99 (serial) with peaks 1.8-4.8 GiB; the 4 newest rows run at 4.5-6.6 CPUs with peaks 2.9, 3.8, 3.9 and 4.8 GiB. Base: serial 2.0-3.0 GiB (median about 2.5), threaded 4,774 MiB.
- The Base forecast of 4,766 MiB is exactly `1.15 x R-7 p95` of its last eight rows, and that p95 is the single threaded 4,774 MiB row (without it about 3.3 GiB). Keyed to Base's own crate, not Materials.
- rustc already gets `-Zthreads=1` and `-Z no-parallel-backend` for finals. The unaccounted threads are `wasm-ld`, which threads its section passes by default and nothing in the repo limited it (`--threads` appears nowhere in the scheduler, `.cargo/config.toml` or the compiler ingress).
- Debuginfo is already 0 in the prototype profile; output is 170-230 MiB wasm.

Fixes (both test-driven, language-agnostic corpus plus independent Python oracle):
1. `🎟️admission/🦀️.rs` (`compiler_parallelism`, `main`): a `cdylib` final for a `wasm32-*` target now also gets `-C link-arg=--threads=1`. Corpus `🧪️tests/🧵️frontend-threads/🔣️.json` gains four cases, oracle `🐍️.py` implements the rule independently, `🟦️.ts` drives the compiled scheduler binary with a wasm response file. Corpus entries `one-token` and `other-z-flags-kept` were stale against the Rust rule (single token already appends `no-parallel-backend`) and were regenerated from the oracle. `repo test-file` on the suite: 4 pass, 82 expects; the Rust unit table was extended in step but not run here.
2. `🧩️component-pass/🧵️job-slots/🟦️.ts` plus schema `Policy.serialFinalCpuRatio = 1.5`: `serialFinalHistoryV1` makes `componentPassPeakMegabytesV1` forecast finals only from observations that were serial (cpu <= 1.5 x wall), falling back to all rows when none is. Oracle kind `serial` in `🧪️tests/🧵️local/🐍️oracle/🐍️.py`, test `a final is forecast from the serial runs...`. All six job-slots test files pass (local 11 tests, 10,994 expects; others 41 tests).

Expected effect: Base forecast about 3.3 GiB instead of 4.77 GiB (local pass about 4.4 GiB incl. footprint and controller, from 5.8 GiB); the real peak should drop toward the 2.0-3.0 GiB serial band because the link no longer threads. Not yet measured: the 2 GiB serial measurement run was killed by the warden at 1,016 MiB (critical swap, reference cc569107) and later runs never got admission, so the `/usr/bin/time -l` link measurement and the learned row are still missing.

Still running: `publish-component.sh base <manifest> 4700` waits for a 4.7 GiB grant (spare 0.3-1.2 GiB). Even the lowered forecast needs about 6.7 GiB available above the floor, so the host must drain or the serial peak must prove smaller. Energy, Eurocode, Structural, Materials and the sweep are therefore not started.

## Follow-up 3: Base link refusal at 4,700 MiB

- The serial-only forecast WAS in effect: the driver logged `it needs 3409 MB` for the Base pass (was 4,766). The 4,846 MiB requirement is 3,409 forecast + 512 pass footprint + 925 controller RSS (`process.memoryUsage().rss` baseline), so the controller baseline, not the forecast, was the remaining overhead.
- `publish-component.sh` now sizes its outer grant from the driver: it starts at the given MiB, and a refusal `Component build needs N MiB for the local pass` raises the next grant to N + 400 MiB and retries; warden cancellations and admission timeouts retry unchanged. Logs go to the session scratchpad (`SEMIO_PUBLISH_LOGS`), because log files in the ticket's generated folder are deleted by other agents' cleanup (two of my logs vanished).
- Base rerun at 5,250 MiB has waited about 2,500 s in the capacity queue (spare 0.8-3.7 GiB against 5.25 GiB, no other grants at times, so the shortfall is unattributed host memory such as fseventsd). It is still queued in the background. No other component or the sweep has run.

## Follow-up 4: controller cap and chain

- The component driver controller already runs under the collector cap: `repositoryCommandKernelV1` classifies `repo ./...driver-stages/📜️script.ts component` as `runtime`, which `repositoryCommandMemoryCappedV1` caps (it reads the graph and is not a free verb), and `native-exec.ts` starts it as `repo <script>`, so the front door applies `graphChildEnvironmentV1`. Two cases pinning this were added to `...⌨️command/🧭️selection/🧪️tests/🔣️.json` (checked by the SQLite model; 1 test, 26 expects). The 925 MiB baseline in the 4,846 MiB refusal predates or ignores the cap; the next admitted run shows the effect.
- `publish-chain.sh` (this ticket folder) runs the single lane unattended: base (after the running base loop exits), energy, eurocode, structural, materials, then `verify-standards-publication.ts` (the strict 15-route sweep). It retries on admission timeouts, warden cancellations and source-boundary refusals (carve in flight), grows the grant from the driver's declared requirement, and logs to the scratchpad (`chain.attempts`, `chain-<step>.log`). Started; nothing admitted yet.

## Follow-up 5: restart at 3,400 MiB

`publish-chain.sh` now starts every component at 3,400 MiB and, after a refusal naming N, sets the next grant to max(N + 200, current spare above the floor). Restarted at 21:30; the Base step was admitted after about 10 min of queue, refreshed the descriptor emitter, and the component pass this time chose a fleet slot (fleet answers again: 512 MB local footprint, final placed on a worker). Cargo was then SIGTERM-cancelled by the warden at 9.2 GiB peak swap (run `wmCCFKU`, 0 units) at 21:42; the chain treats that as retryable and has resumed queueing. State: `chain.attempts` and `chain-<step>.log` in the session scratchpad. Nothing published yet.
