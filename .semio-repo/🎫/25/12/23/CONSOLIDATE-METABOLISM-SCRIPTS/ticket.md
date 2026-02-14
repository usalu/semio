# Ticket

## Todos

# Previously

Three separate scripts handled Metabolism asset generation:

- `scripts/generate-metabolism-diff.tsx` - Generated diff files for testing (`diff_kit_metabolism.json`, `diff_kit_metabolism_inverted.json`, `kit_metabolism_diffed.json`)
- `scripts/generate-validation.tsx` - Generated `validation.json` from `kit_invalid.json`
- `scripts/regenerate-metabolism.tsx` - Regenerated `metabolism.zip` and copied to public folders

This separation required running multiple scripts to update all Metabolism assets, and the scripts had overlapping functionality (all loading the kit JSON).

# Plan

1. Create consolidated `scripts/update-metabolism.tsx` that combines all three operations
2. Organize code into regions: File Collection, Regenerate Zip, Generate Diff, Generate Validation
3. Update AGENTS.md documentation to reference the new script
4. Delete the three old scripts

# Changes

## Created Consolidated Script

Created `scripts/update-metabolism.tsx` that performs all three operations in sequence:

1. **Regenerate Zip**: Exports `metabolism.zip` from `kit_metabolism.json` and example files, copies to all public folders
2. **Generate Diff**: Creates comprehensive diff files for testing
3. **Generate Validation**: Generates `validation.json` from `kit_invalid.json`

The script uses regions to organize code:

- `#region 🔖File Collection` - Shared file collection logic
- `#region 🔖Regenerate Zip` - Zip generation and copying
- `#region 🔖Generate Diff` - Diff file generation
- `#region 🔖Generate Validation` - Validation JSON generation
- `#region 🔖App` - React UI component

## Updated Connector Naming

Fixed connector naming in diff generation to use "connector" instead of "port" (`new-connector`, `test-connector` instead of `new-port`, `test-port`).

## Updated Documentation

Updated `AGENTS.md` to reference the new consolidated script instead of the old `generate-validation.ts` script.

## Removed Old Scripts

Deleted the three separate scripts:

- `scripts/generate-metabolism-diff.tsx`
- `scripts/generate-validation.tsx`
- `scripts/regenerate-metabolism.tsx`

## Changes

## Log

## Summary

# Summary

Consolidate three Metabolism scripts into single update-metabolism.tsx
