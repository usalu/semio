# Native Canonical Transform Grammar

The existing `print/🖋️latex/semio-viz-transform.sty` now dispatches the schema's 17 transform kinds from flat authored options. The implementation extends the existing working-table inference engine; it introduces no separate runtime module or external runtime dependency.

Structured filter compares numeric and categorical cells without evaluating expressions. Sorting is stable and supports multiple columns. Grouping preserves original rows and emits integer group labels; aggregates, rollup and summary preserve all group key columns. Fold preserves non-folded source columns, pivot aggregates duplicate keys, and joins omit the right join key while applying prefixes and overwriting colliding columns. Window, cumulative and normalization preserve original row order with grouped computation and custom output columns. Histograms and density tables emit conventional geometry aliases. Wide stack data routes through the existing stack order/offset implementation and restores original source cells. Regression emits predictions at original x positions and computed slope/intercept/r-squared metadata; it reuses the existing OLS and polynomial solvers. Density supports Gaussian, Epanechnikov, triangular and uniform kernels with the same robust Silverman default as TypeScript.

Kind-specific authored keys are validated before inference; unknown options, invalid normalization/regression/kernel/stack enum values, missing required stack/fold columns, nonpositive windows/bandwidths, too-small density grids, unknown columns and reversed bounds produce diagnostics. Empty geometry-producing tables remain empty without invalid numeric output. Existing catalogue shorthand remains part of the same engine, with schema `kind` dispatch occupying the canonical branch.

## Runtime Evidence

The parent's language-neutral JSON/Gherkin fixture initially failed with unknown transform kind keys. Initial completed native implementation compiled in Tectonic and passed 27 of 27 independent D3 numeric column checks. Extended fixtures added duplicate pivot keys, grouped centered windows with custom output columns, degenerate grouped extent normalization, stable multi-column sorting, grouped median, a uniform density kernel, and non-perfect regression metadata. The extended actual Tectonic run passed **37 of 37** independent D3 checks. The latest validation and empty-table change was rerun: **37 of 37 passed**, zero differential mismatches.

The fixture's row order, aliases, preservation, mathematical outputs and metadata are measured from native TeX probe records. Independent references use d3-array, d3-shape and d3-regression. Generated compiler inputs, PDF, logs and probe JSONL remain under this ticket's generated directory for parent cleanup. No modifying Git command was used.

## Files

- `print/🖋️latex/semio-viz-transform.sty`: schema key declarations, diagnostics, generic helpers/dispatch, domain override hook, shared OLS zero-denominator guard and density kernel extensions.
- `print/🧪️tests/🧬️chart-transform-grammar/🔣️.json`: extended authored vectors and extra option combinations.
- `print/🧪️tests/🧬️chart-transform-grammar/🟦️.ts`: extra vector compilation and independent D3 references, regression metadata checks.
- `print/🧪️tests/🧬️chart-transform-grammar/🥒️.feature`: parent authored neutral scenario.
- Ticket `🧫️generic-transform.tex`: preserved authored source fragment input.

The plot/native test owner registered `nativeTransformGrammarChecks` into the existing native grammar target. Parent performs final Nx execution and Rust-source integration compilation after serialized font staging.
Additional strict adapter-only TypeScript invocation was executed after final source edits. It failed on repository-wide missing d3 declaration packages and the existing Tectonic module's DOM/native Canvas types; this invocation is not reported as passing. The production canonical inference build completed earlier with actual strict tsc success. Parent owns final Nx/type packaging checks.
