# Current Surface and Chrome Projection Audit

## Scope and method

This is a read-only audit of current source on 2026-09-26. I used `rg` and direct source reads only. No source, Git, generated-output, browser, native-app, or test command was changed or run. A source test or an execution command below is evidence that the path exists; it is not a claim that it passes now.

The 15-kind authority is `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`. React dispatches the same 15 kinds in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:367-433`. WGPU's production dispatch is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2980-3200`.

## Current 15-surface census

| Kinds | Current WGPU implementation and source coverage | Current executable gap |
| --- | --- | --- |
| Canvas2d | Direct retained painter; stale authority cancellation is entered at render phase zero. Dedicated WGPU test: `🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs`. The draw plugin produces this surface. | No parity probe selects a Canvas2d surface and exercises zoom, draw, transfer, or cancellation through both live renderers. |
| World3d | Direct production host through `render_world3d_surface_step`; bespoke pointer dispatch is deliberately excluded from generic routing. Source/React interaction witnesses exist for world input and residency. CAD and Puzzle3d produce this surface. | The browser parity state probe can choose any app-declared node, but has no orbit, pan, wheel, pick, or context-menu World3d scenario. |
| NodeGraph | Engine texture route; bespoke pointer dispatch. `⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` and React interaction tests cover source-level behavior. DAG and flow plugins produce it. | No live parity interaction drives wire editing, selection, pan, or wheel. |
| TextEditor | Engine texture route with retained WGPU test `🎞️Scenes/🧪️tests/🔬️wgpu-text-editor/🦀️.rs`; several plugins produce a text editor. | No dual-renderer typing, selection, completion, or clipboard scenario. |
| Table | Direct list painter with `🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs`; sourcing and other plugins produce tables. | No live sort, selection, horizontal/vertical scroll, or row activation receipt. |
| Paint2d | Engine texture route; `⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs`; raster produces it. | No live draw, marquee, camera, or navigator parity scenario. |
| VirtualFileSystem | Direct list painter and `🎞️Scenes/🧪️tests/🔬️wgpu-virtual-file-system/🦀️.rs`; space produces it. | No browser parity proof for hierarchy, navigation, selection, or scroll. |
| TiledMap | Engine texture route with engine-surface tests and React gesture-lifecycle source tests; GIS produces it. | No live map pan, zoom, selection, or tile-miss acceptance. |
| Board2d | Engine texture route with `⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs`; block produces it. | No live board drag, hover, or wheel scenario. |
| IconRender | Direct retained painter and `🎞️Scenes/🧪️tests/🔬️wgpu-icon-render/🦀️.rs`; shooting produces it. | No mounted icon-frame/pixel contract tied to a real app fixture. |
| InkCanvas | Direct retained painter and `🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs`; React has editing, interaction, and clipboard tests; note produces it. | No live parity scenario for edit, selection, clipboard, or pointer cancellation. |
| GraphTimeline | Direct list painter and `🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs`; VCS produces it. | No live scroll, branch selection, or context-menu parity scenario. |
| BlockList | Direct list painter and `🎞️Scenes/🧪️tests/🔬️wgpu-block-list/🦀️.rs`; forms/playbook produce it. | No live expansion, selection, transfer, or context-menu parity scenario. |
| DiffView | Direct list painter and `🎞️Scenes/🧪️tests/🔬️wgpu-diff-view/🦀️.rs`; split/unified paint and context-menu source tests exist. | `rg --glob '*.rs' 'SurfaceKind::DiffView' ✏️s` had no plugin producer. Add a real fixture app before claiming live product parity, then test scroll and both modes. |
| EventFeed | Direct list painter and `🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs`; row click/context-menu paths are source-covered. | `rg --glob '*.rs' 'SurfaceKind::EventFeed' ✏️s` had no plugin producer. It also has a confirmed timestamp behavior mismatch described below. |

The table deliberately does not treat an old report as a current gap. Current source has a renderer and source-level regression path for every kind. The unsatisfied full-goal evidence is the absence of a kind-specific, live dual-renderer scenario for each user-visible interaction contract.

## What the existing browser and native paths prove

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:146-205` is a real browser WGPU parity runner: it starts React and WGPU pages, boots WGPU through `#semio-wgpu-canvas` and `semioWgpuIntrospection`, compares structural dumps, compares raster regions, and runs a behavioral probe. This is important runtime infrastructure, but it is not a per-surface acceptance matrix:

- `🏗️structure/🟦️.ts:163-185` makes `componentScene` a visual leaf. Its internal text, color, and semantics are excluded from structural comparison.
- `🔬️probe/🟦️.ts:333-344` currently defines only `state` (one arbitrary eligible app node) and `shell`. Its own source identifies text-editor typing, panel DnD, and 3D orbit as future per-playground scenarios.
- The `storybook-hosts-wasm` Playwright suite boots React host stories and their WASM engines. It is useful React-host evidence, not a WGPU-renderer acceptance run.
- The browser parity owner starts browser servers and Chromium only. It does not invoke the native entrypoint, so it cannot establish native graphics, native accessibility, OS locale/time-zone behavior, or native clipboard behavior.

