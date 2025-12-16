---
slug: DESIGN-WINDOWS-LAYOUT
summary: Fix design app window layout to be 50% diagram | 50% scene
---

## Problem

The design app Windows test was failing because the scene window was positioned at x=19 instead of between x=400-800 (expected for a 50/50 split).

## Root Cause

The GoldenLayout v2 configuration was using the deprecated `width` property (numeric) instead of the proper `size` property (string with percentage).

- Old: `width: 50` → would be normalized to `"50%"` but wasn't being applied correctly by GoldenLayout
- New: `size: "50%"` → the proper GoldenLayout v2 syntax

## Fix

Changed the defaultLayout configuration in `js/js/sketchpad/apps/design/App.tsx`:

```typescript
// Before
{
  type: "stack",
  width: 50,  // deprecated numeric width
  content: [...]
}

// After
{
  type: "stack",
  size: "50%",  // proper GoldenLayout v2 size string
  content: [...]
}
```

Also changed window titles from "Diagram"/"Scene" to "diagram"/"scene" to match lowercase text selectors in the test.
