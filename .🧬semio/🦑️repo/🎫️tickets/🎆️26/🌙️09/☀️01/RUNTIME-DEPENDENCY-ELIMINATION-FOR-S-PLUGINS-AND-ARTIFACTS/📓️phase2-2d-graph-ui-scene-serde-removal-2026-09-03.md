# Phase 2 serde removal — `semio-framework-2d`, `semio-framework-graph`, `semio-framework-ui-scene`

Scope: the three crates assigned this pass. `semio-framework-geometry` (serde deleted) and
`semio-framework-3d` (serde moved to `[dev-dependencies]`) were already proven green before this
session and were not touched again.

## Result summary

| crate | outcome |
|---|---|
| `semio-framework-2d` | **Already correct, left untouched.** A peer pass had already stripped `serde_json` and documented `serde` as genuinely unremovable (see below). Verified the claim still holds and re-confirmed 0 errors. |
| `semio-framework-graph` | **`serde` fully removed.** `serde_json` kept, narrowed to one production call site forced by an external crate outside this ticket's scope (see below). 0 errors, 183/183 tests pass. |
| `semio-framework-ui-scene` | **`serde` kept, left untouched, reason recorded.** Genuinely not fixable from inside this crate without either extending the framework `🌱️value::DslValue` enum or breaking byte-exact wire-format oracle tests. See full analysis below. |

## `semio-framework-2d` — verified, not re-touched

`⚙️engine/🦀️.rs`'s `PathSegment` enum is dual-derived (`Serialize, Deserialize` +
`value_derive::ToValue, FromValue`) and the crate's `Cargo.toml` carries a docstring citing a
confirmed `cargo check -p semio-framework-os-flow` error (15× E0277) proving
`semio-framework-os-flow`'s `🖍️drawing/🦀️.rs` (`DrawingNode`, `SceneNode`, a third private struct —
none `#[cfg(test)]`) derives `Serialize`/`Deserialize` over `Vec<semio_framework_2d::PathSegment>`/
`Option<Vec<...>>` in production. `os-flow` is not one of this ticket's crates. Re-verified the
claim by grepping `PathSegment` usage in `os-flow`'s drawing module directly — still true. `cargo
check -p semio-framework-2d --message-format short` — 0 errors, unchanged.

## `semio-framework-graph` — serde fully removed

### What forced `serde` to stay before this pass

All production serde usage was already dual-derived (`ToValue`/`FromValue` alongside `Serialize`/
`Deserialize`) from an earlier additive-only session (see
`📓️graph-infinite-replication-tovalue-additive-2026-09-02.md`, 41 types converted, 0 errors). This
pass removed the serde half everywhere it was safe to, across five kinds of call site:

1. **Plain dual-derived types** (`PropertyValue`, `PropertyKind`, `PropertyDef`, `ManifestAxes`,
   `PortDirection`, `PortModelAxis`, `DirectednessAxis`, `TrinityManifest`, `TrinityNodeKindDef`,
   `TrinityEdgeKindDef`, `TrinityPortKindDef` in `🛂️manifest/🦀️.rs`; `QueryResultKind`,
   `QueryResult`, `TokenClass`, `TokenSpan`, `Completion`, `DiagnosticSeverity`, `Diagnostic`,
   `Hover`, `SemanticToken` in `🗣️dsl/🦀️.rs`) — dropped `Serialize, Deserialize` from the derive
   list and every `#[serde(...)]` attribute, keeping `value_derive::ToValue, FromValue` +
   `#[value(...)]` only. Verified none of these types cross a serde boundary externally (Trinity
   plugin, the only real external consumer, already routes its own JSON through `pack::json!`/
   `pack::JsonValue`/`dsl::FromValue` — confirmed via grep, it never calls
   `serde_json::to_string(&trinity_manifest)` or similar).
