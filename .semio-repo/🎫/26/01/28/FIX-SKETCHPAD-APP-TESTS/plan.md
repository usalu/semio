# Plan: Fix Sketchpad App Tests

## Problem Analysis

### Issue 1: Parallel Test Execution Race Conditions
The Kit, Type, and Design tests were failing because they wait for "Metabolism" text to appear after uploading a kit, but it timed out after 60 seconds. When running 9 parallel workers, tests would share browser state and interfere with each other.

### Issue 2: D3 Force Simulation DOM Detachment
After fixing the parallel issue, the Kit test was failing with "element was detached from the DOM" when clicking diagram nodes. This was caused by the D3 force simulation continuously re-rendering nodes, detaching them from the DOM mid-click.

## Root Causes

1. **Parallel execution**: Multiple tests running simultaneously caused race conditions where tests would interfere with each other's state
2. **Force simulation instability**: D3 force simulation continuously updates node positions, causing React Flow to re-render and detach DOM elements

## Solution

### Fix 1: Disable Parallel Test Execution
Modified `playwright.config.ts`:
- Changed `fullyParallel: true` to `fullyParallel: false`
- Changed `workers: process.env.CI ? 1 : undefined` to `workers: 1`

### Fix 2: Wait for Diagram Stabilization
Added `waitForDiagramStabilization()` helper function in `sketchpad.test.ts` that:
- Tracks node positions over 500ms intervals
- Returns when positions stabilize (change < 1px)
- Called before diagram node interactions

## TODOs

- [x] Analyze test failures in detail
- [x] Fix parallel test execution by forcing sequential workers
- [x] Add `waitForDiagramStabilization` helper function
- [x] Call stabilization helper before diagram node clicks
- [x] Run tests and verify all pass (6/6 passed in 7.6m)

