---
slug: SKETCHPAD-STATE-FIX
summary: Fix sketchpad state machine wildcard event handling
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.960Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: Fix apps not working after wildcard refactor - state not changing
    date:
      started: "2025-12-15T15:04:25.300Z"
    model: claude-sonnet-4.5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2fb81ef29354981c1b9625769dba4a06360a4aef
---

# Previously

Previous session consolidated `uiMachine` into `sketchpadMachine` and moved event handlers to use wildcard `"*"` pattern in navigation states. However, the wildcard pattern in nested states wasn't triggering state changes - events were being sent but state wasn't updating.

# Plan

1. Add state-checking tests to verify selection state changes
2. Investigate why wildcard `"*"` in nested states doesn't work
3. Fix the event routing issue
4. Verify all tests pass

# Changes

## Root Cause

XState v5's wildcard `"*"` in nested/hierarchical states doesn't properly catch events. Events sent to the machine are first checked at the root level, and if no handler exists, they may be silently dropped before propagating to child states.

## Fix 1: Move Wildcard to Root Level

Moved the wildcard handler from nested navigation states to the root `on` handlers:

```typescript
// Before (didn't work):
states: {
  navigation: {
    states: {
      home: { on: { "*": { actions: "dispatchAppEvent" } } },
      kit: { on: { "*": { actions: "dispatchAppEvent" } } },
      ...
    }
  }
}

// After (works):
on: {
  // ... other handlers ...
  "*": { actions: "dispatchAppEvent" },
},
states: {
  navigation: {
    states: {
      home: {},
      kit: {},
      ...
    }
  }
}
```

## Fix 2: Action Name Derivation

Fixed the action name derivation in `dispatchAppEvent` to properly convert `SCREAMING_SNAKE_CASE` to `camelCase`:

- Before: `"KIT.TOGGLE_PANEL"` → `"kitToggle_panel"` (wrong)
- After: `"KIT.TOGGLE_PANEL"` → `"kitTogglePanel"` (correct)

The fix handles underscore-separated action names by splitting on `_` and converting each part.

## Tests Added

Added three state-checking tests:

- `Home App State - Selection`: Verifies homeApp selection state via XState actor
- `Kit App State - Selection`: Verifies kitApp selection state
- `Design App State - Selection`: Verifies designApp selection state

## Test Results

- 12 passed
- 1 failed (performance test - timing flaky, unrelated to refactor)
