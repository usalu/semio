# Input lane stall: the second back-to-back input stays pending forever

## Symptom
Surfaces journey step "module toggles": two `setFilterModule` clicks back to back. The second ledger entry never settles (`outcome: null`), no refusal, no console warning, no `superseded`.

## Trace (probe `probe-input-lane.ts`, `[DEBUG] ils` logs on the host, `SEMIO_RUNTIME_DIAGNOSTICS=1` for the guest)
1. UI funnel -> `dispatchProgramAction` -> `plugin.handleAction` -> command lane (`runQueuedTurn`, `serializeCommandIngressForActor`). Both commands ran their lane turn and answered an operation id (320, 321). The lane, the mailbox and the worker action lane were NOT the problem: both inputs were dispatched, both lane turns completed.
2. The input settles only when its typed operation completes (`awaitOperationSettle` <- `OperationCompleted` frame <- `drainTypedOperations` polls). Only `OperationCompleted:320` ever arrived. Operation 321 never completed and its mutation never applied (pool showed 5 rows, `aria-pressed` of the second toggle false).
3. The guest turn that retired 321 emitted a typed-operation fault page (lane 11): `plugin.internal: batched publication original generation or revision is stale`. Guest: op 321 captured `mounted.config_generation` at admission; op 320's config commit then moved the config store generation (`generationBefore 1 -> generationAfter 2`), so `config_store.admit_apply_batch(..., mounted.config_generation, ...)` refused op 321. (Artifact edits have a benign `interactive-job.stale` cancel for the same race; the config lane had no equivalent, it failed with a plain fault.)
4. Host: the fault page was observed by the background drain call (`adopts=false`) while the owning command call had already returned. `TypedOperationRouter.fault` published it to the notice subscribers only; nothing settled the waiter of operation 321, so the input stayed pending until the 30 s watchdog. That is the stall: a fault without an owner was never turned into a ledger outcome.
5. Timing: with a pause between clicks, op 320 commits before op 321 is admitted, so no overlap and no fault. That is why single clicks passed.

## Root causes and fixes
Two defects, both fixed at the root:

1. Guest, config lane (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`): a config batch is applied against the stored base at apply time (the product fix `ToggleFilterModule` made every config edit rebase-safe), so it now publishes against the live config generation instead of the generation captured at admission. Overlapping config operations apply in issue order instead of faulting.
2. Host, never stall (`🔌️PluginRuntime/📬️typed-result`, `🔗️channel-adapter`, `🧷️handle-contract`, `🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch`): a fault page whose owning host call already returned now also settles the waiter of its operation (`operationFaultSettlers` -> completion with `fault`), and `dispatchProgramAction` ends the ledger entry as `refused(dispatch-failed, <fault code>)` (notice already shown by the fault subscriber, so `alreadyNotified`). Any future operation fault therefore ends its input with a typed refusal immediately, never a 30 s hang. Progress and cancellation are untouched (no new queue, no lane change).

## Tests
- TS `🧪️tests/🏁️operation-fault-settle/🟦️.ts` (2 pass): two back-to-back inputs both settle; second settles refused with the fault its operation ended in; both complete -> both applied.
- TS `🔌️PluginRuntime/📬️typed-result/fault-notices.test.ts` (15 pass): an unattributed fault settles the waiter of its operation.
- Rust `✏️s/🔌️plugins/🪵️sourcing/.../✏️editor/🧪️tests/🧬️app/🦀️.rs`: `back_to_back_module_toggles_sent_while_the_first_still_publishes_both_apply` (unsettled wire dispatch of two toggles, then settle, both module ids present).

## Verification
- Probe, rapid clicks, 3 of 3 runs: ledger `[applied, applied, applied, applied]`, both toggles pressed, pool 8 rows (before: 5 rows, second entry `null`).
- Full run on 6081 with the rebuilt hub wasm (peer-started `rebuild-sourcing-retry.sh`, built after my guest edit; my own duplicate rebuild was killed to avoid a second restart): journey exit 0 (34 PASS), editing exit 0 (40 PASS), surfaces exit 0 (68 PASS, 0 FAIL, en+de, rapid back-to-back clicks kept in the journey), history exit 1 (12 FAIL, identical count to before, e.g. "discard kept the draft"; unrelated to this change).
- Rust test `back_to_back_module_toggles_sent_while_the_first_still_publishes_both_apply` is added but NOT executed: `repo cargo` refused the curation workspace build (host memory governance, units over their carving budget) and a native cold build was skipped (17 GB free disk). The behaviour is covered end to end by the probe and the surfaces journey.
- Debug logs: all `[DEBUG] ils` prints removed (grep over `🧰️framework` is clean).

## Open
- The guest staleness model for the document lane is unchanged (`interactive-job.stale` cancel with "try again"); with fix 2 such a refusal now ends the input visibly instead of hanging. A host-side ordered replay of stale refusals was not added.
- Peer-owned `[DEBUG]` prints in the guest (`archive/replacement wait`, `retained store close`) and `claude-clear dispatch` in the worker action lane were left untouched.
