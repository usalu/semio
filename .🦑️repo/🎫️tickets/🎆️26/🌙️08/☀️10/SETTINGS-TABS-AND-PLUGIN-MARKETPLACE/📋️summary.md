# Settings Tabs And Plugin Marketplace Summary

## Outcome

- Replaced the separate General, Theme, and Keybindings panel toggles with one Settings branch whose internal tabs are General, Theme, and Hotkeys.
- Updated hotkey navigation to open the nested Settings → Hotkeys path.
- Replaced the separate Plugins and Extensions panel toggles with one Marketplace panel.
- Integrated extensions beneath their owning plugin via `extendsHost`; extensions without a listed host plugin remain visible beneath a synthetic missing-host section.
- Kept extension URL/file installation inside Marketplace and preserved plugin and extension lifecycle actions through one explicit host API.
- Updated the renderer barrel exports and English chrome label from Keybindings to Hotkeys.
- Updated the WGPU parity comments to reference the singular React Settings builder.

## Verification

- `bun nx run @semio-tech/framework-renderer-react:test-long -- -t createFrameworkSettingsPanelTab --reporter=verbose`
  - Passed: 1 test, 303 skipped.
- `bun nx run @semio-tech/framework-renderer-react:test-long -- -t createFrameworkMarketplacePanelTab --reporter=verbose`
  - Passed: 6 tests, 298 skipped.
- `bun nx run @semio-tech/framework-renderer-react:lint`
  - Passed: region/host-contract lint.
- `git diff --check HEAD` over the implementation files
  - Passed with no whitespace errors.
- Legacy React API/id scan
  - No remaining `createFrameworkSettingsPanelTabs`, Plugins/Extensions host APIs, or `framework.settings.plugins` / `framework.settings.extensions` references.

## Verification Limitations

- The normal renderer test target exceeded its repository-enforced 15-second budget after Vitest started; it did not report an assertion or transform failure before termination.
- An unfiltered `test-long` attempt became inactive without completing and was interrupted after approximately two minutes; the targeted tests above were then run successfully.
- The existing `http://127.0.0.1:6070/` launch was reachable, but both initial load and a clean reload stopped at Vite's `[plugin:vite:esbuild] The service is no longer running` overlay before the app mounted. Browser console error/warning capture was empty because the Vite overlay replaced the application. Runtime dock interaction was therefore not claimed as verified, and the shared dev process was left untouched.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ChromePanels/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
