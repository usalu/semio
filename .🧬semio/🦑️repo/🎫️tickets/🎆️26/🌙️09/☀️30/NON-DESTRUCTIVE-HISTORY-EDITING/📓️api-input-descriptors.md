# 📓️ Input Descriptor API (W1-D, compiled and tested)

1. Kernel `protocol::Mutation<P>` (`📡️replication/🎮️mutation`; defaults; `#[derive(Mutations)]` generates per-variant impls):
   - `const INPUT_SCHEMAS: &'static [&'static str]` (row-aligned with `DESCRIPTORS` / `SemanticMutation::kinds()`)
   - `fn input_schema(&self) -> Option<&'static str>` (leaf `MutationLeaf::PAYLOAD_SCHEMA`; `None` = not editable)
   - `fn payload_value(&self) -> DslValue` (derive: leaf payload without aggregate tag/wrapper)
   - `fn with_payload_value(&self, value: DslValue) -> Result<Self, ValueError>` (derive: same variant only)
   - `MutationLeaf::PAYLOAD_SCHEMA: &'static str` (derive `include_str!`s `payloadSchema` relative to the descriptor)
2. Reader (`semio_framework` root, `🛂️manifest` region `🔖️MutationInputs`):
   - `trait InputSchemaResolver { fn resolve(&self, id: &str) -> Option<DslValue>; }` (blanket impl for `Fn(&str) -> Option<DslValue>`)
   - `mutation_input_defs(schema_json: &str, resolver: &dyn InputSchemaResolver) -> Result<Vec<ActionArgDef>, InputSchemaError>`
   - `registered_input_schema_document(id: &str) -> Option<DslValue>` (resolver over the `schema://` export registry)
   - `input_label_glossary() -> &'static BTreeMap<String, LocalizedLabel>`
   - `InputSchemaError { code: InputSchemaErrorCode, pointer: String, detail: String }`; codes `Malformed, RefUnresolved, UiInvalid, WidgetIncompatible, LabelMissing, OptionLabelMissing, LocaleMissing` (wire camelCase)
   - TS twin (`🛂️manifest/🟦️.ts`): `mutationInputDefs(schema, resolver)`, `class InputSchemaError{code, pointer}`, `inputLabelGlossary()`, `argControl()`
   - Input ids: single-segment RFC 6901 pointer relative to the parent value (`"/dx"`, nested object fields `"/x"`); `ActionArgDef::key()` decodes it. Root `oneOf`/`anyOf` refused `malformed`; `allOf` merged.
3. Shapes (Rust + generated TS):
   - `ArgSchema::Number { min, min_exclusive, max, max_exclusive, step, integer, unit, snaps: Vec<f64>, snap_source: Option<SnapSource>, soft_min, soft_max, precision: Option<u32>, display_unit, display_factor, scale: Option<NumberScale> }`
   - `ArgSchema::Vector { dims: u32, unit, step }` (replaces `Vec3`), `ArgSchema::Reference { kinds: Vec<String>, domain, granularity, many, min_items, max_items }`
   - `SnapSource { Step, Config { key }, Snapshot { pointer } }`, `NumberScale { Linear, Log }`, `ArgPresentation` += `Stepper, Dial, Segmented`
   - `ActionArgDef { id, label, schema, presentation, required, default, description: Option<LocalizedLabel>, group: Option<String>, order: Option<i64> }`; `json_schema(&self, terminology, locale)`; step is UI-only
   - `ActionArgControl`: `Number | Stepper | Slider | Dial | Segmented | Vector | Reference | Text | Toggle | Select | IconSelect | ArtifactKind | SurfaceApp`; `control()`: presentation wins, else integer → Stepper, else both bounds → Slider, else Number
4. Roster: `WireMutationRosterEntry.inputs: WireMutationInputs` (tag `status`: `Declared{inputs}` | `Refused{error}` | `Opaque`).
5. Glossary `🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json` (258 names, en + de); meta-schema `$defs/InputUi` in the manifest schema, registered as `x-semio-ui`.
6. Candidate validation: `semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema_json, &[doc_json…])?.validate_json(&instance_json)`; `semio_framework::mutation_input_instance(schema_json: &str, resolver: &dyn InputSchemaResolver, payload: &DslValue) -> Result<DslValue, InputSchemaError>` / TS `mutationInputInstance(schema, resolver, payload)` splice every root `const` (incl. allOf members) into a `payload_value()` before validation (landed, tested).
7. Open: every plugin descriptor (`✏️s/🔌️plugins/*/🔣️.json` + `🛂️.descriptor.semio`) is stale → central `describe` regeneration.
