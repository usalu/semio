# Home Host Panel Owner Implementation

## Ownership boundary

`ViewModel.panelJson` is the Home/Studio host session's panel owner. Its strict JSON-text payload contains only the exact configured `activePanelTab`, the bounded `spawnedApps` instance roster, and optional `activeSpawnedId`. Home document/config state is not an owner of panel selection. Closing or toggling a panel retains its last selected leaf; the contract has no empty-id sentinel.

The browser Pack/base64 carriage was replaced with the same JSON text used by native. Both facets reject unknown fields, malformed JSON, empty/control/over-256-character identifiers, more than 64 spawned apps, duplicate spawned ids, an `activeSpawnedId` absent from the roster, carried text beyond the 65,536-character view-context capacity, and a complete serialized payload beyond that capacity. An absent `panelJson` may be initialized from the configured host app's first flattened leaf. Invalid carried data is never converted into that initial state.

## Host action route

Native `ShellState::dispatch_action` qualifies `setActivePanelTab` before app-command conversion and before `ProgramBridgeEntry::handle_action`. A configured host claims the route only when the controller is the exact active-session app controller or configured host-app controller. A claimed action must carry one bounded `tabId` that names an exact flattened leaf of the resolved app's `panel_tabs`; containers, stale ids and malformed arguments reject without guest fallthrough.

The reducer changes only `activePanelTab`, preserving `spawnedApps` and `activeSpawnedId`. It writes the active session's `panel_json`, mirrors that same payload into `DirectoryHomeProjection.view_state` when the projection owns the active Home instance, selects and opens the local Workbench/Display or Details/Settings side from the leaf's `PanelGroup`, and synchronizes shell chrome. Pointer-hit, palette and tutorial routes now address the active session's exact controller so they cross this reducer in host mode.

The browser route applies the same controller/leaf qualification before its guest dispatch. It writes only the strict session panel payload, preserves spawned focus, and opens the leaf's exact desktop anchor or mobile panel path. Installed app catalogue entries used for spawn effects, palette rows and the development probe are derived from loaded manifests; they are no longer copied into `panelJson`.

