# result-rust-port (executor rust-port, wave 1022a)

CARGO HOLD respected: no cargo run was started. Every edit below is WRITTEN BUT UNVERIFIED by a compile unless a row says otherwise. Verify with
`sh C/rust-health-tools/chain10.sh 1 <prefix>` (teaching tests, then root check) once the coordinator releases cargo.

Tools (all idempotent, dry run first): `C/rust-port-tools/{errs.py,ctx.py,derivedeps.py}` (error summarizer per failing unit, error-with-source-context printer, derive(Mutations) dependency adder); the existing `C/apps-b-tools/{portdiff,portkind,fixselfdiff}.py` and `C/apps-a2-tools/{addreference,portcalls}.py` were run over the non-plugin tree via `C/rust-port-tools/{cand,callers,mutations-sites}.txt` (pre-edit backups `C/🗑️generated/rust-port/pre-port.tar`, `pre-callers.tar`).

## Framework-wide diff/mutation API port (outside `✏️s/🔌️plugins`)
| step | result |
|---|---|
| census | 250 candidate `.rs` files (impl Mutation/MutationDiff/DiffAlgebra/MutationKind, `apply_diff`, `#[mutations(`); the framework hand impls and all leaf kinds were already ported by peers (portkind: 0 of 250) |
| portdiff | 1 file: sync demo fixture (`DemoDiff`: `MutationDiff` split into empty `MutationDiff` + `ColdMutationDiff {absorb, apply}`), hand edit because apply was already `&mut` form |
| addreference | 10 aggregates got `reference = <snapshot>, reference_diff = <diff>` (job-test-mutations, contributed-mutation-wire, test-app-mutations x2, mutation-fixtures dummy/surface/transaction, declaration-channels x3); the other ~25 non-plugin aggregates already had it |
| portcalls | 13 candidate files + 59 caller files (`.diff(`/`.inverse(`/`apply_diff(` call sites and `Mutation`->`ColdMutation` imports, incl. all os-config leaf law tests, mcp, hub, teaching-free); second dry run = 0 |
| derive dependencies | `derivedeps.py`: 10 crates got the crates `derive(Mutations)` expands to: `schema-state`/`os-command` as dependencies, `os-store-one-item` as DEV-dependency (only the generated `#[cfg(test)]` law uses it; a normal edge could cycle through kernel). The derive-expand crate itself is excluded on purpose (cycle). |
| teaching / semio-tech / mit-bestand | no `#[mutations(` aggregates or Mutation impls there: nothing to port |

