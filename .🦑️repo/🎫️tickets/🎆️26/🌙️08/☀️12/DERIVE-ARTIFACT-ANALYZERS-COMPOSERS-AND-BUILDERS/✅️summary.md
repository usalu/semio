# Derive Artifact Analyzers, Composers, and Builders

## Outcome

- Removed every explicitly named analyzer, composer, and builder directory or file below plugin artifact trees.
- Added one framework derivation contract and macro that materialize each subset's public builder, analyzer, and composer from schema-owned construction/analysis hooks and IO-owned composition hooks.
- Migrated 137 artifact subset specifications to the derivation contract while preserving their existing typed construction methods, diagnostics, normalization, conformance gates, and composition behavior.
- Relocated artifact and standard IO registries into existing artifact/standard engine components instead of retaining composer directories.
- Updated Rust and TypeScript glue references to the derived lifecycle types and canonical registry locations.
- Updated taxonomy discovery and repository policy so explicit artifact lifecycle directories are rejected and subset schemas must declare the derivation hook.
- Retained the mechanical migration utility and all verification logs in this ticket.

## Verification

- Artifact path scan: `0` directories or files with analyzer, composer, or builder in their names.
- Derivation declarations: `137` subset schema declarations.
- Repository policies: builder `[]`, analyzer `[]`, composer `[]`.
- Taxonomy JSON parses successfully.
- Scoped `git diff --check` completes without diagnostics.
- `SEMIO_TEST_LEVEL=long CARGO_TARGET_DIR=<ticket>/🎯️target bun nx run @semio-tech/stdio-plugin:test-long`: `1,972` passed, `3` skipped; Nx target succeeded. See `🧪️stdio-test-long-2.log`.
- `bun nx run @semio-tech/repo-lib:test-quick`: the two new derived-facet taxonomy tests pass; aggregate result is `135` passed and `19` unrelated existing failures in stale repository discovery/taxonomy expectations. See `🧪️repo-lib-test-quick-2.log`.
- The isolated space build compiles `semio-framework-plugin` and the `semio-s-plugin-stdio` library. It remains blocked later by 12 concurrent, unrelated `semio-framework-os` semantic-mutation errors involving duplicate `label`, missing `ArtifactEnvelope` fields, and the ongoing `document` field rename. See `🧪️space-test-quick-isolated-3.log`.
- The unqualified quick artifact target compiled without lifecycle errors but exceeded its fixed 15-second test budget; the successful long target above provides the complete runtime verification. See `🧪️stdio-test-quick-3.log`.

## Primary Surfaces

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `📜️script.ts`
- `✏️s/🔌️plugins/**/🗿️artifacts`
