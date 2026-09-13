# Gates green — 2026-09-13 (lane: gates-green)

Acts on `📓️audit-gates-status-2026-09-13.md`, which measured 5 of 7 gates red. This lane fixed forward,
at the owning layer, the gates on the procedural-3d user path. Every count below was produced by a
foreground run in this session; nothing is carried over from the audit except where a row says so.
Raw logs: `🗑️generated/gates-green/*.txt` (delete at ticket close; this report is durable).

⏱️ Measurement times, since the tree moved under several lanes all afternoon: gate 1 at 14:52, gate 2 at
17:10, gate 3 at 17:45, gate 4 at 17:48, gate 5 at 18:49. Each verdict is that gate's state at that
moment, not a claim about the tree now.

## TL;DR

| gate | before (this session) | after (this session) | verdict |
|---|---|---|---|
| 1. `@semio-tech/procedural-js:test` | `3 pass, 8 fail` — 11 tests / 11 files | `35 pass, 0 fail` — 35 tests / 11 files | **GREEN** |
| 2. React engine `test long` | `Tests 19 failed \| 1066 passed (1085)`, `Files 5 failed \| 30 passed (35)` | `Tests 1085 passed (1085)`, `Files 35 passed (35)` | **GREEN** |
| 3. `verify publication-retirement-authority` | compile failure `E0004` (never reached a test) | `2 passed; 0 failed`, exit 0 (12m59s) | **GREEN** |
| 4. `--test example-geometry` | `16 passed; 1 failed` (audit's 5m07s run) | `15 passed; 2 failed` (27.64s) | **RED — kernel lane's, not fixed here** |
| 5. generation3d `--lib --features component-app-assembly` | unmeasurable under load (audit) | `413 passed; 18 failed; 1 filtered out` of 431 (237.11s) | **RED — reported only, per brief** |

Gates 1, 2 and 3 are green and were fixed by this lane. Gates 4 and 5 are re-measurements the brief
asked for and explicitly told this lane not to fix; both are reported with attribution below.

**Two findings the brief did not anticipate, both worth acting on:**

1. The featured `--lib` suite does not merely run slowly — **it deadlocks**, and this session located
   the single test responsible (§5). That is why the 09-13 audit could not measure it in an hour.
2. `example-geometry` runs **11x faster** than the audit's run (5m07s → 27.64s) and its two remaining
   failures are NOT the one the audit saw (§4).

## 1. `@semio-tech/procedural-js:test` — fixture path, fixed schema-first

**Before** (`🗑️generated/gates-green/01-js-before.txt`, exit 1): `3 pass`, `8 fail`, `Ran 11 tests across
11 files`. All 8 bundled examples died at import time with
`ENOENT … /📚️examples/<example>/🧪️tests/🧩️example/🔣️.json`.

**Root cause.** Commit `8add1df147` (2026-09-12 21:17:59 +0200) split each example's generated fixture out
of its test directory into a sibling `🧫️fixtures/` tree — it moved the file and repointed the eight Rust
`include_str!` call sites in the same commit, but left the TypeScript reader pointing at the old location:

- `✏️s/…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts:99` — `loadExample` read `join(here, "🔣️.json")`, i.e. the
  caller's own `🧪️tests/🧩️example/` directory, which no longer holds a fixture.

`loadExample` already resolved the DSL from the example ROOT (`join(here, "../../🖼️assets", …)`), so the
two reads disagreed about what `here` meant. Fixed at the one owning layer — no per-example change:

- the two locations are now stated once, schema-first, as `EXAMPLE_ASSETS_DIR` and `EXAMPLE_FIXTURE_PATH`
  beside `EXAMPLE_GEOMETRY_FIXTURE_SCHEMA`, and `loadExample` derives the example root once and resolves
  BOTH reads from it. The eight example test files are untouched.
- stale docstring corrected in the Rust twin (`🧪️tests/🧩️geometry/🦀️.rs:6` said the fixture sits at
  `🧪️tests/🧩️example/🔣️.json`; it reads `🧫️fixtures/🧩️example/🔣️.json`).

**After** (`02-js-after.txt`, exit 0): `35 pass`, `0 fail`, `Ran 35 tests across 11 files`. The count rises
because the 8 example files now execute their real assertions instead of failing at import.

## 2. React engine `test long` — 19 failures → 0

Command: `SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` from
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`.

**Before** (`03-react-before.txt`, exit 1): `Test Files 5 failed | 30 passed (35)`,
`Tests 19 failed | 1066 passed (1085)`.
**After** (`08-react-after.txt`, exit 0): `Test Files 35 passed (35)`, `Tests 1085 passed (1085)`.

Five independent causes, each fixed at its owning layer. Every one is a consumer that a peer's landed
change left behind — no peer hunk was reverted.

### 2.1 `🖱️world3d-interaction` — 13 failures (the dominant cluster, and the one on the user path)

`TypeError: undefined is not an object (evaluating 'target.position.set')` at
`🧱️elements/🌐️World3dHost/🟦️.tsx:2477` (`applyGumballLivePreviewPoseToObject3D`), reached from
`resetGumballPreviewPosesFromInstances` (line 3195) inside the mount-time `useLayoutEffect` (3216) — so
the crash happened on MOUNT and took the whole file down, not just the gumball tests.

**Root cause, confirmed at runtime, not assumed.** A temporary `[DEBUG] ` log in `registerInstanceRoot`
printed, for every instance:

```
[DEBUG] registerInstanceRoot extrude@solid#0 object undefined HTMLUnknownElement
```

The `<group ref>` of `WorldInstanceMesh` hands back a real three.js `Group` only under the r3f
reconciler. This host also mounts under plain react-dom — the jsdom gesture suite and
`renderToStaticMarkup` — where the identical ref yields an `HTMLUnknownElement`. `registerInstanceRoot`
stored it anyway, violating its own `Map<string, Group>` invariant, and the gumball preview writer then
dereferenced `.position` on a DOM node. `World3dHost/🟦️.tsx` last moved in `d8dce87ca0`
(2026-09-13 14:41:31) — the live gumball/transform lane.

**Fix** (owning layer = the registration boundary, not the writer): new exported predicate
`isWorldInstanceRootObject3D`, using three's own `isObject3D` brand (survives a realm boundary that a
bundled `instanceof` does not), and `registerInstanceRoot` admits only what it can actually pose. The
debug log was removed.

Verified in isolation (`06-world3d-after.txt`): `Test Files 1 passed (1)`, `Tests 14 passed (14)` — the
file's 14th test was already passing, so this is 13 recovered plus 1 held.

### 2.2 `🔬️engine-contract` — `ReferenceError: panelTreePanelHost is not defined`

The helper was declared with `const` inside `describe("s workflow flow routing")` (line 7447) and called
from `describe("registry-derived utilities and activation (P5)")` (line 8419) — different describe
callbacks, so the name is simply not in scope. **Fix:** hoisted the single declaration to module scope
beside `noopAction`, with a docstring, and deleted the block-scoped copy. No assertion changed.

### 2.3 `🔬️engine-contract` — `transformMode` expected `"move"`, received `"transform"`

`leftoverWorldGumballPoseV1` (`🧱️elements/🛠️ShellHelpers/🟦️.tsx:483`) returns `"transform"` for an armed
gumball. It returned `"move"` until commit `b2064cc237` (2026-09-13 03:15:38), which introduced the
unified transform mode — `isWorldTransformGumballMode` and `gumballKindForTransformMode` both accept
`"transform"` as the handle-driven mode. The fixture
`🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json` last moved at `8add1df147`
(2026-09-12 21:17:59) and still declared `"move"` — stale by ~6 hours relative to its producer.
**Fix forward:** the fixture now states `"transform"`. The producer was not touched.

### 2.4 `📨️browser-frame-transport` — stale source-text assertion

The test asserted the frame worker's source contains the literal
`message.probe === "structure" ? bindings.dumpStructure : bindings.dumpFrameStats`. The worker
(`🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:319`) now dispatches three probes — a third,
`"accessibility" → bindings.dumpAccessibility`, was added between them by the wgpu status-a11y lane.
**Fix forward:** the assertion now checks the three probe→binding mappings independently, so a fourth
probe does not break it on formatting alone.

### 2.5 `📌️view-state-carriage` — `expected undefined to deeply equal []`

The test called `buildSpacePanelState([], [])` and read `empty.programs`. Commit `b2064cc237` removed the
program roster from the carriage entirely — that IS the law the test asserts — changing the signature to
`buildSpacePanelState(spawnedApps, activePanelTab, activeSpawnedId?)` and making `isSpacePanelState`
refuse any key outside `activePanelTab | spawnedApps | activeSpawnedId`. The test still spoke the
pre-fix API, so `empty.programs` was `undefined`.

**Fix forward, schema-first:** the fixture `🧫️fixtures/📌️panel-carriage/🔣️.json` now declares
`activePanelTab` and `carriedFields` (replacing the now-meaningless `programsAreEmpty: true`), and the
test asserts the carriage's key set equals exactly those fields AND that `isSpacePanelState` rejects a
panel carrying `programs`. The law is now enforced structurally rather than by an empty array.

### 2.6 `🔬️engine-contract` — `packValueFromBase64: expected pk: prefix`

The same commit `b2064cc237` switched `panelJsonFromState` from `packValueToBase64` to strict
`JSON.stringify`. The spawned-focus test still decoded the result with `packValueFromBase64`.
**Fix forward:** it now decodes with the carriage's own declared reader, `parsePanelState` — the one
reader the module publishes — and the test's `panel` literal no longer carries the removed `programs`
field.

### 2.7 `🔌️PluginRuntime` — retained render patch

`expected undefined to match object { surface: "window", revision: 1, root: 0 }`. Commit `b2064cc237`
made `applyRetainedWindowPatches` refuse a patch that names no surface
(`[DEBUG] … published a patch naming no surface body — refused`) — the same law the sibling test
"acknowledges only patches that identify the exact retained surface" asserts — and keys retained
surfaces by `retainedSurfaceId(instance, body)` = `"<instance>:<body>"`. The retention test still fed an
unaddressed patch and looked the surface up under the bare key `"window"`.
**Fix forward:** the test now sends `surface: pluginSurfaceRef(1, "window")` and looks up
`retainedSurfaceId(1, "window")`. The stricter host rule is untouched.

## 3. `verify publication-retirement-authority` — GREEN

**Before:** the audit recorded `error[E0004]: non-exhaustive patterns:
&…WindowMeasure::Number { .. } and &…WindowMeasure::Progress { .. } not covered` at
`🧰️framework/…/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs:85` — a compile failure, so the gate never
reached a test.

**Root cause.** `WindowMeasure` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1064`)
declares exactly six variants — `Select`, `Slider`, `Number`, `Progress`, `Toggle`, `Group`. `Number`
and `Progress` were added by `d8dce87ca0` (2026-09-13 14:41:31); the consumer's `walk` last moved at
`521b618cee` (2026-09-12 10:48:12) and still matched only three leaf variants plus `Group`.

