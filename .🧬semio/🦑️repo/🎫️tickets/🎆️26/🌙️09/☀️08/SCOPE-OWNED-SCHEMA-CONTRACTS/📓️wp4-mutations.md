# WP4 — Mutation Payload Schemas

Partition: every `🧬️mutations/**` subtree repo-wide **except** `✏️s/🔌️plugins/🗄️stdio/**` (sibling worker) and
`.🧬semio/🦑️repo/🎫️tickets/**` (frozen captures of the 2026-08-12 SEMANTIC-MUTATIONS-OVERHAUL ticket; they hold
only `🦀️.rs`, no descriptors or schemas).

Scripts kept in this folder (inputs, not generated output):
- `wp4-mutation-aggregates.py` — census + canonicalizer (`--apply`, `--plugins-only`, `--only <prefix>`, `--out <json>`).
- `wp4-mutation-validate.mjs` — draft-07/ajv validator (`bun <ticket>/wp4-mutation-validate.mjs`).

## 1. What the partition looked like before

159 `🧬️mutations` roots, 79 with an aggregate JSON Schema, **1765 mutation leaves**.

| plugin | aggregates G-A | aggregates G-B | leaves | A `🧬️.schema.json` | B `🧬️schema/🔣️.json` | C `🔣️.schema.json` | D vcs pair | E seq pair | F missing |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| ✒️writer | 0 | 1 | 14 | 14 | 0 | 0 | 0 | 0 | 0 |
| ➗️mathematical | 1 | 0 | 17 | 17 | 0 | 0 | 0 | 0 | 0 |
| 🌀️procedural | 2 | 0 | 37 | 35 | 0 | 0 | 0 | 0 | 2 |
| 🌊️flow | 1 | 0 | 10 | 10 | 0 | 0 | 0 | 0 | 0 |
| 🌍️gis | 1 | 4 | 24 | 14 | 10 | 0 | 0 | 0 | 0 |
| 🌿️vcs | 0 | 1 | 6 | 0 | 0 | 0 | 6 | 0 | 0 |
| 🎞️animate | 1 | 0 | 12 | 12 | 0 | 0 | 0 | 0 | 0 |
| 🎥️shooting | 1 | 0 | 41 | 41 | 0 | 0 | 0 | 0 | 0 |
| 🎪️demonstrator | 1 | 0 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| 🎬️sequence | 0 | 1 | 13 | 5 | 0 | 0 | 0 | 8 | 0 |
| 🏗️fem | 1 | 1 | 50 | 50 | 0 | 0 | 0 | 0 | 0 |
| 🏛️architect | 1 | 0 | 268 | 268 | 0 | 0 | 0 | 0 | 0 |
| 🏭️process | 1 | 0 | 16 | 16 | 0 | 0 | 0 | 0 | 0 |
| 💠️lowpoly | 0 | 1 | 17 | 17 | 0 | 0 | 0 | 0 | 0 |
| 💡️reasoning | 1 | 0 | 13 | 13 | 0 | 0 | 0 | 0 | 0 |
| 📋️forms | 1 | 0 | 22 | 22 | 0 | 0 | 0 | 0 | 0 |
| 📏️layout | 1 | 0 | 32 | 32 | 0 | 0 | 0 | 0 | 0 |
| 📐️cad | 1 | 0 | 21 | 21 | 0 | 0 | 0 | 0 | 0 |
| 📕️norm | 16 | 0 | 393 | 393 | 0 | 0 | 0 | 0 | 0 |
| 📖️playbook | 1 | 0 | 13 | 13 | 0 | 0 | 0 | 0 | 0 |
| 📜️imperative | 0 | 1 | 8 | 8 | 0 | 0 | 0 | 0 | 0 |
| 📸️remodel | 1 | 0 | 43 | 43 | 0 | 0 | 0 | 0 | 0 |
| 🔋️energy | 1 | 0 | 275 | 275 | 0 | 0 | 0 | 0 | 0 |
| 🔱️trinity | 0 | 2 | 35 | 35 | 0 | 0 | 0 | 0 | 0 |
| 🕸️dag | 1 | 0 | 18 | 18 | 0 | 0 | 0 | 0 | 0 |
| 🖍️draw | 1 | 0 | 14 | 0 | 0 | 11 | 0 | 0 | 3 |
| 🖨️raster | 1 | 0 | 12 | 12 | 0 | 0 | 0 | 0 | 0 |
| 🗒️note | 1 | 0 | 38 | 38 | 0 | 0 | 0 | 0 | 0 |
| 🧩️puzzle | 3 | 0 | 89 | 86 | 0 | 0 | 0 | 0 | 3 |
| 🧱️block | 3 | 0 | 104 | 104 | 0 | 0 | 0 | 0 | 0 |
| 🪐️space | 0 | 2 | 5 | 5 | 0 | 0 | 0 | 0 | 0 |
| 🪵️sourcing | 0 | 1 | 3 | 3 | 0 | 0 | 0 | 0 | 0 |
| 🧰️framework + 💻️os | 6 | 14 | 101 | 5 | 96 | 0 | 0 | 0 | 0 |
| **total** | **50** | **29** | **1765** | **1626** | **106** | **11** | **6** | **8** | **8** |

