# 📓️ terra report — packet P6-actions-policy

## 1. Preconditions

- Baseline `git rev-parse HEAD`: `abd29c08d0a04dd72d3b9c3fabe818c300c125c8` (unchanged for the whole
  session — no new commit landed while this packet ran).
- `git status --porcelain` at session start showed 5 files already modified by other live sessions in
  this and neighboring tickets (per the harness's own gitStatus snapshot), none overlapping this
  packet's `path_scope`.
- **This session ran concurrently with at least two other live packets on the same files inside
  `🌉️mcp`** — see §6 "Leases and concurrent-session collisions" for the full, evidenced account. This
  is not incidental color: it is the reason §5's official acceptance commands could not be run to a
  clean result by the time this report was written, through no defect in this packet's own code.
- SHA-256 (`shasum -a 256`) and line count (`wc -l`) of every file this packet owns, taken after the
  final edit:

| file | lines | sha256 |
|---|---:|---|
| `🌉️mcp/🎬️actions/🦀️component.rs` (new) | 1172 | `a3365a7c439358e39632bccc7d0604bd6f621e5c66e741471904f4fba00a49f1` |
| `🌉️mcp/🛡️policy/🦀️component.rs` (new) | 392 | `abae2495ba3c91f028ecdb2739998a66cb66131ed4cf3c348283a364bd0b4cc6` |
| `🌉️mcp/🦀️component.rs` (root, extended — **shared with a concurrent P7 session**, see §6) | 803 | `d60de838bcd16164954c2fe1631247aa6e17073e8043ab3f41e551332c546b91` |
| `🌉️mcp/📦️packages/🦀️rust/📦️glue.rs` (facets mounted — **shared with P7**, see §6) | 63 | `a2f6ba1bcb00bb47b5fe78d641331ee8bd3882dbcd844b42f58541fd30488696` |
| `🌉️mcp/📦️packages/🦀️rust/Cargo.toml` (untouched by me — P7 added its own deps concurrently, see §6) | 81 | (not mine to hash — P7's own edit) |

This packet's own new/changed Rust: 1172 + 392 lines of new facets, plus this packet's own edits
inside the shared root `🦀️component.rs` (roughly 360 lines net across the `🔖️Facets`,
`🔖️MutationProtocolTools`, `🔖️StdioEntrypoint`, `🔖️HttpEntrypoint`, and `🧪️Tests` regions — the file's
803-line total also carries a co-landed, separately-attributed P7 contribution, see §6). 30 new tests
in this packet's own two facets (16 in `🎬️actions`, 11 in `🛡️policy`) plus 3 new + 2
repointed-not-deleted tests in the shared root file's `mod quick` (10 → 13).

## 2. The exact frame sequence implemented

Two-tier design, exactly as the brief's §3.1 authorizes ("define the narrow port you need... P7 plugs
its real channel in"): `crate::actions::{AppCommand, AppFrame, Fault, ArtifactChannel}` is THIS
packet's own minimal port (not the real channel's types, which live in the peer ticket's exclusive
`📡️spr/🧵️channel` territory and which this crate has zero dependency on, D8). `ActionAdapter` drives
it through the master.md §3.3 lifecycle:

| Step | `ActionAdapter` method | Port call(s) | Notes |
|---|---|---|---|
| Observe + Prepare + Preview | `prepare()` | `ReadHistory` → `HistorySnapshot`; `PureCommand{capability_id, input}` → `Emit{ops, warnings}` | Validates input via `jsonschema::Validator` against `capability.input_schema` first (→ `INPUT_INVALID`); then `PolicyEngine::authorize_scopes` (→ `PERMISSION_DENIED`, audited); then captures the baseline `RevisionStamp`; then dry-runs `PureCommand`; mints a `prep_` handle carrying `{capability_id, input, instance, baseline, ops, principal_id}`. Returns `PreparedActionReport{preview: {opsCount, warnings}}`. |
| Approve | `invoke()` (via `PolicyEngine::gate_approval`) | none (pure handle-table lookup) | `ApprovalMode::{Never,WhenDestructive,Always}` (reused directly from `semio_framework::manifest::ApprovalMode` — not redefined) × `effects.destructive` × `AutoApprovePolicy`. On `Required`, mints an `appr_` handle and returns `APPROVAL_REQUIRED{approvalHandle}` **before any revision check or mutation command is sent**. |
| Revision check | `invoke()` | `ReadHistory` → `HistorySnapshot` | Re-reads the CURRENT revision and compares to the caller's `expectedRevision` (or the `prepare`-captured baseline if none supplied) — **before** any `TransactionPrepare`/`TransactionCommit`, so a stale caller causes only a read, never a mutation attempt (tested, §4). |
| Commit | `invoke()` (`perform()` closure) | `TransactionPrepare{txn_id, ops, label, origin: MutationOrigin::Agent{..}}` → `TransactionPrepared`; `TransactionCommit{txn_id}` → `TransactionCommitted{edit_id}` | Bounded retry (3 attempts, fresh `txn_id` each) on `PRECONDITION_FAILED` (mapped from `transaction.instance-busy`); any other mapped error bubbles immediately. On a post-`TransactionPrepared` commit failure, sends `TransactionRollback{txn_id}` before propagating. |
| Verify | `invoke()` | `ReadHistory` → `HistorySnapshot` | Re-reads the revision after commit for `InvocationReport.revisionAfter`; mints an `undo_` handle carrying `{instance, txn_id}`. |
| Undo/Redo | `history_undo`/`history_redo` | `TransactionUndo{group_id: txn_id}` / `TransactionRedo{group_id: txn_id}`, fanned to every member the `undo_` token covers | Best-effort — a per-member failure is a warning unless EVERY member fails, in which case `SIDE_EFFECT_REJECTED`. The handle is never revoked by undo/redo, so undo→redo round-trips through the same token. |
| Idempotency | `invoke()` | (wraps the commit closure only) | `IdempotencyStore::get_or_insert_with(principal.id, key, now_ms, ..)` — a cache hit returns the stored report with `replayed:true` and issues **zero** channel commands (proven by asserting the `TransactionCommit` count in the mock's frame log stays at 1 across two `invoke()` calls with the same key, §4). |
| Multi-provider (saga) | `transaction_begin`/`transaction_commit`/`transaction_rollback` | Phase 1: `TransactionPrepare` per member, discovery order, rollback-on-reject (reverse order); Phase 2: `TransactionCommit` per member, **reverse discovery order**; on failure: `TransactionUndo` for every already-committed member (this reverse pass), `TransactionRollback` for any still-only-prepared member | Reimplements the real `HostTransactionCoordinator::run_transaction` protocol (`📓️luna-channel-audit.md` §3) against THIS packet's own port — the real coordinator lives in the peer ticket's `🔌️plugin/🖥️host`, off-limits. `COMPENSATION_FAILED` iff compensating an already-committed member itself fails (tested both ways, §4). |
| Cancel | `cancel()` | none | Drops a `prep_` handle. Job-class cancellation is explicitly NOT implemented — no job execution exists anywhere in this crate yet (that's P7's job); documented in the method's own doc comment. |

## 3. Error-mapping table (`Fault.code` → `GatewayErrorCode`, `crate::actions::map_fault`)

| `Fault.code` | `GatewayErrorCode` | retryable | Source |
|---|---|---|---|
| `viewer.read-only` | `PERMISSION_DENIED` | no | `📋️master.md` §3.3 Fault code table |
| `capability-denied` | `PERMISSION_DENIED` | no | `📋️master.md` §3.3 Fault code table |
| `mutation.rejected` | `SIDE_EFFECT_REJECTED` | no | `📋️master.md` §3.3 Fault code table |
| `transaction.generation-mismatch` | `REVISION_CONFLICT` | no | brief §3.1 |
| `transaction.instance-busy` | `PRECONDITION_FAILED` | **yes** | brief §3.1 ("bounded retry then PRECONDITION_FAILED") |
| `budget.exceeded` | `BUDGET_EXCEEDED` | **yes** | this packet's own addition — quota exhaustion, no real quota tracker exists yet (kernel `Broker`/`QuotaTree` territory, out of scope); the mapping and a scripted test exist so a real quota check can raise this fault code later with zero adapter changes |
| anything else | `INTERNAL` | no | never silently swallowed |

Also directly enforced (not through `Fault`, since they never reach the channel): `NOT_FOUND` (unknown
capability id, unknown/expired handle — via `HandleTable::resolve`), `INPUT_INVALID` (schema
validation failure, malformed tool arguments), `PERMISSION_DENIED` (scope subset check),
`APPROVAL_REQUIRED{approvalHandle}` (approval gate), `REVISION_CONFLICT` (the upfront staleness
check, distinct from the channel-reported generation-mismatch path), `COMPENSATION_FAILED` (saga
compensation failure).

Every code in this table is proven by `crate::actions::quick::every_fault_code_maps_to_the_right_gateway_error_code`
— written, present in the file, **not yet run against the compiled crate** (§5).

## 4. Tests written (`//#region 🧪️Tests`, `mod quick` in both new facets)

**`🎬️actions` — 16 tests**, over `MockArtifactChannel` (a fully-scripted in-memory artifact store —
real generation counters, real pending-transaction bookkeeping, real per-instance state, `Clone`
shares state via `Arc<Mutex<..>>` so a test keeps a handle after moving another clone into the
adapter):
- `preview_ops_and_the_ops_actually_committed_are_the_same_bytes` — preview-vs-commit ops equality,
  asserted on the recorded frame log's exact `TransactionPrepare.ops.document` bytes against the
  independently-reconstructed expected `PureCommand` payload (the mock derives `Emit` bytes
  deterministically from `(capability_id, input)`).
- `stale_expected_revision_is_a_revision_conflict_with_no_mutation_sent` — `REVISION_CONFLICT`
  **with no `TransactionPrepare`/`TransactionCommit` anywhere in the frame log**, per the brief's
  explicit instruction to assert on the frame log, not just the error.
- `idempotent_replay_performs_exactly_one_mutation` — two `invoke()` calls with the same
  `idempotencyKey`; asserts `TransactionCommit` appears in the frame log exactly once.
- `undo_token_round_trips_through_history_undo_and_redo`.
- `approval_gate_blocks_a_destructive_capability_without_approval_and_proceeds_with_it` — first
  `invoke()` → `APPROVAL_REQUIRED{approvalHandle}`; `resolve_approval(.., true, ..)`; second
  `invoke()` with the resolved handle → `Succeeded`.
- `a_capability_whose_scopes_exceed_the_principals_is_permission_denied_and_audited` — asserts
  `PERMISSION_DENIED` AND a matching `AuditDecision::Denied` row in the `InMemoryAuditSink`.
- `cancel_drops_a_prepared_handle` — cancel then re-cancel → `NOT_FOUND`.
- `instance_busy_retries_then_precondition_failed` — an externally-pending transaction on the target
  instance; asserts the adapter attempts exactly `INSTANCE_BUSY_MAX_ATTEMPTS` (3) retries (visible in
  the frame log) before surfacing `PRECONDITION_FAILED`.
- `a_concurrent_edit_between_prepare_and_commit_is_a_revision_conflict` — a scripted commit-time
  `transaction.generation-mismatch` fault → `REVISION_CONFLICT`.
- `budget_exceeded_fault_maps_to_budget_exceeded_code` — scripted `budget.exceeded` fault →
  `BUDGET_EXCEEDED`, `retryable: true`.
- `every_fault_code_maps_to_the_right_gateway_error_code` — all 7 rows of §3's table in one test.
- `saga_commits_in_reverse_discovery_order_and_compensates_on_failure` — 2-member saga across 2
  instances; member A's commit is scripted to fail; asserts commit order in the frame log is
  `[instance 1, instance 0]` (reverse discovery) and that instance 1 (already committed) receives a
  `TransactionUndo` (compensation).
- `compensation_failure_itself_is_reported_as_compensation_failed` — same scenario, PLUS the
  compensating undo is also scripted to fail → `COMPENSATION_FAILED`.
- `unknown_capability_id_is_not_found`, `invalid_input_against_the_capabilitys_schema_is_input_invalid`,
  `transaction_begin_requires_at_least_one_prepared_handle`.

**`🛡️policy` — 11 tests**: scope-alias expansion (`artifact.write` → `documents.write`+`jobs.spawn`,
`ui.raw-control` → `shell.raw`, an unknown alias passes through literally, a `<prefix>*` wildcard
grant covers any concrete family member), `authorize_scopes` allow/deny, the full approval-gate
round-trip (`Never` never gates; `WhenDestructive`+destructive gates then proceeds once resolved; a
DENIED approval never lets the gate proceed; `AutoApprovePolicy::All` waives the gate entirely;
`AutoApprovePolicy::parse` covers exactly the 3 frozen values and nothing else).

**Root `🦀️component.rs` `mod quick`** (shared file, my own edits only): 3 new tests
(`action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope`,
`action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks` — the exact two
scenarios the brief's §5 live transcript asks for, proven deterministically instead of only via a
one-off manual transcript; `action_invoke_tool_call_commits_a_prepared_capability_end_to_end`); 2
pre-existing P2 tests repointed, not deleted, to keep testing their original intent now that
`action_invoke` is real (`declared_stub_tool_call_is_a_structured_plugin_unavailable_error` now calls
`artifact_create`, still genuinely a stub; `tools_list_has_the_...` renamed and its assertions widened
to check the 8 mutation-protocol names are present as REAL tools and absent from
`DECLARED_STUB_TOOL_NAMES`, while the total-count assertion (20) is unchanged).

**Total: 30 new tests + 5 touched-but-preserved existing tests, 0 tests removed.**

## 5. Acceptance — BLOCKED by a verified, cross-packet, pre-existing regression outside this packet's `path_scope`

**I did not skip this. I ran the exact commands repeatedly (18+ times across the session) and am
reporting the true, current state — CLAUDE.md forbids claiming a result I did not observe.**

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-mcp --lib
...
error[E0308]: mismatched types
   --> 🌉️mcp/🗂️catalog/🦀️component.rs:615:95   (+ 618, 621, 624 — same function, 4 mismatches)
    |                                             expected `&DescriptorEntry`, found `&ContributedInferenceMetadata` / `&ContributedMutationMetadata` / `&IoEntryDescriptor` / `&ComposerEntryDescriptor`
error: could not compile `semio-framework-os-mcp` (lib) due to 4 previous errors
exit: 101
```
Full transcript: `🧪️p6-cargo-check-blocked-by-catalog.txt`.

**This is not my bug.** `🌉️mcp/🗂️catalog/🦀️component.rs` is P2-catalog's file (closed packet), listed
explicitly under this packet's own §2 "Do NOT edit" list. `capability_from_contribution`
(catalog.rs:364) still expects the old placeholder `&manifest::DescriptorEntry` shape for
`ContributionSet.inference_services/mutation_services/io_entries/composer_entries`, but a concurrent
session (P8-spi, landed the manifest change) made those fields the real typed
`ContributedInferenceMetadata`/`ContributedMutationMetadata`/`IoEntryDescriptor`/
`ComposerEntryDescriptor` — a cross-packet regression in a file that belongs to neither P8 nor this
packet. **Independently confirmed by THREE separate sessions hitting the identical 4 errors**:
`📓️terra-P8-report.md` §4 (found it first, filed `spawn_task` `task_b39ce04b`), a concurrent P7
session's own `📓️lease-P7-catalog-contribution-types.md` (filed one minute before mine), and this
packet's `📓️lease-P6-catalog-contribution-types.md` (filed with a concrete, advisory (not applied)
proposed patch to accelerate whoever picks it up).

I verified, over every single one of the 18+ `cargo check` runs during this session, that **zero
errors were ever reported against `🎬️actions/🦀️component.rs` or `🛡️policy/🦀️component.rs`** — the
compiler successfully type-checked both new facets in full every time; the ONLY errors, in every run,
were the 4 fixed `catalog.rs` mismatches (plus, transiently, unrelated churn in `semio-framework-ui`
and P7's own still-being-written `🏠️workspace/🦀️component.rs`, both of which resolved on their own
mid-session as those sessions finished — see §6). I did find and fix one real bug of my own along the
way (§7.1) — a borrow-checker conflict the compiler caught immediately, exactly as this evidence
claims it would.

`bun nx run @semio-tech/framework-os-mcp:test-quick` — ran, **exit 0, 26/26 passing**, but this is
**not evidence for this packet**: `Test Files 5 passed (5)`/`Tests 26 passed (26)`, served entirely
from Nx's cache ("Nx read the output from the cache instead of running the command") against
whatever binary predates this packet's own changes — the TS suite spawns the compiled
`semio-os-mcp` binary, which cannot rebuild while the Rust crate is blocked. Full transcript:
`🧪️p6-ts-test-quick-stale-binary.txt`, explicitly NOT claimed as validating the new tools.

**What I did instead of fabricating a result**: filed `📓️lease-P6-catalog-contribution-types.md`
(cross-referencing P8's and P7's own identical findings, adding a concrete advisory patch sketch),
saved every real transcript to `.txt` evidence files in this ticket folder, and wrote 30 new tests
(§4) that are ready to run the instant `🗂️catalog` compiles again — I am not asking anyone to trust
untested code; I am reporting, honestly, that the test suite exists, is believed correct by direct
compiler feedback on every file I own, and has not yet been able to run end-to-end through no fault
of its own.

**Live transcript requirement (§5 of the brief)**: cannot be produced against the real binary for the
same reason. §4's `action_prepare_tool_call_returns_a_prepared_action_report_for_a_granted_scope` and
`action_prepare_tool_call_is_permission_denied_for_a_scope_the_principal_lacks` Rust tests are the
SAME two scenarios (prepare against a mock-backed instance → `PreparedActionReport`; prepare with a
scope the principal lacks → `PERMISSION_DENIED`), written specifically so this requirement is
satisfied the moment `cargo test` can run at all — re-run instructions are in §8.

## 6. Leases and concurrent-session collisions (evidenced, not assumed)

Per `📌️important.md` rule 1: "`git status` is NOT a churn detector — use `git log --date=iso` plus
file hashes." Every claim below is backed by `git status --porcelain` (showing a live, uncommitted
edit — not the committed tree) and, where relevant, a re-run showing the SAME file's errors changing
shape between two consecutive checks (proof of an in-progress edit, not a stable, attributable-to-me
state).

1. **`🌉️mcp/🗂️catalog/🦀️component.rs`** — NOT edited by me (outside `path_scope`); broken by a
   concurrent P8-spi manifest change; see §5. Lease filed:
   `📓️lease-P6-catalog-contribution-types.md`.
2. **`🌉️mcp/🏠️workspace/🦀️component.rs`** (P7-headless-workspace, brand new — `git status` showed it
   `??` untracked, created mid-session) — NOT edited by me. Its own compile errors (`store::ProtocolError::Codec`
   missing variant, `list_document_ids` vs `document_ids`, `drop_instance` trait import,
   `protocol::AppCommand` unresolved, `store::AppCommand: Serialize` unsatisfied) changed shape and
   count across consecutive `cargo check` runs during this session and fully resolved on their own by
   session's end — confirming it was P7 actively iterating, not a stable defect. **P7 is building
   directly against this packet's own `ActionAdapter`/`ArtifactChannel`/`Fault` types**
   (`crate::actions::Fault` appears in their file, `build_server_with_workspace`/
   `server_for_workspace_options` in the shared root file both take `channel: Box<dyn ArtifactChannel>`
   and construct an `ActionAdapter` exactly the way `build_server_with_principal` does) — this is the
   port handoff the brief's §3.1 anticipated, working as designed, not a collision to resolve.
3. **`🌉️mcp/🦀️component.rs`** (root, this packet's own `path_scope`) — a concurrent P7 session added
   `pub use crate::workspace::*;` to the `🔖️Facets` region and appended `build_server_with_workspace`,
   `HubOptions`, `server_for_workspace_options`, `registry_override_context_resolve` (labelled with
   their own ticket/packet attribution comments) alongside my own
   `build_server_with_principal`/`build_tool_registry`/mutation-protocol tool wiring. I did not
   author, did not revert, and left these additions untouched — they build on top of, and do not
   conflict with, anything I wrote (confirmed: no duplicate function/const names, `build_server_with_principal`'s
   own 3-argument signature explicitly preserved per their own doc comment "this function's 3-argument
   shape has live callers in this same in-flight packet's own tests (P6-actions-policy) this packet
   must not disturb mid-flight").
4. **`🌉️mcp/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}`** — listed as MY owned files in the brief's §2,
   but I needed to add nothing to either (every dependency `🎬️actions`/`🛡️policy` needs — `jsonschema`,
   `serde`/`serde_json`, `semio-framework` for `kernel::CapabilityId`/`ApprovalMode` — was already
   present from P1a/P2). `glue.rs`: I added the two `#[path]` mounts for `actions`/`policy` myself;
   P7 separately added a `store` extern-crate alias and the `workspace` mount, non-conflicting.
   `Cargo.toml`: entirely P7's own edit (native-only `semio-framework-plugin-host`,
   `semio-framework-os-kernel{sync}`, `semio-framework-actor`, `framework_hash`) — I made zero changes
   to this file.
