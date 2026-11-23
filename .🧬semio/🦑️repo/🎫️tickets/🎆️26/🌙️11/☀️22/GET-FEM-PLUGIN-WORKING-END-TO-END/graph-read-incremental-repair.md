# Graph read: incremental repair on a live tree

Scope: `🕸️graph/📥️load/🟦️.ts` (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`). Nothing running was stopped or restarted; all validation is unit tests plus the repair-rounds fixture world.

## What was wrong (from the loader, not guessed)

Each outer round of `loadBuildGraphOnceV1` does: owners round, `settledInventory(true)`, `facts-hold`. Three full verifications per round, and:

1. `settledInventory` created a fresh `trust` set per call, so every repair re-verified the memo rows of ALL owners (434-3600) again, though only a few directories changed.
2. The taken memo (`replay`) re-verified every kept owner's rows again per pass.
3. The gate (`observationsHold`) stopped at the first stale row, so the stale set named only the 1-64 rows in flight. The rest of the dirty owners were only found by the per-owner re-verification of 1 and 2, which is why they could not be dropped.
4. `repairRounds = 4` then caps a read that pays three full passes per round, so a tree that moves every minute never converges cheaply.

## Design

A look that names EVERY changed row makes the dirty set exact; then only dirty owners need work, and one full verification per round (the gate) remains as the correctness check.

* `observationsHold(..., exhaustive)` (new trailing parameter, also on `buildGraphSourceFactsHoldV1`): keeps verifying after the first stale row and reports each one to `onStale`; a contradicting pair of rows is reported and still fails the look, but the remaining rows are checked as well. Default behaviour unchanged.
* The loader gate and the failure-path hold use `exhaustive`.
* `retire(moved, drifted)` after a repair-needed gate: when the stale list is complete (`staleCount === stale.length`, below `sealStalePaths`) and the change is explained (named paths, or no membership/admission drift), the owner facts that read a moved path are evicted (`evictOwnerFactsReferencingV1`, same rule as the taken memo) and the read-wide `readTrust` set survives. Otherwise (list truncated, membership moved with no named row, failure path) `readTrust` is reset: full verification, as before.
* `readTrust` is created once per read (was per `settledInventory`). It is reset at the start of every pass that does not follow a complete gate, so the first pass keeps its old early catch of an edit made during the first owners round.
* Taken memo: `trustSurvivors()` / `distrust()`; `createPassReuseV1.boundary({complete})` evicts the moved paths and then trusts or distrusts the survivors; `replay` skips `holds` for trusted entries (`stats().trusted`).
* Soundness: trust never skips the seal gate. Every round's `facts-hold` verifies all observed rows (inventory rows incl. retained owner rows, inference rows) with real probes; an entry trusted from the previous complete gate that changed since is caught by that gate and evicted next round. The sealed graph is therefore still backed by a full verification, and a stuck stale fact still ends as `unrefreshed-facts`.
* Inner membership retry (`BUILD_MEMBERSHIP_CHANGED`) already evicted by named paths; it now shares the read-wide trust, so a retry pays only for the evicted owners.

Measured effect (repair-rounds fixture world, 200 owners, 3 edits across 4 owners rounds, count of verification calls, `[DEBUG]` scratch test removed):

| | verification calls | owners rounds |
| --- | --- | --- |
| trust reset every round (old per-round behaviour, same code otherwise) | 804 | 4 |
| shared trust + exhaustive gate | 204 | 4 |

That is 4 x 200 owner re-verifications down to 1 x 200 + a few; on the real tree each avoided owner hold is a pass over its rows (up to ~97 k rows for the largest owners), so the saving is the 854 k row references per repair named in `graph-read-performance.md`. Not measured on the live tree (not allowed to disturb the running read).

## Excluded paths

`workspaceBuildFilesV1` uses `ROSTER_RULES_V1.descends`, which already excludes `.🧬semio` (tickets), `.cache`, `.git`, `.nx`, `node_modules`, `target`, `🗑️generated`, generated directories from policy. Checked against the persisted memo of scope `171969…` (73.8 MB): `.🧬semio` occurs 13 times only as config strings (nx input globs, not observed rows); `.cache` occurs as ~165 distinct `lstat` rows of the TypeScript toolchain package that analysis really reads (not membership), `🗑️generated` only as exclusion globs. So tickets/caches/generated output do not trigger repairs; the observed churn (`✏️s/🔌️plugins/🏗️fem/…`) is real source. Guard test added; no code change needed.

## Changed files (all under `…/📚️library/🏗️build`)

* `👁️observation/🟨️.mjs`, `🟨️.d.mts`: `exhaustive` look.
* `🕸️graph/📁️workspace/🟦️.ts`: `buildGraphSourceFactsHoldV1(..., exhaustive)`.
* `🕸️graph/📥️load/🟦️.ts`: `readTrust`, `retire`, exhaustive gates, boundary `complete`, docstring of `observeOwnerFactsMemoizedV1`.
* `🕸️graph/📥️load/♻️pass-reuse/🟦️.ts`: `boundary({complete})`.
* `🕸️graph/📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs`, `🟨️.d.mts`: `trustSurvivors`, `distrust`, `trusted` counter.
* Tests: new `👁️observation/🧪️tests/🧹️exhaustive-hold/` (`🔣️.json`, `🐍️.py` independent oracle, `🟦️.ts`); new `🕸️graph/📥️load/🧪️tests/🚫️excluded-membership/🟦️.ts`; added test in `♻️taken-memo/🧪️tests/🟦️.ts`; added test in `🧪️tests/🕸️graph/🩹️repair-rounds/🟦️.ts` (fails without the shared trust: discovered 2 instead of 3, so it discriminates); `🧪️tests/🕸️graph/🌊️seal-under-churn/🟦️.ts` and `🧊️stale-facts/🟦️.ts` now bind the REAL `evictOwnerFactsReferencingV1` over the same memo (they stubbed it to `()=>0`, which only worked because every owner was re-verified).

## Test results (bun, native; run beside the files)

* exhaustive-hold 2/2, excluded-membership 1/1, taken-memo suite 5/5 (incl. pass-reuse), observation listed-children, stale-row, discovery-reads, round-listings green earlier.
* repair-rounds 29/29, seal-under-churn + stale-facts 5/5 (200 seeded churn worlds each).

## Open / caveats

* Another agent is mid-change in the same loader (new import `../🧾️registry-reads/🟨️.mjs`, `ownerRegistryDigestV1`, `../🔕️settle-quiet`, which do not exist yet). With their half-done state the repair-rounds / seal-under-churn / stale-facts / inventory harnesses fail with `ownerRegistryDigestV1 is not defined`, independent of my change. I validated by running scratch copies of the tests with `ownerRegistryDigestV1`/`askOwnerRegistryV1` bindings added (copies deleted). Their harness bindings still need adding. Their write also overwrote my loader edit once (last writer wins); I re-applied it, re-check `grep readTrust` in the loader if it vanishes again.
* Still one full verification per round (the gate) and the membership lstat of all members; removing that needs the watcher change journal (R2 in `graph-read-performance.md`).
* Cold inventory (hours) and a graph-format key flip are untouched.
* `repairRounds = 4` unchanged; with cheap rounds it could be raised later.
* The running `describe fem` loads graph modules at worker start; it does not pick this up until the next slice worker.
