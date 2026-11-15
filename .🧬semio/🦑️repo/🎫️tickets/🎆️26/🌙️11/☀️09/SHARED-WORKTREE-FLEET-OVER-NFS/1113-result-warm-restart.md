# Result: warm restart and cheap state persistence of the replica daemon (2026-11-13)

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Benchmark: `🧰️scripts/1113-bench-restart.ts` (modes `phases`, `load`, `roundtrip`, `checkpoint`, `e2e`; usage in its docstring). The repo MCP did not connect, so the ticket is not closed. Landed after `1113-result-latency.md` existed, as context-anchored patches onto the live files (three-way merged against the live tree twice before, live files unchanged between the last merge and the patch); every touched file was import-checked, all suites were run against the live tree afterwards. The temporary copy is deleted.

## 1. Where the restart time went (live state of g1-server, host load average 15 to 30)

Measured on a copy of the live `.cache/f` in a temporary worktree (read-only on the live tree; git against a copy of the index and a private object directory), old code:

| Phase | Before |
|---|---|
| `loadLocal` (stream + per-entry `JSON.parse`, compact table build, per-path sha256 for the digest) | 4 719 ms (2.5 s on an idle host) |
| case-fold index (`ReplicaFoldsV1.add` of 275 340 paths) | 719 ms |
| `requeue` | 10 ms |
| own-tail replay | 2 ms |
| **`openReplicaV1` total** | **5 503 ms**, longest event-loop stall 861 ms, RSS 362 MiB afterwards |
| first full walk of the tree (`scanSharedV1`, 272 423 entries) | 22 282 ms, blocking the daemon loop (`await publishOnceV1`) |
| `git rev-parse` / `symbolic-ref` / `write-tree` / `config --list` | 18 / 10+13 / 386 / 11 ms |
| first `git status` (the attestation view) | 16 602 ms (15 655 ms warm, git caches nothing) |
| clean stop (`persist(final)`, full rewrite of 106 MB) | 423 ms |

The first attestation needs the git state and the view; the view was asked only at the first `tick`, which ran after the first loop pass, i.e. after the blocking walk. Sequence before: load 5.5 s, then walk 22 s, then the view 16 s: about 44 s under this load (about 30 s idle, as the audit believed). Follow, apply and hint serving also waited for the walk. Normalising git config, `requeue` and the own tail are small (tens of ms).

## 2. What changed

- **Format** (`🔁️replica/💾️state`, schema `🧬️schema/🔣️.json`, Python oracle `🧪️tests/🐍️.py`, vectors `🧪️tests/🧫️fixtures/🔣️.json`; the JSON `State` and `StateLogLine` definitions are gone from the replica schema, `Machine`, `Cursor`, `Clock`, `Lock`, `SyncSwitch` stay):
  - `state.ckp`: 96-byte header (magic `SEMIOSNP`, format version, generation, counts of paths, live files, queue, number of parts, payload length, sha256 of the payload, an 8-byte check of the header itself, zeros; header check, zeros, length, version and payload hash are all verified, so no byte of the file is unguarded), a directory of named parts (24-byte name, u64 length) and the parts back to back: `meta`, `origins`, `fold` (the manifest digest), the path book (`starts`, `lengths`, `hashes`, `folded`, raw 1 MiB `pages`), the local table columns (`flags`, `ms`, `counter`, `origin`, `bytes`, `sha`, `mtime`, `seen`, link `targets`), the queued manifest rows (`queue.*`), the case-fold index (`slots`, and the two directory counters as sparse (bucket, count) pairs), the directory table of the latency tiers (`dirs.mtime`, `dirs.present`, `dirs.members`) and `memo:<name>` blobs. Everything is little-endian typed-array bytes.
  - `state.wal`: 16-byte header (magic `SEMIOWAL`, version, generation of the checkpoint it extends), then frames `u32 length | u32 ~length | first 8 bytes of sha256(payload) | payload` of binary records (files set/remove, queue set/remove, memo set/remove), at most about 1 MiB per frame.
  - Write: temp file in `.cache/f/tmp`, 4 MiB chunks through `FileHandle.write` (the event loop runs between chunks, the tables are never copied), the payload hash taken on the way, header written last, `fsync`, `rename`; then the log restarts empty. Log append: one `fsync` per persist.
  - Load: each part is read with positional reads straight into its final typed array (capacity headroom of 6 % so the first new paths do not copy), hashed on the way; no per-entry parse. The manifest table is rebuilt from the local table and the queued rows with typed-array copies. The per-path term hashes of the digest, the case-fold index and the directory counters are stored, not recomputed.
  - Policy: a checkpoint is written only when the log exceeds `max(8 MiB, checkpoint size / 8)` or more than 50 000 entries changed since the last persist, never by time; a clean stop only appends the pending delta (previously it rewrote 106 MB every time). `rememberV1`/`recallV1` keep named blobs with the state; the daemon keeps there the last working-tree view and the hot set (written at stop).
