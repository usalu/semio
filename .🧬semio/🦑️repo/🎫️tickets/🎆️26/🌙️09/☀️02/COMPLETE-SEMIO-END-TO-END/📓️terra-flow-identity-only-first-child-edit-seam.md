# Flow Identity-Only First Child Edit Seam

## Verdict

**RED — ordinary Flow edits still mutate a cloneable parent `FlowDiff` whose `content` is a new, content-addressed `ArtifactChild` carrying an erased strict `FlowWorkingScene`.** The smallest honest first conversion is one bounded `addWidget` edit against an **already admitted** `SemioMembers::Flow` child. It proves typed child dispatch and leaves parent content identity unchanged; it does **not** claim child genesis, persisted reopening, public factory admission, or atomic parent/member/graph publication.

This is a source audit only. I did not start Cargo/Nx and do not attribute any result from the active shared Flow frontier to this report.

## Current ownership and mutation path

The Flow plugin binds both editor and viewer to `SemioMembers` ([plugin](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🦀️.rs:12), [plugin](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🦀️.rs:30)); its `Flow` arm is the typed `ArtifactStore<SemioFlowSnapshot, SemioFlowMutation>` ([member declaration](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1121)). That is the correct target authority.

Today, however, `AddWidget::handle` builds a host from the *parent* snapshot and returns parent `FlowMutation`s ([command](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:22)). `host_operations` then converts the host fixture delta into parent semantic operations ([bridge](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1860)).

Every ordinary Flow leaf's diff reads `flow_working_scene(base)` and calls `diff_replace_content`; for example, `create-widget` does so at [its diff](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🔺️diff/🦀️.rs:10). `diff_replace_content` remints the content child and stores its local scene in the parent diff ([helper](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:144)). The bridge itself declares that identity changes whenever contents change ([content handle](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:167)); this is incompatible with the stable child-member relation.

`FlowDiff` serializes/clones both whole `artifact` and `content` ([schema](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:15)), while its synchronous `MutationDiff::absorb` overwrites these values ([implementation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs:80)). The generic child retirement deliberately omits the erased local owner ([generic retirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs:302)). Thus merely arranging test disposal cannot make ordinary parent-diff ownership sound.

## Existing typed-child seam

The framework already treats a nonempty `Emit.child_emits` as a composite route ([dispatch branch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20778)). `dispatch_emit_group` requires a pre-existing live `(slot, child_id)` member ([lookup](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20927)), validates the parent's exact dialect, and calls `CompositionCoordinator::dispatch_group` with `genesis = Vec::new()` ([call](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20958)). A child-only emit is therefore a usable first scope: `parent_ops` is empty and the coordinator dispatches a real child edit only after previewing it.

`duplicateWidget` already demonstrates the precise protocol shape. It reads the admitted `SemioFlowSnapshot` via `doc.children.typed_read`, checks a child revision, and emits `ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, ...)` with typed `InsertNode`/`InsertEdge` mutations ([command](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:191), [read/fence](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs:228)). That is a production implementation pattern, not a new transport or queue.

## Exact P0 implementation seam

### 1. Make durable Flow values identity-only

Keep `FlowSnapshot.content` as the stable, owner-free child coordinate established by a future author-side initial-composition receipt. It is not a changing content digest. Remove ordinary `content` replacement and whole-artifact replacement from `FlowDiff`; replace parent mutation diffs with only actual parent scalar fields (`schema`, `camera`, and their legitimate configuration/presence fields). The following current helpers must disappear from the ordinary-diff path rather than be wrapped:

- `flow_content_child_handle[_and_cache]`, `cache_flow_content`, and fallback `flow_working_scene_for_handle` ([Flow source](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:167)); missing child materialization must be a fault, never `Default`.
- `diff_replace_content` and every one of its child-editing callers.
- `FlowSnapshot::from_fixture`/`to_fixture` as a durable cache bridge ([snapshot](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:39)). They may remain as explicit, bounded *temporary* conversion functions supplied with a typed `SemioFlowSnapshot`; they must not recreate or read a local owner.

This is a source cutover, not a compatibility mode: updating the derived schema siblings (`🔗️.graphql`, `🛰️.proto`, `🟦️.ts`) and all fixtures is required at the same time.

### 2. Add one pure, typed host bridge

At the Flow artifact boundary, extract a pure bounded helper conceptually equivalent to:

`flow_child_add_widget(before: &SemioFlowSnapshot, descriptor, x, y, config, session) -> Result<Vec<SemioFlowMutation>, Fault>`.

It must:

1. derive a temporary `FlowFixture` only from `working_from_flow_content_snapshot(before)` ([converter](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:152));
2. run the existing `FlowHost::add_widget` behavior so descriptor/capability semantics stay in the host;
3. inspect the before/after fixture delta; and
4. translate the **single admitted AddWidget outcome** into exactly one `SemioFlowMutation::InsertNode`, including the node position derived from the changed layout.

