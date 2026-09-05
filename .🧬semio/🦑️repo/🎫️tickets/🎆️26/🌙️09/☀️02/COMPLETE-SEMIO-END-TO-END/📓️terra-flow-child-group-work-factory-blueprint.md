# Flow `addWidget` Child-Group Work/Factory Blueprint

## Verdict

**RED — `addWidget` has correct foreground child semantics but no registered retained Child factory.** Its current action remains `BatchOnlyPendingRewrite`; the exact app-owned factory registry contains only host-only and direct parent/config lanes. The current child-publication owner has source-level retained handback but still needs its own native lifecycle proof; it is a prerequisite, not a replacement for this Flow packet.

This is a current-source design audit. No Cargo/Nx command was run and no native/runtime success is claimed.

## Current authority and the useful existing patterns

| Concern | Current source evidence | Required consequence |
|---|---|---|
| Typed child plan | [`child_add_widget_mutation`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:26) reconstructs only from an admitted `SemioFlowSnapshot`, validates one appended node/no changed prior nodes or edges, and returns one `SemioFlowMutation::InsertNode`. [`handle`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs:49) reads the exact `content` child and creates one `ChildEmit`. | Extract/reuse this planner exactly. Do not derive a default scene, re-mint the parent content coordinate, or synthesize raw child bytes. |
| Session capability | `FlowInstanceOperationOwner` is the one app-instance owner of `FlowEvalSession` ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1520)); the existing host job borrows it with `instance_owner.with_mut` ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1341)). `FlowEvalSession::Drop` requires terminal explicit close ([host](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2296)). | `FlowChildGroupWork` must borrow this existing owner only for one synchronous planner turn. It must never allocate a second session or retain a session borrow across yield/checkpoint. |
| Retained OpBinary/checkpoint path | `ArtifactRetainedCommandJob` already pages raw input, decodes actual `OpBinary`, checks an exact tool id, fences a checkpoint with the captured context identity, and delegates bounded work ([retained command](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:243), [decode/preflight](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:440)). | Use this generic job; do **not** copy the host-only scalar grammar. `AddWidget` carries optional text and `f64`, whereas the existing scalar witness is only the six host-only command variants ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1272)). |
| Captured child authority | Typed start copies the immutable `ChildContentView`, canonical revision, child-root generation, and cancellation lease before worker dispatch ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22980)). The current publisher subsequently compares captured and live parent/child revision, dialect, and target identity ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22491)). | Work reads only `context.children.typed_read::<SemioFlowSnapshot>("content", snapshot.content.child_id)`. It must not reread the mutable registry or use the parent local cache. |
| Existing factory lanes | Direct factory is parent/config-only ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1234)); host factory is host-only ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1456)). `build_tool_job` currently selects only those two arrays ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1657)). | `addWidget` is one separate Child factory, never an entry in `FLOW_DIRECT_STORE_TOOL_IDS`. |
| Current policy | The public action is still `BatchOnlyPendingRewrite` ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1963)). The legacy `handle` only blocks existing host/direct ids, then constructs a fresh `FlowEvalSession` ([editor](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1726)). | Move `addWidget` to `Migrated` only in the same source change that registers the factory and adds it to the legacy-route rejection set. This prevents the strict local-session path from being used after the migration. |

## Minimal implementation packet

### A. One scoped instance-owner bridge for generic retained work

`ArtifactOwnedToolJobRequest` already owns `instance_operation_owner` ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13409)). Its `ArtifactOwnedToolJobContext` omits it ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13339)). Add one **scoped callback**, not a public clone/extraction API:

`ArtifactOwnedToolJobContext::with_instance_operation_owner::<T, R>(...) -> Result<R, Fault>`.

The typed operation builder supplies the already-captured handle when constructing the context. The context identity digest remains the immutable document/draft/transient/child identity; it must not hash a mutex pointer or session address. The callback obtains the handle with its current nonblocking `with_mut`; a busy/closing/wrong concrete owner is a bounded fault before an emit is created. No await may occur within the callback.

