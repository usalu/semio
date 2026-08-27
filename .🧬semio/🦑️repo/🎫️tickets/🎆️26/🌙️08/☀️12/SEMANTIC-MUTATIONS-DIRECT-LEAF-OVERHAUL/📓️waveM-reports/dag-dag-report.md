# Facet report — `🕸️dag` / `🕸️dag`

- **facet**: `✏️s/🔌️plugins/🕸️dag` → artifact `🕸️dag`, standard `🔖️1`, subset `✳️any`
- **crate**: `semio-s-plugin-dag`
- **status**: `partial` — all code/schema work landed on disk; gates NOT RUN by this lane (see `gates`).

## mutationsCreated

14 triads, one dir each with a facet-unique emoji, all three leaves (`🦠️mutation`/`🔺️diff`/`↩️inverse`) handcrafted.

| slug | verb | superseded old variant |
|---|---|---|
| `🌱create-node` | `create` | `Nodes(CollectionMutation::Add)` |
| `🗑️delete-node` | `delete` | `Nodes(CollectionMutation::Remove)` (+ the app-level edge-sweep that used to accompany it) |
| `🏷️rename-node` | `rename` | `SetNodes` + `SetEdges` pair emitted by `rename_dag_node` |
| `🔤change-node-name` | `change` | `Nodes(CollectionMutation::Patch{name})` |
| `↔️move-node` | `move` | `Nodes(CollectionMutation::Patch{x,y})` |
| `📐resize-node` | `resize` | `Nodes(CollectionMutation::Patch{width,height})` |
| `🖼️change-node-icon` | `change` | (unreachable before — `DagNodePatch` has no `icon` slot) |
| `🔡change-node-abbreviation` | `change` | (unreachable before — no `abbreviation` slot) |
| `🧮change-node-operator-kind` | `change` | (unreachable before — no `operator_kind` slot) |
| `🔁replace-node-kind` | `replace` | `Nodes(CollectionMutation::Patch{kind})` |
| `🗃️replace-node-properties` | `replace` | (unreachable before — no `properties` slot) |
| `🔀reorder-nodes` | `reorder` | `Nodes(CollectionMutation::Move)` |
| `🔗connect-nodes` | `connect` | `Edges(CollectionMutation::Add)` |
| `✂️disconnect-nodes` | `disconnect` | `Edges(CollectionMutation::Remove)` |

Every verb is in `protocol::APPROVED_VERBS`. `delete-node`'s diff/inverse capture the severed-edge cascade
(edges re-`connect-nodes`ed after `create-node` in the inverse); `rename-node` cascades the id change into
every `"<id>@<port>"` edge endpoint via `DagEdgePatch`.

## genericVariantsRemoved

- `Nodes(CollectionMutation<String, DagNodeSpec, DagNodePatch>)` — forbidden generic wrap.
- `Edges(CollectionMutation<String, DagFixtureEdge, DagEdgePatch>)` — forbidden generic wrap.
- `SetNodes { nodes: Vec<DagNodeSpec> }` — bulk-`Vec` whole-collection setter, deleted with no direct
  replacement (re-expressed as `create`/`delete`/`reorder`/`move` sequences via the new
  `dag_snapshot_mutations` differ).
- `SetEdges { edges: Vec<DagFixtureEdge> }` — same, re-expressed as `connect`/`disconnect` sequences.
- `SetSnapshot { snapshot: DagSnapshot }` — deleted, NO replacement. `DagPlayApp::whole_document_operation`
  now falls through to the trait default (`None`), so the generic `document:in` media importer reports
  `MediaError::NotImplemented`; a real whole-document load goes through `store::ArtifactStore::reset`.
- The hand-written `impl Mutation<DagSnapshot> for DagMutation` (replaced by `#[derive(dsl::Mutations)]`).
- The `kernel_mutation`/`plugin_mutation` bridge into the foreign
  `infinite_board_port_directed_dag::DagMutation` (replaced by a local `DagMutationDsl` mirror).

## filesTouched

### created (42 triad leaves + nothing else)

Under `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`,
for each of the 14 slugs above: `<slug>/🦠️mutation/🦀️component.rs`, `<slug>/🔺️diff/🦀️component.rs`,
`<slug>/↩️inverse/🦀️component.rs`.

### updated

- `…/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum rewritten to `#[derive(…, dsl::Mutations)]`
  `#[mutations(snapshot = DagSnapshot, diff = DagDiff, schema = "dag.dag")]`, 14 single-unnamed-field
  variants + re-exports; added `dag_snapshot_mutations` differ; extended `#[cfg(test)]` region.
