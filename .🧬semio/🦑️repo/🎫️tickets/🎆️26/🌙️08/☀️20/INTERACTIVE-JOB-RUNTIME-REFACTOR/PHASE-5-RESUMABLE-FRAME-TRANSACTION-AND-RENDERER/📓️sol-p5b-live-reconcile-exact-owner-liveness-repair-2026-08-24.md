# Sol P5b Live Reconcile Exact-owner and Liveness Repair

Date: 2026-08-24  
Status: active caller/consumer migration; isolated P5b source predicate and mutation corpus green; not yet source-audit-ready

## Fresh RED audit disposition

The fresh Terra audit invalidated the earlier source-ready claim. The retained B-tree range restart, allocator-capacity byte estimates, whole mounted source-tree transfer, tracker-first ACK, and populated ordinary-Drop paths were real counterexamples. This report remains an implementation journal until the expanded schema-first producer migration is complete; it must not be used as acceptance evidence.

## Boundary

The original boundary expanded after the fresh audit proved that exact ownership cannot begin after a complete dynamic `ComponentTree` already exists. The authorized boundary now includes UI contract `action.rs`, `builder.rs`, `component.rs`, `accessibility.rs`, `surface.rs`, and `document.rs`; runtime `present.rs`; the exact plugin render/tree-conversion and reactor poll regions; and corresponding renderer/plugin consumers. P5a/P5c/P5d/P5e and unrelated stdio/oracle paths remain excluded. The peer commit `e7bd5ecdf7` remains a preservation boundary.

## B1 — retained census and exact backing

- `UiValue::Map` now owns a generation-qualified fixed arena collection and `SurfaceSemanticMapPage` retains its owned `UiMapCursor`. Each grant calls `advance()` once; no B-tree, restarted search, borrowed iterator, raw pointer, lifetime extension, or unsafe traversal remains.
- Map discovery and value consumption are distinct opportunities on the existing fixed scalar/container/entry/value-depth cursor.
- The earlier allocate-and-charge claim was false because `capacity * size_of` is not allocator backing. Live reconciliation is partly migrated away from hash containers, but its linear containers still use `Vec` backing and the broader tree/component/patch schema still owns dynamic `String`/`Vec`/`HashMap` allocations. This is an open blocker, not accepted backing proof.
- `RecordDiffCursor` emits at most one field operation per grant; the old variable-length `diff_record -> Vec<UiPatchOp>` path is deleted.
- Tree retirement uses fixed `SurfaceTreeRetireCursor`; dynamic `retire_forest` is deleted and presentation depth is rejected before fixed retirement capacity can overflow.

## B2 — patch-credit transfer and fixed reactor owner

- Ready transition splits one ledger reservation into candidate and patch leases only after cursor and previous-owner retirement.
- `SurfaceReconcileReadyPatch` carries generation, patch, and its credit from the fixed tracker ready slot into the reactor.
- The unbounded `RefCell<Vec<UiPatch>>` pending store is deleted. `PendingPatchAuthority` is a fixed 64-slot FIFO authority with reconcile-ready, external, and published-credit states.
- Poll transfers at most one patch. A reconcile patch leaves `SurfaceReconcilePublishedPatch` in the fixed authority until matching ACK or durable per-instance close releases it.
- Pending saturation is checked before tracker take; an intervening refusal returns the exact ready authority to the fixed tracker slot.
- Instance close is retained in a fixed close array and advances one pending owner per later poll. FIFO/revision/ACK/last-valid and the O(1) revision accessor remain intact.

## B3 — accepted saturation close

When tracker terminals are full, close still advances a matching capacity-producing terminal before converting unadmitted, rejected, or surface owners. Accepted B3 order is preserved.

## B4 — transactional permanent generation exhaustion

- Issuance is split into `next_generation` and `commit_generation`.
- Begin, unadmitted, and mounted paths acquire aggregate credit and public handback reservation before committing a generation.
- Rejection checks fixed terminal capacity first and successfully constructs an idle-owner terminal before committing its proposed generation. Refusal restores the exact reconciler without cancellation or scalar mutation.
- Terminal saturation at `u64::MAX - 1` therefore repeatedly refuses without consuming `u64::MAX`; after one slot is freed, maximum is issued once and exhaustion remains permanent.

## B5 — fixed O(1) public handback

