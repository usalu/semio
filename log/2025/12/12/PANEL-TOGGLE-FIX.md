---
slug: PANEL-TOGGLE-FIX
summary: Fix panel toggle signature mismatch and investigate store issue
---

# Previously

User reported that pressing the panel toggle (left and right panel group) in the apps doesn't work.

# Plan

1. Investigate panel toggle implementation in `shared.ts` and app files
2. Identify signature mismatch in `togglePanel` functions
3. Fix signature mismatch in `useHomeCommands`, `useKitAppCommands`, `useTypeAppCommands`
4. Test with Playwright MCP
5. Investigate deeper issue with `useAppCommands()` returning undefined app
6. Add panel toggle test to `sketchpad.test.ts`

# Changes

## Signature Fix

Fixed `togglePanel` signature mismatch in three locations where the function was being called with `(origin, panelKey)` but defined to only accept `(panelKey)`:

- `@/js/js/sketchpad/Sketchpad.tsx:13568` - `useHomeCommands().togglePanel`
- `@/js/js/sketchpad/Type.tsx:455` - `useTypeAppCommands().togglePanel`
- `@/js/js/sketchpad/Kit.tsx:1057` - `useKitAppCommands().togglePanel`

All three now correctly accept `(_origin: string, panelKey: keyof PanelVisibility)`.

## Root Cause Discovery

The signature fix was correct but panel toggles still don't work due to a **pre-existing bug** in `useAppCommands()`:

The `PanelToggles` component uses `useAppCommands()` which relies on `store.kitApp(kitGuid)` to get the app store. However, `store.kitApp(kitGuid)` returns `undefined` even when the navigation path correctly contains the kit guid.

Debug output confirmed:

- `navigation: /kits/f042c2a4-3ba5-44b0-b22c-0ae8f568aacc` (correct)
- `appType: kit` (correct)
- `kitGuid: f042c2a4-...` (correct)
- `app: undefined` (BUG - should have the kit app store)

This causes `togglePanel` to silently return without doing anything.

## Test Addition

Added `"Panel Group Toggle"` test to `@/js/js/sketchpad.test.ts` that verifies:

- Right panel group toggle (`semio.sketchpad.navbar.panelToggle.right`) exists and is clickable
- Left panel group toggle (`semio.sketchpad.navbar.panelToggle.workbench`) exists and is clickable

The test documents the known issue with store initialization and doesn't assert on panel visibility.
