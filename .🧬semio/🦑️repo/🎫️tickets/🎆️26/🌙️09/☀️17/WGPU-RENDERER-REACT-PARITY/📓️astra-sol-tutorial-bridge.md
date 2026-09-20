# Astra Sol Tutorial Bridge

Source-stable implementation report, 2026-09-20. This packet implements findings 1–4 and their acceptance laws from `📓️astra-terra-tutorial-theme.md`. It does not claim a native or browser runtime verdict; the root agent owns the current native renderer census, WGPU build, activation, and live probe.

## Neutral contract and oracle

- Added `engine/🧬️schema/🎥️tutorial-bridge/🔣️.json` and `engine/🧪️fixtures/🎥️tutorial-bridge/🔣️.json` before production repair. The fixture carries two non-empty selection domains, two expanded tree ids, a declared dialog id, command-search false, all eight concrete anchor keys, a fully mutated state, sparse close/open deltas, and the five semantic point kinds.
- Added `engine/🧪️tests/🎥️tutorial-bridge/🟦️.ts` to validate the fixture with Ajv, reconstruct it with the existing React apply/capture bridge, apply the shared `composeTutorialUi` delta fold, and compare an independent `fast-json-patch` oracle.
- The first correct focused run was red because both shared Rust and React core folds retained an empty selection domain. `apply_tutorial_ui_change` and `applyTutorialUiChange` now remove a domain when `ids` is empty, matching the manifest contract and the React shell apply behavior.
- The final focused browser-worker command passed 9 files and 96 tests. The exact command and result are recorded in `🗑️generated/astra-tutorial/validation.json`.

## Renderer-owned state bridge

- `ShellState.interaction_selection` is the typed `HashMap<String, DomainSelection>` projection. Both normal action and command dispatches observe the folded `InvocationResult.output.interactionView.selection` after the real completion lane has been combined by `ProgramBridge`; `selectionCleared` clears the projection. Tutorial apply uses the same `set_interaction_selection_projection` seam and never synthesizes pointer input.
- `ShellState.tree_open_states: BTreeMap<String, bool>` is the canonical expansion state. Generic section and tree chevrons update it, the theme editor reads it through `canonical_tree_open`, and tutorial capture emits sorted true ids. Apply records previously known omitted ids as false and mirrors the values into the older widget-renderer `collapsed_sections` keys during the transition.
- Tutorial panel capture and apply iterate `PanelAnchor::ALL` and use `anchor.as_str()`. Apply reconciles the requested path against that exact anchor and never calls the global `reveal_dock_tab` search.
- `Effect::OpenDialog` now uses production `chrome_dialog_request`, which resolves the active locale and terminology from the declared `AppDefinition.dialogs` entry and preserves pre-seeded args in its action descriptor. Tutorial restoration calls the same constructor, clears the stack for absent or unknown ids, and restores a known id.
- `command_panel_open` now captures and applies `search_open`/`OverlayState::Search` in both directions. It no longer aliases the bottom-middle Command dock category.

## Typed semantic point resolver

- `EngineCanvas::resolve_tutorial_surface_point` is the typed surface-registry boundary. It reads the live owner currently attached under the requested surface id and returns surface-local pixels; Shell only adds the current window-content rect origin.
- Node graph hosts resolve Canvas through their live DAG camera, Entity through `entity_screen_json`, Curve through arc-length interpolation over the retained edge polyline, and Domain through the real slider overlay track and min/max domain.
- Tiled maps resolve Canvas and retained position/route Entity geometry; route geometry now also publishes its projected polyline for Curve.
- Board, editor, and raster surfaces resolve Canvas with their own live camera methods. Their intentionally unsupported semantic entity kinds still return `None`, matching the current React registrations.
- World3d resolves Scene and Canvas through the live orbit camera, vortex/object Entity points through retained host geometry, and attraction Entity/Curve points through retained endpoints. The previously discarded optional attraction id is retained for exact semantic addressing. A 2D surface intentionally returns `None` for Scene; World3d and the present React World resolver intentionally expose no Domain track.

## Rust laws authored

- `wgpu-tutorial` now covers two-domain selection, two canonical tree ids, all eight anchors, command true-to-false closure, selection/tree/panel closure, normal completion projection, and detached semantic targets.
- `appearance-tour-and-footer-pills` proves a known dialog restores through the declared dialog constructor, closes when absent, and shares construction with `Effect::OpenDialog`.
- `wgpu-node-graph` attaches the real seven-widget Flow fixture and proves Canvas, node Entity with offset, edge Curve, and slider Domain targets resolve inside the live surface, while Scene is intentionally absent.
- All seven changed Rust source files parse through `rustfmt --emit stdout`. These laws were not executed by this agent because the root agent owns Cargo/native builds.

## Review boundary

No legacy snapshot grammar, runtime dependency, compatibility path, script, navbar/footer geometry, shader, material, or renderer activation was added. The current source has no blanket semantic-variant `None` arm: absence is decided by the detached surface, missing entity, or capability of the attached host.
