# W1 Taxonomy

## Changes
- `newArtifactChildDirs` → `["🏅️standards"]` (examples moved to subsets)
- `standardComponentDirs` / `standardChildDirs` → `["🪆️subsets"]` (engines moved to subsets)
- `subsetComponentDirs` → schema + engine + io
- `subsetChildDirs` → schema + engine + io + examples
- Added `subsetArchetypes`, `ioFidelityClasses`
- Did **not** flip `schemaChildDirs` for inferences (owned by IIF)

## Files
- `🔣️taxonomy.json`
- `📦️packages/�湃typescript/🧪️index.test.ts` expectations
- `🔍️discovery/🟦️component.ts` optional type fields

## Note
Taxonomy now requires subset engines/examples structurally. Policies for presence remain medium until migration completes, otherwise verify gate would red-lock concurrent tickets.
