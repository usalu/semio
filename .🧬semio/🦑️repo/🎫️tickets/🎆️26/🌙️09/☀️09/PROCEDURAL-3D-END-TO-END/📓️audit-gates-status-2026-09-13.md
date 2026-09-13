# Gates status re-audit — 2026-09-13 (read-only, live-run, lane: audit-gates)

Re-measures the seven gates the 2026-09-12 session recorded (`📓️audit-gates-status-2026-09-12.md`,
`📓️close-out-fixes-2026-09-12.md`), which had left the ticket in an all-green state. This session found
it **not** all-green: heavy, ongoing concurrent peer/sibling-lane activity across the shared cargo build
dir and the renderer-react-engine tree (evidenced throughout via `ps`, `git status`, `git log --date=iso`
and file mtimes) has both introduced new live-churn reds and made some gates unmeasurable within budget.
No source file was edited, no modifying git command was run, no server was started/stopped. Raw logs:
`🗑️generated/audit-gates/*.txt` (delete at ticket close, per hygiene rule; this report is durable).

**Headline: 5 of 7 gates red, 1 green, 1 partially-unmeasurable.** Only two of the reds look like they
touch the user-visible procedural-3d path (§7); the rest are peer in-flight churn or repo-wide debt.

## 1. Gate-by-gate results

| # | gate | command | wall | result |
|---|---|---|---|---|
| 1 | generation3d `--lib` (literal, no `--features`) | `cargo test -p semio-s-artifact-procedural-generation3d --lib` | 4.6s | **RED**, exit 101. `147 passed; 5 failed`. See §2 — this build excludes the editor/component module entirely (default features = `[]`), so the known-red reproducer named in the brief does not even compile into this binary. |
| 1b | generation3d `--lib --features component-app-assembly` (supplemental, for parity with 09-12 and to reach the named reproducer) | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1` | killed at 59m46s (2% avg cpu) | **UNMEASURABLE this session.** Compiled in 2m07s (cache-warm), then the 390-test run made almost no progress under sustained contention from ≥5 other concurrent `cargo test`/`cargo nextest` invocations against this exact crate (§3). Killed by me after ~1h; last test in flight was `delete_selection::…never printed ok/FAILED`. Two follow-up attempts to isolate just the named reproducer (`host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`) also failed to return within budget — one hit `--exact` name-mismatch (0 tests matched, wrong qualified path), the retry with a substring filter blocked >15 min on the shared build lock and was killed. Its own file (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`) landed a fresh commit at **14:41:31 CEST**, minutes before my last attempt — i.e. it was still moving during this audit window. **Not measured; not asserted green or red.** |
| 2a | `--test example-geometry` | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry` | 5m07s | **RED**, exit 101. `16 passed; 1 failed`. Was 17/17 green at 09-12 close-out. See §4 — real perf-ceiling trip, in-flight peer fix already targets it. |
| 2b | `--test io-round-trip` | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test io-round-trip` | 2m37s | **GREEN.** `45 passed; 0 failed`. |
| 3 | `@semio-tech/procedural-plugin:test` | `NX_DAEMON=false bun nx run @semio-tech/procedural-plugin:test` | 24m02s (nx: 23m28s) | **RED**, exit 1. The 4 upstream generate tasks succeeded; the test task's own `cargo nextest run --profile fundamental` was **killed by its internal 15 000 ms budget guard** (`[budget] … exceeded 15000ms — killed`) — not a specific assertion failure. See §5. |
| 4 | `@semio-tech/procedural-js:test` | `NX_DAEMON=false bun nx run @semio-tech/procedural-js:test` | 1m17s | **RED**, exit 1. `8 fail, 3 pass` of 11 tests / 11 files. See §6 — a real, pre-existing path bug, all 8 bundled-example fixtures affected uniformly. |
| 5 | React engine `test long` | `cd 🧰️framework/…/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` | 2m15s | **RED**, exit 1. `Test Files 5 failed \| 30 passed (35)`, `Tests 20 failed \| 1059 passed (1079)`. Was fully green (`1009/1009`, `30/30` files) at 09-12 close-out; corpus also grew by 70 tests since. See §7. |
| 6 | `verify publication-retirement-authority` | `bun ./📜️script.ts verify publication-retirement-authority` | 16m09s | **RED**, exit 1. Compile failure (`E0004` non-exhaustive match), not a test failure. Was green (`2 passed`) at 09-12 close-out. See §8. |
| 7 | `dependencies literal-external` | `bun nx run workspace:verify -- dependencies literal-external` | 49s | **RED**, exit 1. `target=0, current=194, oracle-conflicts=15, toolchain-owner-conflicts=2` (was `193`/`16`/`2` on 09-12 — current drifted +1, conflict count drifted -1; same order of magnitude). See §9. |

