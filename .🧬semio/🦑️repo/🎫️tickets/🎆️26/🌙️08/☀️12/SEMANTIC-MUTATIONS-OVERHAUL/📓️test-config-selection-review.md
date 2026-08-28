# Test Config Selection Review

Read-only audit of the mounted `ChangeTestConfigSelection` packet. No compiler or runtime command was run.

## Defect

`ChangeTestConfigSelection` text encoding is not injective: `Some("null")` prints as `change-test-config-selection null`, which parsing interprets as `None`. This loses a valid selection value and violates the required text round-trip law. The text format needs an explicit discriminant or a lossless escaped/string payload form before acceptance.

## Coverage Gap

The retained neutral controller proves only fixture JSON states and checks the source contains the enum. It does not execute the actual Rust `TestConfigDiff` serde round trip nor actual leaf text/binary round trips. Thus it cannot establish that `Identity`, `Clear`, and `Set` survive the real owned serializers. This is a test-ownership gap, not evidence of a second production defect.

## Ownership

The affected source is exclusively the mounted TestConfig fixture tree:

- `🧪️tests/🧬️test-app-mutations/🎚️config/🧬️mutations/📝️change-test-config-selection/🦀️.rs`
- its source-owned tests and the ticket neutral controller.

The staged TestMutation document tree remains unmounted and was not reviewed as an active implementation.
