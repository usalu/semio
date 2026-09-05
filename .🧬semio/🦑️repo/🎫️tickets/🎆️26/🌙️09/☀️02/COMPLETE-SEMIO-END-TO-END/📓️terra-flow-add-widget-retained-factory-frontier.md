# Flow `addWidget` Retained Factory Frontier

## Verdict

**RED — the framework can now retain, publish, acknowledge, and close a Child-only result, but Flow still never admits `addWidget` to that path.** The smallest honest packet is a separate one-tool `FlowChildGroupJobFactory` built on the existing generic `ArtifactRetainedCommandJob`, with a small `FlowChildGroupWork` that owns the already-issued instance-owner handle. It requires no new generic publication lane, no custom raw-wire decoder, no `FlowDiff`, and no Flow-local child disposer.

This is a source-only audit of the current shared tree. No Cargo/Nx/runtime command was run.

## What is now present

| Boundary | Current evidence | Consequence |
|---|---|---|
| Typed child planner | [`add_widget::handle`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:49) reads the parent-declared `content` coordinate through `ChildContentView` and emits exactly one `ChildEmit`. [`child_add_widget_mutation`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:26) proves one appended `SemioFlowMutation::InsertNode`, unchanged prior nodes/edges, and exact returned id/position. | Reuse this planner. Do not reconstruct from `FlowSnapshot.content.local_owner`, a default fixture, or raw child bytes. |
| Session authority | [`FlowInstanceOperationOwner::with_session`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1523) owns the sole `FlowEvalSession`; the host-only route already borrows it through [`ArtifactInstanceOperationOwnerHandle::with_mut`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12776) at [Flow:1339](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1339). | The new work retains only the cloneable handle from `ArtifactOwnedToolJobRequest`; it borrows the session synchronously within one `step`. It never creates/transfers a session or holds a borrow over yield/checkpoint. |
| Generic raw/cancel/close substrate | [`ArtifactRetainedCommandJob`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:386) owns real `OpBinary` pages, checkpoints, preflight, completion, cancellation, and close. It rejects cancellation before each work turn ([388](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:388)). | `AddWidget` has four optional/scalar fields; use this exact decoder instead of the three-field `ScalarRecordWireWitness`. |
| Child publication and freshness | The mounted publisher validates declared `Child` contract ([22901](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22901)), then routes child output through `PendingChildGroupPublication`. Its pre-dispatch rechecks parent revision/generation, captured child root, member dialect/revision, duplicate targets, and cancellation before `dispatch_emit_group` ([22630](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22630)). | Flow work reads only the captured `request.context.children`; it must never touch the live member map or call `dispatch_emit_group`. |
| Child-first close and completion handback | Generic close calls `Emit::close_child_one` before it retires `emit` ([retained-command:605](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:605)); `PendingChildGroupPublication` does the same ([plugin:16190](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16190)). `ArtifactToolCompletion::complete` returns the rejected emit and ephemeral owners ([13335](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13335)), which the generic job restores before faulting ([retained-command:516](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:516)). | The earlier pre-publication Child leak is source-repaired. The Flow work must return its `Emit` to this generic owner; it must not call completion itself or drop a `ChildEmit`. |

## Exact Flow REDs

