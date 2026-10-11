# 2026-10-10 fd-presence: one mechanism for `ArtifactPresenceSnapshot`

## Result
`store::ArtifactPresenceSnapshot` (supertraits now `RetireOwned + ToValue + FromValue`) has **provided** `decode_presence_native` / `encode_presence_native`. A plugin presence becomes snapshot-capable with one line:
`impl store::ArtifactPresenceSnapshot for MyPresence {}`.
No blanket impl, so the hand-written `NoPresence` (empty bytes) and any custom impl are untouched. No derive: a derive in the value-derive crate would have to name the product crate (layering inversion) or take an owner path; a provided-method trait needs neither.

## Mechanism (`🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs`)
Same machinery as `🌉️mcp/🏠️workspace/🚪️io/🔤️json` (reference precedent), generic over the value traits:
- decode: `owner.receive::<(JsonGrammarCursor<DslValue>, Option<DslValue>, Option<JsonError>, Option<P>), P>`; funded grammar steps (`normal_step_demands` -> `body.admit_frontier` -> `step` -> `body.record_progress(normal_step_progress)`), then `P::from_value_controlled` under `native.scoped_maximum(owned + remaining capacity grant)`, receipt `{items 1, copy size_of::<P>, capacity = native owned delta}`. Parsed tree stays in the receiving frame and is retired by funded retirement. `JsonMemberPolicy::Reject` (duplicate keys refused, as serde-derived fixtures expect).
- encode: `owner.receive::<(Option<DslValue>, JsonBorrowedWriteCursor, Option<String>), String>`; tree from `to_value_controlled` (same scoped ceiling), written by the funded borrowed write cursor; output `String::into_bytes` (no copy).

## Accounting (measured with the heap witness, `[DEBUG]` rows in the test)
- encode: heap requested == `progress.retained_capacity_bytes` and heap released == `progress.released_bytes`, exact, all rows.
- decode: heap requested == `retained_capacity_bytes` exactly; released receipt is a lower bound of real releases (fixture: 576 B short, constant) because `FromValue::from_value_controlled` has no release receipt for its internal temporaries (value-core follow-up if exact release is wanted). The error direction is conservative (over-reports retention).
- refused/undersized/canceled: typed error, every partial owner funded back to zero net heap through the recipient (tested for items/copy/capacity/depth denial and cancellation, both directions).

## Tests
- `🔤️json/🧪️tests/🔬️unit/🦀️.rs` + language-agnostic `🔤️json/🧫️fixtures/🔣️.json` (3 accepted rows incl. nested struct, Option, String with escapes/emoji, f64, u64::MAX, enum, whitespace; 8 refused rows). Oracle: serde_json decode equality, serde_json encode **byte** equality, decode(encode(x))==x, canonical-bytes fixed point.
- Ported `🔌️plugin/🧪️tests/📢️publication-fixtures-presence/🦀️.rs`: `PublicationPresence` now `impl ArtifactPresenceSnapshot for PublicationPresence {}` (180 lines of hand-rolled reader removed); test asserts encode+decode heap == receipts vs serde; `🧫️native.json` grant raised (old 8 KiB cannot fund the JSON frontier). NOT compiled: plugin crate is red.

## Verification (slot-gated, private dirs)
- `cargo check -p semio-framework-os-kernel --lib`: green (presence files warning-free).
- `--lib --target wasm32-wasip2`: green.
- `--lib --tests`: BLOCKED by a pre-existing syntax error in `🏪️store/🧪️tests/🔬️unit/🦀️.rs` (`with_fixture_identity!(` unclosed near line 11031, present at HEAD; since 10:58 a second half-edit near line 11851 by someone else). Not mine. Also the new `🪆️child/🧬️retained-clone` file briefly failed `Copy` earlier, fixed by its owner.
- So the tests were run from a standalone crate with the identical test source: `.🧬semio/🦑️repo/⚡️cache/play-fleet/fd-presence/proof` (sync script `sync-proof.py`), `cargo test --offline --test presence`: `test result: ok. 3 passed; 0 failed`.

## Files touched
New: `🏪️store/👥️presence/📸️snapshot/🔤️json/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs,🧫️fixtures/🔣️.json}`.
Shared, edited: `🏪️store/👥️presence/📸️snapshot/🦀️.rs` (trait + `mod json`), `🔌️plugin/🧪️tests/📢️publication-fixtures-presence/{🦀️.rs,🧫️native.json}`, corrections #34. Store root and derive crate untouched.
