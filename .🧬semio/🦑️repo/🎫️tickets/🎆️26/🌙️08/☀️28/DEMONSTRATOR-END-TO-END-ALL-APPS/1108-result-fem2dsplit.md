# Lane fem2dsplit: fem-2d model over its source file budget

## Cause
`semio-s-artifact-fem-2d-model` Source closure (lib, cfg-independent) was 34 files (28 `.rs` plus 6 `include_str!` assets: sql, 2 grammar, 2 protocol, demo dsl) against the default 32-file / 256 KiB boundary; bytes were ~260 KB. Not caused by recent edits; the budget was already exhausted (never raised).

## Split (no limit raised)
- 4 test modules left the lib closure for `[[test]]` targets of the 2d model (public API only):
  - `🚪️io/🧪️tests/🔬️dialect-conformance/🦀️.rs` -> `dialect_conformance` (grammar dialect, protocol dialect, protocol bytes vs encoded pack; replaces the two `#[cfg(test)]` conformance mods in io text/binary snapshot).
  - `🧬️schema/💡️inferences/🧪️tests/🔬️default-laws/🦀️.rs` -> `inference_default_laws` (replaces the two `#[cfg(test)] mod tests` of inferences and bounds).
- Generic code shared 2d/3d: `write-pack.rs` was byte-identical in 2d and 3d modulo names. Now one macro `semio_s_fem_receiving::write_pack!(Snapshot,SnapshotValue,"envelope")` in `🧩️receiving/📨️write-pack/🦀️.rs` (no new dependency); each model's `snapshot/🦀️.rs` invokes it; both `write-pack.rs` files removed.
- Result: 2d lib closure 34 -> 29 files, ~260 KB -> ~255 KB.
- Other 2d/3d near-duplicates remain (native.rs 61 differing lines of ~700, receive-record, identity-index, record, projection); each needs a trait/macro abstraction and is left as future shared-crate work.

## Cargo.toml
`◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml`: two new `[[test]]` targets, no dependency change (LOCK NEEDED logged; Cargo.lock unaffected).

## Verified
- W-fem2dsplit-1 native `--tests` of the 2d model: rc0 (source admission passes; lib, lib test and all test targets compile).
- W-fem2dsplit-w2 wasm32-wasip2 (2d model + receiving, with the write_pack macro): rc0 (w1 was red only in upstream value-retirement arc-custody, since fixed by its owner).
- W-fem2dsplit-t2 native run (after the macro; t1 before it, same result): cold_reference_graph 5/5, dialect_conformance 3/3, inference_default_laws 2/2, original_fem_receiving: 2 ok, 2 FAILED (below).
- Matcher stall is gone: the receive now completes (`KeySettlement` follows the acknowledgement).

## Open: the two receive tests now fail on a heap receipt, not a stall
`original_job_receives_and_retires_fem_rows` and `..._typed_record_into_caller_slot` panic at `original-receiving-tests.rs:27` (`received`: `(heap.requested_bytes, heap.released_bytes) == (receipt.retained_capacity_bytes, receipt.released_bytes)`) with a constant delta of +225 requested / +69 released bytes in both tests. The matcher lane's temporary `println!("[DEBUG] ack ready ...")` at `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎛️drive.rs:33` formats (allocates) inside the observed heap region; the print output appears in both failures. Most probable cause; to confirm, rerun after the matcher lane removes the `[DEBUG]` line.

## Rerun
- `"$C/1105b-tools/chk.sh" W-fem2dsplit-<n> "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" wasm -p semio-s-artifact-fem-2d-model -p semio-s-fem-receiving`
- `"$C/1105b-tools/chk.sh" W-fem2dsplit-<n> "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" native --tests -p semio-s-artifact-fem-2d-model`
- `"$C/1105b-tools/runtime/tst.sh" W-fem2dsplit-<n> "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.toml" native -p semio-s-artifact-fem-2d-model --test original_fem_receiving`
- 3d (write-pack macro touched it): same with `🧊️3d/Cargo.toml` and `semio-s-artifact-fem-3d-model`.
- 3d model (uses the macro too): W-fem2dsplit-3d1 native `--tests` queued behind the fleet at write time, NOT verified. Only the `write_pack!` invocation line in the 3d `snapshot/🦀️.rs` changed there.

STATUS: FIXED-verified W-fem2dsplit-w2 (wasm) + W-fem2dsplit-1/t2 (native --tests, source budget passes); 2 receive tests still FAIL on a heap receipt delta (+225/+69 B), most likely the matcher lane's `[DEBUG]` println in drive.rs:33; 3d recheck W-fem2dsplit-3d1 pending
