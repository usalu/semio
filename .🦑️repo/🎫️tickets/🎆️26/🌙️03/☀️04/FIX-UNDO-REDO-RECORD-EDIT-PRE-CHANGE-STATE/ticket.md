---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fix root cause bug where `recordEdit` computes `inverseKitDiff` using post-change kit state instead of pre-change state. This causes undo to be a no-op (sets values to what they already are). Fix all `executeCommand` callers across Design, Kit, Quality apps. Add missing undo/redo keyboard shortcuts for Kit and Type apps. Fix and pass Design Undo Redo Playwright test.

## Changes

- [x] Design.tsx: Move `recordEdit(result)` before `kitStore.change(result.kitDiff)`, remove duplicate recordEdit call
- [x] Kit.tsx: Move `recordEdit(result)` before `kitData.change(result.kitDiff)` + add undo/redo/delete hotkeys via useHotkeys
- [x] Quality.tsx: Move `recordEdit(result)` before `kitStore.change(...)` + wrap qualityDiff as kitDiff before recording
- [x] Type.tsx: Add undo/redo hotkeys via useHotkeys (ctrl+z, ctrl+y, ctrl+shift+z)
- [x] sketchpad.test.ts: Fix store access pattern (nested Map), clean up all debug artifacts

## Log

- Root cause: `recordEdit()` calls `kitStore.snapshot()` AFTER `kitStore.change(kitDiff)` applied the change. `inverseKitDiff(newState, diff)` then computes inverse with the new state as "original", producing an inverse diff that sets values to what they already are (no-op).
- Fix: Call `recordEdit(result)` BEFORE `kitStore.change(result.kitDiff)` so the snapshot captures pre-change state.
- Vite cache issue: Old Vite processes served stale cached code even after source changes on disk. Fixed by killing all stale Vite processes and starting fresh.
- Test results: Drag 72px, undo reverts to 0px, redo restores to 72px, Ctrl+Shift+Z works, Escape no crash.

## Todos

- [x] Open ticket and gather context
- [x] Fix recordEdit pre-change state bug in Design, Kit, Quality
- [x] Fix Quality recordEdit qualityDiff gap
- [x] Add undo/redo hotkeys to Kit.tsx and Type.tsx
- [x] Fix test store access pattern
- [x] Clean up debug artifacts from test file
- [x] Run unit tests (14/14 pass)
- [x] Run Playwright undo/redo test (passes)
- [x] Close ticket

## Plan

1. In each `executeCommand`, move `this.recordEdit(result)` to BEFORE `kitStore.change(result.kitDiff)` (but AFTER `this.change(result.diff)` for app state)
2. Fix Quality's `qualityDiff` → wrap as `kitDiff` before recording
3. Fix Playwright test store access (Map<string, Map<string, DesignAppStoreInstance>>)
4. Run tests to verify
5. Clean up all debug artifacts from test file
