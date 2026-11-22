# FEM plugin root Rust workspaces: status

Fallback note: `repo cargo --store=shared` hung >15 min (door.capacity cache freeing, then `cargo:build-entry` stuck on the bun runtime lease). I used native cargo with `--config build.rustc="rustc" --config build.rustc-wrapper=""`. Those runs used the C: build-dir `.cache/b/r/m/u/l/d/b` (the coordinator's G: settings arrived after my last run). No cargo process of mine is running; `C:\b\fem*` and `C:\b\fwk` target dirs deleted.

## Check (`check --all-targets`)
- Root workspace (app-surface, sqlite, receiving): PASS after fixes.
- `⚙️engine/📍node`: PASS.
- `⚙️engine/🧱️material`: PASS after fix.
- `🏭️bridge`: PASS.

## Test
- Root workspace: PASS (all test results ok, exit 0).
- `🏭️bridge`: PASS (0 tests).
- `⚙️engine/📍node`: still FAILING (2 tests, planar and volumetric `native_node_original_receiving_projection_and_every_cancelled_frontier`). Last run got past the stall but failed or timed out after more than 60 s; the log is `🗑️generated/plugin-root-rust-native-node-test.log`. The cause is not yet determined.
- `⚙️engine/🧱️material`: 14 of 14 failing before the harness fix, with the same stall. It has not been re-run since the fix.
- `🧪️tests` folder: no Rust crates wired into these workspaces.

## Fixes
1. `🧩️sqlite/🧮️projection/🧪️tests/🦀️.rs`: the work-loan test uses the new `OriginalWorkRecipient` and `OriginalWorkAuthority::with_loan` APIs (a single `FiniteWork` recipient). `actual_work` is now compared as `u128`.
2. `⚙️engine/🧱️material/📋️table/🧪️tests/🦀️.rs`: `value.map(|boxed|*boxed)` because the projection output is now `Box<FemMaterial>`.
3. Framework, minimal: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎟️natural-publication/🦀️.rs` now uses `reserved_birth.as_ref().is_none_or(OriginalConstructionAdmission::terminal_is_empty)`.
4. Framework root cause of the "receiver stalled: 1024 consecutive funded turns performed no work" panics:
   - Nested schema-metadata turns now need `grant.maximum_items >= NATIVE_SCHEMA_METADATA_ITEMS` (3). `drive.rs` returns zero-work idle otherwise.
   - The shared harness `🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs` still granted 1 item.
   - The harness now uses `grant_items(demand, NATIVE_SCHEMA_METADATA_ITEMS)` for Nested selections.
   - `NATIVE_SCHEMA_METADATA_ITEMS` is now re-exported from `🛬️decode/🧳️owned/🦀️.rs` (`native_decoding::owned`).
   - The framework's own `native_shared` test also failed with this stall. I have not re-run it after the fix.

All `[DEBUG]` edits were reverted (framework files restored from backups; no `DEBUG` remains in receiver-driver, derive receive, record source or sealed-shared-receiver).

## Open
- Find why the node test still fails after the harness fix (run `native_node...` with `--nocapture`). The material test needs a re-run, and likely the same for the 2d/3d artifact tests that include `receiver-driver.rs`.
- Disk: the shared C: build-dir fills the disk, and several runs hit os error 112. Re-run with `CARGO_TARGET_DIR=G:\t\root` and `CARGO_BUILD_BUILD_DIR=G:\t\rootb`.
