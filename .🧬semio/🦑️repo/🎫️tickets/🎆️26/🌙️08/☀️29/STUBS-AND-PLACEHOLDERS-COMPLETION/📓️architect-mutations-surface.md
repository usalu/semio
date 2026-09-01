# Architect `🧬️mutations/🟦️component.ts` — two defects fixed

File owned exclusively for this slice:
`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

## Defect 1 — 266 undefined-type errors

Root cause: the file referenced 266 payload-interface names (`CreateInformationRequirement`,
`DeleteGrowthPlan`, …) that had no TS declaration anywhere — only the Rust structs existed, one per
triad leaf at `🧬️mutations/<triad-dir>/🦀️.rs`.

Fix: mechanically mirrored all 266 Rust payload structs into this file as local `export interface`
declarations, field-for-field, using a scratchpad Python script (not committed — lived only in
`/private/tmp/.../scratchpad/`) that:

1. Parsed `pub enum ProgramMutation { Variant(super::module::Variant), … }` from the top-level
   `🧬️mutations/🦀️component.rs` to get the ordered (variant, module) list (266 entries).
2. Matched each `module` (snake_case) to its triad directory by stripping the emoji prefix from
   each directory name and comparing the kebab-case remainder — 266/266 matched, zero misses.
3. Regex-parsed each triad's `pub struct Variant { pub field: Type, … }` body — 266/266 parsed
   cleanly, no unmatched lines, no empty-field structs, no name mismatches between the enum variant
   and its struct name.
4. Mapped Rust field types to TS: `EntityId` → `string` (kernel `EntityId` is
   `#[serde(transparent)]` around a `String`, confirmed in
   `🧬️kernel/🦀️component.rs:10-14`, so it serializes as a bare string, matching the sibling
   schema file's own choice to type every `*_id`/`*_ids` field as `string`/`string[]` rather than
   importing an `EntityId` alias); `String` → `string`; every other field type is a whole
   register/kernel struct, referenced by name and imported via `import type` from the sibling
   `../🟦️component.ts` (the registers/kernel mirror a peer agent wrote in this same ticket).
   Across all 266 leaves only 4 distinct "kinds" of field type occur: `EntityId`, `String`, or one
   of 66 entity/kernel struct names — no `Option<T>`, `Vec<T>`, `bool`, or numeric fields exist in
   any mutation payload.
5. Cross-checked the full 66-name import list against the sibling file's exports — zero missing.

**Shadowed-global trap** (flagged explicitly in the brief): the register type `Function` (used by
`CreateFunction`/`ReplaceFunction`'s `function: Function` field) collides with JS's global
`Function` type. Handled by importing it aliased: `import type { Function as FunctionEntity } from
"../🟦️component.ts"` and using `FunctionEntity` in this file's own declarations — never a bare
`Function` reference. Checked the rest of the 66 imported names against other known DOM/JS globals
(`Object`, `Record`, `Event`, `Range`, `Document`) — none of the other 65 collide; `Document` was
already avoided because the sibling schema file names that register `DocumentRecord` (Rust name is
`ArtifactRecord`, mapped here to `DocumentRecord` to match), which is what's imported for
`CreateDocument`/`ReplaceDocument`'s `document: DocumentRecord` field.

**Manual spot-check** (in addition to the full fixture cross-check under defect 2): hand-read the
Rust `🦀️.rs` source and compared field-for-field against the generated TS for 21 variants spanning
every field-type shape in the file: `CreateInformationRequirement`, `DeleteInformationRequirement`,
`RenameInformationRequirement`, `ReplaceInformationRequirement`, `ConnectAdjacency`,
`DisconnectAdjacency`, `ConnectTrace`, `DisconnectTrace`, `CreateFunction`, `DeleteFunction`,
`RenameFunction`, `ReplaceFunction`, `CreateDocument`, `ReplaceDocument`, `RenameGovernance`,
`ReplaceGovernance`, `RenameMeta`, `RenameProject`, `ReplaceProject`, `CreateAccessRule`,
`ReplaceStakeholder`. All 21 matched exactly (field names, order, and types).

## Defect 2 — wrong union discriminant casing (and a structural bug the casing fix exposed)

The Rust enum is internally tagged: `#[serde(tag = "mutation", rename_all = "camelCase")]`. For an
internally-tagged enum wrapping a newtype-of-struct variant, serde flattens the payload struct's
own fields into the *same* JSON object as the tag — there is no nested `payload` property. The
previous file had both defects: PascalCase tags (`"CreateInformationRequirement"` instead of
`"createInformationRequirement"`) *and* a nested `{ mutation, payload: {...} }` shape that doesn't
exist in the wire format at all.

Fixed shape, per variant:
```ts
export interface CreateInformationRequirement { informationRequirement: InformationRequirement; }
export interface CreateInformationRequirementOp extends CreateInformationRequirement { mutation: "createInformationRequirement"; }
```
i.e. the `Op` interface *extends* (intersects with) its flat payload interface and adds the tag —
matching serde's internally-tagged-flatten output exactly.

Tag derivation: serde's `rename_all = "camelCase"` on a PascalCase compound-word variant name is
just "lowercase the first letter" (no underscores to split on) — e.g. `RenameGrowthPlan` →
`renameGrowthPlan`. Field names use the standard snake_case → camelCase conversion (e.g.
`new_name` → `newName`, `information_requirement` → `informationRequirement`).

### Fixture-confirmed count: 266 / 266 (full population, not a sample)

Every one of the 266 triads has exactly one committed fixture at
`🧬️mutations/<triad-dir>/🧪️tests/<case>/🦠️mutation/🔣️component.json`. A verification script
loaded all 266, and for every one confirmed both (a) the fixture's `"mutation"` value equals the
generated camelCase tag, and (b) the fixture's non-`mutation` top-level key set equals exactly the
generated payload interface's field-name set (proving the flat/no-`payload`-wrapper shape, not just
the tag casing). Zero mismatches.

