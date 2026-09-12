# Terra Print Macro Staging Acceptance Audit — 2026-09-12

## Scope and disposition

This read-only audit covers the graph macro staging closure: the canonical graph owner and fixed package shim, compiler-only materialization to `_.tex`, per-invocation shared-library staging, producer input registration, the focused native production seam, and taxonomy enforcement. No product, lifecycle, Git, or AGENTS file was changed.

**Current disposition: accepted.** Independent static and portable controls pass, the registered native Nx control is green, and the fixed-shim literal-admission repair passes both Root’s broader boundary suite and this audit’s separate probe. The focused native control is intentionally bounded to a light (`dark: false`) print fixture and is not evidence for Mit Bericht, dark-mode, or panel-rendering execution.

## Exact ownership and materialization

| Concern | Current evidence | Result |
| --- | --- | --- |
| Semantic graph owner | `🧰️framework/🛍️products/📓️print/🔨️modules/🕸️graph/📐️.tex:164-167` defines `\SemioGraphState` and `\SemioGraphEdgeKind`; it is the only scoped TeX implementation context. | Pass |
| Fixed package identity | `🧰️framework/🛍️products/📓️print/🖋️latex/semio-graph.sty:1-3` contains only LaTeX package declaration and `\input{🔨️modules/🕸️graph/📐️.tex}`. `semio.cls:80` loads this fixed package identity. | Pass |
| Native compiler leaf | `🧰️framework/🛍️products/📓️print/🔨️modules/📥️source-staging/🟦️.ts:7-10` maps exactly `📐️.tex` to `_.tex`. The staging loop detects destination collisions before removing or writing its target (`:14-52`). | Pass |
| Exact library catalog | `.../tectonic-template-compilation/📇️catalog/🔣️.json:4-7` declares only `🖋️latex` and `🔨️modules/🕸️graph/📐️.tex`; `📇️catalog/🟦️.ts:8-10,20-21` validates and exposes that declared list. | Pass |

I independently ran the actual-catalog staging seam. It asserted that the declared two-root library stages the canonical graph source only at `modules/graph/_.tex`, preserves its bytes, omits a staged `📐️.tex` leaf, and rewrites the staged `semio-graph.sty` input to `modules/graph/_.tex`.

## Isolation and production paths

`publishPrintArtifact` at `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts:70-86` creates a private temporary root per invocation. It stages caller-owned document entries from `document.sourceRoot` to `source`, then separately stages `productRoot` with `printLibrarySources()` to `library` (`:75-76`). It passes that staged library to both dark and light compilation branches.

The compiler’s search paths contain `library/latex`, the staged library root, staged work and output directories, and fonts (`:194-202`); `TEXINPUTS` is constructed from the same staged paths (`:220-223`). Neither path uses the live print-library source tree. The temporary root is removed in `finally` (`:86`), and abort checks run before staging and before publication (`:74,83`). The dark path and panel recompile path both carry the same `libraryRoot` argument (`:175-190`).

`♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/📜️script.ts:29-31` supplies the Bericht root as a distinct `sourceRoot` to that same production seam, with `dark: false`. This establishes the cross-caller architecture in current code. It is not a native Mit compilation control.

## Taxonomy and fixed-shim gate