(Counts are a working-tree snapshot; peers added one `🔋️energy` leaf mid-run — see §3 — so the final leaf count is 1766.)

### Correction to WP0 §6 Family G

WP0 left the G-A/G-B ratio open ("only 4 modules sampled"). The full scan says **50 of 79 aggregates were G-A**,
and the drift is worse than "architect hand-duplicates a stub":

- **39 aggregates were not mutation unions at all.** `🧬️mutations/🔣️.json` in `📕️norm`×16, `🧩️puzzle`×3,
  `📐️cad`, `🎥️shooting`, `🌀️procedural`×2, `➗️mathematical`, `🏭️process`, `🎞️animate`, `💡️reasoning`,
  `📋️forms`, `📏️layout`, `🗒️note`, `🖍️draw`, `🎪️demonstrator`, `🌊️flow`, `🌍️gis`, and 6 framework/os fixtures
  held a **whole-artifact snapshot schema** (`type: object` + the artifact's own state properties, `x-semio-state:
  "artifact"`). `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/…/🧬️mutations/🔣️.json` was a byte-copy of the module
  schema `…/🧬️schema/🔣️.json` with only `$id`/`title` changed — pre-migration content stranded at the mutation root.
- **11 aggregates were `oneOf` unions over inline `$defs`** (architect, fem2d, fem3d, puzzle2d, block×3, remodel,
  energy, dag, raster, playbook) — the drift class WP0 confirmed on architect.
- 5 aggregates classified "G-B" still had to be rewritten: `🏗️fem/🧊️3d/🌐️any` used inline `oneOf` branches with no
  `$defs` at all, and `🎬️sequence/✳️any` plus the four `🌍️gis` roots referenced leaves by absolute `$id` URL, which
  the new leaf `$id`s invalidate.
- **Dialect:** 24 aggregates and 213 leaf schemas were 2020-12, not draft-07.

## 2. What changed

### 2.1 Leaf payload schemas → `<leaf>/🧬️schema/🔣️.json`

Every leaf in the partition now carries exactly one payload schema at the taxonomy default
(`mutationPayloadSchemaLocation` / `mutationPayloadSchemaRelativePath()`), and its descriptor's `payloadSchema`
reads `🧬️schema/🔣️.json`.

| family | leaves relocated | source file(s) removed |
|---|---:|---|
| A flat `🧬️.schema.json` | 1621 (1616 plugins + 5 `💻️os/🎚️config`) | `<leaf>/🧬️.schema.json` |
| C flat `🔣️.schema.json` (`🖍️draw`) | 11 | `<leaf>/🔣️.schema.json` |
| D `📋️.schema.json` + `🧬️wire/🔣️.schema.json` (`🌿️vcs`) | 6 | both, plus the empty `🧬️wire/` dir |
| E `🧬️.schema.json` + `🛜️wire/🔣️.schema.json` (`🎬️sequence`) | 8 | both, plus the empty `🛜️wire/` dir |
| B already canonical (`🌍️gis` 10, framework/os 96) | 106 | — (re-stamped in place) |
| F authored (see §2.3) | 8 | — |
| **total files removed** | | **1656** |

Every relocated schema is stamped:
- `"$schema": "http://json-schema.org/draft-07/schema#"` (2020-12 migrated: `unevaluatedProperties`/
  `unevaluatedItems` dropped, `prefixItems` → `items` array + `additionalItems: false`),
- `"$id": "https://semio.tech/schema/<scope>/mutation/<semanticKind>.json"` where `<scope>` is the mutations
  root path with the structural segments (`🗿️artifacts`, `🏅️standards`, `🪆️subsets`, `🧬️schema`, `🧬️mutations`,
  `🔌️plugins`, `🔨️modules`, `🛍️products`, `🧩️extensions`) removed and each remaining segment reduced to its
  ASCII tail — e.g. `s/space/home/1/any`, `s/norm/en1990/1/any`, `framework/os/config`. All 1766 `$id`s are unique.
- `"title"` = the descriptor's `aggregateVariant` (the Rust payload type name), so the export id is stable and the
  `mutation/schema-parity` leaf identity check (`source.includes(semanticKind) || source.includes(variantName)`)
  still holds on the moved file.

Relative `$ref`s inside relocated flat schemas were deepened by one segment (3 files in `📸️remodel`:
`../../🔣️.json#/properties/camera` → `../../../🔣️.json#/properties/camera`).

**D/E wire pairs** are now two exports of one leaf module:
`{ $schema, $id, title, "$ref": "#/$defs/Payload", "$defs": { "Payload": …, "Wire": … } }`.
The wire envelope's cross-file `$ref` (`…/payload.json#/properties/tag`) became the intra-document
`#/$defs/Payload/properties/tag`. The root `$ref` keeps "the leaf schema is the payload" true for the aggregate.

### 2.2 Aggregates → pure `$ref` unions

All **59 plugin aggregates** (58 `🔣️.json` + `✏️s/🔌️plugins/📕️norm/🎚️config/…/🧬️mutations/🔣️.schema.json`,
renamed to `🔣️.json` per contract §B) are now:

```json
{ "$schema": "http://json-schema.org/draft-07/schema#", "$id": …, "title": …, ["description": …,]
  "oneOf": [{ "$ref": "./<leaf>/🧬️schema/🔣️.json" }, …],
  "x-semio-mutationKinds": ["<semanticKind>", …] }
```

No `$defs`, no inline payload content, no absolute-URL refs. `$id`, `title` and `description` are carried over
from the previous file where present. `x-semio-mutationKinds` is what keeps
`policyMutationStructuralBreachesView`'s aggregate identity check (`📜️script.ts:29085`,
`surface.source.includes(semanticKind)`) satisfied — a `$ref` path alone does not contain the semantic kind for
nested leaves such as architect's `ℹ️information-requirement/🌱️create` → `create-information-requirement`.

