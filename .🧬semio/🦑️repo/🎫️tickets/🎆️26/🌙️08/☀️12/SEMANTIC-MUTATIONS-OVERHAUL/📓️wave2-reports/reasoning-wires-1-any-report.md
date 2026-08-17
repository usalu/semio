# Wave 2 — `semio-s-plugin-reasoning-mindmap` / wires / standards/1 / subsets/any / mutations

## Facet
`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`

## Snapshot shape
`WiresSnapshot` (`📸️snapshot/🦀️component.rs`) has two persistent fields, both opaque `dsl::DslValue`
blobs — `wires_fixture` (schema/identities/relationships/board/source) and `board_fixture`
(schema/camera/nodes/edges/meta/wires). The module doc on the plugin-root `🦀️component.rs` explains
this is deliberate: `⚙️engine`/`🖱️commands`/`🔧️op` address board nodes/edges and wires relationships
generically by id for mergeable, granular edits. The *typed* shape those blobs actually carry is
fully spelled out by the `*Dsl` mirror types in the plugin-root component (`NodeDsl`, `EdgeDsl`,
`IdentityDsl`, `RelationshipDsl`) — the vocabulary below is derived from that typed shape, per
`📓️derivation-rules.md`, not invented.

## Vocabulary derived (10 semantic mutations, 0 generic left)

