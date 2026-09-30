# 📓️ W1-D — Input descriptors (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, work package W1-D (design §6, plan "W1-D", coordinator refinement:
payload accessors live on the kernel `Mutation<P>`). Status: **implemented, compiled, tested**. The open items are listed at the end.

## 1. What landed

### 1.1 `x-semio-ui` vocabulary (schema-first)

- Meta-schema: `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` gains `$defs` `InputUi` (the annotation value), `InputLabel`
  (`{en,de}` or the `{native,reuse}×{en,de}` matrix, every locale required and non-empty), `InputLabelLocales`,
  `InputLabelGlossary` and `MutationInputCorpus`. Honest formats: `x-semio-formats: ["🔣️jsonschema","🟦️typescript"]`
  (corpus: jsonschema only). TypeScript format: types plus `parseInputUi` / `parseInputLabel` / `parseInputLabelLocales` /
  `parseInputLabelGlossary` in `🛂️manifest/🧬️schema/🟦️.ts`. `schema check` reports no finding on these exports.
- Keys: `widget` (slider|stepper|dial|toggle|select|segmented|text|multiline|vector|reference|hidden), `role`
  (value|target|discriminator), `label`, `description`, `step` (UI only, never `multipleOf`), `precision`, `softMin`, `softMax`,
  `snaps`, `snapSource` (`{step:true}` | `{config:key}` | `{snapshot:pointer}`), `unit`, `displayUnit`, `displayFactor`,
  `scale` (linear|log), `group`, `order`, `options` (`{enumValue: InputLabel}`), `ref` (`{kind: string|string[], domain?, granularity?}`).
  W1-F's shape was accepted verbatim; the reply to W1-F set the rules for nested fields and option labels.
- Registration: `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json` adds
  `"x-semio-ui": {"$ref": ".../framework/manifest/schema.json#/$defs/InputUi"}`. The strict Ajv oracle
  (`🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts`) now adds the manifest schema document before it registers keywords
  (`SEMIO_SCHEMA_VENDOR_VOCABULARY_DOCUMENTS_V1`). The engine-contract test used to add that document itself, so three
  duplicate `.addSchema(manifestFixtureSchema)` calls were removed there.

### 1.2 One input vocabulary (`🛂️manifest/🦀️.rs` and its twins)

- `ArgSchema::Number` gains `min_exclusive`, `max_exclusive`, `snaps`, `snap_source`, `soft_min`, `soft_max`, `precision`,
  `display_unit`, `display_factor` and `scale`. There is a new helper `ArgSchema::number(min, max, step, integer)`.
- `ArgSchema::Vector { dims, unit, step }` replaces `Vec3`, and the builder `ActionArgDef::vector(id, label, dims)` replaces `vec3`.
- `ArgSchema::Reference { kinds, domain, granularity, many, min_items, max_items }`. The field is `kinds`, because a field
  named `kind` collides with the enum's serde/value tag.
- New enums `SnapSource { Step, Config{key}, Snapshot{pointer} }` and `NumberScale { Linear, Log }`.
- `ArgPresentation` gains `Stepper`, `Dial` and `Segmented`.
- `ActionArgDef` gains `group: Option<String>` and `order: Option<i64>`. `description` is now `Option<LocalizedLabel>` (it used
  to be an English-only `String`), and `describe()` takes a `LocalizedLabel`.
- `ActionArgDef::key()` decodes the single-segment pointer id of a mutation input.
- `ActionArgControl` gains `Stepper`, `Dial`, `Segmented`, `Vector` and `Reference`, and `Number` and `Slider` carry the new
  numeric facets. A Slider's `min`/`max` are its travel range: the soft range when one is declared, otherwise the hard bounds.
- `control()` resolves the widget in this order: the presentation wins; otherwise an integer is a Stepper; otherwise a
  number bounded on both sides is a Slider; otherwise it is a plain Number field. As a result `index()` args are now Steppers
  (they used to be Sliders over 0..u32::MAX).
- `json_schema(terminology, locale)` resolves the localized description. It emits `exclusiveMinimum`/`exclusiveMaximum`,
  `x-semio-format: reference` + `x-semio-ref`, and a vector's `minItems`/`maxItems`. It no longer emits `multipleOf`.
  The MCP catalog now threads locale and terminology into it.
- **Deleted**: `ConfigFieldShape`, `ConfigFieldSpec` and `CommandFieldSpec`. `ConfigSpec.fields` and
  `CommandVariantSpec.fields` are now `Vec<ActionArgDef>`.
- Call sites moved in the same change:
  - shooting config, as three localized selects;
  - the workflow binding type check and the host `build_configure_config`, plus their tests;
  - the MCP source-builder test;
  - the draw arg descriptions, now en+de;
  - generation3d, stdio mesh, stdio brep and the clipboard `paste` action, now using `vector(…, 3)`;
  - plugin `declared_argument_alternatives`, which gained `Vector` and `Reference` arms;
  - the wgpu staged-arg rows, which resolve the localized description and use `..` patterns;
  - the React `renderStagedArgControl`: stepper/dial go to a number input, segmented to the select, a vector gets N inputs, a
    reference gets a comma list, and the slider gets `snapValues`;
  - `ResolvedActionArgDef` resolves the description.
- TS twin: `argControl()` in `🛂️manifest/🟦️.ts`. The legacy "missing schema" fallback was deleted, and string `options` are
  read as optional, matching the wire. The projection `🧬️schema/📽️projection/🦀️.rs` is updated (versions bumped; `SnapSource`
  and `NumberScale` added; the three deleted types removed). The gitignored `🤖️generated/🪪️manifest/🟦️.ts` was re-rendered
  from the projection; the script is `🗑️generated/w1d/render_manifest_ts.py`.

### 1.3 Payload schema and payload accessors (derive)

- `MutationLeaf::PAYLOAD_SCHEMA: &'static str` is in `📡️replication/🎮️mutation/🦀️.rs`. `#[derive(dsl::MutationLeaf)]`
  resolves the descriptor's `payloadSchema` beside the descriptor. The helper is `mutation_leaf_payload_schema_path` in
  `🗣️dsl/✨️derive/🦀️.rs`: it accepts a relative portable path, rejects `..`, empty segments, backslashes and `compose`, and
  requires a no-follow regular file. The derive then emits `include_str!`, which also makes cargo track the file.
- Kernel `Mutation<P>` (coordinator refinement) gains `const INPUT_SCHEMAS: &'static [&'static str]`, `fn input_schema()`,
  `fn payload_value()` and `fn with_payload_value(value)`, with defaults: `&[]`, `None`, the whole value, and a whole-value
  decode. `#[derive(Mutations)]` generates the precise versions: each leaf's `PAYLOAD_SCHEMA` row-aligned with `DESCRIPTORS`,
  the leaf payload without the aggregate tag, and a rebuild of the same variant only. `SemanticMutation<P>: Mutation<P>`
  exposes them.
