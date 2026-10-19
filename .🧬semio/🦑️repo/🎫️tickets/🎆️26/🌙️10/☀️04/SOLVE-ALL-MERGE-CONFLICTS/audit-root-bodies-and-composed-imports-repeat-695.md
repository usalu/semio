# Independent Root Bodies And Composed Domain Imports — Repeat 695

Read-only source review against B b84bf526c7ba26f99c4054b47da1c1a4d5064e3d, H 8f91299152ceab6c19df64c251f5ad23252ce18b, S 6e8eff657fb70675db7ad30b890f61b7a3e201d7. No source/Git/index mutations, compiler, build, tests, runtime, resolver or generator. Findings below are introduced reconciliation defects, not requests to repair inherited authored code.

## Root Body And Fixture Witnesses

Read root-primary-body-preservation-repeat-695.md, root-fem2d-context-preservation-repeat-695.md and their exact-range result files; inspected actual receiving owners and immutable source.

Independently extracted all 13 H functions in the actual FEM2D app/context owner. All 13 current whole functions match H after whitespace normalization and exactly the intended MountedOwnerPolicyV1::INTERACTIVE_COMPONENT→fixture_mounted_policy substitution. The unit file has no incoming pub(crate) inline context shadow. The actual Fem2dApp remains a two-field raw-app plus OriginalFixtureScope owner; close/drop/dispatch/publication/render continue using that same scope. Incoming helper names correspond to these actual functions, rather than an unrelated copy.

Both actual FEM3D app and app-2 context diffs retain the H original scope and all other helpers. The existing fem3d_app delegates to new fem3d_peer_app(LOCAL_ACTOR_ID); the latter retains H construction and binding with only actor argument and incoming fixture policy changes. Close/drop/publication/history still use app.1.cx(). This preserves peer intent without retaining the obsolete one-field unit shadow.

Actual Energy test close (15), generic Puzzle work close (23), and puzzle5d_record_from_projection (95) whole function text equals immutable S modulo whitespace. Energy close includes original caller.turn_with_decoder, decoder terminal retirement and one caller retire after terminal job close. The actual Puzzle3D FillToolRunJob close_step (127) matches the entire incoming phase match body, forwarding the same cx to both Run and Revalidate and preserving whole OriginalTurnFailure; current signature qualifies the same types. Browser3D bridge (126) is whole H exact after newline normalization; funded_turn and original paid retirement survive. Subspace modal-input owner (135) is independently whole H exact.

For2, the actual animation function H diff changes only fixture policy while retaining &mut app.0 and app.1.cx(). For24, the actual pump delegates to app.maintain; inspected the defining P2D testkit method at409–416: it quotes retirement demand, funds the grant, and uses self.1.cx().with_retained_loan before app.maintenance_step(cx, loan). History functions35/36/40 retain whole H bodies;37–39 preserve same-original app/remote scopes with only fixture policy and redundant reference syntax changes.

All three empty presence JSON schemas (31/85/130) parse with properties {} and no stale required field list. All three TypeScript presence owners (32/86/131) are whole S exact; Rust3D presence129 is whole S exact with the four canonical framework aliases. This is source behavior intent, not a runtime correctness claim.

No additional concrete source-loss finding was established in these inspected root body/context witnesses. The separate extended binding scan below remains open until repairs and fresh recheck.

## Extended Composed Domain Rust Binding Scan

Used final-source-static input paths, bounded to domain Rust. Compared current normalized Git blob identity against immutable H/S and excluded exact authored files. Scanned 748 composed files, masked comments/literals and tracked lexical brace-scope identity for imports. Thirty-two candidate files reduced to22 introduced same-scope binding defects after excluding anonymous `_`/self bindings and requiring each immutable B/H/S total use count≤1. Actual duplicate statements inspected; no cfg-exclusive pair among the22 listed. Authored inherited duplicates outside this filter are not restoration findings.

