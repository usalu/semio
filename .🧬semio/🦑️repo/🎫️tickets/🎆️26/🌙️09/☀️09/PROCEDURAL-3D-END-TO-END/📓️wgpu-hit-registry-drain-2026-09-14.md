# The Retained Hit Registry Is Now a Retained Authority (lane `wgpu-hit-registry-drain`)

`📓️wgpu-end-to-end-verification-2026-09-14.md` §D left the biggest open wgpu defect: **a real click on a
retained row missed roughly half the time**, because `InputState`'s pointer registry was ONE vector that
the frame build retired entry by entry while `InputState::hit_at` scanned it. This lane makes the
registry a retained authority — double-buffered and generation-stamped — with a law over the shared
fixture, and proves it on 6118.

---

## 1. TL;DR

* **Root cause, one sentence.** `FrameBuildPhase::InputFrame` retired `InputState::hit_targets` one entry
  per frame-build boundary step and the chrome walk re-minted the SAME vector at the end of the same
  build, while `InputState::hit_at` resolved against it — so any pointer event arriving inside a build
  hit-tested a partially or fully emptied registry.
* **Fix, at the owning layer.** `ui_wgpu::wgpu::HitRegistry` — two buffers plus a generation stamp.
  `resolved` is the last COMPLETE frame's registry and the only one `hit_at` ever scans; `staging` is the
  one the build retires and re-mints. They swap when the chrome walk runs to the end. Retirement stays
  bounded at one entry per step. The shell's owner map (`retained_hit_windows`) is double-buffered and
  published in the SAME step, so an id the pointer resolves is always one the map can name.
* **Law, over the shared fixture.** `frameCycle` in `🎯️retained-hit-targets/🔣️.json`, twinned in Rust and
  TypeScript: 100 consecutive presses, each replayed at **twelve** instants of a frame build.
  **Rust 1 200 presses resolved, 0 missed**; the single-buffer counter-model in the same test
  **0 resolved, 500 missed**. TypeScript: 12/12 suite green, same counts re-derived independently.
* **Runtime, on 6118, WITHOUT arming.** 50 consecutive clicks on `Add Generation` → **50/50** press
  resolved, **50/50** release resolved, **50/50** dispatched `addGeneration`. 20 navbar-control clicks →
  **20/20** press and release resolved. **0 pointer events met `targets=0`** in 140 button events.
* **The counterfactual, measured on this build.** The trace now prints the staging census too.
  **116 of 140** pointer-button events landed while the build had already retired part of the previous
  registry, and **100 of them landed at `staged=0`** — the buffer the old code resolved against, empty.
  Those 100 are exactly lane A's 50 presses and 50 releases.

---

## 2. Root cause, with file:line

Three files owned the one-buffer design.

**`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs`** — one vector, written by the walk and read
by the pointer:

```rust
pub hit_targets: Vec<HitTarget<E>>,                                  // :215 (before)

pub fn hit_at(&self, x: f32, y: f32) -> Option<&HitTarget<E>> {      // :263 (before)
    self.hit_targets.iter().rev().find(|target| target.rect.contains(x, y))
}
```

**`…/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13288`** (before) — the bounded retirement,
one entry per boundary step, on that same vector:

```rust
FrameBuildPhase::InputFrame => {
    if self.input.hit_targets.pop().is_some() {
        return FrameBuildBoundaryStep::Pending;
    }
```

**`…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12214`** (before) — the chrome walk refilled it at
`FrameSetup`, and dropped the owner map with it:

```rust
// 🎯️ One chrome walk mints one pointer registry. `FrameBuildPhase::InputFrame`
// has already drained `InputState::hit_targets` for this build, so the owner
// map is dropped with them …
self.retained_hit_windows.clear();
```

So the window between `InputFrame`'s first `pop()` and the chrome walk's last `register_hit` is a window
in which the registry is incomplete — and, for the whole span between the last `pop()` and the walk
reaching a given body, EMPTY. A frame build is thousands of boundary steps under load, which is why §D
measured a move and a press 295 ms apart landing on opposite sides of it, and why a previous lane's move
of the dispatch from press to release (`🐚️Shell/…/🦀️.rs`, `📓️wgpu-retained-controls-wires-2026-09-13.md`
§7) did not save it: both the down and the up landed inside the same drained window.

