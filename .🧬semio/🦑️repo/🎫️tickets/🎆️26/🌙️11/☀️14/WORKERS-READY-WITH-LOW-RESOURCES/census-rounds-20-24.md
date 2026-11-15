# Census of rounds 20 to 24 (read-only audit)

Scope: `W/workers/g1-m01` … `g1-m10` as on disk, all tries that ENDED after 2026-11-15T04:30Z. Data read up to about 07:01Z (2026-11-15). The folder is live: g1-m01, g1-m02, g1-m03 and g1-m07 are still writing. Times are UTC. "not in the files" means the data is not there. Quoted lines are verbatim unless marked (cut). `g1-m05` has no folder. Method: grep and tail on the files named below; no whole-file reads. The bootstrap logs (`~/native-bootstrap-round-*.log`) are not in W, so first error lines come from the service logs and sampler copies in W. Where the census says "not in the files", that is the reason.

Format reference: `census-rounds-13-19.md` (same layout).

Tries ended after 04:30Z: 26 (g1-m01 4, g1-m02 7 of which 3 are invalid, g1-m03 2, g1-m04 7, g1-m06 4, g1-m07 2). Still running at cut-off: g1-m01 round-17 try 7, g1-m02 round-24 try 1, g1-m03 round-14 try 8. g1-m04 and g1-m08 made no try after 04:30Z. g1-m10 made no try in the whole period (only the tools setup).

---

## 1. Per worker and per try (ended after 04:30Z, plus running tries)

Column key: "restore" = the `graph: generation … restored` line. "plan" = the first `graph: N stale projects …` line. "fleet" = fleet wait line. "setup" = nx setup tasks (see section 5). "Err" = first error line, verbatim.

### g1-m01 (macOS, Mac mini M4, 16 GB), round-17 rules

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r17.3 | 04:30:19 – 05:03:38 | 75 | not in the files | fleet: `graph: waiting for the fleet: 0 of 3 projects arrived`; end: `graph: nothing to wait for any longer: the fleet was asked for 4 stale projects and delivered 0 within 300 s` (p12-try-3, sampler copy) | 0 | `graph: nothing to wait for any longer: the fleet was asked for 4 stale projects and delivered 0 within 300 s` (the bootstrap refusal line is not in the files) | fleet refusal (small wanted list) |
| r17.4 | 05:06:41 – 05:43:55 | 1 | `graph: generation 229 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 19 s, from a state 28 min old with 3146 projects stale` | fleet: `graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 8 s ago, estimated time left: about 4 min` (05:23:55) | 0 | no error line in the files; sampler: `HARD STOP: memory pressure level 4 in two samples in a row` (05:42:54Z) | own hard stop, memory pressure level 4 (old rule) |
| r17.5 | 05:47:19 – 06:11:07 | 1 | `graph: generation 222 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 14 s, from a state 106 min old with 73 projects stale` | plan: `graph: project @semio-tech/mit-bestand-demonstrator (project) grew by 2265 MB, more than the 1756 MB this machine gives one project` (06:09:25) | 0 | sampler: `HARD STOP: memory pressure level 4 in two samples in a row` (06:10:28Z) | own hard stop, memory pressure level 4 (old rule) |
| r17.6 | 06:14:29 – 06:49:48 | 1 | not in the files | `graph: a peer builds the graph; this run waits as its front door alone, without a loader (40 s, generation 223 was the newest when it queued)`; later `graph: plan answered, managed 110 MB, resident 198 MB (released 0 MB)` | 0 | not in the files (rc 1, no error line, no hard stop) | unnamed rc 1 |
| r17.7 | 06:52:51 – running at 06:58 | – | not in the files | `graph: waiting for memory headroom (2016 of 842 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 966 MB and has waited 13 s …` | – | – | running |

Earlier tries 1 and 2 ended before 04:30Z and are excluded. Setup of round 16 (step 0) ended 04:10:24, exit 0 (section 5).

