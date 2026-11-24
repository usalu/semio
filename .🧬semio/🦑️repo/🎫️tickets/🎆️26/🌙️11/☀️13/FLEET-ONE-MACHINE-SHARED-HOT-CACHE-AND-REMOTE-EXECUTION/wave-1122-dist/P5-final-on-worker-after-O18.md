# P5: the component final on a worker after O18 (2026-11-24 08:27 to 13:56)

## Verdict
NOT MEASURED, and a defect explains why. In two pinned attempts (pin `g1-m06`, the only worker that advertised `semio-linker` when I started) the final never ran on a worker to completion: after 887 s and 829 s the requester fell back to compiling locally with `pin-failed ... Missing blob`. So there is no worker run time, no granted codegen thread count, no returned bytes, and no learned peak for a remote run. Remote is NOT shown faster than local; it was not shown at all. No code changed, no ticket, no git write, nothing under `/Users/ueli/.sinc/play`, nothing restarted.

## 1. Precondition (done)
- `repo fleet status --json` does not carry worker properties. They are in `repo remote-execution status --join http://192.168.178.70:8981` (JSON `server.workers[].properties`, 3 to 4 min because of source admission). NOTE: `remote-execution workers` JOINS this machine as a worker (I started it by mistake, stopped it within seconds in its loading stage; no registration happened, `ps` clean).
- 08:30: only `g1-m06` advertised `semio-linker=0fbcef956b2adb56`. I computed the coordinator's identity from `lib/rustlib/*/bin` (6 files, same recipe as `offloadLinkerIdentityV1`): also `0fbcef956b2adb56`, so m06 matched. All other workers had none, from which the 40 minute wait was not needed.
- 10:55 (after someone restarted the scheduler at 10:49:39 CET, counters back to zero): 10 of 13 workers advertise `0fbcef956b2adb56` (m01, m02, m03, m06, m07, m08, m10, m11, m12, m13, server); m04 (`b619747e99bde2a8`) and m05 (`237bd2b560366154`) have other linker identities and other toolchain digests (m04 `e77b4888...`, m05 `20b34680...`), i.e. correctly refused. Worker sidecar start times could not be read (status has no start field; the scheduler start is `startedAt`).
- Later `remote-execution status` returned `server: null` (shape check failed: the scheduler now emits `counters.restored`, the CLI reader in this tree requires it too and reports "Closed fleet load required: /counters/restored is required" in `fleet status`): landing in progress by a peer, not mine.

## 2. What ran (scripts kept: `P5-run.sh` = P4-run.sh with lane p5, `P5-guard.py`; events: `P5-final-events.jsonl`)
Scratch in the session scratchpad (4.7 GiB after stage A, below 15 GiB; guard aborts at `df` < 100 GiB). `.cache/b/r` was mounted (`/dev/disk15s1`), cargo home cloned with `cp -c`. Same command, flags, slots (2 + 6 light), `-j8` as P4; upload of finals off.

