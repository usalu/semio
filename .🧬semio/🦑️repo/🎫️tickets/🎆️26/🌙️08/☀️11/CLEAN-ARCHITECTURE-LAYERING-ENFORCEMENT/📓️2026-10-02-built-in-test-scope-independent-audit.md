# Built-In Test Scope Independent Audit

Read-only source audit, 2026-10-02. No compiler, Cargo, Nx, test execution, or Git mutation was performed. Rust name-resolution behavior below is a proof obligation, not a measured compiler result.

## Finding

`Library/🔍️discovery/🟦️.ts:6754` admits a function-body local scope when every attribute is exactly bare `test` or literal string-valued `doc`. The predicate inspects token spelling only. It does not consult import aliases, provider identity, enclosing scope bindings, source inclusion, or captured module participation. Therefore the implementation has no evidence distinguishing a built-in `test` marker from a same-spelled imported attribute provider. This is a concrete authority gap even before a native oracle establishes the exact Rust shadowing cases.

## Available Evidence

`inspectRustMutationMetadataFacts` exposes scoped `crateAliases` with source, alias, kind, conditional, and restricted status. It records `use` and reexport routes through `rustUseAliases`, plus extern aliases. These facts can detect explicit aliases to `test` and wildcard providers; their current consumers intentionally filter conditional routes, which must not silently establish absence of an attribute shadow.

`inspectRustModuleGraph` exposes contexts, targets, and accepted/refused participations and accepts already captured compile references. Its include traversal carries source scope and source chain. Provider analysis should use this captured graph and source reader; reparsing an isolated file cannot establish bindings contributed by included files or another module's public wildcard exports.

Existing mutation metadata provider-route machinery resolves scoped aliases, local targets, Cargo provider identity, and facades. It demonstrates available architecture, but its fixed MutationLeaf contract is not a generic proof that an arbitrary imported attribute cannot bind `test`.

## Recommended Closed Contract

Keep the spelling predicate as syntax classification, then require a separate origin proof before assigning local function scope. Refuse uncertain explicit `test` aliases, wildcard imports, conditional aliases, unavailable exports, conflicting routes, and imported source contributions unless the captured source graph proves that no relevant attribute binding reaches the marker. Resolve safe `use super::*` against the actual parent context and exports so genuine PDF/Home witnesses remain admissible; blanket wildcard rejection would unnecessarily lose those witnesses.

Add closed hostile rows for `use provider::rewrite as test`, direct `use provider::test`, grouped aliases, public reexports reached through `super::*`, external wildcard imports, conditional aliases, and included aliases. Each needs a real proc-attribute compiler oracle before asserting Rust execution semantics. Retain ordinary built-in test and literal-doc rows as positive witnesses.

Treat `doc` separately: literal assignment syntax is narrower than an attribute invocation, but this audit does not establish whether an imported same-spelled attribute is ignored, ambiguous, or rejected by Rust. Add a compiler-backed import-plus-literal-doc case and refuse unresolved origin until its constraint is measured. Do not generalize bare `test` conclusions to `doc` without that evidence.

## Fixture And Harness Review

The three appended rows retain the original corpus and use `nativeHarness: true`. Their supplied body bounds are 17..185, 44..212, and 45..213; these correspond to the opening-body token through closing-body token convention used by the inspected function visitor. The third row retains unsupported-expression refusal for dormant cfg_attr rewrite spelling.

The macro-owner native route conditionally adds `--test` to rustc and `--nocapture` to binary execution, and requires the one-passed test summary. This is the correct source-level route for executing the appended test functions. Native evidence remains pending; no pass claim is made here.