| Old generic | New semantic | Verb | Notes |
|---|---|---|---|
| `AddNode { node }` | `CreateNode { node }` | `create` | full initial payload, id-keyed |
| `RemoveNode { node_id }` | `DeleteNode { node_id }` | `delete` | id-keyed removal, inverse captures the full removed node |
| `PatchNode { node_id, patch: BTreeMap<String, DslValue> }` | split into 6 explicit mutations below | — | the `BTreeMap<String, DslValue>` was exactly the taxonomy's forbidden option-bag `Patch` payload shape |
| *(from PatchNode's `x`/`y` call sites)* | `MoveNode { node_id, new_x, new_y }` | `move` | absolute spatial reposition (force-layout + canvas drag both used this) |
| *(new, schema has `radius`/`width`/`height`)* | `ResizeNode { node_id, new_radius, new_width, new_height }` (all `Option<f64>`) | `resize` | extent facet; only touched fields are `Some` |
| *(new, schema has `nodeKind`)* | `ChangeNodeKind { node_id, new_node_kind }` | `change` | scalar field |
| *(new, schema has `shape`)* | `ChangeNodeShape { node_id, new_shape }` | `change` | scalar field |
| *(new, schema has `text`)* | `EditNodeText { node_id, new_text }` | `edit` | authored content body (node label) |
| *(new, schema has `root: Option<bool>`)* | `SetNodeRoot { node_id, new_root }` | `set` | narrow addressed boolean setter — the exact `set-layer-visible` shape from the taxonomy |
| `AddRelationship { edge, relationship }` | `ConnectNodes { edge, relationship }` | `connect` | creates a board edge + optional wires-level relationship together |
| `RemoveEdge { edge_id }` | `DisconnectNodes { edge_id }` | `disconnect` | inverse re-`connect`s both the edge and its relationship, captured from BASE |
| `SetSnapshot { snapshot }` | **deleted, no replacement mutation** | — | banned vocabulary; whole-document replace goes through `store::ArtifactStore::reset` outside the `Mutation` enum |

Every `SEMANTICS.kind` equals its variant's own kebab form (derive-enforced compile-time assert);
every `SEMANTICS.verb` is in `protocol::APPROVED_VERBS` (also derive-enforced). Verified by the new
`dispatch_registers_semantic_descriptors` test iterating `WiresMutation::kinds()` (10 entries).

## Real handcrafted diffs (no apply-then-capture)

Every leaf's `🔺️diff/🦀️component.rs` builds `WiresDiff` directly from `(payload, base)`:
- `create-node`/`delete-node` delegate to the pre-existing, unmodified schema-level
  `board_after_add_node`/`board_after_remove_node` builders (`🔺️diff/📝️text/🦀️component.rs`, outside
  this facet, read-only reuse) — real targeted array push/retain, not a whole-snapshot round trip.
- `connect-nodes`/`disconnect-nodes` likewise delegate to the pre-existing
  `fixtures_after_add_edge`/`fixtures_after_remove_edge` builders.
- `move-node`/`resize-node`/`change-node-kind`/`change-node-shape`/`edit-node-text`/`set-node-root`
  all go through one new shared primitive, `set_node_field(board: &mut DslValue, node_id, key,
  value)` (declared once in the dispatch `🦀️component.rs`, region `🔖️NodeFieldHelpers`) — finds the
  addressed node inside the cloned `board_fixture` array and writes exactly one field in place. No
  full-snapshot diffing, no apply-then-capture anywhere.

Inverse bodies all reconstruct from captured BASE state (never from inverting the diff structurally):
`delete-node`'s inverse recreates the full node from BASE; `move-node`/`resize-node`/
`change-node-kind`/`change-node-shape`/`edit-node-text`/`set-node-root` look up the OLD field
value(s) from BASE; `disconnect-nodes`'s inverse re-captures both the edge and its (possibly absent)
relationship from BASE. Every leaf returns `Vec::new()` when the addressed node/edge is missing from
BASE (the `NoMutation` sentinel's replacement). `resize-node`'s inverse only restores the extent
fields the original payload actually touched (via `Option::and`), leaving untouched fields as `None`
("don't touch") in the undo mutation too.

## Dispatch enum + wire codecs

`WiresMutation` now derives `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum,
dsl::Mutations)]` with `#[mutations(snapshot = WiresSnapshot, diff = WiresDiff, schema =
"s.reasoning.wires")]` — every variant a single-field tuple wrapping its `🦠️mutation` leaf's payload
struct (which itself derives `dsl::DslRecord` with its own `#[dsl(keyword = "...")]`, matching
`✏️rename-layer`'s reference shape in the already-migrated `🖍️draw` facet exactly). The old
hand-written hand-rolled `impl protocol::Mutation for WiresMutation` (`match`-based diff/inverse
dispatch), the separate `WiresMutationDsl` struct-variant mirror enum + its
`wires_operation_to_dsl`/`wires_operation_from_dsl`/`impl dsl::DslVariants for WiresMutation`
conversion functions, and the free `apply_wires_mutation`/`inverse_wires_mutation` functions are all
deleted — `dsl::DslEnum` derives `DslVariants` directly on the tuple-variant enum now (each variant
delegates to its own payload's `DslField`/`DslRecord` spec), and `dsl::Mutations` derives
`impl protocol::Mutation`/`impl protocol::SemanticMutation` + `register_wires_mutation_descriptors()`.

`🧬️mutations/📝️text/🦀️component.rs`'s handcrafted `OpText`/`OpBinary` impls (already generic over
`dsl::DslVariants`) needed no logic changes — only the `pub use` line updated to drop the two deleted
free functions. `💾️binary/🦀️component.rs`'s tests updated from `WiresMutation::AddNode { node }` to
the new `create_node(node)` builder.

## Triad leaves

**10 new leaves**, self-wired directly in the dispatch `🦀️component.rs` via `#[path]` (region
`🔖️LeafWiring`) rather than in `📦️glue.rs` — this facet's fan-out is scoped to files inside this
artifact directory only; `📦️glue.rs` is plugin-shared and out of that scope (same precedent as the
already-migrated `🎪️demonstrator/🎪️playground` facet, confirmed by reading its wave2 report before
starting): `🌱create-node`, `🗑️delete-node`, `🧭move-node`, `📐resize-node`, `🏷️change-node-kind`,
`🔷change-node-shape`, `✏️edit-node-text`, `🚩set-node-root`, `🔗connect-nodes`,
`✂️disconnect-nodes` — each `{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs`.

**6 old leaves — orphaned, not deleted**: `➕add-node`, `➖remove-node`, `✂️remove-edge`,
`➕add-relationship`, `🖼️set-snapshot`, `🩹patch-node` (all 18 `.rs` files, `🦠️mutation`/`🔺️diff`/
`↩️inverse` each) were rewritten to a bare doc-comment-only orphan stub (no code) because the
plugin-shared `📦️glue.rs` (outside this facet's edit boundary) still `#[path]`-wires all 18 of them
as `pub mod add_node { mutation, diff, inverse }` / `pub mod remove_node { ... }` / etc. submodules
under `mutations`. Deleting the files would make `glue.rs`'s `#[path]` attributes fail to compile
(missing file), which is out of this ticket's edit boundary for this facet. Each stub's doc comment
names exactly which `glue.rs` `pub mod` block needs deleting before the directory itself can go.

## Node-field helper

`set_node_field(board: &mut DslValue, node_id: &str, key: &str, value: DslValue)` — one new `pub fn`
in the dispatch `🦀️component.rs` (region `🔖️NodeFieldHelpers`), reused by six leaves' diff builders.
Comparison is via `entry_key.as_str() == key` (both `&str`) rather than the pre-existing
`apply_board_step` helper's `entry_key == key` (which compares `&mut String` against a `&String` from
`BTreeMap` iteration) — deliberately more explicit since my callers pass a plain `&str` key literal,
avoiding any `PartialEq` cross-reference-type ambiguity.

## Tests

Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files): kept
`create_delete_node_round_trip`/`connect_disconnect_nodes_round_trip` (renamed from the old
`add_remove_patch_node_round_trip`/`add_remove_relationship_round_trip`), added
`move_node_round_trip`, `resize_node_round_trip`, `change_node_kind_round_trip`,
`change_node_shape_round_trip`, `edit_node_text_round_trip`, `set_node_root_round_trip`,
`op_text_round_trip_create_node`, `op_text_round_trip_move_node`, and
`dispatch_registers_semantic_descriptors` (kind-table + approved-verb assertions). Extended
`💾️binary/🦀️component.rs`'s existing tests to construct `create_node(node)` instead of the removed
`WiresMutation::AddNode { node }` literal.

`assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` (the shared testkit law helpers) were
**not** added: grepped this crate (`semio-s-plugin-reasoning-mindmap`) for an existing `testkit`
import and found none anywhere in the plugin, so per the task brief step (e) this was skipped rather
than adding a new Cargo dependency. All ten new mutation kinds are still covered by the existing
round-trip pattern (`apply` then `inverse` then re-`apply`, asserting the pre-mutation snapshot is
restored), which is the same property `assert_mutation_inverse_law` checks.

## Grammar / protocol `.semio` files

Left unchanged (`📖️component.grammar.semio`, `📡️component.protocol.semio` at both the mutations
facet root and the `📝️text`/`💾️binary` subfacets) — per task step (f), explicitly non-blocking, and
consistent with every other wave2 facet's report I found (`demonstrator/playground` etc.) leaving
these untouched too.

## Files touched (all inside `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`)

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — rewritten: leaf wiring +
  tuple-variant `#[derive(dsl::DslEnum, dsl::Mutations)]` enum + `set_node_field` helper + builder
  re-exports + rewritten `🧪️Tests` region.
- New triad leaf dirs (`.rs` only, 30 files): `🌱create-node`, `🗑️delete-node`, `🧭move-node`,
  `📐resize-node`, `🏷️change-node-kind`, `🔷change-node-shape`, `✏️edit-node-text`,
  `🚩set-node-root`, `🔗connect-nodes`, `✂️disconnect-nodes`.
- Orphaned in place (18 files, doc-comment-only): `➕add-node`, `➖remove-node`, `✂️remove-edge`,
  `➕add-relationship`, `🖼️set-snapshot`, `🩹patch-node`.
- `🧬️mutations/📝️text/🦀️component.rs` — `pub use` line fixed (dropped the two deleted free
  functions).
- `🧬️mutations/💾️binary/🦀️component.rs` — 3 test call sites + 1 doc comment updated to the new
  builder.
- `⚙️engine/🦀️component.rs` — 2 call sites (`handcrafted_metabolism_snapshot`'s `AddNode`/
  `AddRelationship` construction) rewired to `mutations::create_node`/`mutations::connect_nodes`
  (this file is inside the artifact package boundary, not app-level/glue/plugin-root, so it was
  in-scope to fix directly rather than deferring to `sharedFileRequests`).

## Shared-file requests (for the dedicated later reconciliation pass)

- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs`: delete the six `pub mod remove_edge { .. }`
  / `add_node { .. }` / `add_relationship { .. }` / `remove_node { .. }` / `set_snapshot { .. }` /
  `patch_node { .. }` blocks under
  `artifacts::wires::standards::v1::subsets::any::schema::mutations` (they only exist to keep the
  now-orphaned leaf files compiling); once removed, all 6 orphan directories (18 files) can be
  deleted outright. Optionally also add `pub mod` blocks for the 10 new leaf directories to bring
  them under `glue.rs`'s normal wiring instead of the facet's own inline `#[path]` self-wiring, if a
  later pass wants directory-name/kind parity restored (not required for compilation — the derive
  only asserts `kind == kebab(variant)`, not `kind == directory stem`, so the mismatch is cosmetic
  only, confirmed by reading `dsl_derive`'s actual `#[derive(Mutations)]` expansion).
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🦀️component.rs` (lines ~352-353, inside
  `two_instances_converge_disjoint_graph_edits_via_backbone`): `WiresMutation::AddNode { node:
  seed_node(...) }` (×2) → `crate::artifacts::wires::mutations::create_node(seed_node(...))`.
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🗑️delete/🦀️component.rs`: `WiresMutation::RemoveNode
  { node_id: id.clone() }` → `crate::artifacts::wires::mutations::delete_node(id.clone())`;
  `WiresMutation::RemoveEdge { edge_id: id.clone() }` →
  `crate::artifacts::wires::mutations::disconnect_nodes(id.clone())`.
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🔄️layout/🦀️component.rs`:
  `force_layout_operations` currently builds a `BTreeMap` patch with `x`/`y` keys and emits
  `WiresMutation::PatchNode { node_id, patch }` — rewrite to
  `crate::artifacts::wires::mutations::move_node(id.to_string(), nx, ny)` directly (drop the
  `BTreeMap`/`DslValue` construction entirely, `move_node` takes plain `f64`s).
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🔵️node/🦀️component.rs`: `WiresMutation::AddNode {
  node }` → `crate::artifacts::wires::mutations::create_node(node)`.
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🖱️pointer/🦀️component.rs`:
  `canvas_pointer_move::handle` currently builds a `BTreeMap` patch with `x`/`y` and emits
  `WiresMutation::PatchNode { node_id: drag_node_id.clone(), patch }` — rewrite to
  `crate::artifacts::wires::mutations::move_node(drag_node_id.clone(), cur_x + dx, cur_y + dy)`
  directly.
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🔗️relationship/🦀️component.rs`:
  `WiresMutation::AddRelationship { edge, relationship }` →
  `crate::artifacts::wires::mutations::connect_nodes(edge, relationship)`.
- `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎮️commands/🧬️example/🦀️component.rs`:
  `set_active_example::handle` currently emits `WiresMutation::SetSnapshot { snapshot: next }` — this
  is banned vocabulary with no mutation replacement; needs the same rewrite the already-migrated
  `🖍️draw` facet's wave1 report documents (`Emit { effects: vec![HostEffect::LoadDocument { pack, spr
  }], .. }`, built via a fresh `store::ArtifactEnvelope`/empty `ArtifactVcs` +
  `store::print_document_pack`, the same host-owned whole-store-swap primitive `apps::space`'s
  `open_space` command already uses) rather than a plain builder-fn swap.

## Verification

`cargo check -p semio-s-plugin-reasoning-mindmap` — **blocked, churn-retry-exhausted**. All 3
attempts (initial + 2 retries, ~65s backoff each, per this ticket's workspace-churn protocol) fail
identically, before ever reaching this facet's own code:

```
error: couldn't read `.../📦️packages/🦀️rust/./././../../🎛️apps/🔌️wires/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs:427:13
```

Root cause confirmed by inspection: `📌️panels/` only contains `📄️artifact`/`🔍️inspection`/
`🛍️catalogue` today (no `📄️document`), and `📄️artifact/🦀️component.rs`'s own doc comment reads "the
document tree..." — i.e. a concurrent session mid-renamed `📌️panels/📄️document` →
`📌️panels/📄️artifact` and hasn't yet updated `📦️glue.rs`'s `pub mod document;` `#[path]` to match.
Both the panel directory and `glue.rs` are outside this facet's `🗿️artifacts/🔌️wires` edit boundary
(app-level panels + plugin-shared glue, both explicitly off-limits), and this exactly matches this
ticket's own `📓️status.md` note about a concurrent, repo-wide panel-restructuring pass running
elsewhere. Not fixed here per the workspace-churn policy — not this ticket's file to touch.

Since the full crate check never reaches this facet's code, as an additional best-effort sanity
check (beyond what the task requires) every touched/new `.rs` file under this facet was run through
`rustfmt --check` — all parsed as syntactically valid Rust (only formatting diffs, zero parse
errors), across all 10 new triad leaves' 30 files, the rewritten dispatch `🦀️component.rs`, the 18
orphaned stub files, and the two edited `📝️text`/`💾️binary` files.
