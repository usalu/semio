# WAL Fail-Stop Flush

Status: production repair, neutral corpus, source oracle, and exact native validation are green.

## Contract

- `SegmentWriter::commit_and_flush` computes the final retained length and requires `WalStorage::append` to return that exact value before an acknowledgement is possible.
- Commit, retained-copy, append, short-success, and sync failures poison the live writer. Subsequent submit, flush, and rotate calls return `DbError::Closed` until close and reopen.
- The physical appended length is recorded before sync, so a sync error cannot cause a later retry to append the already-landed suffix twice.
- A poisoned writer remains deterministically closable even though its failed transaction counter remains nonzero; close releases ownership without converting the failed operation into success.
- `FaultStorage` exposes an exact one-shot `fail_nth_sync` boundary and separate observed/delegated sync counters.

## Neutral corpus and laws

- `🧪️fixtures/🛑️fail-stop/🔣️.json` plus its Draft 2020-12 schema describes short append, append error, sync error, and post-seal successor failure.
- Three exact retained laws prove the physical suffix is respectively torn, absent, or complete; every second attempt is `Closed`; reopen preserves exactly the last complete prefix without duplication.
- The successor law proves a committed pre-seal transaction replays exactly once after successor creation fails.
- The testkit law proves the injected sync failure occurs after its append, fires once, and does not count as delegated.

## Evidence

- Registered source oracle: `wal-recovery-check: 44 checks clean`, exit 0.
- `rustfmt --check` parsed both Rust units; it returned exit 1 for repository formatting differences and is not claimed as a formatting pass.
- `git diff --check` is clean for the bounded patch.
- Exact native receipt: `🗑️generated/wal-recovery-exact/exact-cargo-laws-pQEcDQ/00`; all ten recovery/fail-stop/close laws plus eight established WAL baselines passed, 18 assertions total.
- Native executable SHA-256: `a3f2bc3d24dc99467d9f1cfbcd4ec9f2289fddd563a2dacefd54b196fafaccdb`.
