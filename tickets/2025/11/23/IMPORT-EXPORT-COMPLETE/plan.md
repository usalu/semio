# Import/Export Complete Implementation

## Analysis

### Current State

- Metabolism.json is the correct source of truth
- Three placeholder assets exist: MetabolismKitDiff, MetabolismKitDiffInverted, MetabolismKitDiffed
- Test verifies diff operations work correctly (compute diff, inverse diff, apply diff)
- Import/export may lose information or violate schema

### Goals

1. Generate MetabolismKitDiff with comprehensive changes using all kit diff features
2. Generate MetabolismKitDiffInverted (inverse of the diff)
3. Generate MetabolismKitDiffed (result of applying diff to original)
4. Fix any schema compliance or data loss issues in import/export
5. Ensure all tests pass

## Implementation Plan

### 1. Generate Metabolism Diff Assets

Create `scripts/generate-metabolism-diff.ts` to:

1. Load original MetabolismKit
2. Create a comprehensive diff that exercises all features:
   - Add/remove/update types
   - Add/remove/update designs
   - Add/remove/update qualities
   - Add/remove/update ports
   - Add/remove/update files
   - Add/remove/update authors
   - Modify nested properties (connectors, pieces, connections, etc.)
3. Compute inverse diff
4. Apply forward diff to get diffed kit
5. Write all three assets to `assets/semio/`

### 2. Fix Schema/Import/Export Problems

Based on test failures, identify and fix:

- Missing fields in JSON schema
- Data loss during export/import
- Type mismatches
- Optional vs required field handling

### 3. Validate

Run tests until all assertions pass:

- Computed diff matches generated diff
- Computed inverse diff matches generated inverse diff
- Applying forward diff produces expected result
- Applying inverse diff returns to original
- Import/export roundtrip preserves all data

## Implementation Steps

1. Implement generation script with comprehensive diff
2. Run tests and identify failures
3. Fix schema/serialization issues
4. Iterate until all tests pass
