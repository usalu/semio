# Print and Mit Bestand Merge Audit

Source: `temp/merge/print` and `temp/merge/mit-bestand`, directory snapshots without Git metadata. Integration target is the current shared `🐙ueli/⛳wip` checkout at `fe7c8a8f8b` plus its existing local work. No modifying Git operations are used.

Current product roots: `🧰️framework/🛍️products/📓️print` and `♻️mit-bestand`. Preserve current product modules, compiler-safe staging and semantic filenames. Port incoming visualization packages, report family, actor-network validation/rendering and source content. Exclude incoming build output and exploratory `_full.mjs`/`_sheet.mjs`.

Research is a separate submodule with Neo4j as current authority. Compare incoming historical research to existing archive by content before placing any unmatched documents in the documented intake path; do not replace graph authority or guidance.

Repository goal resource read through the owned stdio MCP server; ticket opened via `ticket_open` under Consistent Repo History. Model metadata uses supported `codex` because the MCP model vocabulary does not accept `gpt-6`.

## Content Decisions

Imported print layout, navigation, bibliography backrefs, graph/tree/viz styles and gallery sources. Retained current bare-name font search and generated design-token authority. Imported all three report documents and their supporting assets; remapped references to current semantic paths. Preserved the newer demonstrator and presentation architecture; incoming changes there are older API/path variants. Diagnostic `zz-verify-skalierung.tex` remains an input snapshot only.

## Shared Report Appendices

The incoming Zwischenbericht referenced five appendices absent from its own snapshot directory. These documents existed in the incoming Forschungsbericht. Their source now lives once under `♻️mit-bestand/📋️bericht/📎️anhang`, referenced by both reports and staged with each build. Report-specific project appendices remain separate. This includes the actor ledger and its rendered fragments.

## Compiler Build Budget

The full interim report exceeded the generic 600-second subprocess ceiling during active TeX rendering. TeX compilation now uses the shared 1,200-second build budget (`SEMIO_BUILD_BUDGET_MS` override), as other compiler invocations do. No timeout was treated as a successful test.
