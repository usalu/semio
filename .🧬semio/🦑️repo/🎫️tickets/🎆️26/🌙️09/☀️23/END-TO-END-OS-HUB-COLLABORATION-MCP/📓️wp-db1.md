# WP-DB1 — db Engine Write/Replay Throughput

Slice: DB1 (session 12). Hubs 8140–8149. Private cargo target: `.tmp-ticket/wp-db1/target`. Durable data/logs:
`.🧬semio/🌐hub/s12-db1-*`. Captures: `wp-db1/generated/` (expendable).

Problem (H9 17:45, `db-storm-1.txt`): 24 documents × 128 opaque edits took 1472 s (~7.7 s per 16-envelope submit; index
maintenance per commit: `IndexHandle::put_batch` → `maybe_auto_merge` → `merge_adjacent`, `read_run`/`list_step`/`replace_step`
on fs); reopen storm of 24: 24.2 s (p50 11.8 s, max 22.7 s), each hello replays the whole WAL, one worker turn per envelope.

## Session 13

Agent: DB1 (session 13, Opus 5.5), started 19:07. Rules: `📓️session-13-preamble.md`. Logs `.🧬semio/🌐hub/s13-db1-logs/`.

| # | Item | Status |
|---|------|--------|
| 1 | Profile write + reopen paths, cost model per operation (extend to reopen) | in progress |
| 2 | Write path: batched index maintenance, bounded merge, fsync only where needed, registry mutex off I/O | s12: landed (2 tasks/commit); re-verify |
| 3 | Reopen: hello from checkpoint + tail, batched replay turns, bounded storm | TODO |
| 4 | Benchmark laws fs/sqlite + postgres/neo4j; H9 storm 24×128 (1472 s) + reopen storm (24.2 s) re-measured | TODO |
| 5 | Crash/fence/durability laws green (full db nextest) | TODO |

### Log (session 13)

- 19:07 started; read AGENTS.md, preambles 13/12, this report, `wp-db1/` (the draft `index-handle-new.rs` is IN the tree
  at `🔢️index/🦀️.rs:1237`, evolved: `put`/`delete` single-entry helpers, `verify` checks declared run sizes). Last s12
  state (`s12-db1-logs/nextest-1.txt`, 18:38): 777/779, the two failures = the fs/sqlite throughput laws on the storm ratio
  (fs step11: grow 24×8×16 in 770 ms, ack p50 68 ms; storm welcome p50 7.3 s vs solo 121 ms). Load 39, 111 GiB free,
  docker 29.5.3.

- 19:14 build-1 (7 min, load 84). **fs throughput law on the tree as found** (`s13-db1-logs/fs-1.txt`, load 84): commits
  2 tasks each (s12 fix holds); grow 24×8×16 11.3 s, ack p50 1176 ms vs durable append p50 188 ms (6.3×, bound 6×);
  solo reopen mount 1009 ms + hello 77 ms; **storm of 24: 38.5 s, welcome p50 20.9 s, max 35.5 s** (> 30 s).
  H9's long law (`h9-storm-1.txt`, load 89): **grown in 26.8 s (was 1472 s), storm 16.6 s (p50 6.6 s, max 14.4 s)**.
- 19:1x **reopen cost model** (`sample` of the storm, `wp-db1/sample-fold.py` + `fold-query.py` over
  `generated/s13-storm-sample-1.txt`): workers spend 53 % in jobs, **44 % in `select_and_pop` waiting on the pool's global
  maintenance / deferred-wake mutexes** (idle scans lock both registries up to 48× per scan; the deferred-wake scan walks
  2048 slots under its lock), 3 % stealing. Of the job time 71 % is **stack probing of debug poll frames**: the mount
  chain `run_open_document_mount` 1.6 MiB → `open_retained` 1.7 MiB → replay block 0.5 MiB → record loop 1.2 MiB
  (prologue `sub x9, sp, #0x187, lsl #12` …, `wp-db1/frame-sizes.sh`), ~5 MiB probed on EVERY poll, and the replay
  re-polled the chain ~25× per envelope: the WAL cursor's bounded-step `Yield`s were each turned into `yield_once`, and the
  artifact's two-stage retained decoder (`ArtifactWalEnvelopeDecode` → `ArtifactWalRetainedEnvelope` with 64 inline
  1 KiB `DbIoText` dependencies → `ArtifactWalEnvelopeAdapter`) returned `Pending` after every field/page phase.
