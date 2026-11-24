# P4: one real component final on a fleet worker, with timing (2026-11-24 02:25 to 08:08)

## Verdict
PROVEN, twice, after two failures. The final `semio_hub_stdio_binary_component` (wasm32-wasip2 cdylib, release, the play driver's cargo flags, main's code and sources of 2026-11-24) compiled on a fleet worker and returned a complete 157 MB component:
- Run 1, final pinned to `g1-m12`: 1,686.6 s total (worker run 1,149 s), exit 0, bytes shipped 2.61 GB, returned 157 MB.
- Run 2, `SEMIO_OFFLOAD` unset (= `distribute` with O13 local-first and O17 expensive rule), fresh target: the gate sent the final to `g1-m10` by itself while 567 of 760 compiled units stayed on the coordinator. 1,688.2 s total (worker run 1,617 s), exit 0.
- Two other pinned attempts failed for reasons that are NOT the placement (cache sidecar restart; an lld "unknown file type" on `g1-m06`); both below with the exact text.
- Remote is not faster than the local compile of the same final (local fallback 1,358 s), because every worker granted 1 codegen thread. Its value is freeing the coordinator.

No code was changed (no small clearly correct defect found in an area without a lock; the defects below need design). No ticket, no git write, nothing under `/Users/ueli/.sinc/play` written (read once: its cargo registry was cloned with `cp -c` into the scratch, because main's `.cache/b/r/m/cargo/home` registry is empty), no process of the play build touched, nothing restarted.

## Method
- Scripts (kept, inputs): `P4-run.sh` (= `P3-run.sh` + `SEMIO_OFFLOAD` unset when the mode argument is empty, `P4_UPLOAD`, `P4_CARGO_HOME`), `P4-guard.py` (process group, aborts at `df` free < 100 GiB or scratch > 15 GiB, optional kill when the final's rustc appears), `P4-run-short.sh` (short message format, used once). Final events: `P4-final-events.jsonl` (20 rows, all wrapper events of the final from the four attempts).
- Command: `cargo build --locked -Z trim-paths --offline --manifest-path 🌎️hub/Cargo.toml -p semio-hub-stdio-binary-component --lib --target wasm32-wasip2 --profile release --keep-going --message-format=json -j8` with the play driver's selectors and `CARGO_ENCODED_RUSTFLAGS` (`-Z threads=0 -C link-arg=--max-memory=536870912 -C link-arg=-zstack-size=8388608`), `SEMIO_UNIT_CACHE_SLOTS=2`, `LIGHT_SLOTS=6`, remote cache read-write for dependencies, upload OFF for every final run (so no final can come back as a fleet hit).
- Stage A: all dependencies built with the mode unset, stopped (guard) when the final's rustc appeared: 728 artifacts in 3,040 s; 163 `offload-run`, 15 declined, 8 fallbacks; so about 565 of 728 units compiled on the coordinator (local-first), 22 percent went to workers. Median remote unit 5.2 s.
- Stages B/D re-ran cargo on the same target/cache; because peers kept editing framework sources, each rerun recompiled the changed cone (pinned runs sent that cone to the pinned worker too, which is why the pin runs show many `offload-run` rows besides the final).
- Peak memory: the final's crate peak ledger (requester side, written from the worker's measurement) is 2,586 MB after the first success, 2,674 MB after the third. The O11 family estimate was 7,938 MB and the cold bound 6,144 MB: the estimate is about 3 times the measurement.
- Worker code: I could not read the attested digests (`repo remote-execution status` hangs in `build:source-admission` again, `/v1/fleet/load` has no digest). Instead the result itself attests it: the event detail ends in `, 1 codegen threads` (O16 requester hunk, returned by the worker's `auxiliary_metadata`), present on all three workers that ran the final (m06, m12, m10). So they run O16 code.

## Evidence per placement
| | Pin g1-m12 (run 1) | Natural, ran on g1-m10 (run 2) |
|---|---|---|
| origin | `g1-m12` (task list: EXECUTING on g1-m12 for the full run) | `g1-m10` |
| total offload-run | 1,686,647 ms | 1,688,203 ms |
| plan | 760 ms | 1,556 ms |
| upload to the store | 9,927 ms | 2,704 ms |
| queue and execute | 1,675,261 ms | 1,683,564 ms |
| worker fetch | 23,141 ms | 58,230 ms |
| worker run (rustc + wasm-component-ld) | 1,148,982 ms (19.1 min) | 1,616,605 ms (26.9 min) |
| worker upload of outputs | 3,732 ms | 7,694 ms |
| download and write | 709 ms | 387 ms |
| away from it | 499,383 ms (waiting in the scheduler queue, see 3) | 985 ms |
| token wait | 0 ms | 0 ms |
| codegen threads granted | 1 | 1 |
| bytes shipped / uploaded blobs / returned | 2,610,873,478 / 408 / 156,811,481 | 2,610,930,177 / 469 / 156,913,772 |
| component | 156,810,678 bytes | 156,912,969 bytes, sha256 `ec969c89bf6fd64033c9dd11bf8c2d3fbf04e4a4e11dbf8a46cba4f1ccff9bd4` |
Sizes differ by 100 KB because peers changed sources between the runs (key differs: `7c1a385c...` vs `2b7c3a72...`).

Gate decision line (natural run): on success the gate logs no reason, only the progress lines of the unit: `Unit semio_hub_stdio_binary_component is compiling on another machine.` (DE `wird auf einer anderen Maschine kompiliert`), then `... offloaded to <origin> (1688203 ms).` Cheap units in the same run: 193 `offload-run`, 760 compiles, 399 local hits, plus lines such as `Unit semio_plugin_actor_input stays here: no other machine has room.` (reason `final-wait`: expensive by the seed but no worker had room within 30 s, compiled here). So the O17 rule is observed working both ways: the final left by itself with a free local slot, and an expensive-by-seed lib that found no room stayed. The `reason` itself is not in the offload log; only the wrapper's progress text shows it (suggest logging `gate.reason` in the event).

## Local comparison (not repeated; one data point)
After the first pinned attempt the final fell back and compiled locally: 1,358,255 ms (22.6 min, `miss` event, recorded in the cost folder) under coordinator load 40 with the local grant (rustc CPU time 31 CPU-minutes over 22 min wall, peak RSS seen 1.75 GB, so about 1.4 cores). Remote with 1 thread: 19.1 / 26.9 min. Not compared fairly beyond that: no spare machine time for a second local compile and the play build is running. Conclusion: no speed gain without more than 1 codegen thread; workers run at load 5 to 12 on 10 cores, so O16's grant gives 1.

## Every decline, fallback and failure with its reason
1. Attempt 1, pin `g1-m03` (04:10 to 04:33): worker ran the final 22.8 min, then `offload-local-fallback pin-failed: g1-m03: infrastructure 14 Content store unreachable: http://192.168.178.70:6094: undefined and no other machine can take the action; run the action locally`. The cache sidecar on :6094 was down at that moment (port closed at 04:38, listening again later with a new pid 77308; restarted by another lane, not by me). The finished result was lost and the coordinator recompiled the final locally (22.6 min). Total 45.4 min. REMOVES IT: result download/upload against the content store must retry with backoff (the store was back within minutes); falling back after 22.8 min of finished worker compute is the most expensive possible reaction. Not fixed here (offload client, needs a design for "action finished, outputs fetch failed").
2. Attempt 3, pin `g1-m06` (05:13 to 05:35): worker ran 1,175 s with 1 codegen thread, then rustc returned exit 1: `error: linking with wasm-component-ld failed: exit status: 1 ... rust-lld: error: unknown file type: .../out/<crate>-<hash>.<crate>.<hash>-cgu.NN.rcgu.o.rcgu.o` for exactly 10 objects (semio_framework_kernel_history cgu.01, semio_plugin_actor_input cgu.15, semio_plugin_job_receiving cgu.02, semio_plugin_runtime_ops cgu.01/04/06, semio_s_artifact_stdio_binary_snapshot cgu.00, ...). The rlib members of those crates are LLVM bitcode (magic `BC C0 DE`, rlibs are fine: 16 members, 0.1 to 2.3 MB each, not empty). NOT reproduced on `g1-m12` and `g1-m10` with the same inputs afterwards, nor with a toy thin-LTO cdylib with and without `-Z no-parallel-backend`. Unresolved; hypotheses: a bitcode object not consumed by ThinLTO and handed to lld on that one worker (sandbox/tmp truncation of the `.rcgu.o.rcgu.o` copies), or a rlib shipped from a stale partial upload. NEEDS: retain the worker sandbox of a failing final (the `out` dir) and compare `file` of the 10 objects. Scratch deleted, so only the excerpt above survives.
3. Queue: run 1 waited 8.3 min in the scheduler queue (QUEUED, no worker) although `g1-m12` was idle with 4.9 GB spare: the placed peak at that time (family estimate 7,938 MB, ledger not yet updated) exceeded every worker's spare. After the measurement (2.6 GB) was in the ledger the natural run waited 0.9 s. REMOVES IT: replace the 7,938 MB family estimate by measured values early (this final measured 2.6 GB; O11's family p90 and cold bound 6,144 MB are 2 to 3 times too high on a thin-LTO final of this size); until then every first final queues for minutes or falls back.
4. Phase A and run-wide totals over about 2,100 events: 1,629 `offload-run` (median 4.1 s, p90 12 s, 247 GB shipped, 9.5 GB returned; per worker: 760 on one, 592 on another, 182 on a third), 55 `offload-declined` (41 `unmapped-path`, 12 `not-portable` build scripts, 2 `no-idle-machine`), 33 `offload-local-fallback` (25 `pin-failed`, 7 `infrastructure`, 1 `incomplete-inputs`; by text: 11 `incomplete-inputs` (rmeta-only shipping, `semio_offload_inputs_incomplete`), 7 `deadline 240000 ms`, 6 `no-endpoint`, 3 content store unreachable, 1 `no other machine can take the action`).
5. Store 507: NONE this time (0 of about 2,100 events). With free disk (140 to 195 GiB) the 27.75 GiB cap did not bite (the closure plus 2.6 GB of final inputs, 408 new blobs). The earlier 507s of P3 were the disk-full coordinator, not the cap alone; the cap itself is unproven either way here.
6. Fleet cache: all 2,330 lookups of stages A/B missed (0 fleet hits, 8 local hits); natural run 399 local hits (my own cache). The cold fleet namespace of P3 persists.
7. Remote compile error without diagnostics: two attempts (pin m03, 03:03 to 03:08) failed `semio-framework-value-native-owned` remotely with exit 1 and only warnings in the JSON (no `level: error` message anywhere); the same unit compiled locally 3 minutes later and remotely 40 minutes later. A peer had just added a dependency to that crate's manifest. Likely a source race, but the wrapper dropped whatever the worker said: exit 1 with no error text makes such failures unreadable. Worth a separate fix (return the worker's stderr verbatim, including text outside the JSON).
8. The shared tree was unbuildable several times during the five hours (peer edits): `cargo --locked` failed with "cannot update the lock file" (four times, each lasting 5 to 20 min), a manifest of a new crate without its workspace root (`norm/artifacts/Cargo.toml` missing), a compile error from an in-flight `debug_histogram` change, another from `digest_window`. Each required waiting and a retry; a build of main is only provable between peers' edits.

