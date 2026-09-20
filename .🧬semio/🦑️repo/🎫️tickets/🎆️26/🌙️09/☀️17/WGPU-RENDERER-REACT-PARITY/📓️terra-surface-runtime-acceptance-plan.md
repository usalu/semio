# WGPU Surface Runtime Acceptance Plan

**Scope.** Source-only inventory for the 15 contract `SurfaceKind` variants at `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`. No renderer, browser, Cargo, or Nx command was started for this audit.

This supersedes the two implementation-status claims in [the prior route audit](./📓️terra-surface-runtime-routes.md): Layout now owns a typed catalogue-drop decoder, and Ink now has clipboard/cancellation paths. Those are no longer reasons to exclude either family from runtime acceptance.

## What Existing Probes Can Prove

`🐍️parity-interact-probe.mjs` is a Puzzle3d/World3d probe. Its default URLs select `?plugin=puzzle3d` (`:25-50`); its WGPU `surfacePoint()` filters for `control.kind === "World3d"` and errors if none exists (`:627-632`); and its five scene operations are World3d orbit, pan, wheel zoom, instance pick, and context menu (`:708-713`). It can still check shared chrome with scene steps excluded, but cannot accept a Board, map, Paint2d, TextEditor, Table, or feed without a per-family scene adapter.

`🔬️canvas-interactions/📜️script.ts` has only Draw and Note targets (`:51-85`). Note performs real clipboard/cancellation operations (`runNote`, `:451-461`). Draw currently verifies that its exact Canvas2d target mounted, then returns cases marked `ready-for-physical-calibration` (`runDraw`, `:464-468`); it does not yet execute the five fixture gestures. Thus a green Draw receipt would presently prove discovery only.

The native render-entry test treats World3d, NodeGraph, TiledMap, and Board2d as direct pointer families; Canvas2d, Paint2d, TextEditor, InkCanvas, GraphTimeline, Table, VirtualFileSystem, DiffView, and EventFeed use generic scene routing (`🧪️tests/🔬️wgpu-render-entry/🦀️.rs:4-10`). That is useful renderer coverage, but does not prove an activated product app supplied a populated scene or that its domain action reached the guest.

## Canonical Draw, Layout, and Note Activation Targets

The activation router constructs `@semio-tech/framework-os-dev:activate-<variant>-<renderer>-<profile>` (`🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:124-136`). The exact targets for a later controlled dual-renderer run are:

```text
@semio-tech/framework-os-dev:activate-draw-react-dev
@semio-tech/framework-os-dev:activate-draw-wgpu-dev
@semio-tech/framework-os-dev:activate-layout-react-dev
@semio-tech/framework-os-dev:activate-layout-wgpu-dev
@semio-tech/framework-os-dev:activate-note-react-dev
@semio-tech/framework-os-dev:activate-note-wgpu-dev
```

These resolve to Draw `s.draw.drawing@1/*#editor` (`🖍️draw/📦️packages/🦀️rust/Cargo.toml:18-19`), Layout `s.layout.layout@1/*#editor` (`📏️layout/📦️packages/🦀️rust/Cargo.toml:18-19`), and Note `s.note.note@1/*#editor` (`🗒️note/📦️packages/🦀️rust/Cargo.toml:18-19`). No target was run here.

## Current Application Acceptance Matrix

