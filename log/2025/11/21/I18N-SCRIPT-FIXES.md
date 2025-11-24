---
date: '2025-11-20T23:00:00.000Z'
slug: I18N-SCRIPT-FIXES
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migration from 2025-11-21_I18N-SCRIPT-FIXES.md
model: unknown
---
# i18n Script Fixes - 2025-11-21

## Issues Fixed

### Missing ID Detection

1. **Action IDs** - Added detection for `actionId` attributes in both string and JSX curly brace formats:
   - `semio.sketchpad.app.home.createTemporary`
   - `semio.sketchpad.app.home.createKit`
   - `semio.sketchpad.app.home.createLocal`
   - `semio.sketchpad.app.home.createRemote`

2. **Tooltip IDs** - Added detection for `tooltipId` attributes in both formats

3. **Panel Toggle Tooltips** - Enhanced detection for dynamically generated panel toggle IDs:
   - `semio.sketchpad.navbar.panelToggle.workbench.show`
   - `semio.sketchpad.navbar.panelToggle.tools.show`
   - `semio.sketchpad.navbar.panelToggle.toolbar.show`
   - `semio.sketchpad.navbar.panelToggle.hud.show`
   - `semio.sketchpad.navbar.panelToggle.stats.show`
   - `semio.sketchpad.navbar.panelToggle.details.show`
   - `semio.sketchpad.navbar.panelToggle.chat.show`
   - `semio.sketchpad.navbar.panelToggle.settings.show`

4. **Tooltip Namespace** - Added support for `tooltip.*` and `settings.*` prefixes:
   - `tooltip.manual`
   - `tooltip.tutorial`

5. **Metadata Keys** - Added `description` to the metadata keys list for proper validation

## Script Changes

### Pattern Detection Enhancements

Added new regex patterns to `PATTERNS` array:

- `actionId` with JSX curly braces: `/\bactionId\s*=\s*\{["']([^"']+)["']\}/g`
- `tooltipId` patterns (string and curly brace formats)
- `createPanelDefinition` pattern to extract panel definition IDs

### Scanner Improvements

Enhanced `scanSourceFiles()` function:

- Added dynamic panel toggle pattern detection
- Added inline tooltip construction pattern detection
- Extended ID prefix filter to accept `tooltip.` and `settings.` in addition to `semio.sketchpad.`

## Results

### Before Fix

- **Total IDs detected**: 389
- **Missing entries**: 14
- **Valid**: 96.5%

### After Fix

- **Total IDs detected**: 403
- **Missing entries**: 0
- **Valid**: 100.0%

## Added Entries

All missing translations were successfully added to both `de.json` and `en.json` locale files with proper structure:

- All entries have `label.normal` and `label.beginner` fields
- Panel toggle tooltips are properly nested under their respective panel keys
- Action descriptions follow the established pattern

## Validation

Run `node scripts/i18n.mjs validate` to verify all entries are present.
Run `node scripts/i18n.mjs report` to generate a detailed report.
Run `node scripts/i18n.mjs fix` to add missing entries, fix placeholders, and clean unused keys.
