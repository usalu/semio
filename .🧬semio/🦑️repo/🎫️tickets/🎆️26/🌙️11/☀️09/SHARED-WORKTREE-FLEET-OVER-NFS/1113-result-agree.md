# Result: agreement at common checkpoints on a moving tree (claude-agree, 2026-11-13)

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Landings: `1113-landings.md` (21:22Z and 21:51Z). Scripts: `🧰️scripts/1113-agree/` (`land.py`, `runtests.sh`, `fixture-agree.py`, `live-probe.ts`). Contract: `1113-git-state-contract.md` (new bullet `checkpoints` in the Attestation section). Operator text: README of the module, section "Attestation, fleet and self-restart", paragraph "Agreement on a moving tree (checkpoints)".

## 1. Design

**Problem.** `fleet --wait N` needed every origin fresh, settled (cursors = heads, queue empty) and identical at one wall-clock moment. On a tree that eight machines edit all the time that moment does not exist, and a machine that is a few events behind looked like a failure.

**Principle.** The fold of the journal events below a cursor vector (a *cut*: one `segment/bytes` per origin) is deterministic (LWW by HLC). So (a) two machines at the same cut hold the same shared files, and (b) two machines at different cuts hold the same entry for every path that no event between the cuts touches. Agreement is evaluated at points of the event history, not of the wall clock.

**What a daemon attests (`checkpoints`, optional, absent in older code).** Three components, each with its own cut, because shared files, git state and view are read at different moments:
- `manifest`: taken on every tick at which the incoming queue is empty and no apply is in flight (then the files table is exactly what the cursors name; the replica ticks sequentially, so nothing publishes in between). Carries the cut, the digest and the same digest in **256 buckets** (bucket = low 8 bits of FNV-1a over the UTF-8 bytes of the NFC path; the first 2 bytes of every term fold into its bucket). The buckets are maintained incrementally next to the existing XOR digest in `CompactTableV1.swap` (one extra FNV and 2 XORs per changed row, rebuilt in one pass when a saved state is adopted, zeroed by `clear`): a checkpoint costs a copy of 1024 hex characters and no hashing of the manifest.
- `git` (`HEAD` commit, stage tree) and `view` (digest, 256 buckets over the `XY path` lines, mask of *pending* buckets). A reading takes time while events keep arriving, so it carries a **span**: `cursors` = the last fully applied cut before it began, `until` = the cut after it ended. The view checkpoint is also refreshed whenever the incremental view is current (nothing pending) at an empty-queue tick.

**How `fleet` compares (`compareCheckpointsV1`, `gapBetweenV1`, `readGapWindowsV1`).** For two origins it reads from the journals the events that lie between their spans (per origin: after the lowest `from` up to the highest `to`; at most 50 000 events and 8 segments per origin) and compares everything outside the buckets those events touch. Defined precisely:
- *manifest*: the buckets not touched by a gap event (all 256 when the cuts are equal). Undecided when the gap touches every bucket or a journal cannot be read (compacted, too far).
- *git state* (`head.commit`, `indexTree`): equal expected unless a gap event touches the git state (`.git/HEAD`, `.git/refs/heads/**`, `packed-refs`, `shallow`, the index realm, `.gitignore`/`.gitattributes`/`.gitmodules`); then undecided. Objects, remote-tracking refs and tags are not git-state events.
- *view*: the buckets that neither a gap event nor a pending bucket of either side touches; undecided when a gap event touches the git state (a commit or stage change rewrites the lines). An origin without git is exempt.

