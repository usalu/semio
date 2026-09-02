# 🔢️ `DslValue` integer fidelity — `u64`/`i64` no longer collapse to `f64`

## Headline

**`u64` now round-trips as `3600`, not `3600.0`.** A dedicated regression test
(`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs::u64_field_through_to_value_and_to_json_string_renders_as_a_bare_integer`)
proves `to_json_string(&DslValue::object([("ttlSecs".to_string(), 3600u64.to_value())]))` produces
literal `{"ttlSecs":3600}`, and that a genuine `f64` field (`3600.0`) still renders `{"ratio":3600.0}`
— the two stay distinguishable end to end. **No already-shipped regression was found** — the defect
was caught before anything depending on the lossy encoding landed on disk or over the wire; see
"Audit of already-converted integer-bearing types" below for how that was established.

**Note on provenance**: this ticket has multiple concurrent agent sessions working the same live
tree. Several of the changes below (the `pack::json::Number ↔ protocol::value::Number` bridge,
`to_json_string`/`from_json_str`, the `rename_all_fields` derive fix referenced elsewhere) were
found already in place, mid-flight, when this session reached them — their own docstrings say so
explicitly (e.g. `🎒️pack/🔤️json/🦀️.rs`: *"a concurrent session landed that walk in this same file
while this one was in flight"*). This document describes the FINAL state and what THIS session
verified/added, not a claim of sole authorship.

## The new `DslValue::Number` representation

`🧰️framework/🔨️modules/🌱️value/🦀️.rs`:

```rust
#[derive(Clone, Copy, Debug)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

pub enum DslValue {
    Null,
    Bool(bool),
    Number(Number),   // was: Number(f64)
    String(String),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}
```

Shape mirrors `pack::json::Number` exactly (`UInt`/`Int`/`Float`, same `as_f64`/`as_i64`/`as_u64`/
`is_integer`, same cross-`UInt`/`Int` `PartialEq`) so the `pack::json ↔ DslValue` bridge
(`🎒️pack/🔤️json/🦀️.rs`'s `from_dsl_value`/`to_dsl_value`) is a straight variant-to-variant map, never
a widen-then-guess. Ergonomic constructors added: `DslValue::uint(u64)`, `DslValue::int(i64)`,
`DslValue::float(f64)`; accessors `as_f64`/`as_i64`/`as_u64` added directly on `DslValue` too.

### `impl_number_codec!` split three ways (`🌱️value/🔁️codec/🦀️.rs`)

The single macro covering `f64, f32, i8..i64, isize, u8..u64, usize` was split into
`impl_uint_codec!` (`u8/u16/u32/u64/usize` → `Number::UInt`), `impl_int_codec!` (`i8/i16/i32/i64/
isize` → `Number::Int`), `impl_float_codec!` (`f32/f64` → `Number::Float`). This is the ONE choke
point every `#[derive(ToValue, FromValue)]` struct/enum with a primitive numeric field bottoms out
on — fixing it here retroactively fixes fidelity for every already-derived type with zero per-type
changes. `FromValue` for every integer type accepts all three `Number` variants (`as` casts, same
truncation semantics the old `f64 as $ty` path had for in-range values); grep confirmed no
hand-written `ToValue` impl for a primitive integer type exists outside this macro anywhere in
`🧰️framework`/`✏️s`.

### Bridges updated for lossless round-tripping

- `pack::json::from_dsl_value`/`to_dsl_value` (`🎒️pack/🔤️json/🦀️.rs`) — variant-for-variant map,
  `protocol::value::Number::{UInt,Int,Float}` ↔ `pack::json::Number::{UInt,Int,Float}`.
- `impl From<&DslValue> for serde_json::Value` (`🌱️value/🦀️.rs`) — `Number::UInt`/`Int` now produce
  `serde_json::Value::Number` from the native integer `From` impls (bare `3600` on the wire);
  `Number::Float` keeps the old `json!(*v)` path (`.0` preserved for whole floats, `Null` for
  NaN/±Infinity, unchanged from before).
- `impl From<&serde_json::Value> for DslValue` — now checks `n.is_u64()`/`is_i64()` (which reflect
  the ACTUAL literal-text shape the number parsed from, not a widening guess) before falling back to
  `Float`, instead of always widening to `f64`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`'s `pack_rt::json_value_to_dsl`/
  `dsl_value_to_json` simplified to delegate directly to the bridges above (previously hand-rolled,
  always-`f64` duplicates). `renormalize_whole_number_floats` re-targeted to canonicalize a
  fractionless `Number::Float` into `Number::UInt`/`Int` (previously a no-op-shaped `f64 as i64 as
  f64` round trip) so `json_values_equal`'s documented "3 vs 3.0" semantic-equality contract still
  holds now that the two are structurally different `DslValue` shapes.
- The os DSL text parser/writer (`🗣️dsl/🧬️schema/🦀️.rs`'s `parse_dsl_value`/`print_dsl_value`) now
  distinguishes `TokenKind::Int`/`Float` on the way in (parses `Int` text as `u64` first, `i64` on
  sign, `f64` only as a last resort) and prints `UInt`/`Int` without a decimal point on the way out —
  this DSL text format gained the same fidelity, not just JSON.

### Deliberately left unchanged (documented, not silently dropped)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️.rs`'s binary `encode_dsl_value`/
  `decode_dsl_value` (the `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS` frozen wire
  format, with a TS `PackValueCodec` mirror) still tags every `DslValue::Number` as `TAG_F64` (8-byte
  `f64`) regardless of variant — fixed only for compilation (`n.as_f64()`), not for fidelity. That
  binary tag set is a SEPARATE frozen contract from this ticket's JSON/`.spr` concern, has its own TS
  mirror that would need a matching update, and changing it was out of scope for this pass. The
  matching `🎒️pack/🧪️testkit/🦀️.rs` fuzz generator was deliberately kept `Number::Float`-only for
  the same reason (a generated `UInt`/`Int` fixture would fail its own round-trip assertion against
  the still-`f64`-only wire tag). Flagged here for whoever picks up that binary format next.

## Audit of already-converted integer-bearing types

Searched `🧰️framework` and `✏️s` for any hand-written `ToValue`/`FromValue` impl on a primitive
integer type outside `🌱️value/🔁️codec/🦀️.rs`'s macros, and for any `DslValue::Number(EXPR as f64)`
construction bypassing them — **none found**. Every `#[derive(ToValue, FromValue)]` type with a
`u64`/`i64`/`u32`/etc. field (including `📇️directory`'s already-converted `Identity` cache and every
type this ticket moved off serde elsewhere) automatically gained fidelity the moment the codec macro
was fixed, with no per-type edit needed. This is a structural argument, not a full manual audit of
every one of the "hundreds of types" this ticket converted — the repo is greenfield with no
production users yet (per `CLAUDE.md`), so no already-persisted `.spr`/hub bytes exist to have
silently regressed; the only artifacts at risk were on-disk TEST fixtures baked with the old
`Number(1.0)`-shaped literals, all of which were found and fixed (see "Test fixtures updated" below)
with their owning suites re-passing.

`📇️directory/🪪️identity`'s `Identity` cache conversion (noted "harmless" in
`📓️directory-spr-serde-removal.md` because it never left the process) now gets exact integer
fidelity as a side effect, at zero additional cost.

### Test fixtures updated (found via the type-checker, not by inspection)

`🌱️value/🦀️.rs`, `🌱️value/🔁️codec/🦀️.rs`, `🌱️value/✨️derive/…/tests/🛡️deny-unknown-fields-enums.rs`
(partly by a concurrent session), `🎒️pack/🔤️json/🦀️.rs`, `📡️replication/📡️wire/🏠️local-interaction/
🦀️.rs`, `📡️replication/🔗️causal/🦀️.rs`, `📡️replication/🧾️wire/🦀️.rs`, `🗣️dsl/🧬️schema/🦀️.rs`,
`🎒️pack/🔢️value/🦀️.rs` (os product), `🎒️pack/🧪️testkit/🦀️.rs`, `🏪️store/🦀️.rs`'s
`pack_value_fixture_corpus`. Each `DslValue::Number(literal)` site was reclassified by hand as
genuinely-integer (→ `DslValue::uint`/`DslValue::int`) or genuinely-fractional (→ `DslValue::float`)
based on what it was testing, not mechanically.

## Round-trip and boundary tests — verbatim passing output

New tests in `🌱️value/🔁️codec/🦀️.rs`: `u64_round_trips_as_uint_and_f64_round_trips_as_float`,
`i64_min_and_max_round_trip_exactly`, `u64_max_round_trips_exactly_beyond_f64_2_pow_53`,
`negative_zero_float_round_trips_and_stays_a_float`,
`whole_float_and_same_valued_integer_are_distinct_dsl_values`. New test in `🎒️pack/🔤️json/🦀️.rs`:
`u64_field_through_to_value_and_to_json_string_renders_as_a_bare_integer` (the exact `ttl_secs`
scenario named in the ticket). New tests in `🌱️value/🦀️.rs`: integer/negative/float `serde_json`
bridge distinction, `uint_round_trips_as_bare_integer_text_through_serde_json_value`,
`whole_float_keeps_its_decimal_point_through_serde_json_value`.

```
$ cargo test -p semio-framework-value-derive
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 14 tests
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p semio-framework-pack
running 88 tests
test result: ok. 88 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo check -p semio-framework-os-kernel
    Finished `dev` profile [unoptimized] target(s) in 8.66s
(0 errors)

$ cargo test -p semio-framework-replication --lib
running 238 tests
test causal::tests::causal_add_fixture_has_exact_required_descriptor ... FAILED
test result: FAILED. 237 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.94s

$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm
    Finished `dev` profile [unoptimized] target(s) in 10.24s
(0 errors)
```

The single `replication` failure, `causal::tests::causal_add_fixture_has_exact_required_descriptor`,
is **unrelated to numeric fidelity** — confirmed by inspection: the mismatch is a `payloadSchema`
STRING path (`"🛂️schema.json"` produced by code vs `"../🛂️schema/🔣️.json"` in the on-disk fixture),
not a number. This is the on-disk fixture drifting from the code under the concurrent
`26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` rename ticket (the same class of failure
`📓️verified-outcomes.md` already documents as "unrelated concurrent taxonomy fixture"). Baseline was
226-237/229-238 passing depending on measurement time in this ticket's own docs; not touched here.

### 1-ULP float-parsing bug interaction (checked)

`📓️pack-json-float-precision.md`'s finding (a one-ULP mismatch near 2^53 is an oracle
misconfiguration, not a `pack::json` defect — `float_roundtrip` feature) is orthogonal to this
change: it concerns decimal-text → `f64` parsing precision, not integer-vs-float TAGGING. No
interaction found; `pack::json::Lexer::read_number`'s float path is untouched by this work, and the
new `Number::UInt`/`Int` paths never go through float parsing at all (they parse as `u64`/`i64`
directly in `pack::json`'s lexer, which already existed prior to this session).

## `directory`/`spr` — the 59 declined refs

**Unblocked.** The root cause named in `📓️directory-spr-serde-removal.md` (`DslValue::Number` being
`f64`-only) no longer exists. Verified directly: `PayloadHash`'s existing hand-written `ToValue` impl
(`📡️replication/🆔️ids/🦀️.rs:114-118`, `self.0.iter().map(ToValue::to_value)`) now produces
`DslValue::Array` of `Number::UInt` bytes via the fixed `u8` codec, and the `pack::json`/
`serde_json::Value` bridges both preserve that as literal integers on the wire — a `.spr`
`Contributed`-origin entry would encode `[18,52,…]`, not `[18.0,52.0,…]`, if `MutationOrigin`/
`PayloadHash` were converted off their current native `serde` derive.

One correction to the original decline's framing: `spr/history`'s two `serde_json::to_string(&meta.
origin)`/`from_str` call sites (now at `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/
🦀️.rs:813,850`) currently work via `MutationOrigin`'s own **native** `#[derive(serde::Serialize,
serde::Deserialize)]` (`📡️replication/🎮️mutation/🦀️.rs:1522`), not through the `DslValue` bridge —
`PayloadHash` similarly derives serde natively (`#[serde(transparent)]` over `[u8; 32]`, which serde
always encodes as genuine integers). Neither was ever actually at risk from the OLD `f64`-only
`DslValue`, because neither routes through `DslValue` today. The decline was about the WOULD-BE
conversion (dropping `MutationOrigin`'s native serde derive in favor of `ToValue`/`FromValue` +
`os_pack::json::to_json_string`, the broader goal of this ticket) — that conversion is what was
unsafe under the old `DslValue` and is now safe. Did not perform the 59-ref `directory`/`spr`
conversion itself in this pass (large, separate scope: a real external hub's HTTP/WS contract with a
byte-identical TS client and golden fixtures, ~20 types) — flagging as unblocked for its owner per
the ticket's instruction.