Five roots have an aggregate but no leaves of their own (they union across sibling subsets):
`➗️mathematical/➗️equation/✳️any`, `🏗️fem/◻️2d/🌐️any`, `🏗️fem/🧊️3d/🌐️any`, `🖍️draw/🖍️drawing/✳️any`,
`🗒️note/🗒️note/✳️any`, `🎬️sequence/🎬️sequence/✳️any`. For these the union was rebuilt from the aggregate
`🦀️.rs` enum's variant list matched against the artifact's leaves across all subsets (15/25/25/14/33/8 variants,
all resolved, none missing), with `../../../<subset>/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` refs.

The other format aggregates (`🦀️.rs`, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`, `📖️.grammar.semio`) were checked and
contain **no inline payload definitions** — the Rust ones are thin one-field-wrapper enums over the leaf types,
as WP0 §6 G-A stated. They were not restructured.

### 2.3 Family F — missing / mis-declared payload schemas (8 in this partition)

WP0 §5 counted 5 non-stdio leaves whose descriptor requires `json-schema` with no file at the declared path;
the stricter scan found 3 more in `🖍️draw` whose descriptor pointed at a Rust type through a path the
`mutation/descriptor-bijection` gate rejects (`🦠️mutation/🦀️.rs#Type` — the non-JSON branch at
`📜️script.ts:29045` only accepts `🦀️.rs#Type`), so all 8 were breaching.