- The 6 hand-written puzzle projection bridges (`impl Mutation<Value>` and `impl Mutation<PuzzleNdPlaySnapshot>` for 2d, 3d and
  5d) forward all four. W1-F was notified and confirmed the block is kept.
- Two real leaves had no normative payload schema, so the derive refused them. That was the "31 crates stop at stdio-png"
  report. Both schemas are now authored:
  - `✏️s/…/📷️png/…/📸️set-snapshot/🧬️schema/🔣️.json` refs the png `snapshot.json`;
  - `✏️s/…/🏗️fem/…/🫧️transient/…/⏱️set-playback-clock/🧬️schema/🔣️.json` uses a local `$defs` clock, because the transient
    schema is not a catalogued scope.

  `schema-mutation-leaf-schema-absent` went from 12 to 10, and there is no `schema-mutation-leaf-id` finding for either.

### 1.4 Reader (Rust + TS twin)

- Rust (`semio_framework`, region `🔖️MutationInputs` of `🛂️manifest/🦀️.rs`):
  - `pub trait InputSchemaResolver { fn resolve(&self, id:&str) -> Option<DslValue>; }`, with a blanket impl for `Fn`;
  - `pub fn mutation_input_defs(schema_json:&str, resolver:&dyn InputSchemaResolver) -> Result<Vec<ActionArgDef>, InputSchemaError>`;
  - `pub fn mutation_input_instance(schema_json, resolver, payload:&DslValue) -> Result<DslValue, InputSchemaError>`, which
    splices every root `const` discriminator back in so the owned validator accepts a `payload_value`;
  - `pub fn registered_input_schema_document(id)`, a resolver over the `schema://` export registry;
  - `pub fn input_label_glossary()`;
  - `InputSchemaError { code: InputSchemaErrorCode, pointer, detail }`, where the code is one of
    malformed | refUnresolved | uiInvalid | widgetIncompatible | labelMissing | optionLabelMissing | localeMissing.
- TS twin in `🛂️manifest/🟦️.ts`: `mutationInputDefs`, `mutationInputInstance`, `inputLabelGlossary`, and the
  `InputSchemaError` class, which is thrown.
- Rules:
  - `$ref` resolves locally (`#/$defs/…`, also inside a foreign document) or through the resolver.
  - Nullable `oneOf`/`anyOf` unions are unwrapped.
  - `allOf` members are merged.
  - A root payload union is refused as `malformed`, because it has no flat inputs.
  - `x-semio-ui` layers merge key by key, and the outer layer wins.
  - A `const` or `role: discriminator` input is skipped.
  - Objects and array items that are objects are read recursively.
  - An array of 2 to 4 numbers of fixed length is a `Vector`.
  - A string or string array named `<kind>Id(s)` or `<kind>_id(s)` is inferred as a `Reference` (a leading `new` is
    stripped; `newId` is never a reference).
  - For integers, `exclusiveMinimum`/`exclusiveMaximum` fold to inclusive bounds; for numbers they set the `*_exclusive` flag.
  - An integer's default step is 1.
  - The label comes from `x-semio-ui.label`, else the glossary, else the input is refused. `options` labels come from
    `x-semio-ui.options`, else the glossary, else the input is refused.
  - Each input's `id` is the single-segment RFC 6901 pointer relative to its parent (e.g. `/dx`). Errors point into the
    payload (`/node/x`, and `/list/-/field` for array items).
- Glossary: `🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json`, schema `InputLabelGlossary`. It has 258 names with en+de labels and covers
  the 243 names that occur at least 3 times, i.e. every one of the 200 most frequent input names. It also adds dx, dy, dz,
  angle, factor, pivotX, pivotY, targets, opacity and scale. The list was computed from the 2,834 leaf schemas by
  `🗑️generated/w1d/input_names.py`; the translations were written by hand.

### 1.5 Roster

`WireMutationRosterEntry.inputs: WireMutationInputs`, a tag-`status` enum:

- `Declared { inputs: Vec<ActionArgDef> }`;
- `Refused { error: InputSchemaError }`;
- `Opaque`, for a contributed plan, or when the aggregate publishes no schema.

It is computed from `OwnerMutationRoster = fn() -> (&'static str, &'static [SemanticDescriptor], &'static [&'static str])`. The
builder's three `*_mutation_roster` thunks pass `Mutation::INPUT_SCHEMAS`.

### 1.6 Repo lint `schema-mutation-input-ui`

- The code is registered in `SCHEMA_DIAGNOSTIC_CODE_TABLE` (`🦑️repo/🔨️modules/🧪️test/🟦️.ts`, emitter `harness`) and in the
  protocol enum `🧪️test/🧬️schema/🔣️.json` `SchemaDiagnosticCode`. The invariants code-table suite passes (5/5).
- Implementation: `mutationInputUiReport` in `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`.
  - It reads every catalogued mutation leaf with the TS reader.
  - It resolves `$ref`s through every `$id` document found under the catalogued scope directories.
  - It evaluates one top-level input at a time, so every refused input is counted.
- Command: `bun ./📜️script.ts schema mutation-inputs [--census] [--under <path>] [--json]` from the test module. Strict mode
  exits 1 on any finding; `--census` always exits 0. The nx targets are `@semio-tech/repo-test-domain:test-schema-mutation-inputs`
  and `…:test-schema-mutation-inputs-census`. The census target was verified by running it through nx.
- It is deliberately **not** wired into `schema check` or `test schema`. It would fail those gates until the W2-R* rollout
  finishes.

### 1.7 Language-agnostic corpus and oracles

- Corpus: `🛂️manifest/🧫️fixtures/🧫️mutation-inputs/🔣️.json` (schema `MutationInputCorpus`). It has 22 cases: 9 declared and 13
  refused. It covers annotation, dial, log slider with soft range, glossary inference, local `$defs` overrides with option
  labels and nullable unions, registry refs (including a local ref inside a foreign document), nested objects, arrays and
  vectors, `allOf`, an empty payload, and every error class.
- Rust: `🛂️manifest/🧪️tests/🧪️mutation-inputs/🦀️.rs` compares canonical JSON (sorted keys, integral numbers written as
  integers) of every case. It also checks control derivation, the glossary, the wire round trip and `mutation_input_instance`.
- TS (`bun:test`): `…/🧪️mutation-inputs/🟦️.ts` checks the same canonical strings, which proves Rust and TS produce
  byte-equal output. It also checks:
  - npm `jsonschema` 1.5 validation of the corpus and the glossary;
  - a third-party cross-check: every hard bound, option and requirement a descriptor declares is probed as accept/reject
    verdicts against the leaf schema, and `jsonschema` must agree on all of them (at least 60 verdicts);
  - the strict Ajv oracle compiles every annotated leaf;
  - `parseInputUi`, `argControl` and `mutationInputInstance`.
- Python adapter: `…/🧪️mutation-inputs/🐍️.py` runs the same verdicts with Python `jsonschema` (Draft7Validator + `referencing`)
  and validates the corpus and glossary against the manifest exports. A deliberately lying descriptor (max 2 instead of 1) was
  confirmed to be caught (`value:max`).
