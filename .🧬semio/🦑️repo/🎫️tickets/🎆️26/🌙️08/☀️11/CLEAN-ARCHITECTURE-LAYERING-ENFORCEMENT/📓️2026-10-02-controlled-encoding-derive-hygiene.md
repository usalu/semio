# Controlled Encoding Derive Hygiene

Read-only source review; no compiler/tests executed. Native High reported a caller field control collision; Execution High owns correction.

Actual expansion owner: `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs`. Shared variant_destructure_patterns:698 remains shorthand original field bindings (skip→field:_). It is consumed by ordinary ToValue enum branches at1643/1706/1735 and path machinery. Do not globally replace its binding names without changing every corresponding field access.

Current controlled named enum path:2303 now explicitly maps fields to positional __source_field_{index} aliases. controlled_named_output:2270–2285 must use those same positional accesses consistently for skip predicate, serializer bridge, flatten and ordinary field encoding. Caller field names control/__output/__payload/__wrapper/__tag/__source_field_0/__emit_0 then stay only field labels and cannot shadow generated controller/output temporaries. Named struct encoding uses &self.field and has no caller-field local binding; single tuple/newtype uses &self.0, unit enum has no field binding.

Concrete remaining hygiene obligation: generated to_value_controlled signature:1801 and payload closures around2310 use unqualified Result. A valid caller-defined type or generic parameter named Result can shadow core Result, producing incorrect generated type resolution. Use fully qualified ::core::result::Result in generated control methods/closures. Source review does not establish that unqualified Vec/String/format in other existing generated methods are collision-free; scan each generated expression boundary before claiming full derive hygiene.

Shared ordinary serialization/path helpers still bind caller field identifiers. A named enum field __entries/__segment/__rest/__len can collide with generated local names in ordinary/path branches around1012–1068. This is distinct from the controlled-only field-control correction. Preserve current ordinary output/path parity with hostile fields; do not change shared destructuring as an incidental fix.

## Existing Owner Route And Corpus

Registered project `@semio-tech/value-rs`, package project target test-controlled-encoding calls its permanent script. Script:24 runs Cargo semio-framework-value --lib filtered controlled_value_encoding_. Existing runtime owner is `🔁️codec/🧪️tests/🛫️controlled/🦀️.rs`; fixture `🔁️codec/🧫️fixtures/🛫️controlled/🔣️.json`; schema `🔁️codec/🧬️schema/🛫️controlled/🔣️.json`. Current test controlled_value_encoding_derived_representations_match_independent_serde compares actual controlled DslValue to independent serde_json and verifies fixture output presence. Extend exact per-case identity expectations rather than only any matching representation.

## Small Closed Native/Serde Matrix

- Named struct hostile fields control/__output: ordinary and controlled output equal Serde; actual custom/skip/flatten paths represented.
- Transparent named struct hostile field control and single tuple newtype: same parity, progress/cancellation preserved.
- Unit enum, single tuple enum and named enum under external/adjacent/internal tagging: named hostile fields control/__output/__payload/__wrapper/__tag/__source_field_0/__emit_0; ordinary/controlled/Serde parity, retained fixture output IDs.
- Same named enum with skip_serializing_if, flatten and explicit controlled serializer bridge: exact bridge calls and canceled result/error behavior; no ordinary serializer execution through missing controlled bridge.
- Caller local type Result or generic Result: actual derive/native compile success and output parity after qualified generated return types. Include a private caller macro format if testing generated macro qualification, with independent semantic expectation.
- Ordinary value_at_path/value_shape_at_path/value_key_at_path on hostile __entries/__segment/__rest/__len enum fields: exact root and field observations, not only controlled encode proof.

Current controlled_to_body:2294–2304 refuses unit structs, multi-field tuple structs and multi-field tuple variants. This is explicit current shape support, not a hygiene result. Do not claim those shapes supported from unit enum/newtype tests; add closed negative derive diagnostics or implement a separate schema-owned shape feature if required. Union refusal likewise remains explicit.

A field-prefix ban would reject valid callers and is not a hygienic solution. Positional controlled-only bindings plus qualified owned generated APIs give a bounded correction without changing unrelated ordinary methods.
