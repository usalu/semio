# Census of rounds 13 to 19 (read-only audit)

Scope: `W/workers/g1-m01` … `g1-m10` as they are on disk. Latest data point read: `g1-m01/round-17.samples.txt`, last line 04:31:20Z (2026-11-15). The folder is live, so later tries may exist. Times are UTC. "not in the files" means the data is not there. Quoted lines are verbatim, except where marked "(cut)". Full lines are in the named file.

Round 19 (`ROUND.md`, 04:25 UTC) has no output in any worker folder. `g1-m02` has `run-round-19.py` and `tools-setup-round-19.py` only. Round 19 says `native_rc=7` is not a listed code, so please read section 5 as well.

No worker folder for `g1-m05` exists. `g1-m10` has no `state.json` and no native try in rounds 13 to 19 (only step 0 of round 16 and the agreement wait of round 18).

---

## 1. Per worker and per try, round 13 on

Column key: "restore" = the `graph: generation … restored` line. "fleet" = `waiting for the fleet: x of N` or the fleet expiry line. "plan" = the plan refusal or the last plan stage line. "setup" = nx setup tasks reached (see section 4).

### g1-m01 (macOS, Mac mini M4 class)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13.1 | 01:40:51 – 01:45:57 | 75 | `generation 211 (5101 projects) restored … from a state 26 min old with ? projects stale` | `plan stage environment grew the plan by 4943 MB` (log); env 6853 MB | 0 | `The project graph could not be read on this machine yet: the plan stage environment grew to 6853 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (environment) |
| r13.2 | 01:49:01 – 02:05:06 | 75 | not in that try's log (restore in `host-007c265f8fa9.log` line 146) | fleet: `delivered 46 within 845 s`, 703 wanted | 0 | `The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 46 within 845 s, 657 are still missing…` (cut) | fleet refusal |
| r13.3 | 02:08:08 – 02:28:18 | 75 | not in files | fleet: `delivered 0 within 900 s`; `0 MB per project, can infer 0` (per report) | 0 | `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 703 and delivered 0 within 900 s, 703 are still missing…` (cut) | fleet refusal, 0 MB per project |
| r13.4 | 02:31:20 – 02:45:55 | 1 | not in files | none (service failed) | 0 | `The resident service met an unexpected failure (reference 675127cb); try again, the details under that reference are in the service log …` | service failure, closure-missing (see blocker 4) |
| r13.5 | 02:48:58 – 03:05:08 | 75 | not in files | plan inference: `grew to 7264 MB` | 0 | `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 7264 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (inference) |
| r13.6 | 03:08:10 – 03:34:50 | 75 | not in files | fleet: `delivered 199 within 1495 s`, 267 wanted | 0 | `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 267 and delivered 199 within 1495 s, 68 are still missing.` | fleet refusal, 0 MB per project |
| r17.1 | 04:10:29 – 04:15:36 | 1 | not in files | not in files | 0 | `04:15:36Z TRY 1 end: native_rc=1; hard stop: none; failed tasks: none; reference: b72e2264` (the bootstrap message of `b72e2264` is not in the files) | unnamed (blocker 9) |
| r17.2 | 04:18:38 – 04:27:16 | 75 | not in files | fleet: `delivered 0 within 301 s`; 656 wanted | 0 | `graph: nothing to wait for any longer: the fleet was asked for 656 stale projects and delivered 0 within 301 s; 656 remain…` (cut, from `round-17.p12-try-2.txt` 04:27:09Z) | fleet refusal (301 s window) |
| r17.3 | 04:30:19 – running at 04:31:20 | – | – | – | – | not in files (try in progress at cut-off) | – |

Setup tasks: none in any round 13 to 19 try. Step 0 of round 16 (`repo tools setup all`) ran 03:38:49 to 04:10:24, exit 0, wall 1895 s (`round-17.tools.txt`).

### g1-m02 (macOS)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13.1 | 01:40:24 – 01:46:04 | 75 | `generation 211 (5101 projects) restored … from a state 26 min old with ? projects stale` | `plan stage environment grew the plan by 4953 MB`; env 6712 MB | 0 | `The project graph could not be read on this machine yet: the plan stage environment grew to 6712 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (environment) |
| r13.2 | 01:49:37 – 02:06:43 | 7 (raw, `round-13.tail.txt` line 200) | none (new service log) | `703 stale projects, 0 taken from the fleet, 19 too heavy`; fleet 46 of 703, then no progress 905 s | 0 | `graph: 703 stale projects, 0 taken from the fleet, 19 too heavy for this machine (…` (cut); stop by runner: `no new progress line for 905 s` | stall stop, native_rc=7 (blocker 8) |
| r14.1 | 02:12:39 – 02:21:12 | 7 (per report) | `generation 216 (5100 projects) … from a state 4 min old with 0 projects stale` | `environment` 7.9 s; `0 projects taken from the fleet, 50 inferred here` | 0 | none; SIGINT sent by runner at 02:21:12Z when round 15 was published | interrupted, native_rc=7 |
| r14.2 | 02:22:18 – 02:38:52 | 75 | not in try log | fleet: `graph-wanted-expired {"wanted":"521","arrived":"0","seconds":"901"` ; 521 stale, 94 too heavy | 0 | `graph: nothing to wait for any longer: the fleet was asked for 521 stale projects and delivered 0 within 901 s; 521 remain…` (cut) | fleet refusal |
| r14.3 | 02:42:04 – 02:59:44 | 75 | not in try log | fleet: 530 stale, 96 too heavy, `delivered 0 within 902 s` | 0 | `graph: nothing to wait for any longer: the fleet was asked for 530 stale projects and delivered 0 within 902 s…` (cut) | fleet refusal |
| r14.4 | 03:03:20 – 03:20:29 | 7 (raw, `round-14.tail.txt` line 200) | not in try log | fleet: `waiting for the fleet: 321 of 530 projects arrived` (stall 908 s) | 0 | `graph: waiting for the fleet: 321 of 530 projects arrived` (repeated, stall) | stall stop, native_rc=7 |
| step 0 (r16) | 03:21:32 – 03:47:26 | exit 1 | – | – | – | `Internal error: Complete acquired dependency retirement plan required` then `repository:command failed with exit code 1` | retirement plan (blocker 5) |
| r17.1 | 03:47:35 – 03:48:28 | 4 (raw, `round-17.tail.txt` line 200) | none | none | 0 | `macOS denied the resident service access to your Documents folder while it answered a request (reference da146983).` (preceded by `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)`) | Documents access EPERM, consent (blocker 3) |
| r18.1 | 03:50:55 – 04:02:28 | 75 | `generation 221 (5100 projects) … from a state 9 min old with 0 projects stale` | `plan stage inference took 258.9 s`; `environment took 7.8 s`; second inference 158.2 s; `result` stage refused | 0 | `The project graph could not be read on this machine yet: the plan stage result (during result) grew to 6581 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (result) |
| r18.2 | 04:05:36 – cut off about 04:13 (session restart) | none (no end line) | not in try log | `graph: a worker process was lost (exited); its project is inferred again` (04:17:06Z, sampler) | 0 | no end line; `(none)` for key lines | not measured (session restart) |

Round 19 for g1-m02: scripts only, no output.

### g1-m03 (macOS, Mac mini M4, 16 GB)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13.1 | 01:41:16 – 01:47:53 | 75 | none in try log | `plan stage inference grew the plan by 4985 MB`; inference 6766 MB | 0 | `The project graph could not be read on this machine yet: the plan stage inference grew to 6766 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (inference) |
| r13.2 | 01:50:53 – 02:17:05 | 1 (hard stop by runner) | `generation 211 (5101 projects) … from a state 36 min old with ? projects stale` | `owners 703 of 703` passed | 12 passed, 2 started (section 4) | `Internal error: Original Script process interrupted` (after `memory pressure level 2 in six samples in a row`) | self-inflicted SIGINT (pre-round-15 rule) |
| r14.1 | 02:23:11 – 02:42:20 | 75 | not in try log | `569 stale projects, 0 taken from the fleet, 208 too heavy`; fleet 0 of 569 in 902 s | 0 | `graph-wanted-expired {"wanted":"569","arrived":"0","seconds":"902","missing":"569"…` | fleet refusal |
| r14.2 | 02:45:21 – 02:47:29 | 75 | not in try log | `657 stale projects, 0 taken from the fleet, 187 too heavy`; `delivered 0 within 0 s` | 0 | `graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 0 within 0 s` | fleet refusal, 0 s window (anomaly) |
| r14.3 | 02:50:29 – 02:52:38 | 75 | not in try log | same as r14.2 | 0 | same as r14.2 | fleet refusal, 0 s window |
| r14.4 | 02:55:38 – 03:22:18 | 75 | not in try log | `657 stale, 226 too heavy`; fleet `delivered 144 within 1495 s` | 0 | first in try log: `2026-11-15T02:56:13.900Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)`; then `graph: nothing to wait for any longer: the fleet was asked for 657 stale projects and delivered 144 within 1495 s; 513 remain…` | fleet refusal, EPERM logged (blocker 3) |
| step 0 (r16) | 03:25:18 – 03:52:06 | exit 1 (wall 1607 s) | – | – | – | `Internal error: Complete acquired dependency retirement plan required` / `repository:command failed with exit code 1` / `tools_rc=1` | retirement plan (blocker 5) |
| r14.5 | 03:52:07 – 04:06:50 | 1 | `generation 221 (5100 projects) … from a state 10 min old with 0 projects stale` | no plan refusal; `graph: plan answered, managed 106 MB, resident 165 MB (released 0 MB)` | 0 | `request-failed 51f8a5ee Error: ENOENT: closure-missing:5d52130ef60a0c3016d05460bad79e740c2d0bca1c1a6c1f151da50830ba608a build-input-closures leaf is not on this machine` (04:06:35Z); monitor: `The resident service met an unexpected failure ()` | closure-missing, empty reference (blocker 4, new) |
| r14.6 | 04:09:51 – no end line in files | – | – | sampler: `build:projects … inference:275:4774` | – | not in files | not in files (snapshot) |

