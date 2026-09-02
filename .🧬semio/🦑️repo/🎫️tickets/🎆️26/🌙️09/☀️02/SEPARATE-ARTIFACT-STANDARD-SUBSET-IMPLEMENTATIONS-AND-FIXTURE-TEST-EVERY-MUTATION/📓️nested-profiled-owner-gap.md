# Nested Profiled Owner Gap

## Decision

Selected option 1. Moving the three live GIS mutation trees would rewrite Rust module paths and
leave the framework's actual defect intact: profiled contribution owners may legitimately be
facets below a subset root.

`mutationCatalogProblems` now derives the profile from the first `🏅️standards` marker and accepts
the declared `🏅️standards/<standard>/🪆️subsets/<subset>` only when it is a complete owner-path
segment prefix. Thus a nested owner such as
`.../🪆️subsets/✳️any/✏️editor/🎚️config` is valid, while `✳️anywhere` and mismatched coordinates are
still rejected.

## Registration Constraint

The requested three catalogs cannot be added honestly without a second framework change. Each
vocabulary has direct Rust tests but no canonical Gherkin case or test-host adapter. A new catalog
therefore creates `mutation-catalog-unclaimed`; giving it a distinct editor-state capability also
creates `capability-without-manifest`. Reusing the document capability would instead create
`test-only-mutation` against the artifact runtime manifest. A no-oracle decision is explicitly
prohibited for runtime mutation capabilities.

No metadata was fabricated to bypass those gates. The three paths remain unregistered until their
direct suites are represented as canonical cases with real adapters and the editor-state capability
is modeled outside the artifact runtime-mutation manifest gate (or is given a genuine runtime
manifest and qualifying oracle).

## Verification

- `bun test --test-name-pattern='nested facet owner' 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` passed: 1 test, 3 assertions.
- `bun ./📜️script.ts test contract` was run before and after. The shared worktree remains red from
  unrelated work; the after sweep reported 1,953 high-priority breaches. Its selected counts were
  `unregistered-mutation-vocabulary: 13`, `mutation-catalog-unclaimed: 8`,
  `contribution-manifest-invalid: 0`, `mutation-vector-unregistered: 0`, and
  `mutation-catalog-capability-mismatch: 0`.
