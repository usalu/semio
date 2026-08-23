# P5b Live Reactor Reconcile Cutover

Date: 2026-08-23  
Status: rejection remediation source-audit-ready; not accepted

## Pre-edit caller census

- `semio-framework-ui-runtime::SurfaceReconciler::reconcile` has one mounted production caller: `plugin/⚛️reactor/🩹️patches::PatchTracker::diff`.
- The production reactor `poll` calls `PATCHES.diff` after `plugin_render` for every `(instance, surface)` in its dirty-render pass. This is product/plugin reachable.
- The only retained `SurfaceReconcileCursor` caller is `FrameTransaction::reconcile_tree`; the repository has zero production `FrameTransaction::new` callers, so that cursor does not protect the mounted reactor path.
- `PatchTracker::revision` calls `SurfaceReconciler::snapshot`, cloning every retained node to read one scalar revision.
- `SurfaceReconciler::mark_rejected` clears both retained maps in one call.
- The test-support `patches_diff` hook mirrors the mounted call but is `#[cfg(test)]` and is not a second production caller.

## Pre-edit cap and ownership census

- Mounted reconciler slots: dynamic `HashMap<String, SurfaceReconciler>`, no surface/item/byte cap.
- Pending dirty renders: dynamic `Vec<(u32, String)>`, then a run-to-completion `for` loop.
- Candidate traversal: dynamic `Vec`, `HashMap`, and `HashSet` owners, reserved only after a complete presentation traversal.
- Node/key/record/patch byte admission: none before tree handoff.
- Reconcile cursor generation: base revision assertion only; no operation generation/ABA witness.
- Cancellation/supersession: cursor ordinary-drop through `Option = None` in the dormant frame path; mounted path has no cursor.
- Terminal/rejected owner retrieval: none.
- Close: no app/instance/realm reconcile close authority; rejection clears all retained owners at once.

## Cutover caps

The implementation packet uses these fixed limits:

- 64 generation-tagged surface slots per actor;
- 4,096 presented nodes per operation;
- 32,769 retained source/derived semantic owner credits per operation;
- 256 bytes per surface/key identifier;
- 16 KiB page size for admitted semantic backing;
- 2 MiB maximum simultaneous source/candidate/record/effect bytes per operation;
- 8 MiB actor aggregate reconcile bytes.

Admission reserves an operation slot and its fixed per-operation maximum before the tree leaves the caller. The retained census then consumes one node/edge/scalar per `StepContext` grant and rejects cap/+1 with the exact tree/reconciler authority still publicly retrievable. Candidate state is not published until a complete-generation commit.

## Files in scope

- UI-runtime `🦀️reconcile.rs` and its existing crate export.
- Plugin reactor `🩹️patches/🦀️component.rs`.
- Narrow live scheduling/close wiring in plugin reactor `🦀️component.rs`.
- Existing root `📜️script.ts` interactivity verifier region.
- This report.

FrameTransaction, layout, GPU/presentation, P3/P4/P8 domain state, dependencies, ticket metadata, and Git state are excluded.

## Implemented source boundary

- The production `SurfaceReconciler::reconcile` entry is removed from the mounted build (`#[cfg(test)]` only). `PatchTracker` no longer calls it.
- `SurfaceReconcileJob` takes the reconciler and `ComponentTree` by value only after reserving a generation slot and the fixed maximum item/byte authority.
- `SurfaceReconcileCursor` advances one presentation edge, one node, one identity, one record, one removal edge, or one scalar phase transition per `StepContext` grant. Before the first key/record clone, its semantic census covers component payloads, accessibility, bindings, menus, nested values, vector/map backing, and child-id backing for the simultaneously retained source, record, and effect copies. It rejects a node/page above 16 KiB with the original node still held and no candidate mutation.
- Completion first retires the cursor and the previous retained shadow one semantic owner per grant. Only the consistent candidate boundary can publish a patch and release its credit.
- `SurfaceReconcileRejected` and `SurfaceReconcileTerminal` expose generation, take/retry or resume, one-owner `close_step`, `terminal_is_empty`, and Drop handback into a fixed generation-keyed terminal registry.
- Mounted `PatchTracker` uses fixed surface, rejected, terminal, deferred, unadmitted, and close registries. `reserve_mounted` installs a generation-tagged unadmitted slot before `plugin_render` can materialize a tree; its grant commits the exact tree by value or cancels the empty reservation. The unadmitted 65/+1 API returns the exact tree at saturation, and mounted retry uses the original generation rather than minting an ABA-prone successor.
- Local terminal saturation leaves the exact faulting job in its surface slot. No tracker-owned path drops into the unmounted global handback registry merely because the local terminal ring is full.
- Instance close now covers ready patches, deferred surface keys, reserved/rendered unadmitted entries, rejected owners, active/idle surface slots, and matching terminal cursors. It advances a matching terminal while the close generation remains armed, prevents stale ready publication, and cannot report terminal emptiness while any listed class or surface slot remains.
- The reactor calls `PatchTracker::drive_one` exactly once per actor turn, retrieves at most one ready patch, advances one close opportunity, and includes reconcile/close readiness in `TurnStatus::MoreWork`.

