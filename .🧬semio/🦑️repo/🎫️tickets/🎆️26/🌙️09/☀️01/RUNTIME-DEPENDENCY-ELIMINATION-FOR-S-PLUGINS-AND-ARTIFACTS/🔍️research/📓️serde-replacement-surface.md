# Serde Replacement Surface — Phase 1 Findings

Scope: establish the first-party replacement for `serde`/`serde_json` before the mass rewire.
Companion: `📓️serde-fanout-playbook.md` (the mechanical Phase 3 recipe, pilot status, traps).

## (a) The JSON text codec already exists

`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` — `pack::json`:

```rust
pub enum Value { Null, Bool(bool), Number(Number), String(String), Array(Vec<Value>), Object(Object) }
pub enum Number { UInt(u64), Int(i64), Float(f64) }
pub fn parse(input: &str) -> Result<Value, JsonError>
pub fn parse_bytes(input: &[u8]) -> Result<Value, JsonError>
pub fn to_string(value: &Value) -> String
pub fn array(items: impl IntoIterator<Item = Value>) -> Value
pub fn object(pairs: impl IntoIterator<Item = (String, Value)>) -> Value
```

Confirmed spec-correct (own docstring): RFC 8259 nesting/escapes/surrogate pairs, ECMA-262
number-formatting split so `42` and `42.0` never collide, NaN/±Infinity → `null` matching
`serde_json`. Deliberately NOT built: pretty-printing, arbitrary-precision integers (falls back to
`f64` outside `[i64::MIN, u64::MAX]`, same as `serde_json` without `arbitrary_precision`). This is
the crate to reach for wherever code needs literal JSON **text** (fixtures, wire bytes,
`include_str!` test data).

`pack::json::Object` is `Vec<(String, Value)>` — insertion-order preserving, last-value-wins on
duplicate keys (same externally observable behavior as `serde_json::Map`'s default `BTreeMap`
backing, just ordered instead of sorted). **Key ordering contract: insertion order, not sorted.**

## (a) The Serialize/DeserializeOwned replacement — did NOT exist, now does

Searched for an existing `ToValue`/`FromValue`-shaped pair first (`grep -rn "trait ToValue\|trait
FromValue\|trait Encode\b\|trait Decode\b" 🧰️framework`) — **none existed.** Designed and
implemented one:

`🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️component.rs` (new file, wired into
`🌱️value/🦀️component.rs` via `pub use codec::{FromValue, ToValue, ValueError};`):

```rust
pub trait ToValue { fn to_value(&self) -> DslValue; }
pub trait FromValue: Sized { fn from_value(value: DslValue) -> Result<Self, ValueError>; }
pub struct ValueError(pub String);
impl ValueError { pub fn under(self, segment: impl Display) -> Self { .. } }  // path-prefixing
```

