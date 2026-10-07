# Complete Pack Caller and Intrinsic Object Ownership

The direct neutral replacement owner is General package semio-framework-pack, Rust crate pack, complete schema-record namespace pack::record. Its pack::value namespace is the intrinsic protocol value, so replacement must select record explicitly. Kernel Cargo already declares semio-framework-pack; each other package still requires its own exact dependency/import join. This is a source census, not execution or deletion admission.

## Concrete OS Store Route

Physical owner: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs. The pack_rt facade at 6468 forwards complete encode_document/decode_document and controlled equivalents to crate::os_pack; these carry RecordSpec/RecordValue and complete document framing. At 6530 the value_bridge_spec authors exactly field 1, value, Shape::Value, RecordLayout::Lines. encode_pack_value at 6535 puts the original cloned DslValue in FieldValue::Value and invokes complete encode_document. decode_pack_value at 6543 decodes the complete file and extracts that field. The external JSON bridge at 6591 delegates through this owned DslValue bridge; it does not own a replacement codec.

The separate encode_wire_value at 6552 uses encode_record_body; exact controlled wire decode at 6565 and controlled/into/span APIs at 6570–6587 are explicitly containerless. They require direct General record body APIs and must not be replaced by complete-document APIs or vice versa. Store-specific OperationByteOutput and source ownership must be joined to the existing neutral General sink/source interface before retiring a facade; blind textual alias replacement is insufficient.

## Real Shape::Value Object Producer

The space-history SQLite native owner is 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs. Its spec at 10–13 authors field 1 value Shape::Value with controlled schema allocation. object at 18–23 preserves supplied fields in array order. project at 28–77 constructs authors id/name/avatar, timestamp actor/physical_ms/logical, checkpoint id/parentId/message/authors/timestamp/members, and root checkpoints/alternatives/activeAlternativeId in authored order. These are actual production Object payloads with deliberately nonlexical sequences, not only sorted unit samples. The declared schema specifies Value, not a lexical object-map order. Its projection retirement at 79–81 uses General IntrinsicRetirement.

## Unit Presort Does Not Author the Schema

OS value unit owner 🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs:105–109 explicitly presorts both Vec-backed Map and Object to accommodate the encoder’s sorting and order-sensitive PartialEq. It describes observed implementation and arranges a roundtrip input; it does not exercise unsorted Object occurrences, duplicate keys or an authored schema sorting obligation. The production projection above proves nonlexical Object construction exists.

The OS controlled encoder sorts intrinsic Object unconditionally, including when EncodeOptions.canonical is false. General retains Object occurrence order and duplicate occurrences, while schema Map remains key-sorted. General’s existing ordering fixture/law is the explicit authority for this separation; changing all General Objects to lexical order would lose authored occurrence semantics. The prior complete private/export and canonical-order reports identify exact differing algorithm bodies and fixtures.

## Retirement Boundary

Replace concrete complete-document callers with General pack::record complete-document functions using their current RecordSpec, options, reports and controls. Keep authored typed projection and caller-owned container envelopes. Replace containerless wire callers with the corresponding exact General record-body APIs. Root’s Kernel local OS value mount, OS component ten codec forwarders and Store pack_rt forwarding surface can be retired only after full caller/dependency/type/source ownership and actual whole controlled outcomes are sealed. No compatibility pub use, generic JSON fallback, body framing substitution or blanket global renaming is proposed.

## Retained Current Textual Census

The exact current direct-document symbol census is retained under generated/pack-complete-document-direct-caller-census-2.txt: 513 textual match rows across 224 physical Rust files, SHA256 ea6baa5f68c4cd2fde835cf4d830933d235caad651227445cdbfc29c1d60eb35. This includes inline tests, module tests and production; these counts are not runtime caller counts and do not prove exhaustive AST ownership classification. Source-specific production joins above remain the authoritative finite findings.