2. **`KindDef.presentation`/`Manifest.edge_tips`/`Manifest.kind_compatibility`** — were
   `Option<serde_json::Value>`/`Vec<serde_json::Value>` (the two fields the earlier additive pass
   could not derive `ToValue`/`FromValue` for). Retyped to `Option<dsl_core::DslValue>`/
   `Vec<dsl_core::DslValue>` — `DslValue` *is* the schema-erased value type, so the hand-written
   `ToValue`/`FromValue` impls collapsed from a conversion to an identity copy. Confirmed the one
   real external field-access site (`♾️infinite`'s
   `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4346`: `row.presentation.as_ref().is_some_and(|p|
   p.get("color").is_some())`) compiles unchanged — `DslValue::get`/`as_str` mirror
   `serde_json::Value`'s own method names exactly.
3. **`PropertyDef.value_type: ValueType`'s serde `deserialize_with`/`serialize_with` hooks** —
   replaced with a `#[value(deserialize_with = "value_type_from_value")]` hook (value_derive
   supports this attribute, confirmed via its own source and test fixtures). See the "bug found and
   fixed" section below — this one was NOT a mechanical no-op.
4. **`🗣️dsl/🦀️.rs`'s `queryable` module** (`BoardQueryableGraph` and its fixture-JSON parsing,
   ~170 lines) — rewired from `serde_json::Value` to `dsl_core::json::Value`
   (`semio-framework-pack`'s hand-rolled JSON tree, re-exported through `os-kernel` as
   `dsl_core::json`, explicitly built as "the `serde_json::Value` replacement" with a matching
   `.get`/`.as_str`/`.as_array`/`.as_array_mut`/`Index<&str>` API). `GraphDslError::Json` switched
   from `serde_json::Error` to `dsl_core::json::JsonError`. Confirmed zero external callers ever
   pass a `serde_json::Value` into this module's public API (every public fn takes `&str`/returns
   `String`/`Option<String>` — `raw_fixture: Value` is a private field), so this was safe to change
   with no ripple.
5. **`⚙️engine/🦀️.rs`'s `property_bag_from_json`/`property_bag_to_json`** — kept the
   `&serde_json::Value`/`Option<serde_json::Value>` *signature* (forced — see below) but rewrote
   the *body* to route through the existing `DslValue <-> serde_json::Value` bridge
   (`🌱️value/🦀️.rs`) instead of `serde_json::from_value`/`to_value`, so `PropertyValue`/
   `PropertyBag` no longer need `Deserialize`/`Serialize` at all.
6. **The manifest-registry codegen** (`📜️script.ts`, the crate's own `📜️script.ts`-is-canonical
   script) — this was the actual runtime construction path for every `Manifest` (`nakagin_manifest()`
   etc.): it embedded the source `*.manifest.json` text as a `pub const ..._MANIFEST_JSON: &str`
   and called `serde_json::from_str::<Manifest>(...)` at first use. Changed the generator to emit
   `dsl_core::json::from_json_str::<Manifest>(...)` instead (the `ToValue`/`FromValue` analog of
   `serde_json::from_str`, already present in `pack::json`), and to stop deriving
   `Serialize, Deserialize`/`#[serde(rename = "...")]` on the per-manifest `*Kind` enums it also
   emits (their `ToValue`/`FromValue` already comes from the separate hand-written
   `🛂️manifest/🦀️generated-value-bridge.rs`, kept as instructed — adding the derive there too would
   have produced a conflicting/duplicate trait impl). Regenerated all 9 files with
   `bun ./📜️script.ts generate`; `🤖️generated/` is gitignored, so this is not a tracked diff.

### Bug found and fixed (would have shipped silently broken without running tests)

