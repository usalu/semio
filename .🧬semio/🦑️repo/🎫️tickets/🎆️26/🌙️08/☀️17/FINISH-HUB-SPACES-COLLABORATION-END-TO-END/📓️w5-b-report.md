# Lane 5-B report — wire `🛢️db`'s sqlite/postgres/neo4j Cargo features (Amendment 2 fix)

## Summary

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` declared `sqlite = []`,
`postgres = []`, `neo4j = []` as empty features with no optional driver dependencies wired, so
enabling any of them compiled storage code (`🗄️storage/🐘️postgres`, `🗄️storage/🌐️neo4j`, the sqlite
backend) against `rusqlite`/`sqlx`/`neo4rs`, which were not dependencies at all — 76 errors under
`cargo check -p semio-hub --all-features`, pre-existing since 2026-08-12 (Amendment 2). Fixed by
wiring the three features to real optional dependencies, matching `🌎️hub/📦️packages/🦀️rust/Cargo.toml`'s
own working pattern for its directory backends. Two genuine, previously-uncompiled import bugs in
`🛢️db`'s own postgres/neo4j storage backends were found and fixed along the way. The
**hub's own** postgres/neo4j directory backends (`🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j}/🦀️component.rs`,
written by the `PRESERVE-SEEDED-DIALOG-CONTEXT-ARGUMENTS` predecessor ticket, lane 1-A) compiled
**clean on the first try** once `🛢️db` itself compiled under these features — no changes needed there.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` — `sqlite`, `postgres`,
  `neo4j` features now read:
  ```toml
  sqlite = ["dep:rusqlite"]
  postgres = ["dep:sqlx", "dep:tokio"]
  neo4j = ["dep:neo4rs", "dep:base64", "dep:tokio"]
  ```
  New deps (all `optional = true`, `rusqlite`/`sqlx`/`neo4rs`/`tokio` target-gated
  `cfg(not(target_arch = "wasm32"))`, matching `rusqlite`'s pre-existing placement so wasm builds of
  this crate are unaffected):
  - `rusqlite` — was already an unconditional (non-optional) target dependency; made `optional = true`
    and gated behind `sqlite`, matching the sqlite backend's own doc comment ("mirroring this crate's
    own `Cargo.toml` gating `rusqlite`"), which described intent the `Cargo.toml` never actually
    implemented.
  - `sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "tls-rustls",
    "postgres"] }` — the storage backend (`🗄️storage/🐘️postgres/🦀️component.rs`) uses the full `sqlx`
    facade crate (`sqlx::postgres::{PgPool, PgPoolOptions}`, `sqlx::Executor`, `sqlx::Postgres`), not
    the split `sqlx-core`/`sqlx-postgres` crates the hub's own directory backend uses one level up —
    pinned to the same major version (`0.8`) so the workspace resolves to the `0.8.6` already in
    `Cargo.lock` rather than a second `sqlx` major version.
  - `neo4rs = { version = "0.8" }` — same version the hub's `Cargo.toml` already pins.
  - `base64 = { version = "0.22", optional = true }` (not target-gated — pure Rust, needed by the
    neo4j storage backend for byte-property encoding, matches the `0.22` version used repo-wide).
  - `tokio = { workspace = true, features = ["rt-multi-thread"], optional = true }` — both
    `PostgresStorage` and `Neo4jStorage` own a dedicated background runtime
    (`tokio::runtime::Builder::new_multi_thread()`, `Handle::try_current()`,
    `task::block_in_place`) to bridge the crate's synchronous `db_storage` trait onto their
    async-only drivers; neither backend compiled before because `tokio` was not a dependency of this
    crate at all (the crate's separate, pre-existing `tokio = []` feature flag is unused dead
    scaffolding — no code in the crate references it; left untouched, out of scope). Using `dep:tokio`
    from the `postgres`/`neo4j` features avoids the name collision with that pre-existing empty
    `tokio` feature (legal under the edition-2021 feature resolver, but the two must be referenced
    explicitly rather than relying on the removed auto-implicit-feature behavior).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️component.rs` (real bug, never
  compiled before) — imports were:
  ```rust
  use crate::db_ids::{check_len, DbError, ArtifactId, DurabilityClass, EpochFence};
  use db_storage::{CatalogStorage, DbStorage, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
  ```
  `DurabilityClass`/`EpochFence` actually live in `crate::db_durability`, not `db_ids` (confirmed
  against the working sqlite backend's own imports); and `db_storage` needs the `crate::` prefix —
  a bare `use db_storage::...` from within a `#[path]`-glued sibling module does not resolve to the
  crate-root `pub mod db_storage` under 2018+ path rules. Fixed to:
  ```rust
  use crate::db_ids::{check_len, DbError, ArtifactId};
  use crate::db_durability::{DurabilityClass, EpochFence};
  use crate::db_storage::{CatalogStorage, DbStorage, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};
  ```
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs` — identical bug,
  identical fix (same two import lines).

No other files touched. `🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j}/🦀️component.rs` were read in full but
needed **no changes** — see below.

## Commands run + results (all logs in `$T`, this lane's prefix `🧪️5-b-`)

**`🛢️db` itself, each feature in isolation, then combined:**
- `cargo check -p semio-framework-os-kernel-db --features sqlite` → **0 errors** (`🧪️5-b-db-check-sqlite.txt`)
- `cargo check -p semio-framework-os-kernel-db --features postgres` → first run **6 errors** (the
  import bugs above; `🧪️5-b-db-check-postgres-1.txt`) → after the fix, **0 errors**
  (`🧪️5-b-db-check-postgres-2.txt`)
- `cargo check -p semio-framework-os-kernel-db --features neo4j` → **0 errors** on the first run after
  the same import fix was already applied (`🧪️5-b-db-check-neo4j-1.txt`)
- `cargo check -p semio-framework-os-kernel-db --all-features` → **0 errors**, `Finished` in 41.71s
  (`🧪️5-b-db-check-all-features.txt`)

**The hub, now that `🛢️db` compiles under these flags:**
- `cargo check -p semio-hub --all-features` → **0 errors** (`🧪️5-b-hub-check-all-features-1.txt`).
  This compiled `🌎️hub/📇️directory/🐘️postgres/🦀️component.rs` and `…/🌐️neo4j/🦀️component.rs` for the
  first time ever — **no changes were needed to either file.** Lane 1-A's report
  (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/PRESERVE-SEEDED-DIALOG-CONTEXT-ARGUMENTS/📓️w1-a-report.md`) flagged
  both as "written to parity but never compiled — not verified against the actual crate API surface",
  specifically calling out `sqlx_core::transaction::Transaction`/`Pool::begin()`/`.bind(&serde_json::Value)`
  against JSONB, and `neo4rs::Txn::handle()`'s exact 0.8 shape, as best-effort guesses. All of it
  compiled clean.
