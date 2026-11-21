# Dag Config Ordinal Receiving

The Dag config binary codec now implements `OpBinaryReceiving` through the OS DSL's original ordinal receiver. The same mutation declaration selects `ReplaceConfig` at ordinal 0 and `ChangeCamera` at ordinal 1 for both the existing whole-buffer codec and bounded receiving. The receiver binds the parsed Pack record through the mutation records' owned DSL derivations, including the nested `DagConfig` block for replacement.

The existing language-neutral mutation vectors now pin the full wire bytes of both operations. Python's `struct.pack("<BBd", field_id, 5, value)` independently produced the tagged little-endian payloads. The Rust law compares production encoding to the committed bytes, uses Serde JSON as the typed oracle, and exercises original receiving on each operation embedded in a larger source. The shared law driver grants one item per turn and settles paid retirement after taking the typed output.

The Python reference matched the checked-in vectors and the Dag config diff passed `git diff --check` through `repo`. The focused Rust test `language_neutral_mutations_match_json_oracle_and_restore_base` is queued with the editor verification on the shared native MCP build lane.
