# Ticket

## Todos

````markdown
---
date: "2025-11-28T18:49:22.576Z"
slug: PIECE-HOVER-FIX
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix piece hover and selection not working in diagram
model: sonnet-4.5
---

# Previously

Problem: React Flow's `ViewportPortal` component creates a wrapper div with class `react-flow__viewport-portal` that was intercepting all pointer events, preventing hover and click interactions from reaching the piece nodes in the diagram.

# Plan

1. ~~Add CSS policy to make `react-flow__viewport-portal` have `pointer-events: none`~~
2. ~~Create Playwright test to verify piece hover and selection works~~
3. ~~Clean up debug logging~~

# Changes

## 1. Fixed ViewportPortal pointer-events issue

**File:** `js/compose/globals.css`

Added CSS policy to disable pointer events on React Flow's viewport portal container:

```css
.react-flow__viewport-portal {
 pointer-events: none !important;
}
```
````

This allows pointer events to pass through the viewport portal and reach the piece nodes underneath.

## 2. Added Playwright test for piece hover and selection

**File:** `js/compose/sketchpad.test.ts`

Added comprehensive test that:

- Creates a kit, type, and design
- Drags a type to the diagram to create a piece
- Verifies hover adds `ring` class to avatar
- Verifies click adds selection `ring` class
- Verifies selection persists after deselection

## 3. Cleaned up debug logging

Removed temporary `[DEBUG] [PIECE-HOVER-FIX]` console.log statements from:

- `Sketchpad.tsx`: `useIsPieceHovered` hook
- `Design.tsx`: `hoverPiece` command, `handleMouseEnter` callback, `PieceNodeInner` render

```

```

## Changes

## Log

## Summary

# Summary

""
