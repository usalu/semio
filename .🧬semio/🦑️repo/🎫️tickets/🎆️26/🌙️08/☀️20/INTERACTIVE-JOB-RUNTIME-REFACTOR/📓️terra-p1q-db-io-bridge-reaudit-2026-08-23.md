# Terra Re-audit — P1q DB I/O Bridge Remediation — 2026-08-23

## Verdict

**REJECT — source-only P1q re-audit. Phase 1 remains RED.**

The live remediation closes the earlier Hub/CLI synchronous DB-bootstrap paths, and the retained-I/O invariants remain intact. One required adversarial verifier case is absent: the mutation named `hub-sqlite-pre-open` injects a direct `rusqlite::Connection::open`, rather than reinserting the prior `std::fs::create_dir_all(parent)` before `SqliteStorage::open`. The generic production predicate would reject that reinsertion, but the required mutation has not actually been exercised. A passing current self-test therefore cannot establish the specifically requested SQLite-parent regression guard.

## Current Live-Source Result

| Gate | Result | Independent evidence |
| --- | --- | --- |
| Hub DB setup | PASS | `hub` `connect_db` calls `Database::open_at(pool, &root, profile)` for FS and `SqliteStorage::open(pool.clone(), ...)` for SQLite; it contains no `std::fs`, `rusqlite`, or `create_dir_all`. `main` calls `connect_db(&data_dir)` directly before unrelated extension-directory setup. |
| CLI DB setup | PASS | Production CLI setup has no `std::fs`, `rusqlite`, or `create_dir_all` path before its pooled `FsStorage`/`Database` constructors. |
| Constructor ownership | PASS | `FsStorage::open` owns root creation inside `run_blocking_op(... Lane::Io ...)`; `SqliteStorage::open` owns parent creation, `Connection::open`, and schema setup in the same retained operation. |
| Submission census | PASS | Production storage census is exactly **30 FS + 30 SQLite = 60** retained submissions: 57 trait operations and 3 opens. |
| Authority and ownership | PASS, source-only | Strong `Arc<WorkerPool>` ownership, `DbIoPages` exact owner return, 16 KiB pages, 64 item slots, 1 MiB operation cap, 16 MiB aggregate cap, 4,096 list items, terminal take/resume/close, and injected testkit pool remain present. Scans found no optional pool, inline/open-inline, or `with_pool` fallback. |

The remaining Hub `create_dir_all` calls are outside DB setup: `connect_directory` owns the distinct identity-directory subsystem, and the later extension-module directory is after database construction. They are not the rejected P1q bypass.

## Required Fixture Gate

| Required regression | Current mutation | Result |
| --- | --- | --- |
| FS root precreate | `hub-fs-pre-open` reinserts `std::fs::create_dir_all(&root)` before `Database::open_at` | PASS |
| SQLite parent precreate | `hub-sqlite-pre-open` inserts `rusqlite::Connection::open(&path)`, not `std::fs::create_dir_all(parent)` | **FAIL** |
| Main data-dir precreate | `hub-main-pre-create` reinserts `std::fs::create_dir_all(&data_dir)` before `connect_db` | PASS |

The predicate at `📜️script.ts:4026-4028` would reject either `std::fs::create_dir_all(parent)` or `rusqlite` in `connect_db`; that is useful static coverage but is not a substitute for the specifically requested adversarial mutation. Repair by adding or replacing the SQLite mutation with an exact `std::fs::create_dir_all(parent)` insertion immediately before `SqliteStorage::open` and keeping the self-test rejection assertion. The direct-rusqlite mutation may remain as an additional case.

## Commands Run

```text
rustfmt --edition 2021 --check --config skip_children=true <scoped Hub and DB Rust files>
# exit 0

bun './📜️script.ts' verify interactivity --self-test
# exit 0; DENY mode clean, one declared test-only blocking-bridge allowlist finding

rg production scans for synchronous Hub/CLI setup, optional/inline pool paths, testkit pool creation, owner-taking write facades, retained submissions, limits, and terminal APIs
# live source results recorded above

git diff --check
git diff --cached --check
git diff HEAD --check
git diff{, --cached, HEAD} --check -- <P1q Hub/DB/script scope>
# all exit 0 with no output
```

No Cargo, Nx, Wasm, browser, network, or root-lint command was run.

## Concurrent-Diff Classification

The current working tree contains concurrent Trinity modifications at:

- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`

All three whole-tree diff checks are clean in this stable snapshot. Consequently there is no Trinity EOF drift to ignore and no exemption was used.

## Residuals Outside This Source Packet

Phase 1 remains RED pending build/runtime evidence and the known broader boundaries: indivisible retained `std::fs`/Rusqlite backend calls, whole-result and snapshot allocation paths, `into_vec`, `ArtifactHandle` blocking boundaries, and generated Compose/runtime work. This re-audit neither ran nor implies Cargo compilation, runtime DB I/O behavior, timing, or shutdown behavior.