### g1-m04 (Linux, `posix-shared`; `round-13.md` = paused, no tries)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13 | not run (paused by round 13) | – | – | – | – | – | – |
| r14.1 | 02:09:24 – 02:29:33 | 75 | not in files | `517 stale projects, 0 taken from the fleet, 68 too heavy`; fleet 0 of 517 | 0 | `graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 901 s; 517 remain…` (cut) | fleet refusal |
| r14.2 | 02:32:36 – 02:52:50 | 75 | not in files | same as r14.1 | 0 | same as r14.1 | fleet refusal |
| r14.3 | 02:55:53 – 03:24:55 | 75 | not in files | not in files (only `tries` line) | 0 | not in files | not in files |
| r14.4 | 03:27:59 – 03:49:39 | 75 | not in files | `517 stale projects, 246 taken from the fleet, 1 too heavy for this machine (semio-framework-value-native-outcome (about 960 MB); it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 271` | 0 | the line above; `graph: waiting for the fleet: 0 of 271 projects arrived` | fleet wait, 0 MB per project (blocker 2) |
| r14.5 | 03:52:42 – 04:10:36 | 75 (`retry_lines=1`) | not in files | `271 taken from the fleet, 72 too heavy`; `graph: waiting for the fleet: 0 of 246 projects arrived` | 0 | `2026-11-15T04:10:36.772Z graph-owner-too-heavy {"named":"@semio-tech/framework-os-kernel (about 2334 MB)","limit":"1024"}` | per-project limit 1024 MB (blocker 10) |
| r14.6 | 04:13:38 – no end line in files | – | – | `waiting for the fleet: 0 of 517 projects arrived` (to 04:19:10Z) | – | not in files | not in files (snapshot) |