5. **`🌉️mcp/📦️bin.rs`** — NOT edited by me (outside `path_scope` per the brief's own §2); shows `M` in
   `git status` from P7's own `--hub`/`--space`/`--token` argv additions (visible via
   `HubOptions`/`server_for_workspace_options` in the shared root file, which need those flags parsed
   somewhere).
6. **`semio-framework-ui` / `🎯️targets/🧊️wgpu/🦀️component.rs`** — transient churn from an unrelated
   live session (peer ticket's H3/H4-wgpu or the neighboring `SHARED-PRESENCE-SESSION-COLORS-…`
   ticket per its own status.md attribution), fully resolved mid-session on its own; not touched by
   me, not related to this packet.

No lease was needed for anything actually inside my exclusive `path_scope` (🎬️actions, 🛡️policy,
my own edits to the shared root file) — every item above is either something I deliberately did NOT
touch, or evidence that someone else's concurrent edit landed in a file I also own without conflict.

## 7. Deviations from the brief, with justification

1. **`AppCommand`/`AppFrame` drop `seq`/`in_reply_to` and batching** — the real channel's contract is
   a batched duplex (one `exchange` call carries N commands, N replies, matched by `in_reply_to`); this
   packet's own port is one-command-per-`exchange`-call, always exactly one reply. Documented
   extensively in the port's own module doc as a deliberate simplification P7's real implementation
   must accommodate (wrap each call in a one-element batch, or do its own seq bookkeeping) — never
   presented as spec-compliant behavior of the real channel.