### g1-m02 (macOS, Mac mini Mac16,10, 16 GB), rounds 20 to 24

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r20.1 | 05:02:41 – 05:23:58 | 7 (own SIGINT at 05:23:22 after 907 s without a progress line) | `graph: generation 229 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 13 s, from a state 23 min old with 3173 projects stale` | `graph: 3173 stale projects, 0 taken from the fleet, 9 too heavy for this machine (…); it can give about 3512 MB per project), it can infer 88 of the others (22 per wave)` | 0 | no error line; first closure line: `[graph] closure-missing 0160357ce062320aa974ddb16dccf1c9666892d01001ee49f3c3879a50f42407 named by 1 projects: re-derived` | stall stop, native_rc=7; closure-missing fix seen (section 2) |
| r22.1 | 05:27:51 – 05:33:59 | 1 | not in the files | none (EPERM before the plan) | 0 | `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)` then `where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=86077 ppid=86049` (05:33:21Z) | access denied (EPERM, launchd route) |
| r22.2 | 05:36:59 – 05:52:47 | 7 (own SIGINT at 05:52:07 by the stall monitor; it read the wrong log, the fleet line kept changing) | not in the files | plan: `graph: 175 stale projects, 0 taken from the fleet, 10 too heavy for this machine (…); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)`; fleet: `waiting for the fleet: 0 of 175 projects arrived; the server last progressed 20 s ago, estimated time left: about 11 h 13 min` … `… 344 s ago, estimated time left: about 3 min` | 0 | no error line; stopped by monitor | false stall stop (monitor bug, see section 6) |
| r23.1 | 05:52:58 – 05:53:37 | 1 | – | – | 0 | none; I sent one SIGINT after about 10 s (my mistake, see round-23.samples.txt line 18) | self-inflicted |
| r23.2 (INVALID: two runners side by side) | 05:53:42 – 06:04:13 | 75 | – | fleet: `waiting for the fleet: 0 of 175 projects arrived; the server last progressed 581 s ago, estimated time left: about 3 min` | 0 | not captured (run invalid) | invalid |
| r23.3 (INVALID: two runners) | 06:07:13 – 06:11:46 | 1 | – | – | 0 | `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)` (06:09:17Z), `where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=1242 ppid=1235` | invalid; access denied (EPERM) |
| r23.4 (INVALID, stopped by hand 06:23Z) | 06:14:44 – about 06:23 | 75 / stopped | `graph: generation 230 (5100 projects) restored from the remote cache … from a state 15 min old with 73 projects stale` (06:15:31) | fleet: `waiting for the fleet: 0 of 73 projects arrived; the server last progressed 54 s ago, estimated time left: about 3 min` | 0 | not captured | invalid |
| step 0 r19 | 04:18:16 – 04:32:38 | 1 (exit 1, 862 s) | – | – | – | `Internal error: Original physical body, drain and cleanup settlement failed: Protected maintenance and accepted drainage failed: Producer reader and completed input validation failed: Current complete Source binding or whole receipt changed; Original process admission closed` (cut) | setup, see section 5 |
| step 0 r20 | 04:32:55 – 05:02:41 | exit 1 (1786 s) | – | – | – | `The dependency store returned no complete retirement plan (malformed, 227696 bytes, first violation: /value/roles/names must contain at most 16 item(s)); nothing was retired. Run the setup again; if this repeats, the dependency store image is damaged and must be reset` | setup, see section 5 |
| step 0 r24 | 06:23:27 – 06:43:29 | exit 1 (1202 s) | – | – | – | same text as step 0 r20 (round-24.tools-setup.md line 34) | setup, see section 5 |
| r24.1 | 06:43:29 – running at 06:58 | – | not in the files | sampler: `graph: plan answered, managed 108 MB, resident 417 MB (released 0 MB)` | – | – | running |

The g1-m02 round-20 try 1 closure report (round-20.md) says it was fine to stop at 907 s; the round-22 rule (fleet wait with S under 600 is healthy) was not known at the time.

