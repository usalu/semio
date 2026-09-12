# Native Home Panel Action Owner Follow-up

## Scope and evidence status

This is a read-only source audit of native WGPU panel-tab routing. No production source was changed, no native target was run, and no Cargo command was run. Conclusions marked **source finding** come from the paths and lines below; they are not runtime observations.

## Determination

**Source finding:** the WGPU shell currently sends `setActivePanelTab` to Home whenever Home is the active session, even where the source action descriptor names Studio as its controller. The generic dispatcher uses `controller_id` only to find a plugin program; it builds the action address from the active session. Removing Home's action declaration or Home's action-to-command mapping before the native shell intercepts this route would therefore fault.

The correct existing owner seam is the host-action branch in `ShellState::dispatch_action`, immediately before generic program selection. It already owns the analogous `setActiveUtility` handling. A controller name by itself is not sufficient to infer the recipient app.

## Configured host versus current action recipient

`PluginHostConfig` configures `home` as the landing app and `studio` as the host app:

- [`🖥️hosts/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🖥️hosts/🦀️.rs:3) declares the config shape; its `space` row at line 9 is `{ landing_app_id: "home", host_app_id: "studio" }`.
- [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3092) resolves that config. `host_app()` uses `host_app_id` at line 3099, and `host_controller_id()` returns Studio's controller at line 3105.
- The native boot branch resolves the landing app from `landing_app_id`, creates the Home session, and seeds its `ViewModel.panel_json` at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3298`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3298). The panel seed uses the host catalogue tab at line 3302 while the session app is Home at lines 3301–3320.

Thus, the initial native shell has a **Home session** and a **Studio host controller**.

## Actual WGPU routes

| Origin | Descriptor controller | Active session at the relevant route | Generic action address | Current result |
| --- | --- | --- | --- | --- |
| Right panel tab hit | Studio, through `host_controller_id()` | Home at boot | Home | Local right-tab selection, then Home receives the action |
| Search-palette panel item | `session.app.controller_id` | Home at boot | Home | Home receives the action; source has no local active-tab update |
| Guided-tour right tab | Studio, through `host_controller_id()` | Whatever session is active | Home when Home is active | Same session-derived recipient issue as the direct right hit |
| Left panel tab hit | Studio only when the active session itself is Studio | Studio | Studio | Does not dispatch while Home is active |

The source evidence for those routes is:

- A direct right-tab hit first writes `active_right_tab`, then dispatches `setActivePanelTab` using `host_controller_id()` at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6710`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6710).
- Left-tab handling only obtains a controller after checking that `session.app.id == host_app_id`, at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7034`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7034). Its condition excludes an active Home session.
- Search-palette items dispatch through `session.app.controller_id` at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7290`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7290). With Home active, this is Home's controller. This block contains no `active_left_tab`, `active_right_tab`, or `panel_json` update.
- Guided-tour tab selection follows the same controller policy at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12093`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12093).

## Why a Studio controller still invokes Home

`ShellState::dispatch_action` finds a program using the descriptor's `controller_id`, but then constructs the action invocation from `self.session`:

1. It selects a program by controller, falling back to the session plugin, at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4910`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4910).
2. It then constructs `ActionAddress { plugin_id, app_id, ... }` from `session.plugin_id` and `session.app.id` at line 4920. The descriptor controller is not used as the address app.
3. The WGPU bridge forwards the resulting invocation to the app runtime at [`🌉ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:196`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:196).
4. Native plugin handling validates that the addressed app's window kind owns the action at [`🔌️plugin/🦀️.rs:27526`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27526).

Consequently, source predicts that a Home session whose action is allowed to reach this generic path needs Home's `setActivePanelTab` action declaration. If the Home declaration is removed, the runtime validation branch returns `window kind {kind} does not own action {action}` at [`🔌️plugin/🦀️.rs:27544`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:27544). Home's mounted editor main kind is `s-home-main` at [`🏠️main/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:30), so the predicted Home fault is `window kind s-home-main does not own action setActivePanelTab`.

If the declaration is retained but the Home mapping is removed, Home's current `command_from_action` mapping at [`✏️editor/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:486) instead predicts its `s.home.unhandled-action` fault. These are source-derived outcomes, not executed failures.

## Exact host handler to extend

The existing host-owned action branch is `ShellState::dispatch_action`'s `setActiveUtility` case at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4890`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4890). It updates shell-owned UI state before the program-selection block at line 4910. This is the narrow native ownership boundary to extend for a host panel-tab action, with the action handled before `ProgramBridge` is selected.

The implementation also needs one source-route adjustment: Search palette currently manufactures `setActivePanelTab` with Home's controller while Home is active. An intercept restricted only to `host_controller_id()` would catch direct/guided right selection but would still let this palette route invoke Home. The palette item must route recognized shell panel selection through the existing host action policy, or the host branch must recognize the same narrowly-defined shell panel action for the current host configuration. The current source offers no evidence that either is already true.

Do not treat every arbitrary app action with the same spelling as a shell action. Qualification must use the existing configured host/landing relationship and the shell's own panel-tab catalogue, rather than a new plugin-local config or an unrestricted action-name catch-all.

