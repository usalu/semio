# Owned Export Fixture Audit

Three source assertions failed. The complete Core channel law stopped at the obsolete 19-word `Checkpoint` input. The authored job/helper laws separately compared the first actor failure helper to the Core selector and the guest FFI declaration, and both exposed the same stale 20-word `PublishActorFailure` row. The third failure was an independent check of the same data error.

The checked-in `OwnedSemioExport::core_type` is the host selector authority. The Rust exports in `📣️macros/⚛️wp4-reactor/🦀️.rs` define the guest Core boundary. They agree on all 36 exports. `Checkpoint` and `Describe` take no operands. `Restore` and the twelve other byte-pair operations take two `i32` words. `ChannelVersion` returns an `i64`. Actor failure publish, read and acknowledge each take the failed epoch and call kind followed by the 19-word original job budget, for 21 `i64` words. Borrow takes six words, and chunk acknowledgement takes 23. The existing actor failure arity fixture independently records those field counts.

The two export parameter fixtures live under the interpreter tests, where they feed the Bun source oracles and native interpreter test. Their test schemas and source assertions were updated to match. The channel schema admits the 23-word helper, and the source test checks the exact parameter and result widths in all 36 guest declarations as well as Core selection. The shared vector expectation distinguishes 21 `i64` words from `AdmitActorInput` with two `i32` words and 19 `i64` words. No production binding, WIT, build configuration, or guest source changed.

All 12 relevant Bun source tests passed, including independent WebAssembly execution of the full channel. Both changed source tests passed strict TypeScript checking. The pinned native interpreter harness passed all 21 tests, including the Wasmtime job signature and actor failure arity laws.

## Test files

- [🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🔣️.json>)
- [🧬️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🧬️.json>)
- [🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts>)
- [core-signature.🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/core-signature.🟦️.ts>)
- [🧬️channel/🔣️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🧬️channel/🔣️.json>)
- [🧬️channel/🧬️.json](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🧬️channel/🧬️.json>)
- [🧬️channel/🟦️.ts](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔌️exports/🧬️channel/🟦️.ts>)
