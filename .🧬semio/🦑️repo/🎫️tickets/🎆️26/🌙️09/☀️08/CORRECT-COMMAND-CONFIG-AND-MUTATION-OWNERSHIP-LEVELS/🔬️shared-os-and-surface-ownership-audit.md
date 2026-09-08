# Shared OS And Surface Ownership Audit

## Scope

Read-only audit of shared ownership outside the Jack/Rewriting implementation work. It covers the OS shell, framework surface state, and representative non-Trinity plugin implementations. No code or generated output was changed.

## Ownership Rule

Classify each value before placing its schema, command, or mutation:

| Scope | State class | Examples | Correct owner |
| --- | --- | --- | --- |
| OS user preference | Persisted local-only, per OS user/device | locale, terminology, appearance, selected theme, UI driver, keybindings | `🧰️framework/🛍️products/💻️os/🎚️config` |
| OS device/shell presentation | Persisted local-only, device-sensitive | desktop/tablet layout, dock/window arrangement | OS shell configuration, never an artifact app |
| Surface view state | Ephemeral local-only; optionally persisted local-only under the surface identity | camera, viewport, LOD display choice, current editor selection | shared framework surface/window state keyed by app + window |
| Surface presence | Ephemeral shared | remote selection, cursor, camera when collaborators need it | framework presence/interaction contract |
| Artifact domain state | Persisted shared and event-sourced | nodes, layers, rules, content, timeline data | artifact subset schema and mutation leaves |

An app command is allowed to request a shell or surface transition, but it must not define an app-specific config mutation for a value owned above that app. Values in the first two rows must not appear in plugin artifact snapshots, diffs, config schemas, config mutations, or generated wire schemas.

## P0: Canonical OS Preferences Exist In The Shell But Not In OS Config

The shell already declares these OS commands in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`:

- `os.setAppearance`
- `os.setThemeId`
- `os.setLayout`
- `os.setLocale`
- `os.setTerminology`
- `os.setDriver`

`buildOsCommands` defines them and `dispatchOsCommand` handles them locally (lines 3303–3525). The only existing OS config facets are opening preferences, merge policy, and identity: `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs` and `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`.

The React shell instead reads and writes independent `StoragePort` keys. Its `UiPrefsState` includes appearance, layout, driver, locale, terminology, theme, custom drivers/themes, and keybinding overrides in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx` (lines 716–719, 1059–1072, 1142–1164). The UI package provides direct storage helpers for appearance, layout, locale, terminology, and theme in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` (lines 1896–1987).

This is a boundary violation even before plugin copies are considered: those commands are visibly OS commands but do not have one schema-first, event-sourced OS config authority. The renderer should consume the OS preference projection. UI-library types may remain in the UI module but need explicit framework re-exports at the OS contract boundary.

### Required OS Preference Facet

Add an OS config facet, for example `os.config.ui-preferences`, with direct mutation leaves and language-neutral schemas for:

- `locale`
- `terminology`
- `appearance`
- `themeId`
- `driverId`
- `keybindingOverrides`

Keep `layout` and dock/window arrangement in an OS shell/device preference facet because those values depend on the current device’s display geometry. Do not put device layout into an artifact or a collaboration-shared config log.

All should be persisted local-only per OS user/device. A later explicit account-profile synchronization design can project the user-preference subset across devices; it must still remain outside artifacts and plugin config. The command palette should dispatch the OS config mutations, after which every mounted app receives the resolved OS preference projection rather than a copied value.

## P0: Locale Is Duplicated Across Plugin Config And Artifact Wire Schemas

This confirms the stated Jack example is systemic. Non-Trinity examples include the following paths; each lies in an editor config or editor command subtree:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`

There are also direct app `set-locale` commands in Writer, Layout, Forms, Playbook, Note, Remodel, Imperative, Animate, Sequence, Puzzle, Flow, GIS, FEM, VCS, Sourcing, Reasoning, Raster, and Procedural artifacts. Representative command/mutation leaves are:

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🦀️.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🦀️.rs`

The policy should reject semantic field names `locale` and `uiLocale`, semantic commands `set-locale` / `setLocale`, and matching config mutation leaves below a surface config owner. It must exempt artifact-domain metadata such as the DWG product-information `locale_id`, which describes imported file provenance rather than the OS display language.

## P0: Terminology Is Also OS-Wide

The OS shell defines `os.setTerminology` beside `os.setLocale`, and uses the active terminology to resolve app labels, breadcrumbs, commands, dialogs, and utility labels throughout `ShellHelpers`. Therefore terminology cannot be a puzzle-app property. Existing plugin-owned commands demonstrate the duplication:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📖️set-terminology/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📖️set-terminology/🦀️.rs`

