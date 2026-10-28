# result-envleak (lane envleak, 10/27): ambient environment vs Bazel action keys and plan reuse

FOR coordinator: batch ready `C/envleak-tools` (14 files, flat, base = live now: `python3 -I C/envleak-tools/stage.py check` prints 14 x base).
- apply : `python3 -I C/envleak-tools/stage.py apply`   (exit 2 on drift, atomic, idempotent; `--root DIR` or `STAGE_ROOT=DIR` for a mirror)
- laws  : `sh C/envleak-tools/verify.sh "$(git rev-parse --show-toplevel)"`   (live; in a mirror: `python3 -I C/mirror-tools/mirror.py run <name> --peers-ok -- sh C/envleak-tools/verify.sh "$TMPDIR/sm/<name>"`)
- revert: `python3 -I C/envleak-tools/stage.py revert`   (removes the 5 new files and the emptied directory)
- product proof after apply: see "Product proof" below (3 runs, you start them). Landing costs ONE replan (the witness preload text is part of the plan key); the runtime digests do not change (see Evidence), so no action re-executes because of the landing.

## Verdict in one paragraph

(a) is proven and fixed. (b) as worded ("agent variables reach action keys") is NOT reproducible: the Bazel action keys never see the shell (`--incompatible_strict_action_env`, the live environment travels in `runtime.json` outside every key) and the real runtime inputs digest the same under CLAUDE_*/XPC_*; the only paths from an agent shell are (b1) the plan witness recording ~100 ambient variables as `e` rows plus one `E` enumeration row (any change = replan) and (b2) `runtime.json` handing the whole shell (incl. NEO4J_PASSWORD, SSH_AUTH_SOCK) to every action process. (b1) is fixed; (b2) is documented, not changed. The "(b) reran the actions" observation at 02:17 was most likely (a) in disguise (the shell that was profiling had `SEMIO_REPO_PROFILE` exported); the three-run proof below separates the two.

## Root cause (a): the profile switch changes the digest of every `runtime` input

1. `nx.json:12-20` `sharedGlobals` (part of `default` and `production`, so of almost every task) holds two `runtime` rows: `repo tools exec --light node -- -p "process.platform.concat(process.arch)"` and `repo tools exec --light bun -- --version`. A `runtime` row is a command whose output is hashed into the task fingerprint.
2. `📜️script.ts:576-581` `bazelRuntimeStdoutV1` and `:738-743` `ports.runtime` (the plan-reuse recheck) both ran `captureOwnedCommandV1(process.execPath,["exec",runtime],root,{env:original})`. `original` is the issued task environment = the WHOLE ambient environment (`executor:101-103` `bazelTaskBaseEnvironmentV1` spreads `process.env`; `📥️caller:52` `issueBuildTaskEnvironmentV1` keeps it).
3. `captureOwnedCommandV1` returns the merged 64 KiB tail of stdout AND stderr (`🎛️owned-execution/🟦️.ts:54`, `📥️capture/🟦️.ts:86` `append` keeps one `tail` for both streams).
4. The child is a full `repo` process. With `SEMIO_REPO_PROFILE=json|1` it flushes the phase profile to STDERR on exit (`⏱️profile/🟨️.mjs:93-96`, `flush` :83): `{"pid":..,"total":..,"rows":[..milliseconds..]}`. pid and timings differ per run.
5. So the digest of both global runtime rows differs per run: `taskFingerprint` (:583-588) writes a new `.input` for every cached task that uses `production`/`default` (the 7 actions seen: assets:build, framework-actor-rs:typegen, ui-contract-rs:generate, ui-styling-tokens:generate, ui-rs:generate, framework-rs:generate, course:build) -> Bazel re-executes them (`INFO: 8 processes: 1 internal, 7 worker`, `Elapsed 28.4 s` in `gate-g-tools/run/g22-prof2.log`), the course `.🧾️store/build.json` is rewritten (its mtime is 02:16 = the last g22 profile run), the planner's `d .../.🧾️store changed` observation breaks the witness (`planning again: d ...package/.🧾️store changed`, g22-prof2.log line 184) and the next run replans. Every profiled run therefore both replans (x row digest moved) and re-executes.

