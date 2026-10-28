# result-sealflip (lane sealflip, wave 1027b, 10-27 11:18-12:15): making the sealed flip land-ready

`C` = this ticket, `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Tools: `C/sealflip-tools/`. Outputs: `C/🗑️generated/sealflip/`. I used one mirror, `sfm` (live at 11:20 + B1 + D2 + the batch + the flip), and dropped it at the end. Nothing was edited live and no official run was made.

## UPDATE 14:35: THE FLIP — rehearsed on current live (sealflip G40 + agedlight G42 + C1/D1/memo*/planmemo/profilefresh live): GREEN, nothing to fix
- Rehearsal on fresh mirror `sfm2` (live 14:15, created with the `✏️s` catalog path; dropped):
  - `land.py sealed-flip apply --root` OK.
  - `post-flip.sh` GREEN (sealed laws store 15/0, trampoline 8/0, processes 3/0, interleavings 3/0, door 7/0, startup 4/1 = known baseline). Promotion 1608 files, 12.4 s.
  - hotsim (`hotsim-gflip.jsonl`): `a951ce23 ×4 → 40b0bbc7` only at the explicit promote, while the live oracle moved at both peer edits. No promotion and no exit ≠ 0 elsewhere.
  - `SEMIO_REPO_IMPLEMENTATION=rust|go bun repo --help` and `… leases status`: exit 0.
  - Full suites (sealed, door, memo/slice/verified, plan-reuse ×3, witness ×3, executor, implementation, environment, actions, fingerprint, inventory, stream, text, loader-id): failure set before the flip == after (`g-base.fails` == `g-flip.fails`; only the known envelope reds + the startup `leases-entry` red).
- `sealflip` `stage.py check` vs live says 8 applied / 7 drifted. The drift is peers' later edits on top of the landed files (memo schema/test, witness schema/test, trampoline + its law = agedlight, guide). The batch semantics are intact live: `modeVariable` = `SEMIO_REPO_SEAL`, `promotion.rule` = `explicit`, both names denied in the memo key and ignored by the witness, and the restart flag is passed to `startVerdictV1`. Do NOT revert the batch by its stage.py any more; it would refuse on the drift anyway.
- Live store: pointer `26f92099bb6d` is 27.8 h old. After the flip, the first command of any kind would promote it by itself (aged + drifted); `post-flip.sh` promotes first anyway.
- FOR coordinator: LIVE LANDING SEQUENCE
  1. Make sure no verdict run of yours is in flight (a nested `repo` started mid-run would pick up the new pointer).
  2. `python3 -I C/self-g-tools/flip.py apply` (dry run on live OK; keeps the file mode).
  3. `sh C/self-g-tools/post-flip.sh /Users/ueli/Documents/semio`. Must print `POSTFLIP exit 0` and `Promoted the sealed snapshot …`. Under fleet load the promotion may take 13-40 s, single-flight.
  4. Product runs:
     - course:build, then hot x3. Expect memo served + plan reused, ≈10 s, peak < 1.5 GB as before.
     - PEER-EDIT PROOF: append one newline to `B/⏱️profile/🟨️.mjs`, then course hot. It must still be memo-served + `[bazel] plan reused` with NO `Promoting` line; the `not promoted` notice appears at most once per 5 min. Restore the file byte-exact. In live mode this edit means a memo miss + replan.
     - PROMOTE PROOF: `SEMIO_REPO_SEAL=promote bun repo` (exit 0, prints the new digest when the tree differs), then course hot once (one memo miss/replan expected: new digest), then hot again (served).
     - print-rs:test hot, repo-dashboard-rs:build, `SEMIO_REPO_IMPLEMENTATION=rust bun repo --help` exit 0.
  5. Revert: `python3 -I C/self-g-tools/flip.py revert` (one line; the snapshots stay and are harmless).
- FOR coordinator: STANDING PROMOTE RULE (the gate is the only promoter)
  - After EVERY landing whose files are in the tool's closure, run `SEMIO_REPO_SEAL=promote bun repo` (exit 0 required) BEFORE that group's verdict runs, and only between runs. Otherwise the verdict tests the old snapshot. The closure is everything under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**`, `🧰️framework/🔨️modules/🏃️process/**`, and their imported schemas and packages; when unsure, promote, since a promote of an unchanged tree is `fresh` in < 1 s.
  - A promote that exits 1 names the blocking `path:line:col`: that landing is broken, so revert it.
  - Lanes editing the tool run with `SEMIO_REPO_SEAL=live` and never promote.
  - Self-healing without the gate: a restart after a format-stamp move promotes, and a snapshot older than 24 h promotes a drifted tree.
  - Product metric for the rule: a peer edit between landings costs nobody a replan. Each landing costs exactly one memo miss + one replan per request (attribute with `bun C/self-g-tools/attribute.ts`: every moved generation digest must have a promotion in its window).

