# SPR Testkit Fixture Source Findings

The initial absence finding was incorrect: its `🦀️.rs` filename filter excluded the actual `🦀️component.rs` fixture owner. The corrected read-only source is `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`.

- `AddOp` (1116) is the lawful `i64` add fixture. `AddDiff` applies/absorbs deltas and supplies DiffAlgebra; AddOp prints/parses `add <i64>` and inverses with negation. Immediate law callers are the text, apply/inverse, absorb, inverse, and deterministic tests at 1213–1237 and 1322.
- `MissingTargetOp` (1155) deliberately returns a typed error outcome with no inverse. `BuggyMissingTargetOp` (1167) deliberately violates that missing-target law by returning a forward delta and an inverse; callers at 1298/1304 must remain negative-law fixtures.
- `NondeterministicOp` (1179) stores skipped mutable call state and deliberately changes its diff per invocation; caller 1328 is the negative deterministic-law fixture.
- `RejectedForwardOp` (1196) deliberately returns a fatal forward outcome and no inverse; caller 1243 proves the inverse-law helper rejects it.

Any future direct-leaf adoption must isolate only lawful `AddOp` under a direct owner with an explicit i64 payload schema, checked arithmetic/diff policy, exact text and binary codec, and a lawful inverse boundary for `i64::MIN`. The other four types are intentionally unlawful test doubles and must remain separate negative-law fixtures, not be repaired or given fabricated leaf metadata.