| Actual owner | Introduced duplicated binding(s), current line witnesses | Immutable B/H/S counts |
|---|---|---|
|`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs`|`InteractiveJob` 28/28|InteractiveJob:0/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/🎲️apply-board-events/🦀️.rs`|`OnceLock` 20/24|OnceLock:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️.rs`|`BuiltNode` 10/12, `Component` 10/13, `NumberStepperProps` 10/16, `Trigger` 10/17|BuiltNode:1/1/1, Component:1/1/1, NumberStepperProps:1/1/1, Trigger:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`|`Component` 12/14, `NumberStepperProps` 12/17, `Trigger` 12/18|Component:1/1/1, NumberStepperProps:1/1/1, Trigger:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs`|`RowActionPlacement` 18/20|RowActionPlacement:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📸️snapshot/🦀️.rs`|`ColdMutation` 151/184, `ColdMutationDiff` 151/184, `ColdDiffAlgebra` 151/184|ColdMutation:0/0/1, ColdMutationDiff:0/0/1, ColdDiffAlgebra:0/0/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs`|`ScratchHashMap` 25/26, `ScratchHashSet` 25/26|ScratchHashMap:0/0/1, ScratchHashSet:0/0/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️accept-suggestion/🦀️.rs`|`Puzzle5dFreshIds` 4/7, `PUZZLE5D_GRANULARITY_PART` 4/8, `Puzzle5dActionCtx` 5/6|Puzzle5dFreshIds:1/1/1, PUZZLE5D_GRANULARITY_PART:1/1/1, Puzzle5dActionCtx:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📡️proximity-connect/🦀️.rs`|`puzzle5d_grip_full_id` 5/6, `world_grip_position` 5/7, `Puzzle5dDocument` 5/8, `Puzzle5dGrip` 5/9, `Puzzle5dPart` 5/10|puzzle5d_grip_full_id:1/1/1, world_grip_position:1/1/1, Puzzle5dDocument:1/1/1, Puzzle5dGrip:1/1/1, Puzzle5dPart:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📨️engagement-submit/🦀️.rs`|`PUZZLE5D_DEFAULT_UTILITY` 14/15|PUZZLE5D_DEFAULT_UTILITY:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️engagement-abort/🦀️.rs`|`Puzzle5dActionCtx` 5/7, `PUZZLE5D_DEFAULT_UTILITY` 6/8|Puzzle5dActionCtx:1/1/1, PUZZLE5D_DEFAULT_UTILITY:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🖌️brush/🧪️tests/🔬️unit/🦀️.rs`|`close_app` 9/10, `dispatch` 9/11, `dispatch_armed` 9/12, `projection_of` 9/13, `window_view` 9/14|close_app:1/1/1, dispatch:1/1/1, dispatch_armed:1/1/1, projection_of:1/1/1, window_view:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs`|`Puzzle5dSnapshot` 10/20, `Puzzle5dGrip` 11/12, `Puzzle5dGrip2d` 11/13, `Puzzle5dGrip3d` 11/14, `Puzzle5dMeta` 11/15, `Puzzle5dPart` 11/16, `Puzzle5dPart2d` 11/17, `Puzzle5dPart3d` 11/18, `Puzzle5dPartAnchor` 11/19|Puzzle5dSnapshot:1/1/1, Puzzle5dGrip:1/1/1, Puzzle5dGrip2d:1/1/1, Puzzle5dGrip3d:1/1/1, Puzzle5dMeta:1/1/1, Puzzle5dPart:1/1/1, Puzzle5dPart2d:1/1/1, Puzzle5dPart3d:1/1/1, Puzzle5dPartAnchor:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦀️.rs`|`Puzzle5dSnapshot` 3/5|Puzzle5dSnapshot:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦀️.rs`|`Puzzle5dTargetVolume` 4/5|Puzzle5dTargetVolume:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦀️.rs`|`Puzzle5dSnapshot` 3/5|Puzzle5dSnapshot:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦀️.rs`|`Puzzle5dSnapshot` 5/7|Puzzle5dSnapshot:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦀️.rs`|`Puzzle5dSnapshot` 4/6|Puzzle5dSnapshot:1/1/1|
|`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦀️.rs`|`Puzzle5dSnapshot` 3/5|Puzzle5dSnapshot:1/1/1|
|`✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs`|`Element` 6/25, `ElementContext` 7/26, `ElementResult` 8/27, `MemberUdl` 9/28, `PlaneStress` 10/37, `PlateMoments` 11/38|Element:1/1/1, ElementContext:1/1/1, ElementResult:1/1/1, MemberUdl:1/1/1, PlaneStress:1/1/1, PlateMoments:1/1/1|
|`✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🦀️.rs`|`InteractiveJobCloseStep` 7/35, `OriginalTurnFailure` 9/37, `JobOutcomeBorrow` 11/39, `JobOutcomeDescriptor` 13/41, `JobOutcomeView` 15/43, `JobPayloadStream` 18/46, `Operation` 20/48, `RetainedCloneGrant` 22/50, `RetainedCloneProgress` 24/52, `RetainedJobPayload` 26/54, `RetainedJobPayloadWriter` 28/56, `StepContext` 30/58, `RetirementDemand` 31/59, `VecD` 33/97|InteractiveJobCloseStep:1/1/1, OriginalTurnFailure:1/1/1, JobOutcomeBorrow:1/1/1, JobOutcomeDescriptor:1/1/1, JobOutcomeView:1/1/1, JobPayloadStream:1/1/1, Operation:1/1/1, RetainedCloneGrant:1/1/1, RetainedCloneProgress:1/1/1, RetainedJobPayload:1/1/1, RetainedJobPayloadWriter:1/1/1, StepContext:1/1/1, RetirementDemand:1/1/1, VecD:1/1/1|
|`✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs`|`Dof` 6/39, `Element` 7/40, `ElementContext` 8/41, `ElementResult` 9/42, `MemberUdl` 10/43, `ShellState` 11/44, `SolidStress` 12/45|Dof:1/1/1, Element:1/1/1, ElementContext:1/1/1, ElementResult:1/1/1, MemberUdl:1/1/1, ShellState:1/1/1, SolidStress:1/1/1|

## Additional Actual Use-Tree Splice Defects

- P2D apply-board-events actual imports: line6 doubled windows::crate::editor prefix; line15 missing braces in `use semio_framework_tool_machine::ChartGesture, GestureChart;`.
- P5D world3d utilities/transform actual import19 missing braces in `use crate::editor::puzzle5d::entities::puzzle5d_action, puzzle5d_axis_index;`.
- FEM LDLT actual import32 missing braces in `use semio_framework_value::HeapSortCursor, HeapSortWork, ValueRefusalKind;`. Immutable S explicitly supplies a correctly grouped use with list::PagedList and these names, proving the malformed current import was introduced.
- P3D `✏️editor/🧵️retained/🗂️document/🦀️.rs` actual47–66: `pub use` is interrupted by original_birth_demands/from_original_construction for Puzzle3dKindWeightWork, followed by `pubself::create_attraction_work::*;`. The constructors sit in the aggregate reexport facade rather than the defining KindWeightWork impl. This is a concrete wrong-receiver source splice. Restore the reexport and preserve incoming constructor bodies at the actual defining child while retaining H close methods.

All22 binding findings and these use-tree defects were sent promptly to root and domain executor. This report is an open checkpoint until their repairs are independently reread; no whole-domain completion claim is made.

## Fresh Independent Recheck After Extended Repairs

The checkpoint findings above have been repaired. Independently reread all24 actual affected files (22 duplicate owners plus malformed-use owners and P3D facade); lexical brace-scope duplicate binding and use-tree checks now return zero findings. This includes nested scopes and excludes anonymous/self pseudo-bindings. The earlier22-file table remains historical evidence of the concrete introduced defects, not a current open finding.

The P3D document facade now has the complete `pub use self::create_attraction_work::*;`. In the actual defining `⚖️kind-weight-work/🦀️.rs`, both original_birth_demands and from_original_construction independently match the whole immutable S methods after whitespace and action_bus namespace normalization. The defining struct and impl are in that same child. A separate bounded748-composed-file source-scope check finds no original_birth_demands/from_original_construction definitions outside an enclosing impl.

Current KindWeight close is intentionally composed: `close_original_nested_step(cx, |cx, _grant| self.close_owners.close_step(cx))`. It preserves the same original caller around the incoming cx-owning child. This is not whole immutable H byte equality: H used the grant-only close_original_granted_step wrapper. The executor's byte-identical-before/after repair statement concerns the current composed close body, not immutable H. No additional source restoration is requested from this distinction.

Domain AST syntax-splice repairs are independently classified in audit-rust-ast-provenance-repeat-695.md. Parser provenance is separated from build/runtime correctness; no build, compiler, test or runtime was run.
