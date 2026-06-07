# Summary

Implemented settings panel host parity with display:

- `SettingsHostApi`, `SettingsHostContext`, `useSettingsHost`, `createFrameworkSettingsPanelTabs`, and general settings tree (compact, expertise, optional mode) in platform renderer.
- Auto-merge framework settings tabs in `resolveAppPanelTabsByKind` for all apps using `PlatformView` / `mountPlatform` (including sketchpad).
- Migrated navbar mode select and footer compact toggle into the settings panel; removed duplicates.
- Playground: right-side `details` | `settings` switching, settings navbar toggle, dynamic surface chrome.
- UI i18n: `ui.settings.tab.*` (en/de) and `readStoredUiChromeExpertise` / `writeStoredUiChromeExpertise`.
- Tests extended in platform renderer and ui/react.

## Files

- `ui/react/index.ts`
- `framework/product/platform/renderer/react/index.tsx`
- `framework/product/playground/renderer/react/index.tsx`
- `.repo/🎫/26/06/03/SETTINGS-PANEL-HOST-PARITY/ticket.md`
