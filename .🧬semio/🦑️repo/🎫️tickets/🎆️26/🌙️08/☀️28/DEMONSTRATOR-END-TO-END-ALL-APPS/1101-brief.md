# 🧭️ 1101 Fleet Brief — mit Bestand Demonstrator End To End (wave 1101)

Ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Folder = C = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS`. Coordinator = main chat (Opus). NEVER open, close or reopen the ticket.

## Goal
`@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → `serve` on 6029 → `test-e2e` green. All 8 panes (generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik) render and act in a real browser. The coordinator alone runs activate, prepare, serve, test-e2e and `dependencies sync|lock`.

## Context to read first
`C/1028-HANDOVER.md` sections "WAVE 1030" and "RESUME POINT 10/31", `C/1030-brief.md` (Addendum 11:55 describes the work-accounting API), and your lane's prior result file named in your prompt. Prior agent ids cannot be resumed: you inherit their lane. Their scripts in `C/1030-tools/<lane>/` may be copied and adapted.

The peer's 🌱️value / 🧵️job / ⏳️async work-accounting API (`RetainedWorkOutcome{result,work}`, `RetirementStep::{Progress,Failure}(..,u64)`, staged `RetainedCloneSourceAdmission` instead of `admit_owned/admit_borrowed`, `NativeReceive::receiver()`, `RetainedWorkLedger`) is final. Mirror its current definitions exactly, read them before porting a caller, never redesign or revert it. Mechanical helpers: `C/1030-tools/fwfix/autofix.py <log> <path-substr> [--apply]`, `wrapsig.py <dir>`.

## Output
- Generated output: `C/🗑️generated/1101/<lane>-*.{txt,rc}`. Never delete or sweep `🗑️generated`.
- Scripts: `C/1101-tools/<lane>/`.
- Result: `C/1101-result-<lane>.md`, first line `STATUS: ...`, updated after every check. Say GREEN only for what a finished run proved; say UNVERIFIED otherwise. Cite the log file.
- Final message: at most 5 lines plus the result path.

## Building
- Cargo only through `repo cargo --store=shared --configuration=prototype -- check --manifest-path <M> [--target wasm32-wasip2] --keep-going -p <crate>...`. Raw cargo is refused. Template: `C/1101-tools/chk.sh <lane>-<n> <manifest> <wasm|native> <args>`. Crate check manifests are usually `🌎️hub/Cargo.toml` or the plugin's workspace `Cargo.toml`.
- Native cargo misses wasm-gated code: a compile verdict for a guest crate needs `--target wasm32-wasip2`.
- One batched check per workspace at a time; never several parallel checks of your own. Capacity queue waits of 30+ minutes are normal.
- Run checks detached with nohup and an `.rc` marker, then wait in the foreground with repeated Bash calls: `for i in $(seq 1 18); do [ -f <rc> ] && break; sleep 30; done` (at most 9 minutes per call). macOS has no `timeout`. Background children die when your turn ends, so never end your turn while your own check runs.
- Always `cd /Users/ueli/Documents/semio` in every Bash call. Use `/usr/bin/grep` (the shell's `grep` alias misses matches on emoji files). In zsh do not write a bare `=====`.
- Never pattern-kill; kill only pids you recorded. No sub-agents, foreground only. No Monitors. Do not start servers.

## Editing
- Peers edit the same tree concurrently: re-read before every edit, make compile-atomic edits (definition before re-export), do not revert foreign changes. If a file you need was modified by someone else in the last 10 minutes, do other work and retry. Do not stop because others work nearby.
- No git modifying commands (commit, stash, checkout, restore, rm, worktree). No Bash `sed -i` codemods without a fail-closed scope.
- Per AGENTS.md: schema-first, concise code, no comments inside definitions, docstrings start with a unique emoji, no compat shims or adapters, no limit raises, no new `#[allow]`, no `.unwrap()/.expect()` in non-test code, no migration scripts left in the source tree, temporary logs prefixed `[DEBUG] ` and removed before you finish, paths under 256 chars, one `🦀️.rs` per member dir. New features get a language-agnostic test.
- Fix root causes in the owning crate. If the root cause is in another lane's crate, state it in your result file and fix it anyway when it is small and the file is quiet.

## Addendum 02:05 — unwrap rewrite repair
Lane `policyfix` ran an automated unwrap rewrite in framework core, which left compile errors such as `ValueError` undeclared and `?` inside closures returning `&_` (mostly 🌱️value). `policyfix` alone repairs these: do not fix them yourself, wait and re-probe. Own-lane compile errors stay yours. Automated rewrites across crates are forbidden for all lanes.

## Addendum 02:40 — framework gate (all lanes except replay, policyfix, p2d, actinfra)
The fleet is starving itself: checks wait 40+ minutes in the capacity queue, then stop at the same framework crates. Do not start new cargo checks until the marker `C/🗑️generated/1101/framework-green` exists. Wait for it with the bounded loop (at most 9 minutes per Bash call, repeat). Stop your own queued checks now if they have not reached rustc, killing only pids you recorded. Static work (reading, editing your own crates, tests to write, offline ports) continues meanwhile. Check again as soon as the marker appears, one batched check per lane.

## Addendum 03:05 — no ad-hoc graph queries
`repo show` and `repo graph` without `graph_policy=current` can rebuild the graph and use 13+ GiB. Lanes must not run them from Bash. Read `📋️project.json` directly instead. Only `actinfra` may run graph queries.

## Addendum 07:20 — hub06 owners
`C/🗑️generated/1101/hub06.txt` is the current whole-hub frontier. Owners: flow-flow-model → flowext; fem-2d-model → fem; procedural 2d/3d editor and viewer transients → gen; stdio-contract-native-print → describe; puzzle-jobs → hubplug; framework plugin-retained-command, guest-contract, reactor-jobs, publication-mounted → plugcore; plugin-time-travel, typing-edit, infinite-board-engine → plugtools. The framework gate marker still stands.
