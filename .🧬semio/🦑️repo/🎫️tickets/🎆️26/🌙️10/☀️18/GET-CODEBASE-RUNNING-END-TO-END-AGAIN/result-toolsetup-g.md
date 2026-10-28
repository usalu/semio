# result-toolsetup-g (wave 1026g, executor toolsetup-g): what the non-graph phases of a hot `bun repo run <project>:<target>` do

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`. Tools `C/toolsetup-g-tools/`, staged batches `C/toolsetup-g-tools/staged/<name>/`.

FOR gate-g (19:20): batch `C/toolsetup-g-tools/staged/probes` (4 files in `B/🔭️witness`; `sh C/toolsetup-g-tools/staged/probes/apply.sh` | `check.sh` | `revert.sh`, laws `sh .../verify.sh`). One line: the planner-skip (landed 10:26) NEVER hits because the planner child spawns four read-only host probes (sysctl boot identity, hdiutil roster, mount table, ps child list) that the world witness taints; a closed table of exactly those argument vectors is exempt. Expected effect on a hot course no-op: `tool-setup` 32-48 s -> about 3-6 s (to be proven by the first hot pair after the apply; see "Proof still owed"). Land it before the next rotating product build and run that build twice.

## 1. Where a hot no-op spends its time (measured: bench2-g q4, 18:53-19:03, load 19-25, course:build, Bazel, 32 GB host, the 16 GB profile is NOT in force)

Phase classes are bench2-g's (`C/bench2-g-tools/phases.json`: pattern classes over the run log). `tool-setup` there is NOT a pinned-tool installer: it is everything after the graph is ready until Bazel's `INFO: Invocation ID`, i.e. Bazel slot readiness + `bazel.implementation` capture + the captured planner child + workspace generation.

| run | wall | queue-capacity | boot | admission | verify | graph | tool-setup | execute | finish | Bazel actions |
|---|---|---|---|---|---|---|---|---|---|---|
| q4 prime (first run after peer edits, graph refresh 2 projects) | 368.7 | 0 | 0.3 | 2.8 | 20.0 | 197.0 (+4.1 queue) | 81.7 | 62.1 | 0.6 | 8 processes: 1 internal, 1 local, 6 worker |
| q4 s1 (hot) | 76.8 | 19.2 | 0.4 | 5.7 | 4.7 | 8.5 | 37.9 | 0 | 0.4 | 1 process: 1 internal |
| q4 s2 (hot) | 59.9 | 0 | 0.4 | 6.5 | 6.3 | 13.8 | 32.2 | 0 | 0.7 | 1 process: 1 internal |
| q4 s3 (hot) | 83.4 | 0 | 0.4 | 2.8 | 5.3 | 26.9 | 47.6 | 0 | 0.4 | 1 process: 1 internal |
| q1 prime-2 (earlier, load 52, 16 GB profile, after peer edits; not a no-op) | 1525 | 120 | 1.6 | 63.5 | 61.2 | 776 | 206 | 295 | 1.7 | 7 processes: 1 hit, 1 internal, 6 local |

So the premise of the brief ("5-6 actions re-execute even when nothing relevant changed") is true only for the PRIME runs: every hot sample executes ZERO actions (`1 process: 1 internal` = the Bazel snapshot target). The re-executions of q1 and the q4 prime follow real input changes (host-capacity sampler edit, graph module edits, schema edits: the graph log names them: `@teaching/architecture-course is stale: the input file …/🌡️host-capacity/…/🟦️.ts changed`), because the course project's command closure contains build-library files (infer-f's 3739 of 4020 projects) — that is exec-f's OPEN cargo/command-closure item, not a tool-setup defect.

### Hot s2 timeline inside the single `repository:command` child (50 s of 60 s wall; 10 s heartbeats + line order)
| step | time | what |
|---|---|---|
| `build:source-admission` ×96 ticks | 6.5 s | `captureCompilerBuildSourceClosureV1` = Nx authorizer source capture with the memo ON (default): stat-identity validation of the build library's import closure |
| `dependencies:generation-verify:*` to 81920 twice | 6.3 s (with graph-facts start) | `ensureBuildEnvironmentV1`: `fast` verify (every tier, 82 k stats) then `recheck` (all tiers but provider observations, again ~82 k stats) in the same shared custody |
| `Verifying graph facts` 75,868 + `Assembling the graph` 198 projects + `198 projects planned by a child process` | 13.8 s | graph generation load/verify (graphlat-g's lane) |
| `[bazel] session slot s1: warm server and package`, `ready 58602460/58602460` | <1 s | pinned Bun for hermetic actions: `prepareBazelBunV1` reads and SHA-256s the 58.6 MB executable every run under an EXCLUSIVE lease `bazel-bun-<sha>` (measured 0.06-0.15 s hash, 63 MB RSS; a convoy lock rather than a time sink) |
| `build:implementation-source` ×198 (twice: 175-302 and 304-375) + `bazel:captured-planner` + `bazel:source-snapshot` | ~29 s | `publishBazelImplementationV1` (implementation + adapter captures) and the CAPTURED PLANNER CHILD planning the same 198 projects again |
| Bazel itself | 0.41-0.52 s | `Elapsed time: 0.414s … 1 process: 1 internal` |

So on a hot no-op the Bazel side is 0.5 s; 29 of the 50 s are a replan that was supposed to be skipped.

## 2. Finding 1 (fixed, staged): the planner skip never hits

`prepareBazelPlanReuseV1` (noop2-g, landed 10:26) records a plan only when the planner child's world witness is untainted. EVERY logged run since (6 + 2 logs, incl. all three hot q4 samples) prints

`[bazel] this plan cannot be reused later: the planner read something that cannot be checked: spawned Bun.spawn /usr/sbin/sysctl @ captureOwnedCommandLiveV1; spawned Bun.spawn /usr/bin/hdiutil @ captureOwnedCommandLiveV1; spawned Bun.spawnSync ps @ liveChildPidsV1` (the e2e logs add `/sbin/mount`).

Zero `plan reused` lines exist in any log of the wave (grep over `C/🗑️generated`, `gate-g-tools/run`). The four spawns are the native-storage custody (boot identity `sysctl -n kern.bootsessionuuid` / `-n kern.boottime`, image roster `hdiutil info -plist`, mount table `mount`) and the capacity gate / lease yield (`ps -axo pid=,ppid=` for live children): infrastructure liveness probes of the HOST, none of which can change what the planner produces, but the witness taints ANY spawn that is not wrapped in `allow()`; only the Bazel adapter wraps its own git/runtime-command spawns (`witness.allow("git"|"runtime", …)`), the process library's infrastructure probes were never wrapped.

Fix (schema-first, one place): the witness `Policy` carries a closed table `hostProbes` of exact (program basename, argument vector) rows plus `hostProbeFiles` (Linux boot identity files `/proc/stat`, `/proc/1/stat`, `/proc/sys/kernel/random/boot_id`, whose stat identity changes constantly and would otherwise make the witness racy/never-holding). The self-contained preload gets both through its configuration (`worldWitnessEnvironmentV1` takes optional overrides for tests). A spawn is exempt only when its command (array, `{cmd}` or file+args form; never shell strings, `exec`, `fork`, `Bun.$`) matches a row EXACTLY; `sysctl -w`, `hdiutil attach`, a `ps` from another directory, a different argument tail, anything else still taints. Rows: `sysctl -n kern.bootsessionuuid`, `sysctl -n kern.boottime`, `hdiutil info -plist`, `mount` (no arguments) and `ps` with ANY arguments (read-only by nature; the repo runs at least six `ps` vectors: child listing, `-o cputime=`, `-o etime=`, ancestry listing, tree RSS, termination listing, and the taint dedupe would have hidden all but the first). The program must be a bare name or live in `/bin /sbin /usr/bin /usr/sbin` (`hostProbeDirectories`), so `/nonexistent/ps` still taints.

The taint reason now also names the COMMAND LINE (first 8 arguments, 60 chars each) instead of the program only, because taints are deduplicated by reason and a second `ps`/`hdiutil` argument vector would otherwise hide behind the first; if the first hot pair still prints a `cannot be reused` line, it names the exact remaining vector to add.

Laws (`B/🔭️witness/🧪️tests/🟦️.ts`, +2 tests; FAIL on the base file, pass on the new one):
1. closed table: the probes (incl. two `ps` vectors) leave `tainted` empty and the probe file out of the observations; `sysctl -w`, `hdiutil attach` and `/nonexistent/ps … --extra` still taint, with the full command line in the reason; with an empty table the same scenario taints and observes the file again (so the exemption is the only reason it passes).
2. drift law: every `sysctl` (program, arguments) pair declared by the native-storage schemas (`bootSession`, `bootEpoch.darwin`) is a row of the table, so the two cannot diverge silently.
Before/after (same test file, path-mirror with the live `🔨️modules` symlinked): BASE witness = 6 pass, 2 fail (both new laws red); NEW witness = 8 pass, 0 fail. Plan-reuse oracle (real planner functions, 4 tests) 4 pass 0 fail against the NEW witness in the mirror and 4/4 against the live tree. Linux/Windows: the Linux file rows are exercised through the injected `hostProbeFiles` in law 1 (platform-neutral); the Windows `powershell Get-CimInstance Win32_Process` child listing of the watchdog is NOT in the table (its command is a long inline script; moving it into the schema is the follow-up, until then a Windows planner stays unreusable, EN: open / DE: offen).

Proof still owed (needs one official run pair, after 19:35 and after gate-g applies `probes`): run 1 must print NO `cannot be reused later` line (or only a different, named reason), run 2 must print `[bazel] plan reused: N observations … the planner was not started (X s)`; then `tool-setup` should be the reuse check (key + 40 k stats ≈ 0.2-0.3 s per noop2-g, git digest port, runtime port) + Bazel 0.5 s.

## 3. Finding 2: verify (4.7-6.3 s hot, 15-61 s under load/after edits) — what it is and the next cut
Not a tool re-verification. It is the dependency generation receipt (`ensureBuildEnvironmentV1`): `fast` verify = 81,920 lstat/stamp checks (every tier incl. provider observations), then `recheck` (same tiers minus providers) = 81,920 again, both inside the same shared store custody; plus `Verifying graph facts` (75,868 facts) which the phase classifier also counts as verify. At ~39 µs per lstat on a loaded host that is 3 s per pass at load 20 and 9-60 s at load 50 (the q1 run: 8.6 s + 8.8 s of pure verify progress). The recheck exists to catch a source change between the first look and the custody; a change-journal cursor taken before the `fast` pass would make the `recheck` an O(1) journal question (`checkWorldViaJournalV1`, falls back to the stat walk when the journal cannot vouch). NOT done in this batch: it touches `B/🚀️bootstrap/🟦️.ts` + the generation store and needs a hot pair to prove; est. -2.5 s quiet, -8 s at load 50.

## 4. Finding 3: admission (2.8-6.5 s hot, 63 s in q1)
`build:source-admission` is the Nx authorizer source capture of the build system's own import closure (the memo is on by default; the hot cost is the stat-identity validation of ~1,400 files + ancestry directories, noop2-g: lstat 93 k -> 13.8 k). Under load 52 it was 63 s because each lstat waits for a CPU. Same journal cursor would make it O(1); not staged.

## 5. What is NOT a problem (checked)
- Pinned-tool setup (`🧰️tool-setup`, Bazel/Bun/toolchains): not on the `repo run` path; Bazel server stays warm (`session slot s1: warm server and package`); `ready 58602460` is the 58.6 MB pinned Bun hash (0.1 s, but exclusive-lease: convoy risk with 20 agents, stamp-by-identity cure is trivial and not worth a batch).
- Bazel analysis/execution on a hot no-op: 0.4-0.5 s, 0 actions re-executed in all three samples (action keys hold).
- The remaining cost drivers are outside this lane: capacity queue (200 % sizing = 7168 MB for a pure cache replay; 19.2 s in s1), graph generation load (8.5-27 s), and the lock-order diagnostic `bazel-slot-gate (rank 2) requested while graph snapshot (rank 4) is held shared` printed by every run (hold-and-wait smell; FOR graphlat-g / lease owners).

## 5b. Side findings
- LEAK (plan-reuse oracle, `B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts`): each run of its decision-cycle test starts a change-journal service on demand (`takeBazelPlanCursorV1` -> `connectChangeJournalV1(root).cursor()`) for the temp checkout; the service (bun ~33 MB RSS + `semio-repo-watch` ~5 MB, ppid 1) outlives the test and the deleted checkout until its 2 h idle shutdown (`idleShutdownMilliseconds` 7200000). Live right now: 8 such pairs (17:38, 18:11, 19:03, 19:06, 19:11, 19:14, 19:15, 19:15), several certainly from other lanes' test runs. Cure for the owner (noop2-g / watch-g): the oracle sets `SEMIO_REPO_CHANGES=off` (client switch, default on) except in the one case that tests the journal, and that case stops its service. NOT fixed here (I cannot attribute the live pids by lstart, so none was killed).
- BATCH CONFLICT: conv-g's staged batch edits `🔭️witness/🟨️.mjs` and `🟨️.d.mts` (docstrings only) from the same base hashes as `probes`. Twin batch `staged/probes-after-conv` = my change 3-way merged onto conv-g's NEW files (diff3, one hand-resolved hunk in the .d.mts); witness 8/8 + plan-reuse 4/4 on the merged copy. `check.sh` of the twin reports `drifted` for the two files until conv-g is applied, `base` for all four afterwards. If `probes` lands first, conv-g's two witness files must be re-taken from `toolsetup-g-tools/mirror/afterconv/🔭️witness/`.
- `[lease-order] Lock order violated: bazel-slot-gate (tool-singleton, rank 2) is requested while …graph/m/snapshots/<sha>.json (graph, rank 4) is held shared` is printed by every hot run (also q1); a rank-2 lease taken while holding a rank-4 shared lease is the hold-and-wait shape rev3-g warned about; harmless while the graph lease is shared-only, FOR lease/graph owners.

## 6. Status (newest first)
- 19:25 twin batch `probes-after-conv` staged; `ps` rows widened to any arguments (6 production `ps` vectors exist and the taint dedupe hid all but the first), program must be bare or in a system directory; taint reasons name the full command line. Witness 8/8, plan-reuse oracle 4/4 against the new witness. Waiting for 19:35 for the baseline profile run.
- 19:20 batch `probes` STAGED and self-tested: `stage.py` apply/check/revert (sandbox simulation: apply, idempotent re-apply, revert, drift refusal exit 2), base hashes in `bases.json`.
- 19:10 analysis of bench2-g q4 hot samples: planner skip has never fired; four spawns identified.