### g1-m03 (macOS, Mac mini M4, 16 GB), round-14 series

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r14.6 | 04:09:51 – 06:21:20 | 1 (monitor: `try 6 ended 2026-11-15T06:21:20Z native_rc=1`) | `graph: generation 221 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 8 s, from a state 31 min old with 0 projects stale` | first plan line (service line 178, 04:44Z): `graph: 277 stale projects, 0 taken from the fleet, 5 too heavy for this machine (…; it can give about 3512 MB per project), it can infer 48 of the others (12 per wave)`; `graph: waiting for the fleet: 0 of 277 projects arrived`; `graph: nothing to wait for any longer: the fleet was asked for 277 stale projects and delivered 0 within 300 s` | failed: `@semio-tech/framework-os-mcp-rs:build:prototype`, `workspace:deps-build-tools:prototype`, `workspace:deps-cargo:prototype` | `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)` (04:46:46.137Z) then `where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=88966 ppid=88925`; end: `Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=1` | access denied (EPERM) during fleet refusal; then 2 h of retries; Nx 130 at the end |
| r14.7 | 06:24:25 – 06:47:18 | 130 (own SIGINT at 06:46:36 after 15 min with no new progress line) | not in the files (`round-14.try-7.service.txt` line 2 is the plan) | plan: `graph: 1203 stale projects, 0 taken from the fleet, 5 too heavy for this machine (@semio-tech/repo-lib (about 9652 MB), …)`; last progress: `graph: attached to the graph build of another command on this machine (generation 232, 884 s), no second build is started` (06:46:54Z) | 0 | none; SIGINT ends it | stall stop; waiting on another build (new line, section 6) |
| r14.8 | 06:50:36 – running at 06:58 | – | – | – | – | – | running |

Try 5 (03:52:07 – 04:06:50, rc 1, closure-missing=1, `The resident service met an unexpected failure`) and tries 1 to 4 ended before 04:30Z and are excluded.

### g1-m04 (Linux, SteamOS, 16 GB), round-14 series, tries 6 to 12

| Round.try | Start – end (UTC) | native_rc | First plan (stale / taken / too heavy / asked) | Fleet | Peak RSS of largest process | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r14.6 | 04:13:38 – 04:41:31 | 75 | 517 / 0 / 75 / 517 | `nothing to wait for … delivered 24 within 1369 s; 493 remain` | 1963 MB | `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 517 and delivered 24 within …` (bootstrap line, cut; per worker report) | fleet refusal, 0 MB grant |
| r14.7 | 04:44:33 – 04:57:52 | 75 | 540 / 0 / 77 / 540 | 0 delivered (worker report: 414 s) | 1844 MB | `… it can infer only 0 of the stale projects …` (cut) | fleet refusal, 0 MB grant |
| r14.8 | 05:00:56 – 05:17:56 | 75 | 540 / 0 / 77 / 540 | 0 delivered (667 s) | 1754 MB | same class (cut) | fleet refusal, 0 MB grant |
| r14.9 | 05:21:01 – 05:38:02 | 75 | 540 / 0 / 77 / 540 | 0 delivered (302 s) | 6722 MB | same class (cut) | fleet refusal, 0 MB grant |
| r14.10 | 05:41:04 – 06:00:08 | 75 | 540 / 0 / 77 / 540 | 0 delivered (301 s) | 6100 MB | same class (cut) | fleet refusal, 0 MB grant |
| r14.11 | 06:03:09 – 06:18:30 | 75 | 540 / 0 / 77 / 540 | 0 delivered (301 s) | 5729 MB | same class (cut) | fleet refusal, 0 MB grant |
| r14.12 | 06:21:32 – 06:36:13 | 75 | 540 / 0 / 77 / 540, then 703 / 0 / 5 / 703 | 0 delivered within 301 s | 1805 MB | `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 703 and delivered 0 within 301 s, 703 are still missing` (round-14.tail.txt) | fleet refusal, 0 MB grant |

Plan lines quote (try 12, service log): `graph: 540 stale projects, 0 taken from the fleet, 77 too heavy for this machine (@semio-tech/framework-os-dev (about 22726 MB), …; it can give about 0 MB per project), it can infer 0 of the others (0 per wave)`. Source for the table: `round-14.md` (worker report) and `round-14.tries.txt`. Setup did not run (`workspace:setup` never got past the graph step). Round-24 step 0 is in section 5.

### g1-m06 (macOS, Mac mini, 16 GB), round-18

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r18.6 | 04:28:44 – 04:30:07 | 75 | none | none | 0 | `Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure` | missing package (dependency install) |
| r18.7 | 04:33:15 – 04:34:36 | 75 | none | none | 0 | same | same |
| r18.8 | 04:37:46 – 04:39:12 | 75 | none | none | 0 | same | same |
| r18.9 | 04:42:18 – 04:43:40 | 75 | none | none | 0 | same; then `The stamps moved again after the restart: run the command again.` | same |

