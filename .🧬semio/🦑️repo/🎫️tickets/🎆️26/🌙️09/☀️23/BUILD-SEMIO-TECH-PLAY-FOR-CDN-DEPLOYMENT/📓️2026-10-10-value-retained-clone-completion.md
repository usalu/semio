# 2026-10-10 Value Retained-Clone Completion (value-core executor)

Verification dirs: `.🧬semio/🦑️repo/⚡️cache/play-fleet/value-exec/{target,build}`; cargo wrapped in `🚦️cargo-slot.sh value-core`.
Package tests must use `--manifest-path <crate>/Cargo.toml` (from the root, `-p` drops a non-member crate's dev-deps).

## Root cause of the 133 errors
The pushed commit c44e964 spliced the NEW sealed-shared custody blocks next to the OLD ones. Direction taken: keep the new API, delete the old duplicates, implement what the new call sites and tests expect.

## Value changes (`🧰️framework/🔨️modules/🌱️value`)
- `🧬️retained-clone/🦀️.rs`: removed old `RetainedCloneBinding`, non-generic `RetainedCloneBorrowAuthority`, `FixtureBorrowAuthority`; dropped dead imports.
- `🧬️retained-clone/🔗️source/🦀️.rs`: rewrote `RetainedCloneSource<T, A = T>` on `SealedShared<A>` + sealed `RetainedCloneLeaseOwner`; `admit_owned`, `admit_borrowed` (callable as `RetainedCloneSource::<T>::admit_borrowed(authority, project, grant)`), `constructor_*`/`owned_constructor_*`/`borrowed_constructor_*` quotes (owner header + lease header + payload scaffold + alias copy priced before allocation), `borrow`/`try_borrow`/`project_owned(…, grant)`, `close_step` + `next_close_*`, `take_authority` + `next_take_*` + `RetainedCloneSourceTake`, `RetireOwned` facade cursor.
- `♻️retirement/🦀️.rs`: rebuilt `FixedArray` cursor, removed the superseded `[T;N]` leaf impl, `next_close_byte_demand = Some(0)` for FixedArray/PlainArray/Set/Map cursors.
- `📋️list/🦀️.rs`: `next_reserve_copy_byte_demand`, `next_capacity_copy_byte_demand`, `reserve_capacity_one_funded`.
- `🧬️retained-clone/🌱️dynamic/🦀️.rs`: first-turn binding quote + grant-based `bind`.
- `🛬️decode`: `NativeDecodeControl::{ledger_identity, callback_identity, retirement_recipient_identity, with_retirement_child}` (`🧬️child/🦀️.rs`), recipient `reserve/settle/with_reserved_owner`, `RetireOwned` for the recipient wired in.
- `🏷️type`: `ValueType` derives `RetainedClone`; `Number` is `RetireOwned`.

## Other crates touched
- io-sqlite-snapshot: `SqliteSnapshotLedger` (`control.ledger.*`), `with_retirement_owner`, `install_native_retirement_recipient`, `transfer::{SchemaValidationStorage,construct_database_into,validate_*_into}` wired.
- job: `pub mod retained_work`. async: `PublicationClaim/Handle/Permit` exported (module was unwired). tool-run: duplicate leaf `RetireOwned` impls and duplicate test global allocator removed. umbrella `semio-framework` + action-bus tests: duplicate global allocator and stale inherent method removed.
- pack-json canonical trees: tuples 2-4, u16 u32 u128 i8 i16 i128 isize f32 () Number DslValue ValueType BTreeMap/HashMap; replication `HistoryFoldIndex`; mesh-engine `MeshData` family; manifest `ProgramContributionEntry`/`TopicContribution`; graph manifest types (+ `RetainedClone` derives). Each proven against serde_json (tests named `canonical_native_*`, `graph_manifest_canonical_*`, `mesh_data_canonical_*`, `program_contribution_canonical_*`).

## Results (exact)
- `cargo check -p semio-framework-value --lib` native: exit 0, 0 errors; `--target wasm32-wasip2`: exit 0.
- `cargo check --manifest-path … --lib --tests` value: exit 0.
- value lib tests (per-test runner, 293 tests): last full per-test run **260 ok / 29 FAIL / 4 TIMEOUT**; afterwards `original_scalar_clone_requires_original_authority` and both `controlled_value_refusal_*` tests were fixed and verified passing individually (expected full run: 263 / 26 / 4, not re-run in full).
- Remaining value failures are stale-vs-newer-design assertions, not compile errors:
  - 18 `retirement::*`, `factory_retirement::*`, `types::*`, `original_result`, queue tests expect `copied_bytes == payload length` for `String`/`Vec<u8>` retirement, while the newer `Bytes` cursor test (commit 48e4756, "native metadata retirement invented payload copy") defines copy 0.
  - 4 `paged_list` tests loop forever: after `begin_close` the cursor quotes release 128 and then never progresses (derived scaffold close).
  - `source_custody_tests::…terminal_ticket…`, `owned_mutex`, `decoder/encoder_refused_partial_owner` (24 vs 40 B), `sqlite_snapshot_nested_native_stages` (progress completed 1 vs 3) need owner decisions.
- pack-json lib tests: 85 pass, 1 fail (`canonical_native_paged_tree_arbitrary_depth…`: source close depth quote 65-130 for the 80/257-deep tree vs fixture policy depth 64; the retirement of a deep tree through lease → payload → controlled layers grows 3 frames per level).
- tool-run lib tests: 42 ok / 2 FAIL (`original_tool_run_writer…` narrow-work child progresses under copy grant 0; `…pending_tick_and_writer…` unwrap None).
- Not done: exact release receipt for `FromValue::from_value_controlled` temporaries (needs an explicit `released_bytes` ledger in `NativeDecodeControl`); `neutral_owner` integration test (derive crate) still uses removed cfg(test) fixtures (`fixture_from_authority`, `RetainedCloneBorrowAuthority::new`).