## 2. Item 1 detail — the literal no-`--features` `--lib` run

Ran exactly the command given: `cargo test -p semio-s-artifact-procedural-generation3d --lib`. The crate's
`Cargo.toml` declares `default = []` and gates the entire editor/component module tree behind
`component-app-assembly` (`semio-framework-ui`, `semio-framework-os-flow`, dispatch-macros, etc. all become
optional deps only under that feature — `📦️packages/🦀️rust/Cargo.toml:17-19`). So the literal command
compiles and runs only the `standards::v1::…` schema/mutation/snapshot subset (152 tests total) — it never
touches `editor::generation3d::…`, where the brief's named known-red reproducer
(`host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`,
`✏️editor/🧪️tests/🔬️unit/🦀️.rs:1687`) lives. Result: **147 passed; 5 failed; finished in 1.39s**
(`🗑️generated/audit-gates/01-lib-test.txt`).

| failing test | panic | attribution |
|---|---|---|
| `mutations::binary::retained_authority_laws::insufficient_fuel_and_expired_deadline_yield_before_initializer_progress` | `P3 initializer did not reach terminal-empty close` | peer, in-flight |
| `mutations::binary::retained_authority_laws::cancelled_and_stale_aba_initializers_retire_to_terminal_empty` | `cancelled P3 initializer must terminate within its bounded owner budget` | peer, in-flight |
| `mutations::binary::tests::document_text_round_trip_with_operation_applied` | `generation3d fixture store did not reach its terminal-empty ownership witness` | peer, in-flight |
| `snapshot::text::tests::command_envelope_round_trip_holds_for_an_applied_operation` | same witness message | peer, in-flight |
| `mutations::component::tests::store_applies_widget_create` | same witness message | peer, in-flight |

All five assert against the P3/terminal-empty publication-retirement machinery in
`🧰️framework/…/🔌️plugin/🦀️.rs` and `🧰️framework/…/🏪️store/🦀️.rs`. These two files are **not** mid-edit
right now (working tree is clean there), but `git log --date=iso` shows an active, still-landing commit
stream against exactly these two files today: `8add1df147` (21:17:59 09-12), `78b716e661` (00:05:03),
`a672f196c8` (00:30:39), `b2064cc237` (03:15:38), `5b6f77afcf` (11:27:07) — five commits in ~14 hours, each
hundreds of lines, the tip landing about an hour before this run. This is the same peer retirement/store
refactor the 09-12 close-out already attributed 6 lib-test reds to (§1.3 of that report); it is still
actively landing, just now via auto-committed increments instead of sitting uncommitted. Not this ticket's
files, not a new regression — continuation of already-logged peer churn.

## 3. Item 1b — why the featured run could not be measured

`ps aux` at the time of the stall showed **at least five other `cargo test`/`cargo nextest` invocations
already running against `semio-s-artifact-procedural-generation3d` concurrently** with mine, started between
12:18 and 13:07 local time by other sessions/lanes: a filtered run on `undo_redo_round_trips_flow_graph_edits`,
a full `--test example-geometry` run, a filtered run on `generate_interactions node_graph_edit`, an
`io-round-trip` run, and my own. My own compiled process (child pid, `--test-threads=1`) accumulated only
**~2% average CPU** over the ~1 h it ran before I killed it — consistent with CPU starvation from the fleet,
not a hang in the binary itself (`🗑️generated/audit-gates/02-lib-test-feature.txt` shows real forward
progress through the alphabetically-early test names before I terminated it). A later isolation attempt hit
`Blocking waiting for file lock on artifact directory` — the shared cargo build dir's own lock, held by one
of those peers. Per this lane's own ground rules ("if cargo blocks on a lock, wait"), I waited a cumulative
~1h40m across three attempts before concluding it was not going to clear inside this session's budget.
**This is a live-load, not a code, finding**: re-run item 1b when the machine is quieter.

## 4. Item 2a detail — `example-geometry` regressed to 16/17

```
thread 'sphere_cut_with_torus_evaluates_to_the_difference_volume' panicked at
…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs:414:5:
sphere-cut-with-torus: tessellate_geometry took 2308598 us against a 2000000 us ceiling — the
tessellator regressed algorithmically, see 📓️kernel-performance-2026-09-13.md
```

