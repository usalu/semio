# Result cargo-plan (executor cargo-plan, 2026-10-20): resumable cargo window runner

C = this ticket folder. Total write pause respected: everything written lives in `C/cargo-plan-tools/` and this file. No cargo, no `bun repo` command, no dependency sync was run (show projects pid 88832 was running). The runner was NOT executed (only `DRY=1` listings and an isolated test of its helper functions, see Verification).

## 1. Headline

| item | value |
|---|---|
| steps in the plan | 447 in total: **205 required** (8 lock + 197 cargo) and 242 optional (`TIER_B=1`: 8 lock-extra + 234 covered) |
| crates in required cargo steps | **644** distinct (workspace, crate) pairs in 110 workspaces; every step has at most 8 `-p` |
| crates in the optional tier | 1,440 more (own tests of crates whose lib is already covered by a required root) |
| workspaces to lock-refresh | 62 required (used by the window) + 60 optional = 122 candidates, all stale by mtime now |
| touched scope | 2,041 packages of 3,421 (290 workspaces), 0 touched packages left unscheduled |
| estimated duration | required about 1,430 min = **24 h** (plausible 14 to 30 h); optional tier about 1,930 min = 32 h more. Milestones: through activation closure (step 027) about 7.5 h, through touched roots (122) about 15 h, through named crates (196) about 21 h, through wasm (205) about 24 h |

The estimates are model numbers (value 10 to 18 min per step, hub lane roots 25 min, touched roots 3 min + 0.8 min per crate, named 2 min + 0.6 min per crate, lock 2 min per workspace, wasm 15 to 20 min). They are not measurements; `results.tsv` records the real seconds per step, recalibrate after step 010.

## 2. Files (all in `C/cargo-plan-tools/`)

| file | purpose |
|---|---|
| `run.sh` | the runner (zsh). Reads `steps.tsv`, resumable, see section 4 |
| `steps.tsv` | the plan: id, group, kind, alarm, manifest, sub, flags, target, packages, estimate, label (generated) |
| `plan-commands.txt` | the same plan as readable text: exact command per step, alarm, collision rule, log path, marker path |
| `gen.ts` | generator (`bun C/cargo-plan-tools/gen.ts`, 2 s): builds the package index from every workspace manifest (explicit member lists), maps touched files to crates, orders by dependency depth, chunks to at most 8 |
| `scan.sh`, `scan/` | the three `find` scans (manifests since 10-19, manifests since 10-20, `.rs` since 10-19; last scan 10:31) that `gen.ts` reads |
| `lock-members.tsv` | workspace to member manifests, used by the runner's freshness test |
| `value-family.txt`, `lane-roots.json` | frozen inputs: the 35 value-family packages (`🗑️generated/value-fix-pkgs.txt`) and the 36 hub lane roots plus extra roots (`🗑️generated/aud-act-roots.json`, `aud-act-extra-roots.json`) |
| `summary.json`, `touched.tsv` | counts per group and source; every touched crate with the reasons it is in scope |

Start (from the main session, detached; children of a subagent turn die with it):
`cd /Users/ueli/Documents/semio && nohup "C/cargo-plan-tools/run.sh" >/dev/null 2>&1 & disown` (C expanded). Monitor `C/🗑️generated/cargo-window/runner.log`; results in `results.tsv`; a stop leaves `STOPPED`. Re-run the same command to resume. If manifests change before the window (queued items 1 to 4 of HANDOVER-1020, cycle-break and lay17 deltas, lock/sync), run `C/cargo-plan-tools/scan.sh` and `bun C/cargo-plan-tools/gen.ts` first; the runner refuses a changed plan while `.done` markers exist (`ACCEPT_NEW_PLAN=1` or delete `*.done`).

## 3. The plan, ordered (step ids are the file order)

Command shape (every cargo step): `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- <check|test> --manifest-path <ws>/Cargo.toml -p a -p b ... [--target T] <flags>`. Flags: `--lib --bins --tests --keep-going --message-format=short` (bin-only packages `--bins --tests`; `test` adds `--no-run`; wasm `--lib`). Alarm 3,300 s per step (the value-fix §5 alarm) and 3,000 s for lock steps. Collision rule for every step: before each launch (and before each retry) the runner waits while any foreign `bun repo dependencies`, `bun repo show projects` or `bun repo cargo` process exists (checked with `ps`, up to 4 h, then stop). Log `C/🗑️generated/cargo-window/<id>.log`, marker `<id>.done`, errors `<id>.errs`.

