STATUS (latest: act16 ENDED rc 1 at 19:47:41, 3 failed actions, 74 passed; see section 'act16'; earlier statuses follow):  act15 ENDED rc 1 at 14:59:50 (Bazel 77/77, 3 failed actions, 74 passed, 5964 s; table in section 'act15'). 1 infra fix applied (untested end to end), 2 compile faults owned by peers. Recommend rerun after those two compile faults are fixed. Older act14 status:  act14 ENDED rc 1 (graph planning, no Bazel action ran). GREEN for the graph: full inference of all 2815 projects with SEMIO_GRAPH_COLLECT_ERRORS = 0 failures (🗑️generated/1101/actinfra-graph.txt line 13694, actinfra-collect-summary.txt); scoped `repo` closure (40 projects) rc 0 (actinfra-showrepo.txt). Recommend RERUN act15 now. Compile/runtime status of Bazel actions still UNVERIFIED (never reached).

## Failure of act14 (only one; no Bazel action ran)
| action | class | owner | error summary | log location |
| --- | --- | --- | --- | --- |
| build:graph-plan (project `repo` assembly) | infra (graph / policy table drift) | coordinator / repo caching lane | `build.test.aggregate-refused route-targets:javascript-input-identity:0`: closed suite table `🗂️suites/🔣️.json` gained route `javascript-input-identity` but `⚡️caching/📋️project.json` had no matching `repo ./📜️script.ts test javascript-input-identity` target, so `requireCanonicalTestAggregateV1` refused at `project-assembly/🟨️.mjs:350` | act14.log lines 16872-16934 |

## Proof the aggregate check passes (seconds, no graph)
- `1101-tools/actinfra/aggregate-check.ts` (bun, calls `requireCanonicalTestAggregateV1` on the real project.json): `aggregate ok repo 192` (192 dependsOn).
- `bun test 🗂️suites/🧮️aggregate/🧪️tests/🟦️.ts`: 7 pass, 0 fail. That suite already contains the guard (`a stale or self-referential checked declaration refuses ...`, line 62 reads the real project.json), so this class of failure is caught in 0.5 s by the framework gate; add that test file to the pre-activation gate.
- Only the root project `repo` is subject to this check (`requireCanonicalTestAggregateV1` returns early when `root !== policy.root`), so no other project can raise this refusal.

## Why 100 minutes before Bazel (answer: not peer edits)
Single process (pid 1984), no restart. Phases from act14.log (line numbers): source admission/evidence 1-405 (~5 min); pass 1 `plan()` graph facts verify (473029 facts, line 438-666) + input closures verify (3999, 668-6402) about 43 min at host load 37-41; pass 2 the same two phases again (6419-10456) ~25 min; then inference of 1844 projects (10456-16872, graph worker disabled: cache declined leaves, 1655 s in-process, plus `waiting for memory (6.7 of 3.6 GB)` stalls); failure at project `repo`.
Cause of pass 2 (`🕸️graph/🕰️generations/🟨️.mjs`): `refresh()` runs `plan()` without custody; on a delta it passes the plan to `build()` as a hint, but `build()` trusts the hint only if `now() - hint.atMs <= hintMaximumAgeMilliseconds` (schema `🧬️schema/🔣️.json` = 1000 ms). Any builder custody wait over 1 s re-runs the whole `plan()`. This is deliberate and tested (`🧪️tests/🪞️plan-once`, adversarial test: an edit during the wait must be in the generation). Second (more likely, the log shows only 17 lines between the passes) mechanism: `loadGenerationGraphV1` first attempts a shared reader with no builder; `refresh()` plans, `build()` returns `needsBuild`, and the plan is dropped; the second attempt under builder custody plans again. Either way a delta costs two full verifications. Not peer edits: one process, no restart, no `[restart]` lines.

## Fix for early surfacing (applied)
Verification cost is not reduced (design is tested, correctness: edits during the wait must be built). Instead the known refusal class now fails before any verification:
- `🕸️graph/🟦️.ts` `generationHostV1.preflight()`: reads `⚡️caching/📋️project.json` and runs `requireCanonicalTestAggregateV1` (the same function project assembly uses); skipped when the file does not exist (fixture workspaces).
- `🕸️graph/🕰️generations/🔌️load/🟨️.mjs`: `await host.preflight?.()` first in `loadGenerationGraphV1` (before identity, source verification, plan); `.d.mts` gets optional `preflight?()`.
- Test `🔌️load/🧪️tests/🛫️preflight/🟦️.ts` (2 pass): a refusing preflight stops the load before `identity()`; real project.json passes, with independent Python oracle `🐍️.py` (suite ids without a target = []).
Takes effect on the next process start. Other project-assembly refusals still surface only after inference; diagnostic instrument `SEMIO_GRAPH_COLLECT_ERRORS=<dir>` (🕸️graph/🧾️collect-errors) collects every per-project failure without publishing.

