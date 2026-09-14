# Example Oracle Strength Audit — 2026-09-14

READ-ONLY audit. No files other than this one were modified; no builds, no `cargo test`, no browser probes were run. All numbers below are read directly from committed fixtures and the most recent files under `🗑️generated/`.

## 0. What "the 8 examples" and "the native oracle" are

Artifact root: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`. The 8 bundled examples (directory name → picker label used by the probes):

| slug | picker label |
|---|---|
| `🪢️rectangle-wire-preview` | Rectangle Wire Preview |
| `📦️rectangle-extrude-volume` | Rectangle Extrude Volume |
| `🧹️face-sweep-extrude` | Face Sweep Extrude |
| `🍄️hexagonal-mushroom-column` | Hexagonal Mushroom Column (boot default) |
| `🐚️box-shell-preview` | Box Shell Preview |
| `📐️box-fillet-preview` | Box Fillet Preview |
| `🧲️sphere-box-fuse` | Sphere Box Fuse |
| `🍩️sphere-cut-with-torus` | Sphere Cut With Torus |

Native oracle: `📦️packages/🦀️rust/Cargo.toml:79-81` binds `[[test]] name = "example-geometry"` to `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` (943 lines) — i.e. `cargo test -p semio-s-artifact-procedural-generation3d --test example-geometry` (not run here). Each example has one `#[test]` in `assert_example` (geometry, `🦀️.rs:610-649`) and one in `assert_delivery` (published preview, `🦀️.rs:904-942`), both reading the same committed fixture `📚️examples/<slug>/🧫️fixtures/🧩️example/🔣️.json`. The fixture schema (`🦀️.rs:27-106`) has **no vertex-count field at all** — only `expect.minTriangles` (a floor), `expect.closed`, `expect.volume`±tol (cross-checked against `parry3d` at `🦀️.rs:463-473,556-563`), `expect.boundingBoxMin/Max`±tol, and `delivery.meshes` — the only field asserted as an **exact** count (`🦀️.rs:854`: `assert_eq!(run.payload_meshes, delivery.meshes, …)`).

Committed expectations, read from each `🧫️fixtures/🧩️example/🔣️.json`:

| example | `delivery.meshes` (exact) | `expect.minTriangles` (floor) | `expect.volume`±tol | `expect.boundingBoxMax` |
|---|---|---|---|---|
| rectangle-wire-preview | 1 | 0 (wire, not closed) | null (edgePerimeter=7.0±1e-6) | [2, 1.5, 0] |
| rectangle-extrude-volume | 1 | 12 | 12.0 ± 1e-4 | [2, 2, 3] |
| face-sweep-extrude | 1 | 12 | 12.0 ± 1e-4 | [2, 1.5, 4] |
| hexagonal-mushroom-column | **3** | 20 | 3.897114317029974 ± 1e-4 | [0.5, 0.433…, 6] |
| box-shell-preview | 1 | 24 | 3.903999999999999 ± 1e-3 | [2, 2, 2] |
| box-fillet-preview | 1 | 24 (delivery floor 92) | 7.888634923940582 ± 5e-3 | [2, 2, 2] |
| sphere-box-fuse | 1 | 24 (delivery floor 440) | 9.70845078963702 ± 0.02 | [1.5, 1.5, 1.5] |
| sphere-cut-with-torus | 1 | 24 (delivery floor 1114) | 37.841142613316 ± 0.05 | [2.1475, 2.1475, 2.2] |

All 8 fixtures currently declare `"kernelStatus": "green"`, and `KERNEL_STATUSES` (`🦦️.rs:152`) accepts only `"green"` — the ticket's earlier notes (`📓️example-geometry-tests-2026-09-09.md`, `📓️audit-examples-2026-09-12.md`, 2/8 passing with `blocked-on-*` statuses) describe a state the fixtures have since moved past; do not quote those older numbers as current.

