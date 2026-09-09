# Exact Window Configuration and Jack Ownership

## Boundary

Persisted local view configuration is partitioned by concrete window instance. The framework owns the heterogeneous `WindowConfigOwnerRegistry`, one typed store and generation authority per `(window_instance_id, window_kind_id)`, and addressed mutation envelopes. Plugin apps register the window kinds they own and publish through the window configuration lane. App configuration no longer serializes a keyed approximation of window state.

`ConfigView.window` carries the exact typed window snapshot selected from the trusted `ViewModel` projection. Rendering, context menus, measures, and engagements consume this projection. Measures and engagements are evaluated once per bounded live window instance and returned under the concrete instance ID, so sibling windows of one kind cannot overwrite each other.

Window configuration packs cross the existing app channel. Channel version 15 defines `LoadWindowConfig`, `ReadWindowConfigs`, and `WindowConfigs`; Rust, TypeScript, React, and WGPU use the same address and bytes. Reads are paged, loads replay the stored mutation log into the matching typed owner, and owner close retires the partition independently of document, app configuration, and app transient state.

## Jack

Jack application configuration now contains only the authored shared query. Graph camera overrides and LOD choice live in `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config`, whose schema is `trinity.jackgraphwindowcfg`. The owner accepts only the Jack graph window kind.

`setViewport` and `setLodMode` obtain the exact address from the command's trusted `ViewModel`; neither payload carries a window selector. They publish `WindowConfig` mutations and leave the document and Jack application configuration unchanged. Rendering falls back to the document fixture camera when that concrete window has no override. Window measures read the exact addressed LOD and are exposed under the concrete instance ID.

The Jack editor caret remains ephemeral local state in the editor window transient owner. Query source is shared authored application configuration, while query execution and typed results remain application transient state shared by the runner and results views.

## Language-neutral contracts

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/🪟️window-config.json` defines the Rust/TypeScript channel vectors.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🔣️.json` defines independent camera and LOD traces for two concrete windows.
- The TypeScript oracle validates the Jack schema with Ajv, applies the trace with `fast-json-patch`, and asserts that the application config schema has no camera or LOD property.

## Validation

- `abstraction-ownership-validation:window-config-channel-native`: passed, 1 Rust test.
- `abstraction-ownership-validation:window-config-channel-typescript`: passed, 2 TypeScript tests with 313 unrelated tests skipped.
- `abstraction-ownership-validation:jack-window-config-oracle`: passed with Ajv and `fast-json-patch`.
- `abstraction-ownership-validation:jack-check`: passed `cargo check semio-s-artifact-trinity-jack` in 1 minute 38 seconds.
- `abstraction-ownership-validation:enforce`: passed 11 ownership vectors, 4 nested-schema vectors, 3 command-source vectors, and 104 document-only artifact contracts with zero misplaced declarations.
- `abstraction-ownership-validation:jack-window-config-native`: running at the time of this report update; no pass is claimed.
- `abstraction-ownership-validation:jack-query-ownership-test`: not rerun after the window configuration migration yet.

## Writer consumer trace

Writer has no page, print, or export preference in `WriterConfig`; media export reads only the document. The existing fields are all editor runtime state:

- `camera` is produced by `SetCamera` and consumed by the main text-editor scene plus the inspection panel. It belongs to the concrete main editor window configuration; inspection must receive an explicit inspected-window address rather than silently treating app config as the active editor.
- `editor_settings` contains line-number visibility, font size, line height, and tab size. The main editor scene, its measures, engagement option, and option components consume it. It belongs to the concrete main editor window configuration.
- `editor_selection` is produced by text selection, consumed by the main scene, rename commit, completions, and presence projection. It belongs to the concrete main editor window transient partition.
- `engagement_input` is an in-progress engagement-bar draft consumed by the addressed window engagement and submit action. It belongs to the concrete main editor window transient partition.
- `format_signal` and `lint_signal` are computed invalidation counters. `lint_signal` changes diagnostics; `format_signal` has no document ownership. These should become typed computed transient events. If one result is intended to serve every sibling view, it can remain application transient; otherwise the command's exact window address must select the window transient owner.
- `revision` is only a blanket mutation counter and is not a domain value. It should be removed when the concrete owners provide their own generations.

The Writer testkit now supplies a real `writer-main-test` window instance and asserts measures and engagements by that exact ID. This repairs the tests for the framework's concrete-window iteration without prematurely migrating Writer storage.
