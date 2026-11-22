# Opus 5.5 Coordination 2026-11-22

Coordinator: Opus 5.5 main session. Executors: Sonnet 5.5. Auditors: Haiku 5.5. The repo MCP is not attached, so ticket bookkeeping is manual.

## Findings

- Production wgpu paints only through the retained `semio-ui-wgpu-paint`, `🔘️controls` and `🧰️kit`.
- The immediate `🪀️widgets` `render_*` path, the hosting `shell`/`host`/`scene_slots`/`accessibility` modules, the light `minimap`/`presence_bar`/`tabs` modules and the `🖌️render/🎯️targets/*` backends are dead. Removal is in progress; the `🖌️render` backends are left to their owners.
- The ui wgpu crates and their law crates are members of `🧰️framework/Cargo.toml`. Run them with `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p <crate>`.
- Concurrent parity work pushed `semio-ui-wgpu-paint` and `semio-ui-wgpu-events` past the 4000 active-line source boundary. Both are being carved into in-place sub-crates:
  - `semio-ui-wgpu-paint-widgets`
  - `semio-ui-wgpu-paint-sync`
  - `semio-ui-wgpu-events-commit`
  - `semio-ui-wgpu-events-focus`

  Paint's cap must go back from 4300 to 4000 afterwards.
- Window layout exists in four shapes: the contract, the declaration, the wgpu DockNode and React's TS copy. The contract `WindowLayout` is the chosen single model, with a schema and a corpus, see `📓️opus55-window-layout-unification.md`. Migrating both consumers is the next wave.
- The host has 16 GB of RAM, and parallel cargo runs push it into heavy swap. Builds, not agents, are the throughput limit.

## Audits

- `📓️opus55-audit-basic-widgets.md`
- `📓️opus55-audit-structure-overlays.md`
- `📓️opus55-audit-shell-chrome.md`: sampled. Its sign-in P0 was false, since a 760-line wgpu HubSignIn exists.
- `📓️opus55-audit-os-elements.md`: several low-confidence verdicts were disproved, for example terrain and the media host.

## Workstreams

| Stream | Report |
| --- | --- |
| Hub workspace first-run, replay, delegations | `📓️opus55-hub-workspace-first-run.md` |
| Ring, Slider, IconSelect | `📓️opus55-fix-ring-slider-iconselect.md` |
| Text input: selection, IME, multi-line, refusal | `📓️opus55-fix-text-input.md` |
| Select segmented, Button labels, Image, i18n | `📓️opus55-fix-select-button-image.md` |
| Tree keyboard, header, guides | `📓️opus55-fix-tree.md` |
| Table columns, keys, sort, empty state | `📓️opus55-fix-table.md` |
| Overlays: glass, focus return, placement, scrollbars, splitter | `📓️opus55-fix-overlays.md` |
| Shell panels: TaskManager, UtilityTree, ChromePanels | `📓️opus55-fix-shell-panels.md` |
| Scene hosts: Paint2d, BlockList, TiledMap | `📓️opus55-fix-scene-hosts.md` |
| TextEditor, World3d, media | `📓️opus55-fix-editor-world-media.md` |
| Dead code removal and engine cell | `📓️opus55-dead-code-removal.md` |
| Crate carve | `📓️opus55-crate-carve.md` |
| Window layout unification | `📓️opus55-window-layout-unification.md` |

## Status at 14:10 CET