**Deliberately over `DslValue`, not `pack::json::Value`.** `DslValue` (`Null/Bool/Number(f64)/
String/Array/Object`) already lives in `🌱️value/🦀️component.rs`, which is physically inside
`semio-framework-replication` (crate name `protocol`) — the crate that OWNS `MutationDiff`/
`Mutation` (`🎮️mutation/🦀️.rs`, same crate, `pub mod mutation` / `pub mod value` siblings in
`📦️packages/🦀️rust/🦀️.rs`). `pack` (which owns `pack::json::Value`) DEPENDS ON `protocol`/
replication (`pack`'s `Cargo.toml`: `semio-framework-replication = { workspace = true }`; `pack`'s
glue re-exports `protocol::codec`/`protocol::source`) — the dependency arrow runs
`pack → replication`. Defining `ToValue`/`FromValue` against `pack::json::Value` from inside
`replication` would need the opposite arrow and create a cycle. `DslValue` is already positioned
exactly where the trait needs to live ("the schema-erased dynamic value both sides of a
replication link speak" — its own docstring), so the trait bounds on it instead. Scalar/container
base cases (`bool`, numeric widths, `String`, `Option<T>`, `Vec<T>`, `Box<T>`,
`BTreeMap<String, T>`, `DslValue` itself) are hand-written in the same file; everything else goes
through the derive macro.

**`#[derive(ToValue, FromValue)]`** — new crate `semio-framework-value-derive`
(`🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust`), house style copied from
`semio-framework-schema-derive`/`semio-framework-machine-derive` (`proc-macro = true`, `syn`/
`quote`/`proc-macro2`, a `📦️glue.rs` wiring `#[proc_macro_derive(...)]` onto `expand_*` functions
in the owner `🦀️component.rs`). `#[value(...)]` container/field attributes — see the fan-out
playbook for the exact supported set (chosen from the repo-wide survey, not guessed).

## (b) The trait-bound rewrite — landed

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`:

```diff
- pub trait MutationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
+ pub trait MutationDiff<P>: Clone + Default + crate::value::ToValue + crate::value::FromValue {

- pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
+ pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
```

This is the fix the ticket calls "the single most important thing" — every plugin implementing a
mutation was forced onto `serde` purely by this supertrait, independent of whether the plugin's
own code ever touched `serde_json` directly. Verified: `semio-framework-replication`'s own test
fixtures (`CausalAddDiff`/`CausalAddOp` in `🔗️causal/🦀️.rs`, the crate's smallest possible
`Mutation`/`MutationDiff` pair) needed hand-written `ToValue`/`FromValue` impls to keep compiling
— done, `cargo test -p semio-framework-replication --lib` is green (225/226, the 1 failure is
unrelated concurrent churn — see the fan-out doc for the exact diff).

`MutationOutcome<D>`/`MutationMessage` (the other replication-owned structs that carry a generic
diff) were **left on their existing `#[derive(serde::Serialize, serde::Deserialize)]`** —
untouched, deliberately. Framework code is exempt from the ban (only `✏️s/` manifests must reach
zero third-party), and `#[derive(Serialize)]` on a generic `MutationOutcome<D>` only requires
`D: Serialize` on the *generated impl*, not on the struct itself — so a `MutationOutcome<PlaybookDiff>`
value is fully usable everywhere except literal `serde_json::to_string(&outcome)`, which nothing
in the codebase currently calls (checked: no `serde_json::to_string` / `to_value` call site takes
a `MutationOutcome<_>` or a bare `Mutation`/`MutationDiff`-bounded generic directly). If that
changes, the fix is the same shape: add `ToValue`/`FromValue` alongside, don't remove `Serialize`.

## (c) `#[serde(...)]` attribute survey — repo-wide, real counts

```
find ✏️s -name "*.rs" -not -path "*/target/*" -print0 | xargs -0 grep -h '#\[serde(' | sort | uniq -c | sort -rn
```

Top of the distribution (full table has ~80 distinct spellings, long tail is <5 occurrences
each):

| count | attribute |
|---|---|
| 3516 | `rename_all = "camelCase"` |
| 1427 | `default` (bare) |
| 1164 | `default, skip_serializing_if = "Option::is_none"` |
| 416 | `rename_all = "camelCase", default` |
| 265 | `default, skip_serializing_if = "Vec::is_empty"` |
| 139 | `tag = "phase", content = "value", rename_all = "camelCase"` (adjacently-tagged) |
| 86 | `tag = "mutation", rename_all = "camelCase"` (internally-tagged) |
| 83 | `skip_serializing_if = "Option::is_none"` (no default) |
| 77 | `rename_all = "camelCase", deny_unknown_fields` |
| 73 | `flatten` |
| 62 | `tag = "kind", rename_all = "camelCase"` |
| 22 | `default = "path"` |
| 14 | `deny_unknown_fields` (standalone) |
| 11 | `tag = "kind", content = "value", rename_all = "camelCase"` |
| 10 | `rename_all = "kebab-case"` |
| ~10 each | `rename = "..."`, `transparent`, `bound(...)`, a handful of `rename_all = "lowercase"` / `deserialize_with` / `serialize_with` / `alias` |

**The derive macro (v1) supports**: `rename_all` (camelCase/kebab-case/lowercase/snake_case),
`rename`, bare `default` and `default = "path"` (container- and field-level), `skip_serializing_if
= "path"`, and `tag = "..."` (internally-tagged enums only). That set alone covers every pattern
above 60 occurrences except adjacently-tagged (`tag` + `content`) and `flatten` — see "What's next"
below, this is a real, named gap, not an oversight.

**Deliberately NOT supported** (hand-write instead — see fan-out doc's trap list for exact
recipes): `tag` + `content` (adjacently-tagged, 139+11 occurrences — the single biggest gap),
`flatten` (73), `transparent`, `deny_unknown_fields` (parsed, currently a no-op), `alias`,
`serialize_with`/`deserialize_with`, `bound(...)`.

## Key ordering

`DslValue::Object` is `Vec<(String, DslValue)>` (see `🌱️value/🦀️component.rs`) — **insertion
order preserved**, not sorted, matching `pack::json::Object`'s contract. The derive macro's
generated `to_value()` pushes fields in declaration order, so wire output is deterministic and
matches source order (asserted implicitly by every round-trip test written this session; no test
currently asserts the ordering CONTRACT itself in isolation — worth adding one field-reorder-vs-
value-equality test to `🌱️value/🔁️codec/🦀️component.rs` in a follow-up).

## Number handling — divergence from `serde_json`, documented not accidental

`DslValue::Number` is a single `f64` (unlike `pack::json::Number`'s `UInt/Int/Float` split). This
means an `i64`/`u64` round-tripped through `ToValue`/`FromValue` loses precision above 2^53,
identical to what `serde_json`-via-`f64` would do but WITHOUT `pack::json`'s integer-preserving
split. This is the existing `DslValue` contract (it predates this ticket, used throughout
`os_dsl`) — not something this wave introduced or could cheaply change without touching every
existing `DslValue` consumer. Flagged, not fixed: if a plugin's mutation diff carries a real
64-bit integer that needs exact round-trip (a hash, a large counter), `ToValue`/`FromValue` is
lossy for it today. No plugin hit this in the pilot (`PlaybookDiff`'s numeric fields are all
`usize` indices well under 2^53), but the fan-out should watch for it.

## Differential test against `serde_json` — where it lives

`🌱️value/🔁️codec/🦀️component.rs`'s own `#[cfg(test)] mod tests` covers scalar/container round
trips and the double-`Option` collapse behavior directly (unit tests, not yet a `serde_json`-oracle
differential — see "What's next"). The PRE-EXISTING `🌱️value/🔀️serde/🦀️component.rs` bridge
(`ValueSerializer`/`ValueDeserializer`, `to_dsl_value`/`from_dsl_value`) already carries its own
`serde_json`-adjacent round-trip tests (newtype-wrapping-scalar, tuple-enum-variant) — those keep
passing unchanged (225/226 in the full `--lib` run) since that module wasn't touched.
`semio-framework-pack`'s own `[dev-dependencies]` already carries `serde_json` as the RFC-8259
oracle for `pack::json` (`retained-canonical-pack-laws.json` fixture, `float_roundtrip` feature) —
that's the pattern to extend for a `ToValue`/`FromValue`-vs-`serde_json`-vs-`pack::json` three-way
differential test, not yet written (see "What's next").

## What's next (not done this session — real, scoped follow-ups)

1. **Adjacently-tagged enum support** (`tag = "phase", content = "value"`) in the derive macro —
   139+11 occurrences repo-wide, concentrated enough (grep the exact crates before assuming it's
   spread thin) that it likely blocks several fan-out crates outright. Do this before assigning
   those crates out.
2. **`flatten` support** — 73 occurrences. Second-biggest gap.
3. **Delete `🌱️value/🔀️serde`** (the serde-bridge, `to_dsl_value`/`from_dsl_value`,
   `ValueSerializer`/`ValueDeserializer`) once nothing needs it — see fan-out doc trap #6 for the
   exact precondition (every `ArtifactChild<S>`-composing type gets its own real `ToValue`/
   `FromValue`). Tracked, not done: CLAUDE.md forbids leaving it as a permanent second path.
4. **A real `serde_json`-oracle differential test** for `ToValue`/`FromValue` (not just
   `pack::json`) — round-trip a constant-seeded LCG-generated value tree through both
   `#[derive(ToValue, FromValue)]` and `#[derive(serde::Serialize, serde::Deserialize)]` on the
   same fixture struct, assert structural equality. Declare `serde_json`/`serde` in
   `semio-framework-value-derive`'s (or `replication`'s) `[dev-dependencies]` only, never
   `[dependencies]`.
5. **`pack::json::Value` ↔ `DslValue` conversion** — currently no bridge exists between the two
   sibling JSON-shaped trees. Needed wherever a plugin holds literal JSON text (not just an
   in-memory value) that also needs to flow through `ToValue`/`FromValue`. Small (structural
   `From`/`TryFrom`), not yet written.
   **DONE** (process/sourcing batch, ticket root ticket, 2026-09-01): landed in
   `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (new `//#region 🔖️DslBridge` — lives
   here not in `🌱️value` because `pack` already depends on `protocol`/replication, the reverse
   edge would cycle) as `dsl_to_json`/`json_to_dsl` (structural, total, no `TryFrom` needed — both
   trees are six-shape and total) plus the convenience pair `to_json_string<T: ToValue>`/
   `from_json_str<T: FromValue>`, the `serde_json::to_string`/`from_str` analog every plugin routes
   through once it stops deriving serde. Reachable from any plugin as
   `semio_framework_os_kernel::json::{to_json_string, from_json_str, dsl_to_json, json_to_dsl}` (the
   existing `component.rs: pub use pack::*;` glob chain inside `os_pack` already carries `pack::json`
   up to the kernel crate root — no separate re-export needed). Round-trip + `serde_json`-oracle
   differential tests added alongside the existing `pack::json` test suite. See
   `📓️serde-fanout-process-sourcing.md` (this same research folder) for the consuming manifests.
6. **`ArtifactChild<S>`/`ArtifactRef` real `ToValue`/`FromValue`** (see fan-out doc trap #3/#6) —
   blocks every composed-artifact plugin from a fully clean conversion, currently bridged through
   the serde path as a named, tracked interim step.
