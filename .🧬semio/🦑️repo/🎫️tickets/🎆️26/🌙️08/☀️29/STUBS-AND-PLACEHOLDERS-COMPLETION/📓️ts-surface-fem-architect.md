# TS surface repair — 🏗️fem and 🏛️architect

## Scope
Fix the real `tsc --strict` errors newly visible in `🏗️fem` (76) and `🏛️architect` (69) after the
barrel-import path repair earlier in this ticket. Repro command (per plugin):

```
bunx tsc --noEmit --strict --target ESNext --module ESNext \
  --moduleResolution bundler --esModuleInterop --skipLibCheck --allowImportingTsExtensions \
  "✏️s/🔌️plugins/<plugin>/📦️packages/🟦️typescript/📦️index.ts"
```

## Result
| Plugin | Before | After |
|---|---|---|
| 🏗️fem | 76 (75×TS2304, 1×TS2552) | **0** (verified, exit code 0) |
| 🏛️architect | 69 (68×TS2304, 1×TS2552) | **0** (verified, exit code 0) |

## 🏗️fem — TS2304/TS2552 (undefined entity names)
All 76 errors were in the ◻2d and 🧊3d artifacts' `🧬️schema/🟦️component.ts`,
`🧬️schema/📸️snapshot/🟦️component.ts` and `🧬️schema/🔺️diff/🟦️component.ts` — each file referenced
entity types (`FemNode`, `FemElement`, `FemMaterial`, `FemSection`, `FemSupport`, `FemLoadCase`,
`FemCombination`, `FemAnalysisSettings`, `FemCamera`, `FemRegion` for 2d / `FemSolid` for 3d) that
were never declared anywhere in TS.

Convention check (via already-passing sibling plugins `📋️forms`, `🧩️puzzle`, and fem's own
`✏️editor/🎚️config/🧬️schema/🟦️component.ts`): each of `schema/component.ts`,
`schema/snapshot/component.ts`, `schema/diff/component.ts` is **self-contained** — it declares its
own local copies of every entity type it needs rather than importing from a sibling file. Fem's own
`schema/mutations/component.ts` (already passing, not touched) confirmed the exact field shapes
(camelCase, discriminated unions tagged on `kind`).

