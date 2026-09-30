# 📓️ W2-S-C — Payload parity for the design apps

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, parity executor W2-S-C, 2026-09-30.

- Contracts: `📋️design.md` §6, `🧭️plan.md` ("W2-R brief", "W2-S parity brief"), the coordinator notes of this run (colour widget, 3D
  grid snaps, draft-07 dialect).
- Scope: `🎥️shooting`, puzzle `🧊️3d`/`🖐️5d`, `🌀️procedural`, `📐️cad`, `🖨️raster`, `🗒️note`, `📋️forms`, `🧱️block` and `🖍️draw`.
  Puzzle 2d was not touched.

## 1. Outcome

**DONE.** Every finding of the lint as briefed is fixed, and so is every Rust/schema drift a new scanner found beyond the committed
fixtures.

| Check | Before | After |
|---|---|---|
| `schema mutation-payloads`, classes invalid / aggregate / opaque / layout / undescribed / unmapped / unresolved | **279** | **0** |
| `schema mutation-inputs`, strict, every scope path | 1 (`leafUncatalogued`, draw) | **0** |
| `🧪️w2-s-design-rust-parity.py`: Rust wire form ↔ leaf schema, nested records and unit enums included | 176 | **0** |
| W2-R schema checker: committed fixture payloads against their leaf schemas | 104 failures | **0** |
| W2-R schema checker: `x-semio-ui` against `InputUi` | 1,133 annotations valid | **1,487 valid, 0 invalid** |
| Ajv cross-check `🧪️w2-s-design-check.ts` | – | 367 leaves, 24 aggregates, 325 fixture wires: **0 failures** |

Nine findings of a newer lint remain. W2-S added its `unwitnessed` class and its feature-row reading after this brief was
written, and none of the nine is a schema drift (§6):

- one lint false positive in shooting;
- five cad object kinds that have no wire fixture by design, plus one cad extension leaf;
- two raster leaves whose tests carry their own JSON vectors instead of a fixture bundle.

The before counts are per scope from `🗑️generated/w2s-c/start-*.json`:

| plugin | invalid | aggregate | opaque |
|---|---|---|---|
| shooting | 28 | 31 | |
| 3d | 5 | 35 | |
| 5d | 9 | 49 | |
| procedural | 9 | 28 | |
| cad | 6 | 19 | |
| raster | 3 | 15 | |
| note | 1 | 34 | |
| block | 2 | 2 | |
| forms | | | 3 |
| draw | | | |

## 2. What changed

### 2.1 Aggregate rule

I applied W2-S's `🧪️w2-s-aggregate-rule.py --apply` to the scope prefixes: **228 files written**, 108 already conformed.

- Every internally tagged leaf now declares `mutation: {const: <wire>}` first in `properties` and `required`.
- Every aggregate is the `oneOf` of its leaf `$ref`s in Rust variant order.
- The 11 "manual" rows are editor config and presence aggregates that have no aggregate document, so there is nothing to check
  (block, procedural, note, forms, shooting).

### 2.2 Rust was the outlier, so Rust was fixed (camelCase wire, all at once)

**Shooting (31 leaves).** Every `ShootingMutation` payload struct gains `#[value(rename_all = "camelCase")]`, plus the `cfg(test)`
serde twin. The committed renames are:

- `assetIds`, `assetId` and `shotId`;
- `newName`, `newUrl`, `newLabel`, `newWidth`, `newHeight`, `newFormat`, `newShape`, `newCamera`, `newEnabled`, `newAzimuth`,
  `newElevation`, `newIntensity` and `newRoughness`;
- `toIndex` and `savedCamera`.

The same renames went into, all in the same change:

- 26 committed `🦠️mutation` fixtures;
- 15 leaf-test JSON literals;
- the TS twin `🧬️mutations/🟦️.ts`;
- the `🎥️mutate-shooting-1` Examples table and Python reference.

The feature prose that said the payloads are snake_case was rewritten.

**Procedural.**

- All 14 `Generation2dMutation` leaves and both `SetGenerationPreview` leaves (2d and 3d) gain `#[value(rename_all = "camelCase")]`.
  This fixes `question_id` → `questionId` and `preview_text` → `previewText`.
- 3 fixtures were updated.
- The `🌀️mutate-procedural-2d-1` `ARGUMENTS` map now reads `questionId`.
- The 2d/3d case prose no longer lists the snake_case argument as a divergence.

**Forms.** All 12 leaves gain `#[value(rename_all = "camelCase")]`, giving `stepId`, `blockId`, `toStepId`, `toIndex`, `newTitle`
and `newDescription`. The same renames went into:

- 8 leaf schemas (the `x-semio-ui` annotations moved with them);
- 8 fixtures;
- 9 TS twins, whose docstrings no longer say snake_case;
- the `🌵️mutate-forms-1` Python reference;
- four test doc comments.

