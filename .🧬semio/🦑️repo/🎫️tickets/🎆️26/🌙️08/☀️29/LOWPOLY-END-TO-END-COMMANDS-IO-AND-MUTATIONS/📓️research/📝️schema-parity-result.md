# Lowpoly schema parity — mutations, snapshot, diff (non-Rust representations)

Scope owned for this pass: every non-Rust `component.{json,proto,graphql,ts}` leaf under
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/**`
(`$A` below). Rust is read-only source of truth; `✏️editor/`, `🚪️io/` and the TypeScript package
are other agents' ownership and were not touched.

## 0. Divergence from the briefing report

The briefing referenced
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/📓️research/📝️schema-and-mutations.md`
— **this file does not exist** in the ticket's `📓️research/` folder (confirmed via `find`; the folder
holds nine other `.md` reports — `📝️typescript-implementation.md`, `📝️typescript-and-descriptors.md`,
`📝️canonical-shape-diff.md`, `📝️editor-commands.md`, `📝️batch-vs-interactive-dispatch.md`,
`📝️build-and-verify.md`, `📝️interactive-job-migration-recipe.md`, `📝️engine.md`,
`📝️tests-and-oracle.md` — none of which cover this schema/mutations ground). I proceeded from the
briefing's own gap list and verified everything directly against on-disk files instead.

Verified divergences from the briefing's claims:

1. **"TypeScript mutations component is a 61-byte stub"** — confirmed true (`export {};`).
2. **"protobuf and graphql schemas define only diff/artifact shapes, no mutation union"** — **wrong
   in a worse way**: `🧬️mutations/🛰️component.proto` and `🧬️mutations/🔗️component.graphql` (and
   `🧬️mutations/🔣️component.json`) were **not empty/missing** — they contained a **verbatim copy of
   the ARTIFACT-lane schema** (`message LowpolyMutation { string schema = 1; repeated LowpolyObject
   objects = 2; }` etc.), i.e. actively wrong content mislabeled as the mutation dispatch type, not a
   gap. This is a more serious defect than "missing."
3. **"Snapshot and Diff types are present in json/rust/proto/graphql but MISSING from
   TypeScript"** — **wrong**: `📸️snapshot/🟦️component.ts` and `🔺️diff/🟦️component.ts` both already
   exist, are non-trivial, and are reasonably complete. No TypeScript work was needed there.
4. **"18 domain types"** — confirmed accurate, but they are **not** all in the ~36KB
   `$A/🧬️schema/🦀️component.rs` file as implied. That file defines exactly one struct
   (`LowpolyArtifact`, plus impls/tests/helpers — hence the 36KB). The 18 types are spread across
   three Rust files: 7 in the root `🦀️component.rs` (`LowpolyArtifact`, `LowpolyTransform`,
   `LowpolyPaintLayer`, `LowpolyObject`, `LowpolyObjectPatch`, `LowpolySelectionTargets`,
   `LowpolySelection`), 1 in `📸️snapshot/🦀️component.rs` (`LowpolySnapshot`), and 10 in
   `🔺️diff/🦀️component.rs` (`LowpolyDiff`, `LowpolyStringList`, `LowpolyObjectsDelta`,
   `LowpolyObjectPatchEntry`, `LowpolyPaintLayersDelta`, `LowpolyIndexedPaintLayer`,
   `LowpolyIndexedPaintLayerPatch`, `LowpolyPaintStrokeAt`, `PixelRun`, `LowpolyPaintLayerPatch`).
   7+1+10 = 18. `LowpolyObject`/`LowpolyPaintLayer`/etc. themselves live one directory level up, in
   `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs` (outside `$A`, out of scope).
5. **JSON schema's per-mutation `oneOf`** — confirmed genuinely absent (the gap as briefed); the 17
   per-mutation `🔣️payload.schema.json` leaves already existed and are correct enough to reference.

## 1. Generated vs. handwritten — determination: **handwritten**

Evidence:
- `$A/🧬️schema/🦀️component.rs`'s `lowpoly_artifact_schema_descriptor()` embeds all four non-Rust
  leaves (artifact/snapshot/diff/mutations × json/ts/proto/graphql) via `include_str!(...)` — Rust
  *reads* these files at compile time; it does not write them.
- `@semio-tech/framework-schema:generate` (`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`)
  is a **different** "schema" — an entity-kind icon/emoji catalog for the CLI/VS Code, unrelated to
  artifact component schemas. Read in full; it never touches `component.{json,ts,proto,graphql}`.
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📜️script.ts` is a proc-macro crate's
  `cargo test` runner only (`test` target, no `generate`).
- The lowpoly plugin's own `📋️project.json` targets are `test`/`test-quick`/`test-long`/
  `test-exhaustive`/`describe` — `describe` runs `describePluginComponent` (an inspection/printout
  helper), not a file writer.
- Repo-wide `rg` for `writeFileSync` combined with `component.(json|ts|proto|graphql)` found only
  test files (`🧪️index.test.ts` × 2, a `transaction-v2` test) — no generator.
- The sibling `stdio` `✳️object` plugin's analogous mutations leaves show the same hand-authored,
  slightly-inconsistent-with-each-other style (its own JSON schema is a loose `{mutation, payload}`
  shape that doesn't even reference its own per-mutation payload schemas) — consistent with
  independent handwriting per plugin, not a shared generator.

Conclusion: hand-edited the four owned leaf files directly; no generator input to redirect to.

## 2. What actually changed

Only `$A/🧬️schema/🧬️mutations/{🟦️component.ts, 🛰️component.proto, 🔗️component.graphql,
🔣️component.json}` were rewritten. Nothing under `$A/🧬️schema/📸️snapshot/`, `$A/🧬️schema/🔺️diff/`,
or the top-level `$A/🧬️schema/{json,ts,proto,graphql}` was edited (see §4 for why, and what's left).

### Wire-format ground truth (not assumed — read off committed fixtures)

Before writing anything I read all 17 `🧪️tests/…/🦠️mutation/🔣️component.json` fixtures (one per
mutation family, all pre-existing, presumably CI-exercised). They prove two things conclusively:

1. **`LowpolyMutation` serializes serde's default externally-tagged shape**: `{"MoveObject": {...}}`,
   **not** `{"mutation": "moveObject", "payload": {...}}` (the shape the sibling `stdio`/`✳️object`
   plugin's TS/JSON happen to use — that plugin's own mutations component.json therefore does *not*
   match its own fixtures either, but that's out of scope here). I built `LowpolyMutation`'s TS union,
   the JSON `oneOf`, and the GraphQL output union to match the *actual* tag exactly (PascalCase Rust
   variant name as the sole object key).
2. **`LowpolyObject.mesh` is `null | { childId, target: ArtifactRef }`** on the wire (fixture:
   `🌱️create-object/…/🦠️mutation/🔣️component.json` has `"mesh": null`;
   `🕸️create-mesh/…/🔺️diff/🔣️component.json` has `"mesh": {"childId": "...", "target": {"artifactId":
   ..., "dialect": {...}}}`) — never `meshJson: string`. I used this proven shape for the
   `LowpolyObject`/`ArtifactRef`/`ArtifactDialect`/new `LowpolyMeshHandle` types I authored inside the
   mutations files (§4 explains why the *existing*, pre-committed artifact/snapshot files were left
   alone despite sharing the same stale field).

### Files rewritten (summary)

- **`🟦️component.ts`**: real `LowpolyMutation` discriminated union (17 members, externally tagged),
  plus `LowpolyTransform`, `LowpolyPaintLayer`, `LowpolyObject`, `LowpolyMeshHandle`, `ArtifactRef`,
  `ArtifactDialect`, `PixelRun`, and a `LOWPOLY_MUTATION_TAGS` const tuple mirroring Rust's `KINDS`
  intent (PascalCase tags, since the wire tag is PascalCase not kebab-case).
- **`🛰️component.proto`**: one `message` per mutation payload (`CreateObject` … `EditPaintLayer`) plus
  shared `LowpolyTransform`/`LowpolyPaintLayer`/`LowpolyObject`/`LowpolyMeshHandle`/`ArtifactRef`/
  `ArtifactDialect`/`PixelRun`, dispatched through a `LowpolyMutation { oneof mutation { ... } }`
  envelope (field numbers 1–17, declaration order = Rust enum order).
- **`🔗️component.graphql`**: per-payload `type`+`input` pairs, a `LowpolyMutationKind` enum, an
  `LowpolyMutationInput` (`kind` discriminant + 17 optional payload fields — GraphQL has no native
  input-side discriminated union), and an output-side `union LowpolyMutation = CreateObjectMutation |
  …` of 17 wrapper types.
- **`🔣️component.json`**: a `oneOf` of 17 branches, each `{"type":"object","required":["<Tag>"],
  "properties":{"<Tag>":{"$ref":"<dir>/🔣️payload.schema.json"}}}` — references, never duplicates, the
  pre-existing per-mutation payload schemas.

## 3. Parity matrix

### 17 mutations × 5 representations (after this pass)

| # | kebab kind | Rust enum | TS | proto | graphql | JSON oneOf |
|---|---|---|---|---|---|---|
|1|create-object|✅|✅|✅|✅|✅|
|2|delete-object|✅|✅|✅|✅|✅|
|3|reorder-objects|✅|✅|✅|✅|✅|
|4|rename-object|✅|✅|✅|✅|✅|
|5|change-object-smooth-shading|✅|✅|✅|✅|✅|
|6|move-object|✅|✅|✅|✅|✅|
|7|rotate-object|✅|✅|✅|✅|✅|
|8|scale-object|✅|✅|✅|✅|✅|
|9|create-mesh|✅|✅|✅|✅|✅|
|10|delete-mesh|✅|✅|✅|✅|✅|
|11|insert-paint-layer|✅|✅|✅|✅|✅|
|12|remove-paint-layer|✅|✅|✅|✅|✅|
|13|rename-paint-layer|✅|✅|✅|✅|✅|
|14|change-paint-layer-visible|✅|✅|✅|✅|✅|
|15|change-paint-layer-opacity|✅|✅|✅|✅|✅|
|16|change-paint-layer-blend-mode|✅|✅|✅|✅|✅|
|17|edit-paint-layer|✅|✅|✅|✅|✅|

Before this pass: TS = stub (0/17), proto/graphql = wrong content (0/17 correct despite files
existing), JSON = no union at all (0/17). Rust was always 17/17 (read-only source).

### 18 domain types × 5 representations

No new domain-type work was done (all 18 already existed non-Rust-side, artifact/snapshot fully,
diff shallowly — see §4). Table reflects **current** state, unchanged by this pass:

| type | rust | ts | json | proto | graphql | note |
|---|---|---|---|---|---|---|
|LowpolyArtifact|✅|✅|✅|✅|✅||
|LowpolyTransform|✅|✅|✅|✅|✅||
|LowpolyPaintLayer|✅|✅|✅|✅|✅||
|LowpolyObject|✅|✅ (stale `meshJson`)|✅ (stale)|✅ (stale)|✅ (stale)|see §4.1|
|LowpolyObjectPatch|✅|✅ (stale, misplaced field)|shallow only|shallow only|shallow only|see §4.2|
|LowpolySelectionTargets|✅|✅|✅|✅|✅||
|LowpolySelection|✅|✅|✅|✅|✅||
|LowpolySnapshot|✅|✅|✅|✅|✅||
|LowpolyDiff|✅|✅|✅ (shallow nested)|✅ (shallow)|✅ (shallow)|see §4.2|
|LowpolyStringList|✅|✅|✅|✅|✅||
|LowpolyObjectsDelta|✅|✅|shallow (`added`/`patched` items untyped)|shallow|shallow||
|LowpolyObjectPatchEntry|✅|⚠️ missing `paintLayers` sibling field|—|—|—|see §4.2|
|LowpolyPaintLayersDelta|✅|✅|—|—|—||
|LowpolyIndexedPaintLayer|✅|✅|—|—|—||
|LowpolyIndexedPaintLayerPatch|✅|✅|—|—|—||
|LowpolyPaintStrokeAt|✅|✅|—|—|—||
|PixelRun (diff)|✅|✅|—|—|—||
|LowpolyPaintLayerPatch|✅|✅|—|—|—||

## 4. Handoff items (found, not fixed — outside this pass's bounded scope)

All confirmed against Rust source and/or committed test fixtures; none touched, because fixing them
correctly reaches into files this pass didn't own the blast radius for (concurrent I/O ticket work)
or requires design decisions (JSON null-vs-absent semantics for ~30 diff fields) better made in a
dedicated ticket.

### 4.1 `LowpolyObject.mesh` vs. stale `meshJson: string` (HIGH — spans artifact + snapshot + 3 payload schemas)
`LowpolyObject.mesh` is `Option<store::ArtifactChild<SemioMeshSnapshot>>` in Rust (confirmed at
`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs:103-116`, explicit docstring: "replaces
the old opaque `mesh_json: String`"). Confirmed on the wire via
`🧬️mutations/🌱️create-object/🧪️tests/inserts-obj-mast-between-hull-and-fin/🦠️mutation/🔣️component.json`
(`"mesh": null`) and `🧬️mutations/🕸️create-mesh/…/🔺️diff/🔣️component.json` (`"mesh": {"childId":…,
"target":{...}}`). Yet **every** non-Rust `LowpolyObject` definition still owned by this facet uses
`meshJson: string` (required): `$A/🧬️schema/{🟦️component.ts,🔣️component.json,🛰️component.proto,
🔗️component.graphql}` and the same four under `📸️snapshot/`. Also: the `🌱️create-object`
`🔣️payload.schema.json` (not owned by this facet — sibling leaf, not one of the four owned
filenames) omits the `mesh` field from `LowpolyObject` **entirely** (neither `mesh` nor `meshJson`),
confirmed by an `ajv` run that rejects the real fixture with `must NOT have additional properties`
at `CreateObject/object` (see §5). Recommendation: a dedicated pass renaming `meshJson` →
`mesh: LowpolyMeshHandle | null` across artifact + snapshot (both facets, all 4 langs) plus fixing
the 3 payload schemas, done together so no representation goes stale relative to the others.

### 4.2 `LowpolyDiff`/`LowpolyObjectPatch` gaps (MEDIUM)
- **Structural bug in `📸️snapshot/../🔺️diff/🟦️component.ts`** (pre-existing, not introduced by this
  pass): `LowpolyObjectPatch` incorrectly nests a `paintLayers?: LowpolyPaintLayersDelta` field that
  Rust does not have there — the real Rust `LowpolyObjectPatch` is `{name, smoothShading, transform,
  mesh}` only. The real sibling field lives one level up, on `LowpolyObjectPatchEntry.paintLayers`
  (confirmed at `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️component.rs:189-200` and
  `$A/🧬️schema/🔺️diff/🦀️component.rs:110-118`, and on the wire in every paint-layer mutation's
  `🔺️diff/🔣️component.json` fixture, e.g. `🔖️rename-paint-layer/…`, which shows `"paintLayers": {...}`
  as a sibling of `"patch"`, not inside it).
- **`🔺️diff/🔣️component.json` is far shallower than its TS sibling**: `LowpolyObjectsDelta.added`/
  `patched` are typed as bare `{"type":"object"}` instead of `LowpolyObject[]` /
  `LowpolyObjectPatchEntry[]`; none of `LowpolyObjectPatchEntry`, `LowpolyPaintLayersDelta`,
  `LowpolyIndexedPaintLayer(Patch)`, `LowpolyPaintStrokeAt`, `PixelRun`, `LowpolyPaintLayerPatch`
  exist as `$defs`. Same shallowness in `🔺️diff/🛰️component.proto`/`🔗️component.graphql` (not
  individually re-verified line-by-line, but the JSON gap and the TS/proto sibling sizes strongly
  suggest the same). A real diff-apply/undo pipeline validated against this JSON schema would
  currently accept malformed `added`/`patched` payloads silently.
- Additionally, `🔺️diff/🔣️component.json`'s top-level fields (`schema`, `paintUtility`,
  `activePaintLayer`, …) are typed without `"type":"null"` alternatives even though **every**
  `LowpolyDiff` field is `Option<T>` and serializes as an explicit `null` when absent (confirmed on
  every diff fixture — e.g. `"schema": null` in `🔖️rename-paint-layer/…/🔺️diff/🔣️component.json`).
  Only the double-`Option` fields (`activeObjectId`, `hoveredObjectId`, etc.) already allow `null`;
  ordinary presence/config/artifact scalars like `schema`/`paintUtility` do not, so a real diff
  payload as actually emitted would fail strict validation against this schema today.

### 4.3 Per-mutation `🔣️payload.schema.json` byte-field bug (MEDIUM — not owned by this facet)
`pixels`/`bytes` fields (Rust `Vec<u8>` with `#[serde(with = "…_base64")]`) are typed
`{"type":"array","items":{"type":"integer"}}` in three payload schemas —
`🌱️create-object/🔣️payload.schema.json` (`object.paintLayers[].pixels`),
`➕️insert-paint-layer/🔣️payload.schema.json` (`layer.pixels`), and
`🎨️edit-paint-layer/🔣️payload.schema.json` (`runs[].bytes`) — but the real wire value is a base64
**string** (confirmed by fixtures, e.g. `"pixels": "AAAAAAAAAAA="`,
`"bytes": "/wAA/w=="`). This is exactly why 3 of the 17 real fixtures fail validation against the new
`oneOf` union in §5 below — the union and its `$ref` wiring are correct; the referenced files have a
pre-existing, independently-confirmed bug, and are outside this facet's four owned filenames.

## 5. Verification (every command run, with real output)

### 5.1 JSON Schema — ajv, structural compile + real-fixture validation
Script: `verify_mutations_schema.mjs` (ad hoc, not committed) — loads
`🧬️mutations/🔣️component.json`, dereferences each `oneOf` branch's `$ref` against its sibling
`🔣️payload.schema.json` (each of which is independently `ajv.compile()`d standalone first), then
validates all 17 real committed `🧪️tests/…/🦠️mutation/🔣️component.json` fixtures, plus one hostile
fixture with two variant keys at once.

```
[OK] mutations component.json, with every $ref dereferenced against its sibling payload.schema.json,
     compiles; oneOf has 17 branches

Found 17 real committed mutation fixtures.

[PASS] ↗️move-object/…            [PASS] ➖️remove-paint-layer/…      [PASS] 🌫️change-paint-layer-opacity/…
[FAIL] ➕️insert-paint-layer/…     [PASS] 🎛️change-paint-layer-blend-mode/…
[FAIL] 🌱️create-object/…          [PASS] 🏷️rename-object/…           [PASS] 👁️change-paint-layer-visible/…
[FAIL] 🎨️edit-paint-layer/…       [PASS] 💀️delete-object/…           [PASS] 📐️scale-object/…
[PASS] 🔀️reorder-objects/…        [PASS] 🔄️rotate-object/…           [PASS] 🔖️rename-paint-layer/…
[PASS] 🔘️change-object-smooth-shading/…  [PASS] 🕸️create-mesh/…       [PASS] 🧨delete-mesh/…

14 passed, 3 failed out of 17.
Hostile fixture (two mutation keys at once) rejected: true
```

The 3 failures are **exactly** the pre-existing `🔣️payload.schema.json` bugs in §4.3 (`array` vs.
base64 `string` for `pixels`/`bytes`, and `create-object`'s missing `mesh` key) — i.e. the new
`component.json`'s dispatch/discrimination logic is proven correct (14/17 real fixtures pass end to
end through referenced, unmodified sibling schemas; the hostile double-tagged fixture is correctly
rejected by `oneOf`'s exclusivity), and the 3 failures are attributable, with cited evidence, to
files this facet does not own.

### 5.2 Protobuf — no protoc/buf toolchain in this repo (stated plainly, not substituted)
Checked: `which protoc buf` → both not found. `Cargo.toml` repo-wide has no `prost-build` or
`tonic-build` dependency. No Python `protobuf` package installed. **This repo has no protobuf
compiler toolchain at all** — I cannot claim `protoc` compilation succeeded. Best-effort structural
check instead (brace balance, duplicate message names, every field type resolvable to an in-file
message or a proto3 scalar, `syntax`/`package` declared):

```
[proto] brace balance: OK
[proto] 25 messages declared: LowpolyTransform, LowpolyPaintLayer, ArtifactDialect, ArtifactRef,
        LowpolyMeshHandle, LowpolyObject, PixelRun, CreateObject, DeleteObject, ReorderObjects,
        RenameObject, ChangeObjectSmoothShading, MoveObject, RotateObject, ScaleObject, CreateMesh,
        DeleteMesh, InsertPaintLayer, RemovePaintLayer, RenamePaintLayer, ChangePaintLayerVisible,
        ChangePaintLayerOpacity, ChangePaintLayerBlendMode, EditPaintLayer, LowpolyMutation
[proto] duplicate message names: none
[proto] referenced field types not resolvable in-file or as a proto3 scalar: none
[proto] declares syntax = "proto3": yes
[proto] declares package: package semio.s.lowpoly.lowpoly.mutation;
```

### 5.3 GraphQL — no `graphql`/graphql-js package in this repo (stated plainly)
Checked: no `graphql` entry anywhere in the root `package.json`; `node_modules/graphql` absent.
**No GraphQL SDL parser is available in this repo** — did not add one (would be a new runtime/dev
dependency). Best-effort structural check instead (brace balance, no duplicate type names, every
field-position type reference resolves to an in-file definition or a GraphQL builtin scalar, every
`union LowpolyMutation` member resolves to a declared `type`):

```
[graphql] brace balance: OK
[graphql] 68 type-system definitions declared, 0 duplicates
[graphql] referenced types not resolvable in-file or as a GraphQL builtin scalar: none
          (two regex false-positives from doc-comment text — "dsl::Mutations", "ArtifactChild<...>"
          inside `#`/string comments — manually confirmed not real field references)
[graphql] union LowpolyMutation: 17 members, unresolved: none
```

### 5.4 TypeScript — `tsc`, scoped (real, proven) + full-repo (attempted)
The repo's root `tsconfig.json` has `"include": ["**/*.ts", ...]` — a whole-monorepo check. I wrote a
throwaway consumer,
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/verify-mutations-ts.ts`,
that imports the real `🧬️mutations/🟦️component.ts`, exhaustively switches over all 17 tags (with a
`never`-exhaustiveness check), and asserts a fabricated 18th tag is rejected
(`@ts-expect-error`). Ran with the repo's own compiler options scoped to just this file:

```
$ bunx tsc --noEmit --strict --target ESNext --module ESNext --moduleResolution bundler \
    --skipLibCheck --isolatedModules --lib ESNext,DOM verify-mutations-ts.ts
