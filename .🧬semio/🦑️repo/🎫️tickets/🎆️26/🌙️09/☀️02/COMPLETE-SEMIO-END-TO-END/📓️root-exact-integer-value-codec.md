# Exact Integer Value Codec

The actor native law exposed fractional integer coercion. The shared value
decoder also wraps signed/unsigned and narrow-width inputs before public plan,
lease, version, generation, and socket-expiry validation. The read-only evidence
and direct boundary census are in `📓️terra-directory-exact-integer-codec-audit.md`.

The scalar contract will accept only integer-token `UInt`/`Int` values that fit
the target primitive exactly. All `Float` values, including mathematically whole
floats, are distinct and rejected. There is no compatibility conversion. Domain
range checks remain separate; actor body's explicitly bounded semantic-number
type remains local to that domain.

Tests come first: raw JSON strings preserve token distinctions and full64-bit
magnitudes. JSON Schema and independent BigInt arithmetic validate the neutral
policy; a native value-owner law compares every primitive admission/result to
serde JSON, then checks direct nonfinite and cross-signed values. The registered
actor native target will run the primitive prerequisite as a separate exact
replication-library group before the actor schema law.

Native test-first receipt: session97846 / exact-cargo-laws-Z1NZxR reached the
production assertion and failed: `u8` incorrectly admitted raw
`-9223372036854775809`. The source independent oracle passed (8 targets ×38 raw
tokens,104 admissions). The integer macros now use checked `Number::as_u64` or
`Number::as_i64` followed by primitive `TryFrom`; the native rerun is pending.

The first repaired native rerun is GREEN2: session80124 / yYxoQH. The scalar
law covers38 raw tokens ×10 primitives plus60 direct float denials and20
cross-signed admissions. Replication executable SHA-256:
`a2971ec6f5ae27bbad89f199a4317280f408c7d4cc2b203e7761978bf8f6561a`.
The unchanged49-case actor law also passed on kernel SHA-256
`0136434aecc34841f27716df1de73b330293382aa4d77749332569192b28e7e4`.

The registered target now additionally selects all13 shared codec tests and a
new kernel law exercising the actual first-party JSON parser against serde for
every raw token/primitive, plus the real `DocumentOpenIntentV1.version` and
`SocketGrantReceiptV1.expiresAtMs` typed projections. These additional selectors
are native-pending; they are not covered by the preceding two-law receipt.

Expanded current run is GREEN15: session50874 / exact-cargo-laws-xvoyyB.
All13 shared codec tests passed on the same replication SHA above. Both kernel
laws passed on SHA-256
`93d9f89209cbfc55ec9774734da0a27ac8a79de3b677714e3fa4883706c51449`.
The production JSON parser and both public typed authority projections agree
with serde's exact integer admission across the complete38-token corpus.
No catalog actor propagation, asset delivery, or editor activation is claimed.
