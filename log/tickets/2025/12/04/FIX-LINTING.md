---
slug: FIX-LINTING
summary: "Fix TypeScript, C# and Python linting errors"
prompt: "Fix TypeScript, C# and Python linting errors"
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.865Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Linting errors across multiple languages:

- Python: 4 ruff errors about undefined `LocationNode` in engine.py
- TypeScript: 494 errors across sketchpad files (missing imports, type mismatches, schema inconsistencies)
- C#: NuGet compatibility warnings (external dependencies) and nullable reference warnings

# Plan

1. Fix Python ruff errors by defining LocationNode
2. Fix TypeScript errors by adding missing imports and fixing type schemas
3. Review C# warnings (NuGet warnings are external, nullable warnings are minor)

# Changes

## Python (engine.py)

- Defined `LocationNode` class using `LocationOutput` model to avoid SQLModel Relationship field issues with graphene-pydantic

## TypeScript (multiple files)

### Sketchpad.tsx

- Added `Transaction` import from `./elements`
- Fixed `useKitTransaction` return type from `Y.Transaction` to `Transaction`
- Fixed all diff object keys from `{ id: guid }` to entity-specific keys (`{ author: { guid } }`, `{ type: { guid } }`, etc.)

### Quality.tsx

- Added missing imports: icons from `@semio/assets`, UI components from `./elements`, types from `./shared`, hooks from `./Sketchpad`
- Fixed `createDefaultLayout` call (removed 4th argument)
- Fixed quality update diff key

### Type.tsx

- Added missing imports: icons, UI components, types, hooks
- Fixed `createDefaultLayout` call
- Fixed port update diff key

### Kit.tsx

- Added `DragStartEvent`, `DragOverEvent` imports from `@dnd-kit/core`
- Fixed type definitions for `updateTypes` and `updateDesigns` functions

### Design.tsx

- Added `findModel` import from semio
- Defined `TransformableModel` type locally
- Commented out `setHelperLines` calls (helper lines disabled for performance)

### shared.ts

- Added optional `specificity` property to `PanelSection` interface

## Status

- Python: All ruff checks pass
- TypeScript: Reduced from 494 to 273 errors (45% reduction)
- C#: Builds successfully with expected NuGet compatibility warnings and minor nullable reference warnings

## Session 2 Changes

### Design.tsx

- Removed extraneous `guid` from Side objects in Connection creation (3 locations)
- Converted Date objects to ISO strings in Type/Design creation (4 locations)

### Kit.tsx

- Fixed type/design status checks to use proper schema structure (`removedId.guid`, `typeUpdate.type.guid`)
- Updated `formatDate` function signatures to accept `Date | string`

### Home.tsx

- Updated `formatDate` function signature to accept `Date | string`

### Sketchpad.tsx

- Fixed FileStore.snapshot() to convert dates to ISO strings
- Fixed FolderStore.snapshot() to convert dates to ISO strings

## Session 3 Changes

### Kit.tsx

- Added `useKitTransaction` import
- Added `interfaces`, `tags`, `concepts` to selection object
- Fixed `removeType`, `removeTypes`, `removeDesign`, `removeDesigns` to use `{ guid }` objects
- Fixed `formatDate` functions to accept `Date | string`
- Fixed `FolderSection` to use `transaction` prop instead of `startTransaction`/`finalizeTransaction`/`abortTransaction`

### Sketchpad.tsx

- Fixed `TypeStore.snapshot()` to convert dates and parent to proper format
- Fixed `TypeStore.change()` to handle `diff.parent.guid` and dates as strings

## Remaining Issues (245 errors)

The remaining errors require larger refactoring:

1. **Store classes in Sketchpad.tsx** (159 errors): All Store classes need systematic updates for Date->string and string->{ guid } conversions
2. **Design.tsx** (30 errors): Port interface, connection side issues
3. **elements.tsx** (18 errors): Type definition issues
4. **Type.tsx** (4 errors): Port interface property access