- `…/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` — local `DagMutationDsl` mirror + handcrafted
  `OpText`/`OpBinary` codecs and to/from bridge (din16798 pattern); own `#[cfg(test)]` round-trip region.
- `…/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — test fixtures re-pointed off `SetNodes`.
- `…/🧬️schema/🧬️mutations/📖️component.grammar.semio`, `🔗️component.graphql`, `🔣️component.json`,
  `🛰️component.proto`, `🟦️component.ts` — rewritten to describe the real 14-kind vocabulary
  (`🟦️component.ts` now exports real types, not `export {}`).
- `…/🧬️schema/🔺️diff/🦀️component.rs` — added `DagNodeExtraPatch` / `DagNodeExtraPatchEntry` and the
  `extra_patched` field on `DagNodesDelta` (see deviations).
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_nodes_delta` honours `extra_patched`;
  `absorb_nodes_delta` folds it; removed the two dead `*_delta_from_collection_mutation` helpers; test
  re-pointed off `SetNodes`.
- `…/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — command-envelope test fixture re-pointed off `SetNodes`.
- `…/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — `remove_nodes_operations` now emits one `delete-node` per
  node (cascade lives in the mutation, not the caller); dropped the `CollectionMutation` import.
- `🎛️apps/🕸️dag/🦀️component.rs` — removed the `whole_document_operation` override; its test now asserts
  `None`.
- `🎛️apps/🕸️dag/🎮️commands/🕸️graph/🦀️component.rs` — `connect`/`disconnect`/`move`/`set-fixture`/
  `reorganize` handlers re-expressed on the new vocabulary.
- `🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs` — `add_node`/`rename_dag_node`/`patch_dag_nodes`
  re-expressed on the new vocabulary (`rename_dag_node` is now a single `rename-node`).
- `📦️packages/🦀️rust/📦️glue.rs` — 5 old triad mounts replaced by 14 new ones (same `#[path]` depth);
  added `extern crate semio_framework_os_kernel as vcs;`.

### removed

- `…/🧬️mutations/🔗nodes/`, `➡️edges/`, `📋set-nodes/`, `📋set-edges/`, `📄set-snapshot/` (all stub leaves).

## sharedFileRequests

None. Every edit is inside `✏️s/🔌️plugins/🕸️dag/`. The foreign kernel file
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` was read
only — its own `DagMutation`/`SetSnapshot` variants are untouched and are now unreferenced by this plugin.

## allowlistKeysToRemove

Verified free of `SetSnapshot` / `NoMutation` / `CollectionMutation` by
`grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/🕸️dag --include="*.rs" --include="*.ts"`
returning zero hits (comments and doc-comments reworded too — the sweep is content-level, not
token-level):

- `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎮️commands/🔧️nodes/🦀️component.rs`
- `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎮️commands/🕸️graph/🦀️component.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

(These are exactly the three `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` entries matching `🕸️dag` at
`📜️script.ts:5769-5771`. `📜️script.ts` was NOT edited by this lane.)

## gates

**`cargo check -p semio-s-plugin-dag` — NOT RUN by this lane.** Deferred to the coordinator's consolidated
verification pass, on the coordinator's explicit instruction, because ~10 lanes were contending for one
shared cargo build lock. No pass is claimed and none was observed.

**`cargo test -p semio-s-plugin-dag --lib` — NOT RUN by this lane.** Same reason.

**`bun ./📜️script.ts policy` — NOT RUN by this lane.** Same reason.

Two `cargo check -p semio-s-plugin-dag` attempts were started *before* the stop instruction arrived; both
have since terminated and both were **blocked by foreign churn before this facet was ever type-checked**.
Neither is claimed as a pass. Recorded verbatim as `blocked-churn`:

**Attempt 1** reached this crate and failed on a single foreign, pre-existing module-resolution error:

```
error: couldn't read `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/./././../../🎛️apps/🕸️dag/📌️panels/📄️document/🦀️component.rs`: No such file or directory (os error 2)
   --> ✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs:486:13
    |
486 |             pub mod document;
    |             ^^^^^^^^^^^^^^^^^
```

This is the `📌️panels/📄️document` missing-file churn the fanout brief pre-declares as another dev's
in-flight work: `📌️panels/` on disk holds `📄️artifact`, `🔍️inspection`, `🛍️catalogue` but no `📄️document`,
and `git log` shows that glue.rs mount line predates this lane's edits. It is a module-resolution error that
fires **before** type checking, so it tells us nothing either way about this facet's own correctness — no
error attributable to these edits was observed, and none was ruled out either.