- Runner: `bun ./📜️script.ts test-mutation-inputs` in `🧰️framework/📦️packages/🦀️rust` (bun, then `.venv` python, then cargo),
  with nx target `@semio-tech/framework-rs:test-mutation-inputs`.
- Accessor test: `📡️spr/🎮️command/🧪️tests/🧪️mutation-payload/🦀️.rs` exercises `INPUT_SCHEMAS`, `input_schema`,
  `payload_value` (no `operation` tag) and `with_payload_value` (same kind, refuses a foreign shape) on the counter fixture
  aggregate.
- Derive test: `resolves_the_payload_schema_beside_the_descriptor_and_refuses_escapes` in `🔬️mutation-leaf-derive`.

## 2. Verification (all foreground, gated; `cargo test` with `CARGO_INCREMENTAL=0` and private `target-nde-w1d`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework` | ok. The only warning is a peer's `5905:79 unnecessary qualification`. |
| `cargo check -p semio-framework-os-kernel -p semio-framework-plugin` | ok. The first attempt hit a peer's store error (`🏪️store/🦀️.rs:18104 transaction`); the retry was green. |
| `cargo check -p semio-framework-os-renderer-wgpu` | ok (34 s) |
| `cargo check -p semio-s-artifact-puzzle-2d` | ok (leaves derive with PAYLOAD_SCHEMA) |
| `cargo check -p semio-s-plugin-stdio -p semio-s-artifact-draw-drawing -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-note-note -p semio-s-artifact-forms-forms -p semio-s-artifact-cad-cad` | ok (44 crates checked, including all stdio artifacts and png) |
| `cargo check -p semio-s-artifact-fem-3d -p semio-s-artifact-shooting-shooting -p semio-s-artifact-procedural-generation3d -p semio-framework-os-mcp -p semio-framework-artifact-workflow-workflow -p semio-framework-os` | ok (78 crates checked) |
| `cargo test -p semio-framework --lib -- manifest::mutation_inputs_tests` | 6 passed |
| `cargo test -p semio-framework --lib -- manifest::app_label_tests` (with the above) | 89 passed |
| `cargo test -p semio-framework --lib` (full) | 286 passed, 7 failed. All 7 are `manifest::kernel::ui_turn_patch_tests::*` (the UiTurnPatches transport, not my area). |
| `cargo test -p semio-framework-os-kernel-dsl-derive` | 17 passed |
| `cargo test -p semio-framework-os-kernel --lib -- mutation_payload_tests mutation_laws_fixture` | 15 passed |
| `cargo test -p semio-framework-plugin --lib -- mutation_roster_entries_are_deterministic_across_repeated_calls declared_argument` | 3 passed (the roster rows carry `Declared` inputs / `Opaque`) |
| `bun test ./…/🧪️mutation-inputs/🟦️.ts` | 28 passed |
| `.venv/bin/python …/🧪️mutation-inputs/🐍️.py` | ✔ 75 jsonschema verdicts over 22 corpus cases |
| `bun ./📜️script.ts test-mutation-inputs` (framework-rs) | exit 0 (28 TS, 75 Python verdicts, 6 Rust) |
| `bunx tsc --noEmit` (framework TS project) | 0 errors |
| `bunx tsc --noEmit` (os TS project) | 15 errors, none in files I touched: the engine-contract `Component` literals at 4843–4920 (UI contract), frame-worker, browser-bundle worker, package-integration, and hub-edit-durability |
| tsc over the lint orchestration and the oracle | 0 errors |
| `bun test …/🏷️schema-vocabulary/🟦️.ts` | 2 of 3 passed. The failure is the pre-existing undeclared `x-semio-fixture` in `🌎️hub/🧫️fixtures/🐳️docker-image-v1/🔣️.json`; `x-semio-ui` itself compiles strict. |
| `bun test …/🧬️schema-invariants/🟦️.ts` | code-table suite 5/5. The full file is 117 passed, 5 failed; all 5 fail on pre-existing tree state (acceptance schema dialect, a stale catalog in `🎯️acceptance`, eligibility vectors). |
| `bun ./📜️script.ts schema generate` + `schema check` | catalog regenerated (3,551 scopes). No finding on the new manifest exports or the two new leaf schemas; `mutation-leaf-schema-absent` went from 12 to 10. |
| `verify taxonomy report` | clean for `🧪️tests/🧪️mutation-inputs`, `🧫️fixtures/🧫️mutation-inputs`, the png leaf and `🎮️command/🧪️tests/🧪️mutation-payload` (the 5 errors there are pre-existing). The manifest root scope and the glossary file could not be checked: the report crashes on a peer's modified `📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` (frozen digest). The fem3d leaf directory already exceeded 240 bytes (`path-too-long`), and the new schema file inherits that. |
| `cargo test -p semio-framework-plugin --lib` (full) | did not compile because of peer errors: `🧩️extension/🚪️retirement` `CapabilityRequest`/`SnapshotRetirementStep::Blocked`, and `⚛️reactor` inbound-request test types |

## 3. Census (`schema mutation-inputs --census`, after `schema generate`)

Result: 3,276 of 5,118 top-level inputs across 2,839 leaves carry a UI descriptor, with 1,842 findings. Puzzle 2d alone
(`--under ◻️2d`) has 103/103 inputs of 36 leaves declared and 0 findings.