- 19:2x–19:56 **reopen fixes (db crate):**
  1. ONE decoder of the WAL command form, `db_sync::decode_wal_command` (one bounded step; counts/lengths checked against
     what the record still holds before allocating; `WalBytesCursor::field` copies page slices directly). Replaces the
     artifact's decode+adapter pair (open replay and `artifact_ledger_tail`), sync's `decode_retained_command_envelope` +
     `decode_protocol_field`, and the hello's text/payload/envelope builders (−~600 lines). `WalBytes::hash` is one step.
  2. `db_wal::WalReplayTurn` (1 ms): WAL walks (open replay, ledger tail, `replay_sync_state`, hello replay) continue a
     cursor `Yield`/close step inline and yield to the executor once per turn instead of per frame/record.
  3. Hello replay decodes only the replica's missing tail (`DatabaseSyncHelloReplay`): earlier commands are hashed into
     the chain and counted, their id read for the head, never materialized.
  Laws: `wal_command_decoder_agrees_with_the_protocol_codec_and_refuses_what_a_record_cannot_hold` (reference =
  `protocol::decode_envelope`; multi-page, 512 deps, non-ASCII, cancel, deadline, trailing, overcount, truncated,
  over-long text) PASS; `retained_hello_replay_decodes_only_the_replicas_missing_tail` (heads 0/4/6 vs whole-WAL replay)
  PASS; `sync_replay_ignores_neutral_aborted_command_snapshot_and_cas` PASS. Record-loop frame 1.2 MiB → 376 KiB.
- 19:58 fs law (`fs-2.txt`, load 88): storm hellos refused after 30 s (`database sync hello admission saturated`) — 8
  hello slots, sessions release slowly under the pool contention; H9 law (`h9-storm-2.txt`, load 86): storm 32.0 s (noise:
  its growth phase, untouched, was 41 % slower than at 19:21). Wall time at load ~88 is not a usable signal; next: the
  pool scheduler contention (below) and census-based counts.
- 20:0x **pool scheduler (semio-framework-async, guest-linked):** maintenance registry publishes per-lane pending counts
  (atomics written under its lock), deferred-wake registry an occupied count, native workers a per-lane queue length —
  idle scans and steals read atomics and lock only a non-empty queue. Written, compile pending (rule 22: < 10 rustc).
- 20:05 coordinator rule 22 (memory): DB1 runs no server/hub/serve/browser (none were running); the pg/neo4j throughput
  laws will reuse `semio-hub-backend-{postgres,neo4j}` instead of starting containers.
- 20:15–20:21 async pool change: `cargo check -p semio-framework-async --lib --tests` PASS (native), `--target wasm32-wasip2`
  through the wasm mutex PASS (rmeta rebuilt 20:2x); recorded in `wp-w3/requests/db1.txt`.
- 20:2x LD (rule 23) changed `MutationEnvelope` (`observed`, `target`) and its codec while I worked; LD's codemod had patched
  my field-by-field decoder with empty fields, which would have misread the new layout. **`decode_wal_command` now
  delegates to `protocol::decode_envelope`** (gather the record once if it spans pages, trailing-bytes check) and
  `wal_command_id` reads only the first field (`protocol::read_str`); LD confirmed `mutation_id` stays first and bounded
  the untrusted `dependencies` / `read_vec_envelope` capacities (`.min(bytes.len())`, hostile wire input) on my report.
- 20:3x H12 (via coordinator): `DbIoCensusV1` gained `admission_waits` / `admission_wait_micros` / `admission_refusals`
  per kind + `total_*` (counted once per waiting submission in `submit_db_io_task_admitted`; waiting backend opens under
  `BackendOpen`); `describe()` appends `admission=waits/µs/refusals`. H12 consumes these names on
  `/admin/api/observability`. db `cargo check --lib --tests` (all drivers) PASS 20:48 (`check-3.txt`).
