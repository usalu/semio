# Home Native Host Panel Owner Implementation Plan

## Scope

This is the bounded next implementation plan requested after the Architect exact-window slice. It refreshes the two earlier Home reports against the current native Shell and Home sources. No Home, Shell or framework production source was changed while preparing it.

The slice moves `setActivePanelTab` onto the native host/session owner before removing Home's duplicate field, mutation, command and manifest action. It does not create a Home `WindowConfig`, does not move directory or client identity fields, and does not change the separately retained `spawnedApps` / `activeSpawnedId` state.

## Verified Current Source

- The configured Space host remains `plugin_id = space`, `landing_app_id = home`, `host_app_id = studio` in `.../🔌️plugin/📇️registry/🤖️generated/🖥️hosts/🦀️.rs`.
- Native `SpacePanelState` is declared in WGPU Shell and serialized in `ViewModel.panel_json`. Its current Rust shape is exactly `active_panel_tab`, `spawned_apps`, and optional `active_spawned_id`.
- `ShellState::dispatch_action` intercepts framework/sync/check-in and updates `setActiveUtility` before selecting a `ProgramBridge`, but it has no `setActivePanelTab` host branch. Generic invocation still takes `plugin_id`, `app_id`, mode and window identity from the active session even when the descriptor controller resolved another program.
- A right panel hit sets only `active_right_tab`, then sends `setActivePanelTab` with the configured Studio controller. A left hit sends only while the active app itself is Studio. Tour reveal repeats those asymmetric rules.
- Search items create panel actions from the active session's `panel_tabs` and active session controller. The current Home manifest has no panel tabs, so an active Home session currently produces no Home panel palette rows. A palette acceptance case is still needed for an active app such as Studio that does own panel tabs.
- `switch_to_managed_app` copies the landing Home session's `ViewModel` into `DirectoryHomeProjection` and later restores it. A host write to `panel_json` therefore reaches the existing session retention seam without a Home config mutation.
- Home still declares `active_panel_tab`, `HomeConfigMutation::SetActivePanelTab`, `HomeCommand::SetActivePanelTab`, a retained config publication row/proof, the `setActivePanelTab` manifest action, limits fixtures and unit tests. No mounted Home editor/viewer renderer reads the field.
- The browser helper still exposes a TypeScript `SpacePanelState` with an always-empty `programs` member while native Rust has already removed that member. The new owner needs one strict shared schema rather than preserving this shape drift.

## Ownership Contract

Create a domain-neutral host panel-state schema beside the Shell host/view-context boundary. Version 1 contains:

| Field | Owner and constraint |
| --- | --- |
| `activePanelTab` | Host/session selection; a bounded non-empty panel leaf id |
| `spawnedApps` | Existing spawned-instance owner; bounded by the window-instance capacity and unchanged by panel selection |
| `activeSpawnedId` | Existing optional spawned focus; unchanged by panel selection and, when present, identifies a retained entry |

The schema must reject unknown fields and must not contain `programs`, directory data, app configuration, or a window id. Its string, list and encoded-byte limits must derive from the actual `panel_json` carriage capacity and the existing app/window instance capacities rather than introduce arbitrary larger constants. `activePanelTab` and the configured catalogue/default leaf are always nonempty. Closing a panel changes its separate visibility flag and retains its last selected leaf, so no empty or null selection is needed. Mirror the strict contract in JSON Schema, TypeScript and Rust. Use the existing neutral panel-carriage fixture to prove both language implementations accept the same record and preserve `spawnedApps` and `activeSpawnedId` byte-for-byte while changing `activePanelTab`.

`ViewModel.panel_json` remains the carriage. Before choosing its one codec, inventory every native and browser producer, consumer, persistence path, projection and fixture of `panel_json`, including empty/default writes and guest-facing view construction. Record that ledger in the implementation report. The current browser uses Pack/base64 while native uses JSON text, so the implementation must align every inventoried path through the shared schema without a fallback decoder or dual-format admission.

## Native Host Action Route

Add one narrow qualification helper immediately before generic program selection in `ShellState::dispatch_action`:

1. Require a configured host and the exact `setActivePanelTab` verb.
2. Require one bounded `tabId` argument.
3. Resolve the descriptor controller to either the configured host app or the active session app.
4. Require `tabId` to equal a flattened leaf owned by that resolved app's `panel_tabs`; group/container ids do not qualify.
5. Treat an invocation that claims the configured host panel verb/controller but fails argument, catalogue or state validation as a rejected host action. It must return an error and must not fall through to a guest bridge. An unrelated verb remains eligible for ordinary guest dispatch. Matching the verb alone never grants host authority.

For an accepted leaf, a single host reducer must:

- decode the active session's schema-valid panel state;
- change only `activePanelTab`;
- preserve `spawnedApps` and `activeSpawnedId` exactly;
- update the matching local left/right tab, open flag and panel kind from the leaf's `PanelGroup`;
- write the encoded state back to the same session's `ViewModel.panel_json`;
- synchronize session chrome and return before `ProgramBridge` selection.

Direct left/right hits, search-palette selections and tour reveal should all enter this reducer through qualified descriptors. Remove their current local-only/asymmetric selection writes once the reducer owns both local and retained state. The direct Home-session/Studio-controller case qualifies against Studio's actual panel catalogue; an active Studio palette action qualifies against the active Studio catalogue. A fake app that merely declares the same verb, a stale tab id, a parent panel node, or an unrelated controller must not qualify.

