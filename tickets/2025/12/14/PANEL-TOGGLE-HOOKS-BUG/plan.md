# Previously

User reported React hooks order error when switching from details to settings panel in Kit app:

```
Uncaught Error: Rendered more hooks than during the previous render.
```

# Plan

1. Investigate Panel component in elements.tsx to understand section rendering
2. Trace the hooks error to identify which component violates Policies of Hooks
3. Fix the pattern in all affected apps (Kit.tsx, Type.tsx, Design.tsx)
4. Add expanded panel test to verify fixes

# Changes

## Root Cause

Found that `SettingsContent` components were defined **inside** `useEffect` hooks in Kit.tsx, Type.tsx, and Design.tsx. This violates React's Policies of Hooks because:

1. When panel switches from "details" to "settings", the `content()` function renders a different component
2. The `SettingsContent` function defined inline uses hooks (`useTheme`, `useLanguage`, etc.)
3. React sees different hook counts between renders, causing the error

Example of bad pattern (in Kit.tsx useEffect):

```tsx
const SettingsContent: FC = () => {
  const [theme, setTheme] = useTheme(); // Hook inside inline component
  // ...
};
addSection("settings", { content: SettingsContent });
```

## Fix

Moved settings content components to **module level** (outside any hooks) and wrapped with JSX function:

1. **Kit.tsx**: Added `KitSettingsContent: FC` in Settings region (~line 5542)
2. **Type.tsx**: Added `TypeSettingsContent: FC` in Settings region (~line 2693)
3. **Design.tsx**: Added new Settings region with `DesignSettingsContent: FC` before DesignApp

Updated useEffect registrations to use wrapper pattern:

```tsx
addSection("settings", {
  content: () => <KitSettingsContent />, // Wrapper function
  // ...
});
```

This pattern matches the existing correct implementation in Home.tsx (`HomeSettingsContent`).

## Test Added

Added "Panel Section Switching" test to sketchpad.test.ts that:

- Tests each panel kind individually (details, settings, chat, workbench, toolbar)
- Tests rapid panel switching to catch hooks errors
- Checks console for React hooks errors
- Covers Home, Kit, Type, and Design apps

Test passed in 2.0 minutes with no hooks errors detected.
