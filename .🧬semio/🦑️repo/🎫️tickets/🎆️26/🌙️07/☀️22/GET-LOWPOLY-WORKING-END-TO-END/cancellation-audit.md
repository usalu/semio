# Lowpoly Cancellation Contract Audit

Read-only source audit, 2026-11-04. No production edits, builds or browser execution.

## Finding

The static composition descriptor contains execution.cancellable:false for migrated Lowpoly commands, but the traced typed-operation transport does not consult this flag. Therefore the claim that this descriptor flag hides retained-operation cancellation is unsupported by the actual predicates. There is a separate concrete presentation gap: Lowpoly does not render the available retained-operation progress/cancel controls.

## Evidence

- Owner editor: ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor.
- Owner 🧵️retained-commands/🦀️.rs:103-105 uses ToolExecutionContract::resumable; 359-367 publishes an initial Progress turn; 455-456 returns that contract for the migrated factory.
- Framework plugin 🧵️instance/📄️docs/📥️staging/🧾️typed-operations/🦀️.rs:21-34 explicitly requires ToolCancellationPolicy::PerOperation from admission.proof.contract(). It does not inspect descriptor semantics.
- Framework plugin 🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs:137 accepts a ToolCancellationLease and :266 stores Some(cancellation_lease).
- Framework plugin 🧵️instance/👷️jobs/📥️staging/⏳️operation-progress/🦀️.rs:38-50 snapshots live nonterminal operations with progress and their cancelling state. Its cancellation dispatch :67-79 checks exact operationId/generation plus instance/actor authority. The final predicate :73 is operation_cancellation_target_admitted(true, operation.terminal_seen, operation.cancellation_lease.is_some()); then :77 cancels that lease.
- Framework plugin ⏳️operation-progress/🦀️.rs:61-62 admits present && !terminal && cancellable. Here cancellable is lease presence, not action semantics.
- Its operation_progress_controls :66-110 emits status, progress and a Cancel button for each status record. :95 disables only when operation.cancelling; :98 binds cancelTypedOperation with exact operationId and generation. There is no descriptor flag predicate.
- Lowpoly 🕹️play-app/🦀️.rs:31-68 renders all six body types directly to a ComponentTree. It does not read doc.operations(), call operation_progress_controls, or add a status node. Its render and render_with_request_context :320-340 delegate to that function. A repository-wide production grep for operation_progress_controls found only the Raster caller and the framework helper definition; no Lowpoly caller. A focused Lowpoly grep found no operation_progress_scope override or status_json/statusJson producer.

## Host Surface Cancellation Is Separate

World3dHost 🎯️marquee-and-bridges/🟦️.tsx:174 shows Cancel when parsed status.cancellable is true. Native WGPU surface parsing (Shell staging 🫧️surfaces/🧱️chrome-paint-and-navbar/🦀️.rs:99) requires status JSON cancellable:true plus a nonempty cancelAction. This is a runtime surface status contract, not composition action execution.cancellable. Lowpoly supplies no such status producer in the inspected plugin.

Lowpoly 🎮️commands/🖌️paint/🦀️.rs:231-237 maps CanvasPointerUp.cancelled:true to phase abort/captureLost. This aborts an open paint gesture (documented at :8 and covered by the existing an_aborted_stroke_leaves_zero_trace test), not a cancelTypedOperation request targeting an expensive retained command. It cannot substitute for the missing operation cancel affordance.

## Scope and Next Correction

Source evidence establishes a missing Lowpoly presentation path, not runtime failure of the retained cancellation lease. Wire the owner view to doc.operations() with localized accessible progress/cancel controls and a corresponding dirty scope, then verify live cancellation before publication with operation identity. Merely changing descriptor cancellable flags would not reach either traced UI predicate. No runtime claim was made.

All successful inspection commands used repo tools exec --light with git grep or node. An initial repo exec --light attempt was rejected as Unsupported build option: --light before the search ran; it performed source-admission logging but no requested build target.
