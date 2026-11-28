# Curation workspace, last 3 failures fixed (2026-11-27)

Result: full workspace `cargo test --workspace --no-fail-fast -- --test-threads=1`: 44 test binaries, **226 passed, 0 failed, 0 ignored** (was 223/3).

Environment: native cargo. The root `.cargo/config.toml` now points `build.rustc` at the launcher ingress script and `rustc-wrapper=repo`, which refuses plain cargo. Workaround used: `CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER=` plus own `CARGO_TARGET_DIR` and, essential, own `CARGO_BUILD_BUILD_DIR` (the shared default build dir `.cache/b/r/m/u/l/d/b` is raced by peers: `failed to load metadata ... invoked.timestamp`), `bun` on PATH, `RUST_MIN_STACK` as in config.

## 1. `sqlite_snapshot_curation_cancel_reaches_known_stock_and_ordinal_frontiers`
Root cause (framework): `decode_framed` in `🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs` ran every constructor inside `ReceivingSettlement::with_control`, which lends the SQL control and coalesces all native events raised meanwhile into first and last (allocation-free replay introduced when the event Vec was removed). The native constructors (`decode_sqlite_snapshot_record_native` and `..._prefixed_record_native`) never use the lent control, so the 256-unit stage frontiers of `native.step()` (2048 stock rows) were swallowed; only `(0,2048)` and the last event reached the caller, so cancellation at `completed>=256` never fired (measured with temporary prints).
Fix: `decode_framed` now hands the constructor the settlement itself; only the public `decode_sqlite_snapshot_record_receiving` (constructor takes the control) lends it via `settlement.with_control`. The two native wrappers call `decode_framed` directly and forward every event live.

## 2. and 3. `history_edits_end_to_end`, `conflict_history_edits_end_to_end`
Symptom `operation output payload extent exhausted`: the canonical replacement comparison (`🧵️operation-wire/🧳️replacement/⚖️canonical`) quotes the output extent as the persisted wire length (28 bytes) and re-encodes the typed mutation. Temporary prints showed the encoder produced canonical JSON `op{"mutation":"createCuratedItem"...` while the persisted wire is the ordinal Pack binary (`1,0,...`).
Root cause (sourcing): `SourcingMutation` declared `dsl::artifact_persisted_binary_operation!`, whose installer is `with_original_binary_receiving()` (JSON re-encoding source). `persisted_document_store_owners` in the artifact-app trait overwrites `owners.operation_wire` with that installer, discarding the ordinal wire that `admit_sourcing_curation_store_owners` set. Replacement therefore compared binary bytes against a JSON re-encoding.
Fix: `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs` now has an explicit `store::os_store::ArtifactPersistedOperation` impl installing `with_original_ordinal_binary_receiving()` (same pattern as `DrawingMutation`).

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs`

All temporary `[DEBUG]` prints (including a peer's, which I removed from operation-wire output/canonical as the cause was found) are gone. Target dirs and `cur3-*` logs deleted. Wasm not built (hub rebuild picks both changes up).