**Headline finding, checked directly**: `grep -rl "minTriangles\|expect.json\|boundingBoxMin\|parry3d" <every probe .mjs/.ts in the ticket>` returns **zero files**. None of the runtime probes — React or wgpu — ever reads the committed `🧫️fixtures/🧩️example/🔣️.json`, or compares a runtime mesh/triangle/volume/bbox number against it. Every runtime "mesh count" check in this audit is an independently-invented, un-reconciled number.

## 1. Example × renderer strength table

Probe files inspected: `🐍️journey-probe.mjs` (React, all 8×edit+view), `🔍️browser-probe.ts` interact mode (React, gesture-only, no built-in assertion — the assertion lives in whichever battery calls it), `🐍️react-gap-probe.mjs` (React, single boot example, document/graph selection not 3D-canvas selection), `🐍️wgpu-battery.mjs` (wgpu orchestrator), its `examples` lane → `🐍️wgpu-example-matrix-probe.mjs` (all 8×edit+viewer), its `world3d-editor`/`world3d-viewer` lanes → `🐍️wgpu-world3d-interaction-probe.mjs` (hexagonal-mushroom-column ONLY). Latest evidence: `🗑️generated/react-verify/scoreboard.json` (2026-09-14 15:18, 16/16 green), `🗑️generated/react-u64/journey-restaged/results.json` (15:44, fixed-window, none converged), `🗑️generated/wgpu-verify/scoreboard.json` (15:32, 11/15 green).

| Example | Renderer | Probe(s) & what they assert | Measured (latest run) | Grade |
|---|---|---|---|---|
| rectangle-wire-preview | React | `journey-probe.mjs`: convergence = phase/ratio/fault only (`:67`), widget-id **exact set** match (`:84`), mesh count read but not compared | edit meshes=1 (= oracle 1, uncompared), view meshes=0 | **weak** — mesh count happens to match oracle but nothing asserts it; no hover/select/camera at all for this example |
| rectangle-wire-preview | wgpu | `wgpu-example-matrix-probe.mjs`: `scenePasses>0`, geometry-content-starts-with-digit regex (`:108-117`), `body` type detection | edit/viewer meshes=2, instances=0, body="wire" (`wgpu-verify/examples/results.json`) | **weak** — correctly flags "wire" vs "solid", but meshes=2 vs oracle `delivery.meshes=1` is unreconciled; no hover/select/camera |
| rectangle-extrude-volume | React | same as above | edit/view meshes=1 (= oracle 1, uncompared) | **weak** |
| rectangle-extrude-volume | wgpu | same as above | edit/viewer meshes=3 vs oracle 1 | **weak**, larger unreconciled gap |
| face-sweep-extrude | React | same as above | edit/view meshes=1 (= oracle 1, uncompared) | **weak** |
| face-sweep-extrude | wgpu | same as above | edit/viewer meshes=3 vs oracle 1 | **weak** |
| hexagonal-mushroom-column | React | same as above, PLUS this is the boot-default example `react-gap-probe.mjs` exercises for dock/export/inspector/i18n steps (strong, but none of those are 3D-canvas hover/select) | edit meshes=3 (= oracle 3, uncompared); **view meshes=0** | **weak**, viewer-role gap (see §2) |
| hexagonal-mushroom-column | wgpu | `wgpu-example-matrix-probe.mjs` PLUS the only per-gesture probe, `wgpu-world3d-interaction-probe.mjs` h1-h9 via `world3dVerdict()` (`🐍️wgpu-battery.mjs:378-409`): action-name-existence per gesture (hover/select/shift-add/clear/marquee/zoom/orbit/pan) | edit/viewer meshes=3 (= oracle 3, uncompared); latest `world3d-editor` run **RED**: `h7_wheel_zoom` failed — camera string unchanged after a wheel event (`wgpu-verify/world3d-editor`) | **moderate** for gesture coverage (real action-name check, only example with any), but **currently failing** on camera/zoom |
| box-shell-preview | React | same journey checks, PLUS the only React example with gesture coverage: `react-battery.mjs`'s `interact` lane (`:87-108`) runs `browser-probe.ts --steps=example,hover,select,orbit --example=box-shell-preview`; verdict is a **regex over the whole run's console log** for the strings `hoverTarget\|interactionHover`, `selectedIds\|interactionSelect`, `setCamera` (`🐍️react-battery.mjs:100-108`) | edit/view meshes=1 (= oracle 1); interact steps all `ok:true` in `react-verify/interact/` | **weak-moderate** — some interaction signal exists, but it is log-text existence over the whole run, not scoped to the step or to a target id |
| box-shell-preview | wgpu | `wgpu-example-matrix-probe.mjs` only — no per-gesture probe targets this example | edit/viewer meshes=3 vs oracle 1 | **weak**, no hover/select/camera at all |
| box-fillet-preview | React | journey checks only | edit/view meshes=1 (= oracle 1, uncompared) | **weak** |
| box-fillet-preview | wgpu | matrix probe only | edit/viewer meshes=3 vs oracle 1 | **weak**, no hover/select/camera |
| sphere-box-fuse | React | journey checks only | edit meshes=1 (= oracle 1); **view meshes=0** | **weak**, viewer-role gap |
| sphere-box-fuse | wgpu | matrix probe only | edit/viewer meshes=3 vs oracle 1 | **weak**, no hover/select/camera |
| sphere-cut-with-torus | React | journey checks only | edit meshes=1 (= oracle 1); **view meshes=0** | **weak**, viewer-role gap |
| sphere-cut-with-torus | wgpu | matrix probe only | edit/viewer meshes=3 vs oracle 1 | **weak**, no hover/select/camera |