This is narrower than duplicating the generic raw OpBinary/checkpoint state machine in Flow and avoids a local `FlowEvalSession`, whose `Drop` is intentionally not permissive.

### B. `FlowChildGroupWork`

Add `FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget"]` and a `FlowChildGroupWork` implementing `ArtifactCommandWork<EditorApp<FlowPlayApp>>`.

Its state is only `completed`/`closing`; it owns neither a Flow scene nor a member store. `extent` accepts only `FlowCommand::AddWidget`, only a captured context, and returns exactly `1`. Its first and only work turn:

1. verifies it is live and the command is `AddWidget`;
2. reads the captured parent coordinate and `context.children.typed_read::<SemioFlowSnapshot>("content", &snapshot.content.child_id)`;
3. invokes the existing typed planner while borrowing `FlowInstanceOperationOwner::with_session` through the scoped context bridge;
4. returns `ArtifactCommandWorkStep::Complete(Emit { child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, vec![insert])], ui_scope: Full, ..Default::default() })`.

No parent/config/draft/presence/transient mutation is allowed. The work makes no progress after it owns an output, so the generic job transfers that output to its own pre-publication state and enters `Publish` on its next turn. Its checkpoint encoding is empty; generic checkpoints before the work turn cover only exact raw-page cursor/context. A restored worker re-plans only before publication, against the existing freshness fence. It must reject a checkpoint whose work phase is already true rather than invent a serialized session or child mutation continuation.

### C. Safe generic close for a pre-publication Child emit

Before Flow uses this work, extend the retained-command job's close path. It currently directly drops `self.emit` ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:586)); that bypasses the existing byte/item retirement protocol of `ChildEmit::close_one` ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10414)).

Add a crate-private child-aware `Emit` retirement step in the retained-command module: drain every `child_emits` entry with that existing method before the `Emit` is removed; after the list is empty, retain the existing bounded scalar/Vec disposal order for the other lanes. Do not add an erased Flow disposer or a global cleanup queue. This is required for cancellation after planner completion but before the mounted publisher takes the result.

### D. `FlowChildGroupJobFactory` and source policy

Use the generic payload/job:

- `Payload = ArtifactRetainedCommandPayload<EditorApp<FlowPlayApp>>`;
- `Job = ArtifactRetainedCommandJob<EditorApp<FlowPlayApp>>`;
- `ToolJobFactoryKey` contains the one exact controller/`addWidget` key;
- `Migrated`; `DOCUMENT_SCHEMA = FLOW_DOCUMENT_SCHEMA`;
- `PUBLICATION_CONTRACTS = [{ tool_id: "addWidget", lanes: &[Child] }]` only;
- fixed contract `resumable(16_384, 256, 1, 16_384, 7_500, 1, 1)`, matching current Flow retained raw/output pages but pinning one semantic work item and one child emit;
- reject an oversized raw owner or oversized checkpoint owner while returning the original retained inputs; otherwise use `ArtifactRetainedCommandJob::from_wire[_with_checkpoint]`.

In `FlowPlayApp`:

1. append `FlowChildGroupJobFactoryProofs` with exactly one `addWidget` row to `bounded_first_step_tool_proofs` (the current direct and host proof sets are at [1471](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1471) and [1501](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1501));
2. register this third exact factory in `register_tool_job_factories` ([1651](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1651));
3. add a third `build_tool_job` branch that constructs `FlowChildGroupWork` and `ArtifactRetainedCommandPayload::try_new_with_context`; no alternate input path or direct `dispatch_emit_group` call;
4. add its one id to the `handle` legacy rejection condition, then classify `addWidget` as `Migrated`.

The framework's `PendingChildGroupPublication` is the sole next owner after `completion.complete`: it captures and later calls `dispatch_emit_group` ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16079), [22472](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22472)). The Flow factory must not retain another member/group/receipt owner.

## Current child-publication boundary