## What it takes for finals to go remote by themselves (confirmed by the run)
`SEMIO_OFFLOAD` unset or `distribute`, slots and light slots > 0, `-j` = local + light (all present in `P4-run.sh`), a reachable scheduler and store, and at least one worker with spare above the final's ledger peak. Then the final left the coordinator with a free local slot (run 2) and finished; cheap units stayed local-first (78 percent of compiles).

## Open items, smallest first
1. Retry the store fetch/upload with backoff before falling back (point 1): turns a 45 min loss into a 1 min delay.
2. Seed the family peak from measurements (2.6 GB here) so the first placement does not queue 8 min.
3. Log the gate reason (`final`, `final-wait`, `local-slot`) in the offload event; return worker stderr verbatim (point 7).
4. Reproduce point 2 (lld unknown file type on m06) with the sandbox retained.
5. O16 grants 1 codegen thread on loaded workers: to make a remote final faster than local, the grant needs idle cores (a worker held for finals, or fewer cheap units on it).

## Cleanup
Scratch `scratchpad/p4` (target, cache root, clone, logs, cargo-home clone, 5.2 GB) deleted. Host-CAS holder files `~/.cache/semio/cas/v1/h/unit-cache-c7b421a4c6a74e61.json` (the cache root) and `unit-cache-851167ae9cc89bdd.json` (its `cp -c` clone) and their keeper dirs under `k/` deleted; the other two holders (the play clone and main's root) untouched. `df` free 146 GiB after. Kept: `P4-run.sh`, `P4-run-short.sh`, `P4-guard.py`, `P4-final-events.jsonl`, this report. No process left running.
