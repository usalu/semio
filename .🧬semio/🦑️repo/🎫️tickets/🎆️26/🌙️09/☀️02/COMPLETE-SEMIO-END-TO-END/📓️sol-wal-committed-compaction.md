# WAL Committed Compaction Consumer

Status: green. Both compaction consumers use the borrowed committed-transaction cursor. The schema-first neutral abort/commit fixture, independent source oracle, exact native law, Nx targets, launch seeds, and coordinated all-features runtime proof are complete.

## Contract

- Ordinary and retained compaction share `committed_compaction_horizons` and `committed_compaction_payloads`; neither production walk can decode the raw `WalReplayCursor` stream.
- Each cursor exposes one verified committed transaction at a time. Every borrowed record is closed through `close_record_step` until false, each fully consumed transaction is finished, and each cursor is closed through `close_owner_step` until false on success, cancellation, or scan error.
- Horizons are seeded from `WalCommittedCursor::segment_indices()`, not inferred from emitted body records. The header-only highest segment therefore remains the protected live segment.
- Only committed `Frontier`, `SnapshotPub`, and CAS payload records can affect retention and payload deletion. Aborted records never reach either effect accumulator.

## Neutral proof

`db/🗜️compact/🧪️fixtures/🧾️committed-effects` defines one sealed segment containing an aborted high snapshot/CAS pair followed by a committed low frontier/CAS pair, plus a header-only active successor. At floor 10 the committed segment is deleted, its committed payload is reclaimed, the aborted payload remains, and the header-only successor remains.

The Bun oracle validates the fixture against JSON Schema and independently evaluates committed transaction effects before auditing the Rust consumer cutover.

## Gates

- `@semio-tech/framework-os-kernel:wal-committed-compaction-check`
- `@semio-tech/framework-os-kernel:wal-committed-compaction-native-check`
- `db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects`
- launch orders 411.069/411.070

## Evidence

- The first direct source invocation evaluated the neutral fixture successfully and stopped only on an overly broad source-marker assertion. That assertion was corrected to inspect the dedicated terminal-close helper.
- The next registered rerun did not enter the command because the concurrently edited repository library imported a missing `getWorkspaceRoot`; no green receipt is claimed.
- After that shared export settled, direct and registered Nx source runs both returned `wal-committed-compaction-independent-oracle: abort effects excluded, committed effects retained, header-only highest preserved`; the registered target exited successfully.
- Coordinated plugin-registry generation and the explicit `check-generated --skip-nx-cache` gate are green. Generated launch entries are at lines 5282/5289 and retain exact ticket-local artifact and shared target paths.
- `rustfmt --check` parses the changed Rust owner and reports formatting-only differences. Focused `git diff --check` is green.
- The coordinated combined DB receipt `wal-committed-transactions-exact/exact-cargo-laws-sYMpBg/00` records a green all-features build and laws 0 through 5 green, so the compaction module and its native fixture law type-compiled in the exact test binary. Law 6, root-owned `db_sync::tests::sync_replay_ignores_neutral_aborted_command_snapshot_and_cas`, then failed with `Unavailable("wal cursor deadline reached")` before the compaction selector was reached. This is not a compaction runtime verdict; the warmed coordinated rerun remains required.
- The corrected coordinated rerun `wal-committed-transactions-exact/exact-cargo-laws-wc3Pia/00` executed all 13 exact laws green. The compaction law passed as selector 12, the Nx session exited 0, and the all-features DB test executable SHA-256 is `6b49a4a76da7df7b09455059e4de5f1acc80ed605e4344756783233032e92d99`.
