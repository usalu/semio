# 📓️ db-trait-flip — completion plan (W6 resume after TCC restore)

## Starting state
`cargo check -p semio-framework-os-kernel-db --lib` → **83 errors** (63 × E0277, 12 × E0308, 1 × E0425).
`db_storage`'s trait family was already flipped to `DbFuture<'a, T>`; **no caller was converted**
(`grep -c "async fn"` = 0 in every db component). The packet was interrupted between the two halves.

## Sync/async boundary (the decision this plan fixes)
The module doc of `db_storage` (`🔖️Async-first`) already states the contract: every call is
*driven to completion by its caller*; there is no detached-spawn path. So:

- **Pure-logic layers become `async fn`** — `db_snapshot`, `db_wal`, `db_index`, `db_compact`,
  `db_sync`, `db_cluster`. These own no threads. `async fn` on inherent methods is
  `wasm32`-clean and needs no boxing (unlike the trait family, which needs `DbFuture` for dyn).
- **Thread-owning layers keep their sync signatures and bridge once** — `db_artifact`
  (bodies run inside `ArtifactAuthority`'s actor thread), `db_engine` (per-submit bridge threads),
  `db_cli`, `🌎️hub`'s bin. These call `db_actor::block_on`.

This honours the handover's hard constraint: **do not convert db-actor threads, do not delete
`db_engine`'s per-submit bridge threads** — those belong to the pending runtime/db refactor.
Blocking moves *outward one level* (out of each backend body, into the one thread that already
owned the call); no thread's shape changes.

## `inline_fs_runtime` (E0425)
`db_engine::Database::open_at` references a fn that was never written. `db_cli` already carries a
private `CliRuntime` doing exactly the job. Per CLAUDE.md ("if code is repeated it MUST be close to
each other"), the single implementation moves next to the thing that needs it — `db_storage`, beside
`FsStorage` — and both `db_engine::open_at` and `db_cli::open_fs_storage` use it. `CliRuntime` is
deleted, not duplicated.

## Order (bottom-up)
1. `db_snapshot` → 2. `db_wal` → 3. `db_index` → 4. `db_compact` → 5. `db_sync` → 6. `db_cluster`
→ 7. `db_artifact` (bridge) → 8. `db` facade → 9. `db_engine` (bridge + runtime) → 10. `db_cli`
→ 11. `🌎️hub` bin.

`db_artifact` is not in the handover's 9-file list; it breaks only once `SnapshotManager` goes
async. That is expected cascade, not scope creep.

## Gate (rule 25 — earned this same wave)
Both `--lib` AND `--all-targets`, plus the test suite, plus a `wasm32-unknown-unknown` check
(memory: native cargo misses `cfg(target_arch = "wasm32")` code).