Representative table (full 266-row table omitted for space; every row was fixture-checked):

| Rust variant | TS tag | Fixture-confirmed |
|---|---|---|
| `CreateInformationRequirement` | `createInformationRequirement` | yes |
| `RenameGrowthPlan` | `renameGrowthPlan` | yes |
| `DeleteBenchmarkRecord` | `deleteBenchmarkRecord` | yes |
| `ConnectAdjacency` | `connectAdjacency` | yes |
| `DisconnectTrace` | `disconnectTrace` | yes |
| `RenameMeta` | `renameMeta` | yes |
| `ReplaceGovernance` | `replaceGovernance` | yes |
| `CreateFunction` | `createFunction` | yes |
| `ReplaceProject` | `replaceProject` | yes |
| `CreateDocument` | `createDocument` | yes |

(All other 256 variants: also `yes`, verified by the same automated pass — every triad directory
under `🧬️mutations/*` had a matching fixture and every fixture matched.)

## Verification (actually run)

- Direct-file typecheck — **0 errors** (confirmed twice, once after an initial JSDoc bug — see
  below):
  ```
  bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --esModuleInterop --skipLibCheck --allowImportingTsExtensions \
    "✏️s/…/🧬️mutations/🟦️component.ts"
  ```
  → no output (0 errors).

- Barrel stayed at 0:
  ```
  bunx tsc --noEmit --strict … "✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📦️index.ts"
  ```
  → no output (0 errors). Unaffected because this file still isn't imported from the barrel.

- `git status --porcelain -- <this file> | wc -l` → `1` (only this file touched by this slice).

**Bug caught during verification**: the first draft's file-header docstring used a `/** … */` block
comment containing the literal glob `tests/**/mutation` — the `**/` inside it prematurely closed
the block comment, corrupting everything after it (`Invalid character` / `Unterminated template
literal` at EOF). Fixed by rewording to `tests/<case>/mutation` (no `**/` sequence). Re-ran the
direct-file typecheck after the fix to confirm 0 errors — did not just trust the first (broken)
attempt.

## Files touched
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`
  (rewritten: 336 lines → 676 lines, adding 266 payload interfaces + fixing 266 discriminant tags
  and the payload-nesting shape).

No other files were modified. Scratchpad-only generator/verification scripts lived in
`/private/tmp/claude-501/-Users-ueli-Documents-semio/c17a0f0b-94f9-4f2f-bbd0-8ff82df33749/scratchpad/`
(`gen.py`, `render.py`, `leaf_data.json`, `component.ts` draft) and were not committed to the repo.
