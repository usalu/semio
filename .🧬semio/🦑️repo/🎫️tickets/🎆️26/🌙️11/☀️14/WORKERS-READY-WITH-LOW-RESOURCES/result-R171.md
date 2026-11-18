# R171: one machine reads and seals a selected read, the others fetch it (living report)

Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R171/` (`patches.py`, `window.sh`, `battery.sh`, `preload.ts`). Scratch: `scratchpad/R171/`. Landing: `landings.md` "R171 landing 1" (01:32:59 to 01:33:23 UTC, hold 24 s; a first try at 01:30 was restored by my own battery compare, see 4).

## 1. What was built (schema first)
New member `🕸️graph/📥️load/🤝️selected-exchange/`: schema (`Policy` with the declared `revision`, `Head`, EN/DE texts), `🟦️.ts` (the exchange), `🔌️attempt/🟦️.ts` (adapter to the load module), `📏️revision/` (law helper), tests. Edits of `📥️load/🟦️.ts` (3 anchored, attempt function untouched): import, `selectedAdmissionFactsV1` beside the owner-facts memo, helper `selectedThroughFleetV1` and its one call for a scope inside the graph custody.

1. **Publication.** After a selected attempt seals, the adapter reads back the sealed graph and receipt, builds the head and publishes: blobs first, head last by compare-and-swap. Head key = sha256(domain, revision, platform, scope digest (selection + registry)); no path, root, HOME or host name. The head carries `owners` (per-owner digest of the neutral rows, R156 grouping by holder of the file) and `membership`.
2. **Acceptance.** Before the attempt: look up the head, verify every blob against its name and every owner digest against its rows, localize rows to this root. Per owner: equal digest in the machine's own memo, else all rows replay in the tree (`buildGraphSourceFactsHoldV1`). Matching owners seed the admission facts memo (zero re-observation); if all hold, the graph and a receipt in this machine's spelling are written and the normal attempt serves it from the receipt (zero inference). Differing owners are read by the normal attempt (records of unchanged owners come from the owner-record exchange) and the result is published as the new head. Never rejects the whole generation for one owner.
3. **Machine independence.** Rows go through `neutralRowsV1` (root token, stamps zeroed, location rows dropped); documents through `neutralizeRootV1`.
4. **Semantics revision.** Constant `revision` in the schema policy, part of the head key. Law `📏️revision`: history of (revision, digest); digest over the declared stamp of the 19 inferrers, the conformance corpus of the derived inferrers and the graph schema. Fails when a pinned input moves without a bump or a revision has two digests. NOT covered: edits of inferrer code that move no stamp, corpus or schema (stamps stay hand-bumped), and result values of inferrers without derived stamps; the "handful of synthetic owners per family" corpus was not built (needs the real inference harness).
5. **Concurrency.** Miss: claim on the head key (lease 30 s, heartbeat 8 s). Holder reads; others wait with a named bound (25 min, slice 15 s, progress text EN/DE naming the holder), then fetch. Lease lapse: waiter claims and reads (`taken-over`), owner records already published serve it. Wait bound hit, fleet down, denied, no upload right, damaged head: read locally, never hang. An attempt error propagates unchanged.
6. Telemetry: stderr line `[selected-exchange] {outcome,held,differing,published,waitedMilliseconds}` and `sources.exchange` in `slice-memory.json`.

## 2. Proof
- `🧪️tests/🟦️.ts` 17 (model fleet, 2 to 3 machines with separate roots, homes, stamps): second machine on identical tree installs, 0 observations/takes/inferences; k=2 of 40 changed: held 38, differing exactly those, 2 observations, 4 inferences (changed + dependents), third machine converges with 0; 1 of 40 differing keeps 39; own-digest fast path touches no tree; simultaneous start: outcomes read+waited, 16 inferences for 16 owners, waiting text with bound; dead holder: taken-over, 6 taken + 4 inferred; live holder beyond bound: reads; fleet down/raising: reads; keys/heads/documents byte-equal across two roots, homes, stamps and contain no root/home/hostname; key varies only with scope/revision/platform; corrupt blob, valid blob under another name, foreign scope, other revision, damaged digest, garbage: miss/fault never installed; Ajv schema + independent digest recomputation; no re-write of an equal head; no claim without upload right.
- Mutation evidence (scratch, each turned red): key with HOME, whole-reject on one differing owner, no blob check, no claim, stamps kept in rows, root kept in admission, revision check dropped (needed an extra law).
- `🔌️attempt` 3 (install writes graph/receipt in the local spelling, seeds memo, partial seeds only holders, throw propagates), `🔗️wiring` 3 (red on live, green staged), `🖥️real-server` 3 against the real fleet cache server (blobs, CAS head, claims, `await` on stored head), `📏️revision` 4. All also green under the genuine-birth harness (`--birth=setup`).
- Real server probe on the fleet cache of this machine: claim granted, second claim held, release, await lapsed (no head written).
- Battery after landing equals baseline (candidate 1/2 and stamps 21/1 are baseline reds).

## 3. Not verified
- No real selected read ran through the new path yet (slice lab is R161's; load 60). The adapter is proven against models and the real server, not against a real 590-owner read. Facts rows replay across machines assumes content-hash fallback when stamps differ (documented behaviour of the observation check).
- Whether a published head fits the server limits for 590 owners (about 50 KB of head, MBs of blobs): unmeasured.
- Rows under a worker's HOME (tool files) will differ per machine: those owners count as differing (safe, slower).
- Replica lag ping-pong (a lagging machine republishing older owners) is by design converging but unobserved.

## 4. For others
- Any test that extracts `loadBuildGraphUnpinnedV1` needs the binding `selectedThroughFleetV1` (added to `adopt-registry`, `candidate`).
- Changing the stamp table, the conformance corpus or `🕸️graph/🧬️schema/🔣️.json` now fails law `📏️revision` until `revision` is bumped and a history row appended.
- Measurement freeze from 02:05: I make no further framework edits; new `📏️revision`/`🖥️real-server` tests were created before it.

## 5. Watch log
(appended below)
- 01:38 UTC: no worker has restarted a read on the landed code yet (no `[selected-exchange]` line, no `sources.exchange` in any `slice-memory.json`; g1-m08 at "owners done 590 of 590" after 1781 s, g1-m03/m07/m10 still taking, g1-m02 at 23458 s in an old process). The stale-stage rule hands each over; the freeze 02:05 to 03:30 is the undisturbed measurement. Still to record (not yet seen): outcome and wall of the first worker that waits for or installs a head, `owners.pass`, and the `workspace` stage time when another machine has sealed. I make no framework edit during the freeze; I did not poll further.
