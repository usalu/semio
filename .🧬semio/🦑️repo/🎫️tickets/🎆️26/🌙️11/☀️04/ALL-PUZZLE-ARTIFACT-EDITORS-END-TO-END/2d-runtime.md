# Puzzle 2d Runtime

Owned the 2d snapshot import boundary. The old admission predicate accepted an explicit foreign schema, null schema, numeric schema, or malformed nodes whenever the other branch of its schema-or-nodes predicate passed. Extended the existing language-neutral payload input corpus before changing admission. A direct evaluation of the original predicate reproduced four new rejecting vectors as incorrectly accepted. Admission now validates any explicit schema and any present nodes array, preserving schema-free arrays and schema-only empty boards.

The existing native snapshot_import_requires_the_explicit_payload_contract law consumes the expanded corpus and compares every accepted native projection against serde_json as a third-party JSON oracle. A native suite is queued with repo run @semio-tech/puzzle-2d-rs:test --lang=en -- --lib; it has not yet run because repository capacity is saturated by other agents. No runtime test success is claimed.

Files changed:
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json

The corpus now includes its language-neutral admission JSON Schema. The retained 2d-import-oracle.cjs ticket input compiles that schema with the existing third-party Ajv installation; all 14 vectors pass. A mounted runtime law invalid_snapshot_import_keeps_the_live_board_and_its_history now dispatches malformed object imports after creating a node, requires a visible notice, prohibits LoadDocument and edits, checks the live snapshot remains identical, and undoes the seed afterward to prove the history stays usable. Native execution is still pending.
