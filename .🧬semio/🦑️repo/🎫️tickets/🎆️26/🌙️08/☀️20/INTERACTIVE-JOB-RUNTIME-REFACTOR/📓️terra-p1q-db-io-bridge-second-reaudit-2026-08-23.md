# Terra Second Re-audit — P1q DB I/O Bridge Verifier Repair — 2026-08-23

## Verdict

**ACCEPT — source-only P1q remediation. Phase 1 overall remains RED.**

The sole finding in the preceding Terra re-audit is repaired. This acceptance covers the narrow live-source and verifier packet only; it does not claim compilation, real backend I/O, timing, cancellation-under-stall, generated Compose reconciliation, or Phase 1 acceptance.

## Repaired Regression Guard

`📜️script.ts:4044-4064` now creates `hubSqliteParentPreOpen` by inserting, immediately before `SqliteStorage::open(pool.clone(), &path)`, the required former setup shape:

```rust
if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
```

The `hub-sqlite-pre-open` mutation uses that exact source. The separate `hub-sqlite-direct-open` mutation preserves the direct-Rusqlite regression case. The existing `hub-fs-pre-open` and `hub-main-pre-create` mutations still respectively reinsert the FS-root and main-data-dir mkdirs.

Crucially, the self-test does more than reject the mutated string. At `📜️script.ts:4063` it requires the resulting failures to include exactly:

```text
Hub DB setup performs synchronous filesystem/SQLite work before the retained pooled constructor authority
```

That proves the SQLite-parent mutation is rejected by the intended Hub pre-open rule, rather than parser behavior or an incidental rule.

## Independent Source Result

| Gate | Result | Evidence |
| --- | --- | --- |
| Live Hub path | PASS | `connect_db` sends FS construction directly to `Database::open_at(pool, &root, profile)` and SQLite construction directly to `SqliteStorage::open(pool.clone(), ...)`; its slice contains no `std::fs`, `rusqlite`, or `create_dir_all`. `main` reaches `connect_db(&data_dir)` without a precreate. |
| Live CLI path | PASS | Production CLI setup has no synchronous FS/Rusqlite pre-constructor path. Its sole `create_dir_all` hit is in tests. |
| Pooled constructor authority | PASS | `FsStorage::open` owns root bootstrap inside `run_blocking_op` on `Lane::Io`; `SqliteStorage::open` owns the parent mkdir, SQLite open, and schema setup inside its retained operation. |
| Retained submission census | PASS | **30 FS + 30 SQLite = 60** live submissions, matching 57 trait operations plus 3 open/schema authorities. |
| Earlier ownership packet | PASS, source-only | Strong pool ownership, `DbIoPages` exact-owner handback, 16 KiB/64-item/1 MiB/16 MiB/4,096 limits, generation freshness, terminal take/resume/close, and injected testkit authority remain present. Scans found no optional pool, `None => work/job`, `open_inline`, `.with_pool`, testkit subsystem `WorkerPool::new`, or slice/`Vec` write-facade signature. |

The remaining Hub `create_dir_all` occurrences belong to the separate identity-directory path and later extension-module directory setup. They are outside the main-to-DB-constructor slice and do not constitute a P1q storage setup bypass.

## Checks Run

```text
rustfmt --edition 2021 --check --config skip_children=true <scoped Hub and DB Rust files>
# exit 0

bun './📜️script.ts' verify interactivity --self-test
# exit 0; DENY mode clean. One existing, declared test-only blocking-bridge allowlist finding.

rg production scans for Hub/CLI pre-constructor work, owner-taking writes, optional/inline pools, testkit pool construction, retained submissions, and limits
# source results recorded above

git diff --check
git diff --cached --check
git diff HEAD --check
git diff{, --cached, HEAD} --check -- <P1q Hub/DB/script scope>
# all exit 0 with no output
```

No Cargo, Nx, Wasm, browser, network, or root-lint command was run.

## Residuals and Boundary

Phase 1 remains RED. Retained backend `std::fs`/Rusqlite operations remain indivisible, whole-result/snapshot materialization and `into_vec` residuals remain, `ArtifactHandle` retains its separate blocking boundary, and generated Compose/runtime validation is still open. None was widened or accepted by this focused verifier repair.
