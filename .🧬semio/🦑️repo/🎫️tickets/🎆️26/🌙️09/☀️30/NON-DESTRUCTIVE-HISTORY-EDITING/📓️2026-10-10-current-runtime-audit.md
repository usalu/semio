# Current Runtime Audit — 2026-10-10

Read-only source audit; no compiler or runtime tests executed. Historical October 9 reports do not certify this current tree.

## Receipt Boundary

Current `🧵️job/🦀️.rs:897–945` StepContext borrows the caller’s actual `RetainedCloneProgress`. `retained_grant()` subtracts items, copy, capacity and release independently; depth remains immutable. `consume_retained()` adds actual progress to the external recipient even when authority was exceeded, then returns a refusal. There is no identity/deduplication mechanism: a producer already using this same context must not have its output receipt consumed again. Passing a cumulative progress value instead of a per-call delta likewise double-debits. Errors carrying the same already-collected progress need forwarding, not recollection. Overflow does not complete the normal addition and needs genuine error custody.

Current Reactor `🔄️turn/🦀️.rs:751–763,1559` snapshots original progress and returns its actual per-turn delta through `budget.retained.return_original`. Extension close intersects mounted independent policy with remaining context and consumes its actual receipt once (1034–1040). This licenses no runtime claim.

Current Plugin exchange signature (44375) now requires original borrowed context, but construction branch (44381) still calls `advance_typed_operation_output` without that context. Its publication helper (43619) calls `advance_typed_operation_publication(identity)` and takes constructor/preparation/history/tool receipts, checking them against their returned producer grants only. It does not debit the exchange context. Continuation/publication wrappers at 43700,43866,44080,45161,45172 remain this frontier. Required fix: forward context through the exact actual producers, supply remaining full grant, preserve original receipt/error/output custody; collect each receipt exactly once at an explicit boundary. Root is already assigned these spans.

Concrete visible API gap: dispatch fixture `🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs:384` still calls `plugin_exchange(runtime,7,input)` although current API requires identity and context. This is source evidence of a stale caller, not a compiler census.

## Host Driver

New policy owner `🖥️host/🔔️wake/📃️policy/🦀️.rs` declares required independent admission/drive/wake/operation policies. Treasury retains finite original operation currency, one held epoch, rejects forged/duplicate returns and does not renew. `begin_turn` intersects wake policy with remaining operation grant; debit preserves depth. Current recursive Host search finds treasury construction only in its native test, with public reexport in Host. Actual Host production uses `GuestRelayWakeAuthority.drive_policy` and `.policy` (5095,5941), so the finite treasury is not yet visibly installed in this production owner. Required driver cannot be replaced by default/CPU-derived/demand-derived authority. UI owns this span.

## Shortest Honest Upper Path

Use existing original Plugin native test package plus original continuation/context laws after caller ports. Avoid whole Host/Wasmtime build as first proof: previous Host law timed out while compiling; current source does not establish a completed Host runtime. Next prove Plugin original context and once-only receipts, then Reactor/Host real driver ingress, then smallest existing editor acceptance cohort, then full strict matrix and live renderer journeys. Core must finish actual retained decode/apply/replay entrance before archive/history completion can be credited.

## Actual Commands

Full strict matrix runner is `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- bun <ticket>/native-matrix/📜️script.ts run all all`. It partitions ungated/assembly owners, delegates execute-group through repository-owned Cargo/Nextest runner, and requires three distinct PASS names for every direct registration plus child law names. `execute-group ... named` is only the inner runner; `run` performs the strict receipts. This audit did not rerun census, so 148/444 remains the recorded obligation, not a refreshed count.

Registered live commands are `bun nx run @semio-tech/framework-os-dev:time-travel -- --serve http://127.0.0.1:6012/ --renderer react --locales en,de --chords en,de --out <ticket>/🗑️generated/ui-live/full-react-current` and the same command for `http://127.0.0.1:6112/ --renderer wgpu` and `full-wgpu-current`. Ticket dashboard routes: `bun nx run @semio-tech/repo-dashboard-rs:launch -- run ticket:26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING/test-history-react-ende-live` (or `test-history-wgpu-ende-live`). The EN/DE launch entries call these same canonical controls. Neither route was executed here; the recorded 24-live obligation remains uncredited.

Ticket has no root `project.json`/`📜️script.ts`; commands source is `🎮️commands.json`, projected into `.vscode/launch.json`. Framework Rust package router registers `test`, source controls and history controls in its own `📜️script.ts`. Keep execution through canonical Bun/Nx routes and repository-owned test runner.