Machine note in `round-18.md`: Mac restarted 03:57Z; launcher unchanged; `repo tools setup all` step 0 is in section 5. The worker has written nothing since 04:43:50Z. Its round-18 report says `Rounds 12 to 17 were not run here`.

### g1-m07 (macOS), rounds 18 and 21

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r18.1 | 04:09:43 – 04:33:12 | 1 (hard stop 04:31:06, one SIGINT after 15 min without a progress line) | not in the files | headroom: `graph: waiting for memory headroom (691 of -6 MB reserved, 1 projects running): …` (negative reserve, new) | 0 | `Internal error: Original Script process interrupted` (tries file) | own SIGINT, 15 min no progress (old rule) |
| r21.1 | 06:17:58 – 06:58:42 | 130 (hard stop 06:57:41, one SIGINT) | not in the files | fleet at the stop: `graph: waiting for the fleet: 0 of 703 projects arrived; the server last progressed 430 s ago, estimated time left: about 47 min` (06:57:39) | failed: `workspace:setup` (state.json) | none; ended by SIGINT | SIGINT during a healthy fleet wait (see section 6) |

### g1-m08 (macOS), round-18 only

No try ended after 04:30Z. Last try: `round-18-try-1-result.json`: start 03:55:11Z, end 04:13:44Z, native_rc 75, fleet_refusal true. `next.md` (06:21:38Z) says `round: 24` and "follow ROUND.md round 24". The worker has written nothing else since 06:21:38Z.

### g1-m10 (macOS), round-16 tools setup only

No native try in rounds 16 to 24. Tools setup only (section 5). Last write `round-16.tools-progress.md` at 04:33:26Z.

---

## 2. Fix → evidence on a worker

| Expected line (ROUND.md) | Seen on a worker and try | Verbatim example |
|---|---|---|
| `3512 MB per project` (first plan line) | **Seen**: g1-m02 r22.2 (05:36–05:52), g1-m02 r20.1 (05:02–05:23), g1-m03 r14.6 (04:09–06:21). Not seen on g1-m04 (Linux, `0 MB`) or on g1-m01 r17.3–17.7 in the files | g1-m02 r22.2: `it can give about 3512 MB per project), it can infer 192 of the others (48 per wave)` (exact target); g1-m02 r20.1: `… it can give about 3512 MB per project), it can infer 88 of the others (22 per wave)`; g1-m03 r14.6: `… it can give about 3512 MB per project), it can infer 48 of the others (12 per wave)` |
| `graph: plan stage … settled from X to Y MB` | **Seen**: g1-m03 r14.6 (service lines 229, 278, 298, 443) | `graph: plan stage inference settled from 6635 to 6906 MB`; `graph: plan stage inference settled from 6677 to 5856 MB`; `graph: plan stage result settled from 6898 to 6548 MB`. Invalid run g1-m02 r23.2 shows `graph: plan stage inference settled from 7654 to 9360 MB` (the second number is higher) |
| `[graph] closure-missing … refetched \| re-derived \| asked of the fleet` | **Seen, all three words**: g1-m02 r20.1 (refetched 2, re-derived 2, asked of the fleet 1); g1-m03 r14.7 and round-14.stall.txt (re-derived) | `[graph] closure-missing 7ee0751a4ac8e51565a6b2bcf3cd4ed4ebeafdb2d55e634b9e1104385797f4ff named by 3119 projects: refetched`; `[graph] closure-missing 135d24b504cd908d498056c666c80d6175c19c9b1f932e48805a51d20f2d569e named by 3 projects: asked of the fleet`; `[graph] closure-missing 7143a7deaa3ba919893aefa5354df74795ac63f70f9e90262dc144994f2965bb named by 70 projects: re-derived` |
| `where: root=… entry=… route=…` | **Seen**: g1-m03 r14.6 (4 times), g1-m02 r22.1, g1-m02 r23.3 (invalid) | `where: root=/Users/ueli/Documents/semio/.cache/b entry=.reset.json code=EPERM route=launchd pid=86077 ppid=86049` (g1-m02 r22.1, 05:33:21Z); `… pid=88966 ppid=88925` (g1-m03 r14.6, 04:46:46Z) |
| `access-proof version=3` | **Not seen** in any file (grep over all W). Only in the round-24 scripts (`g1-m02/run-round-24.py`, `g1-m04/run-tries-24.sh`) | not in the files |
| a line saying the launchd route was refused for this boot | **Not seen** in any file | not in the files |
| `waiting for the fleet: x of N projects arrived; the server last progressed …` | **Seen**: g1-m01 r17.4 (05:23), g1-m02 r22.2 (05:36–05:52), g1-m07 r21.1 (06:57), g1-m02 r23.2 and r23.4 (invalid) | `graph: waiting for the fleet: 0 of 3146 projects arrived; the server last progressed 8 s ago, estimated time left: about 4 min` (g1-m01); `… 0 of 175 projects arrived; the server last progressed 20 s ago, estimated time left: about 11 h 13 min` (g1-m02 r22.2); `… 0 of 703 projects arrived; the server last progressed 430 s ago, estimated time left: about 47 min` (g1-m07 r21.1) |
| `waiting for memory headroom (… needs X MB and has waited …` | **Seen**: g1-m01 r17.4–17.7; g1-m03 r14.6 (281 lines); g1-m04 r14.6 (67 lines); g1-m07 r18.1 (142 lines); g1-m02 r23.2 (invalid) | `graph: waiting for memory headroom (768 of 262 MB reserved, 1 projects running): @semio-tech/mit-bestand-bericht needs 768 MB and has waited 0 s; after 60 s it runs alone …` (g1-m01 r17.4, 05:40:55Z) |
| per-scope `restored` / `installed` lines of `repo tools setup all` | **Not seen**. The setup logs in W have no per-scope restored or installed line. The only match is the header `Funding the dependency store: the dependencies of every scope are installed into it now` (g1-m02 step 0 r24) | not in the files |
| `Complete acquired dependency retirement plan required` (blocker from round 16) | **Not seen after 04:30Z**. Last seen in the census of rounds 13 to 19 (g1-m02 step 0 of round 16, 03:21–03:47Z). Other workers only quote it in notes | not in the files (after 04:30Z) |
| `Cannot find package '@semio-tech/pack-json-decode-ts'` | **Seen** (still a blocker): g1-m06 step 0 r18 (04:04Z) and tries 6 to 9 (04:28–04:43Z) | see section 1, g1-m06 |

