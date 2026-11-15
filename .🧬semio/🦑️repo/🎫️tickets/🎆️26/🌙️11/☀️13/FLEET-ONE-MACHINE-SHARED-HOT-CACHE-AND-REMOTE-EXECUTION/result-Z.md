# Package Z: workers that stay present, offload that fires end to end (2026-11-14 09:10 to 10:05)

Scripts: `🧰️scripts/Z/` (`proof.ts` + `mkworkspace.py` = the proof, `stage.py`/`take.py`/`land.py`/`land_docs.py` = scratch staging and landing, `table.py`). Landing: `landings.md` "Package Z".

## Verdicts
1. Worker presence, disk: the 3.5 GiB reading was real (statfs on `.cache/f` equals `df`: 60.05 GiB both at 09:12) and healed by itself 60 s later (agent log: "without a worker: 3.5 GiB free" 08:07:57Z, registered 08:08:57Z). The model was still wrong: a floor of 20 GiB at join. Now judged with every admission (`worker.disk`: 3 GiB reserve + 0.5 GiB per action, slots = floor((free-3)/0.5), at most the cores): 60 GiB gives 10 slots, 5.9 GiB (g1-m01) gives 5, 3.5 GiB gives 1, below 3.5 GiB none. The worker stays registered, says `disk-full` (EN/DE, with the numbers) and serves again within 2 s of the space, no restart (test).
2. Real disk need of a worker, measured: blob cache 256 MiB; outputs of 176 shared-target rlib/rmeta/dylib: p50 190 KB, p90 5.9 MB, p99 = max 140 MB; the 25-unit proof shipped 0.9 MB and returned 0.28 MB. Worst case 10 slots is about 3 GiB, typical below 50 MB. 20 GiB was 7x too high; yes, a small-disk machine serves small units.
3. Worker presence, crashes: the 08:05 crash was `Cannot find module …host-capacity/🎛️control` on all agents while a folder was replaced; the daemon's back-off (1 s doubling to 60 s) plus a 60 s readiness re-check put workers back after about 2 to 3 min. The launcher now restarts a crashing role itself (1 s doubling to 15 s, log line with the status); readiness re-check 15 s; static readiness no longer contains disk.
4. Second cause of "rejoined without a worker", found live at 09:54 right after my own landing restarted the scheduler: after any scheduler restart every agent looped on `403 this worker belongs to another origin` (the scheduler's owner map is memory only, an unknown id was treated as foreign). Fixed in `🖥️server` (`mine`: unknown id passes, the worker then re-registers); test fails before, passes after (`🛰️sidecar/🧪️tests`, scheduler SIGKILL + restart, agent not restarted). After the landing all six reporting machines had a worker (10:01).
5. Counters 539 refused / 289 fell back: no reasons were kept; files last changed 05:36 / 05:55, i.e. all events come from the benchmark runs of J/O (01:20 to 05:55), none since. Reconstruction from their reports (not additive): refused = no-idle-machine/queue-timeout, mostly the policy refusals mislabelled as no-idle-machine (fixed by O) and probe flaps (fixed by O); fell back = 130 incomplete-inputs + 94 infrastructure (cache store 500 on a disk below 25 GiB) + 56 worker-policy + a few. Now counted per reason in `.cache/f/offload-counters/reason/<reason>` (`readOffloadReasonCountersV1`).
6. The top FAULT, never seen live because nobody offloaded since T's landing: the worker allow-list (`🧱️actions`) refused what T's planner sends (`SEMIO_CWD_NAME`, `SEMIO_SOURCE`, `SEMIO_REMAPS`, proc-macro `-C link-arg=-Wl,-install_name,@rpath/lib…dylib`), and the cluster tests hid it (they run workers with `actions=any`). Scratch fleet with the old code: 0 of 25 offloaded, 25 x worker-policy. Fixed with tight validation (relative, no `..`, bounded, install name only for `@rpath/lib<name>[-hash].dylib`), 2 new action tests with attacks, and the determinism test now asserts the planner output passes the validator (fails 3 of 5 on the old validator).
7. g1-m07: not a worker problem. Its replica applies changes 315 s late (apply p50 315533 ms against 145-632 ms elsewhere, attests up to 350 s apart); sidecars start only while the daemon is healthy, so its agent comes and goes (absent 09:08, idle 09:46 and 10:01). Needs its owner; prompt below.
8. g1-m04 (linux), g1-m05 (windows): NOT reporting and NOT shown. `build-agent` has `requires: platform-peer` in `🤝️shared-worktree/🗺️topology/🔣️.json`, and each is alone on its platform (m05 also has apply p50 309 s). Not changed (replica packages W/Y). One-word fix for them: `requires: "none"` for build-agent; on Windows the agent would then report load only (offloads from Windows are declined by the client), check that the sidecar can start there.
9. Port wording: README and catalog already said 8980 gRPC / 8981 HTTP; the ambiguous example "Run the service on 127.0.0.1:8980" (catalog + help snapshot, EN/DE) now names both, and a README bullet (EN/DE) states ports, the published `execution.endpoint` (gRPC) and the HTTP routes. The mismatch lived in the audit/design wording.
10. `--offload=auto` end to end: SEEN, live and in scratch (below). Output identity: 50 of 50 rlib/rmeta byte-identical to a local compile in every phase, scratch and live.

## The proof (`🧰️scripts/Z/proof.ts`)
Standalone cargo workspace outside the worktree (4 layers x 6 crates + top = 25 libs, const loop about 2.6 s each, nightly-2026-07-20, `[workspace]`, private target dir, 45 MB scratch), real `cargo build` through the repository wrapper with the environment `unitCacheEnvironmentV1` builds (shim, lane, remote cache env), salt per phase so no cache is hit. Per phase: local reference compile, then the offloaded compile into the same paths, artifacts compared by sha256. Local pressure for `auto`: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=2048` (the host-capacity simulation hook), calm: 65536. Scratch mode starts a signed fleet on this Mac (scheduler, 3 real agents, signed cache, test identities, own store). Live mode uses this machine's real identity and the real fleet (read of facts, signed calls); it writes counters into `.cache/f` and 3 x 25 small entries into the real cache.
- Scratch, old code: always 0/25 (25 x worker-policy). Scratch, new code: always 25/25, auto 25/25 (median round trip 2780 / 3008 ms), calm 0/9, identical 50/50 each.
- LIVE, real fleet (10:01 CET, after landing): always 15/25, auto 19/25, calm 0/25; every cargo exit 0; identical 50/50 in all three. The units that stayed local were `no-idle-machine` (the minis' headroom was used up at that moment; g1-m02 was refusing for memory): a correct decision, counted under its reason. Offloaded per machine (scheduler `offloadedIn`): always m01 3, m03 3, m06 4, m07 5; auto m01 4, m03 5, m06 4, m07 6. Round trip median 2723 / 2768 ms for a unit that compiles in about 2.6 s locally: the overhead is below 150 ms. Bytes (always): shipped 632 KB, uploaded 125 B (the store already had the files), returned 166 KB. Wall time did not improve (24 units of 2.6 s fit this machine's free cores; offload pays only when the requester would wait).

### live phase `always`: exit 0, wall 21197 ms (local reference 17209 ms), offloaded 15/25, median round trip 2723 ms, shipped 632338 B, uploaded 125 B, returned 165978 B, byte-identical 50/50, per machine (scheduler `offloadedIn`) {'g1-m01': 3, 'g1-m03': 3, 'g1-m06': 4, 'g1-m07': 5}, declines {'no-idle-machine': 10}

| crate | ran on (worker host) | round trip ms | shipped B | uploaded B | returned B | stayed local because |
|---|---|---|---|---|---|---|
| pz000 | m6-01-lan-46255 | 2656 | 464 | 7 | 10318 |  |
| pz001 | m4-04-lan-78704 | 2521 | 464 | 7 | 10318 |  |
| pz002 | m4-02-lan-68058 | 5299 | 465 | 7 | 10318 |  |
| pz003 | m4-03-lan-67568 | 3000 | 465 | 7 | 10318 |  |
| pz004 | m4-03-lan-67568 | 3007 | 465 | 7 | 10318 |  |
| pz005 | m4-02-lan-68058 | 5317 | 465 | 7 | 10318 |  |
| pz100 | m6-01-lan-46255 | 1864 | 15123 | 9 | 11166 |  |
| pz101 | local |  |  |  |  | no-idle-machine |
| pz102 | local |  |  |  |  | no-idle-machine |
| pz103 | m4-04-lan-78704 | 2106 | 15123 | 9 | 11166 |  |
| pz104 | local |  |  |  |  | no-idle-machine |
| pz105 | local |  |  |  |  | no-idle-machine |
| pz200 | local |  |  |  |  | no-idle-machine |
| pz201 | local |  |  |  |  | no-idle-machine |
| pz202 | local |  |  |  |  | no-idle-machine |
| pz203 | local |  |  |  |  | no-idle-machine |
| pz204 | local |  |  |  |  | no-idle-machine |
| pz205 | local |  |  |  |  | no-idle-machine |
| pz300 | m4-04-lan-78704 | 3978 | 68748 | 9 | 11488 |  |
| pz301 | m4-02-lan-68058 | 2222 | 68748 | 11 | 11488 |  |
| pz302 | m4-02-lan-68058 | 2318 | 68748 | 9 | 11488 |  |
| pz303 | m6-01-lan-46255 | 2723 | 68748 | 9 | 11488 |  |
| pz304 | m4-04-lan-78704 | 2153 | 68748 | 9 | 11488 |  |
| pz305 | m4-03-lan-67568 | 3906 | 68748 | 9 | 11488 |  |
| pztop | m4-02-lan-68058 | 5074 | 186816 | 9 | 12810 |  |

### live phase `auto`: exit 0, wall 19253 ms (local reference 16447 ms), offloaded 19/25, median round trip 2768 ms, shipped 829139 B, uploaded 133 B, returned 211394 B, byte-identical 50/50, per machine (scheduler `offloadedIn`) {'g1-m01': 4, 'g1-m03': 5, 'g1-m06': 4, 'g1-m07': 6}, declines {'no-idle-machine': 6}

| crate | ran on (worker host) | round trip ms | shipped B | uploaded B | returned B | stayed local because |
|---|---|---|---|---|---|---|
| pz000 | m6-01-lan-46255 | 2345 | 462 | 5 | 10314 |  |
| pz001 | m4-02-lan-68058 | 2737 | 462 | 5 | 10314 |  |
| pz002 | m4-04-lan-78704 | 2433 | 463 | 5 | 10314 |  |
| pz003 | m4-02-lan-68058 | 2768 | 463 | 5 | 10314 |  |
| pz004 | m4-03-lan-67568 | 4205 | 463 | 5 | 10314 |  |
| pz005 | m4-03-lan-67568 | 4207 | 463 | 5 | 10314 |  |
| pz100 | local |  |  |  |  | no-idle-machine |
| pz101 | local |  |  |  |  | no-idle-machine |
| pz102 | local |  |  |  |  | no-idle-machine |
| pz103 | local |  |  |  |  | no-idle-machine |
| pz104 | local |  |  |  |  | no-idle-machine |
| pz105 | local |  |  |  |  | no-idle-machine |
| pz200 | m4-04-lan-78704 | 2572 | 37897 | 9 | 11300 |  |
| pz201 | m6-01-lan-46255 | 4109 | 37897 | 9 | 11300 |  |
| pz202 | m4-02-lan-68058 | 2243 | 37897 | 9 | 11300 |  |
| pz203 | m4-04-lan-78704 | 3142 | 37898 | 9 | 11300 |  |
| pz204 | m4-03-lan-67568 | 3744 | 37898 | 9 | 11300 |  |
| pz205 | m4-02-lan-68058 | 2294 | 37898 | 9 | 11300 |  |
| pz300 | m4-02-lan-68058 | 3177 | 68710 | 7 | 11484 |  |
| pz301 | m6-01-lan-46255 | 2439 | 68710 | 7 | 11484 |  |
| pz302 | m4-04-lan-78704 | 1963 | 68710 | 7 | 11484 |  |
| pz303 | m4-03-lan-67568 | 4746 | 68710 | 7 | 11484 |  |
| pz304 | m4-03-lan-67568 | 2853 | 68710 | 7 | 11484 |  |
| pz305 | m4-02-lan-68058 | 3168 | 68710 | 7 | 11484 |  |
| pztop | m6-01-lan-46255 | 2476 | 186718 | 7 | 12806 |  |

## Tests run on live code after landing (all green)
`🧪️tests/🔎️why` 7, `🧪️tests/💽️disk` 6 (new: slot table, python oracle on 400 volumes, agent keeps its registration and recovers without restart, refusal "disk" is a valid task result, launcher restarts a crashing role / back-off / stop during back-off / status 75), `🧱️actions` 4 (2 new), `🚀️offload/🧪️tests` 25 (1 new: reasons), `…/🎯️determinism` 5, `…/🌐️cluster` 8 (bytes asserted), `🛰️sidecar/🧪️tests` 3 (scheduler restart added), `🧪️tests/🌐️fleet` 9, `🧪️tests` main 17, `🗓️scheduler` 8, `⌨️command` 7, `🚢️fleet-status` 14, `⚙️operations` 15. Real `repo` command through the path: `repo fleet status --json`.

## Landed files
R = `🏗️build/🌐️remote-execution`: `R/🟦️.ts` (refusal "disk"), `🏃️execution/🧬️schema/🌐️remote-execution.json` (disk policy, refusal), `🔣️policy.json` (`worker.minimumFreeGigabytes` replaced by `worker.disk`; part of the replica daemon's code digest, the daemon rolls once), `R/🛰️agent/🟦️.ts`, new `R/🛰️agent/💽️disk/🟦️.ts`, `R/🛰️agent/🩺️readiness/🟦️.ts`, `R/🛰️sidecar/🟦️.ts` (launcher), `R/🛰️sidecar/👷️agent/🟦️.ts`, `R/🌡️load/🔎️why/🟦️.ts` (named fields), `R/🧱️actions/🟦️.ts`, `R/🖥️server/🟦️.ts` (403 fix), `R/🚀️offload/🟦️.ts`, `📊️counters`, `🧬️schema`, tests above, `⚙️operations/🔣️catalog.json` + help snapshot, `🦑️repo/README.md`.

## NOT verified / know this
- Linux and Windows agents; the minis' own logs (I can only read g1-server's); the `disk-full` text on a real small-disk mini (g1-m01 has 5.9 GiB, its row now shows a worker); the 409 divergence adopt path end to end (needs two origins uploading different bytes for one key; all 150 compared files were identical, so it never triggered; unit coverage of T stands).
- A crash-looping role is now invisible to the daemon (`restarts 0`); only the log line "ended with status N" shows it.
- Wall-clock gain of offload was not shown (see above). The proof leaves about 75 small entries of the `pz*` crates in the real fleet cache.
- Shared slips: the landing was done with `landing.lock` (V's lock waited for, mine taken 09:53 and 09:58); the 403 fix went in as a second small landing.

## Prompt for a Mac mini owner (disk use by top-level consumer, read-only)
"Report, without deleting anything, where this Mac's disk goes. Run: `df -h /System/Volumes/Data`; `tmutil listlocalsnapshots /`; `du -xk -d 1 ~ 2>/dev/null | sort -rn | head -15`; `du -xsk ~/Library/Caches ~/Library/Developer ~/.cargo ~/.rustup ~/.bun ~/.cache ~/.npm 2>/dev/null`; `du -xk -d 1 /Users/Shared/semio/.cache 2>/dev/null | sort -rn | head`; `du -xsk /Users/Shared/semio-store /private/var/folders 2>/dev/null`. Print one table: consumer, GiB, and whether it can be rebuilt (cache) or not. Also paste `tail -30 /Users/Shared/semio/.cache/f/sidecars/build-agent.log` and `uptime`."