`state.json` (round 14, try 5): `"native_rc":75,…,"failed":["workspace:setup"]`. No task line for `workspace:setup` exists in the files.

### g1-m06 (macOS, restarted 03:57Z)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| step 0 (r18) | 04:04:15 – 04:05:46 | tools_rc 75 | – | – | – | `Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure` / `The stamps moved again after the restart: run the command again.` | new (blocker 6) |
| r18.1 | 04:06:07 – 04:07:31 | 75 | never reached | never reached | 0 | same two lines | blocker 6 |
| r18.2 | 04:10:38 – 04:12:03 | 75 | never reached | never reached | 0 | same | blocker 6 |
| r18.3 | 04:15:10 – 04:16:38 | 75 | never reached | never reached | 0 | same | blocker 6 |
| r18.4 | 04:19:41 – 04:21:04 | 75 | never reached | never reached | 0 | same | blocker 6 |
| r18.5 | 04:24:12 – 04:25:37 | 75 | never reached | never reached | 0 | same | blocker 6 |

Rounds 13 to 17 were not run here (`round-18.md`). The state says `"tries":5`.

### g1-m07 (macOS, Mac mini, 16 GiB)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13.1 | 01:41:03 – 02:04:54 | 75 | not in files | fleet: `asked for 651 and delivered 39 within 870 s` | 0 | `The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 651 and delivered 39 within 870 s, 612 are still missing…` (cut) | fleet refusal |
| r13.2 | 02:08:17 – 02:27:08 | 75 | not in files | fleet: `asked for 696 and delivered 0 within 901 s` | 0 | `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 696 and delivered 0 within 901 s, 696 are still missing…` (cut) | fleet refusal, 0 MB per project |
| r13.3 | 02:30:28 – 02:47:57 | 1 | not in files | not reached | 0 | `The resident service met an unexpected failure (reference 47fd9038); try again, the details under that reference are in the service log …` (preceded by `closure-missing build-input-closures/c90f7d707bb1778170c7d891638c3b5f4eea3c036c308e84c90dd2f5a59fda99 named by @semio-tech/repo-identity-go`) | service failure, closure-missing (blocker 4) |
| r13.4 | 02:51:12 – 03:01:07 | 75 | `plan stage inference starts (managed 189 MB …)` | `plan stage inference took 486.8 s`; 6570 MB | 0 | `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6570 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (inference) |
| r15.1 | 03:02:22 – 03:21:44 | 75 | not in files | inference 6586 MB | 0 | `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6586 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (inference) |
| r15.2 | 03:25:11 – 03:37:55 | 4 | not in files | not reached | 0 | `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)` (03:35:29Z) then `macOS denied the resident service access to your Documents folder while it answered a request (reference da146983).` | Documents access, consent (blocker 3) |
| step 0 (r17) | 03:56:32 – 04:09:43 | exit 1 (791 s) | – | – | – | not in files (`round-17.tools-setup.txt` has only times) | not in files |
| r18.1 | 04:09:43 – stall 04:31:06 (no end) | – | not in files | `plan stage inference starts (managed 197 MB, resident 2363 MB)` at 04:16:31Z | – | stall: last line `Reading the project graph slice .` (`round-18.stall.txt`) | stall (blocker 11) |