| Contract family | Real registered specimen and route evidence | Existing physical interaction / next minimum acceptance |
|---|---|---|
| Canvas2d | Draw canvas (`🖍️draw/.../✏️editor/🦀️.rs:1790`) is the intended `drawing.play.composite` route. Layout Blueprint is a second Canvas2d specimen (`📏️layout/.../📐️blueprint/🦀️.rs:25,45`). | Draw fixture names draw, select, modifiers, Escape, and double-click, but executes none. Add one calibrated draw plus Escape/non-commit receipt. For Layout, drag each supported catalogue kind, assert preview then one creation and terminal preview retirement. |
| World3d | Puzzle3d, `s.puzzle.puzzle3d@1/*#editor` (`🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:37-38`). | The existing generic probe already supplies the five World3d gestures; retain it as this family's acceptance route. |
| NodeGraph | Flow main is an editable `NodeGraphScene` (`🌊️flow/.../🪟️windows/🌊️main/🦀️.rs:13-115`), activated by `activate-flow-<renderer>-dev`. | No physical Flow adapter exists. Add a Flow-scoped pick/drag or connection attempt with an application-visible consequence; generic World3d coordinates are invalid here. |
| TextEditor | Writer main declares TextEditor (`✒️writer/.../🪟️windows/✒️main/🦀️.rs:34,110`), variant `writer`; Imperative script is a second code-like TextEditor (`📜️imperative/.../🪟️windows/📝️script/🦀️.rs:19`). | No current physical probe. Add focus, text insertion, selection/copy, and Escape/cancel checks against one populated Writer or Imperative document. |
| Table | Imperative main declares a populated Table (`📜️imperative/.../🪟️windows/📋️main/🦀️.rs:21`), variant `imperative`. | No current physical probe. Assert a known row is visible, then row focus/selection and one keyboard move. |
| Paint2d | Raster composite is Paint2d (`🖨️raster/.../🪟️windows/🖼️composite/🦀️.rs:1-46`), variant `raster`, app `s.raster.raster@1/*#editor`. | No current physical probe. Add a stroke or tool gesture with a paint-model/raster consequence and a cancellation case. |
| VirtualFileSystem | Space Media VFS declares VirtualFileSystem (`🪐️space/.../🪟️windows/🗂️media-vfs/🦀️.rs:21,67`), variant `space`, app `s.space.space@1/*#editor`. | No physical probe. This implementation currently exposes only the root through a no-op `flatten_media_vfs_rows`; acceptance may assert root rendering and focus only, not recursive file operations. |
| TiledMap | GIS map declares and renders TiledMap (`🌍️gis/.../🪟️windows/🗺️map/🦀️.rs:25,95`). | No physical probe. Add hover/pick and wheel/pan against a seeded GIS document; generic World3d selection cannot substitute. |
| Board2d | Puzzle2d Overview declares Board2d (`🧩️puzzle/.../◻️2d/.../🪟️windows/👁️overview/🦀️.rs:11-42`), app `s.puzzle.puzzle2d@1/*#editor`. | No physical probe. Add the app's selection/brush gesture and an observed selection or document result. |
| IconRender | Shooting Icon declares IconRender (`🎥️shooting/.../🪟️windows/🖼️icon/🦀️.rs:1-73`), variant `shooting`. | No physical probe. This is principally visual acceptance: seed an icon request, assert mounted kind and nonempty rendered icon bounds/pixels. |
| InkCanvas | Note composite is InkCanvas. Clipboard schema/fixture and mounted React oracle are in `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🖋️ink-clipboard/🔣️.json` and `🧑‍🎨engine/🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx`. | `canvas-interactions` already exercises Note copy/paste, editor priority, and trusted pointer cancellation. Re-run it after each renderer activation; it is current source coverage, not an implementation omission. |
| GraphTimeline | VCS history renders GraphTimeline (`🌿️vcs/.../🪟️windows/📜️history/🦀️.rs:19,39`), variant `vcs`. | No physical probe. Seed history, assert a graph item, then test item focus/selection and wheel/keyboard navigation. |
| BlockList | Forms builder renders BlockList (`📋️forms/.../🪟️windows/🧱️builder/🦀️.rs:20,44`), app `s.forms.forms@1/*#editor`. | No physical probe. Seed a builder list, assert a named block, then selection/reorder or palette action with an application receipt. |
| DiffView | No registered plugin producer was found for `SurfaceKind::DiffView`; source references are renderer contract/reconcile/paint and its renderer tests. | There is no real application activation specimen. Do not report physical coverage from the renderer unit test; add a registered app/fixture first. |
| EventFeed | No registered plugin producer was found for `SurfaceKind::EventFeed`; source references are renderer contract/reconcile/paint and its renderer tests. | There is no real application activation specimen. Add an app/fixture before scheduling physical acceptance. |

`Code` and `Terminal` are not members of the 15-variant contract. Code-like acceptance belongs under TextEditor using Writer or Imperative Script only after the selected window is verified as `TextEditor`. This inventory found no registered Terminal surface route, so it cannot be counted as a substitute for any contract family.

## Corrected Layout and Ink Boundaries

Layout's `canvasDrop` route now decodes the catalogue MIME witness and strict raw JSON in `catalogue_args` (`📏️layout/.../✏️editor/🦀️.rs:364,482-508`) before addressing the Blueprint Canvas2d (`:638-643,854`). The remaining runtime question is end-to-end delivery and terminal preview cleanup, not absence of a decoder.

Ink has the clipboard contract, neutral fixture, React mounted oracle, and the Note physical probe described above. The remaining acceptance is a fresh React/WGPU Note receipt, not the stale claim that WGPU has no clipboard/cancellation route.

## Package Helper Source Review

The current package-integration helper has a real Bun child oracle (`🧪️tests/🧩️package-integration/🟦️.ts:25-28`), independently checks catalog schema/WebCrypto digest and target drift (`:343-359`), and exercises caller-CWD preservation under three environments (`:418-430`). The parent-reported 23/23 result was not rerun. This source review found no separate actionable defect in the helper.

**Confidence:** high for probe limits, registered-kind inventory, and Draw/Layout/Note status; high for the absent DiffView/EventFeed product specimens because the inventory returned only framework/renderer references; medium for the individual future interaction choices, which must be calibrated against a seeded app document when their adapter is added.
