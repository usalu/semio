# Graph Incremental Load: Churn-Tolerant Slice Inventory

Taken 2026-11-22. Scope: the whole-phase restart of a selected slice load (`session-s`, about 3,100 of 3,600 owner checks, then back to 25-735).

## 1. Trigger of the restart

Evidence in `🗑️generated/session-s-retry.log`: at slice second 4092 the progress line drops from `(217526 distinct source observations) (3138 completed owner checks) (994944 verified memo rows)` to `(190546 distinct source observations)` with no owner or memo counters. The browser agent saw the same resets at second 4134 (35) and 6693 (25), each right after the observation count changed. The step "inventory" was never left (no `owners-round` text), so the restart is inside `settledInventory` in `🕸️graph/📥️load/🟦️.ts`:

1. `buildGraphMembershipV1` (or the inventory walk through `vanishedMemberAsChangeV1`) throws `BUILD_MEMBERSHIP_CHANGED` as soon as one member vanished or changed type. With replication that happens every few thousand seconds. An earlier attempt died on `lstat ...🧰️scratch` (`session-s-retry-3.log`).
2. `settledInventory` evicts the memo of the owners that read the named path (already correct) and starts a new round with a new step and report.
3. That round asked every other owner again through `observeOwnerFactsMemoizedV1`, which called `hold(memoized.rows)` for each: about 1 M row checks, CPU bound on one core, 0.6 owners per second. The next membership change arrives before it converges; the round count has no budget by design.

Not the cause: facts digest (compared only after inference), checkpoint (16384 memo entries and 256 packs hold 3,600 owners; no discard path hit), worker restart (the observation count would restart near 3,784, not 190,546).

## 2. Fix: only dirty owners are re-checked

`🕸️graph/📥️load/🟦️.ts`
- `observeOwnerFactsMemoizedV1` takes an optional `trust` (`has`/`add` on entry objects). An entry verified, recovered or observed earlier in the same `settledInventory` call answers without a second `hold`. Evicted owners have no memo entry and are observed again.
- `settledInventory` owns one trust set for its membership follow-up rounds. A settle round that repeats the whole admission (`onRound`, two rows of one source contradict) drops the trust, so a trusted stale entry cannot keep a conflict alive.
- Consistency: trust never leaves one `settledInventory` call. Repair rounds and the seal start empty, and the seal still verifies every observed row, owner rows included. A change to a trusted owner is found by the next inventory or the seal and repaired for that owner alone, or sealed as `sealed-with-later-changes`, as before.

## 3. Biggest per-owner cost

Rows of kind `file` whose recorded stamp differs from the disk (files a replica rewrote with the same bytes) were re-hashed by streaming the whole file in the async hold path, once per row object, round and seal. The sync path already had `fileShaMemoV1`; the async one did not use it.

`👁️observation/🟨️.mjs`: new `fileShaAsyncV1` uses and fills `fileShaMemoV1` (same calm-stamp rule). No schema change.

Not done: no profile was possible (worker stderr is `/dev/null`, `sample` has no JS symbols), so the CPU hotspot inside an owner check is not pinned. Remaining candidate: repair-round inventories still verify every memo entry once per round (at most `repairRounds` = 4); trust could be carried from a fully passing seal hold.

## 4. Tests added

- `🧪️tests/🕸️graph/🩹️repair-rounds/🟦️.ts`: a follow-up verifies no already verified owner (holds equal the quiet run, only owner-2 observed again); a change to a trusted owner is still caught and sealed with version 2 only; a settle round forgets the trust (4 extra holds); the `trust` contract. The harness settle fake supports `settleRounds` and `onRound`.
- `👁️observation/🧪️tests/🧱️file-digest/🟦️.ts`: a calm file with a moved stamp is read once for many rows; a rewrite is read again.

## 5. Verification status (honest)

Not run to green. `repo run repo:test-build-workspace-graph-laws` ran 6,300 s and failed in a dependency I did not touch (`@semio-tech/framework-actor-rs:typegen`, then `Original funded graph publication namespace required`). Two retries without dependencies were refused by the PreToolUse hook (host client unreachable). No before/after slice-load measurement: the live worker (PID 90651) runs old code and a fresh load takes hours.

To run: `repo:test-build-workspace-graph-laws`, `repo:test-build-observation`, `repo:test-build-graph-protocol`.
