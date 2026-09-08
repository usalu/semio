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
