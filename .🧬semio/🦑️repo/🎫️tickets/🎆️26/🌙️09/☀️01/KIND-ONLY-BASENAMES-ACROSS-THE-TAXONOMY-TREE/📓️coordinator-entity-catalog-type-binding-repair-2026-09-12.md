# Entity Catalog Type Binding Repair

The coordinator found a generated TypeScript binding defect while checking the queued generator extraction. The current entity catalog lives at `schema/🤖️generated/🏷️entity-kinds/🟦️.ts`, but its producer emitted `../🟦️` for the schema-owned types. Native TypeScript resolution returned no module for that import. The normal freshness check still passed because the producer and generated file shared the same wrong coordinate.

The producer now emits `../../🟦️`. Its ordinary `generate` route regenerated the TypeScript projection. No compatibility facade or copied type definitions were introduced. Generator ownership extraction and the generated Rust leaf move remain pending in the existing execution packet.

## Verification

- First-red native TypeScript resolver evidence: `🗑️generated/coordinator/schema-entity-type-import-resolution.json`; the original type-only import resolves to null.
- The package's existing generation command completed with exit zero.
- The repaired generated leaf has zero native TypeScript syntax diagnostics, and its sole type import resolves exactly to `schema/🟦️.ts`. The executed check emitted `[DEBUG] generated entity catalog type import resolves to its actual schema owner`; exact resolution is retained in `schema-entity-type-import-resolution-repaired.json` under coordinator generated output.
- The existing entity-kind Vitest suite passed six tests in 1.33 seconds. It compares native runtime catalog/first-wins behavior with the owned parser and AJV and checks the three projections' source provenance. This is not a whole-project typecheck.
- The registered `bun nx run @semio-tech/framework-schema:check --skip-nx-cache` route completed with exit zero using private Nx workspace/cache directories under this ticket.

This is a two-file import repair covered by existing schema/runtime cases and the actual native resolution failure/pass. No test was added merely to repeat a relative string.

## Exact Owned Product Files

- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts`

This retained report is also an authored ticket file. Earlier prerequisite source hashes are historical review provenance and must not become permanent implementation snapshots.