- **Takeover of the old state**: if `state.ckp` is missing and `state.json` exists, the loader reads it once (the old stream reader plus the old JSON log with its generation rule), then writes `state.ckp` (generation + 1) and removes `state.json` and `state.log`. Verified on the live state: the fingerprint (every local row, queue row and manifest row, digest, shared count, path count) is identical after takeover and after the next reload from the binary files (`roundtrip` mode). No re-join is needed; a daemon that finds both old and new files keeps the new one and removes the old ones.
- **Corruption**: a checkpoint that is truncated, has a flipped byte, a wrong header, or is missing while the log names a generation, a log of a newer generation than the checkpoint, or a frame failing its checksum before the end of the log (not a torn tail) makes `openReplicaV1` say `state-rebuilt` (new event name in the schema), start from empty tables, fold the store (newest snapshot plus journal), take the cursors and clock from the fold and queue every winner, then write a fresh checkpoint. The follow then adopts every identical file, keeps newer local edits and moves older ones to `lost/` (the usual conflict rule). If the fold fails (store unreachable) the tables stay empty and the normal follow rebuilds them. A torn log tail is cut off at the next start; a log of an older generation is stale and ignored.
- **Start path** (`runV1`, applied on top of the latency tiers): the first full sweep is no longer a blocking `publishOnceV1`; it runs as the background walk every later sweep uses (cancellable by the signal, result published by the loop). The attester starts (code digest and git state) in parallel with the first follow round and the first tick waits only for that. `normaliseGitConfigV1` runs after the first round. The attestation view is seeded from the memo `view` with `stale: true` (optional field of the attestation `View`; a stale view is exempt from the `view.digest` agreement like a missing one) and replaced when the real `git status` returns; `view.txt` in the store already matches the persisted digest, so it is not rewritten. The hot set of the previous run is restored (so edits made during a self-restart are re-checked within a second) and the directory table is persisted (`DirectoryTableV1.columns/adopt` added to `🎣️tiers`), so the directory sweep starts warm.
- Memory: see section 4. README: new section "Local state and warm restart".

## 3. Timings after

| Measure | Before | After |
|---|---|---|
| `openReplicaV1` on the live state (loaded host) | 5 503 ms | **102 ms** (first run in the process; 78 to 104 ms over three; longest event-loop stall 20 ms, was 861 ms) |
| state file | 106 MB JSON | 72.7 MiB `state.ckp` |
| clean stop | 423 ms, rewrites 106 MB | about 0 ms (appends the delta) |
| checkpoint write, 275 000 entries | about 0.4 s stringify + write, blocking in 1 MB steps | **147 ms, longest event-loop stall 7 ms** |
| blocking `writeFileSync` of the same 75 MiB file | n/a | 79 ms, all of it blocking; a worker thread needs a second 75 MiB copy and still stalls 6 ms: the chunked write is the cheapest |
| 272 000 synthetic entries (test) | n/a | write 140 to 151 ms, load 77 to 157 ms (36 MiB) |
| first full walk | blocks follow, apply, publish for 22 s | 15 to 22 s in the background, nothing waits |
| first attestation after a restart, 40 000-file synthetic tree (real git, `e2e` mode) | 2.0 / 2.4 / 3.0 s (waits for walk and view) | **0.14 / 0.17 s** with the stale view (first start with no persisted view: 1.0 s) |
| first attestation, projected for 272 000 files | about 44 s under load | bounded by load (0.1 s), the code digest and the git state (about 0.4 s `write-tree`): 1 to 2 s |

The `git status` view (16 s) and the walk (15 to 22 s) still cost what they cost; they now run in the background after the first useful round. Nothing was restarted on purpose; the live daemon of g1-server restarted itself on the code change, took over its state (`state.ckp` generation 27) and runs on the new code.

## 4. Memory

Process RSS right after open (after the forced `gc`): **362 MiB before, 174 MiB after** for 275 340 files. What is attributable (sizes from the checkpoint): path pages 46 MiB, path book columns 5 MiB, local table 19 MiB (71 bytes per path), manifest table mirror 17 MiB, case-fold index 4 MiB plus two 4 MiB counters, directory table about 2 MiB, runtime baseline 22 MiB. Removed: the parsed JSON entries kept alive during load (385 MiB RSS delta in the audit), the `widen` copies of every column during growth (arrays are allocated once at their final size plus 6 %), per-entry strings; the case-fold hashing no longer walks every path name at start. Not removed: the manifest table is still a full copy of the local table (17 MiB, would need a sparse overlay across `journal`); the live daemon at 544 MiB RSS is dominated by walks, journal reads and git output rather than by the tables (not profiled further).