The next high-value acceptance work is therefore a schema-first `surface-behavior@1` fixture family, one app-backed row per produced kind. Each row should name the plugin/app fixture, surface id, deterministic initial document, input sequence, expected action/value or selected-state result, and allowed visual oracle. The runner should drive the existing React/WGPU Playwright pages through each renderer's own structural bounds, then compare the declared outcome. DiffView and EventFeed need a small fixture producer first. This belongs to the parity execution/probe owner, not a surface renderer implementation packet.

## Confirmed EventFeed locale and time-zone mismatch

React owns the behavior at `🧱️elements/📡️EventFeedHost/🟦️.tsx:30-35`: it calls `new Date(timestampMs).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" })`. Its result depends on the host locale, host time zone, and hour cycle.

WGPU instead calls `event_feed_time_of_day_utc` in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:4491-4497` and paints its result at `:4595`. The function computes a UTC modulo-day `HH:MM:SS`. Its current test at `🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs:38-40` pins that UTC behavior, so it validates the divergence rather than React parity.

WGPU's host locale door only folds a tag to `"en"` or `"de"` (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10228-10260`). It contains neither an IANA time zone nor hour-cycle data. The browser boot reads language only (`🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:40-43`). A UI language value cannot reconstruct `toLocaleTimeString` output.

The minimal domain-neutral boundary is an accepted-frame host temporal formatting port, not a Rust time-zone database or a UTC fallback:

```text
TemporalProfileV1 { locale: BCP-47, timeZone: IANA zone, hourCycle: h11|h12|h23|h24|default, revision }
EventFeedTimeFormatRequestV1 { acceptedFrame, profileRevision, timestampsMs: bounded[] }
EventFeedTimeFormatReplyV1 { acceptedFrame, profileRevision, labels: bounded[] }
```

The browser adapter uses `Intl.DateTimeFormat` with the supplied profile and the same hour/minute/second options as React. A native adapter obtains the equivalent profile and labels through the operating-system host boundary. The renderer keeps only the supplied labels for the matching accepted frame. A missing or stale reply leaves the timestamp unannounced rather than inventing UTC text.

Required tests are a neutral schema fixture around a DST boundary with an `en-US` 12-hour and `de-DE` 24-hour case; a browser `Intl` oracle; stale-revision and absent-reply rejection in the renderer; and a native-host adapter oracle. Rewrite the existing UTC-specific test rather than retaining a competing legacy path.

## Confirmed ChromeGroupItem pressed-state gap

React's `PanelTabBar` uses a native button with `aria-pressed={isActive}` because re-pressing an open pane tab folds it (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:393-435`). This is a pressed button state, not a checked switch and not tab selection.

WGPU builds a pane chip with `active: !folded` and `HitKind::Toggle` in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:19769`. It paints through `render_retained_chrome_group_item_step(..., false)` and manually adds `pane_overlay_hits` at `:19786-19794`. The painter only calls `note_chrome_control_name` when `register_hit` is true (`:17761-17764`), so that pane chip publishes neither its painted label nor its active state to the presentation map.

`chrome_accessibility_checked` at `:20348-20353` only knows widget toggle metadata and panel anchors. Pane chip ids do not reach either route. The shared `AccessibilityProjectionNode` contract (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:158-213`) has `checked` and `selected`, but no `pressed`. The browser mirror has the same omission and writes `checked` solely as `aria-checked` (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:10-85`).

The appropriate narrow fix is:

1. Add optional `pressed` to the shared Rust projection and the browser projection type. The browser mirror emits it only as `aria-pressed`.
2. Extend `ChromeControlPresentation` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:31120-31159` with `pressed`.
3. Publish `role: "button"` and `pressed: Some(item.active)` only for a `ChromeGroupItem` whose actual interaction is a toggle button. Suppress derived `checked` for a node that has this pressed presentation, so it never emits both ARIA states.
4. Call that presentation sink with the actual registered control id at manual-hit branches: pane chips (`:19769-19794`) and utility toggles (`:18328-18342`). The utility item carries a placeholder control id while the real hit key is `framework.utilityRail.toggle.{id}`, so deriving state solely inside the painter would attach it to the wrong key.
5. Keep visual `active` for regular buttons and collection rows out of `pressed`: it can mean an expanded branch or active descendant, not a toggle state. The projection menu rows instead need their existing `option` role plus `selected: Some(selected)` at their manual registration (`:19915-19945`).

Required regressions: a Rust projection test for an unfolded and folded pane chip asserting `role == "button"`, `pressed == Some(true/false)`, and `checked == None`; a browser-mirror test asserting `aria-pressed` and absence of `aria-checked`; preservation of a real switch's existing `aria-checked`; and the shared Rust/TypeScript projection-fixture equality test. No source change was made by this audit.

## Ownership boundaries

- Shell chrome presentation and the shared accessibility projection/mirror are the Shell/WGPU accessibility owner’s slice.
- EventFeed temporal formatting needs a host-I/O plus renderer slice and a later native host adapter; it should not be approximated in the Scene painter.
- Per-surface browser acceptance belongs to the parity execution/probe owner.
- Marketplace bridge work is being handled separately by the Select owner; this audit made no Marketplace change.