## `OrderedMap`/`Dictionary` disposition (Priority Two) — attempted, then correctly reverted by a peer

This session initially gated both `Dictionary`'s `Serialize` derive and `OrderedMap<V>: Serialize`
to `#[cfg(test)]`, based on a grep that only checked for a direct `OrderedMap<_>` field in
`💻️os/🧠️neural/⚙️engine/🦀️.rs` and found just `Dictionary.pairs`. **That audit was incomplete**: a
concurrent session caught it and reverted both gates with a corrected finding, left in the code as
the current, accurate docstrings on both sites. Three production consumers in that same file still
require `Dictionary: Serialize` transitively: the `Value` enum (has a `Dictionary` variant),
`Neuron` (holds a `Value`), and a `serde_json::to_string(&merged)` call in the evaluator's
pending-extension branch — none of which are `OrderedMap<_>` fields themselves, so the earlier
textual grep missed the chain entirely. Gating either impl broke `os-kernel` for `wasm32-wasip2`.
This session accepted the correction (per this repo's live-concurrent-session rule: never revert a
peer) and cleaned up the resulting docstrings rather than re-reverting. **Both `Dictionary::Serialize`
and `OrderedMap<V>: Serialize` remain unconditional in production** — `cargo check -p
semio-framework-os-kernel`: 0 errors with both restored.