Fixes that did not land on a worker as described:
- Mac grant `3512 MB per project` followed by `it can infer 192 of the others (48 per wave)`: exact on g1-m02 r22.2 only. In the other tries the inference count is 88, 48 (g1-m03) or 36 and 68 in the runs of round 22 and 23.
- Linux (g1-m04): `0 MB per project` in every plan line of tries 6 to 12 (round 14 fix for Linux not seen).

---

## 3. Ranked distinct blockers since 04:30Z

Count = tries (of the 26 ended) or setup runs affected. Time = first and last in the files.

1. **Fleet refusal: `nothing to wait for any longer … delivered 0 within S s`** (or `delivered 24 within 1369 s`). Tries: g1-m04 r14.6–r14.12 (7), g1-m03 r14.6 (4 windows of 300 s; 277, 104, 633, 648 stale), g1-m01 r17.3 (1). First 04:44:13Z (g1-m03), last 06:36:12Z (g1-m04). Signature: `graph: nothing to wait for any longer: the fleet was asked for 703 stale projects and delivered 0 within 301 s; 703 remain`.
2. **Zero MB grant per project (Linux only)**: `it can give about 0 MB per project), it can infer 0 of the others (0 per wave)` on g1-m04 tries 6 to 12 (7 tries; 04:13–06:36Z). Mac machines show 3512 MB now (section 2).
3. **Dependency store / `repo tools setup all` exit 1**: g1-m02 step 0 r19 (04:18–04:32, 862 s), r20 (04:32–05:02, 1786 s), r24 (06:23–06:43, 1202 s); g1-m10 attempt 2 (04:06–04:18, 684 s); g1-m07 round-17 setup (04:33–06:17, 6254 s). Signatures: `Internal error: Original physical body, drain and cleanup settlement failed: … Current complete Source binding or whole receipt changed` and `The dependency store returned no complete retirement plan (malformed, 227696 bytes, first violation: /value/roles/names must contain at most 16 item(s)); nothing was retired.` (last seen 06:43Z).
4. **Missing package `@semio-tech/pack-json-decode-ts`**: g1-m06 step 0 r18 (04:04–04:05, exit 75) and tries 6 to 9 (04:28–04:43Z), 5 tries. Signature: `Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure`.
5. **Access denied (EPERM on `.cache/b/.reset.json`, route=launchd)**: g1-m03 r14.6 (04:46:46Z, 05:32:23Z, four times), g1-m02 r22.1 (05:33:21Z), g1-m02 r23.3 (06:09:17Z, 06:11:29Z, invalid run). Signature: `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)`. No `access-proof version=3` line anywhere, so the fix round 24 asks for is not on any worker.
6. **Stall stops (no new progress line for 15 min or 907 s, own SIGINT)**: g1-m07 r21.1 (06:57:41Z, during a healthy fleet wait), g1-m03 r14.7 (06:46:36Z, waiting for another build), g1-m07 r18.1 (04:31:06Z), g1-m02 r20.1 (05:23:22Z), g1-m02 r22.2 (05:52:07Z, monitor read the wrong log), g1-m02 r23.1 (05:53, my error). Count 6.
7. **Memory pressure level 4 hard stops (old rule)**: g1-m01 r17.4 (05:42:54Z), r17.5 (06:10:28Z). Count 2.
8. **Nx 130 at the end of a try**: g1-m03 r14.6 (06:21:20Z): `Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=1`. Count 1.
9. **rc 1 with no error line in the files**: g1-m01 r17.6 (06:49:48Z). Count 1, not explained.
10. **Own SIGINT caused rc 1 or 130**: g1-m07 r18.1 (`Internal error: Original Script process interrupted`), g1-m02 r23.1. Count 2.