**Fix forward** (`🔬️world3d-host-unit/🦀️.rs:94`): the leaf arm now reads
`WindowMeasure::Toggle { id, .. } | … Slider | … Select | … Number { id, .. } | … Progress { id, .. }`,
so every leaf contributes its id to the addressability assertion the test makes. `Group` keeps its own
arm. The enum was not touched.

I also swept for any OTHER exhaustive `match` over `WindowMeasure` that the two new variants would
break: `grep -a` for `WindowMeasure::Toggle {` in a match-arm position across all `*.rs` outside the
ticket archive returns construction sites and `matches!` guards only — this was the single exhaustive
match in the tree, so no second `E0004` was expected, and none appeared.

**After** (`🗑️generated/gates-green/12-pubret.txt`, 17:32:01 → 17:45:00, 12m59s):

```
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 0.06s
exit=0
```

## 4. `--test example-geometry` — 15/17, and NOT the failure the audit saw

Command: `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features
component-app-assembly --test example-geometry`.

**Result** (`🗑️generated/gates-green/14-example-geometry.txt`, 17:48:00 → 17:48:48):
`15 passed; 2 failed`, **finished in 27.64s** — against the audit's 5m07s wall for the same target, so
the kernel-performance lane's exact-predicate rewrite has clearly landed and is working. The kernel was
not touched by this lane.

