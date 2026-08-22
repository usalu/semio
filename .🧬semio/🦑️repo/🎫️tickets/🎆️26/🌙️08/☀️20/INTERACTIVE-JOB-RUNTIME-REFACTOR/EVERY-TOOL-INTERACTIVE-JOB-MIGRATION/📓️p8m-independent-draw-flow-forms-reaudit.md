# P8m Independent Draw, Flow, And Forms Re-Audit

## Verdict

**RE-AUDIT READY (source/static repair).** Every P0/P1 source finding recorded below has a concrete repair in the current tree. This is not a runtime-pass verdict: Cargo, native, release, Wasm, allocation, and timing lanes were not run here.

Scope repaired: Phase 8 framework store/plugin composition code and current Draw/Flow/Forms sources/tests. No Cargo command, cache/target deletion, modifying Git command, ticket-status mutation, or JSON-file edit was performed.

## Repair Disposition

- Canonical revision is now a repository-owned incremental SHA-256 accumulator. Length-prefixed, domain-separated initial/applied/redo/cursor/checkpoint records cover the initial pack digest and complete serialized edit records. Mutation-time reconciliation extends/truncates only the changed tail; cold load/reset reconstructs the accumulator. An interior-ABA fixture changes a middle edit while preserving cursor length, IDs, tail, and final snapshot, then proves load/reset identities differ/reconstruct correctly.
- Public composition exposes `SnapshotRead<T>` and `ErasedSnapshotRead`, not concrete `Arc`; `ChildContentView::typed_read` preserves opaque ownership. Forms' `ChunkAddressableJson` owner and all chunk-`Arc` methods are crate-private. Static census finds no former `snapshot_arc`, `snapshot_any`, `typed_arc`, or public Forms `arc` surface.
- Forms committed values own bounded chunk leaves inside `FormsTryValues`; serde/DSL reopen reconstructs IDs from owned chunks. Clearing scalar/bulk staging registries after serialization does not remove completed content. Staging/order/capacity/identity failures are typed `Invalid`/`Busy`/`Order`/`Conflict` mutation messages rather than ignored booleans.
- Public plugin admission scans body size, every string, and nesting before generic serde. Draw/Flow/Forms additionally receive command-specific raw `actionId` caps before deserialization: Forms 16 KiB, Draw 8 KiB, Flow 8 KiB. Exact maximum, maximum-plus-one, malformed structure, and hostile-string fixtures cover this boundary; command-local fields/checkpoints retain tighter limits.
- `AppCommandJob` passes its factory-assigned `Operation` into `AppOperationContext`, carrying actual app instance, parent document, operation, generation, and the full canonical base revision. Draw/Flow/Forms continuations and registry keys derive from it; no play-app constant is used as durable job authority. Forms preserves the initiating operation across a multi-action chunk upload. Same-document/two-app and shared-child/two-parent cancellation/restart fixtures cover collision isolation.
- Draw uses the canonical store revision; Flow validates both parent canonical revision and child canonical revision; Forms/Flow/Draw bound continuation decode and reject stale identity before advancing. Flow/Draw process registries are not authoritative for reopen.

The detailed findings below are retained as the historical rejection record. Their source descriptions refer to the pre-repair tree and are closed by the disposition above.

## What Is Actually Present

- The public app route snapshots the app, creates `ChildContentView`, constructs an `Operation`, and dispatches `AppCommandJob` ([framework plugin component:11131](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11131), [13066](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L13066)). Its one worker turn invokes `A::handle`, then the ordinary emit/diff/apply route completes the action ([11148](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11148)). Draw, Flow, and Forms public-action tests exercise that wrapper, rather than only calling a handler directly.
- `ChildContentView::new` reads each member revision and snapshot handle without pack encode/decode or a graph clone ([framework plugin component:8507](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L8507)). Flow's duplicate search bounds each worker slice and uses the real production `SemioMembers` registration ([Flow plugin component:12](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️component.rs#L12), [29](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🦀️component.rs#L29)).
- Draw has 32-work slices and a 64-session admission ceiling; Flow has its 64-row slice and child-content revision check; Forms has explicit 4 KiB *post-decode* chunks, 16,384 input chunks, 64 live input sessions, and expiry ([Draw pointer component:627](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L627), [Flow duplicate component:202](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L202), [Forms set-try-value component:348](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L348)). These local limits do not repair the findings below.

