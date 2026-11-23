# Stale Port: io/control snapshot, failure, octets, nested-io

Crate `🧰️framework/🔨️modules/🚪️io/⏱️control`. Lib tests compile; 30 of 35 pass (`cargo test --lib`, other agents' temporary `[DEBUG] eprintln` inside product code makes heap-exact laws flaky; rerun after they are removed). `typed-settlement` target still does not compile. The duplicate `[[test]] original-native-failure` target was removed from `Cargo.toml`: the same file already runs as lib module `failure::tests` (`use super::*`), so the standalone target could never compile.

## Shared support (new)
- `🛫️snapshot/🧪️tests/authority.rs`: `Work(bool)`/`Physical` original recipients (`Context`, `quote.complete`), `with_decode`/`with_encode`/`with_natives` (loans installed with `reborrow`, one shared `'b` for both natives), `funded`/`invoked`/`settle`, `erased_close`, `controlled_step`, `decode_close(_installed)` (birth via `with_retirement`, close inside the native's own `with_effect`), `encode_close` (`close_retirement_recipient_into` with a local reply slot), `warm()` (first-use thread bookkeeping outside heap observation). Reuses `receiver-driver.rs` (`bank`, `funded`, `admit`, `receive_traced`, `close`, `close_source`, `retire`).
- `🧾️failure/🧪️tests/source.rs`: recorded Source authorities (`Source`/`Check`/`Span`, `with_recorded_source`, `with_source_authorities` with Work and construction on the same turn).
- Dev-dependency `semio-framework-value-testing-allocator` added.

## Product fixes
- `⏱️failure-owner/🦀️.rs` and `🔁️encode/🦀️.rs`: `NativeSnapshotFailure` fields `cause`/`allocation`/`encode_effect` were `Option<Box<..>>`; `From<..>` conversions therefore allocated an unreceipted 2544-3632 byte box, against the module law ("without cloning or allocating a wrapper") and the `*_without_allocation` tests. Fields are inline again (22 failure tests then passed their exact heap checks).
- `🦀️.rs` `NativeSnapshotEncodeAllocation::reset_completed`: now settles the claimed producer work first (as the marker's `reset_completed` already did) and tolerates an accepted allocation outcome.
- `🛫️snapshot/🧾️settlement/🦀️.rs` `begin_original_settlement`: the third parameter `maximum_work` (selected work) is restored; it was hard-coded to 1 although every caller and the fixture pass it.
- `🛫️snapshot/🧪️tests/🧫️fixtures/🔣️.json` + `🟦️.ts`: `terminalChild.copyBytes` (0) became `copyWords` (2): a terminal ticket's header release now quotes `size_of::<Option<Box<dyn ..>>>()` copy bytes (`RetirementHeaderRelease::demand`).

## Ported tests (before -> after)
| Test | Before | After |
| --- | --- | --- |
| `octets` (4) | `NativeEncode*Recipient/Issuer`, `native_decoding::*Recipient` | `OriginalWork/ConstructionRecipient` + `Authority::with_loan`; close via `EffectBank` + `with_original_invocation` + `take_admitted`; refused encode asserts `encode_effect().primary == AllocationFailed` and `allocation.state.actual` |
| `failure` (17) | `receive_wire` hand-driven decoder, `step(grant)` | shared receiver driver; `close = driver::retire`; `measured_child_work` with `OriginalWorkAuthority`; depth law via `check_additional_depth` on a Source control (a real traversal now fails later at nested `ValueError` objects, not at `UnsupportedOwner`) |
| `snapshot` (8) | `OriginalRetirementClosePort`/`DeniedControl`, `receive(closure->output)` | `receive(&mut output, 6-arg closure)`, authorities lent by `with_decode`; terminal-child test keeps header quote/denial/exact-close, drops the old failure-storage refill/port-denial block (that API no longer exists) |
| `nested-io` (7) | `receive_nested(closure)`, `close_retirement_recipient(grant)` | `receive_nested(&mut output, 5-arg closure)`, outputs set (`*out=Some(())`), `with_natives` |

## Open
- `original_receiving_nested_io_dual_control_empty_phase_change_keeps_original_wallet`: observed heap is 108 bytes above the wallet receipt (27525 vs 27417); the extra allocation is inside the authority-backed frame admission, not yet located.
- `original_receiving_nested_io_decode_retains_child_ticket_and_same_wallet`, `original_receiving_canceled_publication_retains_exact_io_error_fields`: closing the decoder recipient after a canceled nested receive does not finish (recipient close stays pending/zero progress under the creating authorities).
- `encoded_octets_keep_original_backing_prefix_and_receipt` (`first-copy-receipt`): recipient submitted 2 items, wallet `body.progress()` only 1; a refused receipt does not reach the local wallet (suspected product bug in `effect_allocated_with_work` when the receipt is refused).
- `original_receiving_publication_header_denials_retain_exact_output_backing`: intermediate retire now demands more copy than the fixture's `zeroCopyBytes` (1), so `zero` refuses before the 1 byte intermediate is charged; the layout law/fixture needs re-deriving.
- `typed-settlement` (`🧪️tests/♻️settlement/🦀️.rs`): needs `RetireOwned::RetirementCursor`, original Work/construction recipients and, above all, a product API to record performed work on `NativeSnapshotEncodeSettlement` (`record_effect` no longer takes the actual work and `settle_original_effect` needs `submitted_progress`, which only `record_before_post` sets).
