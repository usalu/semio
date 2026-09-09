# GIS Terrain Window Config Ownership

## Result

The GIS Terrain viewport camera is persisted-local configuration owned by one exact registered `gis3d-main` window. The app-level `Gis3dConfig` and its configuration mutation lane are gone. `Gis3dPlayApp` now uses `NoConfig`/`NoConfigMutation`, and rendering reads `GisTerrainWindowConfig` from the addressed window owner.

`setCamera` publishes one `WindowConfig` operation. It does not change the terrain document, app config, presence, or transient state. `setExaggeration` remains a document operation.

## Authority and publication

- `GisTerrainWindowConfigOwner` is registered for the concrete `gis3d-main` window kind with schema `gis.gisterrainwindowcfg`.
- The direct command route requires the host-supplied `ViewModel`; the retained route requires the `ViewModel` captured in `ArtifactOwnedToolJobContext`.
- `addressed` requires `ViewModel.window_id`, locates that id in the trusted registered window roster, and verifies its kind is `gis3d-main` before creating `WindowConfigMutation::of`.
- No payload window id or surface id is accepted. The neutral schema rejects `windowId` and other unknown ownership fields.
- Rendering obtains the exact window snapshot through `ConfigView::window::<GisTerrainWindowConfigOwner>()` and uses its `camera_json` in `World3dScene`.
- The retained publication contract for `setCamera` is `ArtifactToolPublicationLane::WindowConfig`.

## Transient and presence inventory

The app has no local transient view field. It retains `NoTransient`/`NoTransientMutation` with the framework no-transient disposer and retirement factory. The registered-app regression asserts `window_transient_generation` is `None` for both concrete windows. No empty transient type was invented.

`Gis3dPresence.camera_json` remains a distinct app-level peer-presence record. Repository usage tracing found no `setCamera` presence publication and no render read from that record: the local viewport source is exclusively the exact window config. The presence record is outside this local-window migration and represents a separate shareable peer-state channel.

## Schema and oracle

The former app-config subtree was relocated under the terrain window and renamed across Rust, JSON Schema, TypeScript, GraphQL, proto, mutation, diff, fixtures, and oracle facets. `cameraJson` carries `x-semio-state: config` and `x-semio-owner: window`; the TypeScript surface carries `@state config @owner window`.

The language-neutral contract runs the production TypeScript parser and Ajv over the same accepted and rejected values, including rejection of a payload-owned `windowId`. It then executes a two-window state trace that proves each camera update affects only its addressed window.

- `🗑️generated/gis-terrain-window-config-oracle-1.log`: intentional red; the generated production parser accepted unknown fields while Ajv rejected them.
- `🗑️generated/gis-terrain-window-config-oracle-2.log`: terminal green after the production parser adopted the shared strict `parseSchemaRecord` helper; `accepted=2 rejected=4 windows=2 owner=gis3d-main`.
- `🗑️generated/gis-terrain-window-config-typescript-1.log`: terminal green strict TypeScript compile.
- `🗑️generated/gis-terrain-window-config-ticket-oracle-1.log`: terminal green through the permanent ticket command.
- `🗑️generated/gis-terrain-window-config-rustfmt-check-3.log`: terminal green final targeted Rust formatting check.

The root Nx attempt in `🗑️generated/gis-terrain-window-config-root-oracle-1.log` waited on another graph process and then reported that the newly registered target was unavailable from that stale graph. It is not counted as a source or oracle failure because the checked-in root target exists and the ticket permanent route completed the same oracle and TypeScript checks. A direct root-script attempt was interrupted after it remained in shared initialization without producing output; it has no validation result.

## Native runtime regression

`gis_terrain_window_config_isolates_two_registered_windows_and_reloads` uses two boxed registered app instances and two registered `gis3d-main` window instances. It dispatches different cameras through captured `ViewModel` contexts and asserts:

- exactly one `WindowConfig` receipt for each operation;
- the document and app `NoConfig` pack stay unchanged;
- each exact window reaches config generation 1;
- both windows have no transient generation;
- rendering returns the camera stored for that exact window;
- exported window config packs reload into a second registered app without crossing identities;
- both app instances reach close.

The regression runs inside an 8 MiB test thread, matching the established plugin test boundary. Native Cargo validation was not started because the parent owns the shared Cargo queue.

## Registered commands

- Root Nx targets: `workspace:gis-terrain-window-config`, `workspace:gis-terrain-window-config-native`.
- Ticket Nx targets with the same suffixes.
- Root permanent command: `bun ./📜️script.ts verify gis-terrain-window-config [native]`.
- Ticket permanent command: `bun ./📜️script.ts gis-terrain-window-config [native]`.
- Launch orders: 311.170 and 311.171 in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`.
- The native route uses ticket `🗑️generated/cargo-trinity`, `CARGO_INCREMENTAL=0`, package `semio-s-artifact-gis-gisterrain`, feature `component-app-assembly`, and test filter `gis_terrain_window_config_`.

## Exact file ledger

Relocated and renamed from `✏️editor/🎚️config` to `✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/⚙️config`:

- `🔮️oracle/🔣️.json`
- `🦀️.rs`
- `🧪️tests/🔬️unit/🦀️.rs`
- `🧪️tests/🧬️direct-leaves/🦀️.rs`
- `🧫️fixtures/📷️set-camera-applied/➡️after.json`
- `🧫️fixtures/📷️set-camera-applied/⬅️before.json`
- `🧫️fixtures/🧬️direct-leaves/🔣️.json`
- `🧫️fixtures/🧬️direct-leaves/🧬️schema/🔣️.json`
- `🧬️schema/🔗️.graphql`
- `🧬️schema/🔣️.json`
- `🧬️schema/🔺️diff/🔣️.json`
- `🧬️schema/🔺️diff/🦀️.rs`
- `🧬️schema/🛰️.proto`
- `🧬️schema/🟦️.ts`
- `🧬️schema/🦀️.rs`
- `🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json`
- `🧬️schema/🧬️mutations/🎥️set-camera/🦀️.rs`
- `🧬️schema/🧬️mutations/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs`
- `🧬️schema/🧬️mutations/🎥️set-camera/🧬️schema/🔣️.json`
- `🧬️schema/🧬️mutations/🔣️.json`
- `🧬️schema/🧬️mutations/🦀️.rs`

Created under the exact window config subtree:

- `🧫️fixtures/🔬️window-config-ownership/🔣️.json`
- `🧪️tests/🔬️contract/🟦️.ts`

Removed:

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/⚙️config/📌️.empty.md`
- the now-empty former `.../✏️editor/🎚️config` subtree

Updated app and test files:

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`
- `.../✏️editor/🦀️.rs`
- `.../✏️editor/🎮️commands/👁️view/🦀️.rs`
- `.../✏️editor/🎮️commands/👁️view/🧪️tests/🔬️unit/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `.../✏️editor/🧫️fixtures/🧫️retained-command-limits/🔣️.json`
- `.../✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs`
- `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs`
- `.../🧪️tests/🎚️mutate-gis-gisterrain-1-config/🥒️.feature`
- `.../🧪️tests/🎚️mutate-gis-gisterrain-1-config/🦀️.rs`

Updated permanent validation files:

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- ticket `validation/📜️script.ts`
- ticket `validation/project.json`