The only scoped `.tex` implementation context is `print-graph-macro-source` at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:2800-2811`, scoped precisely to `📓️print/🔨️modules/🕸️graph/*.tex`. An independent taxonomy query returned this single scoped TeX entry, so no repository-wide TeX-source exemption is present. The fixed `semio-graph.sty` package identity and its `tex` package-glue disposition appear at `:19486-19496` and `:23427-23433`.

The direct portable control verifies the canonical path, the `scoped:print-graph-macro-source` classification, and kind-only findings for `📐️reconcile.tex`, `reconcile.tex`, and `_.tex`; it also confirms that the neighboring documentation path remains outside this implementation scope. It accepts the fixed shim only as delegation and rejects bodies that introduce TeX definitions.

## Registered producer inputs

`@semio-tech/print:test-macro-staging` invokes `bun ./📜️script.ts test macro` and declares the stager, compiler tree, exact graph leaf, `latex` tree, taxonomy, discovery, and runtime prerequisites in `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json:109-145`.

I independently parsed current project definitions. The exact graph leaf is an input of that print target and of these six Mit targets: `build-forschungsbericht`, `build-kompaktbericht`, `build-zwischenbericht`, `test`, `test-exhaustive`, and `test-long`. The three producer target definitions explicitly list the graph leaf at `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json:34-92,94-152,154-212`.

## Executed controls

| Invocation | Result | What it establishes |
| --- | --- | --- |
| Direct exported `verifyPrintMacroStaging()` with `SEMIO_PRINT_OUTPUT_DIR` directed to this audit’s ticket scratch | Pass before and after the literal-gate repair — `Print macro staging: schema, references and collision atomicity PASS` | Fixture schema, canonical mapping and rewritten references, taxonomy/hostile-name ownership, fixed-shim grammar, and pre-mutation collision atomicity. | 
| Actual-catalog `stagePrintSources(productRoot, printLibrarySources(), auditScratch)` probe | Pass — exact two declared roots; canonical graph → `modules/graph/_.tex`; shim reference rewritten | Production catalog-to-staging closure without a compiler invocation. |
| `bun ./📜️script.ts test quick` in Mit Bericht’s TypeScript package | Pass | Validates the Mit catalog and producer/Nx ownership contract; it reported its document-router bundle boundary plus `20`, `10`, and `1` resolved document sources for Zwischenbericht, Forschungsbericht, and Kompaktbericht. |
| Registered `bun nx run @semio-tech/print:test-macro-staging --skip-nx-cache` | Pass — my observed pre-literal-repair run completed in 1m25s; Sol’s final post-literal-repair rerun completed in 2m52s | Both runs published two `dark: false` production-seam PDFs with SHA-256 `3226b206b9f90b1a3336df7c8238265d7a97f79ae38077bfd30b6da4095ed64a`; independent PDF.js marker passed. | 

The native control is `verifyPrintMacroStagingNative()` at `.../🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:211-233`. It invokes `publishPrintArtifact` twice, uses the actual graph `\SemioGraphState` and `\SemioGraphEdgeKind` control document, requires byte-identical PDFs, logs SHA-256, and reads the `Graph macro staging reached` marker using PDF.js. Its document has `dark: false` (`:216`).

## Limits and actionable findings

The fixed-shim literal-admission breach was repaired in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:10400-10405`. Metadata is restricted to inert literal characters; target text must be NFC and consist of literal, non-empty path segments, with control syntax, backslash, caret, and traversal segments rejected. Root’s current review records 2 literal positives and 11 hostile parser/fixed-disposition controls passing (`📓️coordinator-tex-shim-literal-review-2026-09-12.md`; `generated/coordinator/tex-shim-literal-hostile-repaired.json`). This audit separately passed the actual fixed shim plus five hostile forms: metadata `\def`, literal-backslash target, `^^5c` encoded command, `^^2e^^2e` encoded traversal, and `\csname` target expansion. The post-repair portable macro verifier also passed. Root’s ticket-owned native caret control remains supporting evidence that the former encoded traversal was semantically material under pinned offline Tectonic 0.16.9; it loaded its private caller-parent target and logged `ENCODED PARENT TARGET REACHED` (`📋️latex-literal-controls`; `generated/coordinator/tex-literal-native/result.json`).

The only execution limits are deliberate and must remain explicit:

- The native macro control exercises a print fixture source root, not a Mit Bericht source root. Cross-owner isolation is established by the producer seam and active target inputs, not by a native Mit compile.
- The native control uses `dark: false`. The code threads the staged library through dark and panel paths, but no native dark or panel-rendering result is claimed here.
- This audit does not assert the behavior of other TeX implementation domains; the scoped graph rule is intentionally narrow.

The concurrent Sol registered run is complete and green. I observed its live Tectonic invocation using the ticket-local staged `library/latex`, staged library root, staged source and output, and staged fonts search paths; this aligns with the current compiler construction. The literal-admission boundary is repaired and independently probed. Generated audit logs and staged scratch are disposable and will be removed after this report is finalized.