## 5. Crash safety tests (`🔁️replica/💾️state/🧪️tests/🟦️.ts`, `🔁️replica/🧪️tests/🟦️.ts`)

- Checkpoint write killed after every single byte `0 .. size-1` (injected `killAt`): the previous checkpoint is byte-identical and loads; the full write succeeds.
- Checkpoint truncated at every length and flipped at every byte position: always `StateCorruptV1`, never a partial read.
- Log append killed at every byte: scan keeps exactly the complete frames, the tail is cut and a following append yields the clean full log.
- Log cut at every byte (clean or torn at the right offset, same records), a flipped byte in every position (corrupt in the middle, torn only in the checksum or payload of the last frame), zero-filled tail (torn), foreign magic (corrupt).
- Replica level: torn tail truncated, stale generation ignored, matching generation replayed (including the case-fold index), log rollover to a new checkpoint above the limit, a stop never rewrites the checkpoint; five corrupted shapes of the checkpoint (cut by one byte, half, 40 bytes, one flipped byte, empty), a log of a newer generation and a missing checkpoint with a log each give exactly one `state-rebuilt`, then no loss, no republish (`published:0`), identical trees, oracle compare exit 0.
- Takeover test with the old JSON state and log (including a torn line).
- Vectors: the Python oracle (struct, hashlib, optional jsonschema) reads the checkpoint of the fixture and reproduces the files, the queue, the memos and the manifest digest, verifies every path hash of the book, and replays the log cases; the TypeScript codec loads the same bytes.

## 6. Test counts (macOS, live tree, after landing)