- `cargo test -p semio-hub --all-features` (default fail-fast) → stopped after the lib target:
  **11 passed, 3 failed** (`🧪️5-b-hub-test-all-features-1.txt`). Re-run with `--no-fail-fast` to see
  the whole picture (`🧪️5-b-hub-test-all-features-nofailfast.txt`):
  - `unittests 📦️glue.rs` (the lib, includes `directory::sqlite::tests::*`, `directory::tests::*`
    core decider-law tests, and the 3 new `directory::postgres::tests::*` integration tests):
    **11 passed, 3 failed**
  - `unittests 📦️bin.rs`: **18 passed, 0 failed**
  - two further empty targets (build-script/doc-tests): 0/0 each
  - **The 3 failures are exactly, and only, Docker being unavailable in this environment**:
    ```
    thread 'directory::postgres::tests::seed_creates_default_space_and_membership' panicked at
    🌎️hub/📦️packages/🦀️rust/../../📇️directory/🐘️postgres/🦀️component.rs:644:59:
    start postgres container: Client(Init(SocketNotFoundError("/var/run/docker.sock")))
    ```
    (same for `event_log_replay_matches_projections` and `user_space_membership_round_trip`).
    `docker info` was checked directly and confirmed unavailable in this environment
    (`🧪️5-b-hub-test-all-features-nofailfast.txt` and the interactive session both show this).
    **These 3 postgres integration tests are the only tests skipped/failed, and they are skipped for
    an environmental reason (no Docker daemon), not a code defect.** The neo4j backend has no
    integration tests at all (`🌎️hub/📇️directory/🌐️neo4j/🦀️component.rs`'s own test module is a
    documented no-op: "Neo4j has no in-memory test mode; integration tests run against a
    live/testcontainers instance per the verification plan (HP-3) — not exercised in unit-test CI").

**Absolute-rules regression baseline (default features = sqlite, unaffected by this fix):**
- `cargo test -p semio-hub --lib` → **11 passed, 0 failed** (`🧪️5-b-regression-hub-lib.txt`) — matches
  the required baseline exactly (the 3 postgres tests are `#[cfg(test)]`-gated inside the `postgres`
  module, invisible at default features).
- `cargo test -p semio-hub --bin os-hub` → **18 passed, 0 failed** (`🧪️5-b-regression-hub-bin.txt`) —
  matches the required baseline exactly.
- `cargo test -p semio-framework-os-kernel --lib` → **988 passed, 0 failed**
  (`🧪️5-b-regression-kernel-lib.txt`) — matches the required baseline exactly.
- **No regressions on any of the three absolute-rules numbers.**

**Nx target:**
- `bun nx run os-hub:test-quick` → runs `cargo nextest run -p semio-hub --profile quick --all-features`
  under the hood (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`'s `TestScript`, unchanged, outside my lease).
  **The target now runs to completion instead of failing to compile at all** (`🧪️5-b-nx-test-quick.txt`):
  `Starting 32 tests across 2 binaries` → `15/32 tests run: 12 passed, 3 failed, 0 skipped`, then
  nextest's own default fail-fast stops scheduling the remaining 17 once 3 failures land (`17/32 tests
  were not run due to test failure (run with --no-fail-fast to run all tests, or run with
  --max-fail)`) — this fail-fast-on-first-failures behavior is `cargo-nextest`'s own default and the
  hub's `TestScript` (outside my lease) does not override it; it is not something this lane's fix
  controls. The 3 failures are the identical 3 Docker-dependent postgres tests confirmed above (the
  `cargo test --no-fail-fast` run already proved the other 17 all pass — `📜️script.ts`'s own comment
  even anticipates exactly this: "postgres's own tests still need a live Docker daemon regardless of
  this flag — pre-existing, not a regression from the merge"). The nx process therefore exits
  non-zero, and I am reporting that plainly rather than claiming a clean pass: **the target is
  mechanically alive again (it was previously unusable outright, per Amendment 2, because
  `--all-features` didn't compile); its only failures are the 3 Docker-gated postgres integration
  tests, confirmed via the separate `--no-fail-fast` run to be the only failures in the entire
  32-test suite.**

## What is now compiler-verified that was not before

- `semio-framework-os-kernel-db` under `--features sqlite`, `--features postgres`, `--features neo4j`,
  and `--all-features` — all four combinations, 0 errors. Previously `postgres`/`neo4j` did not even
  parse as valid feature-gated builds (the drivers weren't dependencies).
- `semio-hub --all-features`, including `🌎️hub/📇️directory/🐘️postgres/🦀️component.rs` and
  `…/🌐️neo4j/🦀️component.rs` — 0 errors, no changes needed to either file. These two files (the
  postgres and neo4j **directory** backends written by the predecessor ticket's lane 1-A) had never
  been compiled by anything before this lane.
- `cargo test -p semio-hub --all-features`: the 3 real postgres directory-backend integration tests
  (testcontainers-based) now build and run (they previously could not compile at all); they fail only
  for the environmental reason documented above.
- `bun nx run os-hub:test-quick` / the hub's `cargo-nextest` profile: unblocked from total unusability
  (Amendment 2 explicitly said "never `bun nx run os-hub:test*`") to running its full 32-test suite,
  32/32 of which are now buildable and 29/32 of which are runnable/passing in this Docker-less
  environment (the 3 postgres tests need Docker; the neo4j backend has no integration tests to run at
  all, only the compiler now sees it, which it did not before).

## Docker

`docker info` was checked directly in this environment and is unavailable
(`docker.sock` does not exist — the same `SocketNotFoundError` the tests themselves hit). The 3
postgres integration tests (`directory::postgres::tests::{seed_creates_default_space_and_membership,
event_log_replay_matches_projections, user_space_membership_round_trip}`) could not be run to a real
pass/fail against a live database in this environment; they are reported as failed (per the actual
test run), not silently skipped, and the failure is unambiguously the missing Docker daemon, not a
code defect — the same panic message and location (`🐘️postgres/🦀️component.rs:644:59`, inside
`test_directory()`'s `Postgres::default().start().await.expect(...)`) for all three.

## What is NOT done

- Postgres directory-backend tests were never run against a real Postgres in this environment (no
  Docker). They are compiler-verified and structurally sound (schema bootstrap, event-log
  append/project, `sqlx-core`/`sqlx-postgres` query shapes) but their actual runtime correctness
  against a live Postgres is unconfirmed here.
- I did not modify `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (outside my lease) to add `--no-fail-fast` to
  the nextest invocation, which would let `test-quick` report a clean "29 passed / 3 Docker-skipped"
  instead of exiting non-zero after nextest's own fail-fast cutoff. That is a test-runner-configuration
  question for whoever owns `🌎️hub/📦️packages/🦀️rust/**`, not a `🛢️db` defect.
- The crate's pre-existing, separate `tokio = []` feature (in `🛢️db`'s `Cargo.toml`, distinct from the
  new `dep:tokio` optional dependency this lane added) remains dead/unreferenced by any code in the
  crate — left untouched as out of scope for this lane's brief (sqlite/postgres/neo4j only).

## sharedFileRequests

None — everything needed was inside my lease (`🛢️db/**`, and read-only inspection confirmed
`🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j}/🦀️component.rs` needed no edits).

## Debug logging

None added; none to remove.
