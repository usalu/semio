# 💠️ Lowpoly schema layer — end-to-end validation (re-established, not inherited)

Session date: 2026-09-05. Ran against HEAD while a peer's repo-wide emoji-uniqueness rename wave
(`ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY`) was live in this exact tree; files were
re-read live rather than trusted from a stale listing, and unrelated rename churn was ignored.

Schema root:
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/`

A prior session claimed "ajv validates 17/17 recorded fixtures." That claim was **not** inherited —
everything below was re-run from scratch this session, with real command output kept as evidence.

## 1. Mutation enum vs directory set vs json/proto/graphql/ts catalogs

**Result: 17/17, set-equal in both directions, same declaration order, same spelling, no fixes needed.**

The Rust dispatch enum lives at `🧬️mutations/🦀️.rs` (`pub enum LowpolyMutation`, `#[derive(dsl::Mutations)]`,
17 variants) with a `pub const KINDS: &[&str]` mirror and an existing unit test
(`kinds_match_the_enum_and_the_catalog`) that already asserts `KINDS` against both the derive's own
`kinds()` and the oracle manifest — I read it, did not just trust it.

Directory set under `🧬️mutations/` (excluding the two shared non-mutation triad dirs `💾️binary/`
and `📝️text/`, which hold cross-mutation binary/text-format glue, not a mutation): exactly 17
directories, one per variant:

```
↗️move-object              ➕️insert-paint-layer        ➖️remove-paint-layer
🌫️change-paint-layer-opacity 🌱️create-object            🎛️change-paint-layer-blend-mode
🎨️edit-paint-layer          🏷️rename-object             👁️change-paint-layer-visible
💀️delete-object            📐️scale-object              🔀️reorder-objects
🔄️rotate-object            🔖️rename-paint-layer        🔘️change-object-smooth-shading
🕸️create-mesh              🧨delete-mesh
```

Cross-checked field-for-field against the enum's declaration order (`CreateObject, DeleteObject,
ReorderObjects, RenameObject, ChangeObjectSmoothShading, MoveObject, RotateObject, ScaleObject,
CreateMesh, DeleteMesh, InsertPaintLayer, RemovePaintLayer, RenamePaintLayer,
ChangePaintLayerVisible, ChangePaintLayerOpacity, ChangePaintLayerBlendMode, EditPaintLayer`):

- `🧬️mutations/🔣️.json` (dispatch `oneOf`, 17 branches, each `$ref`-ing its sibling
  `<mutation>/🧬️.schema.json`) — same 17, same order, same PascalCase tag.
- `🧬️mutations/🛰️.proto` (`message LowpolyMutation { oneof mutation { ... } }`, fields 1-17) — same
  17, same order.
- `🧬️mutations/🔗️.graphql` (`enum LowpolyMutationKind`, `input LowpolyMutationInput`, output
  `union LowpolyMutation`) — same 17, same order.
- `🧬️mutations/🟦️.ts` (`export type LowpolyMutation = | {...}`, `LOWPOLY_MUTATION_TAGS`) — same
  17, same order.

Per-mutation payload field names were also spot-checked directly from each `<mutation>/🦀️.rs`
struct against the aggregator json/proto/graphql/ts payload shapes (e.g. `move-object`: `id,
new_position` → `id, newPosition`; `create-mesh`: `id, child_id, target, mesh_workspace` → `id,
childId, target, meshWorkspace`) — all match.

## 2. ajv fixture validation — real run, real output

Helper script (kept, per instructions, as an input file in this ticket folder — not generated
output): `🔬️validate-lowpoly-fixtures.ts`. Uses the repo's own `node_modules/ajv` (v8.20.0,
already present at the repo root, no new dependency added). Run with:

```
bun "./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🔬️validate-lowpoly-fixtures.ts"
```

**Actual output (trimmed to the summary lines; full per-fixture PASS list also printed and was
inspected in full):**

```
Discovered 17 mutation directories with a 🧬️.schema.json payload leaf.
[... ajv "unknown format ... ignored" notices for the custom format hints float/double/uint32/
     int64/uint64 — see note below, not an error ...]

85/85 fixture validations passed.