## Red libs (rust-health table), root cause + fix
| lib | status | root cause / fix |
|---|---|---|
| plugin-guest-contract | fixed, unverified | `original_output_tests` (mounted in the lib under the `original-actor-context-testing` feature) and the lib tests used the pre-696 actor API: `RetainedTurnInput` gained `work: OriginalActorTurnWork`, `return_original(spent, work)`, `admit_step(input, policy, clock)`, `close_step(input, policy)`, `close_failure_step(grant, &mut work)`, `Fault.domain: Option<Box<DslValue>>`. Ported output, job/output, failure, failure-debit and admission tests (fuel/deadline/clock-binding added to the Lane helper; budgets now take the held guest policy) |
| plugin-host-effects | fixed, unverified | `run_router_effect_job` still used `ComputePool::prepare_job_params` (3-arg) and `completion.close_step(&cancel, grant)`. Rewritten on the services-test pattern: direct `BatchJobParams`, one `JobPayloadAuthority` per effect (retired through the wallet before the outcome returns), close turns through `StepContext::with_payload_authority` for completion and unadmitted job, cancel alias returned for refused admission |
| infinite-board-engine | fixed by apps-a2's Camera revert (11:54), unverified | E0117 came from the unified `pub use canvas::Camera` + `artifact_retire_leaf!`; the engine owns `Camera` again with `From` conversions |
| config-merge-policy | fixed, unverified | the aggregate `MergePolicyConfigMutation` lived in the leaf crate lib, below the mutation collection (source-authority gate). Aggregate moved into the collection file `🧬️mutations/🦀️.rs` like every other config aggregate; the carved crate keeps only the schema (`🛡️change-merge-policy/🧬️schema/🦀️.rs`: setting, diff, consts; lib path + description updated); the leaf file is mounted by `os-config` as `pub mod change_merge_policy` (workspace members, locks and leaf descriptor untouched) |
| db-io-registry | fixed (rust-health), verified by reading | custody fields `pub` |
| db-actor | fixed, unverified | `Emit` trait not in scope for `Arc<NullEmit>`: `use db_observe::{Emit as _, ..}` |
| mcp-port | fixed, unverified | `#[serde(with="crate::inference::dsl_json")]` -> imported `dsl_json` module |
| mcp-probe-document | fixed, unverified | `SQLITE_SCHEMA` lives on `sqlite_snapshot::ArtifactSqliteSnapshot`; tests use the crate-root `pack::snapshot::ArtifactDsl` alias (`store::pack` does not exist) |
| mcp-hub-remote | fixed, unverified | `descriptor_digest_v1`/`hex_lower` -> `os_directory::io::binary::{descriptor_digest,artifact_hash}`; dead `validate_snapshot` import removed (the function is `cfg(test)` only); in-crate tests use `crate::remote::`; hub-pair `cfg(test)` counters/wipe accounting -> `cfg(any(test, feature = "testkit"))` (hub-remote tests enable the feature) |
| mcp-foundation (test) | fixed, unverified | `const FIXTURE = fixture_text!(..)` is a runtime read: function |
| repo-tickets | fixed, unverified | peer refactor removed `decode_agent` but kept the call: `decode_agent` restored and `ticket.agents = agents` (sessions merged from agents as before) |
| repo-hooks | fixed, unverified | provider registry renamed `get_editor_provider` -> `get_agent_client_provider` |
| run-admission | fixed (rust-health), verified by reading | `ColdMutation::diff` |
| sqlite-snapshot-testing-backing | fixed (rust-health), verified by reading | `sqlite_snapshot::ArtifactSqliteSnapshot` |
| flow-host-source | fixed (rust-health), verified by reading | `#[value(send_only)]` on SeedOwners/WorkSources/SourceOwners |
| value-derive `neutral_owner` | fixed, unverified | not a cursor bug: `RetainedCloneSource::constructor_copy_bytes()` is 0 since the SealedShared redesign, so the fixture grant (`maximumCopyBytes` 8 + constructor bytes) could never fund the derived record's final construct step (`size_of::<Record>()`); the unbounded `loop` in the borrowed test then spun forever. Grant now `fixture + size_of::<Record>()`, loop bounded by an assertion |
| os-store-one-item-batch `mutation_apply` | rewritten, unverified | see section below |

(plugin-window-config / plugin-operation: apps-b.)

## `os-store-one-item-batch` `mutation_apply` (55 errors in ws20)
Root cause: commit 696 fused two generations of the suite into one file (old by-value `Document`, `OriginalMutationIssuer` and the `ConfigApplyMutation<Document>` stub with a mangled `exchange` body next to the newer immutable-root tests) and the retained mutation/diff traits then changed arity again. Nothing in that file compiled since. Fixes (all in `📬️paged-one-item/🧬️mutation-apply/`):
- lib `🦀️.rs`: public read accessors `MutationApplyEdit::stage()` and `inverse_rows()` (the external test target read private fields).
- `🧪️tests/🌳️immutable-root`: `Document` no longer derives `SnapshotProjection` (the record derive yields `Output = Box<Self>`, but the paged factory requires `Output = S`); it implements `ImmutableRootProjection` (title alias + `PersistentList::project`), which gives `SnapshotProjection<Output = Self>` through the blanket impl.
- `🧪️tests/🧳️semantics` and `/🔺️apply`: `decide`/`inverse`/`preflight` gained the `body` parameter, `RetainedFieldPreparation::advance(source, body, grant)`, `protocol` alias ambiguity replaced by the crate path.
- `🧪️tests/🦀️.rs`: one coherent driver on the funded API (`Receiver` over `OriginalJobDecodeOwner::receive`, the pattern of `snapshot-clone/⚖️law`), counting `OriginalMutationIssuer`, `Outcome::Completed` carries the plain post and the original diagnostics, stale `ConfigApplyMutation<Document>` stub and the superseded by-value large-document test removed, `quick` tests step the edit through `step_edit` and the new accessors, scalar config test and `source-custody` test advance through the receiver (zero-item denial is observed inside the receiving turn so the `(0, 0)` heap law still measures only the owner).
- 7 stdio/shooting plugin tests still called `drive_one_item_preparation_law(factory, snapshot, mutation, grant)`: `C/rust-port-tools/lawbody.py` inserted the body argument (`1 << 16`), idempotent.
Not run: cargo hold. Expect a second round of small type errors in the fixture code (it is ~1,500 dense lines against four redesigned traits).

