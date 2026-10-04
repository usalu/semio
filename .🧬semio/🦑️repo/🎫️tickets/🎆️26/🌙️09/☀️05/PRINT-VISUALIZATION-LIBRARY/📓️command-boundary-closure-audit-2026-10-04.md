# Current Print Command Boundary Closure Audit — 2026-10-04

Executed existing independent esbuild fixture selectors with exact test build options: Node/ESM bundle, packages external, bun external, write false and metafile true. No production edit or budget change. Raw sorted paths and all import edges are retained at generated/command-boundary-closure-audit.json.

Measured compiler library/script22/23 and paint library/script7/9. The prior recorded19/20 compiler baseline differs by exactly three font-owned graph nodes, not two: pure metrics TypeScript, its metrics JSON, and tracked font catalog JSON. Removing these three names from the current22-node inventory gives19. Historical exact prior metafile was not available here, so exact baseline set equivalence is inferred from prior recorded counts and current ownership edges, not independently reconstructed.

The concrete import route is compiler -> font-catalog parent -> pure metrics -> metrics JSON and tracked catalog JSON. Parent stagePrintFonts uses printSansMetrics().sha256 to reject stale authored Anta metrics. Tracked catalog JSON previously read via filesystem by the parent is now a static import of the pure metrics owner. These are this ticket font derivation/contract features; they are not unrelated process/workspace imports. Earlier report already records leases/publication in17/18 baseline and workspace-payload/paint-resolution in19/20 baseline.

No test source or repository package barrel occurs in compiler inventory. No source change to frozen renderer/theme/chart is needed to resolve this metadata budget mismatch. Choose a schema-owned per-entry exact source inventory or intentional documented capacity policy; do not blindly increase the common limit. Compiler script requires23 if preserving the current admitted runtime graph.

## Exact Compiler Library Source Inventory

- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📥️source-staging/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts`

## Exact Internal Import Edges

- `🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json` (import-statement).
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` → `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` → `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🔣️.json` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` → `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts` → `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/📥️source-staging/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` → `🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📦️payload/🟦️.ts` (import-statement).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts` (import-statement).
