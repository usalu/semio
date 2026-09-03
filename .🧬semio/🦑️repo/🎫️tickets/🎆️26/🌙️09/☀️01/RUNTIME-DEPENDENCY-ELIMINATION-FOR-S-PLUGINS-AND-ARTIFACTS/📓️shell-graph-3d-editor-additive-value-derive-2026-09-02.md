# Shell / Graph / 3d / Editor — Additive `ToValue`/`FromValue` Pass (2026-09-02)

Scope: the four modules assigned this session —
`🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/`,
`🧰️framework/🔨️modules/🕸️graph/`, `🧰️framework/🔨️modules/🧊️3d/`,
`🧰️framework/🔨️modules/✍️editor/`. ADDITIVE ONLY — no `Serialize`/`Deserialize` removed anywhere
(confirmed via `git diff | grep -i serde` across all four: zero removed lines).

## Before / after (finder script exit)

Finder: `find <module> -name '*.rs' -not -path '*🧪*' -not -path '*🏭*' -not -path '*🔬*' | xargs grep -ln '#\[derive(' | while read f; do grep -q ToValue "$f" || echo "$f"; done`

| Module | Files uncovered before | Files uncovered after | Notes |
|---|---|---|---|
| 🖥️shell | 1 (`🦀️.rs`, 36 derive sites) | 0 | |
| 🕸️graph | 3 (`🖊️drawing`, `🧮️algorithms`, `⚙️engine`) | 9 (all `🤖️generated/*.rs`) | The 9 remaining are the SAME 21-enum set a prior agent's `🛂️manifest/🦀️generated-value-bridge.rs` already covers file-externally — the finder script can't see cross-file coverage. Verified byte-identical enum-name match against the bridge before/after (see below). |
| 🧊️3d | 4 (`🥽️mesh`, `🌀️rigid`, `🧿️collision`, `⚙️engine`) | 0 | |
| ✍️editor | 1 (`🦀️.rs`, 11 derive sites) | 0 | |

`cargo check -p semio-framework` (framework gate): 0 errors before, 0 errors after.
`cargo check -p semio-framework-editor -p semio-framework-graph -p semio-framework-3d -p semio-framework-os-shell`: 0 errors, 0 warnings in any touched file.

## Per-module detail

### 🖥️shell (`semio-framework-os-shell`)
29 types covered via `#[derive(ToValue, FromValue)]` + `#[value(crate = "::protocol::value", ...)]`
mirroring each `#[serde(rename_all/tag/rename_all_fields)]` container attribute exactly (all 28
`#[serde(...)]` attrs in this file are container-level only — no field-level attrs at all besides
the ones the JsonValue bridges below add). 2 transparent newtypes (`IconName`, `AppRole`). 6
`JsonValue`/`Option<JsonValue>`/nested-`HashMap<..,JsonValue>` fields bridged via
`#[value(with = "...")]` (`json_value_bridge`, `json_value_option_bridge`,
`staged_command_args_bridge`, `staged_action_args_bridge`) since `ToValue`/`FromValue` isn't
implemented for `serde_json::Value` anywhere reachable (orphan rule).