The forms text/binary codec (`🚪️io/…/📝️text`) uses its own kebab keys, so it is unaffected. The editor's `args_bridge::fold` emits
both spellings, so it keeps decoding either.

### 2.3 The schema was the outlier, so the schema was fixed

**Puzzle 5d.**

- `create-part`, `add-part-grip` and `replace-part-grip`: `part2d`/`part3d`/`grip2d`/`grip3d` → `2d`/`3d`, which is Rust's own
  `#[value(rename = "2d")]`.
- The `required` lists now follow the emission: `partKind`, `anchor` (skipped when fixed), `gripKind` and the optional presentation
  fields are optional; `2d`, `3d`, `angle` and `position` are required.
- The catalog `isAbstract` → `abstract`.

**Puzzle 3d.**

- `change-object-anchor` enum `Fixed`/`Derived` → `fixed`/`derived`, and the option labels were re-keyed.
- `create-object` gains `object.anchor`.
- Vortex records in `create-object`, `add-object-vortex` and `replace-object-vortex` gain `label`.
- The catalog `isAbstract` → `abstract`.

**Nullable `Option` fields.** An `Option` field that value_derive emits as `null` is now `type: [T, "null"]` (or a `null` branch).
The first scan found 79 such nodes over puzzle 3d/5d, shooting (`index`, `asset.orientation`), cad, raster (`parentId`), note, block
and forms, and more nested ones appeared after the 5d renames.
The Rust-driven pass also corrected every nested `required` list.

**Shooting.** `create-asset` gains `asset.scale`, a nullable vector with a full annotation.

**Procedural.**

- The widget union was lossy: it was PascalCase `kind` consts with no fields. It is now the real
  `#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]` union, with 9 variants: `neuron`, `inputSlider`,
  `inputNote`, `inputImage`, `variable`, `outputPreview`, `outputAction`, `outputExport` and `cluster`.
- Every field is annotated, and so are the `$defs` the cluster variant reaches (`Tree`, `Neuron`, `Synapse`, `FlowUi`,
  `FlowNodeGui`, `NodeChrome`, `FlowPreviewGui`, `FlowChannelRef`, `CameraJson`, `WidgetLayout`, `Dictionary`).
- The variant labels were kept and now sit on the real kinds.
- The union is generated by one function and written into each of the 4 leaves (2d create/replace, 3d create/update).
- `create-generation`: `valuesJson` (a string) → `values` (an answer map), matching `FormGeneration.values`.

**Cad.** The `create-{shape,building,structure-classic,energy}-model` leaves gain the required `target`, which is the child
`ArtifactRef` URI.

**Raster.**

- `create-layer.layer` now `$ref`s the record document's exact `RasterLayerNode` union.
- That union's branches carry the variant labels (`pixel`, `group`, `adjustment`) and full field annotations.
- `add-layer-asset.asset.data` is the base64 string Rust emits, not an array of bytes.
- `change-layer-pixels` moves from draft 2020-12 to the repository dialect draft-07 (coordinator note).

**Forms.** `Question.params` is `Option<DslValue>`, so the opaque `type: object` became an untyped value.

**Draw.** `update-path-geometry`'s `$id` moves into the catalog scheme:
`…/s/draw/drawing/1/transform/mutation/update-path-geometry/schema.json`. This cleared `leafUncatalogued`.

### 2.4 Colour widget and 3D grid snaps (coordinator note)

**Colour.** Every RGBA vector colour in scope is now `widget: color` with `items {type: number, minimum: 0, maximum: 1}`:

- the draw `StrokeStyle.color` (record document);
- `replace-layer-stroke.stroke.color`;
- `replace-layer-fill` `$defs/Color`, used by the solid fill and the gradient stops (hand-formatted file, text edit);
- the note stroke block `color`, which is `SemioRgba` 0..1.

The committed fixtures stay valid. The hex-string colours (block/puzzle kind colours, shooting sun/ambient, raster brush) are Rust
`String`s, and the `color` widget requires a vector, so they stay `text`.

**Grid snaps.** Puzzle 3d/5d 3D offsets (`unit: m` origins, positions and catalog points) get `snapSource {config: "gridSpacing"}`,
the world grid pitch both editors' config carries.

Shooting, cad and block have no grid config. Their 3D offsets get `snapSource {step: true}`:

- shooting asset origin and `drag-assets` `dx`/`dy`/`dz`;
- cad origins;
- block vortex/grip positions, with step 0.1.

Camera poses are excluded.

## 3. Tooling (ticket root, kept)

