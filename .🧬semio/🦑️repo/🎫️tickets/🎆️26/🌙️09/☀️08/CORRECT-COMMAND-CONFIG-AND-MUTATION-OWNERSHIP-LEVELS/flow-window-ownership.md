# Flow Window Ownership

## Result

Flow document state now contains only the ordered shared content children. Camera and editor settings are owned by the exact concrete `flow-main` window config. Generation draft input and duplicate progress are owned by the exact invoking window transient for `flow-main`, `flow-generations`, `flow-generation-form`, and `flow-generation-preview`.

Commands derive concrete owner IDs from the trusted `ViewModel`. Addressed config and transient reads reject stale IDs and wrong window kinds. Generation has no implicit fallback to the main window. Host contributions are read from host-owned request context and are absent from document/config/transient persistence.

All twelve non-Rust artifact, snapshot, and diff facets use the shared `ArtifactChild` identity shape and omit camera, inline widgets, inline synapses, and inline layout. The Rust child metadata uses canonical artifact kind `s.stdio.semio`; the target subset remains `flow` in the child dialect.

The former app config owner, retained app-config copy, `SetContributions`, and kind-wide `DuplicateWidgetStep` worker were removed. Duplicate-widget execution now validates the selected Flow child dialect, reads its owned child payload, emits child mutations, and stores only resumable scalar progress in the exact window transient.

## Validation

- The registered Nx oracle target `workspace:flow-window-ownership-oracle` passed and emitted `[DEBUG] flow-window-ownership windows=2 transients=2 documentBytes=206 staleAndWrongKind=rejected`.
- A direct repeat through the same permanent ticket script passed with the same trace. Output: `🗑️generated/flow-window-ownership-oracle.log`.
- Ajv validates the language-neutral fixture against the concrete config/transient schemas. `fast-json-patch` independently applies the mutations and confirms two same-kind windows retain distinct settings, config survives same-byte document reload, transient clears, document bytes remain unchanged, and stale/wrong-kind targets are rejected.
- The obsolete-symbol audit found zero `FlowConfig`, `FlowConfigMutation`, `SetContributions`, `DuplicateWidgetStep`, or `flow::config` references in Flow Rust/TypeScript/JSON/GraphQL/Proto/TOML sources.
- The first native target invocation failed before compilation because the validation target requested a nonexistent Flow feature `component-app-assembly`. The permanent validation script now invokes the featureless Flow package correctly.
- The ticket-project invocation waited for graph construction and then reported `Cannot find project 'abstraction-ownership-validation'` in the current Nx graph. A second invocation through the registered root `workspace:flow-window-ownership-native` target remained in graph construction without spawning Cargo and was interrupted for handoff. Session `50289` ended with status 130 and its empty pre-Cargo log is `🗑️generated/flow-window-ownership-native.log`. No native compile or runtime pass is claimed.

The native runtime test uses a real registered `VcsArtifactApp<EditorApp<FlowPlayApp>>` with two `flow-main` windows. It covers independent camera/settings, config pack restore, unchanged document bytes, same-byte reload preserving window config while clearing window transient, generation scope, and stale/wrong-kind rejection.

The exact follow-up command is:

```sh
NX_DAEMON=false NX_NO_CLOUD=true NX_ISOLATE_PLUGINS=false \
NX_WORKSPACE_DATA_DIRECTORY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/nx-workspace-data-flow-native" \
NX_CACHE_DIRECTORY="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/nx-cache-flow-native" \
bun x nx run workspace:flow-window-ownership-native --output-style=stream --skip-nx-cache
```

The target sets `CARGO_TARGET_DIR` to this ticket's single `🗑️generated/cargo-trinity` directory and `CARGO_INCREMENTAL=0`.

## Exact File Ledger

The Flow base below is:

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any`

Created:

- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🟦️.ts`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🔣️.json`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🔗️.graphql`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧬️schema/🛰️.proto`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧫️fixtures/🔬️window-ownership/🔣️.json`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🧬️schema/🟦️.ts`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🧬️schema/🔣️.json`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🧬️schema/🔗️.graphql`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/🧬️schema/🛰️.proto`

Deleted:

- `✏️editor/🎚️config/🦀️.rs`
- `✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️editor/🎚️config/🧬️schema/🟦️.ts`
- `✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️editor/🎚️config/🧬️schema/🔗️.graphql`
- `✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️editor/🧵️retained/🎚️config/🦀️.rs`
- `✏️editor/🧵️retained/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/👥️set-contributions/🦀️.rs`
- `✏️editor/🎮️commands/👣️duplicate-widget-step/🦀️.rs`
- `✏️editor/🎮️commands/📋️duplicate-widget/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/📌️.empty.md`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient/📌️.empty.md`

Updated document contracts:

- `🧬️schema/🦀️.rs`
- `🧬️schema/🟦️.ts`
- `🧬️schema/🔣️.json`
- `🧬️schema/🔗️.graphql`
- `🧬️schema/🛰️.proto`
- `🧬️schema/📸️snapshot/🦀️.rs`
- `🧬️schema/📸️snapshot/🟦️.ts`
- `🧬️schema/📸️snapshot/🔣️.json`
- `🧬️schema/📸️snapshot/🔗️.graphql`
- `🧬️schema/📸️snapshot/🛰️.proto`
- `🧬️schema/🔺️diff/🦀️.rs`
- `🧬️schema/🔺️diff/🟦️.ts`
- `🧬️schema/🔺️diff/🔣️.json`
- `🧬️schema/🔺️diff/🔗️.graphql`
- `🧬️schema/🔺️diff/🛰️.proto`
- `🧬️schema/🔺️diff/📝️text/🦀️.rs`
- `🧬️schema/🧬️mutations/🦀️.rs`

Updated Flow runtime/editor files:

- `✏️editor/🦀️.rs`
- `✏️editor/🧵️retained/🦀️.rs`
- `✏️editor/🧵️retained/🗿️artifact/📬️preparation/🦀️.rs`
- `✏️editor/🧵️retained/🗿️artifact/📬️preparation/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️retained/🗿️artifact/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧵️retained/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🧪️tests/🔬️source-contract/🟦️.ts`
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🟦️.ts`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🟦️.ts`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
- `✏️editor/🧫️fixtures/📡️host-wire/🔣️.json`
- `✏️editor/🧫️fixtures/🧫️grant-frontier/🔣️.json`
- `👁️viewer/🦀️.rs`
- `🦀️.rs`

Updated generation files:

- `✏️editor/🎭️modes/🧬️generate/🎮️commands/➕️add-generation/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🎮️commands/🎚️update-generation-values/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🎮️commands/🎯️select-generation/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🎮️commands/🏷️rename-generation/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🎮️commands/🗑️remove-generation/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️.ts`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🟦️.ts`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs`
- `✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🟦️.ts`

Updated command files:

- `✏️editor/🎮️commands/↔️set-proximity-distance/🦀️.rs`
- `✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs`
- `✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs`
- `✏️editor/🎮️commands/✂️disconnect/🦀️.rs`
- `✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs`
- `✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`
- `✏️editor/🎮️commands/➕️add-widget/🦀️.rs`
- `✏️editor/🎮️commands/➕️add-widget/🧪️tests/🔬️unit/🦀️.rs`
- `✏️editor/🎮️commands/➖️remove-widget/🦀️.rs`
- `✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs`
- `✏️editor/🎮️commands/🏁️flow-eval-resolve/🦀️.rs`
- `✏️editor/🎮️commands/🏷️rename-flow-widget/🦀️.rs`
- `✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs`
- `✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs`
- `✏️editor/🎮️commands/📏️set-grid-factor/🦀️.rs`
- `✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs`
- `✏️editor/🎮️commands/🔗️connect-media-ports/🦀️.rs`
- `✏️editor/🎮️commands/🔦️open-spotlight/🦀️.rs`
- `✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs`
- `✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️editor/🎮️commands/🖱️context-menu-at/🦀️.rs`
- `✏️editor/🎮️commands/🖼️replace-image/🦀️.rs`
- `✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs`
- `✏️editor/🎮️commands/🗺️reorganize/🦀️.rs`
- `✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs`
- `✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs`
- `✏️editor/🎮️commands/🛍️set-catalogue-sections/🦀️.rs`
- `✏️editor/🎮️commands/🧮️evaluate/🦀️.rs`
- `✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs`
- `✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs`

Updated registration/report files:

- `📋️project.json`
- `📜️script.ts`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/flow-window-ownership.md`