Neither remaining failure is the audit's `tessellate_geometry took 2308598 us` trip; that one is gone.

| failing test | message | reading |
|---|---|---|
| `sphere_cut_with_torus_evaluates_to_the_difference_volume` | `FlowHost::evaluate took 1798354 us (best of 3) against a 1545000 us ceiling — the op chain regressed algorithmically, see 📓️kernel-performance-2026-09-13.md` | 16 % over a ceiling the kernel lane itself set today (`maxEvaluateMicros: 1545000` in that example's fixture), measured while 8 other cargo runs shared the machine. Their own report says these ceilings are "meant to convict an algorithmic regression… never to police machine-to-machine variance", so this may be headroom rather than a regression — **their call, not mine.** |
| `delivery_box_fillet_preview` | `box-fillet-preview: the preview cost 3 tessellate round trips, budget 2 — one round trip is one whole flowEvalTick` | Deterministic, not timing — a real delivery regression against `maxRoundTrips: 2`. `🧵️preview-eval/🦀️.rs` was **uncommitted and modified at 17:45:52** (+25/−7), i.e. mid-edit two minutes before this run. Attribution: the live preview/generate-mode lane. |

## 5. generation3d `--lib --features component-app-assembly` — 413/431, and the suite's hang located

### 5.1 The suite deadlocks; one test is responsible

The audit could not measure this suite in an hour and attributed it to CPU starvation. With the machine
quiet (`rustc=0`, 3 cargo processes) the run **still made zero forward progress**: byte-identical output
across a 60-second window, the test binary at **0.0 % CPU** — blocked, not slow — with exactly ten tests
reported `has been running for over 60 seconds`, all in
`editor::generation3d::commands::flow_eval_tick` / `flow_tessellate_resolve` /
`flow_tessellate_cancel_resolve` (`🗑️generated/gates-green/16-lib-featured-hung.txt`).

Those ten share the module's serial `context::lock()`. Re-running with a single test skipped —
`--skip an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain` — made **all ten of them
pass** and the suite complete in 237 s. So one test acquires the serial lock and never releases it, and
every later test in those modules queues behind it forever. That is the whole reason this gate has been
unmeasurable across two sessions.

**This is a finding, not a fix.** The skip is how the counts below were obtained; nothing was changed to
work around it, and the hanging test is left exactly as it is for its owning lane.

### 5.2 Counts

`🗑️generated/gates-green/18-lib-featured-skip.txt`, 18:45:41 → 18:49:42:

```
running 431 tests
test result: FAILED. 413 passed; 18 failed; 0 ignored; 0 measured; 1 filtered out; finished in 237.11s
```

Per the brief these 18 are reported, not fixed — the disposer-teardown-ownership lane owns the teardown
family. Grouped by panic message:

| count | message | family |
|---:|---|---|
| 5 | `registered fixture typed operation did not retire within 30 seconds` | typed-operation retirement |
| 2 | `Generation3d app fixture did not reach its terminal-empty close witness, last pending close authority: Generation3d evaluation session awaits its exact close grant` | terminal-empty close |
| 1 | `P3 authority refresh immediately before production maintenance: "generation3d-publication.contended"` | publication retirement |
| 1 | `the law leaves the process-wide registry as it found it: "flow.registry-retirement-full"` | registry retirement |
| 2 | `the default fixture must evaluate and tessellate at least one preview mesh` / `the first render of a fresh document must actually tessellate` | viewer preview render |
| 2 | `assertion failed: !instances.is_empty()` / `at least one instance` | viewer preview instances |
| 1 | `wireframe must drop the shaded triangle channels` | viewer show-mode |
| 2 | `nodeGraphViewport requires viewport` / `nodeGraphViewport decodes from its own action id` | action bridging |
| 1 | `remote snapshot merge is fail-closed until the app-owned streaming envelope decoder…` | vcs merge |
| 1 | (rotate/scale/translate selection transform neurons) | selection transform |

The five viewer-preview and show-mode failures sit on the same `🧵️preview-eval` / `👁️viewer` files that
were being edited live during this window (§6), so they are plausibly the same in-flight work as §4's
round-trip regression rather than settled defects. I did not trace them individually — outside the brief.

## 6. Peers editing the same files during the runs

Two compile failures interrupted these measurements, both from one in-flight peer change: the viewer
config leaf `active_example_id` migrating `String` → `Option<String>` across its whole schema fan-out
(`🧬️schema/{📜️.wit,🔗️.graphql,🔣️.json,🛰️.proto,🟦️.ts,🦀️.rs}`, all uncommitted).

1. **17:45–17:47**, blocking gate 4: `E0308` in `👁️viewer/🦀️.rs` and
   `👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs`. Both files were rewritten by their owner while my
   compile was still running (mtimes 17:47:12 and 17:47:27, after my 17:45:17 start). I changed nothing
   and simply re-ran — it compiled.
2. **17:49–17:54**, blocking gate 5: `E0277` (`Option<String>: From<&str>` not satisfied) in
   `👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs:32`, a file untouched since
   2026-09-12 11:47 and clean in `git status` — a stale consumer the migration had not reached. Per the
   brief I fixed it forward minimally (`active_example_id: Some(example_id.into())`). **That edit was
   then superseded**: the owning lane rewrote the whole file minutes later, changing the helper's
   signature to `viewed_node_ids(example_id: Option<&str>)` with `example_id.map(str::to_string)` and
   adding the `None` = never-picked vs `Some("")` = the picker's `No example` row distinction. Their
   version is the better one and is what stands in the tree; **no net change from me remains in that
   file**, and I did not revert any of their work.

Separately, the 17:11 attempt at gate 3 was lost to a repo-wide cargo lock wedge (35 `cargo` processes,
0 `rustc`, 0 test binaries, oldest stuck 2 h 59 m; evidence in
`🗑️generated/gates-green/11-cargo-lock-wedge.txt`). The coordinator cleared it at 17:42 — they were
orphans of lanes killed by the 15:00 rate limit. I withdrew only my own queued invocation and signalled
no peer process. Gates 3–5 above were all run after that.

## Attribution summary

Every red this lane fixed was a CONSUMER left behind by a landed peer change; none was caused by this
lane, and no peer hunk was reverted.

| red | landed by | when |
|---|---|---|
| JS fixture ENOENT ×8 | `8add1df147` fixtures/tests split (Rust side updated, TS reader not) | 2026-09-12 21:17:59 |
| World3dHost gumball crash ×13 | `d8dce87ca0` gumball live-preview lane | 2026-09-13 14:41:31 |
| `transformMode` fixture drift | `b2064cc237` unified transform mode | 2026-09-13 03:15:38 |
| panel carriage signature / encoding ×3 | `b2064cc237` roster-free, JSON-encoded carriage | 2026-09-13 03:15:38 |
| retained patch must name a surface | `b2064cc237` stricter retention rule | 2026-09-13 03:15:38 |
| `WindowMeasure` `E0004` | `d8dce87ca0` added `Number`/`Progress` variants | 2026-09-13 14:41:31 |
| frame-worker probe assertion | wgpu status-a11y lane, third `accessibility` probe | — |
| `panelTreePanelHost` scope | pre-existing cross-`describe` reference in the test file | — |
| viewer `E0308` ×2 / `E0277` ×1 (blocked gates 4 and 5) | live, uncommitted `active_example_id: String → Option<String>` schema migration | 2026-09-13 17:45–17:54, mid-run |

Gates 4 and 5's own failures are NOT this lane's and were not fixed — see §4, §5 and the closing list.

## Files

Changed by this lane:

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🟦️.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` (docstring only)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🪪️world-surface-identity.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧫️fixtures/📌️panel-carriage/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📌️view-state-carriage/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs`

Touched, then superseded by its owning lane (no net change from me remains):

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs` — see §6.2

Raw logs written by this lane (`🗑️generated/gates-green/`): `01`–`08` (gates 1–2), `09`+`11`–`12`
(gate 3 and the lock wedge), `10` (typecheck), `13`–`14` (gate 4), `15`–`18` (gate 5, including
`16-lib-featured-hung.txt`, the deadlock evidence).

## What is NOT claimed

- **Gates 4 and 5 are RED and this lane did not fix them** — the brief scoped both to re-measurement.
  Gate 4's two failures belong to the kernel-performance lane (its own evaluate ceiling) and to the live
  preview/generate-mode lane (the round-trip budget). Gate 5's 18 belong to the disposer-teardown
  family and to preview/viewer files that were being edited during the run.
- **Gate 5's 431-test count was obtained with one test skipped.** `413 passed; 18 failed; 1 filtered
  out` is the run with `--skip an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain`.
  Without that skip the suite never terminates, so there is no unskipped reading — the skipped test's
  own verdict is unknown, and any count for the full 432 would be a guess.
- **The deadlock was located, not fixed.** Evidence: ten tests blocked at 0.0 % CPU on the module's
  serial lock with the machine idle, all ten passing once that one test is skipped. I did not read that
  test's body or attempt a repair — it belongs to the flow-tick/tessellate owner.
- **Gate 4's evaluate-ceiling failure is not adjudicated.** 1798354 us against a 1545000 us ceiling is
  16 % over, measured with 8 other cargo runs sharing the machine, on a ceiling its own lane set today
  and documented as a regression detector rather than a variance policeman. Whether that is a real
  regression or insufficient headroom is the kernel lane's call, not mine; I neither raised the ceiling
  nor touched the kernel.
- **My §6.2 one-line fix no longer exists in the tree** — the owning lane rewrote that whole test file
  with a better `Option<&str>` signature while my build ran. Their version stands; I reverted nothing.
- **The repo-wide typecheck gate is red and was not fixed** — 789 `error TS` across the tree
  (`🗑️generated/gates-green/10-react-typecheck.txt`). It is not one of this lane's five gates. What IS
  claimed: none of those errors is at a line this lane wrote — the four in `World3dHost/🟦️.tsx` are at
  1473/5157/6878/6883, none in the new predicate or in `registerInstanceRoot`, and the edited test
  files and fixtures contribute none.
- **`dependencies literal-external` and `procedural-plugin:test`** were outside this lane's brief and
  were not run.
- **No runtime browser verification** was done on 6018. The World3dHost gumball fix is proven by the
  jsdom suite (13 recovered tests) and by a `[DEBUG] ` log that printed the actual offending value
  (`HTMLUnknownElement`); it is NOT claimed to have been observed in the served app. In the served app
  the ref is a real `Group`, so the predicate admits it and behaviour there is unchanged by
  construction — but that was not watched in a browser.
- **The cargo lock wedge was diagnosed here, and cleared by the coordinator, not by me.** I signalled no
  peer process; I killed only my own two invocations (the queued one at 17:29, the deadlocked one at
  18:42).
- No modifying git command was run. No dev server was started, stopped or restaged. Nothing under
  `🗑️generated/` that this lane did not create was touched.
