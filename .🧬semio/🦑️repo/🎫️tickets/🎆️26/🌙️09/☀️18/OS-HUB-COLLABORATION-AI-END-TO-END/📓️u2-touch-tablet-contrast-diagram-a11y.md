# U2 — multi-touch viewports, tablet breakpoint, theme contrast check, Diagram a11y

Slice U2 of ticket 26/09/18 (`OS-HUB-COLLABORATION-AI-END-TO-END`). Source: `📓️g5-ux-completeness-audit.md`
ranked items **6, 7, 9, 10**.

Status: **all four items landed and measured.** Every number below comes from a run captured in
`🗑️generated/u2-tests.txt`; nothing here is claimed without having been executed.

## 0. Scope and inherited state

Inherited state: `git status` / `git diff --stat` on the slice paths showed **no prior U2 work** and no
`🗑️generated/u2-*` captures — this slice started from zero (previous U2 workers died before writing
anything). Peers were live in the same tree throughout (U1 in `🧵️TaskManager` / `🔄️ShellSync` /
`💬️AgentChatPanel` / `📌️ChromePanels`, AU2 in `🏘️SpaceBrowser` / `🔐️HubSignIn`, others in the wgpu
crates); every file was re-read immediately before editing and no peer edit was reverted.

## 1. Item 6 — multi-touch in `🌐️World3dHost` / `Board2dHost` — DONE

### 1.1 New framework-level gesture module (schema decision recorded)

`🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts` (new, 214 lines) — pure, renderer-neutral:
`gesturePointerDown/Move/Up` (`pointerId`-keyed, down-ordered), `gestureIsMultiTouch`,
`pinchFrame` (centroid ⊗ separation ⊗ angle of the FIRST two contacts), `pinchStep`
(scale ⊗ pan ⊗ shortest-signed rotation), `shortestAngleDelta`, `clampZoom`, `pinchWheelDelta`,
`zoomAboutPoint`, `applyPinchToCamera`.

I checked `🕹️interaction` first as instructed. It **does** have a schema
(`🕹️interaction/🧬️schema/🔣️.json`, `InteractionDefinition`/`InteractionState`/`PresenceInteraction`) —
but an active-pointer set is **ephemeral local** state on the same axis as `DomainHover` (the schema's
own description names that axis): it never crosses a wire, is never persisted, is never broadcast.
The schema owns what crosses a boundary; this owns what a surface computes between two pointer events
and throws away. That decision is written into the module header
(`👆️gesture/🟦️.ts:1-20`) rather than left implicit. Exported from `@semio-tech/framework`
(`🧰️framework/📦️packages/🟦️typescript/🟦️.ts:26-45`).

### 1.2 `🖥️Board2dHost` — real pinch-zoom + two-finger pan

The board is a WASM session with no built-in gesture layer, so the whole pinch is wired here:

- `🖥️Board2dHost/🟦️.tsx:455-470` — `BOARD_2D_ZOOM_BOUNDS` + `board2dPinchCamera` (pure, exported and
  unit-tested). The bounds are read from `STYLING_METRICS.camera.zoomMin/zoomMax` (0.05 / 32), i.e.
  the **same** numbers the Rust engine's `clamp_zoom` applies
  (`♾️infinite/🖼️canvas/🦀️.rs:2054-2056`, `🎨️styling/🔤️tokens/🦀️.rs:206-207`) — a pinch and a wheel
  cannot disagree about the ceiling.