## Other findings and fixes
- `portcalls.py` adds `ColdMutation`/`ColdDiffAlgebra`/`ColdMutationDiff` to every `use` that names the base trait, even when another `use` of the same scope already imports it (E0252). `C/rust-port-tools/dedupuse.py` (scope-aware, idempotent) removed 16 duplicates in the non-plugin files I touched and 42 in plugin files (apps-a2/apps-b output); a second dry run reports 0. Rerun it over any tree the portcalls tool was applied to.
- store testing prelude now re-exports `ColdMutation`, `ColdMutationDiff`, `ColdDiffAlgebra` (bin-01..10 and sync-tests use them through the glob); `replay-retirement` uses `ledger::ColdReplayProjection`.
- Syntax proof without cargo: `C/rust-port-tools/syntax.sh <files>` (rustfmt parse only, writes nothing) reports no error for every Rust file I edited, nor for the 4,573 plugin files that mention a Cold* name.
- Locks are stale (new path-dependency edges): run `update --workspace --offline` for the root, framework, hub, teaching and plugin workspaces on first "cannot update the lock file".
- Still red and NOT mine (test-crate carve, see ws20): os-store-tests-01..09 / sync-tests-1,2 (missing `os_store`/`os_vcs`/`os_spr` crate-root aliases in carved fixtures, `receiving_fixture`, `semio_framework_job_payload`/`async` dev-deps, `canonical-edit` tests need the new two-argument encode/cursor calls), kernel / dsl / host-space / flow-vcs-laws / spr-laws-tests / durable-group-tests / member-open-tests companion tests, `world_model_unit`.

## Commands run
No cargo. `rustfmt --edition 2021 --emit stdout <file>` (syntax only) on edited files; `python3` codemods listed above; `git diff HEAD` / `git show` read-only.
t1 pid 79802 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t1.cargo.log
t2 pid 82415 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t2.cargo.log
t3 (update+check) pid 86342 logs .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t3-update.log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t3.log
t4 pid 87928 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t4.log
t5 pid 94603 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t5.log
t6 pid 95569 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t6.log
t7 pid 99925 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t7.log
t8 pid 11325 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t8.log
t9 pid 12370 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/t9.log
u1 (mutation_apply test) pid 15224 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/u1.log
u2 pid 28298 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/u2.log
u3 pid 31667 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/u3.log
u4 pid 33937 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/u4.log
u5 pid 35480 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/u5.log
v1/v2 tests-all pid 35930 logs .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/v1.log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/v2.log
w1 pid 49997 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/w1.log
w2 pid 55064 log .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/rust-port-logs/w2.log