- **`🧪️w2-s-design-rust-parity.py`** is a new, independent Rust-side oracle.
  - It lexes every `MutationLeaf` payload struct (leaf `🦀️.rs` or `🦠️mutation/🦀️.rs`) and every `ToValue` struct and unit enum of the
    owning plugin.
  - It derives the value_derive wire form: `rename`/`rename_all`, `Option` without skip emitted as `null`, skipped fields omitted.
  - It compares that form recursively with the leaf schema. Classes: missing / extra / nullable / required / enum.
- **`🧪️w2-s-design-parity-fix.py`** does every edit above.
  - It is idempotent: `--dry-run` after the run reports 0 files.
  - It refuses non-canonical JSON (hand-formatted files get text edits) and re-reads every file before writing it.
  - It runs the Rust-driven nullable/required pass off the scanner.
- **`🧪️w2-s-design-check.ts`** is the third-party cross-check.
  - It compiles every leaf and every aggregate document of the scope (block included) with the strict Ajv oracle
    `semioSchemaAjvV1` and requires draft-07.
  - It validates every committed fixture's whole aggregate wire with Ajv. The lint validates with npm `jsonschema`, so the two
    validators now agree.
  - It reads every leaf through W1-D's `mutationInputDefs`.
- W2-S's `🧪️w2-s-aggregate-rule.py` was run, not modified. W2-R's two design checkers were re-run.

## 4. Verification (all run, foreground, gated)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads --under <each scope path>` (test module) | 0 briefed findings in all 10 paths; newer-lint rows in §6 |
| `bun ./📜️script.ts schema mutation-inputs --under <each scope path>` | **0 findings, exit 0**, all 10 paths (for example cad 51/51, draw 37/37, block 162/162) |
| `python3 🧪️w2-s-design-rust-parity.py <scope>` (via the fix script) | 0 remaining |
| `python3 🧪️w2-s-design-parity-fix.py --dry-run` | 0 files would change |
| `.venv/bin/python 🧪️w2-r-design-check-schemas.py` | 1,487 annotations in 563 documents and 270 fixture payloads of 262 leaves: **0 failures** |
| `bun 🧪️w2-r-design-check-inputs.ts` | 262 leaves, 1,083 inputs: **0 failures** (`color` = 2 top-level readers) |
| `bun 🧪️w2-s-design-check.ts` | 367 leaves, 677 inputs, 24 aggregates, 325/376 fixtures validated (51 have no aggregate document): **0 failures** |
| `cargo check -p semio-s-artifact-shooting-shooting -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-forms-forms --target wasm32-wasip2` | **ok** (warnings only, none in the touched files) |
| `cargo test -p semio-s-artifact-shooting-shooting --lib` (private `target-nde-w2s-c`, `CARGO_INCREMENTAL=0`) | **359 passed, 0 failed** (includes every leaf fixture/canonical-JSON test) |
| `cargo test -p semio-s-artifact-procedural-generation2d --lib` | **177 passed, 0 failed** |
| `cargo test -p semio-s-artifact-procedural-generation3d --lib` | **158 passed, 0 failed** |
| `cargo test -p semio-s-artifact-forms-forms --lib` | **228 passed, 0 failed** after the second fix (see §4.1) |
| `bun ./📜️script.ts oracle exhaustive --owner 🎥️shooting --case 🎥️mutate-shooting-1` (Python second implementation, private output scope) | **62/62 mutation scenarios passed**. The one error is `identity-round-trip`, which is subject-only by design ("adapter has no oracle registration"). |
| `… oracle exhaustive --case 🌀️mutate-procedural-2d-1` / `🧊️mutate-procedural-3d-1` | **29/29** and **29/29 passed** |
| `… oracle exhaustive --case 🌵️mutate-forms-1` | 0/21, all failing before any payload is read (see §4.1) |
| `bun -e 'testFormsQuestionPlacement()'` (forms placement TS twin, Ajv + fast-json-patch) | **ok** |

About the first shooting test attempt: it did not compile because a peer was editing the stdio xml/json leaves in flight. That
peer is adding `#[mutation_leaf(payload = Apply)]` while the derive grows `payload`. The retry was green.

### 4.1 Notes on the test runs

- **Forms, first run: 227/228.** `question_placement_matches_shared_events` read
  `✏️editor/❓️questions/📍️placement/🧫️fixtures/🔣️.json`, whose expected `createBlock` events still carried `step_id`. The fix
  went into three files:
  - the fixture (a hand-formatted text edit);
  - the TS twin `📍️placement/🟦️.ts`;
  - its test `📍️placement/🧪️tests/🟦️.ts`.

  After the fix: 228/228 in Rust, and the TS test is green.
- **Forms Python oracle: 21/21 fail on document shape, not on payloads.** This is independent of this change. The reference refuses
  every committed snapshot: "a form document may carry only [id, results, schema, structure, title, version], found [definition,
  responses, …]". The failure happens before any payload key is read. The committed forms document grew `definition`/`responses`,
  and the reference `🧪️tests/🌵️mutate-forms-1/🐍️.py` was never updated. Owner: the forms case.
