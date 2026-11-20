# Event batches whose answers carry ui patches

## Cause
`captureActorActivation().turn()` in `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🪪️exact-activation/🟦️.ts` crosses a host batch of N events as N one-event crossings (a reactor turn admits one event) and merged the N answers into one result. `mergeSequentialTurnAnswers` threw `actor-poll.event-batch-carried` when more than one answer carried `uiPatches` or `uiPatchReceipt`, because a patch page and its receipt are one unit (`recordInstanceTurn`, `captureInstanceUiPatch`, one receipt per acknowledgement, strictly increasing `patchSequence`). Batches of N events come from typed-operation acknowledgements (`typedOperationAcknowledgements`) that `settlePluginTurn` and the command drive hand to the next turn; at boot (fem3d model and results windows) two of them routinely answer with patches.

## Design
Chosen: the batch stops after the first crossing whose answer carries ui patches or a receipt. That answer is returned (merged with the earlier non-carrying answers) together with `unsent`, the remaining events in order. The caller accepts the patch page, acknowledges it, and puts `unsent` first in front of the next events it owes.

Why not a list of patch-carrying answers: the guest would see events 2..N before the host accepted and acknowledged page 1. The guest bounds publication capacity per surface (`beginPatch` answers busy while a patch is unacknowledged), so that changes behaviour. With the stop, nothing crosses before the page is acknowledged, exactly as with one crossing per event.

Command page: rides only with the crossing of the last event. If the batch stops early the page was not sent; callers resubmit the same page with `unsent` until `unsent` is empty (page loops of the React command drive and the wgpu command drive).

The throwing path is removed; no fallback exists. Only the last answer can carry patches, so the merge takes patches and receipt from the last answer.

## Fix (callers of batches > 1)
- `📮️shard-client/🪪️exact-activation/🟦️.ts`: `carriesUiPatches`, early stop, `unsent` on the result, throw removed.
- `🖼️wire-turn/🔖️turn-result/🟦️.ts` and `PluginRuntime/🪝️wire-codec/🟦️.ts`: `WireTurnResult.unsent` plus `coerceTurnResult`.
- `PluginRuntime/🔁️turn-settle/🟦️.ts` (`settlePluginTurn.acknowledge`): `unsent` first. Covers refresh, extension completion, document port, job completion, operation drain.
- `PluginRuntime/🚚️handle-turn-drive/🟦️.ts`: `acceptTurn` folds `unsent`; the command page loop resubmits until nothing is unsent.
- wgpu: `🧾️typed-operations-and-handle` `WgpuTypedOperationDrive.observe` queues `unsent` before the acks (covers all its settle loops); `🖼️render-and-turn-drivers` command page loop resubmits.
- `🎠️kernel/🎛️activation-registry/🟦️.ts` `runQueuedTurn`: loops over `unsent`.
- Inbound request drive and bench harness submit single events: unaffected.

## Tests
`🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🎟️activation-lease/🟦️.ts` (the lease tests, run through `🎭️actor-transport`): four rows of a 3-event batch (patches on first+second, second only, none, last only). Asserts one event per crossing, event order a,b,c with no loss or duplicate, command page only on the crossing of the last event, patches and receipt per carrier answer unchanged, effects concatenated in order.

Run: `bun test ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎭️actor-transport/🟦️.ts` gave 180 pass, 3 fail. The 3 failures do not touch this change (generated worker source expectation, missing `SEMIO_TEST_ARTIFACT_DIR`, component codec lane). `wgpu-typed-operation-reply` 6/6 pass.

## Browser check (http://127.0.0.1:6087, reload, about 60 s)
- No `event-batch-carried` error and no "The input could not be delivered"; page stays up.
- Model window renders the 3D truss model (screenshot).
- Results window shows "Analysis queued" (page text: `Model`, `Results`, `Analysis queued`).
- Open item, separate from this fix: a banner reports `semio · fem · fem3d has not answered "Note Shell Command"` growing to 28 s while the guest keeps draining (DEBUG gate logs show `ingress.resume`/`more-work` slices). The analysis never left "queued" in the observed time.

## Changed files
- 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🪪️exact-activation/🟦️.ts
- 🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts
- 🧰️framework/🔨️modules/🎠️kernel/🎛️activation-registry/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🪝️wire-codec/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🔁️turn-settle/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🧾️typed-operations-and-handle/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🔌️plugin-module-loader/🖼️render-and-turn-drivers/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🎟️activation-lease/🟦️.ts