## UPDATE 12:40: re-cut on live, verify GREEN, `stage.py check` = 15 × base against live
- FOR coordinator: READY. Apply: `python3 -I C/sealflip-tools/staged/sealflip/stage.py apply`. Verdict: hot x2 (expect one replan, because the witness policy changed), repo-dashboard-rs:build, and `SEMIO_REPO_IMPLEMENTATION=rust bun repo leases status` exit 0. Then the flip (`land.py sealed-flip apply`), `sh C/self-g-tools/post-flip.sh`, and the proof run from step 4 below. Revert: `stage.py revert`.
- Cause of the gate's `verify exit 1`:
  - (a) Mirror `gsf` was created without `✏️s/🧑‍💻dev/🧩️catalog/📣️publication`. The capture of the sealed entry lstats `✏️s`, so every promotion and `identity.ts` failed with `ENOENT … lstat …/gsf/✏️s`; the hot path printed `?`.
  - (b) After that was fixed, the hot path showed `capture`/`memo`. `hotsim.py` relied on the flip being applied in the mirror; on an unflipped mirror the default is `live`.
  - Fixes: `hotsim.py` sets `SEMIO_REPO_SEAL=sealed` itself and records the first error line of a failed run. `rebase.sh` and `verify.sh` refuse a mirror without that subtree and print the exact `mirror.py create … --only library process launcher "✏️s/🧑‍💻dev/🧩️catalog/📣️publication"` command. The coordinator's `rebase.sh` exec fix is kept (`rebase.sh.pre-gate`).
  - The batch itself did not change: 15 files, re-cut by `rebase.sh` on the live files of 12:2x.
- The `🚪️door/🧪️tests/🚦️startup` 4/1 was never a cause. `verify.sh` accepts it as the known baseline red, and its message is identical without the batch (`leases-entry reached heavy groups`). Law failure sets on `gsf` with and without the batch: identical (`gsf-base.fails` == `gsf-sf.fails`), with store 14→15 and trampoline 6→7 new laws green.
- Verified on `gsf` (recreated from live at 12:2x: B1 minus red-leasesentry, walk-footprint + seal2 live, rev3fix not live): `verify.sh` GREEN, 9 law files. Then flip + `post-flip.sh` GREEN (promotion 5.9 s), then `verify.sh` GREEN again. Hot path: `c6a6e46a ×4 → eaa48b6b` only at the explicit promotion, while the live oracle moved at both peer edits. The dashboard switch `rust` → exit 0.
- rev3fix and red-leasesentry are NOT prerequisites. The batch touches none of their files, and every check above ran without them. The flip needs only conv-g + seal2, and both are live.
- Mirror `gsf` dropped.

