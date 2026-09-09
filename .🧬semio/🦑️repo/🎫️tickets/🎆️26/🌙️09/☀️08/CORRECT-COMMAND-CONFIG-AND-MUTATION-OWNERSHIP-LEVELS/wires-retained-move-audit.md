# Wires Retained Pointer-Move Ownership Audit

## Scope and conclusion

This is a read-only source audit of the Wires `canvasPointerMove` migration. It
confirms that drag state belongs solely to the exact captured canvas window, while
`MoveNode` remains a durable Wires document operation. The narrow ownership
target is therefore:

| concern | owner and publication lane |
| --- | --- |
| active node and last pointer coordinates | `WiresCanvasTransientOwner` on the captured canvas `WindowTransient` |
| node position | the Wires parent `ArtifactStore<WiresSnapshot, WiresMutation>`, published as `WiresMutation::MoveNode` |
| command preparation/cancellation | a Wires document one-item preparation, with retained exact base root |

The implementation must **not** call the existing `MoveNode::diff` or a generic
one-step `Mutation::diff/apply` from retained work. Both are full-document
operations today. There is no safe shared bounded document-clone factory to
reuse. A Wires-specific staged candidate (or a new genuinely bounded shared
`DslValue`/content cursor) is a prerequisite, not an optional refinement.

The companion
`wires-window-transient-ownership.md` records the already-complete concrete
canvas owner and retained down/up work; this audit only specifies the remaining
move and retirement work. It did not run a new test command.

## Source-confirmed current path

1. `✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs` still synchronously reads
   `WiresConfig.drag_node_id/drag_last_x/drag_last_y`. It finds the node through
   `find_board_node`, emits `MoveNode`, writes `SetDrag`, and uses a
   `drag:<id>` coalesce key. This is the sole remaining Wires
   drag-**motion** config consumer; `setActiveExample` also writes its reset.
2. `✏️editor/🦀️.rs` only registers `canvasPointerDown` and
   `canvasPointerUp` as retained. `WiresWindowDragWork` already requires the
   captured `WindowTransientSnapshot` and validates
   `WiresCanvasTransientOwner`. Down scans
   `content.local_owner::<WiresWorkingScene>()`, one object field at a time;
   up publishes an exact-window `SetDrag { None, 0, 0 }`.
3. `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🦀️.rs` defines
   precisely that concrete canvas-only owner. It is registered by
   `ReasoningWiresPlayApp::register_window_transient_owners`.
4. `✏️editor/🎚️config/🦀️.rs` contains only the three drag fields and
   `SetDrag`. It and `WiresConfigPreparationFactory` are removable once
   movement publishes through the document lane.
5. `🧬️schema/🧬️mutations/🧭move-node/🔺️diff/🦀️.rs` calls
   `wires_working_board(base)`, which clones the working board. Its
   `diff_board_fixture` path builds `nodes.to_vec()`/`edges.to_vec()` and
   invokes `wires_content_child_with_owner`; that serializes a neutral graph
   and rehashes it. `apply` also clones the Wires snapshot. This establishes
   the unbounded barrier.

## Smallest clean migration

### 1. Retain pointer move beside down/up

Make the direct `canvasPointerMove::handle` fail with the same
retained-owner fault shape used by direct down/up; it must no longer accept or
read config. Add `canvasPointerMove` to:

- `WIRES_RETAINED_TOOL_IDS`;
- `WIRES_RETAINED_PUBLICATION_CONTRACTS` with exactly
  `Artifact` and `WindowTransient` lanes;
- the bounded-first-step proof list;
- the retained route fixture/manifest, changing its classification from
  `BatchOnlyPendingRewrite` to `Migrated`.

Change the app generic parameters and command signatures from
`WiresConfig/WiresConfigMutation` to framework
`NoConfig/NoConfigMutation`. This is an established app shape, for example
`✏️s/🔌️plugins/🔋️energy/…/✏️editor/🦀️.rs`. Remove the Wires config subtree,
its custom preparation, config owners/disposer/factory, app schema, fixtures,
and all config test references; do not leave an empty Wires-specific config
record.

### 2. Use the captured state, then boundedly discover the durable target