- 20:4x rule 22/25: pg/neo4j throughput laws no longer start containers; they read the hub's own env
  (`OS_HUB_DATABASE_URL`, `OS_HUB_NEO4J_URI`/`_USER`/`_PASSWORD`). New `os-hub-ts backend run <postgres|neo4j> -- <cmd…>`
  (claimHubBackend → run with the claim's env → release; nx `os-hub-ts:backend-run`). V1's `reopen-storm-check` got
  `postgres`/`neo4j` entries (claimed-only, not in `all`) and a regex that also captures the unit law (V1 agreed).
  Killed my own stalled `cargo check` (pid 66584, 20 min without rustc, rule 25) and its waiter.
- 20:52–20:54 **fs throughput law after decoder + turns + pool fix** (build-3; `fs-3.txt`…`fs-5.txt`, load 95–108):
  solo reopen mount 68–102 ms (was 1009 ms at load 84), hello 8–21 ms; **storm of 24: 2.0–2.5 s, welcome p50
  1.6–1.9 s, max 1.8–2.5 s** (was 38.5 s / 20.9 s / 35.5 s); grow 24×8×16 3.3–4.5 s (was 11.3 s). Still red: ack p50
  318–447 ms vs durable-append p50 40–70 ms (7–8×, bound 6×) and storm p50 ≈ 20× solo (bound 8×) — the storm is no
  faster than 24 serial reopens (2.06 s vs 24 × 94 ms), so something still serializes mounts (next: phase breakdown).
- 20:55 **H9's unit storm law** (probe run, load ~95): **grown 24 × 128 in 12.5 s (baseline 1472 s), storm 605 ms,
  welcome p50 110 ms, max 482 ms (baseline 24.2 s / 11.8 s / 22.7 s)**.
## Status

| # | Item | Status |
|---|------|--------|
| 1 | Profile write + reopen paths, cost model per operation | TODO |
| 2 | Write path: bounded index maintenance per commit, fewer fs round trips | TODO |
| 3 | Reopen path: hello from checkpoint + tail, batched replay turns, bounded storm | TODO |
| 4 | Benchmark laws (ratio + absolute), fs/sqlite + postgres/neo4j | TODO |
| 5 | Crash/fence/durability laws green | TODO |

## Log

- 17:47 started; read AGENTS.md, preambles 11/12, wp-h9 (Session 12, 17:45), wp-h10. db crate
  (`semio-framework-os-kernel-db`) is native-only (reverse deps: `semio-hub` only), its dev closure pulls no plugin/stdio
  crate → db tests allowed under rule 24 with `nice -n 15`. Load 45, 138 GiB free.
- 17:5x–18:05 **baseline cost model (fs, debug, current tree + a new DB I/O census)**. Census = process-wide counters
  per `DbIoTaskKind` of typed tasks submitted, executor steps and worker-pool turns (`db_storage::db_io_census()`,
  `DbIoCensusV1::since`). One 16-envelope `Fsync` commit on fs costs **~630 DB I/O tasks and ~14 400 executor steps
  (each step its own worker-pool job)**: IndexWrite 97, IndexRead 336 (35 steps each: 1 data page + 30 unused output
  pages returned one per step), IndexList 146 (~17 steps each: one `read_dir` per listed run, O(N²)), IndexDelete 49,
  WalAppend 1, WalSync 1. Cause: `ArtifactEngine::submit` records 3 index entries per envelope + 1 frontier = 49
  single-entry `put_batch` calls per commit; each lists all runs (twice), writes a run, then `maybe_auto_merge` reads
  5 whole runs just to learn their entry counts and merges a pair (2 reads, 1 write, 1 delete). Every run write on fs
  = temp file + `F_FULLFSYNC` + rename + a directory fsync for EVERY ancestor of the index dir (`durable_directory`
  walks to `/` on every step). Commit cost is flat in history (629→639 tasks over 8 commits) but huge.
  Found on the way (both worse than the index for concurrency): (1) `db_io_executor_execute` held the process-wide
  backend-registry mutex for the whole step, so EVERY blocking-lane I/O step of the process (every fsync) ran one at a
  time; (2) the pool's idle workers spin on queue locks (`native_pool::steal`, `WorkerMaintenanceRegistry::has_pending`)
  — `sample` of the law: ~7.3 s of 5 s × 12 threads in `__psynch_mutexwait` (semio-framework-async, guest-linked,
  frozen — noted, not touched). Capture: `wp-db1/generated/baseline-fs-sample-1.txt`, log
  `.🧬semio/🌐hub/s12-db1-logs/baseline-fs-1.txt`.
- 18:05–18:30 **write path root fixes (db crate only, native-only, compile-checked + law runs):**
  1. `db_io_executor_execute` no longer holds the process-wide backend-registry mutex during a step: the step borrows
     the executor while `executing_steps` counts it (close/retire/lease require it zero), so blocking-lane steps of
     different operations run concurrently (every fsync of the process used to be serialized).
  2. One worker-pool turn drives a task's resumable steps for up to 2 ms / 256 steps (`DB_IO_EXECUTOR_TURN_*`,
     cancellation checked between steps) instead of one pool job per step (an index read was 35 jobs).
  3. fs: `list_step` lists a directory in ONE `read_dir` (was one per listed entry, O(N²)); `durable_directory`
     remembers directories it already made durable (bounded 4096) instead of fsyncing every ancestor on every step.
  4. Document authority (`ArtifactRunner`) and sync hello re-poll a self-woken future inside the same worker turn
     (2 ms budget) instead of resubmitting a pool job per yield (`repoll_within_turn`, hello `poll_one`).
  5. A commit stages all its WAL command records under ONE DB I/O operation (was one ledger slot + one 16 KiB page per
     record → 4 concurrent 16-envelope commits exhausted the process ledger: `wal page admission rejected`), and no
     longer writes the `Outbox` WAL record (a byte-identical duplicate of every command, read nowhere; the decoder
     keeps the record kind).
  6. Index: run ids carry their entry count (`[format:4][kind:4][sequence:50][entries-1:6]`, format 1; format-0 runs of
     older data roots are ignored), so appends list once and merge at most one pair with sizes from ids (no reads);
     `put_sorted_run` bulk-encodes caller bytes into one run write. The artifact engine keeps a per-document **index
     backlog** (command/inverse/actor-seq/frontier entries) and writes only FULL 64-entry runs (≤ 5 per kind per
     commit; a failed write is retried next commit and surfaces past 4096 pending); open refills the backlog from the
     WAL suffix past each kind's newest run (`indexed_through`); compaction drains it (partial runs) before deleting
     WAL segments; query consistency reads the engine frontier + backlog before the index. Fixed on the way: every
     envelope of a batch was recorded under the SAME actor_seq key (the batch's final one).
  7. fs durable sync on Apple = the file's own `fsync` + one shared device-wide `F_FULLFSYNC` per flush group
     (`FsFullSyncGroup`): a flush that starts after a member's fsync returned covers it (group commit); elsewhere
     `sync_all` as before.
  8. fs/sqlite per-operation cursor state is keyed by the operation's ledger slot (unique among live operations);
     `operation % 64` collided once steps ran concurrently (`filesystem read cursor capacity exhausted`).
- 18:25 **cost model after (fs, same law):** a 16-envelope Fsync commit = **2 DB I/O tasks** (WalAppend + WalSync),
  every 4th commit +3 IndexList +3 IndexWrite (full runs); 0 index reads. Solo reopen: mount 105 ms (16 tasks), hello
  **688 ms → 7.9 ms** after (4). 8×4×16 growth: ack p50 3.2 s → 27 ms.
- 18:27 full fixture (24 docs × 8 × 16, 10 workers, fs, debug, load ~25): grown in 4.3 s (baseline extrapolates to
  ~25 min), **ack p50 410 ms, p95 1.03 s** (24-writer durable-append floor p50 36 ms), late/early 746/279 ms (late
  window holds the index-flush commit); **storm of 24: welcome p50 3.3 s, max 6.4 s, solo 132 ms** — the storm still
  serializes somewhere (next).
- 18:3x coordinator: H9 saw 3 db laws fail. Root causes + fixes: two history laws submitted with `Memory` durability
  and depended on slow commits tripping the 20 ms group-commit flush before `history()` replays the durable WAL (now
  they submit `Fsync`, like the hub); the compact law needed ≥ 2 WAL segments and the outbox duplicate had doubled its
  WAL (now 12 × 50 KB fills). history/compact filter **73/73** (in-process, 3 runs; one run showed
  `artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each` failing in its isolation child — 6/6 PASS
  alone, pure in-memory law, watching). Full db nextest running (pid 86552).