Reject `terminology`, `uiTerminology`, `set-terminology`, and `setTerminology` from surface config just as for locale. App manifests may declare terminology-specific labels; they may not select the user’s terminology.

## P1: Active Utility Is A Host Per-Window State, Yet Plugin Config Persists It

The OS shell independently documents `activeUtilityByWindowId` as host-owned and keyed by window instance: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx` lines 457–468. It also says the active mode-level tool is host-owned and never document state.

Draw’s command confirms the intended ownership in its own docstring, then violates it by emitting plugin config:

`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪛️set-active-utility/🦀️.rs` calls `Emit::config(DrawingConfigMutation::SetActiveUtility...)` even though its comment calls it a “Host-owned utility switch”.

Direct config mutation copies occur in:

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧰️set-active-utility/🦀️.rs`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧰️set-active-utility/🦀️.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧰️set-active-utility/🦀️.rs`

Move this state to the shared shell/surface window-state contract. It is ephemeral local-only. If peer awareness is desired, project the active utility into presence; do not make it a config mutation, artifact diff, or wire field in the domain snapshot.

## P1: Camera, Viewport, LOD, And Selection Need A Shared Surface Boundary

These are not OS-global settings, but they are also not artifact-domain configuration. They are user-specific view state. The repository contains both the correct and incorrect placements:

- Correct local-window behavior: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs` updates `Puzzle2dActionCtx` runtime camera and calls `puzzle2d_window_only_scope()`.
- Incorrect persisted config copies: Writer, Layout, Note, Sequence, Remodel, Equation, GIS Map, GIS Terrain, and Shooting all define `set-camera` mutation leaves beneath `✏️editor/🎚️config/🧬️schema/🧬️mutations/`.
- Writer also persists editor selection in `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📐️set-editor-selection/🦀️.rs`.
- Shooting persists shot selection and camera in its editor config, while GIS Map separately has both config and presence `set-camera` leaves. This makes the distinction between a user’s stored view and peer-visible current view ambiguous.
- GIS Map persists `set-lod-mode` in editor config; LOD is a rendering/display choice and belongs to the same local surface-view category.

Define one framework-owned, typed surface-view contract. It may contain common 2D/3D camera and selection primitives, with plugin-owned extension payloads for domain-specific view data. Key it by surface identity and window identity. Persist it locally only when restoring a user workspace is desired. Send only a deliberately selected projection through the existing generic presence interaction channel; `ShellState` already derives peer selections/hover from `PresenceInteraction` in `Shell/🟦️.tsx` lines 371–430.

Do not use a blanket field-name ban for `camera`, `selection`, `grid`, or `lod`: a domain artifact may legitimately model a physical camera, a selected sequence shot, or a grid. Enforce their owner by state class and location: display-only values under `✏️editor/🎚️config` must declare surface-window/presence ownership, while artifact operations that alter domain data remain plugin-owned.

## Enforcement And Validation

Extend the language-agnostic policy using the existing schema-owner discovery and field extraction in the root `📜️script.ts`:

1. For every surface config schema, diff schema, protobuf schema, config mutation leaf, and command descriptor, reject `locale`, `uiLocale`, `terminology`, `uiTerminology`, `appearance`, `uiAppearance`, `themeId`, `uiThemeId`, `driverId`, and `keybindingOverrides`.
2. Reject `set-locale`/`setLocale` and `set-terminology`/`setTerminology` in plugin surface commands and config mutation leaves. Require their canonical OS command/mutation descriptors instead.
3. Reject `activeUtilityId` and `set-active-utility` from surface config. Ensure their transitions use the framework’s app-window state; allow only the explicit presence projection where needed.
4. Add a classification rule for `camera`, `viewport`, `selection`, `lod`, and grid-view settings: require a surface window/presence owner and reject a surface shared-config owner unless the schema declares documented artifact semantics.
5. Validate OS UI preference changes by dispatching their typed config mutations, reloading a new shell instance, and asserting that two different plugin surfaces resolve the same locale/terminology/theme without app config mutations. Validate a second user/window keeps an independent camera/selection and that optional presence reports it without changing the artifact snapshot.

Recommended checks after implementation:

```sh
bun nx run framework-os-rust:test
bun nx run framework-os-typescript:test
bun nx run semio-script:check-command-config-ownership
```

The exact project target names must be confirmed from the active `📋️project.json` before execution; no verification command was run by this read-only audit.
