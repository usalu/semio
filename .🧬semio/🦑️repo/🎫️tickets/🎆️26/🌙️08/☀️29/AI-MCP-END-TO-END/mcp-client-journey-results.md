# MCP Client Journey Results (2026-11-25)

## Follow-up with the published Note model

The native catalog contains the Note owner and capabilities. The products partition now contains the Note component, descriptor JSON and Pack, prepared actor image and publication receipt. The earlier storage gate explains the first 15/16 run. The retained Note model removes that prerequisite from the current typed-create probe.

The earlier configured-launch trace reached the first scalar Poll progress receipt. Its execution fuel and posted usage then remained constant until the 150-second client operation budget expired. The last trace records 1,087,718,190 operation copy bytes spent, with 962,305,589 posted outside the interpreter in the stage reported as 0. The trace records no acknowledgment grant or outstanding guest hold, so it does not identify what prevented that receipt from settling.

The Poll session now checks the next ordinary drive grant when scalar progress, scalar result, raw-word or fixed-receipt acknowledgment cannot proceed. An outstanding guest loan with an insufficient copy grant yields the existing typed WorkLimit error, including required and available bytes. The shared acknowledgment fixture now includes the scalar progress receipt at one byte below its required stock and checks that refusal keeps the request, loan, epoch and owner intact.

The configured repository client journey currently fails in this sandbox before protocol initialization. Its storage receiver calls `/usr/sbin/sysctl -n kern.bootsessionuuid`, which returns `Operation not permitted`. `mcp-sdk-create-note.ts` uses the installed MCP TypeScript SDK, published native binary and canonical prepared Note model for a narrower direct stdio probe. It reports discovery, the typed create result, snapshot and session liveness. Its two optional arguments select ticket-local operation and driver policies for diagnostic comparisons.

| Direct SDK policy | Typed create | Follow-up |
| --- | --- | --- |
| Checked-in policies | `BUDGET_EXCEEDED` at 151.52 s, Poll wall budget | Snapshot `NOT_FOUND`, ping succeeded |
| Isolated transit copy reserve 1 GiB, same 3 GB operation limit | `BUDGET_EXCEEDED` at 56.20 s, retained items | Snapshot `NOT_FOUND`, ping succeeded |
| Transit 1 GiB copy and 2,097,152 items, same operation limits | `BUDGET_EXCEEDED` at 82.60 s, retained items | Snapshot `NOT_FOUND`, ping succeeded |
| Transit 1.5 GB copy and 12,000,000 items, same operation limits | `BUDGET_EXCEEDED` at 119.00 s, semantic work with cleanup fuel retained | Snapshot `NOT_FOUND`, ping succeeded |
| Same transit, turn fuel 32,768 and drive items 16,384 | `BUDGET_EXCEEDED` at 117.91 s, semantic work with cleanup fuel retained | Snapshot `NOT_FOUND`, ping succeeded |
| Same turn settings, transit work fuel 250,000,000 inside the checked-in 1,000,000,000 total | `BUDGET_EXCEEDED` at 67.75 s, next guest reservation could not be issued beside cleanup and transit work | Snapshot `NOT_FOUND`, ping succeeded |
| 4,000,000,000 operation work, 2,000,000,000 transit work, 1.5 GB and 12,000,000 transit copy and items | `BUDGET_EXCEEDED` at 124.28 s, retained items | Snapshot `NOT_FOUND`, ping succeeded |
| Same work totals, 2 GB and 24,000,000 transit copy and items, turn fuel 262,144 | `BUDGET_EXCEEDED` at 152.12 s, Poll wall budget | Snapshot `NOT_FOUND`, ping succeeded |

Each session initialized as `semio@0.1.0` and listed 28 tools. The copy-only variation reached a later constrained dimension. Of the 48,000,000 issued items, the host had spent 1,483,600, the guest held 46,483,632 and cleanup had reserved 32,768, leaving zero spendable. With the larger copy and item reserves, the host moved past the retained-item refusal and exhausted spendable work fuel. These diagnostics ran against the existing native publication, before rebuilding the acknowledgment change. The Poll TypeScript suite passed all three tests, including the SQLite check for the shared acknowledgment fixture.

The new scalar acknowledgment case initially produced WorkLimit with the original loan intact and then exposed a second fault during cancellation cleanup. Issuing an Infrastructure cleanup loan still set aside the ordinary transit grant after ordinary execution had consumed it, causing `cleanup copy exceeds operation currency`. The cleanup admission now keeps the host cleanup reserve and uses the remaining finite Infrastructure scope. The ordinary loan path retains both reservations. The pinned offline Rust regression passed all three acknowledgment frontiers and their final Source cleanup after that change.

The returned-frame fixture now lets the native interpreter enter a 512-local callee one instruction at a time, then consumes the ordinary capacity stock while that frame is live. The pooled return requires a capacity allocation. The fixture verifies the typed refusal, unchanged guest loan and successful cleanup, and the same guest still returns the expected content under the independent Wasmtime oracle. The complete native Poll suite passed 18 of 18 tests.

## Typed kernel output capsule

The rebuilt optimized gateway with a diagnostic 16 billion physical work budget reached a ready, captured 745-byte Note turn output. The typed create failed in the host's ParserClose phase after receiving that capsule. The kernel encoder publishes an object with a single `Ok` property holding the turn result. The headless MCP receiver attempted to receive the whole JSON object as a bare `TurnResult`.