Why the same hazard was latent without the profile: the merged tail also carries anything a busy machine prints to stderr (`[capacity] waiting`, lease-order warnings). Standard error is diagnostics; it must never key anything.

## (b): where an agent shell really enters

| Path | What enters | Effect | Status |
|---|---|---|---|
| Bazel action key | nothing from the shell: `--incompatible_strict_action_env` (script.ts:919 area), rules use `use_default_shell_env`+fixed `env`, live env in `runtime.json` "outside every action key" (script.ts:543) | none | OK, no change |
| `taskFingerprint` | only `SEMIO_BUILD_MODE`, the task's declared `env` rows, toolchain policy names (Go `policy.environment`, native, `RUSTUP_HOME`) and the `runtime` rows | none, EXCEPT via (a) | fixed via (a) |
| runtime input child | whole ambient env | profile (a); agent vars: none measured | env now neutral (below) |
| plan witness `e`/`E` rows | the planner spreads `process.env` (`bazelTaskBaseEnvironmentV1`, `{...process.env,...}` in script.ts:585-ish and publishCaptured) -> the witness records 97 `e` rows + one `E` enumeration (live `plan.receipt.json`: 65,392 observations, kinds e=97, E=1; names include CLAUDE-free product.py env but every desktop/agent/session name when present) | any ambient change (new CLAUDE_CODE_SESSION_ID, XPC_*, TERM_SESSION_ID, SECURITYSESSIONID) = `planning again: e X changed` (replan, ~40-60 s instead of 20 s) | fixed |
| `SEMIO_AGENT_SESSION`, `AI_AGENT` | the identity of the agent (derived by the repo CLI from CLAUDE_CODE_SESSION_ID); also an `e` row | alternating plain shell vs product.py (which drops CLAUDE*) = different identity = replan | fixed (witness ignores them) |
| `runtime.json` -> action processes | the complete ambient environment (58 names in the live file, incl. NEO4J_PASSWORD, SSH_AUTH_SOCK; mode 0600) | not a key; actions can read anything | NOT changed (behaviour risk; follow-up: allowlist the action channel by schema) |

## Evidence (all measured here, no product build)

1. Real runtime child, 5 settings (`C/envleak-tools/oracle-runtime.ts`, mirror, the real `repo tools exec --light bun -- --version`):

| setting | merged (base) | keyed (fixed) |
|---|---|---|
| off | bbfd2b082608 | bbfd2b082608 |
| SEMIO_REPO_PROFILE=json | 9696e7b43ad8 | bbfd2b082608 |
| json again | e38a46bcf6bd | bbfd2b082608 |
| SEMIO_REPO_PROFILE=1 | 4b92c84aede1 | bbfd2b082608 |
| CLAUDE_CODE_SESSION_ID + XPC_FLAGS | bbfd2b082608 | bbfd2b082608 |

   `bbfd2b08...` is also the digest the live plan witness recorded for this runtime input (`x` row of `plan.receipt.json`, sha256 `bbfd2b0826085b4399473db29892babf1365e83e28bf666a1dd3788a23b071b5`): an independent third witness. The same equality holds for the other two recorded rows (`4665f651...` node --version, `70f2620f...` node -p). The CLAUDE_* row of the BASE column proves the claim "agent variables do not move the runtime digest"; the profile rows prove (a).
2. `gate-g-tools/run/g22-prof1..3.json`: 8 actions, `7 worker`, 55-60 s every time with the profile; `g21-hot2/3` (same code, no profile): `1 process: 1 internal`, plan reused, 20 s.
3. Base state of the fingerprint law file: `🧪️tests/📥️fingerprint` was already RED on live (2 fail, `bazelRuntimeStdoutV1 is not defined`: the witness2 landing wrapped the runtime call without updating the harness). The batch repairs the harness and adds the new law (4 pass).

## Fix (schema-first, one policy, both call sites)

