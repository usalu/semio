# DB Compile Gate Repair

## Scope

Restored the `semio-framework-os-kernel-db` production compilation gate that blocked the hub security/E2E lane. Changes are confined to DB-owned sources under `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`. The hub and shared OS store blockers discovered downstream were attributed but not modified in this lane.

## Initial reproduction

The isolated downstream reproduction used a private target below this ticket:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/target" \
SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/nextest" \
bun nx run os-hub:test-quick --skip-nx-cache
```

It stopped in `semio-framework-os-kernel-db` with 263 compiler errors. The narrow repair loop was:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/target" \
bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db --all-features --message-format short
```

The production error-count sequence was `263 → 99 → 59 → 3 → 0`; the last deny-lint pass also repaired six `dangerous_implicit_autorefs` sites.

## Root-cause groups

The 263 diagnostics were compiler cascades from seven underlying groups:

1. **Incomplete retained DB I/O migration.** Async-native backends referenced the missing `DbIoAsyncDriverFuture`; production laws referenced a test-gated `db_io_test_pool`; callers still treated retained `DbIoPages`, `DbIoU64List`, and query streams as plain vectors/slices.
2. **Schema/value contract drift.** The VCS hash projection, diff, and mutation lacked the canonical `ToValue`/`FromValue` contracts and mutation descriptor required by `ArtifactStore`.
3. **Backend adapter mismatches.** PostgreSQL/Neo4j driver trait-object coercions and Neo4j byte buffer ownership no longer matched the DB I/O result taxonomy.
4. **Async Send/dynamic dispatch mismatch.** `AuthzHook` and `VersionGraph` futures did not expose the worker-safe Send contract required by DB actors; enum arms also disagreed on the erased future type.
5. **Borrow/lifetime defects.** Temporary facet borrows produced E0716, simultaneous mutable/immutable borrows produced E0502, and raw-pointer page access triggered the deny lint.
6. **Terminal ownership/API drift.** Retained WAL, history replay, snapshot, query, and index owners did not consistently drain/close or preserve exact returned owner types.
7. **Stale cfg(test) fixtures.** Tests constructed byte slices where retained pages are required, compared typed hashes to arrays, matched owned timeout strings as literals, expected old sync welcome types, and omitted worker-poll instrumentation.

## Changes

- Completed the retained DB I/O type surface: public async driver future alias, test-pool availability for production laws, typed page/list equality and iteration needed at boundaries, and deterministic `Debug` support for retained fault owners.
- Reconciled memory/filesystem/PostgreSQL/Neo4j implementations with the current typed-result ownership model, including exact `Bytes`/`Vec<u8>` conversion and trait-object coercion.
- Reworked WAL, snapshot, index, compact, sync, query, artifact, and state call sites to pass retained owners, avoid temporary facet borrows, and drain terminal owners explicitly.
- Added canonical value derives and a mutation descriptor for the version-graph hash schema; removed its obsolete serde requirement after protocol actor/timestamp values stopped being serde wire types.
- Unified `VersionGraph` on a boxed Send future and bridged the non-Sync inner OS store future at the worker boundary.
- Made authorization futures explicitly Send and repaired history replay result ownership.
- Added focused test-fixture support for retained pages, worker-lane polling, typed content hashes, owned timeout strings, and sync welcome acknowledgements.
- Updated DB test fixtures for the current snapshot/WAL/query ownership APIs without relaxing assertions or lints.

## Verification

### Production DB gate

Final current-tree command:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/target" \
bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db --all-features --message-format short
```

Result: exit `0`; `semio-framework-os-kernel-db` finished the dev profile with zero errors. Existing warnings remain and were not relaxed.

### DB test discovery compile

Command:

```sh
CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/target" \
SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/repair-db-compile-gate/nextest" \
bun nx run '@semio-tech/framework-os-kernel:test' --skip-nx-cache -- \
  -p semio-framework-os-kernel-db --all-features --message-format short -- --list
```

The DB lib-test compile reduced from 80 errors (`48 E0277`, `13 E0308`, `4 E0609`, `4 E0369`, `2 E0615`, `2 E0061`, and one each of `E0689`, `E0618`, `E0282`, `E0046`) to 9 (`8 E0277`, `1 E0282`). Those final nine DB diagnostics were repaired. The final discovery attempt then stopped before compiling the DB test binary because the shared OS kernel dependency currently has two non-DB E0277 errors at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19828` and `:19833`: `InteractionState` does not implement the serde `Serialize`/`DeserializeOwned` contracts expected there. Consequently an exact nonzero DB test count is not available from the current tree.

### Hub downstream gate

The first post-repair `os-hub:test-quick` reached hub compilation and exposed three hub-owned E0433 errors at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1104`: `DirectoryCommandResponse`'s derive expanded through a canonical crate name while hub imported the package only as `directory`. The hub owner repaired that site.

The current rerun now stops earlier at the same two shared OS store `InteractionState` serde errors above. DB diagnostics do not recur, but the hub share/directory filters cannot compile or execute until that shared-framework blocker is repaired.

### Diff integrity

```sh
git diff --check -- '🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db' \
  '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-db-compile-gate.md'
```

Result: no whitespace errors.

## Residual blockers

- Shared OS store: two `InteractionState` serde-bound errors at `🏪️store/🦀️.rs:19828` and `:19833` block DB test discovery and hub compilation.
- Hub focused share/directory tests were not executed because compilation stops at that upstream blocker.
- Warnings remain in DB and dependencies; this lane did not relax or suppress them.
