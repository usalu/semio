# Flow App-Owned Retained Factory Runtime Blocker

## Verdict

**RED — Flow's `addWidget` reaches neither the app-owned factory nor the retained Child/ACK pipeline.** The foreground handler is a real typed-child planner, but the live Flow registration has only direct Artifact/Config and host-only factories. `addWidget` remains `BatchOnlyPendingRewrite`, `build_tool_job` returns `None` for it, and legacy `handle` still creates a new local `FlowEvalSession` for it. The smallest honest packet is one Flow-owned Child factory/work plus one bounded generic pre-publication `ChildEmit` retirement correction.

This is a current-source audit. No Cargo, Nx, or runtime command was run. Existing framework Child/ACK tests are source-present, not evidence that a Flow factory has executed.

## Current evidence

| Boundary | Current source | Consequence |
|---|---|---|
| Real child plan | [`child_add_widget_mutation`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:26) reconstructs from `SemioFlowSnapshot`, permits exactly one appended node, preserves prior nodes/edges, and produces `SemioFlowMutation::InsertNode`. [`handle`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:49) reads the exact `content` child and emits one `ChildEmit`. | Reuse this planner; do not restore the former parent-cache/`FlowDiff` route or synthesize a default child. |
| Flow session authority | [`FlowInstanceOperationOwner`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1523) retains the sole `FlowEvalSession`; `with_session` fails once closing/missing ([1533](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1533)). The host-only job already borrows it synchronously through [`ArtifactInstanceOperationOwnerHandle::with_mut`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12776) at [1339](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1339). | A new work item can retain the cloneable *handle*, but only invoke `with_session` inside one synchronous work step. It cannot create, transfer, or hold a session across a yield/checkpoint. |
| Generic retained substrate | [`ArtifactCommandWork`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:87) gets the immutable job context; `ArtifactRetainedCommandJob` owns raw OpBinary pages, checkpoint, completion, and worker lifecycle ([167](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:167)). | Use this actual generic job. Do not duplicate the host-only scalar decoder: `AddWidget` has optional text and `f64` fields. |
| Captured child freshness | Typed start captures immutable `ChildContentView`, parent revision and child generation before factory construction ([23048](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23048)); Child publication checks captured/live parent, root, child dialect/revision, duplicate targets and cancellation before group dispatch ([22536](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22536)). | Work must read `context.children`, never the mutable child registry. The framework remains the only dispatcher. |
| Missing factory admission | `FLOW_DIRECT_STORE_TOOL_IDS` excludes `addWidget` ([763](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:763)); direct contracts expose only Artifact/Config ([1234](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1234)). `bounded_first_step_tool_proofs`, factory registration, and `build_tool_job` join only direct and host groups ([1645](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1645), [1651](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1651)). | This is the immediate end-to-end blocker. A manifest declaration alone cannot execute the action. |
| Policy bypass remains live | The manifest classifies `addWidget` as `BatchOnlyPendingRewrite` ([1963](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1963)); legacy `handle` rejects only host/direct ids and otherwise allocates a new `FlowEvalSession` ([1733](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1733)). | Migrate/reject atomically with factory admission. Otherwise two semantically different add-widget paths survive. |
| Pre-publication close gap | `ChildEmit` has explicit byte/item close semantics ([10415](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10415)), but generic retained-command close currently `drop`s an entire pending `emit` ([586](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:586)). | Cancellation after planning but before `completion.complete` does not use the only bounded Child close primitive. Fix this before claiming the Flow cancellation path. |

## Smallest implementation packet

### 1. Flow-specific Child work and factory

In the Flow editor module, add a dedicated `FLOW_CHILD_GROUP_TOOL_IDS = &["addWidget"]`, `FlowChildGroupWork`, and `FlowChildGroupJobFactory`; do not add `addWidget` to the direct array.

`FlowChildGroupWork` owns only:

- an `ArtifactInstanceOperationOwnerHandle` cloned from the existing `ArtifactOwnedToolJobRequest`;
- `completed` and `closing` booleans.

`extent` accepts only `FlowCommand::AddWidget` with a captured context and returns one work item. Its one `step` obtains `SemioFlowSnapshot` via `context.children.typed_read("content", &snapshot.content.child_id)`, then calls the shared add-widget planner inside `handle.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(...))`. The callback is synchronous and may only return a `SemioFlowMutation`; it must not retain an owner/session/child store. The returned `Emit` has exactly one `ChildEmit` for the captured child, `UiDirtyScope::Full`, and no parent/config/draft/presence/transient/effect/event lane.

Make the existing payload-to-descriptor/default-position/planner sequence one shared `add_widget` helper callable by foreground `handle` and this work item. Reject non-finite coordinates before invoking the host. This prevents one action vocabulary from acquiring two planning semantics.