**Verdicts (rows with `basis: "checkpoint"`).**
- `ok`: manifest checkpoint at most 60 s old, **no event older than 60 s missing from its cut** (`oldestUnseenV1` against the store heads, so a machine that stopped following is `lagging` however fresh its attestation), and each component decided at least once against another origin and equal. It need not be at the head, idle or settled.
- `lagging`: no common checkpoint yet (still moving, not wrong): checkpoint older than 60 s, an old event missing, a component never decided, or an arbitrary tie (below).
- `diverged`: different values at the same position of the history. The row names `manifest`, `head.commit`, `indexTree` or `view.digest` and the buckets as `#n=hex`. Arbitration per bucket: the origin that disagrees with most others goes first, an origin that equals others at that bucket counts for it; with two origins the larger id is named; a tie among three or more origins is *not* guessed: both sides are `lagging` until a third checkpoint decides (found in the first chaos tests: a dropped event in a bucket that the other origins' gaps also touched blamed the healthy server).
- `code` and the `guards.*` fields are state, not history: they are compared among all live origins as before.
- Older code without `checkpoints` stays compared the old way, among its own kind only, row basis `now`, and the warning `no-checkpoints` says so. `fleet --diff` also lists the not-ok rows and the checkpoint differences; its exit code follows the checkpoint verdict when origins attest checkpoints (a moving tree always differs a little in raw lines) and the line comparison otherwise.

**The working-tree view (decision and justification).** The view is local disk state and only comparable for the paths that did not change after the cut. It therefore is not compared as it is now but at its own checkpoint and bucket-wise outside (a) the paths of events between the cuts and (b) the paths the daemon knows to be changed and not yet shown in the view (watcher events, hints, applies, publishes; everything after a restage or an overflow). Paths named by the watcher or a hint and not yet published (`unsettled`) are pending too, and a named, unpublished change of the git state keeps the git and view readings back for at most 30 s (after that the name is taken for noise: an IDE refreshes `.git/index` all the time). A change on disk that the daemon does not know (tamper, a missed event) is exactly what stays unexplained and is `diverged` in `view.digest`.

**Cost.** Attestation grows from about 1.6 KB to about 5 KB (three cuts of eight origins, two 1 KB bucket strings), still one atomic write per second at most. `fleet` reads the journal events between the cuts (the window is empty when everything is at the head).

## 2. Files

`🩺️attest/🟦️.ts` (types, buckets, gap, compare, judge, `createAttesterV1` checkpoints, `readGapWindowsV1`, `fleetReportV1`), `🩺️attest/🧬️schema/🔣️.json` (Checkpoints, Cut, Buckets, Mask, row `basis`/`checkpointSeconds`, warning `no-checkpoints`, fixture sections), `🔁️replica/🟦️.ts` (table buckets, snapshot fields `buckets`/`busy`, `unsettled`, watcher and hints note view dirt), `🔁️replica/⌨️command/🟦️.ts` and `🔁️replica/📖️texts/🔣️.json` (column `basis`, `fleet.differs.checkpoint`, `fleet --diff` rows, `fleet.warning.no-checkpoints`; EN and DE), `README.md`, tests below. `status` shows the same table through `showFleet`.

## 3. Tests

- **Language-agnostic fixture** (`🩺️attest/🧪️tests/🧫️fixtures/🔣️.json`, generated by `🧰️scripts/1113-agree/fixture-agree.py`): `bucketing` (12 paths incl. NFC, 19 git-path vectors, 3 manifest bucket vectors, 2 view bucket vectors) and 13 `agreements` (cuts that never meet, same cut, one dropped event, difference outside / inside the touched buckets, pending bucket, git event in between, different HEAD, a reading that spans events, an old event missing, unreadable events, stale checkpoint, older code). Judged by the TypeScript module and by the independent Python oracle (`🐍️.py` + `🐍️agreement.py`, hashlib only; jsonschema 4.25.1 validates the fixture): 97 vectors, 0 mismatches.
- `🩺️attest/🧪️tests/🟦️.ts`: 128 pass (incremental buckets equal the recomputation over 600 random operations, after `adopt` and after reopening; checkpoint cuts are taken only with an empty queue and copied; spans widen over events that arrive during a reading; a fetched remote ref does not hold readings back, an unpublished stage change holds for 30 s only; older tests adapted to checkpoint semantics).
- **Property test with the chaos harness** `🔁️replica/🧪️tests/🧭️agree.ts` (4 tests, real git, in-process replicas, edits on every machine at every step, rounds on random subsets, attesters wired like `runV1` with delta views): (1) 4 replicas, 60 steps, `fleet` reaches `ok` with rows still behind the heads or with queues, and never reports `diverged`; (2) a file changed on disk behind the daemon (`stale` op) is `diverged` in `view.digest` on exactly that replica at a common checkpoint while the edits continue, no other replica is named; (3) one event dropped from a replica's files table (`clearId`) is `diverged` in `manifest` on exactly that replica; (4) commits and stage changes among the edits never produce a divergence. Passed 3 of 3 repeated runs plus the live run. The harness `checkFleetV1` now ticks until the checkpoints are complete.
- Whole module on the live tree after each landing: exit 0 (attest 128, replica 94, agree 4, chaos 9, clash 20, adopt 23, git 45, tiers 58, journal 129+18, boundary 69, topology 65 ...). 20-seed mixed and 10-seed hostile (5 machines, 60 steps) chaos sweeps on the copy: 30/30.

## 4. Live

- First landing 21:22Z: the daemon of g1-server restarted onto the new code within about a minute and attested `checkpoints` (5 KB). **Regression found on the live fleet and fixed in the 21:51Z landing:** the first version held the git/view readings back for any `.git/` path named by the watcher; the auto-fetch writes `.git/objects/**` and `.git/refs/remotes/**` all the time, so the git checkpoint of g1-server and g1-m02 aged to 86 s and 618 s and every checkpointed origin was `lagging`. Now only paths that decide HEAD/stage/view hold, for at most 30 s (tested). After that all three components were decided between all live origins (`live-probe.ts`).
- g1-m01, g1-m03 and g1-m05 have sync off (excluded; their daemons keep running the old code because nothing replicates to them), g1-m02, g1-m06, g1-m07 followed the code by themselves within minutes.
- Two real differences were found and are not mine to repair: (a) `guards.caseMismatch` 0 on g1-m02 versus 10 on g1-m07 and g1-server (named as `diverged`, a volume/guard difference unrelated to checkpoints); (b) at 21:57Z g1-m02 was `diverged` in `view.digest` with 87 local-only `??` entries only on that machine (ticket mirror trees below `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/toolsetup-g-tools/mirror/` and `…/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/wbs-tools/sb-config/🎨️styling`), exactly what `fleet --diff` prints; at the final run they were gone and g1-m02 was `ok`.

### `fleet --wait 300 --lang en` from `🔁️replica`, verbatim (299 progress lines "Waiting for agreement: N origins pending, S s left." omitted; N was 2 for 142 s, 3 for 133 s, 4 for 15 s, 5 for 9 s)

```
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 3s 5/0 now off 2f03e0cd8033 78/0/81 -/-|-/- off (sync is off, not compared)
  g1-m02 macos 0s 0/0 cp:0s on 2d6054177087 154/0/210 214/660|557/12929 ok
  g1-m03 macos 4s 5/0 now off 2f03e0cd8033 71/0/72 -/-|-/- off (sync is off, not compared)
  g1-m04 linux 19258s 6/0 now on 7757dd83d95b 123/0/292 694/1209|180/346 stale (attestation too old)
  g1-m05 darwin 12387s 7/536 now off 7a383a08ec1e 0/0/272364 -/-|-/- off (sync is off, not compared)
  g1-m06 macos 202s 3/0 cp:202s on 2d6054177087 152/0/208 6/6|1644/34919 stale (attestation too old)
  g1-m07 macos 2s 0/0 cp:2s on 2d6054177087 154/0/210 691/3077|368/2951 ok
  g1-server macos 0s 0/0 cp:0s on 2d6054177087 154/0/210 164/426|277/2221 ok
  warning: g1-m04 runs older code (7757dd83d95b) that attests no checkpoints; it is compared as it is now, only with other origins of that kind, and not with the others
The fleet is not in agreement (8 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
exit 1
```

**Verdict: exit 1, but every origin that runs the new code and attests is `ok` or `stale` for a reason outside the comparison.** g1-server, g1-m02 and g1-m07 are `ok` at common checkpoints while the tree keeps changing (they were never settled in the old sense: their cursors are behind the heads most of the time). The two blockers are `g1-m04` (sync on, old code, silent for 5.3 h and not retired: `retire g1-m04` or restarting it is the operator's call) and `g1-m06` (new code, its attestation 202 s old at the end: its daemon stopped attesting after it had joined; not investigated, not repaired). Before the fix a `fleet` on the same fleet said `lagging (not settled)` for every origin.

## 5. Not verified / notes

- Linux and Windows not run; the bucket hash and comparison are OS-neutral (Python oracle agrees).
- Old-code readers reject new attestations (closed schema, `checkpoints` unknown) and show the new origins as `missing`; compaction and the drift repair use only code that runs on the same machine, but a `fleet` run on an old machine is wrong until it restarts (no compatibility layer by design).
- The first view of a restarted daemon still takes 30 to 50 s on this tree (`view` checkpoint absent, row `lagging` meanwhile).
- A local edit that no tier has noticed yet (no watcher event, no hint) shows as `diverged` in `view.digest` until a tier finds and publishes it (up to the file/directory tier interval); `fleet --wait` keeps polling through it. This is the same condition as a tamper and is not distinguishable in one snapshot.
- Freshness still uses the writer's wall clock (skew above 60 s makes a machine look stale/future).
- The tie rule (three or more origins, no arbitration) can leave two origins `lagging` for some seconds after a real divergence in a bucket that all other gaps touch; it resolves with the next checkpoints.
- Compaction: a gap that reaches into compacted segments is undecided (`lagging`), never `diverged`.
