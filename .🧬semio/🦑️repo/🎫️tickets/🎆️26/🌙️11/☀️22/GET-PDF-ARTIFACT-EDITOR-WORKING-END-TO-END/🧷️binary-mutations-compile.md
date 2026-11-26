# Binary mutations crate compile break

Crate `semio-s-artifact-stdio-binary-mutations` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`).

## Root cause
Another session reshaped the replication/editor API; the crate's unit tests were not updated (lib itself compiled):
- `ColdMutation` (`diff`, `inverse`), `ColdDiffAlgebra::inverse`, `ColdMutationDiff::absorb`, `OpText`, `OpBinary` are no longer reachable without an explicit `protocol::` import.
- `ArtifactEditor::command_from_action` now takes `&mut Option<DslValue>` and `&mut Option<Fault>`.
- A first check also failed in `semio-plugin-instance-typed-ops` (`ArtifactBoundedToolFault` undeclared); another session fixed that in parallel (now `borrowed_report`).

## Changes
- `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: one import of the protocol traits.
- `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`: import `ColdDiffAlgebra`, `ColdMutationDiff`.
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `command_from_action(.., &mut .., &mut None)` calls; fixture root of `history_edit_acceptance_law!` fixed to `"../../../.."` (was a path under the mutations dir that held 0 cases).
- `🚪️io/📝️text/🔺️diff/📖️.grammar.semio`: `splice-item` now braced (`{ offset=.. remove-len=.. insert=".." }`), matching the printer's keyed-record-list shape; fixes `diff_grammar_conformance_law`.

## Results (via native-cargo gate)
- `check --tests`: green.
- `test -p semio-s-artifact-stdio-binary-mutations`: 48 passed, 2 failed.
  Remaining failures are in the shared history-edit acceptance harness, not compile: `history_edits_end_to_end` ("original report requires its installed semantic and retirement issuers") and `conflict_history_edits_end_to_end` ("archive hydration ... DecodeForward: Replay: truncated at offset 16"). Previously masked by the wrong fixture root.
- `test -p semio-hub-stdio-pdf --lib`: compiles, 0 tests, ok.

## Follow-up: history-edit harness failures (root causes, fixed)
Result: `semio-s-artifact-stdio-binary-mutations` 50 passed, 0 failed.

The binary artifact never declared the store/wire owners the history machinery requires (the earlier fixture-root fix only unmasked this). Four independent causes:
1. `🧬️schema/🧬️mutations/🦀️.rs`: the vocabulary has an authored tagged `OpBinary`, but was registered with the Pack-Value receiver (`original_pack_value_operation_receiving` + `artifact_persisted_binary_operation`), so persisted bytes were read in another format ("Replay: truncated at offset 16"). Now `original_decoded_operation_receiving` + `artifact_persisted_encoded_binary_operation`, as the other authored-codec vocabularies (model, writer, sequence).
2. `✏️editor/🦀️.rs`: `BinaryEditor` did not install the cold-replay store owners, so no semantic/retirement issuers existed ("original report requires its installed semantic and retirement issuers"). Added `document_store_owners_source_demands` and `build_document_store_owners` using `cold_replay_document_store_owners` (same as xlsx/pdf/process3d).
3. Same hook: the operation wire now installs `with_original_encoded_binary_receiving()` and `with_original_target(whole_document_target)` (a byte-range edit conflicts with the one opaque buffer), fixing "no original receiving capability" and "no authored conflict path issuer".
4. `🧰️framework/.../🗃️document/🗄️document-archive-load/🦀️.rs`: the archive admission rejected an empty `parent_pack`. The raw binary carrier law makes an empty buffer an empty pack, so a zero-byte document could not be reloaded. The check now requires only the history (`parent_spr`), which always holds the envelope.

## run-s10-final.sh (PDF test group)
- Unit stage (`s10-final`, exit 0): `-p semio-s-artifact-stdio-pdf -io-export -page-editor-tests -testkit -snapshot-2 --lib`: 38, 19, 71, 2 and 66 passed, 0 failed.
- Routes stage (`s10-routes`, exit 101): did not run; `semio-framework-dsl-record-model` fails to compile (`unresolved import semio_framework_dsl_record_ownership::retirement::retire_field`). `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🛫️encode/🦀️.rs` was edited seconds before (it replaces the local `retire_field` by a re-export), but `♻️retirement/🦀️.rs` does not define it yet. This is another session's in-progress move, so it was left untouched; rerun the routes stage once it lands.

## Routes stage rerun (after the `retire_field` move)
`s10-routes` (exit 101): `semio-dsl-record-model` now compiles, but `semio-plugin-guest-contract` fails: `unresolved import crate::original_input` (`🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️turn-output/🦀️.rs:8`, 14 errors). The plugin tree (`🧬️guest-contract`, `⏯️tool-run`, `⏪️time-travel`, `🧵️retained-command`, `⚛️reactor/🎟️context`) has files modified within the last 15 minutes, i.e. another session is mid-introduction of an `original_input` module; left untouched. No route tests ran, so there are no pass counts yet. Rerun once that module lands.
