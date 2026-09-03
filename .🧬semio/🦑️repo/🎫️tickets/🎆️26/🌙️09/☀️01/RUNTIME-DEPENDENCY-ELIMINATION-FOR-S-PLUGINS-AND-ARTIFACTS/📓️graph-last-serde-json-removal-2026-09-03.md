# `semio-framework-graph`'s last `serde_json` — removed

## Function found

`🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs`: `property_bag_from_json(&serde_json::Value) -> PropertyBag`
and `property_bag_to_json(&PropertyBag) -> Option<serde_json::Value>`. These were the only two
`serde_json`-typed signatures left in the crate (plus one test using `serde_json::json!`).

## External callers found

Grepped the whole repo (`grep -rln "property_bag_from_json\|property_bag_to_json" --include="*.rs"`):
only 3 files total.

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs:11` — re-export only (`pub use graph::{...}`), never called directly.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` — 4 call
  sites (lines 9508, 9540, 9565, 9613), all inside `NormalPort::sync_descriptor`, one per
  `NodeDescJson`/`HandleDescJson`/`EdgeDescJson`/`WireDescJson`'s `user_data` field.

## Conversion done

Renamed and re-typed the graph engine functions (old names were misleading once the JSON type left
the signature):

```rust
pub fn property_bag_from_value(value: &dsl_core::DslValue) -> PropertyBag { ... }
pub fn property_bag_to_value(bag: &PropertyBag) -> Option<dsl_core::DslValue> { ... }
```

Bodies now operate on `DslValue` directly (no `serde_json::from_value`/`to_value` round trip) —
strictly less code than before, using the existing `dsl_core::FromValue`/`ToValue` traits already
in scope.

Updated the crate's own test (`⚙️engine/🦀️.rs`, `mod tests`): `serde_json::json!("not-an-object-map")`
→ `dsl_core::DslValue::String("not-an-object-map".to_string())`; renamed the two tests
(`property_bag_value_round_trips_and_empty_bag_serializes_to_none`,
`property_bag_from_value_falls_back_to_default_on_unparsable_shape`) and the `#subregion`/`#region`
tags to match.

## `♾️infinite` call sites — why the field type did NOT need to change

`NodeDescJson`/`HandleDescJson`/`EdgeDescJson`/`WireDescJson` keep `user_data: Option<serde_json::Value>`
unchanged. That field is populated by real `#[derive(serde::Deserialize)]` on these structs, which in
turn are reached from `NormalPort::parse_fixture_v1(&self, raw: &serde_json::Value)` →
`serde_json::from_value::<FixtureJson>(raw.clone())` — a genuine external JSON boundary
(`➕️normal/🦀️.rs:9711`), not something this ticket's 3-crate scope owns. That confirms the Cargo.toml
comment written by the earlier phase-2 agent was accurate: `♾️infinite`'s board ports really do hand
this crate a `serde_json::Value`-shaped `user_data`.

The fix was a one-line bridge at each of the 4 call sites, using the `DslValue::from(&serde_json::Value)`
bridge that already exists in `🌱️value/🦀️.rs` (the same bridge `property_bag_from_json`'s body used to
call internally — it just moved one call frame outward, to the actual JSON-boundary caller instead of
living inside the graph crate):

```rust
// before
let properties = n.user_data.as_ref().map(property_bag_from_json).unwrap_or_default();
// after
let properties = n.user_data.as_ref().map(|v| property_bag_from_value(&dsl::DslValue::from(v))).unwrap_or_default();
```

(`dsl` is the crate-root `extern crate semio_framework_os_kernel as dsl;` alias already used
throughout `♾️infinite` — no new import needed.) Applied identically for `h`/`e`/`w` (Handle/Edge/Wire).
`property_bag_to_json`/`_to_value` has no call sites in `♾️infinite` (import-only, never invoked), so
only the import name at `🎲️board/🦀️.rs:11` needed updating.

This is NOT a "relocate the problem" bridge in the sense the ticket warned against — `♾️infinite`
still needs `serde_json::Value` for its own much larger JSON-boundary surface (dozens of
`serde_json::from_str`/`from_value` call sites across `🌍️world/🦀️.rs`, `🕸️dag/🦀️.rs`, fixture
parsers, etc. — confirmed by repo-wide grep), so nothing was gained by trying to push `DslValue` further
into that crate for this one field. `semio-framework-graph` itself, however, now has zero
`serde_json`-typed signatures and zero `serde_json` references in its own source.

## Cargo.toml

`serde_json = "1.0.140"` line removed from `semio-framework-graph`'s `[dependencies]` (the `serde`
line was already gone, removed by an earlier phase-2 agent). No `[dev-dependencies]` entry needed —
the crate's own test no longer needs `serde_json` either.

## Verification (target dir `…/scratchpad/iso3`, `RUSTC_WRAPPER=""`)

- `cargo metadata --no-deps --format-version 1` — exit 0.
- `cargo check -p semio-framework-graph --message-format short` — **0 errors** (258 pre-existing
  warnings, all in a generated file `🛂️manifest/🦀️generated-value-bridge.rs`, unrelated to this change).
- `cargo test -p semio-framework-graph` — **183 passed; 0 failed** (same count as before this
  ticket's Phase 2 serde-derive removal; includes the 2 renamed property-bag tests).
- `cargo check -p semio-framework-os-infinite --message-format short` (the consumer) — 3 errors,
  **all in `🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`** (E0283 type-annotation-needed at lines 1701,
  1702, 5343) — a file this ticket explicitly excludes and that `git status` shows as currently
  modified by a concurrent peer session (uncommitted, not touched by this ticket). Zero errors in
  any file this ticket edited (`🎲️board/🦀️.rs`, `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs`).
  Baseline measured before edits: 2 pre-existing errors, both in the excluded `🎠️kernel/🦀️.rs`
  (`CapabilityGrant: ToValue`/`FromValue` not satisfied) — also untouched by this ticket, also
  outside its 3-crate scope.
- `cargo check -p semio-framework --message-format short` — **0 errors** (unchanged from baseline).

## Files touched

- `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs` — function rename/re-type + body simplification, 2 tests updated.
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` — `serde_json` dependency line removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️.rs` — import rename only.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` — import
  rename + 4 call sites bridged through `dsl::DslValue::from(v)`.

Not touched: 🛂️manifest, 🎠️kernel, any test/oracle/fixture/probe/generator directory, `✏️s/🔌️plugins/**`.
