# Terra Integrated P5b Source Acceptance Reaudit — 2026-08-24

## Verdict

**RED — reject.** The narrow reconciler repair is materially improved, but the fully integrated production route does not meet P5b B1–B5. Three independent live counterexamples remain: uncredited dynamically allocating reactor work sets, dynamically keyed renderer retained authority with lossy close/error branches, and the production-reachable dynamic/recursive `WindowMeasure` rail including all three `ActionDescriptor` helpers.

This is a source-only audit. I read the P5b contract, both prior Terra RED reports, the Sol core report, Packet A, Packet B, the assist packet, and both coordinator residual reports before tracing the live code. No implementation source was edited.

## What Does Hold

- The direct `SurfaceReconciler::reconcile` compatibility path is test-only (`ui runtime reconcile.rs:295`), while the reactor uses `PATCHES.reserve_mounted(surface)` then `grant.commit_source(tree.root)` (`reactor component.rs:1532-1540`).
- The reconciler uses retained fixed/page-owned map census (`reconcile.rs:587-706`), one `drive_one` opportunity, fixed credit/reservation, and fixed handback machinery. The mounted tracker checks the published owner before the tracker ACK (`reactor component.rs:1123-1127`, `1734-1767`).
- The fixed turn-patch transport is genuinely connected: shard-side `UiTurnPatchTransportProducer` (`plugin host shard component.rs:161-201`), renderer token claim (`renderer glue.rs:3342-3354`), then retained application (`renderer glue.rs:5032-5035`). `ExchangeSurfaceDocument` carries a `UiDocumentLease` in a `UiFixedList` (`renderer glue.rs:3813-3836`), and the interpreter reads exactly one page per opportunity (`Interpreter component.rs:1238-1260`).
- `ProgramBridge::render_with_document` gives first render a bounded `AdvanceRetained` opportunity loop and fails closed on exhaustion (`ProgramBridge component.rs:269-289`). The loop is a bounded publication opportunity, not a terminal-owner drain.
- The retained UI engine/tree side has fixed document ingress, one-page application, fixed validation stack, stale-generation checks, and incremental tree close (`ui-wgpu engine.rs:231-362`, `tree.rs:35-153`).

Those facts are necessary but not sufficient for integrated acceptance.

## B1–B5 Findings

| Gate | Result | Live evidence |
|---|---|---|
| B1: one-opportunity retained work, no recursive/dynamic conversion | **FAIL** | The core reconciler passes its narrow census shape, but the live shell expands `WindowMeasure::Group` recursively (`Shell component.rs:10717-10727`) and `partition_window_measures` clones into two dynamic `Vec`s (`ui-wgpu component.rs:944-957`). This is production renderer work, not a test/oracle-only helper. |
| B2: exact credited fixed backing for every retained/working owner | **FAIL** | The P5 reactor creates `Vec<(u32, String)>` and `HashMap<u32, Vec<UiIntent>>` for every poll (`reactor component.rs:1036-1040`), populates them from events (`1068-1120`, `1481-1524`), then iterates all entries to render (`1532`). `PatchTracker` also retains heap `String` values in slots/deferred/unadmitted state and only checks `surface.len() <= 256` at admission (`patches component.rs:296-321`, `399-446`). Length does not own or credit the allocation backing/capacity. |
| B3: terminal-first, lossless incremental close | **FAIL** | `RetainedSurface::advance_document_one` calls `closing.close_step()` once then drops its only closer by assigning `self.closing = None` (`renderer glue.rs:4140-4144`). `RetainedSurface::close_step` does the same for `published` and `closing` (`4229-4237`). A `UiDocumentLease::close_step` only releases one alias and retires one arena item; it reports nonterminal while the slot remains active (`ui contract document.rs:741-753`), and its Drop performs no further retirement once released (`755-762`). The retained close witness is therefore discarded before terminality. |
| B4: transactional/no-ABA generation | **FAIL** | Core tracker generation preflight/commit is sound, but renderer document/patch generations use `SEQ.fetch_add(1, Ordering::Relaxed)` (`renderer glue.rs:3162`, `3338-3340`) without checked exhaustion. After wrap it returns `0` (rejected by the document arena, `ui contract document.rs:519-522`) and later reuses `1`, creating a real ABA/reuse route. `advance_document_one` simply drops a build whose generation is zero (`renderer glue.rs:4149-4151`) rather than publishing a retained fault/owner. |
| B5: bounded refusal/saturation/drop and exact handback | **FAIL** | `KernelPoolState` owns `retained: HashMap<(u32, SurfaceId), RetainedSurface>` and `pending_rejections: HashMap<(u32, SurfaceId), (UiRevision, String)>` (`renderer glue.rs:4288-4300`), populated without fixed admission by `.entry(...).or_insert_with` (`5039-5049`). It also drops an in-progress `UiDocumentBuilder` on stale revision, failed credited clone, missing builder, or `try_push` error (`4149-4165`), with no retained rejection, requeue, or terminal close owner. The drop merely releases the arena slot (`ui contract document.rs:706-712`); it does not preserve the current document build or drive it to terminality. |

## Renderer Authority and Close Counterexamples

`RetainedSurface` itself uses bounded `UiFixedList` for queued patches, which is good, but it is nested in the unbounded `KernelPoolState::retained` map. Thus the map, its `SurfaceId` keys, per-surface snapshot state, document leases, and rejection strings are live mutable working state with neither a fixed slot admission nor an exact aggregate credit.

The defect is not made safe by global arena cleanup. `UiDocumentArena::retire_one` retires a single node/scalar per invocation (`ui contract document.rs:639-675`). Once `RetainedSurface` throws away the released lease after one step, there is no exact close owner to prove terminal completion. The later global `close_ui_document_page_one` calls cannot restore the lost per-surface closer or a refusal witness.