- NEW `🔧️tools/🏗️bazel/🌿️environment/` (module `🟦️.ts` + `🧬️schema/🔣️.json` + `🧪️tests`): the policy of the environment of a KEYED command. `Policy.diagnostic` (SEMIO_REPO_PROFILE, SEMIO_LEASE_TRACE, SEMIO_PROGRESS_DEADLINE_TRACE, NODE_DEBUG, BUN_CONFIG_VERBOSE_FETCH) and the ambient set of the witness policy never reach the command; its digest is its STANDARD OUTPUT only (separated owned capture, bounded 4 MiB / 65,536 lines); a failed command is refused in EN and DE with stderr as the explanation. API: `bazelKeyedCommandEnvironmentV1(env)`, `runBazelRuntimeCommandV1(root, runtime, env, control, capture?)`.
- `📜️script.ts`: `bazelRuntimeStdoutV1` (planner) and `ports.runtime` (plan-reuse recheck) call `runBazelRuntimeCommandV1` (+1 import line, 2 call-site edits; same `JSON.stringify` digest, so existing `x` digests stay valid).
- `🔭️witness`: policy gains `ambientEnvironment {prefixes: CLAUDE, ANTHROPIC, __CF, XPC_; names: TERM_SESSION_ID, SECURITYSESSIONID, LaunchInstanceID}` (one source: the keyed-command module reads it through the new `isAmbientEnvironmentNameV1`); the witness records no `e` row and no `E` entry for them, the recheck of an old `e` row of such a name holds; `ignoredEnvironment` += `SEMIO_AGENT_SESSION`, `AI_AGENT`.
- `🗂️suites`: routes the new module (build-bazel-execution) and the previously UNROUTED `🔭️witness` tests (build-observation); the suite-coverage law was red on live for that reason (now 7/7).
- Why a declared denylist for the child but an allowlist for the key: what can KEY an action is already a closed allowlist (mode, declared `env` rows, toolchain policy names, `runtime` rows); the new law proves it. The runtime child must still run a full `repo` (PATH, HOME, envelope variables ...), so it gets the issued environment minus the declared diagnostic/ambient set, and because only stdout keys, whatever else it sees cannot key.

## Laws (red on base, green with the fix)

| Law | Base | Fixed |
|---|---|---|
| `🌿️environment/🧪️tests` keyed env drops diagnostic+ambient, keeps the rest; independent Python oracle (`🧮️resources/🐍️oracle`) recomputes from both schemas; real children: merged tail differs run to run under the profile (characterisation), keyed stdout identical for off/json/json/1/CLAUDE+XPC; failing command refused EN+DE | module absent (red) | 2 pass |
| `🧪️tests/📥️fingerprint` NEW "action keys never depend on an ambient host, agent or session variable or on a diagnostic switch" (schema-first corpus `🔣️ambient.json`/`🧬️ambient.json`, 8 rows: profile json/table, two agent sessions, desktop vars, undeclared variable, build mode -> same key; the declared variable -> different key) | file RED already (harness stale, 2 fail) | 4 pass |
| `🔭️witness/🧪️tests` NEW "an ambient host, agent or session variable and the identity of the agent are never an input" (no `e` row, no `E` entry, change of ambient values and of the agent identity keeps the witness holding, a change of an ordinary variable breaks it) | RED: `CLAUDE_FIXTURE_ID` has an `e` row (mirror elred) | 12 pass |
| `♻️plan-reuse` 5/5, `🧭️ignore-sources` 15/15, `🗂️suites` 7/7 | suites RED (witness unrouted) | green |
| real-process oracle (above) | merged: 4 distinct digests | keyed: 1 digest = recorded witness digest |

