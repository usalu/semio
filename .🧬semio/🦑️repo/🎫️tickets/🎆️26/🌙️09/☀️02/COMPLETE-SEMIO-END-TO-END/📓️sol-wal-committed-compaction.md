# WAL Committed Compaction Consumer

Status: green for the current four-law native group. Both compaction consumers use the borrowed committed-transaction cursor. The schema-first neutral abort/commit fixture, independent source oracle, exact native laws, Nx targets, launch seeds, and coordinated all-features runtime proof are complete. True async fairness inside synchronous index compaction remains a separately documented limitation.

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
- Later actor-owned compaction receipts `exact-cargo-laws-BFzudy/00` and `exact-cargo-laws-qNCVO9/00` both timed out in the fourth capacity/reuse law. The retained implementation reconstructed `handle.compact` after each short control exhaustion, so slow filesystem awaits repeatedly discarded logical index progress.
- The retained path now creates one `IndexCursorControl` and awaits one `handle.compact` future per index kind. Its cooperative mode replenishes its deadline/fuel and yields the OS thread without unwinding that future; hard non-retained controls retain their prior deadline/fuel errors. The neutral fixture records `indexBudgetContinuation: same-owned-future-cooperative-yield`, and the source oracle excludes the reconstruction loop.
- `wal-committed-compaction-exact/exact-cargo-laws-kBRcLB/00` executed all four current native laws green. Executable SHA-256: `629c0bc217ad547bab80a7486c6108a9c2b8a94e743ba6d4d1cebbabb18f4c6f`.
- The clean current-source rerun after diagnostic removal and task/result lease repairs is `wal-committed-compaction-exact/exact-cargo-laws-CTTmHs/00`: 4/4 green, executable SHA-256 `2885e0fc2d788ea28c8c9fdef86242cfa090f133049365fd8693c56f5cca62e0`.
- This proof establishes retained logical progress and bounded completion, not true WorkerPool fairness: `std::thread::yield_now()` does not expose an async scheduling opportunity. The long-term exact seam is either awaited grant opportunities within the same future or an owned index-compaction cursor; reconstructing a destructive compaction future remains forbidden.