- **Waves:** wave 1 (13 streams), the chrome gaps, chrome motion and tutorial, wave 2 (input, render and features) and the window-layout migration (contract model, React, Rust) are all written. The archivist commit `01324745f80` includes them.
- **Lane A (ui wgpu):** all 25 crates compile with `--tests`, and the law crates are green: engine 72, engine-scene 21, events 115, events-control 14, paint 108, layout 49, layout-laws 29, shader-contract, device, wasm check. The two exceptions are retained-clone tests, which are blocked by another session's in-flight `🧵️job` retained-charge work.
- **Lane C (TS):** ui-react and framework typecheck green. renderer-react has 34 errors, all from Rust-generated files that lane B regenerates. Playwright Chromium is not installed, so the browser tests cannot run (environment).
- **Lane D (layout migration):** green on contract presentation 43, layout 31, dock-laws 75 (corpus and validator) and TUI (unit-laws 103). Dashboard terminal is 89/91.
- **Lane B (OS renderer):** shell-actions blockers are fixed. Results so far:
  - native-probe ported to the store snapshot API, schema-first.
  - hub-workspace 75 and hub-first-run 13 pass.
  - The shell test crates are being re-run after compile fixes.
  - input-2 has an arena lifetime fault, now owned by a separate stream.
  - An `asset-decode-jobs` resume race is being fixed.
- **Blockers:**
  - At 14:00 the Sonnet weekly limit was reached and resets at 16:00 Berlin, so streams in flight were cut off.
  - `semio-os-store-engine` source cap overrun from another session's in-flight carve.
- **Pre-existing defect:** the `semio-ui-contract` test unit exceeds its 4000 active-line cap at HEAD, so its tests cannot run. Fix by moving in-crate test modules into a companion laws crate.

## Next Waves

1. Green builds, then run every law crate for the touched crates.
2. Migrate React and wgpu onto the contract window layout and delete the redundant models.
3. World3d point-cloud pipeline.
4. Paired runtime parity, `repo run @semio-tech/framework-os-dev:parity -- verify <variant>` per playground, once the host is quiet.

## Paused 2026-11-22 ~17:00 CET (user request)

**Written since 14:10, code-only and uncompiled unless noted:**
- **Wave 3 reports:** tree-drag, table-inputs, extension-a11y-groups, chrome-a, chrome-b, video-media, helpers-runtime and editor-submenu.
- **Wave 4 reports:** browser-import-menus, native-media-runtime, extension-invoke-caps and local-files-band.
- **Fixes, each in its `📓️opus55-*.md` file or integration log section:**
  - asset-decode receipt fix
  - ui arena reentrant fixture
  - 8 drifted source-text anchors
  - frame-job fault-naming production bug
  - tree drop-binding production bug
  - browser extension capability scoping (security)

**Verified:**
- `semio-ui-contract` testkit seam and companion laws crate: the contract is under 3300 lines, laws 18 and conformance 43 pass.
- Lane E: the combined `check --tests` over all 25 UI wgpu packages, including paint-overlay, is green.
- Lane B: hub-workspace 75/75 and hub-first-run 13/13 pass.

**Resume in this order:**
1. **Lane E step 2:** combined OS-crate `check --tests` with no timeout wrapper. Framework-workspace crates (deflate, raster, paint-overlay) need `--manifest-path 🧰️framework/Cargo.toml`. Then step 3 (laws/tests), step 4 (`semio-renderer-wgpu --features native-bin`, `semio-wgpu-native`), and `@semio-tech/s-dev-composition:generate-stdio-format-claims` plus its check, with no timeout because it planned for more than 25 minutes. See "## Lane E" in `📓️opus55-integration.md`.
2. **Lane B:** first re-check the chrome-1 failure in `semio-renderer-wgpu-shell-plugins` (`cannot find module semio_ui_retained_tree`, likely another session's concurrent edit). Then re-run the shell test crates one at a time, with docs-3, windows-1/2 never run yet and input-2 last. Then the renderer native and wasm32 checks (the browser worker wire was never compiled for wasm32), `semio-wgpu-native`, the scene/element tests and the shell TS generator. See "## Lane B".
3. **Runtime parity:** the paired verify (`repo run @semio-tech/framework-os-dev:parity -- verify <variant>`) needs Playwright Chromium. That is pending the user's permission to download it.
4. **Re-audit:** after green, run a Haiku re-audit against React (20 components, chrome, OS elements), then fix the remaining gaps listed at the end of each wave report.

**Build policy that worked:** writers only write code; one or two integration lanes compile using combined `-p` invocations. With more parallel builds the 16 GB host thrashes.
