# Exact Typed Admission Repair

The live-core terminal log nested-edit-domain-green-2.log reproduced missing cold mutation/retirement caller bounds, FaultCode formatting through a missing Display implementation, and calls to a nonexistent RetireOwned::retire_owned.

The repair uses protocol::Mutation::retire_cold for unissued operations. Mutation is now an explicit bound alongside ColdMutation and RetireOwned at the generic stage and its scalar/placement/dispatch callers. Plain MutationMessage rows contain only ordinary owned strings/vectors and scalar metadata; dropping these cold rows uses their lawful ordinary destruction. Fault text reads the actual FaultCode string.

Public resolve_norm_edits now passes each authored edit unchanged to the domain resolver against the preceding admitted typed projection. Numeric UI adaptation is isolated in private resolve_field_edits, used by resolve_set_fields and the placement wrapper. A ready form can therefore validate exactly the arguments it later commits.

Native regressions use the existing real sparse-diff model plus protocol::Mutation metadata and an explicit domain retirement counter. The closed neutral nested-edit schema and corpus now include typedAdmission cases covering exact unsigned values and refusal of fractional/text drafts. The native test observes the exact raw value received by the resolver, confirms the earlier sibling is present, compares accepted output with serde_json JSON Pointer writes, and verifies no source mutation. Existing semantic Error/Fatal refusal now also verifies both unissued operations retire once. A UI regression keeps fractional draft rounding on the private adaptation path.

Files changed in this execution:
- app-surface/✏️edit-rules/🎮️commands/🦀️.rs
- app-surface/⌨️command-args-and-dispatch/🦀️.rs (dispatch_live_command bound only)
- app-surface/🕹️direct-manipulation/📍️placement-drag/🦀️.rs (bounds and private adaptation call only)
- app-surface/✏️edit-rules/🎮️commands/🧪️tests/🦀️.rs
- app-surface/✏️edit-rules/🎮️commands/🧪️tests/🧫️model/🦀️.rs
- norm/🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json
- norm/🧪️tests/🪆️nested-edit/🧬️schema/🔣️.json
- norm/🧪️tests/🪆️nested-edit/🟦️.ts

Verification handle 83547 terminated before compilation with CargoLockRefused: the live-editor manifest inherited semio-framework-plugin-decl-staged-form without a norm workspace.dependencies entry. The form owner was notified to correct the declaration. This was an ordinary shared prototype repo cargo test for both semio-s-plugin-norm-app-surface-edit-rules and semio-s-plugin-norm-app-surface-live-core in the norm workspace. Log: 🗑️generated/typed-admission-native.log. No compiler or test green result is claimed at this checkpoint.

The form owner corrected the real missing workspace dependency. Repeat native verification is active as handle 20419 with the same requested package selection and original gates; log 🗑️generated/typed-admission-native-2.log. The existing independent strict Ajv oracle now validates exact typed admission cases and rejects contradictory expected outcomes. No test result is claimed until execution.

Independent oracle execution completed through ordinary repo tools exec bun: 2 passed, 0 failed, 12 assertions. The first run refused the new schema under strictRequired; declaring count in each local conditional schema fixed the schema without relaxing strictness. Logs: typed-admission-oracle.log (red), typed-admission-oracle-2.log (green). Native handle 20419 remains active; no native green is claimed.

Native repeat 20419 also terminated before compilation with the same missing inherited dependency. A fresh bounded source read after termination confirms norm/Cargo.toml has no staged-form dependency key, while live-editor still inherits it. The form owner was informed; a generated-manifest refresh or concurrent overwrite is only a hypothesis, not established evidence. Both requested production packages remain unverified.

The form owner identified its original replacement matched an absent workspace-dependencies section and therefore made no change. There is no evidence of concurrent overwrite or manifest regeneration. It now uses a direct canonical first-party dependency in live-editor; read-after-write confirmed this actual path. Native repeat 11516 is active with the same original package selection and gates, log typed-admission-native-3.log.

Native repeat 11516 terminated after manifest selection during ordinary source admission, before rustc: semio-framework-plugin-instance-app exceeded its existing remaining source-byte boundary at staging/composer-api (18,870 observed bytes against 18,527 remaining, 17 files remaining). The staging owner was notified to refactor actual source within the existing boundary; no limits were changed. No norm native compiler/test acceptance exists yet.

## Metadata And Coverage Review

The native model now implements the real Mutation contract in addition to ColdMutation, so refusal cleanup invokes an explicit domain-owned retire_cold method rather than a nonexistent generic convenience or self-issued physical grant. Its four declared operation rows distinguish applied replacements/warnings from rejected Error/Fatal operations, and its payload codec uses its authored tagged operation representation. Production stage callers require the same actual Mutation bound.

The fractional count case is the decisive regression: the previous public stage silently rounded 5.6 to an unsigned 6 before domain decoding, while the repaired public stage must deliver 5.6 unchanged and refuse it. The test also sees the preceding admitted sibling, proves the source remains unchanged, and counts retirement of the unissued first operation. Separate semantic Error/Fatal tests count disposal of both retained operations; warning operations remain replayable. The UI path separately demonstrates retained rounding behavior.

These are generic typed native laws and an independent Ajv/serde_json oracle, not proof of every real norm domain or browser journey. Actual production package execution and the broader domain/browser matrix remain necessary.

Native repeat 36675 passed the composer source owner but terminated before rustc at semio-framework-plugin-reactor-turn: task-resumes-and-wire-effects was 9,848 bytes against 8,652 remaining (12 files remaining). The framework owner is extracting its genuine staged transport logic into an existing lower production owner. A separate edit-rules-only run 50465 was attempted to verify its boundary laws independently; actual source selection still contained the reactor dependency (431 units) and hit the same refusal. No native package has executed tests yet. All native retries are now held until that actual production refactor is ready.

Read-only fifteen-family vocabulary/acceptance coverage is recorded in family-edit-admission-coverage.md. Actual registered target routes and explicit editor package rosters were verified for all fifteen projects and recorded in matrix-execution-handoff.md; the matrix graph planner has not been launched while parent browser 35572 plans its graph.

## Current Core Retry

Retry 72230 passed source custody and reached Rust compilation, then failed at the AOT mutation derive's missing public CompiledMutationInputDefinitions export in protocol. Runtime producer owner repaired the explicit export chain. The same original edit-rules plus live-core two-package gate resumed as handle 91699 with generated/typed-admission-native-6.log. No test filter or package substitution was used.

Retry 91699 reached Rust and terminated with Cargo status 101 (outer repo status 1): the newly extracted staged-forms package referenced semio_framework_pack::PackLimits in staged.rs line 53, but the declared dependency's explicit library name is pack. Native transport owner corrected the source reference to pack::PackLimits; no manifest or lock change was needed. My initial missing-dependency diagnosis was incorrect. The AOT public export failure from the previous retry did not recur. No edit-rules or live-core diagnostic surfaced before this upstream stop.
