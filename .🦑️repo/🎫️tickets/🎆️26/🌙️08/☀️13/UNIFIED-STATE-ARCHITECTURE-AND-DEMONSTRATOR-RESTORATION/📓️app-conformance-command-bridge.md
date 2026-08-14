# App Conformance and Command Bridge

## Inventory

- 33 top-level plugin Nx projects expose `test-quick`.
- The generated playground catalogue contains 58 runnable variants.
- Catalogue validation found 58 unique variants, no duplicate React/WGPU ports, and no missing Rust crate paths.

## Shared failure

The first sequential plugin matrix stopped in `semio-framework-plugin` before app code. The bridge still consumed removed flat `AppDefinition.actions`, `CommandRef`, and `CommandScope` fields while the manifest now owns action definitions inside window kinds and command definitions inside apps or modes. The framework crate root also exposes the kernel execution `CommandInvocation`, which collides by name with the manifest owner-address protocol used by `ShellHost` and the plugin runtime.

## Repair

- Manifest action and command invocations are imported under explicit `Manifest*` aliases, keeping the owner-address runtime protocol distinct from the kernel execution envelope.
- App and mode command definitions are validated and indexed from structural containment.
- Window actions are materialized into `WindowKindDefinition.actions`; centrally declared builder actions can be assigned through `window_kind_action_refs` without restoring a flat manifest compatibility layer.
- Framework-injected history, clipboard, tutorial, utility, tool, and interaction actions are tested through nested window ownership.
- Layout and Space moved their scoped action declarations to the builder-only reference method.
- Existing in-file tests were migrated from removed command scopes/references and extended for addressed app/mode command ownership and active-mode gating.

## Validation

- `bun nx run @semio-tech/framework-os-dev:test-quick --skip-nx-cache`: 12/12 tests passed.
- `bun nx run @semio-tech/vcs-plugin:test-quick --skip-nx-cache`: 51/51 tests passed, 0 skipped, exit 0.
- The shared `semio-framework-plugin` library compiled successfully in that Nx run before downstream stdio and VCS compilation.
- The first Space pass compiled through the shared plugin bridge, then exposed a separate host `E0560` from an obsolete `AppDefinition.actions` initializer. The state-infrastructure lane removed the remaining host flat-action initializers and updated the host fixtures to nested window interactions.
- The host lane then reported both its ticket-local direct check and canonical `bun nx run @semio-tech/framework-os-host-rs:check --skip-nx-cache` passing.
- The resumed Puzzle/Space Nx matrix uses the ticket-local `🎯️target-app-conformance` target. Puzzle compiled through its repaired window ownership and the shared plugin bridge. Space then exposed three stale flat-action test assertions; these were moved to the actual Home-main and Studio-workflow window catalogs, and Home now explicitly assigns all eight Home actions to its main window.

## Source-wide flat-action migration

The exact conformance audit requested by the app lane is now empty:

`rg -n '\bdefinition\.actions\b' ✏️s/🔌️plugins 🧰️framework/🛒️products/💻️os/🔨️modules/🔌️plugin -g '*.rs'`

All 37 observed stale accesses were replaced with window-owned action lookup. Assertions about a specific utility/window use that owning window directly; cross-window catalog assertions flatten `window_kinds[*].actions` only inside existing tests. No flat production compatibility API was restored.

## App-lane file inventory

- Shared bridge: `🧰️framework/🛒️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.
- Layout/Space ownership: `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs`, `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs`, and `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs`.
- Puzzle ownership: `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs`, `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🦀️component.rs`, and `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️component.rs`.
- Puzzle tests: `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` and `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`.
- `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🦀️component.rs`.
- `✏️s/🔌️plugins/💠️lowpoly/🎛️apps/💠️lowpoly/🦀️component.rs`.
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`.
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🦀️component.rs` and `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎮️commands/📚️set-active-example/🦀️component.rs`.
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🦀️component.rs` and `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🎮️commands/📚️set-active-example/🦀️component.rs`.
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🦀️component.rs`.
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🌐️shell/🦀️component.rs`, `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs`, and `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs`.
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🦀️component.rs`.
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs` and `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎮️commands/🀄️add-tile/🦀️component.rs`.
- `✏️s/🔌️plugins/🎬️sequence/🎛️apps/🎬️sequence/🦀️component.rs`.
- `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🦀️component.rs`.
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`.
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🦀️component.rs`.
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🦀️component.rs`.

## Puzzle 5d downstream repair

The first demonstrator parity shard then reached Puzzle 5d and found two `E0308` failures: its 2D and 3D `WindowKindDefinition.actions` fields still contained `ActionRef` values. The window leaves now leave the owned-definition vectors empty, and `create_puzzle5d_app` assigns the centrally declared action definitions to those windows through `window_kind_action_refs`. The existing `window_kind_actions_scope_transform_to_3d_only` test already covers the resulting ownership matrix: transforms and 3D camera are 3D-only, board events are 2D-only, and intentionally shared actions remain on both.

## Systemic registry audit

`bun nx run @semio-tech/plugin-registry:check --skip-nx-cache` currently reports approximately 5,684 existing taxonomy/path violations across 25 plugin roots. Many are false missing-target reports for existing `#[path = "../../..."]` files, so this is a registry path-resolution blocker rather than a per-app state conformance failure.

## Current affected-suite matrix

All Cargo-backed app checks below use `CARGO_TARGET_DIR=.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION/🎯️target-app-conformance`.

| Nx project/check | Result | Evidence |
| --- | --- | --- |
| `@semio-tech/framework-os-dev:test-quick` | Pass | 12/12 tests. |
| `@semio-tech/vcs-plugin:test-quick` | Pass | 51/51 tests, 0 skipped, exit 0. |
| `@semio-tech/puzzle-plugin:test-quick` | Pending independent rerun | Its compile passed the repaired Puzzle 5d window ownership and advanced to Space in the aggregate run, but Nx reported the aggregate failed; no independent final pass is claimed. |
| `@semio-tech/space-plugin:test-quick` | Blocked outside app lane | App production and migrated tests compile past the removed flat catalog. `cargo test --no-run` then fails in shared `framework/ui/wgpu/draw.rs:1903` with two lifetime errors around `draw_raster_layers` → `draw_silhouette_mask`. The shared WGPU/state owner has the broader native drift set. |
| Remaining affected batch: Animate, Cad, Dag, Draw, FEM, GIS, Lowpoly, Note, Playbook, Raster, Sequence, Shooting, Trinity | Pending shared WGPU green | Launching them now would repeat the same shared dependency failure; rerun on the warmed ticket target once its owner reports green. |

Neither the current Space check nor the pending affected suites are represented as passing. The two source audits are green: both `\bdefinition\.actions\b` and the broader `\.definition\.actions\b`/obsolete `AppDefinition { actions: ... }` searches return no matches in plugin and shared-plugin Rust sources.

A companion app-source audit also returns no matches for removed `CommandRef`/`CommandScope` vocabulary or direct kernel `semio_framework::CommandInvocation` imports in `✏️s/🔌️plugins`; the intentional manifest invocation protocol remains explicitly aliased only in the shared plugin bridge.
