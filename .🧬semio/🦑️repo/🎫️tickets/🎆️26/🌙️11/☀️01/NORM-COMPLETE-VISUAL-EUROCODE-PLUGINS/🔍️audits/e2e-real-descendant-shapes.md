# Real ISO And VDI Descendant Shapes

Source inspection only; no native execution, test or build.

## ISO 16757

Model API: `semio_s_artifact_norm_iso16757_model::Iso16757Snapshot::reference_fixture()`. It authors `part_number_rule = part_5::PartNumberRule::Script { function_id: "partno", source: "dn * 10 + 50" }` and `part_number_inputs["dn"] = CatalogueValue::Decimal { value: 50.0 }`.

Model source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.
Enum source in its `🔄️part-5-exchange/🦀️.rs` declares `#[value(tag="kind",rename_all="camelCase")]` and named Script fields. Therefore the rule value is internally tagged, with source directly addressable:

```json
{"kind":"script","functionId":"partno","source":"dn * 10 + 50"}
```

The exact descendant is `partNumberRule.source`, a String; no Script or payload wrapper appears. Its existing parent edit rule uses kind `replace-part-number-rule`, payload field `newRule`. Aggregate alias is `semio_s_artifact_norm_iso16757_mutations::Iso16757Mutation::ReplacePartNumberRule`; concrete payload is its public `replace_part_number_rule::mutation::ReplacePartNumberRule { new_rule }`. Preserve functionId and kind when changing source.

Change source to `dn * 10 + 150`. With dn=50, independent arithmetic yields 650 rather than 550. The exact existing check is `iso16757.5.6.10.partNumber`; native inference exposes `evaluate(&Iso16757Snapshot) -> CheckReport` in the inference crate. Its checks/🔄️part5 implementation invokes calculate_part_number and records the computed result. This check remains passing, so utilization alone does not witness the changed part number. Use computed quantity/explanation in a native integration assertion. Browser report-value visibility remains a separate prerequisite.

The scalar input dn value is genuinely `partNumberInputs.dn.value`, but there is no parent typed replacement rule for partNumberInputs in the inspected roster. Source is the suitable existing typed-parent descendant witness.

## VDI 3805

Model API: `semio_s_artifact_norm_vdi3805_model::conforming_blatt_3_dataset()`, publicly reexported from snapshot/reference dataset. It authors first product id and identity.article_number both RAD-600, configuration.id cfg.RAD-600, sheet3, Radiator attributes with standard_output_w1200, heat_exponent_n1.3, length_m0.6, height_m0.6, depth_m0.1, connection_type flange.

Source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️reference-fixture/🦀️.rs`.

SheetAttributes in sibling `🧮️sheet-attributes/🦀️.rs` has `#[value(tag="kind",rename_all="camelCase")]`, variant Radiator(RadiatorAttributes). The owned ToValue derive generator at `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🎯️expand-entries/🦀️.rs` explicitly obtains the newtype payload object's entries and inserts the kind tag, yielding:

```json
{"kind":"radiator","standardOutputW":1200,"heatExponentN":1.3,"lengthM":0.6,"heightM":0.6,"depthM":0.1,"connectionType":"flange"}
```

The native controlled decoder independently admits those exact flat keys for radiator; no payload/value wrapper exists. Thus `catalog.products[id=RAD-600].configuration.attributes.heatExponentN` exactly matches FieldMeta and addresses a Number. Set1.4. The check `vdi3805.3.n.RAD-600` compares n to range[1.1,1.5], with max1.5: utilization13/15 becomes14/15 (87%→93% rounded editor display), remaining Pass.

Parent rule kind is `change-product-configuration`, payload field `newConfiguration`, SelectorKey::Id payload id. Aggregate alias is `semio_s_artifact_norm_vdi3805_mutations::Vdi3805Mutation::ChangeProductConfiguration`; concrete public payload is `change_product_configuration::ChangeProductConfiguration { id, new_configuration }`.

Important selector constraint: its diff source `🧬️schema/🧬️mutations/🎛️change-product-configuration/🔺️diff/🦀️.rs` searches `p.identity.article_number == payload.id`, whereas the inspector selector uses product.id. The committed RAD-600 witness sets both equal and is safe. This does not prove generic documents with different IDs are safe. Missing article emits Error mutation.target-missing; equal configurations emit admitted Warning mutation.no-op.

The fixture synchronizes native record210 with typed attributes. The inspected configuration mutation only replaces the typed configuration; its diff does not call sync_typed_attributes_into_records. Additional consistency checks may therefore change after this edit. Assert the exact n check and inspect global report changes independently; do not claim all checks remain unchanged.

## Integration Evidence Boundary

Both proposed leaves have the exact native value shapes advertised by their FieldMeta. Both can be resolved through an existing typed parent replacement after the shared descendant implementation passes. These conclusions are source-derived. No successful domain edit, check recomputation, persistence, browser interaction or undo/redo execution is claimed here.