- The unbounded intrusive `Box` chain, linear generation retrieval, and uncredited `from_*` constructors are deleted.
- A fixed 384-slot registry reserves slot/epoch/generation before retained-state construction. Drop publishes only into its exact reservation.
- `SurfaceReconcileHandbackKey` retrieves by `registry.slots[key.slot]` and validates epoch/generation, so public take is O(1).
- `try_from_reconciler` and `try_from_reserved_sources` return either a credited terminal or the exact input owners.
- `close_surface_reconcile_handback_one` advances one abandoned public owner per mounted poll and republishes it into the same reservation until terminal-empty.

## Hostile fixtures

Runtime reconcile additions:

- `retained_map_page_advances_each_key_once_without_rewalking_prior_entries`;
- `allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation`;
- `public_drop_handback_is_lossless_at_terminal_cap_and_plus_one` now covers fixed cap acceptance, exact cap+1 reconciler return, O(1) keyed take, and incremental close;
- existing identifier/page/depth/zero-fuel/cancel/stale/persistent-credit fixtures remain required.

Mounted tracker additions:

- `terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation`;
- existing cap+1, mounted pre-reservation, FIFO, ACK, ABA, all-owner close, B3 terminal saturation, and permanent maximum fixtures remain required.

## Permanent verifier

The P5b predicate now requires the actual retained map page/range path, every production backing-admission branch, retained field diff, fixed retire cursor, split patch credit, fixed reactor pending/publication owner, ACK/close release, reserve-before-generation-commit order, fixed public registry, O(1) take, and all hostile fixtures.

The original corpus is expanded with faithful mutations for BTree re-walk, omitted Vec admission, dynamic retire forest, eager variable diff, dynamic reactor pending storage, missing published ACK, premature patch-credit release, generation commit before reservation, rejection generation before terminal capacity, saturating maximum, lossy handback, linear public take, uncredited terminal construction, and missing new fixtures. The isolated permanent P5b baseline and every P5b mutation pass/kill.

## Scoped evidence

- Isolated live P5b predicate baseline after assembling action/builder/component/accessibility/surface/document/present sources: PASS (`[]`).
- Isolated `interactivityLiveReconcileSelfTests(process.cwd())`: PASS (`p5b-mutations=green`). Ineffective historical mutations were replaced with ordered, source-present mutations for mounted entry-point regression, exact root loss, dynamic fixed-list backing, dynamic reconcile ops, commit-before-reservation, and clone-to-abort.
- Current and cached scoped `git diff --check` over the four production/verifier files: PASS.
- `rustfmt --edition 2024 --config skip_children=true --check` parsed all three Rust sources. Formatting-clean is not claimed because shared files retain current peer formatting deltas outside this remediation.
- Whole `bun 📜️script.ts verify interactivity --self-test`: RED before P5b in the unrelated Puzzle fill-envelope baseline (`FillBuilder still materializes a whole preview/result envelope inside one worker grant`). This is recorded as a global-only external failure, not relabelled as P5b.

No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run. No compile, allocator-runtime, browser, or runtime-acceptance claim is made. P5b is not yet returned for independent audit. A fresh independent Terra audit is required only after the fixed/admitted producer, dynamic contract schema, patch publication, false-handback, and deep-close fixtures/mutations are complete.

## Current residual blockers

This section supersedes any earlier future-tense “required” wording above:

- Selected P5b contract schema now uses fixed `UiText`, `UiFixedList`, `UiFixedMap`, `UiFixedBytes`, fixed document tables/ops, and fixed admitted `BuiltChildren`; `TreeNode` is the `BuiltNode` alias. Dynamic `String`/`Vec`/`HashMap` fields named by the P5b schema predicate are gone.
- `ComponentTreeProducer` is mounted behind the pre-materialization grant, generation/cancel/deadline qualified, complete-only, fixed-depth, duplicate-aware, and incrementally closeable. Its deep maximum/+1, stale, cancel, deadline, duplicate, and complete-only fixtures are present.
- Live reconcile traversal/postorder/seen/id/removal/ops and retained indices are fixed linear owners. Semantic value duplication and component/binding/menu patch duplication are fallible credited aliases; live saturating counters are gone.
- Remaining blocker: plugin/renderer/reactor callers still contain old `String`/`.into()`/`ActionId::v1`/`BuiltNode::new`/`.build`/infallible child/action calls. These must migrate to `UiText`, `try_v1`, `try_new`, `try_build`, and the exact `try_*` builder APIs before a compile-plausible handoff.
- Remaining blocker: `limits.rs` and exact renderer document-application consumers still expose the former synchronous whole-patch validation/application shape. Fixed text/credited-copy compatibility is partly migrated, but the retained one-op/one-node/one-child application authority is not complete.
- Populated fixed-value serde remains intentionally refused. Every live populated action/catalogue construction route must use `UiListBuilder`/`UiMapBuilder`; no route has yet been proven unreachable strongly enough to accept a fail-closed-only producer path.

