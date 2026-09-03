## Unblock `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs`

**Result: 0 errors in this file.** No edit was needed — it was already fixed (by another
concurrent session) by the time I checked. `🕹️interaction/🦀️.rs` only re-exports and consumes
`HierarchyProvider`/`HoverSpec`/`SelectionSpec`/`DomainSelection`; it never defines them.

### Where the 4 types actually live
Traced `dsl::`/`protocol::` (both `extern crate semio_framework_os_kernel as …` aliases in
`🧰️framework/📦️packages/🦀️rust/🦀️.rs`) → `os_spr::wire::*` (`pub use protocol::wire;` +
`pub use self::wire::*;` in `os_spr`, `🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:157-201`) →
actual definitions in **`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`** (lines ~1515/1570/1628/1830),
a third crate (`semio-framework-replication`) outside both my file and the two sibling
agents' files (`🎠️kernel`, `🛂️manifest`).

- **`HierarchyProvider`, `HoverSpec`, `SelectionSpec`** (wire.rs ~1515/1570/1628): each carries
  `#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]` **plus** a
  hand-written `impl crate::value::ToValue`/`FromValue` alongside — diagnosis (a), fixed
  additively in the defining crate. This is what unblocked my file's `InteractionDefinition`
  (`hierarchy`/`hover`/`selection` fields, ~line 47).
- **`DomainSelection`** (wire.rs ~1830): derives only `Clone, Debug, Default, PartialEq, Eq` (no
  serde) plus hand-written `ToValue`/`FromValue`. My file only `pub use`s it (line 27, no trait
  bound needed there), so it never errors here regardless.

Confirmed no double-import-shadowing issue (diagnosis b) — `🕹️interaction/🦀️.rs` imports `ToValue`
only once, from `dsl::{FromValue, ToValue}`.

### Verification
`cargo check -p semio-framework --lib` → 0 errors. `cargo check -p semio-s-plugin-lowpoly
--all-targets` → 0 errors whose path is `🔨️modules/🕹️interaction/🦀️.rs`.

### Residual, out of scope — do not fix here
2 errors remain in a **different** file that also happens to contain the folder name
`🕹️interaction`: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs:14`
— `InteractionState: serde::Serialize`/`Deserialize` not satisfied. `InteractionState` is also
defined in `📡️replication/📡️wire/🦀️.rs` (~line 1904) and currently derives only
`Clone, Debug, Default, PartialEq` — its serde derive appears to have been dropped mid-edit
(that file has live uncommitted changes from another session; not one of the 3 files this
ticket assigned). Not my file, not my 4 types — reporting, not touching.