| owner | leaves | inputs | declared | missing | labelMissing | malformed | optionLabelMissing | refUnresolved |
|---|---|---|---|---|---|---|---|---|
| stdio | 988 | 1638 | 1058 | 580 | 500 | 51 | 16 | 13 |
| norm | 484 | 895 | 525 | 370 | 367 | 0 | 3 | 0 |
| energy | 291 | 737 | 411 | 326 | 326 | 0 | 0 | 0 |
| architect | 268 | 332 | 195 | 137 | 137 | 0 | 0 | 0 |
| remodel | 36 | 56 | 13 | 43 | 40 | 0 | 3 | 0 |
| wfc | 72 | 133 | 98 | 35 | 33 | 0 | 2 | 0 |
| block | 105 | 162 | 129 | 33 | 32 | 0 | 0 | 1 |
| fem | 59 | 85 | 52 | 33 | 33 | 0 | 0 | 0 |
| os | 18 | 34 | 2 | 32 | 28 | 1 | 3 | 0 |
| puzzle (2d+3d+5d) | 106 | 277 | 248 | 29 | 25 | 0 | 4 | 0 |
| layout | 31 | 89 | 61 | 28 | 28 | 0 | 0 | 0 |
| shooting | 39 | 67 | 40 | 27 | 27 | 0 | 0 | 0 |
| cad | 25 | 47 | 33 | 14 | 9 | 0 | 5 | 0 |
| procedural | 45 | 57 | 43 | 14 | 12 | 0 | 0 | 2 |
| note | 34 | 60 | 47 | 13 | 13 | 0 | 0 | 0 |
| forms | 14 | 26 | 15 | 11 | 11 | 0 | 0 | 0 |
| raster | 17 | 43 | 32 | 11 | 11 | 0 | 0 | 0 |
| draw | 17 | 35 | 25 | 10 | 10 | 0 | 0 | 0 |
| reasoning | 12 | 27 | 17 | 10 | 7 | 0 | 0 | 3 |
| playbook | 12 | 23 | 14 | 9 | 9 | 0 | 0 | 0 |
| process | 16 | 26 | 17 | 9 | 9 | 0 | 0 | 0 |
| lowpoly | 17 | 40 | 32 | 8 | 8 | 0 | 0 | 0 |
| trinity | 17 | 25 | 17 | 8 | 7 | 0 | 0 | 1 |
| writer | 10 | 14 | 6 | 8 | 8 | 0 | 0 | 0 |
| dag | 17 | 36 | 29 | 7 | 7 | 0 | 0 | 0 |
| imperative | 7 | 14 | 7 | 7 | 7 | 0 | 0 | 0 |
| mathematical | 16 | 33 | 26 | 7 | 7 | 0 | 0 | 0 |
| animate | 11 | 15 | 10 | 5 | 5 | 0 | 0 | 0 |
| space | 5 | 8 | 4 | 4 | 4 | 0 | 0 | 0 |
| flow | 10 | 27 | 24 | 3 | 3 | 0 | 0 | 0 |
| gis | 16 | 25 | 22 | 3 | 3 | 0 | 0 | 0 |
| sequence | 8 | 13 | 10 | 3 | 3 | 0 | 0 | 0 |
| vcs | 6 | 6 | 3 | 3 | 3 | 0 | 0 | 0 |
| demonstrator | 1 | 1 | 0 | 1 | 1 | 0 | 0 | 0 |
| window | 6 | 8 | 7 | 1 | 1 | 0 | 0 | 0 |
| sourcing | 3 | 4 | 4 | 0 | 0 | 0 | 0 | 0 |

The `malformed` count (52) is the root payload unions. They need a W2-R decision: split the leaf, or make the payload an
object with a discriminated field. The 21 `refUnresolved` are genuine schema gaps: relative-file `$ref`s, `$id`s outside every
catalogued scope (`framework/value`, `os/store/link`, `snapshot-patch`), and two pdf `diff.json#/$defs/*` that land on nothing.

## 4. Open items

1. **Plugin descriptors are stale.** Every committed `✏️s/🔌️plugins/*/🔣️.json` and `🛂️.descriptor.semio` needs a central
   `describe` regeneration. Three things changed: the injected clipboard `paste` action's `position` arg is now
   `{kind: vector, dims: 3}`; arg descriptions are `LocalizedLabel`s; and the shooting config fields are `ActionArgDef`s.
   Until then, anything that decodes a committed descriptor sees the old `vec3` or string shapes.
2. W1-E / W2-B:
   - The UIDialog story (`📨️UIDialog/📖️stories/🧪️.story.tsx`, owned by W1-E) maps only number/slider controls, so its integer
     `quantity` now falls back to text. It should handle `stepper`.
   - The React staged renderer's handling of reference and dial is an interim fallback (a comma list and a number input) until
     W1-E's `reference_list` and dial recipes land.
   - wgpu staged rows still fall through to a text input for the new kinds.
3. The lint is standalone. Wire it into `test schema`/CI once the W2-R* rollout reaches 0 findings.
4. Unrelated failures seen and left alone: the peer `🏪️store` compile error (transient; the retry was green),
   `ui_turn_patch_tests` (7), the plugin full-suite compile errors (extension retirement, reactor test), the 15 os tsc errors
   (UI contract `Component` literals and others), the hub `x-semio-fixture` vocabulary gap, and the taxonomy report crash on the
   cad frozen fixture.
5. Design naming deviations, both agreed or forced:
   - accessor names follow the coordinator refinement (`input_schema`, not `payload_schema`) and live on `Mutation<P>`;
   - `ArgSchema::Reference.kinds`, not `kind`, because of the serde tag collision.

## 5. Files

- Created:
  - `🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json`
  - `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/🧫️mutation-inputs/🔣️.json`
  - `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/{🦀️.rs,🟦️.ts,🐍️.py}`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🧪️mutation-payload/🦀️.rs`
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json`
  - `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/⏱️set-playback-clock/🧬️schema/🔣️.json`
- Modified, framework:
  - `🛂️manifest/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🧬️schema/🟦️.ts,🧪️tests/🔬️app-label/🦀️.rs}`
  - `🧬️schema/📽️projection/🦀️.rs`
  - `📡️replication/🎮️mutation/{🦀️.rs,🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs}`
  - `🧰️framework/📦️packages/🦀️rust/{🦀️.rs,📜️script.ts,📋️project.json}`
- Modified, os:
  - `🗣️dsl/✨️derive/{🦀️.rs,🧪️tests/🔬️mutation-leaf-derive/🦀️.rs}`
  - `📡️spr/🎮️command/🦀️.rs`
  - `🔌️plugin/{🦀️.rs,🏗️builder/🦀️.rs,🧪️tests/🔬️app-artifact-contribution/🦀️.rs,🧪️tests/⚖️declared-verb-verdicts/🦀️.rs,🧪️tests/🔬️app-app-builder/🦀️.rs}`
  - `🌉️mcp/{🗂️catalog/🦀️.rs,🧪️tests/🧱️source-builders/🦀️.rs}`
  - `🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`
  - `🖥️host/{🦀️.rs,🧪️tests/🔬️instance-unit/🦀️.rs}`
  - `📺️renderer/🧑‍🎨engine/🧱️elements/{🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs,🛠️ShellHelpers/🟦️.tsx}`
  - `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
  - `🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json`
  - `🧪️tests/🧬️schema-oracle/🟦️.ts`
- Modified, repo:
  - `🧪️test/{🟦️.ts,🧬️schema/🔣️.json,🧬️schema/📋️orchestration/🟦️.ts,📋️project.json}`
  - `📚️library/🔣️schema-catalog.json` (regenerated)
- Modified, plugins:
  - puzzle `◻️2d|🧊️3d|🖐️5d/…/🧬️schema/🧬️mutations/🦀️.rs` (bridge forwarding)
  - procedural generation3d `✏️editor/🦀️.rs`
  - stdio semio `🔺️mesh|🧊️brep/✏️editor/🦀️.rs`
  - shooting `✏️editor/🦀️.rs`
  - draw `✏️editor/🦀️.rs`
  - trinity jack `✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- Scratch, in `🗑️generated/w1d/`:
  - `input_names.py`, `sample_names.py`, `glossary_source.py`, `render_manifest_ts.py`, `tsconfig.lint.json`
  - the logs `check-*.txt`, `test-*.txt`, `tsc-*.txt`, `census.tsv`, `schema-check-after.txt`

## 6. Follow-up (coordinator items after the first report)

All runs are foreground and gated. Rust tests use `CARGO_INCREMENTAL=0` and the private `target-nde-w1d`.

