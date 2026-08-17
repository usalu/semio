# W0-A Kernel Spine — Report

Delivers C1–C5 + C10. All commands run from `/Users/ueli/Documents/semio`. Full compiler/test output in this folder: `🧪️w0-a-cargo.txt` (check + filtered test), `🧪️w0-a-cargo-test-full.txt` (unfiltered full-crate test run).

## Acceptance (real, ran)

- `cargo check -p semio-framework-os-kernel` → **clean**. `Finished \`dev\` profile [unoptimized] target(s) in 4.81s`, 0 errors, 10 pre-existing-shape warnings (2 unrelated `unexpected cfg` for `feature="js"`, rest `unused_qualifications`/`dead_code` — one new `dead_code` on `PendingEdit.line_no` is a direct, expected consequence of deleting a `validate` call that used to read it).
- `cargo test -p semio-framework-os-kernel --lib -- os_spr::command` → **37 passed; 0 failed**.
- `cargo test -p semio-framework-os-kernel --lib` (unfiltered, whole crate) → **879 passed; 1 failed** — `os_store::component::tests::dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing`. This is `CompositionCoordinator`'s (1-E's exclusive region) phase-1 rejection test; it now fails because `SpaceMember::validate_wire` (which I had to touch to keep the crate compiling — see "Boundary crossings" below) no longer calls the deleted `Mutation::validate`. Not fixed — real rejection there is 1-A/1-E's C6 algorithm work.

## C1 — Severity (`🗣️dsl/⚠️diagnostic/🦀️component.rs`)

