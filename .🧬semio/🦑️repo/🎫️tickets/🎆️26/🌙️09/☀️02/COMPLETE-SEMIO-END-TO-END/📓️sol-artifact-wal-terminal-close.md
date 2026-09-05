# Artifact WAL Terminal Close

Status: implementation, registered source oracle, and exact native laws are green.

## Ownership repair

- `SegmentWriter` retains its `SprWriter` and `SharedBuf` as explicit optional owners with checked access.
- A normal close rejects pending records and requires `force_flush`; it never silently discards an uncommitted transaction.
- Close relinquishes the `SprWriter` clone before retiring the uniquely owned page buffer one step at a time.
- `ArtifactWal::close_step` and `terminal_is_empty` expose deterministic owner retirement, and later submit/flush/rotate operations reject with `DbError::Closed`.
- Rotation poisons the sealed writer before successor creation so a failed successor cannot reactivate the old sealed owner.

## Laws

- `artifact_wal_repeated_open_close_is_page_budget_neutral` performs 18 open/close cycles, exceeding the retained operation-page pool ratio, then proves a fresh append still succeeds.
- `artifact_wal_close_rejects_pending_records_and_closed_writes` proves pending close rejection, explicit flush, terminal empty, and closed-operation rejection.

## Evidence

- Registered source oracle: `wal-recovery-check: 39 checks clean`, exit 0 for this revision.
- Exact receipt: `🗑️generated/wal-recovery-exact/exact-cargo-laws-BuR4F8/00`; both terminal-close laws and the three recovery/replay laws passed.
- Native executable SHA-256: `dd5345f6881e1b55d296a6bbef6200879e8cad108e4ef43713a2d64b6f72084f`.
- Final combined regression receipt after fail-stop and capacity integration: `🗑️generated/wal-recovery-exact/exact-cargo-laws-pQEcDQ/00`; all 18 selected laws passed, including both terminal-close laws and post-seal successor failure.