| leaf | authored from | result |
|---|---|---|
| `🌀️procedural/🌀️generation2d/…/➕create-generation` | `CreateGeneration { generation: FormGeneration }` + the module's own `$defs.FormGeneration` | `{generation: FormGeneration}` |
| `🌀️procedural/🧊️generation3d/…/➕create-generation` | same | same |
| `🧩️puzzle/🧊️3d/…/📏scale-object` | `ScaleObject { id: String, new_scale: Option<Puzzle3dScale> }`, `Puzzle3dScale = Uniform(f64) \| Vec3([f64;3])` (documented wire shape: bare number or 3-array) | `{id: string; newScale?: number \| [n,n,n]}` |
| `🧩️puzzle/🧊️3d/…/📐scale-target-volume` | `ScaleTargetVolume { id, new_scale }` | same shape |
| `🧩️puzzle/🖐️5d/…/📏scale-part3d` | `ScalePart3d { id, new_scale: Option<Puzzle5dScale> }` | same shape |
| `🖍️draw/…/🎨️style/🎨️replace-layer-fill` | the leaf's own orphaned `🔣️.schema.json` (complete, `title: ReplaceLayerFill`) | relocated; descriptor now points at it and declares `json-schema` |
| `🖍️draw/…/🎨️style/🖊️replace-layer-stroke` | same | same |
| `🖍️draw/…/🧱️structure/➕️create-layer` | `CreateLayer { parent_id: Option<String>, index: Option<usize>, layer: Box<DrawingLayerNode> }` | `{parentId?, index?, layer}` with `layer` a cross-scope `$ref` to `https://semio.tech/schema/s/drawing/drawing/artifact.json#/$defs/DrawingLayerNode` (contract §B cross-scope form; the module already owns that export, so it is referenced rather than duplicated) |

The puzzle/procedural leaves have no `🧪️tests` example payloads to validate against (their fixtures are
Rust-value-driven, WP0 §4); the shapes follow the neighbouring leaves' committed convention exactly
(`🌀rotate-target-volume` is the byte-for-byte template for the three puzzle scale leaves: `Option<T>` → present
but not `required`).

### 2.4 Consumers rewritten

| file | change |
|---|---|
| 12 aggregate `🦀️.rs` (writer, gisterrain, demonstrator, sequence, imperative, energy, trinity×2, space×2, sourcing, vcs) | `assert_eq!(descriptor["payloadSchema"], …)` and `owner.join(…)` literals → `"🧬️schema/🔣️.json"`; the now-duplicate payload/wire assertion blocks in vcs (6) and sequence (8) collapsed to one each |
| `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📜️script.ts` | `ConfigMutationSourceScript`: leaf path, aggregate path (`🔣️.schema.json` → `🔣️.json`), `payloadRef` now `aggregate.oneOf[0].$ref`, `validateMutation` now validates the payload directly (the aggregate no longer describes a `{Variant: payload}` wrapper), and the four "undeclared wire form" negatives were re-expressed at payload level (`{Snapshot:{}}`, `{SetSelectedCheckIndex:{index:1}}`, `{index:1,unknown:true}`, `{index:"5"}`); plus `ajv.addVocabulary(["x-semio-mutationKinds"])` and `$id`-keyed leaf registration (see §4.4) |
| `✏️s/🔌️plugins/🌿️vcs/…/✳️any/🔮️oracle/🔣️.json` | mirrored `payloadSchema` values (6) and the rationale/`specificationSource` prose |
| `✏️s/🔌️plugins/🧩️puzzle/◻️2d/…/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts` | `LEAF_SCHEMA` constant |
| 22 `🧪️tests` oracles (`🐍️.py`, `🟦️.ts`, `🥒️.feature`) across vcs, demonstrator, fem×10, process, reasoning, imperative, energy, dag, puzzle×3, space×2 | docstring/feature path references |
| 3 aggregate `description` strings (fem2d, fem3d, lowpoly) | `<kind>/🧬️.schema.json` → `<kind>/🧬️schema/🔣️.json` |

Repo-wide fixed-string sweep confirms **zero** remaining `🧬️.schema.json` / `🔣️.schema.json` / `📋️.schema.json` /
`🧬️wire/` / `🛜️wire/` references inside any `🧬️mutations` tree in the partition, and **zero** `*.schema.json`
files left under those trees. Remaining matches elsewhere in the repo are fixture schemas outside mutation trees
(e.g. `✏️editor/🧪️fixtures/*/🧬️.schema.json`) and historical ticket markdown — neither is WP4's.

### 2.5 End state (`wp4-mutation-aggregates.py`, no `--apply`)

```
{"mutationRoots": 159, "aggregatesPresent": 79,
 "aggregateKinds": {"G-B": 73, "G-A": 6},
 "aggregateShapes": {"oneOf-union": 70, "object-snapshot": 6, "allOf": 2, "uninhabited": 1},
 "leaves": 1766, "families": {"B": 1766},
 "gaByScope": {"framework": 6}}
```

