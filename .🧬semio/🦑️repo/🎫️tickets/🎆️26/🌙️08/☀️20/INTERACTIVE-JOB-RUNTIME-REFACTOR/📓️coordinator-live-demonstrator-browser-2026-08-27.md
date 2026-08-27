# Live Demonstrator Browser Verification

## Surface and Method

The coordinator used the in-app browser-control skill against the already-running local Vite service at `http://127.0.0.1:6035/`. The server was discovered from the actual listening process, not started or modified by this check. No user browser storage, credentials, or private session state was read. The skill kept interactions on visible controls and read-only DOM/console inspection.

The landing page rendered six German-labelled app launch buttons and onboarding. After skipping onboarding, the Generator/Procedural3d shell rendered Workflow and Vorschau panels while other app shells remained in plugin-loading status. Opening Generator exposed its onboarding and three sliders. This proves partial live UI rendering, not successful boot of all six apps or a current-source Wasm rebuild.

## Observed Runtime Failures

- At 2026-08-27T03:03:40.488Z the console recorded `[DEBUG] action failed interactionSelect ... validation failed: edit history insertion requires its exact mutation retirement factory`.
- At 2026-08-27T03:04:16.450Z the demonstrator#2 worker trapped with `Error: unreachable`; the visible stack passed through `QualifiedBoundedFirstStepProof`, `VcsArtifactApp<EditorApp<CadPlayApp>>::with_registry_on_bus`, and `PluginBuilder::editor`. The shell reported `Framework OS boot failed`.
- Current source independently explains the CAD startup failure: `AppActionRegistry::tool_job_registration` requires `row.factory == BOUNDED_FIRST_STEP_FACTORY` even for concrete app factory declarations such as `CadRetainedCommandJobFactory`; app factory registration occurs only after this check. The static source verifier already admits the concrete factories, so its green result does not establish runtime startup parity. A dedicated executor has been assigned the exact runtime proof-to-registration join and regression test, without relaxing owner/factory validation.
- Current source installs an interaction-store retirement owner, but this check has not rebuilt the served Wasm. The interaction failure therefore remains unassigned between current-source defect and served-artifact divergence until a fresh build/reload proves which applies.

## Accessibility Finding

All three visible Workflow sliders had no `aria-label` and no `aria-labelledby`; the accessible DOM snapshot listed them merely as `slider`. Their values/ranges were 6 in [0,10], 0.5 in [0.1,2], and 6 in [3,12]. These controls need meaningful localized accessible names before the all-app accessibility gate can pass.

## Actual Parameter Interaction

After opening Generator and skipping its onboarding, the coordinator pressed ArrowRight on the first visible Workflow slider. Its DOM value changed from 6 to 6.5, but the console then recorded at 2026-08-27T03:06:40.915Z:

```text
[DEBUG] action failed nodeGraphEdit ... typed command 'nodeGraphEdit' remains fail-closed until its live immutable roots and publication primitives are admitted into TypedCommandFullOperationJob
```

The visual control change is therefore not evidence of an accepted document edit. Additional console events reported interactionHover/interactionSelect history-owner rejection, extension-load timeouts, a missing pinned Sourcing app, and a failed dynamic module load. A concurrent peer Wasm build was observed separately, so those loading events need a stable rebuilt-artifact rerun before attribution. The proven source-level hardcoded-factory check and the explicit nodeGraphEdit fail-closed response are not resolved by that caveat.

## Required Follow-Up

Repair and test the exact runtime factory join, rebuild the actual served Wasm through the canonical task runner, reload, rerun startup/interaction checks, and then run full command/cancellation/replay/timing and device-size tests. Do not report the current six-app surface as end-to-end working.

## Parameter Path Source Follow-Up

The Flow-backed renderer's slider callback still calls `session.setSliderValue`, then `commitFixtureThrottled`. That function obtains `session.documentJson()` and dispatches `nodeGraphEdit` with a complete `setFixture` JSON string. The 80 ms throttle reduces dispatch frequency but does not split document serialization, fixture replacement, or diffing into bounded operations.

Procedural3d's `node-graph-edit` handler parses the complete operations array, clones a Flow host from the fixture, loops over all suboperations, and calls `commit_fixture` for the full result. Its source still declares a generic bounded-first-step proof even though the live framework rejects the operation before this path can run. The ordinary slider needs an exact small parameter-edit feature plus retained authoritative Store publication; whole fixture replacement must be its own resumable import/edit feature. Merely relabelling the generic catalog would not fix the observed interaction.

The shared graph slider accessibility repair is queued with the Store-sealer executor after its source handoff. It covers the schema producer in Infinite's DAG host and every `GraphSliderOverlays` consumer, with meaningful localized labels and keyboard tests. It does not by itself repair parameter publication.

The existing Demonstrator TypeScript `test` route only prints `[DEBUG] demonstrator ts ok`; it is not a behavioral app gate. Fresh artifact verification must use actual native/Wasm tests and the live browser workflow. The shared OS-dev `plugin` route owns canonical plugin builds; its `build` route additionally sets ship mode and builds the renderer/bundle, so these are different verification scopes.

## Active Generator Feature Packet

The coordinator assigned the shared renderer and Procedural3d exact parameter feature to the Store/graph executor. Its acceptance gate is a small typed widget/value command, an actual registered app-owned factory, retained candidate/inverse preparation and Store sealing under the production one-item/4,096-byte grant, and authoritative undo/preview publication. The slider callback must not request `documentJson()` or submit whole-fixture replacement; latest-wins cancellation and keyboard/drag completion must be tested. Other whole-fixture editing routes remain separately unfinished.

The Flow executor has now preserved authored labels through the required Widget, NodeChrome, WidgetDescriptor, facade, and exact Procedural2d/3d mounted codecs/fixtures; source tests pass, native tests remain queued. The five shared graph DOM accessibility tests passed earlier. These two results do not establish that the currently served Wasm has the labels or accepts slider edits.
