# Preparation close-start custody

ArtifactReplayPreparationTask::into_retirement currently consumes its cursor while calling both cancel and begin_close. NativeSemanticPreparation implements both callbacks by calling the same begin_close, which sets its own flag and calls projection, edit and optional row close starts. Several concrete drivers then mutate their own close flag and invoke nested cursor cancel/begin. A single task-handoff work unit therefore does not quote those producer actions.

A correct conversion separates the original cursor handoff from each actual close-start action. Its selected action and metadata copies must be quoted before the same original work loan authorizes the effect. The native and fixture implementations need explicit close-start demand and outcome ports all the way through the projection/edit drivers; a completed cursor must still skip directly to the already-tested one-work boxed shell release. The current code is uncompiled during the shared native continuation migration.

The Store PagedOneItemEdit production implementation delegates start to RetainedMutationOwner, RetainedDiffOwner, and MutationMessageConcat. The first two owners each delegate again to an abstract preparation/apply trait, including generated implementations; a measured parent flag alone does not account for those child calls. The concatenation owner now has an authored explicit paid close-start port and separate terminal-row shell; its native and neutral laws have not run.

RetainedFieldPreparation now defers its field cursor start to the selected close turn, using the shared demand/work/paid-receipt API. Its own parent flag remains a separate outstanding staging task. The authored native zero-copy denial law has not run; neutral SQLite 32559 passed 1/1, 1 assertion. Operation start neutral suite passed 2/2, 4 assertions after environment restore.

The retained set-field leaf now selects its authored and displaced field start independently; each field quotes and posts its own bool flag, then separately borrows the value cursor paid-start receipt. Existing field tests were updated to the current explicit work-bound, rich-outcome and caller-held allocation-marker signatures. A zero-copy denial and repeated-start native law is authored; it has not run. The common value owner has not declared native source readiness.

The DSL retained aggregate generator had an extra close_demand work parameter absent from its defining trait and its own call. It now matches the trait, and the existing syn generator test asserts the generated arity. Other actual bounded decision/inverse work parameters were preserved.

Normal-capacity replication message Cargo check 48287 exited 101 before its target in the shared retained-clone library, reporting 30 compiler errors while that owner was being migrated. It captured an older source snapshot and is not a message or native-law acceptance. Native owner was notified.

The retained mutation fixture now matches the current bounded decision and inverse interfaces. Its selected test child and each parent binding or ledger action quote one work, so its calls use a one-action ceiling; this is not an original-caller acceptance. IO test allocation callbacks now use the canonical shared NativeAllocationEvent. Physical allocation receipt laws refuse unexpected historical admission, and the SQLite fixture validates its cumulative admission request separately from physical posts. These source changes have not been compiled.

Concat close-start neutral SQLite law passed 1/1 with one assertion under normal capacity. This verifies only the neutral fixture; no native acceptance is implied.

Diff owner start neutral fixture and independent SQLite selected-parent/child oracle passed under normal capacity (43294: 1/1, 20 assertions). It is a specification-first law only; production diff staging and real original-caller proof remain outstanding.

The retained-row-patch macro now selects each resident field start separately and forwards the field owner work and bool-copy receipt before close. Its own parent flag and the surrounding patch, diff and sequence starts still need explicit paid staging; this edit remains uncompiled.

RetainedRowPatchFields and RetainedRowPatchApply now expose selected paid bool starts; the apply owner starts its fields in a separate turn before forwarding their close. The only fields implementation is the row-patch macro. Diff apply and sequence integration remain outstanding, and no compilation acceptance is claimed.

Canonical allocation request names were aligned in ten IO files. Five SQLite transfer/artifact production call sites still use former request-only forwarding closures and need genuine original event-loan and physical settlement migration; changing names alone does not make those call sites source-ready.

SQLite cumulative allocation estimates now forward explicit Admit events to the original port, and the four admission fixture callbacks check Admit and retain the original request and cancellation checks. These changes are uncompiled; transfer and artifact native child call sites still require owned event-loan and physical settlement migration.

The diff neutral corpus was extended with independently denied zero-work parent and child turns. The updated normal-capacity SQLite oracle passed 1/1, 28 assertions; this remains a specification-only result.

A native local patch-apply law is authored for copy denial, one-byte and one-work admission, repeat zero, and zero backing allocations. It has not compiled or run and does not stand in for a real original-caller law; existing containing tests also still require the current bounded advance API.

The existing SQLite original-allocation neutral suite passed under normal capacity (3/3 tests, 36 assertions). The Rust event handling and native physical child settlement remain uncompiled.

The main sparse patch native fixture now uses paid starts, bounded rich advance/close calls and a caller-held source-admission marker. Its finite test work ceiling follows the actual Fields birth quote plus the containing action. Other fixture modules and native source compilation are not yet verified.

The close-demand fixture now retains its measured fields birth and starts through the paid port. Native value is separately preparing a same-caller RetainedAllocationEffect argument for clone cursor advance; retained field/patch call sites will need that exact source-owned marker when the contract lands. No fixture compilation is claimed.

The field fixture no longer uses an unlimited work ceiling: StringCursor selects at most the logical body bytes per copy turn, and output handoff requires the child and parent work actions, so its finite test bound is max(body, 2). Demand calls now pass that bound. The imminent original allocation-effect parameter still must be integrated.

The SQLite export fixture original-port callbacks also match only explicit Admit and retain their cumulative ledger/refusal assertions. Native child forwarding remains a distinct unresolved phase-aware owner problem.

The allocation callback now returns NativeAllocationRefusal, which may own a ValueError or retain a handle in the original mediator. SQLite ValueError-shaped control methods and IO failure codec cannot discard or format that handle: they still require typed resident propagation and exact mediator resolution, including paid decode-side owner transfer. The preceding Admit edits are not source-ready under this newer contract.