### 6.1 Discriminated root unions

- Both readers accept a payload `oneOf`/`anyOf` whose members all pin one property to a distinct string `const`.
  - The union yields one required variant selector: a `String` with options, `Segmented` for up to 4 variants, `Select` above that.
  - Each option is labelled by the member's `x-semio-ui.label`, else by the glossary; every locale is required.
  - The selector is followed by the union of the member fields, each with `group` set to the variant value.
  - `mutation_input_instance` / `mutationInputInstance` pick the variant the payload names and splice that variant's constants.
- All 52 root unions in the tree are discriminable, so there is no non-discriminable list.
  - glTF uses `phase` apply/restore; the glossary labels both.
  - `os.config` uses `mutation` with 10 variants; its members carry labels.
  - glTF `restore` exposes a whole diff. This is now handled by the hidden stop in §6.5.
- Corpus: `discriminated-union-by-const`, `discriminated-union-annotated-variants`, `refuses-an-undiscriminated-union` and `refuses-an-unlabelled-variant`.

### 6.2 Cycle guard (priority item)

- Rust `InputSchemaReader::guarded` and TS `guarded` keep the resolution path as `document#fragment` keys, starting at `#`.
- A `$ref` that comes back while it is still on that path yields `ArgSchema::Any` (a structured value) instead of recursing. The guard covers input schemas and array items.
- The lint never crashes. Any throw that is not an `InputSchemaError` becomes the finding `readerFault`.
- Corpus: `recursive-tree-is-a-structured-value`, which covers `Node → children → Node` plus a direct `$ref: "#"`.
- The 11 affected leaves (pdf outlines/acro-form, semio value/drawing) now read with no `readerFault`.

### 6.3 Integer-id references

- `ReferenceIdType { String (default, off the wire), Integer }` is a field `id_type` on both `ArgSchema::Reference` and `ActionArgControl::Reference`. Wire and TS: `idType?: "string" | "integer"`.
- `role: target`, `widget: reference` and `ref` are admitted on `type: integer` and on arrays of integers.
  - Name-based inference stays string-only, so energy node numbers stay numeric.
  - A number UI key on a reference is `uiInvalid`.
- Helpers, all twins:
  - `ReferenceIdType::id_value` / `referenceIdValue` turn a selection text into the payload value.
  - `reference_id_text` / `referenceIdText` give the chip text.
  - `REFERENCE_ID_INTEGER_MAX` is 2^53−1.
- Corpus: `integer-references`, `refuses-a-step-on-a-reference`, and a `referenceIds` table (13 rows). The Python oracle holds the table to its own parser and to jsonschema.
- Consumers updated: the React `StagedReferenceField` (new vitest case), the wgpu staged reference row, and W2-A's `⏪️time-travel` coerce, selection draft and chips.

### 6.4 Collecting mode (`mutation_input_audit` / `mutationInputAudit`)

- Every refusal at every nested pointer is recorded, and reading continues:
  - a fault in the label, description, group, order, role or widget keeps the input;
  - a value fault drops only that input's schema;
  - an undeclared UI key is skipped;
  - an unresolved `$ref` drops only that field.
- Findings are de-duplicated and in reading order. `findings[0]` equals the fail-fast error.
- The corpus gains optional `expectedFindings`. Every case is checked for findings in both languages, and for inputs where it declares them.
- New cases: `collects-every-nested-finding` (7 findings) and the two-variant `refuses-an-unlabelled-variant`.
- The lint audits each leaf once. Census columns are owner, leaves, inputs, declared, missing, findings, then per-code counts.
  - `declared` counts top-level inputs whose subtree is clean.
- Catalogued module scopes at a leaf position (e.g. `os.store.mutation.*`) now count as leaves.
- New finding `leafUncatalogued`: a walk of the disk flags any leaf schema module with no catalogued scope.

### 6.5 Hidden stop and vendor keywords

- `widget: hidden` on an object or array yields `{kind: any}` with presentation hidden. The reader does not descend into it, so no nested labels are needed. The input's own label is still required.
- `role: discriminator` was already skipped.
- Corpus: `hidden-input-is-not-read-into`.
- `x-semio` / `x-semio-mutation` had no code consumer in leaves. They were hand-copied into the `🟦️.ts` descriptors, and the glTF generator only mentions them in comments.
  - I removed them from all 120 glTF mutation leaf schemas (script `🗑️generated/w1d/strip_x_semio_leaf_keys.py`).
  - 120 of 121 glTF leaves compile under the strict `semioSchemaAjvV1`. The one failure is `set-snapshot`, whose `snapshot.json` sub-document my probe did not load; it is not a keyword error.
  - Collection manifests keep `x-semio`, the taxonomy semantic extension key.

### 6.6 Color widget and vector facets

- `widget: color` means an sRGB `Vector` of 3 components (RGB) or 4 (RGBA, straight alpha), each within 0..=1. Hex strings stay `text`.
  - It requires items `{type: number, minimum: 0, maximum: 1}`; otherwise the result is `widgetIncompatible`.
  - `ArgPresentation::Color` leads to `ActionArgControl::Color { alpha }`.
- `Vector` now carries `min`, `max`, `unit`, `step`, `snaps`, `snapSource`, `precision`, `displayUnit` and `displayFactor`.
  - These are validated by the same number reader.
  - softMin, softMax and scale stay number-only.
  - A facet on a non-numeric array is `uiInvalid`.
- A plain numeric array passes its facets and `unit` down to its items; the item's own annotation wins. This keeps the 12-month norm climate arrays valid.
- React: `StagedColorField`, a native picker plus an alpha field "a" (vitest). The wgpu and time-travel color controls are routed to W1-E and W2-A by main.
- Corpus: `vector-with-grid-facets`, `color-rgb-and-rgba`, `numeric-array-items-take-the-array-facets`, `refuses-a-color-outside-the-unit-range` and `refuses-a-grid-snap-on-a-plain-array`.

### 6.7 refUnresolved fixes at the source

| Gap | Fix |
|---|---|
| stdio `snapshot-patch` (7 leaves) | The contract schema sat under an ineligible owner (`📇️registry/🧬️contract/✏️editing/🩹️patch`). I moved it into the catalogued `s.stdio.registry` scope as `$defs/SnapshotPatch`, labelled its `edits`/`edit`, and repointed the 7 leaves. The contract test now reads the registry export through `semioSchemaAjvV1` and matches aggregate branches by `$ref` (31/31). |
| `framework.value` ambiguity (reasoning 3, trinity 1) | `🧭️path` path-edit `$id` → `framework/value/codec/path/schema.json`, title `TypedValuePathEdit`. |
| block `set-brush-preview` | Inlined `Block3dBrushPreview`/`Vec3`, because the window-transient lane is an ineligible owner. |
| procedural viewer `set-preview-camera` (config, presence) | The config leaf now holds `$defs/Generation3dViewCamera`; the presence leaf refs the config leaf `$id`. |
| wav `patch-data` | Relative `../set-data/schema.json` → the absolute set-data leaf `$id`. |
| pdf `PdfPathSegment`/`PdfPageBox` | The defs were missing from `🔺️diff`. I added them to all four twins (JSON, TS with parsers, GraphQL, proto) from the Rust enums. |
| draw `update-path-geometry` | Its off-scheme `$id` is now in the catalog scheme with title `UpdatePathGeometry`, and the aggregate `$ref` is updated. It is catalogued now. |

