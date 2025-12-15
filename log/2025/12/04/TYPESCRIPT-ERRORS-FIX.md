---
date:
  created: '2025-12-04T19:19:30.464Z'
  updated: '2025-12-04T19:19:30.464Z'
slug: TYPESCRIPT-ERRORS-FIX
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix TypeScript errors in sketchpad components
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

TypeScript compilation had ~70 errors across multiple sketchpad component files including Design.tsx, Kit.tsx, Type.tsx, Home.tsx, Tutorials.tsx, Docs.tsx, elements.tsx, and Sketchpad.tsx. Many errors related to type mismatches between schema definitions and code usage, particularly around guid handling (string vs { guid: string }), date serialization, DiffStatus enum narrowing, and property access on potentially null values.

# Plan

1. [x] Fix Design.tsx errors (location, dates, ports, DiffStatus, reactFlowInstanceRef, connection coordinates)
2. [x] Fix Kit.tsx errors (concept name lookups, useKitTransaction naming)
3. [x] Fix elements.tsx errors (translation keys, ActionGroupProps, BreadcrumbItemProps, variant support)
4. [x] Fix Home.tsx errors (concept handling in KitShallow)
5. [x] Fix Docs.tsx errors (useEffect cleanup, command registration)
6. [x] Fix Type.tsx errors (parent guid, interface guid, compatibleInterfaces)
7. [x] Fix Tutorials.tsx errors (Button variant prop support)

# Changes

## Design.tsx

- Added explicit type annotation for `diffStatus: DiffStatus` to prevent narrowing issues
- Used string comparison (`status as string`) for DiffStatus in isSelected block
- Added null checks and non-null assertions for `lastPostition` and `reactFlowInstanceRef.current`
- Changed `x`, `y` to `u`, `v` in connection objects to match ConnectionSchema
- Added optional chaining for `connection.port?.guid`
- Fixed `port.interface` to use `.guid` property
- Cast deprecated `compatibleInterfaces` access to `any`
- Fixed parent comparison to use `.guid` property

## Kit.tsx

- Fixed concept lookup to use `kitConcepts` for name resolution
- Changed `selectedConcepts.includes(c)` to `selectedConcepts.includes(c.guid)`
- Renamed `useKitTransaction` to `useKitAppTransaction`

## elements.tsx

- Cast dynamic translation keys to `any` in DescriptionTooltipContent
- Removed `id` from `Omit` in ActionGroupProps to allow id prop
- Cast `element.props` to `object` for spread operator
- Added `level` prop to BreadcrumbItemProps
- Used `!!` to convert hasOptions to boolean
- Cast itemContent to `React.ReactElement<any>` for cloneElement
- Removed duplicate export of BreadcrumbItemData
- Added `variant` option to buttonGroupItemVariants (default, ghost, outline)

## Home.tsx

- Fixed concept handling for KitShallow where concepts is `string[]` not `Concept[]`

## Docs.tsx

- Wrapped useEffect cleanup to return void instead of boolean
- Cast command to `any` in registerCommand

## Type.tsx

- Fixed `type.parent` to use `.guid` property
- Fixed port update structure to use `{ port: { guid: id }, diff }`
- Fixed `port.interface` to use `.guid` property
- Cast deprecated `compatibleInterfaces` to `any`

## Remaining Issues

13 TypeScript errors remain, all in node_modules:

- postcss, vite, @dnd-kit/core, @dnd-kit/sortable, @types/mdx
- These are third-party type definition issues requiring package updates or `skipLibCheck: true`