`Severity { Info, Warning, Error, Fatal }`, that declaration order, `#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]` (Ord/PartialOrd are new — the enum didn't have them before). `as_u8`/`from_u8` (0..3) added.

`Hint`→`Info` repo-wide, done in my lease only:
- `🗣️dsl/⚠️diagnostic/🦀️component.rs`: the enum itself (`Hint` variant renamed to `Info`, reordered).
- `🕸️graph/🗣️dsl` = `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`: its own **separate** `DiagnosticSeverity` enum (`Error, Warning, Information, Hint`, an LSP-shaped mirror, unrelated type to `os_dsl::Severity`) — its `Hint` variant renamed to `Info` per explicit lease instruction, despite already having an `Information` variant (the two are now adjacent-but-distinct, as they were before under different names).

**Not touched** (found via `grep -rn "Severity::Hint"`, outside my lease, other lanes' files): `🔌️plugin/🦀️component.rs:18426` (2-A), `✏️s/🔌️plugins/🗄️stdio/.../📄️pdf/.../🧬️schema/🦀️component.rs` ×2 (3-E/FULL-STDIO). Both will fail to compile until those lanes land — expected red window.

## C2 — Message & Outcome (`📡️spr/🎮️command/🦀️component.rs`, new region `🔖️Message` after `🔖️Mutation`)

```rust
pub struct MutationMessage { pub level: Severity, pub code: FaultCode, pub message: String, pub target: Vec<String>, pub op_index: Option<u32> }
// (all fields pub — matches this crate's existing convention, e.g. MutationEvent/ForeignStep)
impl MutationMessage {
    pub fn info/warn/error/fatal(code: impl Into<FaultCode>, message: impl Into<String>) -> Self;
    pub fn at(self, target: impl IntoIterator<Item = impl Into<String>>) -> Self;
    pub fn at_op(self, op_index: u32) -> Self;
}

pub fn worst_level(messages: &[MutationMessage]) -> Option<Severity>;

pub struct MutationOutcome<D> { /* private: diff: D, messages: Vec<MutationMessage> */ }
impl<D: Default> MutationOutcome<D> {
    pub fn empty() -> Self;
    pub fn fatal(code, message, target) -> Self;   // forces diff = D::default()
    pub fn error(code, message, target) -> Self;   // forces diff = D::default()
}
impl<D> MutationOutcome<D> {
    pub fn new(diff: D) -> Self;
    pub fn diff(&self) -> &D;
    pub fn messages(&self) -> &[MutationMessage];
    pub fn into_parts(self) -> (D, Vec<MutationMessage>);
    pub fn info/warn(self, code, message) -> Self;              // 2-arg chainable, targetless
    pub fn absorb_messages(self, messages: impl IntoIterator<Item = MutationMessage>) -> Self;
    pub fn stamp_op_index(self, op_index: u32) -> Self;
    pub fn worst_level(&self) -> Option<Severity>;
    pub fn is_applicable(&self, policy: MergePolicy) -> bool;
    pub fn map<D2>(self, f: impl FnOnce(D) -> D2) -> MutationOutcome<D2>;
}
```

**Deviation from the literal contract text, documented in the code (`🔖️Message` region doc comment):** the frozen prose lists both a static `MutationOutcome::error(code,msg,target)` (3-arg) AND a chainable `.error(..)` (2-arg) instance builder — these cannot coexist under the same name (Rust E0592: an inherent associated fn and an inherent method of the same name conflict across impl blocks whenever their generic bounds overlap, regardless of arity). I kept `error`/`fatal` as the static whole-outcome-rejecting shortcuts (the common single-verb case) and `info`/`warn` as the 2-arg chainable builders (matches the fan-out recipe's own literal example: `MutationOutcome::new(diff).info("mutation.cascade", ..)`). A leaf needing a **targeted** error/warning alongside a non-empty diff builds the message directly and calls `.absorb_messages([...])`.

Frozen 7 codes are documented in the doc comment table; nothing else introduces a code — grepped for stray `"mutation\."` string literals in my own edits, only the 7 appear.

## C3 — MergePolicy (`📡️spr/🧾️wire/🦀️component.rs`, region `🔖️Policies`)

`MergeStrategyKind`/`ConflictRule` deleted. `UndoPolicy`/`StateClass` untouched (StateClass lives in its own `🔖️StateClass` region, not touched).

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergePolicy { LaissezFaire, #[default] Normal, Vigilant }
impl MergePolicy {
    pub fn rejects(self, level: Severity) -> bool;  // LF: Fatal; Normal: >=Error; Vigilant: >=Warning
    pub fn as_u8(self) -> u8;    // 0..2
    pub fn from_u8(value: u8) -> Option<Self>;
}
```

## C4 — Traits (`📡️spr/🎮️command`)

- `Mutation<P>::diff -> MutationOutcome<Self::Diff>`; `validate`/`merge_strategy`/`conflict_rule`/`reconcile` deleted from the trait.
- `MutationKind<P,Op>::diff -> MutationOutcome<Op::Diff>`; `validate` deleted. `SemanticMutation`/`inverse`/`label`/`target`/`foreign_steps` unchanged.
- `MutationDescriptor` loses `conflict_rule` (struct field + `new(id, schema_version, state_class)`, now 3-arg). **Golden fingerprint re-baked**: `334fc1a502f10a879eec47edbbd526249432f67f4ccee1daa10a1f40821c8bdd` (recomputed by running the test and reading its own panic output — not hand-derived).
- `Planner<P, Op>`: `call` now computes `op.diff(&self.base)`, folds its messages into `self.messages` with the step's index as an outermost `target` prefix (`"step-{index}"`), stops (returns `Err(PlanError::StepRejected(reason))`, `reason` = joined `Fatal` message texts) only on a `Fatal` message — `Error`/`Warning`/`Info` still advance `base` and continue planning. New `messages()`/`into_parts()` accessors.
- `plan_of` no longer calls the deleted `CompositeMutationKind::validate`.
- `fold_plan_diff -> MutationOutcome<Op::Diff>`: **all-or-nothing** — if planning fails (`PlanError`) or any collected message reaches `Error`/`Fatal`, the diff is `Default::default()` but every message is kept; a `PlanError` additionally contributes one `Fatal` `"mutation.invariant"` message (`error.to_string()`). Otherwise folds normally via `MutationDiff::absorb`, still carrying any `Info`/`Warning` messages.
- `fold_plan_inverse`/`plan_foreign_steps`: mechanically adapted (`.diff(..).into_parts().0`), semantics unchanged.
- `CompositeMutationKind::validate` deleted.
- `vcs::apply_mutation(snapshot, op) -> (P, Vec<MutationMessage>)` (`🌿️vcs/🦀️component.rs`).

## C5 — Conflicts (new `📡️spr/⚔️conflict/🦀️component.rs`)

Landed exactly the C5 type list (`ConflictId`, `ConflictKind`, `ConflictStatus`, `ConflictResolution`, `Conflict`, `EditMessages`, `DispatchReport`, `MergeReport`), re-exported from `📡️spr/🦀️component.rs`. `ConflictId::new(kind, artifact_id, mutation_ids, hlc)` = `blake3::hash` (this crate's existing direct-`blake3` convention, same as `🌿️vcs::content_addressed_entity_id`/`📡️spr/🎮️command::descriptor_fingerprint` — no new dependency) over `(kind-tag, artifact_id, sorted mutation ids, hlc physical_ms/logical/actor)`, hex-formatted as `"conflict-{hex}"`. 4 unit tests included (determinism/content-sensitivity, serde round trip for `MergeReport`, distinctness).

`MutationEnvelope` (embedded in `ConflictKind::Quarantined`) has no `Serialize`/`Deserialize` in this crate, so `ConflictKind`/`Conflict` derive only `Clone, Debug, PartialEq` (not serde) — matches `MutationEnvelope`'s own convention. `ConflictId`/`ConflictStatus`/`ConflictResolution`/`EditMessages`/`DispatchReport`/`MergeReport` all derive serde.

## C10 — Deletions

- `📡️spr/🔀️crdt/**` — directory deleted (`rm -rf`).
- `📦️glue.rs` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`) — `pub mod crdt` `#[path]` entry replaced with `pub mod conflict` pointing at the new `📡️spr/⚔️conflict/🦀️component.rs`.
- `pub use crate::os_spr::crdt::merge_concurrent_diffs;` deleted from `📡️spr/🦀️component.rs`; replaced with `pub use crate::os_spr::conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport};`.
- `MergeStrategyKind`, `ConflictRule` — deleted (see C3).
- `Mutation::{merge_strategy, conflict_rule, validate, reconcile}` — deleted (see C4). `MutationKind::validate` — deleted. `CompositeMutationKind::validate` — deleted.
- `ReconcileReport`/`ReconcileSeverity` — deleted from `📡️spr/🎮️command`.
- Testkit (`📡️spr/🧪️testkit/🦀️component.rs`): `assert_crdt_commutative`/`assert_crdt_idempotent` deleted, their sole test (`crdt_commutative_and_idempotent_hold_for_every_strategy`) and its now-orphaned `RegisterDiff`/`meta_at` fixtures deleted. `assert_mutation_inverse_law` adapted (`.diff(..).diff().apply(..)`). Bench crate (`📡️spr/🧪️testkit/benches/protocol.rs`): `bench_crdt_merge`/region `🔖️Crdt` deleted, removed from `criterion_group!`.
- `MutationDescriptor.conflict_rule` — deleted (constructor + fingerprint input; pin re-baked, see C4).
- **7 `protocol_crdt` doc-mentions repo-wide** (grepped exhaustively, exactly 7 hits): rewrote the 6 in my lease (`📡️spr/🎮️command` `absorb` docstring — the actual "absorb docstring referencing protocol_crdt" the C10 text names; `📡️spr/🦀️component.rs` crate doc; `📡️spr/🧾️wire` region doc; `📡️spr/⚔️conflict` new module doc, self-referential; `📡️spr/🧪️testkit/benches/protocol.rs` module doc; `🌿️vcs/🦀️component.rs` `🔖️MergeStrategy` region doc). **Left untouched, out of lease**: `🛢️db/⚔️conflict/🦀️component.rs:13` (2-E's `🛢️db` lease) — still says `protocol_crdt::merge_concurrent_diffs`, now stale; flagging for 2-E.

## Mechanical return-type adaptation of every in-crate hand-written `impl Mutation`/`impl MutationKind`

**Kernel crate (`semio-framework-os-kernel`, gates my acceptance bar):**
- `🏪️store/🦀️component.rs`: `replay_mutations`/`ingest_remote` minimal-wrapped as instructed. Also — **required, not optional, since Rust compiles the whole crate as one unit** — fixed everything else the C1–C5/C10 deletions broke here: all 13 `apply_mutation(...)` call sites (`.0` appended, messages discarded — 1-A's real algorithm still to come), all 5 `Mutation::validate`/`validate_wire`'s internal `.validate()` call removed (mechanical deletion only, no replacement rejection logic), `reconcile_with_last` turned into a same-signature no-op stub (`Mutation::reconcile` no longer exists; `SpaceConflict`/`materialize_document_snapshot_with_conflicts`/`snapshot_with_conflicts` all kept alive as-is on top of the stub rather than deleted, since C6/1-A explicitly owns their real replacement and touching their public shape seemed riskier than a no-op body), `impl From<ReconcileReport> for SpaceConflict` deleted (source type gone), `SpaceHistoryMutation`'s `diff` wrapped, 4 test-fixture `impl Mutation` (`DemoMutation`/`LossyMutation`/`TimestampedMutation`/`ValidatedMutation`) wrapped, `ValidatedMutation`'s `fn validate` override deleted, one test (`reset_and_apply_reject_malformed_history_or_invalid_mutations_before_persisting`) trimmed of its now-untestable last paragraph and renamed, one test (`default_reconcile_hook_is_a_no_op_for_existing_document_kinds`) trimmed of its direct `.reconcile()` call and renamed. **Boundary crossing**: `validate_wire` (line ~5468) lives in the `🔖️Space`→`SpaceMember` sub-region, nominally 1-E's (`🔖️Composition`/`🔖️Space`/`🔖️CompositionCoordinator`) — I only removed its dead `.validate()` call (same signature, no new logic) because it was a hard compile error blocking the whole crate; flagging for 1-E to land the real `preview_wire`/`merge_policy()` per C6.
- `📡️spr/🔗️causal/🦀️component.rs`: one test-only fixture (`CausalAddOp::diff`) wrapped — `📡️spr/🔗️causal` is nominally 1-A's lease per the ownership table, but its test module is compiled into the same crate and had a hard `E0053`; fixed for the same "whole crate is one compile unit" reason as above.

**Downstream crates named in the fan-out recipe / ownership doc, NOT part of `semio-framework-os-kernel`** (verified: `🪐️space` mounts only via `🖥️host`'s glue with `extern crate semio_framework_os_kernel as protocol`; `🔁️workflow`/`🌊️flow` mount via `semio-framework`'s and `🌊️flow`'s own glue with the same kernel-aliasing pattern; `♾️infinite` is its own crate; `💻️os/🎚️config` is mounted into `🖥️host` **and every plugin's own glue.rs** — so it isn't gated by any single `cargo check`). Adapted the same way (mechanical `MutationOutcome::new(diff)` wrap, `.diff(..).diff().apply(..)` at call sites, deleted-`validate` handling) but **not independently compiled** — `cargo check -p semio-framework-os` (the host crate) stops first on unrelated `🔌️plugin` crate errors (2-A's lease: `NoTransientMutation`/`NoPresenceMutation`/`NoConfigMutation`/`InteractionConfigMutation`'s own `diff` signatures, plus `.validate()`/`.apply()` call sites in `🔌️plugin/🦀️component.rs` itself), confirming the expected red window rather than anything wrong in my edits:
  - `🪐️space/🦀️component.rs`: `SpaceMutation`/`CollectionMutation` (the space-local one) `diff` wrapped, both `fn reconcile` overrides deleted. `reconcile_space_atelier_invariant`/`reconcile_collection_integrity`/`dedupe_folder_names`/`dedupe_entry_names` converted from `Vec<ReconcileReport>` to `Vec<MutationMessage>` (mapped: former `Warning`→`mutation.clamped`, former `Info`→`mutation.cascade`, former `Blocking` (folder-cycle)→`mutation.clamped` too, since none of the frozen 7 fit "severe but still applied" and `Error`/`Fatal` both imply no state change happened, which is false here — the original free-form id string now lives as the first `target` segment). 8 tests updated to match (`.target.first()` instead of `.id`).
  - `🔁️workflow/🦀️component.rs`: `WorkflowMutation`/`RunMutation` `diff` wrapped. `RunMutation::validate` (a REAL enforcement — "sealed run is immutable", not a no-op) converted to a plain inherent method `check_not_sealed` (trait no longer has `validate`); its one caller `apply_run_operation_checked` updated to call it directly instead of through the trait.
  - `🌊️flow/🌿️vcs/🦀️component.rs`: `FlowMutation::diff` wrapped.
  - `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`: `DagMutation::diff` wrapped.
  - `💻️os/🎚️config/🧬️schema/🧬️mutations/{🦀️component.rs,📌️set-default-app/🦠️mutation,🧹clear-default-app/🦠️mutation}`: `OpeningConfigMutation`/`SetDefaultApp`/`ClearDefaultApp` `diff` wrapped; `apply_opening_config_mutation` and one test updated to `.diff(..).diff().apply(..)`.

## Left for other lanes (precise, not fixed by me)

- `🔌️plugin/🦀️component.rs` (2-A): `NoTransientMutation`/`NoPresenceMutation`/`NoConfigMutation`/`InteractionConfigMutation::diff` return types, plus `.validate()`/`.apply()` call sites at lines ~10824/11872/11876/12983/13116/13265/13396 (approximate, shift as the file changes) — confirmed via `cargo check -p semio-framework-os` (fails there first, before host's own files are even reached). Also the `Severity::Hint` site at `🔌️plugin/🦀️component.rs:18426`.
- `✏️s/🔌️plugins/🗄️stdio/.../📄️pdf/.../🧬️schema/🦀️component.rs` (3-E/FULL-STDIO): 2 `Severity::Hint` sites.
- `🛢️db/⚔️conflict/🦀️component.rs:13` (2-E): stale `protocol_crdt::merge_concurrent_diffs` doc mention.
- `🏪️store/🦀️component.rs` `🔖️CompositionCoordinator`/`SpaceMember::validate_wire` real algorithm (1-A/1-E, C6): my stubs make the crate compile and keep 879/880 tests green, but `dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing` needs the real `preview_wire`/policy-based rejection to pass again.
- `SpaceConflict`/`reconcile_with_last`/`materialize_document_snapshot_with_conflicts`/`snapshot_with_conflicts` (1-A, C6 explicitly names these for deletion) — I did NOT delete them (kept as no-op stubs instead) to avoid destructively touching 1-A's `🔖️ArtifactStore`/`🔖️Backbone`/`🔖️Materialize` regions' public shape more than the minimum needed to compile; 1-A should replace with the real `⚔️conflict`-based mechanism per C6.
- C6/C7/C8/C9 (store real algorithm, history persistence, channel, guest/host/UI wiring) — not started, not mine.

## Files touched (created/edited/deleted)

- Edited: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`, `📡️spr/🧾️wire/🦀️component.rs`, `📡️spr/🦀️component.rs`, `📡️spr/🧪️testkit/🦀️component.rs`, `📡️spr/🧪️testkit/benches/protocol.rs`, `📡️spr/🔗️causal/🦀️component.rs`, `🌿️vcs/🦀️component.rs`, `🗣️dsl/⚠️diagnostic/🦀️component.rs`, `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`, `🏪️store/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`, `🪐️space/🦀️component.rs`, `🔁️workflow/🦀️component.rs`, `🌊️flow/🌿️vcs/🦀️component.rs`, `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`, `💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs`, `💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦠️mutation/🦀️component.rs`, `💻️os/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦠️mutation/🦀️component.rs`.
- Created: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⚔️conflict/🦀️component.rs`.
- Deleted: `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔀️crdt/` (whole directory).
- Scratch/logs (this ticket folder): `🧪️w0-a-cargo.txt` (combined check+test), `🧪️w0-a-cargo-test.txt`, `🧪️w0-a-cargo-test-final.txt`, `🧪️w0-a-cargo-test-full.txt`, `🧪️w0-a-cargo-combined.txt`.