**It was never a race with the drain alone — it is that the pointer and the build shared one buffer.**

---

## 3. The fix

### 3.1 `HitRegistry` — the retained authority

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:215-256`

```rust
pub struct HitRegistry<E> {
    staging: Vec<HitTarget<E>>,
    resolved: Vec<HitTarget<E>>,
    generation: u64,
}
```

* `register` (`:228`) pushes into `staging` — never resolvable.
* `resolve` (`:236`) scans `resolved` in reverse — the last COMPLETE frame, nothing else.
* `retire_step` (`:246`) pops exactly ONE entry of `staging`. The bounded discipline is unchanged.
* `publish` (`:240`) swaps the two and bumps the stamp. The outgoing registry becomes the next build's
  retirement work, so a completed frame never frees a whole registry in one step.
* `close_step` (`:250`) drains both, so terminal close is still reachable.

`InputState`'s surface (`:318-348`): `publish_hits`, `retire_hit_step`, `hits()`, `staged_hits()`,
`hit_generation()`, and `hit_at` unchanged in signature. `InputState::clear_frame` is **deleted** — it was
a second, unbounded way to empty the registry and had no caller anywhere in the repo.

### 3.2 The frame loop retires staging, never resolved

`…/🧊️renderer/🦀️.rs:13327-13334`

```rust
FrameBuildPhase::InputFrame => {
    // ♻️ Retires the registry the PREVIOUS build left staged, one entry per boundary
    // step. The pointer's own authority is the last COMPLETE frame's buffer, which this
    // never touches — see `ui_wgpu::wgpu::HitRegistry`.
    if self.input.retire_hit_step() {
        return FrameBuildBoundaryStep::Pending;
    }
```

### 3.3 One publish, for the registry AND its owner map

`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2685, 7183, 7191, 12265, 12458`

The shell's `retained_hit_windows` (control id → the body that minted it, and that body's rect) is the
second half of a pointer resolution: without it a resolved id cannot be routed into a retained tree. It
was cleared at chrome `FrameSetup` and rebuilt during the walk — the same defect in a second place. It is
now double-buffered against the registry and published in the same step:

```rust
fn publish_retained_hit_registry(&mut self, input: &mut InputState<ActionDescriptor>) {
    std::mem::swap(&mut self.retained_hit_windows, &mut self.retained_hit_windows_staging);
    input.publish_hits();
}
```

called from `ShellChromeFramePhase::PersistPreferences` (`:12458`) — the ONE point at which a chrome walk
has actually run to the end. The two early exits that abandon a walk (`:12274`, `:12283`, when a bounded
collection cannot begin a new generation) publish **nothing**, so an abandoned build leaves the last
complete registry resolvable. That is the whole point of the double buffer.

### 3.4 The traces now carry the census

`…/🐚️Shell/…/🦀️.rs:6819` and `…/🧊️renderer/🦀️.rs:13903` print `targets=` (resolvable), `staged=` (what the
build in progress holds) and `gen=` (published generations). `staged=` is precisely what the old single
vector held at that instant, which is what makes §5.3's counterfactual a measurement rather than a guess.

---

## 4. The law, over the shared fixture

Oracle: `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` — a new `rules.frameBuffering`
clause and a new `frameCycle` block (`presses: 100`, `retireStepsPerFrame: 1`,
`phases: ["retire","mint","publish"]`, and the expectations
`doubleBufferedResolved: 100`, `doubleBufferedMissed: 0`, `singleBufferedMissedAtLeast: 1`).

**Rust** — `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs`,
`a_frame_build_never_empties_what_the_pointer_reads`. It drives the REAL `InputState` through the real
production cycle: prime two complete builds, then 100 frames of *retire one entry per step → re-mint one
entry at a time → publish*, replaying a press at **every** instant — before the first retirement, after
each single-entry retirement, after each minted entry, and after the publish. Every press must resolve
the same row and the same action the preceding move resolved. It also asserts that an unpublished
registry resolves nothing, that the resolvable registry never changes size during a build, that
retirement drops exactly one entry per step, and that the generation advances exactly once per completed
build.

```
test wgpu::input::retained_hit_target_tests::a_frame_build_never_empties_what_the_pointer_reads ... ok
test wgpu::input::retained_hit_target_tests::the_defect_point_no_longer_answers_the_window ... ok
test wgpu::input::retained_hit_target_tests::every_fixture_case_registers_resolves_and_orders_on_the_live_registry ... ok
test result: ok. 3 passed; 0 failed; 416 filtered out

[DEBUG] retained-hit-targets frame cycle: 100 presses over 5 entries —
        double-buffered resolved 1200 missed 0; single-buffered resolved 0 missed 500
```

**1 200 presses, 0 missed.** The counter-model in the same test — one `Vec`, drained by the build and
scanned by the pointer, i.e. the exact shape the renderer had — **misses 500 of 500**. The law
discriminates; it is not decorative.

**TypeScript twin** — `…/📺️renderer/🧑‍🎨engine/🧪️tests/🎯️retained-hit-targets/🟦️.ts`,
`resolves every press of every frame build, and the single-buffer model does not`. A second, independent
implementation of the same double-buffer over the same fixture, driving the same 100-press cycle and the
same counter-model.

```
✓ 🎯️ retained hit targets > resolves every press of every frame build, and the single-buffer model does not  6ms
Test Files  1 passed (1)     Tests  12 passed (12)
```

### 4.1 The red, recorded

The law was run against the single-buffer read before the fix was in place (`HitRegistry::resolve`
scanning `staging` — today's production shape under the new names):

```
🗑️generated/wgpu-hit-drain/ui-law-red.txt
  panicked at …🎯️retained-hit-targets/🦀️.rs:381: a registry that was never published must resolve nothing
🗑️generated/wgpu-hit-drain/ui-law-red-drain.txt   (that first assert skipped, to reach the drain itself)
  panicked at …🎯️retained-hit-targets/🦀️.rs:383: the move must resolve the row
test result: FAILED. 0 passed; 1 failed
```

`the move must resolve the row` failing on an unmoved pointer over a painted row IS §D's
`targets=0 hit=None`.

---

## 5. Runtime proof on 6118

Build: `@semio-tech/framework-renderer-wgpu:wasm`, `Successfully ran target`, dist republished
`10:02`, `dist/wasm-dev/semio-framework-os-renderer-wgpu_bg.wasm` carrying the new `staged=` trace
(verified by `strings`). Probe: `🐍️wgpu-hit-drain-probe.mjs` (new, this lane), evidence in
`🗑️generated/wgpu-hit-drain/full2/`.

**The probe does NOT arm the hit.** `📓️wgpu-end-to-end-verification-2026-09-14.md` §4.3 had every
generate-mode probe jiggle 0.25 px until the shell's own trace answered the row before clicking, because
without it the click was lost. This probe clicks after one settling move and nothing else — a user cannot
jiggle, so neither does the probe.

### 5.1 Lane A — 50 consecutive clicks on `Add Generation` (`?mode=generate`)

| measure | value |
|---|---|
| clicks | **50** |
| press (`down=true`) resolved the row | **50 / 50** |
| release (`down=false`) resolved the row | **50 / 50** |
| dispatched `addGeneration` | **50 / 50** |
| misses | **0** |
| pointer events that met `targets=0` | **0** |
| pointer events that resolved nothing | **0** |
| resolvable registry size, every event | `targets=42` |

The row's page point is re-derived from `dumpStructure` + the dock plan before every click, because each
dispatch inserts a roster row above it and the row slides down — a moving target is not a drained
registry, and scoring it as one would be a lie.

A representative pair, pointer never moved between them:

```
83245 [DEBUG] wgpu-shell pointer button x=160.696 y=138 down=true  targets=42 staged=42 gen=286
        hit=Some((TreeItem, Some("tree.label.procedural3d-play-generate.add-generation"), Some("addGeneration")))
83245 [DEBUG] wgpu-shell retained press window=generation3d-generations kind=TreeItem down=true  action=Some("addGeneration")
83275 [DEBUG] wgpu-shell pointer button x=160.696 y=138 down=false targets=42 staged=42 gen=286
        hit=Some((TreeItem, Some("tree.label.procedural3d-play-generate.add-generation"), Some("addGeneration")))
83275 [DEBUG] wgpu-shell retained press window=generation3d-generations kind=TreeItem down=false action=Some("addGeneration")
```

Compare §D's own 6118 measurement on the pre-fix build, at the same point, same probe family:

```
64741  os_host pointer hit       x=160.696 y=138 targets=42 hit=Some((TreeItem, "…add-generation"))
65036  wgpu-shell pointer button x=160.696 y=138 down=true  targets=0 hit=None
65109  wgpu-shell pointer button x=160.696 y=138 down=false targets=0 hit=None
```

### 5.2 Lane B — 20 navbar-control clicks

| measure | value |
|---|---|
| controls discovered by a sweep of the chrome band | 19 points, **12 distinct** |
| clicks | **20** |
| press AND release resolved the control the preceding move resolved | **20 / 20** |
| misses | **0** |
| pointer events that met `targets=0` | **0** |
| pointer events that resolved nothing | **0** |

Controls exercised: `playground.navbar.fixture`, `playground.navbar.modes.generate`,
`playground.navbar.roles.{editor,viewer}`, `ui.panelToggle.{settings,details,workbench,display,chat}`,
`ui.fullscreen.toggle`, `dock.tab..procedural-view-preview`, `dock.tab..procedural-view-preview.close`.

The predicate is §D's law itself — *a press resolves the same target the preceding move did* — so each
click's expectation is re-read from the shell's live trace immediately before the click. That matters:
clicking `playground.navbar.roles.viewer` replans the entire dock, and a coordinate taken from a sweep
several replans earlier then names a control that genuinely no longer exists there. The first run of this
lane scored those as misses (9/20); they were stale coordinates, not drained registries — the trace
showed `targets=27` with `hit=None`, a FULL registry answering a point nothing paints. The rerun rereads
the target and answers **20/20**. Both runs are kept: `🗑️generated/wgpu-hit-drain/full/lane-b.json`
(stale-coordinate run) and `…/full2/lane-b.json`.

The gate is resolution, not dispatch: a navbar `Select` carries no `ActionDescriptor` at all
(`hit=Some((NavbarItem, Some("playground.navbar.fixture"), None))`) — the shell opens its dropdown
internally and logs no dispatch line — so demanding a dispatch witness from every chrome control would
fail on controls that never had one. `withEffects` (a new dock plan / render / command following the
click) is reported beside it: **2 of 20**.

### 5.3 The counterfactual, measured — how wide the window really was

`staged=` is exactly what the old single vector held at that instant: the staging buffer goes through the
same retire-then-refill sequence, at the same boundary steps; the only change is that `hit_at` no longer
reads it. Over the full2 run:

```
pointer-button events = 140    mid-build (staged < targets) = 116    targets == 0 -> 0    hit=None -> 0
   targets=42 staged=0   x100   (all 42 entries already retired when the pointer event landed)
   targets=26 staged=0   x2
   targets=27 staged=3   x4     (24 of 27 retired)
   targets=67 staged=25  x2     (42 of 67 retired)
   targets=35 staged=27/28/30   x6
   targets=66 staged=58  x2

move events = 694    mid-build = 355 (51%)    targets == 0 -> 3 (boot, before the first walk ever published)
   targets=42 staged=0 x240     targets=59 staged=0 x33     targets=59 staged=1 x16   …
```

**100 of lane A's 100 press/release events landed at `staged=0`.** Under the single buffer every one of
them would have scanned an empty vector and answered `hit=None` — §D's trace, 100 times. Half of all
pointer MOVES land mid-build too, which is why hover was flaky and why the navbar sweeps in
`📓️wgpu-end-to-end-verification-2026-09-14.md` §5.4 found a different set of ids on every run.

### 5.4 Battery

`cd T && bun 🐍️wgpu-battery.mjs --only=generate-add,generation-roster,chrome`

`cd T && bun 🐍️wgpu-battery.mjs --only=generate-add,generation-roster,chrome` — and then again with the
arming REMOVED from the probes (§5.5).

| run | probes | result |
|---|---|---|
| `battery-1.txt` 10:22, probes still arming | generate-add, generation-roster, chrome | generate-add **0/2**, generation-roster **5/5**, chrome **4/5** |
| `battery-2.txt` 10:35, probes still arming | generate-add | generate-add **2/2** |
| `battery-3.txt` 10:39, probes DE-ARMED | generate-add, generation-roster, deferred-commit, chrome | generate-add **2/2**, generation-roster **5/5**, deferred-commit **2/3**, chrome **4/5** |
| `battery-4.txt` 10:50, probes DE-ARMED | deferred-commit | deferred-commit **2/3** |

* **`generate-add` 2/2** — `{"verdict":"pass","dispatched":4,"seconds":66.76, after:[{instances:1,lines:1,stateMeshes:3}]}`
  for `hexagonal-mushroom-column` and `box-shell-preview`, **without the arming**. `battery-1`'s 0/2 was
  `blocked-no-row` with `targetsFound: 0` — the app never reached a live window, its console carrying
  `wgpu renderer fault: worker-present-failed: offscreen prepared frame admission: prepared render
  revision is stale: live=21, packet=13` at 9.7 s of boot, in BOTH examples. A peer lane rebuilt the
  renderer wasm at **10:34** and edited `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (10:28) and
  `🪟️winit-app/🦀️.rs` (10:24) DURING that battery; the identical run 7 minutes later was green. Recorded
  as peer churn, not carried as a result.
* **`generation-roster` 5/5** — every step, armed and de-armed: the row dispatches `addGeneration`, the
  roster grows, the Form binds 8 nodes, the preview reaches `instances=1 lines=1 state-meshes=3`, a
  second Add lands a second row and selecting it moves the editor.
* **`chrome` 4/5 — up from 2/5** (`📓️wgpu-end-to-end-verification-2026-09-14.md` §5.4). The navbar answers
  its own ids, the example picker is reachable, the mode AND role buttons are reachable, a `mod+alt+→`
  chord replans the dock. §5.4's complaint was that *which* ids a sweep found changed run to run, "because
  the sweep is hit-testing the same drained registry §D measures"; both runs here found the same full set.
  The one red step, `the world3d cancel control is declared`, is not a hit-registry matter at all:
  `surface_overlay_controls_for` (`🐚️Shell/…/🦀️.rs:10037`) only mints `shell.world3d.cancel::<surface>`
  while that surface's status reports `cancellable`, and the example settles before the World3d surface
  first attaches — the same open item as §5.5's `status:pill-while-computing`, owned by
  `wgpu-a11y-status-i18n-runtime`.
* **`deferred-commit` 2/3, twice.** Its two INPUT steps are green de-armed — `rename commits and the guest
  runs it`, and `a Form slider drag reaches the guest` with `armed: true`, `seatedPresses: 1`,
  `leftTheInputAuthority: 7`, `admittedByTheGuest: 7`, `guestRanTheCommand: 7`, `dispatchFailed: 0`. The
  red step is `the preview PIXELS move with the slider`, and it is red the same way in both runs, for
  BOTH the dragged sample and the probe's own IDLE CONTROL (`differing: 0, fraction: 0, sameLength: true`
  on each), with `censusChanged: false`. §7 says what that is and is not.

