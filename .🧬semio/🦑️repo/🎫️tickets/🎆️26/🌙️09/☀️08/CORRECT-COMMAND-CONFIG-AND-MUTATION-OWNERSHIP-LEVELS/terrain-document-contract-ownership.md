# Terrain Document Contract Ownership

The test-first neutral oracle failed on the existing artifact schema rejecting the native `mesh` child as an additional property. Log: `🗑️generated/terrain-document-contract-red-1.log` (Nx exit 1). Twelve schema facets and native conversion/identity corrections are in progress; no runtime pass is claimed.

The persisted artifact and snapshot must preserve the supplied mesh handle. Deriving mesh content after an explicit terrain edit belongs to mutation preparation, not the artifact↔snapshot conversion boundary. Child metadata must name the actual `s.stdio.semio` artifact kind; its `v1/mesh` coordinate selects the subset. The global child identity remains equal to the target artifact ID.

Full child envelope persistence and Terrain window config ownership remain separate open work in the main checklist.

## Current Implementation And Validation

Twelve JSON/TypeScript/GraphQL/Proto facets now preserve the optional shared mesh handle. Snapshot TypeScript reuses the artifact parser; diff TypeScript reuses the artifact type rather than redeclaring it. Native artifact→snapshot and full replacement preserve the supplied child identity. Empty deltas preserve it as well; explicit geometry edits continue to derive the mesh in mutation application. Artifact and snapshot delta application now use one implementation.

Native slot metadata names `s.stdio.semio`, generated target IDs equal child IDs, and five committed fixture targets were corrected (see terrain-fixture-file-ledger.md). Existing mutation fixture adapters still derive placeholder mesh IDs with DefaultHasher; replacing that unstable persisted identity derivation with proper composition identity remains part of the open Terrain composition work.

The registered oracle plus strict TypeScript passed (Nx exit 0; `terrain-document-contract-green-1.log`), covering four committed native snapshots, two diffs and neutral rejection vectors. Native session 81925 is queued in `terrain-document-contract-native-1.log`; no native result yet.

Commands `terrain-document-contract` and `terrain-document-contract-native` are registered in root/ticket scripts and projects and both launch registries.


## Parent Edits Preserve Mesh Identity

Four neutral diff vectors cover an empty delta, exaggeration, imported features and full-artifact replacement. The independent fast-json-patch oracle and first-party TypeScript reducer agree exactly; Bun/Nx oracle plus strict TypeScript passed (`terrain-child-preservation-green-1.log`, exit 0) after the missing-reducer red test (`terrain-child-preservation-red-1.log`). Native document-contract assertions now apply those vectors and a real typed exaggeration mutation. Native verification is pending.

Native sparse diffs and the mutation helper retain the existing owned mesh handle when parent scalars change. Explicit artifact replacement takes its supplied handle. The fixture-report bridge and both committed mutation tests now decode exact authored handles instead of reminting both expected and actual snapshots. The unused snapshot-remint helper was removed. Exhaustive Rust/Python parity now includes `mesh`. Committed snapshot identities were not rewritten.

Open integration: new-document defaults still use a DefaultHasher construction seed; replace it with explicit identity allocation at document construction. Terrain's retained reducer still edits the parent only. Child payload stores, grouped publication and recursive archives remain required.

Additional exact file ledger:

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-exaggeration/🧪️tests/⛰️raises-exaggeration-from-1-to-2-5/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥change-imported-features/🧪️tests/📥️imports-harbor-position-descriptor/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏔️mutate-gisterrain-1/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏔️mutate-gisterrain-1/🐍️.py`


Map's native failure exposed the same absent deny_unknown_fields setting on Terrain artifact/snapshot/diff derives. Their closed-record annotations are now explicit before the queued Terrain retry reaches compilation. This change matches the already passing Ajv/TypeScript invalid-field cases; native verification remains pending.