| # | group | step ids | steps | crates | est | content |
|---|---|---|---:|---:|---:|---|
| 1 | lock | 001 to 008 | 8 | 62 ws | 124 min | workspaces used by the window whose manifests changed (framework, root, hub, then plugin workspaces in lockref order). Per workspace: skip if `Cargo.lock` is newer than every member manifest, else `metadata --locked --format-version 1 --manifest-path <ws>`; on a lock refusal `update --workspace --manifest-path <ws>` (online, registry lock) and `metadata` again |
| 2 | value | 009 to 016 | 8 | 48 | 92 | §5 of `result-value-fix.md` in its order: (retained-map, retained-clone, snapshot-read, persistent, registry, schema); (native-owned, edit, native-decoding, native-decoding-tests, derive, derive-expansion); the remaining 22 family crates in three steps ending with the facade `semio-framework-value`; rg-core downstream (pack-json, pack-json-float, pack-json-source-sink, tool-run, tool-run-tick-wire, replication, replication-mutation-retained, replication-mutation-messages); job + job-payload; root-workspace crates (pets-staging-draft, pets-staging, os-store-testing-prelude, os-store-testing-fixtures) |
| 3 | course, proctor | 017 to 019 | 3 | 12 | 40 | 017 root workspace: `semio-framework-course`, `-course-presence` and the 5 course companion crates; 018 `🎓️teaching/Cargo.toml` check of `teaching-proctor-domain`, `-base`, `teaching-proctor`; 019 the same two with `test --no-run` |
| 4 | activation | 020 to 027 | 8 | 44 | 195 | engine roots surface, editor, canvas (framework) and os-node-graph, plugin-describe, os-infinite, os-renderer-wgpu (root workspace, as the audit found), then the 36 hub lane roots (`semio-hub-*-component`, 5 steps of at most 8, `🌎️hub/Cargo.toml`) and `semio-hub-puzzle` |
| 5a | touched | 028 to 122 | 95 | 215 | 467 | touched crates that no other touched crate depends on (roots), dependency order inside each workspace; their lib closure covers the touched crates below them. 70 of the 95 steps are one-crate workspaces |
| 5b | named | 123 to 196 | 74 | 327 | 358 | crates named in the `result-rg-*`, `split-*`, `rs-sweep*`, `tco2`, `fixups*`, `fem-fix`, `course-img`, `cc2`, `lockref`, `value-fix` files that 5a only covers as a dependency: lib + tests again, so no listed crate is left without its own test targets |
| 6 | wasm | 197 to 205 | 9 | 42 | 150 | `check --target wasm32-unknown-unknown --lib`: surface, editor (framework), os-node-graph, os-flow-wasm, os-renderer-wgpu (root), hub puzzle; then `check --target wasm32-wasip2 --lib`: canary `semio-hub-lowpoly-component` (step 200) first, then the other 35 lane roots in five steps |
| opt | lock-extra | 206 to 213 | 8 | 60 ws | 120 | lock refresh of workspaces whose manifests changed but no required step uses |
| opt | covered | 214 to 447 | 234 | 1,440 | 1,808 | `--tests` only, for every other touched crate (tier B) |

Scope derivation (union, 2,041 packages): manifest mtime since 10-19 00:00 (1,099) and since 10-20 00:00 (394); a `.rs` file mtime since 10-19 00:00 mapped to the crate whose module directory (the directory above `📦️packages`) is the deepest ancestor (1,644); the rs-sweep scopes (830); every package name that appears in the executor result files listed in the task (435).

## 4. Runner behaviour

