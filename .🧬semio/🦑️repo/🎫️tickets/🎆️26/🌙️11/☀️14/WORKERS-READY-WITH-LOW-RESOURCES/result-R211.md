# R211: why the fleet's owner records were refused, the peer that blocked the read, and what the read adopts now (living report)

Written from 2026-11-18 15:02 UTC, continues R208. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R211/` (`land.sh`, `land2.sh`, `stage/` base and new copies, `law/` the checked-in laws, `mk.py gen.py bat.sh t.sh preload.ts` the staging, `an4.ts` is in the scratch only). Scratch: `scratchpad/R211/` (`samples.txt`, `reasons.py`, `sample.py`). No download, no slice-lab run, no pkill, no sub-agent; one kill only of my own two stuck staged test processes.

## 1. Who blocked the read ("a peer builds the graph") and may the door serve a refresh and a selected command together

- The 13:37 read of R208 sat 37 minutes in `source.served`. The log of that read (`.cache/f/graph-server/read.1795014117190.log`) names the holder at its last waits: `graph: 1 refresh(es) go first, one of them rebuilds the whole workspace ...; holder: pid 39126 (... show project repo-mcp), started 2026-11-18 14:01:54 UTC, code 36fe7308d114, start 0 of 0, 1088 s without progress`. So the peer was another agent's `repo show project repo-mcp` (the verification command of the R209/R210 line of work): before R210's landing a `show project p` was a WHOLE read with a structural builder claim (`claimsConflictV1`: a structural claim conflicts with every other claim), so it took the builder custody of the generation store (`.cache/b/g/m/generations/s/<token>.json`, `holder`, `ticket`, `state held`) and the server's refresh queued behind it. Before 14:01 the log names no holder (the door board prints holders only from the first notice on); at 15:06 a dead claim file of pid 46352 (`show projects`, queued 14:17:28, held, last progress 14:21:24) was still in that directory: a holder that died without releasing it. `landing.lock` and the door board show live holders only; a dead holder's claim is judged by its lease, so it does not block, but nothing removes the file.
- Since R210's landing a `show project p` is a selected read (owners of p, not structural), so it conflicts only with claims that share an owner or are structural: the whole refresh IS structural, so a selected command still queues behind the refresh until the refresh gives way at a wave boundary (it does, `SEMIO_GRAPH_BACKGROUND=1`, via the ask/deposition protocol of the custody). The door should NOT serve both at once without that handover: a selected read that shares owners with a running whole build would read half-built state; the handover at wave boundaries is the right shape. R210's `repo show project @semio-tech/repo-mcp-rs --json` of 15:04 to 15:09 UTC held the door for five minutes (80 percent CPU) while the refresh read of that moment was not running (it had failed, section 2), so it delayed nothing.

## 2. The finding that blocked everything after 15:00: the graph server's own read ended after nine seconds

R210's landing at 15:00 made every whole question default to the graph policy `current`, which refuses (`GENERATION_NONE_SEALED`, "a question about every project never starts the hours of inference it would need") when no sealed generation of this code identity exists. The graph server starts its whole read as `repo show projects` WITHOUT a policy (`startGraphReadV1`, `🌐️remote-execution/🛰️sidecar/🕸️graph/🔭️read/🟦️.ts`), so since 15:00 every refresh ended with status 1 after about nine seconds (15:02, 15:12, 15:29, 15:32 UTC) and no read ever reached its owners; the next attempt came ten minutes later (the fleet claim `graph-refresh` for 600 s). Fixed in my landing 1 (section 4): the read's environment names `SEMIO_BUILD_GRAPH_POLICY=fresh`.

## 3. What the stale reasons were (the read printed them from 15:57 UTC)

Read A (15:46 to 16:32 UTC, carried landing 1 only): 62 stale at the end, 5 stale-row lines, all `file` over edited `Cargo.toml`: no `analysis` row, no `closure` row, no `exists` row named. 24 of the first 28 stale records were refused without any line (section 5). Read B (16:35 UTC onwards, carried landing 2, which tells every verdict and the reason a record is refused):

| first failing row / reason | what it is | share |
|---|---|---|
| `reuse` "inventory records named by the record are not stored here (source or lock closures)" | the rows of the record all hold, the farm child says `holds`, and the record is refused because the leaves `build-input-closures/<sha256>.json` (6 per Rust owner) and `build-lock-closures/<sha256>.json` (1) that its value names are not on THIS machine (`inferenceInventoryReferencesHoldV1`, the `reusable` check of `observeProjectInferenceV1`) | 61 of 88 lines at 17:01, 49 distinct owners; 19 of 22 at 16:50; every record that the sample of `an5.ts` took out of the fleet cache had 6 of its 7 leaves missing |
| `file` over `Cargo.toml` | real edits of the tree by other agents while the fleet produced (`window/.../rust/Cargo.toml` 14:15, `wire-limits/.../Cargo.toml` 16:47) | about 25 lines |
| `exists` over `🤖️generated/` (1) | my precedence bug in landing 1, fixed in landing 3 (section 4) | 1 |
| `readdirNames` of `✍️editor/📦️packages/🦀️rust` (1) | the listing holds `.🧾️store`, a directory that git ignores; now machine-local (section 4) | 1 |
| `analysis` (any reason) | none in 88 lines | 0 |

So the dominant reason was NOT a context-dependent judgment but a cache state: the leaves are files of the producing machine's own store; the record names them by content key and the fleet does not carry them. A fresh process judging the same records (R208 `an3.ts`, mine `an4.ts`) says every non-closure row holds; the leaves explain the rest.

## 4. Landings (details in `landings.md`)

1. Landing 1, 15:34 UTC: the graph server's read names `SEMIO_BUILD_GRAPH_POLICY=fresh` (law `🌱️fresh-policy`, 2 red on the base); analysis rows are judged by parser role and revision, not by the id a process gave the parser (law `🪪️parser-identity`, real child processes: a fresh process, a child that judged 50 other owners with an older revision of the role registered first; 3 red on the base); one predicate `isMachineLocalPathV1`/`machineLocalRowV1` and the schema list `machineLocalPathKinds` (15 kinds) for recording and judging (law `🤖️machine-local-paths`, one test per kind with its control).
2. Landing 2, 16:30 UTC (diagnosis): a stale record whose rows each hold gets a row of the kind `whole` (`staleWholeRowV1`), every named row carries `judge <n> of pid <p>`, the farm verdict asked after the import is told (its row, or `verdict other|missing`). Laws `🧮️stale-whole`, `🧮️stale-fleet-told`. It also contained a regression (below).
3. Landing 3, 17:02 UTC (repairs of my own): (a) the `reusable` wrapper of landing 2 broke the law `📥️reuse` "compact reference admission" from 16:30:59 to 17:02:02 (my battery did not include that directory): removed, the reuse line is gone again; (b) the precedence bug in `current()`: `machineLocalRowV1(kind,path)||kind==="file"?a:b` parsed as `(local||file)?a:b`, so every non-file machine-local row of a stored record was still judged by the file check; fixed, the law now asks the judge of a stored record (`firstFailingRowV1`) for all 15 kinds (14 red on the base); (c) `.🧾️store` is a machine-local directory name.
The interrupt: the read of 15:46 UTC was interrupted ONCE at 16:32:23 UTC (`kill -INT 15786`, the runner whose command line holds `read-record.json`, parent 1) after the pool and graph roles had restarted on landing 2 (16:31:50); read B started 16:35:32. Read B does not carry landing 3.

## 5. Why 24 of 28 stale records had no named row in read A

The take asks a farm child for a verdict BEFORE it fetches the record (the child answers `missing`, no row) and again, through `observeProjectInferenceV1`, after the fleet import; only the first answer was told (`looked.staleRow`), the second (`farm.observe(...)`) was dropped. R208's 48 lines of 14:42 UTC were the first answers of records that were already stored here.

## 6. Samples of the two reads (R202 format reduced; `samples.txt`, `table.py` in the scripts folder)

"taken" = fetched + stale (records the read took out of the fleet cache); the counter "stale" is the read's own count of owners whose taken record was refused and which were then inferred (or are still to be inferred) here. `present` = owners whose record was already stored here (inferred by an earlier read of this machine).

Read A, started 15:46:03 UTC (policy `fresh` fixed, parser identity, machine-local predicate), owners from 15:55, interrupted 16:32:23:

| time UTC | done | present | fetched | inferred here | stale | stale per 100 taken (fetched+stale) | stale-row lines | pools published (sum of wave) |
|---|---|---|---|---|---|---|---|---|
| 15:55:55 | 0 | 0 | 0 | 0 | 0 | - | 0 | 19 |
| 16:07:46 | 58 | 3 | 37 | 18 | 52 | 58 | 4 | 127 |
| 16:11:14 | 64 | 3 | 42 | 19 | 58 | 58 | 4 | 214 |
| 16:21:13 | 104 | 3 | 47 | 54 | 62 | 57 | 5 | 179 |
| 16:31:15 | 108 | 3 | 47 | 58 | 62 | 57 | 5 | 237 |

Read B, started 16:35:32 UTC (also tells every verdict), owners from 16:44, still running at 17:32 (runner 58428):

| time UTC | done | present | fetched | inferred here | stale | stale per 100 taken (fetched+stale) | stale-row lines | pools published (sum of wave) |
|---|---|---|---|---|---|---|---|---|
| 16:45:17 | 19 | 7 | 2 | 10 | 6 | 75 | 6 | 193 |
| 16:51:18 | 56 | 23 | 6 | 27 | 19 | 76 | 34 | 247 |
| 17:01:21 | 87 | 52 | 7 | 28 | 27 | 79 | 88 | 47 |
| 17:11:23 | 94 | 55 | 10 | 29 | 27 | 73 | 177 | 37 |
| 17:21:25 | 94 | 55 | 10 | 29 | 27 | 73 | 179 | 226 |
| 17:31:27 | 95 | 55 | 10 | 30 | 27 | 73 | 184 | 67 |

Rate of read B: 95 owners in 48 minutes of the owner phase (1.6 per minute, the last 30 minutes 0.4 per minute: 22 to 27 owners are running at once, the longest for 2500 s, load 34 to 43, the farm children get 15 to 50 percent of a core). Fleet side: the six pool roles published 36 to 247 records per wave (about 40 per minute fleet-wide), the read took 10 of them in read B.

## 7. Answers

- Are the fleet's records now adopted, at what rate: NO, not at a rate that matters. Read A adopted 47 in its first 20 minutes (R208's read: 0), read B 10 (its first 50 owners were mostly `present` from read A). 57 to 79 of every 100 records taken from the fleet are refused afterwards, and of the owners named in read B 59 are refused for ONE reason (33 more for edited `Cargo.toml` files, 2 for machine-local rows; no `analysis`, `closure` or `whole` row), which is not a judgment but a cache state: the record's value names 6 source-closure leaves and 1 cargo-lock leaf (`build-input-closures`, `build-lock-closures`, small JSON files named by their sha256) that exist only in the store of the machine that produced it (checked: none of the six leaves of `value-native-storage-rs` is in this machine's CAS either, so the producers do not upload them). `inferenceInventoryReferencesHoldV1` therefore refuses every Rust owner of the fleet, and the owner is inferred here (150 to 220 CPU-seconds each, R207).
- Projected finish of the whole read: not within days on this machine: 4690 owners left at 0.4 to 1.6 per minute is 49 to 195 hours, the sampler's own eta 784 hours at the last rate; R207's idle machine figure (13 to 32 hours) and the fleet figure (about 40 records per minute published, 2 to 4 hours) both need the records to be adopted, which needs the next fix.
- What the next fix is (NOT done, needs its own ticket): make the leaf part of the record. The producer's export (`exportProjectRecordV1` + the documents of `createProjectFleetPortV1`, `documentsOf`/`closureDocsOf`) adds one document per leaf the value names (`captureInferenceInventoryReferencesV1(value)` gives the names; 7 per Rust owner, 76 bytes to 150 kB, 4 kB on average, 5081 + 1176 files and 21 MB on this machine for all 4785 owners), the taker writes each after checking `sha256(bytes) == key` (`publishInputClosureV1`) before `reusable` runs; the fetch predicate `isClosureDocument` and the entry schema in `🪪️graph-state/📦️project-records` need the new document name, the pool producers must run the code (they restart by themselves on a landing), and the records already published need to be re-exported (`exportProjectRecordV1` reads the local record, so a restarted pool republishes them). Law: a record exported on one root and taken on a machine whose leaf directories are empty is reused and its leaves exist afterwards; hash mismatch refuses. I did not build it: 40 minutes remained after the reason was named, it changes the entry format of the fleet and needs the cross-machine test.
- The cheaper alternative I rejected: accepting the record and leaving the missing leaf to the load (`closure-missing` -> `settleClosure`): that repairs ONE key per round (`CLOSURE_REPAIR_ROUNDS_V1`), so thousands of missing leaves would fail the load instead of costing a local inference each.

## 8. Not verified, what others must know

- The effect of landing 3 (precedence fix, `.🧾️store`) in a live read: read B started before it. Its farm children run the code of 16:30. A coordinator interrupt would carry it; I used my one.
- R210: the selected read `repo show project <name>` queues behind the whole refresh (structural claim) until the refresh gives way at a wave boundary; nothing in the door serves both at once, by design (section 1). A dead holder's claim file (`410d65...` of pid 46352, 14:12) was still in `.cache/b/g/m/generations/s/` at 15:06; I did not touch it.
- The graph server retries a failed refresh only every ten minutes (`graph-refresh` fleet claim of 600 s plus a three minute hold after a failure): after a landing that restarts the role while a read is gone the next read starts up to ten minutes later; between 15:02 and 15:46 nobody noticed that the server had stopped reading (the log of the failed read said so in one line: `GENERATION_NONE_SEALED`). A watch on `read.log` for "No whole read is running" would have caught it at 15:03.
- `🧩️field` fails 2 of 3 on the base when run without the artifact directory of the nx runner; `⚡️incremental`, `🆔️record-revision`, `🌊️owner-scale`, `📦️projects` fail on the base too (counts in the landing entries).
- Staged laws can hide: the first child-process law used `URL.pathname` (percent-encoded emoji) and passed staged with an absolute path; the first landing of it was restored by the script. A law that spawns a process must be run once unstaged against the base before the window.
- No download, no slice-lab run, no kill except `kill -INT 15786` (the authorised interrupt) and my own two stuck staged test processes of 15:27; no sub-agent; all scratch outside the worktree; the scripts folder holds `land.sh`..`land5.sh` with `stage/` (base and new copies), `law/` (the checked-in laws), `mk.py gen.py bat.sh t.sh preload.ts` (staging), `an4.ts an5.ts` (read-only scanners), `reasons.py sample.py table.py samples.txt`.