Extend `WiresWindowDragWork`, or introduce a dedicated
`WiresPointerMoveWork`, with this state machine:

1. Read `window.get::<WiresCanvasTransientOwner>()`; reject absent/wrong-kind
   or invalid payload before any work. If it has no `drag_node_id`, complete
   with no document mutation and no transient write.
2. Materialize only
   `input.snapshot.content.local_owner::<WiresWorkingScene>()`. This is the
   actual Wires local owner. Do not assume an `Arc<SemioGraphSnapshot>`.
3. Reuse down's bounded scan: inspect one `DslValue` object field per
   `step`, retaining only the target node id and its finite `x/y` values.
   The work records the cursor and returns `Progress { stage:
   "canvas-move-hit", … }` until resolved. It must not call
   `find_board_node` or `wires_working_board`.
4. On a found target, compute
   `new = current + (payload - transient.last) / max(camera.zoom, 1e-6)`,
   retain a `MoveNode`, and ask the Wires document preparation to make its
   durable candidate. Only after the document edit is prepared may completion
   publish both that `MoveNode` and a `WindowTransientMutation::SetDrag`
   for the exact captured `window_id`.
5. If the captured node is absent from the current Wires scene, complete with
   no artifact mutation and reset that captured window to
   `SetDrag { None, 0, 0 }`. This prevents a stale drag id from surviving a
   concurrent delete or replacement. Pointer-up takes the same reset path.

The move extent must be derived from valid input and the bounded scan/candidate
capacity. Returning the blanket `WIRES_RETAINED_WORK_ITEMS` after a payload
check is only sound if every downstream clone/materialization/hash stage is
also cursor-bounded by that capacity.

### 3. Prepare the document in real bounded stages

Add `build_document_store_one_item_preparation_factory` for
`WiresSnapshot/WiresMutation`. The existing `WiresConfigPreparationFactory`
cannot be repurposed: it is typed to the disappearing config and its first
advance uses `Mutation::diff`. The Forms generic preparation has the same
one-step `inverse/diff/apply` shape, so it is also unsuitable for Wires.

The document preparation must retain the request's `SnapshotRead`,
`MoveNode`, description, and live authority, and perform only bounded units:

1. Validate operation/generation/base revision, finite coordinates, identifier
   and description byte limits, document lane, and a source-derived work
   ceiling. Reject before allocating an unbounded candidate.
2. Build the changed `WiresWorkingScene` with a resumable clone cursor that
   accounts for every node, edge, `DslValue` field, nested value, and copied
   byte. Patch just the discovered target coordinates while cloning.
3. Incrementally construct the corresponding neutral graph bytes/hash and the
   replacement `WiresContentChild`. Factor the current
   `wires_content_child_handle/wires_content_child_with_owner` construction
   so the cursor produces the exact same child identity and attaches the
   completed `Arc<WiresWorkingScene>`; calling either current helper at the
   final step would reintroduce a full serialization/hash.
4. Build the post `WiresSnapshot`, inverse `MoveNode`, and protocol edit;
   prepare exactly one item through the supplied live authority. Progress
   checkpoints must cover these stages and their retained bytes.

No current shared store or DSL facility supplies step 2 or 3. The closest
retained materialization cursor is Note-specific, while the generic store
contract explicitly leaves splitting a mutation to the domain. The smallest
clean change is therefore a Wires-owned cursor. If a framework
`DslValue` clone/serialization cursor is added first, Wires should use that
interface; it must not hide ordinary `Clone`, `to_vec`, JSON serialization,
or hashing within a reported one-item step.

### 4. Keep child authority and retirement exact

For the current Wires design, the canonical durable authority is the **parent
Wires document store**. The content child is an immutable
`ArtifactChild<SemioGraphSnapshot>` whose real local materialization is
`WiresWorkingScene`; `WiresSnapshot` retirement in
`🧬️schema/♻️retirement/🦀️.rs` explicitly takes that owner and returns
`BudgetExhausted` until it can do so. The document candidate must consequently
create the new child through the bounded equivalent of
`wires_content_child_with_owner`, never mutate the shared `Arc` and never
attach a different neutral-graph local owner.

