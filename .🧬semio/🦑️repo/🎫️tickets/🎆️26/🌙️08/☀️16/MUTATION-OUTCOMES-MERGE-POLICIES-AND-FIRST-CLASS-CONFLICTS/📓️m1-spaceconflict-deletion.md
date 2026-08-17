# Lane M1 — finishing C10's deletion of `SpaceConflict` (report)

## Starting point

L1 left 23 `SpaceConflict` hits across 5 unleased files (its own lease, `🏪️store/🔄️sync/🦀️component.rs`,
was already migrated to `MutationMessage` and is the precedent this lane follows). This lane's lease
was every file still containing a reference.

## Sites migrated

### `🔨️modules/🏪️store/🦀️component.rs` (2 hits)
- Deleted the `pub struct SpaceConflict { kind, uri, message }` definition and its doc comment
  (region `🔖️Backbone`). Its store-level twins (`reconcile_with_last`,
  `materialize_document_snapshot_with_conflicts`, `ArtifactStore`'s own `snapshot_with_conflicts`)
  were already gone from an earlier lane — only the struct itself and one doc-comment name-drop
  survived here.
- Reworded the one doc comment that name-dropped `SpaceConflict` (line ~2094) to stop naming the
  deleted type.

### `🔨️modules/🔁️workflow/🦀️component.rs` (3 hits, live)
- `validate_workflow_parameter_config_binding(binding, parameter_type, config_spec) -> Result<(), SpaceConflict>`
  → `-> Result<(), protocol::MutationMessage>`. Both `Err(..)` sites rebuilt as struct-literal
  `MutationMessage { level, code, message, target: vec![uri], op_index: None }` (not the
  `MutationMessage::error(...)` builder — that builder is regex-gated to the frozen 7 `mutation.*`
  codes by `📜️script.ts`'s `policyMutationMessageCodeBreaches`, and `workflow/parameter-binding-invalid`
  is intentionally outside that set, same as `🏪️store/🔄️sync`'s `externalDivergence` precedent).
  **Severity: `Warning`** — this function is called only from `reconcile_workflow_snapshot`'s
  post-merge integrity pass, never from a `Mutation::diff` path, so the Fatal/Error "diff carries no
  change" laws don't bind it; the stale binding is always dropped regardless of this message's level,
  so `Warning` ("state changed to survive a conflict, worth surfacing") fit better than `Error`/`Fatal`
  ("rejected") or `Info` (routine side effect).

### `🖥️host/🦀️component.rs` (12 hits, live — the big cluster)
`reconcile_workflow_snapshot`'s whole pipeline, called by `OsWorkflowStore::snapshot_with_conflicts`:
- Return types `Vec<SpaceConflict>` → `Vec<protocol::MutationMessage>` on `reconcile_workflow_snapshot`,
  `drop_workflow_cycle_edges`, and `OsWorkflowStore::snapshot_with_conflicts`.
- 4 push sites converted to struct-literal `MutationMessage`, `uri` → `target: vec![uri]`, same code
  string kept as `FaultCode`: `workflow/edge-orphaned`, `workflow/edge-type-mismatch` (×2),
  `workflow/edge-cycle`. **Severity: `Warning` for all four**, same reasoning as
  `validate_workflow_parameter_config_binding` above — every rule in this pipeline is corrective
  (drop always happens), never a rejection, so none can honestly be `Error`/`Fatal`, and all are
  consequential enough (real data loss on the dropped edge/binding) to outrank `Info`.
- Test `concurrent_delete_and_wire_reconciles_without_a_dangling_edge`: strengthened the assertion
  from `conflict.kind == "workflow/edge-orphaned"` to also require `conflict.level == Warning` (new —
  the old struct had no level at all) and `conflict.target == vec!["edge-race"]` (previously
  unasserted).
- **Deleted as dead code**: `instance::validate_parameter_config_binding` (a second, parallel
  `Os*`-typed copy of the same type-check, doc-commented as "ported verbatim" from the same source as
  the live `workflow::` version) had zero callers repo-wide except its own 3 unit tests
  (`validates_matching_parameter_config_bindings`, `rejects_mismatched_parameter_config_bindings`,
  `rejects_parameter_config_binding_to_unknown_field` — all 3 deleted with it). Doc-comment
  cross-references to the deleted function (3 sites) repointed at the live
  `workflow::validate_workflow_parameter_config_binding`.

### `🔨️modules/🔌️plugin/🦀️component.rs` (5 hits)
- Dropped `SpaceConflict` from two `use store::{...}` import lists.
- **Deleted as dead code**: testkit helper `assert_graph_merge_preserves_referential_integrity`
  (`probe: impl Fn(&VcsArtifactApp<A>) -> (P, Vec<SpaceConflict>)`) had zero callers repo-wide — no
  app's test suite ever invoked it; the one real exercise of this scenario is host's own
  `concurrent_delete_and_wire_reconciles_without_a_dangling_edge`, written directly against
  `OsWorkflowStore` rather than through this generic helper.

### `📦️packages/🦀️rust/📦️glue.rs` (1 hit)
- Comment-only reference (`store::SpaceConflict`/etc. example) reworded to `store::ArtifactDsl`.

## Verify (real numbers, this session)

- `grep -rn "SpaceConflict" --include='*.rs' . | grep -v '.🦑️repo/' | wc -l` → **0**
  (`🧪️m1-spaceconflict-grep` — re-run inline, see transcript).
- `cargo check -p semio-framework-os-kernel -p semio-framework -p semio-framework-plugin -p semio-framework-plugin-host`
  → **0 errors** (`🧪️m1-cargo-check.txt`).
- `cargo test -p semio-framework-os-kernel --lib` → **987 passed, 0 failed** — matches baseline exactly
  (`🧪️m1-cargo-test-os-kernel.txt`).
- `cargo test -p semio-framework --lib` → **137 passed, 0 failed** — matches baseline exactly
  (`🧪️m1-cargo-test-framework.txt`).
- `bun ./📜️script.ts verify mutation-outcome-law` → **passed, 0 breaches**
  (`🧪️m1-verify-mutation-outcome-law.txt`).
- Bonus, beyond the mandated list (this lane's actual behavior change lives in
  `semio-framework-os`, feature `os-host-full`, not in the mandated crate list):
  `cargo check -p semio-framework-os --features os-host-full` → **0 errors**
  (`🧪️m1-cargo-check-os.txt`). `cargo test` on the same → 103 passed / **7 failed**
  (`🧪️m1-cargo-test-os.txt`), and `cargo test -p semio-framework-plugin --lib` → 214 passed / **6
  failed** (`🧪️m1-cargo-test-plugin.txt`). All 13 failures are pre-existing/concurrent-lane churn,
  not caused by this lane:
  - `git diff --stat` on the 5 files this lane touched shows far more uncommitted change than this
    lane made (e.g. 572 lines in `🏪️store/🦀️component.rs` vs. this lane's ~15-line struct deletion)
    — other lanes are live in these exact files right now, per CLAUDE.md's concurrent-dev model.
  - None of the 13 failing test names touch `SpaceConflict`/`MutationMessage`/anything this lane
    edited (VCS edit-reference validation, conflict-identity seeding, mesh/GLB export format,
    missing fixture directories, DSL parse errors).
  - The one host test this lane's own diff touches,
    `concurrent_delete_and_wire_reconciles_without_a_dangling_edge`, fails **before** reaching this
    lane's changed code — at `store_b.dispatch_apply(WorkflowMutation::ConnectPorts{..})`
    (`mutation.apply.missing-target`), a `ConnectPorts`-apply code path this lane never touched; `git
    diff` on `🔁️workflow/🦀️component.rs` independently shows an unrelated in-flight edit to that
    same cascade/inverse logic by another lane.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`

Ticket not closed (shared ticket — never closed by a lane).
