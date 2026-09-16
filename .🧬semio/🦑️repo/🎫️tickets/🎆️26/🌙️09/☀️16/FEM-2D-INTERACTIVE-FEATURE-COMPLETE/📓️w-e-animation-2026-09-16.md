# ⏯️ Slice E — animated deformation + results/playback panel (2026-09-16)

Owner: slice E. Crate `semio-s-artifact-fem-2d`, editor behind `--features component-app-assembly`.

## 1. Design

### 1.1 Playback lives in the results window config, never in the document

`Fem2dResultsWindowConfig` gains `animation: Fem2dResultsAnimation { phase: f64 (0..=1), playing: bool, speed: f64 (cycles/s), loop_mode: Fem2dLoopMode {Loop,PingPong,Once}, waveform: Fem2dWaveform {Ramp,Sine}, reverse: bool }`. `reverse` carries the ping-pong direction so `speed` never goes negative (the plan's open question, decided this way).

**The default phase is 1.0, not 0.0.** A results window that nobody has touched must draw the FULL deformed shape — exactly what it drew before playback existed. `Ramp` at phase 1 is amplitude 1, so every pre-existing render is bit-identical. Phase 0 would have opened every results window on an undeformed structure.

`amplitude()`: `Ramp → phase`, `Sine → sin(2π·phase)` (signed, so the structure swings both ways). ONE amplitude drives the deformed shape, the reaction read-outs, the moment diagram and the modal/buckling mode amplitude, so a frame is one consistent instant rather than a mix of poses.

### 1.2 The clock

No monotonic clock is readable from a wasm guest inside a command, so `resultAnimationTick` advances by the FIXED `ANIMATION_TICK_SECONDS` (33 ms) and the host's `Effect::DispatchAction { delay_ms: 33 }` is what keeps that delta honest (~30 fps).

Two invariants, each with a law:

- **One clock per window.** `setResultAnimation` arms a hop only on the TRANSITION `!playing → playing`. Moving the phase slider mid-playback, or pressing play twice, arms nothing — two chains would advance the same window by two frames each tick and it would never slow down again.
- **A stopped tick is inert.** A tick that finds `playing == false` writes NOTHING and re-arms NOTHING. That is how a pause, a closed window, or a second chain that raced the first dies out instead of spinning the guest forever.

`Once` parks at 1 and clears `playing` itself (the final tick owes no next frame); pressing play on a finished `Once` run rewinds to 0 (`Fem2dResultsAnimation::start`), so the play button is never a dead control. `PingPong` bounces off both ends by flipping `reverse`.

### 1.3 Window addressing

`Effect::DispatchAction` carries no window of its own; the React ShellHost redispatches it under the `resolvedTargetViewState` of the dispatch that emitted it (`🏛️ShellHost/🟦️.tsx` `scheduleDispatchAction`), so the chain keeps addressing the window the user pressed play in. `windowId` rides on the re-arm's args as the address it was armed for.

New helper `results::config::addressed_window_id(cfg, view)` decides which partition a command reads AND writes:

1. the captured `ConfigView::window` when it is a results window — `WindowConfigOwnerRegistry::capture` binds it to `view.window_id`, or for a PANEL projection (which carries no `window_id`) to `focused_window_id`, so the snapshot in hand already names the exact partition. **Reading one partition and writing another is the bug this closes** (the puzzle3d Settings-panel incident, wave B36);
2. otherwise the `ViewModel` roster, with the same stale/kind checks `addressed` already had.

A roster naming no results window REFUSES rather than clobbering a partition with defaults. Consequence, stated as an open issue in §6: the transport only acts while a results pane is focused.

`ResultAnimationTick` stays `{}` — carrying a `windowId` on the payload would buy nothing, because `ConfigView` can only ever hand a command the ONE captured partition; an address it cannot read is an address it must not write.

### 1.4 `{field, value}` — how a persistent control names what it changed

The host merges a control's own scalar under the single key `value` (`🛠️ShellHelpers/🟦️.tsx` `uiIntentPayload`), and a node has ONE binding — so a slider physically cannot say which field it just moved. Three commands therefore gained `field: Option<String>, value: Option<String>` (appended; both `None` in every pre-existing row, so the pinned pre-migration wire bytes are unchanged):

| command | fields it accepts |
|---|---|
| `setResultDisplay` | `sourceId`, `mode`, `modeIndex` |
| `setResultAnimation` | `phase`, `phaseStep` (relative, for the step buttons), `playing`, `speed`, `loopMode`, `waveform`, `reverse` |
| `setAnalysisSettings` | `modalCount`, `bucklingCount`, `deformationScale` |

`field` absent ⇒ the typed staged-form vocabulary, unchanged. Unknown field or unparseable value ⇒ a `Fault`, never a silent default.

`setResultAnimation` with NOTHING named at all (the bare `space` chord — none of its `ActionArgDef`s are `.required()`, so no staged form opens) toggles play/pause.

### 1.5 Results cache

Thread-local `RESULTS_CACHE`, keyed by `(app_instance_id, canonical_base_revision)` off `Option<AppRenderOperationContext>`. Holds the `HashMap<String, StaticResult>` from `fem2d_solve_all` plus up to `RESULTS_CACHE_MODES = 8` normalized mode shapes per `(source, count)`. Exactly ONE entry is ever resident: a different key drops the whole entry rather than growing a second one, so a long editing session cannot leak the guest heap one revision at a time. `operation == None` (every fixture render) bypasses the cache and solves into the caller's frame. Proven: thirty frames over one revision cost one solve.

## 2. Files

Owned and written:

- `…/🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}` — the `animation` block plus both enums, on all five surfaces.
- `…/📊️results/🎚️config/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json` — `animation` on all three valid rows, four new invalid kinds (`animation-null`, `animation-unknown`, `bad-loop-mode`, `bad-waveform`), two new patch ops.
- `…/📊️results/🎚️config/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts` — the new kinds; `exact()` now takes an explicit optional-key list instead of hard-coding `resultSourceId`.
- `…/📊️results/🎚️config/🦀️.rs` — `Default`, `amplitude`, `advanced`, `start`, the tick/speed/step constants, `addressed_window_id`, `addressed_to`.
- `…/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs` — four playback laws + the new invalid kinds.
- `…/📊️results/🦀️.rs` — `🔖️ResultsCache` region; `render`/`render_static`/`render_modal`/`render_buckling` take the animation and the cache key; `static_layers` extracted; `playback_caption_layer`. Slice A's `interaction` threading and `fem2d_structure_layers_with` were preserved verbatim.
- `…/📊️results/🧪️tests/🔬️unit/🦀️.rs` — appended a `🔖️Animation` region (5 laws).
- `…/🎮️commands/⏯️set-result-animation/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` (new folder).
- `…/🎮️commands/⏱️result-animation-tick/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` (new folder).
- `…/🎮️commands/👁️set-result-display/🦀️.rs` — `field`/`value`.
- `…/🎮️commands/🧮️set-analysis-settings/🦀️.rs` — `field`/`value`.
- `…/📌️panels/📊️results/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs` (new folder).

Shared files touched (minimal, re-read before each edit):

- `✏️editor/🦀️.rs` — **UNAVOIDABLE**: three bridge arms had to gain the two new payload fields or the crate would not compile. Nothing else in the file changed. See §3.
- `✏️editor/🗣️terminology/🦀️.rs` — two labels appended (`display`, `playback`), all four cells; both differ in German, so `FEM2D_LABELS_IDENTICAL_BY_DESIGN` needs no entry.
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🧪️tests/🪟️window-config-ownership/🦀️.rs`, `🎮️commands/🧮️set-analysis-settings/🧪️tests/…`, `🎮️commands/👁️set-result-display/🧪️tests/…` — struct literals extended with `field: None, value: None` (mechanical). The harness `dispatch`'s `kind` match already routed `SetResultAnimation`/`ResultAnimationTick` to the results window when I got there — left as found.

## 3. What the coordinator must add to `✏️editor/🦀️.rs`

**`command_from_action` arms — already applied by me** (the crate did not compile otherwise). Keep them exactly:

```rust
"setAnalysisSettings" => Ok(Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings {
    modal_count: number("modalCount").map(|value| value.max(0.0) as u32),
    buckling_count: number("bucklingCount").map(|value| value.max(0.0) as u32),
    deformation_scale: number("deformationScale"),
    field: text("field"),
    value: scalar_text("value"),
})),
"setResultDisplay" => Ok(Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay {
    source_id: text("sourceId").filter(|id| !id.is_empty()),
    mode: text("mode").unwrap_or_else(|| "static".into()),
    mode_index: number("modeIndex").map(|value| value.max(0.0) as u32).unwrap_or_default(),
    field: text("field"),
    value: scalar_text("value"),
})),
"setResultAnimation" => Ok(Fem2dCommand::SetResultAnimation(set_result_animation::SetResultAnimation {
    phase: number("phase"), playing: flag("playing"), speed: number("speed"),
    loop_mode: text("loopMode"), waveform: text("waveform"),
    field: text("field"), value: scalar_text("value"),
})),
```

**Still owed by the coordinator — `FEM2D_PUBLICATION_CONTRACTS`.** It still has 18 rows against 33 commands, so `retained_routes_cover_every_command_exactly_once` cannot pass. The two rows this slice needs:

```rust
ArtifactToolPublicationContract { tool_id: "setResultAnimation", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::HostOnly] },
ArtifactToolPublicationContract { tool_id: "resultAnimationTick", lanes: &[ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::HostOnly] },
```

`HostOnly` is not optional: both handlers emit `Effect::DispatchAction`, and `every_route_declares_the_lane_its_handler_emits` refuses an emitted lane that is not declared.

**Optional, recommended:** declare `field`/`value` on the three actions' `action_args` so the staged form can drive them too. Not required — the panel authors these args itself.

## 4. Verification

(filled in below after the runs; logs under `🗑️generated/`)

## 5. Test inventory

- config (`🎚️config/🧪️tests`): default-is-a-still-full-deformation, waveform→amplitude, loop-mode advance (Loop/Once/PingPong both ends), `start` rewinds a finished `Once`. Plus the four new neutral invalid kinds in the existing fixture law.
- `setResultAnimation`: arms exactly one clock, bare gesture toggles, `{field,value}` reaches every transport field incl. `phaseStep`, unknown field/value refused, re-arm shape.
- `resultAnimationTick`: advances + re-arms while playing, inert when stopped (and after a pause), stops itself at the end of a `Once` run.
- results window render: static scene follows the phase (sine peak ≠ zero ≠ trough), default draws the full deformation and no caption, running scene carries `phase 0.42`, mode shapes follow the phase, cache solves one revision once / evicts on a moved revision / never caches without an operation.
- results panel: every section+control present and bound to its owning command with the right `field`, every control tagged `windowId`, play↔pause label switch, source select offers cases AND combinations.
- TS: `testFem2dResultsWindowConfigContract` (Ajv 2020 + fast-json-patch) over the extended fixture.

## 6. Open issues

1. **The transport needs a focused results pane.** `ConfigView` can only hand a command the ONE captured partition, so a gesture dispatched while a MODEL pane is focused is refused (`fem.window.kind`) rather than writing defaults into a results window it cannot read. Pre-existing behaviour for `setResultDisplay`; now shared by the whole transport. A real fix needs a framework change (capture by `args.windowId`, or expose sibling window configs read-only) — out of this slice's scope.
2. **Speed/phase range constraints live in the neutral JSON schema and the TS parser but not in the Rust `FromValue`** (the derive has no range hook). The commands clamp on the way in, so an out-of-range value cannot be produced by the app; the fixture deliberately carries no out-of-range invalid case, because Rust would admit it and the "native admitted invalid neutral cases" assertion would fire.
3. **`delay_ms: 33` is clamped to ~1 tick/s by a hidden/unfocused renderer** (`🏛️ShellHost/🟦️.tsx`'s own note, and `📓️project-hidden-browser-pane-throttles-plugin-boot`). Playback in a hidden pane will crawl; it will not break.
4. The step buttons use the RELATIVE `phaseStep`, not an absolute phase computed at render time, so two fast clicks before the next render still advance twice.
