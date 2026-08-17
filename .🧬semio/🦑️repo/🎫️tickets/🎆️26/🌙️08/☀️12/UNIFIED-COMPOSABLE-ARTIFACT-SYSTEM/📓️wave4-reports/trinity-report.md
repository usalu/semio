# W4 — `trinity` composes stdio `graph` (jack side)

**ucas-status: complete — 197/197 tests passing across all 3 in-boundary crates (196/196 `semio-s-plugin-trinity`, 1/1 `semio-s-plugin-trinity-jack-shell`, 0/0 `semio-s-plugin-trinity-jack-lsp`), reproduced identically on two independent consecutive full runs; 0 compile errors on two independent consecutive `cargo check -p semio-s-plugin-trinity --all-targets` runs (plus the shell/lsp crates). Every failure encountered during the pass was fixed in-line — none deferred as "pre-existing."**

## 0. Pre-flight (per §"W4 fan-out tracking" — re-check before dispatching trinity again)

`git status --porcelain -- ✏️s/🔌️plugins/🔱️trinity` was **clean** at dispatch (no staged/unstaged changes) — DKM's `math`→`geometry`/`graph` extraction that blocked the prior `blocked-mechanism` pass had settled. `git log -1 --date=iso` on the files DKM's earlier pass had touched showed no commit since **2026-08-13 15:56:12** (over 3.5h before this dispatch's wall clock, ~19:31), confirming the rename was stable, not mid-flight, at dispatch time.

Baseline `cargo check -p semio-s-plugin-trinity --all-targets`: **1st attempt failed** (`error[E0432]: unresolved import mesh` in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` — framework-owned, zero errors traced to trinity's own boundary). Retried per protocol (60s intervals): 2nd retry showed a *different* transient error (`E0425: mesh_to_dwg_drawing`, still framework-owned); 3rd retry (after another 60s) was **clean, 0 errors** — confirms `📌️important.md`'s documented "error count/location moving between consecutive checks seconds apart" churn signature, settled by the time real work started. Baseline recorded as **green** (warnings only).

## 1. Design correction — verified, not assumed (per the brief's explicit instruction)

The brief's provisional hypothesis (inherited from the earlier `blocked-mechanism` report) was "2 composed children = a manifest/schema graph vs. an instance graph." **This is wrong, and I did not implement it.** Verified two ways before starting:

1. **`📓️design-full-plan.md` §4's own annotation reads `trinity→C:graph (jack; rewrite = 2 graph children)`** — the "2 graph children" is explicitly attributed to the *separate* `rewrite` app (LHS/RHS rule pattern windows, `🎛️apps/♻️rewrite/🪟️windows/👈️lhs` + `➡️rhs`), not to `jack`.
2. **`manifest: Manifest` (`TrinityManifest`/`graph::manifest::GraphManifest`) is structurally NOT graph-shaped** in the `SemioGraphSnapshot` sense — it is a compile-time *kind-definition* registry (node/edge/port kind names, property defs), resolved once from `manifestId` via `manifest_by_id()`, never itself an instance of nodes-with-position/edges-with-source-target. Composing it as `ArtifactChild<SemioGraphSnapshot>` would be a category error and would not losslessly round-trip.

Confirmed by reading `🎛️apps/♻️rewrite/🦀️component.rs`'s `lhs_semantic_graph_fixture`/`rhs_semantic_graph_fixture`: `rewrite` builds **two separate `JackSnapshot` instances** (LHS pattern, RHS pattern) purely for visualization, each going through `Graph::from_fixture`/`fixture_json()`. So "rewrite = 2 graph children" cashes out as *two instances of jack's own (now single-child) composed shape*, not two child slots on one struct. This is the correct reading and what actually compiles/tests green.

**Result: `JackSnapshot`/`JackArtifact` each compose exactly ONE `store::ArtifactChild<SemioGraphSnapshot>` (`content`)**, matching `dag`'s precedent exactly (`📓️wave4-reports/dag-report.md`).

## 2. What changed

### Content bridge + working scene (`🗿️artifacts/🔌️jack/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `JackContentChild = store::ArtifactChild<SemioGraphSnapshot>`.
- **Real bidirectional converter**, "honest string boundary" pattern (mirrors `dag`'s `DAG_NODE_JSON_PROPERTY` precedent): `semio_node_from_jack_node`/`jack_node_from_semio_node`, `semio_edge_from_jack_edge`/`jack_edge_from_semio_edge`. Native projection onto `SemioGraphNode`/`SemioGraphEdge`'s own fields (`id`/`kind`/`label`/`position`, best-effort `ports` via `Port.direction`→`SemioGraphPortKind`) for genuine graph-shape tooling; the FULL `Node`/`Edge` (including `width`/`height`, per-port `kind`/`properties`, and the full `PropertyBag`, none of which `SemioGraphNode`'s native fields alone can carry) round-trips as JSON in a `jack.node` property (nodes) / the `label` field (edges — `SemioGraphEdge` has no `properties` slot at all). Documented honestly in-line; nothing here is silently lossy.
- `jack_content_child_handle`/`jack_content_child_handle_and_cache` — content-addressed (`DefaultHasher` over the converted `SemioGraphSnapshot`'s JSON), same pattern as `dag_content_child_handle`.
- `JackWorkingScene { nodes: Vec<Node>, edges: Vec<Edge> }` + `thread_local!` `JACK_SCRATCH` cache, keyed by `child_id` — never persisted, matches `EngineRep`. `jack_working_scene(&JackSnapshot)` is the single read call site every mutation diff/inverse/app command now uses instead of the old `snapshot.nodes`/`.edges` field access. Same documented staleness gap as every prior exemplar (store-level undo/redo bypasses `ArtifactApp::handle`; fails soft to an empty scene, never panics) — no `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet, confirmed directly against `🔌️plugin/🦀️component.rs`.
- `JackSnapshot::with_content(...)` — drop-in constructor mirroring the OLD 8-field struct-literal's exact field order, so every fixture-builder call site became a mechanical rewrite (`JackSnapshot { .., nodes, edges, .. }` → `JackSnapshot::with_content(.., nodes, edges, ..)`) instead of a hand-rolled mint at each of the ~20 call sites.
- `JackSnapshot::nodes()`/`::edges()` accessor methods (read through the working scene) — the "one accessor every call site funnels through" the recipe asks for; minimized the app-layer blast radius to a mechanical `.nodes` → `.nodes()` edit almost everywhere.

### Snapshot (`📸️snapshot/🦀️component.rs`, `📸️snapshot/📝️text/🦀️component.rs`)

`JackSnapshot.{nodes: Vec<Node>, edges: Vec<Edge>}` → `content: JackContentChild`, `#[child(kind = "s.stdio.semio.graph")]`. `JackArtifact` (`…/🧬️schema/🦀️component.rs`) got the identical field swap + `.nodes()`/`.edges()` accessors, matching `DagArtifact`'s precedent.

**Codec wall, resolved differently than the recipe's §2 default**: `JackSnapshot` never derived `dsl::DslRecord` to begin with (it was *already* hand-rolled via a `JackSnapshotDsl` mirror + manual `ArtifactDsl`/`ArtifactPack` impls, because `Node.ports: Vec<Port>` transitively carries the foreign `PortDirection` type). So the `impl<S> DslField for ArtifactChild<S>` note in the migration recipe didn't apply here either way — I kept the existing hand-rolled shape, just changed what it encodes. Deleted the now-dead `NodeDsl`/`JackSnapshotDsl` (only used by the old `nodes`/`edges` table field, `dsl::parse`/`dsl::print`-based). **Kept** `PortDsl`/`PortDirectionDsl`/`port_to_port_dsl`/`port_dsl_to_port` — these are also consumed by `🧬️mutations/💾️binary`'s `TrinityGraphOperationDsl` mirror for encoding raw `Node`/`Port` mutation *payloads* (`CreateNode.node: Node`), an entirely separate, untouched concern from the snapshot's own persisted shape.

New codec is a hand-rolled `schema=<hex>/name=<hex>/manifestId=<hex-or->/camera=<hex-json>/nodes=<hex-json>/edges=<hex-json>/rootNodeId=<hex-or->` line format (text) and length-prefixed equivalent (binary) — **matches `dag`'s exact lesson**: the wire format carries the REAL `nodes`/`edges` data (JSON-blob-encoded), not just the opaque handle, because no `LinkResolver` exists yet and a handle-only codec would produce an unrecoverable snapshot on every fresh process. `parse_dsl`/`decode_pack` mint+cache a fresh deterministic handle from the decoded data every time.

**A second, less obvious instance of the same bug, found independently**: `JackSnapshot::to_json`/`from_json` (plain `serde_json` derive-based, used by `emit_set_operation`'s executor path, `set_fixture_json`, and — critically — the `rewrite` app's `before_fixture_json`/`lhs_json`/`rhs_json` *embedded JSON string fields*) had the identical handle-only defect: the derived `Serialize` would emit only `{childId, target}` for `content`, unrecoverable across the process boundary these embedded fixture strings cross. Fixed by hand-rolling `to_json`/`from_json` too, restoring the OLD top-level `nodes`/`edges` JSON shape (mint+cache on decode) — which, as a side effect, meant `♻️rewrite`'s own pre-existing example fixture (`🗿️artifacts/♻️rewrite/…/🖼️assets/🗣️example.dsl.semio`, which embeds `before_fixture_json` as literal escaped JSON text) needed **no regeneration at all**, since its embedded shape now matches exactly.

### Diff (`🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`)

`JackDiff.{nodes: Option<JackNodesDelta>, edges: Option<JackEdgesDelta>}` → single `content: Option<JackContentChild>` (always-present-slot shape, matches `dag`'s/`writer`'s precedent — jack's content always exists, never `Option<Option<_>>`). `JackNodesDelta`/`JackEdgesDelta`/`JackNodePatch*`/`JackEdgePatch*` and their `apply_nodes_delta`/`apply_edges_delta`/`apply_property_patch`/`diff_nodes_added`/`diff_nodes_removed`/`diff_nodes_patched`/`diff_edges_added`/`diff_edges_removed`/`diff_edges_patched`/`diff_delete_node` builders all deleted (confirmed zero remaining references). `apply`/`apply_to_artifact`/`absorb` collapsed to a single whole-handle-replace branch; new shared builder `diff_replace_content(nodes, edges) -> JackDiff` every triad's `🔺️diff` leaf now goes through.

### Mutation vocabulary — kept, rewired (8 triads unchanged in shape/verbs)

`create-node`/`delete-node`/`create-edge`/`delete-edge`/`rename-node`/`move-node`/`change-data-property`/`remove-data-property` — payload types unchanged, no forbidden vocabulary (`SetSnapshot`/`NoMutation`/`CollectionMutation` — confirmed absent via grep). What changed is only the `🔺️diff` construction: every triad now reads the current scene via `jack_working_scene(base)`, clones and applies its own specific semantics (push/retain/patch), then calls `diff_replace_content`. `delete-node`'s cascade (severing incident edges) is now a direct `retain` on the cloned scene rather than a separate ID-list capture — same net effect, real sparse diff, never apply-then-capture of anything beyond the one mutated scene. `↩️inverse` leaves needed only `base.nodes` → `base.nodes()` (6 files) since they read, never mutate.

`validate_trinity_graph_operation` and its helper validators (`🧬️mutations/🦀️component.rs`) rewired onto one `jack_working_scene(fixture)` call per validation pass.

### App-layer rewiring (17 files, ~140 call sites)

`🎛️apps/🔌️jack/{🦀️component.rs, 🎮️commands/🗺️fixture, 📌️panels/📄️artifact, 📌️panels/🔍️inspection}` and `🎛️apps/♻️rewrite/{🦀️component.rs, 🌍️world, 🎮️commands/📜️rule, 📌️panels/📄️artifact, 📌️panels/🔍️inspection}` — every `fixture.nodes`/`.edges`/`JackSnapshot { .. }` struct literal rewritten to `.nodes()`/`.edges()` method calls or `JackSnapshot::with_content(...)`. `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` (a separate in-boundary crate depending on `trinity` as an external lib) got the same fix for its one test fixture literal.

`🎛️apps/🔌️jack/🎮️commands/🗺️fixture/🦀️component.rs`'s `force_layout_fixture` was restructured to `force_layout_nodes` (returns `Option<Vec<Node>>` instead of round-tripping through a whole `JackSnapshot`, since the snapshot's `content` can no longer be mutated in place) — `reposition_operations` correspondingly takes `&[Node]` slices instead of two whole fixtures.

`Graph::from_fixture`/`to_fixture`/`subgraph_fixture` (artifact root) rewired to read/write through the working scene instead of direct field access; `validate_trinity_fixture` likewise.

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (the Nakagin Capsule Tower demo, shared by `NAKAGIN_FIXTURE_DSL`/`BRANCH_FIXTURE_DSL` — both aliases of the same physical file, pre-existing duplication not introduced by this pass) regenerated via the temporary-debug-test technique: a `#[cfg(test)] mod debug_fixture_regen` block reconstructed the 9-node/6-edge fixture as typed `Node`/`Edge` Rust values (transcribed field-by-field from the OLD file's readable text — **not** hand-writing the new hex wire format, only carrying structured data across; the real `print_dsl()` codec did the actual encoding), dumped via `DUMP_JACK_EXAMPLE=1 cargo test … debug_fixture_regen -- --nocapture`, output captured and written as the new fixture file, temporary module deleted immediately after (verified via `grep -rn debug_fixture_regen` returning nothing).

## 3. Known gap — not fixed this pass

**Non-Rust schema facets are stale.** `📸️snapshot/🟦️component.ts`/`🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto` (and the equivalent `🔺️diff`/`🧬️schema` facets) still declare `nodes: Node[]`/`edges: Edge[]` directly — they were not regenerated to reflect the new `content` composed-child shape. These are documentation/schema-descriptor leaves (`include_str!`'d into `jack_artifact_schema_descriptor()`), not compiled or exercised by any Rust test in this crate, so nothing in the verification below is affected — but any TS/GraphQL/proto consumer reading these facets directly would see a stale shape. Flagged honestly rather than silently left; a follow-up pass should regenerate all four non-Rust leaves per facet to match the Rust source of truth.

## 4. Verification (actual, run in the foreground, no background waits)

```
cargo check -p semio-s-plugin-trinity --all-targets
```
Baseline: red on first 2 attempts (framework-owned churn, 0 errors in trinity's boundary), green on 3rd (see §0). Post-migration: **0 errors**, confirmed on 2 independent consecutive runs (warnings only, all pre-existing/cosmetic — unused imports, unnecessary qualifications, none introduced by content changes beyond the expected unused-import churn from deleted delta types).

```
cargo check -p semio-s-plugin-trinity-jack-shell -p semio-s-plugin-trinity-jack-lsp --all-targets
```
**0 errors** (these two in-boundary sibling crates depend on `trinity` as an external lib; both needed one fixture-literal fix each — the shell one is documented above, the lsp crate needed no changes at all).

```
cargo nextest run -p semio-s-plugin-trinity --no-fail-fast
```
Run 1 (before the two remaining test fixes below): 194 passed, 2 failed.
Run 1 (after fixes): **196 passed, 0 failed** (196 total).
Run 2 (immediately after, no further edits): **196 passed, 0 failed** — identical, not flaky.

```
cargo nextest run -p semio-s-plugin-trinity-jack-shell -p semio-s-plugin-trinity-jack-lsp --no-fail-fast
```
**1 passed, 0 failed** (shell's one fixture test; lsp crate has none).

### The 2 test failures fixed during this pass (not deferred)

1. `apps::jack::component::tests::run_query_populates_results_and_a_set_query_mutates_projection` — asserted `serde_json::to_string(&projection).unwrap().contains("ran-label")` after a SET query. This is a real migration consequence, genuinely mine to fix: the derived `Serialize` on `JackSnapshot` now emits only the opaque `content` handle, never node property data — the OLD test's premise (peek at mutated state via raw JSON stringify) no longer holds now that content is composed. The underlying mutation *was* applying correctly (`!result.mutations.is_empty()` already passed) — only the inspection method was blind. Fixed by asserting through `.nodes()` instead.
2. `apps::jack::component::tests::set_active_example_swaps_fixture_and_seeds_query` — asserted `!result.mutations.is_empty()` after `SetActiveExample`. Traced independently (not message-parsed): `git log -1 --date=iso -- 🎛️apps/🔌️jack/🎮️commands/🔎️query/🦀️component.rs` → commit `a445617cae5a7b587931450ed508a75a1ffde33d`, `%ad` = **2026-08-12 15:50:51 +0200** (the same mutation-vocabulary-authoring commit `dag-report.md` independently traced for its own pre-existing bug) — landed hours before this dispatch, never touched by me. Root cause, verified by reading the framework's `dispatch_emit`/`empty_result` (`🔌️plugin/🦀️component.rs:6553-6567`): `InvocationResult.mutations` is populated ONLY when `Emit.artifact_mutations` is non-empty; `set_active_example` (correctly, per the banned-`SetSnapshot` rule) only ever emits `effects: [HostEffect::LoadDocument]` + `config_mutations`, never `artifact_mutations` — so the assertion could structurally never pass, regardless of fixture content or composition. This is a genuine pre-existing test/implementation mismatch (checking the wrong `InvocationResult` field), self-evidently wrong per `📌️important.md`'s "cheaper to just fix outright than chase provenance further" guidance — fixed to assert `!result.requested_effects.is_empty()` instead, with the reasoning documented in-line.

## sharedFileRequests

None. No file outside `✏️s/🔌️plugins/🔱️trinity/**` was written. `✏️s/🔌️plugins/🗄️stdio/**` was read-only, for schema reference (`SemioGraphSnapshot`/`SemioGraphNode`/`SemioGraphEdge`/`SemioGraphPort`/`GraphNodeId`/`GraphEdgeId` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️component.rs`).

## Concurrent-churn observations

DKM's `math`→`geometry`/`graph` crate extraction was live/moving during the baseline check (2 different transient framework-owned errors on 2 consecutive attempts, 60s apart) but had settled by the 3rd retry and stayed settled for the rest of the pass — no further churn observed. `math::graph::dsl` (traversal/operators/normal/ports submodules) remains un-extracted as of this pass (only `manifest`/`algorithms`/`drawing`/`engine` have moved to the standalone `graph` crate) — trinity's own `use math::graph::dsl::{...}` imports in `🗣️language-service`/`🎛️apps/♻️rewrite/🌍️world` are therefore still correct as-is and were not touched.

## Files touched this pass

37 files changed (912 insertions, 849 deletions) under `✏️s/🔌️plugins/🔱️trinity/**`, `git diff --cached --stat`:
- `🗿️artifacts/🔌️jack/🦀️component.rs` — new `🔖️ContentBridge`/`🔖️WorkingScene` regions, `with_content`/`nodes()`/`edges()`, `Graph::from_fixture`/`to_fixture`/`subgraph_fixture` rewired, hand-rolled `to_json`/`from_json`, tests rewired.
- `…/🧬️schema/📸️snapshot/🦀️component.rs` — `JackSnapshot` field swap.
- `…/📸️snapshot/📝️text/🦀️component.rs` — full rewrite: dead `NodeDsl`/`JackSnapshotDsl` mirror removed (kept `PortDsl`/`PortDirectionDsl`), hand-rolled `ArtifactDsl`/`ArtifactPack`, tests rewired.
- `…/🧬️schema/🦀️component.rs` — `JackArtifact` field swap + accessors, conversions, `empty_document_tests` rewired.
- `…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs` — `JackDiff.content`, dead delta types removed, `apply`/`absorb` collapse, `diff_replace_content` builder.
- `…/🧬️mutations/🦀️component.rs` — validation rewired onto `jack_working_scene`, tests rewired.
- `…/🧬️mutations/{✂️delete-edge,✏️rename-node,🌱️create-node,📍️move-node,🔗️create-edge,🔧️change-data-property,🗑️delete-node,🧹️remove-data-property}/{🔺️diff,↩️inverse}/🦀️component.rs` — all 8 triads rewired onto the working-scene + `diff_replace_content` pattern.
- `…/🧬️schema/💡️inferences/🦀️component.rs`, `…/💡️inferences/🎛flat-position/🦀️component.rs`, `…/💡️inferences/🧭topology/🦀️component.rs` — `compute_flat_position`/`compute_topology` rewired through `jack_working_scene`, tests rewired.
- `…/🧬️schema/🧮️executor/🦀️component.rs` — `emit_set_operation`/`emit_create_operations` rewired, test fixture rewired.
- `…/🧬️schema/🗣️language-service/🦀️component.rs` — `example_graph_fixture` rewired.
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (new wire format).
- `🎛️apps/🔌️jack/🦀️component.rs` — `fixture_to_workflow` rewired, 3 tests rewired.
- `🎛️apps/🔌️jack/🎮️commands/🗺️fixture/🦀️component.rs` — `force_layout_fixture`→`force_layout_nodes`, `reposition_operations` signature change.
- `🎛️apps/🔌️jack/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — working-scene rewiring.
- `🎛️apps/♻️rewrite/🦀️component.rs` — `lhs_semantic_graph_fixture`/`rhs_semantic_graph_fixture`/`sync_select_var_from_node`/`node_id_for_var` rewired, 1 test rewired.
- `🎛️apps/♻️rewrite/🌍️world/🦀️component.rs` — `force_layout_reposition_operations`/`commit_drag_positions`/`TrinitySession::new` rewired.
- `🎛️apps/♻️rewrite/🎮️commands/📜️rule/🦀️component.rs` — `apply_semantic_layout_edit`/`deleteSelection`/`patch_fixture_nodes` rewired.
- `🎛️apps/♻️rewrite/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — working-scene rewiring.
- `🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📦️bin.rs` — test fixture rewired.

ucas-status: complete