That referenced report **does not exist yet** (`find`/`grep -rl` across the repo: no hits). The ceiling and
the forward reference are themselves part of an **in-flight, uncommitted peer edit**, currently live:

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — `MM`, mtime 13:06 today, +56/−? — adds the `budget: ExampleBudgetExpectation` fixture field, `assertBudgetContract`, and `EXAMPLE_BUDGET_CEILING_MICROS = 2_000_000`, explicitly citing `ticket 26/09/09/PROCEDURAL-3D-END-TO-END`, `📓️kernel-performance-2026-09-13.md`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs` — `M`, mtime 13:24 today, +119/−24 — mid-rewrite of `insert_point_into`'s Steiner-refinement cavity handling (index-based `retain` instead of full Vec rebuild), with a docstring naming the same ticket/report and describing the exact quadratic-cost defect this fixes.

So: this is the **same sibling lane** (kernel-performance, same ticket, different lane) actively fixing the
very regression its own new budget assertion just caught — the fix is mid-flight, not yet complete/committed,
and the report file it will write hasn't landed. **Attribution: in-flight sibling lane, same ticket, not
this audit-gates lane's or the close-out lane's regression.** It sits on the user-visible boolean-kernel
tessellation path (§7), but the owner is already working it.

## 5. Item 3 detail — `procedural-plugin:test` killed by its own budget guard

```
[budget] cargo nextest run --binaries-metadata …/binaries-metadata.json --no-tests warn
  --status-level fail --final-status-level fail --test-threads 7 --profile fundamental
  -- --skip quick:: --skip long:: --skip exhaustive:: exceeded 15000ms — killed.
  Trim it, or assign it to a higher level (quick/long/exhaustive).
```

`fundamental: 15_000` is a hard-coded 15-second ceiling in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:1084` (`runTestBudgeted`, the same generic
budget wrapper item 3's `example-geometry` fixture references). Per the 09-12 close-out, this exact
nextest profile normally finishes in **3.9–6.4 seconds** (`18-procedural-plugin-test-final.txt`,
`24-close-ladder-final.txt`). Under this session's measured load (§3 — 5+ concurrent cargo/nextest
invocations against the same and sibling crates) it did not get enough CPU inside 15 s and was killed by
the wrapper before any pass/fail line was ever printed — so **no assertion evidence exists either way**
for the close-ladder law or the rest of the `fundamental` profile in this run. This is a measurement
artifact of system load, not a code regression signal; re-run in isolation to get a real reading.

## 6. Item 4 detail — `procedural-js:test`, all 8 examples ENOENT

```
ENOENT: no such file or directory, open '…/📚️examples/🐚️box-shell-preview/🧪️tests/🧩️example/🔣️.json'
  at loadExample (…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts:99:30)
```

