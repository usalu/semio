STATUS: PARTIAL. VERIFIED: semio-os-io native lib (W-fixtures-105 rc 0) and semio-os-store-durable-group-wire native lib (W-fixtures-103, Checking + warnings only). UNVERIFIED: the three test crates (W-fixtures-106 refused at job-context source admission, since cleared per coordinator 23:17).
(Older text below is the first-run state.) No verdict yet for semio-plugin-testing-fixtures, semio-os-store-testing-doubles, semio-plugin-testing-laws (artifact-app-laws unreached too). W-fixtures-101 rc 1, log 🗑️generated/1105b/W-fixtures-101.txt.

## Manifest and command (for pane lanes)
The three crates are NOT members of the hub workspace (🌎️hub/Cargo.toml) and not of the 🧰️framework workspace. They are members of the REPO ROOT workspace `Cargo.toml` (lines 159 doubles, 401 plugin-testing, 403 fixtures). Package names: semio-plugin-testing-fixtures, semio-os-store-testing-doubles, semio-plugin-testing-laws.
Command (native, no wasm: test crates):
  C/1105b-tools/chk.sh W-fixtures-<n> Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws
(chk.sh runs from /Users/ueli/Documents/semio; the manifest is the root `Cargo.toml`.) The root workspace resolves with --keep-going; cold run took ~24 min (queue ~1 min, planning, ~1400 s check).

## W-fixtures-101 verdict
Only three crates failed, all upstream of mine (dependents skipped by keep-going, hence no verdict):
- semio-os-store-member-open (36 errors), store/🧩️composition/🚪️open/…/🏭️operation/🛂️receiving/🦀️.rs: store7 (claimed 22:30).
- semio-os-io (32): 🚪️io/🧭️io-mechanism/🦀️.rs (decode_document_archive_head_controlled needs output slot; receive_nested needs 3rd generic; to_sqlite_database_receiving / validate_sqlite_snapshot_subset_decoding take a new owner slot; RetainedWorkOutcome needs .result) plus 🚪️io/🧰️testing/🦀️.rs:35 (close_retirement_recipient now takes &mut RetainedAllocationEffect, not a grant). UNCLAIMED.
- semio-os-store-durable-group-wire (25): lifted/🗄️durable-group/🏗️staged-root-retirement-and-commit-phases, planning/🪪️decision, 🧷️member, 🎒️pack, outcome-and-journal-receipt-types; includes E0050 recover/receive missing a parameter vs NativeAllocationAuthorityPort and E0046 missing source_turn. UNCLAIMED.
Green by absence (not in the failed list): snapshot-clone, one-item-publication, text-io, replay, presence, codec-registry.
store6's one-item-batch, a dependency of testing-doubles, was not reported failing: skipped behind member-open or green; the rerun tells.

## Ported
Nothing: I cannot port my crates blind behind these reds and did not start the unclaimed os-io/durable-group-wire ports (store-protocol work, store owners). Rerun after those three are green; then port the errors in the three test crates, including store4's testing-doubles retained edit and panede's artifact-app-laws edits.

## Final state (wrap-up)
Ported and compiled:
- semio-os-io io-mechanism: typed export/import and snapshot hops rewritten onto receive_nested output-slot form (native_snapshot_turn meter, decode_move/encode_move frame effects, staged workspaces per stage, manual RetireOwned for the two typed frames, settled_failure), attach_metadata_decoding onto receive with effect quotes, native_carrier onto the head-decode slot; io testing fixture closes through with_work_recipient + with_effect. Green: W-fixtures-105.
- semio-os-store-durable-group-wire: close(effect) in planning pack/member/decision/bound; native.with_effect for advance-side steps; close_journal_owner(effect), close_assembly_publication(publication, effect); staged root uses factory_retire_snapshot and admit_owned_retirement. Green: W-fixtures-103.
- codec-registry sqlite_snapshot_retirement::settle made pub (additive).
Known fallout, UNPORTED (surfaces when the closure compiles): durable-group parent crate callers still grant-shaped: map-assembly-open advance 201-211 (close_assembly_publication; use receiving.with_effect), retire_originals_step and recovery-completion 301-302 (planning close), commit-steps 324 close_journal_owner and decision/output_retirement step(grant). Parents have `receiving`/`native` in scope except commit-steps advance(grant) which needs an effect from its caller.
Not done: own errors of semio-plugin-testing-fixtures, semio-os-store-testing-doubles (store4 retained edit), semio-plugin-testing-laws (panede edits): never reached by the compiler.
Rerun: C/1105b-tools/chk.sh W-fixtures-107 Cargo.toml native --tests -p semio-plugin-testing-fixtures -p semio-os-store-testing-doubles -p semio-plugin-testing-laws   (from /Users/ueli/Documents/semio; root Cargo.toml is the manifest). No [DEBUG] lines added.
