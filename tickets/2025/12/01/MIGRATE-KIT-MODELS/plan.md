# Previously

The kit_metabolism.json.old file contained the old schema with `representations` on types. The new kit_metabolism.json had the new schema with `models` but was missing the actual model data.

# Plan

1. Analyze schema differences between old and new kit format
2. Create migration script to extract models (formerly representations) from old kit
3. Execute migration and verify results

# Changes

Created `scripts/migrate-kit-models.ts` to migrate:

**Schema Changes Identified:**

- OLD: `representations` array with `url`, `description`, `tags`, `attributes`
- NEW: `models` array with `guid`, `url`, `description`, `tags`, `attributes`
- OLD: Type variants via `name` + `variant` (e.g., name="Base", variant="Blob")
- NEW: Type hierarchy via `name` + `parent.guid` (e.g., name="Blob", parent points to "Base")

**Parent Name Mapping:**
Some type names were simplified in the new schema:

- "Capsule with Balcony" -> "Balcony"
- "Ellipsoid Capsule" -> "Ellipsoid"
- "Trapezoid Capsule" -> "Trapezoid"

**Migration Results:**

- 45 types migrated with 6 models each (270 models total)
- Kit metadata: only `preview` field was missing and migrated
- 4 new abstract parent types (Capsule, Ellipsoid, Trapezoid, Balcony) have no models by design