- **Shooting Python oracle.** The first run errored 3 scenarios: a peer had renamed the fixture directory
  `✨️polishes-scene-material-to-quarter` → `✨️polishes-scene-material-quarter`. The rename is staged, and the Rust include had been
  updated, but the case had not. I aligned the case's feature table and the Python `VECTORS` to the directory on disk, and the rerun
  was 62/62. The Rust test-case directory keeps its own name.

## 5. Not verified / follow-ups owed by others

- **Catalog.** The schema catalog (`📚️library/🔣️schema-catalog.json`) needs a central `schema generate`, for two reasons:
  - the pinned hashes of the more than 300 schema documents edited here are stale (228 by the aggregate rule, 104 leaf and 5
    record documents by the fix script, with overlap);
  - the draw leaf's new `$id` scope `s.draw.drawing.1.transform.mutation.update-path-geometry` gets registered.

  The mutation-inputs lint already reads the leaf.
- **Plugin descriptors.** Shooting, procedural and forms change their mutation wire, and every touched plugin changes its
  `PAYLOAD_SCHEMA`. They owe the central `describe` regeneration.
- **Procedural artifact documents** (`generation2d|3d/…/✳️any/🧬️schema/🔣️.json`) still describe `Widget` as a JSON string and
  `FormGeneration.valuesJson` as a string. Their TS twins and GraphQL projections do the same. The leaves are exact now, but the
  document-level schema, `🟦️.ts` and `🔗️.graphql` are a generator-level drift outside the mutation lint.
- **Draft 2020-12.** Non-leaf documents in scope still declare it: raster editor commands (`set-brush-hardness`, `set-brush-color`,
  `move-layer`), procedural transforms, config and commands, and forms editor config and panels. Only the raster mutation leaf was
  in the coordinator's note and the lint's reach.
- **Hex colours.** The hex-string colours cannot become `widget: color` without changing the payload type.

## 6. Remaining rows of the newer lint (not schema drift)

1. **Shooting — lint false positive.** `🎥️mutate-shooting-1/🥒️.feature` line 173 is reported as unmapped
   `"identity-round-trip"`.
   - `mutationFeatureRows` takes every step docstring with `kind` and `params` as a wire row.
   - The `@mode-round-trip` scenario's `{"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}` is a harness plan,
     not a mutation.
   - Five features repo-wide carry this row.
   - Fix in the lint: skip kinds that name no mutation leaf, or read only `@mode-mutate` scenarios. This is W2-S's file.
2. **Cad — five object-lifecycle kinds unwitnessed by design.** The kinds are `create-object`, `delete-object`, `move-objects`,
   `rotate-objects` and `scale-objects`.
   - `🔮️oracles/🔣️.json` records why: their state is the pane child materialization, which every wire codec skips. A committed
     snapshot pair is therefore impossible.
   - The `mutation-fixture-bundle-v1` taxonomy contract forbids a mutation-only bundle.
   - They are covered by Rust-constructed laws beside each leaf.
   - The lint needs a recorded-exemption input, or the store needs a persisted child resolver first.
   - `🧩️extensions/🏢️aec-building/…/🏢️create-building-storey` has no fixture bundle either.
3. **Raster — two leaves unwitnessed.** `change-layer-mask` and `change-layer-transform` are tested through their own
   `🧪️tests/🔣️.json` vectors and not through a committed fixture bundle. A proper bundle (before/after/diff/outcome/mutation plus the
   standard leaf test) is expressible and owed.

## 7. Files

- **Created (ticket root, kept):** `🧪️w2-s-design-rust-parity.py`, `🧪️w2-s-design-parity-fix.py`, `🧪️w2-s-design-check.ts`, and
  this report.
- **Modified:** 243 files by the fix script and follow-up edits, plus the 228 files the aggregate rule wrote. The full list is in
  `🗑️generated/w2s-c/touched-files.txt`.

  | plugin | leaf schemas | record schemas | fixtures | Rust | TS | case files |
  |---|---|---|---|---|---|---|
  | shooting | 6 | | 26 | 46 | 1 | 2 |
  | procedural | 6 | | 3 | 16 | | 4 |
  | forms | 8 | 1 | 9 (8 `🦠️mutation` + the placement vectors) | 16 | 11 | 1 |
  | puzzle 3d/5d | 39 | | | | | |
  | cad | 9 | | | | | |
  | raster | 4 | 1 | | | | |
  | note | 11 | 1 | | | | |
  | block | 19 | | | | | |
  | draw | 2 | 1 | | | | |

- **Scratch:** `🗑️generated/w2s-c/` holds the lint JSONs, the scan outputs and `touched-files.txt`.
