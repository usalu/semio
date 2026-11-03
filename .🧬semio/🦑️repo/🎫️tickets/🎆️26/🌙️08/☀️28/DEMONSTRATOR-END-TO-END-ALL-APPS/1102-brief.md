# 🧭️ 1102 Fleet Brief — mit Bestand Demonstrator End To End (wave 1102)

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Folder = C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Coordinator = main chat (Opus). NEVER open, close or reopen the ticket. No sub-agents. Foreground only.

## Goal
`@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → `serve` on 6029 → `test-e2e` green and `C/1101-tools/e2eready/run.sh http://127.0.0.1:6029/` all PASS. All 8 panes (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) boot, show their fixture, act (with undo/redo) in en and de in a real browser. The coordinator alone runs activate, prepare, serve, test-e2e, graph queries and `dependencies sync|lock`.

## State at 02:30 Nov 2
- The 1101 coordinator session died at ~02:07; its agents are gone (their ids cannot be resumed; you inherit the lane). The coordinator killed their orphaned runs except the slabs component verification (`C/🗑️generated/1101/actinfra-comp.txt`).
- act17 (Bazel 22/77) failed on a component-driver regression (`Distinct Cargo target flag views ...`). The fix `requireCompatibleCargoFlagViewsV1` is in the tree (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`), untested by a unit row.
- Whole-hub wasm check was GREEN at 23:33 (`gen-hub-x10`). The 01:05-02:07 patch window applied an unknown subset of `C/1101-tools/<lane>/patches/`. The coordinator re-runs the hub check (`C/🗑️generated/1102/hub-a.txt`), then starts act18.
- No pane smoke test has passed natively except energie compiling (2 tests failed on missing framework schema publication, see `1101-result-rtpanes.md`). No browser run against a server exists.

## Context to read first
`C/1102-brief.md` (this), your lane's prior result file `C/1101-result-<lane>.md` (if any), `C/1101-audit-matrix.md`. Older: `C/1028-HANDOVER.md` sections "WAVE 1030" and "RESUME POINT 10/31". Scripts in `C/1101-tools/<lane>/` may be copied to `C/1102-tools/<lane>/` and adapted (output paths → `🗑️generated/1102`).

The peer work-accounting API (`RetainedWorkOutcome{result,work}`, `RetirementStep::{Progress,Failure}(..,u64)`, staged `RetainedCloneSourceAdmission`, `NativeReceive::receiver()`, `RetainedWorkLedger`) is final. Mirror its current definitions exactly; never redesign or revert it.

## Output
- Generated output: `C/🗑️generated/1102/<lane>-*.{txt,rc}`. Never delete or sweep `🗑️generated` (no rm on it, no clean targets, no purge).
- Scripts: `C/1102-tools/<lane>/`. Patches: `C/1102-tools/<lane>/patches/`.
- Result: `C/1102-result-<lane>.md`, first line `STATUS: ...`, updated after every check. Say GREEN only for what a finished run proved; say UNVERIFIED otherwise. Cite the log file.
- Final message: at most 5 lines plus the result path.

## Building and testing
- Cargo only through `repo cargo --store=shared --configuration=prototype -- <check|test> --manifest-path <M> [--target wasm32-wasip2] --keep-going -p <crate>...`. Raw cargo is refused. Template: `C/1102-tools/chk.sh <lane>-<n> <manifest> <wasm|native> <args>` (check); copy and adapt for `test`. Usual manifests: `🌎️hub/Cargo.toml` or the plugin workspace `Cargo.toml`.
- Native cargo misses wasm-gated code: a compile verdict for a guest crate needs `--target wasm32-wasip2`.
- ONE cargo/repo run per lane at a time, small batches (a few crates). Never set `SEMIO_LEASE_PRIORITY` (activation alone runs interactive). Queue waits of 30+ minutes are normal; do not raise deadlines or limits.
- Run detached with nohup and an `.rc` marker, then wait in the foreground with repeated Bash calls: `for i in $(seq 1 18); do [ -f <rc> ] && break; sleep 30; done` (at most 9 minutes per call). macOS has no `timeout`. Background children die when your turn ends: never end your turn while your own run is in progress; record its pid and kill only that tree (cargo + rustc children) if you must stop.
- `bun test` only with explicit `./` paths (a bare path scans the whole checkout and exhausts file descriptors).
- Always `cd /Users/ueli/Documents/semio` in every Bash call. Use `/usr/bin/grep`. In zsh do not write a bare `=====`; zsh does not word-split unquoted variables (use arrays).
- Never pattern-kill (`pkill -f`, `killall`). No Monitors. Do not start servers. No `repo show`/`repo graph`.

## ACTIVATION FREEZE (in force from now until `C/🗑️generated/1102/act-green` or `C/🗑️generated/1102/window-open` exists)
act16 died from source churn mid-compile. During act18:
- ALLOWED edits: files under a `🧪️tests/` directory; crates outside the demonstrator closure (ifc-4, ifc-4x3 and their details crates, wfc, draw, layout); your own files under C.
- FORBIDDEN edits: production source in the activation closure (`🧰️framework/`, pane, stdio and hub crates), any `Cargo.toml`/`Cargo.lock` in it, `♻️mit-bestand/` outside `🧪️tests/`. Exception: a lane the coordinator names explicitly by message, for the crates it names.
- A test or review shows a production fault: diagnose it fully (root cause, file:line) and write the exact fix as a patch (unified diff or a small apply script with a fail-closed scope) in `C/1102-tools/<lane>/patches/`, plus one line in your result file. Patches are applied in the next patch window.
- When `window-open` appears: apply your patches by hand, compile-atomically, check each with a narrow wasm `--lib` check, send the coordinator one line. When `window-closed` appears the freeze applies again; finish any edit within 2 minutes.

## Editing (whenever edits are allowed)
- Peers edit the same tree concurrently: re-read before every edit, make compile-atomic edits (definition before re-export), do not revert foreign changes. If a file was modified by someone else in the last 10 minutes, do other work and retry. Do not stop because others work nearby.
- No git modifying commands (commit, stash, checkout, restore, rm, worktree). No `sed -i` codemods without a fail-closed scope. No automated rewrites across crates.
- Per AGENTS.md: schema-first, concise code, no comments inside definitions, docstrings start with a unique emoji, no compat shims/adapters/deprecations, no limit raises, no new `#[allow]`, no `.unwrap()/.expect()` in non-test code, no migration scripts in the source tree, temporary logs prefixed `[DEBUG] ` and removed before you finish, paths under 256 chars, one `🦀️.rs` per member dir, UI strings in en and de with no default language. New features get a language-agnostic test plus a third-party oracle where feasible.
- Fix root causes in the owning crate. If the root cause is in another lane's crate, write it in your result file.

## Addendum 13:05 — RESTART, act18 running, FLEET CARGO SLOTS (all lanes)
The coordinator session died ~03:05 at host load 76 (fleet overload); every lane stopped and the machine idled until 13:00. Your earlier detached runs are finished or dead; read their .rc/.txt before rerunning anything.
- act18 (`activate-prototype`, interactive) started 13:03: `C/🗑️generated/1102/act18.log`, `act18.status`. The ACTIVATION FREEZE above applies until `act-green` or `window-open`.
- EVERY `repo cargo`/`repo test`/`repo run` command a lane starts MUST run through the fleet slot semaphore: `C/1102-tools/slot.sh <lane>-<n> <command...>` (C/1102-tools/chk.sh already does). The coordinator sets the slot count in `C/1102-tools/slots.k` (3 now). Waiting for a slot is normal: wait with bounded loops, do static work meanwhile. Never bypass it, never remove other lanes' slot dirs.
- Results from before the restart: hub-a rc 1 = deadline only (no compile error). Extension wasm checks: 19/21 green; `cad-aec-building` and `flow-primitive` died on deadlines, not compile errors. describe-1 green.

## Addendum 14:45 — PATCH WINDOW OPEN (all lanes)
act18 was stopped (exit 130, 14:35): every component describe stage fails deterministically (emitter request 2 GiB + 2 MiB > reader cap). Marker `C/🗑️generated/1102/window-open` exists. No activation runs during the window; fleet slots raised to 6.
Apply order = `C/1102-window-plan.md` section 3, run as five parallel STREAMS; within a stream batches are strictly sequential, each owner applies ONLY its batch, runs the batch's narrow check through `C/1102-tools/slot.sh`, fixes what its patches broke, then creates the marker `C/🗑️generated/1102/win-<batch>-done` (e.g. `win-B1-done`) and messages the coordinator one line. The next owner in the stream waits for that marker with bounded loops.
- S1 actinfra: B0 + its new patches `emitter-request-bytes.diff` (+test) and `worker-spare-instances.diff` (update the fixtures/tests that pin the old worker flags in the same batch). Marker `win-B0-done`.
- S2 policy: B1 → B2 → (phase) B1P → (policy) B5 → (phase) B5P → (stdio) B6. Lane phase supersedes the policy twins listed in `C/1102-tools/phase/` supersedes file.
- S3 testkit: B3 → (testport) B4.
- S4 uii18n: B7 → (cadi18n) B8 → (cadi18n) B10. cadi18n `cad-mode-edit.patch` only after B7.
- S5 policy: B9; e2eready: B11 (independent).
- ALL OTHER LANES (runtime, fem, energy, gen, puzzle, gis, hubplug, cad, flowext, capext, describe, reacthost, rtpanes, stdio fixes outside B6): apply your own production fixes NOW when none of your target files appears in the plan's inventory/overlap matrix; otherwise wait for the `win-<batch>-done` marker of the batch touching them. Check each fix with a narrow wasm `--lib` check (plus native test) via slot.sh. Message the coordinator one line when your fixes are in and checked.
- Manifest/Cargo.lock edits: allowed in the window; message the coordinator, who runs `dependencies lock` once after all streams finish.
- A batch that breaks a crate it does not own: the batch owner fixes it (definition-first, compile-atomic), never reverts other lanes' work.
The window closes when the coordinator writes `window-closed` (after every stream marker exists and the hub + extension checks pass); then act19 starts and the freeze applies again.

- Addendum 15:25: window batch checks use slot labels starting with `W-` (e.g. `chk.sh W-b1-1 ...`); they get 2 extra priority slots that test runs cannot take.

## Addendum 18:15 — window closes after B1P + B4
B0 B1 B2 B3 B7 B8 B9 B10 B11 are done. The window closes as soon as `win-B1P-done` and `win-B4-done` exist; then the coordinator runs `dependencies lock`, writes `window-closed` and starts act19 (freeze again). B5, B5P, B6 are DEFERRED to the window after act19: do not apply them. Every lane: finish and check your own production fixes NOW; anything not compile-checked when `window-closed` appears becomes a patch for the next window. W- priority slots: 3.

## Addendum 18:50 — WINDOW CLOSED, act19 running, FREEZE again
`window-closed` exists. act19 (`C/🗑️generated/1102/act19.log`) started 18:50 with the B0 fixes (emitter request bytes, parallel RepoBuild workers). The ACTIVATION FREEZE section applies again: finish any edit within 2 minutes; production fixes become patches in your patches dir. Normal fleet slots: 3. Pending for the next window: B5, B5P, B6 and every new patch.

## Addendum 20:25 — MINI WINDOW W2 (45 min, named lanes only)
act19 stopped: component describe fails on every component ("Finite unlinked artifact file required"), caused by B0's retention patch (intermediate rows recorded as current). `window-open` exists again, but ONLY these lanes may edit production, and only their named patches, each narrow-checked (W- slot labels):
- actinfra: retention-intermediates-norows.diff (+ size-law test), worker-demand.diff + worker-demand-caller.diff (+ test). Bun tests.
- runtime: apply-worker-continuation-fault.py (+ law test). Wasm --lib check + rerun editor-action-producer.
- flowext: flow-retirement-push-cold-depth.py, flow-child-group-work-detached.py. Wasm --lib check.
- cad: mount-editor-unit-tests.sh (+ the pre-ported tests). Native compile of the cad editor tests.
- fem / energy / gen / puzzle / hubplug / gis / testkit: ONLY a patch you already have written and checked; message the coordinator first with the patch name.
Everything else, including B5, B5P, B6 and the policy follow-ups, stays deferred. The coordinator writes `window-closed` at ~21:10 and starts act20.

## Addendum 21:10 — W2 closed, FREEZE; slot.sh is FIFO now
`window-closed` exists; the freeze applies (production edits only as patches). act20 starts once actinfra's single-component proof (W-comp) passes. `C/1102-tools/slot.sh` is now a fair FIFO queue (W- labels first, then arrival order): just call it and wait; do not kill and requeue (you lose your place).

## Addendum 21:45 — WINDOW W3 OPEN (all lanes, until act20 starts)
The single-component proof passed describe but fails at `jco transpile` (wasm trap, actinfra investigating); act20 cannot start before that is fixed, so production edits are allowed now. `window-open` exists.
- Apply your prepared, written patches now, each narrow-checked (wasm `--lib` for guest crates, plus your native test) through `C/1102-tools/slot.sh` (FIFO; W- label for window checks). Fix what you break, definition-first, compile-atomic.
- Order constraints: S2 resumes: policy B5 → phase B5P → stdio B6 (markers win-B5-done, win-B5P-done, win-B6-done). stale's `hydration-job-authority.py` supersedes energy's creation-only fix: energy must not apply its version.
- Manifest/lock edits: tell the coordinator.
- When the coordinator writes `window-closed`, the freeze applies again within 2 minutes.

## Addendum 06:25 — W3 CLOSED, act20 running, FREEZE
`window-closed` exists; act20 started. Freeze applies (production edits only as patches; tests allowed). Slots 2. Pending for W4: runtime's window-config birth spin fix (shared path), energy's effect-only producer branch for setActiveExample (decision: option 1, framework-generic), stale's harness renew_turn, graph-layout-run port, retained-command test visibility (cad/process3d editor units), prototype-wasm debug=0.

- Addendum 06:45: during activation W- labels get NO extra slots (`slots.wextra`=0) and normal slots are 1. W- is for window batch checks only.

- Addendum 09:30: lane cargo PAUSED (slots.k=0) until act21 exits — the host (fseventsd 15.8 GB) leaves the activation only 1 job. Do static work; queued slot.sh waiters simply wait.

## Addendum 10:00 — act21 stopped, WINDOW W4 OPEN
act21 failed deterministically: build path of `semio-framework-replication-mutation-retained-field` exceeds 256 UTF-16 units (crate name appears twice). W4 order = `C/1102-window-plan-w4.md` plus: actinfra crate rename (first), actinfra readArtifactBytes precise-error patch, runtime genesis pack batch (B1), energy effect-only producer + editor dev-dep (B2), policy W4 (B3/B4), fem (B5, tests repaired not dropped), testport law test + durable-group asserts, stale renew_turn if ready. B6 debug0 stays deferred. Slots 4, W- extra 3. Check with W- labels. Manifest changes → coordinator runs `dependencies lock --all` at the end. `window-open` exists.
