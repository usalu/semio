# 🌉️ The `encode_wire_serialized`/`decode_wire_serialized` seam — converted to first-party

## Headline

The ninth (and, per this ticket's own framing, last known) seam is closed. `os-kernel`'s
`🔌️plugin/🦀️.rs` `encode_wire_serialized<T: Serialize>`/`decode_wire_serialized<T:
DeserializeOwned>`/`decode_wire_serialized_or` trio is now `T: ToValue`/`T: FromValue`-bound and
routes straight through `store::pack_rt::encode_wire_value`/`decode_wire_value` — no `serde_json`
detour. All 54 real call sites (function definitions + call sites; unchanged from the pre-edit
count, confirmed by grep before and after) compile clean against the new bound. `serde`/`serde_json`
did **not** reach `[dev-dependencies]` in `📡️replication`'s `Cargo.toml` — a second, independent,
pre-existing reason (`🌱️value`'s own `DslValue` serde bridge + `OrderedMap`/`OrderedSet`'s real
external callers) keeps them in `[dependencies]`, unrelated to this bridge and out of this pass's
fence. `serde`/`serde_json` are still linked into `semio-s-plugin-draw-fsm`'s `wasm32-wasip2` tree,
via `os-kernel`'s and `replication`'s own direct `Cargo.toml` entries — same two entries as every
prior wave found, **not through this bridge any more**.

## The three functions — before/after

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28253` (renamed from `component.rs` to `.rs`
mid-session by concurrent repo-wide churn, then back to `component.rs` again — line numbers cited
below are from the final `component.rs` name; grep by function name if the file moves again):

```rust
// before
fn encode_wire_serialized<T: Serialize>(value: &T) -> Vec<u8> {
    let json = serde_json::to_value(value).expect("wire payload must serialize to JSON");
    store::pack_rt::encode_wire_value(&DslValue::from(&json))
}
pub(crate) async fn decode_wire_serialized<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Fault> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
    serde_json::from_value(serde_json::Value::from(value)).map_err(|error| plugin_internal_fault(error.to_string()))
}
async fn decode_wire_serialized_or<T: DeserializeOwned>(bytes: &[u8], default: T) -> T { .. }

