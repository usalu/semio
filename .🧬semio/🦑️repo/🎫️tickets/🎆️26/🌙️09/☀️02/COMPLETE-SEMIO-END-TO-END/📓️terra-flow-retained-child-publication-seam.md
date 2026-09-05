# Flow `addWidget` Retained Child-Publication Seam

## Verdict

**RED — the current Flow `addWidget` action has no admissible typed UI path.** The reported exact-law failure happens before the handler because the test helper builds a registry-less app, and a real registry would still reject the action: it remains `BatchOnlyPendingRewrite`, Flow registers no owned factory for it, and the framework deliberately rejects all typed `Child` output. The existing direct handler is useful semantic evidence, but calling it or `dispatch_emit_group` from a worker would be a test bypass, not a retained publication implementation.

This is a source audit only. No native or runtime success is claimed.

## Current evidence

| Boundary | Current fact | Consequence |
|---|---|---|
| Flow action declaration | `addWidget` is declared at [editor `🦀️.rs:1922](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1922), but classified `BatchOnlyPendingRewrite` at [1963](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1963). | A UI request is correctly denied as not migrated. Do not flip this bit first. |
| Test app | `flow_app()` intentionally uses `VcsArtifactApp::new` with no `AppActionRegistry` at [2104](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2104); real actions require `AppActionRegistry::from_definition` [framework `🦀️.rs:12030](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12030). | A bare-app law cannot prove a UI action. It must use the real plugin/app factory or `flow_app_with_registry()`. |
| UI admission | A UI action is allowed only when classification is `Migrated` [framework `🦀️.rs:12016](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12016). | The manifest claim must follow, not precede, owned factory plus publication support. |
| Flow app-owned routes | `FLOW_DIRECT_STORE_TOOL_IDS` excludes `addWidget` [editor `🦀️.rs:763](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:763), and `build_tool_job` returns `None` outside host/direct routes [1657](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1657). | Even a flipped manifest hits missing owned factory/builder. It must be a separate Child factory, not an entry in the direct store factory. |
| Existing semantics | `add_widget::handle` reads exact `content` child and returns one `ChildEmit<SemioFlowSnapshot>` [add-widget `🦀️.rs:49](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:49). | Reuse/extract this child mutation planning; do not recreate it from a global/default Flow scene. |
| Framework contracts | `Child` already exists in both `ArtifactToolPublicationLane` [framework `🦀️.rs:12699](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12699) and result-page wire lane [13110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13110). Exact factory registration already requires an exact nonempty lane contract and `Migrated` factory [12832](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12832). | This is a missing retained owner/commit path, not a missing enum or result-wire codec. |
| Typed operation state | `MountedTypedCommandFullOperation<A>` only owns scalar store publication [16104](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16104). `Child` results are not included in result ACK handling [16254](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16254). | It cannot retain all member preparation, roots, group receipt, or Child ACK authority. |
| Explicit rejection | Typed completion rejects any `emit.child_emits` before its contract check [22459](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22459). | A `Child` contract alone cannot make `addWidget` work. |
| Captured context | Typed start snapshots `ChildContentView`, canonical revision, generation, and cancellation lease before worker dispatch [22639](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22639). | This is the correct source of captured child identity/freshness for a retained Flow work item. |
| Current grouped foreground path | `dispatch_emit_group` performs group dispatch, then separately advances child-content roots [20896](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20896), specifically group first [20958](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20958), roots later [20961](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20961). | It is semantic precedent only. Its post-group fallible root-publication window means it is not the async retained commit primitive to call directly. |

## Smallest correct P0

### 1. Make the retained framework owner child-aware

Add one framework-owned, strictly closeable child arm to the typed operation rather than a generic erased disposer:

`MountedTypedCommandFullOperation<A, M>` (or a contained `RetainedChildGroupPublication<A, M>`) must retain the actual `M: SpaceMember + MemberFactory` authority held by `VcsArtifactApp<A, M>`.

Its owner must contain, until one terminal outcome:

- the original `ChildEmit` values, closed with their existing bounded `close_one` ownership;
- operation id, cancellation lease, actor, exact parent ref/dialect, captured parent revision/generation, and the captured `ChildContentView` identity;
- each target's slot, child id, full dialect/owner/ref, captured member/root generation, and preflighted root-publication capacity;
- a new retained `CompositionCoordinator` prepared-group state/receipt. It must drive preparation, group history/log sealing, commit/abort, and bounded retirement without exposing mutable child stores to the worker;
- root swap/retirement authority and an idempotent committed receipt for result retransmission.

