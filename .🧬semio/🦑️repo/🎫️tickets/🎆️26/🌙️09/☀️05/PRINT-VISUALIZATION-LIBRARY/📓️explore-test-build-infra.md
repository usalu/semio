# Print Build/Test Infrastructure — Exploration

Read-only exploration by a Sonnet explorer (2026-09-05), persisted by the coordinator. Root: `🧰️framework/🛍️products/📓️print/`.

## 1. Test levels

Entry: `📦️packages/🟦️typescript/📜️script.ts:55-58` → `PrintPipelineVerificationCommand` (`🎮️commands/🧪️print-pipeline-verification/🟦️.ts:8-22`).

**A. Leveled tests** (`test [fundamental|quick|long|exhaustive]`, nx `test`, `test-quick`, `test-long`, `test-exhaustive`):
- `resolveTestLevel` from the shared repo library (`…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1136-1142`); always runs `verifyPrintPipelineQuick()`, plus `verifyPrintPipelineLong()` when level ≥ long. `exhaustive` does the same work as `long`.
- `verifyPrintPipelineQuick()` (`🧪️tests/🟦️.ts:21-121`): pure, no tectonic. Asserts gallery id↔filename fixture (`🗺️gallery-identities.json`), 81 sections, template filtering, token stylesheet macros, panel-glass style shape, dark-source derivation, no emoji in `\documentclass` options, template ids vs `🧫️merge-contract.json`, taxonomy leaf parsing vs markdown-it oracle, staging behaviour + collisions, PDF naming, and a battery of whitebox regexes against `.sty` sources (table/window internals). Ends with `verifyVisualizationCoverage()`.
- `verifyPrintPipelineLong()` (`🧪️tests/🟦️.ts:124-134`): needs tectonic. Provisions fonts, builds 6 templates × light/dark, only checks PDF files exist.

**B. Viz tests** (`test viz [coverage|full]`, nx `test-viz`, `test-viz-full`, `cache:false`):
- `verifyVisualizationCoverage()` (`🔨️modules/📊️visualization-gallery/🟦️.ts:106-116`): no tectonic. Leaves parsed two ways vs `🖼️assets/🔣️viz-taxonomy.json`, 80 unique sections, no duplicate leaves, zero missing `% viz-covers:`, `viz-api.tex` mentions 17 `\SemioViz*` commands. Purely textual.
- `verifyPrintVisualizationBuild()` (`🧪️tests/🟦️.ts:137-166`): needs tectonic. Builds all gallery PDFs, asserts existence; for `viz-api` only: pdfjs-dist text contains 6 fixed strings (`Cell text`, `Module text`, `Chip text`, `Label text`, `Title text`, `42`); `pdfStableHash` identical across two rebuilds (determinism, not correctness).

**Tectonic acquisition** (`🔨️modules/🖨️tectonic-template-compilation/🟦️.ts:216-264`): probes system `tectonic`, else downloads pinned 0.16.9 release into `.🧬semio/🦑️repo/⚡️cache/tectonic/<version>/`. Windows gap: extraction tries `unzip` then `tar`, but `runCmdStatus` throws on spawn ENOENT (`📚️library/🏃️process/🟦️.ts:94-98`), so a stock Windows box without `unzip` throws instead of falling back to `tar`. Not verified at runtime. Fonts: `🔨️modules/🔤print-font-catalog/🟦️.ts:39-58` downloads 3 TTFs on demand with magic-number validation.

## 2. Per-component assertions

None. 82 gallery `.tex` files, but no test asserts anything about an individual visualization's rendered output. Coverage is comment-based (`% viz-covers:`), build success is `existsSync`, content checks apply to one probe document with 6 generic strings. `sharp`/`@napi-rs/canvas` are used only for the panel-glass production feature, never for image assertions.

## 3. Repo convention for language-agnostic tests + third-party cross-validation

Print does not follow it. The convention elsewhere: Gherkin `🥒️.feature` + `defineTestAdapter({ implementation, scenarios: { name: { oracle, subject } } })` run through `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/` (multi-language runners, `🏃️runner/🦀️.rs` orchestrator).
- Differential example: `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/✅️satisfy-version-requirements/🟦️.ts` — oracle = npm `semver`, subject = own `versionSatisfies`, vectors from the Gherkin data table.
- Spec-as-oracle example: `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities/🟦️.ts`.
- Multi-language parity: `…/🧪️test/🧪️tests/🖥️host-protocol-parity/` with `🟦️.ts`, `🐍️.py`, `🐹️.go`, `🔷️.cs`, `🦀️.rs` adapters on one feature.
- Print's `🧫️merge-contract.json` carries dead fields (`documentCommands`, `graphCommands`, `treeCommands`).

## 4. launch.json registration

`launch.json` is generated from `.vscode/🧩️launch.seed.jsonc` by `generateLaunchJson()` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts`). Print entries are static objects in the seed's `configurations` array: `🛠️dev🖨️print📊️viz` (seed 353-358), `🛠️dev🖨️print` (360-368, group `3_dev`, order 175), `📦️build🖨️print📊️viz` (6023-6029), `🧪️test🖨️print📊️viz` / `🧪️test🖨️print📊️viz🌕️full` (6030-6043), `📦️build🖨️print` (6149-6158, group `4_build`, order 102), `📦️preview🤖️print-latex-tokens` (6457-6466, order 206.07). Naming: `<action-emoji><verb><product-emoji>print[<scope-emoji>]`.

## 5. Third-party dependencies

`package.json:19-26`: `@napi-rs/canvas`, `pdfjs-dist ^6.1.200`, `sharp` as runtime `dependencies`; `markdown-it` dev. All used in `🖨️tectonic-template-compilation/🟦️.ts` (`renderPrintPanelGlass` 131-183, `loadPdfCanvas` 185-193, private) and the test file (text extraction). No exported type leaks. Two other packages pin `pdfjs-dist ^5.4.296` independently (animate plugin, ui react target).
