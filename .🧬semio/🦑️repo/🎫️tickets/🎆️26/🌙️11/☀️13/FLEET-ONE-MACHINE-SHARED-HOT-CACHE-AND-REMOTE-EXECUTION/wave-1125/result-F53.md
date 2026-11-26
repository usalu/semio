# F53: owner registry advice, self-healing read, prevention law

Landing record: `T/landings.md` "Wave 1125 F53". Applier: `T/🧰️scripts/F53/apply.py`.

## 1. Wrong advice
- The real command is the root script command `repo ./📜️script.ts verify build-owners sync`. `repo run workspace:verify -- ...` never existed. A second message (`pendingCrate` of the graph generation) said "repo verify syncs build-owners", also not a command.
- Single source: the operations catalogue (`⚙️operations/🔣️catalog.json`, new `repairs` list, schema `Repair`), read through `buildRepairCommandV1("build-owners-sync")`. Refusal (`OwnerRegistryRefusal`) and `pendingCrate` (en/de) carry `{command}`.
- Proof: registry-follow test renders every message in en and de, requires the command to appear, `workspace:verify` to be absent, and every backticked `repo ...` in the three text tables to resolve. Commands-registry test: each catalogue repair names a root command and a lane its verify module handles (source oracle). The command itself ran for real at 01:24 (read-only variant passed).

## 2. Self-heal: why the scoped repair did not add the row
Reproduced in a scratch git repo: a scoped reconcile of the named root does add the row, so the mechanism was fine. Causes found in the loop (`settleOwnerRegistryV1`):
1. One fault names one root (the native closure throws at the first missing owner) and the budget was 4 rounds that every round consumed, adopted or not. With 50+ hand-written crates the budget ended after 4 crates ("after 4 follow-up rounds"); the last round also threw before even trying to repair.
2. The repair `catch{}` swallowed every error (a peer's registry write, "changed during reconciliation", an invalid file), so a failed repair looked like "registry moved: false".
3. A scoped walk of a missing directory threw ENOENT (non-git fallback).
Fixes: the first refusal also sweeps every ownerless crate of the tree (additions only, `reconcileBuildOwnerRegistry(..., additionsOnly)`: no row is removed, moved or renamed), only rounds that added no row count against the budget, the repair error and a per-root reason (`explainOwnerlessRoot`: manifest absent, virtual, excluded tree, name already taken by another row, native root only) are in the refusal cause. Registry writes run under a lock (`withOwnerRegistryLockV1`: atomic lock directory in the temp dir, never in the replicated tree; holder pid and age; ended or stale holder is taken over by rename; a waiting writer refuses after 30 s instead of hanging) and a hand edit that lands between read and rename now reruns the reconciliation on the new file (up to 4 times) instead of throwing. Atomic temp + rename stays.
Tests: 9 ownerless crates with one refusal give 2 attempts and 9 rows; sweep leaves a vanished row alone; absent/virtual/name-taken roots refuse at once with reason and command; an invalid registry shows "the repair failed"; lock refusal, release after failure, ended-pid and stale takeover; the peer-race test now keeps the peer row and still adds ours.
Pool `ownerless` skip (F31): it files failures of the owner pool's own books. The read path does not use it: a read meets the fault through `registryFaultTextV1` and `settleOwnerRegistryV1`, which now heals instead of skipping, so nothing more is needed there; a root that cannot be adopted still refuses (the dependency is real), with the reason.

## 3. Prevention
`ownerlessCrateManifests(root)` (one manifest listing, no per-row checks) and `runCargoManifestOwnerRowLawV1` in the cargo capability tests, wired into `capability-contract-check` next to F49's parse law. Live: 0 ownerless crate manifests, 35 s under load 63. Tooling that scaffolds crates (crate-tree, mutation-tree) already reconciles; hand-written manifests are caught by the check.

## Not verified
- The repair loop was proven with the injected-fault harness (as the existing farm tests do), not by a real 2-hour whole read; the coordinator's read is untouched.
- `capability-contract-check` was not run through the repo CLI; the law function ran directly with bun on the live tree.
- Generations load test "progress counters" fails (113/1): `build:graph-phase` resource now ends in `state.detail` (⏱️phases, a peer change), not caused here.
- The registry lock is per machine (temp dir); the replica's last-writer-wins across machines is unchanged.
