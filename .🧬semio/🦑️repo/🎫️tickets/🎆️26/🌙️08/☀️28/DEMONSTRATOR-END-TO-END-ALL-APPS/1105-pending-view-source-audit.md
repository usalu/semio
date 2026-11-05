# Pending View Source Ownership Audit

Scope: read-only kernel/plugin sources, 2026-11-05. Root, products and OS AGENTS read. No source edits, builds, tests, cache changes or lifecycle operations.

Paths below use P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/` and M = `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/`.

## Actual Owners and Loans

- P/⚛️reactor/🪟️surfaces/🦀️.rs:58–83: SurfaceBinding owns Option<ViewModel>; SurfaceContexts owns both fixed binding slots and the last full host view, Option<ViewModel>. The full view is explicitly the pending-effects roster, distinct from each body projection (70–73). These are direct owners, not Arc<ViewModel>.
- The same file:108–133 moves the mounted host view into the full cache and stores a role projection per binding. update_view at 165–178 replaces views/projections and increments context version. remove at 212–220 can remove the owner. Therefore existence across frames does not itself pin one immutable generation.
- borrow(surface) at 141 and view() at 158–160 return turn-scoped references to actual owners. They also consult cached_source.original_authority() while the corresponding original capture has moved the original out. They do not return a retained transferable lease.
- SurfaceWindowLease at 17 and borrow_window_lease at 148–152 bind registry address, context version, slot and binding version, then reborrow the actual ViewModel for one turn. They do not pin an immutable owner. On update the lease invalidates rather than retaining an old source.
- M/🪟️selection/🦀️.rs:5–17 stores view/roster/target addresses, lengths and caller epoch, and validates them on each operation. It owns no roster. Safe use requires a real lifetime/epoch owner; pointer and length checks alone cannot establish cross-frame immutability or defeat ABA.
- No OriginalViewModel declaration/use was found by tracked framework search.

## Genuine Existing Pinning Mechanism

P/⚛️reactor/🪟️surfaces/read.rs provides OriginalSurfaceViewCapture. At 7 it retains OriginalSourceBinding, decoder identity, registry identity, admission, cursor and closure state. admit at 9 uses actual original construction/receiving authority. At 17 it takes cached_capture, a registry mutation lease. At 18 it takes the actual Option<ViewModel> from full cache or selected binding into RetainedCloneSource::admission_owned, then installs the source in registry.cached_source. That source owns the original; this is stronger than address validation.

The original remains readable through SurfaceContexts::view/borrow above, while insert/update_view/remove assert that cached_capture is absent (surfaces/🦀️.rs:109,167,213), OriginalSurfaceRefresh::advance waits if capture exists (native.rs:26), and close_original_step waits too (native.rs:47). close_read at read.rs:23 returns original authority to its exact vacant destination before releasing the lock. Drop at 30 requires paid closure and cache restoration.

**Limitation:** current public advance/advance_surface (13–21) necessarily runs RetainedFieldCursor<ViewModel> to materialize a complete clone; take_into (22) transfers that clone. There is no public pin-only admission/read-donor API here. Borrowing registry.view during a capture can see its actual source, but using the current capture as a pending poll owner would still trigger the prohibited full clone and would need deliberate retained custody and closure. Do not claim that a ready-made transferable no-clone OriginalViewModel API exists.

Existing consumer P/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/response.rs:9,21 retains the boxed capture and output ViewModel in OriginalResponseActionSource; 30–32 drives original source restoration/closure. It is action-response metadata ownership, not pending-effects ownership.

## Who Supplies Pending Effects and What Survives None

- P/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🏁️exchange-finish/🦀️.rs:22–29 calls pending_effects only for flags.mutated. At 25 it explicitly clones SurfaceContexts::view into a local Option<ViewModel>, then passes a borrow at 26. This clone is not retained as a pending continuation source.
- P/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs:272–276 decodes a local RefreshRequest containing ViewModel, updates registry, then borrows request.view_state for pending_effects. Again only a local borrow.
- The same file:81–93 plugin_render_surface borrows the actual binding context, calls app.render and returns None when render is pending. Registry retains the owner, but no per-render immutable view pin is established here. On the next frame another host refresh may replace its view.
- P/🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs:34–46 retains render identity/work/tree across None. At 118–128 it explicitly retains an Arc of the original document snapshot and checks Arc::ptr_eq on resume. This is **snapshot** retention, not ViewModel retention. At 153 it still passes the current borrowed view_state.
- P/📜️declaration/👁️view/🔭️artifact-view/🦀️.rs:27–38 ArtifactPendingView carries committed snapshot borrow, children borrow, admitted SnapshotRead, render operation, generation and instance owner. Its snapshot lease can be donated by take_snapshot_read; it contains no ViewModel owner/read donor. Its generation is document/tool operation identity, not a registry view epoch.

## Callback and Scheduling Boundary

Current context reaches P/⚛️reactor/🔌️app/🦀️.rs:487 and P/🧵️instance/🔌️plugin-app-impl/🖼️render-surface/🦀️.rs:299, but is dropped at the ArtifactApp call at 309. Early yield at 300 and preparation refusal at 303 become empty effects, indistinguishable from completion.

Minimal coherent signature migration (design recommendation): thread the same &mut StepContext through ArtifactApp::pending_effects (P/📜️declaration/🧩️app/🎛️artifact-app-trait/🦀️.rs:510), ArtifactEditor::pending_effects (surface/✏️artifact-editor-trait/🦀️.rs:358), ArtifactViewer::pending_effects (surface/👁️artifact-viewer-trait/🦀️.rs:308), and editor/viewer adapters (surface/🖊️editor-app/🦀️.rs:268; surface/📖️viewer-app/🦀️.rs:247). Use a typed Result<Option<Vec<Effect>>, OriginalTurnFailure> at every layer, or a distinct first-party pending-effects failure that preserves OriginalTurnFailure and SDK failure without flattening. None must mean pending; Some(empty) means completed without effects. Update PluginApp and both real runtime callers, plus public fixtures/implementors. Do not silently reuse a render failure name as semantic ownership.

with_original_mut already supplies busy=>Ok(None), typed failure and the actual mutable session owner without allocation (P/📜️declaration/👁️view/🛬️lifted/👷️job/🦀️.rs:247–250); it supplies no view-source lifetime.

Actual render continuation: dirty-surface-render/🦀️.rs:49–65 responds to render None using grant.cancel_and_defer. patches/🎰️surface-slots-and-reconcile-grant/🦀️.rs:235–240 sets slot.pending_render=true. turn/🎯️dirty-poll-surfaces/🦀️.rs:70 drains take_pending_render_on_original, whose paid take is patches/🚪️tracker-admission-and-reservation/🦀️.rs:38 onward. patches/🏃️tracker-drive-and-work-predicates/🦀️.rs:189,209,312 incorporates pending_render_ready; turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs:117–124 arms reconcile work; turn/🌀️poll-kernel-turn/🦀️.rs:58–62 passes this to settle_turn_work; turn/🔦️more-work-sources-and-tracing/🦀️.rs:241 includes it in more_work.

**Unresolved design:** pending_effects currently has no pending state/result at either caller, so render deferral does not automatically reschedule a suspended pending poll. A retained per-instance poll owner, original view read lease, typed outcome and explicit more_work/poll continuation have to be integrated at exchange/turn ownership; returning empty effects or relying on an unrelated future mutation is insufficient.

## Recommended Next Code Owner

Start at SurfaceContexts/read.rs: factor the actual source pin/admission/restoration mechanism into a funded original read lease that exposes a turn-scoped borrow and retains its real owner/mutation lease across pending frames **without** running RetainedFieldCursor<ViewModel>. Then have the runtime pending continuation retain that lease and validate original context/decoder/registry identity until paid completion/cancellation closure. Bind the selector epoch to that actual leased source. Thread this donor and the genuine cx through pending declarations/adapters. A small roster-specific donor could reduce authority, but requires a real owned roster/source lifecycle; it cannot be created safely by storing a raw pointer and length.