## Panel state, persistence, and demonstrated missing routes

`SpacePanelState` is explicitly the host-owned state serialized into `ViewModel.panel_json`; it includes `active_panel_tab`, spawned app entries, and active spawned id at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:707`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:707). Its decode/encode helpers only read or write `view_state.panel_json` at lines 3143–3149.

**Source finding:** ordinary native tab selection currently does not synchronize that model.

- The direct right route updates only local `active_right_tab` before generic dispatch. The renderer reads that local field when drawing the panel at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11156`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11156).
- Normal mutation application changes `ViewModel.panel_json` only for the host `setPanel` operation at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5673`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5673). Home's current command emits a config mutation only at [`⚙️set-active-panel-tab/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-active-panel-tab/🦀️.rs:9); it does not issue `setPanel`.
- The native persisted panel layout deliberately stores panel open/kind/width data but no active tab id at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2713`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2713); snapshot and restore follow at lines 4086–4165.
- `DirectoryHomeProjection` retains the session view state while switching away from Home and reinstates it when returning to the managed landing app at [`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5835`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5835). Since direct selection did not alter the retained `panel_json`, this path cannot restore its exact selection.

The demonstrated source gap is therefore twofold: a direct right click gets an immediate local visual selection but has no `panel_json` write for retained-session/reopen state; a palette selection has neither a local active-tab write nor a `panel_json` write before it reaches Home. Tutorial snapshot code separately serializes local tabs and restores a supplied panel JSON at lines 9879–9961; it is tutorial-specific and does not establish an ordinary click persistence route.

## Home mounted reader check

No `active_panel_tab` read was found under the mounted Home editor or viewer mode trees. The editor main renderer receives the config and renders directory rows using the directory and `client_id` at [`🏠️main/🦀️.rs:174`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs:174); the viewer main follows the same directory/client input pattern at [`🏠️main/🦀️.rs`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs:68). This confirms source has no mounted Home editor/viewer render producer to preserve when removing the Home config field. It does not substitute for a runtime render check.

## Smallest executable native parity law

Extend the existing WGPU panel-anchor test module at [`🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:83), which already verifies shell panel layout snapshot/restore, with one dispatch-level law once the narrow host branch is extracted into its existing test seam:

> Given the configured Space host (`landing = home`, `host = studio`), an active Home session, and a shell panel tab `x`, native dispatch of the host panel-selection action updates the local shell selection and the session `ViewModel.panel_json` decoded as `SpacePanelState(active_panel_tab = x)` while preserving spawned entries and active spawned id. It completes without entering `ProgramBridge`; switching away and back through `DirectoryHomeProjection` exposes the same decoded panel state.

That one law covers the unsafe current combination: Studio controller plus Home session. Add the same assertion for a search-palette produced selection so that its descriptor route cannot silently retain the Home action dependency. The executable test must use the existing bridge/test seam or a narrowly extracted private shell-state reduction; no new Home command, plugin-local duplicate, or general configuration object is required.

## Proposed implementation ledger

Root implementation-preparation review confirmed the native `SpacePanelState` is declared in Shell's WGPU implementation while the browser type is exported from Shell's TypeScript module. The existing native panel-anchor law seam constructs a real ShellState but has no host program manifest by default; a dispatch-level law needs the configured Space/Home/Studio relationship and recognized panel catalogue, not just a synthetic unqualified action spelling. No Home/native production changes or tests were made during this preparation. Preserve the domain-neutral host boundary and the separate spawned-instance owner while defining the shared schema; do not introduce another Home configuration record.

Further root preparation: `ProgramBridgeEntry::from_wasm` accepts an explicit manifest without instantiating a guest; it stores a KernelClient handle and the path. A native acceptance fixture can therefore provide the real configured Space/Home/Studio app roles while deliberately having no live guest instance: a correctly intercepted panel action must complete and update panel state, whereas generic guest dispatch cannot satisfy it. Reuse the existing command-registry AppDefinition fixture builder through a test-only shared seam rather than duplicate its large definition. Observe direct and palette descriptors, panel JSON and local selection; distinguish an unavailable guest error from accepted host handling. No fixture/test code is authored yet.

Qualification must inspect the actual active session's recognized panel leaves as well as any host-contributed catalogue. Palette currently enumerates `session.app.panel_tabs`, and the guided tour checks the same metadata; accepting only Studio's first catalogue tab would miss legitimate shell tabs. Do not infer that a tab's physical right/left position or a matching action spelling alone gives it host authority. Root has not yet resolved the full cross-app/host panel schema; this remains a required implementation design point, not authorization to add an unrestricted intercept.

| Area | Existing file | Smallest responsibility |
| --- | --- | --- |
| Native host action owner | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | Handle qualified host panel selection before generic program dispatch; synchronize `SpacePanelState` into the active session view state; make palette use that route. |
| Native acceptance | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` | Assert the parity law and retained-session restore. |
| Home cleanup, only after native route passes | Home manifest/mapping/config and their existing tests | Remove the action/config ownership now proven absent from mounted renders. |

No native targets were executed for this audit. The listed routes, gaps, and predicted faults require validation by the proposed executable parity law before claiming runtime behavior.