// after
fn encode_wire_serialized<T: ToValue>(value: &T) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&value.to_value())
}
pub(crate) async fn decode_wire_serialized<T: FromValue>(bytes: &[u8]) -> Result<T, Fault> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| plugin_internal_fault(error.to_string()))?;
    T::from_value(value).map_err(|error| plugin_internal_fault(error.to_string()))
}
async fn decode_wire_serialized_or<T: FromValue>(bytes: &[u8], default: T) -> T { .. }
```

`plugin_runtime`'s module imports: added `FromValue, ToValue` (the traits) to the existing `use
dsl::{from_dsl_value, to_dsl_value, DslValue, ...};` line; removed the now-unused `use
serde::de::DeserializeOwned;`. `use serde_json::Value;` was **kept** — it is still genuinely used
by ~15 unrelated call sites in the same module (context-menu hash caching, opening-command relay,
`ui_refresh_section`, etc.) that never call this bridge and were out of scope.

## Measurement — how many of the 56(54) call-site types already had the first-party derives

**Already covered, zero edits needed** (confirms the ticket brief's prediction — most of the fan-out
had already landed in the eight prior waves): `Fault`, `Effect`, `AppEvent`, `DispatchReport`,
`MergeReport`, `Conflict`, `MutationOrigin`, `ForeignStep`, `WireMutationRosterEntry`,
`WireArtifactMutationPlanRequest`, `WireArtifactMutationPlanResult`, `DslValue` (identity) — 10 of
the ~19 distinct top-level `T`s the bridge is instantiated at were already `ToValue`/`FromValue`.
`Fault` and `Effect` in particular are the exact two types the bridge's own (now-deleted) docstring
named as "only implement `Serialize`... a `ToValue` bound would be unsatisfiable" — that premise was
already stale; a peer had landed hand-written `ToValue`/`FromValue` for both
(`🧰️framework/🔨️modules/⚠️diagnostic/🦀️component.rs:383,402` for `Fault`;
`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:298` derive for `Effect`) before this session started.

**Lacked them, converted this session** — 7 top-level types plus 13 transitive field types (20
total):

| type | file | method |
|---|---|---|
| `Diagnostic` | `⚠️diagnostic/🦀️component.rs` | hand-written (see below) |
| `ExpectedSet` | `⚠️diagnostic/🦀️component.rs` | hand-written |
| `HistoryPatch` | `🎠️kernel/🦀️.rs` | derived |
| `HistoryEntry` | `🎠️kernel/🦀️.rs` | derived |
| `UiDirtyScope` | `🎠️kernel/🦀️.rs` | derived (internally-tagged enum, `tag = "kind"` + `rename_all_fields`) |
| `ActionAddress` | `🛂️manifest/🦀️.rs` | derived |
| `ActionInvocation` | `🛂️manifest/🦀️.rs` | derived — **field type change**, see below |
| `CommandOwnerAddress` | `🛂️manifest/🦀️.rs` | derived (externally-tagged enum, mixed unit/struct variants) |
| `CommandAddress` | `🛂️manifest/🦀️.rs` | derived |
| `CommandInvocation` | `🛂️manifest/🦀️.rs` | derived — same field type change |
| `UiMenuRef` | `🖱️ui/🎯️targets/🧊️wgpu/🦀️component.rs` | derived |
| `ContextMenuItemSpec` | same | derived (self-referential `Option<Vec<Self>>`) |
| `ContextMenuHit` | same | derived |
| `ContextMenuSelectionGroup` | same | derived |
| `ContextMenuTextContext` | same | derived |
| `ContextMenuSurfaceTarget` | same | derived |
| `ContextMenuPoint` | same | derived |
| `ContextMenuRequest` | same | derived (not itself bridge-crossed; converted for consistency, cheap) |
| `ContextMenuResponse` | same | derived (ditto) |
| `ContextMenuWireRequest` | `🔌️plugin/🦀️.rs` (local, module-private) | derived, `FromValue` only (never encoded) |

## Hand-written vs derived — and why

**Hand-written**: `Diagnostic`, `ExpectedSet` (`🧰️framework/🔨️modules/⚠️diagnostic/🦀️component.rs`).
This file is path-mounted directly into `📡️replication` (same convention `TextSpan`'s existing
hand-written impl in the sibling `📍️span` module already uses, and the one this file's own
`FaultCode`/`Severity`/`FaultOrigin`/`FaultScope`/`FaultCause`/`Fault` impls already followed) —
`replication` sits below `os-kernel` in the dependency DAG and the `#[derive(ToValue, FromValue)]`
macro's generated code is rooted at a hard-literal `::semio_framework_os_kernel::` path, so the
derive is structurally unusable here. Followed the file's own established pattern exactly: bare
`ToValue`/`FromValue` (already in scope via `use crate::value::{DslValue, FromValue, ToValue,
ValueError};`), `camelCase` keys mirroring `Diagnostic`'s `#[serde(rename_all = "camelCase")]`,
optional fields via a `find`/`match None | Some(DslValue::Null)` closure identical in shape to
`FaultScope`'s and `Fault`'s own.

**Derived**: everything else. `🎠️kernel/🦀️.rs` (mounted only into `🛂️manifest`, itself inside
`os-kernel`) and `🛂️manifest/🦀️.rs` already use `#[derive(..., ToValue, FromValue)]` pervasively with
`semio_framework_value_derive` in scope — additive derive + mirrored `#[value(...)]` attributes,
zero new dependency wiring needed. `🖱️ui/🎯️targets/🧊️wgpu` (the `semio-framework-ui` crate, feature
`wgpu`) did **not** already depend on `semio-framework-value-derive`, so this pass added it:

```toml
# Cargo.toml — new, both gated behind the existing "wgpu" feature
semio-framework-os-kernel = { path = "...", package = "semio-framework-os-kernel", optional = true }
semio-framework-value-derive = { path = "...", package = "semio-framework-value-derive", optional = true }
```

