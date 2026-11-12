# Framework test drift, group A

## State
All 16 group-A crates were migrated to the current retained-work / effect-threaded API (tests, test support, dev-dependencies). One combined `check --tests --keep-going` (log `🗑️generated/drift-a-all.log`, package list `🗑️generated/drift-a-pkgs.txt`) at 13:17 left only 3 crates failing; their fixes were applied afterwards and are NOT yet re-checked (build pause). No tests have been run yet.

## Per crate
- async-tests: compiled clean. Added effect helper `close_step(grant,&mut dyn RetirementCursor)` in `⏳️async/🧪️testing/🪙️effect` (drive_reply based), admit_root `.result`, testkit accessors `current()/node()/into_cursor()` on `CancelTokenRetirement` (+ `CancelTokenRetirementCursor` export), retirement()->retirement_seed().
- pack-intrinsic, pack-record-encode-cursor, pack-record-preflight, replication, replication-causal, replication-codec (bytes tests via EffectBank helpers), replication-command-ingress, replication-edit, replication-local-interaction, replication-error, io-binary-source, io-control-testing (local CloseWork/with_effect duplicate and 1<<20 ceiling removed), compiled clean at 13:17. See sub-reports: framework-test-drift-a-{seq,pack,edit,io,wire}.md.
- replication-retained-key: two E0596 (`&mut lookup` on a `&mut` binding) fixed in `close`; unverified.
- replication-wire-core: `🎟️work` tests lacked `warm`/`close_step`; imported from `super::tests` (now pub(crate)); unverified.
- diagnostic: E0432 `crate::fault_wire_cursor_tests` unresolved in `🏭️message/🧪️tests/🦀️.rs:4`. The module exists (cfg(test)) in the lib root; cause not found, needs a rebuild to read the full error.

## Open
- Recheck (retained-key, wire-core, diagnostic) and then `test` of all 16 crates once builds resume.
- Seq report notes `RetainedDiffSequence` has no `preflight_work_demand`/`close_work_demand` quotes; the test's SequenceApply returns a ceiling and 1 (not framework quotes) and one projection-count assertion was dropped (no public OwnedFieldProjection constructor). Needs a library quote or a follow-up.
- Pack agent edited shared rig `🌱️value/📝️text/📦️paged/🧪️tests/encoder-rig.rs:60` (witness->identity).
