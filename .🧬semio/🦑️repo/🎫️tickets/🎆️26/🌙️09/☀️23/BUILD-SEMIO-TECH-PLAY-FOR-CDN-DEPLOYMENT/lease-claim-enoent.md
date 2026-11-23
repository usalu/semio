# Lease claim ENOENT

Changed: `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` (`retireResourceQueueClaimsV1`), copied byte-identical to `/Users/ueli/.sinc/play/`.

- Scan sites were already vanish-tolerant: `captureResourceLeaseCandidateV1` (lstat throwIfNoEntry:false), `claimStillPresent` in `readClaimIdentity`, probe catch (rethrows only if claim still exists), final `remove` (ENOENT tolerated, dev/ino mismatch still throws).
- Fix added: `remove` returns whether it unlinked; `retired`/`allocatedBytes` count only an actual unlink, a claim that vanished is skipped.
- Claims test (`♻️retirement/🧪️tests/📋️claims`): still 1 pass / 4 fail, unchanged. Its error `lstat '.../claims/missing'` is the test's own `fs.lstat` of a fallback name: it expects the idle claim file to survive `release()`, but `releaseQueueClaimV1` (HEAD) unlinks the claim at release ("a claim never outlives its acquisition"). The test is stale vs that design, not a scan ENOENT.
- Other files (bun test, standalone): many fail with harness errors ("Explicit ticket generated artifact directory required", "Original parent Script process invocation required"), i.e. need `repo test_file`; pre-existing/unrelated. Pass: standing 26/26, order 78/78, admission 17/17, sweep 10/10, create-relief 5/5, windows, etc. Before/after counts identical (no regression).