## Source ledger

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/📌️panel-state/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs`
- root `📜️script.ts`, `📋️project.json`, `.vscode/launch.json`, and `.vscode/🧩️launch.seed.jsonc` for registered orders `311.236` and `311.237`.

## Validation status

The earlier schema-only registered oracle was green in 4.9 seconds. Browser/native parity was then added. Oracle attempt r2 reached the test and found one incorrect relative import; the type owner and import were corrected. Fresh oracle r3 stopped before the target because the global Nx `@nxlv/python` plugin worker exited before connecting.

Registered `.237` native attempts have not yet reached the Home laws:

- r1 completed Nx graph construction and the updated oracle/strict TypeScript phase, waited on the shared Cargo artifact lock, and then observed the concurrently authored `engine/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs` as unavailable through its existing module path. The exact file now exists and the original three-parent path resolves to it.
- r2 tested a one-parent path against the wrong `wgpu/🧪️tests` location and failed. That probe was reverted; it left no source change.
- r3 used the restored exact path, completed Nx dispatch, and acquired the Cargo lane during the announced Pack inflater API cutover. It failed in Pack format before renderer compilation because the new allocation argument and incremental close API had not yet reached that caller.

The preserved pretest logs are `🗑️generated/home-host-panel-owner-native-r1.log`, `🗑️generated/home-host-panel-owner-native-r2.log`, and `🗑️generated/home-host-panel-owner-native-r3.log`. They establish no Home runtime result. After the Pack owner declared its Segment and mounted callers coherent, registered `.237` r4 compiled and passed both selected native laws on a 2 MiB stack: two passed, zero failed, 526 filtered, 0.02 seconds test runtime and 1 minute 49 seconds Nx wall time with zero cache hits. Its preserved log is `🗑️generated/home-host-panel-owner-native-r4.log`.

An additional strict-codec audit found that the browser decoder treated a present empty `panelJson` as absence. The language-neutral oracle now refuses that case, and `parsePanelState` recognizes only an actually absent property as no panel state. Registered `.236` r4 passed the independent Ajv/JSON Patch oracle and strict TypeScript compile in 8.8 seconds with zero cache hits. After duplicate cleanup, fresh registered `.236` r5 passed the same oracle, the Home source-negation scan and generated-manifest ownership checks in 11.5 seconds with zero cache hits. The preserved logs are `🗑️generated/home-host-panel-owner-oracle-r4.log` and `🗑️generated/home-host-panel-owner-oracle-r5.log`.

The follow-on host session identity cutover passed registered `.236` r8 with zero cache hits: 13 resolved-host-context vectors, 29 canonical directory/bootstrap checks, browser 10/10 and one related renderer integration law passed in a 6.5-second Nx run. Its evidence is `🗑️generated/home-host-panel-owner-oracle-r8.log`. No post-identity native or full Home result is claimed; registered `.237` remains queued after the independent review described in `home-host-session-identity-milestone.md`.

The native law uses a real `ProgramBridgeEntry::from_wasm` with a missing wasm path and a synthetic absent instance. Valid and invalid claimed host actions must complete or return a typed `host-panel.*` rejection before that bridge. A foreign controller is the control case and must reach the missing guest failure. The same law checks strict JSON, exact leaf/container distinction, preservation of two spawned apps and their active focus, side/kind/open chrome state, active-session persistence, and `DirectoryHomeProjection::active_session` restoration.

Post-removal registered `.237` r5 reran the same renderer gate successfully: two passed, zero failed and 526 filtered on the normal 2 MiB stack in 0.03 seconds. The required full Home library then compiled and ran all 90 tests. Eighty-five passed and five failed; the complete output is preserved in `🗑️generated/home-host-panel-owner-native-r5.log`. Every cleanup-affected Home config default, text operation, inverse, `SetClient` and retained config close law passed inside those 85 results. The five failures remain durable integration feedback:

1. `retained_command_fixture_matches_exact_routes_and_serde_json_boundaries` compares the fixture's command-declaration order with `HOME_RETAINED_TOOL_IDS`' lane-grouped order. Raw `HEAD` contains the same relative mismatch with the removed panel route present in both lists; the cleanup deleted only that exact row.
2. `directory_projection_round_trip_preserves_documents_and_rejects_corruption` rejects the unchanged projection fixture as `s.home.directory-projection-malformed`. Its test, fixture and `directory_from_json`/`directory_to_json` functions are outside the mirror-removal diff. This belongs to the open Home directory read-model contract.
3. `creates_studio_via_home_action` fails the tool proof on unchanged `applyDirectoryEventPage`: the runtime proof observes `generated_migrated={}` although the current source and authored manifest label the remaining 17 routes migrated. Deleting the panel route cannot justify changing those classifications; each route needs the broader truthful retained-execution audit.
4. `home_document_text_round_trips_through_the_store` reaches the current Store requirement and reports `edit history insertion requires its exact mutation retirement factory`. The test and Home document mutation sources were not changed by this slice. The Home document store needs its exact retirement factory under the broader ownership work.
5. `direct_owner_descriptor_surfaces_and_catalog_correspond` expects `.../any/🔣️oracle.json`. That file is absent in both the current tree and raw `HEAD`, and the failing structural test is untouched by this slice. Its catalog path/schema belongs to the broader Home document contract.

The panel acceptance is therefore the green native renderer result plus the green post-removal oracle/source/manifest gate. The full Home result remains red and is not represented as a passing app suite.

## Home duplicate removal

After native r4 accepted the host/session route, the obsolete Home app mirror was removed atomically. `HomeConfig.active_panel_tab`, `HomeConfigMutation::SetActivePanelTab`, the Home command/action and module mount, retained factory/proof/publication tables, command fixture, Home subset route row, all JSON/TypeScript/Rust/GraphQL/Proto field facets, language-neutral mutation vectors and generated Home manifest action are absent. Descriptor positions and Proto field numbers were compacted directly; no reserved legacy slot, alias or fallback decoder remains.

The registered verifier recursively scans Home `.rs`, `.ts`, `.json`, `.graphql`, `.proto`, `.py` and `.feature` source for all former field, mutation, command and action spellings. It also parses the generated plugin manifest and requires zero Home editor/viewer `setActivePanelTab` actions while requiring all three Studio actions to remain. The root plugin schema's Studio `SpacePlayRetainedCommandLimits` declaration remains unchanged.

Studio's `SpaceConfig.active_panel_tab` still duplicates host/session selection. Home's folded directory read model and receipts remain a Home-specific authenticated read model, while the former Home and Space client identity copies have moved to the host-owned `ViewModel.sessionIdentity` projection. Exact source and browser evidence for that follow-on milestone are in `home-host-session-identity-milestone.md`; its post-change native and full Home gate remain pending. The seventeen Architect batch-only actions listed in `architect-exact-window-ownership-implementation.md` also remain open and are not implied complete by the Home acceptance.
