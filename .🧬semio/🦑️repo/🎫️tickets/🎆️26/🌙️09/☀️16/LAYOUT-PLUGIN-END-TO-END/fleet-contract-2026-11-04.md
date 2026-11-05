# Layout Catalogue Renderer Transport Contract — 2026-11-04

## Scope and Findings

The prior test named an independent JSON Schema oracle but only checked unique fixture IDs and the presence of drag-over/drop actions. Invalid types, missing fields, fabricated kind fields, and unexpected transport properties were unobserved.

The existing relative imports all resolve successfully. The existing composition script runs the renderer configuration, whose explicit include list already contains the Layout catalogue test. No import-depth or registration repair is necessary.

No canonical JSON Schema for the camelCase renderer catalogue action envelope was found in the framework or Layout schema documents. The Layout args bridge consumes the renderer envelope separately from its snake_case native commands. The schema introduced here owns that transport contract; it does not claim that arbitrary dragData JSON is a valid Layout catalogue item.

Checked the actual renderer publisher at Canvas2dHost/🔸️publish-catalogue-drag/🟦️.tsx: drag-over emits x/y/width/height/types; drop emits x/y/width/height/dragData. Canvas2dHost/🔶️scene/🟦️.tsx injects surfaceId before publishing every action. The schema requires exactly these action-specific fields and rejects unknown envelope/argument fields, so transport key drift cannot silently pass.

## Test-First Procedure

Added language-neutral transport.json vectors and drove them through the previous shallow acceptance checks. Expected rejections cover string coordinates, missing surface/dimensions/types, non-string MIME entries, fabricated kind fields in place of dragData, object dragData, unknown action, null args, and additional fields. The original valid drag-over and drop envelopes remain accepted.

Submitted the focused declared owner command `repo run @semio-tech/s-plugin-composition:test-layout-catalogue` for the red phase. It is queued behind existing owner requests and subject to normal capacity admission. No light test execution or custody/capacity override is used.

## Validation

The red-phase command did not reach the test runner: after 507 seconds of normal capacity admission it was cancelled with SIGINT (exit 130). It requested 1638 MB; the final refusal was that two learned owner reservations were still growing. No runtime red result is claimed.

Implemented the complete domain-owned JSON Schema, compiled both by the owned validator and the existing strict Ajv oracle interface. Neutral vectors, original positive catalogue cases, and actual mounted host emissions now require matching acceptance. A sequential green owner invocation was submitted using the same command (session 75424). It subsequently admitted, prepared the graph and three prerequisite targets, and completed with exit 0. Nx reported successful test-layout-catalogue execution and all three prerequisites, with zero cache hits (0/4). The focused renderer configuration has passWithNoTests:false and selects the catalogue file, so the command executes the neutral transport, fixture-envelope, and mounted-host laws. The final task run duration was 51.7 seconds after admission/graph preparation.

All new artifact absolute paths were checked below the 256-character limit; no runtime dependency or dependency manifest change was introduced.

## Files

- `✏️s/🧪️tests/🏘️plugin-composition/🛍️layout-canvas-catalogue/🟦️.ts`
- `✏️s/🧪️tests/🏘️plugin-composition/📜️script.ts`
- `✏️s/🧪️tests/🏘️plugin-composition/📋️project.json`
- `✏️s/🔌️plugins/📏️layout/🧪️tests/🧫️fixtures/🛍️canvas-catalogue/transport.json`
- `✏️s/🔌️plugins/📏️layout/🧬️schema/🛍️canvas-catalogue/🔣️.json`

## Baseline Oracle Dialect Repair

The root's executed TypeScript baseline exposed two independent oracle failures: font resources and Blueprint window ownership constructed Ajv2020 while their committed schemas declare draft07. The shared semioSchemaAjvV1 interface already constructs draft07 correctly; repaired the two callers to use it. No meta-schema validation was disabled and no adapter or dependency-store edits were needed.

The font schema also required coverage fields without declaring their property schemas. Strict required-property checks now remain enabled: added the missing text/string, preferred/font-index integer, and missingScalar/codepoint integer declarations, and a negative preferred-string assertion. The parent owns the sequential full baseline rerun.

Additional updated files:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🔮️oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧪️tests/🔬️window/🟦️.ts`

## Complete Remaining Ajv2020 Caller Audit

The root full Bazel TypeScript run executed 77 tests: 75 passed, 2 failed, and strict checking did not run. The actual failures identified drawing/composition Canvas publication and export checkpoint replay callers constructing Ajv2020 for declared draft07 schemas. A full scoped read found seven remaining Ajv2020 callers before this repair, rather than assuming a fixed caller count.

| Caller | Actual Schema Dialect | Decision |
| --- | --- | --- |
| Drawing composition Canvas publication | draft07 | Use existing strict semioSchemaAjvV1 interface |
| SVG paint | draft07 | Use existing strict semioSchemaAjvV1 interface |
| Export checkpoint replay | draft07 | Use existing strict semioSchemaAjvV1 interface |
| Output text fragments | draft07 | Use existing strict semioSchemaAjvV1 interface |
| Output storage grants | draft07 | Use existing strict semioSchemaAjvV1 interface |
| Export retained plan funding | 2020-12 | Preserve matching Ajv2020 |
| Drawing member birth | 2020-12 | Preserve matching Ajv2020 and existing explicitly registered draft07 reference meta-schema |

Each of the five repaired callers already sits behind a local test-only oracle facade and now delegates schema admission to the shared first-party strict draft07 oracle. Schema and meta-schema validation remain enabled. Reviewed the actual consumed schemas and their test fixtures: all required fields already have property declarations, so no additional schema edits were necessary. No negative cases, schema dialects, runtime dependencies, dependency manifests, dependency-store files, or infrastructure were changed. The root owns the next full test invocation; no duplicate test was started here.

Additional updated oracle files:

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🖼️composition/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🎨️faithful-primitives/🎨️svg-paint/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📍️checkpoint/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/📝️text/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/🧪️tests/oracle.mjs`

## Consolidated Lane File Roster

These 14 paths are the complete created/updated file roster owned by this execution lane; no files were removed. The first five cover the catalogue contract and focused owner gate; the subsequent eight cover matching oracle dialects and the existing font contract; this report is the final path.

- `✏️s/🔌️plugins/📏️layout/🧬️schema/🛍️canvas-catalogue/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🧪️tests/🧫️fixtures/🛍️canvas-catalogue/transport.json`
- `✏️s/🧪️tests/🏘️plugin-composition/🛍️layout-canvas-catalogue/🟦️.ts`
- `✏️s/🧪️tests/🏘️plugin-composition/📜️script.ts`
- `✏️s/🧪️tests/🏘️plugin-composition/📋️project.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧪️tests/🔮️oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config/🧪️tests/🔬️window/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧵️drawing/🖼️composition/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🎨️faithful-primitives/🎨️svg-paint/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📍️checkpoint/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/📝️text/🧪️tests/oracle.mjs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📤️output/🧪️tests/oracle.mjs`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/fleet-contract-2026-11-04.md`

The sampled focused-gate output exposes successful Nx completion and exit 0 but does not retain an individual Vitest test-count line. The catalogue file declares 12 transport vectors, one fixture-envelope law, and eight mounted host cases; this is a source inventory, not a claimed observed test-count line.

Post-repair scoped search across all Layout TypeScript/JavaScript/module sources confirms only the two matching 2020-12 oracle consumers remain.