`KernelPollOutput` now declares the published success envelope and shares its tag with the encoder. MCP receives that type and borrows the embedded `TurnResult`. The actor's separate failure-receipt path still owns cancellation and errors from producing a turn, before the success encoder runs. All three neutral kernel JSON tests passed, including the Ajv and independent SQLite envelope checks. The native decoder fixture passed with the same capsule, exact Serde equality, and terminal retirement of the value and both Source owners. The full optimized native Poll suite passed all 19 tests with one test thread. The returned-frame fixture now yields during bounded retries so concurrent fixture execution has time to release the shared origin.

The rebuilt gateway progressed beyond ParserClose and reported a Core wire grant refusal. A direct SDK trial using 4 billion physical work and the checked-in Infrastructure reserve reproduced the Wake admission refusal after 147.4 seconds. Enlarging Infrastructure moved the typed create to the Poll wall deadline at 152.85 seconds. Both outcomes left the artifact absent and the SDK session responsive. Wake's wire admission refusal now reports the loan purpose, enclosing or purpose scope, header extent, and required and available values for each retained axis and work fuel. The error keeps its existing ownership or work refusal kind.

With those diagnostics active, the SDK reached the empty Poll while waiting for instance 0 to become idle. The Infrastructure loan's 152-byte header required 93 retained items and had 63 available out of the 65,536 authored items. Copy, capacity, release, depth and work all exceeded their minimums. The refusal arrived after 140.24 seconds, and the snapshot and ping again reported an absent artifact and a responsive server.

Increasing the diagnostic Infrastructure item reserve and allowing the cold-open loop 210 seconds advanced it to frozen-history capture (stage 10) under a 144-byte loan. At 212.10 seconds, the guest had spent 1,349,182,545 execution fuel. The wall timeout did not distinguish frozen Input from CloseOutput or identify which capture substep remained pending.

## Earlier run before the Note model was available

Driver: `client-journey-driver.ts` (this folder). It wraps the repository's own `runOsMcpClientJourney` (`🌉️mcp/🟦️.ts`, pins `✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts`) in a `ScriptProcessEnvelope` (`createNativeBuildCallerCapabilitiesV1` + `createScriptProcessEnvelope` + `withScriptProcessEnvelope`), so no graph service is needed. Run with the bundled bun and `SEMIO_OS_MCP_BIN=.cache/mcp-fallback-target/debug/semio-os-mcp`. Flags: `--skip-build` (do not build component models), `--note-only` (build only the `note` model).

### Result with 15 of 16 checks passing

| Step | Result | Detail |
|---|---|---|
| os: initialize | PASS | server `semio@warming`, protocol 2025-06-18 |
| os: warm gateway announces native catalog | PASS | `tools/list_changed` after 6.6 s |
| os: first tools/list | PASS | 28 tools, all 10 required present |
| os: first resources/list, prompts/list | PASS | 7 resources, 5 prompts |
| os: context_resolve | PASS | channel headless |
| os: tools/list pagination, foreign cursor rejected | PASS | 1 page, order identical; code -32602 |
| os: resources/list pagination | PASS | 7 unique URIs |
| os: capabilities_search | PASS | 20 hits |
| os: capability catalog health | PASS | zero skips, zero duplicate ids |
| os: artifact_create (probe kind) | PASS | revision 1 |
| os: artifact_open | PASS | kind `os.agent.probe/v1` |
| os: capabilities_search ranking | PASS | 18 note mutation hits, scores monotone, `note.s.note.note@1/*#editor.addBlock` at rank 0 |
| os: capabilities_describe (pinned verb) | PASS | input `{"kind":"text"}` |
| os: artifact_create (typed `s.note.note`) | FAIL | `PLUGIN_UNAVAILABLE`: plugin `note` has no retained component model |
| action_prepare/invoke, artifact_snapshot mutation, history_undo/redo, transactions, inference_run/submit/events/cancel, restart | NOT REACHED | all need the note and wfc component models |

## Root cause of the one FAIL

Not a gateway, catalog or native-server defect. The typed Note path needs the wasm32-wasip2 `note` guest component plus its descriptor (and `wfc` for inference). They do not exist on this machine (no `📦️component.wasm` anywhere under the repo or `.cache`). The repository's own builder (`buildComponentClosureV1`, facet `model`) refuses to start:

`Native build storage refused: prototype-00 needs 19741 MiB (17693 MiB predicted + 2048 MiB reserve), 9590 MiB remain` (first attempt for both plugins: 18942 MiB remain; the image has no grow available).

Disk fell from 70 GiB to 20 GiB free during the session while other agents' builds ran, so the requirement of about 19.7 GiB plus the free-space floor cannot be met, even for the `note` model alone. The builder also reclaims cached build units when it refuses (it removed 3141 units, 5.1 GiB, on the first attempt); that is its own policy. I deleted nothing else by hand.

Also seen: the descriptor emitter was reported stale (4 changed sources, first `🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs`) and rebuilt on each attempt (about 8 minutes at -j 1), so it adds to the cost every run.

## Fixes made

None. No source defect was found in the reachable steps. The older ticket script `build-mcp-journey-models.ts` is stale (it imports `ensureDescriptorEmitterV1`, which no longer exists; the current exports are `ensureFreshDescriptorEmitterV1` and `withDescriptorEmitterV1`). `client-journey-driver.ts` supersedes it.

## To finish the journey (needs about 25 GiB free)

`SEMIO_OS_MCP_BIN=$PWD/.cache/mcp-fallback-target/debug/semio-os-mcp <bun> .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/client-journey-driver.ts` (add `--note-only` to skip wfc). After the models are staged, rerun with `--skip-build`. Expect earlier tickets' known risks next: the Note Poll copy-budget refusal (`mcp-note-export-and-replay-2026-11-21.md`) and the `artifact_create initial` null-pack issue being investigated separately.
