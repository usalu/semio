---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Optimized Ring drag performance by introducing local drag state, rAF-throttled parent updates, and suppressed CSS transitions during drag.

## Changes

- `elements.tsx` OrbProps: Added `dragging?: boolean` prop.
- `elements.tsx` Orb: Suppress `transition-all duration-150` when `dragging=true` for instant visual updates.
- `elements.tsx` Ring: Added `localT` state for immediate orb repositioning during drag without waiting for parent re-render. Added `pendingT` ref + `requestAnimationFrame` throttle so `onOrbChange` fires at display refresh rate instead of every pointer event. Cancel rAF on unmount and pointer cancel. Pass `dragging` flag to Orb.

## Log

- Identified 3 bottlenecks: (1) every pointer move triggers full kit diff round-trip, (2) CSS transitions add 150ms lag, (3) no local state so orb waits for parent re-render.
- Added local `t` override during drag so orb moves instantly.
- Throttled `onOrbChange` via `requestAnimationFrame` to batch parent updates at ~60fps.
- Suppressed CSS transitions on dragged orb via new `dragging` prop.
- Build passes. All 14 tests green.

## Todos

- [x] Analyze Ring drag performance bottleneck
- [x] Optimize Ring with local drag state + rAF throttle
- [x] Add dragging prop to Orb to suppress CSS transitions
- [x] Build and verify
- [x] Run tests

## Plan

1. Identify bottlenecks in Ring drag pipeline.
2. Add local `t` state during drag for instant visual feedback.
3. Throttle `onOrbChange` via requestAnimationFrame.
4. Suppress CSS transitions on dragged orb.
5. Build and test.
