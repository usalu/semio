# Input Publication Audit — 2026-10-03

## Conclusion

**P1 — queued-draft acknowledgement has no causal identity.** The current implementation waits for both an applied outcome and an observed normalized value before sending the next command. It can nevertheless accept a publication from another writer when that publication has the same normalized value as the command being awaited. A binding-guard change does not prevent this case because the accepted branch clears the pending acknowledgement before it treats the change as external.

The result is a wrong-target follow-up: a queued command may be sent against the foreign publication's current guard. For the CSV editor, positional row keys make that unsafe when a row is removed and later reinserted at the same ordinal.

## Reproducing Interleaving

1. The retained node is A under guard r1; the user queues B, then C.
2. The first dispatch is in flight with awaiting equal to B.
3. A foreign edit publishes B under guard rForeign before the first dispatch settles.
4. The render accepts it solely because awaiting equals published, clears awaiting, and bypasses conflict detection.
5. The local dispatch settles as applied; the drain loop sends C using the foreign node and its guard.

The value can be equal after number normalization (001.50 and 1.5), so normalized value equality cannot distinguish an acknowledgement from this foreign update.

## Exact Source Evidence

- [useCommitDraft](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1359) records only the planned publication string in awaiting. Its render-time acceptance condition is value equality, before its external-change conflict branch.
- [InputView](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1448) gives the hook an authored draft scope and a separate serialized binding guard. This protects ordinary foreign changes, but neither is a receipt for the command in flight.
- [InputOutcomeV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:87) exposes an inputSeq only. It has no committed document revision, node version, or retained-publication identity.
- [UiDocumentStore.buildIntent](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx:532) mints a per-surface seq and records the source surface, revision, node, and nodeKey. [onIntentStable](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8570) converts it through [uiIntentToActionDescriptor](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2637), which returns only the action address and payload. Thus the existing sequence does not reach the host action or a publication.
- [BuiltNodeStoreCacheV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2134) increments a local retained-store revision when reloading a changed node. [publishBuiltNodesV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:2177) accepts only key/node entries. This revision proves only a later local render; it is not native commit provenance.
- [editable_table_window_row_at](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:36319) assigns each editable cell a stable draft target at [draft_target](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:36342). The CSV editor supplies positional row identities (row-{row}), so a reused ordinal cannot prove the logical row is unchanged.

## Required Correction

Carry a causal receipt from the native successful mutation to the retained publication, then require it alongside InputOutcomeV1 settlement before draining the next command. The receipt must name:

- the dispatched input or an opaque native mutation identifier;
- the committed document or row revision and stable cell identity that the action actually mutated; and
- the publication built from that exact committed revision.

The native action result must produce the receipt when it commits. The refresh that acknowledges it must be pinned to that commit or prove it rendered that revision, and the retained node or snapshot must preserve the receipt for InputView to compare. A host-only marker added when a refresh is scheduled is insufficient: another commit can land between operation settlement and a coalesced refresh, and that render would then carry the wrong cause.

UiIntent.seq, InputOutcomeV1.inputSeq, the store revision, the value, and the binding guard are useful parts of diagnostics and matching, but none currently provides this end-to-end proof. The receipt can remain host/renderer transport metadata; it does not need to become an authored action argument.

Until that receipt exists, no correct rule can accept a same-value publication after a guard change: treating it as an acknowledgement risks the interleaving above, while treating it as foreign would also reject valid acknowledgements whose own revision changes the guard.

## Coverage to Add

Add an interleaving test beyond the existing settlement-first/publication-first and external-guard cases:

1. Queue B, then C from A/r1.
2. Publish foreign B/rForeign before the first outcome settles.
3. Settle the first outcome as applied.
4. Assert a conflict and no dispatch of C.

The receipt implementation should replace that assertion with acceptance only when the published receipt is the first command's receipt, plus a CSV regression that removes and reinserts a same-valued row at the same positional key while a draft is pending.

## Lifecycle Assessment

The current hook has sound local protections for the other inspected cases: it marks the lane busy before sending, checks mount/owner identity before every subsequent dispatch, clears queued work on unmount, and re-runs the drain from an effect after a retained render. These prevent re-entrant duplicate sends and post-unmount follow-ups. They do not solve publication attribution.

## Release Optimizer Assessment

Static inspection found the release lane ordered correctly. [prepareReleaseComponent](/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:97) runs JCO optimization into a staging directory, checks cancellation, bounds the staged release component at the 64 MiB admission limit, and renames only after validation. The package path optimizes before its release-component gate at [catalog build](/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1209). The contract includes interface preservation, a single-core-module check, native WebAssembly execution, repeat-output equality, and cancellation cleanup at [release optimizer contract](/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:357). No ordering or admission defect was found by this static audit.

## Audit Boundary

This was read-only source inspection. No build, test, browser run, or production-file change was performed, and concurrent edits were assessed only as represented in the files read.


## Root Integration Checkpoint

The native receipt path is now consumed by the React draft queue. Revision equality is exact decimal-u64 equality; revisions are hash-derived and must not be ordered numerically. While completion is pending, changed publications remain unclassified. Once applied completion arrives, only the normalized value at its exact revision acknowledges the queued input. The original dispatch revision means publication is still pending. Any other revision preserves the draft and blocks queued edits as a conflict. Missing completion receipts for inputs carrying publicationRevision also block the queue.

Fresh React editable-controls gate passed59/59 (`retained-input-causal-green-2.log`). A new Escape-after-completion-before-publication regression then failed as expected (58 pass/1 fail): discarding cleared the awaiter and allowed a new commit against old state. The fix retains the awaiter until publication unless it was an already-settled conflict. Fresh gate passed59/59 (`retained-input-causal-discard-green-1.log`). An earlier causal red run was an import-resolution failure, not a valid behavioural red witness; its path was repaired. Bilingual accessible recovery is being added separately.
