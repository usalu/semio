# Literal Object Versus Schema Map New Law Authority

Read-only current law/source authority. No execution.

New unit object_keys_encode_in_canonical_key_byte_order uses Shape::Value and FieldValue::Value(DslValue::Object), then asserts sorted/reordered literal inputs encode identically in ordinary and controlled helpers. This conflicts with actual DslValue::Object(Vec<(String,DslValue)>) occurrence semantics and existing neutral intrinsic law98, which roundtrips exact ordered duplicate keys and complete scalar words through both actual helpers. Reordering literal Object changes semantic value; it is not a canonical map normalization.

The legitimate canonical key-order contract belongs to schema-owned FieldValue::Map/Shape::Map. Ordinary encode_map509 and controlled branch92 sort key bytes; controlled retains paid index frontier, actual checkpoints and original index tie order while omitting Absent. Retarget new law to nested schema Map values, preserving both ordinary/controlled byte equality assertions and nested coverage. Controlled encoding should call encode_record_body_controlled with the actual schema, since encode_value_record_body_controlled intentionally handles intrinsic Value and cannot establish schema Map behavior.

Retain separate literal Object assertions for actual original occurrence roundtrip and ordinary/controlled parity; existing intrinsic fixture already establishes duplicate/literal semantics. Add or retain rejection of the incorrect equality by asserting differently ordered intrinsic inputs produce distinct wire/decoded ordered values. No assertion should be deleted/excluded to obtain green. TS object/JSON projection sorting is a separate projection contract and cannot override native ordered occurrence authority.
