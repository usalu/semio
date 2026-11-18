# R172: giving blocks back (fixed images, replica store) and the refusal text (living report)

Everything below is staged in scratch and lands at 03:30 UTC. Nothing of R172 is live. R169 is live since the 02:00 window (live files equal its mirror, checked by sha). Scripts: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R172/` (`mkpatch.py`, `patches.json`, `land.py` apply/restore, `window.py`). Staged tree: `scratchpad/R172/mirror` (library root layout). Landing = `python3 window.py` after copying `patches.json`, `land.py` and the mirror (they are in scratch; the mirror must exist there). Restore tested byte-exact on a scratch mock of the 7 modified files (apply: 7 modified, 6 created, every file sha equal to the tested mirror; restore: sha list identical).

Corrections to R169 found on the way (read-only measurements):
- **F1 was overstated by 2x.** `ls -s` counts 512-byte blocks; R169 read them as KiB. Real host allocation of the eight images: rust 22.6 GiB (10.4 inside), dependencies 13.75 (13.2), tools 3.38 (3.14), graph 2.67 (1.91), nx 0.52 (0.5), the rest tiny. **Slack is about 13.5 GiB, of which 12.2 GiB is the rust image**, not 57 GiB. The other consumers of that table that used `ls -s` totals are also halved.
- **F4 was wrong: the replica store HAS a collector.** `shared-worktree/🗜️compaction` (snapshot, journal segment retirement, unreferenced blobs, superseded snapshots, litter; crash-tested; store-wide lock) exists. `replica.compaction.automatic` is `false` in the topology and the last applied compaction on this store was 2026-11-14 09:14 UTC (it freed 29 GB then).
- `regrowElasticImageIfQuiescentV1` (existing, in `♻️reset`) has **no caller** anywhere; `repositoryPartitionRegrowDueV1` is only consulted when the graph image runs out of room inside. Nothing ever reacts to host pressure. The due rule also needs allocated >= 3x used, here rust is 2.2x: not due even if called.

## A. Fixed cache images

Formats and ownership (read in code + `hdiutil imageinfo`): `fixed.dmg` per partition (`.cache/b/{r,d,t,g,p,n,o,l}`), raw read/write UDIF, case-sensitive APFS inside, created 64 MiB then `hdiutil resize -size` (sparse on the host APFS), attached `-nobrowse -owners on` at `<partition>/m`, receipt `image.json` pins size and identity (the identity protocol compares inode and size, so the backing must stay a fixed-size file). Lease: `native-fixed-image:<directory>`, exclusive for creation, retirement and reset, shared by users of the mapping. I use only that existing lease (via the product's regrow); lease order untouched.

Measured on synthetic images (law test `🖼️image-slack/🧪️tests`, last test):
| format | blocks freed inside | returned while attached | after detach + `hdiutil compact` | content after |
|---|---|---|---|---|
| UDIF raw (the product's) | never | `compact` fails (busy) | `compact` fails "Function not implemented" | n/a |
| SPARSE | no | no | 179 MB -> 12 MB | digest equal |
| SPARSEBUNDLE (default bands) | no | no | 191 MB -> 20 MB | digest equal |
The inner APFS sends no TRIM through the raw image driver (allocation unchanged 5 s after deleting 160 MB).

**Decision.** Keep fixed raw images. Reasons: the identity/receipt protocol, journaled retirement and 20+ laws rest on a single fixed-size regular file; sparse formats need the image detached for compaction (the same exclusive lease as a retirement), change the backing size while growing and compacting, and the bundle is a directory. So content-preserving compaction buys nothing over retirement for the elastic images, and the images that cannot be recomputed (dependencies, tools) have only 0.8 GiB slack here. Return blocks by **retiring an idle recomputable elastic image (graph, products, rust)** through the product's own exclusive non-waiting regrow, last among the host-relief collectors (after LRU CAS, after the replica store), only under volume pressure. A directory with APFS clones instead of an image is not possible for these (the images exist to give a hard bound and per-partition pressure). If a future macOS adds TRIM or `compact` for raw images, the last law test fails and the decision is reopened.

Built:
- Member `🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/`: `measureImagesV1` (allocated blocks vs `statfs` inside minus 64 MiB overhead), `decideImageSlackV1` (elastic partitions only, slack >= max(1 GiB, 10 % of allocation), largest first, only as many as the volume needs; skips named `not-elastic`, `unmounted`, `below-floor`, `not-needed`), `collectImageSlackV1` (volume read after each retirement, stops at target, named outcomes `retired`, `held`, `journal`, `failed` in EN/DE, cancellation between images), `describeImageSlackV1`. Policy `ImageSlack` and Writers row in the disk-recovery schema; the row `image-slack` is now collector `image-reset` (was manual).
- `♻️reset`: `regrowElasticImageOutcomeV1(invocation, control, partition, pressed)` returning `retired | held | journal | skipped` (old boolean function delegates), script verb `regrow <profile> [--pressed]` printing one JSON line, three targets `cache-regrow-{graph,products,rust}` (`repo ./📜️script.ts regrow <p> --pressed`). `held` is the exclusive non-waiting try (`timeoutMs: 0`) ending silently while any command maps the image: this is the "shared holders -> skip with a named reason" rule. The real port spawns `repo run repo-cache-reset:cache-regrow-<partition>` in a clean environment marked `SEMIO_DISK_RECOVERY_CHILD` (killed with the signal; the reset journal makes a killed retirement resumable: whole or none).
- Real binding in `🔌️real`: measure/describe/collect for `image-slack`.

Proofs (all in scratch against staged files, transpiler cache disabled, 15 tests): decision table on this Mac's real numbers (only rust is a candidate; nx/tools/dependencies "not-elastic", graph "below-floor"); independent Python oracle agrees on 40 random machines; fake-product laws (largest first, stop at target, claim not trusted, held and journal named EN/DE, failing port isolated, cancellation); **real hdiutil synthetic image** (recipe create 64m + resize 256m): fill 160 MB, delete -> allocation stays (`after.allocated > full.allocated - 20 MiB`), slack > 80 MiB, retire -> host blocks returned (new image allocation < old - 100 MiB), a `held` image keeps content (**digest law**: sha of `keep` and allocation unchanged); **kill law**: retirement SIGKILLed after detach -> image attaches with digest intact, SIGKILLed after remove -> no image and a new one is born. Not proven: the production retirement end to end (spawned `repo run ...regrow`), see Open.

## B. Replica store

Read-only inventory of `/Users/Shared/semio-store/v1` (81.5 GiB; this Mac is the group server `g1-server`, `.cache/f/machine.json`):
| kind | size | what references it | retention today |
|---|---|---|---|
| blobs | 59.6 GiB | snapshot entries and events after the snapshot | `compaction`: unreferenced and older than `safetyHours` 3 |
| bootstrap (`worktree-*.tar` 9.5 GiB, entries, code, kit) | 9.6 GiB | rebuilt hourly, one archive | by construction one copy |
| fleet (`install/blobs` 7.3 GiB, 25 states; `fetch` 0.7; `setup`) | 8.0 GiB | `pruneFleetInstallsV1` keeps newest 36 states | count, not liveness (not changed here) |
| snapshots | 2.4 GiB | 24 files of about 99 MB | compaction keeps 2 (not run since 11-14) |
| origins evidence | 61 MB | heads/attestations | tiny |
| trash, tmp | 0 | | |
Measured with the store's own `storeReportV1(deep)` (35 s, read-only, plan semantics): **94,257 blobs / 43.8 GiB reclaimable now**, 29 journal segments / 40 MB, newest snapshot 14.4 h old (a fresh snapshot would free more).

Rule for deletions (README "Compaction", code): one store per group, one writer; workers mount it and read it; nothing a worker deletes is published as an explicit delete; compaction runs on the group server only, under `v1/compaction.lock`, retires only what a snapshot covers and every fresh attested origin has passed, and a replica whose cursor lies in a removed segment rejoins from the snapshot. Deletion on the server affects every worker of the group (they read the store) and, via the relay copies, other groups' stores only through their own compaction. Encoded: `replicaStoreRoleV1` (group server iff `originId == groupId + "-server"` and absolute `storeRoot`), worker/none -> named skip, never a call.

Retention by construction: no new number. Liveness (snapshot reference) plus the store's own `safetyHours`/`staleHours` decide; the collector just runs `fleetCompactV1(apply)` with the topology's settings when the volume is short. The README's claim "keep every doomed file in memory" bounds one run (0.6-1.7 GiB per day of garbage; this run: 94k blobs).

Built: member `🗃️replica-store/` (`collectReplicaStoreV1`, role reader, policy `ReplicaStore`), Writers row `replica-store` (priority 85, class `store`, collector `store-compaction`, command `cd .../🤝️shared-worktree/🔁️replica && repo ./📜️script.ts compaction --apply`), real binding (`compactReplicaStoreV1` lazy-imports the replica module, only a server loads it; `measure` = what the store report says would be reclaimed, 0 on a worker). Proofs (10 tests, scratch): role table, worker and none never call and say who the server is (EN/DE), locked/refused/aborted named; **synthetic store with the real compaction**: 40 unreferenced old blobs removed, 30 referenced keep their sha (**digest law**), bytes promised by the report equal bytes the directory lost, a worker leaves the tree byte-for-byte, a held lock removes nothing; registered writer measures 0 on a worker and a dry run removes nothing; **kill law**: the real compaction SIGKILLed twice in the middle of its blob reclaim (4600 blobs) leaves every referenced blob intact. New finding: a killed compaction keeps `compaction.lock` for two hours (no liveness of the owner), the next attempt is `locked` (named, no harm) until the lock is not refreshed for 2 h; the test ages the lock and the next run finishes (trash empty).

Recommendation (not done, fleet-wide): set `replica.compaction.automatic` to `true` in the topology so the server compacts every 10 minutes by itself; the gate path is only the emergency path. Today it is `false`.

## C. Refusal text

`consumersOfV1` lists the five largest consumers and always adds `image-slack` and `replica-store` (policy `namedConsumers`) when they hold bytes, each with root, size and the exact command, from the writer's own `describe`: image slack names the largest retirable image and `repo run repo-cache-reset:cache-regrow-rust`, the store names the store root and the `compaction --apply` command (on a worker: "on g1-server: ..."). Law: with seven larger consumers the two are present in EN and DE with `12 GiB`, `44 GiB`, the commands and the store path. A named consumer without bytes is not invented.

## Staged battery (scratch, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; no cache entry names the scratch path, checked by mtime-bounded grep)
disk-recovery 19 (two R169 assertions widened: commands may start `cd `, collectors `image-reset` and `store-compaction` have their own laws), image-slack 15, replica-store 10, regrow 2, cache-gc directory 2, budget 7, verify 4, launch 9, quota 3 pass. Existing `♻️reset/🧪️tests`: 8 pass / 5 fail **identically on live and staged** (pre-existing, `(unnamed)` and four Nx owner tests).

## Landing at 03:30 (≤ 3 min)
`python3 window.py` in `🧰️scripts/R172/` (waits for hh:00-05/hh:30-35 unless `R172_NOWAIT=1`, takes `landing.lock`, applies, runs the test directories incl. the new ones, restores on red). Everything below `🧰️framework`; the workers' read goes stale once. Worth landing together with whatever R169/R170 land at that window; nothing in R172 touches lease order or the file the others edit except the disk-recovery member and the three `♻️reset` files (re-run `mkpatch.py` if any of them changed live: it diffs live against the mirror).

## Open / not verified
- The spawned `repo run repo-cache-reset:cache-regrow-<p> ` chain (CLI -> script verb -> exclusive try -> reset) has never run; only its parts have laws. First live run must be watched; on error the outcome is `failed` and the next collector runs.
- Retiring the rust image discards the unit cache (rebuild cost, tens of minutes): it only runs below the reserve after every cheaper collector, never while held.
- Non-elastic images (dependencies, tools, nx, diagnostics) return nothing automatically; named in the decision as `not-elastic`.
- `fleet/install` keeps 36 states by count (7.3 GiB); a liveness-derived keep was not implemented.
- `measure` of the store runs the deep report (35 s under load) only when a refusal is built.
- A session accident: two `grep -rI` runs started in the workspace root (a `cd` with a mistyped emoji path failed), both stopped by task stop after under two minutes; read-only.
