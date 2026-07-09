---
name: Restore old S navbar parity
overview: Restore the old S/playground navbar structure in the shared `os-shell.tsx` renderer (used by all 25 plugin playgrounds) and make the example dropdown work generically for any plugin that declares examples, not just S.
todos:
 - id: generic-example-dropdown
   content: "Make the navbar example dropdown generic: derive from the active session's own plugin manifest examples, dispatch to session.app.controllerId, fix activeExampleId default/reset"
   status: completed
 - id: restore-panel-toggle-and-modes
   content: Add activeLeftPanelKind/activeRightPanelKind state, split leftPanelTabs/rightPanelTabs accordingly, rebuild panelToggles as the old 4-icon (display/workbench/details/settings) group, and add a mode ButtonGroup navbar item (set App chrome={false})
   status: completed
 - id: remove-new-navbar-items
   content: Remove navHistory, breadcrumb, search/find toggles, and inline theme/compact/expertise controls from navbarItems; keep underlying keybindings/dialogs; clean up now-unused helpers
   status: completed
 - id: update-e2e-home-nav
   content: Update s-studio-e2e-verify.mjs's home-navigation step to use the "← Home" content-bar button instead of the removed breadcrumb
   status: completed
 - id: verify-generic-and-s
   content: Run framework-renderer-react vitest, rerun S studio E2E, and manually smoke-test the example dropdown/panel toggles/mode switcher on at least 2 non-S playgrounds (e.g. draw, puzzle2d)
   status: completed
isProject: false
---

## Root cause

`os-shell.tsx` (`framework/renderer/react/os-shell.tsx`) is the **one shared renderer** used for every plugin playground (`draw`, `puzzle2d`, `raster`, `s`, etc. -- see `framework/product/os/dev/js/plugin-registry.ts`), replacing the old `PlaygroundView` (`framework/product/playground/renderer/react/index.tsx` at commit `f8376e848`, 2207 lines). During the Rust/WIT plugin migration, the navbar was substantially redesigned and the example dropdown was hardwired to the S plugin only, so:

- 23 of 25 plugins declare `.example(...)` in their manifest builder (confirmed in `draw`, `puzzle2d`, `puzzle3d`, `puzzle5d`, `raster`, `writer`, `note`, `layout`, `cad`, etc.) but only S gets a working dropdown, because `os-shell.tsx` line ~1233-1237 does:
  ```ts
  const sPluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === "s")?.manifest, [loadedPlugins]);
  ```
  and the navbar item is gated on `session?.app.id === S_PLAY_APP_ID` (line 1273).
- The navbar composition itself diverged structurally from old S:

| Old navbar (`PlaygroundView`)                                                                                                                   | New navbar (`os-shell.tsx`)                                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| logo + app title                                                                                                                                | logo + app title (kept)                                                                                                        |
| example dropdown (`NavbarExampleSelect`, any app with `exampleContribution`)                                                                    | example dropdown, S-only                                                                                                       |
| fill                                                                                                                                            | back/forward/up history `ButtonGroup` (new, not in old)                                                                        |
| 4-icon panel toggle group: display / workbench / details / settings (`PanelToggleGroup`, switches `activeLeftPanelKind`/`activeRightPanelKind`) | breadcrumb (new, not in old)                                                                                                   |
| mode `ButtonGroup` (icon+label per `app.modes`)                                                                                                 | example dropdown (S-only, as above)                                                                                            |
| _(theme/compact/expertise lived in a "Settings" side-panel tab, not the navbar)_                                                                | fill, search toggle (new), find toggle (new)                                                                                   |
|                                                                                                                                                 | 2-icon panel toggle (left/right only, no display/workbench/details/settings split)                                             |
|                                                                                                                                                 | theme `Select`, compact `Toggle`, expertise `Select` inlined directly in navbar (new)                                          |
|                                                                                                                                                 | _(no mode switcher in navbar -- `App` component renders an internal `Select` dropdown instead when `chrome={modes.length>1}`)_ |

The underlying settings/display panel-tab infrastructure already exists and is reused (`framework/renderer/react/os-chrome-panels.tsx`: `createFrameworkDisplayPanelTabs`, `createFrameworkSettingsPanelTab`), it's just flattened into one tab list per side instead of toggled via dedicated navbar icons.

## Decisions (confirmed with user)

1. **Example dropdown fix is generic** -- driven by the active session's own plugin manifest, not hardcoded to S.
2. **Navbar restored to old structure** -- move theme/compact/expertise back to the Settings panel tab only, restore the 4-icon display/workbench/details/settings panel toggle group and the mode `ButtonGroup`; drop the newly-added history-nav buttons, breadcrumb, and search/find toggles from the navbar (their underlying keybindings/dialogs stay functional, just not exposed as navbar buttons -- flagged below in case full removal is preferred instead).