Turning this into an independently mutable composed SemioGraph child would be
a separate composition migration. Current Wires has no registered live child
store and its child target uses the stable artifact id `wires-content` rather
than the generated child id expected by composition admission. Do not add an
ad-hoc child mutation path as part of pointer move.

On cancellation/close, release the candidate, prepared value, mutation,
description, then return the exact base `SnapshotRead` to its registry before
dismissing the authority. Each release must consume a granted budget and
`terminal_is_empty` must prove all owners are gone. The shared retained
placement fix must release the projection cache before the document
snapshot-return retirement pump; otherwise this Wires preparation can retain a
displaced root and deadlock close.

### 5. Reset lifecycle and undo grouping

`setActiveExample` currently emits `LoadDocument` and a config `SetDrag`
reset. Once config is gone it cannot name every concrete canvas itself.
`LoadDocument` application must reset Wires canvas transient state for all
windows attached to the replaced document, and it needs a multi-window test.
Resetting only the caller would leave stale drag identities in other windows.

The old synchronous move requests `coalesce_key: drag:<id>`. The retained
one-item request/authority API exposes no retained amend-last operation. A
fresh edit per move changes undo behaviour. Preserve one-undo-per-gesture only
if the retained store protocol gains an explicit bounded gesture/coalescing
admission path; otherwise record the intentional per-move history behaviour
and update the existing drag/undo test. Do not claim coalescing survives merely
by putting the same string in a new edit.

## Required validation

1. Contract/route fixture: all three pointer tools are migrated; down/up have
   `WindowTransient` only and move has exactly `Artifact +
   WindowTransient`; config has no wire or schema.
2. Real retained single-window sequence: down, move, up yields the expected
   `MoveNode` delta and one exact-window `SetDrag` per successful move;
   document history/inverse proves durable move and config is absent.
3. Two canvas windows: a drag in A cannot move from B; wrong owner kind and
   stale generation reject without publication.
4. Work trace: a large scene demonstrates one `DslValue` field per hit-test
   turn and progress checkpoints throughout scene copy/content materialization
   and hashing.
5. Preparation: invalid/non-finite/oversized input rejects before publication;
   cancellation at each stage leaves document and window state unchanged;
   close returns the exact base root and reaches terminal-empty under bounded
   grants.
6. Replacement and deletion: missing target and `LoadDocument` reset every
   affected canvas drag state.
7. Retain the native mutation oracle for forward/inverse `MoveNode`; it must
   match the language-agnostic contract fixture after the staged path is
   introduced.

## Evidence locations

- Current move/config path:
  `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs`
- Retained down/up, factory, route registration:
  `…/✏️editor/🦀️.rs`
- Exact canvas transient owner:
  `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️canvas/🫧️transient/🦀️.rs`
- Working-scene local owner/content handle:
  `…/🦀️.rs` at the Wires artifact root
- Existing whole-board diff:
  `…/🧬️schema/🧬️mutations/🧭move-node/🔺️diff/🦀️.rs` and
  `…/🚪️io/🔺️diff/📝️text/🦀️.rs`
- Exact Wires snapshot retirement:
  `…/🧬️schema/♻️retirement/🦀️.rs`

## Shared cursor reuse addendum

Further source inspection found the existing Store `ArtifactCanonicalJsonCursor` and `ArtifactCanonicalJsonReader`. They already encode typed immutable sources in at most 256-byte chunks, preserve native field order, and retain string escape progress. The Wires preparation should reuse this encoder for the neutral content projection and edit sealing. It does not provide a bounded owned `DslValue` copy.

The missing copy operation belongs beside the framework `DslValue`, not inside Wires or the OS store. A source-owning clone cursor will copy one structural item or at most 256 UTF-8 bytes per advance, enforce depth and retained-capacity limits before allocation, and dispose partial output in bounded steps on cancellation. Wires will supply its exact immutable snapshot read through a domain-owned source selector. The cursor must return that source only after completion or bounded cancellation so the document preparation can return the exact read to the Store registry. Neutral fixtures and a serde_json oracle will validate shape, numeric fidelity, progress, limits, and cancellation before Wires adopts it.

## Native gesture regression in progress