**Attempt 2** (started before the stop instruction, completed after it; its output file was read, no new
cargo command was issued) got *further from* a verdict, not closer: it died in the shared framework kernel
and never reached `semio-s-plugin-dag` at all.

```
For more information about this error, try `rustc --explain E0753`.
warning: `semio-framework-os-kernel` (lib) generated 47 warnings
error: could not compile `semio-framework-os-kernel` (lib) due to 18 previous errors; 47 warnings emitted
```

`semio-framework-os-kernel` is the crate every plugin aliases as `store`/`protocol`/`dsl`/`vcs`; 18 errors
including `E0753` (malformed doc comment) in `🧰️framework/**` is another session mid-refactor, matching the
`Concurrent Cargo Workspace Churn` pattern. Explicitly **out of this lane's bounds** — not investigated, not
touched. Note the shell exit code for this attempt was `0` only because the command was piped through
`tail`; the exit status reflects `tail`, not `cargo`. The cargo build itself failed.

Net: **this facet has never been compiled.** Attempt 1 stopped at module resolution in this plugin;
attempt 2 stopped one layer lower, in the framework kernel. Everything under `lawTests` is written-but-
unexecuted.

## lawTests

Written but **not executed** (see `gates`). In `…/🧬️mutations/🦀️component.rs`'s `🧪️Tests` region:

- Round-trip helper (`apply` then apply `inverse`, assert equality with the pre-state) over:
  create→move→resize→delete chain, `rename-node` (asserting edge-endpoint cascade), `delete-node`
  (asserting sever/reconnect), `reorder-nodes`.
- `protocol::testkit::assert_mutation_inverse_law` for `create-node`, `delete-node`, `rename-node`,
  `move-node`, `resize-node`, `connect-nodes`, `disconnect-nodes`, `reorder-nodes`.
- `protocol::testkit::assert_mutation_diff_absorb_law` for `move-node`.
- `dispatch_registers_semantic_descriptors` — iterates `DagMutation::kinds()`, asserts every verb is in
  `APPROVED_VERBS` and that the vocabulary is exactly 14 kinds.

In `…/🧬️mutations/📝️text/🦀️component.rs`'s `🧪️Tests` region: per-kind `assert_op_line_round_trip` for
three hand-picked kinds plus `every_variant_op_text_round_trips` iterating a 15-value sample covering all
14 variants (both `Some`/`None` arms of `change-node-operator-kind`). In `💾️binary/🦀️component.rs`:
`assert_op_text_binary_equivalence` re-pointed onto `delete-node`.

## deviations

1. **`kind` / `properties` granularity — kept coarse (`replace`), as the brief's default.** I read
   `🎮️commands/🕸️graph`, `🎮️commands/🔧️nodes` and all three `📌️panels`. The only fine-grained
   kind-editing gesture that actually exists is `patch_dag_nodes` with `field ∈ {name, value, min, max}`,
   and `engine::node_patch_for_field` implements the slider arms by cloning the node, mutating one field
   of `DagNodeKind::Slider`, refitting the widget, and emitting a **whole-`kind`** patch — i.e. the
   existing gesture is already whole-kind-replacement at the engine level. Minting
   `change-node-slider-value` etc. would have meant rewriting that engine function too, which is
   speculative granularity the brief tells me not to invent. `properties` has **no** editing gesture
   anywhere in the plugin, so it likewise got the default `replace-node-properties`. `field == "name"`
   does route to the dedicated `change-node-name` rather than through `replace-node-kind`.