## 2026-08-24 continuation — fixed renderer document seam

- `document.rs` now publishes an eight-slot fixed `UiDocumentArena`. A generation/epoch-qualified `UiDocumentBuilder` admits one exact `UiNodeRecord` owner at a time and publishes only after the declared root is present.
- `UiDocumentLease` has an explicit eight-alias cap, returns a credited single-node `UiDocumentNodePage`, and retires one record or scalar per `close_step`. Ordinary builder/lease Drop only marks the fixed slot for mounted retirement; it does not walk the complete document.
- `SurfaceDocumentProducer` validates generation, cancellation/deadline budget, surface, revision, root, and retained node count on every opportunity. It credits and admits exactly one retained reconciler record before yielding. `SurfaceDocumentOutcome` is the direct renderer transfer owner; no `UiSnapshotState`/`UiNode`/`Vec` compatibility adapter is part of this seam.
- Reactor poll advances `close_ui_document_page_one` once beside the accepted reconciliation and fixed-value retirement drivers.
- Exact fixtures cover complete-only publication, generation/revision/page metadata, credited alias lifetime, incremental close, fixed maximum, and maximum + 1 returning the exact rejected node owner.
- The permanent P5b predicate and ordered mutation corpus now kill dynamic document slots, missing lease generation, whole-document reads, omitted mounted close, missing/stale producer checks, whole retained collection, and missing outcome ownership.
- Reactor command cursor/timer cleanup now uses checked page-index/live-count advancement and fallible fixed page assembly. Saturating terminal cursor increments and the named panicking timer/page/close transitions are absent and mutation-guarded.

Current scoped evidence after this continuation:

- `bun -e 'import {interactivityLiveReconcileSelfTests as t} from "./📜️script.ts"; t(process.cwd()); console.log("p5b=green")'`: PASS (`p5b=green`).
- Edition-2021 `rustfmt --check` parsed `document.rs`, `reconcile.rs`, and reactor `component.rs`; formatting cleanliness is not claimed.
- Scoped `git diff --check` for the document/reconcile/reactor/verifier files: PASS.
- No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run. Compile/runtime acceptance is not claimed.

Residual blockers remain unchanged in kind: the Packet A caller migration and Packet B renderer conversion must land without compatibility adapters; `limits.rs` still needs retained transactional patch application; populated nested DSL-to-UI values still need a retained iterative producer rather than synchronous recursion; the completed aggregate then requires a fresh independent Terra source audit.

## 2026-08-24 continuation — retained patch and fixed turn ownership

- `UiPatchApplyProducer` now owns the transactional clone/apply/remove/validate phases. Each opportunity advances one patch census item, source record clone, patch op, removed child/node, validation node/child, or dangling-node check. Generation mismatch and cancellation reject without publishing the draft; an expired deadline advances nothing.
- Ready and rejected results retain the exact candidate/original/patch owners and expose incremental `close_step` plus final `take_state`. A fixed eight-slot generation/epoch handback arena is reserved before admitted work; abandoned producers/outcomes/rejections move remaining state, patch ops, removed record, and duplicate-key aliases into mounted one-owner retirement. Arena maximum + 1 returns the original state and patch in the rejection.
- Reactor poll now advances `close_ui_patch_owner_one` once. The former production `apply_patch` entry point is test-only, so mounted consumers cannot re-enter synchronous whole application.
- `ArtifactApp::render`, `ArtifactEditor::render`, and `ArtifactViewer::render` now return the public `UiAssemblyResult<ComponentTree> = Result<ComponentTree, PluginAssemblyError>`. Editor/viewer forwarding preserves that exact result. `PluginApp::render` is the sole outer `Fault` translation boundary; both app render calls map the owned assembly error and no render cache/revision transition panics.
- Kernel `TurnResult.ui_patches` is now `UiTurnPatches`, a named fixed one-patch authority backed by `UiFixedList`. It has exact fallible push, borrowed/owned iteration, fixed serde admission, maximum + 1 exact-owner refusal, and one-op/one-patch close. Reactor publication and WIT conversion consume this authority directly.
- Shard turn encoding returns `Result<semio_framework_actor::TurnResult, Fault>` and maps patch/effect/ingress serialization failure instead of replacing it with empty bytes. Exact kernel `TurnResult` constructors in host/runtime/run/shard use `UiTurnPatches::default`.
- Command ingress uses two fixed handback cells and restores into the exact cell checked out at turn start. The panicking `previous.is_none()` owner transition is gone and mutation-guarded.

