# Review: sync correctness of journal, boundary, replica

Method: hand tracing only (nothing can run here). Files: `📒️journal/🟦️.ts`, `🚧️boundary/🟦️.ts`, `🔁️replica/🟦️.ts`, `🔁️replica/⌨️command/🟦️.ts` (command: no defect found, it only routes; relay `--once` and the loop both call the changed `relayOnceV1`). Line numbers are those of the files after the edits.

Cross-module contract: every name `🔁️replica` imports from journal and boundary exists with the implemented argument order and return shape (`appendEventsV1(store,origin,events)`, `writeBlobV1(store,origin,sha,source)`, `readEventsV1(store,origin,cursor)`, `mergeClockV1(last,hlc,wall)`, `tickClockV1(last,wall)`, `formatHlcV1(clock,origin)`, `writeSnapshotV1(store,origin,hlc,manifest,cursors)`, `scanSharedV1(worktree,ignore,signal)`). One additive change: `readNewestSnapshotV1(store,onlyOrigin?)`. `winsV1` is now also imported by the replica.

## Defects found and fixed

| # | Scenario | Where | Defect | Fix |
|---|---|---|---|---|
| 1 | 6, 2 (crash) | replica `runV1` first iteration | After a crash between materialise and the (5 s throttled) state save, the restart published first: the tree holds remote content R2 that state does not know, the scan republished it as a LOCAL put with a newer hlc, which beats a third machine's newer edit R3 in the meantime (lost update). Same for a remote delete unlinked but not yet recorded (a delete is republished over a re-created file). | The first loop iteration now follows before it publishes (`recovering` flag). The stale cursor re-reads R2/R3, queued paths are skipped by publish, `same` content just refreshes state. |
| 2 | 5, 6 | replica `persist` | Order was state, cursor, clock. A crash after the cursor but before the clock left a cursor past events whose hlc was never merged into `clock.json`: the next local publish after restart could get a smaller hlc than events already consumed (skewed peer). | Order is now clock, state, cursor, so any persisted cursor implies a persisted merged clock. |
| 3 | 3, 4, 1 | replica `materialise` end | mtime recorded for echo suppression was `lstat(target)` AFTER the rename. A user edit between rename and lstat (same size) was recorded as "state" and never published. | The stamp is taken from the staged temp file before the rename (`rename` keeps mtime on POSIX and NTFS). Same value, no window. |
| 4 | 9 | replica `observe` | For every missing touched path the whole state map (270k) was scanned to find children: `rm -rf` of a directory with 10k files = 10k x 270k string tests. | Children are scanned only when the path itself is not a state entry (i.e. it was a directory). |
| 5 | 9, 8 | replica `runV1` watcher | Watcher events of ignored trees (`node_modules`, `target*`, `.nx`, `dist`) were queued. A build easily crosses `touched.size>5000`, which sets `full=true`, so the "15 s safety net" became a full 270k-file scan every 150-250 ms. | The watcher callback drops paths the current matcher ignores (parent-directory rule included). |
| 6 | 7, 9 | replica `followOnceV1` | One origin with a corrupt segment or an unreadable head (`ESTALE`, bad JSON) threw out of `followOnceV1`: no origin was followed and nothing was applied until fixed, on every poll. | Head read and event read errors are isolated per origin and reported once (`error` event); the other 49 origins proceed. |
| 7 | 10 | replica `followOnceV1` | `ENOENT` on a compacted segment only emitted `rejoin-required` on every 250 ms poll and the follower stayed stuck. | New `recoverFromSnapshot`: reads the newest snapshot written BY THAT ORIGIN (its own compaction boundary), folds its manifest by hlc, queues the winners, sets the cursor to `snapshot.cursors[origin]`, re-reads in the same round. Only when no usable snapshot exists `rejoin-required` is reported (once, cleared on success). |
| 8 | 10 | replica `foldStoreV1` | Join/verify/snapshot/compact used the globally newest snapshot's cursor for an origin; that snapshot can be older for that origin than the owner's compaction boundary, so the origin was skipped ("rejoin-required") and its events silently missing from the fold. | On `ENOENT` the fold retries from that origin's own newest snapshot. |
| 9 | 10 | replica `relayOnceV1` | Relay did not copy snapshots; a follower in another group could never recover, and a fresh group store lacked the blobs of events living in compacted segments. | New `relaySnapshot`/`relayBlob`: before segments are considered, the newest `snapshots/<origin>/*.json` of every origin homed at the peer is copied (all referenced blobs first, hash-verified, then tmp + rename of the snapshot; idempotent: nothing is copied when the file exists). Segments/head code unchanged except for using the shared blob helper. |
| 10 | 10 | replica `compactV1` | Segment deletion used the cursor of a SECOND fold taken after the snapshot. Events appended (and a segment sealed) between the two folds are not in the snapshot file but their predecessor segment was deleted: joiners lose them. | Boundary is read from the snapshot file just written/found. |
| 11 | 10 | replica `compactV1` | Blob GC ran even when some origin could not be folded (`rejoin-required`): blobs of that origin's live entries looked unreferenced. Also the age check used a stat taken before the second fold, so a blob re-used (touched) meanwhile could still be deleted. | GC and segment deletion are skipped when the fold reports an incomplete origin; every doomed blob is re-statted right before `unlink`. |
| 12 | 10, 9 | journal `writeBlobV1` | A blob that already exists is not rewritten, so its mtime stays old. Re-publishing an old, currently unreferenced content (revert) races with GC (unreferenced + older than retention) and could lose the blob after the event was appended. | An existing blob is touched (`utimes`) on reuse. |
| 13 | 7 | journal `readEventsV1` | For a sealed (non-head) segment the reader trusts everything to EOF; if a read returns a partial last line (stale NFS attribute), the cursor was still advanced into the next segment, skipping the tail forever. | A partial trailing line in a sealed segment stops the read at the last complete line; the next poll continues in the same segment. |
| 14 | 11 | journal `replaceFile` (head.json, snapshots) | `rename` over an existing file with EPERM/EBUSY/EACCES (Windows reader holding it open) failed the whole publish. | 5 attempts with exponential backoff. |
| 15 | 8 | replica `refreshIgnore` | A transient read of `.gitignore` as empty (editor truncate-then-write, checkout) rebuilt the matcher with no rules; the next scan would have published every ignored tree (node_modules, target*) to the fleet. | An empty text is not accepted while state holds `.gitignore` as a live put; a real deletion is accepted once its delete is in state. |
| 16 | 2, 8 | replica `publishOnceV1` | A shared FILE replaced by a DIRECTORY of the same name was only `forget`-ed (no delete event); remote machines kept the file and then blocked forever on `parent-is-a-file` for `p/x`. | A candidate that is now a directory is published as delete. Paths that merely became ignored/refused are still dropped without an event (decision kept). |
| 17 | 2 | replica `prune` | `startsWith(root+sep)` with a worktree option carrying a trailing separator never matched: empty directories were not pruned. | Root normalised through `dirname(join(worktree,"x"))`. |
| 18 | 4, 9 | replica `runV1` | The touched batch was cleared before `publishOnceV1`; a failed publish (store hiccup) dropped those paths until the next full scan. A failing full scan re-ran immediately every poll (nextScan only set on success). | Touched paths are restored on failure; `nextScan` is set before the scan. |

