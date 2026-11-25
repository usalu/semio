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