`loadExample(here, …)` (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🟦️.ts:97-99`) reads
`join(here, "🔣️.json")`, i.e. it expects the fixture JSON **co-located with the calling test file** in each
example's `🧪️tests/🧩️example/` directory. `git blame` puts this exact line at commit `ebbace9b320`
(2026-09-09 17:57:33) — unchanged since, not touched by today's budget-field edit (§4's diff hunk starts at
line 47/116, never touches lines 97-99). But the actual committed fixture files live one directory over, at
`…/🧫️fixtures/🧩️example/🔣️.json` — confirmed via `git log --follow`, which walks the same commit lineage
back to the identical 09-09 commit, i.e. **git considers today's `🧫️fixtures/…/🔣️.json` the same file
history as what the reader still expects to find under `🧪️tests/…`**. `ls` on `🧪️tests/🧩️example/` for
`box-shell-preview` shows only `🟦️.ts` + `🦀️.rs`, no json; `🧫️fixtures/🧩️example/` (folder mtime 2026-09-12
12:26) holds the `🔣️.json`. So a fixtures-vs-tests directory split landed at some point on/after 09-12 and
the TS reader was never repointed. This affects **all 8** bundled examples uniformly (box-shell-preview,
face-sweep-extrude, hexagonal-mushroom-column, box-fillet-preview, rectangle-extrude-volume, sphere-box-fuse,
sphere-cut-with-torus, rectangle-wire-preview — matches the 8-of-11-files-failed count exactly). **This is a
real, standing bug, not live peer noise** — the reader line itself hasn't moved in 4 days while its target
moved out from under it. It blocks the JS-side geometry/delivery-fixture parity check for every bundled
example; the Rust-side equivalent (`example-geometry`, item 2a) reads correctly, so this is JS/TS-only.
Not on the runtime user path — it's a dev-time parity oracle — but it is real and actionable, and cheap to
fix (repoint `join(here, "🔣️.json")` to `join(here, "../../🧫️fixtures/🧩️example", "🔣️.json")` or equivalent,
once the fixtures directory's exact final convention is confirmed with whoever did the 09-12 split).

## 7. Item 5 detail — React engine `test long`: 20 new/regressed failures

Versus the 09-12 close-out baseline (`1009 passed (1009)`, `30 passed (30)` files, three consecutive clean
runs), this run is `20 failed | 1059 passed (1079)` across `5 failed | 30 passed (35)` files — the suite
also grew by 70 tests since. `git status` on the whole renderer-react-engine tree at run time showed **25+
modified/added files** spanning `wgpu` (frame-worker, browser-boot, browser-frame-transport, renderer.rs,
shell chrome-parity), `World3dHost`, `ShellHelpers`, `NodeGraph`, `Interpreter`, `EngineCanvas`, a brand-new
`app-catalogue-attempt` feature, and `engine-contract` itself — i.e. very heavy concurrent editing of
exactly this tree while the gate ran.

| failing cluster | count | site | attribution |
|---|---|---|---|
| `🖱️world3d-interaction/🟦️.tsx` | 13/14 | `TypeError: undefined is not an object (evaluating 'target.position.set')` in `applyGumballLivePreviewPoseToObject3D`, `🧱️elements/🌐️World3dHost/🟦️.tsx:2479` | **peer, in-flight.** `World3dHost/🟦️.tsx` is ` M`, mtime **13:47:30 today**, +161/−88 lines uncommitted. |
| `🔬️engine-contract/🟦️.ts` | 4/605 | `ReferenceError: panelTreePanelHost is not defined`; `packValueFromBase64: expected pk: prefix`; `transformMode` expected `"move"`, received `"transform"` (vocabulary rename mid-flight); node-graph successor-canvas-size race (`expected 1 to be ≥ 2`) | **peer, in-flight** — the test file itself is `MM` (staged + further modified). The last one is the *same class* of flake the 09-12 close-out already fixed once with a `waitFor` (§2.4 of that report) — it can recur under load even with that fix in place, or the fix itself may be mid-edit again; not re-diagnosed further given budget. |
| `📨️browser-frame-transport/🟦️.ts` | 1/31 | — | source (`🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`) and its test (`🧪️tests/📨️browser-frame-transport/🟦️.ts`) both ` M`/`M ` live-modified. |
| `📌️view-state-carriage/🟦️.ts` | 1/3 | panel-capacity assertion | not individually traced further; consistent with the same live-churn wave (view-state/panel plumbing touched by the wgpu/World3dHost edits above). |
| `🧱️elements/🔌️PluginRuntime/🟦️.tsx` | 1/108 | instance-open retained UI lifecycle patch reuse | not individually traced further; same wave. |

None of these 20 look attributable to this audit-gates lane (read-only, no edits made) or to the settled
09-12 close-out work. The dominant cluster (13/20) is a real crash in `World3dHost` caused by a peer edit
that was still being saved as this gate ran — this **is** on the user-visible gumball/transform-gesture path
in the 3D viewer, currently broken, owned by whoever is mid-editing `World3dHost/🟦️.tsx` right now. This
whole result is a live-churn snapshot; expect it to differ again on the next re-run.

## 8. Item 6 detail — `verify publication-retirement-authority` regressed to a compile failure

Was green (`2 passed; 0 failed`) at 09-12 close-out. Now:

```
error[E0004]: non-exhaustive patterns: `&semio_framework_ui::wgpu::WindowMeasure::Number { .. }` and
  `&semio_framework_ui::wgpu::WindowMeasure::Progress { .. }` not covered
  --> 🧰️framework/…/🔌️plugin/📦️packages/🦀️rust/../../🧪️tests/🔬️world3d-host-unit/🦀️.rs:85:23
note: `semio_framework_ui::wgpu::WindowMeasure` defined here
  --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1064:5
error: could not compile `semio-framework-plugin` (lib test) due to 1 previous error
```

`🖱️ui/…/🧊️wgpu/🧩️component/🦀️.rs` is `MM`, mtime **13:42:41 today** — a live peer just added `Number`/
`Progress` variants to `WindowMeasure`. `🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs` (the file with the
non-exhaustive match) has mtime 2026-09-12 11:46 — over a day stale relative to today's enum change, hasn't
caught up. **Attribution: peer, in-flight, framework `🖱️ui`/`🔌️plugin` test harness — not
procedural/generation3d-specific, not this ticket's own file.**

## 9. Item 7 detail — `dependencies literal-external`

`target=0, current=194, oracle-conflicts=15, toolchain-owner-conflicts=2` (09-12: `193`/`16`/`2`) — drifted
by ±1 in both directions, same order of magnitude, still repo-wide pre-existing debt. `generation3d` appears
exactly once in the whole report, as 1 of ~100 crate paths listed under a single shared `rust:serde_json`
oracle-conflict row (every crate depending on `serde_json` is listed together) — not a generation3d-specific
or newly-introduced conflict. Same conclusion as 09-12: not this ticket's to fix, no procedural-3d-specific
signal.

## 10. Summary table — what changed since the 09-12 close-out, and who owns it

| gate | 09-12 close-out | 09-13 (this run) | owner |
|---|---|---|---|
| generation3d `--lib` (featured) | 379 passed / 6 failed, all peer | not measured (contention) | n/a this session |
| generation3d `--lib` (literal, no features) | not run in that form | 147 passed / 5 failed, all peer (same publication-retirement class) | peer, in-flight (continuing) |
| `--test example-geometry` | 17/17 green | 16/17 — 1 real perf-ceiling trip | peer, in-flight (same-ticket kernel-performance lane, actively fixing) |
| `--test io-round-trip` | not separately reported | 45/45 green | — |
| `procedural-plugin:test` | 16/19, 1 pre-existing (generation2d) | killed by its own 15s budget under load, unmeasured | system load, re-run needed |
| `procedural-js:test` | not run in the 09-12 audits | 3/11, 8 ENOENT — real, standing fixture-path bug | pre-existing since ≥09-12 fixtures split, unowned |
| React engine `test long` | 1009/1009, 30/30 files, 3 consecutive clean runs | 1059/1079, 30/35 files | peer, in-flight (World3dHost dominant) |
| `verify publication-retirement-authority` | green, 2/2 | compile failure (E0004) | peer, in-flight (🖱️ui WindowMeasure) |
| `dependencies literal-external` | RED, 193/16/2 | RED, 194/15/2 | repo-wide pre-existing, unowned by this ticket |

## 11. On the user-visible procedural-3d path?

- **Yes, in-flight-owned**: item 2a's tessellation-budget trip (sphere-cut-with-torus) — boolean-kernel
  tessellation performance is user-facing (preview latency); the same-ticket kernel-performance lane is
  actively mid-fix.
- **Yes, in-flight, unclear owner**: item 5's `World3dHost` gumball crash — the interactive
  transform-gesture path in the 3D viewer is currently broken by a peer edit still being saved.
- **No**: items 1/1b (publication-retirement P3 machinery, framework-owned), item 3 (measurement artifact
  of load), item 4 (JS fixture-path bug — dev/CI tooling only, not shipped), item 6 (framework `🖱️ui`/
  `🔌️plugin` test harness, not procedural-specific), item 7 (repo-wide dependency debt).

## Files referenced (read-only; none edited)

`📓️audit-gates-status-2026-09-12.md`, `📓️close-out-fixes-2026-09-12.md` (baselines read for comparison);
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml` (feature/`[[test]]`
discovery); `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`
(`test long` registration); root `📜️script.ts` (`verify publication-retirement-authority`, `verify
dependencies literal-external` registration); `.vscode/launch.json` (confirmed the `workspace:verify --
dependencies literal-external` nx spelling). Live-state files inspected via `git status`/`git log
--date=iso`/`stat -f %Sm` for attribution only (none edited): `🧰️framework/…/🔌️plugin/🦀️.rs`,
`🧰️framework/…/🏪️store/🦀️.rs`, `✏️s/…/🧊️generation3d/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/🧩tessellation/🦀️.rs`,
`✏️s/…/🧊️generation3d/…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts`,
`✏️s/…/🧊️generation3d/…/📚️examples/🐚️box-shell-preview/{🧪️tests,🧫️fixtures}/🧩️example/…`,
`🧰️framework/…/🌐️World3dHost/🟦️.tsx`, `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts`,
`🧰️framework/…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`,
`🧰️framework/🔨️modules/🖱️ui/…/🧊️wgpu/🧩️component/🦀️.rs`,
`🧰️framework/…/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs`,
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

Raw command output: `🗑️generated/audit-gates/01` through `12` (`.txt`), including the two
known-red-reproducer isolation attempts (`03`, `12`) and the killed featured `--lib` run (`02`).
