# Previously

State management refactor started. Components use a mix of:

- `use*Commands` hooks that return command functions
- XState-based state management with selectors
- Yjs-based state sync (should only be for Kit data)

Goal: Migrate to triadic hooks pattern: `[STATE, SETSTATE, CANSETSTATE] = useSelector()`

# Plan

1. Create triadic hooks for global settings (theme, language, expertise, mode, layout) ✅
2. Refactor Home.tsx to use triadic hooks instead of useSketchpadCommands ✅
3. Refactor Type.tsx to use triadic hooks ✅
4. Run tests to verify refactoring ✅
5. Update AGENTS.md with new triadic hooks examples ✅

6. Refactor Kit.tsx settings to use triadic hooks ✅
7. Refactor Design.tsx settings to use triadic hooks ✅
8. Refactor Quality.tsx settings to use triadic hooks ✅

## Remaining (larger architectural tasks):

8. Remove yjs import from Design.tsx (DesignStore still uses yjs internally)
9. Extend state machine with proper states for CANSETSTATE
10. Refactor remaining kitAppCommands usages (navigation, creation)

# Changes

## Sketchpad.tsx

- Added triadic hooks for global settings:
  - `useThemeTriadic(): GranularHookResult<Theme>`
  - `useLanguageTriadic(): GranularHookResult<string>`
  - `useExpertiseTriadic(): GranularHookResult<Expertise>`
  - `useModeTriadic(): GranularHookResult<Mode>`
  - `useLayoutTriadic(): GranularHookResult<Layout>`
  - `useFullscreenTriadic(): GranularHookResult<boolean>`

## Home.tsx

- Refactored `SettingsContent` component to use triadic hooks internally
- Removed props from SettingsContent (setTheme, setLanguage, etc.)
- Added disabled state to UI elements based on `canSet*`
- Updated imports to use triadic hooks instead of read-only hooks
- Removed unused destructured values from useSketchpadCommands

## Type.tsx

- Refactored `SketchpadSettingsContent` component to use triadic hooks internally
- Removed dependency on `useSketchpadCommands` for settings
- Added disabled state to UI elements based on `canSet*`
- Updated imports to use triadic hooks (useThemeTriadic, useLanguageTriadic, etc.)
- No yjs import needed (Type.tsx was already clean)

## AGENTS.md

- Added global settings triadic hooks to the examples section
- Documents: useThemeTriadic, useLanguageTriadic, useExpertiseTriadic, useModeTriadic, useLayoutTriadic

## Kit.tsx

- Refactored `SettingsContent` component to use triadic hooks internally
- Removed dependency on `useSketchpadCommands` for settings
- Added disabled state to UI elements based on `canSet*`
- Updated imports to use triadic hooks (useThemeTriadic, useLanguageTriadic, etc.)
- Note: Still uses `sketchpadCommands` for navigation (navigateToDesign, navigateToType, etc.)

## Design.tsx

- Refactored `SketchpadSettingsContent` component to use triadic hooks internally
- Removed settings-related destructuring from `sketchpadCommands`
- Added disabled state to UI elements based on `canSet*`
- Updated imports to use triadic hooks
- Note: Still imports yjs (used by DesignStore class which is still active)
- Note: Still uses `sketchpadCommands` for navigation

## Quality.tsx

- Refactored settings section to use triadic hooks internally
- Removed settings-related destructuring from `sketchpadCommands`
- Added disabled state to UI elements based on `canSet*`
- Updated imports to use triadic hooks
- Note: Still uses `sketchpadCommands` for `setActiveInteraction` in FunctionNode and QualityAvatar

## Test Status

All 5 sketchpad tests pass:

- Home
- Kit
- Type
- Design
- Docs