---

## 4. Workers with nothing written since (newest file mtime, UTC, as of about 07:01Z)

| Worker | Newest file (UTC) | Silent since | Note |
|---|---|---|---|
| g1-m01 | 07:00:52 `round-17.p12-try-7.txt` | (writing) | try 7 running |
| g1-m02 | 07:00:58 `round-24.p12-try-1.txt` | (writing) | round-24 try 1 running |
| g1-m03 | 07:00:58 `round-14.try-8.p12-samples.txt` | (writing) | try 8 running |
| g1-m04 | 06:38:30 `round-24.step0.txt` | 22 min | round-24 step 0 started 06:38:29; no end line in the files |
| g1-m06 | 04:43:50 `state.json` | about 2 h 17 min | last try ended 04:43:40; no later round |
| g1-m07 | 06:59:05 `state.json` | (writing) | round-21 try 1 ended 06:58:42 |
| g1-m08 | 06:21:38 `next.md` | 39 min | no try since round 18 (03:55–04:13) |
| g1-m10 | 04:33:26 `run-tools-setup.sh` | about 2 h 27 min | round-16 setup attempt 3 started 04:33:23; no end in the files |

---

## 5. Every `repo tools setup all` run

| Machine | Round | Start (UTC) | Wall | Exit | Last error line (verbatim) |
|---|---|---|---|---|---|
| g1-m01 | 16 (step 0) | 03:38:49 | 1895 s | 0 | none. Last line `[repository:command] still running after 1870 s` (`round-17.tools.txt`) |
| g1-m02 | 16 (step 0) | 03:21:32 | about 26 min | 1 | `Internal error: Complete acquired dependency retirement plan required` then `repository:command failed with exit code 1` (census 13–19) |
| g1-m03 | 16 (step 0) | 03:25:18 | 1607 s | 1 | not in the files (`round-14.monitor.txt` line 18 `step 0 ended 2026-11-15T03:52:06Z exit=1 wall=1607s`) |
| g1-m07 | 18 (attempt 1) | 03:56:32 | 791 s | 1 | not in the files (`round-18.tools-setup-attempt1.txt`) |
| g1-m06 | 18 (step 0) | 04:04:15 | 91 s | 75 | `Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure` (then `tools_rc=75`) |
| g1-m10 | 16 (attempt 1) | about 03:08 | ended by SIGTERM at about 04:06:35Z | none (SIGTERM) | `Internal error: Original Script process terminated` (`round-16.tools-progress.md` line 31) |
| g1-m10 | 16 (attempt 2) | 04:06:42 | 684 s | 1 | `Internal error: Original physical body, drain and cleanup settlement failed: ... Current complete Source binding or whole receipt changed; Original process admission closed` (line 33) |
| g1-m10 | 16 (attempt 3) | 04:33:23 | – | not in the files | not in the files |
| g1-m02 | 19 (step 0) | 04:18:16 | 862 s | 1 | `Internal error: Original physical body, drain and cleanup settlement failed: … Current complete Source binding or whole receipt changed; Original process admission closed` |
| g1-m02 | 20 (step 0) | 04:32:55 | 1786 s | 1 | `The dependency store returned no complete retirement plan (malformed, 227696 bytes, first violation: /value/roles/names must contain at most 16 item(s)); nothing was retired. Run the setup again; if this repeats, the dependency store image is damaged and must be reset` |
| g1-m07 | 17 (setup) | 04:33:44 | 6254 s | 1 | not in the files (`round-17.tools-setup.txt` has only the start and end lines) |
| g1-m02 | 24 (step 0) | 06:23:27 | 1202 s | 1 | same text as round 20 (`round-24.tools-setup.md` line 34) |
| g1-m04 | 24 (step 0) | 06:38:29 | – | not in the files | not in the files (`round-24.step0.txt` has only the start line) |