The rejection path has the same issue: `apply_ui_patch` makes a temporary `UiPatchApplyProducer` then immediately drops it and stores only a dynamic reason string (`renderer glue.rs:5042-5049`). That does not retain an exact rejected patch owner for a bounded close/refusal lane.

## WindowMeasure Is Production-Reachable and Violating

The three requested helpers are not dead definitions or frozen wire-schema fields.

- `WindowMeasure` owns `String`, `Vec<MeasureSelectItem>`, `Vec<WindowMeasure>`, and dynamic `ActionDescriptor`/`DslValue` values (`ui-wgpu component.rs:13-20`, `841-924`).
- `Shell::measures_for_kind` clones those measures on the ordinary shell render route (`Shell component.rs:10585-10587`). Both the general and utility rails call `partition_window_measures` and render every result (`10607`, `10656-10660`, `10688`, `10702-10705`).
- The Group helper recursively calls itself (`10717-10727`).
- The Select helper clones ID/value/items into a dynamically collected widget `Vec` and clones `on_change` (`10730-10740`); Slider clones `on_change` (`10748-10761`); Toggle clones `on_change` (`10769-10776`).

Therefore Select, Slider, and Toggle each violate the requested no-dynamic-action/no-`Vec`-staging condition on a production renderer path. This cannot be excused as retained-document wire data: the values are cloned, partitioned, recursively converted, and installed in widgets during rendering. The native ProgramBridge currently returns an empty dynamic measure map, but the shell also takes `kind.options.measures` directly; the rail remains normal production code and plugin definitions construct `WindowMeasure` values.

## TurnResult and Host/Shard Boundary

The selected renderer runtime constructs `GuestRuntimes::Owned(OwnedRuntime::new())` (`renderer glue.rs:4320`). The retained turn transport on the selected shard/renderer path is fixed and its token is single-claim.

The alternate Wasmtime host is nevertheless not P5-ready: it explicitly drains and discards `emit_patch_sink`, then returns `UiTurnPatches::default()` (`plugin host component.rs:2017-2035`); the async runtime has the same empty patch result (`plugin host runtime.rs:272-298`). I did not charge this as the selected renderer path's direct counterexample, since it is not the `OwnedRuntime` selected at `4320`; it remains an unclosed cross-host transport gap and prevents a broader host-independent GREEN.

## Caller Census, Laws, and Static Checks

- Re-running Packet A's stated caller predicate against the current tree produced **106**, not 107, Rust caller files. The report's 107-file count is stale relative to this source snapshot. The current exact-caller scan found no `Vec<UiTreeItemNode>`, `Vec<UiTreeSectionNode>`, or `Vec<BuiltNode>` declarations. Its broad JSON hits include fixtures and separate payload/wire functions, so they are not independently charged here as P5 retained working-set violations. The live WindowMeasure and reactor counterexamples above are sufficient and directly in scope.
- Source-present hostile laws include mounted one-opportunity, cap-plus-one exact owner, terminal-full matching admission, exhausted-generation, document lease incremental close, single-claim transport, and stale/ABA/interrupted-close law symbols (`patches component.rs:961,998,1175,1216`; `document.rs:822`; `kernel.rs:1619`; `engine.rs:1900`). Current worktree and staged scoped `git diff --name-status` showed no deleted audited test file; only the reactor component was modified in the unstaged scoped diff.
- Isolated source verifier passed: `bun -e '...interactivityLiveReconcileSelfTests...';` printed `p5b-live-reconcile-selftest=green`. It is not sufficient: its predicates cover the reconciler/tracker and selected strings but do not reject the live reactor `dirty_render`/`dirty_intents`, renderer `retained`/`pending_rejections` maps, or WindowMeasure rail.
- Scoped `git diff --check` and staged `git diff --cached --check` were clean. Scoped `rustfmt --check` was clean for renderer glue, ProgramBridge, Interpreter, ui-wgpu engine/tree/component, and Shell, but non-clean for reconcile, tracker patches, reactor, document contract, kernel, and shard bridge. No formatter was run in write mode.

## Bounded Repair Packets

1. Replace P5 reactor `dirty_render`/`dirty_intents` and tracker `String` storage with fixed, pre-admitted `SurfaceId`/`UiText` slots and fixed intent queues. Reserve actual backing before insertion; add exact saturation/refusal/close tests.
2. Replace `KernelPoolState` retained/rejection `HashMap`s with a fixed, generation-qualified surface registry. Preserve the rejected patch/document builder in a fixed owner slot until a terminal close or retry outcome is observed.
3. Make document close own its lease until `close_step()` returns terminal. Do not set `closing`/`published` to `None` after one step. On builder/record admission failure, retain an exact failure/handback witness and leave rebuild pending or publish a bounded rejection.
4. Replace `next_seq` with checked nonzero generation allocation that permanently refuses at exhaustion; propagate that refusal without dropping a build.
5. Remove or migrate the production WindowMeasure rails to the fixed retained document/schema. Until then, hard-fail the route rather than clone dynamic actions/items/groups. Add negative source mutations for Group recursion and Select/Slider/Toggle action cloning.
6. Either complete fixed `UiPatch` conversion in the Wasmtime/async host or explicitly remove those selectable lanes; add a host-to-renderer populated-patch law so a default/empty `TurnResult` cannot mask emitted patches.

## Acceptance Conclusion

Do not mark P5b GREEN. The core reconciler and direct retained document transport have several correct pieces, but B1–B5 require the whole mounted path. The live dynamic queues/maps, nonterminal close-owner erasure, wrapping sequence, and production WindowMeasure conversions are all reachable violations.