The additive-pass comment on `PropertyDef.value_type` claimed *"No `with` clause needed: `ValueType`
already implements `dsl_core::ToValue`/`FromValue` directly ... so the derive's default per-field
conversion is already equivalent"* — **this was wrong for the JSON-fixture decode path.**
`ValueType`'s native `FromValue` only decodes its own internally-tagged wire shape
(`{"kind": "boolean"}`); every real `*.manifest.json` fixture spells `valueType` as a bare string
(`"boolean"`/`"text"`/...) or a `{"schema": "..."}` object — the exact gap the old serde
`deserialize_with = "deserialize_value_type"` hook existed to bridge, with a try-native-first,
fall-back-to-string/schema-alias-parsing shape. Deleting that hook outright (trusting the comment)
left `cargo check` green but broke behavior: `cargo test -p semio-framework-graph` failed 3/183
with `ValueError("nodeKinds.41.properties.0.valueType.kind")` (`nakagin_manifest_loads`,
`validator_rejects_unknown_node_kind`, `manifest_by_id_resolves`). Fixed by porting the exact same
try-native-first/fallback logic into a new `value_type_from_value` function wired via
`#[value(deserialize_with = "value_type_from_value")]`. Re-ran `cargo test -p semio-framework-graph`
— **183/183 pass.** This is the concrete instance of this ticket's own rule *"You MUST NOT say that
a test is passing when you didn't run it"* — `cargo check` alone would have shipped this broken.

### Cargo.toml

```toml
# serde removed entirely.
# serde_json kept — narrowed to `⚙️engine/🦀️.rs`'s property_bag_from_json/property_bag_to_json,
# whose only external caller (♾️infinite's board ports, out of this ticket's three-crate scope)
# holds its own `user_data` field as `serde_json::Value`. Only the `Value` type is named — no
# Serialize/Deserialize derive or trait bound remains anywhere in this crate.
serde_json = "1.0.140"
```

### Files touched

- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🦀️.rs`
- `🧰️framework/🔨️modules/🕸️graph/🤖️generated/🦀️*.rs` (all 9, regenerated — gitignored, not a
  tracked diff)

Not touched: `🧮️algorithms/🦀️.rs`, `🖊️drawing/🦀️.rs` (never had real serde usage, only stale
comments), `🛂️manifest/🦀️generated-value-bridge.rs` (kept as instructed; a concurrent peer session
already had an unrelated staged diff on this file at session start — left alone).

## `semio-framework-ui-scene` — genuinely not fixable from inside this crate

`🦀️pack.rs` is a hand-rolled binary codec that works by implementing serde's own `Serializer`/
`Deserializer` traits (`PackSerializer`/`PackDeserializer`, ~640 lines) generically over any
`T: Serialize`/`T: Deserialize`, and `🦀️scenes.rs`'s `SceneDoc` trait bounds on
`Serialize + serde::de::DeserializeOwned`. This is categorically different from every other crate's
serde usage in this ticket: it isn't using `serde_json` for JSON convenience, it *implements the
serde trait machinery itself* as the wire codec.

Investigated whether this can be rebuilt over the crate's own `ToValue`/`FromValue` (all 15
`SceneDoc` payload types plus their nested record types already have full hand-written
`ToValue`/`FromValue` — see the ticket brief's "45 hand-written impls / 108 tests" note, confirmed
still true) instead of the serde trait objects. Concluded it is not safely rebuildable within this
crate's scope:

- The natural approach is to route `to_bytes`/`from_bytes` through `DslValue`
  (`protocol::value::DslValue`, this crate's dependency) instead of the serde visitor pattern.
- `DslValue` has exactly six variants: `Null, Bool, Number, String, Array, Object`. `pack.rs`'s
  wire format additionally has **first-class tags for `char` (`TAG_CHAR`) and raw `bytes`
  (`TAG_BYTES`)**, and a real `Some`/`None` distinction independent of "key present" (`TAG_SOME`/
  `TAG_NONE`) for enum/optional handling. None of the 15 `SceneDoc` types happen to *need*
  `char`/`bytes`/bare-`Option`-without-`skip_serializing_if` fields — but `pack.rs`'s own test
  suite (`🎬️RetainedSceneOracle`, `owned_scene_neutral_vectors_match_native_serde_packet`) is an
  **independent-fixture byte-for-byte oracle test** (hex bytes from
  `🧬️contract/🧵️retained/🧪️fixtures/🔣️owned-scene.json`) that explicitly exercises `char`, `bytes`,
  and enum-variant cases to prove the codec's *general* capability, not just the 15 known types.
- A `DslValue`-based rewrite cannot represent `char`/`bytes` at all without extending
  `🌱️value::DslValue` itself (a framework module change, outside this ticket's three-crate scope)
  — so it would either narrow the codec's proven capability (silently regressing something the
  oracle test suite is specifically there to catch) or require deleting/narrowing that oracle test,
  which this ticket's playbook explicitly forbids ("Do NOT delete an oracle test to make a strip
  succeed").
- No Cargo.toml edit was made (nothing to revert) — `serde` was already the crate's only
  non-dev, non-path dependency, `serde_json` already `[dev-dependencies]`-only.

**Recommendation for whoever picks this up next**: this needs either (a) `DslValue` gaining
`Char`/`Bytes` variants (and a bare-vs-skip-if `Option` wire distinction) as a framework-wide
`🌱️value` change, with `pack.rs` rewritten against the widened type, or (b) accepting the codec's
`char`/`bytes` support is genuinely unused by any real `SceneDoc` type today and formally narrowing
its contract (updating the module doc + deleting the two oracle cases *deliberately*, as a scoped
decision by whoever owns that call, not as a side effect of a strip).

## Verification

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=…/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework-2d --message-format short         # 0 errors (unchanged)
cargo check -p semio-framework-graph --message-format short      # 0 errors
cargo check -p semio-framework-graph --tests --message-format short  # 0 errors
cargo test  -p semio-framework-graph --message-format short      # 183 passed; 0 failed
cargo check -p semio-framework-ui-scene --message-format short   # 0 errors (unchanged, untouched)
cargo check -p semio-framework --message-format short            # 34 errors — see below
cargo metadata --no-deps --format-version 1 >/dev/null; echo $?  # 0
```

