# F3 resolve wedge (status: root cause NOT yet proven, laws written but not green)

## Done
- `editor_app` compile fix: tests `🎮️commands/🔗️add-combination` and `🧊️add-solid` `🧪️tests/🔬️unit/🦀️.rs` imported `semio_s_artifact_fem_3d_editor::...::commands::{add_solid,add_combination}`; now `semio_s_artifact_fem_3d_editor_commands::...` (where the modules live).
- `editor_app_2` compile fix (other agent's new law): `remove-selection` test `ArtifactView::new(&*base, ..)` (base is `Retiring<Fem3dSnapshot>`). Also `add-combination/🦀️.rs:50` closure lifetime error (`single`) replaced by a fn.
- Results law harness (`🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️unit/🦀️.rs`): `pump_results` used `fixture_pending_effects`, whose context has no parent source supplier, which is why `superseded_solve...` failed at 15:43 with `original caller has no parent source supplier`. It now polls `PluginApp::pending_effects` under the app's own scope `app.1.cx()` with `fixture_renew_turn` before each poll (as `settle_registered_typed_operation` does).
- New laws (same file): `burst_of_edits_with_undo_redo_settles_3d`, `example_switch_during_a_running_solve_settles_on_the_new_example_3d`. `superseded_solve...` now also asserts the initial solve settles.

## Findings (native, debug build)
- Before the harness fix every poll returned `None` (round 0 and 25) so no `SpawnJob` was ever produced and the caption stayed `Analysis queued`: not a job wedge but the poll never advancing. After renewing the turn the result was not observed (builds were repeatedly invalidated by concurrent framework edits, see below).
- Each `render_body` round cost about 2.7-3 s in the debug build (render itself, 75 rounds = 224 s), before any solve. In the browser every refresh pays this, so the "25-33 s per edit" is plausibly (number of prepare/preflight/spawn refresh rounds) x (render cost), not solver time. Needs a profile of `render` (mesh/scene build per frame) in the wasm build.
- Execute disabled while a solve is pending: the shell disables Execute only when `unresolvedActionArgs` is non-empty (`🧰️framework/🔨️modules/🧩️action-argument-resolution/🟦️.ts`, used by `🎛️command-registry/🟦️.tsx:471`). It also counts an unavailable choice catalog as invalid (`invalidActionChoiceArgs`). Nodal Load, Member UDL, Area Load, Self Weight and Add Solid are exactly the commands with closed-choice args (node, element, load case, material). Hypothesis: the option catalogs are fetched through a guest call that waits on the in-flight job turn. Not yet verified or fixed.

## Not done
- Root cause of superseded solves not settling; cooperative cancellation inside the solver loop (House, example switch); Execute gating fix; law results (no green run). The build was invalidated repeatedly by other agents' edits to the framework (`native-owned`, `LazySlots`, dwg/binary crates) and by `CARGO_BUILD_BUILD_DIR` rule changes (full rebuilds of about 25 min each).
- Code reviewed with no defect found by reading: `reconcile`/`MountedJob::cancel`/`retire_one` in `🧵️session` do cancel the previous shell and emit `CancelJob`; shell capacity is 32, so superseded shells that are never retired would starve `allocate()` (return `Vec::new()` silently), a candidate for the wedge to check next.

## Scripts
`rv-law.sh`, `rv-retry.sh` (retries when other agents' crates break); outputs in `🗑️generated/rv/`.

## Update 2 (coordinator priorities)

### (1) Execute disabled: catalog hypothesis FALSIFIED by code
- The 3D args are plain required text/number args (`📋️manifest/🦀️.rs`: `ActionArgDef::text("nodeId"...).required()`, `materialId`, no `options`, no choice format). `unresolvedActionArgs` therefore depends only on required presence and never on a catalog, so no catalog waits on a job.
- The real gate is the staged-action dialog (`🧱️elements/🏛️ShellHost/📝️staging/session.ts`): Execute is `disabled={!state.ready}`, `ready = !executing && current() && lease.current() && session.canExecute()`. `lease.current()` (`🔌️PluginRuntime/🧺️handle-ui-intake/lease.ts`) is false while a surface patch is pending (`pending.get(instance).has(surfaceId)`), and `refresh()` re-queries the guest form (`source.query`) on every lease abort or document revision change. The form query is a guest turn on the same plugin instance as the solve job, so with a solve pending the query waits behind job and render turns (each about 3 s in debug, see above) and `pending`/`!ready` lasts as long as that queue. This explains 73 s, more than 90 s and "forever" (revision churn re-arms `refresh`). Not provable natively; needs a browser trace of `source.query` latency during a solve.
- Fix direction (not implemented): serve the staged form from a turn that is not queued behind job steps (priority lane for `action_prepare`/render), or stop re-querying on job-progress publications that do not change the document revision. No law possible at Rust level; a renderer vitest on `StagedActionSession` with a slow query is the right law and is not written.

### (2) Shell capacity 32 (law written, not yet run)
- `burst_of_edits_with_undo_redo_settles_3d` now issues 45 edits (each followed by 2 pump rounds, then undo/redo) and asserts the result settles with no driver left. Not run to completion: builds keep being invalidated.
- Code reading: a superseded shell is only freed by `retire_one` through maintenance (`retiring[]` marks it). `reconcile` returns `Vec::new()` silently on `allocate()` or `reserve_credit` failure, or when `retire_retained_results`/`retain_retiring` refuse. No fault is surfaced. I did not change it blind: marking the request faulted would permanently fail that revision on a transient shortage. Proposed fix once the law shows a drop: return an explicit `Fault` effect for overflow, and run `retire_one` in the same refresh turn that supersedes.

### (3) Renew-variant pump
- Implemented (see above); `superseded_solve...` run `x` is in progress at time of writing.

## Update 3 (final state of this session)
- Item (3): with the renew-variant pump, `pending_effects` STILL returned `None` for 40+ consecutive rounds (`Analysis queued`, 0 jobs, no `SpawnJob`); rounds also slowed from 3 s to about 18 s as other agents' builds loaded the machine. Not yet known which early return in `🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs:pending_effects` fires (`ensure_for_roster`, `grant`, or a non-default `preparation.progress` every poll). Next step: temporary `[DEBUG]` in that function (framework, other agents' area) or drive the poll the way the host does (fresh turn per poll, same original source parent), then see whether `reconcile` is ever reached. `reconcile` never printed in an earlier instrumented run, so the wedge in this harness is before job registration, i.e. in preparation/admission, not in cancellation.
- Pump now runs 600 rounds (was 4000); laws are in place but unverified: `superseded_solve...`, `burst_of_edits_with_undo_redo_settles_3d` (45 edits), `example_switch_during_a_running_solve_settles_on_the_new_example_3d`.
- No [DEBUG] prints remain in source. `C:\t\rv` (target) and `C:\v` (stale, rm blocked) remain.
