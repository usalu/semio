# result-docs-1020e: build guide for concurrent agents, EN then DE (staged, not applied)

**READY** (staged, verified, NOT applied; live-apply freeze respected). Executor `docs-1020e`, wave 1020e. `C` = this ticket. Tools and staged text: `C/docs-1020e-tools/`. Nothing outside `C/` and `/private/tmp/docs1020e*` (deleted) was written; no `bun repo`, cargo or Bazel; the doc-law tests ran in an overlay, one process at a time, niced; no sub-agent.

## What it adds to the product README (`🧰️framework/🛍️products/🦑️repo/README.md`)
The guide `Build tools guide` (EN) / `Build-Werkzeuge im Alltag` (DE), applied by `docs-build` at 18:25, gets ten new sections each, EN first, DE second, same structure (10 headings, 7 tables, 6 code fences, 73 commands, identical numbers and `SEMIO_*` names, checked): EN before `Cross-platform notes`, DE before `Hinweise je Plattform`.

| EN section | DE section | covers |
|---|---|---|
| Many agents in one folder | Viele Agenten in einem Ordner | status table of 24 features (live or staged), seven working rules |
| Incremental development | Inkrementelle Entwicklung | reuse unit and re-done work per layer, `repo incremental`, `repo concurrency`, what a Rust edit costs (before and after per-crate keys), faster loops |
| Project graph and graph policy | Projektgraph und Graph-Richtlinie | live shared reader and compact publication, generations, `--graph-policy=fresh\|current`, `SEMIO_BUILD_GRAPH_VERIFY`, why a cold graph does not converge while peers edit |
| Shared caches and budgets | Geteilte Caches und Budgets | who shares what, cache ledger, `repo cache status\|gc`, lock-aware collection, dependency generations (retention, admission), CI remote cache, across machines |
| Host capacity | Rechnerkapazität | `repo capacity status\|run`, admission dimensions, reading a `[capacity]` wait, `SEMIO_HOST_CAPACITY=off`, ramp and bounded overtaking |
| Leases, lock order and queues | Leases, Sperrreihenfolge und Warteschlangen | lock classes and ranks, four rules, `SEMIO_LEASE_ORDER`, `repo leases status`, limits |
| Sessions: Bazel slots, Cargo lanes and Nx pools | Sitzungen: Bazel-Slots, Cargo-Lanes und Nx-Pools | per-session Bazel slots (`SEMIO_BAZEL_SERVERS`, `SEMIO_BAZEL_SERVICES`), Cargo lanes, unit cache, shared build-script runs, Nx graph receipt, daemon cap, put slots |
| Test sandbox | Test-Sandbox | API, ports, reclaim, the law and its ledger, `SEMIO_TEST_GUARD` |
| Parity verification | Paritätsprüfung | `repo verify build-parity`, `repo inputs` |
| Recovery after a reboot or a crash | Wiederherstellung nach Neustart oder Absturz | what the boot sweep recovers and leaves, five steps |

Eighteen anchored edits (derived from the README itself, every anchor unique): both section blocks; entry sentence of the guide; command block (`repo dependencies ensure`, `capacity`, `incremental`, `verify build-parity`) and operations table rows (`dependencies ensure`, `capacity`, `incremental`, `inputs`, `verify build-parity`, plus `concurrency` while live); Bazel default statement (Nx still default, flip gated on the three product runs, pin the tool in scripts and CI); the stale "No target in the repository declares it yet" of the Remote execution section (class table `🏗️build/🏃️execution/🔒️hermetic`, three canary projects); the "A command waits" bullet (`[capacity]` lines, lease status); a pointer from the incremental recipe to `repo incremental`; a dated Verified / Not yet verified paragraph. Eight further edits are deferred until their feature is live (`repo leases status` and `repo cache status [--tasks]|gc` in the command block and the operations table).

## Live or staged: decided from the tree, enforced at apply
Each feature has a probe (catalogue operation / flag / subcommand, a directory, a policy value) in `docs-1020e-tools/lib.py`; the fragments use `{{@id}}`, `{{?id::live::staged}}`, `{{^id::today::staged}}`. `make-edits.py` renders them against the live tree and records the states in `staged/edits.json`; `apply.py` recomputes them and REFUSES when any moved ("rerun make-edits.py"), so a staged statement can never be applied after the feature went live or the reverse. Proven in the rehearsal and in practice: `concurrency` and `crate-keys` went live while I worked (the coordinator applied conc-harness and gran-keys at about 22:09) and the first apply attempt refused. States at 22:18: live = capacity, incremental, concurrency, sync-incr, crate-keys, input-precision, sandbox, parity, boot-sweep, hermetic classes; staged = capacity ramp/fairness, leases, cache ledger, graph policy and generations, Bazel sessions, Cargo lanes and build scripts, Nx graph receipt/daemons/put slots, dependency generations and admission, TypeScript units, CI, Bazel default flip. After any set lands: `sh C/docs-1020e-tools/refresh.sh` (regenerates from the pristine README, checks, dry run); a second apply REFRESHES the state words (the tool remembers what it wrote, backup in `backup/`).