1. [`FLOW_DIRECT_STORE_TOOL_IDS`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:763) and [`FLOW_HOST_ONLY_TOOL_IDS`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1260) omit `addWidget`.
2. [`bounded_first_step_tool_proofs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1645), [`register_tool_job_factories`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1651), and [`build_tool_job`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1657) only join those two cohorts. The actual request already provides the owner handle at [plugin:13441](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13441); Flow simply discards it on the add-widget path because no such path exists.
3. The manifest still says `BatchOnlyPendingRewrite` ([Flow:1963](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1963)).
4. The legacy foreground handler blocks only existing direct/host routes, then allocates a fresh `FlowEvalSession` ([Flow:1726](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1726)). Once `addWidget` is migrated it must join that denial predicate; otherwise it has two non-equivalent session/child-authority paths.
5. The selected Flow native gate runs only the foreground semantic law ([script:52](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:52)). It has no factory, worker, result-ACK, cancellation, or close proof.

## Minimal patch inventory

### A. One Flow-only child factory

In the Flow editor module, add:

- `FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget"]` and `FLOW_CHILD_GROUP_RAW_BYTES = 16_384`;
- `FlowChildGroupWork { instance_owner: ArtifactInstanceOperationOwnerHandle, completed: bool, closing: bool }` implementing `ArtifactCommandWork<EditorApp<FlowPlayApp>>`;
- `FlowChildGroupJobFactory`, structurally parallel to `FlowDirectStoreJobFactory`, with one key, `Migrated`, `FLOW_DOCUMENT_SCHEMA`, the existing resumable contract `(16_384, 256, 1, 16_384, 7_500, 1, 1)`, and exactly:

```rust
ArtifactToolPublicationContract {
    tool_id: "addWidget",
    lanes: &[ArtifactToolPublicationLane::Child],
}
```

The factory uses `ArtifactRetainedCommandPayload::try_new_with_context` and `ArtifactRetainedCommandJob`; its wire restore branch must have the same owner-returning oversized-input/checkpoint behavior as [`FlowDirectStoreJobFactory`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1193). Do not make a scalar witness or a custom Flow job.

`FlowChildGroupWork::extent` is exactly one only when:

- the tool id and command are `addWidget`/`FlowCommand::AddWidget`;
- `context` exists and its captured `content` child can be read as `SemioFlowSnapshot` at `snapshot.content.child_id`;
- `kind` plus optional `neuron_kind` is within Flow's existing 16,384-byte retained-text envelope, both coordinates are finite, and the captured child node/edge counts remain within the existing 256-item work envelope.

`step` is a single synchronous turn: form `ArtifactView::with_children(snapshot, history, context.children.clone())`, then call `instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(|session| add_widget::handle(..., session)))`. It must fail closed for a busy, closing, wrongly typed, or missing owner. It must prove the result has exactly one `ChildEmit` whose slot/id equal the captured parent coordinate and has no artifact/config/draft/ephemeral/effect/event output before returning `Complete`.

Factoring the descriptor/default-position/typed-delta logic out of `add_widget::handle` is allowed only to make both callers use **one** routine. That routine must reject non-finite coordinates and capacity overflow before `flow_host_with_session` executes. No hand-written `ChildEmit`, raw child-pack, parent mutation, `FlowWorkingScene` cache, or global `FlowEvalSession` is permitted.

`FlowChildGroupWork` has no strict owner of its own: it owns a cloneable handle, not the `FlowEvalSession`. Its `begin_close` sets `closing`; default one-item `close_step` is valid only after the work has discarded all planned data. The generic job remains responsible for raw input, checkpoint, captured Arcs, completion, and the Child emit's incremental close.

### B. Wire Flow into the only legitimate runtime route

In the same Flow change:

1. add the one factory proof to `bounded_first_step_tool_proofs`;
2. register it beside host/direct factories;
3. add the child cohort to `build_tool_job`; construct `FlowChildGroupWork` from `request.instance_operation_owner`, then produce the ordinary generic retained payload;
4. change only `addWidget` from `BatchOnlyPendingRewrite` to `Migrated`;
5. include the child cohort in `FlowPlayApp::handle`'s `flow.retained.legacy-dispatch` guard;
6. add its one exact factory-publication row to the source/neutral fixture and extend the existing launch seed, then regenerate launch data. Do not edit generated `launch.json` directly.

The worker must never call `dispatch_emit_group`. Only the mounted publisher may do so after its existing captured-root/revision/dialect/cancellation checks; it returns a `Child` result receipt and waits for a host ACK before terminal retirement.

## Required evidence

| Gate | Exact assertion |
|---|---|
| Language-neutral Flow fixture | `flow-add-widget-retained-child-v1`: accepted slider/note, exact stable `content` coordinate, one `InsertNode`, no parent mutation, EN/DE progress labels. Hostile rows: wrong key/tool/schema, raw > 16 KiB, checkpoint > framework cap, malformed/trailing OpBinary, over-cap text, nonfinite values, missing/wrong child slot/id/dialect, stale parent/root/child revision, duplicate target, owner busy/closing, cancellation before work and after planning. Every denial has zero host/session call and zero visibility. |
| Factory/native source law | Real `Plugin<FlowApps>` and `flow_app_with_registry()` prove one factory/key/proof/Child lane and `Migrated` classification. The legacy handler rejects `addWidget`; a registry-less `flow_app()` may no longer be used as UI evidence. |
| Real runtime lifecycle | Load one parent and one real `SemioMembers::Flow` `content` child through the public `LoadDocument` then `LoadChildren` protocol ([plugin:32037](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32037)). Start the registered action, then drive `plugin_step_live_cleanup` → `plugin_continue_typed_operations` → `plugin_acknowledge_typed_operation_result` ([plugin:30308](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30308), [31457](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31457), [31736](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31736)). Assert Worker → Publishing → exactly one Child page/ACK → Terminal page/ACK → no pending operation, exact child node change, unchanged parent coordinate, and bounded full app close. A loop that calls only `advance_typed_operation_publication` is invalid: it never drives the Worker stage. |
| Runtime hostile/cancel law | Delayed ACK, duplicate/stale/cross-instance token, cancellation before worker and immediately after work, stale live root/revision/dialect, duplicate child target, and forced group rejection. Before linearization: parent/child/history stay unchanged. After a successful Child ACK: no second group dispatch on retry. All child outputs close under 0/1/4096 grants to terminal empty. |

## Ordering and nonclaims

Implement this only after the live generic Child publication/ACK packet is accepted. Its full public runtime law additionally depends on public retained `MemberFactory::open`/`LoadChildren` for the real Flow child; current `register_content_child` is a testkit-only shortcut ([Flow:2095](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:2095)). Until that opener is accepted, a factory-shape law can be native evidence, but not a public runtime success claim.

This packet covers one pre-existing content child only. It does not claim atomic parent/member/graph root exposure, global composition history, child genesis, restart/reload, browser/native installation, WGPU rendering, or migration of any other Flow action.