(no output, exit 0)
```

Proved this is a *real* type-check, not a no-op: temporarily typo'd `newPosition` →
`newPositionTYPO` in the same file and re-ran —

```
verify-mutations-ts.ts(49,62): error TS2561: Object literal may only specify known properties, but
'newPositionTYPO' does not exist in type '{ id: string; newPosition: [number, number, number]; }'.
Did you mean to write 'newPosition'?
```

— then reverted (confirmed 0 occurrences of the typo remained). This proves the new
`LowpolyMutation` union's field names are load-bearing and checked, not merely present-but-unused.

I also launched the full-repo `bunx tsc --noEmit -p tsconfig.json` in the background. It did not
finish within this task's working window (the monorepo's `**/*.ts` include is large enough — and at
least one other concurrent `tsc --noEmit` was already running system-wide from unrelated work — that
a full pass did not complete). I'm reporting this plainly rather than claiming a full-repo pass
succeeded: the scoped check above is the real, completed proof for the files this pass changed.

### 5.5 Cross-language consistency — five-way (six-way) mutation-name diff
Script: `five_way_diff.py` — extracts mutation names from Rust's `KINDS` const (kebab→PascalCase) and
enum declaration order, the new TS union's tags, the new JSON `oneOf`'s branch keys, the new proto
`oneof`'s message types, and the new GraphQL union's members (stripped of the `Mutation` suffix):

```
Counts: {'rust (KINDS)': 17, 'rust (enum order)': 17, 'typescript': 17, 'json-schema': 17,
         'protobuf': 17, 'graphql': 17}

rust (KINDS, kebab->pascal)        MATCH (order+set)
rust (enum variant order)          MATCH (order+set)
typescript (union tags)            MATCH (order+set)
json-schema (oneOf keys)           MATCH (order+set)
protobuf (oneof message types)     MATCH (order+set)
graphql (union members)            MATCH (order+set)

ALL FIVE AGREE ON THE SET OF 17 NAMES: True
```

All six extractions (Rust counted twice, by two independent methods) agree on both the **set** and
the **exact declaration order** of all 17 mutation names.

## 6. Files touched

- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts` (rewritten: stub → real union)
- `.../🧬️mutations/🛰️component.proto` (rewritten: wrong artifact-copy content → real oneof dispatch)
- `.../🧬️mutations/🔗️component.graphql` (rewritten: wrong artifact-copy content → real union/input)
- `.../🧬️mutations/🔣️component.json` (rewritten: wrong artifact-copy content → discriminated `oneOf` referencing existing payload schemas)
- This report and one kept verification script:
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS/🗑️generated/verify-mutations-ts.ts`

Nothing else was modified. No Rust file, no `✏️editor/`, `🚪️io/`, or TypeScript-package file was
touched.