`🌊️flow`'s other `OrderedMap<WidgetLayout>`/`OrderedMap<FlowNodeGui>` usages were checked
separately and are genuinely test-only (`WidgetLayout`/`FlowNodeGui` still dual-derive `serde` +
`ToValue`, a separate deferred migration wave; their three `serde_json::to_string(&layout())` call
sites in `✏️s/🔌️plugins/🌊️flow/…` are all inside `#[semio_framework_async_macros::async_test]`
functions) — that part of the original audit held up.

**Not unblocked**: `serde` still cannot be dropped from `semio-framework-replication`'s
`[dependencies]`. Beyond the now-confirmed-necessary `OrderedMap<V>: Serialize`, `🌱️value/🦀️.rs`'s
own `impl serde::Serialize/Deserialize for DslValue` (the transitional bridge for a serde-deriving
type that HOLDS a `DslValue` field — `ui_wgpu`'s `ActionDescriptor` is the named production caller)
is still real and unconverted, as is `impl From<&DslValue> for serde_json::Value` and `OrderedSet:
Serialize + Deserialize` (real callers in `🌊️flow/**`/`🌀️procedural` per the crate-level docstring).
The tenth seam is NOT closed this session; `Dictionary`/`Value`/`Neuron` in the neural engine need
their own `ToValue`/`FromValue` conversion first (a real, separate piece of work, not attempted
here), and only then can `OrderedMap`/`Dictionary`'s `Serialize` be narrowed.
