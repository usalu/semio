# DAG Retirement Integration

## Failure and boundary

The extracted `semio-framework-artifact-infinite-dag` package initially passed 50 of 55 tests. Four failures were store mutation/history tests with `edit history insertion requires its exact mutation retirement factory`; the fifth codec failure was independently repaired by the coordinator.

`DagStore::new` deliberately installs no default owner catalog. The extracted artifact therefore needs to own the factories for its exact `DagSnapshot` and `DagMutation` types. Host code must construct stores through that artifact boundary rather than duplicating registration.

## Implementation

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs` now implements:

- resumable retirement for DAG strings, DSL values, property trees, ports, node kinds, nodes, edges, snapshots, and every one of the 14 mutation variants;
- exact snapshot, initial-owned-snapshot, and mutation retirement factories;
- `MemberStoreOwner<DagMutation>` for `DagSnapshot` with the typed cursor disposer;
- terminal-empty assertions so retained ownership cannot be silently dropped.

Each `close_step` honors zero item/byte grants with `Blocked`, advances one owned frontier item per nonzero grant, truncates string bytes within the byte grant, and preserves every remaining owner in the explicit frontier. Snapshot `Arc` ownership is detached before nested payload retirement.

The artifact exposes `create_dag_store(id, snapshot)`. It creates the envelope and immediately installs `DagSnapshot::member_store_owners()`. All four package test stores and the browser `DagSnapshotVcs` constructor now use this one factory.

The focused retirement regression reads the language-neutral mutation fixture with both the first-party pack parser and `serde_json`, converts the complete first-party parse back through its JSON writer, and requires structural equality with the complete serde oracle. It retires a decoded mutation with one-item/seven-byte grants, verifies zero item and zero byte grants block without advancing, and reaches the terminal-empty witness. It then covers both `Arc<DagSnapshot>` ownership paths: a two-owner `Arc` detaches and leaves the observer as the sole intact owner, while a verified final owner hands its snapshot into the resumable nested frontier before terminal close.

## Changed files

- created `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`;
- updated the DAG artifact root to mount the retained implementation;
- updated `🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs` with the store factory and four test constructors;
- updated the Infinite directed-DAG browser host constructor to call the artifact factory.

## Validation

Passing before the focused runtime gate:

```text
cargo metadata --offline --no-deps --format-version 1
rustfmt --edition 2021 🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs
```

The focused command uses the shared ticket target:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-framework-artifact-infinite-dag --lib --message-format short
```

Result: 55 passed, 0 failed. This includes mutation application, undo/redo, document text/pack round trips, command envelope round trips, all direct mutation leaf contracts, the language-neutral package oracle, and terminal-empty bounded store teardown.

The new single retirement-frontier regression is run separately so its grant and ownership assertions remain explicit:

```text
CARGO_TARGET_DIR=<ticket>/🗑️generated/cargo cargo test -p semio-framework-artifact-infinite-dag --lib retained::tests::neutral_fixture_retires_exact_mutation_shared_snapshot_and_final_snapshot_owners --message-format short
```

Result: 1 passed, 0 failed, with 55 unrelated tests filtered out. The full first-party/serde fixture equivalence and all bounded grant/ownership/terminal assertions passed.

## DAG casing audit

`DagNodeKind` intentionally does not use an enum-wide `rename_all_fields`. The authoritative mutation fixture `🌿️vcs/🧫️fixtures/🔣️mutations.json` declares `variadic_inputs` and `variadic_outputs`, including hostile camel-case and wrong-type vectors; the source preserves those names with explicit field renames. `AppInstance` separately declares its established `instanceId`, `pluginId`, `appId`, and `appIcon` fields. Existing direct intrinsic fixture tests parse these vectors with the first-party codec and independently with `serde_json`, so no production or fixture change was required.