Monitor false match: `g1-m07/round-15.tries.txt` records `sig=[  --strict … doctor: exit with status N …]` for r15.2. That is help text, not an error; the real r15.2 failure is the EPERM line above.

### g1-m08 (macOS, Mac mini)

| Round.try | Start – end (UTC) | native_rc | Restore | Plan / fleet | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|---|---|
| r13.1 | 01:42:22 – 01:46:52 | 75 | not in files | plan stage environment `7995/6553` | 0 | `The project graph could not be read on this machine yet: the plan stage environment grew to 7995 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (environment); observer `PermissionError EPERM from one os.killpg SIGINT attempt` (blocker 12) |
| r13.2 | 01:49:52 – 02:05:02 | 75 | not in files | fleet: `waiting for the fleet: 2 of 703` … `48 of 703`; `delivered 48 within 805 s, 655 are still missing` | 0 | `The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 48 within 805 s, 655 are still missing…` (cut) | fleet refusal |
| r14 | no native try (superseded, round 15 not started) | – | – | – | – | – | – |
| r15 | no native try (superseded before start) | – | – | – | – | – | – |
| step 0 (r16) | 03:08:46 – 03:40:50 | 0 (`exit_code: 0`, wall 1923 s) | – | – | – | `[lease-order] Lock order violated: javascript-dependency-bindings:/Users/ueli/Documents/semio (namespace, rank 5) is requested while native-fixed-image:…/.cache/b/d (image, rank 7) is held exclusive.` (warning, step passed) | pass |
| r17.1 | 03:41:44 – 03:52:11 | 75 (`superseded_round: 18`) | not in files | plan stage inference grew to 6910 MB | 0 | `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 6910 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (inference) |
| r18.1 | 03:55:11 – 04:13:44 | 75 | not in files | `graph: 609 stale projects, 0 taken from the fleet, 10 too heavy`; `…it can give about 3512 MB per project), it can infer 140 of the others (35 per wave)` | 0 | `The project graph could not be read on this machine yet: the plan stage result (during result) grew to 6558 MB, more than the 6553 MB this machine gives a plan.` | plan too heavy (result) |