- Skips a step when `<id>.done` exists with rc 0 (or any rc unless `RERUN_FAILED=1`). The marker holds `rc cls attempts secs errors`.
- Compile and resolution failures (`error[E`, `could not compile`, `Source refused before compiler`, `failed to select a version`, wrong-workspace, lock refusal that persists) are recorded and the run continues; `--keep-going` is always passed. A lock refusal first triggers `update --workspace` and one retry.
- Infrastructure errors stop the run with no marker for the current step (so a rerun repeats it): `System image plist required`, `Bounded unique parent kernel roster required`, stale loaded repository, topology changed, queued cache maintenance, module export churn are retried (60 s times attempt, 6 attempts) and then stop with rc 75; `Original compiler pressure owner required`, `Closed Source Rustup request required`, `Selected newline lsof fields required`, `Build scheduler compilation failed`, ENOSPC, network or registry errors, `Original parent Script process invocation required` stop immediately with rc 75; free disk below 40 GiB stops with rc 74; an alarm that expires twice for one step stops with rc 124 (the whole process tree is killed, so no orphan cargo keeps the target lock); three consecutive failures that match no known pattern stop with rc 70; the collision wait exceeding 4 h stops with rc 75.
- Environment: `SEMIO_SCRIPT_POLICY` with `maximumElapsedMilliseconds` 14,400,000 (as `cc-tools/run.mjs` and `lockref-tools/one.sh`) and `SEMIO_BUILD_BUDGET_MS=3,600,000` (a precaution; the build-budget enforcement was not re-read in source).
- Knobs: `ONLY=<regex on "id group">`, `FROM=<n>`, `UNTIL=<n>`, `TIER_B=1`, `RERUN_FAILED=1`, `DRY=1` (prints the commands, writes nothing), `WASM_BUILD=1` (wasm steps use `build`), `MIN_FREE_GB`, `RETRIES`, `GATE_MAX`, `ACCEPT_NEW_PLAN`.

## 5. Preconditions for the window

1. `bun repo show projects` finished, `tools setup nx|bazel` done (chain9), no `bun repo dependencies lock|sync` running; the runner enforces the last part but cannot know the manifest deltas of HANDOVER item 4 are applied. Apply them, then rescan and regenerate.
2. Source tree quiet during the window: a peer manifest edit makes cargo report `Original Cargo workspace topology changed` (retried, then stop).
3. Fleet cargo runs stopped (the gate waits for them, so they delay rather than break the plan).
4. Disk 130 GiB free at 10:31; the 40 GiB guard stops the run, it deletes nothing.

## 6. Verification

- `bun C/cargo-plan-tools/gen.ts` ran (package index 3,421 packages in 290 workspaces; no unresolved name; every touched package scheduled at least once).
- `zsh -n run.sh` clean; `DRY=1` with `FROM`/`UNTIL`/`ONLY` printed the exact commands of the lock, value, course, proctor, activation, touched and wasm steps without side effects (an earlier numbering; the final ids are in `plan-commands.txt`).
- Helper functions exercised in an isolated copy (outside the repo): `busy` detects the live `bun repo show projects` (pid 88832) and its child; `classify` returns compile / retry / hard / lock / unknown / ok / compile(Source refused) for seven sample logs; `run_alarm` returns 124 after the alarm and leaves no `sleep` orphan; the disk guard and `stop` work.
- NOT run: the runner itself, any cargo, `metadata`, `update`, wasm checks, and the freshness test (`do_lock`) end to end.

## 7. Limits and open items

- The audit's flow browser package (`🎁️browser-package`) is a TypeScript package, not a Rust crate; the flow wasm crate `semio-framework-os-flow-wasm` replaced it in the wasm group.
- Wasm steps use `check`, not `build`: the real component pipeline (describe, jco transpile, wasm-opt) and the exact cargo argv of `framework-os-dev … prepare|activate` were not read. A passing check proves compile viability only; `WASM_BUILD=1` switches to `build` but does not reproduce the activation's cargo invocation either.
- The activation closure covers the path-dependency closure of the 36 hub roots only (audit §3); the metadata `depends-on` / `consumes` closure (28 demonstrator components) is not included.
- Test binaries are only linked for the proctor (`test --no-run`, step 019); everything else is `check --tests`, which proves type-checking but not linking.
- Tier A relies on lib-closure coverage (a touched crate below a touched root is compiled as a lib of the root); an error in a covered crate surfaces in the root's step. The named subset gets its own test check (5b); the rest only in the optional tier B.
- Crates behind the `semio-framework-job` / `semio-framework-value` source gate may report `Source refused before compiler` (recorded as class `compile`, run continues).
- Estimates are unmeasured; the first two lock steps and the first value step give the real rate.
- Temporary output goes to `C/🗑️generated/cargo-window/` and must be deleted when the ticket closes; `scan/`, `touched.tsv`, `lock-members.tsv` and `plan-commands.txt` are regenerable tool output and can be deleted with the tools if the ticket is cleaned.
