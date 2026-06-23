---
goal: clean-code/sketchpad
---

# Ticket

## Summary

Fixed Kit designs toggle deselection by adding unstable_useTransitions={false} to BrowserRouter and MemoryRouter in Sketchpad.tsx. Root cause: React Router 7 wraps history listener state updates in React.startTransition by default, causing useSearchParams to return stale values when a second click fires before the transition commits. Setting unstable_useTransitions={false} makes navigation state updates synchronous, ensuring the component re-renders with fresh state between clicks.
## Root Cause

In React Router 7 (`BrowserRouter` / `MemoryRouter`):
```js
let setState = React.useCallback((newState) => {
  if (unstable_useTransitions === false) {
    setStateImpl(newState);
  } else {
    React.startTransition(() => setStateImpl(newState)); // DEFAULT: deferred!
  }
}, [unstable_useTransitions]);
React.useLayoutEffect(() => history.listen(setState), [history, setState]);
```

Chain of events:
1. Click 1 → `toggleKind("designs")` → `searchParams` has no `kind` → appends `kind=designs` → `setSearchParams(newParams)` → `navigate("?kind=designs")` → `history.push` → URL changes immediately
2. History listener fires → `startTransition(() => setStateImpl(newLocation))` → React **schedules** state update as low-priority transition
3. Playwright's `waitForURL(/kind=designs/)` resolves (URL already changed)
4. Click 2 fires BEFORE the transition commits → component still has old state → `searchParams` has NO `kind=designs` → `toggleKind("designs")` **adds** `kind=designs` again instead of removing it
5. URL stays at `?kind=designs`

## Fix

Pass `unstable_useTransitions={false}` to both `BrowserRouter` and `MemoryRouter` in Sketchpad.tsx. This makes navigation state updates synchronous, ensuring `useSearchParams` is up-to-date before the next interaction.

## Changes

- `compose/js/sketchpad/Sketchpad.tsx`: Add `unstable_useTransitions={false}` to BrowserRouter and MemoryRouter

## Log

- Traced through Radix `@radix-ui/react-toggle-group` (ToggleGroupImplSingle), `@radix-ui/react-toggle`, `@radix-ui/react-use-controllable-state`, `@radix-ui/react-roving-focus`, `@radix-ui/react-slot`
- Confirmed the Radix chain correctly handles deselection for `type="single"` ToggleGroup
- Found the root cause in React Router 7's `BrowserRouter` using `startTransition` by default
- Both `BrowserRouter` and `MemoryRouter` have the `unstable_useTransitions` prop to control this behavior

## Todos

- [x] Investigate root cause
- [x] Add `unstable_useTransitions={false}` to BrowserRouter and MemoryRouter
- [x] Run the failing test to verify fix
- [x] Run full test suite to check for regressions (5/6 pass; 1 pre-existing Design test failure unrelated)
- [x] Remove debug logging
- [x] Close ticket

## Plan

1. Edit Sketchpad.tsx to pass `unstable_useTransitions={false}` to both router components
2. Run the Kit toggle test to verify the fix
3. Run the full test suite to ensure no regressions
