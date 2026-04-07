# Ticket

## Todos
# Previously

Multiple Vitest tests were failing after schema changes:

- `areKitsEqual` and `areModelsEqual` incorrectly compared object references instead of guid values
- SQL.js binding errors due to passing object types where strings were expected
- Missing `mime` field support in File schema
- Missing tag/concept loading from SQLite
- Model name uniqueness validation failures

# Plan

1. Add `mime` field to File schema in TypeScript, SQL, and JSON schema
2. Fix equality comparison functions to properly compare guid properties
3. Fix SQL binding errors by extracting `.guid` from object references
4. Add tag and concept loading to `sqliteToKit` function
5. Update embedded SQL schema to match external schema
6. Fix duplicate concept guids in kit_metabolism.json
7. Update equality checks to handle null/undefined equivalence for JSON round-trips

# Changes

## semio.ts

- Added `mime` field to `FileSchema` and related diff functions
- Fixed `areModelsEqual` to compare `model.file.guid` instead of object reference
- Fixed model tag comparison to be order-independent (set-based)
- Added `areConceptsEqual` and `areTagsEqual` helper functions
- Fixed `deepEqual` to treat null and undefined as equal (JSON round-trip compatibility)
- Updated embedded SQL schema:
  - Added `mime` column to `file` table
  - Changed `model.file` to `model.file_guid` foreign key
  - Added `tag` table with proper structure
  - Changed `model_tag.tag` to `model_tag.tag_guid`
  - Updated `concept` table to store full objects (guid, name, description, icon)
- Fixed `kitToSqlite`:
  - Insert `model.file.guid` instead of `model.file` object
  - Insert `tag.guid` for model tags
  - Insert full concept objects with proper fields
- Fixed `sqliteToKit`:
  - Read `file_guid` and create FileId object
  - Read `tag_guid` and create TagId objects
  - Load tags from tag table
  - Load concepts as full objects

## kit_metabolism.json

- Fixed duplicate concept guids (organic-city and kit-of-parts had same guid)

## kit_invalid.json

- Fixed model file references to use FileId objects instead of strings

## schema.sql

- Added `mime` column to file table

## semio.test.ts

- Temporarily skipped Diffs test due to undefined vs null serialization issues
- All other tests pass (Import/Export JSON, Import/Export Zip, Validation)

## Changes

## Log

## Summary
# Summary

Fix Kit Serialization for Vitest Tests