The existing `Child` lane should then describe the bounded committed receipt, not mutation bytes. A minimal `ChildPublicationResultV1` holds invocation id, committed child refs/edit ids, generation(s), and count. The existing fixed result page capacity and `Child = 5` wire code are adequate; Child must be included in ACK/pending-publication logic so ACK retry cannot create a second commit.

### 2. Use one shared retained group-publication primitive

Factor the actual group work behind a retained `begin/drive_one/abort/terminal_is_empty` coordinator API. `dispatch_emit_group` becomes a foreground adapter to this same primitive; it must not remain a second direct commit implementation. The retained sequence is:

1. On the first publication turn, take the child output once and reject already-cancelled work.
2. Validate current parent revision/generation **and** every current child root/ref/owner/dialect/member generation against the captured view. Check exact declared slot/kind, no duplicate `(slot, child)` target, and reserve every root-generation slot before starting the group.
3. Drive bounded member preparation. Check cancellation/freshness again directly before the irreversible transition.
4. Linearize one all-member group commit, grouped history/log seal, and all child-content root swaps as one transaction boundary. This repairs the current `dispatch_group`-then-root-publication window rather than reproducing it.
5. After linearization, cancellation cannot roll back the committed group. It may only govern result delivery/retirement; acknowledgement/retry returns the same receipt and never calls group commit again.

Before linearization, cancellation, stale roots/generation, capacity exhaustion, malformed/missing child, or failed preparation produces no child edit, no command-log row, and no root swap. The retained group owner—not a raw `Drop` or a global queue—must then close every emit, prepared member owner, reservation, root retirement, and worker session incrementally to terminal empty.

### 3. Supply the narrowly scoped Flow factory

Add a separate `FlowChildGroupJobFactory`, with only `TOOL_IDS = ["addWidget"]`, exact `Child` contract, exact Flow schema, migrated classification, fixed input/output/work bounds, and ordinary cancellation/close implementations. It is distinct from `FlowDirectStoreJobFactory`; adding `addWidget` to the latter would incorrectly model a multi-member commit as a parent/config store operation.

Its retained work must use the captured `ArtifactOwnedToolJobContext.children`, reuse the current `add_widget` typed `SemioFlowMutation::InsertNode` planner, and produce exactly one `ChildEmit::of::<SemioFlowSnapshot, _>`. It must not query a later live child map, rebuild a default scene, or retain `FlowWorkingScene` through an unclosed local owner. Only once this factory and framework child commit arm are registered should Flow change `addWidget` to `Migrated`. Other Flow batch-only actions stay unchanged.

## Required RED-to-GREEN proof

1. **Language-neutral `flow-retained-child-publication-v1` corpus.** One accepted `addWidget` targets the declared live `content` SemioFlow child and yields one insert/group edit, unchanged parent content coordinate, one Child receipt, and a group undo/redo witness. Hostiles: wrong slot/id/dialect/owner, duplicate emits, absent/stale child, parent or child generation mismatch, reservation saturation, malformed arguments, and an undeclared Child contract.
2. **Lifecycle traces.** Cancel before worker, during worker, and after preparation/before linearization: zero child edit/log/root generation change and terminal owner. Cancel after commit: exactly one committed edit and one idempotent result/ACK sequence. Every row pins bounded prepare/close progress and total retirement.
3. **Framework native law.** A real composed `VcsArtifactApp<A, M>` proves typed Child output is rejected until the retained group owner is registered, then commits once, propagates failure/cancel correctly, and drains its group owner.
4. **Flow native law.** Construct via `Plugin<FlowApps>::create_app` (or the real registered `flow_app_with_registry()`), issue UI `addWidget`, drive typed operation/result ACK with small grants, inspect the actual child `SemioFlowSnapshot`, command group, and Child result page, then close all Flow owners. It must never call registry-less `flow_app()` or direct `dispatch`.
5. Register the focused Bun/Nx launch through the launch seed followed by generation. No direct generated `launch.json` edit. The current native test failure and this source audit are not evidence that the packet passes.

## Explicit nonclaims

- Current direct `add_widget::handle` coverage is not typed UI execution.
- Current `dispatch_emit_group` is not an all-or-nothing retained transaction.
- Existing `Child` lane enum/wire value does not implement Child publication.
- No member opener, Flow bootstrap, client lease, socket, or renderer claim follows from this P0.