Tests added in `🔁️replica/🧪️tests/🟦️.ts`: recovery of a follower whose segment file is gone (from snapshot, no `rejoin-required`), relay copying snapshot plus its blobs. No existing test or fixture needed changes (no covered behaviour was altered; relay result counters keep their shape, blobs copied for a snapshot are counted in `blobs`).

## Traced and found correct (no change)

- Scenario 1: blob, event, head, state order; echo suppression by (bytes, mtimeMs, executable); chmod-only change republishes; executable bit survives on Windows via the previous state.
- Scenario 3: both converge on the greatest `(ms,counter,origin)`; the loser with an unpublished edit is copied to `lost/<hlc>/<path>` and state is set to the winner, so the next scan publishes nothing (no ping-pong). A loser whose edit was ALREADY published is not copied to `lost/` (its bytes stay in the blob store): matches the contract wording ("unpublished local edit") and the existing fixtures, so left.
- Scenario 4: publish runs before follow in a loop iteration and `base` is the previous state sha; an event older than a published local edit loses at `ingest` (manifest holds the newer hlc). An older event that arrives while the edit is still inside the 150 ms debounce (or not yet noticed without a watcher, up to `scanSeconds`) wins and the edit goes to `lost/`: accepted by the contract.
- Scenario 5: `ingest` merges every received hlc before the next `mint`; `tick` = `max(wall,last.ms)` with counter+1, so B's event after A's (+30 s) events is greater. An hlc more than 60 s ahead is refused by `mergeClockV1`; `ingest` catches it, reports `clock-refused`, the loop continues.
- Scenario 6: torn tail beyond `head.bytes` is never read (reader limit) and truncated by the next append of the writer; a relay copies only bytes up to `head.bytes`; re-publish after a crash between append and head cannot duplicate (truncate then rewrite). A lost state save only causes a benign duplicate put of identical content (after fix 1 never a stale override).
- Scenario 7: segments start at 0, initial cursor `{0,0}`, `start.bytes>=head.bytes` guard, sealed segments read to EOF (limit `Infinity`) and the cursor moves to the next segment; head.json is replaced atomically.
- Scenario 12 (matcher), hand-traced against the real `.gitignore`: `target*` (wild, `[^/]*`), `[Bb]in/` (class, dir only, parent rule hides everything below), `*.[1-9]` (class + range), `**/🤖️generated/` and `**/🤖️generated.*` (`(?:.*/)?`), `/.semio/` (anchored, dir only), `**/.nx/`, `.cache` (any depth, files and dirs), `**/🕸️bindings/**` (contents only), `**.vscode-test` (`**` not slash-delimited behaves as `*`), `Generated\ Files/` (escaped space), `*.synctex(busy)`; `!**/🔖️*/` and `!**/🔖️*/**` re-include `🏅️standards/🔖️1.2/…` after the broader `*.[1-9]` family and a later rule can re-exclude, as in git. Flags are `su`; every non-alphanumeric ASCII literal is emitted as `\u{..}`; astral/emoji characters are literal. Note: the repo `.gitignore` does not contain a bare `*.[1-9]` line (only `*.[1-9]R`, `*.t[1-9]`, ...), the trace used it as a hypothetical.
- Scenario 9 other points: all hashing/copying streams (`hashFileV1`, `copyFile`), pools are bounded (8 and 16), no sync fs in the loop, state is rewritten once per `persistMilliseconds`, not per file.
- Scenario 11: store paths use forward slashes, every fs call goes through `join(...parts)`; unlink/rename retry with backoff on EBUSY/EPERM/EACCES; executable bit skipped on win32; symlink creation failure becomes `skipped`.

## Left as is (design limits, need a decision)