Mutation-envelope-only tally: 17/17
```

Every mutation directory has exactly one recorded test case (17 total), each with 5 fixture files:
`🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`,
`🔺️diff/🔣️.json`, `🎯️outcome/🔣️.json` — 85 files. Validated:

- **17/17** `🦠️mutation` fixtures against `🧬️mutations/🔣️.json` (the dispatch envelope, with each
  `oneOf` branch's `$ref` manually dereferenced against the sibling `<mutation>/🧬️.schema.json`
  file before compiling — ajv's own URI-based `$ref` resolver percent-encodes the emoji path
  segments and cannot match them back to a literal registered id, so the script inlines the
  referenced payload schema instead of relying on ajv's resolver).
- **17/17** `📸️snapshot/⬅️before` and **17/17** `📸️snapshot/➡️after` against `📸️snapshot/🔣️.json`.
- **17/17** `🔺️diff` against `🔺️diff/🔣️.json`.
- **17/17** `🎯️outcome` fixtures are valid JSON but have **no lowpoly json-schema to validate
  against** — `{"status": "applied"}` is a generic protocol-level `MutationOutcome` shape, not one
  of the five lowpoly-owned representations in scope for this ticket. Not a defect; noted so the
  85-count is auditable (68 real schema validations + 17 read-as-JSON-only outcome files = 85).

Total: **85/85 passed**, of which **68/68** were real ajv schema validations and **17/17** were the
mutation-envelope fixtures the prior claim referred to specifically. The prior "17/17" claim is
re-established with fresh, real ajv output — not inherited.

Note on the "unknown format ... ignored" console lines: the lowpoly json-schemas annotate numeric
fields with non-standard `format` hints (`float`, `double`, `uint32`, `int64`, `uint64`) that exist
purely to preserve Rust's numeric width across the five representations (a documentation/codegen
aid, e.g. so a proto/graphql generator downstream knows `f32` vs `f64`). Ajv's default build has no
format vocabulary for those names, so it logs a warning and skips format-checking that keyword —
it does **not** skip the surrounding `type`/`required`/`additionalProperties` checks, which is what
actually caught the two historical defects below when they existed. This is expected ajv behavior
for custom formats, not a lowpoly defect, and every fixture still validated correctly against
`type`/`required`/`additionalProperties`.

## 3. The two known historical defects — confirmed gone; scan for the same class

- **Stale `meshJson: string` field**: `grep -rn '"meshJson"|mesh_json"'` across every `*.json`,
  `*.ts`, `*.proto`, `*.graphql` file under the schema root returned **zero matches**. The only
  surviving `mesh_json` occurrences anywhere in the schema tree are Rust **local variable /
  function names** (`snapshot_from_mesh_json`, test helper `tiny_mesh_json()`, doc-comments
  explaining the deliberate replacement) — never a serialized struct field. `LowpolyObject.mesh` is
  a real `LowpolyMeshHandle` (`childId` + `target: ArtifactRef`) everywhere: rust, json (with
  `oneOf: [null, LowpolyMeshHandle]`), proto (`optional`-by-message-presence), graphql
  (nullable `LowpolyMeshHandle`), ts (`LowpolyMeshHandle | null`). Confirmed gone.
- **Dangling `$ref` to undefined `LowpolySelection`**: `LowpolySelection` is now fully defined in
  every representation — `📸️snapshot/🦀️.rs` (imported from `crate::artifacts::lowpoly`),
  root `🔣️.json` `$defs.LowpolySelection` (referenced from `properties.selection` and matched by a
  real definition, fields `targets`/`keys`/`mode`/`ids`), `🛰️.proto` `message LowpolySelection`,
  `🔗️.graphql` `type LowpolySelection`, `🟦️.ts` `interface LowpolySelection`. Same for the diff
  family (`🔺️diff/🔣️.json` also defines `LowpolySelection` in its own `$defs`, since the diff
  document doesn't share a `$ref` target file with the artifact/snapshot ones). Confirmed gone.
- **Scan for new instances of the same class of defect** (a `$ref` to a `$defs` entry that was
  never defined in the same document): wrote a small Python pass over every non-fixture
  `🔣️.json` under the schema root that collects all `$defs` keys and all `#/$defs/...` refs and
  reports any ref without a matching def. **Zero missing definitions found** across
  `🔣️.json` (root), `📸️snapshot/🔣️.json`, `🔺️diff/🔣️.json`, `🧬️mutations/🔣️.json`, all 17
  per-mutation `🧬️.schema.json` leaves, and `💡️inferences/🔣️.json`.
- Also specifically checked the **inferences family** (`💡️inferences/`, a 5th schema family
  alongside artifact/snapshot/diff/mutations, added by ticket
  `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`) because its json
  schema references `#/$defs/LowpolyBounds` from a nested `📦bounds/` mutation-shaped leaf that
  only carries `🦀️.rs` + `🟦️.ts` (no per-leaf json/proto/graphql, by design — it's a pure
  computed inference, not a wire payload). Confirmed `LowpolyBounds` **is** defined in
  `💡️inferences/🔣️.json`'s own `$defs` (not dangling), and the `LowpolyInference` shape
  (`objectCount`, `bounds`) matches field-for-field across rust/json/proto/graphql/ts.

## 4. Root artifact / snapshot / diff field parity (broader check than the ticket strictly asked for)

Went one level deeper than the mutation catalog and also diffed the **full field sets** of
`LowpolyArtifact` (37 fields), `LowpolySnapshot` (2 fields: `schema`, `objects`), and `LowpolyDiff`
(38 fields: the artifact's 37 plus a nested `artifact` escape-hatch field) across all five
representations at the schema root and under `📸️snapshot/` and `🔺️diff/`. All five representations
agree exactly on: field names, camelCase spelling, nullability (`Option<T>` → optional key in
artifact/snapshot vs. `Option<Option<T>>` → required-but-nullable key in diff, consistently applied
by the `ArtifactSchema` derive and confirmed against a real diff fixture, which does emit every key
with an explicit `null` rather than omitting it — see `🏷️rename-object`'s recorded diff fixture).
Nested diff delta types (`LowpolyObjectsDelta`, `LowpolyObjectPatchEntry`, `LowpolyPaintLayersDelta`,
`LowpolyIndexedPaintLayer(Patch)`, `LowpolyPaintStrokeAt`, `PixelRun`, `LowpolyPaintLayerPatch`)
were also field-diffed rust-struct-vs-json-`$defs` and match exactly.

## What was fixed

**Nothing.** No genuine inconsistency was found anywhere in the lowpoly schema tree at HEAD: the
mutation enum/directory/json/proto/graphql/ts catalogs are set-equal in both directions with
matching order and spelling; both historically-known defects (stale `meshJson`, dangling
`LowpolySelection` `$ref`) are confirmed gone and no new instance of that defect class exists
anywhere in the tree; 85/85 recorded fixtures validate against their schemas with real ajv output
(68/68 real schema checks, 17/17 outcome files valid JSON with no applicable lowpoly schema). The
lowpoly schema layer is self-consistent at this commit.

## Files touched this session

- Added (kept, per instructions — an input file, not generated output):
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🔬️validate-lowpoly-fixtures.ts`
- Added (this report):
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️e2e-schema-validation.md`
- No files under `✏️s/🔌️plugins/💠️lowpoly/` were modified — none needed to be.