### `semio-framework` (framework-wide gate) — 34 errors, 0 attributable to this pass

All 34 errors are `E0277` (`X: serde::Serialize`/`Deserialize` not satisfied) on types in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**` (`WorkflowParameter`,
`WorkflowInputBinding`, `WorkflowOutputBinding`, `WorkflowEdge`, `WorkflowNode`,
`WorkflowParameterBinding`, `RunNodeStatus`, `PortFingerprint`, `RunOutputArtifact`). `🔁️workflow`
does not depend on `graph`/`2d`/`ui-scene`, and `git status`/mtime confirm this module has active
*uncommitted* changes from a concurrent peer session mid-flight on its own serde removal
(`🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/**/🦀️.rs`, `🦀️.rs` itself all modified, `🦀️.rs` mtime
`2026-09-03 00:08`, same session window as this one). Confirmed zero mention of `graph`, `2d`, or
`scene` in the error list. This matches this ticket's own documented pattern (see
`project-concurrent-cargo-workspace-churn` in the fleet's memory) — reported here rather than
"fixed" since `🔁️workflow` is outside this ticket's three-crate scope and touching it risks
clobbering the peer's in-flight work.

### `cargo tree -i serde` for `semio-s-plugin-draw --target wasm32-wasip2`

`semio-framework-graph` and `semio-framework-ui-scene` no longer appear as *direct* dependents of
`serde` in the inverted tree (graph shows up only nested under `semio-framework-os-kernel`, i.e. as
a consumer of os-kernel's own still-serde-carrying `[dependencies]`, not as a direct requirer).
`semio-framework-2d` still appears as a direct dependent — expected, matches the documented
`os-flow` blocker. The final payoff command still lists all four serde crates
(`serde serde_core serde_derive serde_json`) because `semio-framework-os-kernel` itself — not one
of this ticket's three crates — still carries `serde`/`serde_json` unconditionally (see this
ticket's own "os-kernel green, 119 → 22" update: 17 serde/serde_json entries still open there,
tracked separately). That crate is the actual remaining bottleneck for the plugin-level payoff
number to move.