1. A received event more than 60 s ahead is refused permanently: the cursor moves past it (byte cursors cannot hold one event back), so that file change never reaches this machine unless the origin republishes. Holding the stream until the wall clock catches up needs per-event offsets.
2. `state.json` is rewritten whole every 5 s while dirty: ~50-60 MB JSON at 270k files, a 0.3-1 s event-loop stall. Raise `persistMilliseconds` or shard the state if measured to hurt; crash recovery is now safe with a stale state (fix 1).
3. Coarse-mtime file systems (FAT, some SMB): a same-size edit within one mtime tick of the recorded observation is not detected by the size+mtime gate until size/mtime changes. Fine on APFS, NTFS, ext4, which is the supported local disk.
4. Status `rejoinRequired` is still computed from missing segments although the daemon now recovers automatically from a snapshot.
5. A refusal by `.gitignore` (path ignored on the receiver) keeps the entry in the in-memory manifest; a later ignore change re-queues them, which can emit many `refused` lines once.

## Unverifiable without running

All of it was hand-traced only: TypeScript typing of the edits (e.g. the synthetic-event cast in `foldStoreV1`), `bun:test` behaviour of the two new tests, real rename/mtime semantics on NTFS, NFS attribute caching with the sealed-segment fix, `fs.watch` recursive availability on Linux Bun, and the Python oracle comparison after relay snapshot copies (oracle ignores `snapshots/`, unchecked).

## Round 2: the three design limits ("Left as is" items 1-3)

Hand-traced only, nothing executed. Files: `📒️journal/🟦️.ts`, `🔁️replica/🟦️.ts`, `🔁️replica/🧬️schema/🔣️.json`, `🔁️replica/🧪️tests/🟦️.ts`.

### 1. Future-dated events are held, not lost

- Journal: new `readEventSpansV1(store,origin,after)` returns `{events,ends,cursor}`; `ends[i]` is the byte cursor just behind `events[i]` (segment + `from+newline+1`). `readEventsV1` is now a thin wrapper returning only `{events,cursor}`, so every existing caller and test is unchanged.
- Replica `followOnceV1`: new `admit` cuts the batch at the first event whose hlc is more than 60 s ahead of `settings.now()`. Events before it are ingested (so the local clock is merged only with accepted hlcs), the cursor becomes `ends[stop-1]` (or stays untouched when the first event is the late one), the other origins are unaffected. `clock-skew` (`origin`, `hlc`, `path`, `seconds` = ceil of the skew) is reported once per origin per outage (`noted` key `skew:<origin>`, cleared when the origin is read without a late event). The in-memory `held` map (origin -> wall ms at which the event becomes acceptable) lets later polls skip the origin without re-reading its segment; once the wall clock reaches it, the origin is read again from the held cursor and the event is applied. Wall time comes from `settings.now` (`ReplicaOptionsV1.now`, already injectable).
- `foldStoreV1` had the same defect (filtered future events but moved the cursor past them, so snapshots and `compactV1` could drop them for ever). It now uses the same `admit`: fold stops at the late event, the cursor stays in front of it, `clock-skew` goes to the sink.
- The `clock-refused` path in `ingest` stays as a defensive net but can no longer trigger for follow batches.
- Schema: `clock-skew` added to `EventName`, optional `seconds` added to `ProgressEvent`.

### 2. State persistence: checkpoint + append-only delta log

- `state.json` = last full checkpoint, now with a required `generation` (integer). `state.log` (`LocalPathsV1.log`) starts with a header line `{"generation":g}` followed by one line `{"kind":"files"|"queue","path":..., "entry":<StateEntry|QueueEntry|null>}` per changed entry (`null` = removed). Schema definition `StateLogLine` (also in the root `oneOf`), `State` gains `generation`.
- Change tracking: every mutation of `files` goes through `setFile`/`forget`, every mutation of `queue` through the new `enqueue`/`dequeue`; both record the key in `replica.changed` (`f<path>` / `q<path>`). `persist` takes the keys, appends the current value of each (deduplicated, a batch per persist) only when there are changes, at the existing `persistMilliseconds` cadence. Order is unchanged: clock, state (log append or checkpoint), cursor.
- Open: `loadLocal` reads the checkpoint, then `replayLog` replays the log. Bytes after the last newline are a torn tail and are ignored (and truncated by `openReplicaV1` before anything is appended); a complete line that does not parse or has a wrong shape throws `Closed replica state log required: line n`. A log whose header generation differs from the checkpoint's is stale and is discarded (truncated to 0). `statusV1`/`verifyV1` use `loadLocal` and therefore see the replayed state.
- Rollover: `checkpoint()` writes `state.json` (tmp + rename) with `generation+1`, then truncates the log. A crash between the rename and the truncate leaves a log with the old generation: ignored on open, so an old log can never regress entries that the new checkpoint already holds. A checkpoint is written when the log exceeds 16 MiB (right after the append that crossed it, or at the next persist) and on clean shutdown (`closeReplicaV1` -> `persist(replica,true,true)`, only if there were changes or a non-empty log). A failed persist forces the next one to checkpoint (a partially appended line cannot be trusted).
- Cost at 270k entries: steady state appends only changed lines; the 60 MB rewrite happens at shutdown or after 16 MiB of log. A join of the whole tree writes its first batch to the log and checkpoints immediately because the batch exceeds 16 MiB.

### 3. Racy timestamps