Grade key used above: **vacuous** = gesture performed, nothing checked (this is `browser-probe.ts`'s own hover/select/orbit steps in isolation, `:416-446` — no assertion of any kind lives in that file; a grade only appears above where some *caller* layered a check on top). **weak** = an existence/`>0`/regex-text check stands in for a value comparison, and/or the number is never reconciled with the native oracle. **moderate** = a real state comparison exists (exact set-equality, before/after inequality, or a named action string) but only proves "the right kind of thing happened," not "the right target/value." No cell in this table reached **strong** (an exact runtime number checked against the committed fixture) — that comparison does not exist anywhere in the probe set.

## 2. Runtime mesh count vs. the native oracle (`delivery.meshes`, `example-geometry` fixtures — not run, read from source)

**React DOM, edit lane** (`journey-probe.mjs`/`react-verify` `data-meshes-json` length) numerically **matches** `delivery.meshes` for all 8 examples (1 for seven of them, 3 for hexagonal-mushroom-column) — but this is coincidental: nothing in `journey-probe.mjs` or `react-battery.mjs` reads the fixture to make the comparison, so a regression that changed the published count would only be caught by a human reading two unrelated JSON files side by side.

**React DOM, viewer lane** — 4 of 8 examples report **meshes=0** while still graded `converged`/`ok:true`: `view:Hexagonal Mushroom Column`, `view:Sphere Cut With Torus`, `view:Sphere Box Fuse`, `view:Rectangle Wire Preview` (`🗑️generated/react-verify/scoreboard.json`, journey step details). This is possible because `converged()` (`🐍️journey-probe.mjs:73-86`) only requires `settled()` (phase/ratio/fault, `:67`) plus (when `expectWidgets` is given) the graph widget-id set — it never requires `meshes > 0` for a non-empty example, only for the explicit "No example" case (`:79`). Whether the viewer preview is actually blank for these 4 examples, or the DOM host simply hasn't hydrated `data-meshes-json` by the time the probe samples it, is not distinguishable from this evidence — either way it is unasserted.

**wgpu, both lanes** — every non-hex example reports meshes = 3 (solid) or 2 (wire) against an oracle `delivery.meshes` of 1, a consistent unreconciled gap (`🗑️generated/wgpu-verify/examples/results.json`). `hexagonal-mushroom-column` alone reads 3 = 3, but nothing confirms this is the same 3 meshes the oracle means (column + 2 caps) rather than a coincidental (solid + wire-companion + 1) on the wgpu side. `wgpu-battery.mjs:109-112`'s own comment ("generation3d always ships its `@wire#0` companions") documents *some* structural over-count is expected, but no probe subtracts or labels the companion entries before comparing, and no fixture states what the wgpu-side raw count *should* be.

## 3. User-visible behaviours never asserted, per example

Across all 8 examples, in both renderers, none of the six probe files (nor `wgpu-battery.mjs`'s aggregation of their output) ever assert:
- **Hover highlight** — visual styling of the hovered mesh (colour/outline). Only "did an `interactionHover`-shaped action fire" is checked (`🐍️wgpu-battery.mjs:382-387`, `react-battery.mjs:100`), and only for hexagonal-mushroom-column (wgpu) and box-shell-preview (React) — the other 7 examples per renderer get **zero** hover coverage of any kind.
- **Hover TARGET identity** — that the hovered mesh/part id matches what a screen-center ray over that example's geometry should hit. `step.authorityAfter?.hover` is captured into `detail` at `🐍️wgpu-battery.mjs:386` but never compared to an expected id.
- **Selection visual outline / gumball appearance** after a click.
- **Inspector values after a 3D-canvas selection.** `react-gap-probe.mjs`'s `inspection-edit`/`doc-panel-select` steps (`:211-310`) do assert inspector-value round trips, but only after selecting from the **Document tree panel**, never after clicking a mesh in the World3d canvas — no probe connects a canvas click to inspector content for any example.
- **Selection round-trip correctness** (does clicking mesh A then mesh B leave exactly B selected, not both) — `wgpu-world3d-interaction-probe.mjs` h4's shift-add step explicitly accepts either an `interactionSelect` OR a `translateSelection` (gumball) action as passing (`🐍️wgpu-battery.mjs:396-399`), which can mask a broken additive-select as a gumball hit.
- **Undo of a selection or transform** made via the 3D canvas. `react-gap-probe.mjs`'s `wire-undo` step (`:181-209`) is a real, strong exact-round-trip assertion, but it undoes a **graph wire cut**, not a 3D interaction.
- **Camera-fit-to-example** (does the camera frame the example's own bounding box on load/switch). No file in this set asserts camera position/target/zoom against a value — `wgpu-journey-probe.mjs:544-566` only checks the camera JSON string is *different* before/after a wheel event, and the one probe with per-example camera gestures (`wgpu-world3d-interaction-probe.mjs` h7-h9) only checks that a `setCamera` action fired — and as of the latest run, `h7_wheel_zoom` is failing outright for the one example it covers.
- **Exact vertex/face counts vs. the native oracle**, for any example — the oracle itself never commits an exact vertex count either (only a triangle floor), so this gap exists on both sides.
- **The published volume/geometry numbers** (`expect.volume`, `delivery.meshes`) reaching any UI surface the user reads (e.g., an inspector "volume" field) — never checked anywhere at runtime.

## 4. Prioritized oracle upgrades

1. **Cross-reference the committed fixture in every mesh-count check (both renderers, all 8 examples).** Load `📚️examples/<slug>/🧫️fixtures/🧩️example/🔣️.json` from `journey-probe.mjs`'s `EXPECTED_WIDGETS` map (`:46-56`, add a sibling `EXPECTED_MESHES`) and from `wgpu-example-matrix-probe.mjs`'s per-row `measure()` (~`:155-185`), and assert `meshes === fixture.delivery.meshes` (React: exact, since the DOM count already matches) or, for wgpu, first reconcile the companion-mesh delta (see item 5) then assert exact. No new product export needed for React (`data-meshes-json` already carries per-mesh `data.indices`/`data.positions`, so a triangle-count check is also free); wgpu needs a `dumpMeshStats(windowId)` export beside the existing `dumpFrameStats`/`dumpStructure` (`globalThis.semioWgpuIntrospection`, used at `wgpu-example-matrix-probe.mjs:130-145`, `wgpu-journey-probe.mjs:76`) that returns per-mesh `{role, indices.length, positions.length}` instead of the current regex-over-console-text "does `positions` start with a digit" check (`wgpu-example-matrix-probe.mjs:108-117`).

2. **Extend hover/select/camera coverage from 1 example per renderer to all 8.** wgpu's `world3d-editor`/`world3d-viewer` lanes hardcode `example=hexagonal-mushroom-column` (`🐍️wgpu-battery.mjs:186,191`); React's `interact` lane hardcodes `--example=box-shell-preview` (`🐍️react-battery.mjs:87-88`). Parameterize both the way `wgpu-example-matrix-probe.mjs` already loops `EXAMPLES` (`🐍️wgpu-battery.mjs:47,80`), and run `world3dVerdict()`/the React regex-verdict once per example.

3. **Turn hover/select from action-name-existence into target-id value comparison.** `world3dVerdict`'s `acted()` helper (`🐍️wgpu-battery.mjs:382-387`) only checks `actions.some(a => a === needle)`; it already reads `step.authorityAfter?.hover`/`.camera` into `detail` but never asserts against an expected value. Needs: (a) the probe to derive an expected hovered/selected id from the example's own geometry (e.g. via the already-read `dumpStructure(windowId)` retained-node `mounted_layout` rects, `wgpu-journey-probe.mjs:76,520`), and (b) `assert authorityAfter.hover === expectedId`. On the React side, `data-interaction-json` is declared and captured (`browser-probe.ts:236`) but never read for its content by any verdict logic — document its schema (`{hoverTargetId, selectedIds, camera}`) and have `react-battery.mjs`'s verdict (`:100-108`) read that attribute instead of regexing the whole run's console log for the strings `"hoverTarget"`/`"interactionHover"`/`"setCamera"`.

4. **Fix the React viewer-role mesh gap before trusting `converged`.** `journey-probe.mjs`'s `converged()` (`:73-86`) never requires `meshes > 0` for a non-empty example — only the "No example" row is required to be exactly 0 (`:79`). Add a symmetric floor (`previews.every(h => h.meshes > 0)` whenever `expectWidgets.length > 0`) so the 4 currently-passing-with-0-meshes viewer steps (`view:Hexagonal Mushroom Column`, `view:Sphere Cut With Torus`, `view:Sphere Box Fuse`, `view:Rectangle Wire Preview` in `🗑️generated/react-verify/scoreboard.json`) either genuinely converge or fail loudly.

5. **Reconcile the wgpu mesh-count metric with `delivery.meshes` before comparing them at all.** Every non-hex wgpu example currently reads meshes=3 (solid) or 2 (wire) against an oracle of 1 (§2). Either the fixture needs a wgpu-side expected count that accounts for the companion wire mesh (`wgpu-battery.mjs:109-112`'s own comment), or the product needs to label each published mesh's role so a probe can filter to "solid" before comparing — otherwise item 1's cross-reference will either false-fail on every wgpu example or have to special-case a fudge factor nobody has verified.

6. **Give the wgpu camera/zoom check a real assertion and fix the currently-red gesture.** `h7_wheel_zoom` is failing in the latest `world3d-editor` run (camera string unchanged after a wheel event, `wgpu-verify/world3d-editor`) — this is a live break in the one example that gets any camera coverage at all, and should be triaged before the coverage is widened per item 2. Longer-term, extend the `camera="…"` console line / `authorityAfter.camera` payload with a `fitBoundsMin/Max`, so a probe can assert the camera actually frames the example's committed `expect.boundingBoxMin/Max` from the same fixture the native oracle reads — today only "the string changed" is ever checked (`wgpu-journey-probe.mjs:544-566`), never "changed to something correct."

7. **Loosen or remove the h4 gumball escape hatch once item 3 lands.** `🐍️wgpu-battery.mjs:396-399` treats a `translateSelection` action as an equally-valid pass for the "shift-click adds to selection" gesture, which can hide a broken additive-select behind a coincidental gumball hit. Once hover/select assert target ids (item 3), this hop should require the actual selection set to have grown, regardless of which action name carried it.