Additional hostile fixtures and verifier mutations cover patch handback cap/ABA, abandoned partial producer close/reopen, turn patch cap + 1 exact return, fixed serde visitor cap + 1, incremental turn patch close, dynamic turn patch storage, missing exact push, infallible shard encoding, omitted handback drivers, and panicking command ingress.

Current scoped evidence:

- Edition-2021 `rustfmt --emit stdout` parsed kernel, contract limits, central plugin, reactor, shard, host runtime/component, and run exact consumer files.
- Scoped `git diff --check` over the P5b files: PASS.
- Isolated `interactivityLiveReconcileSelfTests(process.cwd())`: PASS (`p5b=green`), including every ordered mutation added above.
- No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run. Compile/runtime acceptance is not claimed.

The dynamic JSON→DSL→UI action adapter is now deleted. `ActionFactory::action` accepts an already-admitted `Option<UiValue>`, and the framework history action constructs its populated argument through fallible `UiMapBuilder` admission. Residual blockers are narrower but still source-audit blocking: the two `ui_value_to_json` recursive collection folds remain in the typed command bridge; external Packet A render/helper/action callers must finish propagating `UiAssemblyResult` and constructing fixed values; Packet B must consume the live document/patch/turn authorities without reconstructing `UiSnapshotState`/`UiNode`/`Vec`; and the shard JSON representation still relies on nested contract serializers whose populated fixed-list policy must be reconciled with the required paged wire encoding. A fresh Terra audit is not requested yet.

## 2026-08-24 continuation — lossless turn abandonment and nonpanicking ready handoff

- First-patch admission into `UiTurnPatches` now reserves one exact slot in a fixed 64-slot epoch-qualified retirement arena before transferring the patch. Capacity + 1 returns the identical patch owner. Populated ordinary `Drop` moves the fixed patch page into that reservation, and reactor poll advances `close_ui_turn_patch_owner_one` by at most one patch operation or patch owner.
- Local hostile fixtures cover retirement capacity + 1, stale epoch release refusal, and one-operation/one-owner close. The P5b verifier now requires the fixed arena, exact reservation field, populated Drop, mounted close driver, and fixtures; ordered mutations kill missing Drop, dynamic retirement backing, and omitted close driving.
- `SurfaceReconcileJob::take_ready` no longer panics or strands an admission ledger credit when candidate/patch/credit state is incomplete or the generation-qualified credit split refuses. Every refusal restores the exact candidate, patch, and unsplit credit to the unchanged job.
- The production semantic map page, traversal completion, completed census, fresh-record assembly, and deep tree retirement paths no longer use `expect` owner transitions. `UiPatchApplyProducer` validation no longer aliases an empty fixed stack to index zero.
- Edition-2021 `rustfmt --emit stdout` parsed the exact touched Rust sources, scoped `git diff --check` passed, and the isolated P5b baseline plus ordered mutation corpus reported `p5b=green`. No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run.

The shard transport remains source-audit blocking. `to_actor_turn_result` still performs whole `serde_json` encoding, while populated fixed contract collections deliberately refuse ordinary serde and the renderer/run consumers still invoke whole JSON decode. No success claim is made for populated turn patch transport, and no token/registry adapter was introduced. The required result is still a retained bounded page codec with direct fixed-owner decode/application and exact interrupted-close ownership.

## 2026-08-24 final core continuation — retained transport, fixed table/grid owners, and typed-command cursor

