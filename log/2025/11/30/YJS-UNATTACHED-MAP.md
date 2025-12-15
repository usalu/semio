---
slug: YJS-UNATTACHED-MAP
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix Y.js Invalid access warnings when importing kits
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

When importing a zipped kit in the home app, thousands of console warnings appeared:

```
Sketchpad.tsx:1148 Invalid access: Add Yjs type to a document before reading data.
```

This was caused by Y.js maps being accessed (read/write) before they were attached to a Y.Doc.

# Plan

1. Identify all places where Y.Map/Y.Array operations occur before the type is attached to a Y.Doc
2. Fix by ensuring push to parent Y.Array happens BEFORE Store constructor calls
3. Verify fix with E2E test

# Changes

## Root Cause

In `createType()` in `KitStore`, the order of operations was wrong:

1. Create `Y.Map` (detached)
2. Create `TypeStore` (calls `.set()` on the detached map) ← causes warnings!
3. Push to `yTypes` array (attaches to doc)

The Store constructors set values on the Y.Map using setters like `this.guid = type.guid;` which call `yType.set("guid", guid)`. When the map isn't attached to a Y.Doc yet, Y.js logs a warning.

## Fix

Changed the order in `createType()` to push the Y.Map to the doc BEFORE creating the TypeStore:

```typescript
// Before (wrong order):
const yType = new Y.Map<YTypeVal>();
const yTypeStore = new TypeStore(this, yType, type); // Sets values on detached map
this.yTypes.push([yType]); // Attaches after - too late!

// After (correct order):
const yType = new Y.Map<YTypeVal>();
this.yTypes.push([yType]); // Attach to doc FIRST
const yTypeStore = new TypeStore(this, yType, type); // Now safe to set values
```

This pattern was already correct in other creation functions like `createDesign`, `createQuality`, `createAuthor`, etc.

## Verification

Added Y.js warning detection to the E2E test in `sketchpad.test.ts`:

```typescript
const yjsWarnings: string[] = [];
page.on("console", (msg) => {
  if (msg.type() === "warning" && msg.text().includes("Invalid access")) yjsWarnings.push(msg.text());
});
// ... test imports metabolism.zip ...
expect(yjsWarnings).toHaveLength(0);
```

Test output confirms the fix:

```
Y.js warnings count: 0
✓ [chromium] › sketchpad.test.ts:205:5 › Drop metabolism.zip creates temporary kit with files (39.4s)
1 passed (40.7s)
```
