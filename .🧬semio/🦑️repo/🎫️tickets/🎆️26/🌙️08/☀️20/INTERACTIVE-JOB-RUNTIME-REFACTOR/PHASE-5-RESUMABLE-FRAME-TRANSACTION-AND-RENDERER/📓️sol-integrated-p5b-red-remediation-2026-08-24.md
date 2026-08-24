# Sol Integrated P5b RED Remediation — 2026-08-24

## Status

Source remediation complete for the bounded findings in the Terra integrated P5b source acceptance
reaudit. The isolated permanent P5b verifier is GREEN. No Cargo, Nx, Wasm, browser, runtime, stress,
or broad build matrix was run.

## Counterexample Mapping

| Integrated RED counterexample | Repair |
|---|---|
| Reactor poll allocated dynamic dirty-render and dirty-intent work sets | DirtyPollOwners now owns fixed UiFixedList surface and per-instance intent slots. Admission is fallible, deduplicated, nonpanicking, and returns the exact SurfaceId or UiIntent at maximum + 1. |
| PatchTracker retained heap String identities | Surface, deferred, unadmitted, rejected, mounted-terminal, and live-slot identities now retain SurfaceId and UiText backing. Mounted reservation and defer return exact typed owners on refusal. |
| Renderer retained surfaces and rejections used unbounded HashMaps | RetainedSurfaceRegistry and PendingSurfaceRejectionRegistry are fixed 64-slot, instance/surface/generation-qualified registries with one-slot close cursors and terminal-empty witnesses. Per-surface queued and rejected patches are fixed lists. |
| A UiDocumentLease or builder was erased after one nonterminal close step | Published, replaced, exchange-rejected, stale, cancelled, and faulted document owners persist in closing, exchange-closing, or a closing RetainedDocumentBuild until close-step and terminal-empty witness completion. Rejected records retire in a later opportunity. |
| Renderer sequence allocation wrapped through zero and reused identities | RendererSequenceAuthority reserves with checked nonzero addition, commits transactionally, rolls back abandoned reservations, issues u64 maximum once, then refuses permanently without state mutation. |
| Absolute renderer rejection could consume a turn patch before admission | UiTurnPatches transactional transfer moves one patch and restores it to the already-credited turn retirement owner on refusal. Renderer patch admission returns the exact patch through every registry, sequence, and queue refusal; dropping the refused TurnResult follows the existing incremental turn-patch handback lane. |
| WindowMeasure partition cloned two dynamic vectors | WindowMeasurePartition borrows into two fixed UiFixedLists and returns the exact borrowed maximum + 1 measure. |
| Shell Group height and render recursively traversed children | Height and render use explicit fixed 64-entry stacks. Overflow fails closed without partial action publication. |
| Shell Select, Slider, and Toggle cloned dynamic widget/action owners and collected Select items | The rail renders borrowed UI-native controls. A fixed UiText action registry retains controller/action identity; click and drag materialize only the dispatched event at the interaction boundary. No ActionDescriptor clone or Select Vec conversion remains in the rail. |

## Hostile Laws Added

- WindowMeasure partition maximum + 1 returns the identical borrowed measure pointer.
- Renderer sequence admits maximum once, permanently refuses repeated post-maximum requests, and
  rolls an uncommitted reservation back transactionally.
- Renderer surface and rejection registries return the exact maximum + 1 SurfaceId and patch owner,
  then drain to terminal empty one opportunity at a time.
- Stale, cancel, and fault document builds retain their closer after the first nonterminal
  opportunity and reach terminal empty.
- Builder close persists after one step; ordinary lease Drop reaches terminal through the global
  one-page closer.
- Refused renderer turn-patch transfer restores the exact credited retirement owner.

## Permanent Verifier

The P5b source predicate now reads the live reactor, tracker, renderer glue, UI WindowMeasure source,
Shell rail, and document/kernel schema. It rejects dynamic poll work sets, tracker heap surface
identity, dynamic renderer registries, missing generation/surface/cursor witnesses, closer erasure,
wrapping generation, lossy renderer patch transfer, dynamic WindowMeasure partition, recursive
traversal, and cloned action/item conversion. Nine integrated mutations cover those exact production
forms in addition to the existing P5b mutation corpus.

## Evidence

~~~text
bun -e 'import {interactivityLiveReconcileSelfTests as t} from "./📜️script.ts"; t(process.cwd()); console.log("p5b-live-reconcile-selftest=green")'
p5b-live-reconcile-selftest=green
~~~

Scoped edition-2021 rustfmt was applied only to the touched Rust sources. Broad compilation and
runtime evidence remain deferred to the one-owner final matrix as required by the packet.