**`ShellCommand` (63 variants) is a hand-written `impl ToValue`/`impl FromValue`, not derived** —
3 variants (`StageActionArg.value`, `StageCommandArg.value`, `OpenDialog.seed_args`) carry a
`JsonValue` field on an ENUM VARIANT, and `#[value(with = "...")]` is deliberately unsupported
there (derive docs: "flatten/with/serialize_with/deserialize_with on an enum variant's own named
field remain Deliberately NOT supported"). Hand-write mirrors the derive's own
`tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase"` shape exactly — proven
byte-identical to `serde_json::to_value` in the round-trip test below (never dispatches back
through `ToValue`/`FromValue` on `ShellCommand` itself, per the ticket's infinite-recursion rule).

Deliberately excluded: `schema_metadata::SchemaMetadata` (`#[cfg(feature = "typegen")]`,
`&'static str` fields, never had `serde`, purely compile-time codegen metadata never crossing any
wire — `FromValue` would need an owned `Self` a decoded string can't honestly become without
leaking).

Cargo.toml gained `semio-framework-value-derive` (path dep) + `semio-framework-replication`
(workspace dep) — this crate is intentionally kernel-free ("pure, no I/O, no clock"), so it can't
reach the derive's default `::semio_framework_os_kernel` crate path.

Round-trip test added: `component::tests::value_round_trip_matches_serde_shape` — covers plain
derive types, JsonValue bridges (present/absent), a populated `ShellState`, and `ShellCommand`
including all 3 blocked-JsonValue variants, PLUS an explicit `assert_eq!` against
`serde_json::to_value` for `StageActionArg` proving the hand-written bridge's wire shape is
byte-identical to what serde would have produced.

Tests: 11/11 pass (10 pre-existing + 1 new), including the pre-existing `write_fixtures`
(`#[ignore]`) and `fixtures_produce_expected_output` parity tests — unaffected by this change.

### 🕸️graph (`semio-framework-graph`)
- `🖊️drawing/🦀️.rs`: 1 type (`ForceLayoutOptions`) — plain derive, never had `serde`.
- `🧮️algorithms/🦀️.rs`: 4 types covered (`Adjacency`, `IdIndex`, `CycleError`, `UnionFind`) — all
  plain derive. 1 documented skip: `OrderedFloat(f64, usize)` — private, module-internal Dijkstra
  sort key, never crosses a wire, and a 2-field tuple struct has no `#[value(...)]` equivalent at
  all (only single-field newtype tuple structs are supported, via `transparent`).
- `⚙️engine/🦀️.rs` (2489 → ~2650 lines): 18 types covered, 2 documented skips.
  - `CoreEdge<E>`, `NodeRecord`, `EdgeRecord<E>`: plain derive (`E` is always `NodeId`/`HandleId` =
    `u64` in practice, so the derive's auto `E: ToValue + FromValue` bound always holds).
  - `Directed`, `Undirected`, `Normal`, `Ported`, `UnitWeight`: **hand-written**, not derived — the
    derive rejects a SEMICOLON-terminated unit struct (`Fields::Unit`) outright
    ("`#[derive(ToValue, FromValue)]` supports named-field structs ..., not tuple/unit structs");
    changing to brace form (`struct Foo {}`) would ripple into the one value-level construction
    site (`let w = UnitWeight;`) and is avoidable — hand-write is a 6-line `DslValue::Null`
    round-trip per type instead.
  - `Storage<P: PortModel, D: Directedness>`: plain derive with `#[value(bound =
    "P::Endpoint: dsl_core::ToValue + dsl_core::FromValue")]` (overrides the auto per-type-param
    bound, since `P`/`D` themselves are never touched — both live only behind a `#[value(skip)]`
    `PhantomData` field) + FOUR hand-written `u64`-keyed `BTreeMap` `with`-bridges
    (`storage_nodes_bridge`, `storage_edges_bridge` — generic over `E`, `storage_adjacency_bridge`,
    `storage_handle_owner_bridge`) following the ticket's own `ShardTable`/`actor_shard_map_to_value`
    precedent, since `NodeId`/`EdgeId`/`HandleId` are bare `u64` (not `String`), so none of them can
    use the `BTreeMap<String, T>`-only blanket impl.
  - `EdgeRef`, `UnitWeight`(above), `Csr` (+ `csr_node_index_bridge`, same `u64`-key pattern),
    `GraphError`, `NodeShape`, `HandleRole`, `ElementSemantics`, `Node`, `Handle`, `FlowEdge`,
    `FlowNetwork`: plain derive. `Node`/`Handle.center` bridges through a hand-written
    `point_bridge` — `geometry::Point` (`📐️geometry`, a different owner's module) has no
    `ToValue`/`FromValue` and was NOT edited; the bridge uses only `Point`'s existing public
    `x`/`y` fields.
  - **`GraphError::NotImplementedForKind`'s two fields changed `&'static str` → `String`** (both
    call sites: the `Display` impl and one test literal, `.to_string()`-ed) — `FromValue` needs an
    owned `Self`, and no runtime-decoded string can honestly become `&'static str` without leaking
    memory; a greenfield, no-legacy-API repo (CLAUDE.md) fixes the field type instead of working
    around it with `Box::leak`.
  - Documented skips: `Interner<L: Ord + Clone + Hash>`, `MappedHeap<K: Ord, V: Eq + Hash + Clone>`
    — both are ONLY ever instantiated inside this file's own `#[cfg(test)] mod tests` (confirmed by
    repo-wide grep: zero external consumers anywhere), `MappedHeap`'s one test instantiation is
    `MappedHeap<i64, &str>` (a BORROWED `V`, which structurally cannot implement `FromValue`), and
    both would need a `#[value(bound = "L: ToString + FromStr + ...")]` override (the blanket
    `HashMap`/`BTreeMap` impls need `K: ToString`, which the derive's auto `L: ToValue + FromValue`
    bound does not provide) — a real API constraint added for zero present benefit.

Round-trip test added: `engine::tests::value_round_trip_matches_serde_shape` — covers marker
types (`Null` round-trip, no `PartialEq` so checked by re-encode-and-compare), `EdgeRef`,
`GraphError::NotImplementedForKind`, `Node`/`Handle` (exercising `point_bridge`), and a POPULATED
`Storage<Ported, Directed>` (2 nodes, 2 handles, 1 edge — exercises all four `u64`-keyed `BTreeMap`
bridges) plus a `Csr` built from that storage (exercises `csr_node_index_bridge`).

Tests: 183/183 pass (182 pre-existing + 1 new).

**Pre-existing bug found, NOT fixed (out of scope, flagged as a separate background task):**
`🛂️manifest/🦀️generated-value-bridge.rs` (written by a prior agent, not touched this session) has
`format!("unknown X `{{other}}`")` — DOUBLED/escaped braces — at all 21 "unknown variant" error
sites, so the literal text `{other}` prints instead of the actual bad value (`other` binding is
never used, confirmed by 21 "unused variable: `other`" warnings). Functionally harmless
(`FromValue` still correctly returns `Err`), but the messages are wrong. Flagged via
`spawn_task` (task_2ad330b1) rather than fixed here since it's a different agent's file and outside
this session's assigned scope.

### 🧊️3d (`semio-framework-3d`)
Crate root (`📦️packages/🦀️rust/🦀️.rs`) gained `extern crate protocol as dsl_core;` +
`extern crate semio_framework_value_derive as value_derive;` (placed AFTER the crate's `//!` inner
doc comments — `extern crate` before them is `E0753`). `dsl_core` resolves to `protocol::value`
(`semio-framework-replication`, added unconditional) rather than `semio-framework-os-kernel`
because os-kernel is feature-gated behind `brep` here (default-on, but every `#[value(...)]`
container names `crate = "::protocol::value"` explicitly rather than relying on a feature-dependent
default).

- `🥽️mesh/🦀️.rs`: 13 types covered. `Vec3`, `VertexId`, `HalfEdgeId`, `FaceId`, `EdgeId`
  (all newtype tuple structs) use `transparent`. `HalfedgeMesh.uv_seams: HashSet<u32>` bridges
  through a hand-written `u32_hashset_bridge` (no `HashSet` blanket impl exists in
  `🌱️value/🔁️codec`). All `#[serde(default)]`/`#[serde(default = "path")]` field attrs mirrored
  1:1 as `#[value(default)]`/`#[value(default = "path")]`.
- `🌀️rigid/🦀️.rs`: 5 types (`Vector3`, `Point3`, `Quaternion`, `UnitQuaternion` (transparent
  newtype), `Isometry3`) — plain derive, never had `serde`.
- `🧿️collision/🦀️.rs`: 1 type (`Aabb3`, private) — plain derive.
- `⚙️engine/🦀️.rs`: 5 types (`Aabb`, `ParamDomain`, `FaceGroup`, `MeshTransfer`,
  `PointClassification`) — plain derive/mirror, no gaps.

Round-trip test added: `mesh::tests::value_round_trip_matches_serde_shape` — covers `Vec3`,
`VertexId`, `WeldMode`, `MeshKernelError`, and a REAL non-empty `HalfedgeMesh` from `box_prim`
(exercises `u32_hashset_bridge` with actual seam data), with an explicit `assert_eq!` against
`serde_json::to_value` proving byte-identical wire shape (`HalfedgeMesh` has no pre-existing
`PartialEq`, so fidelity is checked by re-encoding the decoded value and by the serde comparison,
not `assert_eq!(decoded, original)`).

Tests: 84/84 pass (83 pre-existing + 1 new).

### ✍️editor (`semio-framework-editor`)
Cargo.toml gained `semio-framework-value-derive` (path dep) — `semio-framework-os-kernel` was
already an unconditional dependency, so the derive's default `::semio_framework_os_kernel` crate
path resolves without a `#[value(crate = "...")]` override.

9 types covered: `EditorCanvasTheme` (private, all 7 fields `Color` — bridged through a
hand-written `color_bridge` using `Color`'s existing public `components()`/`new()`, since `Color`
lives in `♾️infinite`, a different owner's module, and was not touched) + 8 `...Json` structs
(`EditorSettingsJson`, `SemanticTokenJson`, `SelectableSpanJson`, `ByteRangeJson`,
`PlaceholderJson`, `DiagnosticJson`, `TextEditJson`, `TextRangeJson`, `TextPosJson`) — each derives
`FromValue` ONLY (not `ToValue`), because every one of these is `#[derive(..., Deserialize)]`-only
in the existing code (one-way WASM-boundary JSON parsing that is never serialized back out) — the
additive twin mirrors exactly what serde derives, not more. `#[serde(rename_all/rename/default =
"path")]` mirrored 1:1 as `#[value(...)]`.

Deliberately excluded: `EditorError` — both variants wrap a genuinely foreign, non-data error type
(`serde_json::Error`, opaque parser diagnostic state with no `ToValue`/`FromValue` anywhere; and
`store::PackError`, another owner's module) and `EditorError` itself never crosses a wire (an
internal `Result<_, EditorError>` — the actual WASM boundary returns `JsValue` per the type's own
docstring).

Tests: 82/83 pass. The 1 failure (`component::tests::char_boundary_helpers_handle_multibyte`,
emoji/UTF-8 boundary arithmetic) is CONFIRMED PRE-EXISTING and unrelated — `git diff` shows zero
changes anywhere near that test or the `next_char_boundary`/`prev_char_boundary` functions it
exercises; not touched or caused by this session's work.

## Environment notes
- `iso3` target dir was shared/flaky mid-session: a concurrent session renamed
  `🤖️generated/🦀️draw_layers.rs` → `🦀️drawing_layers.rs` while `🛂️manifest/🦀️generated-value-bridge.rs`
  (a prior agent's file, not mine) still referenced the old name, producing transient
  `E0433: cannot find draw_layers in generated` errors across `-p semio-framework-graph` AND
  everything that transitively built it (`-p semio-framework-editor` via `cargo test`, which for
  unclear reasons pulls graph into its test build though `cargo check -p semio-framework-editor`
  alone does not). Confirmed via `ls` that the old path was genuinely gone, and via a fresh retry a
  few minutes later that the churn resolved itself (the other session finished the rename +
  updated the bridge file) — 0 errors on a clean re-run. Not caused by, or fixed by, this session.
- Also hit (and fixed, in MY OWN newly-added bridge code only): `extern crate X as alias;`
  declared at a crate root is NOT reliably reachable from a nested submodule via `use
  super::alias;` unless that submodule is a DIRECT child of the crate root — `use crate::alias;`
  (explicit, unambiguous) or a bare `alias::` reference (relies on the extern prelude) both work
  reliably regardless of nesting depth; used the explicit `crate::`/local-`use`-inside-the-bridge-
  module form throughout after the first failure in `🧊️3d/🥽️mesh`'s `u32_hashset_bridge`.

## Verification commands run
```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework --message-format short                                    # 0 errors
cargo check -p semio-framework-editor -p semio-framework-graph -p semio-framework-3d \
  -p semio-framework-os-shell --message-format short                                      # 0 errors, 0 warnings in touched files
cargo test -p semio-framework-os-shell --lib     # 11/11
cargo test -p semio-framework-graph --lib        # 183/183
cargo test -p semio-framework-3d --lib           # 84/84
cargo test -p semio-framework-editor --lib       # 82/83 (1 pre-existing, unrelated failure)
git diff -- <the four modules> | grep -i serde   # zero removed Serialize/Deserialize lines
```