Per-scope `restored` / `installed` lines: none in any setup log (section 2). No run printed `Complete acquired dependency retirement plan required` after 04:30Z.

---

## 6. Anything new and unnamed

- `graph: attached to the graph build of another command on this machine (generation 232, 884 s), no second build is started` (g1-m03 r14.7, 06:46:54Z, 128 sampler lines). The try then waited 15 min and was stopped.
- `graph: a peer builds the graph; this run waits as its front door alone, without a loader (550 s, generation 222 was the newest when it queued)` (g1-m01 r17.6, 47 sampler lines; g1-m07 r21.1, 7 lines).
- `graph: waiting for memory headroom (691 of -6 MB reserved, 1 projects running)` and `(496 of -147 MB reserved …)`: a negative reserve (g1-m07 r18.1, 04:31Z). The headroom accounting goes below zero.
- `graph: No verified change journal cursor is recorded for generation 222; verifying the tree once.` (g1-m07 r21.1, 55 lines).
- `Internal error: Owned Nx invocation failed (130): nx run workspace:setup --parallel=1` (g1-m03 r14.6 end).
- The round-24 setup error text now says `… the dependency store image is damaged and must be reset` (g1-m02 r20, r24). The worker is told to reset the store; no reset was run.
- Monitor bugs: g1-m02 round-22 monitor read the alphabetically last service log (not the newest), which made it SIGINT a healthy fleet wait (05:52:07Z). g1-m07 round-21 monitor applied the 15-min no-progress rule to a fleet wait with `server last progressed 430 s ago` (06:57:41Z). Round 22 says the 15-min rule does not apply to that phase.
- Two runners side by side on g1-m02 from 05:53Z to 06:23Z: tries 2 to 4 of round 23 are invalid (`round-23.samples.txt` lines 802 and the NOTE).
- Peer edits: the g1-m02 round-20 report says an architecture course session edited shared-worktree files during step 0 of rounds 19 and 20 (`round-20.md` line 25). Step 0 failures on g1-m02 may come from that.
- Time zone: g1-m10 writes `05:05 CET` for the SIGTERM that the file also stamps `04:06:35Z`. Mixed time zones, not checked.
- `[capacity] run: grown from 4 to 6 of 8 job(s) after 390s … memory freed up` (g1-m04 r14.6). New capacity behaviour, info only.
- `no slice worker` sampler lines (g1-m01 324 lines, g1-m07): the slice sampler finds no slice worker while the run waits. Probably a sampler state, not a failure.

---

## Notes on data gaps

- The bootstrap logs (`~/native-bootstrap-round-*.log`) and the tools-setup logs (`~/tools-setup-round-*.log`) are not in W. First error lines of tries are taken from service-log copies. Where no copy exists the table says "not in the files".
- g1-m01 round-17 tries 3 to 7 have sampler files only. g1-m07 round-17 setup has only start and end lines.
- Nothing about `access-proof` or the launchd refusal appears in any file, so these fixes cannot be marked as seen.
