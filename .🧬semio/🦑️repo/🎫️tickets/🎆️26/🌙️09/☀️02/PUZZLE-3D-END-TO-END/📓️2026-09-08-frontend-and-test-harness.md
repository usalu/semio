# Puzzle 3D — Frontend Rendering & Test Harness Map (2026-09-08)

Scope: map the React (`SEMIO_RENDERER=react`, port 6013) and wgpu/wasm (`SEMIO_RENDERER=wgpu`, port
6113) rendering paths and the full test harness for puzzle3d, so a later session can drive a browser
and verify "every window works and every tool works." Read-only exploration; no builds run.

## TL;DR — selector/hook cheat sheet

**Two window panes** (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:31-32`):
- `#puzzle3d-main-top` — `<div data-slot="window" data-elevation-root="" id="puzzle3d-main-top">`
- `#puzzle3d-main-perspective` — same shape, `id="puzzle3d-main-perspective"`
- Id assigned via `windowElementId` (from `@semio-tech/framework`, consumed by `World3dHost/🟦️.tsx`).
  Confirmed literal DOM shape in
  `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx:588-591`.
- Each pane mounts an `@react-three/fiber` `<Canvas>` inside `World3dHost` — use
  `canvas.getContext('webgl2')`/`getImageData` blank-check per the runtime-verification-plan's §1.

**Navbar example select** (`🏛️ShellHost/🟦️.tsx:7369`, component defined in
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx`):
- Wrapper `id="playground.navbar.fixture"` → rendered as
  `#playground.navbar.fixture.trigger` (Select trigger button to click),
  `#playground.navbar.fixture.select` (Radix Select root), `#playground.navbar.fixture.label` (sr-only label).
- Dispatches `{controllerId: session.app.controllerId, action: "setActiveExample", args: {exampleId}}`
  (`🏛️ShellHost/🟦️.tsx:7355-7362`).

**Fill tool** (utility id `fill`, panel schema from
`✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` and `✏️editor/🗣️terminology/🦀️.rs:25,27`):
- Toolbar toggle button: `#fill` (`UtilityTree/🟦️.tsx` `UtilityRibbonItems` — leaf `id`/`aria-label`
  come straight from the utility node id/label, no prefix). Click to activate; dispatches
  `SET_ACTIVE_UTILITY_ACTION_ID` and mounts the utility's option row.
- Options group: `#fill-params` (`WindowMeasureTreeGroup`/tree item `id = measure.id`,
  `🛠️ShellHelpers/🟦️.tsx:2632-2710`).
- Slider: `#fillCount` — literally `<Slider id={measure.id} ...>` in `WindowMeasureSlider`
  (`🛠️ShellHelpers/🟦️.tsx:2523-2564`), value 3, min 1, max 9, step 1. Drag dispatches
  `{controllerId:"puzzle", action:"setFillCount", args:{value}}` on `onValueCommit`/`onValueChange`
  (reveal-group sliders like this one only round-trip through WASM on gesture release — see below).
  **Note:** the fill-count slider is a `reveal`-tagged measure (`measure.reveal` set) — mid-drag it
  writes straight into `worldRevealCutoffStore` (main-thread visibility cutoff, no WASM round trip);
  only the final `onValueCommit` value sends `setFillCount`. A test asserting "every drag tick sends an
  action" will be wrong for this control; assert on release instead.
- `fillBuildTick` progress: dispatched on an interval from `World3dHost/🟦️.tsx:4366`; look for
  `statusLabel: "Fill progress"` / `"Füllfortschritt"` inside the parsed `fill_preview_json` blob — see
  `parseWorldBrushPreview()` (`World3dHost/🟦️.tsx:1161-1298`) and the engine-contract tests at
  `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts:4721-4853` for the exact wire shape and 4096-byte
  wire cap.

**Context menu** — right-click on the world canvas → `World3dHost` calls `openSurfaceContextMenu()`
(defined in `🗣️Interpreter/🟦️.tsx:520-531`, imported by `World3dHost/🟦️.tsx:91`) → renders via
`ContextMenuController` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx`). DOM: root
`role="menu"`, items `role="menuitem"`, icon slot `data-slot="context-menu-icon"`. Puzzle-specific items
come from the plugin's `ContextMenuItemSpec` rows, mapped in `World3dHost/🟦️.tsx:1104-1138`
(`mapContextMenuSpecs`).

**Panels** (tab kind ids — click the tab, then read its tree body):
- Inspection: `framework.panel.inspection` (`FRAMEWORK_PANEL_TAB_INSPECTION_ID`); tree root id
  `puzzle3d-play-inspector`, rows `puzzle3d-play-inspector.schema/.domain/.objects`
  (`✏️editor/📌️panels/🔍️inspection/🦀️.rs:17,37-41`).
- Artifact: `framework.panel.artifact` (`FRAMEWORK_PANEL_TAB_ARTIFACT_ID`) —
  `✏️editor/📌️panels/🗿️artifact/🦀️.rs:25-26`.
- Catalogue: `framework.panel.catalogue` (`FRAMEWORK_PANEL_TAB_CATALOGUE_ID`) —
  `✏️editor/📌️panels/🛍️catalogue/🦀️.rs:22-23`.
- Settings: `puzzle3d.panel.settings` (app-local `PANEL_TAB_ID`, NOT a `FRAMEWORK_*` constant) —
  `✏️editor/📌️panels/⚙️settings/🦀️.rs:15,21`; fields include
  `puzzle3d-play-settings.overlap-budget/.proximity-radius/.chunk-size` steppers.
- Every tab renders through `panelTabDefinitionToNode()` (`🛠️ShellHelpers/🟦️.tsx:1547-1576`), whose
  tree-item `id` is exactly the tab kind id above. Framework-owned tabs (Display/Settings/general) use
  the separate `framework.settings.*`/`framework.display.*` id family in
  `📌️ChromePanels/🟦️.tsx` (e.g. `framework.settings.appearance`, `framework.settings.language`).

**Brush/gumball/marquee pointer path** — all in `World3dHost/🟦️.tsx`:
- `dispatch("worldPointerDown", …)` at line 4763, `dispatch("worldPointerMove", …)` at line 4797 (pane
  disambiguated via `paneSuffixFromSurfaceId`).
- Gumball: `UnifiedGumball` component, driven by `gumballConfigForTransformMode`/
  `worldGumballConfigForProjection`; live mid-drag preview is applied imperatively to the Three.js
  object (no WASM round-trip until `onDragEnd` → `translateSelection`/`rotateSelection`/
  `scaleSelection`). Visibility gated by `selection.gumballActive && isWorldTransformGumballMode(mode)`.
- Marquee: `SelectionMarquee` + `marqueeModeFromModifiers`/`marqueeCoverageFromGesture` (rectangle vs.
  lasso — lasso is utility id `selectLasso`, exposed via `SelectionUtilityOptions`,
  `🛠️ShellHelpers/🟦️.tsx:2710+`).
- Brush placement: `BrushMeshRegistrar` registers glb meshes (`registeredPuzzle3dBrushMeshes`), ghost
  preview parsed by `parseWorldBrushPreview`, placement action built by `brushObjectPlacementArgs()` →
  dispatches `addBrushObject`. Instance-mesh picking is force-disabled during fill/brush/volumeBrush/
  surfaceBrush engagements (`World3dHost/🟦️.tsx:1271-1278`) so clicks don't fall through to selection.

## 1. React renderer path (port 6013)

- **Shell host**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
  (9013 lines — grew since the plan doc's `:6423` reference; `NavbarExampleSelect` usage is now at
  `:7369`, its import at `:329`). Owns the whole shell reducer, dock/panel layout, example-select
  wiring, utility activation (`setActiveUtilityForWindow` at `:3411`, effect handling `:4242-4264`),
  action dispatch (`onAction`/`applyHostEffects`), and every `[DEBUG]` trace listed in §5 below.
- **3D scene renderer**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
  (5323 lines). Uses real `three` + `@react-three/fiber` + `@react-three/drei`
  (`OrbitControls`, `GizmoHelper`/`GizmoViewport`, `Grid`, `TransformControls`-equivalent via
  `UnifiedGumball`). This is the component behind BOTH `puzzle3d-main-top` and
  `puzzle3d-main-perspective` — camera/projection differ per window instance via
  `worldProjectionGumballPlane`/`projectionSpec`, not via a different component.
- **Generic scene-host adapter layer**: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx` and the
  published package mirror `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx`
  — declares the "Scene host surface for puzzle/cad R3F + three.js", re-exports `ThreeCanvas`,
  `OrbitControls`, `useThree`, etc. `🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx` is a separate, smaller
  three.js host used elsewhere (not puzzle3d's own windows).
- **Component dispatch table**: `🗣️Interpreter/🟦️.tsx:287-304` (`resolveComponentSceneHost`) maps
  `ComponentKind` → host component; `"world-3d"` → `World3dHost`. Also owns the generic declarative
  control renderer (`renderUiControl`, slider/select/toggle/button/input cases, `:760-825`) used for
  puzzle3d's settings-panel steppers, and the `UiNodeView`/`SliderView`/`ButtonView`/`InputView` family
  (`:960-1080`) used for retained-document (wgpu) UI nodes — DOM ids there are `node-${record.id}`
  and `data-ui-node-id={record.id}`, a DIFFERENT id scheme from the React-path `WindowMeasureSlider`
  (`id={measure.id}` directly). A generic browser harness should check BOTH id shapes depending on
  which renderer target is live.
- **Utility toolbar**: `🧱️elements/🎛️UtilityTree/🟦️.tsx` (405 lines) — ribbon/tree renderer for the
  `utilities` taxonomy, grouped by `UTILITY_CATEGORY_ORDER` (`selection`, `utilities`, `history`,
  `sync`). Mounted per-window as `<UtilityTree id="ui.utilities.${windowId}" .../>`
  (`🛠️ShellHelpers/🟦️.tsx:2826`).
- **Panels**: `🧱️elements/📌️ChromePanels/🟦️.tsx` (1389 lines) — framework-owned Display/Settings tree
  builders (`FRAMEWORK_SETTINGS_PANEL_ID` etc., `:71-81`). Puzzle3d's own panels (inspection/artifact/
  catalogue/settings) are schema-authored in Rust (`✏️editor/📌️panels/*/🦀️.rs`) and rendered through
  the same generic tree/tab machinery (`panelTabDefinitionToNode`, `🛠️ShellHelpers/🟦️.tsx:1547`).
- **Shared helpers** (huge, load-bearing file): `🧱️elements/🛠️ShellHelpers/🟦️.tsx` — window-measure
  tree rendering (sliders/groups/selects/toggles), panel-tab-to-tree mapping, coalescing/in-flight-
  skipping dispatchers used by `fillBuildTick`/`suggestionsTick` (`:2418-2456`), and
  `registeredPuzzle3dBrushMeshes`/`PUZZLE3D_FILL_REVEAL_GROUP_ID`/`worldRevealCutoffStore` (puzzle3d-
  specific constants living in a generically-named file).

## 2. wgpu/wasm renderer path (port 6113) — brief map + status

Key files:
- `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` (`render_with_document_js` — the browser retained-document
  render entrypoint; was a stub through §43, implemented at §44).
- `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` — top-level dedicated worker; owns the 30s browser-owned
  boot ceiling, renderer-bootstrap stage timing (`[DEBUG] renderer-bootstrap stage=…`, line 331).
- `🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` — `spawnShardWorker()`, the 60s boot-stall
  timeout, `boot-liveness` heartbeat, and (since §41) main-thread `MessageChannel` relay so shard
  workers no longer need to be nested inside the frame worker.
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` — `MainThreadShardWorker`,
  `settleInstanceLifecycle` (open→receipt→ACK), `performRender`'s `uiPatches` application. Carries the
  heaviest `[DEBUG]` trace density outside ShellHost (see §5).
- `🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime` and `🧰️framework/🔨️modules/🎭️actor/📮️shard-client`
  — `ShardClient` (turn submission, `captureInstanceLifecycle().open()`, heartbeat/rebuild), the actual
  actor-turn transport shared by every renderer target, not puzzle3d-specific.

**Status** (per `📓️findings-2026-09-05.md` §39-46, all verified against a running app on 2026-09-07):
boot progressed through 7 blockers in order — (1) lifecycle turn wrongly judged by the 8ms per-frame
ceiling [FIXED §39], (2) this Browser pane's Chrome instance could not create nested dedicated workers
[WORKED AROUND §41 by spawning shards on the main thread over a `MessagePort` — real Chrome is expected
to support nested workers natively but that was never independently verified here], (3) hand-rolled
`instance-open` event missing `activationGeneration`/`requestSequence` [FIXED §42], (4) the open
handshake's receipt was never ACKed, so instances silently never registered [FIXED, part of §44], (5)
`render_with_document_js` was an unconditional `Err(...)` stub [IMPLEMENTED §44], (6) `ArenaFull` from
an undersized resident permit, not arena slots [FIXED §44, third attempt], (7) various browser-owned
budget overruns [tuned §44]. §44 reports the render path as structurally complete and the boot reaching
**font-atlas 68%**, verified with progress markers through what §46 calls **94%** of the boot sequence.
The LAST blocker recorded (§45-§46) is **not in puzzle3d's own code**: `semio-framework-os-flow` fails
to compile because a concurrent, unrelated ticket (`COMPOSABLE-STDIO-ARTIFACT-PACKAGES`) is mid-way
through extracting flow's artifact into its own crate — the manifest half landed, the source half
(`🌉️bridge/🦀️.rs`, `🖥️host/🦀️.rs`) had not, as of the 2026-09-05 write-up. **A later verification
session must re-check whether that peer migration has since finished** before assuming the wgpu path is
still blocked — this is exactly the kind of "check mtimes, don't refight a stale blocker" situation
flagged in this session's own memory notes.

## 3. Test harness inventory (cheapest → most expensive)

| # | What | Command | Rough cost |
|---|---|---|---|
| 1 | Rust unit tests — puzzle3d editor/viewer/panels (18 `🦀️.rs` files under `🧪️tests/🔬️unit`, plus `🎚️config`, `📚️examples/…/🧪️tests/🧩️example`, `🧪️tests/🔬️testkit`) | `cargo test -p semio-s-artifact-puzzle-3d` (crate name from `🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`) | seconds–low minutes; standard cargo unit-test cost, no wasm/browser involved |
| 2 | Rust design-parity-schema tests (`🗿️artifacts/🧊️3d/🧪️tests/🔬️design-parity-schema/🦀️.rs`) | same crate, folded into #1 | negligible add-on |
| 3 | puzzle-js vitest — example fixture round-trip (`📦️packages/🟦️typescript/vitest.config.ts`, `include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts"]`) | `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts test` | seconds; node-environment vitest, no browser |
| 4 | Root TS self-tests wired into `verify interactivity` (`interactivityPuzzleFillEnvelopeSelfTests`, `interactivityPuzzleFillP4eSelfTests`, `interactivityPuzzleFillPreviewJsonSelfTests`, `toolJobPuzzleReservedRoutesExact` used inline) | `bun 📜️script.ts verify interactivity` | seconds–low minutes; static-source policy checks (regex/string scans over committed `.rs` sources), not a build |
| 5 | puzzle-js publication-authority audit (`ArtifactToolPublicationContract` lane exactness vs. `PUZZLE3D_RETAINED_TOOL_IDS` etc.) | `bun ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts publication-authority-audit` | seconds; static source scan |
| 6 | `.feature`+`.py`+`.rs` mutation harness (`🧪️tests/🧊️mutate-puzzle-3d-1`) — the no-oracle mutation case for `Puzzle3dMutation`, asserts observability + footprint-completeness laws in-role against committed `(before,mutation,diff,outcome,after)` fixture quintets | discovered as its own generated Nx project via the repo's test-discovery library (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, globs `**/🧪️tests/*/component.feature`) — **did not resolve the exact generated project name in this pass; run `bun nx show projects \| grep -i puzzle` (or equivalent) to find it before invoking**, then `bun nx run <project>:test` | low; single Rust test host process, no wasm |
| 7 | Storybook stories + Playwright smoke — `.storybook/stories/puzzle/3d/World.stories.tsx` (`ConcreteForest`, `NakaginCapsuleTower`), spec at `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts` | `bun run test:storybook` (builds `storybook-static/`, serves it, runs Playwright with `PLAYWRIGHT_BASE_URL` set) **or** directly: `bunx playwright test ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts --config .storybook/playwright.config.ts` against an already-served `storybook-static/` | minutes; real Chromium via Playwright, but against a MOUNTED COMPONENT (`World3dHost` alone), not the full app shell — no navbar/example-select/panels in this harness |
| 8 | `◻️storybook-2d` (puzzle 2D Board/Fixtures stories) and the broader `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts` | same `.storybook/playwright.config.ts`, other `testMatch` entries | minutes |
| 9 | `dev:puzzle:3d` / `dev:puzzle:3d:concrete-forest` live dev server (`bun nx run workspace:dev -- 3d [fixture concrete]`) + manual/agent browser drive against the FULL shell (ShellHost + both windows + panels) per `📓️runtime-verification-plan.md` | `bun run dev:puzzle:3d` then navigate a real/automated browser to `localhost:6013`; `SEMIO_RENDERER=wgpu` env var for the 6113 wgpu path (no dedicated npm script found — pass the env var manually) | slow to first paint (this session's memory notes record ~20-plugin WASM cold boots being normal — be patient before judging errors); the only harness that exercises the real navbar/example-select/panel/context-menu/fill-tool DOM described in the TL;DR |

**No standalone Playwright/e2e project exists for the full ShellHost app itself** — only for individually
mounted Storybook stories (`.storybook/stories/framework/os/os.stories.tsx`,
`.storybook/stories/framework/os/Wgpu.stories.tsx` exist as stories but I did not find a spec file
driving them through `.storybook/playwright.config.ts`'s `testMatch`). A later session building the
"every window/tool works" verification will most likely need to drive `localhost:6013` directly (item 9
above) via a real browser tool, using the selector map in the TL;DR, rather than extending the
Storybook-scoped Playwright harness — the latter tests components in isolation, not the composed shell.

The legacy-looking `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e` and
`♻️mit-bestand/🧺️demonstrator/test-results` directories were NOT investigated further — the `♻️`
("recycled"/archived) prefix and the repo's stated no-legacy-code posture suggest this is an archived
bundle, not the active harness; confirm with the ticket owner before relying on it.

## 4. Runtime verification plan — automatable vs. missing hooks

Read in full: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️runtime-verification-plan.md`.

**Automatable now, with hooks that exist in source** (mapped to TL;DR selectors above):
- §1 Shell boot & window render — `#puzzle3d-main-top` / `#puzzle3d-main-perspective` exist verbatim.
- §2 Default example (Concrete Forest, 1 object) — no direct object-count DOM hook found in `World3dHost`
  besides the Storybook story's `data-testid="puzzle3d-world-debug"` `<pre>` (which is Storybook-only,
  not present in the live app shell) — see "missing" below.
- §3 Example switching — `#playground.navbar.fixture.trigger` to open, then a Radix `SelectItem` per
  option (no stable per-option id found; select by visible text/locale label).
- §4 Fill tool — `#fill` (activate), `#fill-params` (group), `#fillCount` (slider) all exist; commit
  fires on release, not on every drag tick (see reveal-group note in TL;DR).
- §6 Console fault codes — all of `interactive-job.not-ui-safe`, `ui-dispatch.plugin-internal`, the
  puzzle work-capacity fault, `RawWireLimit`, and the live-cleanup fault are plain string/JSON console
  output (`console.error`/`console.warn` in `ShellHost/🟦️.tsx`) — greppable via
  `read_console_messages`/`javascript_tool` console hooking, no extra instrumentation needed.

**Hooks that do NOT exist in the live app shell today and would need adding**:
- No `data-testid` for scene object count / selected object label in `World3dHost` or `ShellHost` —
  the ONLY place `data-testid="puzzle3d-world-debug"` exists is the isolated Storybook story
  (`.storybook/stories/puzzle/3d/World.stories.tsx:315`), not wired into the live `dev:puzzle:3d` app.
  A browser check of "object count == 1" today has to fall back to counting rendered mesh nodes via
  `javascript_tool` poking the Three.js scene graph, or reading the inspection panel's
  `puzzle3d-play-inspector.objects` tree row text (exists, see §1 panel map) — the latter is the
  more robust hook and IS already present in Rust (`✏️editor/📌️panels/🔍️inspection/🦀️.rs:39`).
- No stable per-option id on the example-select dropdown items (`SelectItem key={row.id} value={row.id}`
  — `value` IS the example id and is a legitimate selector target, e.g.
  `[role="option"][data-value="nakagin-capsule-tower"]` depending on the underlying Select
  implementation's DOM — worth confirming against the live-rendered Radix markup rather than assuming).
- No dedicated `data-testid` for "Fill progress" status text in the live app — it lives inside the
  `fill_preview_json` wire payload parsed by `parseWorldBrushPreview`, not rendered as a labeled DOM
  node by itself (the label text does appear somewhere in the brush/fill ghost overlay UI, but this
  session did not trace that render call site precisely — worth a follow-up read of
  `World3dHost/🟦️.tsx` around the `fillBuildPreview`/`statusLabel` usage before relying on visible text
  matching).
- Vortex indicator toggle ("Vortex Show") — schema exists
  (`✏️editor/🎭️modes/✏️edit/☑️options/🌀️vortex/🦀️.rs`) but this session did not locate its rendered
  toggle id; likely another `WindowMeasureToggleControl` (`id = measure.id`) — confirm the measure id
  string before scripting a click.

## 5. `[DEBUG]`-prefixed logs found (candidates for removal at ticket close)

**Puzzle plugin itself (`✏️s/🔌️plugins/🧩️puzzle/**`): zero `[DEBUG]` logs found** — the plugin's own
Rust/TS sources are clean.

Everywhere else searched, `[DEBUG]` logs are dense and NOT puzzle3d-specific — they are framework-wide
shell/renderer instrumentation that puzzle3d happens to exercise:

- `🏛️ShellHost/🟦️.tsx` — ~55 occurrences (tutorial lifecycle, hot-swap, extension install/uninstall,
  history snapshot, document backbone, `setActiveUtility failed` at `:5040`, `setActiveTool failed` at
  `:5068`, `action failed` at `:5223`, `skipping undeclared action` at `:5188`, etc.). Full list
  reproducible with `grep -n "\[DEBUG\]" 🏛️ShellHost/🟦️.tsx`.
- `🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx` — 4 occurrences (scoped-presence socket/roster/close).
- `🌐️World3dHost/🟦️.tsx` — 5 occurrences: `hoverSuggestion` (:1133), `world3d viewport reattached to
  scene camera` (:3926), `brushPreview` (:4084), `world3d viewport detached from shared scene camera`
  (:4112), `gumball drag begin`/`gumball drag end` (:4717, :4732).
- `🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs` — 4 occurrences (native credential/scale-flag checks).
- `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` — 1 (`renderer-bootstrap stage=… took …ms`, :331).
- `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` — 7 occurrences, including the
  `publishRetainedDocument … turns=… collected=… anyPatches=… effects=…` diagnostic (:301) that §44 of
  the findings doc explicitly used to catch the silent-ACK bug — **this one may be intentionally load-
  bearing for future debugging, confirm before deleting**.
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` — ~9 occurrences, several thrown as
  `Error("[DEBUG] …")` (i.e. the `[DEBUG]` text is embedded in a real thrown error message, not just a
  console log — deleting the prefix would change error text that other code may match against, check
  call sites before touching). **Line 559's own comment states: "The `[DEBUG] ` line is deliberate,
  permanent[...]"** — i.e. this file's author has already opted at least one of these OUT of the
  "temporary, must be removed" convention. Treat every `[DEBUG]` in this file as needing an explicit
  read of its surrounding comment before removal, not a blanket strip.
- `📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` — 5 occurrences, all inside
  test assertions/diagnostics (lower priority, but still matches the literal prefix).

**Recommendation for whoever closes out DOM/hook additions on this ticket**: do NOT delete the
`plugin-bridge.ts:301` and `shard-client.ts:559`+ logs reflexively — re-read the surrounding comments
first, since at least one is explicitly documented as a permanent diagnostic despite the `[DEBUG]`
prefix, which conflicts with CLAUDE.md's "temporary logs" framing for that prefix. Everything in
`ShellHost/🟦️.tsx` and `World3dHost/🟦️.tsx` reads as genuinely temporary trace instrumentation added
during this ticket's debugging and is a reasonable removal candidate once the boot/render path is
confirmed stable end-to-end.

## Files touched by this exploration (read-only)

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️runtime-verification-plan.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️findings-2026-09-05.md`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎛️UtilityTree/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/**` (frame-worker,
  browser-frame-transport, plugin-bridge, native-entrypoint)
- `🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime`, `🧰️framework/🔨️modules/🎭️actor/📮️shard-client`
- `✏️s/🔌️plugins/🧩️puzzle/**` (Rust editor/viewer/panels/commands tree; TS self-test files; vitest
  config; puzzle-js `📜️script.ts`)
- `.storybook/playwright.config.ts`, `.storybook/stories/puzzle/3d/World.stories.tsx`,
  `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial/🟦️.ts`
- `package.json` (root `dev:puzzle:3d*` scripts)

No files were edited, no builds/dev servers/tests were run.