Fix: added a `//#region 🔖️Entities` block to each of the 6 files, mirroring the Rust structs
field-for-field, verified against:
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🦀️component.rs` (2d: `FemNode`, `FemDof`, `FemElement`
  Bar/Beam, `FemMaterial`, `FemSection`, `FemSupport`, `FemLoad` Nodal/MemberUdl/Area, `FemLoadCase`,
  `FemRegion`, `FemCombinationTerm`, `FemCombination`, `FemAnalysisSettings`, `FemCamera` — all
  `#[serde(rename_all = "camelCase")]`, `FemElement`/`FemLoad` are `#[serde(tag = "kind", rename_all
  = "camelCase")]`).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️component.rs` (3d: same shapes but **not** identical to
  2d — `FemElement` is Bar/Frame (Frame has extra `roll`), `FemMaterial` has extra shear modulus `g`,
  `FemSection` has `iy/iz/j` (not just `iy`), `FemCombination.terms` is `BTreeMap<String, f64>` →
  `Record<string, number>` (not `FemCombinationTerm[]` like 2d), `FemCamera` is `{ json: string }`
  (opaque string), not `{ x, y, zoom }` like 2d's config-lane `FemCamera`. Caught this divergence by
  reading each Rust file separately per the ticket's serde-caution note — did not assume symmetry).
- Cross-checked every shape against `🧬️schema/🧬️mutations/🟦️component.ts` (2d and 3d), which
  already had correct, passing local copies of these same entities for mutation payloads.

`diff/component.ts` additionally needed a full local `Fem2dArtifact`/`Fem3dArtifact` interface for
its `artifact?:` replacement field — duplicated field-for-field from the sibling `schema/component.ts`
(same duplication-across-siblings convention `🧩️puzzle` already uses for `Puzzle2dArtifact`).

One `TS2552` (fem `Fem2dElement` typo diagnostic "Did you mean 'Element'?") resolved automatically
once `FemElement` was declared.

## 🏛️architect — TS2304/TS2552 (69 undefined register-entity names)
All 69 errors were in one file: `🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`
(`ProgramArtifact`), referencing 69 register-entity types (`ProgramMeta`, `ProjectDefinition`,
`Stakeholder`, … through `Governance`, `AdjacencyKind`) that had zero TS declarations anywhere in
the plugin (confirmed via `client entity-emojis`-style grep sweep — no `component.ts` under
`🧬️schema/🗄️registers/` or per-mutation-leaf folders; only the dispatch-union
`🧬️schema/🧬️mutations/🟦️component.ts` exists, and it only declares `*Op` wrapper types +
`ProgramMutation`, not the payload/entity interfaces themselves — that file is not on this plugin's
`📦️index.ts` import graph and so wasn't checked by this tsc invocation; it was left untouched).

Source of truth: `🗄️registers/🦀️component.rs` (11,401 lines, ~65 entity structs — "all 65 feature
areas" per its own header comment) and `🧱️kernel/🦀️component.rs` (shared `EntityId`, `EntityHeader`,
`Ownership`, `TextField`, `TaggedNote`, `TimestampMeta`, `Priority`, `LifecycleStatus`, `TraceLink`,
`QuantitySpec`).

Key structural finding: **every one of the 65 register entities uses
`#[serde(flatten)] pub header: EntityHeader`** — the header's fields (`id`, `name`, `description`,
`status`, `priority`, `ownership`, `tags`, `notes`, `timestamps`) serialize flat onto the entity, not
nested under a `header` key. Mirrored this as `interface X extends EntityHeader { ...ownFields }`
(structurally identical to the flattened wire shape) rather than a nested `header: EntityHeader`
field, which would have produced the wrong JSON shape.

Two deliberate non-1:1 name choices, both verified before accepting:
- `ts field functions: Function[]` — TS's `Function` interface (global, callable-signature type)
  was silently shadowing the intended domain type before this fix (no compiler error, but
  semantically wrong — an array of "callable functions" instead of "program function rows"). Fixed
  by declaring a **local** `export interface Function extends EntityHeader {...}` (module-scoped
  declarations shadow the ambient global by TS name resolution rules; verified no merge conflict —
  `tsc` after the change is clean). Mirrors Rust `pub struct Function` (`🦀️component.rs` line 1011).
- `ts field documents: DocumentRecord[]` — no Rust struct named `DocumentRecord` exists; the actual
  struct is `ArtifactRecord` (`document_type`, `title`, `version`, `file_ref`, … — unambiguously the
  same "document management register" entity). Declared `DocumentRecord` as `ArtifactRecord`'s exact
  field mirror under the pre-existing TS field's own chosen name (did not rename the existing
  `ProgramArtifact.documents` field, which is out of this ticket's scope).

Given the scale (69 entities × field lists, several dozen supporting enums), generated the TS from
the Rust source programmatically rather than by hand-transcription, to eliminate manual-copy field-
name/type-drift risk: a Python script (kept at
`/private/tmp/claude-501/.../scratchpad/gen_ts.py`, scratch-only, not part of the repo) parsed each
target `pub struct`/`pub enum` block from the two Rust files, converted snake_case fields to
camelCase, mapped `Option<T>` → optional field, `Vec<T>` → `T[]`, `BTreeMap<K,V>`/`HashMap<K,V>` →
`Record<K,V>`, `EntityId`/`String` → `string`, numeric Rust types → `number`, and unit-only enums
(all of them here, `#[serde(rename_all = "camelCase")]`) → camelCase string-literal unions;
recursively pulled in every referenced custom type. Zero warnings on the final run (all 69 target
names plus their transitive dependencies resolved against the Rust source). Spot-verified several
generated interfaces (`Adjacency`, `Stakeholder`, `Governance`, `Function`) line-by-line against the
Rust source by hand — camelCase field names line up positionally with a `#[serde(rename_all =
"camelCase")]` snake→camel conversion of each Rust field, and optionality lines up with `Option<T>`.

Inserted the generated 107 type declarations (enums + interfaces, ~1650 lines) as a
`//#region 🔖️Entities` block at the top of `program/schema/component.ts`, ahead of the pre-existing
`ProgramArtifact` interface (left untouched).

## Verification
```
$ bunx tsc --noEmit --strict ... "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📦️index.ts"; echo $?
0
$ bunx tsc --noEmit --strict ... "✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📦️index.ts"; echo $?
0
```
Both plugins are now fully clean under the ticket's repro command — no remaining errors to report as
unfixable.

## Files touched (mine only — verified via `git status --porcelain` on each exact path)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️component.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️component.ts`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️component.ts`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️component.ts`

No other files under `🏗️fem` or `🏛️architect` were modified by this session. `git status` shows many
more files changed under both plugins (barrel `index.ts`, mutation/presence/io schemas, `.g4`/
`.proto`/`.graphql`/`.ebnf`/`.json`/`.protocol.semio` mirrors) — those are concurrent other-session
edits per this ticket's multi-dev nature, not this pass's work; left untouched and unreported as
mine.

## Unfinished / out of scope
None within the two plugins' `tsc --strict` surface — both are at 0 errors. Not investigated (out of
this task's stated scope): `program/schema/mutations/component.ts`'s own undefined `Create*`/
`Delete*`/`Rename*`/`Replace*` payload types — that file isn't reachable from
`🏛️architect/📦️index.ts`'s import graph so the repro command never touches it, but it would likely
fail its own standalone `tsc` check today. Flagging for whoever owns that file/ticket area next.
