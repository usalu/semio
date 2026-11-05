# Current DSL Receiving Contract

Bounded current source/log review requested by native_recovery. No DSL/derive API edits made during this review; root subsequently assigned the exact declared-shape omission repair separately. SQLite remains native_recovery ownership and was not audited here. Older transport9/physicalred5 diagnostics are prior snapshots: current disk has already changed.

Current APIs under framework/🔨️modules/🗣️dsl/🧬️schema:

- Model root14 reexports borrowed DslReceivingField/Record/Variants and ownedcursor traits. Receiving root10–19 now returns RetainedWorkOutcome<Self,NativeSnapshotFailure> for all3 borrowed receiver traits. ReceivingWork23–32 adds child.work before inspecting child.result; performed marks a selected own action, preserving prefixwork on later refusal.
- Current IO control65–68 requires3typeparams T/O/E and canonical outcome callbacks/results, with E converting into full NativeSnapshotFailure. Prior2generic/Result/tuple callback errors do not describe current DSL receiving source.
- Derive root22–26 exposes DslOwnedReceiving and DslReceiving. Receiving-and-projection emitter generates canonical borrowed impls, meters child before errorfold and retains an intermediate frame through owner.receive_nested. It also emits ownedreceiving; this is distinct from value NativeReceive derive.
- OwnedFieldReceiving contract returns actualwork for demand/step/take/close; step preserves OriginalReceivingPrimary. ScalarReceiving sealedfirstparty leaves report denial0/freshscalar1. OwnedSlot close delegates ControlledRetirement.step then reads real last_turn_work even onerror.

True NativeReceive bridge: schema/🛬️receive/🦀️.rs implements NativeValueSource for actual RecordValue and FieldValue (ordinalkeys, borrowedchildren, typedscalar andtaggedvariant access), without constructing outputs. Owned/intrinsic requires NativeIntrinsicRecord:NativeReceive<DslValue>+Sync and retains actual source/admission/receiver storage, forwarding NativeReceivingOriginal.advance/take/close work. Its declaredshape phase already forwards shape.last_turn_work. Do not conflate this with borrowed whole-field reconstruction.

Concrete omission before assignedrepair: owned::declared_record loops DeclaredRecordShape.step and preserves physicalreceipt but ignores actual last_turn_work on success/refusal. Shape resets0 and selects1 per declarationprobe after grant/checkpoint. Generated receiving_fields calls that wrapper before nestedchild meter, losing real validationCPU. Root assigned neutral/serde regression and canonicalgetterfold; focusedred89544 pending, no fix/pass claim at this point.

Other exact reporting bounds: borrowed DslValue branch meters1 before FromValue::from_value_controlled. Value/codec/container-codecs608–609 delegates controlled::intrinsic, which iterates a Task stack and calls NativeDecodeControl.step/scoped_stage/copy APIs. Those APIs return Result and scoped completed/total progress, not a full returned computational reporter. An outer1 invocation count alone does not prove complete nestedCPU funding. Borrowed String/vector branches similarly meter outercopy/allocation calls while bodywallet records physicalbytes; owned/phased paths do return childwork. No blanket Outcome wrapper or arbitrarytiny-budget preemption acceptance claimed.

Required seam invariant: keep3typeparams/full NativeSnapshotFailure and actual partialtyped frame; add childwork before resultfold, fold actual shape getter on errorsuccess, independently preserve physicalreceipts/originalcause. Do not infer CPU from copied_items/bytes or native completed/total. Native actual_work counts defining effects only where producers explicitlycommit; scope delta does not create missing reports. This source audit does not establish passing DSL/native transport tests.

Exact paths:
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🦀️.rs
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🧬️contract/🦀️.rs
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🦀️.rs
- 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🪢️receiving-and-projection/🦀️.rs
- 🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/🔁️codec/🛬️controlled/🦀️.rs