## Prior Findings (Closed by This Repair)

### P0 — Cursor Revision Is Not A Content Identity

`ArtifactStore::cursor_content_revision` hashes only history lengths, first/last applied/redo identifiers, the final applied mutation tail, and checkpoint ([store component:4503](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L4503)). Two loaded histories with equal lengths/endpoints/tail/checkpoint but a changed interior edit therefore receive the same revision. `set_state` recomputes the materialized state and then calls this same weak hash ([store component:4654](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L4654)). This permits stale/ABA continuation acceptance after load/reset and invalidates Flow's child revision guard as well.

Repair/gate: make the persisted revision a collision-resistant identity of the complete applicable cursor/history (or an incrementally maintained authenticated accumulator that covers every applied/redo edit and initial content), and use it consistently as the operation base revision. Add loaded/reset ABA tests that change a middle edit while preserving length, endpoints, and tail; require rejection before applying or requeueing.

### P0 — `Arc` Is Exported Across The Public Composition Boundary

The framework exposes `ArtifactStore::snapshot_arc` ([store component:4744](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L4744)), `SpaceMember::snapshot_any` ([store component:6921](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs#L6921)), and `ChildContentView::typed_arc` ([framework plugin component:8523](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L8523)) as public APIs. Forms also publicly exposes `ChunkAddressableJson::arc` ([Forms set-try-value component:28](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L28)). This directly fails the requested “Arc remains internal, not exported” rule; documentation calling it an in-process API does not make a `pub` return type internal.

Repair/gate: replace public `Arc` return types with a repo-owned opaque read/slice capability whose concrete ownership remains private. Re-audit all public SDK traits and plugin command payload helpers; compile an external-consumer fixture that cannot name or receive `Arc`.

### P0 — Forms Persists Only Content IDs While The Content Remains Process Global

`FormsConfigMutation::CommitTryValue` stores a `content_id` in `FormsTryValues`, not the content ([Forms config component:565](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L565)). The bytes exist solely in the process-global `TRY_VALUE_BLOBS` map ([Forms config component:31](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L31)); a missing map returns an empty chunk list ([146](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L146)). A completed value cannot survive process loss, nor can a persisted config be faithfully serialized/reopened without that registry. Committed entries have no lifecycle ceiling or eviction policy.

The claimed evidence is internally contradicted: the public scalar test expects the recovered config value to have 8192 bytes ([Forms set-try-value component:1687](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L1687)), though this path stores a short `content_id`. This test cannot demonstrate the stated completed-value result from the inspected source.

Repair/gate: put bounded content blocks (or their durable content-addressed store with a persisted availability/retention contract) in the typed operation/event-log model. Clear every registry after a completed commit, serialize/reopen the parent, and prove the value and its chunks remain available without replaying original user input. Test a full committed-cache ceiling, cleanup, and explicit Busy behavior.

### P0 — Forms Allocates And Copies Arbitrary Action JSON Before Admission

`ChunkAddressableJson::Deserialize` first deserializes an unrestricted `String` and then constructs an `Arc<str>` ([Forms set-try-value component:62](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L62)). The 4 KiB check occurs only later in `stage_command_input` ([348](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L348)), after `handle` clones the `Arc` ([1462](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L1462)). This violates bounded payload admission and the no-eager-full-input requirement.

Repair/gate: enforce a wire/action-body byte limit before generic JSON decoding, use a bounded deserializer or transport chunk envelope, and test malformed/oversize input at the public ActionBus boundary while measuring allocations/failure before a full `String`/`Arc` is created.

### P1 — App/Document/Operation Identity Is Hard-Coded Or Incomplete

Draw registry lookup, cancellation, creation, and admission hard-code `"draw-play"` ([Draw pointer component:635](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L635), [659](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L659), [1097](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L1097)). Flow creates and accepts `"flow-play"` and uses the child ID as its document ID rather than the parent artifact identity ([Flow duplicate component:193](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L193), [223](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L223)). Forms builds its input key with `"forms-play"` ([Forms set-try-value component:348](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L348)). Two app instances/tenants with the same document (or Flow child) therefore share process authority and can supersede/cancel one another. Existing “two-document” tests only make the documents/children distinct; they do not cover this collision.

Additionally, the generic job generates an `Operation` from `store.generation()` ([framework plugin component:13085](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L13085)), but `AppCommandJob` does not pass that operation to `A::handle` ([11155](../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs#L11155)). Plugin continuations instead mint their own global IDs/constant scopes. The actual public-job identity is consequently not the typed persisted continuation identity.

Repair/gate: carry an app-instance identity, parent document identity, operation id, generation, and complete base content revision in one durable continuation record; derive every registry/admission key from it. Add same-document/two-app and shared-child/two-parent collision tests, including cancel and restart.

### P1 — Draw Revision Is Independently Non-Canonical

Draw does not use `ArtifactStore::content_revision`. Its `draw_document_revision` picks the first history command ID, checkpoint/config fallback, or `initial:<document-id>` ([Draw editor component:136](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs#L136)). A load/reset can change the document while retaining that selected value, so the trace continuation check can accept stale work ([Draw pointer component:1073](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L1073)). The stated ABA test merely appends a layer ([1321](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L1321)); it does not exercise load/reset or an equal-first-edit identity collision.

Repair/gate: make Draw use the corrected canonical store content revision, persist it in continuation/config state, and prove stale rejection across reset/load and an ABA state restoration.

### P1 — Forms Drops Failed Staging Silently

`stage_try_value_chunk` and `stage_try_values_batch_entry` return `false` when admission/order/commit fails ([Forms config component:57](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L57), [91](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L91)), but `Mutation::diff` ignores both results ([558](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L558), [571](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L571)). Commit failures return an unchanged config ([565](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs#L565)). The event thus appears accepted while the user value vanishes, violating explicit Busy/cancel/no-silent-eviction requirements.

Repair/gate: make failure a typed, durable admission/cancel outcome which the action path exposes to the caller; add the concurrent 65th shared-staging case and malformed/order-conflict cases at the public ActionBus boundary.

### P1 — Remaining Action/Checkpoint Payloads Have No Decode-Time Envelope

Flow exposes multiple arbitrary `String` fields on `DuplicateWidgetStep` ([Flow duplicate component:35](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L35)) and begins its semantic checking only in `advance_duplicate_widget` after deserialization ([193](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L193)). Draw continuation `base_revision` is likewise an unbounded deserialized string before its comparison. Bounded search work is not bounded action decoding.

Repair/gate: put a bounded codec/admission layer ahead of all action decoding, bound every continuation identifier/checkpoint/string and total action bytes, and test malformed/oversize continuation JSON before handler execution.

### P2 — Tests Are Source Claims, Not Runtime Evidence

The tests use `Instant` `<8 ms` assertions ([Draw pointer component:1283](../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🦀️component.rs#L1283), [Flow duplicate component:317](../../../../../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs#L317), [Forms set-try-value component:1669](../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧪️set-try-value/🦀️component.rs#L1669)). They provide no reproducible measured budget for a release/native/Wasm build, do not exercise action decoding of hostile full payloads, and miss the same-app identity collisions above. Some tests manipulate private registries directly to simulate loss; that is useful only when accompanied by durable-state reopen proof, which Forms lacks.

Repair/gate: after the P0/P1 repairs, run the public ActionBus suite in release/native and Wasm, collect deterministic per-turn work/allocations rather than host-clock-only assertions, and test clear-all-registry plus serialized reopen after both in-progress and completed operations.

## Command Record

Executed read-only catalog validation:

```text
bun ./📜️script.ts verify interactivity tool-jobs --format json
```

Result: exit 0; 775 command rows / 773 unique commands; 50 macro hosts and 50 macro invocations; zero batch-only, forbidden, deleted, or catalog failures. It validates command classification/factory registration only. It does not compile or execute the Rust public action, job, codec, diff, apply, checkpoint, native, release, or Wasm paths.

Read-only source inspection used `rg` and `nl -ba` over the cited framework and plugin components. **Not run by scope:** Cargo tests/checks, native runtime, release runtime, Wasm build/runtime, allocation profiling, or timing benchmarks. No claim of runtime correctness or passing tests is made.

## Exit Gate

Do not close Phase 8 for Draw/Flow/Forms until all P0/P1 repairs land and the corrected public path proves: bounded decode/admission; durable typed continuation/operation records; canonical revision refresh after mutation/load/reset; explicit cancellation/supersession; same-document/app isolation; 65th/full/malformed/depth/input-count/expiry behavior; registry-clear + serialized-reopen replay for in-progress and completed data; no public `Arc`; and release native/Wasm evidence.