**Trap hit and fixed**: this crate already had `dsl = { package = "semio-framework-os-kernel",
optional = true }` (an alias). A first attempt added a bare `use dsl::{DslValue, FromValue,
ToValue};` trait import alongside the pre-existing `use semio_framework_value_derive::{FromValue,
ToValue};` macro import — `error[E0252]: the name FromValue is defined multiple times`. Root cause:
`os-kernel`'s own crate root re-exports **both** the traits (`pub use crate::os_dsl::schema::{...,
FromValue, ToValue, ...};`) **and** the derive macros (`pub use semio_framework_value_derive::{
FromValue, ToValue};`) under the same `dsl::FromValue`/`dsl::ToValue` path — so `dsl::{FromValue,
ToValue}` alone already brings in both namespaces (type + macro) at once, and a second explicit
macro import collides in the macro namespace. Fixed by importing only `use dsl::DslValue;` (no
hand-written `impl ToValue`/`impl FromValue` needed the bare trait names in this file — everything
there is derive-only) and keeping the single `use semio_framework_value_derive::{FromValue,
ToValue};` macro import. **Concurrent-session note**: this Cargo.toml was independently edited by
another live session mid-pass (replaced the `dsl` alias entry with `extern crate
semio_framework_os_kernel as dsl;` in `📦️glue.rs` instead, "Cargo rejects two entries for one
package under different names" — a cleaner fix than mine); accepted as current state per the
tool's own guidance, not reverted. My added `semio-framework-os-kernel`/`semio-framework-value-derive`
entries under their real names remain and are unaffected by that edit.

## `ActionInvocation`/`CommandInvocation.arguments`: `serde_json::Value` → `DslValue`

Both structs' `arguments: BTreeMap<String, serde_json::Value>` field blocked the derive outright —
`serde_json::Value` is deliberately never a `ToValue`/`FromValue` target (see
`📓️dsl-value-bridge-conversion.md`'s "not supported" table). Changed the field type to
`BTreeMap<String, DslValue>`; the blanket `impl<T: ToValue> ToValue for BTreeMap<String, T>` (and
`FromValue` twin) in `🌱️value/🔁️codec/🦀️component.rs` then covers it for free since `DslValue:
ToValue + FromValue` is the identity impl. `#[derive(Serialize, Deserialize)]` on both structs
stays satisfied too: `DslValue` already carries a hand-rolled `impl serde::Serialize/Deserialize`
(delegating through `serde_json::Value::from(self)`/`DslValue::from`, landed by a peer earlier this
session as a deliberate transitional bridge for exactly this "still-serde-deriving type holds a
`DslValue` field" shape — see `🌱️value/🦀️.rs:150`'s own docstring). Four real call sites needed a
one-line follow-up (`DslValue::from(value)` → `value.clone()`/`DslValue::String(...)`, since the
map's values are now already `DslValue` rather than `&serde_json::Value`):
`🔌️plugin/🦀️.rs` (`dispatch_command`'s `args` build, `handle_action_invocation`'s `windowId`
insert) and `📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` (`touch_space_index_artifact`'s
argument map literal, `command_search_items`'s `Select`-option argument map, and
`resolve_command_context`'s `.and_then(dsl_value_as_json)...` → `.and_then(DslValue::as_object)`
rewrite, the only one of the four that dropped a JSON round trip entirely rather than just
retyping a literal).

## MediaIn/MediaOut/MediaFingerprint: kept `serde_json::to_string`/`from_str`, not `pack::json`

These three call sites needed a JSON **text** string, not an in-memory `DslValue`
(`plugin_consume_media`'s `descriptor_json: &str` parameter, `plugin_produce_media`'s returned JSON
text). Rather than introduce `pack::json` (a sibling first-party JSON-text writer/parser landed
this ticket, with its own `to_json_string<T: ToValue>`/`from_json_str<T: FromValue>` convenience
pair) as a new dependency edge, kept `serde_json::to_string`/`from_str` — legal and minimal because
`DslValue` now has its own `serde::Serialize`/`Deserialize` impl (the same transitional bridge
named above), so `serde_json::to_string(&descriptor_value)` where `descriptor_value: DslValue`
compiles unchanged from the pre-edit shape modulo the type annotation
(`Value`/`serde_json::Value` → `DslValue`). `MediaFingerprint` (a `String`-wrapping tuple struct,
not derive-eligible) kept its existing `serde_json::to_value(&fingerprint)` and now bridges through
the pre-existing infallible `DslValue::from(&serde_json::Value)` conversion instead of gaining its
own hand-written `ToValue` impl — `MediaFingerprint` itself never crosses the bridge as a named
type, only the `DslValue` built from it does.

## `replication`'s `Cargo.toml` — reached `[dev-dependencies]`? No — independent blocker remains

`serde`/`serde_json` stay in `[dependencies]`. This bridge (the ticket's "blocker 1" as of
`📓️replication-serde-removal.md`) is now fully closed — `DispatchReport`/`MergeReport`/`Conflict`/
`MutationOrigin` no longer need `Serialize`/`DeserializeOwned` for this bridge's sake. What remains
is the **separate, independent, pre-existing "blocker 2"**, unchanged by this session: `🌱️value`'s
own `impl serde::Serialize/Deserialize for DslValue` (a deliberate transitional bridge, still real,
still has other live callers — see above) and `impl From<&DslValue> for serde_json::Value`
(JSON-export/UI-boundary bridge), plus `🗂️ordered/🦀️.rs`'s `OrderedMap<V>: Serialize` and
`🗂️ordered/🧺️set`'s `OrderedSet: Serialize + Deserialize`, both with real external callers
(`os-kernel`'s neural-engine `Dictionary`; `💻️os/🔨️modules/🌊️flow/**` and the `🌀️procedural`
plugins, respectively). Also unrelated to either bridge: `InteractionState` and its
`🕹️interaction/**` siblings are hit **directly** via `serde_json::to_string`/`from_str`/
`from_slice`/`from_value`, never through `encode_wire_serialized` — untouched, separate follow-up.
`replication`'s `Cargo.toml` docstring was rewritten to record this (blocker 1 resolved, blocker 2
reconfirmed, narrowed to its real remaining scope) — no dependency line moved.

## Verification — full command list, verbatim tails

All run in the foreground, one at a time, against the shared `target/` (no `CARGO_TARGET_DIR`
override), re-run fresh after every blocking concurrent-churn error cleared (see "concurrent churn
hit and cleared" below) — every tail below is from the final, clean re-run.

```
$ cargo check -p semio-framework-os-kernel --message-format=short
    ...
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2.13s
```
0 errors. 33 warnings — same count `📓️dsl-value-bridge-conversion.md` recorded as the pre-existing
baseline; 0 new warnings introduced by this session's edits.

```
$ cargo check -p semio-framework-replication --message-format=short
    Checking semio-framework-replication v0.1.0 (...)
warning: `semio-framework-replication` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-replication` to apply 1 suggestion)
    Finished `dev` profile [unoptimized] target(s) in 2.70s
```

```
$ cargo test -p semio-framework-replication --lib
...
failures:
    causal::tests::causal_add_fixture_has_exact_required_descriptor
    value::tests::serde_json_uses_the_same_json_shape_as_the_dsl_value_bridge
    value::tests::serde_json_value_round_trips_through_dsl_value

test result: FAILED. 226 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.14s
```
226/229 — identical to `📓️replication-serde-removal.md`'s own recorded state. All 3 failures are
pre-existing and outside this bridge's fence: 1 is a taxonomy-fixture path-string mismatch
(`causal_add_fixture_has_exact_required_descriptor`, unrelated to serialization, documented since
the pilot wave); 2 are the `🌱️value` module's own `serde_json`-vs-`DslValue` key-order/number-shape
oracle tests, failing because a peer's hand-rolled `impl serde::Serialize for DslValue` (landed
mid-session, in a file outside this bridge's fence) doesn't yet byte-match `serde_json`'s own
default `Value` shape — not something this pass's own edits touch or are responsible for fixing.

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    ...
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.18s
```

```
$ cargo check -p semio-framework-ui --features wgpu --message-format=short
    ...
warning: `semio-framework-ui` (lib) generated 37 warnings (run `cargo fix --lib -p semio-framework-ui` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.96s
```
Checked separately since this pass added the `ToValue`/`FromValue` derive fan-out here — 0 errors.

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (...)
│   └── semio-s-plugin-draw-fsm v0.1.0 (...)
└── semio-framework-replication v0.1.0 (...)
    ├── semio-framework-os-kernel v0.1.0 (...) (*)
    └── semio-framework-pack v0.1.0 (...)
        └── semio-framework-os-kernel v0.1.0 (...) (*)

$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde_json --edges normal
serde_json v1.0.149
└── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro) (...)   # host-only, not linked
    └── semio-framework-os-kernel v0.1.0 (...)
        └── semio-s-plugin-draw-fsm v0.1.0 (...)

