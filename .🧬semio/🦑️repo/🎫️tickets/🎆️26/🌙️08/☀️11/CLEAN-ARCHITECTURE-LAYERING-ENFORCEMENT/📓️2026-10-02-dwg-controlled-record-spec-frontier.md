# DWG Controlled Record Spec Frontier

Read-only actual-source audit of Hub21 retained E0308 receipt; no compiler or tests run. All eleven factories are in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.

| Factory / source line | Shape::Record site | Fields | Enum labels | Nested ordinary shape calls |
|---|---:|---:|---:|---:|
| dwg_xrecord_value_spec108 |144|9|11|0|
| dwg_table_control_entry_spec333 |339|2|0|0|
| table_control_body_spec430 |460|5|9|4|
| dwg_complex_color_value_spec553 |573|5|9|4|
| table_record_body_spec895 |918|8|7|7|
| dwg_evaluation_variant_spec1535 |1541|2|1|1|
| dwg_evaluation_expression_value_spec1595 |1618|8|8|7|
| dwg_visual_style_property_spec1752 |1758|2|0|2|
| dwg_constraint_node_spec2785 |2822|7|14|6|
| dwg_entity_body_spec2885 |2940|20|19|19|
| dwg_logical_object_body_spec3102 |3205|44|43|43|

Counts are source census, not allocation measurements. All factories currently use eager vec!/FieldSpec::new/String::into and ordinary nested DslField::shape. This file has no existing controlled schema factory/shape hook for these handwritten implementations. The generic visual-style factory additionally requires T::shape_controlled, never T::shape in its controlled branch.

Canonical owner `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🏭️producer/🦀️.rs:6–16` exposes NativeSchemaControl checkpoint/begin_stage/step/advance/charge/copy_text/allocate_vec/scoped_stage/scoped_depth/produce. RecordSpecProducer38–41 explicitly requires ordinary, decoding, encoding function pointers. Its decode/encode47–49 already open scoped depth64 and scoped stage with begin_stage(0). Controlled field53 copies key via copy_text; record59 copies optional keyword; boxed64 charges sizeof Shape before allocation.

Implement each authored factory's separate generic controlled construction body, and bind decoding/encoding directly to that body under the supplied controller. Keep ordinary factory only in ordinary field; do not call it then charge its result. Allocate field Vec with allocate_vec<FieldSpec>(exact field count), set known total work, and step/check cancellation at each authored field. Allocate each enum Vec with its exact label count; copy each label through copy_text and step per label. Preserve field IDs, order, optional flags, layouts, discriminants and labels exactly. Every nested field uses <Type as DslField>::shape_controlled(control)?; primitive explicit Shape constants can remain allocation-free with checkpoints. XRecord point_value requires producer::boxed(Shape::Float,control)? for its Tuple wrapper rather than uncharged Box::new. Any child Record remains a lazy producer; do not recursively unfold metadata while building parent Shape.

Also add shape_controlled to each eleven handwritten DslField implementations: checkpoint then return the matching lazy Shape::Record(producer). Without this, DslField default at os_dsl/🦀️.rs46 deliberately returns 'field owner has no controlled native schema implementation', so changing only ordinary Shape::Record constructors would compile yet fail retained native paths. Existing Vec/Map/array controlled wrappers at245/289/334 own child boxes and depth limits; route nested Vec<f64> through that API. Existing newtype producer475 illustrates separate controlled directions rather than an ordinary bridge.

Required retained native proof: ordinary and both controlled metadata match for all eleven factories; generic visual-style at least one actual scalar and one nested record type; cancellation at field/enum-loop frontiers; metadata-byte ceiling before key/label/box/Vec allocation; depth refusal and exact original serialization/decode corpus. Current Hub compiler receipt proves the type failures only, not these runtime obligations. Root owns DWG edits and NativeHigh owns SQLite caller bindings; no blanket fn-pointer conversion is sound.