Not fixed; this needs a decision:

- `os/store/link.json` (kit 3) and `os/store/child.json` (wfc 2).
- At runtime these resolve: `🏪️store/🦀️.rs` registers `child`/`link`/`blob` as named exports of the one scope `os.store`.
- The catalog, however, maps each `🧬️schema` dir to one scope. Four dirs claim `os.store`: blob, link, child and the store root, and only the blob dir holds the row.
- Either the catalog learns multi-document scopes, or the ids become `os/store/{link,blob,child}/schema.json`. The latter also needs `child/owner.json` → `os/store/child/owner/schema.json`, the runtime registration split per scope, and 54 + 10 + 2 + 2 JSON refs rewritten.
- That is W1-G's area. I recommend the id split, because it matches the one-scope-per-dir law.

### 6.8 Census (on-disk schemas; no `schema generate`, per coordinator)

5317/5323 inputs of 2983 leaves carry a UI descriptor; there are 72 findings:

| Owner | Leaves | Inputs | Declared | Findings |
|---|---|---|---|---|
| stdio | 988 | 1671 | 1668 | 3 refUnresolved (kit → `os/store/link.json`) |
| wfc | 77 | 142 | 140 | 2 refUnresolved (grid2d → `os/store/child.json`) |
| os | 71 | 136 | 135 | 3 labelMissing (`flow…replace-flow-host-snapshot`: `/hostSnapshot/{schema,widgets,synapses}`) |
| norm | 548 | 895 | 895 | 64 leafUncatalogued (en1990/en1991 leaves have no `$id`) |
| every other owner (31) | — | — | all | 0 |

Catalog staleness: none of the 72 is staleness only.
- The catalog on disk (refreshed by a peer at 05:50) already has every in-scheme leaf, including draw `update-path-geometry`.
- The 64 norm leaves need a real `$id`.
- The 5 refs need the store scope decision above.

### 6.9 Verification of the follow-up

- Rust:
  - `manifest::mutation_inputs_tests` 9/9; with `app_label_tests` 93/93.
  - `exports_typescript_bindings` (typegen) ✔.
  - `cargo check -p semio-framework-plugin -p semio-framework-os-renderer-wgpu --lib` is clean.
- TS:
  - mutation-inputs `bun test` 78/78.
  - React staged-arg-controls vitest 9/9.
  - stdio patch contract 31/31.
  - framework tsc 0 errors; lint tsconfig 0 errors.
  - os tsc has 26 errors, all in peer files.
- Python: 144 jsonschema verdicts over 35 cases plus 13 reference ids ✔.
- Peer state seen and not touched:
  - W2-C `🧪️wgpu-time-travel` test `PanelTabKind` (mid-edit).
  - The W2-A time-travel fixture scenario `a-fatal-replay-blocks-finalizing` (their set-slot-children leaf was being edited).
  - The os tsc errors in package-integration and others.

### 6.10 Open after the follow-up

1. The os.store scope split (§6.7). It is W1-G's area.
2. Runtime registration of `s.stdio.registry`:
   - `registered_input_schema_document` only sees registered exports, and the stdio plugin registers no `s.stdio.registry` export.
   - So the runtime history editor still cannot resolve `SnapshotPatch`; it could not resolve the old `snapshot-patch` id either.
   - Register the registry schema via `register_scope_schema_exports`.
3. `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧬️schema/🔣️.json` is now unreferenced. Its content lives in the registry `$defs`. I did not delete it, per the fleet no-delete rule; it should be removed.
4. The 64 norm en1990/en1991 leaves need catalog `$id`s (norm owner), then a `schema generate` (W2-S).
5. The os flow `replace-flow-host-snapshot` needs labels for its three nested inputs, or `widget: hidden` on `hostSnapshot`.
6. The wgpu color control and the time-travel color control (routed by main).

### 6.11 Files (follow-up)

- Framework:
  - `🛂️manifest/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🧬️schema/🟦️.ts,🔣️input-labels.json,🤖️generated/🪪️manifest/🟦️.ts}`
  - `🛂️manifest/🧫️fixtures/🧫️mutation-inputs/🔣️.json`
  - `🛂️manifest/🧪️tests/{🧪️mutation-inputs/{🦀️.rs,🟦️.ts,🐍️.py},🔬️app-label/🦀️.rs}`
  - `🧬️schema/📽️projection/🦀️.rs`
  - `🌱️value/🔁️codec/🧭️path/🧬️schema/🔣️.json`
- OS:
  - `🔌️plugin/⏪️time-travel/🦀️.rs`
  - `📺️renderer/🧑‍🎨engine/🧱️elements/{🛠️ShellHelpers/{🟦️.tsx,🧪️tests/🧪️staged-arg-controls/🟦️.tsx},🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs,🐚️Shell/🧪️tests/🧪️wgpu-time-travel/🦀️.rs}`
- Repo: `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`
- Plugins:
  - stdio `📇️registry/🧬️schema/🔣️.json` and `📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🟦️.test.ts`
  - the 7 `🩹️patch-snapshot` leaves; the wav `🩹️patch-data` leaf
  - pdf `🔺️diff/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`
  - the 120 glTF mutation leaves
  - block `👁️set-brush-preview` leaf
  - procedural viewer `📷️set-preview-camera` (config, presence)
  - draw `✏️update-path-geometry` leaf and the drawing aggregate `🧬️mutations/🔣️.json`
- Scratch, in `🗑️generated/w1d/`:
  - `patch_*.py`, `move_snapshot_patch.py`, `strip_x_semio_leaf_keys.py`
  - `recursive_leaves.ts`, `audit_probe.ts`, `walk_probe.ts`, `strict_gltf_leaves.ts`
  - `census-final.{tsv,json}`, `tsc-os-*.txt`

## 7. Coordinator decisions (os.store split, runtime registry, legacy removal)

### 7.1 os.store: one scope per `🧬️schema` dir

- New document ids:

  | Directory | New `$id` |
  |---|---|
  | `🏪️store/🪆️child/🧬️schema` | `os/store/child/schema.json` |
  | `🏪️store/🪆️child/🏠️owner/🧬️schema` | `os/store/child/owner/schema.json` |
  | `🏪️store/🔗️link/🧬️schema` | `os/store/link/schema.json` |
  | `🏪️store/📦️blob/🧬️schema` | `os/store/blob/schema.json` |

  The store root keeps `os/store/schema.json`.