## Native Test-First Fixture

Extend `.../🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` through a small shared AppDefinition fixture rather than copying a production manifest.

Use `ProgramBridgeEntry::from_wasm` with an explicit Space manifest containing a Home landing app and Studio host app. The path may deliberately be non-runnable because a correctly intercepted host action returns before guest dispatch. Install a real `ActiveSession` for Home with a panel state containing at least two `SpawnedAppEntry` values and a non-empty `activeSpawnedId`.

The native law must prove:

1. A Studio-controller action for an actual Studio panel leaf succeeds while Home is active and no guest can run.
2. `activePanelTab` changes; `spawnedApps` and `activeSpawnedId` are identical before and after.
3. The correct local left/right selection and open/kind fields change from the leaf's actual `PanelGroup`.
4. Switching away and back through `DirectoryHomeProjection` restores the same decoded state.
5. An active Studio search item for the same leaf reaches the same reducer.
6. Wrong controller, unknown tab, group/container id, missing `tabId`, malformed panel state, empty configured catalogue/default ids and an unconfigured host are rejected according to their ownership relation. Any invocation that claimed the configured host panel route must be observed to stop before guest dispatch even when host validation fails.
7. Success does not depend on a Home action declaration, Home command codec, Home config store, window id or live guest instance.

Instrument the fixture's missing/non-runnable guest seam so the law proves guest dispatch was never attempted for both a valid intercepted action and an invalid action that claimed the host route. A successful host-state assertion alone is insufficient evidence of that control-flow boundary.

Keep the existing panel layout tests separate: `PanelLayoutPersisted` owns open/kind/width preferences, while `SpacePanelState` owns session panel selection plus spawned-instance focus.

## Home Duplicate Cleanup

Only after the native host law passes, remove the Home mirror coherently from:

- Home config Rust state/default/validation/footprint/preparation and its JSON, TypeScript, GraphQL and Protobuf facets;
- `HomeConfigMutation::SetActivePanelTab`, its descriptor/codecs/fixtures/tests and mutation-schema references;
- `HomeCommand::SetActivePanelTab`, command module wiring, action mapping, retained tool ids/proof/publication rows and retained-limit fixture;
- Home's `setActivePanelTab` manifest action and classification;
- generated Home artifact/action schema rows that refer to that action.

Then prove Home config Pack/DSL and mutation codecs contain no panel field or operation, both mounted Home renderers remain unchanged, and host selection still works with Home's action removed. Do not remove or rename Shell `SpacePanelState.spawned_apps` / `active_spawned_id` or the browser equivalents.

Studio's own `SpaceConfig.active_panel_tab` is a separate duplicate and is outside this bounded Home cleanup unless its mounted consumers and retained route are audited in the same follow-up. Do not silently broaden this slice to it.

## Registered Verification Plan

The implementation should add two launcher-visible Nx targets through root `📜️script.ts`, `📋️project.json`, `.vscode/launch.json`, and `.vscode/🧩️launch.seed.jsonc`, with reserved launch orders `311.236` and `311.237`:

- a schema/oracle route that validates the strict shared panel schema in TypeScript and independently patches the neutral fixture while preserving the spawned fields;
- a native route filtered to the new host panel owner law plus focused Home config/command negation laws.

Final evidence must record the actual Nx commands, fresh ticket-local Nx directories, selected native test count, debug output, runtime state before/after, and terminal result. No direct Bun script or direct Cargo invocation is final launcher evidence.

The schema/oracle route is now registered as `workspace:home-host-panel-owner-oracle` with launch order `311.236`. After the owner audit removed the unsupported empty-tab sentinel, a fresh registered Nx run passed in 4.9 seconds with zero cache hits:

```text
NX_WORKSPACE_ROOT=/Users/ueli/Documents/semio REPO_ROOT=/Users/ueli/Documents/semio NX_DAEMON=false \
NX_CACHE_DIRECTORY=<ticket>/🗑️generated/home-host-panel-oracle-nx-cache-20260913-3 \
NX_WORKSPACE_DATA_DIRECTORY=<ticket>/🗑️generated/home-host-panel-oracle-nx-data-20260913-3 \
bun nx run workspace:home-host-panel-owner-oracle

NX Successfully ran target home-host-panel-owner-oracle for project workspace
Run duration: 4.9s
Cache: 0/1 hit (0%)
```

The passing route executes the independent Ajv/JSON Patch oracle and a strict no-emit TypeScript compile. It proves the neutral closed shape, required nonempty tab selection, the actual carriage/list/identifier bounds, exact tab-only patch projection, preserved spawned roster/focus, unknown-field rejection, duplicate-ID rejection, aggregate encoded-capacity rejection, over-capacity rejection, and dangling active-ID rejection. It is preparation evidence only; native route `311.237` and production host routing remain gated.

The complete producer/consumer/codec inventory and the selected JSON wire decision are recorded in `home-panel-json-producer-consumer-inventory.md`.

## Completion Boundary

This slice is complete only when native host dispatch owns qualified panel selection before the guest bridge, its schema is identical across Rust and TypeScript, the acceptance law observes retained state restoration, and Home has no panel field/action/command residue. Directory projection/client identity ownership and Studio's separate config mirror remain explicitly open.