The factory mirrors `FlowDirectStoreJobFactory` ([1183](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1183)), but has one exact key, `Migrated`, `FLOW_DOCUMENT_SCHEMA`, and one `ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[Child] }`. It uses `ArtifactRetainedCommandPayload::try_new_with_context`/`ArtifactRetainedCommandJob`, raw cap 16,384, checkpoint cap [`ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:10) (512), one work item, and the existing 16,384-byte output/7,500-ms contract. Its wire constructor rejects either oversize owner while returning the exact input and checkpoint owners.

Then, in the same Flow patch:

1. append its one proof row to `bounded_first_step_tool_proofs`;
2. register it in `register_tool_job_factories`;
3. select it in `build_tool_job`, passing the request's existing owner handle into the new work;
4. move the action to `Migrated` and include its id in the legacy-rejection predicate;
5. move the action-cohort row from BatchOnly/Artifact to Migrated/Child and generate launch entries from their seed, never from generated `launch.json`.

No framework context API is necessary: `ArtifactOwnedToolJobRequest` already carries the exact handle ([13396](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13396)), and `ArtifactCommandWork` itself is factory-owned state. This is smaller and more truthful than extending a generic immutable context with a session capability.

### 2. One framework retirement prerequisite

Before adding the Flow cancellation claim, replace `retire_one!(emit)` in [`ArtifactRetainedCommandJob::close_step`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:537) with an incremental `Emit` close branch. It must first use the existing `ChildEmit::close_one` for every child entry, then preserve the current finite order for the remaining generic vectors/scalars. It must retain `emit` unchanged if either grant is zero or a child cannot release its next UTF-8 byte. No new queue, erased Flow disposer, or direct `Drop` fallback is acceptable.

This is deliberately narrow: after `completion.complete`, the mounted operation already transfers the child output into `PendingChildGroupPublication`, which has its own incremental close and terminal check ([16084](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16084), [16441](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16441)). The missing branch is only cancellation/fault before that transfer.

## Acceptance matrix

| Layer | Required law | Required assertions |
|---|---|---|
| Neutral | `flow-add-widget-retained-child-v1` in the existing Flow editor fixture/router | Accepted `inputNote`/`inputSlider`; exact `content` coordinate; one typed `InsertNode`; unchanged parent coordinate; EN/DE progress; bad key/schema/tool; raw >16 KiB; checkpoint >512; malformed/trailing OpBinary; nonfinite coordinate; unknown descriptor; missing/wrong slot/dialect/stale child; duplicate target; cancellation before work and after work/before transfer. Denials make zero planner/session calls. |
| Framework native | `artifact_retained_command_child_emit_closes_incrementally_before_completion_handoff` | Build a byte-bearing Child emit, cancel after work completion but before `completion.complete`, grant 0/1/4096, verify each close step is bounded, no direct drop, original owner survives a zero grant, and terminal empty follows. |
| Flow factory native | `flow_add_widget_child_factory_is_exact_and_migrated` | Actual `Plugin<FlowApps>`/registry gives exactly one `addWidget` factory/key/proof/Child contract; the old `handle` rejects it; hostile wire/checkpoint owners are returned and do not touch the session. |
| Flow runtime native | `flow_add_widget_child_factory_runs_one_acknowledged_child_gesture_and_retires` | Start through the real registered action/typed admission—not direct foreground `handle`—with a real registered `SemioMembers::Flow` child. Drive Worker → Publishing → Child page/ACK → Terminal page/ACK → retiring using `plugin_step_live_cleanup`, `plugin_continue_typed_operations`, and `plugin_acknowledge_typed_operation_result`; assert one child change, no parent-content repoint, no duplicate retry commit, and full app close. |
| Hostile runtime native | `flow_add_widget_child_factory_rejects_or_retires_without_visibility` | Busy/closing/mismatched instance owner, stale parent/root/child revision or dialect, cancellation before linearization, duplicate Child target, group rejection, lost/duplicate/cross-instance/stale token ACK. Each preserves parent/child snapshots and history until a successful ACK path; every owner reaches terminal empty under bounded grants. |

The existing foreground law `add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content` remains valuable, but it calls the registry-less foreground path. The current `child-edit-check` script selects only that law ([script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:49)); it is not retained-factory/runtime proof. The generic framework law `retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires` ([plugin](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34215)) exercises a test app, not Flow. It also must not be read as an executed result absent a recorded gate receipt.

## Nonclaims and next durable blocker

This packet makes one pre-registered Flow content child eligible for app-owned Child publication. It does **not** prove full atomic parent/member/graph root publication or a durable global composition-history route: `dispatch_emit_group` calls `CompositionCoordinator::dispatch_group` before it captures/replaces `child_content_root` ([21110](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21110)). Therefore child-only `addWidget` must not claim restart/reload/undo correctness beyond the presently live child-store route until the separate atomic publication and global-history packets land. It also makes no browser, native-shell, public member-open, provider, or WGPU claim.
