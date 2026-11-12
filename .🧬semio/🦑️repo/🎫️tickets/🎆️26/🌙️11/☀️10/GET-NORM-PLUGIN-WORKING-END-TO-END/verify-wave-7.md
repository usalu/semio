# Verify wave 7 (verify11-)

Logs: `🗑️generated/verify11-*.log`. No `[DEBUG]` lines remain in source. One cargo run at a time throughout; every run queued behind other agents (the first intrinsic run timed out at 3000 s while still queued).

## Root cause found: release quotes without `copy_bytes`
`PagedList::release_empty_page` now denies unless `grant.maximum_copy_bytes >= next_release_copy_byte_demand()`. Many close quotes still declared `copy_bytes: 0` for a whole-backing release, so a grant equal to the quote was denied, nothing progressed, and callers reported phantom progress (`copied_items:1`) or looped forever. Fixed in:
- `pack/📐️format/🎮️retained/📄️pages-and-source-state` `retained_page_close_demands` (shared by source cursor and symbol table)
- `record-body-cursor` `SymbolsPhysical` no longer zeroes the copy quote
- `pack/🌱️value/🫳️preflight/🏭️schema/⏳️cursor` `close_list_demand`
- `pack/🌱️value/🛬️decode/📦️document` ids release, `pack/🌱️value/🛫️encode/🫳️borrowed/📨️wire` wire release
- `pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor` new `next_close_copy_byte_demand`, used by `retirement_demands`
- `dsl/🧬️schema/🛫️encode/🧮️expression` frames release
Still lacking copy (same pattern, not touched, other owners; about 40 sites, list via `grep next_release_allocation_bytes` without `next_release_copy_byte_demand`): store replay/outbox/ledger, ui fixed-list/resources, math rank, replication dictionary decode, tool-run receive, fem sparse, plugin draw/energy/ifc/layout.

## Step 1/2 results
- pack-intrinsic: 18/18 PASS (was aborting in close). Includes the earlier `advance` fix.
- pack-record-preflight: 3/3 PASS. The "spin" was the close/hash loop not progressing for the same missing-copy reason. The test also had a nested `turn(...)` inside `turn(...)` (`RefCell already borrowed`); the stage loop now measures heap with `observe_backing`.
- io-control-testing: 4/4 PASS. The test wrapper `Watched` implemented `ErasedSnapshotRetirement` without `native_header_release`; it now delegates.
- pack-record-encode-cursor: 11/14 (was 3 failing on close). Deflate close now terminates. Remaining 3:
  1. `borrowed_pack_failure_preserves_real_comparison_and_original_symbol_backing`: failure fixture policy `maximumCopyBytes` 64 < page-metadata quote; fixture raised to 4096 (like the sibling test). Not re-run.
  2. `borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec`: "original work requires its original caller loan"; the expression cursor issues additional original work but the test `turn` bank has no recipient. Needs a loan-carrying harness. OPEN.
  3. `borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages`: "text formatter exceeds its original capacity" from `staged_rig::Stock::for_outputs`. OPEN.
- replication `paged_seal_digest`: FAILS "seal did not settle" (10M turns with `generous()` grant; 87 s). OPEN: likely another zero-copy quote in the canonical edit seal cursor (not examined).
- replication `retained_diff_sequence` (both tests) and replication-causal `original_causal_envelope_clone...`: abort with "original work issuer retains an issued action or committed proof" at test end. Not located; it fires on a clean test exit, so a fixture/`drive` helper leaves an issued action (shared testing crates, other agents' area). OPEN.
- io-sqlite-snapshot-tests: still 58/64. The same 6 failures: 2 "forwarded native allocation requires an actual quoted effect" (retirement test line 43, unit test line 212), 3 receiving-path quote assertions (`work > 0`, 48198, 1641078), 1 `unwrap_err` on Ok in fixed-limits. OPEN (tests need re-expressing over the receiving/quoted path).
- io-sqlite-snapshot lib tests do not compile: `🧩️artifact/📦️blob/🧪️.rs` imports `NativeEncodeAllocationIssuer`, `NativeEncodeConstructionRecipient`, `NativeEncodeWorkRecipient` (and decode equivalents), which no longer exist (retired recipient/issuer protocol; the same names are used in store `native-encoding/tests/control` and `space-history .../json-recovery`, `partial-projection`). Must be re-expressed under the new effect protocol. OPEN.

## Step 5 (✏️s `allocation_stage`)
34 callers across 28 files migrated to `allocation.decode(..)` / `allocation.encode(..)`; remaining `new_forwarded` uses are csv/tsv/txt tests that build their own port. BUILD PAUSE #6 arrived: the per-workspace check loop was stopped after launching root 1 (✏️s/🔌️plugins/🀄️wfc, `verify11-root-1.log`, still running, left to finish). The other 21 workspace roots are listed in scratchpad `roots.txt` and are UNCHECKED. Resume order: run each root with `repo cargo ... check --keep-going` from that directory, one at a time.

## Not done
typegen and the 4 async-tests, step 4 untested crates, heap-assertion plugin crates, remodel + `retained_params`, block-3d `retirement_law`, vcs/draw, testing-acceptance gating re-run (all need builds that were queued behind other agents).
