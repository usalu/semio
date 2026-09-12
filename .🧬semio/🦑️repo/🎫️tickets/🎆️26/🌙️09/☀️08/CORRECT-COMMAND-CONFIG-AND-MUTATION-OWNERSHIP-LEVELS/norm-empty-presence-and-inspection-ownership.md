# Norm Empty Presence and Inspection Ownership

## Current result

The Norm schema descriptor now publishes an empty presence facet. The five local presence schema leaves were removed, so the shared schema catalog no longer advertises a second empty `s.norm.norm.presence` type.

All fifteen Norm variants already bind both editor and viewer apps directly to framework `NoPresence` and `NoPresenceMutation`. The focused source oracle now checks all thirty bindings, the empty descriptor facet, and the absence of local Rust, TypeScript, JSON Schema, GraphQL, and Proto presence leaves.

Inspection selection now belongs to `NormResultsWindowConfig` under each concrete Results window. All fifteen editors register their Results owner, route `SetSelectedCheckIndex` to one exact `WindowConfig` lane, and render Inspection from the focused Results instance. App config remains empty.

## Validation boundary

- **Focused source verified:** `bun nx run @semio-tech/norm-plugin:surface-render-source` passed with 15 variants, 30 apps, 120 render bodies, Ajv, and five hostile vectors.
- **Config contract verified:** `bun nx run @semio-tech/norm-plugin:config-mutation-source` passed with five neutral fixture cases, five hostile payloads, four undeclared wire forms, 13 text assertions, and 25 binary assertions. This confirms the cleanup left `NormConfig` intact.
- **Catalog generation verified:** `bun nx run workspace:schema-generate` and `bun nx run workspace:schema-docs` passed. The generated JSON and Markdown catalogs contain no `app.norm.norm.presence` entry or retired Norm presence-schema path.
- **Global check observed, not green:** `bun nx run workspace:schema-check` reported 6,233 workspace findings and `schema-catalog-stale=1` while concurrent schema edits were still landing. The exact Norm presence identifiers were absent after generation; the global result does not establish a clean repository-wide schema baseline.
- **Results-window oracle verified:** the registered ticket facade passed the independent Ajv/JSON Patch and strict-TypeScript oracle twice for all 15 families, two same-kind Results windows, focused Inspection, and stable document bytes in `🗑️generated/norm-results-window-ownership-native-9.log`.
- **Results-window native verified:** Norm r9 passed both selected laws with 224 filtered in 0.30s. The codec law covers fixture projection, inverse, text, Pack, and two-instance undo/redo. The runtime law isolates selections 1 and 2, lazily materializes the untouched default window, renders focused Inspection from each exact config, admits one `WindowConfig` lane while rejecting every forbidden owner lane, preserves document/app-config Pack and SPR bytes, persists and reopens exactly two packs, edits the reopened left instance without changing the right, rejects stale/wrong-kind/absent identities, and closes both registered apps on an 8 MiB stack.
- **Diagnostic provenance:** Norm r7's bounded cleanup guard preserved the original `interactive-job.live-instance` fault instead of allowing an unclosed Results store to mask it during destruction. The reopened fixture was bound as instance 205 while its dispatch metadata still named 204. The helper now receives the bound instance explicitly; the guard remains unconditional for normal returns, early errors, and panics.

## Exact file ledger

- `✏️s/🔌️plugins/📕️norm/🎚️config/🧬️schema/🦀️.rs`
- Deleted `✏️s/🔌️plugins/📕️norm/👥️presence/🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs,🔗️.graphql,🛰️.proto}`.
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md`
- `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config`
- All fifteen concrete editor registrations and focused Inspection render consumers.
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🧪️tests/🔬️window-ownership/🦀️.rs`

The two generated catalog files also contain concurrent agents' schema changes. This slice's semantic catalog delta is only removal of the Norm app-presence scope.