### g1-m10 (macOS, `/Users/ueli/Documents/semio`)

| Round.try | Start – end (UTC) | rc | Setup | First error line (verbatim) | Class |
|---|---|---|---|---|---|
| step 0 (r16), attempt 1 | start not in files (running at 03:29:02Z); ended by external SIGTERM, file says `05:05 CET` | – | not finished | `Internal error: Original Script process terminated` (`round-16.tools-progress.md`, update 04:06:35Z) | external kill (not a worker fault) |
| step 0 (r16), attempt 2 | 04:06:42 – 04:18:06 (684 s) | exit 1 | not finished | `Internal error: Original physical body, drain and cleanup settlement failed: Protected maintenance and accepted drainage failed: …` (`round-16.tools-setup.last15.txt`) | new (blocker 8) |
| r17, r18 | no native try in files | – | – | – | – |

Round 18 agreement wait (`round-18.agreement-wait.txt`) shows `m10=97445096cc0a` and `srv=cfb539bbd473` (04:25 to 04:30Z), with the agreement not yet reached.

---

## 2. Ranked distinct blockers (round 13 to 19)

Counts are native tries, not lines. First and last are UTC try end times.

| # | Blocker | Tries | Machines | First | Last | Verbatim signature |
|---|---|---|---|---|---|---|
| 1 | Plan stage over the memory budget (refused in `environment`, `inference` or `result`) | 10 | m01 (2), m02 (2), m03 (1), m07 (2), m08 (3) | m01 r13.1 (≈01:45Z) | m08 r18.1 (04:13:44) | `The project graph could not be read on this machine yet: the plan stage <stage> grew to <N> MB, more than the <M> MB this machine gives a plan.` |
| 2 | Fleet never delivers the stale projects: wait expires; also `0 MB per project` (fixed in round 17 by evidence) | 15 confirmed refusals (m04 r14.4 and r14.5 also waited on the fleet, their refusal line is not in the files) | m01 (4), m02 (2), m03 (4), m04 (2), m07 (2), m08 (1) | m01 r13.2 (02:05:06) | m01 r17.2 (04:27:16) | `graph-wanted-expired {"wanted":"…","arrived":"0","seconds":"…"` and `nothing to wait for any longer: the fleet was asked for … and delivered 0 within … s` |
| 3 | Relative filesystem access denied, `fstatat failed (EPERM)`, reference `da146983`; native 4 in two tries | 3 with EPERM (2 end native 4) | m02 (1, rc 4), m03 (1, rc 75), m07 (1, rc 4) | m03 r14.4 (02:56:13) | m02 r17.1 (03:48:28) | `request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)` and `macOS denied the resident service access to your Documents folder while it answered a request (reference da146983).` |
| 4 | Closure leaf not on this machine, with `unexpected failure` (native 1) | 3 (2 with a reference, 1 with an empty one) | m01 (1, ref 675127cb), m03 (1, empty ref), m07 (1, ref 47fd9038) | m01 r13.4 (02:45:44) | m03 r14.5 (04:06:35) | `request-failed <ref> Error: ENOENT: closure-missing:<hash> build-input-closures leaf is not on this machine` and `The resident service met an unexpected failure (reference <ref>)…` |
| 5 | `repo tools setup all` ends with the retirement-plan internal error (exit 1) | 2 | m02 (1, 1554 s), m03 (1, 1607 s) | m02 step 0 end 03:47:26 | m03 step 0 end 03:52:06 | `Internal error: Complete acquired dependency retirement plan required` |
| 6 | Package not resolvable in the schema closure; retry wording | 6 (1 step 0, 5 tries) | m06 | m06 step 0 04:05:46 | m06 r18.5 04:25:37 | `Cannot find package '@semio-tech/pack-json-decode-ts' imported from …/closure` and `The stamps moved again after the restart: run the command again.` |
| 7 | Native try killed by its own runner SIGINT (stall rule or round publish); exit `7` | 3 | m02 (3) | m02 r13.2 (02:06:43) | m02 r14.4 (03:20:29) | `native_rc=7` (raw in `round-13.tail.txt` 200 and `round-14.tail.txt` 200) |
| 8 | Dependency-settlement error during step 0 | 1 | m10 | m10 step 0 attempt 2 (04:18:06) | same | `Internal error: Original physical body, drain and cleanup settlement failed: …` |
| 9 | Unnamed reference `b72e2264` with native_rc 1 | 1 | m01 (r17.1) | 04:15:36 | 04:15:36 | `04:15:36Z TRY 1 end: native_rc=1; hard stop: none; failed tasks: none; reference: b72e2264` (message not in files) |
| 10 | Linux per-project limit of 1024 MB: owner too heavy | 2 | m04 (r14.4, r14.5) | m04 r14.4 (03:49:39) | m04 r14.5 (04:10:36) | `graph-owner-too-heavy {"named":"@semio-tech/framework-os-kernel (about 2334 MB)","limit":"1024"}` |
| 11 | Stall: no progress line for 905 s, 908 s, or the slice read hangs; a `git` child is in the process list | 3 | m02 (2), m07 (1) | m02 r13.2 | m07 r18.1 (04:31:06 stall) | `no new progress line for 905 s` (report) ; `Reading the project graph slice .` (`round-18.stall.txt`) |
| 12 | Tooling: observer or false match | 2 | m08 (1), m07 (1) | m08 r13.1 | m07 r15.2 | `PermissionError EPERM from one os.killpg SIGINT attempt` ; `sig=[--strict …doctor…]` (help text) |