## FOR coordinator (landing order, with a verdict after each step)
- FOR coordinator: 1. B1, in this order (A1 is already live): `for id in conv-g red-runtimelaw red-leasesentry red-witnesshost red-gaterace docs3-conv census2 fdleak-guard docs3-census2 docs3-fdleak-guard; do python3 -I C/backlog-tools/land.py $id apply || break; done`. On `sfm` every id applied cleanly at 11:2x. Verdict: course:build + course:test + repo-dashboard-rs:build.
- FOR coordinator: 2. D2: `for id in walk-footprint rev3fix seal2; do python3 -I C/backlog-tools/land.py $id apply || break; done`. seal2 changes the loader text. Verdict: course:build + hot x3 + repo-dashboard-rs:build.
- FOR coordinator: 3. Batch `sealflip` (15 files): `python3 -I C/sealflip-tools/staged/sealflip/stage.py check`. Every file must say `base`. Today 9 say `drifted`: 8 because B1/D2 are not live yet, and the memo schema because printhot carry landed after 11:20 (its anchor still holds). After steps 1-2, run `sh C/sealflip-tools/rebase.sh $TMPDIR/sm/<your mirror>`. It copies the 15 live files into the mirror, replays `edit.py` (anchored, idempotent, refuses on a moved anchor), rebuilds the batch and runs `verify.sh`. Then `stage.py apply` (keeps file modes; mode law `test_stage_mode.py` OK). Verdict, with the default still `live`: hot x2 (expect one replan, because the witness policy changed), repo-dashboard-rs:build, and `SEMIO_REPO_IMPLEMENTATION=rust bun repo leases status` must exit 0 (exit 2 today).
- FOR coordinator: 4. The flip: `python3 -I C/backlog-tools/land.py sealed-flip apply` (flip.py now keeps the file mode; `.pre-sealflip` copy kept), then `sh C/self-g-tools/post-flip.sh` (it promotes once and now uses `SEMIO_REPO_SEAL=promote`). Verdict: course:build, hot x3 (memo served + plan reused), and THE PROOF: append one newline to `B/⏱️profile/🟨️.mjs`, run course hot. It must stay memo-served and plan-reused and print `[implementation] The live tree has 1 change(s) that are not promoted …` (at most once per 5 min). Restore the file byte-exact. Then `SEMIO_REPO_SEAL=promote bun repo` and course hot once more: one memo miss is expected, because the digest moved. Then print-rs:test hot + dashboard.
- FOR coordinator: 5. Standing rule after the flip: after every landing that touches the build tool, run `SEMIO_REPO_SEAL=promote bun repo` (exit 0 required) BEFORE the verdict runs. Agents that edit the tool use `SEMIO_REPO_SEAL=live`. Announce the rename: `SEMIO_REPO_IMPLEMENTATION=live` silently stops selecting the live tree once the batch lands.
- FOR coordinator: revert, in reverse order: `python3 -I C/self-g-tools/flip.py revert`; `python3 -I C/sealflip-tools/staged/sealflip/stage.py revert`; then D2 and B1 ids in reverse with `land.py <id> revert`. Batch apply/revert/apply round trip on `sfm`: byte-exact against `base/`.

## Verdict: not land-ready as staged; land-ready with batch `sealflip`
With B1 + D2 + flip on `sfm`, `post-flip.sh` is GREEN and the law failure set is unchanged (`bd.fails` == `flip.fails`). The laws do not catch two defects, though:
1. As staged, the flip does nothing for the hot path. In `sealed` mode a heavy command promotes whenever the live closure drifted (`coalesceMilliseconds: 0`). The sealed digest therefore follows the live digest edit for edit, and the run after each peer edit also pays a promotion (4-6 s on the mirror; self-g measured 13-40 s under fleet load). Measured with `hotsim.py` (`hotsim-current.jsonl`): run-1 01c84a, peer edit, run-2 bc4c94 (promoted, 5.3 s), run-3 bc4c94, peer edit, run-4 58309b (promoted, 6.4 s). The live-capture oracle `identity.ts` gives the same digest on every run. The graph memo key (`semioCapturedImplementationIdentity` + loader identity) and the Bazel plan receipt both contain this digest, so every peer edit still means a memo miss and a replan, exactly as in live mode.
2. Switch name collision (HIGH; live today and independent of the flip). `SEMIO_REPO_IMPLEMENTATION` is also the dashboard's repo-domain switch (`rust`|`go`). The `repo-dashboard-rs:run` parameter `repo-implementation` sets it, and the journeys set `rust`. The sealed trampoline refuses every value except sealed|live|promote. Reproduced on `sfm` with both defaults: `SEMIO_REPO_IMPLEMENTATION=rust bun repo leases status` exits 2 with `Expected SEMIO_REPO_IMPLEMENTATION=sealed|live|promote, got "rust"`. Every `repo` command spawned by a dashboard started with that parameter is refused.
3. Key pollution (MEDIUM). The graph memo key takes every `SEMIO_REPO_*` variable. The plan witness records every variable that is not ignored, because the planner spreads `process.env`. So `SEMIO_REPO_IMPLEMENTATION=rust` (dashboard) or an explicitly set seal switch gives a different memo key and plan witness for the same implementation. Alternating dashboard and shell builds would replan each time.
4. Format stamps would turn explicit promotion into exit 75 (found while designing the fix). `🧷️format-stamps` compares a process's loaded stamps with the LIVE schema at every exclusive lease. A landed stamp move would make every sealed command yield. The restart re-enters the trampoline with `SEMIO_STALE_REEXEC>=1`, which without a promotion runs the same stale snapshot until the attempts run out (exit 75).