- `StateEntry` gains optional `seenMs` (wall time at which the content was observed; schema `StateEntry.seenMs`). It is taken BEFORE hashing (publish: before the chunk is prepared; materialise: at its start; join adoption: before the scan), so it can only be earlier than the real observation, which is the safe side.
- `racy(entry)` = no `seenMs`, or `seenMs - mtimeMs < 2000`. Racy entries are never trusted by size+mtime: in `publishOnceV1` the skip gate and in `differs` (used before overwriting a local file) they fall through to a rehash. An unchanged racy file is rehashed once and recorded as a `touch` with a fresh `seenMs`, after which it is trusted again (self-healing, like git's racy index entries).
- Consequences: every file just materialised from the store (mtime = now) is rehashed once by the next scan; a join of the whole tree therefore costs one extra full read at the first daemon scan. A file with an mtime in the future is rehashed on every scan. Fresh local saves cost one extra rehash within `scanSeconds`.

### Tests (`🔁️replica/🧪️tests/🟦️.ts`)

- Added: future-dated event held then applied with an injected fake wall clock (partial batch, cursor bytes, other origin proceeds, one report, clock not merged, fold stops too); state log (line shape against the schema, no checkpoint rewrite while appending, replay after a simulated crash, torn tail, stale generation, replay of a valid removal, rollover above 16 MiB); racy same-size edit detected while a safely old entry is still trusted by size+mtime.
- Changed: the restart test no longer asserts `touched:0` (a freshly materialised file is racy and is rehashed once).

### Interface changes other modules must know

- `ReplicaV1` has new fields `changed`, `held`, `generation`, `logBytes`; `LocalPathsV1` has `log`; `ReplicaEventV1` has `"clock-skew"` and `seconds`; `requireReplicaShapeV1` accepts `"StateLogLine"`. Journal: new export `readEventSpansV1`.
- `state.json` now requires `generation`; there is no reader for the previous shape (never deployed).

### Still unverified

Everything: TypeScript typing of the edits, Ajv strict-mode acceptance of the `StateLogLine` oneOf, timing of the 100k-line rollover test, `appendFile`/`truncate` behaviour on Windows, and the effect of the racy rule on a real first scan after a large join. No journal test was added for `readEventSpansV1` (covered through the replica tests).

## Round 3: integration

Hand-traced only, nothing executed. Scope: the seams between the three concurrent edits (correctness reviewer: `🔁️replica/🟦️.ts`, `📒️journal/🟦️.ts`; round 2: state log, `seenMs`, held future events, `readEventSpansV1`; bootstrap agent: `🔁️replica/🥾️bootstrap/🟦️.ts`, `🔁️replica/⌨️command/🟦️.ts`, `🗺️topology`). Line numbers are those after the edits.

### Seams checked and found consistent (no change)

- Bootstrap archive and settle never read or write `state.json`, `state.log`, `cursor.json` or `clock.json`; they read only `machine.json` (checked against the `Machine` schema definition) and the folded store. Nothing outside `🔁️replica/🟦️.ts` builds a `ReplicaV1`, `LocalPathsV1` or a state entry by hand (tests go through `openReplicaV1`).
- Every import of the bootstrap and command files exists with the current signature and return shape: `foldStoreV1(store,signal,sink,now)` returns `{manifest,cursors,hlc}`; `storeReachableV1`, `requireReplicaShapeV1("Machine"|"VerifyReport")`, `joinV1/seedV1/runV1/statusV1(options,deep,signal)/verifyV1/snapshotV1/relayOnceV1/relayV1/homedOriginsV1` and the result fields the command prints (`published`, `files`, `adopted`, `queued`, `entries`, `created`, `segments`, `blobs`, `snapshots`, `bytes`, `ok`, `checked`, `total`, `drift`, `daemon`, `queue`, `lost`, `origins[]`).
- Event names and fields: all 22 event names and every field the code emits (`stage`, `seconds`, `bytes`, `groupId`, `origin` ...) exist in `ProgressEvent`/`EventName` of `🔁️replica/🧬️schema/🔣️.json`; the `StateLogLine` shape matches `appendLog`/`replayLog`. Text keys: every key the command uses (including the dynamic `error.bootstrap.<key>`, `verify.kind.<kind>`, `@status.*`, `@snapshot.*`) exists with `en` and `de` and the placeholders match the variables passed. The `compact.done` text is unchanged (blob count 0 without `--collect-blobs`).
- Unused/duplicate imports, arity and `readonly` writes in the files touched by several agents: none found (strict TypeScript only; the repo has no `exactOptionalPropertyTypes`/`noUnused*`). No floating promise in `runV1`, `followOnceV1`, `refreshBootstrapV1`, `relayV1`; the two long-running promises of `relay` run under `Promise.all` and both catch everything except the abort.
- New-machine flow (`🗺️topology/🥾️provision/🐚️.sh`): empty disk -> archive verified and extracted -> native bootstrap -> `bootstrap-settle` -> `join` -> `run`. Converges as required: a file newer in the store differs by hash, is moved to `lost/bootstrap-<ms>/`, `join` enqueues and materialises it (no local file, hence no `lost/` copy and no overwrite of newer store content); a file deleted since is moved aside too, the folded manifest holds the tombstone and `join` records it (`present` no longer contains the path); matching files are adopted with `seenMs` taken before the join scan, so extracted files with an older mtime are trusted and the others are rehashed once (racy rule) and recorded as `touch`, never as an event; `run` follows once before its first publish (`recovering`), the cursors come from the fold. Links are not in the tar; `join` creates them.
- First-machine flow: `seed` creates the store directories, `openReplicaV1` writes `machine.json`, `ensureHome` runs before the first append, home marker and relay `homedOriginsV1` agree. Bootstrap refresh works before any snapshot exists (`foldStoreV1` replays the journal).

### Defects found and fixed

| # | Area | Where | Defect | Fix |
|---|---|---|---|---|
| 1 | server flow | `🗺️topology/🥾️provision/🐚️.sh` step 4 (line 182) | On the group server the operator applies the `fstab` part of the server plan (a bind mount of the local export at the store mount). Step 4 then found a different `/etc/fstab` entry for the store mount and failed ("already has a different entry"): neither server nor first machine could be provisioned. | `--server` skips the NFS client entry, requires the bind entry to exist (clear message otherwise) and mounts it. |
| 2 | first machine | same script, header and flag check | `--first-machine` and `--server` were mutually exclusive, but the first machine IS the g1 server: it was provisioned as a worker (NFS entry, no server role). | Both may be combined (`--first-machine --server`); the runbook command for g1 is updated. The relay round is only run for a later server. |
| 3 | server flow | same script, step 2 (line 145) | A server without `--archive` tried to NFS-mount its own export loopback into a temp directory. | A server must pass `--archive <peer mount>/v1/bootstrap`; clear message. |
| 4 | blob GC (decision 4) | `🔁️replica/🟦️.ts:866,887` `compactV1` | Every origin collected blobs. | New last parameter `options:{collectBlobs?:boolean}` (default off): without it only own sealed segments and old own snapshots are removed and the blob scan, second fold and unlink loop are skipped. |
| 5 | blob GC | `🔁️replica/⌨️command/🟦️.ts:30,191` | No switch. | `compact --collect-blobs` (plain flag), passed as `collectBlobs`. |
| 6 | blob GC | `🗺️topology/🟦️.ts:194-205`, `🗺️topology/⌨️command/🟦️.ts:10,69,74`, provision script line 224 | The only compaction unit/timer was the worker one. | `workerPartsV1(..., server=true)` (Linux only, throws otherwise) renders `compact --collect-blobs`; `worker-service --role worker\|server`; `provision.sh --server` installs the server variant, workers and the macOS/Windows jobs never pass it. |
| 7 | server flow | `🗺️topology/🟦️.ts` relay unit (line 156) | `RequiresMountsFor` listed the peer NFS mounts. systemd turns that into `Requires=`: a peer server that is down at boot (the fstab lines are `nofail`) made the relay unit fail to start for good, although the relay itself handles unreachable peers. | `RequiresMountsFor` names only the own store mount; `After=remote-fs.target` stays. |
| 8 | bootstrap | `🔁️replica/🥾️bootstrap/🟦️.ts:196` | Settle emitted its `progress` event only for moved files (the `continue` skipped it), i.e. almost never on a mostly matching 270k-file tree. | The progress check runs for every entry. |
| 9 | bootstrap | same file, `matches` (line 171) | A file whose executable bit differs from the store was adopted by `join` with the store's bit in the state; the daemon's first scan then saw a different bit and published a chmod as a local edit. | On macOS/Linux `matches` also requires the same executable bit; a differing file is moved aside and `join` re-creates it with the store's bit. |
| 10 | bootstrap | same file, `writeBootstrapArchiveV1` (line 123) and `refreshBootstrapV1` (line 161) | A killed build left its staged tar (about 5 GB) in `<store>/v1/tmp` for ever; a failed build (for example a store that was still empty when the relay started) was retried only after `bootstrapMinutes` (60). | Staging files `<origin>-bootstrap-*` older than six hours are removed at the start of a build; a failed build is retried after at most two minutes. |
| 11 | seed (270k files) | `🔁️replica/🟦️.ts:450` `publishOnceV1` | State was persisted only at the end of the round: a seed or first publish interrupted after hours left `machine.json` and the store events but no state, and the next run republished everything. | `persist(replica,false)` after every chunk of 256 (throttled by `persistMilliseconds`, appends only the delta lines). |
| 12 | docs | module `README.md`, repo `README.md` | Still described compaction as collecting blobs on every origin ("known gap"), clock skew as "refused", the racy rule/bootstrap convergence not at all, `state.log` missing from the `.cache/f` list, several command rows without their real flags. | Updated: compact/`--collect-blobs`, `worker-service --role`, `--first-machine --server`, clock skew held and applied later, convergence of an extracted tree, flags of `status`, `snapshot`, `compact`, `verify`. |

Tests/fixtures: `🔁️replica/🧪️tests/🟦️.ts` compaction test now asserts that the default run removes no blob and the `collectBlobs` run does (blob files counted on disk); `🔁️replica/🥾️bootstrap/🧪️tests/🟦️.ts` gained the executable-bit settle test (also progress event and join without republished chmod); `🗺️topology/🧪️tests/🟦️.ts` gained `--role`, the server compaction text, the relay mount line, the `compact --collect-blobs` argument parse and the `--role` use in the script. No fixture JSON needed changes.

### Left as is

- `provision.sh` skips `join` when `machine.json` exists, also when an earlier join was refused or died before it persisted anything (the daemon then resumes from the journal and keeps overwritten local bytes in `lost/`); a rerun after a refused join needs `machine.json` removed by hand.
- `settle` and `join` hash sequentially (one 5 GB pass each); correct, not fast.
- The PowerShell script has no server role (servers are Linux); it is unchanged.
- `status --deep` counts a racy entry as unchanged when size and mtime agree.

### Must be executed (nothing here was)

From the checkout root (the root `bunfig.toml` preloads the runtime pin; a run from the test's own directory misreports), with Bun and Python 3 on `PATH` (the tests call the `🐍️.py` oracles with `python -I` through `testPythonV1`, and skip them without Python):

```
M=🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree
bun test ./$M/📒️journal/🧪️tests/🟦️.ts ./$M/🚧️boundary/🧪️tests/🟦️.ts ./$M/🔁️replica/🧪️tests/🟦️.ts ./$M/🔁️replica/🥾️bootstrap/🧪️tests/🟦️.ts ./$M/🗺️topology/🧪️tests/🟦️.ts
```

Then by hand: `bash -n` and `shellcheck` on `$M/🗺️topology/🥾️provision/🐚️.sh`, and the rehearsal of day 0 with one group of two machines from the module runbook (a real `tar -xf` of a real archive, `bootstrap-settle`, `join`, `verify`; the relay unit with a peer mount that is down; `compact --collect-blobs` on the server only).

## Round 4: executed

Run with Bun 1.4.2 and Python 3.14.4 on `PATH`, from the checkout root. The replica, topology, bootstrap and journal suites called their `🐍️.py` oracles through `testPythonV1`; the boundary test never spawned its oracle, so that one was run by hand.

| Suite | Result of the last run |
|---|---|
| `📒️journal` | 120 pass, 0 fail |
| `🚧️boundary` | 31 pass, 0 fail (oracle by hand: `201 vectors checked, 0 mismatches`, with `pathspec` 1.1.1 from a scratch `--target` directory) |
| `🗺️topology` | 14 pass, 0 fail |
| `🔁️replica` | 35 pass, 0 fail (28 earlier tests, 7 new robustness tests) |
| `🔁️replica/🥾️bootstrap` | 11 pass, 0 fail |

Type-check: `tsc` 5.9.3 (`@types/bun` and `bun-types` 1.4.3, `@types/node` 22.20.5, the locked versions) over the 14 `.ts` files of the module with the repo `tsconfig.json`: 4 errors found in the module's tests, all fixed, 0 left. Six errors remain in `🧬️schema/📄️source/🔗️closure/🟦️.ts`, which the module does not own; they come from workspace packages (`@semio-tech/pack-json-decode-ts`, `@semio-tech/framework-pack-json-ts`) that are not linked because `bun install` was not run.

### Defects fixed

| # | Where | Cause | Fix |
|---|---|---|---|
| 1 | replica `publishOnceV1` | A local delete was published and followed correctly, but the publisher kept the now empty directories (`deep/dir`, `deep`) while every follower pruned them: trees diverged in their directories, contrary to "removed when empty after a delete". The delete event itself was published and applied; nothing was resurrected. | After a chunk of delete events is appended, the emptied parent directories are pruned with the same `prune` the follower uses (a directory that still holds anything is left alone). |
| 2 | boundary oracle `🐍️.py` | With `pathspec` 1.1.1 a trailing `/**` also matches the directory itself when it is tested as `dir/` (`x/**/y/**` matched `x/b/y/`). Git does not: the fixture expecting `a/🧫️fixtures/b/🤖️generated` (directory) as not ignored is the correct side. | `ignored` now evaluates the patterns itself, last match wins, and tests a pattern ending in `/**` against the directory path without the trailing slash. The fixture is unchanged. |
| 3 | tests, type-check | Four type errors in `🔁️replica/🧪️tests/🟦️.ts` (fold manifest comparison, `lines[0]` log line) and `🗺️topology/🧪️tests/🟦️.ts` (peers cast, operation compared with a string). | Casts at the call sites; no assertion changed. |

### New robustness tests (`🔁️replica/🧪️tests/🟦️.ts`)

A seeded PRNG drives 40 random edits or deletes per machine over the same 30 paths, interleaved with random publish and follow rounds (and relay rounds, and kills). Then rounds run until a round does nothing; the test requires convergence, then one more publish round that must publish and delete nothing, a follow round that applies nothing, every tree byte-identical to the folded store manifest (blobs read from the store), and `verify` without drift.

- 5 machines on one store: seeds 1, 2, 3.
- 5 machines on two stores (3 in `g1`, 2 in `g2`) with relays in both directions: seeds 11, 12; both stores must fold to the same tree.
- Kill and reopen in the middle (`replica.release()` without `closeReplicaV1`, the way the existing crash test does, then `openReplicaV1`): seed 21 on one store, seed 22 on two stores.

The runs produce real conflicts (2 to 10 files in `lost/` per machine), so the loser path is exercised.

### Flakiness

- Every suite run 5 times in a row: identical counts each time, no failure.
- A one-off sweep with 25 seeds per scenario (75 chaos runs, not committed): all pass when run alone. Four copies of the whole replica file in parallel on this machine passed all 103 tests that ran and failed only the `afterAll` cleanup hook (default 5 s limit, 8 s needed to delete the temp trees of 100+ tests under 4x load); that is test housekeeping, not a sync race, and does not occur at the committed size.

### Not done

- The boundary oracle is not spawned by any TypeScript test, so `pathspec` validation only runs manually (`python -I` ignores `PYTHONPATH`; a wrapper that inserts the target directory into `sys.path` and `runpy.run_path`s the oracle was used).
- The `bash -n` and `shellcheck` check of `provision.sh` and the day-0 rehearsal were not part of this round.

## Round 5: memory budget and offline edits

Two open problems of the real-tree rehearsal (`rehearsal-report.md`). All numbers: the real tree (269,331 shared files, 122k directories, 4.5 GiB) cloned with `cp -Rc` under the scratchpad, Bun 1.4.2, macOS, `/usr/bin/time -l` and `idle-sample.py`; clones and stores were deleted afterwards, the repository itself never got a `.cache/f/`.

### 1. Memory

Before and after, same host, same tree (peak = `maximum resident set size`):

| Measure | Before | After |
|---|---|---|
| Idle `run` daemon (120 s window) | 1,261 MiB, 1.7 % CPU | **238 MiB**, 1.2 % CPU (234 to 287 MiB over six runs) |
| `seed` | 1,695 MiB, 54.5 s | **480 MiB**, 46.8 s |
| `verify` | 2,184 MiB, 21.9 s | **402 MiB**, 19.6 s |
| `snapshot` | 1,090 MiB | 464 MiB |
| `join` adopting a full clone (snapshot) | 1,929 MiB, 47.9 s | **479 MiB**, 18.9 s |
| `join` into an empty directory (journal fold) | 2.75 GB (rehearsal) | **490 MiB**, 33.3 s |

What changed (profiled with `process.memoryUsage()` and RSS prints at each step of `openReplicaV1`):

- **One id per path** (`ReplicaBookV1`). The three path-keyed maps (`files`, `manifest`, `lower`) held about 270k long UTF-16 strings and one object each. Now the UTF-8 of every path lives once in 1 MiB pages behind a hash table of ids, and `files`, `manifest` and the queue are typed columns by id (`CompactTableV1`: hlc as milliseconds + counter + origin index, sha256 as 32 raw bytes, sizes and times as numbers). `get`/`set`/`has`/iteration behave like a `Map`, so the loops and tests kept their shape. `lower` is gone: `ReplicaFoldsV1` finds a case twin through a hash of the folded name that the book stores when it first sees a path. The queue is one byte per path; its entry is the manifest row.
- **Nothing the size of the tree is a string or an array any more.** `state.json` is written entry by entry and read through the new `streamMembersV1` (same JSON document, any formatting); snapshots likewise (`writeSnapshotV1` streams in sorted order, `readNewestSnapshotV1` streams into a `create()` target); the log is appended in 1 MiB pieces and a persist with more than 50,000 changes becomes a checkpoint; `foldStoreV1` reads the journal one segment at a time (`readEventSpansV1(…, segments)`); the relay streams the snapshot for its blob list and copies the file instead of re-writing a 90 MB string; `compact` reads the snapshot's cursors by streaming.
- **Scans do not materialise the tree.** `scanSharedV1(…, visit)` hands entries over and keeps none, walks depth first (the pending directories stay few; the breadth-first level array was about 20 MB), and refuses only the leaf name instead of re-validating every ancestor (`refuseLeaf`). The ignore matcher decides children without walking or caching their ancestors (`child`) and its caches are capped at 20,000 instead of 400,000 entries. `walkLocalV1` keeps only entries that differ from the state, as columns (`ReplicaFoundV1`), plus one bit per known path; `verify` compares each file when it is hashed and `join` adopts through a compact side table. Join and verify hash with the configured concurrency (join was sequential: 47.9 s to 18.9 s).
- **Collect on purpose** (`Bun.gc(true)` every 2,000 entries of a scan, load or verify and every few publish rounds, and after a large typed array was replaced), because the runtime hands freed pages back to the OS only slowly. Not `--smol`.
- Measured on the way: the first path table used one byte arena that doubled; the dead generations stayed resident (seed 1,187 MiB, idle 564 MiB), so it became fixed 1 MiB pages. Collecting every 2,000 entries brought the idle daemon from 537 to 374 MiB at that point and did nothing for the open itself; the remaining open-time garbage came from per-entry allocations (`Buffer.from(hex)`, `subarray`, generator results, one closure per fold insert), now loops over typed arrays. Reading the state in 16 KiB chunks instead of 1 MiB saved about 25 MiB of high-water. Reserving the columns from the size of `state.json` saved about 15 MiB.

Interfaces that changed (greenfield, callers and tests updated): `foldEventsV1` takes any `{get,set}` target; `writeSnapshotV1` takes `{size,keys(),get()}`; `readNewestSnapshotV1(store, origin?, create?)`; `readEventSpansV1(…, segments?)`; new `streamMembersV1`; `scanSharedV1(…, visit?)` and `FleetIgnoreV1.child?`; `foldStoreV1().manifest` is a `CompactTableV1`; `publishOnceV1`'s fourth argument is a `WalkV1` from `walkLocalV1`; `ReplicaV1` has `book`, `folds`, a compact `files`/`manifest`/`queue` and a numeric `changed` set. On disk nothing changed: `state.json`, `state.log` and the snapshots are the same JSON documents as before.

### 2. Offline edits that are newer than the remote event

Before: a follower that met a remote event for a path with an unpublished local change always moved the local version to `lost/` and applied the remote one.

Now (`materialise` and `followOnceV1`): the path is dirty when the local file differs from `state` and from the incoming content (a local delete is an edit with no time). Then `conflictWinnerV1(localMtimeMs, remoteHlc)`: the local edit wins if and only if its mtime is strictly later than the wall time inside the remote hlc. A local win leaves the file alone, sets `state` to the remote entry with `mtimeMs` 0, reports `kept` and publishes the path at once (`base` = the remote sha256, new hlc greater than everything seen, so it wins everywhere). Otherwise the old path: `overwritten`, loser in `lost/`. A local delete is treated as older than any remote put, so the file comes back and nothing is published. A remote delete against a dirty local edit follows the same mtime rule. `FollowResultV1` gained `kept`; the event name `kept` is in the schema.

Why it is deterministic and loop-free: the verdict is a function of two numbers; every outcome produces a clean path (state = applied or published hlc); a new verdict needs a new local edit whose mtime is later than the wall time of the newest event it meets; the kept edit is an ordinary new event, so the plain hlc order makes all machines converge. The chaos tests (random edits, publishes, follows, relays and kills) still pass unchanged.

The honest limit is stated in the contract and the README: the wall time in an hlc is the time of publication. Writes within the publish latency on different machines are ordered by publish time (the rehearsal measured winners A, B, B for writes 5 to 8 ms apart), an offline edit is ordered by when it is published, and when both machines were offline the first to publish wins unless the other's mtime is later than that publish time. The comparison puts one machine's file clock against another's hlc clock, so it is as good as NTP.

Tests added (all executed): fixture scenarios (run in TypeScript and checked by the Python oracle `compare` on every machine) for offline-newer wins, offline-older goes to lost, local delete against remote put (nothing published), newer local edit against remote delete, older local edit against remote delete, both machines offline (first publisher wins, loser in lost) and both offline with newer edits (converge on the last publisher); conflict vectors in the fixture checked by `conflictWinnerV1` and by `🐍️.py conflicts`; tests for the kept event, the greater hlc, silence of further rounds, a restarted daemon (`runV1`) that publishes the offline edit, the compact tables, the path book (12,000 unicode paths across pages, lone surrogate refused), `streamMembersV1` (three formattings, chunk sizes 7, 4096 and 1 MiB, malformed documents), segment-at-a-time reads and the scan visitor. Live on the real tree with two daemons, the three cases (older, newer, local delete) came out as designed (`rehearsal-report.md`, section 5).

### Verification

Suites (executed): replica 48 tests (35 before), three consecutive runs; journal 122 (2 added); boundary 32 (1 added); topology 22 (the brief said 14; I did not touch that module); bootstrap 11; all pass. The Python oracle ran inside the replica suite.

### Not verified / still open

- Idle RSS varies by about 15 % between runs (234 to 287 MiB); two live daemons showed 290 and 297 MiB under the code of an earlier iteration of this round during edits. The 300 MB target is met, with a small margin on the worst run.
- The first full scan after start still adds about 25 MiB; the periodic scan every 900 s does not grow it further in a 2 min window, a day-long run was not made.
- Linux and Windows memory, the recursive watch on Linux, NFS attribute caching and the hlc/mtime comparison across machines with skewed clocks were not exercised. The live conflict check ran two daemons on one host, so both clocks were the same clock.
- `bootstrap-archive` still materialises its sorted file list (`bootstrap/🟦️.ts`); its memory was not re-measured.

## Round 6: open items of the three verification runs

Suites at the end, three consecutive rounds, all green: journal 126 (was 124), boundary 38, topology 61 (another agent changed it meanwhile), replica 72 (was 58; `🏁️race.ts` now holds 14 injected-fault tests), bootstrap 11. Host repo has no `.cache`; rehearsal data deleted; every process started was stopped by its recorded PID.

1. **Held-open files on Windows (EBUSY/EPERM/EACCES).** Tests inject the errors into `unlink`, `rename`, `copyFile`, `stat` and the journal's `head.json` rename (`node:fs/promises` is mocked in `🏁️race.ts`). Fixes: `applyQueue` reports any sharing error as `blocked` with its code (it was an `error`) and keeps the item queued; a per-path back-off (0.5 s doubling to 30 s) stops a blocked file from costing 1.2 s of retries on every follow round; `publishOnceV1` no longer throws when the journal write is blocked after its retries, it returns the paths for retry and leaves state and cursor unchanged. The incoming event is never lost: it is in the persisted queue when the cursor passes it (test closes and reopens the replica). Symlink without privilege: contract statement added (`skipped`/`symlink-unsupported`, dropped, nothing published, `verify` lists `missing-local`) and tested with an injected EPERM. Windows itself was not run; the errors are injected.
2. **Stale temp files after `kill -9`.** Local `.cache/f/tmp` was already emptied at open (existing test). Store `tmp`: staging names now carry the pid (`<origin>-<pid>-<hex>`); `reclaimStagingV1` removes at `openReplicaV1` (lock held) the own origin's files whose process is dead (a live `compact` of the same origin is spared) and legacy names; `compactV1` removes every file in `v1/tmp` older than 6 hours (relay and other origins too, `.nfs*` skipped). Tested with a real `kill -9`ed child pid, and by age with an injected clock.
3. **Long paths.** The scan already listed policy `too-long`, but a file event (watcher) for a refused path was dropped silently, `status` and `verify` did not report refusals at all, and an OS `ENAMETOOLONG` aborted the walk. Now: `refused` event on scan and on touch, `skipped`/`too-long` when the OS refuses (scan lists it as refused `too-long`), `status --deep` counts (`refused`), `verify` lists (`refused`, `refusedTotal`, not drift); schema, texts (en/de) and README updated. I used `refused` for policy and `skipped` for what only the OS refuses.
4. **`compact --collect-blobs` streaming.** Per-directory `opendir`, at most 2,048 doomed blobs per worker in memory, references published since the fold are added before each deletion batch (replaces the second full fold), progress with `count`/`bytes`, cancellation between files. Tests: after 100 deletions the scan had stat-ed at most 2,256 of 5,000 blobs; a blob referenced after the scan began survives; progress counts. Stale-staging removal is reported as `staging` in the result.
5. **A7 degradation.** Not a code regression. Reduced reruns (20 replicas, 2 groups, same tree; table in `scale-report.md` section D): final code 80 ops/s achieved, join 7.5 s, 1 s quiescence; without the forced GC or with relay concurrency 1 the same (join 6.2 / 6.6 s, latency within noise, CPU 11.6 to 11.7 %); only 24 busy loops on the host change the picture (join 15.3 s, in-group whole-run p95 3.3 s against 1.2 to 1.4 s). A7's 510 s therefore came from host contention beyond what busy loops reproduce (VMs, shared disk); this was not isolated further. Both changes kept; GC lowers max RSS (313 vs 432 MiB).
6. **Relay speed-up end to end.** `relay --concurrency N` added (default 8). Catch-up of 20 origins after 60 s at 80 ops/s: 3.18 and 3.39 s with 1, 0.98 and 0.89 s with 8 (x3.5, same as the microbenchmark). In steady load the cross-group median (about 450 ms) is the same for 1 and 8.

Still open: Windows real runs; the A7 host cause stays a diagnosis by elimination; `status` without `--deep` cannot count refusals (null); a symlink skipped for lack of privilege is not retried after the privilege appears (rejoin).
