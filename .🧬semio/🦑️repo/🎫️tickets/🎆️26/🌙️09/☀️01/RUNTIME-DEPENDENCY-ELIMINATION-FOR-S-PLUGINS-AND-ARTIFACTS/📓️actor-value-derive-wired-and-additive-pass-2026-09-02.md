# Actor: value-derive Cargo edge + additive ToValue/FromValue pass (2026-09-02)

## Cargo.toml change (authorized in this brief)

Added to `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml`:
- `semio-framework-replication = { workspace = true }` — gives actor `protocol::value::{DslValue, ToValue, FromValue, ValueError}`, the SAME nominal type os-kernel uses. Proof: os-kernel's `dsl::schema` module does `pub use protocol::value::{from_dsl_value, ordered, to_dsl_value, DslValue, FromValue, Number, ToValue, ValueError};` (🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:444) — os-kernel does not mount its own copy of `🌱️value/🦀️.rs`, it re-exports replication's. `cargo tree -p semio-framework-replication` shows zero edge to actor (deps: hash, io-base64, value-derive, serde, serde_json, deflate[optional]).
- `semio-framework-value-derive = { path = "../../../🌱️value/✨️derive/📦️packages/🦀️rust" }` — leaf proc-macro crate, confirmed no reverse edge to actor via `cargo tree -p semio-framework-value-derive -i` (only os-kernel dev-dep and replication depend on it).

`🌱️value/🔁️codec/🦀️.rs`'s bare traits module was ruled out standalone (needs `DslValue`, defined in the parent `🌱️value/🦀️.rs`, which also carries unconditional `serde_json` bridge impls — mounting it would make a nominally-distinct `DslValue`). Depending on `semio-framework-replication` (crate name `protocol`) instead reaches the canonical type through its existing, already-cycle-free dependency.

## Additive derive pass

All derive-decorated types with `Serialize`/`Deserialize` across the actor crate tree got `#[derive(ToValue, FromValue)]` + `#[value(crate = "::protocol::value", ...)]` mirroring their `#[serde(...)]` container attrs, keeping every existing `Serialize`/`Deserialize` untouched:
- `🦀️.rs` (crate root): 45 types
- `🚪️lifetime/🦀️.rs`: 4 derived + 1 hand-written (`ActorInstanceLifecycleReceipt` — `with` on enum-variant named fields is a `compile_error!` in the derive, so this one is a hand `impl ToValue`/`impl FromValue`)
- `📤️return/🦀️.rs`: 5 top-level + 1 macro definition expanding to 4 wire-enum types = 9
- `🚪️lifetime/🩹️patch/🦀️.rs`: 1 (`ActorUiPatchReceipt`)

Real total: 45 + 5 + 9 + 1 = **60** derived/hand-written types (brief's "57" undercounted the `wire_enum!` macro's 4 expansions and the `🩹️patch` file, which isn't in the top-level file list).

## Gaps found and closed

- Tuple structs (`PackageId`, `PackageHash`, `ActorId`, `WindowId`, `CoalesceKey`, `ShardId`) need `#[value(transparent)]` — without it the derive's `named_fields` path rejects any non-named-field struct.
- `Mailbox.lanes: [VecDeque<Envelope>; 4]` — `🌱️value/🔁️codec` had no `VecDeque<T>` impl (only `Vec<T>`). Added one (additive, mirrors the `Vec<T>` impl exactly) to `🔁️codec/🦀️.rs`.
- `ShardTable.assignment: BTreeMap<ActorId, ShardId>` / `.exclusive_leases: BTreeMap<ShardId, ActorId>` — the derive's generic `BTreeMap` support is `BTreeMap<String, T>` only; a newtype-integer key needs a stringified-key object shape (matches what `serde_json` itself does for a newtype-struct map key). Hand-wrote `actor_shard_map_to_value`/`_from_value` and `shard_actor_map_to_value`/`_from_value`, wired via `#[value(serialize_with = "...", deserialize_with = "...")]`.

## Verification (iso3, RUSTC_WRAPPER="")

- `cargo metadata --no-deps` — exit 0
- `cargo check -p semio-framework-actor` — 0 errors
- `cargo check -p semio-framework-os-kernel` — 0 errors (no cycle)
- `cargo check -p semio-framework` — 0 errors
- `cargo test -p semio-framework-actor --lib` — 121 passed (113 pre-existing + 8 new `value_round_trip_*` tests added under `component::tests::quick`, exercising a transparent newtype, a plain struct, a tagged enum, the hand-written `ActorInstanceLifecycleReceipt`, and `ShardTable`'s stringified-key map bridge at actual runtime, not just type-check)
- No `Serialize`/`Deserialize` removed anywhere — diffed every changed derive line, all 23 in the main file (plus lifetime/return/patch) retain `Serialize, Deserialize` unchanged with `ToValue, FromValue` purely appended.

## Files touched

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🎭️actor/🦀️.rs`
- `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🦀️.rs`
- `🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs`
- `🧰️framework/🔨️modules/🎭️actor/📤️return/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` (added `VecDeque<T>` impl only)
