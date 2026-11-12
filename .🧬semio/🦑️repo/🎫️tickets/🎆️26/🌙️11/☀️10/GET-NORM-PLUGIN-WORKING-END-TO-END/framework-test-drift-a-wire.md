# Framework test drift: drift-a wire-core

Status: edits complete, NOT compiled (coordinator throttled cargo; awaiting the combined check in drift-a-all.log).

Crate: semio-framework-replication-wire-core (replication/🧾️wire).

Changes (tests only, plus one visibility tweak):
- Cargo.toml: dev-deps `semio-framework-value-testing-effect-bank` and `semio-framework-value-clone-source-testing` by path.
- codec/🛬️retained/🦀️.rs: `mod tests` became `pub(crate) mod tests` so sibling test modules share one helper set.
- codec/🛬️retained/🧪️tests/🦀️.rs: thread-local `EffectBank`, `warm()`, `outcome()`, `turn()`, `close_step(decoder, grant)`, `close_cursor(grant, cursor)` (drive_reply based). Retirement test now uses `Box::new(decoder.retirement_seed()) as Box<dyn RetirementCursor>` and `close_cursor`.
- Siblings (🧾️source, ⏱️timestamp, 🔣️json-string, 🎟️work): `close_step` calls routed through the helper; `warm()` added per test.
- 📦️bytes tests: local `close_step` and `retire_step` (ControlledRetirement::step under an action granted its quoted `next_computational_work`).
- dictionary decode and 🧾️source tests: local `close_step(&mut dictionary,&source,grant)` via `turn`.

Risks to verify on the next check: heap-observed regions around the bank run (expected (0,0) unaffected), `take_admitted` usage, missing imports (`ValueError`, `RetainedCloneProgress`) in the main tests module, `RetainedCloneStep::progress()` in the bytes test.
No library bugs found so far.
