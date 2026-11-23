# Stale Port: io schema, sqlite-snapshot, os io, stdio txt/raw/ifc

Prefix `🧰️framework/🔨️modules/` for framework crates.

| Crate | State |
| --- | --- |
| `🚪️io/🧬️schema` | compiles; 3 pass, 2 fail (open, see below) |
| `🚪️io/🪶️sqlite-snapshot` | compiles; 5 pass, 3 fail (open) |
| `🛍️products/💻️os/🔨️modules/🚪️io` | compiles; all pass (14 lib + serialization/format/routing targets) |
| `stdio/💾️binary/🏅️standards/🔖️raw` (io-registry crate) | compiles; 6 pass |
| `stdio/🔤️txt` | partially ported; lib test still does not compile (open) |
| `stdio/🏗️ifc/.../🔖️4x3/.../✳️any` | not io-stale: 121 unrelated errors from concurrent edits (editor/templates, private `app_testing`, missing `RetainedCloneGrant` imports) |

## io/schema (`⚠️refusal/🧪️tests/🦀️.rs`)
- Before: hand-driven `NativeDecodeControl` with removed `NativeDecodeWorkRecipient`, `NativeConstructionRecipientIssuer`, `testing_admission::capture`.
- After: `receive_io` mounts the shared `receiver-driver.rs` (`admit`, `receive_traced`, `close`, `close_source`, `retire`); the control ceiling is the bank wallet capacity (probe bank measures the admitted source), `accept` is the `Trace.admit` flag. Decode refusals are matched by `kind: <Pascal>` in the stringified failure. Dev-dep `semio-framework-value-testing-allocator` added.
- Open: both receive tests hit a driver accounting assertion: a nested turn (copy 7224, depth 2, capacity 0) allocates 2544 B with a zero receipt (heap `(2544,0)` vs progress `(0,0)`), i.e. an unreceipted allocation in the `NativeSnapshotFailure` nested receiver (`io/⏱️failure-owner`); the `maximumBytes 0` decode row cannot be expressed through the bank (wallet capacity 0 stalls the grant).

## io/sqlite-snapshot (`🧩️artifact/{📦️blob,🧵️paged-text}/🧪️.rs`)
- Before: `NativeEncode{Work,Construction}Recipient`, `…Issuer::loan`, `NativeEncodeAllocationIssuer`, `NativeDecode*Recipient`, `owner.step(grant)`.
- After: shared `🧩️artifact/🧪️support.rs` (`Port` allocation authority port, `Work`, generic `Recipient<F>` with `Context`/`quote.complete`, `close` via `EffectBank` + `with_original_invocation` + `take_admitted`); tests drive `NativeAllocationAuthority::with_send_loan` + `OriginalConstructionAuthority::with_loan` + `OriginalWorkAuthority::with_loan`; work comparisons now `u128`; `state.expected` replaced by `state.state.actual`. Dev-dep testing-admission added.
- Open: `blob::reconstructed_blob_keeps_prefix...` case `complete` reports stage work 1 instead of fixture 4; `paged_text_preserves_original_prefix...` and `paged_text_prework_denial...` stop at "original owner requires recovery before retirement" when closing the allocation state after a refused receipt (the effect state is not ready for retirement; needs the settlement/recovery path).

## os/io (`🧭️io-mechanism` laws, `📤️serialization/📦️owned/🧪️tests`)
- Before: `NativeDecodeWorkRecipient::install_work_recipient(&mut loan)`, `close_retirement_recipient(grant)`, `witness.retirement()`, `admit_shared_retirement`, `artifact_retirement_box_*`.
- After: `with_control` builds `OriginalWorkAuthority` + two `OriginalConstructionAuthority` loans inside `with_loan` closures, installs them via `install_*_recipient`, closes the decode recipient (failure birth through `with_retirement(..).close_original_birth_step`, then `close_retirement_recipient(body, effect)`) and the encode recipient (`close_retirement_recipient_into`) through an `EffectBank`; serializer test returns the lease via `try_admit_one_returned` inside a bank effect and drains the options owner with bank-run `step(body, effect)`. Dev-dep testing-admission added.

