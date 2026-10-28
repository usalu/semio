# result-audit10 (lane audit10, prefix aa, wave 1027b): pre-landing adversarial review of the staged G46+ queue

STATUS 19:00 FINAL. 20 batches reviewed base→new against live (18:40, after scopedcut was re-applied and orchslice landed).
- Verdicts: 13 LAND (several with an order), 3 FIX FIRST with an audit10 copy (`multiwriter` rebase, `split`, `wininstall`), 1 FIX FIRST that needs a lane change (`hold`), 1 HOLD stacked on it (`lastrun`), and `pinned`/`optcache` land in order.
- The three copies are verified on mirror `aam`, which is now dropped.
- Nothing live was touched. No official run, no ticket tool, no process of mine left running.

## Verdicts

| batch | verdict | one-line reason |
|---|---|---|
| quiet16 `hold` (re-cut 18:30) | **FIX FIRST (lane change)** | The width gate counts sockets, not actions. Every real action joins twice: the birth, then its child's execution join while the birth socket stays open. Births that fill the grant's jobs therefore block their own children forever, and execution joins without a deadline have no handshake timer. The birth's hard 10 s handshake also now covers the quiet growth wait (§hold). |
| quiet16 `lastrun` | HOLD (stacked on `hold`) | Sound on its own, but it makes nearly every run quiet, which puts nearly every build on the broken `hold` birth path. |
| builderlock `multiwriter` (final, with audit9's fix) | **FIX FIRST → audit10 copy** | Content is right; the base is not. scopedcut was reverted at 18:26 and re-applied at 18:38, and gatefast `orchslice` (18:38) changed `🔌️load/🟨️.mjs` + `.d.mts`, so the lane copy reads 2 rows `drifted`. Rebuilt from live with the lane's own `make.py`: byte-identical to the lane's except the two load files. On `aam`: 21 + 179 + 1 + 2 pass, `VERIFY OK`. |
| cbudget `clock` | LAND (+ LOW) | A hung *test* still fails at vitest's per-test wall `--testTimeout`. Only a hung *process* (open handles, a stuck worker) now lives up to stretch × budget. That is at most 8× (quick 40 min), and Bazel's class timeout (course 3600 s) still caps it (§clock). |
| cbudget `shard` | LAND after `clock` (optional) | First live `shard_count`. The law demands a covering, non-empty split, so `passWithNoTests:false` is safe while the include list holds. |
| rusttests `envelope` | LAND (+ LOW) | Beneath a real parent the file runs only with what that parent issued. A bare birth gets exactly the `repo run repo-lib:test-<suite>` grant, except the checkout comes from the cwd, there is no host-capacity door, and the elapsed is a flat 300 s (§envelope). |
| pinpub `nxsparse` | LAND | Real production fix: a sparse Nx image was refused at 3 sites. The rule equals store-g's other 25 sites; the declared extent is `size`, so it reduces to "one regular file with one link". |
| pinpub `pinned` | LAND after td-graph `demand` and after optcache, via its rebuild | Test-only. Its fixture base is `demand`'s new fixture. It shares `🧰️support/🟦️.ts` with optcache `store` (both base = live); `git merge-file` merges the two without conflict. |
| optcache `store` | LAND (before `pinned`) | Test-only private store. Its module-level `afterAll` in the shared support module runs at process end under bun 1.3.14 (probe in §optcache), so aggregate runs do not leak the private workspace. |
| dashhot `split` | **FIX FIRST → audit10 copy** | An action-cache entry can be poisoned: a basis file touched during the compile is recorded with digest `""`, and when its bytes return to the keyed bytes, every later `:build` hits that entry and every install refuses it. `repo dashboard` then breaks until a manual `--skip-cache` (§split). |
| progress `umlaut` | LAND, ordered | Sound: only `de` leaves change, whole words only. It shares the host-capacity schema with agents16 `allrun` and four test files with quiet16. Land after agents16, then rebuild with `rebuild.sh` (§order). |
| progress `lang` | LAND | Heartbeat and long-activity lines follow `SEMIO_LANGUAGE` (both languages when none is chosen); the watchdog adds one notice on the first signal. No code parses `running elapsedMs=`. |
| progress `status` | LAND after `umlaut` | Text only. |
| progress `rows` | LAND after `lang` (UX decision) | Rows leave TTYs only. The dashboard runs tasks in a PTY (`⌨️tui/🚇️pty`) but parses no rows; the MCP client reads pipes. |
| windows `winlaunch` | LAND | POSIX behaviour unchanged (EvalSymlinks, no-op job, no rename retry, `realpathSync`). The Windows side is only cross-compiled, so the lane's CI job is the proof. |
| windows `fleetpy` | LAND | Table row + `requireTestToolV1("python")`. |
| windows `wininstall` | **FIX FIRST → audit10 copy** | The script runs with `$ErrorActionPreference="Stop"` and renames `repo.exe` aside before it moves `.partial` in. If that move fails (Defender holding the fresh exe, the lane's own W5), no `repo.exe` is left, and the next install deletes the moved-aside copy (§wininstall). |
| playdemo `vitestext` | LAND | Bytes unchanged since audit9 (17:26). |
| playdemo `wasmlock` | LAND (+ audit9 LOW) | Unchanged since audit9; the ENOENT on a missing lock is still unguarded. |

FOR coordinator: land the audit10 copies INSTEAD of the lane copies. All are `check` = base on live 18:47, use mode-keeping `stage.py` (`test_stage_mode.py` OK), and parse-scan with 0 bad.
- multiwriter:
  - `python3 -I C/audit10-tools/staged/multiwriter/stage.py apply --root <root>`
  - verify `sh C/audit10-tools/staged/multiwriter/verify.sh <root>` (aam: `generations/🟨️.mjs 64359 B < 65536`, scan 12/0 bad, 21 + 179 + 1 + 2 pass, `VERIFY OK`)
  - revert `… stage.py revert --root <root>`
  - On a new drift: `python3 -I C/builderlock-tools/make.py <live root> --out C/audit10-tools/staged/multiwriter` (exit 1 = anchor lost).
- split:
  - `python3 -I C/audit10-tools/staged/split/stage.py apply <root>`
  - verify `sh C/audit10-tools/staged/split/verify.sh <root> [store]` (aam: intent 10/0, catalog 3/0, compiled 6/0 incl. the new law, installation 10/0, `VERIFY split: GREEN`; artifact dir `C/🗑️generated/audit10/art`)
  - revert `… stage.py revert <root>`
  - Rebuild from the lane's bytes: `python3 -I C/audit10-tools/fix_split.py`.
- wininstall:
  - `python3 -I C/audit10-tools/staged/wininstall/stage.py apply --root <root>`
  - verify `sh C/audit10-tools/staged/wininstall/verify.sh <MIRROR root>` (aam: install contract 8/0; negative control bites with the base installer, and the new order law is 0/1 with the lane's installer)
  - Rebuild: `python3 -I C/audit10-tools/fix_wininstall.py`.
- Product proof for split (as dashhot asked, plus one run): `dh4`, `dh5`, `dh6`, then `repo dashboard`. Then one probe that edits a crate-module file OUTSIDE the dashboard project root (e.g. `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/🦀️.rs`, owned by no 📋️project.json): `:build` must miss the cache and compile.
  - 199 of the 452 compiled sources of the live installation sit under no project root, so `default`/`^default` cover them only through the semio engine's crate-module inputs.
  - If that probe hits the cache, the declared inputs are incomplete.

FOR coordinator (order):
1. agents16 `allrun` → `farmfree` → `behind` (audit9).
2. Then `sh C/progress-tools/rebuild.sh` and progress `umlaut` → `lang` → `status` → `rows`. umlaut and allrun both edit `🌡️host-capacity/🧬️schema/🔣️.json` from base 8b2446ec. Run the rebuild after agents16 is committed, because `build_umlaut.py` finds test files with `git grep`, which sees tracked files only.
3. optcache `store` → td-graph `demand` → `sh C/pinpub-tools/rebuild.sh pinned` → pinned.
4. cbudget `clock` (in a process-module group, as the lane says) → `shard` later.
5. nxsparse, envelope, winlaunch, fleetpy, wininstall (audit10), split (audit10), multiwriter (audit10), vitestext, wasmlock: independent of each other.
   - Keep wasmlock with farmid `authority` (audit9).
   - multiwriter's `🕸️graph/🧪️tests/🔼️upgrade-before-custody/🟦️.ts` is the base of quiet16's re-cut.

FOR quiet16: HOLD `hold` + `lastrun` until §hold is fixed.
- Your new hold/lastrun tests pin transliterated German (`vergroessert`, `vergroessern`, `laeuft`, `ausfuehren`, `moechte`, `waechst`). After `umlaut` lands, the catalogs say `vergrößert`, so the re-cut must run `umlaut_words.json` over them.
- umlaut also edits 3 of your test files (`🚪️gate`, library `🌤️followup` and `🔼️upgrade` `🟦️.ts`).

## §hold (HIGH; it would break builds under any grant): the width gate self-deadlocks, and the birth handshake times out on the growth

**Path.**
1. `launchBazelActionBirthV1` (`🎬️executor/🟦️.ts`) connects with `{event:"join",birth}`.
2. It waits for `joined`, sends `start`, waits for `started`, and then runs the action child: `runBazelActionV1`, in process or spawned.
3. That child calls `withBazelCancellation`, which connects AGAIN on the same `SEMIO_BAZEL_CONTROL_URL` with `{event:"join"}` without a birth (role `execution`).
4. The birth socket stays open until the child ends: `closed` → `acknowledged` → `socket.destroy()` in the `finally`.
5. `hold`'s `script.ts` puts `await upgradeBuildCapacityV1(signal); await width.enter(socket,signal)` in `joined()` for BOTH roles.
6. `width.leave` runs only on socket close.

So an action holds one slot through its birth and needs a second one for its own child.

**Repro.**
- `bun C/audit10-tools/width-deadlock.ts C/quiet16-tools/staged/hold/files/03.ts` extracts the staged `createBazelActionWidthV1` and runs it.
- With jobs 1, 3 and 8 and as many births running, the children's execution joins are blocked 1/1, 3/3 and 8/8 (`DEADLOCK REPRODUCED`).
- jobs come from the grant row, so this happens under any grant once Bazel's concurrent births reach the grant's jobs. Bazel is started with `--jobs` equal to the plan's jobs, so that is the normal state of a build with enough parallel actions.
- A single action under a 1-job grant deadlocks too.

**Outcome.**
- `withBazelCancellation` arms its handshake timer only when the invocation has a deadline (`remaining===null?undefined:setTimeout(…)`). Without a deadline the build hangs forever, holding its leases and grant.
- With a deadline, every blocked action fails at the deadline.

**Second defect.**
- The birth client arms `handshake=setTimeout(fail("bazel.birth-handshake-deadline-exhausted"), min(10000 ms, deadline))` and clears it only on `started` (schema `ActionBirth.definitions.Protocol.const.maximumHandshakeMilliseconds = 10000`).
- In base, a quiet grant had already grown at the door, so the birth's `upgradeBuildCapacityV1` returned at once.
- Under `hold`, the first birth waits for the growth in the fair queue, and the waiting is the purpose of the batch (the lane's sim shows tens of seconds on h16). The width wait also counts against these 10 s.
- A birth that waits more than 10 s fails the action, and with it the build.

**Why the lane's laws missed it.**
- The width law is unit-level: one `enter` per action.
- The script law is a text law.
- No law runs a real birth and its execution join under a grant.

**Required (lane), each with a law on a real `bazelCancellationControl` socket pair under a 1-job grant:**
- (a) Only births enter the width gate. The execution join belongs to the action whose birth holds the slot, and every action child carries `SEMIO_BAZEL_MANIFEST_SHA256`, which only a birth issues.
- (b) The growth and width waits must not count against the 10 s birth handshake. Either the server sends a schema-declared `waiting` event that the client accepts and that disarms its handshake (the wait stays cancellable through the signal and socket close), or the waits move between `start` and `started` with the client disarming on `joined`.
- (c) Law: two births on a 1-job grant whose growth takes longer than 10 s both finish, one after the other.

Keep audit8's `followup-150` and audit9's `busy2`/`busy3` laws.

## §multiwriter: rebase only
- The audit9 fix (symmetric stub rule) and builderlock's memory fix (`index=undefined` before the merge) are both present.
- `make.py` re-cut on live 18:40: every anchor was found once, and the size is 64,359 B (cap 65,536).
- The only byte change against the lane copy is in `🔌️load/🟨️.mjs` and `.d.mts`. The base there now carries orchslice's `orchestrationChildrenV1` (slice closure follows declared nested runs). That is independent of the custody hooks (`builder.release()`, `body(builder)`).
- No new finding beyond audit9's not-blocking items.

## §clock: can a hung test run 8× longer and starve the gate?
**Per-test hangs.** A hung test (an awaited promise that never settles) is unchanged: `--testTimeout 300000` stays wall time inside vitest, so it fails at 300 s as before.

**Process hangs.** Only a hung process benefits from the stretch: open handles after the run, a deadlocked worker, a child that never exits.
- It is killed after ∫ dt/stretch = budget, i.e. at most 8 × budget: fundamental 2 min, quick 40 min, long 2 h, exhaustive 4 h.
- Bazel's class timeout caps it first where it is shorter (course: `eternal` 3600 s).
- On the gate host at load 40-70 on 10 cores (stretch 4-7), a hung course:test process now holds its chain step for 20-35 min instead of 5 min.
- It keeps its capacity grant for that time. That blocks peers only when memory is short, and the gate runs products one at a time.
- That cost is bounded and much cheaper than today's false RED.

**Self-inflation.** A process that spins (busy loop) adds to the load average it is stretched by. On a 4-core CI runner, a 4-worker spin alone reaches ratio ≈ 1-2.

**Follow-up (LOW).** Tell a starved process from a blocked one: count at wall speed when the child tree's CPU time over the last tick is about 0. A starved runnable process accrues CPU at about 1/stretch; a hung, blocked one accrues none.

**Windows probe (`spinSlowdownV1`).**
- It spins synchronously on the runner's event loop for 64 ms of CPU every 10 s.
- The wall time of the loop is unbounded: at stretch 8 it blocks for about 0.5 s, and a suspended process blocks for longer.
- Cap the loop at `maximumBasisPoints/10000 × cpuMilliseconds` of wall time. Longer spinning carries no information, because the result is clamped to 8 anyway.
- `process.cpuUsage()` is process-wide (GC and I/O threads included), so it underestimates the slowdown. That errs toward the old wall behaviour, which is safe.
- Windows reports thread times in about 15.6 ms steps, an error of about ±25 % on 64 ms; the EWMA 0.3 smooths it.
- Linux containers: `loadavg()` is the host's and `availableParallelism()` ignores CPU quotas, so a busy Docker host stretches a devcontainer's budgets. That is conservative.

## §shard (LOW)
- `passWithNoTests:false`: a user file filter passed into a sharded run would make 2 shards fail with "No test files found". Today the course target takes no file filter through Bazel.
- The partition law returns green when the course config is missing (`if(!existsSync(config))return;`). That should fail instead.

## §envelope: capabilities a real parent would not grant
**Beneath a parent: no escalation.**
- `matches` re-issues the grant from the parent's OWN arguments, tool, mode and root, and demands JSON equality. The file then runs with exactly what the parent issued.
- It accepts any genuine native-build-caller parent (any target), on purpose: the aggregate `repo-lib:test`.
- Malformed, tampered and `artifacts` grants are refused (the lane's tamper runs).

**Bare.** The capabilities equal `createBuildBootstrapCapabilitiesV1(root,["run","@semio-tech/repo-lib:test-<suite>"],env)`, i.e. what that target's parent issues, with three differences:
1. **The root comes from the cwd** (`nativeBirthCheckoutV1()` walks up from `process.cwd()`).
   - `bun test /abs/<mirror>/…/file` run from the live checkout is born with a native grant on LIVE.
   - With `scratch:"checkout"` (the Rust suite) the sandbox also lands in live `.cache/test-sandbox`, which the mirror seal allows.
   - A real `repo run` never grants on a checkout other than the file's.
   - Fix (LOW-MEDIUM): refuse when `nativeBirthCheckoutV1()` differs from `nativeBirthCheckoutV1(import.meta.dir)`. The library module lives in the same checkout as the files that import it.
2. **No host-capacity door.** A real parent's native launches run inside its admitted grant. A bare birth has `SEMIO_HOST_CAPACITY_GRANT` unset, so `upgradeBuildCapacityV1` answers `absent`, and suites that launch cargo (build-graph-rust) run outside machine admission. This is the same as every other bare test that spawns, but these suites are heavy.
3. **`maximumElapsedMilliseconds` is a flat 300 s**, also for the 15 s caching suites (the lane says so).

**Pre-existing, not introduced here.**
- The helper's claim "files of one bare multi-file run may repeat the identical declaration, one birth serves them all" is false.
- `bun test ./phase ./admission` (both build-inputs-storage) on `aam`: every test of the second file fails with `Build test receiver admission closed`, in both orders (phase first: 7 pass / 1 fail). Each file alone gives 7/0 and 1/0.
- The same happens with two base declarations (`test:leases` claims + frontier: 5/2 together, 4/0 and 3/0 alone).
- So `bun test ./dir` gives the truth only for the first file. This affects every declared file, and the batch adds 15 more.

## §optcache / §pinned
- Probe (scratchpad, bun 1.3.14): a module-level `afterAll` in a support module that two test files import runs once, after the LAST file, and sees the state the second file created. So `privateGraphWorkspaceV1()` created by a later file of an aggregate is still retired.
- The reaper only touches `g??????` roots whose `.owner` pid is gone, so PID reuse keeps a root, which is the safe direction.
- `withWorkingDirectoryV1` `chdir`s the whole process for a body. That is safe while the tests of a file run one at a time (bun's default), so do not mark these tests `concurrent`.

## §split: a cached artifact that is stale at birth is restored forever
**What the code does.**
- The action key is computed from the input bytes when the action starts.
- `compiledDashboardSourcesV1` (export) records digest `""` for every basis file whose `max(mtime,ctime) >= builtAtMs`, i.e. touched during the compile.
- `changedDashboardSourceV1` treats `""` as changed whenever the file's time is ≥ the build start, which stays true from then on.
- `buildDashboardArtifactV1` exports without asking whether the export is current, so the action succeeds and Bazel caches the artifact under the start key.

**Scenario.** It occurs in the fleet's normal edit-and-restore pattern: peer harness EDIT ON/OFF "restore: exact", `edit-probe.py`, agent reverts.
1. During a dashboard compile, an agent edits a basis file (e.g. a `🌱️value` `🦀️.rs`) and restores it.
2. The next `:build` has the same key → cache hit → the artifact with `""` is restored.
3. `install` refuses `dashboard.artifact-behind-sources`. The launcher path's `currentDashboardArtifactV1` is false, it runs `:build` again, gets the same hit, and gets the same refusal.
4. `repo dashboard` stays broken for that key until someone runs `--skip-cache`.

The new law reproduces the "stale forever after the bytes come back" part on the lane's own fixture tree.

**Fix (audit10 copy).**
- New `movedDashboardSourceV1(compiled)`: the first `""` path.
- `exportDashboardArtifactV1` refuses with `dashboard.artifact-sources-moved` (EN+DE in the bootstrap catalog) BEFORE `mkdirSync`/`writeRecord`. The failed action caches nothing, and the next build compiles again.
- The re-export is in `11.ts`. Docstrings are updated.

**Law.** New test in `🧾️compiled/🟦️.ts`:
- `movedDashboardSourceV1` finds the `""` path, and nothing on a clean record.
- A poisoned record stays not current after `rewrite-same` (identical bytes back).
- The export source refuses before it writes anything.
- RED with the lane's module (`Export named 'movedDashboardSourceV1' not found`, 0/1), GREEN with the copy (compiled 6/0).

**Residual (stated by the lane, still true).** A file missing from the declared inputs makes `:build` hit with an artifact that is behind its sources; install refuses it by name (safe, not live). The product probe above checks the biggest class.

## §wininstall
**Fix (audit10 copy).**
- `$aside` is remembered.
- The final `Move-Item` is retried 8× at 10…1280 ms (2.55 s, the launcher's own `renameRetrying` schedule for Defender/indexer locks).
- After the last failure, the old launcher is moved back and the error is rethrown.
- The order law now pins: cleanup → stage → aside → guarded move → backoff → restore → rethrow.
- A new feature scenario covers the failure path.

**Verification.**
- Not parsed by PowerShell here (no pwsh; same limit as the lane). The BOM is kept and the law checks it.
- The lane's CI step 3 should also cover a held `.partial`: run the installer while a reader holds `repo.exe.partial` open for longer than 2.55 s, and expect exit ≠ 0 with `repo.exe` still present.

## Method
- Diffs and live status: `C/audit10-tools/diffs.py`. Overlaps and stacking across 22 manifests (incl. agents16 and td-graph `demand`): `C/audit10-tools/overlap.py`. Output in `C/🗑️generated/audit10/*.diff`.
- Mirror `aam`, created 18:40 (default subtrees):
  - rebased multiwriter verify;
  - envelope applied, with the multi-file probes `C/audit10-tools/multifile.sh` and `multifile-base.sh`;
  - wininstall copy verify + lane negative control;
  - split copy verify + RED (`C/audit10-tools/split-red.sh`).
  - Guard clean on every run. Dropped at the end.
- Other checks:
  - `git merge-file` of pinned/optcache support: clean.
  - Transliteration scan of all staged batches against `umlaut_words.json`.
  - Dashboard compiled-source classification from the live `installed.json` (452 sources, 199 under no project root).

## Files
- `C/audit10-tools/staged/{multiwriter,split,wininstall}/` (manifest, files, base, stage.py, verify.sh).
- Tools in `C/audit10-tools/`:
  - builders: `fix_split.py`, `fix_wininstall.py`;
  - laws and probes: `width-deadlock.ts`, `multifile.sh`, `multifile-base.sh`, `split-red.sh`;
  - review: `diffs.py`, `overlap.py`.
- Logs (sweepable): `C/🗑️generated/audit10/`.
