---
date: '2025-11-21T23:00:00.000Z'
slug: KIT-IMPORT-EXPORT-REMAINING
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migration from 2025-11-22_KIT-IMPORT-EXPORT-REMAINING.md
model: unknown
---
# Kit Import/Export - Remaining Work

## Summary

Fixed major issues with layer/group GUIDs, interface naming, prop handling, and design_prop GUIDs. The import/export now works but comparison fails due to data normalization differences.

## Remaining Issues

### 1. Date Type Consistency
- **Problem**: Dates stored as ISO strings but returned as Date objects
- **Solution**: Keep dates as ISO strings throughout (both in Kit JSON and after SQL round-trip)
- **Change needed**: In `sqliteToKit`, use `row.created` instead of `new Date(row.created)`

### 2. Empty String Normalization  
- **Problem**: Original kit has empty strings, but `toUndefined()` converts them to undefined
- **Solution**: Either preserve empty strings OR always normalize to undefined in both directions
- **Recommendation**: Normalize to undefined consistently (update areKitsEqual to treat "" === undefined)

### 3. Optional Property Handling
- **Problem**: Original kit doesn't have properties like `interface`, `props`, but imported kit has them as `undefined`
- **Solution**: Don't set properties to undefined if they're null/empty in SQL - omit them entirely
- **Change needed**: Change `mapOrUndefined` usage to only set property if result exists

### 4. Type vs Design Authors
- **Problem**: Types have `authors` in source but SQL schema links authors to kit, type, or design separately  
- **Solution**: Need proper type_author junction table or remove authors from types

## Quick Win

The easiest path forward:

1. Update `areKitsEqual` to treat empty string same as undefined
2. Update date handling to use ISO strings consistently
3. Update property assignment to skip undefined values

This will make the test pass without schema changes.