2. **New diff-internal `DagNodeExtraPatch`.** `infinite_board_port_directed_dag::DagNodePatch` (a foreign
   type I must not edit) carries only `name`/`x`/`y`/`width`/`height`/`kind` — it has no slot for `id`,
   `icon`, `abbreviation`, `operator_kind` or `properties`. Rather than remove-and-re-add nodes (which
   would silently reorder the collection and break `reorder-nodes`' meaning), I added a facet-local
   `DagNodeExtraPatch` + `extra_patched: Vec<DagNodeExtraPatchEntry>` on `DagNodesDelta`, applied in place.
   This is a **diff-internal** option-bag, which the taxonomy explicitly permits; it is never a mutation
   payload. `operator_kind` is double-`Option`ed (`Some(None)` clears, `None` leaves untouched).
3. **DSL-mirror JSON escape hatch (used).** `DagNodeSpec`/`DagNodeKind`/`PropertyBag` cannot implement
   `dsl::DslField` cleanly (their `kind` field isn't boxed — the long-standing constraint already documented
   on `DagNodeKindDsl` in `📸️snapshot/🦀️component.rs`). So `DagMutationDsl` carries them as opaque
   `serde_json` strings (`node_json`, `new_kind_json`, `new_properties_json`, `properties_json`) at that one
   wire boundary. Everything else — ids, coordinates, extents, names, icons, `EdgeRouteStyle`,
   `Option<String>`, `Vec<String>` — is a real typed DSL field. The grammar/graphql/json/proto files reflect
   this honestly (JSON-typed fields named `*Json`).
4. **`rename-node` addresses the node `id`, not `name`.** `DagNodeSpec` has both. The editor's rename
   gesture (`rename_dag_node`) rewrites the **id** and cascades edges, so `id` is the identity field per the
   taxonomy's addressing rule 2; the display `name` got its own scalar `change-node-name`.
5. **`reorder-nodes` payload is the full FINAL id order**, not `{from,to}`. The evidence for meaningful
   order is `DagNodesDelta.reordered: Option<Vec<String>>`, which is already a full-order field, and the
   diff's `apply` consumes it as such — a `{from,to}` payload would have to reconstruct the same vector
   anyway. Inverse is BASE's own id order.
6. **New `dag_snapshot_mutations` differ** (mirrors `sequence_snapshot_mutations`) added to the dispatch
   file, so the former `SetNodes`/`SetSnapshot` call sites (`set-fixture` paste, `reorganize`) become
   targeted mutation sequences rather than snapshot swaps. It does not detect id renames (those surface as
   delete+create); the real rename gesture uses `rename-node` directly and does not go through the differ.
7. **`extern crate semio_framework_os_kernel as vcs;` added to this plugin's `📦️glue.rs`** so the dispatch
   test region can call `vcs::apply_mutation`, matching how `🎬️sequence`'s glue.rs is set up.

## incomplete / requeue

- **All three gates unrun for this facet** (`cargo check`, `cargo test --lib`, `policy`) — the substantive
  requeue item. Nothing in this facet has been compiler- or test-verified; the triads, the derive, the
  `DagMutationDsl` mirror and the rewritten call sites are all unexercised code as of this report.
  Highest-risk spots if the consolidated pass turns up errors, roughly in order: (a) the
  `#[derive(dsl::DslEnum)]` field types in `DagMutationDsl` — `Option<String>`, `Vec<String>` and
  `EdgeRouteStyle` are assumed DSL-representable from sibling precedent, not verified here; (b) the
  `dsl::Mutations` derive's kind/verb compile-time assertions across all 14 triads; (c) the
  `math::graph::manifest::PropertyBag` import path in the new diff types and in
  `🗃️replace-node-properties`; (d) `infinite_board_port_directed_dag::EdgeRouteStyle` being reachable at
  crate root (checked by reading the `semio-framework-os-infinite` glue re-export list, not by compiling).
- **`📌️panels/📄️document/🦀️component.rs` missing** — foreign churn, blocks this crate from compiling at
  module-resolution time regardless of this facet. Not mine to fix; whoever owns that panel needs to land
  the file or drop the glue mount at `📦️glue.rs:486`. **This one gates the consolidated pass too** — until
  it is resolved, `cargo check -p semio-s-plugin-dag` cannot reach this facet's code no matter who runs it.
- **`semio-framework-os-kernel` failing with 18 errors (incl. `E0753`)** as of attempt 2 — foreign churn in
  `🧰️framework/**` from another session. Blocks every plugin, not just this one. Flagged for awareness only.
- **Per-triad `🟦️component.ts` mirrors not created.** The brief's step 6 asks for a non-stub `.ts` beside
  every triad `🦀️component.rs`; I wrote the real top-level facade
  (`🧬️mutations/🟦️component.ts`, full discriminated union) but did not add the 14 per-triad `.ts` leaves.
  The deleted stub dirs had had `export {};` files. Requeue if the per-triad mirrors are wanted.
- **`💾️binary/📡️component.protocol.semio` left as-is** — it describes the generic envelope framing
  (magic/header/segment/footer), not a per-variant `record … tag N` table, so the 14-kind vocabulary is not
  enumerated there. The binary set's sibling `.abnf`/`.ksy`/`.spicy` files were likewise not touched.
  Requeue if the coordinator wants the append-only per-variant tag table in the binary set.