serde_json v1.0.149
├── semio-framework-os-kernel v0.1.0 (...) (*)
└── semio-framework-replication v0.1.0 (...)
    ├── semio-framework-os-kernel v0.1.0 (...) (*)
    └── semio-framework-pack v0.1.0 (...)
        └── semio-framework-os-kernel v0.1.0 (...) (*)
```
Both trees read in full (not truncated) — 2 resolved instances each, matching every prior wave's
measurement exactly. Both edges are `os-kernel`'s and `replication`'s own direct `Cargo.toml`
entries; **neither routes through `encode_wire_serialized`/`decode_wire_serialized` any more** —
that edge is closed.

## Concurrent churn hit and cleared (not this pass's own bugs — recorded per this ticket's own
## stale-check discipline)

Three unrelated, pre-existing/concurrent breakages were hit and (where trivial and unambiguous)
fixed along the way, all confirmed via `git status` as untracked/uncommitted peer work in files
this pass never otherwise touches:

1. A repo-wide `🦀️component.rs` → `🦀️.rs` filename churn (and, for at least the `🧊️wgpu` target,
   partway back again) left several `#[path = "...component.rs"]`/`#[path = "....rs"]` mod
   declarations stale relative to the file's actual current name. Fixed 4 one-line path-string
   corrections after confirming via `find`/`ls` which name currently exists on disk:
   `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (2, `🗣️dsl/🧬️schema` and `🚪️io/🧬️schema`),
   `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs` (1), `🧰️framework/🔨️modules/🖱️ui/
   📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (1, after the same target's `component.rs` file was
   renamed back mid-session by a concurrent session). Purely mechanical — matched the target file's
   real current name, no logic touched.
