# Window Taxonomy Audit

## Scope

The structural scope is every window under `✏️s/🔌️plugins/<plugin>/🎛️apps/**/🪟️windows/<window>`.
The required capability facets are `🎬️actions`, `🪛️utilities`, `🎚️options`, and `👥️presence`.
`🍱️panes` and `🪀️widgets` remain allowed but optional because they describe optional composition, not
capabilities every window must expose.

## Baseline

- Windows: 119
- Windows already containing all required facets: 2
- Missing `🎬️actions`: 117
- Missing `🪛️utilities`: 115
- Missing `🎚️options`: 105
- Missing `👥️presence`: 119
- Missing required facets in total: 337

The first three-facet audit preceded the presence requirement, so its total excludes the 119 missing
presence facets. No capability units had a TypeScript component leaf. Every concrete capability member had
a Rust leaf.

The first repair incorrectly represented empty facets with component leaves directly below the facet. It
created 924 invalid facet-level component leaves. Two pre-existing Puzzle 5D action bundles also used that
invalid aggregate shape.

The corrected tree currently contains 120 windows and 480 required facet directories. There are zero
facet-level component leaves. The 460 empty facets contain only `📌️empty.md`; the 62 specific items each
carry both `🦀️component.rs` and `🟦️component.ts`. The two Puzzle action bundles were decomposed into seven
specific action items while preserving their action identifiers and window bindings.

## Enforcement Design

`🔣️taxonomy.json` remains the single vocabulary source. `windowChildDirs` is the structural allowlist,
`windowRequiredChildDirs` declares the four mandatory facets, `windowComponentLangs` declares the Rust and
TypeScript item mirror requirement, and `windowEmptyFacetFilename` declares the only representation for a
facet with no items. The taxonomy self-validator requires both completeness lists to be non-empty and
unique, requires every mandatory facet to belong to the window allowlist and leaf-parent vocabulary,
requires every component language to resolve through `taxonomyLeafFilenames`, and requires a non-empty
marker filename.

The root policy walks every `🪟️windows` directory below every discovered taxonomy owner's apps tree. It
emits high-priority breaches for absent facets, facet-level component leaves, missing or stale empty
markers, and missing language leaves on specific items. Empty facets are valid; facet components are not.

## Verification

- Focused policy scan over plugin owners: zero window breaches.
- Current inventory: zero focused window breaches, zero facet-level component leaves, 460 empty markers,
  and 62 specific items with complete Rust/TypeScript mirrors.
- Shipped taxonomy validation: zero window-contract problems.
- `bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- -t window`: three tests passed, zero
  failed. The policy
  test covers absent facets, empty markers, stale markers, forbidden facet components, and specific item
  language mirrors.
- `bun nx run @semio-tech/puzzle-plugin:test-quick`: blocked before compiling Puzzle by three unrelated
  `semio-s-plugin-stdio` errors (`v_ecma_376::engine` and two missing `engine::sniff_real_bytes` symbols).
- `bun nx run @semio-tech/repo-lib:test-quick`: 143 passed and 18 unrelated pre-existing tests failed.
- `bun nx run @semio-tech/repo-lib:lint`: failed on pre-existing cross-package type errors; none point to
  the window policy, taxonomy contract, or generated component mirrors.