Batch `C/sealflip-tools/staged/sealflip` fixes all four:
- **Own switch.** The policy's `modeVariable` becomes `SEMIO_REPO_SEAL`. The dashboard keeps `SEMIO_REPO_IMPLEMENTATION`.
- **Explicit promotion.** New policy block `promotion: {rule: "explicit", agedMilliseconds: 86400000, noticeMilliseconds: 300000}` (schema-closed), pure function `sealedExplicitActionV1`, and a trampoline branch. Every command, heavy or light, runs the promoted snapshot. A drift is announced in EN and DE, at most once per 5 min, through the shared `checked.json`. A promotion happens only in four cases: `SEMIO_REPO_SEAL=promote`, an empty store, a snapshot older than 24 h whose tree drifted and is not known blocked, or a restart after a stamp move (`SEMIO_STALE_REEXEC>0`; the variable name comes from the format-stamps schema, not a copy). Rule `drift` keeps the old behaviour, and the old laws run under it via `FIXTURE_PROMOTION=drift`. The trampoline takes an injectable `policy`.
- **Keys.** The memo `environmentDenied` and the witness `ignoredEnvironment` now include `SEMIO_REPO_SEAL` and `SEMIO_REPO_IMPLEMENTATION`.
- **Docs.** Sealed README (EN/DE: start rule and promotion policy) and both build-guide "Sealed snapshot" bullets (written so they read correctly before and after the flip).

## Runs on `sfm` (failure sets in `C/🗑️generated/sealflip/*.fails`)
| state | sealed store/tramp/proc/interl/mode | door / startup | memo / slice / verified | plan-reuse x3 | witness x3 | executor, impl, env, actions, fingerprint, inventory, stream, text, loader-id |
|---|---|---|---|---|---|---|
| base0 (live 11:20) | 12/0 5/0 3/0 3/0 2/0 | 7/0, 4/1 (BASELINE) | 8/0 2/0 2/0 | 5/0 15/0 3/0 | 13/0 3/0 4/0 | 5/0, 1/4 env, 2/0, 10/0, 4/0, 1/0, 12/4 env, 3/0, 3/0 |
| bd (+B1+D2) | 14/0 6/0 3/0 3/0 2/0 | same | same | same | same | same failure set, stream 16/4 |
| flip (+flip) | identical failure set to bd | | | | | |
| sffinal (+sealflip+flip) | 15/0 7/0 3/0 3/0 2/0 | 7/0, 4/1 | 8/0 2/0 2/0 | same | 13/0 3/0 4/0 | identical failure set to bd |

The startup red is the same in bd and later (`leases-entry modules` 55 > 26). Door rows stay within budget. The impl/stream reds are envelope reds (`/artifacts is not allowed`, `actual …`) and are identical on base0.

- `post-flip.sh`: GREEN before the batch (`postflip1.txt`) and after it (`postflip2.txt`). The promotion takes 3.9-4.9 s.
- New laws, red on base and green with the fix:
  - `🗂️store` "explicit promotion equals an independent SQL model over the whole grid" (360 cases, probe counts included).
  - `🚦️trampoline` "explicit promotion …" with real processes: no move on a heavy or light edit, the notice once per window in EN and DE, a promote run moves the snapshot, an empty store promotes, an aged snapshot promotes, a blocked aged tree runs the last good snapshot, a restart with `SEMIO_STALE_REEXEC` promotes, and `SEMIO_REPO_IMPLEMENTATION=rust|go` exits 0.
  - The memo key law and the witness ignore law, both extended with the two names.
  - Negative controls: forcing rule `drift` makes the explicit law fail; going back to the old variable name makes it fail; both key laws fail against the base schemas.
- tsc strict on the touched sealed and door files: 0 errors in them (13 sealed files listed; it did report my own typing errors before I fixed them). The remaining tsc errors sit in peer files (`🚧️operands`, mid-landing `📜️journal`).
- `verify.sh` GREEN. It runs the batch-applied check (flip-aware), tsc, 9 law files, the dashboard probe on the real door, and the hot path on the real tool. Hot path result: `20d73a11 x4 -> 39246ec0` only at the explicit promotion, while the live oracle moved at each peer edit (`hotsim-explicit.jsonl`, `hotsim-verify.jsonl`). The real tool with `SEMIO_STALE_REEXEC=1` after a peer edit promotes; a normal run does not.
- Process tree of `repo cache status`, live vs sealed (summed peak RSS over 3 runs each, `treerss.txt`): live 472-542 MB, sealed 497-512 MB, CPU 1.6-1.8 s for both. quietfc's memory accounting is unaffected (seal2 slimmed the sealed door to 255-273 MB).