- The refs in 67 JSON documents are rewritten, e.g. plugin roots, snapshots and diffs, kit and wfc leaves, the store documents themselves and the composition closure. Script: `🗑️generated/w1d/split_store_ids.py`.
- No other code or text used the old ids, apart from two doc comments, which I updated.
- Runtime, in `🏪️store/🦀️.rs` `register_store_schema_exports()`: `STORE_SCHEMA_EXPORTS` is now 4 `ScopeSchemaExports`, `os.store.{child, child.owner, link, blob}`. Each has one whole-document `schema` export, which is the `framework.io` convention.
  - The owner document is registered for the first time.
  - W1-G got a heads-up naming the function.
- The two kit leaves that now resolve `LinkPin` needed a label for `pin`, so I added `Version Pin` / `Versionsfixierung` with a description. This was required by the fix itself.

### 7.2 `s.stdio.registry` at runtime

- The framework has a new plugin-level declaration, `PluginBuilder::schema_documents(ScopeSchemaExports)`.
  - It carries the shared documents of a plugin submodule (`s.<plugin>.<submodule>`) that no single artifact owns. Artifact-level `schema_documents` must equal the artifact's own scope, and identity claims are unique across artifacts.
  - The owning plugin must be the builder itself or a direct dependency. Otherwise the result is `plugin-assembly.schema-documents-owner`.
  - The runtime registry gets `share_schema_documents` and commits them in `publish_declared_catalogs`.
- The contract crate gains `STDIO_REGISTRY_SCHEMA_DOCUMENTS`: scope `s.stdio.registry`, export `schema`, the registry JSON.
  - It is declared by `stdio` (csv, json) and by its hosts `stdio-image` (png, jpg, tiff) and `stdio-media` (mp4, wav).
  - Those two crates gain the `semio-s-artifact-stdio-contract` dependency.
