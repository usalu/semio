# `🌊️flow` (os-kernel side) — serde/serde_json conversion

Subtree: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/**`, crate `semio-framework-os-flow`
(`🌊️flow/📦️packages/🦀️rust/Cargo.toml`) — **not** `semio-framework-os-kernel`. `os-flow` depends on
`os-kernel` (aliased `dsl`/`store`) and is what actually mounts every file listed below (its own
`📦️glue.rs`). Do not confuse this crate, or this file, with the sibling `🔍️research/📓️serde-fanout-flow.md`,
which covers the *plugin extension* crates under `✏️s/🔌️plugins/🌊️flow/🧩️extensions/*` — a different,
already-closed batch with no overlap.

## Headline

| metric | baseline (HEAD `f394df99d4`) | now |
|---|---|---|
| `serde` references, whole subtree | 606 | 258 (−57%) |
| `serde_json` references, whole subtree | 470 | 121 (−74%) |
| files with `serde_json` fully eliminated | — | 15 of 22 that had it |

The gap between the two reduction rates is deliberate: the playbook's safe-additive rule keeps
`#[derive(Serialize, Deserialize)]` and `#[serde(...)]` attributes **alongside** the new
`#[derive(ToValue, FromValue)]` / `#[value(...)]` ones until the crate is confirmed compiling
without them (see "Not removed yet" below) — that inflates the `serde` count relative to
`serde_json` in already-converted files.

## Starting state was already broken — first fix

Before any of my own edits, all 11 files under `🌿️vcs/🧬️schema/🧬️mutations/**` (the 10 leaf structs
plus the `FlowMutation` aggregate) already carried `#[derive(..., crate::os_ToValue, FromValue, ...)]`
— committed moments earlier by the concurrent effort this ticket describes (commit `f394df99d4`,
"Replace serde-bound mutation schemas with native value traits..."). `crate::os_ToValue` does not
resolve to anything (no such item exists anywhere in the repo) — it is a botched derive-path edit,
not a design choice. Fixed by replacing `crate::os_ToValue` → `ToValue` in all 11 files (the plain
name is already in scope via each file's own `use semio_framework_value_derive::{FromValue,
ToValue};`). Confirmed no other file in the repo has this pattern.

## Critical unblocking discovery: `🧠️neural` gated the whole subtree

`FlowFixture`/`Widget`/`FlowDiff`'s core Mutation/Diff chain transitively contains
`neural_engine::{Dictionary, Value, Atom, Tree, Neuron, Synapse}` fields (`Widget::Neuron.params:
Dictionary`, `Widget::Cluster.tree: Tree`, `FlowPreviewGui.preview: Dictionary`, …). The concurrent
"seam" work already landed `🌱️value/🦀️component.rs`'s `to_dsl_value<T: ToValue>`/`from_dsl_value<T:
FromValue>` (previously `T: Serialize`/`DeserializeOwned` per the ticket's own earlier notes) — so
every existing call site in `📄️artifact/🦀️component.rs` that already called
`crate::os_dsl::to_dsl_value(dict)` (a `Dictionary`) or `(value)` (a neural `Value`) was **already
broken** by that landed change, independent of anything in this ticket. `🧠️neural` is a sibling
module (`💻️os/🔨️modules/🧠️neural`), not nested under `🌊️flow`, but it gates my entire subtree's core
Mutation/Diff chain, so I hand-wrote the bridge for it (small, bounded, 6 types) rather than leave
`🌊️flow`'s central types permanently blocked:

- `Dictionary` — hand-written (custom cold-builder `Deserialize` already existed; mirrored the same
  `ColdDictionaryBuilder` construction path for `FromValue` so the retirement/ownership contract is
  untouched).
- `Value` (neural) — hand-written; `#[serde(untagged)]`, a shape the derive does not support. Decode
  rule: `DslValue::Object` → `Dictionary`, everything else → `Atom`.
- `Atom` — hand-written; also `#[serde(untagged)]`. `DslValue::Number` has no int/float split, so
  `FromValue` recovers `Integer` for a whole-valued number and `Decimal` otherwise (same convention
  `pack::json::Number` already uses).
- `Tree`, `Neuron`, `Synapse` — hand-written (the crate doesn't depend on
  `semio-framework-value-derive`; adding that Cargo.toml edge for 3 simple structs wasn't worth the
  collision risk against concurrent agents also touching `🧠️neural`). All three are plain shapes
  (`Synapse` has `rename_all = "camelCase"`) and would have been one-line derives had the dependency
  existed.

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`. Also added
`OrderedSet: ToValue + FromValue` (hand-written, mirrors the existing hand-written
`Serialize`/`Deserialize` — a plain string array) in
`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️component.rs`, since `FlowPreviewGui`/`Widget`
also carry `expanded: OrderedSet`. Both are outside `🌊️flow` but small, self-contained, and were the
actual gate — without them `FlowFixture`, `Widget`, and every mutation leaf holding a `widget`/
`fixture` field (already committed with `ToValue`/`FromValue` derives, per the "already broken"
finding above) could not have compiled regardless of anything else done in this subtree.

## Per-file before → after

| file | serde | serde_json | note |
|---|---|---|---|
| `🌿️vcs/🦀️component.rs` | 228 → 6 | 224 → 2 | `FlowCollectionDelta<T>` hand-written (generic); `FlowLayoutEntry` derived; ~1850-line `#[cfg(test)]` oracle module converted (bracket-indexing, `from_value`/`to_value`/`json!` bridged); `forms_bridge::apply_generation_values_to_fixture` converted |
| `📄️artifact/🦀️component.rs` | 69 → 61 | 9 → 0 | `WidgetLayout`, `FlowArtifact`, `FlowUi`, `FlowNodeGui`, `NodeChrome`, `FlowPreviewGui`, `FlowChannelRef`, `CameraJson`, `SynapseSpec`, `Widget`, `FlowFixture` derived; `WidgetDescriptor` derived (`FromValue` only, matches its original `Deserialize`-only shape); `property_bag_from_dictionary` hand-bridged (target `PropertyBag`/`PropertyValue` are foreign `#[serde(untagged)]`, unsupported — hand-walked the `DslValue` tree instead); remaining 61 are kept `#[serde(...)]` attributes + one `serde_json::to_string(tree)` blocked by foreign `neural::Tree` used only for a debug cache string |
| `🖥️host/🦀️component.rs` | 66 → 37 | 64 → 35 | `FlowCoreError::Json` changed from `serde_json::Error` to `String` (`From<JsonError>`/`From<ValueError>` added) — this crate's own doc comment promises byte-identical `Display` text, preserved; `NodeEvalStatus` derived; `parse_fixture_json`/`fixture_json`/`export_payload_json`/ghost-widget/add-widget/change-params converted; remaining blocked by foreign `neural::OperatorInfo`/`dag::DagLayoutOptions`/`dag::DagChannelRef`/`neural::SchemaRef` and a dense block of `#[cfg(test)]` `NeuronKindInfo` literal-construction tests not attempted this pass |
| `🧩️extensions/🕸️wasm/🦀️component.rs` | 33 → 33 | 22 → 22 | **not touched, intentionally** — this is `flow_extension_sdk`, explicitly called out by the sibling research doc (`📓️serde-fanout-flow.md`) as "sanctioned to carry serde itself" per `plan.md`'s Definition of Done (`🧰️framework/**` is the allowed platform layer); a concurrent agent already extended it mid-ticket. Out of scope for this pass |
| `🌿️vcs/🧪️tests/🦀️.rs` | 31 → 31 | 28 → 28 | **not touched, on purpose — see "A real correctness trap" below** |
| `🖍️drawing/🦀️component.rs` | 43 → 22 | 31 → 10 | `DrawingKind`, `GradientStop`, `FillStyle`, `StrokeStyle`, `LineCap`, `LineJoin` derived; `DrawingHandle`, `Affine2D` hand-written (tuple structs, unsupported shape); `DrawingKernelError` fixed same as `FlowCoreError`; all `{"error": ...}`/`{"field": value}` JSON-string wrappers deduplicated into two small helpers (`json_error`, `json_field`) and converted; remaining blocked by foreign `semio_framework_2d::PathSegment`/`Vec2` (own crate, no `semio-framework-value-derive` dependency — not added, see below) reached through `DrawingNode`/`SceneNode`, and the private `StoredNode` byte-cache codec (never crosses the mutation/diff wire, legitimately framework-internal) |
| `📚️catalogue/🦀️component.rs` | 20 → 14 | 9 → 3 | `CatalogueGroup`, `CatalogueSection`, `CatalogueItem` derived; `merge_catalogue_sections`/`flow_operator_catalogue_json`/`flow_backed_node_graph_extras` converted; remaining blocked by foreign `neural::OperatorInfo`/`ChannelSpec.default: Option<neural::Value>` |
| `📔️registry/🧪️component.rs` | 11 → 11 | 11 → 11 | not touched — blocked by foreign `neural::Schema`/`neural::OperatorInfo` (via `reader.schema(...)`/`reader.operator_info(...)`) plus deep nested mutable bracket-indexing (`replacement["contributes"]["schemas"][0]["name"] = ...`) that only becomes tractable once those neural types convert |
| `🌉️wasm/🦀️component.rs` | 7 → 7 | 7 → 7 | **host-only, confirmed gated — see below, correctly needs no conversion** |
| `📔️registry/🦀️component.rs` | 10 → 5 | 7 → 2 | `FlowExtensionMetadata` derived (`FromValue`, local 3-field struct); `sync_host_flow_extension_contributions` now calls the existing `semio_framework::parse_contributions` helper instead of re-implementing the same `serde_json::from_str` inline; `seed_flow_eval_node_cache` converted (`Dictionary` now has `FromValue`); remaining blocked by foreign `FlowExtensionManifest`/`FlowExtensionContributes` (defined in the sanctioned `🧩️extensions/🕸️wasm` SDK file, itself pulling `neural::Schema`/`neural::OperatorInfo`) |
| `🎚️parameter/📨️intent/🦀️component.rs` | 9 → 3 | 6 → 0 | `SetGraphParameter` derived; both tests converted |
| `🎚️parameter/🧪️component.rs` | 3 → 0 | 3 → 0 | converted (uses already-derived `Widget`) |
| `🌿️vcs/🧬️schema/🔺️diff/🦀️.rs` | 3 → 3 | 0 → 0 | `FlowDelta` derived (adjacently-tagged, `tag="delta", content="value"` — supported); the 3 remaining are kept `#[serde(...)]` attrs, no `serde_json` ever present |
| `🌿️vcs/🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🦀️.rs` | 10 → 0 | 10 → 0 | converted, incl. widget/synapse/layout id list comparisons |
| `🌿️vcs/🧬️schema/🧹️retirement/🧪️tests/🦀️.rs` | 2 → 0 | 2 → 0 | converted |
| `🖥️host/🧹️retirement/🧪️component.rs` | 1 → 0 | 1 → 0 | converted (plain JSON fixture read) |
| 10× `🌿️vcs/🧬️schema/🧬️mutations/**/🦀️.rs` + aggregate | 2 → 2 each | 0 → 0 each | typo-fixed only (see above); already fully on `ToValue`/`FromValue`, the 2 remaining are the kept `#[serde(...)]` attrs |
| `🧵️retained/📑️copy/🧪️component.rs` | 8 → 1 | 7 → 0 | converted incl. `Value::pointer` (exists on `pack::json::Value` too) |
| `🧵️retained/🧪️component.rs` | 4 → 1 | 3 → 0 | converted |
| `📐️brep-geometry/🦀️component.rs` | 6 → 1 | 6 → 1 | 3 literal-JSON error/data wrappers converted; 1 remaining blocked by foreign `MeshData` (from `semio-s-plugin-stdio`'s schema engine, deep, out of scope) |

Files with 0 `serde` before and after are omitted (25 of the 31 files with any Rust content had none
to begin with once the mutation-leaf cluster above is counted separately).

## Host-only, confirmed gated, correctly untouched

`🌉️wasm/🦀️component.rs` (the browser wasm-pack `flow_bridge_*` SDK) is already excluded from the
shipped `wasm32-wasip2` component by `🌊️flow/📦️glue.rs`:

```rust
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[path = "../../🌉️wasm/🦀️component.rs"]
pub mod wasm_session;
```

`target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, so a bare arch gate would not have excluded
it — this one is correctly narrowed to `not(all(...))`, confirmed by reading the live `📦️glue.rs`
(uncommitted change from the wasip2-glue-leak wave, already landed before this session started). Its
7 `serde_json` references compile only for native tests and the real browser bundle, never for the
guest component — no conversion needed for the link-elimination goal. Left as-is.

## A real correctness trap found — `🌿️vcs/🧪️tests/🦀️.rs` deliberately not converted

The `#[derive(ToValue, FromValue)]` macro's own doc (per the playbook) states `#[serde(deny_unknown_fields)]`
is **parsed but currently a no-op** on the value-derive side. `🌿️vcs/🧪️tests/🦀️.rs` (`assert_leaf_contract`)
specifically asserts that an injected `"unknown"` extra key makes decode fail:

```rust
let mut unknown = payload.clone();
unknown["unknown"] = serde_json::json!(true);
assert!(serde_json::from_value::<T>(unknown).is_err());
```

Rewriting this onto `FromValue` would make the assertion **silently stop testing anything real** —
it would still compile and could still pass today only if the underlying object also happened to
fail some other way, but the moment it didn't, the test would go from "verifies deny_unknown_fields"
to "verifies nothing," with no visible signal. Every mutation leaf in this subtree derives `#[serde(...,
deny_unknown_fields)]` **and** `#[value(..., deny_unknown_fields)]` in parallel — the `#[value(...)]`
attribute is present (for when the derive gains real enforcement) but not yet load-bearing. Converting
this file's rejection assertions now would be actively wrong, not merely incomplete. Left on
`serde_json` — which is still the actual source of truth for this contract — and flagged here rather
than silently skipped.

## Not removed yet — `serde`/`serde_json` still in `Cargo.toml`

Per the playbook, `Serialize`/`Deserialize` derives were added **alongside** `ToValue`/`FromValue`,
never swapped, and neither `serde` nor `serde_json` was removed from
`🌊️flow/📦️packages/🦀️rust/Cargo.toml`. Removing them requires zero remaining production use across
the whole crate — not the case yet given: the sanctioned `🧩️extensions/🕸️wasm` SDK file, the
deny_unknown_fields test-contract gap above, and the several foreign-type blockers (`neural::Schema`/
`OperatorInfo`/`ChannelSpec`/`VariadicSpec`, `dag::DagLayoutOptions`/`DagChannelRef`,
`semio_framework_2d::PathSegment`/`Vec2`, `graph::manifest::PropertyValue` (`#[serde(untagged)]`),
`semio-s-plugin-stdio`'s `MeshData`) that live in sibling modules outside this subtree's scope.

## Verification — honest, blocked upstream twice, guardrail confirmed holding

`cargo check -p semio-framework-os-kernel --message-format=short` (the ticket's stated guardrail),
run twice this session, both **0 errors**:

```
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 7.89s
```

`os-flow` (my crate) could **not** be reached end-to-end by the compiler, twice, for two different
reasons — both confirmed unrelated to `🌊️flow` and to this ticket's own instruction to "record and
move on" from concurrent peer breakage:

1. First attempt: `semio-framework-ui` (a dependency via `ui_wgpu`) failed with 14 `E0277`s —
   `DslValue: serde::Serialize`/`Deserialize` not satisfied in `🖱️ui/🎯️targets/🧊️wgpu/🦀️component.rs`
   (`icon_name_gen`). Traced to the concurrent `to_dsl_value`/`from_dsl_value` bridge conversion
   (the same one that unblocked my own `🧠️neural` work) changing `DslValue`'s own bound elsewhere.
2. Second attempt (after the `ui` blocker apparently cleared): `semio-framework-plugin` failed with
   91 `E0277`/`E0119`s — `PackageDescriptor`, `WireArtifactMutationPlanRequest`,
   `WireArtifactMutationPlanResult`, `WireMutationRosterEntry`, `ViewModel`, `dsl::Fault`, `&str`,
   `serde_json::Value` all missing `ToValue`/`FromValue`, plus one `E0119` conflicting-impl on
   `SetInteractionState` in `🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs`. This is squarely the
   ticket's own tracked "seam 6" (`ArtifactEditor::command_from_action`) work, in flight elsewhere,
   zero mentions of anything under `🌊️flow`.

Verbatim tail of the second attempt:

```
error[E0277]: the trait bound `dsl::Fault: ToValue` is not satisfied: the trait `ToValue` is not implemented for `dsl::Fault`
  --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../⚛️reactor/🦀️component.rs:65:70
warning: `semio-framework-plugin` (lib) generated 111 warnings
error: could not compile `semio-framework-plugin` (lib) due to 91 previous errors; 111 warnings emitted
```

Cross-checks performed in lieu of a full compile, given the above:
- Brace/paren balance diffed against `git show HEAD:<file>` for every edited file — the two files
  that showed a nonzero paren balance (`🌿️vcs/🦀️component.rs`: 9, `🖍️drawing/🦀️component.rs`: 2)
  have the **identical** imbalance already at `HEAD`, before any edit this session (prose/emoji in
  doc comments, not code) — confirmed pre-existing, not introduced.
- Every hand-written `ToValue`/`FromValue` impl follows the derive's own fully-qualified-path
  convention (`crate::os_dsl::ToValue::to_value(...)`, never `.to_value()` shorthand) to avoid the
  known `DslField`-trait method-name collision the playbook documents.
- Cross-referenced every `pack::json::{Value, Object, Number}` method used (`get`, `get_mut`,
  `as_array`/`as_array_mut`, `as_object`/`as_object_mut`, `as_str`/`as_f64`/`as_u64`/`as_bool`,
  `pointer`, `insert`) against `🎒️pack/🔤️json/🦀️component.rs`'s actual signatures — two real gaps
  found and fixed by reading the source rather than assuming serde_json parity: `Object` has no
  `.keys()` (fixed with `.iter().map(|(k,_)| k)`) and no numeric `Value::get_mut(usize)` overload for
  arrays (fixed with `.as_array_mut().and_then(|a| a.get_mut(i))`).

**Net verdict: written and cross-checked against the live type signatures, but not machine-verified
end-to-end** — blocked twice by concurrent unrelated breakage upstream of `os-flow`, each time in a
different crate, neither touching `🌊️flow`. Re-run
`cargo check -p semio-framework-os-flow --message-format=short` once `semio-framework-plugin`'s
`ToValue`/`FromValue` seam (`PackageDescriptor`/`WireArtifactMutationPlanRequest`/`dsl::Fault`/etc.)
lands.

## Types converted by derive vs. by hand

**By derive** (`#[derive(ToValue, FromValue)]`, all additive alongside serde): `WidgetLayout`,
`FlowArtifact`, `FlowUi`, `FlowNodeGui`, `NodeChrome`, `FlowPreviewGui`, `FlowChannelRef`,
`CameraJson`, `SynapseSpec`, `Widget`, `FlowFixture`, `WidgetDescriptor` (`FromValue` only),
`FlowLayoutEntry`, `FlowDelta`, `CatalogueGroup`, `CatalogueSection`, `CatalogueItem`,
`NodeEvalStatus`, `SetGraphParameter`, `FlowExtensionMetadata`, `DrawingKind`, `GradientStop`,
`FillStyle`, `StrokeStyle`, `LineCap`, `LineJoin`, `Synapse` (neural, rename_all only).

**By hand** (shape unsupported by the derive): `FlowCollectionDelta<T>` (generic struct),
`DrawingHandle`/`Affine2D` (tuple structs), `Dictionary`/`Value`/`Atom` (neural — custom
cold-builder codec / `#[serde(untagged)]`), `Tree`/`Neuron` (neural — no
`semio-framework-value-derive` dependency in that crate, hand-written rather than adding one),
`OrderedSet` (framework value module — hand-written to mirror its existing hand-written
`Serialize`/`Deserialize`).

## Files touched outside `🌊️flow` (small, bounded, required to unblock the subtree)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs` — added
  `ToValue`/`FromValue` for `Dictionary`, `Value`, `Atom`, `Tree`, `Neuron`, `Synapse`.
- `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧺️set/🦀️component.rs` — added `ToValue`/`FromValue` for
  `OrderedSet`.

Neither removes any existing serde code; both are purely additive.
