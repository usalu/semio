# g1-m10 round 30

Machine: Mac arm64, macOS 27.2.1, 16 GiB RAM, about 131 GiB free. Replica code agreement before the series: g1-m10 and g1-server `203e01b3594f...`, incoming 0 (round-30.agreement-wait.txt). Step 0 (`repo tools setup all`): never exit 0, see round-16.tools-setup.report.md; attempt 6 was cut off by SIGTERM when the earlier session ended (no metrics).

## Try 1
- 2026-11-15T17:39:10Z to 19:17:09Z, wall 5878 s, **native_rc=75**, marker absent, ready false.
- access-proof: `2026-11-15T17:40:02.689Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=launchd pid=98274 ppid=98267 responsible=98267` (no refusal, no EPERM, no closure-missing line).
- Plan line: `graph: 703 stale projects, 0 taken from the fleet, 43 too heavy for this machine (@semio-tech/s-dev-composition (about 23832 MB), workspace (about 20606 MB), @semio-tech/framework-rs (about 18218 MB), ... and 37 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); ...`
- Highest step reached: **step 6 of 6**. All `cut smaller` lines (steps 2 to 6; step 1 is the 16 GB start): round-30-try-1.cut-lines.txt. Slice worker sizes at the cuts: 10442, 10205, 10182, 6937, 7926 MB (limit 6553 MB).
- Final refusal (after step 6 of 6): round-30-try-1.refusal.txt: plan stage inference grew to 10478 MB, more than the 6553 MB a plan may take, only 3143 MB available (2048 MB reserve). Also seen: `graph: plan stage inference settled from 7067 to 7187 MB`.
- Fleet: `0 of 703 projects arrived from the fleet` at every sampled line; `the server last progressed` was current (0 to 154 s early in the try), estimated time left about 37 min then 31 min.
- Peaks (sampler, 30 s): run process 989 MiB, slice workers 7992 MiB, system top 7872 MiB. Peak sample 2026-11-15T19:09:13Z at `191 inferred here, 512 left; the server last progressed 132 s ago`. The graph child's own peak is the 10478 MB in the refusal (log value, not sampled). Closure entries 35031 before, 35074 after. No hard stop.
- Files: round-30-try-1.{log,metrics.json,samples.txt,top3.txt,service.txt,tail.txt,wrapper.txt,cut-lines.txt,refusal.txt}, round-30.tail.txt, state.json.
- Next: try 2 after 5 minutes.

## Try 2
- 2026-11-15T19:42:29Z (5 min after try 1 plus start delay) to 20:05:21Z, wall 1372 s, **native_rc=7** = observer stop `resident-above-8GiB` (rule of round 23/29: a process of the run above 8 GB resident). A slice worker was at 8448 MiB (8192 MiB limit) at 20:05:21Z, progress `[script] graph:slice-worker · Verifying input closures: 717 of 1435 closures`. The stop was by the rule, not a repo result. Not an error of the service: the service had reached step 4 of 6 and was continuing.
- access-proof: `2026-11-15T19:58:21.397Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=launchd pid=76095 ppid=76089 responsible=76089` (no refusal).
- Highest step reached: **step 4 of 6**. Cut lines (slice worker sizes 9871, 10130, 9888 MB for steps 2, 3, 4): round-30-try-2.cut-lines.txt. Settled lines: `plan stage verification settled from 7647 to 4270 MB`, `6708 to 6174 MB`, `6961 to 4279 MB`.
- Peaks (sampler): run process 1198 MiB, slice workers 8448 MiB, system top 7656 MiB. Closure entries 35074 before and after. No `taken from the fleet`/fleet-arrival line this try (the try stayed in the verification stage).
- Files: round-30-try-2.*, round-30.try2.tail.txt, state.json (native_rc 7).
- Note for the coordinator: the 8 GiB hard stop and the service's cut-smaller design conflict: the slice worker legitimately grows past 8 GiB (9.9 to 10.1 GB seen in cut lines) before the service cuts it down. Will keep applying the rule as written unless told otherwise.
- Next: try 3 after 5 minutes.

## Try 3
- 2026-11-15T20:20:37Z to 22:10:52Z, wall 6615 s, **native_rc=7** = observer stop `resident-above-8GiB` (slice worker 8210 MiB), at `Reading the project graph slice (4350 s): owners 111 of 703`. Highest step 3 of 6 (cut lines: round-30-try-3.cut-lines.txt, 10302 and 9990 MB). access-proof: `2026-11-15T21:11:58.366Z access-proof version=3 root=/Users/ueli/Documents/semio protected=Documents sample=ok reproof=on route=launchd pid=13690 ppid=13684 responsible=13684`.
- Fleet: `graph: 8 / 16 / 16 / 16 / 17 projects taken from the fleet with 0 closure entries, 0 of them did not hold here ...` then `19 projects taken from the fleet ... 19 of them did not hold here and were inferred again`. Largest taken-from-fleet count seen: 19 (per delivery batch; not cumulative).
- Peaks: run process 910 MiB, slice 8210 MiB, system top 7839 MiB. Closure entries 34968 before, 35262 after.
- Series ended here: ROUND.md became 31 (stop running setup yourself; see round-31.md). Files: round-30-try-3.*, round-30.try3.tail.txt, state.json.