## Semantic fixtures and permanent mutations

Direct Rust fixtures additionally cover dynamic semantic page +1 before clone, mounted pre-materialization reservation, unadmitted 65/+1 exact pointer return, preserved retry generation, all-class instance close/no stale publish, and terminal saturation followed by one-slot rearm. The permanent interactivity predicate reads the live UI-runtime/tracker/reactor sources and denies:

- cap/page/aggregate drift;
- cursor construction before admission;
- missing generation or cancellation validation;
- two cursor steps in one grant;
- dynamic tracker slots;
- tree cloning or the old synchronous diff seam;
- effect reordering;
- missing mounted pre-reservation, exact unadmitted return, original-generation retry, all-class instance close, local saturation retention, terminal Drop handback, or discriminating fixtures.

## Independent rejection remediation

The independent 2026-08-23 audit rejected the first packet on four paths. This revision closes them in source:

1. Dynamic component/accessibility/binding/menu/nested-value ownership is counted before `seen.insert`, `build_record`, `record.clone`, or `diff_record`; the cap +1 fixture asserts the exact node pointer remains in `held_node` and all candidate collections remain empty.
2. The mounted route reserves a fixed unadmitted generation slot before render materialization. The public cap +1 path returns the exact `ComponentTree`, `drive_one` retries one FIFO rendered owner with its original generation, and instance close handles both empty render reservations and rendered trees.
3. Closing filters ready publication immediately and drains ready/deferred/unadmitted/rejected/active/idle/terminal classes. Terminal emptiness includes every class and the surface slots.
4. Fault and rejection terminal saturation retain the original job/reconciler locally until a terminal slot becomes available; the close path advances that retained terminal rather than waiting behind its own close marker.

## Verification

- `rustfmt --edition 2021 --config skip_children=true --check`: PASS for `reconcile.rs`, mounted `patches/component.rs`, and reactor `component.rs`.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; deny mode clean, one recorded allowlisted blocking-bridge finding, zero unlisted failures.
- `bun 📜️script.ts verify interactivity`: PASS; same clean deny baseline.
- Production scan: zero mounted `patches.diff`, `SurfaceReconciler::reconcile`, `snapshot().revision`, runtime/pool/thread construction, or production tree clone. The remaining `transaction.rs` snapshot revision read and test-only loop/tree clone are outside the mounted P5b route and reported rather than hidden.
- Scoped and whole working/staged/HEAD `git diff --check`: PASS. The scoped working diff reports 2,472 insertions and 276 deletions across four shared source/verifier files; those totals include concurrent accepted/source-wave work and are not an attribution claim. This report is untracked and therefore absent from that diff stat.

## Honest residuals

- This is source-only. Cargo, Nx, Wasm, browser, runtime, and network execution were prohibited, so no compile or runtime claim is made.
- The already-audited reactor actor is the process WorkerPool grant boundary; this packet does not construct another pool/runtime/thread and does not introduce a nested executor.
- `plugin_render` tree construction precedes this adjacent reconciliation authority and remains a distinct producer/materialization residual. This packet guarantees fixed ownership before the produced tree enters reconciliation; it does not claim `plugin_render` itself is resumable.
- FrameTransaction presentation/layout, text, tessellation, GPU upload, multi-window surface/effect/resize stress, and the full Phase 5 runtime matrix remain RED/open.
