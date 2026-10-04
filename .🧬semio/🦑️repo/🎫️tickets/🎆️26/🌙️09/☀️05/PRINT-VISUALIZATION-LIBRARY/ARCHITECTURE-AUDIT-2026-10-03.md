# Print Mutation and Inference Architecture Audit

Read-only production inspection on 2026-10-03. No tests or runtime execution were performed by this audit. Other agents were modifying native inference and bootstrap concurrently; findings describe the observed files.

## Actionable Findings

1. **Inference result contracts disagree across implementations.** `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json` requires `tikz: string` and `diagnostics: string[]` and disallows all additional properties. Its neighboring Rust implementation defines exactly those two fields. The neighboring TypeScript file defines and returns optional `plan`, `tikz`, `scene`, required `complete`, and diagnostic objects `{code,path,message}`. Successful ordinary TypeScript inference therefore violates the declared inference JSON schema; failed inference also omits required `tikz`. The TypeScript field metadata lists plan/tikz/scene whereas native metadata lists tikz/diagnostics. Resolve with one explicit schema-first contract, or separate genuinely distinct inference contracts and identifiers.

2. **Intentional unconfigured native default (resolved by parent context).** `📸️snapshot/🦀️.rs` constructs width/height/layers with no language. The product schema `ChartSpecification` requires language (line 48299), and native inference requires an explicit `en` or `de` (line 141). `ChartInference::default` infers from that default and necessarily returns diagnostics. The parent confirms this is intentional and tests assert invalid inference until language is explicitly authored; it is not an actionable defect.

## Confirmed Structure

Computation files reside under the print product's `🧬️schema/💡️inferences` tree. The TypeScript package and Bun lock workspace entries reference that moved location. A targeted search of package.json, bun.lock, and .vscode/launch.json found no viz-kernel references. This does not assert absence in every historical ticket document.

Native chart mutations implement the existing protocol Mutation trait and descriptor. Native bootstrap builds an ArtifactDeclaration with schema and inference service descriptors through the existing Plugin builder, then registers chart inference through the existing app registry.

TypeScript package declares D3 oracles under devDependencies only. Its package task targets delegate to `📜️script.ts`, and launch.json includes the package name in executable targets. No external runtime imports were found by a targeted import/export scan of the inference TypeScript tree for D3/node/Bun/semio references. This scan was static, not a bundling verification.

## Exact Files Inspected

- `🧰️framework/🛍️products/📓️print/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🦀️.rs`
- `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json` (targeted lines)
- `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🦀️.rs`
- `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🦀️.rs`
- `🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🦀️.rs` (header and targeted lines)
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts` (first 85 lines)
- `package.json`, `bun.lock`, `.vscode/launch.json` (targeted references)


## TypeScript Mutation Follow-Up

Also inspected the entire `🧰️framework/🛍️products/📓️print/🧬️schema/🔀️diff/🟦️.ts`, entire `🧬️mutations/🔣️.json`, and entire `💡️inferences/✅️validation/🟦️.ts`.

`changeVizChartValue` produces the typed `VizChartDiff`, verifies it through `applyVizChartDiff`, rejects invalid results with an empty diff and typed messages, and suppresses equal-value no-ops. Replay clones the base, validates path and guarded before-values, applies edits atomically, and returns the original base on failure. For nonempty edits, the resulting authored chart is validated against the owned `ChartSpecification` JSON schema through `validateJsonSchemaSubset` imported from the existing framework schema validator. No standalone chart validator exists in this path. This is static confirmation; no runtime or tests were executed.