Every leaf in the partition is family B (`<leaf>/🧬️schema/🔣️.json`). Every plugin aggregate is G-B. The 6
remaining G-A are the framework/os single-mutation conformance fixtures of §5 (`object-snapshot` shape by design),
and the 2 `allOf` + 1 `uninhabited` aggregates are also framework/os fixtures, likewise `$ref`-composed.

## 3. Peer churn observed

A peer added `✏️s/🔌️plugins/🔋️energy/…/🧬️mutations/🧫️change-plant-loop-return-temperature` mid-run; it was picked
up on a second `--apply` pass (the converter is idempotent). Another peer was mid-rename of
`🧰️framework/🔨️modules/🎭️actor/📃️pageSchema` → `📃️page`, which broke every `bun 📜️script.ts` entry point for
~15 minutes ("Cannot find module '../📃️pageSchema/🟦️.ts'"); the parity run below was made after it resolved.

## 4. Verification

### 4.1 Structural + ajv (own validator)

`bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/wp4-mutation-validate.mjs`

```
roots=159 leaves=1766 aggregates=79 ajvCompiled=59
WP4 partition (✏️s/🔌️plugins, non-stdio) problems=0
framework/os mutation trees (deferred, see §5) problems=340
  🧰️framework/…/🕸️dag/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧬️schema/🔣️.json: dialect https://json-schema.org/draft/2020-12/schema
  🧰️framework/…/🕸️dag/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧬️schema/🔣️.json: missing $id
  🧰️framework/…/🕸️dag/🧬️schema/🧬️mutations/🗃️replace-node-properties/🧬️schema/🔣️.json: missing title
  …
```

The validator asserts, per leaf: descriptor `payloadSchema` is the taxonomy default **and** the target exists;
the file parses; `$schema` is draft-07; `$id` and `title` are present; `title == aggregateVariant`; `$id` is unique
repo-wide. Per aggregate: dialect is draft-07, every `oneOf` branch is a `$ref`, every relative `$ref` resolves to a
real file, every cross-scope `$ref` has a module owner, and ajv (draft-07) compiles the union with all referenced
leaf schemas registered. **All 59 plugin aggregates compile; 0 problems in the WP4 partition.**

Third-party validation of our own layout, per the repo rule that a feature needs an external oracle: `ajv` 8.x
(draft-07 mode) is the independent implementation compiling the unions here, and `📕️norm`'s own
`config-mutation-source` oracle (§4.2) re-validates the same contract through a second, independently written ajv
harness with hostile-payload negatives.

### 4.2 Rust — `dsl::Mutations` / `#[mutation_leaf]` accepts the new `payloadSchema` paths