## What today's live changes do to the flip
- **Graph memo key.** In sealed mode the loader sets `semioCapturedImplementationIdentity` from the argv digest, which is the pointer digest, so the memo stays on. The key moves only at a promotion once the batch is in. Without the batch it moves at every peer edit (finding 1).
- **envleak / actenv / planfp / rowid (`🔧️tools/🏗️bazel/📜️script.ts`).** Nothing breaks. rowid answers the listed runtime rows from executable identities (bun/node), not from the door. Unlisted rows still start `repo tools exec --light` door chains: through the trampoline that is 2 bun starts, and they never promote under the explicit rule. actenv withholds the seal switch from actions, so nested `repo` runs inside actions use the default mode: after the flip they run the promoted snapshot, which matches the outer run unless the outer run is `live`. The planner witness env issue is finding 3.
- **quietfc.** No extra memory (table above).
- **jdir.** The memo directory follows the journal directory; the sealed store lives in the repo cache, and a promotion writes there. Memo relevance is per read directory, so a write there is irrelevant unless a graph fact reads `.🧬semio/🦑️repo/⚡️cache/implementation` (not observed; low risk).
- **Format stamps.** See finding 4. The proper follow-up (FOR churn-g) is for the stamp registry to read the stamp documents of the current snapshot (`createFormatStampsV1({read})`).
- **Known limit, unchanged.** The Go launcher reads the runtime registry and restart policy from the live tree.
- **Modes.** `sfm` inherited 200 0600 files from live at 11:20, before the 11:25 fix-modes. The B1/D2 tools preserve modes (`red-g stage.py` chmods to the old mode). The flip and my stage.py keep modes.

## Promotion policy (designed and implemented in the batch)
- **When.** Never as a side effect of a command. A promotion happens on:
  - an explicit `SEMIO_REPO_SEAL=promote bun repo` (runs nothing; exit 0 fresh/promoted/served, 1 blocked; it names the blocking `path:line:col` in EN and DE);
  - bootstrap (empty store);
  - self-healing: a snapshot older than 24 h whose tree drifted and is not known blocked, or the restart after a format-stamp move.
- **By whom.** In the fleet, the landing gate only, as part of each landing that touches the tool's closure, before its verdict runs. That makes "one landing = at most one digest move = one memo miss + one replan for everybody", instead of one per peer edit. An agent editing the tool runs its own commands with `SEMIO_REPO_SEAL=live` and never promotes. Humans see the drift notice (count + first path) with both commands in it.
- **Lock-aware.** Promotion is single-flight under the store's SQLite exclusive lock: the OS releases it on death, there is no TTL, and other promoters wait up to `lock.waitMaximumMilliseconds` 90 s, then re-check, so a second promote right after a first one is `fresh`. It takes no build leases (its self-check commands are `leases status` and `capacity status`, both light). A running command keeps the snapshot it started from: the store keeps the current + 2 previous snapshots, quiet for 10 min. A nested `repo` started after a promotion inside a still-running outer build gets the new digest. To avoid that, the gate promotes between its runs, never during one.
- **Multi-agent.** All processes read one pointer; the commit point is the rename of `current.json`. A blocked tree is recorded with its fingerprint and never recaptured. The shared `checked.json` limits drift probes and notices to one per 5 min for all processes together.

## Files
- Batch: `C/sealflip-tools/staged/sealflip/{manifest.json, files/, base/, stage.py, verify.sh}`. It touches 15 files:
  - `B/🏛️sealed/{🔣️.json, 🧬️schema/🔣️.json, 🟦️.ts, 🚦️trampoline/🟦️.ts, README.md, 🧪️tests/{🗂️store,🚦️trampoline}/🟦️.ts, 🧪️tests/🧫️fixtures/📜️trampoline.ts}`
  - `B/🚪️door/🧪️tests/{🟦️.ts, 🚦️startup/🟦️.ts}`
  - `B/🧭️invocation/🕸️graph/🧠️memo/{🧬️schema/🔣️.json, 🧪️tests/🟦️.ts}`
  - `B/🔭️witness/{🧬️schema/🔣️.json, 🧪️tests/🟦️.ts}`
  - `🧰️framework/🛍️products/🦑️repo/📓️build-guide.md`
- Tools in `C/sealflip-tools/`: `edit.py` (anchored, idempotent edits; this is the rebase script), `batch.py`, `rebase.sh`, `hotsim.py`, `identity.ts` (live-capture oracle), `laws.sh` + `laws.txt`.
- Edited ticket tools of self-g (copies kept as `*.pre-sealflip`): `C/self-g-tools/flip.py` (keeps the file mode) and `C/self-g-tools/post-flip.sh` (`SEMIO_REPO_SEAL=promote`).
