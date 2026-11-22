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

## Next Waves

1. Green builds, then run every law crate for the touched crates.
2. Migrate React and wgpu onto the contract window layout and delete the redundant models.
3. World3d point-cloud pipeline.
4. Paired runtime parity, `repo run @semio-tech/framework-os-dev:parity -- verify <variant>` per playground, once the host is quiet.
