# Mac Worker G1 M14 Verification

The Mac mini `Uelis-Mac-mini.fritz.box` is fleet worker `g1-m14` of group `g1`. The full record is in `☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m14-join.md`.

## Final State

- **Joined:** the existing `~/Documents/semio` was adopted with 0 conflicts. The replica and daily compaction services are installed, and the store is mounted from `192.168.178.70`.
- **Consent:** the macOS Documents consent is granted to the launcher. Preflight reports `worktree access: ok`.
- **Native build:** readiness reports `ready yes 0/5`, and `.cache/f/native-bootstrap.done` exists.
- **Fleet:** the view digest is identical to `g1-server` bucket for bucket. The remaining fleet disagreement lies between other origins.
- **Cache:** the g1 cache answers on `:6094`, and the clock is +0.1 s from g1-server.

## Code Fixed on the Way

1. **The agent no longer writes into an unattached macOS partition.** `cacheAreaHomeV1` refuses whenever the mount is not attached and raises `CacheAreaDetachedError`. The build agent then runs without a worker, with the reason `partition-detached`. Test: `🧾️ledger/🧪️tests/🏠️home`.
2. **The capsule recheck tolerates TCC metadata writes.** The recheck compares the capsule directory by `javascriptDependencyCapsuleIdentityV1` (`dev:ino:mode`), so `com.apple.macl` metadata written by TCC no longer breaks it. Test: `💾️storage/🧪️tests/🧷️capsule-identity`.

## Verification Gaps

- These tests passed: `🏠️home` 2/2, `🧷️capsule-identity` plus `🔖️stable-stamp` 10/10, `🧾️generation` 17/17, `🔎️why` 11/11, and 7 of 8 other `🧾️ledger` test files.
- The storage `👓️reading`, `🚧️fence` and main test files only run under the `repo test` harness (they need an issued script invocation), so they were not run here.
- One ledger test file needs `ajv`, which was not installed while the dependencies stage was pending, so it was not run.