The proc-macro (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`, region `🔣️MutationLeafJson`)
parses every leaf `🔣️.json` at expansion time and requires exactly the 14 declared keys, so a bad descriptor is a
build error, not a lint.

`CARGO_TARGET_DIR=…/scratchpad/target-w7 RUSTC_WRAPPER="" CARGO_PROFILE_WASM_DEV_DEBUG=false`
`cargo check --target wasm32-wasip2 -p semio-s-plugin-note` (note: 38 leaves relocated across 10 mutation roots):

```
    Checking semio-s-artifact-stdio-semio v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust)
    Checking semio-s-plugin-note v0.1.0 (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 7m 52s
```

**0 errors.** Warnings in the log are pre-existing `unnecessary qualification` / `unused import` lints in
framework and `🗄️stdio` artifact crates, unrelated to this change (they are present in the same crates before it).

The same command for `-p semio-s-plugin-writer` was run first and got as far as type-checking
`semio-s-plugin-trinity` (a crate whose 35 leaves and 2 aggregate `🦀️.rs` files this WP edited), i.e. macro
expansion of the new descriptors succeeded there too, then stopped on **6 pre-existing peer errors unrelated to
WP4**:

```
error[E0308]: mismatched types
    --> ✏️s/🔌️plugins/🔱️trinity/…/🧬️schema/🛜️wire-runtime/🦀️.rs:1695:43
     |
1695 |                     self.initial_digest = None;
     |                     -------------------   ^^^^ expected `ManuallyDrop<Option<...>>`, found `Option<_>`
     = note: expected struct `ManuallyDrop<Option<ArtifactStoreInitializationDigest>>`
                  found enum `Option<_>`
error: could not compile `semio-s-plugin-trinity` (lib) due to 6 previous errors
```

All six are the same `ManuallyDrop` deref mistake at
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs`
lines 1695, 1696, 1745, 1746 (+2), in a file WP4 did not touch — a peer's in-flight
`ArtifactStoreInitializationDigest` refactor. Reported, not fixed (contract §E).

### 4.3 Repo-wide mutation parity CLI — four runs, none reached a verdict

`bun 📜️script.ts clean taxonomy inventory --kind mutation --format json`
(`policyMutationStructuralBreachesView`, the `mutation/schema-parity` family — WP0 §4).

1. **Run 1 failed at module load**, before any taxonomy work:
   `error: Cannot find module '../📃️pageSchema/🟦️.ts' from '🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts'`
   — a peer's in-flight `📃️pageSchema` → `📃️page` rename that took down every `bun 📜️script.ts` entry point
   repo-wide for ~15 minutes.
2. **Run 2 loaded the taxonomy and enumerated everything** (`inventory source-observation 85864/85864`) and then
   died in content capture:
   ```
   error: [clean taxonomy --kind mutation] admitted source disappeared before content capture:
     🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧪️tests/🧬️schema/🔣️.json.
     at mutationTaxonomySourceIndex (📜️script.ts:21046:30)
   ```
   A peer deleted that file between enumeration and capture. Nothing to do with this partition — the CLI has no
   tolerance for a live tree, and the tree was carrying 3–6 concurrent sessions throughout
   (`load averages: 79.74 76.89 88.41`).
3. **Run 3** (retry-loop attempt 1) exited 1 after its own long enumeration; its stderr was overwritten by the
   next attempt before I read it, so I cannot attribute the cause — recorded as `attempt=1 exit=1` in
   `parity-attempts.txt`.
4. **Run 4** (retry-loop attempt 2) got past both known failure modes — it loaded the taxonomy, enumerated
   `inventory source-observation 85871/85871`, captured content without a disappearing-source abort, and was still
   inside the policy-analysis phase ~50 minutes later, when this report was finalised (it had completed both
   `inventory source-observation 85871/85871` and `inventory source-index 71788/71788`, and kept a live CPU share
   throughout; `load averages: 85.36 75.27 80.25`, easing to `42.24 59.62 72.40`). The loop was left running into
   the session scratchpad
   (`…/scratchpad/parity-full.{json,err}`, attempt log `parity-attempts.txt`) — deliberately outside the ticket
   folder so peer agents do not sweep it. **I am not reporting a verdict I did not see.**

**Measured throughput of the phase that never finishes.** Run 4 completed
`inventory source-observation 85871/85871` and `inventory source-index 71788/71788`, then entered a third phase,
`inventory consumer-graph`, and advanced **6200 → 7400 of 71788 in ~20 minutes** — about 60 items/minute, i.e.
roughly **18 more hours** for that phase alone, on a box also carrying peer builds. At that point I stopped the
retry loop rather than leave a ~20%-CPU / ~1 GB-RSS process competing with other sessions for most of a day
(I also killed an earlier duplicate of my own whose output pipe had been closed by a `head -6`). This is a
measurement, not an estimate of my own patience: `consumer-graph` is the phase that makes
`clean taxonomy inventory --kind mutation` unusable as a per-change gate at this repo size.

Three facts worth carrying forward:

- WP0 §4 recorded this CLI failing repo-wide on an unrelated
  `generatorContracts["print-latex-tokens"/"report-actor-network"].previewTarget` taxonomy-validation error. That
  error did **not** recur in runs 2–4 — the taxonomy loaded and the inventory ran — so a peer appears to have
  fixed it since WP0. Reporting, not fixing, per the brief.
- `inventory consumer-graph` is O(all 71788 admitted sources) and is the dominant cost; WP2's `schema check`
  should either scope it to the roots being checked or cache it, otherwise the new command inherits the same
  18-hour worst case.
- The failure mode in run 2 (`admitted source disappeared before content capture`) makes this gate effectively
  unrunnable on a busy shared tree. Whoever owns WP2's `schema check` command should make the capture tolerate a
  file vanishing mid-scan, or the gate will keep being un-runnable exactly when it matters.

What the gate checks, and why the changes are built to satisfy it, is spelled out inline in §2.2 and §2.1
(`rootHasIdentity` via `x-semio-mutationKinds`, `leafHasIdentity` via `title`/`$id`,
`policyMutationPayloadSchemaProblems`'s local-path + draft-07 requirements). §4.1 reimplements the same predicates
over the same files and is green for the whole partition.

### 4.4 Plugin-owned ajv oracle — `📕️norm` `config-mutation-source`

```
$ bun ./📜️script.ts config-mutation-source            # cwd ✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust
Norm config schema oracle passed: 5 cases, 5 hostile payloads, 4 undeclared wire forms, 13 text vectors, 25 binary vectors
```

This is the independent ajv harness described in §2.4, running against the relocated leaf schema and the rewritten
`$ref`-union aggregate. Two further changes were needed inside it, both consequences of the new aggregate shape and
both recorded here because they generalise to **every** ajv consumer of a mutation aggregate:

- `ajv.addVocabulary(["x-semio-mutationKinds"])` — the annotation keyword §2.2 relies on is rejected by ajv
  `strict: true`. Any strict-mode consumer of an aggregate must declare it (my own validator uses `strict: false`).
- The aggregate's relative `$ref` is resolved by ajv **against the aggregate's `$id`**, not against the file path,
  so `ajv.addSchema(leaf, "./<relative path>")` never matches. The oracle now registers the leaf under its own
  `$id` (`ajv.addSchema(schema)`) and compiles `{ ...aggregate, oneOf: [{ $ref: schema.$id }] }`, keeping the
  literal relative-path assertion as a separate check. `wp4-mutation-validate.mjs` does the equivalent with
  synthetic `urn:wp4:leaf:<n>` keys. **This is the one real ergonomic cost of relative-path unions**; if WP7 would
  rather have aggregates `$ref` leaves by `$id` URL, every leaf now has a unique one (§2.1) and the switch is a
  one-line change in `wp4-mutation-aggregates.py`.

## 5. Deliberately not changed: `🧰️framework` / `💻️os` mutation trees

96 of the 101 framework/os leaves were **already** at the taxonomy default `<leaf>/🧬️schema/🔣️.json`, and none of
their aggregates inlines a payload `$defs` — so WP4's two stated goals (leaf relocation, G-A→G-B) had nothing to do
there. The 5 that were flat (`💻️os/🎚️config/🧬️schema/🧬️mutations/{📌️set-default-app, 🚪️sign-out,
🛡️change-merge-policy, 🧹clear-default-app, 🪪️sign-in}`) were relocated and stamped like the plugin leaves.

The remaining 340 validator findings are all **dialect/`$id`/`title`** on os plugin-host conformance fixtures
(`🕸️dag`, `🌊️flow/🌿️vcs`, `🏪️store/🧫️fixtures/*`, `📡️spr/🧪️tests/*`, `🔌️plugin/🧪️tests/*`, `🔁️workflow`).
They were left alone on purpose:

- Those documents are 2020-12 and their aggregates lean on `allOf` + `unevaluatedProperties` (and one
  `{"not": {}}` "uninhabited roster") to express **negative** conformance expectations. Draft-07 has no
  `unevaluatedProperties`, so mechanically dropping it loosens exactly the fixtures whose job is to reject.
  Migrating them is a semantic rewrite of the plugin-host declaration-channel tests, not a rename.
- Six of them (`🔌️plugin/🧪️tests/🛰️declaration-channels/*`, `📢️publication-fixtures/*`,
  `⚛️reactor/…/🧬️job-test-mutations`) are single-mutation "direct mutations" objects by design, not unions, so the
  G-B shape does not apply to them at all.

This is the WP4-os slice the contract's §D notes already anticipate ("see WP4-os"). Exact file list:
`bun <ticket>/wp4-mutation-validate.mjs` prints it (340 lines under the `framework/os … deferred` heading).

## 6. Cross-partition requests

1. **`📚️library` (tooling worker)** —
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` hard-codes the old
   convention in three live assertions and must be updated, otherwise it fails against every leaf in this partition:
   - `expect(descriptor.payloadSchema, row.directory).toBe("🧬️.schema.json")` → `"🧬️schema/🔣️.json"`
   - `expect(declared.every((mutation: any) => mutation.payloadSchema === "🧬️.schema.json")).toBe(true)` → same
   - the `mutationPayloadSchemaProblems(contract.owner, "🧬️.schema.json", …)` case and its
     `const target = \`${contract.owner}/🧬️.schema.json\`` → `"🧬️schema/🔣️.json"` (note the fake resolver must then
     answer `{kind: "directory"}` for `${contract.owner}/🧬️schema`).
2. **`📚️library` (tooling worker)** — `newMutationDescriptor` (`📜️script.ts:22422`) already scaffolds
   `mutationPayloadSchemaRelativePath()` = `🧬️schema/🔣️.json`, which now matches every existing leaf; **the
   schema-parity check must compare structure, not the filename**, because the whole partition is on one name now
   and a filename comparison would silently pass. Also `📜️script.ts:22575`'s stub should stamp `$id` + `title`
   (title = `aggregateVariant`) so newly scaffolded leaves satisfy §2.1 without a manual edit.
3. **`🧪️test` module (repo product, not `📚️library`)** —
   `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` `payloadSchemaCommand`/`derivePayloadSchemas`
   still writes and probes `join(leafAbs, "🔣️.schema.json")` and refuses with "the leaf carries neither
   `🧬️schema/<contribution>` nor `🔣️.schema.json`". That is now the only generator in the repo that would
   re-introduce a `*.schema.json` file. It should target `🧬️schema/🔣️.json`. (WP0 §6 Family C flagged this as the
   "third generator with a fourth filename opinion".)
4. **Whoever owns `mutation/direct-owner` (WP0 §6 Family H)** — `🖍️draw`'s three `🦠️mutation/🦀️.rs`-only leaves
   still have no leaf-root `🦀️.rs`; I fixed their descriptors' `payloadSchema` (they now name a real JSON schema
   instead of an unresolvable `🦠️mutation/🦀️.rs#Type`), but the structural breach itself is untouched.
5. **`✏️editor` surface owners** — 22 `✏️editor/{🎚️config,👥️presence,🖌️session}/🦀️.rs` files declare
   `protocol::MutationLeafDescriptor { … payload_schema: "🔣️.schema.json" … }` in Rust for owners such as
   `…/✏️editor/👥️presence/🚫️presence-noop`. Their own docstrings say "the `owner` path is registry metadata only
   and names no `🧬️mutations` leaf directory", so these strings resolve to nothing on disk and are outside this
   partition. They are the last `🔣️.schema.json` literals in `✏️s/`; someone should either point them at a real
   schema or drop the field's file semantics.

## 7. Open questions

1. **63 mutation roots have no aggregate `🔣️.json` at all** (`🚪️io/🧬️mutations`, per-subset roots in `📕️norm`,
   `🗒️note`, `🖍️draw`, `🏗️fem`, `🎬️sequence`, `➗️mathematical`, `📐️cad/🧩️extensions`, `📖️playbook/🧩️extensions`,
   framework/os fixtures). `policyMutationStructuralBreachesView` treats a missing root surface as a
   `mutation/schema-parity` breach for every leaf that declares `json-schema`, so those roots are red today — but
   creating the file also flips `rootExists` to true, which then breaches every leaf in the same root that does
   *not* declare `json-schema`. I did not create them: the fix is coupled to a `requiredLanguageSurfaces` decision
   per root, which is a WP2/WP7 call, not a WP4 one.
2. **`x-semio-mutationKinds` is load-bearing.** It is the only thing carrying `semanticKind` text into an aggregate
   whose leaf directory names are nested (architect `ℹ️information-requirement/🌱️create`). If WP2 replaces the
   textual identity check with a structural one, the key can go; until then it must not be dropped.
3. **Aggregate `$id`s are still inconsistent** (`…/writer/1/any/mutation.json`, `…/s/equation/equation/mutation.json`,
   `…/gis/gis3d/mutations`, `…/s.lowpoly.lowpoly/mutations.json`, `https://semio.dev/schema/os/flow-vcs/mutations`).
   I preserved each existing `$id` rather than invent a new scheme, because contract §A says scope ids come from the
   module `$id` and the module-level normalization is another worker's. Leaf `$id`s are derived from the path (§2.1)
   and are therefore *not* always a prefix-extension of their aggregate's `$id` — worth reconciling in WP7.
4. **`🌍️gis`'s four aggregates lost their `allOf` + local `operation` const discriminator** when they became plain
   `$ref` unions (WP0 §3 sample 8 described that shape). The discriminator now lives only in the leaf/`🦀️.rs`
   surfaces. If the `operation`/`mutation` tag must be re-expressible in JSON Schema, the right place is the leaf's
   `$defs.Wire` (the vcs/sequence pattern from §2.1), not the aggregate.