journal 127, boundary 39, git 36, attest 60 (was 58 plus the other agents' tests, +1 stale view), replica `🟦️.ts` 77 (+13 compared with 64 before the latency and this work together: state, memo, takeover, corruption, restart attestation), `🏁️race.ts` 14, `🌿️git.ts` 19, `🌪️chaos.ts` 9, bootstrap 12, topology 56, new `🔁️replica/💾️state/🧪️tests/🟦️.ts` 8. All pass.

## Not verified

- Linux and Windows not run. File names, `fsync`, rename and positional reads are portable, but nothing was executed there.
- Idle-host timings: all numbers come from a host at load average 15 to 30; the load of the old code on an idle host is the audit's 2.5 s.
- The projected 1 to 2 s first attestation for 272 000 files is derived from the measured parts and from the 40 000-file end-to-end run, not measured end to end on the live tree (no daemon was restarted for it, and the live daemon had already restarted on the new code).
- A kill by power loss (data loss before `fsync`) was not simulated; the injected kills model process death.
- The `view` seed is trusted to match `view.txt` in the store; a machine whose `view.txt` was removed shows as unavailable in `fleet --diff` until the fresh view arrives.
- `🌿️git` `write-tree` is still run at every start (0.4 s); persisting its stamp was not done because that module belongs to another agent.
- Ticket not closed (repo MCP failed to connect).

---

# Addendum 2026-11-13 05:36: re-landing after the overwrite, state precedence, reconcile, guards

## Incident analysis, from the state-format point of view

1. The 05:05 landing took over `state.json` and then **removed** `state.json` and `state.log`. That was the first fault: it made the new format the only copy, so any daemon that did not know the format started from nothing.
2. At 05:12 a whole-file copy of `🔁️replica/🟦️.ts`, `🩺️attest/🟦️.ts` and `🎣️tiers/🟦️.ts` from a 04:56 base removed the checkpoint code. Every daemon restarted into old code, found no `state.json`, took that for a fresh machine and published the whole tree: an empty state is indistinguishable from a new machine in the old loader, and nothing in the old publish path asked the store.
3. Second fault, mine: the loader had no guard. "No usable state although `machine.json` exists" was treated as an empty fresh state, never as an anomaly. The format change made that fatal, because the one valid copy of the state was gone.
4. Third fault: cursors. `cursor.json` was rewritten by my daemon alongside the checkpoint, so after the revert it was newer than any old-format state, and a later old-format daemon would have started with a cursor ahead of its state.
5. Afterwards the machines carried both formats: a complete `state.ckp` of 05:06 and a newer, PARTIAL `state.json` that the old code builds up file by file while it republishes. "Newest file wins" would have chosen the partial one.

## What is different now

- **Precedence** (`loadLocal`): the checkpoint, when it loads, is always the base (it is complete); when old-format files are newer, their rows are folded on top only where their hlc beats the checkpoint's (`foldOlderFormat`), the cursors and clock come from the checkpoint. With only an old-format state it is loaded as before. Both cases, for a joined machine, are followed by `takeOverRegistersV1` (the loop of `repair` that takes over every store register the state lacks or holds older and queues it); the follow adopts them when the disk content matches (size first, hash only for the same size) and nothing is published for them. Then a new checkpoint is written.
- **The old files are never removed or written by the new code** for seven days (`compatWindowMilliseconds`), and `cursor.json` / `clock.json` are no longer written at all: cursors and clock are memos (`cursors`, `clock`) in the checkpoint and the log, appended in the same frame batch after the rows they belong to. A machine that goes old, new, old, new finds, at every old-format start, a `state.json`, `state.log`, `cursor.json` and `clock.json` that are consistent with each other (an older, but whole, world; the old daemon catches up by following from the old cursor and replays its own journal tail).
- **Loader guard**: no usable state while `.cache/f/machine.json` exists is `StateCorruptV1("state missing")`, which takes the `state-rebuilt` path (fold of the store, cursors from the fold, every winner queued). A brand-new replica writes an initial checkpoint at its first open, so that "joined without state" is always an anomaly.
- **Publish invariants** (`publishOnceV1`): (1) a joined replica with an empty state rebuilds before publishing and refuses (`state-missing`, reason `store-fold-failed`) while the store cannot be folded or reached; (2) a walk that finds more than 1 000 paths unknown to the state takes the store fold over first; when the fold holds more than 1 000 of them it reports `state-missing` (reason `republish-storm`) and `repaired`, and those paths are queued instead of published; when the fold cannot be read the publish is refused.
- New event name `state-missing` (schema enum). README section "Local state and warm restart" rewritten for all of this.

## Tests added (all in `🔁️replica/🧪️tests/🟦️.ts`, run against the live tree)

- complete checkpoint plus a newer PARTIAL old-format state: zero `published` events, own journal head unchanged, tree equal to the origin, the complete state kept (41 files, not 5);
- partial old-format state alone: reconciled, queue holds the rest, `publish` returns 0, no `published` event, oracle compare exit 0;
- old, new, old, new again: the old-format files are byte-identical after the new code ran, an old-format run in between (with newer files and a newer cursor of its own) is taken over without loss or republish;
- all state files of a joined replica deleted (checkpoint, log, json, old log, cursor, clock): the daemon says `state-rebuilt` once, publishes nothing, converges; with an unreachable store the replica refuses to publish and says `state-missing`;
- more than 1 000 paths unknown to the state while the store holds them: `state-missing` `republish-storm`, nothing published, queue holds them, convergence after the follow.
Adapted tests: cursor/clock now checked as memos; takeover leaves the old files untouched and publishes nothing; the directory-sweep test cools the hot set first (a restarted daemon now remembers its hot set, which is the point of persisting it).

## Landing and verification

- Lock `1113-landing.lock` held by `claude-warm-restart`, merge base = stored version before my landing, ours = live, theirs = my landed blob (`git merge-file`, four conflicts in each of `🔁️replica/🟦️.ts` and `🩺️attest/🟦️.ts`, resolved by hand because the latency agent's code had moved on: `nextWalk`, `timed`, `startedMs`); the other seven files merged without conflict from the stored blobs; the new code was then edited on top and applied to live as context patches. Recorded in `1113-landings.md`.
- All 14 test files of the module passed against the live tree: git 36, tiers 57, journal store 18, journal 127, state 8, bootstrap 12, chaos 9, `🌿️git.ts` 19, race 14, replica store 8, replica 85, topology 56 (the Windows file `🔵️.ts` has no test on macOS), boundary 39, attest 61.
- Live daemon of g1-server (this Mac): restarted onto the new code at 05:32, `machine.json` 05:32:17, a new `state.ckp` (106 MiB: it holds the paths interned by the storm) written at 05:33:53 after the mixed load, `state.wal` appended since; the old files untouched (`state.json` 05:32:14). Its journal did not move after the restart (segment 133, 147 868 bytes, constant over 16 s, no `published` burst), attestation fresh, `queue.incoming` 0, `files` 272 577.
- Not yet verified: the five workers were still appending to their journals 5 minutes after the landing (their module replicas had not picked up the new code yet; they restart themselves five seconds after the sources settle). Their behaviour on the new code is the test of the reconcile on a real partial state and was not observed.

## Not verified (addendum)

- The reconcile of a real 272 000-file partial `state.json` (hashing of equal-size files in the follow) was not timed; on g1-server it ran with a complete state.
- A daemon of the earlier format started on the new files was not run (there is no earlier-format build to run); consistency of its inputs is asserted by byte equality of the old files.
- Threshold of 1 000 for the republish guard is a constant (`stormThreshold`), not a setting.