## Other findings during act14
- Graph worker farm disabled again by `graph cache declined the leaves` (optional quota full, no eviction on the adoption path); in-process inference then ran 1655 s with `waiting for memory (6.7 of 3.6 GB)` stalls. Not fixed.
- `[DATA] Store dependency generation receipt stale ... env:BUN_INSTALL` is informational (did not abort).
- A peer graph build (pid 30824) blocks every other graph query (`a peer builds the graph; this run waits as its front door alone`) so graph proofs queue behind it.

## Failed actions
| action | class | owner | error summary | log |
| --- | --- | --- | --- | --- |

## Timeline act14 (pre-Bazel)
- 01:20 admitted 7/8 jobs at 46 % class size, no queue wait (act09 waited 28 min).
- 01:20-01:50 source evidence (409k ops), generation-verify, graph facts verify (473k), input closures verify: ~1 closure/s while host load 37-41 (peer git status/grep storms; act14 bun child ~0 % CPU, I/O/scheduling starved).
- 02:15 `[DATA] Store dependency generation receipt stale ... configuration-changed env:BUN_INSTALL` (informational so far; did not abort). Then a second graph-verification pass (facts + closures from 0). Watching whether it ends in Bazel.
- Disk-over-budget (27.7 GiB machine budget excess) did not block admission.