## Changes

### 1. Generic example dropdown -- `framework/renderer/react/os-shell.tsx`

- Replace the S-only `sPluginManifest`/`exampleOptions` derivation with one based on the **active session's own plugin**: `loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest.examples`.
- Change the navbar gate from `studioMode && session?.app.id === S_PLAY_APP_ID && exampleOptions.length > 0` to just `exampleOptions.length > 0`.
- Dispatch `{ controllerId: session.app.controllerId, command: "setActiveExample", args: { exampleId } }` (generic controller id, already used elsewhere in this file e.g. line 1357) instead of hardcoded `S_PLAY_CONTROLLER_ID`.
- Fix `activeExampleId` default: currently `useState("demo")`, but example ids vary per plugin (`"empty"`, `"default"`, `"nakagin"`, etc. -- confirmed via `grep .example(` across plugins). Initialize/reset to `exampleOptions[0]?.id` when the active app/plugin changes.

### 2. Restore 4-icon panel toggle group + mode switcher -- `framework/renderer/react/os-shell.tsx`

- Add `activeLeftPanelKind` (`"workbench" | "display"`) and `activeRightPanelKind` (`"details" | "settings"`) state, mirroring old `PlaygroundView`.
- Split `leftPanelTabs` (currently `[...frameworkDisplayTabs, ...pluginLeftTabs]` always combined) into two arrays selected by `activeLeftPanelKind`, and `rightPanelTabs` similarly by `activeRightPanelKind`, using the existing `frameworkDisplayTabs`/`frameworkSettingsTab` from `os-chrome-panels.tsx`.
- Rebuild `panelToggles` as a 4-item `PanelToggleItem[]` (display, workbench, details, settings) that both sets the active kind and toggles panel visibility, matching old `playgroundPanelToggleItems` logic exactly.
- Add a "modes" navbar item: a `ButtonGroup` over `session.app.modes`, highlighting the active mode (reusing the existing `onActiveModeChange` handler already wired to `<App>`), and set `<App chrome={false}>` since mode switching now lives in the navbar instead of `App`'s internal `Select`.
  - Note: current WIT `mode-definition` (`framework/wit/world.wit` line 461) has no `iconId` field, so restored mode buttons will be text-only (old had icons) -- calling this out as a minor, accepted gap rather than extending the WIT protocol.

### 3. Remove new navbar additions -- `framework/renderer/react/os-shell.tsx`

- Remove the `navHistory` `ButtonGroup` (back/forward/up) and `breadcrumb` items from `navbarItems`.
- Remove `search`/`find` `Toggle` items from `navbarItems`.
- Remove the inline `theme` `Select`, `compact` `Toggle`, `expertise` `Select` items from `navbarItems` (already reachable via the restored Settings panel tab).
- Remove the `uri` navbar item (was redundant with breadcrumb; studio URI is still visible via the existing "← Home" content-bar button flow).
- Keep `mod+[`/`mod+]`/`mod+up` (history) and `mod+p`/`mod+f` (search/find) keybindings and their dialogs functional -- only the visible navbar buttons are removed, per the "drop from the navbar" framing. If full removal (including keybindings/dialogs) is actually wanted, flag that as a follow-up.
- Clean up now-unused helpers/imports if nothing else references them (`uriToBreadcrumbItems`, `Breadcrumb` import, `navigateFromBreadcrumb`).

### 4. Update E2E for the removed breadcrumb -- `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`

- The "breadcrumb home navigation" step currently clicks `[data-slot="breadcrumb-link"]`. Since the breadcrumb is removed, switch this step to click the existing "← Home" content-area button (`studioHomeBar` in `os-shell.tsx`, matches old S's exact pattern), which is the same mechanism old S used.

## Verification

- `bun nx run @semio-tech/framework-renderer-react:test` (vitest).
- Re-run S studio E2E (`S_STUDIO_URL=http://127.0.0.1:6070/ node .repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`) with the updated home-navigation step.
- Manually smoke-test at least 2 non-S playgrounds (e.g. `draw`, `puzzle2d` via `SEMIO_PLUGIN=draw`) to confirm: example dropdown now appears and switches fixtures, the 4-icon panel toggle group works, and mode buttons render for apps with multiple modes.

## Out of scope / follow-up

This pass addresses the two concretely identified gaps (navbar structure, example dropdown). The user's "etc" suggests more differences may exist between old S and new S; a systematic full-feature audit of the remaining ~2000 lines of old `PlaygroundView` (footer items, keybindings surface, toolbar slot, mobile layout, presence peers) versus `os-shell.tsx` would be a good follow-up ticket once this lands, since cataloguing every remaining divergence without a specific pointer is a much larger undertaking.