- New test: `registry::tests::the_runtime_registry_resolves_every_patch_snapshot_leaf_after_assembly` in `📇️registry/🧪️tests/🔬️unit/🦀️.rs`.
  - After the stdio assembly, `mutation_input_defs` over `registered_input_schema_document` (the time-travel editor's resolver) reads all 7 `patch-snapshot` leaves to a `patch` object with `edits`.
  - It also reads the kit `bind-representation` `pin` through `os.store.link`.

### 7.3 Legacy removal

- Deleted `📇️registry/🧬️contract/✏️editing/🩹️patch/🧬️schema/🔣️.json` and its now-empty directory, as approved.
- Nothing references it any more.

### 7.4 Verification

- `cargo check` is clean for:
  - `semio-framework-plugin`
  - `semio-s-artifact-stdio-contract`
  - `semio-s-plugin-stdio`
  - `semio-s-plugin-stdio-image`
  - `semio-s-plugin-stdio-media`
- The store crate compiled as a dependency.
- The stdio registry tests (`--features component-app-assembly`) pass 5/5 (1 ignored), including the new runtime test with its kit `pin` assertion. The first rerun waited out a peer's in-progress `semio-framework-ui-contract` edit.
- Lint probe (`🗑️generated/w1d/store_split_probe.ts`): with the four store directories indexed as the next catalogue refresh will index them, the kit `pin` and wfc `media/child` leaves audit clean.
- Census (on-disk, no `schema generate`): the 3 kit and 2 wfc `refUnresolved` now name the new ids. They are catalogue staleness only, because the four store directories get their own scopes at W2-S's refresh.
- The norm figures are in flux: the parity executor is rewriting norm leaves, and the count was 852 at the last run. They are not attributable to W1-D.
- TS results:
  - mutation-inputs 78/78.
  - stdio patch contract 31/31.
  - The lint tsconfig shows errors only in a peer's in-progress `🖱️ui/🧬️contract/…/🔬️graph/🟦️.ts` (`InputProps.snaps`), none in my files.

### 7.5 Files (section 7)

- Framework:
  - `🔌️plugin/🏗️builder/🦀️.rs` (`schema_documents`)
  - `🔌️plugin/🦀️.rs` (`share_schema_documents`)
  - `🏪️store/🦀️.rs` (`STORE_SCHEMA_EXPORTS`, `register_store_schema_exports`)
  - the four store `🧬️schema/🔣️.json`
  - the composition closure schema
- stdio:
  - `📇️registry/🧬️contract/🦀️.rs` (`STDIO_REGISTRY_SCHEMA_DOCUMENTS`)
  - `🔌️plugin/🦀️.rs`
  - `🧩️extensions/{🖼️image,🎵️media}/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml}`
  - `📇️registry/🧪️tests/🔬️unit/🦀️.rs`
  - `🧪️tests/🚢️shipped-fleet/🦀️.rs` (comment)
  - kit `🪢️bind-representation` and `📌change-representation-pin` leaves
  - deleted `📇️registry/🧬️contract/✏️editing/🩹️patch/🧬️schema/🔣️.json`
- Plugins: the 67 JSON documents listed in `🗑️generated/w1d/store-id-files.txt`.
- Scratch: `🗑️generated/w1d/{split_store_ids.py,store_split_probe.ts,patch_plugin_shared_documents.py,store-id-files.txt}`

## 8. Audit fixes (`📓️audit-inputs-leaves.md` §1.1, §2.1(b), §2.2, §2.4, §3.1)

### 8.1 Enum-payload leaves (critical)

- `MutationLeaf` (replication `🎮️mutation`) gains defaulted methods:
  - `input_schema(&self)`, which is `Some(PAYLOAD_SCHEMA)` by default;
  - `input_value(&self)`, which is the whole leaf value by default;
  - `with_input_value(&self, v)`;
  - `from_input_value(v)`.
- `#[derive(MutationLeaf)]` accepts `#[mutation_leaf(contract = ::protocol, payload = Apply)]`.
  - `payload = <Variant>` names the single-field variant whose content is the schema-described, editable payload.
  - That variant answers `Some(schema)`, its content, `Apply(v)` and `Apply(v)` for the four methods.
  - Every other variant (`Restore`) answers `None`: it is not editable, and `with_input_value` refuses.
  - A `payload` that names no single-field enum variant is a compile error.
- `#[derive(Mutations)]` forwards `input_schema`, `payload_value` and `with_payload_value` to the leaf methods.
- It also emits the new `Mutation::from_payload_value(kind, value)`. This constructor, requested by W2-S for `🥒️.feature` witnesses, builds a leaf from its semantic kind and payload alone. The trait default refuses.
- The marker is on all 140 `phase`/`value` leaf enums: glTF 120, svg 9, xml 6, json 5.
- Schemas:
  - svg, xml and json were already flat Apply payloads.
  - W2-S-B1 had already reverted glTF to the flat Apply payload at the root, keeping the whole wire under `$defs`; both glTF lints report 0.
- W2-S mirrors the marker (not the variant name) in the payload-parity lint. W2-S-B1 and W2-S-B2 were told the final shape.
- The time-travel editor becomes correct with no change of its own: `time_travel_editable` checks `input_schema().is_some()`, so `Restore` ops no longer open a dead editor. Apply ops edit exactly the schema-described payload.

### 8.2 Generic payload law (§2.1(b))

- The kernel command module (`📡️spr/🎮️command`), re-exported by spr, gains two functions:
  - `mutation_payload_round_trip_failures<P, M>(ops)`: for every editable op, `with_payload_value(payload_value()) == op` and `from_payload_value(descriptor().semantic_kind, payload_value()) == op`, compared by wire value.
  - `mutation_fixture_ops<M>(root)`: every decodable committed `🦠️mutation/🔣️.json` under a root. It also accepts the `{mutation, before, after}` records.
- `#[derive(Mutations)]` emits `#[cfg(test)] fn semio_payload_law_<aggregate>()` for every non-generic aggregate, so every plugin crate runs it with no hand wiring.
  - It covers the committed fixtures under the aggregate's owner directory (the one that holds its `🧬️schema`).
  - It covers `demo_mutation_cases()` when the aggregate's own file defines one at top level. This is the fixture-independent witness for svg, xml and json.
- Verified green:
  - kernel: 14 tests, including new law and constructor tests;
  - svg 3, xml 2, json 2, glTF 1, puzzle-2d 1, png 1;
  - plugin: 38 tests (laws, time-travel, roster);
  - derive crate: 17.

### 8.3 `nullable` (§2.2)

- `ActionArgDef.nullable: bool` (serde and value default, left off the wire when false) exists in Rust, the projection and the generated TS.
- Both readers set it when the input's own node declares `type: [T, "null"]`, or when a `null` branch was stripped from a two-branch union.
- `json_schema()` emits `anyOf: [T, {type: null}]`.
- Corpus:
  - new case `nullable-versus-optional`: required+nullable, a nullable boolean, a nullable enum, and a plain optional;
  - the `count` and `tint` expectations gain `nullable`.
- Both oracles gain a `:null` verdict per input: admitted exactly when the descriptor says `nullable`.
- Hosts:
  - React `renderStagedArgControl` wraps a nullable control with a pressed "Clear"/"Leeren" toggle (`ui.nullableInput.clear`, en/de). It stages `null`, and pressing it again un-stages. New vitest case; the suite is 12/12.
  - wgpu staged rows add a "Clear <label>" child button that stages `null`; it is disabled when the value is already cleared.
  - `time_travel_coerce` accepts `null` for a nullable input.
  - W2-A's history panel still has to show the clear affordance; routed via main.

### 8.4 Host roster inputs (§2.4)

- `HostMutationRosterEntry.inputs: HostMutationInputs` is a mirror of the guest `WireMutationInputs` (`Declared{inputs}`/`Refused{error}`/`Opaque`, tag `status`). It reuses `semio_framework::ActionArgDef`/`InputSchemaError`.
- The unused serde derives on the host entry are dropped; it only ever decodes the guest wire through `FromValue`.
- `🏃️run` builds its descriptor rows with `Opaque`, which is correct: they are contributed plans, and the descriptor publishes no payload schema.
- New host test: a row carries `Declared` inputs across the wire (`status: declared`); an opaque row reads `status: opaque`. Host tests are 13/13.

### 8.5 Glossary German (§3.1)

- Fixed:
  - `tag`: Schlagwort
  - `before`: Insert Before / Einfügen vor
  - `accessor`: Datenzugriff
  - `scale`/`newScale`: Skalierung / Neue Skalierung
  - `newX`/`newY`: Neue X-/Y-Position
  - `section`: Abschnitt. It is domain-neutral; en1993 leaves should annotate "Querschnitt".
  - `caseId`: Case ID / Fall-ID
  - `turn`/`newTurn`: Wendung / Neue Wendung
  - `gap`/`newGap`: Lücke / Neue Lücke
  - `item`: Listeneintrag
  - `chunk`: Datenblock
  - `at`: Insert At / Einfügestelle
  - `childId`/`child_id`: Kindelement-ID
  - `primitive`: Grundelement
  - `fromPort`/`toPort` en: Source Port / Target Port
- `frame` stays "Rahmen", its domain-neutral sense; animation leaves annotate "Einzelbild".
- New law in Rust and TS: two glossary names may share a German label only if they share the English one. They are then aliases, such as `enabled`/`newEnabled` or `node_id`/`nodeId`. Otherwise one form would show two identical labels. It passes.

### 8.6 Verification

- Rust:
  - `semio-framework` `mutation_inputs_tests` + typegen: 10/10.
  - `semio-framework-os-kernel`, the derive crate and plugin-host tests: as above.
  - `cargo check`: `semio-s-plugin-stdio` for `wasm32-wasip2`; framework-plugin, os-run and os-renderer-wgpu with `--lib --tests`.
- TS: mutation-inputs 80/80; React staged-arg-controls 12/12.
- Python: 200 verdicts over 36 cases, plus 13 reference ids ✔.
- Hub: `cargo check -p semio-hub` fails only in its own bin (`🏗️bootstrap/🦀️.rs:834`, `:2411`): the peer's new `PresencePeer.history_edit` field.
- Operations: I found and broke a fleet-wide fine-grain-locking deadlock (9 idle cargos, 0 rustc, `prebuild_lock_exclusive` in `sample`) by killing the stuck set one pid at a time. Main was told.

### 8.7 Files (section 8)

- Framework:
  - `📡️replication/🎮️mutation/🦀️.rs` (MutationLeaf and Mutation methods)
  - `🗣️dsl/✨️derive/🦀️.rs` (payload marker, `from_payload_value`, emitted law, `mutation_relative_path`)
  - `📡️spr/🎮️command/{🦀️.rs,🧪️tests/🧪️mutation-payload/🦀️.rs}` and `📡️spr/🦀️.rs` (re-exports)
  - `🛂️manifest/{🦀️.rs,🟦️.ts,🔣️input-labels.json,🤖️generated/🪪️manifest/🟦️.ts,🧫️fixtures/🧫️mutation-inputs/🔣️.json,🧪️tests/🧪️mutation-inputs/{🦀️.rs,🟦️.ts,🐍️.py}}`
  - `🧬️schema/📽️projection/🦀️.rs`
  - `🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`
- OS:
  - `🔌️plugin/🖥️host/{🦀️.rs,🧪️tests/🔬️artifact-mutation-router/🦀️.rs,🧪️tests/🔬️host-transaction-coordinator/🦀️.rs}`
  - `🏃️run/🦀️.rs`
  - `🔌️plugin/⏪️time-travel/🦀️.rs` (null coerce)
  - `🌉️mcp/🧪️tests/🧱️source-builders/🦀️.rs`
  - `📺️renderer/…/🛠️ShellHelpers/{🟦️.tsx,🧪️tests/🧪️staged-arg-controls/🟦️.tsx}`
  - `📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- Plugins: the 140 leaf `🦀️.rs` files in `🗑️generated/w1d/phase-enum-files.txt` (marker only).
- Scratch: `🗑️generated/w1d/{patch_leaf_payload_derive.py,mark_apply_payload.py,patch_nullable_rs.py,nullable_probe.ts,phase-enum-files.txt}`
