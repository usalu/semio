# Import Wire Bound Audit

Read-only source audit, 2026-11-05. No native execution or source edit.

## Actual Codec Route

EditorApp::original_action_command_producer defaults through declared_action_command_bridge (surface/editor-app:239). Plugin declaration /👷️job/🎮️command/🧬️op-binary/🦀️.rs implements OpBinaryProducer; ENCODE calls actual OpBinary::encode_op, not JSON or printed DSL.

Lowpoly app_commands macro derives DslEnum and its OpBinary delegates dsl::variants_binary::encode_op. OS dsl /🦀️.rs:193-204 encode_with obtains named record/spec, invokes Pack encode_record_body default, then writes format byte1 and ordinal varint. ImportMeshFile ordinal28 fits one byte. There is no enum keyword text in this frame.

Pack encoder framework /🎒️pack/🌱️value/🛫️encode/🦀️.rs:122 writes TAG_STR_INLINE, UTF8 byte-length varint, raw text.as_bytes; selected symbols instead store text once in symbol table and reference an index. Newlines, quotes and control characters do not expand. Record body writes symbol count, selected symbols, field count, field IDs and values. No full document/footer framing is added by encode_record_body.

## Derived Bound

Two strings: name≤4096 bytes and payload≤2097152 bytes. Inline overhead: format+current ordinal2, symbol count1, field count1, two u16 field IDs≤6, two text tags2, name length varint≤2, payload length varint≤4 =>≤18 overhead. Current generated IDs1/2 reduce this to14. Symbol alternative has at most2 strings and stores each once with length plus small tag/index refs; conservative overhead32 covers either case. Thus name+payload+32 is a supported conservative bound for this exact two-String record/current ordinal. Proposed payload+16384 ceiling is safely above encoded worst-case, though the slack is much larger than required. A77KiB geometry fixture alone would not prove this full-cap bound; actual OpBinary maximum-string case is correct codec evidence.

## Blocking Separate Public Producer Ceiling

Current default OpBinaryProducer independently declares RUNTIME_COMMAND_WIRE_MAXIMUM=65536 at op-binary owner:12. ENCODE demand, encoded-length refusal, WIRE birth and input admission all use it; it is not derived from registered tool row contract. Therefore Lowpoly import factory/proof/retained producer2MiB ceiling does not make public >65KiB imports pass yet. Runtime/root informed immediately. Must resolve through a genuine supported original producer seam; do not merely widen unrelated tools or claim factory-only change completes import path.

## Latest Factory Review

Lowpoly now has two concrete factory types, disjoint small IDs and import-only ID. Import publication HostOnly exactly once. One manual proof method qualifies small rows with LowpolyCommandJobFactory and import row with LowpolyImportJobFactory. Registration sequentially registers both; actual TypeIds derive from concrete F. Retained producer selects lowpoly_command_maximum_raw_bytes per command. These layers align with proposed import wire maximum, but default original OpBinaryProducer ceiling above precedes them and remains blocking at inspected source.