Per-file comparison of the whole `🏗️bazel/🧪️tests` tree (31 files, one bun process each) base vs fixed: identical pass/fail counts everywhere except the fingerprint file (the repaired one). The pre-existing reds are unrelated to this lane and identical on both sides: `📋️contract` 0/3, `📤️arguments` 0/4, `📥️environment` 0/2, `📥️framing` 0/2, `📥️inventory` 0/1, `📥️purpose/🧪️tests` 1/1, `🔏️implementation` 0/5, `🕸️dependencies` 0/13, `🧪️tests/🟦️.ts` 10/2, `♻️plan-reuse/🧫️fixtures` (a fixture module, not a test). Reasons: "Build test receiver admission closed" / mirror without git. `direct-tool-declarations` needs git (not runnable in a mirror); `verify.sh` runs it when the root is a git checkout (live).
Mirror runs: `elbase`/`elred`/`elwork`/`elfix` (all dropped), guard clean on every run; the final proof ran on a mirror built ONLY from `staged/` (`mirror.py create elfix --batch envleak-tools/staged`: 14 files), `verify.sh` all green.

## Product proof (you start them; the three runs separate (a) from (b))

Common: `cd C/gate-g-tools; ` then each run as below; read the log with `grep -E "plan reused|planning again|cannot be reused|INFO: [0-9]+ process|Build completed" run/<label>.log` and the course store stamp with `stat -f '%m' "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/.🧾️store/build.json"` before and after.
1. Landing replan, then hot (keys must not change: expect `INFO: 1 process: 1 internal` or only disk-cache hits, never `7 worker`):
   `python3 product.py ev-hot1 @teaching/architecture-course:build; python3 product.py ev-hot2 @teaching/architecture-course:build` -> hot2 `[bazel] plan reused`, store mtime unchanged.
2. (a) The profile no longer reruns anything (the old behaviour was `8 processes ... 7 worker` x3):
   `SEMIO_REPO_PROFILE=json python3 product.py ev-prof1 @teaching/architecture-course:build; SEMIO_REPO_PROFILE=json python3 product.py ev-prof2 @teaching/architecture-course:build` -> prof2 `plan reused`, `1 process: 1 internal`, store mtime unchanged, wall ~20-25 s (+ profile print). product.py keeps `SEMIO_REPO_PROFILE` (it only drops CLAUDE/ANTHROPIC/__CF/XPC_/TERM_SESSION_ID/SEMIO_SCRIPT_STALL_MS).
3. (b) Plain shell with agent variables, NOT through product.py (two runs, the second with other values, from this agent shell or any shell):
   `cd <repo>; for v in 1 2; do CLAUDE_FIXTURE_X=$v XPC_FIXTURE=$v TERM_SESSION_ID=w${v}t0p0 nice -n 10 bun repo --tool=bazel run @teaching/architecture-course:build > C/🗑️generated/envleak/plain$v.log 2>&1; done`
   -> plain2: `plan reused`, `1 process: 1 internal` (base: `planning again: e CLAUDE_FIXTURE_X changed`, replan ~60 s).
If run 3 still re-executes actions on your side, the cause is NOT the environment: compare `workspaces/s*/<ns>/t_*.input` of two consecutive runs (`.cache/b/p/m/bazel/workspaces/`) and look at which `t_*.input` moved; send me the two logs.

## Not done / follow-ups

- (b2) `runtime.json` hands the full shell (incl. secrets) to every action process: schema allowlist for the action channel (PATH, HOME, TMP*, LANG*, the SEMIO_* envelope, toolchain names) is the clean fix; changing it can break actions that read tool configuration from the shell, so it needs its own product verdict. The planner still spreads `process.env` (97 rows now ignored by policy, not removed); replacing the spread by a picked environment would remove the enumeration entirely.
- `SEMIO_SCRIPT_STALL_MS` (in product.py's DROP): a watchdog knob read by the planner (`e` row); not an input of a plan; not added to the ignore list here (outside the brief), one more line in `ignoredEnvironment` if you want it.
- Files: new `C/envleak-tools/{stage.py,make.sh,verify.sh,oracle-runtime.ts,build_module.py,build_tests.py,edit_script.py,edit_tests.py,edit_witness.py,stage-flat.py,per-file.sh,staged/}`; evidence tables `C/🗑️generated/envleak/{fixed,base}.txt`.
