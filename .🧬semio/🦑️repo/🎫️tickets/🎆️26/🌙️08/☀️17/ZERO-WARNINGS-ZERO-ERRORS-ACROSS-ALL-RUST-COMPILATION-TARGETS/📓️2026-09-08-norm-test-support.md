# Norm Contract Test Support

Pass561 includes the existing child-identity Serde bridge and operation-text escaping helper in the same explicit test-support boundary. Native532 called these helpers dead in the contract crate, while source inspection confirmed downstream EN1990 and DIN18599 tests and serde attributes call them. They are now accessible to those dependent tests; their implementations and all runtime defaults are unchanged.

Pass553 addresses native532's missing shared Serde implementations and retained-disposition oracle after the Norm contract became a separate crate. Its explicit test-support feature exposes the existing oracle assertion and the existing Serde derives for its 17 shared document types. All 15 dependent artifact dev dependencies enable it. Normal default features remain empty; the optional Serde and JSON dependencies are activated only for the oracle feature. The assertion boundary uses owned Rust types and the oracle implementation remains private.

No engineering calculations, document fields, fixture expectations, retirement operations or macro-generated artifact implementations changed. The 18 extended conditional attributes occur before the exported record macro, so the feature condition is not injected into downstream macro expansions.

- `✏️s/🔌️plugins/📕️norm/⚖️compliance/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🧪️tests/🔬️retained-disposition-oracle/🦀️.rs`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📦️packages/🦀️rust/Cargo.toml`

Fresh metadata, compiler and execution of the existing oracle laws remain required.
