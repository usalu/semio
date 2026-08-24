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

## Post-remediation Terra RED Closure

The follow-up Terra reaudit found two independent B3/B5 escapes left outside the first retained
surface repair. Both live callers are now covered.

| Post-remediation counterexample | End-to-end repair | Hostile production law |
|---|---|---|
| P5b-R1 Shell removed index zero after one `close_step`, and its ninth lease received one step before scope-exit Drop | `ShellDocumentRetirementRegistry` owns 64 fixed absolute lease credits. Every slot is epoch/generation/surface qualified. Its cursor advances one slot per opportunity and removes a lease only after `close_step() && terminal_is_empty()`. Window and panel map transfer returns the exact refused map tail; spawned transfer restores the exact option owner before returning the error. | `shell_ninth_document_and_nonterminal_first_close_remain_in_qualified_retirement` crosses the former eight-slot boundary and proves the first false close keeps all nine owners. `shell_absolute_refusal_returns_the_exact_max_plus_one_owner_before_mutation` exhausts the only vacant slot epoch, verifies identity-preserving refusal, retries that same owner, and drains terminally. |
| P5b-R2 command aggregation one-stepped and dropped the ninth surface document | `CommandDocumentRetirementRegistry` pre-reserves eight epoch/batch/generation-qualified destinations before every guest turn. Turn documents move into pending slots before driver resume/observation. Completion moves at most eight exact owners into the public outcome and promotes every remaining slot to `Closing`; fault, stale, suspension, observation, destination saturation, and instance close do the same. The worker maintenance lane advances one close cursor and removes only a terminal witness. | `command_batch_ninth_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired` stages nine real leases across pages, rejects a stale batch generation, returns eight, retains the ninth after its first nonterminal close, then converges all exact owners. `command_document_page_saturation_refuses_before_turn_owner_production` fills all 64 reservation credits and proves maximum + 1 refuses before another guest turn can create a document owner. |

The equivalent retained-surface exchange caller now refuses to mint another alias while its one
qualified `exchange_closing` owner is still nonterminal, preventing the previously unreachable
overwrite seam from becoming lossy if a future caller supplies a saturated output list.

The permanent P5b predicate now reads the exact Shell and command-batch production regions. It
requires both fixed registries, qualifiers, page pre-reservation order, exact map/option restoration,
batch promotion/close transitions, terminal gates, and the four hostile laws. New ordered mutations
erase each terminal guard, drop Shell refusal owners, make the Shell registry dynamic, remove command
preflight, restore the ninth-owner one-step Drop, and remove each new law; every mutation is rejected.

Final scoped evidence:

~~~text
bun -e 'import {interactivityLiveReconcileSelfTests as t} from "./📜️script.ts"; t(process.cwd()); console.log("p5b-live-reconcile-selftest=green")'
p5b-live-reconcile-selftest=green

rustfmt --edition 2021 --check <Shell component.rs> <renderer glue.rs>
# no output

git diff --check -- <P5b script/Shell/renderer paths>
# no output
~~~

No Cargo, Nx, Wasm, browser, network, or broad build command was run.

## Final Formatting-gate Closure

The final Terra reaudit found no functional B1–B5 counterexample and requested one mechanical repair:
edition-2021 Rustfmt on the existing UI runtime `reconcile.rs`. Rustfmt was applied to that exact file
without a logic edit. The complete scoped P5b formatter set is now clean: runtime reconciler, mounted
tracker, reactor, document contract, renderer glue, Shell, UI-WGPU component, and widgets.

~~~text
rustfmt --edition 2021 --check <eight exact P5b Rust sources>
# no output

bun -e 'import {interactivityLiveReconcileSelfTests as t} from "./📜️script.ts"; t(process.cwd()); console.log("p5b-live-reconcile-selftest=green")'
p5b-live-reconcile-selftest=green

git diff --check -- <exact P5b source, verifier, and remediation-report paths>
# no output
~~~

This final pass ran no Cargo, Nx, Wasm, browser, network, runtime, or broad build command.