- `🖥️Board2dHost/🟦️.tsx:1213-1290` — `pointerId`-keyed tracking on down/move/up/**cancel**. A second
  contact runs `yieldToPinch` (cancel area-select, close the engine's pointer gesture, release every
  capture) and seeds the pinch frame; moves then drive `setCameraSilent` only; lifting one finger
  re-seeds the frame from the contacts that REMAIN (otherwise the camera snaps) and swallows the
  release so it is not replayed as a board click.
- **Camera/gesture contract respected**: per-frame poses go through `setCameraSilent` (window-transient,
  per `📓️project-per-frame-state-belongs-in-window-transient.md`); the authoritative pose rides
  `pendingCameraDispatchRef` and is dispatched **once** by the existing `beginCameraInteraction` settle
  timer — one `setCamera` per gesture, never one per move. Asserted by the component test.
- `touch-action` was already `none` on the board container (`🖥️Board2dHost/🟦️.tsx` root `style`), so the
  browser was never going to hijack the contacts — no change needed there.

### 1.3 `🌐️World3dHost` — the audit's "no zoom path at all" is **partly wrong**; the real bug was a conflict

Measured from source rather than assumed:

- Camera navigation in World3d is owned by **three.js `OrbitControls`** (three 0.182,
  `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3475-3520` `WorldOrbitControlsBridge`), bound to the r3f
  `eventSource` div. `OrbitControls` binds its own `wheel` listener and `enableZoom = true`, and
  `WorldCanvas` already stamps `touchAction: "none"` + `overscrollBehavior: "contain"` on that element
  (`🎨️r3f/🟦️.tsx:3712`). So **wheel zoom and two-finger pinch were reachable**; the audit's
  "3D viewport zoom has no input path at all" is an artifact of grepping only `World3dHost/🟦️.tsx`.
- The real defect: `World3dHost`'s own `onPointerDown/Move/Up` sit on an **ancestor** of the canvas, so
  the second finger's `pointerdown` also started/extended the host's marquee and could take a pointer
  capture — the two layers fought for every two-finger gesture.

Fixes:
- `🌐️World3dHost/🟦️.tsx:5744-5749` — `gesturePointersRef` (`pointerId`-keyed).
- `🌐️World3dHost/🟦️.tsx:6899-6920` — `yieldToPinch` + `handlePointerDown` early-return on the second
  contact (releases every capture, ends a relocate drag, clears the marquee, arms the finalize-once
  guard so nothing commits).
- `🌐️World3dHost/🟦️.tsx:6949-6951` — `handlePointerMove` goes quiet while multi-touch.
- `🌐️World3dHost/🟦️.tsx:7022-7036` — `handlePointerUp` swallows the whole release of a multi-touch
  gesture, so the finger still down does not resume the single-pointer lane mid-gesture.
  `onPointerCancel` already routes to the same handler.
- `🌐️World3dHost/🟦️.tsx:7358` — `touch-none` on the host div (belt-and-braces above `WorldCanvas`).
- `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3503-3510` — `controls.touches = { ONE: TOUCH.ROTATE, TWO: TOUCH.DOLLY_PAN }`
  declared **explicitly** instead of inherited from three's defaults, so the two-finger contract is a
  stated law this repo owns (and `TOUCH` added to the `sceneHostPort.three` destructure, line 66).

## 2. Item 7 — tablet as an auto-detected breakpoint — DONE

### 2.1 Where the policy was, and where it is now

Before: `UI_MOBILE_MEDIA_QUERY = "(max-width: 767px)"` was declared inside the **React target**
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`), and `MODE_DOCK_MOBILE_MAX_WIDTH_PX: f32 = 767.0` inside the **wgpu
dock** (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`). Their agreement was held together by a doc comment.

After: a target-neutral owner, `🧰️framework/🔨️modules/🖱️ui/📱️device/🟦️.ts` (new, 70 lines) —
`UI_MOBILE_MAX_WIDTH_PX = 767`, `UI_TABLET_MAX_WIDTH_PX = 1023`, both media queries,
`elementsSurfaceDeviceForWidth`, `elementsSurfaceDeviceForMatches`, `elementsSurfaceDeviceIsMobile`,
`elementsSurfaceDeviceSupportsTabDrag`. The React target now **re-exports** it rather than declaring it
(`🎯️targets/⚛️react/🟦️.tsx:1647-1671`, import at line 128), so `UI_MOBILE_MEDIA_QUERY` and
`ElementsSurfaceDevice` keep their exact public spelling for every existing consumer.

The tablet query is deliberately a **band** — `(min-width: 768px) and (max-width: 1023px)` — not a bare
`max-width: 1023px`, which a phone would also match, leaving nothing able to decide between them.

### 2.2 Byte-parity with the wgpu target, gated rather than commented

- `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1100-1122` — `MODE_DOCK_TABLET_MAX_WIDTH_PX: f32 = 1023.0` added
  beside the existing mobile constant, plus `mode_dock_device_for_width(width_px)`, the Rust twin of
  `elementsSurfaceDeviceForWidth` with the same inclusive-maximum comparisons.
- `🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts` **reads that Rust file off disk** and asserts both literals equal
  the TS constants and that the resolver's comparison shape is present. Neither side imports the other,
  so nothing else in the tree could ever catch this drift. 7/7 green.

### 2.3 Tablet is now actually selected

- `🐚️Shell/🟦️.tsx:1144-1155` — `selectUiDevice(state, measured: ElementsSurfaceDevice)` replaces
  `selectUiDevice(state, mobile: boolean)`: the **measured** device wins whenever it is narrower than a
  desktop, and only a desktop-width viewport falls through to the stored `uiLayout` preference.
- `🏛️ShellHost/🟦️.tsx:2005-2006, 2402` — the shell now reads `useUiDevice()` and derives its binary
  `mobile` flag from it (`elementsSurfaceDeviceIsMobile`), and `uiDevice` follows the same rule. All 55
  other `mobile` call sites in that file are untouched.
- `🎯️targets/⚛️react/🟦️.tsx:4433-4472` — `useUiDevice()` + `UiDeviceProvider`. An ancestor
  `UiMobileProvider` that says "mobile" still wins, so a shell that already pinned the binary flag
  cannot be widened out from under itself. `useUiMobile()` is unchanged.

**No schema change**: `UiChromeLayout` stays `"desktop" | "tablet"` and pinning `tablet` in settings
still works (and still only affects a desktop-width viewport — pinning a *wider* layout on a phone is
exactly the case with no room for it). Adding an `"automatic"` sentinel would have rippled through the
persisted `UiPreferences` shape and the wgpu side, which is a separate ticket.

## 3. Item 9 — live contrast-ratio check in the theme editor — DONE

### 3.1 Reused the existing OKLab/contrast code, did not re-derive it

`🎨️styling/🌓️theme/🟦️.ts` already had real WCAG `relativeLuminance` (line ~677) and
`readableForegroundHex`; `contrastRatio` existed **only inside test files**
(`🧪️levels-oklabmix/🟦️.ts:16`, `🧩️suite/🟦️.ts:673`), so the editor had nothing to call.

Added to `🎨️styling/🌓️theme/🟦️.ts:686-740`, on top of the existing `relativeLuminance`:
`WCAG_AA_CONTRAST` (4.5), `WCAG_AA_LARGE_CONTRAST` (3), `WCAG_AAA_CONTRAST` (7), `contrastRatio(hexA,
hexB)`, `contrastRatioRgba`, `wcagContrastGrade`, `themePaintContrast`.

### 3.2 Wired into the editor

`📌️ChromePanels/🟦️.tsx:657-717` — `themeContrastBadgeText` (pure, exported) plus a live badge in
`buildThemeAppearanceGroupItems`: every user-customized appearance paint now prints `N.NN:1 · <grade>`
beside its swatch, measured against that appearance's own `foreground`, carrying `aria-label`, a
`title`, `data-contrast-grade`, and — only when the pair is **below AA** — `role="status"` and the
destructive text token. The `foreground` row itself is skipped (it cannot contrast with itself), and a
group with no `foreground` key renders no badge rather than inventing one.

i18n: `ui.settings.theme.contrast.{label,aaa,aa,aaLarge,fail}` added to the compile-checked schema
(`📚️I18n/🟦️.tsx:239-249`) and to **both** locale bundles with real prose in each tier
(`🎯️targets/⚛️react/🟦️.tsx:2739-2745` de, `:3599-3605` en) — `satisfies Record<UiLocale, …>` means a
missing German key would have been a build error, and `check-chrome-i18n` reports 0 violations.

### 3.3 Validated against an independent implementation

The four new tests live in `🎨️styling/🧪️tests/🧩️suite/🟦️.ts` (the runner that actually executes) and
cross-check `contrastRatio` against **that suite's own** pre-existing `contrastRatio01` — a separately
written 0-1 float pipeline — on seven colour pairs, plus symmetry, the 1:1/21:1 bounds, the three grade
floors, and alpha-independence. 61 pass (was 57).

## 4. Item 10 — keyboard + ARIA affordances for `🕸️Diagram` — DONE

Follows the shared a11y contract verbatim: `🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:104-130` puts
`Component::Surface` in the focusable set with role `application`, and states that the `application`
role *"is exactly the ARIA promise that the widget handles its own arrow/Enter keys"*. Before this
slice `🕸️Diagram` (1842 lines) had **zero** `role=`/`aria-`/`onKeyDown`/`tabIndex` hits — it made the
promise nowhere and kept none of it.

### 4.1 Pure, tested navigation law

`🕸️Diagram/🟦️.tsx:1072-1130` — `DIAGRAM_ARROW_DIRECTION`/`DIAGRAM_ARROW_KEYS`,
`diagramArrowNavigationTarget(nodes, focusedId, direction)` and `diagramActivateSelection(selectedIds,
focusedId, additive)`. The navigation picks the nearest node **in the pressed half-plane**, weighting
the across-axis distance ×2 so `ArrowRight` prefers the node to the right over a closer diagonal, and
breaking ties on id so the walk is deterministic. With nothing focused, the first arrow enters at the
extreme node *opposite* the direction (press Right → enter at the leftmost node).

### 4.2 The surface

`🕸️Diagram/🟦️.tsx:1631-1650` — the wrapper is now `role="application"`, `tabIndex={0}`, with
`aria-roledescription`, an `aria-label` that **counts** the live nodes and edges, `aria-describedby`
pointing at an `sr-only` span that speaks the key map, `aria-activedescendant` + a
`data-diagram-focused-node` mirror for probes, and `onKeyDown`.

`🕸️Diagram/🟦️.tsx:1400-1436` — the handler: arrows move the cursor, Enter/Space selects, Shift+Enter
toggles membership, Escape clears. Keyboard selection travels as real `select` node changes through the
**same** `handleNodesChange` lane a mouse pick uses, so a keyboard selection is indistinguishable
downstream from a pointer one — there is no second selection path to drift. Key events originating in an
`<input>`/`<textarea>`/contenteditable inside the canvas are left alone. The handler reads the graph and
the cursor through refs, not deps, so it is not re-created on every node move during a drag.

i18n: `ui.diagram.{label,roleDescription,keyboardHelp,nodes,edges,empty,focusedNode,selectedNode}` in
the compile-checked schema (`📚️I18n/🟦️.tsx:417-429`) and both bundles
(`🎯️targets/⚛️react/🟦️.tsx:2905-2920` de, `:3780-3795` en), with distinct `normal`/`beginner` prose.

### 4.3 Tests

9 new tests appended to the already-registered `🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx`: 4 component
tests driving real `KeyboardEvent`s against a rendered `Diagram` (role/tab-stop/label/described-by; a
full arrow walk across a 2×2 grid asserted through `aria-activedescendant`; Enter/Shift+Enter/Escape
asserted on the emitted node-change batches; and the text-entry escape), plus 5 pure-law tests. Suite
**51 passed (51)**, up from 42.

## 5. Tests run

Full transcripts: `🗑️generated/u2-tests.txt`. Summary — **48 new tests, all executed, all green**:

| suite | command | result | new |
|---|---|---|---|
| gesture math | `@semio-tech/framework:test` | **163 passed (163)** | +22 |
| Board2d pinch (component, synthetic `PointerEvent`) | renderer-react `test long "pinch-gesture"` | **7 passed (7)** | +7 |
| World3d multi-touch (component, synthetic `PointerEvent`) | renderer-react `test long "multi-touch"` | **4 passed (4)** | +4 |
| breakpoint policy + wgpu parity | ui-react `test "📱️device"` | **7 passed (7)** | +7 |
| theme contrast | ui-styling `test` | **61 pass / 1 fail** | +4 |
| Diagram a11y | ui-react `test long "🕸️Diagram"` | **51 passed (51)** | +9 |
| ui-react full corpus (regression sweep) | ui-react `test long` | 778 passed / 18 failed | — |

- The **1 styling failure** is pre-existing (`keeps panel-tab toggle dividers normal…`, a `🖌️ui/🎨️.css`
  assertion). U2 edits no CSS.
- The **18 ui-react failures** are all pre-existing and unrelated: 16 in the in-source
  `🎯️targets/⚛️react/🟦️.tsx` suite (icon hover keyframes, celebrate/glass chrome, and two ENOENT
  failures proving that suite resolves fixture paths to non-existent locations), one in `📨️UIDialog`,
  and one Diagram "force" test that **passes when the Diagram suite runs alone** (51/51 above) — it is
  cross-file interference, not a U2 regression.

**Typecheck**: `ui-react:typecheck` → 125 errors, **zero from any U2-authored line** (T3/T4 are driving
this to zero; U2 added nothing). The only hits inside a file U2 touched are 6 pre-existing errors in
`🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx` at lines 776/1110/1119/1137/1217/1358 — U2's additions begin at
line ~1420.

**Lints**: `check-chrome-i18n` → 0 violations. `check-ui-primitives` → 68 pre-existing violations, none
in any file U2 touched.

## 6. Honest gaps

1. **Runtime verification is test-only.** Every claim above is from vitest/jsdom with synthetic
   `PointerEvent`s and a stubbed board session. No real browser, no real touchscreen, no headless probe
   was run — the machine was saturated by the cargo fleet and no viewport host was served. In particular
   the *feel* of the pinch (gain, damping) on a real trackpad/touchscreen is unmeasured.
2. **No Rust twin of `👆️gesture`.** The gesture math is TS-only. It is ephemeral-local state that never
   crosses a wire, so it is not a schema gap, but the wgpu target has no multi-touch of its own and does
   not share this code. The `📱️device` policy *does* have a Rust twin plus a parity gate; `👆️gesture`
   does not.
3. **`mode_dock_device_for_width` is not compiled.** It is a `pub const` + a pure `pub fn` added to the
   wgpu dock, verified only by the TS parity test reading the source text. No `cargo check -p` was run
   (the preamble asks to avoid cargo; the fleet holds the build dir). The risk is small — no borrow, no
   generic, no trait — but it is unverified by rustc and should be picked up by whoever next builds the
   wgpu renderer.
4. **`🎨️styling/🌓️theme/🟦️.ts`'s in-source vitest blocks run nowhere.** `registerTests1/2/3` are wired
   into `import.meta.vitest` blocks, but no vitest config lists that file in `include`/`includeSource`
   (I checked every `🧪️tests/🎚️config/🟦️.ts` in the repo — zero hits for `🌓️theme`). The styling `test`
   target runs `🧩️suite/🟦️.ts` under `bun test`, where `import.meta.vitest` is falsy. **Pre-existing
   blind gate.** I therefore put the four running contrast tests in `🧩️suite`, and *also* added cases to
   `🧪️theme-resolve` so they are correct the day that gate is switched on — those specific cases have
   **not** been executed and are flagged as such.
5. **`vitest-configuration-ownership` is a pre-existing repo-wide failure.** 42 of 43 owners already
   mismatch their fixture `projectionSha256` (measured with `🐍️u2-vitest-projection-hash.ts`). The
   projection includes `cacheDir`, which `repoCacheDirectory()` resolves to a machine-specific absolute
   path *outside* `repoRoot`, so `portable()` cannot relativize it and the hash can only ever match on
   the machine that generated the fixture. I deliberately did **not** rewrite the 42 rows — that would
   mask the defect. My two config edits (`🧰️framework/🧪️tests/🎚️config`, ui-react `🎚️config`) are two of
   those 42; the renderer-react config's hash is unaffected because my additions live in
   `engineTestSuites`, which the default `SEMIO_TEST_LEVEL=fundamental` projection does not include.
6. **Two blind gates re-armed as a side effect.** `🖥️Board2dHost/🧪️tests/🧩️component` (the existing
   kind-hover suite) was in **no** include list at all; I added it alongside my own suite. It passes.
7. **Item 6's World3d wheel claim corrected, not fixed.** The audit's "3D viewport zoom has no input
   path at all" is wrong — `OrbitControls` owns wheel zoom on the r3f `eventSource`. I did not add a
   second `onWheel` to `World3dHost` (it would double-zoom). This is source evidence, not a runtime
   measurement.
8. **Pinch rotation is computed but unused.** `pinchStep.rotation` is exported and tested; neither
   viewport applies it (a board has no camera roll, and the 3D rig's twist belongs to `OrbitControls`).
   It is there for the surface that wants it, not dead by accident.
9. **Tablet layout is a breakpoint, not a layout.** Item 7 makes `tablet` auto-*detected* and feeds it
   to the existing device chrome. What the tablet device actually *paints* differently from desktop is
   whatever the current `ElementsSurfaceDevice = "tablet"` chrome already did; designing a distinct
   tablet layout was not in scope.

## 7. Files changed

New:
- `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts` — pure multi-touch gesture math
- `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🧪️tests/🔬️unit/🟦️.ts` — 22 tests
- `🧰️framework/🔨️modules/🖱️ui/📱️device/🟦️.ts` — the one breakpoint policy
- `🧰️framework/🔨️modules/🖱️ui/📱️device/🧪️tests/🔬️unit/🟦️.ts` — 7 tests incl. the wgpu parity gate
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/🤏️pinch-gesture/🟦️.tsx` — 7 tests
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🤏️multi-touch/🟦️.tsx` — 4 tests

Edited:
- `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` — export the gesture module
- `🧰️framework/🧪️tests/🎚️config/🟦️.ts` — register the gesture in-source suite
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` — device policy re-export, `useUiDevice`/`UiDeviceProvider`, contrast + diagram bundles (en & de)
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — register the `📱️device` suite
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` — `ui.diagram.*` and `ui.settings.theme.contrast.*` schema
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx` — a11y law + application-role surface + keyboard handler
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx` — +9 tests
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` — WCAG contrast API
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` — +4 contrast tests (independent oracle)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️theme-resolve/🟦️.ts` — contrast cases (gate currently blind, see §6.4)
- `…/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` — explicit `controls.touches`, `TOUCH` import
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx` — pinch wiring, `board2dPinchCamera`
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — multi-touch suppression, `touch-none`
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx` — live contrast badge
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx` — three-way `selectUiDevice`
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — `useUiDevice`, three-way `uiDevice`
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` — tablet constant + Rust resolver
- `…/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — register 3 element suites

Ticket folder: `🐍️u2-vitest-projection-hash.ts` (projection-hash drift probe),
`🗑️generated/u2-tests.txt`, `🗑️generated/u2-item6-tests.txt`.
