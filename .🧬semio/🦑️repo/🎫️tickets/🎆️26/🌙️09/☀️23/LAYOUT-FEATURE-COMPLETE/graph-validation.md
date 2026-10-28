# Layout Graph Validation

## Changes

The model owns a canonical graph validator in 🧬️schema/✅️validation/🦀️.rs. ReplaceDocument calls it before equality handling or diff construction, so unsupported schemas, malformed geometry, inconsistent memberships and missing references cannot publish a replacement graph. Validation leaves the supplied document untouched. Sparse diff application remains the only state writer.

CreateParentPage and SetParentPageContent both call the same model-owned normalize_parent_content helper. It checks parent identity/name/dimensions, unique owning layers and frames, existing story/image references, finite positive frame bounds, finite colors, valid text insets/columns and acyclic parent-local text threads. It derives layerIds and layer.objectIds from the authoritative layer/frame rows. Content replacement still removes stale inherited overrides and retains its existing semantic inverse. The former arbitrary 16-layer cap is removed.

Whole-document checks cover unique/non-empty collection ids, globally unambiguous local frame ids for cross-page threading, positive finite page/parent sizes, finite grid/typography/geometry/colors, UTF-8 style-run boundaries and style references, exact layer order/object memberships, bidirectional page/spread membership, parent references, unique valid inherited overrides, inherited/local frame-id collisions, existing text-frame thread targets and thread cycles. Parent frame/layer ids remain scoped to their owning parent; page layer ids remain scoped to their owning page. Thread edges across document pages are admitted. No collection-size caps are added. Zero-size guides/insets/overrides and zero-valued unresolved image metadata remain valid.

## Test-First Witnesses

The language-neutral corpus and native tests were written before the production validator and mutation guards. The corpus began with 49 document and 9 parent cases and was expanded to 61 document and 9 parent cases after initial implementation. Native tests assert validator decisions, unchanged bases on refusals, canonical parent memberships, diff inverses and semantic inverses. Additional native-only cases cover NaN and both infinities across 13 document numeric positions (39 cases), plus parent-creation frame rotation.

## Executed Evidence

- Direct execution of the persistent independent oracle module returned { documents: 61, parents: 9 }. Installed Ajv 8.20.0 validates independent JSON Schema projections of scalar rules, ids, references and memberships. A separate Kahn topological projection checks cycle outcomes. Installed fast-json-patch 3.1.1 constructs every replacement input and parent normalization output from RFC 6902 witnesses. The oracle imports no native validator/mutation code.
- The new graph oracle entry passed the repository OracleRegistryEntry schema using Ajv. A full-registry schema attempt exposed its pre-existing unsupported mutationCatalogs aggregate field; this report does not claim that the entire existing aggregate validates.
- Installed web-tree-sitter 0.20.8 with the installed Rust grammar parsed all seven changed native source/mount files without syntax errors.
- TypeScript 5.9.3 transpileModule reported no syntax diagnostics for the mounted Bun test source with Bundler module resolution.
- All introduced source paths are under 256 UTF-16 code units; the longest inspected source path is 152.

## Unverified Checks

Native Rust compilation, type checking, native test execution, and the Bun test-runner wrapper remain unverified. The available repo native runner was already known to be stale and silent; no native pass is inferred from it. Direct oracle execution and source parsing are the exact evidence above. Embedded Drawing content and referenced artifact internals remain governed by their owning codec/reference contracts; this helper validates the Layout graph itself.

## Files

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️create-parent-page/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📖️set-parent-page-content/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄️replace-document/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/oracle.mjs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/✅️validation/🔣️.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json
