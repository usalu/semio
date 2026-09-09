# Host Preference and Context Ownership Audit

## Scope

Static, read-only audit of the host-preference and view-context integration on 2026-09-08. The audit read the ticket integration notes, the root, OS, and shell `AGENTS.md` files, and traced the canonical OS config, shell, React host, native WGPU host, plugin refresh, and browser actor paths. No production files were changed and no build or test was run because native compiler sessions are active.

## Blocking Findings

### P1: The shell still owns a duplicate durable preference model

`semio-framework-os-config` defines `UiAppearance`, `UiChromeLayout`, `UiLocale`, `UiDriver`, `UiTheme`, and `UiPreferences` in [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs#L44) (lines 44-109), but the shell independently declares the same preference model in [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🦀️.rs#L79) (lines 79-89). The config schema even describes the diff as reusing a “shell-owned preference value” at lines 129-132, which contradicts the claimed canonical config ownership.

The shell then exposes durable `setUi*` command capabilities at [🟦️.ts](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🟦️.ts#L68) (lines 68-78) and applies those commands by mutating `ShellState` at lines 416-465; its Rust reducer does the same at [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🦀️.rs#L301) (lines 301-356). A caller of that command path can therefore mutate a noncanonical preference state without appending a config mutation.

Remove the duplicated durable preference values, schema fields, command vocabulary, capabilities, and reducers from the shell. Re-export config-owned types at its boundary if needed. Commands must append `UiPreferencesConfigMutation` events, then update the shell from the replayed config projection. Retain only genuinely ephemeral host drafts in the shell.

### P1: Native WGPU collapses concrete window identity to window kind identity

The native WGPU refresh path synthesizes one `ViewWindowInstance` per declared `window_kind`, assigning `id == window_kind_id`, then renders and stores documents keyed by `kind.id` in [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L3118) (lines 3118-3155). Its active-utility state is likewise described and updated per window kind at lines 2291-2293 and 7852-7865.

This erases split or otherwise repeated concrete window instances, gives them no independent utility state, and conflicts with the exact-instance `ViewModel::for_window_instance` contract. Build the refresh model from the live concrete dock/window instance collection, preserve every concrete id in `window_instances` and `active_utility_by_window_id`, and key rendered documents by concrete instance id. Resolve each instance’s body key from its window kind.

### P1: Native WGPU context-menu requests lose their target window context

`open_context_menu` resolves only a `surface_id`, then sends the plugin a raw view state augmented only with locale and terminology; it neither projects an exact window context nor includes `windowInstanceId` in the request ([🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L6222), lines 6222-6248). The plugin consumer treats the absent id as a panel and calls `for_panel()` at [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs#L27879) (lines 27879-27886).

Consequently, a context menu invoked in a native window reaches plug-ins as panel-scoped. Resolve the hit surface to its concrete window instance, construct the current full view model used by native refresh, project it with `for_window_instance(target)`, and send `windowInstanceId`. Use panel context only for an actual panel target.

### P1: Generic plug-in refresh passes window fields into panels unchanged

`plugin_refresh_ui` correctly derives exact context for each window, but renders panels using the unprojected request view state at [🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs#L27796) (lines 27796-27820), specifically line 27818. The shared contract requires `for_panel()` to clear `window_id`, `active_window_kind_id`, and `active_utility_id` ([🦀️.rs](../../../../../../../../🧰️framework/🔨️modules/🛂️manifest/🦀️.rs#L4442), lines 4442-4454).

The React host may build a refresh request from a current window state at [🟦️.tsx](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/⚛️react/🐚️ShellHost/🟦️.tsx#L3836) (lines 3836-3849), so the generic plug-in path can leak retained window identity into a panel render. In `plugin_refresh_ui`, construct one panel projection with `request.view_state.for_panel()` and pass it to all panel renders.

## Material Incomplete Consumption

### P2: Native WGPU uses shell preference types and projects only part of the canonical preference state

The native WGPU host imports its preference value types from `semio_framework_os_shell`, while importing config mutations separately ([🦀️.rs](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs#L23), lines 23-29). This preserves the erroneous shell ownership at the native boundary.

Its `UiPrefsSnapshot` holds only appearance, locale, terminology, driver, theme, layout, custom themes, and worker count (lines 12170-12181). The config replay can read all canonical fields, but native load consumes only the selected ids, layout, and custom themes at lines 10007-10017; the persistence projection at lines 12275-12308 similarly has no handling for custom drivers or keybinding overrides. There is no native path connecting `customDrivers` or `keybindingOverrides` to the active native projection.

Import all preference types from the config owner and project every canonical field in the native state. Native controls that cannot use a field yet must preserve its replayed value and avoid producing a competing shell-owned representation.

## Verified Paths

The browser actor pipeline follows the intended ordering and exact-instance projection. React posts `open` followed by `browser-actor-view-state` at [🟦️.tsx](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/⚛️react/🐚️ShellHost/🟦️.tsx#L4745) (lines 4745-4751), republishes current locale, terminology, instances, and utility state at lines 4786-4795, and the worker validates and retains that ephemeral state before refreshing the host at [🟦️.ts](../../../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L5135) (lines 5135-5142). Its render path resolves the concrete descriptor window id and uses window projection before emitting `surface-visible` at lines 1513-1536.

The WGPU bridge also sends the declared body key and concrete surface id to the renderer, so the surface-retention path itself is not the source of the findings above.

## Validation Status

This is a source trace only. It did not run tests, Bun/Nx tasks, Cargo, or native compilers.

## Resolution Evidence — 2026-09-09

The durable shell ownership finding was valid, while its statement that the current Rust shell independently declared the preference structs was stale. The shell schema now reexports `UiAppearance`, `UiChromeLayout`, `UiDriver`, `UiLocale`, `UiPreferences`, and `UiTheme` from `semio_framework_os_config::opening_config`. The nine durable UI preference fields and their nine `SetUi*` command variants, capability ids, reducers, JSON branches, TypeScript branches, and fixtures were removed from the shell. Its owned schema registry no longer republishes the unused preference aggregate, appearance, layout, or locale definitions; only driver and theme payload projections remain for the two ephemeral drafts. The schema gate distinguishes these owned parsers from its config-owned facade reexports. `bun nx run @semio-tech/framework-os-shell:test-quick` passed 7/7 tests. `bun nx run @semio-tech/framework-os-shell-rs:schema-check` passed its focused Rust test and confirmed that all 29 owned `$defs` agree with the Rust registry, rendered mirror, and TypeScript schema facade.

The native window and menu findings were valid and are fixed in the Dock and WGPU Shell hosts. `DockStackTab` now retains a concrete window id separately from its window-kind id. Layout parsing, active-window resolution, drag/drop, split insertion, and layout persistence carry both values. Native refresh enumerates the live dock instances, resolves each renderer body by kind, projects the shared `ViewModel` by concrete id, and stores its UI document by concrete id. Active utility, action, and document-surface routes use the same exact-instance-to-kind mapping. Native context-menu hit testing resolves the dock body under the pointer and supplies its concrete `windowInstanceId` with the current full `ViewModel`; an invocation outside a window body remains panel-scoped.

The incomplete full-preference consumption finding was valid and is fixed. Native WGPU imports canonical types and mutations directly from `semio-framework-os-config`. Its live chrome preference state and dirty snapshot now retain custom drivers and keybinding overrides in addition to custom themes. Loading derives them from replayed `UiPreferences`; persistence compares them and appends canonical `setCustomDriver`, `setCustomTheme`, and `setKeybindingOverride` mutations. The shared language-neutral fixture covers all scalar selections and the three custom maps with lossless nested driver/theme configuration. The TypeScript oracle run `bun nx run @semio-tech/framework-renderer-react:test -- long '../../../../🎚️UiPreferences/🟦️.ts' --run --silent=false --reporter=verbose` passed 1/1 and printed `[DEBUG] canonical OS UI preference events replayed, cross-shell propagated, isolated, and schema-validated`.

Focused native regressions were added for two concrete instances of one window kind surviving layout parse, drag/drop, and persistence; exact context-menu target selection with panel scope outside dock bodies; and full canonical preference event-log encode/decode/replay into the live native host projection. The first native Nx run reached a concurrent `semio-s-artifact-puzzle-3d` dependency and stopped on two `E0308` errors before compiling the WGPU renderer. After that owner corrected the locale signature, the static rerun reached the same dependency and stopped on four newer interaction changes: two missing `InteractionWrite` type errors, one call-arity error, and one immutable `marks` borrow. These failures occur before the renderer target; its owner is resolving them before the focused native laws can be recorded as passed.

Native OS command declarations now use `CommandDefinition::new` with explicit OS-owned icon ids after the generic command catalog stopped inferring ids from command names. Fullscreen keeps `code`; every preference command keeps `settings`; reset-dock keeps `panel-left`. The focused command-registry law checks the complete ordered icon set and its `os.setLocale` fixture uses the same explicit metadata.

## Handoff — 2026-09-09

The host lane changed these source areas:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell`: canonical config reexports, removal of durable UI preference fields and setters, scrubbed fixtures and regenerated schema mirror, plus the owned-schema check's distinction between local parsers and facade reexports.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences`: canonical mutation-event append, replay, projection, subscription, shared neutral fixture, and Ajv oracle.
- React `ShellHost` and `PluginRuntime`, the WGPU TypeScript bridge, and native `ProgramBridge`: full projected `ViewModel`, body key, concrete surface identity, context-menu target, and browser-actor view-state delivery.
- native `Dock` and `Shell`: concrete window-kind/instance separation, exact render/menu/action/utility routing, full canonical preference projection and mutation persistence, and explicit OS command icons.
- native focused test modules `wgpu-unit`, `wgpu-shell-input`, `wgpu-ui-prefs-themes-i18n`, and `wgpu-command-registry`.

No native test process from this lane remains live. Session `57580` completed with the upstream Puzzle3d failure described above, and shell schema-check session `49893` completed successfully. Once Puzzle3d compiles, rerun these focused host laws through Bun and Nx:

- `bun nx run @semio-tech/framework-renderer-wgpu:test-native -- concrete_window_instances_round_trip_without_kind_collapse -- --nocapture`
- `bun nx run @semio-tech/framework-renderer-wgpu:test-native -- context_menu_point_resolves_the_exact_concrete_window_instance -- --nocapture`
- `bun nx run @semio-tech/framework-renderer-wgpu:test-native -- canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript -- --nocapture`
- `bun nx run @semio-tech/framework-renderer-wgpu:test-native -- build_os_commands_covers_every_wired_setting -- --nocapture`

Successful laws emit `[DEBUG]` evidence for concrete window retention, exact native menu targeting, and full canonical preference replay. The host changes are ready for independent source audit while those dependency-gated executions continue.

Captured logs remain under `🗑️generated/native-concrete-window.log`, `🗑️generated/shell-schema-check.log`, `🗑️generated/shell-ts-test.log`, and `🗑️generated/ui-preferences-neutral.log` for the root ticket owner to consume before the ticket's final generated-output cleanup.
