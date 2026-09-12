# Norm Empty Presence and Inspection Ownership

## Current result

The Norm schema descriptor now publishes an empty presence facet. The five local presence schema leaves were removed, so the shared schema catalog no longer advertises a second empty `s.norm.norm.presence` type.

All fifteen Norm variants already bind both editor and viewer apps directly to framework `NoPresence` and `NoPresenceMutation`. The focused source oracle now checks all thirty bindings, the empty descriptor facet, and the absence of local Rust, TypeScript, JSON Schema, GraphQL, and Proto presence leaves.

`NormConfig.selected_check_index` remains unchanged. It describes inspection selection and belongs to the later exact inspection-window ownership migration; this bounded cleanup does not move or reinterpret that state.

## Validation boundary

- **Focused source verified:** `bun nx run @semio-tech/norm-plugin:surface-render-source` passed with 15 variants, 30 apps, 120 render bodies, Ajv, and five hostile vectors.
- **Config contract verified:** `bun nx run @semio-tech/norm-plugin:config-mutation-source` passed with five neutral fixture cases, five hostile payloads, four undeclared wire forms, 13 text assertions, and 25 binary assertions. This confirms the cleanup left `NormConfig` intact.
- **Catalog generation verified:** `bun nx run workspace:schema-generate` and `bun nx run workspace:schema-docs` passed. The generated JSON and Markdown catalogs contain no `app.norm.norm.presence` entry or retired Norm presence-schema path.
- **Global check observed, not green:** `bun nx run workspace:schema-check` reported 6,233 workspace findings and `schema-catalog-stale=1` while concurrent schema edits were still landing. The exact Norm presence identifiers were absent after generation; the global result does not establish a clean repository-wide schema baseline.

## Exact file ledger

- `✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🦀️.rs`
- Deleted `✏️s/🔌️plugins/📕️norm/👥️presence/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md`

The two generated catalog files also contain concurrent agents' schema changes. This slice's semantic catalog delta is only removal of the Norm app-presence scope.