This section supersedes the final blocker paragraph immediately above for the P5b core boundary.

- Kernel turn patches now cross the shard boundary through a fixed 64-slot, epoch/session-qualified `UiTurnPatchTransportProducer` and single-claim `UiTurnPatchTransportLease`. The producer checks session, cancellation, and deadline on every opportunity, advances one patch or operation page, publishes only the complete 32-byte token, and rejects duplicate publication. Abandoned building/published/checked-out owners return to incremental close.
- Shard encoding consumes the exact `UiTurnPatches` owner. Fallible effect/ingress metadata is completed before token transfer, so an early metadata fault leaves producer Drop able to close the retained owner. Renderer glue and the run loop claim the lease directly; non-target outcomes request session close. Reactor poll advances one transport close opportunity. Raw host/renderer `complete` seams propagate transport refusal instead of passing a `Result` as an actor outcome.
- Whole patch JSON transport is absent: scoped source census finds zero `serde_json::to_vec(&result.ui_patches)` and zero `serde_json::from_slice(&result.ui_patches)` matches.
- `GridLayout.columns` and `GridLayout.rows` are `UiGridTracks = UiFixedList<GridTrack, 32>`. `try_push_column`/`try_push_row` return the exact maximum-plus-one track; populated ordinary serde remains refused in favor of retained page transport.
- `TableRowAction` owns contract `UiText`, `Label`, and `ActionBinding`; renderer `ActionDescriptor`, `TableCell`, `UiTreeItemAction`, and whole row JSON staging are removed from `TableWindowKit::render_rows`. `TableRow` cells/actions and `TableRowsView` columns/rows are fixed 32-entry owners with exact fallible push. `render_rows` consumes the view and constructs semantic header/row/button nodes directly.
- `TableRowsView` reserves one of 64 epoch-qualified retirement slots before accepting a populated row or column. Abandoned views hand their exact fixed owners to that reservation; reactor poll advances `close_table_rows_view_one` by one row/action/cell opportunity. Hostile fixtures cover exact row maximum plus one and multi-opportunity abandonment retirement.
- The final two recursive `ui_value_to_json` list/map folds are deleted. `UiCommandJsonProducer` holds a fixed depth-64 frame stack over credited `UiListCursor`/`UiMapCursor` owners, checks a bounded item census, and advances one scalar or collection page between async yields. Deadline advances nothing; cancellation and depth-plus-one preserve the original credited source owner. `ArtifactApp` and `ArtifactEditor` intent defaults await a complete candidate before invoking their existing typed command parser.

Hostile fixtures added in this continuation cover transport round-trip/single claim, truncation/stale/cancel, fixed transport maximum plus one/session close, grid maximum plus one, table row maximum plus one, table abandonment retirement, typed-command cancellation/deadline owner preservation, and typed-command depth plus one. The permanent verifier now includes 140 ordered P5b mutations across reconcile/tracker/reactor, fixed value/schema, document/patch/turn/table/grid/typed-command ownership, and document producer groups; the isolated baseline is green and every mutation is killed.

Final scoped source evidence for the core packet:

- Edition-2021 `rustfmt --emit stdout` parsed the exact kernel, contract action/layout/document/limits, reconcile, reactor, shard, central plugin, host activation, renderer runtime/glue, and run sources touched by the core packet.
- Scoped `git diff --check` over the same production files, verifier, and this report passed.
- `bun -e 'import {interactivityLiveReconcileSelfTests as t} from "./📜️script.ts"; t(process.cwd()); console.log("p5b=green")'` passed with `p5b=green` after the final typed-command and table mutations landed.
- Scoped censuses are zero for recursive `ui_value_to_json` calls, whole turn-patch JSON encode/decode, dynamic `GridTrack` vectors, renderer-backed `TableRowAction`, and dynamic `TableRow`/`TableRowsView` row/action fields.
- No Cargo, Nx, Wasm, browser, network, broad build, allocator-runtime, or UI-runtime command was run. Compile/runtime acceptance is not claimed.

The P5b core is source-audit-ready. The aggregate Phase-5 handoff must still include the concurrently active Packet A external caller/test propagation and Packet B renderer consumer work before a fresh independent Terra acceptance audit is requested; this core report does not relabel those concurrent packets as complete.
