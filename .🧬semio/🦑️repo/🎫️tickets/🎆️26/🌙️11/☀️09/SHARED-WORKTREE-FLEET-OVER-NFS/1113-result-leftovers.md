# Result: leftover findings of the fleet replica (2026-11-13)

Module `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Status table written first (checked against the live code at 10:10) and updated after the landing (column "still open?": **fixed in this pass** = landed 10:42 and green on the live tree). Work copy outside the live module: `/private/tmp/.../scratchpad/lo/w` (kit and scripts: `🧰️scripts/1113-leftovers/`). Repo MCP does not connect, the ticket is not closed.

Legend: **fixed** = already fixed before this pass (evidence is the current code), **fixed in this pass** = open before and landed here, **open** = not fixed (reason given), **unconfirmed** = cannot be settled without the named hardware.

## Cross-OS audit (X01 to X33)

| id | still open? | evidence (current code) / reason |
|---|---|---|
| X01 | fixed | `🗺️topology/🧪️tests/🧫️fixtures/🪟️task-schema.xsd`, replica task `TimeTrigger` order in `🗺️topology/🟦️.ts:254-255`; not run on a Windows host (unconfirmed) |
| X02 | fixed | `🌿️git/🟦️.ts:104-105` (`core.longpaths` pin), `🚧️boundary/🟦️.ts` `pathBudgetV1`/`absoluteUnitsV1`; git for Windows behaviour unconfirmed |
| X03 | fixed | `🌿️git/🟦️.ts:107` (`core.ignorecase=false` in the view pins) |
| X04 | fixed | view supplement for case-only renames (result-cross-os), guard `caseMismatch` |
| X05 | fixed | `ReplicaOptionsV1.modeless` (`🔁️replica/🟦️.ts:21`), pin `core.filemode=false`, guard `modeMismatch` |
| X06 | fixed in this pass | `🌿️git/🟦️.ts:497` `representable` checks only separators/dots/`.git`; `changeOf` (:500) and `foldRegistersV1` apply no path policy |
| X07 | fixed in this pass | `🔁️replica/🟦️.ts:1446` `observe` `lstat`s the NFC spelling and publishes; `listed` (:121) normalises the on-disk name before comparing |
| X08 | fixed | one fold `foldNameV1` (`🚧️boundary/🟦️.ts:32`) used by scanner, `ReplicaFoldsV1`, path book |
| X09 | fixed by the clash rule | `🔁️replica/⚖️clash` retires one of two case variants on every machine (D1 both orders, `⚖️clash/🧪️tests/🟦️.ts`); the observe/scan difference remains but no longer diverges |
| X10 | fixed | `createStoreGuardV1` (`🛡️resilience/🟦️.ts:277`), daemon use `🔁️replica/🟦️.ts:2268`; real NFS stall unconfirmed |
| X11 | fixed by design, unconfirmed | mount task `WINDOWS_MOUNT_TASK` (`🗺️topology/🟦️.ts:47`); needs a Windows host |
| X12, X13 | fixed, unconfirmed | `🗺️topology/⌨️command/🟦️.ts:319,334` (enum state, `mount.exe`); needs a German Windows host |
| X14 | refuted | key is the one in Microsoft's article (result-cross-os) |
| X15 | fixed | `GIT_PINS_V1` (`🌿️git/🟦️.ts:105`), pins hash attested |
| X16 | partly | guards `links`/`trackedRefused` and preflight exist; the path policy and the link-target policy now apply to index registers (with X06, `🌿️git/🟦️.ts` `changeOf`, `representable`, `unsafeLinksV1`). Not fixed: applying index link entries on a Windows volume without symlinks (git itself keeps the entry; detection by guard `links` + preflight is the answer, a skip would make the stage differ per OS) |
| X17 | open | `repairV1` still uses `lstat`. Reason: on a case-insensitive volume the apply path keeps the spelling that is on disk (it cannot rename onto an existing file of another case), so detecting the mismatch in repair would only re-queue and re-apply the same no-op; the case-only rename is visible through the view supplement and the `caseMismatch` guard (X04) |
| X18 | open | measuring head visibility per OS needs the three OS on a real NFS server; no code path to fix, preflight already checks the mount options |
| X19 | fixed in this pass | confirmed on the live store: about 230,000 `._<sha>` files (4 KiB each, one next to nearly every blob) plus `._` files in `heads/`, `snapshots/`, `origins/**`; `reclaimLitterV1` in `🗜️compaction/🟦️.ts`, run by `compaction --apply`. Not yet applied to the live store (nothing in this pass touches it) |
| X20 | mitigated | the daemon retries (`store-unreachable`, `down()` `🔁️replica/🟦️.ts:2268+`) and the boot stage tolerates an unmounted store; only log noise remains |
| X21 | partly | macOS and bootstrap logs rotate in place (`rotateLogV1`, `🛡️resilience/🟦️.ts:384`); the Windows task output capture is in the provisioning scripts (`🗺️topology/🥾️provision`, not touched here) |
| X22 | open | logon-only service on Windows is a documented limit of the provisioning scripts (not touched here) |
| X23 | fixed | `watchFailed` reports an `error` event (`🔁️replica/🟦️.ts:2313`); a preflight watcher-mode line is not added (needs inotify limits on Linux to test) |
| X24, X25, X26 | fixed | `🌿️git/🟦️.ts:68,134` (timeouts, version), `core.protectNTFS/HFS` in the pins, `🚧️boundary/🟦️.ts:23` |
| X27 | fixed in this pass | preflight check `guards.scripts` (`crlfScriptsV1`, warning with the paths); a refusal at publish was not chosen because a refused script would silently stay different on the Windows machine |
| X28, X29, X30 | fixed | `🗺️topology/🥾️provision/🔵️.ps1:266` (`REG_EXPAND_SZ`), principal, 64-bit check |
| X31 | fixed | hint channel exists (`ensureHintsV1`, `takeHintsV1` in `🔁️replica/🟦️.ts`) |
| X32 | fixed in the touched passages | contract and README |
| X33 | fixed in this pass | `platformV1()` of the topology is the default platform of the attester (`🩺️attest/🟦️.ts`) |

## Robustness audit (items 1 to 25)

| id | still open? | evidence / reason |
|---|---|---|
| 1, 2, 7, 8, 9, 10, 11, 13, 16, 17, 21, 22, 24, 25 | fixed | result-resilience, result-boot (store guard, lock with boot time, machine key, voted `code`, segment sealing, counters, backoff, digest closure, `--force` retire, jitter) |
| 3, 4, 5, 6, 20 | fixed / refuted | result-git-robustness (fold-derived known tree, no base for large changes, `mergeTreesV1`) |
| 12 | partly open | `writeBlobV1`/relay re-hash once per process; a receiver of a corrupt blob still waits until some writer re-saves. Not fixed: the store is single-writer per origin and there is no channel back to the author; the only machine that could repair it is one that holds the right bytes, and that case is rare |
| 14 | changed | an unknown event stalls the origin with a distinct error (skipping would lose events) |
| 15 | fixed in this pass | `drifted` in `runV1` (`🔁️replica/🟦️.ts`), `repairableDriftV1` (`🩺️attest`), event `drift` |
| 18 | closed | `lost/`, `noted`, cooling pruned (result-resilience). Path table: measured, not a leak (see "Path table" below); no code change |
| 19 | fixed | see X23 |
| 23 | partly fixed | settle now copies before it moves aside (fixed). The two renames of archive and manifest are unchanged: readers verify the sha256 of the tar against the manifest and retry (5 x 20 s), the relay rebuilds at start and at most 2 minutes after a failure, and the reader contract lives in the provisioning scripts (not touched here) |

## Items from the result files

| item | still open? | evidence / reason |
|---|---|---|
| Duplicate commit under concurrent commits (seed 6038) | fixed in this pass | result-chaos-fixes "Outside this area" |
| Different-case directory hint not kept by direct register writes | fixed in this pass | `🔁️replica/🟦️.ts:955` `directoryKeys` maintained only by `setFile`/`forget`; direct writers: `files.write` at :1064,:1078,:1117,:1971,:2545 and `🧲️adopt/🟦️.ts:280,300` |
| Stale-republish guard in memory only | fixed in this pass | `🔁️replica/🟦️.ts:2023-2056` |
| Attestations of older code lacking `guards` are `missing` in `fleet` | fixed in this pass | `🩺️attest/🟦️.ts:131-141` validates the closed schema (`guards` and `latency` required); the live g1-m01..m06 attestations (code 1c30590cd115) have no `guards`, g1-m05 no `latency` either |
| Contract "Attestation" line | fixed in this pass | `1113-git-state-contract.md:83-87` lacks `fallback`, the platform names, the older-code reading |
| Edit of a path a machine ignores (seeds 4040, 4047, 5074, 8037) | open | by design: ignored paths are forgotten; the harness generator avoids it only partly |
| Retired entries are not copied to `lost/` on machines that never held the bytes | open | by design: the blob stays in the store |
| Linux/Windows runs of the clash apply path, real NFS stall, Linux systemd and Windows task entry of the boot stage, cross-group relay after retirement | unconfirmed | need the hardware named in the original results |
| `settleClashes` cost at 272k queued paths | open | not measurable without the live store (never touched by hand) |


## The fixes, in the order of risk, each with its test

Landing 10:42 (one landing, three-way rule: every file replaced only while byte-identical to the base it was built on, `🧰️scripts/1113-leftovers/land.py`; 23 files; entry in `1113-landings.md`). Tests that fail on the old code are marked (fails before).

1. **Duplicate commit under concurrent commits (seed 6038).** Reproduced with the current harness (`--profile hostile --loose --machines 5 --steps 120`, seed 6038: `c5-b546 appears 2 times`). Cause: `linearizeRefV1` dropped a chain commit that the incoming history already holds as an earlier rewrite only when its merged tree equalled the tip tree. A rewrite keeps the original committer date, so a later commit on the tip can carry an earlier committer time; the merge then lets the original win the path, the trees differ, and the commit was created a second time. Rule now: a commit whose author line and message are in the incoming history is dropped whatever its tree. Test `🌿️git/🧪️tests`: "a local commit that the winning history already holds as an earlier rewrite is dropped even when the later committer time makes its change win the merge" (fails before: two `c5`). Sweep: seeds 6030 to 6100 of the same profile, 70 of 70 pass (before: 6038 failed).
2. **Stale republish, decided from the journal.** `staleRepublishV1(events, hlc)` in `🔁️replica/⚖️clash/🟦️.ts` is a pure function of the events of one path (sorted by hlc): the event is stale when it leaves content, differs from its predecessor, was not written on top of it (`base` differs) and leaves content an earlier event left. Schema `StaleVector`, 13 fixture vectors, Python oracle (`🐍️.py`, no shared code), order independence. The replica feeds it every accepted event (not only the fold winners) and its own publishes, seeds a path without history with the manifest entry, and keeps the memory with the local state (memo `stale.history`, 6 events of 20 000 paths in memory, the newest 1000 paths persisted, written at most every 30 s and at close). The effect stays the re-assertion of the earlier work (the manifest is never changed by the rule), so machines with different memories still converge. Tests in `⚖️clash/🧪️tests`: the vectors (TypeScript and Python), "the 06:14 incident after a restart of the machines that hold the newer content" (fails before: the fleet ended on the stale content), "a machine that reads the newer content and the stale republish in one batch ends on the newer content". Limit: a machine that joins from a snapshot has no older content to compare; it applies the stale content and takes the re-publication of the others.
3. **Different-case directory hint after direct register writes.** `CompactTableV1.writes` counts every write, clear, adopt; the hint (`directoryKeys`) remembers the count it was built at and is rebuilt when any other code (adopt, join, takeover, load, retire, tests) moved it. Test `⚖️clash/🧪️tests`: "files that adopt or join wrote into the registers directly are found below a newer file spelled in another case" (fails before).
4. **Periodic self-heal.** Every two minutes (`driftMilliseconds`) the daemon reads the fleet report; when its own row is `diverged` in `manifest` it emits `drift` and runs `repairV1`, which now also (a) drops manifest rows that a complete store fold lacks, (b) queues manifest entries whose applied row is missing or has another hlc, (c) forgets applied rows without a manifest entry. Backoff 2 min to 1 h, three futile repairs end it until the difference is gone (a path one machine ignores differs by design). Never repairs code, git state or view. Seam `ReplicaOptionsV1.opened`. Test `🩺️attest/🧪️tests`: "a replica whose manifest silently drifted from the settled fleet repairs itself, reports it once and agrees again" (stale applied row, phantom applied row, ghost manifest row; the other machines emit no `drift`).
5. **Older attestations.** `readAttestationV1` reads a file without `guards` (older still: without `latency`) as present: `guards: null`, silent latency, `legacy: true`; `fleet` compares it on code, manifest, head, stage tree and view and warns `guards-unknown` (en/de). The live attestations of g1-m01 to g1-m06 (code 1c30590cd115) are exactly this case. This also lets the code-snapshot pruning and the admission see their code digests. Test: "the attestation of code that predates the guards is present with unknown guards, ..." (fails before: `missing`).
6. **X33.** `platform` of an attestation is the topology name (`linux`, `macos`, `windows`).
7. **X06 and X16 (policy part).** Index registers obey `refusePathV1` (publish: `skipped` `unrepresentable`; apply and the fold used by `verify`: ignored and settled, so one refused name no longer fails the `update-index` batch) and, at publish, the link-target policy (`skipped` `unsafe-link`). The helper `unsafeLinksV1` replaces the inline copy in the guards. Test `🌿️git/🧪️tests`: "index registers obey the path and link policy ...".
8. **X07.** `observe` requires the exact on-disk spelling for non-ASCII names (`readdir` includes the name); a decomposed name is refused `not-nfc` like the scan refuses it. Test `🔁️replica/🧪️tests`: "a name that is stored decomposed on disk is refused by the event path ..." (fails before on APFS; on ext4 `lstat` of the composed spelling finds nothing, the test passes there for another reason).
9. **X19 litter.** `reclaimLitterV1` (`🗜️compaction`) removes `._*` and `.DS_Store` entries older than `safetyHours` from the blob, head, snapshot, origin and segment directories during `compaction --apply`. Measured read-only on the live store: about 230,000 such files next to the blobs (one per blob, 4 KiB each, about 0.9 GB), 4 in `heads/`, 3 in `snapshots/`. It has not run on the live store; `compaction --apply` (or `automatic`) does it. Test: "AppleDouble sidecars and .DS_Store files ... are removed once they are old enough, nothing else is touched, and a compaction apply sweeps them".
10. **X27.** Preflight check `guards.scripts` (22 checks now): tracked `*.sh` with a carriage return are a warning naming the paths (en/de). Tests: `crlfScriptsV1` on a real repository, the warning in both languages.
11. **Bootstrap settle.** The store's content is copied to `.cache/f/tmp/settle-*` before the differing file is moved aside, then renamed into place; a missing blob leaves the local file where it was. Test (fails before: the file was gone).
12. **Contract.** `1113-git-state-contract.md` "Attestation": `fallback`, the platform names, `pending`/`failures` as optional, the reading of older attestations. README: stale rule, drift, litter, older attestations, platform names, rewrite rule.

### Path table (item 18)

Measured with the chaos harness (3 machines, 150 hostile steps, seeds 11 and 12): after settling, ids that are neither a live manifest row, a live applied row, a queue row nor a directory: 0 or 1 of about 215 per machine. The path table is bounded by the manifest (live entries and tombstones) plus the directories; the tombstones stay by design (they are what makes an old `put` lose). Reclaiming them would be a tombstone lifetime, which changes convergence rules (a replica that returns after the lifetime could resurrect a deleted path); not done.

## Not fixed, with the reason

X17 (above), X18, X22, X20 (noise only), X21 Windows capture, the item 12 corrupt-blob repair (no channel to the author), the bootstrap archive rename pair, tombstone expiry, and everything marked unconfirmed: each needs hardware (Windows, Linux with NFS, a stalled mount), a protocol change that old daemons cannot read (the closed event schema), or the provisioning scripts that this pass was told to stay out of.

## Scratch

Kit, patch scripts, runner and lander: `🧰️scripts/1113-leftovers/` (`p1-stale.py` ... `p14-giveup.py`, `land.py`, `runall.sh`, `env.sh`). The work copy and logs lived in the session scratchpad.

## After the landing (live, read-only)

The daemon of this Mac restarted onto code d122429a6bfc through the self-check and attests `platform: macos`. `fleetReportV1` on the live store now lists the machines on the old code as `stale` (their last attestation is 1.5 h old) instead of `missing`; g1-m04 is fresh and `diverged` in `view.digest` (a Linux machine; not touched here). Not observed yet: a `drift` event, the litter sweep (needs `compaction --apply`), and the new code on any other machine (they restart when the admission lets them: another origin attests the digest, which this one now does).


# Pass 2 (2026-11-13, evening): the open items, each with a test that fails on the code before

Landed once (entry in `1113-landings.md`), all 29 test files green on the live tree, daemon restarted onto code 74623e441bfd and attests (macos). Kit: `🧰️scripts/1113-leftovers2/` (`q1`..`q12` patch scripts). "simulated" = proved against a capability switch or a text check, not on the real hardware.

| id | status now | what changed, test (fails before) |
|---|---|---|
| X16 Windows without symlinks | fixed, simulated | option `symlinks` (default probed once at open): link entries stay in manifest and applied state, event `skipped` `symlink-unsupported`, no delete published (walk, observe, hot sweep), ignored by `repair` and `verify`; capability shown by `guards.volume.symlinks`/`links`/warning `symlinks`. Test "a machine whose volume cannot hold symbolic links..." (replica); race test updated (the EPERM fallback now keeps the entry instead of dropping it) |
| X17 | fixed | option `caseInsensitive` (default probed): `repairV1` queues a file whose on-disk spelling differs only by case, apply respells it (`respell`). Test "repair finds a file whose name on disk differs..." |
| case-only rename `diverged` | cannot arise from normal operation | rename put/delete in either order, split into separate rounds, equal and changed content, converge (the structural clash rule retires the old spelling first); tests "a tracked file renamed only by case converges..." and "...one round before its delete..." pass before and after (regression guards). Linux-vs-mac git view stays covered by the view supplement of `🌿️git`; not run on a real Linux+mac pair in this pass |
| X18 | fixed (measured passively) | preflight `visibility` reads p50/p95 of applied events from the replica (warn above 5 s, skip below 5 samples), per OS where the client caches act. Test in topology |
| X20 | fixed | first outage of a daemon that never saw its store is `store-unreachable` reason `waiting-for-mount`, once, then it starts. Test "a daemon that starts before its store is mounted..." |
| X21 | fixed, simulated | `boot.ps1` appends everything the boot program prints to `run.log` (cut to the last 20000 lines above 8 MiB). Text test only; no PowerShell here |
| X22 | fixed, simulated | preflight `logon` (Windows) warns unless Winlogon `AutoAdminLogon` is 1; probe faked |
| X23 | fixed | preflight `watcher` reads failure kind `watch` of the running daemon |
| corrupt/missing blob | fixed | `🔁️replica/🩹️blobs` (schema, Python oracle, ajv): receiver keeps the path queued with back-off, never materialises wrong bytes, counts failure `blob-wanted`, announces `wants.json`; any machine holding the live applied file re-hashes it and replaces the blob (`repairBlobV1`), event `blob-repaired`; fleet warning `blobs`; relay copies `wants.json`; `bootstrap-settle` stages verified (error `blob`). Tests in journal, blobs, replica, attest, bootstrap. Not done: fetching from the other group's store (the relay already replaces damaged copies there) |
| bootstrap rename pair | fixed | archive named by content (`worktree-<16 hex>.tar`), manifest names it; only the manifest rename is visible; previous archive kept 1 h; readers (sh, ps1) updated. Test "a rebuild replaces the archive and the manifest in one visible step..." (a watcher sees no torn pair) |
| tombstone expiry | fixed | lifetime 90 days: snapshots written at hlc S drop deletes older than S-90 d; a follower rejoining from a snapshot deletes live entries older than that horizon which the snapshot lacks. Tests in journal and replica ("a follower that returns after the tombstone lifetime..."). Limit: fresh `join --adopt` still publishes unknown old local files as new work; running daemons keep tombstones in memory until rebuild |

Still open: nothing on the list; hardware-only confirmation remains for Windows (X16, X21, X22), NFS, and a real mac+Linux case-rename run.
