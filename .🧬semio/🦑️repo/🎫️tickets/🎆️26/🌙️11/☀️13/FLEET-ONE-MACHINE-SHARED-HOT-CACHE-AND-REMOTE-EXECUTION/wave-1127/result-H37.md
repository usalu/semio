# Result H37: Nx remote cache, end to end (4 hits, few puts)

## Verdict
Workers ARE configured for the remote Nx cache and DO put; the near-zero hit rate is hash divergence between machines plus runs that never finish, not a missing/broken route. A measured cross-machine hit was NOT obtained (see "Not delivered").

## 1. "0 Nx puts since restart" was a timezone misread
`remote-cache status` prints `startedAt 2026-11-27T02:36:30Z` = 03:36 CET (ps lstart 03:35:36). The "session" window was minutes, not hours. Session nx puts is 2 now (03:5x), lifetime nx hit 4 / miss 496 / put 173 / rejected 51 (03:52). Provenance (`.cache/b/p/m/fleet/cache-server/fleet/provenance.jsonl`, kind nx, numeric keys = Nx task hashes; `inst-...` keys are bun install keys, not Nx) shows 13 task puts since 02:58 from m01 m02 m06 m08 m11 m12 m14 m15, each attributed to its own origin: the signed fleet-identity PUT route works on workers.

## 2. Configuration chain (code read + executed probe)
`⚡️caching/🚀️bootstrap/📜️script.ts:458` calls `nxRemoteCacheEnvironmentV1(env)` then `routeNxThroughFlightV1` (loopback single-flight front); Nx 23.2.0 `DbCache.getHttpCache` uses `NX_SELF_HOSTED_REMOTE_CACHE_SERVER` (= front). Probe with `SEMIO_WORKSPACE_ROOT` set (scratch H37/probe.ts): `NX_SELF_HOSTED_REMOTE_CACHE_SERVER=http://192.168.178.70:6094/main`, `SEMIO_REMOTE_CACHE_SIGNED=1` (fleet-identity, no token needed), front on 127.0.0.1. Without `SEMIO_WORKSPACE_ROOT` it falls back to the local 6090 server (a different store; not the case in the real run, `script.ts:248` sets it). Nx's "share a cache across your team" recommendation is printed regardless (it only checks Nx Cloud), so it is not evidence of a missing remote cache.

## 3. Task-key receipt (H30 question)
The receipt (`.cache/b/g/m/task-keys`) is per machine and only answers repeats of a task the SAME checkout already ran; it is written after the Nx run, so it never prevents the first put, and a second machine has no receipt, so it asks Nx and thus the remote cache. No bypass of puts/gets.

## 4. Same hash across two Ready workers (after H18's quiet door): YES
`claims.jsonl` (nx namespace): hash `6725876443105675084` was claimed by g1-m11 (03:31:20) and by g1-m02 (03:40:26), i.e. both machines compute the identical Nx hash for the same task (norm-din4108-rs build class, expected 450 s). Before H18 no numeric key was ever put by two origins (137 distinct of 139 rows).

## 5. Why still no hit
- m11 ran that task but its workspace stage was cancelled (`Owned Nx invocation failed (130)`, 03:38; coincides with the cache-server restart 03:36:30 that dropped its claim): the 7-10 min task never reached its put, so m02 recomputed it. Workspace runs take 12-32 min (m01 15 m, m06 25 m, m14 32 m) and are killed by restarts/landings before tasks end; a put happens only at task end.
- Nx `DbCache.put` (patched, `nx@23.2.0.patch` ~l.1302-1317) returns SILENTLY (no local, no remote store) when `sourceClosureReceiptHolds` fails, i.e. when any file of the task's source closure changed during the task. In this wave build code churns constantly (and on the coordinator the lanes edit the closure), which fits "H30 cold run, 34 s task, no put" and the few puts per worker run (11 tasks per run, 0-2 puts). Not proven (no log exists: the drop is silent).
- Hashes include the source closure digest, so two machines agree only when their replicas hold the same closure bytes at hash time; landing slots (:00-:10) help.

## Fix (landed 05:01 slot, see landings.md "H37 round 2")
`T/🧰️scripts/H37/apply.py`: makes the three silent `verifySource` returns print a bilingual line `[nx] Source changed during <task>; cache promotion withheld / ...` (same line count, hunk headers stay valid). It changes the Nx tooling identity (every machine re-provisions Nx tools) and the 04:00-04:10 slot was spent on measuring; count the lines in worker logs to confirm or refute the closure-churn cause.

## Not delivered
- Measured cross-machine Nx hit with counters: runA (`repo run @semio-tech/repo-workspace-go:generate-source-policy:prototype --exclude-dependencies`, fresh `NX_CACHE_DIRECTORY`, receipt moved to scratch H37/receipts) took 10:33 and failed with `The resident service met an unexpected failure (c4b7ad0c): Slice worker exited with code 1 ... 🌊️races` (file churn race), before Nx started. Counters before: nx hit 4 / miss 496 / put 173; unchanged by the run. The H30 receipt file was moved away by me (restore from H37/receipts if wanted; it is only a cache of an up-to-date answer).
- The two-checkout Nx e2e (`🚦️single-flight/🧪️tests/🧰️nx-e2e`) is skipped on this machine (needs an unpatched `nx@23.2.0` in `~/.bun/install/cache`), so no isolated proof of put-then-remote-hit exists here either; running it needs one unpatched Nx download (needs your approval).
- Worker-side task hash of norm-din4108:build per machine: Nx does not log hashes; only the claim hash above was compared.

## Round 2
1. Skip lines landed (05:01). The check in the patch compares the task's OWN `semioSourceInputs` receipt (its declared source closure digests) via `sourceClosureReceiptHolds`, so it is not broader than the task's inputs; the only wider check is `originalSelectedGraphHandoffHoldsV1` (selected graph source) when `SEMIO_NX_SELECTED_GRAPH_RECEIPT` is set. No narrowing implemented yet: wait for data. Next step (>=06:05, after workers restarted their Nx tools): `grep -c "cache promotion withheld" $S/origins/*/ready-log-workspace.txt` vs puts in provenance; if the "Selected graph source changed" variant dominates, narrow `originalSelectedGraphHandoffHoldsV1` to the task's project closure. NOT quantified yet (no worker has run the new patch).
2. Slice-worker race: root cause `capturePhysicalOnce` (`🏗️build/📥️inputs/🔏️revision/🟨️.mjs`) threw a code-less `Bounded physical Source revision required` on `🕸️graph/🕰️generations/🟨️.mjs` (log `host-fca22674af5e.log` 04:06 CET): the file was a regular, 1-link, 65 kB file again moments later (replaced at 04:06:51), so a replace-by-link moment (`nlink` 2) was refused as permanent and, having no errno code, was not retried by `retryOnConcurrentEditSyncV1`, killing the slice worker and the whole narrow run. Fix: three 25 ms re-stats before refusing. Not proven live (transient, not reproducible on demand); no new test for the transient (needs a link/unlink racing thread); existing refusal test passes.
3. Cross-machine hit: not provable yet; evidence will be: provenance numeric Nx key put by origin A, then `nx hits` counter rising after origin B's run, and the worker log line `remote cache hit`. Check after 06:05. Not claimed.
