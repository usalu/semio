# WAL Replay Cancellation Close Repair

Status: bounded repair and exact native execution are green.

## Repair

- `WalReplayCursor::close_owner_step` no longer requests a `WalCursorControl` grant.
- Useful replay work still requests grants through `next_step`, segment open/validation, record decoding, and normal segment retirement.
- Terminal cleanup can therefore retire retained `DbIoPages` and `DbIoU64List` owners after cancellation, fuel exhaustion, or deadline expiry without clearing or replacing caller control.

## Law

`wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty`:

1. writes a valid multi-page active WAL segment;
2. opens replay and proves source pages are retained;
3. leaves the caller cancellation flag set after `next_step` rejects;
4. drains every close step while proving cancellation remains set; and
5. proves `terminal_is_empty` without global maintenance.

The shared `wal-recovery-check` source oracle now inspects the precise replay-close body for page/list retirement and absence of `control.grant()`. Its native selector list includes the retained cancellation law.

## Evidence

- Registered recovery source oracle: `wal-recovery-check: 39 checks clean`, exit 0 for the recovery/cancellation/terminal-close revision.
- Exact native receipt: `🗑️generated/wal-recovery-exact/exact-cargo-laws-BuR4F8/00`; all five selected laws passed, including `db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty`.
- Native executable SHA-256: `dd5345f6881e1b55d296a6bbef6200879e8cad108e4ef43713a2d64b6f72084f`.
