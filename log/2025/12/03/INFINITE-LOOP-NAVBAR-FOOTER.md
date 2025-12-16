---
slug: INFINITE-LOOP-NAVBAR-FOOTER
summary: Fix infinite loop and missing navbar/footer in Design/Type apps after import
---

# Previously

When opening the Design app or Type app after importing a kit in the sketchpad test, the app would:

1. Hang for a very long time
2. Show "Maximum update depth exceeded" warning in console
3. Not render the navbar and footer (only canvas loaded)

# Plan

1. Add tests to check for navbar/footer visibility and console errors
2. Identify root cause of infinite loop
3. Fix the infinite loop in footer components
4. Verify tests pass

# Changes

## Root Cause

The `TypeAppFooter` and `DesignAppFooter` components had a useEffect dependency issue:

```typescript
useEffect(() => {
  // ... add/remove footer items ...
}, [appType, addFooterItem, removeFooterItem, allModelTagGuids, tagNameMap, selectedModelTags, addModelTag, removeModelTag]); // <-- Problem
```

The `useTypeAppCommands()` and `useDesignAppCommands()` hooks return new function references on every render (not memoized). This caused:

1. Component renders → new function references from commands hook
2. useEffect deps change → effect runs
3. Effect calls `addFooterItem`/`removeFooterItem` → state update
4. State update → re-render → back to step 1 (infinite loop)

## Fix

For both `TypeAppFooter` and `DesignAppFooter`:

1. Added refs to store command functions: `addModelTagRef`, `removeModelTagRef`
2. Update refs in a separate effect when functions change
3. Removed command functions from main useEffect dependency array
4. Use refs in onClick handlers instead of direct function references

```typescript
const addModelTagRef = useRef(addModelTag);
const removeModelTagRef = useRef(removeModelTag);

useEffect(() => {
  addModelTagRef.current = addModelTag;
  // ...
}, [addModelTag, ...]);

useEffect(() => {
  // ... footer setup ...
  onClick: () => addModelTagRef.current(...) // Use ref
}, [appType, addFooterItem, removeFooterItem, allModelTagGuids, tagNameMap]); // No command funcs
```

## Files Modified

- `js/js/sketchpad/Type.tsx` - Fixed `TypeAppFooter`
- `js/js/sketchpad/Design.tsx` - Fixed `DesignAppFooter`
- `js/js/sketchpad.test.ts` - Added navbar/footer/console error checks to Type and Design tests