Also seen but not counted as blockers: `[lease-order] Lock order violated` (m03 r16 step 0 failed with blocker 5; m08 r16 step 0 passed with exit 0). `Internal error: Original Script process interrupted` (m03 r13.2, the runner's own SIGINT). `graph: a worker process was lost (exited); its project is inferred again` (m02 r18.2, one try cut off).

---

## 3. Later passes: evidence that a point was passed

| Blocker | Later try that passed the point | Quoted line |
|---|---|---|
| 1, `environment` stage | m02 r14.1 and m02 r18.1 | `graph: plan stage environment took 7.8 s (managed 2967 MB, 218 MB more than at its start, resident 3401 MB)` (`round-18.md`); report: `environment` 7.9 s (+57 MB) (`round-14.md`). Fix since round 15 (02:02 UTC). |
| 1, `inference` stage | m02 r18.1 passed inference, but later tries still refused it | `graph: plan stage inference took 258.9 s (managed 2749 MB, 1262 MB more than at its start, resident 3513 MB)` (`round-18.md`). Still refused on m01 r13.5, m07 r13.4, m07 r15.1, m08 r17.1. |
| 1, `result` stage | none; new stage | refused on m02 r18.1 and m08 r18.1 |
| 2, fleet wait | none passed to completion | Partial only: `graph: 517 stale projects, 246 taken from the fleet, …` (m04 r14.4); `owners 703 of 703` passed (m03 r13.2); no try had all stale projects. |
| 2, `0 MB per project` | m08 r18.1 (fixed in round 17) | `…it can give about 3512 MB per project), it can infer 140 of the others (35 per wave)` (`g1-m08/round-18-try-1-required-lines.txt`) |
| 3, EPERM | m03 r14.5 and m02 r18.1 (no EPERM line in the m02 round 18 files) | m03 r14.5: `graph: generation 221 (5100 projects) restored from the remote cache … from a state 10 min old with 0 projects stale`. m02 r18.1: `graph: generation 221 (5100 projects) restored … from a state 9 min old with 0 projects stale`; `graph: plan stage inference took 258.9 s`. |
| 4, service failure | m01 r13.5 and m07 r13.4 | m01 r13.5: `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 7264 MB…`. m07 r13.4 reached plan stages after reference 47fd9038. |
| 4, closure-missing | none after m03 r14.5 in the files (checked: 0 hits in `round-17`/`round-18` files of m02 and m08) | not in the files |
| 5, retirement plan | m01 r17 (step 0, exit 0) and m08 r16 (step 0, exit 0) | `exit code: 0` / `wall time: 1895 s` (`round-17.tools.txt`); `"exit_code": 0` / `"wall_s": 1923.607` (`round-16-tools-setup.md`) |
| 6, pack-json-decode-ts | none; no other machine shows it (0 hits outside m06) | not in the files |
| 7, `native_rc=7` | none | not in the files |
| 8, settlement | none | not in the files |
| 9, `b72e2264` | m01 r17.2 and r17.3 started; try 2 ended native 75 (no `b72e2264`) | `04:27:16Z TRY 2 end: native_rc=75; hard stop: none; failed tasks: none; reference: none` |
| 10, Linux 1024 MB | none | not in the files |

---

## 4. Every setup task seen (round 13 to 19), with machine and round

Native nx setup tasks (`nx run … :prototype`):

Passed, `g1-m03` round 13 try 2 (`round-13.md` lines 353 to 365; raw `.round-13.try-2.clean` lines 1842 to 2342):

- `@semio-tech/ui-rs:generate` ✔
- `@semio-tech/assets:build` ✔
- `@semio-tech/ui-styling-tokens:generate` ✔
- `@semio-tech/framework-schema:generate` ✔
- `@semio-tech/dsl-derive-rs:generate-schema-index` ✔
- `@semio-tech/repo-lib:cpp-setup` ✔
- `repo:generator-inputs` ✔
- `repo-mcp:build` ✔
- `@semio-tech/plugin-registry:generate` ✔
- `repo-mcp:install` ✔
- `@semio-tech/ui-styling-py:deps` ✔
- `@semio-tech/energy-oracle-py:deps` ✔

Started, no result (the try was stopped by a SIGINT): `workspace:deps-js-all` and `workspace:deps-go` (the `>` lines in the "Setup tasks of try 2" list of `round-13.md`; the runner stopped during `workspace:deps-go`).

No other nx setup task result exists in rounds 13 to 19. Every other try either failed before any task (fleet or plan stage) or the stage was `workspace:setup` with no task line (`round-13.md` of g1-m07: "All four failed in `workspace:setup` before any setup task ran"; `state.json` of g1-m04 round 14, g1-m07 round 15, g1-m08 round 18).

Repo tools setup (`repo tools setup all`, step 0 of round 16 and round 17), per machine:

| Machine | Round | Result | Wall | Message |
|---|---|---|---|---|
| m02 | 16 | exit 1 | 1554 s (03:21:32 to 03:47:26) | `Internal error: Complete acquired dependency retirement plan required` |
| m03 | 16 | exit 1 | 1607 s (03:25:18 to 03:52:06) | same |
| m08 | 16 | exit 0 | 1923 s (03:08:46 to 03:40:50) | (passed; `[lease-order]` warning in the console) |
| m10 | 16 | attempt 1 SIGTERM (external); attempt 2 exit 1 | 684 s (04:06:42 to 04:18:06) | `Internal error: Original physical body, drain and cleanup settlement failed: …` |
| m01 | 17 | exit 0 | 1895 s (03:38:49 to 04:10:24) | (passed; `Target //:setup up-to-date`) |
| m07 | 17 | exit 1 | 791 s (03:56:32 to 04:09:43) | not in the files |
| m06 | 18 | exit 75 | 91 s (04:04:15 to 04:05:46) | `Cannot find package '@semio-tech/pack-json-decode-ts' …` |

Stage `workspace:setup` failed before any task (state.json): g1-m04 round 14, g1-m07 round 15, g1-m08 round 18.

---

## 5. New or not-yet-named problems

1. `native_rc=7` (not in 0, 1, 4, 75). Seen three times on g1-m02: r13.2 (raw line `native_rc=7` in `round-13.tail.txt`), r14.1 (per `round-14.md`, after the runner's SIGINT at 02:21:12Z), r14.4 (raw in `round-14.tail.txt`). Each was a runner SIGINT (stall or round publish). The same SIGINT on g1-m03 (r13.2) ended with `native_rc=1` and `Internal error: Original Script process interrupted`. Cause not in the files.
2. Reference `b72e2264`: `04:15:36Z TRY 1 end: native_rc=1; hard stop: none; failed tasks: none; reference: b72e2264` (g1-m01 `round-17.samples.txt` line 306). The bootstrap message and the service log of that try are not in the files.
3. Empty reference: `The resident service met an unexpected failure ();` (g1-m03 `round-14.monitor.txt` line 21, native 1).
4. Module resolution on g1-m06: `Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure`, six runs, and `The stamps moved again after the restart: run the command again.` Not seen elsewhere.
5. Settlement error on g1-m10 step 0: `Internal error: Original physical body, drain and cleanup settlement failed`.
6. New refusal stage `result`: `the plan stage result (during result) grew to 6581 MB` (g1-m02 r18.1) and `6558 MB` (g1-m08 r18.1). Earlier refusals were `environment` and `inference` only.
7. Odd fleet windows: `delivered 0 within 0 s` (g1-m03 r14.2 and r14.3), `delivered 0 within 301 s` (g1-m01 r17.2). The other windows were 805 to 1495 s (for example `within 901 s`, `within 902 s`).
8. Stall with a git child process: g1-m07 r18.1 stopped at 04:31:06Z on `Reading the project graph slice .`; the process list (`round-18.stall.txt`) holds a `git -c core.quotepath=false …` child. g1-m02 r18.2 cut off with `a worker process was lost (exited)` (04:17:06Z).
9. Linux per-project limit: `graph-owner-too-heavy {"named":"@semio-tech/framework-os-kernel (about 2334 MB)","limit":"1024"}` (g1-m04 r14.5). Linux still shows `it can give about 0 MB per project` at r14.4.
10. Observer fault: `PermissionError EPERM from one os.killpg SIGINT attempt` (g1-m08 `round-13-try-1-result.json`), so a sample record was lost.
11. Lock order warning `[lease-order] Lock order violated: javascript-dependency-bindings … is requested while native-fixed-image … is held exclusive.` (g1-m03 r16 step 0 and g1-m08 r16 step 0, which still passed).
12. Hangs: no-progress stalls at 905 s (g1-m02 r13.2) and 908 s (g1-m02 r14.4); g1-m04 r14.5 `[repository:command] still running after 2340 s`; g1-m01 r13 `[repo] still working after 1370 s; last phase started: door.script`.
13. Unexplained `workspace:setup` failure with no task output: g1-m04 r14, g1-m07 r15, g1-m08 r18.
14. Round 19: no results in any worker folder (`run-round-19.py` and `tools-setup-round-19.py` exist only on g1-m02).

---

## 6. Data gaps

- Round 19 results: not in the files.
- `g1-m05`: no folder.
- `g1-m10`: no native try in rounds 13 to 19.
- Try ends not in the files: g1-m01 r17.3 (in progress), g1-m03 r14.6, g1-m04 r14.6, g1-m07 r18.1 (stall), g1-m02 r18.2 (cut off).
- Bootstrap logs for most round 13 to 15 tries are not in the folders; only the `tail`, `service` and `samples` files, plus the reports. Where a first error line is marked "not in the files", that is why.