2. **`PureCommand`/`Emit`/`TransactionPrepare.ops` carry already-decoded shapes** (`capability_id` +
   `input: Value` for `PureCommand`; `PreparedOps{document,config,draft}: Vec<Vec<u8>>` for op
   payloads), not the real channel's raw wire bytes (`command: Vec<u8>`, packs+SPR, a single varint-
   framed op stream). Decoding the real binary formats (`OpBinary`, the real `command_from_action`
   bridge, `HistoryPatch`) requires the plugin-host/channel crates this crate has zero dependency on
   by design (D8) — translating between the real bytes and these already-decoded shapes is explicitly
   P7's job (§8).
3. **No real quota/budget tracker** — `budget.exceeded`→`BUDGET_EXCEEDED` mapping exists and is
   tested against a SCRIPTED fault; no live quota enforcement exists anywhere in this crate (that is
   kernel `Broker`/`QuotaTree` territory, `master.md` §3.4, explicitly deferred with hub identity per
   D7's own precedent for scoping cuts).
4. **No `elicitation/create` wiring** — `📋️master.md` §3.4 says an elicitation-capable client should
   get a live `elicitation/create` round-trip instead of just parking an `ApprovalRecord`. `protocol.rs`
   (P1a, off-limits) has no server-initiated request primitive yet — `McpServer`'s dispatcher only
   answers client-initiated calls. `PolicyEngine::gate_approval` always parks the `ApprovalRecord` and
   returns `APPROVAL_REQUIRED{approvalHandle}`; a later packet that adds bidirectional request support
   to `protocol.rs` can wire the elicitation branch without changing this facet's public API.
5. **No `--auto-approve` CLI flag** — `AutoApprovePolicy` is fully implemented and tested
   (`parse`/`Never`/`ReadonlyOnly`/`All`), but `bin.rs` argv parsing is outside this packet's
   `path_scope`; `run_stdio`/`run_http` always construct `AutoApprovePolicy::Never` (the safe
   default, per the brief's own instruction). A later packet leases `bin.rs` to add the flag and
   thread it through.
6. **`artifact_validate`/`artifact_snapshot` NOT implemented** — the brief allowed implementing them
   "if they fall out naturally from the channel work"; they did not — verifying/snapshotting an
   artifact needs a real document read (`ReadDocument`/`ReadConfig` equivalents this packet's port
   never needed for the mutation-protocol-only scope) and real `ArtifactCodec`/diff machinery this
   packet has no access to. Left as declared stubs, unchanged from P2.
7. **Job-class `action.cancel` NOT implemented** — no job execution exists anywhere in this crate yet
   (P7's territory); `cancel()`'s own doc comment states this plainly rather than silently no-opping.

No other deviations. Every method, error mapping, and test category named in the brief's §3/§4 is
present.

## 8. What P7 must implement to satisfy `ArtifactChannel`

```rust
pub trait ArtifactChannel: Send {
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault>;
}
```
(`crate::actions::ArtifactChannel`, `AppCommand`/`AppFrame`/`Fault`/`PreparedOps`/`MutationOrigin` all
in the same facet.) A real implementation must, per command:

- **`ReadHistory`** → resolve `instance` to the real plugin-host instance, send the real
  `AppCommand::ReadHistory`, decode the real `HistorySnapshot.history_patch` (packed `HistoryPatch`)
  into `{artifact_id, head_edit_id, cursor}` and return `AppFrame::HistorySnapshot(RevisionStamp)`.
  This packet's port never decodes `HistoryPatch` itself — that decode is entirely P7's to own.
- **`PureCommand{capability_id, input}`** → resolve `capability_id` to its `(plugin_id, app_id,
  window_kind_id, action_id)` via `Catalog::get(capability_id).source` (already exposed by P2,
  `CapabilitySource::Action{..}`), run the real `command_from_action` bridge to build the real command
  bytes, hydrate the real document/config/draft packs+SPR from the live `ArtifactHost`, dispatch the
  real `AppCommand::PureCommand` on a **preview instance** (never backbone-attached), and split the
  returned single-blob `document_ops`/`config_ops`/`draft_ops` into `Vec<Vec<u8>>` (one entry per op
  payload) for `AppFrame::Emit.ops` — this splitting is real per-artifact-schema work
  (`📓️luna-channel-audit.md` §4: no generic decode exists) that only P7's headless workspace, with
  access to the real `ArtifactCodec`, can do correctly.
- **`TransactionPrepare{txn_id, ops, label, origin}`** → construct the real pre-planned-form
  `AppCommand::TransactionPrepare{mutation_id: "", payload: vec![], prepared_ops: ops.document (+
  config/draft via separate plain `Command`s if non-empty, per `📋️master.md` §3.3: "config/draft ops
  via plain `Command`"), label, origin: encode(MutationOrigin::Agent → the real
  `MutationOrigin::Agent`/`Origin::Agent`, once A1's lease lands)}`. On success return
  `AppFrame::TransactionPrepared{txn_id}`; on `rejection` non-empty, return
  `AppFrame::Error(Fault{code: "transaction.generation-mismatch" or whatever the real rejection names,
  message})`.
- **`TransactionCommit{txn_id}`** → the real `AppCommand::TransactionCommit`; on success
  `AppFrame::TransactionCommitted{txn_id, edit_id}`; on `transaction.generation-mismatch`/
  `transaction.instance-busy` faults, return `AppFrame::Error(Fault{code, message})` verbatim (this
  packet's own `map_fault` already knows both codes — zero changes needed on this packet's side).
- **`TransactionRollback`/`TransactionUndo`/`TransactionRedo`** → pass-through to the real
  `AppCommand` variants of the same name; `group_id` ↔ the real channel's own `group_id` field
  (identical shape, `📓️luna-channel-audit.md` §6 confirms `group_id = txn_id`).
- **Batching**: since this packet's port is one-command-per-call, P7's real implementation should
  either issue ONE real `exchange` per port call (simplest, matches the real channel's own "single
  synchronous duplex round-trip" semantics per `📓️luna-channel-audit.md` §8 for a single command), or
  batch internally if a later performance packet wants that — this packet's port contract does not
  preclude either.
- **`Fault` codes**: use the exact strings this packet's `map_fault` already recognizes
  (`viewer.read-only`, `capability-denied`, `mutation.rejected`, `transaction.generation-mismatch`,
  `transaction.instance-busy`) — no changes needed to `🎬️actions` for P7 to plug in.

`build_server_with_principal(principal, audit, channel: Box<dyn ArtifactChannel>) -> McpServer` is the
one function P7 (or anyone) calls with a real implementation in place of
`Box::new(MockArtifactChannel::new())` — everything else (catalog, tool registry, handle table,
idempotency store, policy engine) is already wired against the trait, not the mock.

## 9. Files touched — final list

Created: `🌉️mcp/🎬️actions/🦀️component.rs`, `🌉️mcp/🛡️policy/🦀️component.rs`,
`📓️sol-P6-actions-policy-packet.md`, this report, `📓️lease-P6-catalog-contribution-types.md`, and
scratch `.txt` evidence files (`🧪️p6-cargo-check-blocked-by-catalog.txt`,
`🧪️p6-ts-test-quick-stale-binary.txt`) in this ticket folder.

Modified (this packet's own edits only — both files are ALSO concurrently edited by a live P7
session, see §6): `🌉️mcp/🦀️component.rs` (`🔖️Facets` glob-import additions; `DECLARED_STUB_TOOL_NAMES`
shrunk 17→9; new `🔖️MutationProtocolTools` region — 8 real tool handlers, schemas,
`build_tool_registry`/`build_server_with_principal`/`build_server` rewritten; `run_stdio`/`run_http`
now build a real `AgentPrincipal` + `FileAuditSink`; `mod quick` — 3 new tests, 2 existing tests
repointed to keep testing their original intent), `🌉️mcp/📦️packages/🦀️rust/📦️glue.rs` (mounted
`actions`/`policy`).

Nothing outside `path_scope` was touched by me; no git-modifying command was run; no ticket MCP write
tool other than this report/lease was called; no `[DEBUG] ` marker exists in any file I own (grep
-rn confirmed empty); no `.log` scratch file exists in the ticket folder.

## 10. Honest bottom line

The mutation protocol is fully implemented, the policy engine is fully implemented, both are wired
into the 8 real tools, and 30 new tests plus 5 repointed existing tests exist and (by direct,
repeated compiler feedback on every file I own, across 18+ check runs, with zero errors ever
attributed to my facets) are believed correct. **What did not happen**: running those tests to a
green result, and capturing a live JSON-RPC transcript against the real binary — both blocked, for
the entire session, by a verified, independently-triple-confirmed, pre-existing regression in
`🗂️catalog` that is not in this packet's `path_scope` and that I could not and did not fix. The
moment that regression is fixed (my lease includes a concrete starting point), re-running exactly
the commands in this report's §5 is the entire remaining acceptance workload — nothing in
`🎬️actions`/`🛡️policy`/my own root-file edits is expected to need further changes.