The shared framework clone cursor is now green in three native tests; the existing Store JSON encoder has a new direct DslValue bridge undergoing its byte-oracle rerun. A Wires neutral five-step gesture fixture and native registered-app regression now cover positive pointer-down in canvas A, an ignored move in B, two durable moves in A, release, exact window generations, and unchanged app config. Its cold seed goes through the actual document pack codec, retaining nested node metadata and child materialization. `wires-pointer-move-red-1.log` is queued; no gesture result is claimed yet. The target is registered as `wires-pointer-move` in the root script, Nx, and both launch catalogs.

Further source inspection confirms the existing first-party JSON writer preserves unique-key insertion order, while DslValue's serde adapter first builds a sorted serde_json::Value. Wires content hashing must match the first-party writer, including typed graph declaration order and the complete node JSON string. The shared encoding oracle therefore uses serde_json's streaming map serializer and cross-checks the production writer instead of relying on the sorting adapter.

## Candidate construction details for the next implementation step

- `DslValueCloneCursor<R>` owns a domain-provided `DslValueCloneSource`. A Wires selector can retain one exact SnapshotRead plus its typed scene Arc and select wires_fixture, camera, meta, an indexed node, or an indexed edge. Each completed copy returns that same source for the next selection. Release the additional scene Arc before returning the SnapshotRead to its registry.
- The existing Store one-item capacity is 1 MiB retained payload and 65,536 work items. Candidate vectors and raw JSON string buffers must reserve within that aggregate capacity, never reallocate while appending, and fail before publication if capacity or total work is exceeded. Preserve a retirement owner for every allocated partial value.
- Scan source node fields one at a time to discover the target and x/y field indices. Avoid replacing an arbitrary nested value with an untracked ordinary drop; validate coordinate scalars or transfer any displaced value through bounded retirement. Appending absent coordinates to an exactly-sized cloned object would reallocate its whole vector and needs an explicit capacity strategy.
- The neutral graph encoder needs precomputed field indices, because its canonical callbacks must never call DslValue::get (which scans). GraphNodeId and GraphEdgeId serialize as `{value: id}`, not as plain strings. Graph snapshot declaration order is schema/nodes/edges; node order is id/kind/label/position/ports/properties; edge order is id/source/target/kind/label. Each node property is `{key: wires.node, value: {kind: str, value: rawNodeJson}}`. Edge label is the complete raw edge JSON.
- The raw node/edge JSON must be emitted through the shared indexed DslValue encoder. A bounded UTF-8 append step needs to carry at most three trailing bytes across encoder chunks instead of validating a completed large byte vector in one final String::from_utf8 call.
- The existing child identity uses DefaultHasher over the complete JSON String. An incremental writer must include the string hash terminator and prove exact equivalence against the existing child-handle helper in native tests; do not assume every generic Hasher makes arbitrary write chunking equivalent.
- Store live authority exposes begin_one_item_seal, which accepts the exact edit, post Arc, mutation retirement factory, and snapshot retirement factory. Use the retained sealer instead of prepare_one_item (the latter encodes/hashes synchronously). WiresMutation needs an honest typed canonical projection for the admitted MoveNode variant.
- Current retained one-item publication has no amend-last admission. The batch coalesce_key merely routes to ArtifactCommand::AmendLast; setting the same field on a freshly sealed edit does not implement coalescing. Preserve gesture undo through an explicit bounded Store admission or revise the gesture lifecycle so durable publication occurs at an appropriate undo boundary with corresponding tests.

The initial Wires gesture run compiled, then exposed a fixture lifecycle error before a clean gesture failure could be observed: its temporary document envelope reached Drop without explicit bounded retirement. The fixture now transfers that envelope to retire_document_envelope with Wires's actual snapshot/mutation factories immediately after printing the cold seed pack; rerun `wires-pointer-move-red-2.log` is queued. This is not a passing gesture result.

Source inspection also found that Rewriting reset_document_effect still constructs a temporary ArtifactEnvelope and lets it drop after print_document_spr. The new shared envelope terminal-empty invariant makes that path unsafe; its reset/load lifecycle needs a real owned preparation/retirement fix and runtime test, alongside Wires setActiveExample. Existing earlier Rewriting move/config green tests did not exercise resetRule, so they do not validate that path.