Dependencies warm: stage A (natural, stop when the final's rustc appears) took 33 min, 730 compiles, 75 offloaded, 0 fleet-cache hits again (746 remote-miss), 12 `not-portable` + 1 `unmapped-path` declines. Every later run recompiled a cone of 250 to 360 units because peers keep editing framework crates (a source change in a low crate re-keys everything above it), so a "warm" state is valid for minutes only; the keys of the final differ between all four runs (`daea7181`, `fd0c14e6`, `f84bce38`, `28e50cbe`), hence no byte comparison between local and remote output is possible (see 5).

| attempt | mode | key | wall | outcome |
|---|---|---|---|---|
| C | local (`never`) | `fd0c14e6` | 984,962 ms (16.4 min) | ok, 156,858,664 B, sha256 `e2610f7c...`, load 36 to 47 on 10 cores |
| B1 | pin g1-m06 | `f84bce38` | 886,710 ms then fallback | `pin-failed: g1-m06: no-idle-machine Missing blob cd18cb50...3b/4223571` (that digest is the 4.2 MB `.rmeta` of `semio_hub_stdio_binary`, which this very run had just built; I verified the sha256 of the file) |
| B1 local part | after fallback | `f84bce38` | 801,478 ms (13.4 min) | ok, 157,038,946 B, sha256 `1770d6b5...` |
| B2 | pin g1-m06 | `28e50cbe` | 828,600 ms then fallback | `pin-failed: g1-m06: no-idle-machine Missing blob 2c3a800a...51/460` (460 bytes: an action or command proto) |
| B2 local part | after fallback | `28e50cbe` | killed after about 1,420 s at about 100 % of one core | killed by my own disk guard (free fell to 96 GiB, caused by peers: my scratch stayed at 8 GiB), result lost |
Two further pinned runs (m06) were abandoned during the cone recompile: a pin sends every cheap unit of the cone through the pinned worker's one lane (38 to 170 s each in the queue), so a pinned run cannot be warm-then-forced; the warm step must be natural (I did that, 25 min).

Other attempts to measure on the worker were blocked by the disk-guard rule, not by the fleet.

## 3. Defects (file:line)
1. A worker or scheduler "Missing blob" is classified as a non-retriable decline. `🚀️offload/🩺️failure/🟦️.ts:10`: `FAILED_PRECONDITION` (code 9) maps to `no-idle-machine` unless the text starts with the refusal prefix; the status carries `missing[]` (`🗓️scheduler/🟦️.ts:39` `MISSING`, `👷️worker/🟦️.ts:256,258,361` `fail(9,..,[digest])`), and the client `🚀️offload/🔌️client/🟦️.ts:73` turns every non-zero code into `declined(verdict.reason)` without re-uploading the listed digests and re-executing, which is the REAPI contract (FAILED_PRECONDITION + MissingBlobs means upload and retry). For a unit with 14 minutes behind it this is the most expensive reaction (same shape as the O18 store outage fix, which only covers code 14). Fix: on code 9 with `missing`, re-upload those digests from the local tree (the requester has them) and execute once more.
2. Why blobs were missing (hypothesis with evidence, not proven): both failures came after about 14 min in the queue of a pinned, "refusing: no-headroom (slots)" worker (m06 had 3 to 5 running actions the whole time), and the content store was FULL: 212 `infrastructure memo the content store is full (507)` fallbacks plus 507 answers for uploads in my events, host CAS grew from 45 GB to 90 GB during the day. A store at its cap evicts blobs of queued actions. Needs: the leases of an action's inputs must outlast the queue wait (or be renewed while queued), and a pin to a busy worker should time out at the policy's final wait, not 14 min.
3. A scheduler restart during a 20 minute action (10:49:39) zeroes the counters and every in-flight action: callers get `14 Call closed without status` / `ERR_HTTP2_STREAM_CANCEL` (seen: unit `semio_hub_stdio_binary`, 303 s lost, and `semio_plugin_tool_machine`). Restart is the other peers' choice, but the requester falls back to local immediately; the lost remote work is unrecoverable without durable operations.
4. `remote-execution workers` starting a worker from an inspection-sounding verb ("workers") is easy to run by mistake (`⌨️command/🟦️.ts:168`); a `list` verb that reads `/v1/...` would avoid it. Also `repo fleet status --json` has no per-worker properties/start time, so the O18 rollout cannot be read without the 4 minute `remote-execution status`.
5. Scratch harness limit, not a repo defect: local compiles in my scratch (`env -i`, no build broker) do not write the learned peak (`📜️script.ts:256` needs `recallProvenanceV1` from the broker), so a `.peak` appears only for remote runs. In the real repo pipeline the broker exists.

## 4. Learned peak (item 3): not observable
No remote run completed, so the final's `c/semio_hub_stdio_binary_component.peak` never existed (before and after: absent). The cost file `c/semio_hub_stdio_binary_component` = `801414` after the B1 local compile (before B1: absent; before B2: 801414; the killed B2 local did not update it). The family estimate would still have applied: only 1 `_component` peak sample (`wit_component.peak`) exists in the scratch ledger, below `minimumCrates` 3, so the placed peak was the cold bound 6,144 MB (x1.25 placed) rather than the 7,938 MB family estimate; hence why the 8.3 min "no worker fits" queue of P4 did NOT recur: the pinned unit queued for a different reason (slots), see 3.2. The natural run (`SEMIO_OFFLOAD` unset) was not done: the only instance of the natural path in this session is stage A's cone, where 75 cheap units went to workers and the final never started (stop-at-final).

## 5. Table
| | local | P4 (pin m12, 1 thread) | P5 now |
|---|---|---|---|
| wall | 985 s (C, key fd0c, load about 40), 801 s (B1 local part, key f84b); P4 local 1,358 s | 1,687 s pinned total (worker run 1,149 s); natural 1,688 s (worker run 1,617 s) | not completed: 887 s and 829 s lost to `Missing blob`, no worker run time |
| queue | none | 499 s (pin) / 1 s (natural) | about 14 min each, ended by Missing blob |
| codegen threads granted | all local | 1 | not observed (no result returned) |
| worker CPU / peak memory / bytes shipped / returned | n/a | 2,586 MB peak; 2.61 GB / 157 MB | n/a |
| byte identity local vs remote | n/a | n/a | not testable: sources and keys differ per run |
Plain statement: remote is NOT shown faster than local. The best evidence is still P4's: 1,149 s on a worker with 1 thread versus 801 to 985 s locally in this window; O18's fair-share grant has not been exercised on the real final.

## 6. Open, smallest first
1. Handle code 9 `missing[]` in the offload client by re-uploading and re-executing once (defect 1); the pin path would then have survived both of my attempts.
2. Keep (or renew) leases for the inputs of queued actions; cap a pin's wait at the finals' wait policy.
3. Re-run this measurement on an idle worker (`running 0`, headroom) with a quiet tree, when the content store is not at its cap (`repo remote-cache status`), natural path included; use `P5-run.sh` after: natural warm with `P4_STOP_AT_FINAL`, then pinned final.
4. Make `fleet status` list per worker properties and start time.

## Cleanup
Scratch `scratchpad/p5` (target, cache root, 4 snapshots, cargo home clone) deleted. Holder `~/.cache/semio/cas/v1/h/unit-cache-c8fe03f572141cfd.json` and its `k/` dir deleted (the other two holders, play and main, untouched). No process of mine left. `df` free 107 GiB after (it fell from 207 to 96 GiB during the day with my scratch constant at 8 GiB, so peers/CAS/store growth).
