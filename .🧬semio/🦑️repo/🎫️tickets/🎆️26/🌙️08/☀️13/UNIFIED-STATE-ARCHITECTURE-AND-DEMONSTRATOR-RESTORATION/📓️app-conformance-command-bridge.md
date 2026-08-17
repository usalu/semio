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
| `@semio-tech/puzzle-plugin:test-quick` | Pass | 452/452 tests after nested window action ownership and both 21-argument world-scene call repairs. |
| `@semio-tech/space-plugin:test-quick` | Pass | 90/90 tests after nested Home/Studio action ownership, `artifact-ref` fixtures, and stdio descriptor registration. |
| `@semio-tech/draw-plugin:test-quick` | Pass | 94/94 tests. |
| `@semio-tech/note-plugin:test-quick` | Pass | 93/93 tests. |
| `@semio-tech/lowpoly-plugin:test-quick` | Pass | 125/125 tests after nested action ownership and structural DSL fixture repair. |
| `@semio-tech/animate-plugin:test-quick` | Pass | 228/228 tests. |
| `@semio-tech/shooting-plugin:test-quick` | Pass | 105/105 tests. |
| `@semio-tech/trinity-plugin:test-quick` | Pass | 196/196 tests. |
| `@semio-tech/playbook-plugin:test-quick` | Pass | 73/73 tests. |
| `@semio-tech/dag-plugin:test-quick` | Pass, exact total pending | Passed in the joint DAG/Lowpoly rerun after exact node/edge inverse ordering repair; an individual summary capture remains required. |
| `@semio-tech/sequence-plugin:test-quick` | Shared-blocked | Initial 125/127 exposed the same exact-order inverse defect; repaired source has not reached its tests because current shared stdio compilation fails first. |
| `@semio-tech/raster-plugin:test-quick` | Exact rerun pending | Aggregate output was truncated; no exact total is claimed. |
| `@semio-tech/cad-plugin:test-quick` | Exact rerun pending | Earlier result was invalidated by a killed Cargo child corrupting the isolated incremental target. |
| `@semio-tech/gis-plugin:test-quick` | Exact rerun pending | Current 21-argument terrain call is repaired; earlier nextest preparation exceeded the then-active budget. |
| `@semio-tech/fem-plugin:test-quick` | Exact rerun pending | Earlier cold build exceeded the then-active build budget. |

The two source audits are green: both `\bdefinition\.actions\b` and the broader `\.definition\.actions\b`/obsolete `AppDefinition { actions: ... }` searches return no matches in plugin and shared-plugin Rust sources.

A companion app-source audit also returns no matches for removed `CommandRef`/`CommandScope` vocabulary or direct kernel `semio_framework::CommandInvocation` imports in `✏️s/🔌️plugins`; the intentional manifest invocation protocol remains explicitly aliased only in the shared plugin bridge.

## External shared stdio blocker during final app gate

The serial final Sequence validation rebuilt the ticket-local target and reached `semio-s-plugin-stdio`, exposing two successive current-source schema drifts before any Sequence test could run:

- DWG AC1018 `DwgArtifact::to_snapshot` still initialized removed `sections` and `decode_status` fields. This app-conformance lane removed those obsolete fields without an adapter and extended the existing DWG test module to assert exact artifact/snapshot preservation across every current field.
- The next warmed run exposed 43 further errors in concurrent shared stdio/schema work: unresolved XML lexical APIs, missing `XmlDocument.prolog`, stale XML/DWG/PDF/SVG `source` access, SVG lexical/prolog shape drift, ZIP `physical` construction and `ZipPhysicalLayout: DslField`, plus one missing `#[artifact_schema]` declaration.

These 43 errors are owned by the active external stdio/schema lane. They block Sequence, DAG, Raster, CAD, GIS, and FEM from compiling their shared dependency and therefore are not recorded as app-suite failures. Full compiler evidence is retained in `🧪️sequence-final.log`; the final app gate resumes only after the shared stdio check is green.

## DWG gate invalidation audit

The requested low-concurrency canonical stdio rerun used `CARGO_BUILD_JOBS=2`, the 60-minute build budget, the 30-minute test budget, and the warmed `🎯️target-app-conformance`. Full output is retained in `🧪️stdio-dwg-gate.log`.

The compiler observed source while the active schema owner was rewriting DWG, XML, and SVG. It ended with 95 errors: 49 locations under SVG, 41 under XML, two under DWG, and three diagnostics without an artifact path. The earlier 69-error DWG export/caller cluster was absent. The two DWG diagnostics were an internally inconsistent intermediate revision: `decode_dwg` referenced `decode_r2004_physical` and initialized `DwgSnapshot.physical` before those definitions were visible to the same compiler invocation. The current files now contain the physical layout types, `DwgSnapshot.physical`, and `decode_r2004_physical`; their final writes landed at 15:28:10–15:28:11, after compilation had begun. Consequently this run establishes source invalidation, not a current DWG failure or pass.

No DWG source edit was made by the app-conformance lane during this audit. Per cross-thread coordination, all stdio editing and validation is paused until the artifact owners report a stable green stdio revision. The six exact app reruns remain pending rather than failed.

## Provisional post-stdio app snapshot

After the first stable-green stdio handoff, the warmed serial app gate produced two exact green snapshots:

- Sequence: 127/127 passed, 0 skipped, exit 0 (`🧪️sequence-exact.log`).
- DAG: 93/93 passed, 0 skipped, exit 0 (`🧪️dag-exact.log`).

The exclusive stdio owner then reopened shared source for a stricter raw-state audit before Raster began. No downstream gate was left active. These two results are retained as provisional snapshot evidence only; Sequence and DAG must both rerun on the next stable-green handoff before final source-stable totals can be claimed.