### 5.5 The arming workaround is gone

`📓️wgpu-end-to-end-verification-2026-09-14.md` §4.3 taught three probes to JIGGLE the pointer 0.25 px, up
to 60 times, until the shell's own trace answered the row, and only then click. That is a workaround for
§D, and leaving it in would hide the next regression of exactly this defect, so this lane removed it from
the three probes it did not find a peer editing:

| probe | before | after |
|---|---|---|
| `🐍️wgpu-add-generation-probe.mjs` | jiggle ×60 until the trace answers | ONE move, then up to 20 observing samples |
| `🐍️wgpu-generation-publication-probe.mjs` | `armHit`, jiggle ×60 | `settleHit`, one move + observation |
| `🐍️wgpu-deferred-commit-probe.mjs` | `armHit`, jiggle ×60 | one move + observation |

Each keeps the §D history in its docstring and points at this report, so the reason the arming existed is
not lost. `🐍️wgpu-world3d-gaps-probe.mjs` still arms: a peer lane was editing that file at 10:36:50 while
this lane worked, and a concurrent edit there would have collided. It is the one remaining caller of the
workaround.

---

## 6. Files

**Changed (the fix):**

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs` — `HitRegistry` (`:198-258`); `InputState.hits`
  (`:279`); `register_hit` / `publish_hits` / `retire_hit_step` / `hits` / `staged_hits` /
  `hit_generation` / `hit_at` (`:311-348`); `close_step` and `terminal_is_empty` over both buffers;
  `clear_frame` deleted.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` —
  `FrameBuildPhase::InputFrame` retires staging (`:13327`); `os_host pointer hit` trace carries the census
  (`:13903`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` —
  `retained_hit_windows_staging` (`:2685`, `:3484`); `register_retained_body_hits` writes staging
  (`:7183`); `publish_retained_hit_registry` (`:7191`); chrome `FrameSetup` clears staging (`:12265`);
  the walk publishes at `PersistPreferences` (`:12458`); `wgpu-shell pointer button` trace (`:6819`).

**Changed (the law):**

- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` — `rules.frameBuffering`, `frameCycle`.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs` — `fixture_registry`,
  `a_frame_build_never_empties_what_the_pointer_reads`; `load_registry` now publishes.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎯️retained-hit-targets/🟦️.ts` — the twin.

**Changed (call sites, `input.hit_targets.iter()` → `input.staged_hits().iter()` — a test asserting what a
walk just registered reads the staging buffer, which is what it actually means):**

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- `…/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs`, `…/🔬️wgpu-chrome-overlays-tour/🦀️.rs`, `…/📏️wgpu-window-measures/🦀️.rs`
- `…/🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs`, `…/🔬️wgpu-table/🦀️.rs`
- `…/🛰️Dock/🧪️tests/🔬️wgpu-unit/🦀️.rs`

**Added (this lane's probe):**

- `T/🐍️wgpu-hit-drain-probe.mjs`

**Evidence** (`T/🗑️generated/wgpu-hit-drain/`): `ui-law-1.txt` / `ui-law-2.txt` (green), `ui-law-red.txt` /
`ui-law-red-drain.txt` (the red), `ts-twin-2.txt`, `renderer-check.txt`, `wasm-3.txt`,
`full2/{results,lane-a,lane-b}.json` + `console.txt` + screenshots, `full/lane-b.json` (the
stale-coordinate lane-B run), `battery-1.txt`.

---

## 7. What is NOT claimed

* **No A/B on a rebuilt pre-fix renderer.** The before/after is (a) §D's own 6118 measurement on the
  pre-fix build, (b) §4.3's arming deltas on that build, and (c) §5.3's counterfactual from the `staged=`
  census this build prints. Deliberately publishing a broken renderer to the shared 6118 dist would have
  corrupted a peer lane's probe that was running at the time, so it was not done.
* **The guest did not materialise 50 generations.** All 50 clicks resolved and the shell dispatched
  `addGeneration` 50 times (`wgpu-shell retained press … down=false action=Some("addGeneration")` ×50,
  102 `tags=dispatchAction` effects), but `generation3d-generations` published **15** revisions
  (`rev=1…15`, `nodes=19`) over the 61 s the 50 clicks took. Whether the guest coalesces, saturates or
  caps is a guest-side question this lane did not open and does not answer.
* **Hover latency is unchanged, not improved.** `update_hover` ran in `FrameBuildPhase::Hover`, before the
  drain, so it already read a full registry; it now reads the last complete one. Same freshness.
* **One frame of staleness is now explicit.** Between a body's paint and the end of that build's chrome
  walk, the pointer resolves the PREVIOUS complete registry. That is the trade the fix makes on purpose —
  a complete registry one frame old beats a half-built one — and it is what the law pins.
* **The battery lanes still arm.** `🐍️wgpu-add-generation-probe.mjs` and
  `🐍️wgpu-generation-publication-probe.mjs` keep §4.3's arming; §5.4's battery therefore does not
  re-measure §D. `🐍️wgpu-hit-drain-probe.mjs` is the unarmed measurement.
* **Not touched:** §5.1 (wheel zoom in edit mode), §5.2 (`translateSelection` in the viewer), §5.3
  (`World3dSceneBridgeStep::Fault` on the 20th gesture), §5.5 (ARIA mirror, status pill, German). The
  `F`-chord fit camera of §5.6 was gated on a surface resolving under the pointer and should benefit, but
  this lane did not re-measure it.
* **Five `semio-framework-ui --lib` tests fail, and none of them are this lane's.**
  `cargo test -p semio-framework-ui --features wgpu-engine --lib` answers `414 passed; 5 failed`
  (`🗑️generated/wgpu-hit-drain/ui-all.txt`). The lead failure is
  `🧱️ guard 'ui::wgpu_engine': measured slot tables differ from the committed budget — left element_bytes
  159928, right 159896`, and the other four (`ArenaFull` ×2, `must fit fixed process permits`, a permit
  scalar) are sized off that committed budget. The 32 bytes are a peer's: an uncommitted working-tree
  change to `🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` (10:19) adds `pub tool_run_trace: Option<String>` and
  `pub lanes: Vec<SceneLaneRef>` to `Board2dScene`, which sits inside `UiNode` → `UiTree` → `UiWindow` →
  `UiSurfaceSlot`. `InputState` — the only struct this lane grew — is HOST-owned (`AppRuntime::input`,
  passed into the engine by reference) and appears nowhere in `UiWindow` or `UiSurfaceSlot`, so this
  lane's `HitRegistry` contributes zero bytes to that table. The committed budget in
  `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` belongs to the `toolRun*` lane to update.
  Every test in `wgpu::input::` — the module this lane changed — passes, including the new law.
* **`deferred-commit`'s pixel step is red and this lane did not cause it and does not fix it.** Two runs,
  identical: the slider drag seats and reaches the guest (`seatedPresses: 1`, `guestRanTheCommand: 7`,
  `dispatchFailed: 0`), so INPUT is delivered end to end — but the preview crop is byte-identical before
  and after (`differing: 0`), and so is the probe's own IDLE CONTROL, with `censusChanged: false`
  (`instances=1 lines=1 stateMeshes=3 stateDraws=1` on both sides). A pointer registry cannot make a
  delivered, guest-executed slider value fail to change pixels. `📓️…-verification-2026-09-14.md` §1 saw
  this step move 98 % of the preview's pixels at 08:39; between then and now peers rebuilt the renderer
  twice and changed `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`'s scene-lane merge (10:19). Flagged for the
  mesh/refresh lane (`wgpu-edit-convergence-perf`), not diagnosed here.
* **The `?role=viewer` boot axis was exercised only incidentally** — lane B clicked
  `playground.navbar.roles.viewer`, which replanned the dock into `procedural-view-preview`, and the
  clicks after it resolved fine. No dedicated viewer-lane pass was run.
* **`semio-framework-os-renderer-wgpu`'s test cfg does not compile in this tree, for peer reasons.**
  `cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown --lib --profile test`
  ends on 7 errors, 50 warnings (`🗑️generated/wgpu-hit-drain/renderer-tests-check.txt`): a missing
  `semio_framework_async::assert_fixed_slot_tables`, a missing `ProgramBridgeEntry::from_wasm`, a removed
  `ShellState::directory_home` / `DirectoryHomeProjection`, and an unscoped `SurfaceId` — all in test files
  this lane never touched, all peer API churn in flight. It reached crate-wide name resolution (50
  warnings prove the expansion ran), and NOT ONE error names `staged_hits`, `publish_hits`,
  `retire_hit_step` or any file this lane edited, so the six renderer test files it rewrote resolve. They
  were not RUN. The non-test `--lib` check of the same crate on the same target is clean, and the wasm
  build that the 6118 proof ran on succeeded.
