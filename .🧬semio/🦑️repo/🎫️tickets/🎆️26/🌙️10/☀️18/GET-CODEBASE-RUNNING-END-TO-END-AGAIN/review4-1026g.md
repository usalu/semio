# review4-1026g (rev4-g, wave 1026g): the staged batches that skip work on the no-op path

Reviewer: rev4-g, own reading of the staged code (result files only as pointers), 20:35-21:47. Everything ran on `mirror.py` mirrors (`rev4-v`: library + process + launcher + filesystem with `journal-fix` + `vouch` + `plan-once` copied in; `rev4-q`: quietfc batch via its own scripts, then `rev3fix` applied on top), both dropped; guard clean, live tree untouched. `C` = this ticket, `T` = `C/rev4-g-tools/tests/` (the adversarial tests, copies; README.txt says where each one goes), `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

Verdict: one real stale-accept in `vouch` (HIGH 1, fix staged, hold that batch until the fix is in); one behaviour change in `plan-once` that removes the only re-check inside the builder custody (MID 2, test red); the quiet follow-up widens the quiet class to nearly every run and switches four safeguards off for it (MID 3); the plan-reuse witness does not see the file `core.excludesFile` points to, and that path goes live with `probes` (MID 4); journals on bind mounts are blind to host-side edits (MID 5). `journal-fix`, `probes` (as a table), the excluded-directory taint of `rev3fix` and the sampling/profile part of quietfc hold up.

## Ranking

| # | sev | batch | one line |
|---|---|---|---|
| 1 | HIGH | verify2 `vouch` | the recheck asks the journal twice (two fences) and `seal` stores the SECOND cursor: an input changed between the fences was skipped by the recheck and is covered by the sealed cursor, so every later run vouches the changed file |
| 2 | MID | graphlat `plan-once` | hint kept for scoped refreshes removes the replan inside the custody: edits made while the refresh waits for the builder custody (46 min seen) never reach the generation |
| 3 | MID | quietfc | follow-up rule (30 min, any earlier run of the class) + `quietAvoidsSoftStops`: pressure-warn, load, open files, attached images are off for most runs; `maximumQuiet` caps only barrier bypass, not concurrency |
| 4 | MID | witness (live) x `probes` | `core.excludesFile` (the usual `~/.gitignore_global`) is not observed: un-ignoring a file leaves the plan reusable; `probes` is what makes plan reuse fire for the first time |
| 5 | MID | journal consumers | the self-written canary cannot detect host-side edits on bind mounts (Docker Desktop VirtioFS, WSL drvfs/9p, NFS): vouch and plan-reuse cursor then skip silently |
| 6 | LOW | vouch | production glue never passes `linked`; a path through a symlinked directory is vouched while its target changed (real FSEvents test) |
| 7 | LOW | journal-fix | `root-gone` is `existsSync(root)`: a checkout swapped at the same path inside the 1 s heartbeat keeps the old inotify watches |
| 8 | LOW | probes | `ps` with ANY arguments is broad; no law shows the plan equal under different probe outputs (the code reading says it is) |
| 9 | OK | journal-fix, rev3fix excluded-directory, quietfc sampling/profile, composition | checked, notes below |

## 1. HIGH: vouch seals a cursor that covers rows the recheck skipped

Code: `verify2-g-tools/staged/vouch/new/.../📜️changes/🪪️vouch/🟦️.ts` (`openVouchV1`), `.../🧾️generation/💾️storage/🟦️.ts` (`vouched = recheck ? vouch.settled(paths) : vouch.skippable(paths)`), `.../🧾️generation/🟦️.ts` (`verifyGenerationReceiptV1`), `B/🚀️bootstrap/🟦️.ts` (`if(journal&&fast.status==="match")await journal.seal()`).

`verifyGenerationReceiptV1` calls `input.vouched` TWICE per pass: once for the input sources + manifest texts (`known`), once for the frontier rows (`settled`). In the fast pass both calls are `skippable` (rows of the open fence, no new fence). In the recheck both are `settled`: each is a fresh fence, advances `cursor` and merges the rows. `seal()` stores the last `cursor`, i.e. the fence of the SECOND call.

Failing scenario (live tree, one workspace): run R starts its recheck. Fence F1 (call 1): `package.json` of a package is untouched, so it lands in `known` and the input loop `continue`s over it. 200 ms-3 s later (the loops over 5,605 inputs, 327 texts and the evidence decode at load 30-40) a peer agent, `bun add` or an editor rewrites that `package.json`. Fence F2 (call 2) reports the row, but the input was already passed over; the frontier rows do not contain it. The recheck matches, `seal()` stores F2. Next run: `since(F2)` does not contain the change, `skippable` vouches the file, the receipt "matches" although the input differs. The dependency generation is accepted stale until the journal epoch changes (service restart, 2 h idle, memory cap) or that path is touched again; `bun install` is never re-run for it. Same for any manifest text and any frontier row read between the fences.

Evidence: `T/` copy of `⚓️anchor` (unit law on the staged module with a fake journal; calls `settled` twice, changes the input between, seals, reopens): RED on the staged module (`skippable` returns `pkg/package.json`), GREEN with the fix, vouch lifecycle/soundness/Python-model laws 8/8 green with the fix.

Fix (staged): `C/rev4-g-tools/staged/vouch-anchor/` (`vouch-anchor.diff`, fixed module and the law under `new/`): `anchor??=answer.cursor` on the first `settled` of the vouch, `seal()` writes `anchor??cursor`. Sound because every row skipped anywhere was untouched through the first fence (touched rows are re-verified after it, by either group), and an event in (F1,F2] stays visible to the next run. Cost: one extra row set the next run re-checks. The fast pass in cursor mode is unaffected (its complete walk starts after the cursor of `open`). The batch's `bases.json`/`new` hash for the module changes; gate-g takes `vouch-anchor/new/` over verify2-g's `new/` module (the law is a new file).

`HOLD vouch` until the fixed module replaces the staged one.

## 2. MID: plan-once drops the only check inside the builder custody

`graphlat-g-tools/staged/plan-once`: `hint` now exists for scoped refreshes. `refresh()` plans, then waits for the builder custody (`builder`/`lease.exclusive`), then `build()` reuses `hint.plan` and `hint.live` if `current.pointer.generation===hint.generation`. Before, a scoped refresh planned again inside the custody (that replan is the 15-30 s duplicate the batch removes). The generation gate only catches a PEER PUBLICATION; a file edited while the refresh waited is neither in the plan nor in the frontier.

Evidence: `T/plan-once-custody-wait.test.ts` (graph law harness `🏃️world`): refresh closure `p0`, one edit of `p3` before, a `builder` callback that edits `p5` (also in the closure) before running the body; compare the published generation with the independent Python graph of the disk. WITH the edit: RED (edge `p5 -> p7` and the sha of `p5` missing; computed 1). WITHOUT it: green (computed 2, walks 3). The unscoped path already behaved like this, so it is a decision, not a regression of a law; but the custody wait is the longest wait in the system (gate-g saw 46 min), and a graph that is 46 minutes older than the disk feeds the Bazel planner of the same run.

Recommendation: keep the hint only when the custody was taken within a small bound of the plan (`hint.at`, e.g. 5 s; uncontended refreshes keep the saving, a long wait replans). FOR graphlat-g.

## 3. MID: quiet follow-up turns the safeguards off for the common case

`quietfc-g-tools` (checked on `rev4-q`): the forecast is quiet when the newest run of the class, quiet OR BUSY, finished within `quietFollowUpMilliseconds` (30 min); `memory.quietAvoidsSoftStops` makes a quiet request ignore `pressure-warn`, the load room and the CPU ceiling, `files` and `attaches` (`evaluateHostCapacityV1`: `pause=avoids?undefined:soft`, `avoids||candidate*cpu<=freeLoad`). Only critical pressure, swap and memory fit still hold. With 20 agents the 30-min window is almost always open, so "quiet" is the normal admission, not the exceptional one.

Simulation (`evaluateHostCapacityV1` with the staged live policy, 16384 MB host, floor 2048): warn pressure, load 30 on 8 cores (ceiling 16), open files 81 %, 32 attached images, 9000 MB available, 10 quiet 2048 MB waiters: 3 admitted at once (6144 MB reserved); the same host admits a full request with `pressure-warn` waiting. `maximumQuiet` (4) is `quietLeft`, used only to let quiet requests pass the strict barrier; it does not cap concurrent quiet grants (more memory = more quiet admissions). The comment "a wrong forecast costs nothing because the upgrade protects" holds for the graph rebuild, the planner and the first Bazel action (all three call `upgradeBuildCapacityV1`), not for a phase before the graph: nothing upgrades before `ensureBuildEnvironmentV1` (dependency materialization inside a run, if it ever happens there, would run on the 2 GB reservation; not verified, reading only). Runs that wait for an upgrade also hold their quiet grant, the Bazel slot and the pinned generation (rev3 finding 1 class); the lend after `lendAfterMilliseconds` resolves it because the upgrade wait counts as an admission wait, so latency, not deadlock.

Recommendation: `quietAvoidsSoftStops` only for a forecast from quiet history (`followUpMilliseconds===undefined`), at most `maximumQuiet` concurrent quiet grants (count them in `evaluate`, not only for the barrier), and keep `pressure-warn` for the follow-up class. FOR quietfc-g.

## 4. MID: the plan-reuse witness does not look at `core.excludesFile`

`B/🔧️tools/🏗️bazel/📜️script.ts` `observeBazelSourceUniverseV1` lstat-observes `.git/info/exclude`, `.git/config`, `~/.gitconfig`, `$XDG_CONFIG_HOME/git/{ignore,config}`, `/etc/gitconfig` and every `.gitignore`. The file `core.excludesFile` points to (`~/.gitignore_global` is what GitHub's docs tell everyone to configure) is read by `git ls-files -co --exclude-standard`, so it defines part of the universe, but it is not observed.

Evidence: `T/witness-git-config.test.ts` (extends `🙈️excluded-directory`, real git, real planner child): `.git/info/exclude` edit makes the witness differ (`holds:false`, subject `.git/info/exclude`: fine); with `core.excludesFile` set in the repo config, `notes.tmp` ignored by that file at plan time, the file emptied afterwards: `ls-files -co --exclude-standard` now lists `notes.tmp`, witness `holds:true`, `tainted:[]`. A plan recorded before and reused after misses an input file. Pre-existing (noop2-g), but the planner skip never fired (`probes` message in `result-toolsetup-g.md`) so this was never exercised; `probes` is the batch that turns it on.

Fix: after the single-file list, resolve `git config --get core.excludesFile` (expand `~`, honour XDG default) inside `witness.allow("git")` and `lstat` it (or taint when the value cannot be resolved). FOR noop2-g / toolsetup-g; add as a law next to the `info/exclude` case above.

## 5. MID: journals on bind mounts

The service canary (`🔭️watch/🟨️.mjs canary()`) writes a file from the service and waits for its event, so it proves the watcher works for writes made INSIDE the machine that runs it. The devcontainer (`docker-compose` bind mount, `/workspaces/semio`) and WSL workspaces on `/mnt/c` receive host-side edits that VirtioFS / drvfs / 9p do not turn into inotify events. A vouch (and the plan-reuse cursor, already landed) then skips files that changed on the host. Not reproducible here (macOS native).

Fix: treat the backend as `untrusted` when the workspace's file system is not a local one (`/proc/self/mountinfo` types `virtiofs fuse.* 9p drvfs nfs* cifs smb*`, Windows network/redirected drives), with the existing EN/DE refusal text. FOR watch-g / xplat.

## 6-8. LOW

6. `openJournalVouchV1` calls `openVouchV1` without `linked`. `T/real-journal-mutations.test.ts` (real FSEvents service from the compiled watcher, mirror): after `real/x.json` changed, `alias/x.json` (`alias -> real`) is still vouched. Today no receipt path is below a symlinked directory (frontier rows are not descended through links, inputs are root files, `node_modules` is an excluded region) so it is latent; the module's header and `result-verify2-g.md` promise otherwise, and plan-reuse already passes `symlinkedDirectoriesV1`. Pass the same set (from the receipt's source paths) in the glue.
7. `journal-fix` `root-gone` tests the path, not the directory. On Linux, `mv ws ws.old && mv ws.new ws` inside the 1 s heartbeat leaves inotify watches on the old inodes while `exists(root)` is true: epoch unchanged, no events from the new tree. Record `dev:ino` of the root at start and compare.
8. `probes`: the table is closed, exact and sound where it names exact vectors (`sysctl -n kern.bootsessionuuid|kern.boottime`, `hdiutil info -plist`, `mount`); `sysctl hw.memsize`, `vm.*`, `hdiutil attach`, `mount -t ...`, shell forms and other directories still taint, so nothing that feeds admission is exempt. Reading: the outputs go to native-storage custody (boot session, roster, mount table) and liveness (`ps`); the plan is a function of `storage.paths` (from config, not from the roster), and `runBazelTasksV1` repeats `requireBazelStorageV1` in the parent under `withBazelStorageV1`, so skipping the child's check loses nothing. `ps` with ANY argument is the broad row; a law that runs the planner twice under different fake `sysctl`/`mount`/`hdiutil`/`ps` outputs and compares the plan output would pin it (not built). The first hot pair after the apply decides; if a `cannot be reused` line remains it names the vector.

## 9. Checked, no defect

- vouch (soundness part): provider rows are verified only in the fast pass and never vouched; bindings, ready fingerprints, producer and runtime are not vouched; fast pass uses `skippable` (open fence) and the recheck `settled` (fresh fence), so an event between them is re-verified in the recheck (the reason finding 1 is only about the seal). Fences: `cursor()` and `since()` both write a fence file and wait for its record. `T/real-journal-mutations.test.ts` (real FSEvents): atomic save (temp + rename), case-only rename, same-size rewrite with restored mtime, appends through a hard link, rename-in of a directory with a package.json, new directory: all reported, none vouched. 50 rounds of write-then-reopen: 0 missed. Record: keyed by receipt sha256, per consumer, per checkout journal directory, monotone per epoch, atomic rename; concurrent agents only race to a cursor that is sound for the world each of them verified. Journal refusals (disabled, no-service, starting, stale, epoch, floor, gap, lost, fence-timeout, overflow) all give empty sets.
- journal-fix: cap 128 MiB, `directory-gone`, bounded stop and the hard exit only make a service end earlier (new epoch = walk); the stored cursor of an ended service is refused by epoch.
- rev3fix excluded-directory: `bazelNonIgnoredDirectoriesV1` ends in a taint for every excluded-name directory that git does not ignore (also empty ones, also tracked ones), ignored ones stay reusable, `check-ignore` failure counts as "not ignored"; a directory created after the plan changes its parent's observed identity. Law 9/9 on the composed mirror. Cost only: such a directory makes every plan unreusable (0 exist today).
- composition: `rev3fix` applied on top of quietfc on `rev4-q`: 51/51 anchors found once (4 hash drifts, all in files quietfc edited: `gate`, `decision`, schema, `script.ts`); followup 9, upgrade 9, gate 23, handover 9, boot-anchor 3, phases 10, estimates 14, sampling 10 all green. The `probes` and `probes-after-conv` twins edit the same four files: exactly one of them can land.
- quietfc sampling/profile: `sample.json` is reused only for the same declaration (`declared` key), a declaration only shrinks the host, malformed means the real host; the planner reads host facts through `os` and `process.env`, both observed by the witness, and the worker plan is computed in the parent after the plan, so a declared machine cannot leave a stale plan.

## FOR lines

- FOR gate-g: HOLD verify2 `vouch` until `C/rev4-g-tools/staged/vouch-anchor/new/` replaces its module (HIGH 1). `journal-fix` is independent of it and may land.
- FOR graphlat-g: finding 2 (age bound on the hint; law `T/plan-once-custody-wait.test.ts`).
- FOR quietfc-g: finding 3.
- FOR toolsetup-g / noop2-g: finding 4 before `probes` makes plan reuse fire; first hot pair must also be read for `plan reused` and for `cannot be reused`.
- FOR watch-g / xplat3-g: finding 5.
- FOR verify2-g: findings 6, 7.