The current source no longer moves `ChildEmit` owners out for dispatch. `PendingChildGroupPublication::begin_dispatch` only changes the phase ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16096)); `dispatch_emit_group` receives borrowed artifact mutations, child emits, and description ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22572)). Every current pre-dispatch and dispatch-error branch calls `reject_and_fault` and restores the same pending owner to the mounted operation ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22513), [framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22601)). Its close order is child emits, artifact mutations, description, receipt, then bounded fault ([framework](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16129)). The earlier direct-owner-loss finding is therefore source-repaired.

That is source evidence only: the Flow factory needs an exact native lifecycle law proving cancellation, dispatch rejection, preserved pending state, incremental close, and terminal emptiness. It also does not settle the larger all-or-nothing composition transaction/root-exposure boundary documented in [`terra-retained-child-publication-atomic-root-audit.md`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️terra-retained-child-publication-atomic-root-audit.md). This Flow P0 may use only a pre-registered existing content member, no `ChildGenesis`, and must not claim that `dispatch_emit_group` has solved parent/member/graph atomicity.

## Exact RED → GREEN evidence

### Neutral corpus

Add `flow-child-group-work-v1` in the existing Flow editor fixture router. It must be an independent model of:

- accepted `inputNote` and `inputSlider` requests with the exact stable `content` coordinate, one `InsertNode`, unchanged parent edit count, one Child receipt, and EN/DE progress labels;
- wrong controller/tool/schema, raw >16 KiB, checkpoint > cap, malformed/trailing OpBinary, invalid/nonfinite coordinates, unknown widget descriptor, missing/wrong-slot/wrong-dialect/stale child, duplicate Child target, child/root reservation saturation, cancellation before work, and cancellation after work/before publication;
- no planner or session call on every preflight denial; one planner call and one result only on the accepted row; no second commit on result ACK/retry.

The neutral fixture does not claim actual `SemioMembers`, session retirement, group persistence, or socket delivery; native law owns those facts.

### Native laws

Add the following exact FQNs in the existing Flow editor command test surface, then add only these new names to the existing `ChildEditCheckScript` array ([script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:50)). Generate the launch entry from its seed, not by modifying generated `launch.json`.

1. `flow_add_widget_child_group_factory_is_exact_one_and_manifest_migrated`
   - use the real `Plugin<FlowApps>`/`flow_app_with_registry()` path (not registry-less `flow_app()`), prove exactly one factory proof/key/Child contract, and prove legacy `handle` denies this id;
   - positive raw OpBinary pages at 1/7/4096-byte grants; hostile wrong key/schema/raw/checkpoint rows preserve the original retained input and make no session/planner call.
2. `flow_add_widget_child_group_work_uses_captured_semio_content_and_closes_before_publish`
   - use a real pre-registered `SemioMembers::Flow` child; assert one typed child mutation, unchanged parent content ref, correct node position/id, and no parent mutation;
   - cancel before planner and after planner/before publication, then drive the actual job `begin_close/close_step` to terminal empty with small grants; assert the Flow instance session survives because it was borrowed, not owned by the work.
3. `flow_add_widget_child_group_publication_commits_once_or_retires_without_visibility`
   - drive the real mounted typed operation through its Child receipt/ACK; assert child revision/root generation change once, same receipt on retry, and exact close;
   - stale parent, stale child revision/dialect, cancellation before linearization, duplicate target, and forced group rejection leave child snapshot/history/root unchanged and terminal-retire all owners.

The current registered `child-edit-check` only selects `add_widget_dispatches_one_typed_child_edit_without_repointing_parent_content` ([script](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📜️script.ts:50)). It is foreground semantic evidence and must be retained, but cannot prove the three retained lifecycle laws above.

## Nonclaims

- This converts exactly `addWidget`; `duplicateWidget`, all ordinary parent `FlowDiff` leaf paths, genesis, public member open, checkpoint persistence, browser/native startup, and WGPU rendering remain outside it.
- No new Flow session, generic erased disposer, global queue, live-map lookup, or synthetic `SemioFlowSnapshot` is admissible.
- A source/neutral pass does not prove the framework publisher's asynchronous commit/root exposure or the native lifecycle gates.