## stdio raw (io-registry crate)
- Product fix: `🪞️mirror/🦀️.rs` `io_registry::register` was gone (doc comment dangling, test calls it); restored as `register_composer_entries(v_raw::entries())`.

## stdio txt
- Ported in `…/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `with_original_decode/encode`, `decode_original/encode_original` (now via `semio-framework-test-original-snapshot-owners` `with_decoding/with_encoding/decode_native/encode_native`), the bare-factory export/import (`export_sqlite/import_sqlite`), `txt_full_file_retire` and `txt_original_encoding_retire` (bank-run `retire_turn`). Dev-deps added: original-snapshot-owners, testing-admission.
- Open (lib test does not compile): the six `txt_original_encoding_*` / paid-receiving laws still call removed forwarded-encode `close_retirement_recipient`, `original_encoding::receive_into(.., &mut body)` (now 4 args / `RetainedWorkOutcome`), `NativeSnapshotEncodeOwner::receive`, and assert exact heap/receipt numbers that must be re-derived under the effect accounting; `🧪️tests/⚙️preparation` needs `store::snapshot_clone_preparation::drive_one_item_preparation_law` (store crate, not mine).

## Note for the io/control port
`EffectBank::run_outcome` leaves the Work verdict pending; call `bank.take_admitted()` after each run or the next run asserts "original Work verdict must be inspected". Retirement `step`/`close_retirement_recipient` must run under `with_original_invocation(None, effect, ..)`.

## Continuation (second fork)

### io/schema: now 5 of 5 pass
- Product bug fixed: `⚠️diagnostic/🧬️retirement/🦀️.rs` `FaultScope` inline retirement used `type RetirementCursor=Box<dyn RetirementCursor>` with a seed that allocated a `sequence(vec![..])` (224 B, unreceipted; the declared `retirement_birth_bytes` was 224 while the cursor header is 16, hence "original cursor birth differs from its defining layout" and the unreceipted nested allocation seen as heap `(2544,0)`). Now a field-wise `RetirementRecord<Self>` (same expansion as `artifact_retire_struct!`), keeping the Box-original methods.
- Test: the `maximumBytes 0` rows are expressed through the grant (a zero ceiling is a capacity denial, not an `OwnershipLimit` error). `receiver-driver.rs` `Trace` gained `stop_at_capacity`: after the first capacity-demanding turn survived every axis denial, `receive_traced` returns `Err("capacity denied")`; the schema test asserts that for rows with `maximumBytes 0`. Test binary needs `RUST_MIN_STACK` (cargo test sets enough; running the bare binary overflows).

### io/sqlite-snapshot: 5 pass, 3 open (diagnosed)
- `paged_text_preserves_original_prefix_on_receipt_refusal`: after the refused receipt the caller-retained marker keeps a Pending `NativeEncodeEffectSettlement` (its Drop asserts "original effect settlement remains unresolved"); the allocation state is ready after `settle_claimed_work` (helper `support::settle`). Resolution needs the `NativeEncodeEffectRecovery` path (`finish_refused_work`/`release_refused`) driven under the same loans; no public snapshot-level API does that yet.
- `paged_text_prework_denial_keeps_native_ownership_empty`: after the work denial the allocation state still holds a Pending settlement (status `Pending`, outcome `Absent`), so `close(allocation)` stops at "original owner requires recovery before retirement". Likely product gap: a denied work quote in `NativeEncodeControl::effect_allocated`/`NativeSnapshotEncodeOwner::encode_effect_in` leaves the begun settlement unreleased (should be released like `release_unstarted`).
- blob `complete`: `owner.copy_blob` reports stage work 1 instead of 4 (1 allocation + 3 chunks); belongs to the control octets accounting (`io/⏱️control/📦️octets`), owned by the control port.

### stdio txt and ifc
- Both are blocked: `semio-framework-io-control` (lib) currently fails to compile (`🦀️.rs:71:362` expected `&str`, found `String`, an in-flight edit by another agent), so neither `txt2` nor `ifc2` reaches its own crate. No further txt or ifc edits this round; the txt remaining laws and the `drive_one_item_preparation_law` dependency are as listed above.