## Sources that overrode the earlier reports
- `repo capacity`, `repo incremental`, `repo dependencies ensure`, `repo verify build-parity` and `repo inputs` were live (round 2 and cli-zt) but undocumented; the README now documents them. `verify` is a root command, not a catalogue operation: the guide says so and gives the `bun ./📜️script.ts verify build-parity` form.
- The result headers of several 1020d sets said "not applied" while integrate2 applied them at 18:24-18:33 (mem-gov base, boot-sweep, sync-incr, bzl-incr, parity, input-precision, pg-chain, bzl-nocache); the live catalogue and directories decided.
- conc-bazel mentions `SEMIO_BAZEL_REMOTE_CACHE`; the live adapter has no such variable, so the guide does not mention it. The shipped contract is `SEMIO_REMOTE_CACHE_*`.
- READY2/READY3 of conc-deps, conc-nx, conc-cargo, conc-graph and the new ci-bazel (22:07) were folded in (generation-scoped maintenance and size admission, daemon cap and put slots, shared build-script runs, settle-free snapshot publication and shared fact blocks, CI workflow).

## Verification
- `check.py` (read only): every `repo` command of both languages and of the edits against the live catalogue (live operations) or against the tokens their staged tool directories contain (`leases`, `concurrency`, `cache gc`); `verify build-parity` against the argument grammar in its module; every backticked flag known; every `SEMIO_*` name occurs in a result file; every backticked path resolves or is a staged path whose last segments occur in a result file; fences, table widths, U+FE0F in heading emoji; EN/DE equal headings, tables, fences, commands, inline commands, `SEMIO_*` sets and the digits of every section (numbers cannot diverge); secret scan. Negative controls reject a bogus flag, an unknown operation, a flag on the wrong operation, a mistyped path and a changed number (five of five).
- `selftest.sh` (README copy): dry run, write (18), rerun and second write (already applied), features forced live (4 pending, 8 refresh, state change without regeneration refused), peer edit of an applied anchor refused, undo byte-identical.
- Doc laws in an overlay of the checkout (`lawtest.sh`, `overlay.py`: symlink mirror, real directories on the path of every no-follow read, library cloned, private git repository inside the overlay; baseline and applied README):

| law | baseline | with the staged edits |
|---|---|---|
| `owner-projection-policy` | 34 pass | 34 pass |
| `readme-current-source-revision` | 33 pass | 33 pass |
| `frozen-markdown-coordinates` | 35 pass | 35 pass |
| `testing-readme-coordinates` | 14 pass | 14 pass |
| `readme-current-source-activation` | 10 pass | 10 pass |
| `readme-reviewed-fixture-inputs` | 2 pass | 2 pass |
| `readme-move-source-authority` | 13 pass, 4 fail | 13 pass, 4 fail (red before; law-drift row 94) |

  Two tests cannot run in an overlay (they need the workspace-wide git index and registry producer inputs) and read no product README: `owner-projection-policy` "actual scoped normalization …" (scope `.devcontainer/README.md`; passed 35/35 against the unmodified tree earlier today) and `frozen-markdown-coordinates` "a scoped transaction preserves …" (36/36 earlier; it also has its own 15 s timeout, which it exceeded once at load 30). After the apply run all seven once on the live tree.

## Apply (after the coordinator's GO; docs only, no tree lock needed)
```
cd C/docs-1020e-tools
sh refresh.sh            # regenerate for the current tree, check.py, dry run
sh refresh.sh --write    # all-or-nothing, atomic rename, backup + applied.json
python3 -I apply.py --undo
```
Apply it after the sets it describes land (or now: staged words refresh later). Drift (README changed since the base) is refused; `--rebase` only when every anchor still matches. Then run the seven laws above on the live tree, one at a time.

## Statements that age (refresh with `refresh.sh`)
Everything marked staged; the dated Verified paragraph; the cold-graph rates (6 to 33 projects per minute, nine restarts) once conc-graph lands; "the workflow has not run yet" (CI); "no repository target has been run hermetically" after the canary probes; the default-flip sentence when `defaultTool` becomes `bazel`.

## Not done
Not documented because it is not defined: a `repo dependencies gc` verb, a CLI for Bazel slot status, a `--lane` flag, a remote layer for the Cargo unit cache. No live `bun repo` run of any documented command (freeze); all measured numbers are quoted from the executors' results and labelled overlay or live as they are.

# ROUND 2 (READY2, wave 1021a): ten more sections, EN then DE (staged, not applied)

