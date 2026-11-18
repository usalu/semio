# Lane custody refusal on prototype after release

Symptom: `repo cargo --store=shared --configuration=prototype -- test ...` fails with "Previous lane custody is unresolved"; `--store=isolated` works.

Root cause: `withLaneRetentionV1` (unit-cache/lane/retention, line ~139) hands a lane whose recorded shared destination is the release build to a prototype run; it must settle the lane first via `retainRemovedLaneV1`, whose `catch` turns every error into "unresolved lane custody". The real error was "Bounded lane evidence required" from `discoverLaneEvidenceV1`: it bounded the number of scanned directories with `rules.maximumFiles` (4096, the ledger-file bound). Release lanes hold 4.8k to 6.6k directories (`release/build/*`, `semio-compiler-resources`), so every release-used lane was refused forever. Not stale custody: no ledgers or crash residue were involved. Lane identity is not configuration-keyed by design; the settle-then-rebind step is the intended handoff.

Fix: new policy `retention.scanMaximumDirectories` (65536) in the unit-cache schema, used by the directory bound in `discoverLaneEvidenceV1`. Test: `retention/tests/dead-holder` gains "a release-sized lane with more directories than ledger files still settles"; the "limit" fixture now exceeds the new bound; the retention-io policy key test lists the new key.

Verification: diagnostic run of `retainRemovedLaneV1` on the real lanes before the fix printed the bound error for p95b1ca (6644 dirs) and paa3522 (4838); the repro command failed identically. After the fix: see the result in the chat report.

## Verification status

- Unit level: a scratch lane with 4100 directories settles. The new `🪦️dead-holder` test was written, but bun was not on the agent's PATH, so it did not run.
- End to end: `repo cargo --store=shared --configuration=prototype -- test --manifest-path <retirement Cargo.toml>` was still queued on the host capacity lock when the agent stopped; unconfirmed.