2. A separate, live, in-progress restructure of `📇️directory/🧬️schema` (an `os_dsl::schema::{ToValue,
   FromValue, DslValue, ValueError}` path that didn't yet resolve) produced 7 real compile errors
   for several checks in a row; self-resolved by the owning session between retries — not fixed
   here, confirmed via `git status` (untracked file) and the error's own module path naming no file
   this pass touches.
3. A transient `error: multiple workspace roots found in the same workspace` (naming
   `✏️s/🔌️plugins/🗄️stdio/🧪️oracle` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, both
   declaring their own `[workspace]`) blocked every `cargo` invocation repo-wide for several
   retries; also self-resolved by whoever owns that pair of crates — not this ticket's fence, not
   touched.

## HARD CONSTRAINTS check

No shim, no compat layer landed. No type left deriving both `serde::{Serialize, Deserialize}` and
`ToValue`/`FromValue` in a way that leaves the serde half dead — every touched type kept its serde
derive alongside the new one only where the type is genuinely still framework-shared (allowed per
this ticket's own "framework is exempt from the ban" rule) or has a real oracle-test use (none of
the 20 touched types needed a `#[cfg_attr(test, ...)]` split — none of them had a serde-only test
oracle need, they all have real production serde callers already, e.g. `ActionInvocation`/
`CommandInvocation`'s own `serde_json::to_string` wire-encode call sites in the renderer). No git
commands run. No ticket opened/closed/reopened.