## VERIFIED after cargo release (13:14+) -- logs `C/🗑️generated/rust-port-logs/`
`check -p <14 crates> --all-targets --keep-going` (t9): rc 0, 0 errors: guest-contract (lib + `original_actor_poll_input`), host-effects, infinite-board-engine, config-merge-policy, os-config, db-actor, mcp-port, mcp-probe-document, mcp-hub-remote, mcp-foundation, repo-tickets, repo-hooks, os-store-one-item-batch (incl. test `mutation_apply`). Compile fixes found by the real compiler: `MutationApplyEdit::inverse_rows` (peer-corrected), hub-pair `pair` structs `pub`, probe-document `pack::snapshot::ArtifactSqliteSnapshot` + `NumberWord` visibility, guest-contract `wasmtime` dev-dependency + `original_actor_poll_input` port, os-config test builds need the kernel `mutation-testing` dev feature (also dag-vcs, interaction).
Tests run: `os-store-one-item-batch --test mutation_apply` **16 passed, 0 failed** (v1.log). Test-law corrections from the real run: drop-time frees are `requested_bytes == 0` (the Box frees at drop), snapshot box close grants come from `artifact_retirement_box_demands`, `apply_mutation` is documented policy-agnostic (apply count 1 even for refused diagnostics), vector-issuer fixture path, replay routes removed (suffix replay now requires a retained `ArtifactReplayPreparationFactory` over a `Clone + ColdMutation` state; the immutable `Document` is neither).
Facade importers (coordinator addition): `semio_framework_value::{ordered,paged_text}` -> `semio_framework_value_{ordered::ordered,paged_text::paged_text}` in the `app-typed-command-full-operation` child-operations tests (laws, owner, encoder) and the procedural sqlite-native crate; deps `semio-framework-value-ordered` (laws dev-dep, sqlite-native) and `semio-framework-value-paged-text` (tests-01) added; locks refresh on first "cannot update the lock file" (not compiled yet).

## WIND-DOWN STATUS (22.10 ~21:50)

All my detached processes are killed; all `[DEBUG]` prints from this task are removed (router-effect-handler-and-jobs, compute-pool, job payload builder, pack borrowed cursor, guest-contract admission test); touched files pass the rustfmt parse proof. Last compile-verified state: t9 (`check --all-targets` green for all 13 libs incl. `os-store-one-item-batch` test `mutation_apply`, which runs 16/16).

Tests (run results, not compile-only):

| item | status | root cause / remaining |
|---|---|---|
| os-store-one-item-batch `mutation_apply` | GREEN 16/16 | - |
| host-effects `router_effect_original_sources_keep_capacity…`, `original_retirement_loan_closes_router_after_semantic_deadline`, `…box_frame…` | GREEN | - |
| host-effects `router_effect_recording_leases…` | FIXED, unrun | test lacked `original.finish()` per case (fixture caller drop assert) |
| host-effects `router_effect_runs_through_the_retained_compute_session` | RED, test edited, unrun | `ManualRuntime` never advances its clock, so `run_job`'s 1 ms poll sleep never wakes; test now ticks `set_now_ms`. Second finding: `ComputeJobCompletion::close_step` makes zero progress after `result=Err(WorkerLost)` (grant is ample) - suspect cancel-alias custody (`has_original_cancel_alias_witness` / `return_original_cancel_alias_step` → `Blocked`); not diagnosed. |
| host-effects `router_effect_on_a_stopped_compute_pool…` | RED/hung, test edited (lane 2) | lane 0 = Interactive steps on the caller (never touches the pool); test now uses lane 2. Same zero-progress close as above. Kills the run if it hangs (ManualRuntime sleep). |
| guest-contract `original_entry_full_failure_…` | RED, test restructured, unrun | `StepContext.payload_page_granted` allows ONE payload page/copy per context; the test looped ~700 publication turns in one `caller.turn`. Rewritten to one `caller.turn` per publication turn (entry kept outside, address-stable); needs a compile + run. |
| guest-contract `paired_admission…` | RED (`turns < 1024`) | still undiagnosed; each turn already has a fresh context, so not the page gate. Debug print was added then removed; suspect the same copy/fuel budget in `publish_failure_call`. |
| config-merge-policy / os-config / infinite-board-engine / neutral_owner tests | not run | check green; the mutation-testing dev-feature fix is in |
| facade importer re-pointing (11 importers) | check-unverified | edits done, no compile after them |
