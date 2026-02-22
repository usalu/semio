# Implement Drag Feature in Design App

## Goal
SKETCHPAD-IMPROVEMENTS

## Status
closed

## Prompt
Fix drag feature in Design App sketchpad: resolve Map JSON.stringify bug in DerivedNode and useDerived, fix createPathObserver stale reference comparison.

## Plan
1. ✅ Trace reactive chain from drag handler through store updates to React re-rendering  
2. ✅ Fix `createPathObserver` in shared.ts — was comparing stale Y.js references instead of serialized JSON strings
3. ✅ Fix `DerivedNode.recompute()` — `JSON.stringify(Map)` returns `"{}"`, added `jsonReplacer` for Map/Set
4. ✅ Fix `useDerived` getSnapshot — same `JSON.stringify(Map)` issue
5. ✅ Clean up all [DEBUG] console.warn logs
6. ✅ Refactor test to not depend on console warning messages
7. ✅ Fix Kit test — intersect/lasso toggles are now intentionally visible
8. ✅ Run full test suite — all 7 tests pass

## Root Causes

### Bug 1: `createPathObserver` stale reference comparison (shared.ts)
The observer stored a Y.js value reference (`let lastValue = getValueAtPath(root, path)`) and compared it via `.toJSON()`. After Y.js mutation, both `lastValue.toJSON()` and `newValue.toJSON()` read from the SAME mutable reference, producing identical results. Fix: store serialized JSON string instead (via `serializeValue()`).

### Bug 2: `DerivedNode.recompute()` Map serialization (shared.ts)
`JSON.stringify(Map)` returns `"{}"` for all Maps. `DerivedNode.recompute()` used `JSON.stringify(next)` to detect changes, so Map-valued derived nodes NEVER detected changes. Fix: added `jsonReplacer` static method that converts Map→Object and Set→Array for serialization.

### Bug 3: `useDerived` getSnapshot Map serialization (Sketchpad.tsx)
Same issue — `useSyncExternalStore`'s `getSnapshot` used `JSON.stringify(newValue)` to detect changes, which always returned `"{}"` for Maps. Fix: added `jsonReplacerMapSet` helper function.

## Changes
- `semio/js/sketchpad/shared.ts`: Fixed `createPathObserver` to use serialized JSON comparison; added `DerivedNode.jsonReplacer` for Map/Set-aware serialization
- `semio/js/sketchpad/Sketchpad.tsx`: Added `jsonReplacerMapSet` helper; fixed `useDerived` getSnapshot; removed debug logs
- `semio/js/sketchpad/Design.tsx`: Removed all [DEBUG] console.warn logs from drag handlers, command execution, and node computation
- `semio/js/sketchpad.test.ts`: Refactored drag test to use direct store assertions instead of console warning parsing; updated Kit test to expect intersect/lasso toggles (intentionally added by prior ticket)

## Summary
Fixed three bugs preventing the Design App drag feature from working: (1) `createPathObserver` compared mutable Y.js references instead of serialized values, (2) `DerivedNode.recompute()` couldn't detect Map changes due to `JSON.stringify(Map)` returning `"{}"`, (3) `useDerived` getSnapshot had the same Map serialization issue. All 7 sketchpad tests pass.
