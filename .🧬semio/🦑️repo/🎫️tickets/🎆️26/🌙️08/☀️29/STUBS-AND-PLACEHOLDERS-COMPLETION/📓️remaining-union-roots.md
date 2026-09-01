# Remaining 6 Mutation Union Roots — Filled

Filled the 6 empty `🧬️mutations/🟦️component.ts` union-root placeholders (`export {};`) with real
discriminated unions mirroring their sibling Rust `…Mutation` enums, following the two-shape wire
audit from `📓️mutation-union-wire-audit.md`. Shape was determined by reading the attribute block
immediately preceding each `pub enum …Mutation`, then confirmed against a committed
`🧪️tests/**/🦠️mutation/🔣️component.json` fixture per artifact (all 6 had at least one fixture per
variant — no artifact required attribute-only inference).

## Summary table

| Artifact | Shape | Fixture-confirmed | Variants |
|---|---|---|---|
| flow | A (internally tagged, `mutation` camelCase) | yes, all 10 | 10 |
| process3d | A (internally tagged, `mutation` camelCase) | yes, all 16 | 16 |
| cad | A (internally tagged, `mutation` camelCase) | yes, all 20 | 20 |
| present (animate) | B (externally tagged, PascalCase key) | yes, all 9 | 9 |
| note | A (internally tagged, `mutation` camelCase) | yes, all 33 | 33 |
| mathematical | B (externally tagged, PascalCase key) | yes, all 15 | 15 |

Total: 103 mutation variants typed across the 6 files.

## Per-artifact detail