The generic Flow fixture delta emits `AddWidget` and, separately, `ChangeLayout` ([framework bridge](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:188)). Therefore conversion through the current parent `FlowMutation::CreateWidget` is insufficient: its widget does not itself carry layout. The P0 helper must require exactly one added widget and its exactly corresponding layout entry, reject any unexpected node/synapse/layout delta, and construct `SemioFlowNode` via the existing widget/position mapping embodied in `flow_content_snapshot_from_working` ([mapping](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs:134)). Extracting a one-widget mapper from that mapping is preferable to serializing a whole scene.

This permits `addWidget` only. Remove, rename, connect, reorder, move, and the retained preparation path need their own typed conversion packet; they must not fall back to parent `FlowDiff` while the conversion is in progress.

### 3. Route `AddWidget` through the existing child group

Change `AddWidget::handle` to read the exact child first:

`doc.children.typed_read::<SemioFlowSnapshot>("content", &doc.snapshot.content.child_id)`.

Build `ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, typed_ops)` and return it with **no** `artifact_mutations`. The generic emit route will then validate the registered child, ownership graph, parent dialect, child wire operation, and group policy before the child store changes. It also compensates a failed phase-2 child application ([coordinator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19314)).

No new Flow queue is needed. This P0 operation is bounded by one command, one new node, and the existing typed leaf limits. The handler has no cancellation token; it must remain one synchronous bounded preparation step. Longer searches and multipage work continue to require the retained continuation pattern already used by `duplicateWidget`, including its checkpoint and revision checks. Do not claim cancellation coverage for this P0 direct handler.

## Current non-atomic boundary: excluded from P0 acceptance

The initial/reload admission path remains non-atomic: `commit_child_member` awaits `member.set_owner`, then commits the graph, then inserts the child map/root ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19993)). If the operation is cancelled/aborted across those awaits, this may publish an owner or graph edge before the live map/root. `CompositionCoordinator::dispatch_group` has the same separation for `ChildGenesis`: it creates the member, sets ownership, and inserts the graph relation before returning a `created_children` receipt ([store](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19452)).

The P0 law must pre-register an exact child and pass `genesis = []`; it does not exercise either boundary. A later full-publication packet needs retained owner handback, cancellation before/after each awaited stage, and a single atomic relation/map/root commit. Calling the current API "atomic composition" would be false.

## First mounted RED → GREEN law

Add a distinct native law in the Flow editor command test surface, then add only its exact FQN to [`child-identity-check`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:23). The existing ten laws are lifecycle/identity/render evidence, not a typed ordinary-edit proof.

Suggested law name:

`add_widget_dispatches_one_typed_semio_child_edit_without_repointing_the_parent_content`

It must use a real `VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>` and a real, pre-admitted `SemioMembers::Flow` member—not a fake `SpaceMember`. The fixture may call current `register_child` to establish its premise, but must say that this is only a pre-admitted test condition, not public persistence or genesis proof. It should assert:

1. the stored parent `content` `ArtifactRef` and parent edit count are identical before and after;
2. `InvocationResult.member_edits` contains exactly the child edit, and no parent document edit;
3. typed read of `(content, child_id)` contains one new `SemioFlowNode` with the requested id/kind/label/position;
4. a child typed mutation undo restores the exact prior child snapshot while the parent coordinate stays unchanged;
5. missing child, wrong slot, dialect mismatch, duplicate id, and an unexpected host multi-delta all deny before any parent or child edit; and
6. all fixture-owned member/snapshot owners complete their existing domain retirement before test end.

The current `register_content_child` test helper ([Flow editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2095)) is useful only after its parent local-scene dependence is removed; it currently encodes a child from the cache and is not evidence that a persisted parent can be reopened.

Before Rust, add a language-neutral fixture under the existing Flow editor fixture router and use a third-party independent representation only for the pure bridge rows: exact child input, descriptor, before/after expected typed node, expected unchanged parent coordinate, and denials for duplicate/missing/multi-delta/over-limit. It cannot model `Arc`, `ArtifactChild` local ownership, or a fabricated group receipt. The native law is the sole proof of actual `ChildEmit`/`SemioMembers` dispatch.

Register through the existing router and update the launch **seed** before regenerating `launch.json`; never edit the generated launch entry directly. The current callable command after the source fixture is:

`bun x nx run @semio-tech/flow-plugin:child-identity-check`

## Dependency order and nonclaims

1. Make Flow snapshot/diff identity-only and delete cache fallback paths.
2. Implement and independently fixture the one-widget typed host bridge.
3. Land the bounded pre-admitted `addWidget` native law and exact gate registration.
4. Convert the remaining ordinary child operations one semantic family at a time.
5. Only then combine this with the separate retained `MemberFactory::Open` bridge and the atomic child relation/publication packet for real persisted parent reload/genesis.

This first law does **not** establish a public `Plugin<FlowApps>` factory loaded from durable input, a viewer reopen, an initial child receipt, composed checkpoint pins, a multi-user route, or a complete no-`FlowDiff` runtime until all legacy child leaves and cache readers are removed.