## Graph proof (no activation)
- `repo --tool=bazel show project repo --json` (scoped, fresh): rc 0, 40 projects inferred incl. `repo`, output lists `test-javascript-input-identity` (`🗑️generated/1101/actinfra-showrepo.rc/.txt`).
- `SEMIO_GRAPH_COLLECT_ERRORS=/tmp/sa-ce repo --tool=bazel graph --graph-policy=fresh` (diagnostic: infers every project, collects all failures, publishes nothing, therefore exits 1 by design): `Error collection finished: 0 of 2815 projects failed` (`actinfra-graph.txt`, `actinfra-collect-summary.txt`). Took ~1h, since the graph had to be refreshed again (stale after the project.json edit and peers' edits). No other refusal of the `aggregate-refused` kind exists.
- Cost note: even this took 3 passes of fact/closure verification (shared reader, then builder re-entry); the graph the activation now finds is not necessarily fresh if peers edit tooling/sources in between (any edit to repo tooling TS or a project.json makes it stale again and forces another delta).

## Recommendation
Rerun activation now (act15). Before it: no peer should edit `📋️project.json`/tooling TS or run graph-building commands during the first ~10 minutes; run `bun test ...⚡️caching/🗂️suites/🧮️aggregate/🧪️tests/🟦️.ts` (0.5 s) as a gate. Expected pre-Bazel time if the generation is still current: minutes; otherwise one delta refresh. Disk-over-budget warning (27.7 GiB) never blocked admission (host free 119 GB).


## act15 (started 12:51:49; graph phase took ~30 min this time, Bazel started ~13:20)
| action | class | owner | error summary | log location |
| --- | --- | --- | --- | --- |
| `@semio-tech/os-plugin-describe-rs:build:prototype` | infra (source admission) | repo native-build lane (fixed by actinfra) | `Completed Cargo Source differs from admitted artifact plan: 🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🎞️intrinsic-media/🔣️.json`. Cause: `readRustSourceClosureV1`/`selectRustSourceV1` evaluated `cfg` but not `#[test]`; a `#[test] fn` with `include_str!` in the non-`cfg(test)` module `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` (lines 303-391) counted as a live source input of the production unit, rustc never read it. | act15.log 8998; full stderr (8.2 MB, over Bazel's 1 MB cap): `.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/_tmp/actions/stderr-20` line 39292 |
| `@semio-tech/framework-os-node-graph-rs:wasm:prototype` | compile | os store replay owner (crate `replay-owned-state-operations`, 🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs) | E0277 x4 at lines 66 and 71: `Mutation: ColdMutation<_>` and `P: Clone` not satisfied for `foreign_step_source`, `fold_operation::<P, Mutation>`, `<Arc<P> as ColdFoldProjection<P, Mutation>>::advance`; the enclosing impl lacks the bounds `P: Clone, Mutation: ColdMutation<P>` that `effective-operations-and-folds` (lines 149-152, 250-251) now requires. File was edited by a peer at 14:10 (inside the 10 minute window), so not touched. | act15.log 11091; stderr file `.../_tmp/actions/stderr-27` lines 4034-4110 |
| `@semio-tech/puzzle-plugin:wasm:prototype` | compile | stdio pdf lane (crate `semio-s-artifact-stdio-pdf-schema-5`, ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/📦️packages/🦀️rust/🪞️mirror/🦀️.rs) | 5x E0433 `cannot find export in io` for `crate::standards::v1_7::subsets::base::io::export::{blank_pdf_snapshot,encode_pdf}` in the vt subset (mirror, lines 123/153/169). The mirror declares `pub mod export` under `#[cfg(any(test, feature = "testkit"))]` (line 49) but the vt schema uses it in production code. Mirror file edited by a peer at 14:33 (in flight), not touched. | act15.log 16273; stderr `.../_tmp/actions/stderr-31` lines 9944-10000 |

### Fix applied (infra): `#[test]`/`#[bench]` are cfg(test)-gated
`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts` `selectRustSourceV1`: a bare `#[test]` or `#[bench]` attribute owns its item and removes it unless cfg contains `test` (what rustc does). Tests: `🧪️tests/🟦️.ts` 46 pass incl. two new corpus cases (`production drops test functions`, rustc `-Zunpretty=expanded` oracle agrees; `test configuration keeps test functions`). Fixture `🧪️tests/🧫️fixtures/🔣️.json`. The re-export in `📦️packages/🟦️typescript/📦️artifacts/...` needs no change. Takes effect on the next run only.

### Observations (infra)
- The describe command sat in `resource-retirement-inventory` (done=128) for 36 minutes before its cargo started, while 15 peer cargos ran and 6 queued: lease-claim sweep over 1997 claim sqlite files (~13 held leases, process in uninterruptible I/O state at ~28 % CPU). No lease waiter, no deadlock; pure load. Same pattern in the bazel s2 workspace script (pid 94179). Not changed.
- Bazel truncates the failing action's stderr when over 1 MB (`--experimental_ui_max_stdouterr_bytes`); the 8 MB describe stderr was mostly warnings (unused qualification in `🌱️value`). Real errors are only in the saved files above.

### act15 timing and verdict
- Graph + planning: 12:51 to ~13:20 (29 min, vs 100 min in act14; generation warm, preflight in place). Bazel 13:20 to 14:59 (5964 s elapsed, critical path 2611 s): the long poles were describe (36 min in lease-claim sweep before cargo started, 2150 s) and puzzle-plugin wasm (2551 s, compiler cpu/host-capacity gate contention from 15 peer cargos).
- Passed: 74 of 77 actions. Failed: the 3 above. No lease/capacity/image/disk failure; disk-over-budget warning (27.6 GiB) did not block.
- Rerun: YES, after (1) `replay-owned-state-operations` bounds (`P: Clone`, `Mutation: ColdMutation<P>`) are fixed and (2) the stdio-pdf vt `export` gating is resolved. The describe fix is in place but only a rerun can prove it (it may reveal a further admission mismatch). Rerun cost should be small: warm generation (no 100 min graph phase) and disk-cached actions; keep peer cargo/graph builds off during the first minutes and, if possible, during describe and the wasm components.


## act16 (started 15:04:06, ended 19:47:41; Bazel from 15:46)
| action | class | owner | error summary | log location |
| --- | --- | --- | --- | --- |
| `@semio-tech/os-plugin-describe-rs:build:prototype` | infra (concurrent source edit) | peer editing `⏩️replay-owned-state-operations/🦀️.rs` | `Completed Cargo Source differs from admitted artifact plan: ...🏪️store/🔁️replay/⏩️replay-owned-state-operations/🦀️.rs`. The act15 `#[test]` fault is gone (fixture path no longer reported). The file was edited at 17:53:43 while describe compiled (admitted ~17:08, failed ~18:14); the refusal is correct: source changed between admission and completion. | act16.log 12075; `.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/_tmp/actions/stderr-21` |
| `semio-framework-os-infinite:fonts:prototype` | infra (deadline) | repo/infinite fonts script (fixed by actinfra) | `Original resource loan deadline exhausted` ~13 s after waiting for `semio.compiler.host-capacity/v1`: `FontsScript` (`♾️infinite/🧪️tests/🔬️rust-package-commands/🟦️.ts`) bound the whole phase (compiler capacity wait, cargo build of the font tool, publication) to a nonrenewable `fontProtocol.maximumElapsedMilliseconds` = 60 000 ms. | act16.log 15642; stderr-23 |
| `@semio-tech/puzzle-plugin:wasm:prototype` | infra (deadline), then unknown compile | hub puzzle / repo deadline policy | After 4061 s in `native:owner-command` (act15: 2600 s, then real compile errors): `Original resource loan deadline exhausted` raised by a non-progress deadline row (not the progress-deadline notice, so not the 1 h idle window or the cost ceiling). The act15 stdio-pdf E0433 compile fault was not reached again; status unknown. Root cause of the generic deadline row not found. | act16.log 16950; stderr-27 line 10615 |

### Fixes applied in act16 window
- Fonts: phase control is now unbounded (`0`, like `cargo:build` of the same file); the 60 000 ms bound applies only to running the font tool itself (`requestedCompilerAdmissionControlV1(control, fontProtocol.maximumElapsedMilliseconds)`). Waits and the cargo build of the tool keep their own renewable budgets. Syntax checked; takes effect on the next run. No test (script has none).
- `framework-os-node-graph-rs:wasm` (act15 compile fault in `replay-owned-state-operations`) no longer fails: a peer fixed it.

### The 36-minute lease-claim sweep (answer: not a real sweep inefficiency)
- Measured: the sweep phases of an action child (`lease-sweep:claims|resources|nested`, `claim-retirement-scan`, `resource-retirement-scan`) take 0.2 to 1.7 s each in the action stderr (stderr-23). `fleet status` shows the parent with the stale last-published phase `resource-retirement-inventory` for the whole action, which looked like a sweep.
- Real pre-cargo costs of describe in act16: cargo source admission of 304 units about 10 min at load 33-43 (`phase: read, unit 204 of 304` after 8 min), then the compiler gate queue (`semio.compiler.cpu-picker/v1`, 15 peer cargos at 30 % CPU per rustc), then the compile itself and artifact capture (the bun child holds 37 claim databases while it processes captures). Surface-rs wasm took 31+ min of genuine rustc work (rustc children at ~30 % CPU, load 27-43).
- Nothing changed in the lease code. A visibility improvement (publish the real phase instead of leaving the last one) was not made.

### Verdict act16
Rerun NO until: (1) peers stop editing sources of crates in the describe/wasm plans during the run (describe's plan includes `replay-owned-state-operations`); (2) the puzzle-plugin deadline exhaustion is understood (needs a run with `SEMIO_PROGRESS_DEADLINE_TRACE=1` or a look at `createIssuedResourceControlV1` requested budgets for `native:owner-command`); (3) peer cargo load is reduced (load 27-43 on 10 cores stretches each action 3-10x and exposes every deadline).


## act17 (started ~00:30 Nov 2; Bazel 22/77 when two component actions failed)
| action | class | owner | error summary | log location |
| --- | --- | --- | --- | --- |
| `semio-s-plugin-sourcing-slabs-component:component:prototype` | infra (regression, certain for ALL component actions) | peer commit ccac6866cc1 (Nov 1 00:08), repo cargo inputs | `Distinct Cargo target flag views require distinct compiler invocation scopes` | `.cache/b/p/m/bazel/workspaces/s2/0000000000000000/failures/semio-s-plugin-sourcing-slabs-component_component_prototype-0798147e437d.tail.log`, act17.log |
| `semio-hub-energy-component:component:prototype` | same | same | same error | `.../failures/semio-hub-energy-component_component_prototype-1acee1a54f74.tail.log` |

Cause: ccac6866cc1 added `nativeBuildSchedulerInputRequestV1` (`🏗️native-build/🧩️component-pass/🟦️.ts`) to the request list of every component driver (`🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts:72`). The scheduler request has no `--target` (host, `[build] rustflags = -Z threads=0`) while the passes build `--target wasm32-wasip2` (target-scoped rustflags, root `.cargo/config.toml` line 46). `withRepositoryCargoPassInputsV1` merges them into one view set and `withRepositoryCargoConfigurationViewsV1` (`🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:76`) refuses differing `CARGO_ENCODED_RUSTFLAGS`. The guard predates the commit, the request set is new, so each component driver throws before compiling (act15/16 did not have it).
Patch (not applied, tooling frozen): `1101-tools/actinfra/patches/component-scheduler-flag-views.patch` compares flag views only between selections with the same `--target`. Untested; alternative is giving the scheduler request its own reader scope. A test row (two selections, wasm target plus host, must pass; two same-target selections with different flags must still throw) belongs in the dependencies tests.