### flow — shape A
`✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🦀️component.rs`: `#[serde(tag = "mutation", rename_all =
"camelCase")]` directly above `pub enum FlowMutation`. 10 variants, all with
`#[serde(rename_all = "camelCase")]` on their leaf structs **except** `DuplicateWidget`, which
carries none — confirmed via its own fixture (`👯️duplicate-widget/…/🦠️mutation/🔣️component.json`):
fields stay snake_case (`source_id`, `new_id`, `synapse_id`, `from_port`, `to_port`) even though the
`mutation` discriminant itself is still `"duplicateWidget"`. None of the 10 leaves have a sibling
`🟦️component.ts` payload file (unlike `trinity/jack`'s reference shape), so payload interfaces are
defined inline in the union file, importing only `Widget`/`WidgetLayout` from the artifact's root
`../🟦️component.ts`.
File: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

### process3d — shape A
`✏️s/🔌️plugins/🏭️process/…/🧬️schema/🧬️mutations/🦀️component.rs`: `#[serde(tag = "mutation",
rename_all = "camelCase")]` above `pub enum Process3dMutation`. 16 variants, every leaf struct
carries `#[serde(rename_all = "camelCase")]`. Reused existing root-schema TS types
(`Process3dStep`, `Process3dWorkshopMachine`, `Process3dPose`, `Process3dStepOrigin`,
`Process3dCapability`, `ArtifactChildHandle`) rather than redefining them.
File: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

### cad — shape A
`✏️s/🔌️plugins/📐️cad/…/🧬️schema/🧬️mutations/🦀️component.rs`: `#[serde(tag = "mutation",
rename_all = "camelCase")]` above `pub enum CadMutation`. 20 variants (4 child-slot
create/delete pairs for shape/building/energy/structure-classic models, a drawing-collection
create/delete pair, node create/delete/rename, 5 reference-overlay field mutations, replace-
references, replace-reference-media, change-active-model-definition). Defined a local `CadReference`
interface (mirrors `crate::artifacts::cad::CadReference`, camelCase) since the root schema only
exposes the map-keyed `CadReferenceList` shape, not the per-reference type. Reused existing
`CadNode` from the root schema. Verification found 3 pre-existing TS errors unrelated to this file
(`🧰️framework/🔨️modules/🧊️3d/🟦️.ts` missing `tessellate`/`dispose` exports from an unbuilt
`flow_core` wasm pkg, plus one `library` package error) — confirmed identical with the original
`export {};` placeholder in place (temporarily swapped back in and re-ran tsc), so these are
pre-existing/environmental, not introduced by this change.
File: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

### present (animate) — shape B
`✏️s/🔌️plugins/🎞️animate/…/🧬️schema/🧬️mutations/🦀️component.rs`: `pub enum PresentMutation` has
**no** `#[serde(tag = ...)]` at all → serde default externally-tagged shape
`{ "<PascalCaseVariant>": {...} }`. 9 variants, every leaf struct carries
`#[serde(rename_all = "camelCase")]`, so payload fields are camelCase even though the envelope is
externally tagged. Confirmed via fixtures, e.g. `{"RenameTile":{"id":"t-hero","newName":"Hero"}}`.
Reused existing `FigureTileFrame`/`FigureTileSource`/`FigureTileDraft` from the root schema.
File: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

### note — shape A
`✏️s/🔌️plugins/🗒️note/…/🧬️schema/🧬️mutations/🦀️component.rs`: `#[serde(tag = "mutation",
rename_all = "camelCase")]` above `pub enum NoteMutation`. 33 variants (9 document-root scalar
setters, 3 asset mutations, 21 block mutations including table row/column insert/remove). Every
leaf struct carries `#[serde(rename_all = "camelCase")]`. Reused `NoteBlockNode`/`NoteImageAsset`
from the root schema; defined local `NoteTextRun`/`NoteTextParagraph` interfaces (mirroring
`crate::artifacts::note::{NoteTextRun, NoteTextParagraph}`, both camelCase) since no root-schema TS
type existed for them — needed by `EditBlockText.newParagraphs`.
File: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

### mathematical — shape B
`✏️s/🔌️plugins/➗️mathematical/…/🧬️schema/🧬️mutations/🦀️component.rs`: `pub enum
MathematicalMutation` has **no** `#[serde(tag = ...)]` → externally tagged. 15 variants, and
**none** of the 15 leaf structs carry `#[serde(rename_all = ...)]` — confirmed field-by-field
against fixtures, e.g. `{"ChangeNodeLabel":{"id":"n-alpha","new_label":"Alpha"}}` and
`{"UpdateGraphAlgorithm":{"new_algorithm":"","new_algorithm_seed":null}}` — so every leaf field
stays literal Rust snake_case (matches `layout`'s convention, unlike present/note's camelCase
leaves in the same repo). The one referenced structured type, `MathematicalGraph`, DOES carry its
own `#[serde(rename_all = "camelCase")]`, so `ReplaceGraph.graph.algorithmSeed` stays camelCase —
that casing belongs to the referenced type, not to the leaf's own fields (confirmed via the
`replace-graph` fixture's `"algorithmSeed": null`). Reused `MathematicalGraph`/`MathematicalPoint`
from the root schema; defined a local `EquationNodeLabel = number` type alias (mirrors the Rust
newtype `EquationNodeLabel(pub u64)`, which serializes as a plain number, confirmed via the
`change-coefficient` fixture's `"label": 2`).
File: `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

## Verification (all run, actual output below)

```
bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
  --esModuleInterop --skipLibCheck --allowImportingTsExtensions <plugin>/📦️packages/🟦️typescript/📦️index.ts
```

- flow: 0 errors.
- process3d: 0 errors.
- cad: 3 pre-existing errors, all outside this ticket's file (`🧰️framework/🔨️modules/🧊️3d/🟦️.ts`
  wasm-pkg import gaps + one `library` package error); reproduced identically with the original
  `export {};` placeholder swapped back in, confirming they predate and are unrelated to this
  change.
- animate (present): 0 errors.
- note: 0 errors.
- mathematical: 0 errors.

`git status --porcelain` restricted to the 6 target files reports exactly 6 modified files — no
other files touched by this change.

## Honest boundaries

No artifact in this batch had a zero-variant enum or a genuinely-unimplemented facet — all 6 Rust
enums had real variants, and every variant had at least one committed fixture, so no attribute-only
inference or "not yet implemented" doc comment was needed anywhere in this batch.