Tools and staged text: `C/docs-1021a-tools/` (a copy of the round-1 tools that continues from the record of the first apply, 00:06; `applied.json`, `backup/README.79cd6f59….md`). The README at 12:00 already carries round 1 plus peers' edits (sha `e6b6dabb…`); all 22 round-1 texts are still present exactly once, so this round extends and refreshes them in one apply: `cd C/docs-1021a-tools && sh refresh.sh` (regenerate, check, dry run: 2 pending, 28 deferred rows), `sh refresh.sh --write`, `python3 -I apply.py --undo`. Round 1 is not applied again; `docs-1020e-tools` is finished.

## New sections (EN before `Cross-platform notes`, DE before `Hinweise je Plattform`; 20 headings, 10 tables, 14 fences, 152 commands per language, identical)
The native launcher and the runtime registry (routes, `repo --version`, provisioning, registry swap `SEMIO_RUNTIME`, one command form, runtime facade with Bun and Node adapters) · The tool registry and the direct-spawn law (`repo tools list|verify|exec`, `repo verify tool-spawns`, 25 tools, pins as pointers, sanitized environment, `SEMIO_TOOL_GUARD`) · Seams (table runtime, tools, build backend, package manager `SEMIO_PACKAGE_MANAGER_<ECOSYSTEM>`, storage backend `SEMIO_NATIVE_STORAGE_BACKEND`; `--tool=semio`) · The conformance suite (`repo conformance`, 20 scenarios, gap table nx/bazel/semio) · Polyglot units and early cutoff per language (`repo units`, interface and measured cutoff for eight languages, pinned toolchains) · The resident service and `--service` (numbers, fence, trust levels, tokens) · Agent sessions (`repo session`, `SEMIO_AGENT_SESSION`) · The remote cache server (`repo remote-cache serve|status|gc`) · Performance objectives (`repo bench`, 32 objectives) · IDE-free (`.cursor` ban, plans owned by tickets, `agent-client` rename). Graph pinning ("builds survive edits", live) is a bullet of the graph section. Round-1 text that the new sets change is conditional: cargo lane id, sandbox root, the `bun ./📜️script.ts` fallback of the parity section, `repo cache serve` in the CI paragraph, the rules (no JavaScript runtime by hand; your identity is a session), 17 new rows of the status table.

## States at 12:06 (decided from the tree; `apply.py` refuses when one moved)
Live: graph pinning, and everything round 1 listed as staged that landed since (leases, cache ledger, graph generations and policy, Bazel slots, Cargo lanes and build scripts, Nx receipt, daemons and put slots, dependency generations, CI, TypeScript units, capacity ramp): the apply refreshes those state words too. Staged (all of wave 1021a except pinning): launcher, runtime seam, tool registry and law, storage seam, package-manager seam, backend registry and `semio`, conformance, units and toolchains, resident service, agent sessions, remote cache server, bench, IDE-free, dependency admission, default flip. Gated rows for `tools`, `session`, `service`, `remote-cache`, `bench`, `conformance`, `units` in the command block and the operations table appear when their operation is in the catalogue (28 deferred).

## Verification
- `check.py` (extended): the staged commands are checked against the tokens in the executors' tool directories (`tools`, `service`, `session`, `remote-cache`, `bench`, `conformance`, `units`, `verify tool-spawns`), 17 new environment names must occur in their executor's tools, every number and `SEMIO_*` name equal in EN and DE per section; negative controls reject an unknown subcommand, a flag on the wrong operation and an undocumented `verify` subcommand.
- `selftest.sh` rehearses the real round-2 flow on a README copy with the first-apply record: 2 pending, rerun idempotent, features forced live (6 pending), state change refused, drift refused, undo byte-identical.
- Doc laws in the overlay (`lawtest.sh`, baseline README against README with the staged edits): readme-current-source-revision 33/33 and 33/33, frozen-markdown-coordinates 35/35 and 35/35, testing-readme-coordinates 14/14 and 14/14, readme-current-source-activation 10/10 and 10/10; identical reds in baseline and with the edits: owner-projection-policy 33 pass 1 fail and readme-reviewed-fixture-inputs 1 pass 1 fail (the approval hash of an owner source differs from its pin: a peer edited it without re-pinning), readme-move-source-authority 13/4 (pre-existing). Note for the owners: on the live tree every law that loads the taxonomy is red since a peer changed the `preview-generated` command of the dev project (`bun ./🧩️catalog/📣️publication/📜️script.ts preview`) while the taxonomy contract expects `bun ./📜️script.ts preview`; the overlay repairs only that line so the laws give a signal.

## Not done / aging
Statements about staged sets are contracts and measurements from the executors' results; nothing was run through `repo`. The `bun repo` sentences of the older guide text and the `cache serve` reference text are swept by the launcher and remote-cache sets themselves (their README groups). Refresh with `sh refresh.sh` after every set lands.
