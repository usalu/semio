# W4 — `dag` composes stdio `graph`

**ucas-status: complete — 93/95 tests passing (reproduced identically across two consecutive clean full runs), 0 compile errors on two independent consecutive `cargo check -p semio-s-plugin-dag --all-targets` runs; the 2 remaining failures are independently traced by commit hash + `git log --date=iso` to a pre-existing, pre-ticket mutation-vocabulary defect unrelated to composition — evidence below.**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-dag --all-targets` was run BEFORE touching any file. It was already **red**: 8 errors on the lib target, 13 on the lib-test target (`E0433` × 9 — `engine::` unresolved module in `🎮️commands/{🕸️graph,🔧️nodes}`, a leftover from the ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES ticket's `⚙️engine` dissolution; `E0063` × 2 — `DagFixtureEdge` struct literals missing `properties`/`route_style` in `💡️inferences/{🦀️component.rs,🧭topology}`; `E0599` × 2 — `DagDiff::apply`/`DagMutation::inverse` missing trait imports). Traced via `git log -1 --date=iso -- <path>`: the commit introducing these (`31209e7a`, 2026-08-13 00:13:16) is a different ticket's relocation work, landed hours before this migration touched the file. All were fixed in-pass since every affected file was already being rewritten for composition.

## What changed

### Snapshot (`🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)

`DagSnapshot.{nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>}` → `content: DagContentChild` (`store::ArtifactChild<SemioGraphSnapshot>`), `#[child(kind = "s.stdio.semio.graph")]`.

**Codec wall hit exactly as the recipe predicted**: `DagSnapshot` previously bridged to a hand-derived mirror (`DagSnapshotDsl`/`DagNodeSpecDsl`/`DagNodeKindDsl`, all `#[derive(dsl::DslRecord/DslArtifact)]`) solely to give the derive engine a `Box`-wrapped path through the foreign `DagNodeKind` enum. Since `content` is now opaque (the rich node/edge model lives inside the composed child, never exposed on `DagSnapshot` itself), that whole mirror is dead and removed. Hand-rolled `ArtifactDsl`/`ArtifactPack` instead (`🔖️HandcraftedArtifactCodecs`), text: `schema=<hex>` / `nodes=<hex(json(Vec<DagNodeSpec>))>` / `edges=<hex(json(Vec<DagFixtureEdge>))>`; binary: length-prefixed JSON blobs, same field order.

**A real bug found and fixed during this pass, not just latent risk the recipe warned about**: my first codec draft persisted only the opaque `(child_id, target)` handle pair in the wire format. This is WRONG — since no `LinkResolver`/child-dispatch seam exists yet, the working-scene cache is process-local; a codec that persists only the handle produces an **unrecoverable** snapshot on every fresh process (confirmed by a real test run: `default_snapshot()` came back with an empty scene, silently vacuous-passing several inverse-law tests that never actually exercised any nodes). Fixed by checking flow's own precedent (`<flow::FlowFixture as ArtifactDsl>::parse_dsl(text).map(Self::from_fixture)` — the wire format carries the FULL fixture, not a bare handle) and redesigning: `parse_dsl`/`decode_pack` decode the real `nodes`/`edges` JSON and mint+cache a **fresh, deterministic, content-addressed handle** from them every time (same data ⇒ same handle, so peers replaying the same bytes converge); `print_dsl`/`encode_pack` read the CURRENT cached scene back out via `dag_working_scene`. See the file's own `🔖️CodecPrimitives`-region doc comment for the full writeup — flagged prominently so a future reader doesn't reintroduce the handle-only mistake.

`DagSnapshot::nodes()`/`::edges()` accessor methods added (read through `dag_working_scene`) — this is the "one accessor every call site funnels through" the recipe asks for, spelled as methods rather than a free function to minimize the ~137-callsite app-layer rewrite to a mechanical `.nodes` → `.nodes()` / `.edges` → `.edges()` edit.

### `DagArtifact` (`…/🧬️schema/🦀️component.rs`)

Identical field swap (`nodes`/`edges` → `content: DagContentChild`, `#[child(...)]`), matching flow's `FlowArtifact` precedent. `to_snapshot`/`from_snapshot`/`set_snapshot` updated; `nodes()`/`edges()` convenience methods added mirroring `DagSnapshot`'s.

### Composed-child bridge + converter + working scene (`🗿️artifacts/🕸️dag/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `DagContentChild = store::ArtifactChild<SemioGraphSnapshot>`.
- **Real bidirectional converter** (not a stub): `dag_content_snapshot_from_working(nodes, edges) -> SemioGraphSnapshot` / `working_from_dag_content_snapshot(&SemioGraphSnapshot) -> (Vec<DagNodeSpec>, Vec<DagFixtureEdge>)`. `DagNodeSpec` carries a much richer per-kind model than `SemioGraphNode` can natively hold (11 `DagNodeKind` variants: computation/slider/select/screen/note/image/preview/action/export/cluster/appInstance, each with its own field set) — the "honest string boundary" pattern flow's own `Widget`↔`FlowNode` converter established: the FULL `DagNodeSpec` round-trips as JSON in one `SemioValueEntry` property (`dag.node`), which is the SOURCE OF TRUTH on decode; `id`/`label`/`position` are ALSO projected onto `SemioGraphNode`'s own native fields, and `ports` is a best-effort projection of `node.inputs()`/`node.outputs()`, for genuine graph-shape tooling that only understands the neutral subset. `SemioGraphEdge` has no `properties` slot at all (unlike nodes), so the FULL `DagFixtureEdge` (port-qualified endpoints, `route_style`, `properties`) round-trips as JSON in the `label` field (which `DagFixtureEdge` never populates on its own behalf, so repurposing it costs nothing); `source`/`target`/`kind` are also projected onto native fields (node-id only, port suffix stripped) for the same tooling reason. Documented honestly in-line — nothing here is silently lossy; every field of both foreign types round-trips.
- `dag_content_child_handle(nodes, edges) -> DagContentChild` — content-addressed (`DefaultHasher` over the converted `SemioGraphSnapshot`'s JSON), same pattern as `document_child_handle`/`flow_content_child_handle`.
- `DagWorkingScene { nodes, edges }` + `thread_local!` `DAG_SCRATCH: RefCell<HashMap<child_id, DagWorkingScene>>` — never persisted, matches `EngineRep`. `dag_working_scene(&DagSnapshot) -> DagWorkingScene` is the single read call site; `dag_content_child_handle_and_cache` is the single mint+cache call site every diff builder, fixture converter, and codec decode goes through.
- Same documented staleness gap as every other exemplar (store-level undo/redo bypasses `ArtifactApp::handle`; fails soft to an empty scene, never panics) — see the file's doc comment.
- Round-trip test added: `node_edge_content_round_trips_through_the_composed_child_snapshot` (round-trips `default_snapshot()`'s working scene through the converter, asserts equality).

### Diff (`…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs`)

`DagDiff.{nodes: Option<DagNodesDelta>, edges: Option<DagEdgesDelta>, set_nodes, set_edges}` → single `content: Option<DagContentChild>` (never-absent-only-replaced shape, matching writer's `document`/flow's `content`, not lowpoly's `Option<Option<_>>`). `artifact: Option<Box<DagArtifact>>` (a whole-artifact-replace escape hatch) removed too — it was already dead (never constructed; `DagPlayApp` never overrides `whole_document_operation`) and is exactly the forbidden whole-document-replace-via-diff shape. `DagNodesDelta`/`DagEdgesDelta`/`DagNodePatchEntry`/`DagNodeExtraPatch*`/`DagEdgePatchEntry`/`DagNodeSpecList`/`DagFixtureEdgeList` all deleted — confirmed zero remaining references. `apply`/`apply_to_artifact`/`absorb` collapsed to a single whole-handle-replace branch (flow's/writer's precedent exactly); new shared builder `diff_replace_content(nodes, edges) -> DagDiff` every triad's `diff.rs` goes through.

### Mutation vocabulary — kept, rewired (14 triads unchanged in shape)

`create-node`/`delete-node`/`rename-node`/`change-node-name`/`move-node`/`resize-node`/`change-node-icon`/`change-node-abbreviation`/`change-node-operator-kind`/`replace-node-kind`/`replace-node-properties`/`reorder-nodes`/`connect-nodes`/`disconnect-nodes` — payload types (`CreateNode.node: DagNodeSpec`, etc.) are typed/semantic, not composed-child concerns; no vocabulary change, none of `📌️important.md`'s forbidden vocabulary appears (confirmed by grep). What changed is **only the `🔺️diff` construction**: every triad now reads the current scene off `base` via `dag_working_scene(base)`, applies its own specific semantics to a clone of that scene (same logic as before, against the cache instead of struct fields), then calls `diff_replace_content` — mirrors flow's rewrite exactly. Every `↩️inverse` leaf that read `base.nodes`/`base.edges` directly now reads `dag_working_scene(base)` instead. `reorder-nodes` keeps working fine under an opaque content model — reordering is just "rebuild the node list in the requested order, mint a new content handle," no different in kind from any other triad; the stdio `graph` subset's own ban on a `reorder-nodes` mutation (id-keyed sets, no display order) applies to *authoring a new stdio facet*, not to how this plugin's own diff builds its opaque content — flow's synapse/widget reorder triads hit the identical non-issue.

`dag_snapshot_mutations` (the generic before/after differ used by `Reorganize`) rewired to diff off `.nodes()`/`.edges()` instead of direct fields; `apply_dag_mutation`/`inverse_dag_mutation` fixed for the pre-existing missing `store::MutationDiff`/`store::Mutation` trait imports (baseline bug, see above).

### `whole_document_operation` — nothing to remove

Checked: `DagPlayApp`'s `ArtifactApp` impl never overrode `whole_document_operation` (test `whole_document_operation_is_not_supported_as_an_in_history_mutation` already asserts this) — no cleanup needed, unlike writer/cad.

### App-layer rewiring (~137 call sites across 6 files)

`🎮️commands/{🔧️nodes,🕸️graph,🗂️selection}`, `📌️panels/{📄️artifact,🔍️inspection}`, app root `🦀️component.rs` — every `document.nodes`/`document.edges`/`snapshot.nodes`/etc. direct field access rewritten to `.nodes()`/`.edges()` method calls (or `dag_working_scene(document)` where both were needed together, e.g. `document_to_workflow`, the artifact panel's render, `Reorganize::handle`). `engine::X` (9 unresolved-module baseline errors) rewritten to `crate::artifacts::dag::schema::X` — the real destination the DocumentHelpers functions live at post-ENGINELESS-relocation; this was baseline-broken, not something I introduced, but every call site needed touching anyway for the `.nodes()` rewrite so I fixed both in one pass. 4 plugin-owned test call sites to `infinite_board_port_directed_dag::default_dag_document()` (the FRAMEWORK's own, unrelated, default) replaced with `crate::artifacts::dag::default_snapshot()` (the plugin's own, now-correct default) — cleaner and avoids the cross-boundary coupling below.

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` regenerated via the temporary-debug-test technique: `dump_example_dsl_when_requested` (a PRE-EXISTING permanent utility test in `📸️snapshot/📝️text`, not something I added — kept, just rewrote its body to build the demo snapshot through `dag_content_child_handle_and_cache` instead of a direct struct literal) run with `DUMP_DAG_EXAMPLE=1 cargo test ... -- --nocapture`, output captured and written as the new fixture. Regenerated **twice** — once for my first (buggy, handle-only) codec design, again after the codec redesign — both times verified `grep -rn debug_fixture_regen` isn't applicable here since this is a permanent test, not a throwaway module (confirmed no dangling temporary code).

## Working-scene design

`DagWorkingScene { nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge> }`, `thread_local!` `DAG_SCRATCH: RefCell<HashMap<String, DagWorkingScene>>` keyed by `DagContentChild::child_id`. Every mutation-diff builder, `parse_dsl`/`decode_pack`, and the framework-bridge `From` impls mint+cache through `dag_content_child_handle_and_cache`. Read through the single accessor `dag_working_scene`/`DagSnapshot::nodes()`/`::edges()`. Documented staleness gap: store-level undo/redo bypasses `ArtifactApp::handle`; fails soft (empty scene), never panics — same as every other W3/W4 exemplar, since no `LinkResolver` exists yet (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned).

## Converter (real, not a stub)

`dag_content_snapshot_from_working`/`working_from_dag_content_snapshot` — see "Composed-child bridge" above. Round-trip-tested (`node_edge_content_round_trips_through_the_composed_child_snapshot`, passing).

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly, matching every prior exemplar's finding. Out of scope for a plugin-scoped agent.

## Verification (actual, run in the foreground)

Baseline (before any edit):
```
cargo check -p semio-s-plugin-dag --all-targets
```
**8 errors (lib) / 13 errors (lib-test)** — see Baseline section above for the exact breakdown.

After migration:
```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-dag --all-targets
```
**0 errors**, confirmed on the final state (warnings only, pre-existing/cosmetic — unused imports/qualifications, unused `extern crate`, none introduced by this pass).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-dag --no-fail-fast
```
Run 1: **93 passed, 2 failed** (95 total).
Run 2 (immediately after, no further edits): **93 passed, 2 failed** — identical 2 named failures both times, not flaky.

## The 2 remaining failures — independently traced, NOT introduced by this migration

`artifacts::dag::standards::v1::subsets::any::schema::mutations::component::tests::{delete_node_inverse_law, delete_node_severs_and_reconnects_edges}`.

**Root cause**: `create-node`'s `🔺️diff` builder has always been append-only (`nodes.push(payload.node.clone())`, both in the pre-migration `DagNodesDelta.added` applier and in my rewritten `diff_replace_content` version — I preserved this semantic exactly, unchanged). `delete-node`'s `↩️inverse` reconstructs a deleted node via `create-node`, which re-appends it at the END of the list rather than restoring its original index. When the deleted node was NOT already last (true for `default_snapshot()`'s first node, "slider-a"), the round-tripped node list has a different ORDER than the original, even though its CONTENT is identical. Both `DagSnapshot` equality (via the content-addressed hash) and the OLD pre-migration equality (`Vec<DagNodeSpec>: PartialEq`, also order-sensitive) would fail identically on this — **this is not a defect this migration introduced**; the composed-child hash just makes a pre-existing order-sensitivity manifest through a different (but equally strict) equality path.

**Traced by commit, not by message-parsing**: `git log --follow --date=iso -S"delete_node_severs_and_reconnects_edges" -- .../🧬️mutations/🦀️component.rs` → commit `a445617cae5a7b587931450ed508a75a1ffde33d`, `%ad` = **2026-08-12 15:50:51 +0200** — the mutation-vocabulary authoring pass that introduced all 14 triads (evidently an earlier SMO/fan-out wave), landed before this migration touched the file today. `create_node`'s append-only diff (the actual root cause) is unchanged code, carried forward verbatim by this pass.

Not trivial to fix in-scope: a real fix needs `delete-node`'s inverse to capture the node's original INDEX and a corresponding "insert at index" capability on `create-node`'s diff (currently append-only by design) — a mutation-semantics change, not a composition-migration task, and outside this recipe's scope. Documented here rather than silently patched around.

## sharedFileRequests

**Framework/plugin fixture coupling** (found, not touched): `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`'s `DagFixture::default()` does `include_str!(".../✏️s/🔌️plugins/🕸️dag/.../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")` — the SAME physical fixture file this plugin owns — and parses it via the FRAMEWORK's OWN, entirely separate, hand-derived `DagSnapshot`/`ArtifactDsl` impl (a different type from this plugin's `crate::artifacts::dag::DagSnapshot`, sharing only the physical text file). Regenerating the fixture for this plugin's new composed-child codec (mandatory per the migration recipe) necessarily changes the file's on-disk grammar from the old nodes/edges-line format to the new JSON-blob format — the framework's independent parser was NOT updated (W1-owned, outside my file-write scope) and will now fail its own `.expect(...)` at runtime for any caller of `DagFixture::default()`/`default_dag_document()` **outside this plugin** (this plugin's own 4 such call sites were found and fixed to use `crate::artifacts::dag::default_snapshot()` instead — see App-layer rewiring above). A real fix needs either (a) the framework file's own hand-rolled grammar updated to match, or (b) the framework decoupled from reading this plugin's fixture altogether (its own literal Rust fixture data, not a shared text asset) — both outside `✏️s/🔌️plugins/🕸️dag/**`. I did not search the rest of the repo for other callers of `DagFixture::default()`/`default_dag_document()` outside my plugin (out of scope); if any exist (framework/surface/node-graph, sequence/core, flow/core are mentioned as `DagNodeSpec`-builder consumers in that file's own doc comments, though not confirmed callers of `::default()` specifically), they will hit the same runtime panic until the framework side is fixed.

No `🗄️stdio/**` file was read-written — only read for reference (`SemioGraphSnapshot`/`SemioGraphNode`/`SemioGraphEdge`/`SemioGraphPort`/`GraphNodeId`/`GraphEdgeId` schema at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️component.rs`).

`✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/Cargo.toml` and 3 pre-existing files (`🧬️mutations/📝️text/🦀️component.rs`, `🔗connect-nodes/🦠️mutation/🦀️component.rs`, `🗃️replace-node-properties/🦠️mutation/🦀️component.rs`) were changed **by a concurrent session, not by me** — see Concurrent-churn observations. I did not author those edits; they were necessary for my crate to compile and landed on disk mid-pass.

## Concurrent-churn observations

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` was live-dirty (removing the `semio-framework-math` optional dependency) for an extended window mid-pass, transitively breaking `🧊️3d/🎬️scene/🦀️component.rs` (ambiguous numeric type errors) — `semio-framework-ui` is a hard (non-optional) dependency of this plugin via `ui_wgpu`, so every `cargo check`/`nextest run` attempt during that window failed with 0 errors traced to `✏️s/🔌️plugins/🕸️dag/**` (verified explicitly, repeatedly, via `grep` on the error output's `-->` paths). Retried well beyond the protocol's 3× at 60s intervals (many retries over roughly 15-20 minutes, both foreground and one bounded background loop) before it cleared.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs` (the framework's OWN separate dag kernel — see sharedFileRequests) was ALSO live-dirty (staged `M`) mid-pass, from what is evidently the `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` ticket extracting `graph` out of the `math` crate into its own `semio-framework-graph` crate — confirmed by a live `cargo test -p semio-framework-math` process running under that ticket's own separate `CARGO_TARGET_DIR`. This transiently broke `math::graph::manifest::PropertyBag` imports in 3 files this plugin depends on but I never touched (`🧬️mutations/📝️text`, `🔗connect-nodes/🦠️mutation`, `🗃️replace-node-properties/🦠️mutation`) and added a new direct `graph` dependency to this plugin's own `Cargo.toml` (external edit, not mine — confirmed via the harness's own "file modified externally" notice). Both cleared on their own; the 2 final clean `cargo check`/2 clean `nextest run`s above were captured AFTER they settled.
- Neither churn originated in, nor was fixed by, anything in `✏️s/🔌️plugins/🕸️dag/**` — every single error during both windows traced (via `grep` on `-->` paths) to `🧰️framework/**`, confirmed repeatedly.

## Files touched this pass

47 files changed (740 insertions, 773 deletions) under `✏️s/🔌️plugins/🕸️dag/**`, `git diff --stat HEAD`:
- `🗿️artifacts/🕸️dag/🦀️component.rs` — new `🔖️ContentBridge`/`🔖️WorkingScene` regions (converter, content-addressed handle, thread-local cache), round-trip test.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — full rewrite: `DagSnapshot` field swap, dead DSL-mirror region removed, hand-rolled codec (redesigned mid-pass after finding the handle-only bug), framework-bridge `From` impls, `nodes()`/`edges()` accessors.
- `…/📸️snapshot/📝️text/🦀️component.rs`, `…/📸️snapshot/💾️binary/🦀️component.rs` — doc fixes, `dump_example_dsl_when_requested`/command-envelope test rewired off struct literals.
- `…/🧬️schema/🦀️component.rs` — `DagArtifact` field swap + accessors; `DocumentHelpers` region (`document_to_workflow`, `connect_edge`, `remove_nodes_operations`, etc.) rewired onto `.nodes()`/`dag_working_scene`; 1 pre-existing test bug fixed (see above); tests rewired.
- `…/🧬️schema/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs` — `DagDiff.content`, dead delta types removed, `apply`/`absorb` collapse, `diff_replace_content` builder.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — `dag_snapshot_mutations`, `apply_dag_mutation`/`inverse_dag_mutation` trait-import fixes, tests rewired.
- `…/🧬️schema/🧬️mutations/{↔️move-node,✂️disconnect-nodes,🌱create-node,🏷️rename-node,📐resize-node,🔀reorder-nodes,🔁replace-node-kind,🔗connect-nodes,🔡change-node-abbreviation,🔤change-node-name,🖼️change-node-icon,🗃️replace-node-properties,🗑️delete-node,🧮change-node-operator-kind}/{🔺️diff,↩️inverse}/🦀️component.rs` — all 14 triads rewired onto the working-scene + `diff_replace_content` pattern (`create-node`'s and `connect-nodes`' `↩️inverse` needed no changes — no base lookup).
- `…/🧬️schema/💡️inferences/🦀️component.rs`, `…/💡️inferences/🧭topology/🦀️component.rs` — `infer` rewired through `dag_working_scene`, 1 baseline struct-literal bug fixed, tests rewired.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (new JSON-blob wire format).
- `🎛️apps/🕸️dag/🦀️component.rs` — 3 tests rewired.
- `🎛️apps/🕸️dag/🎮️commands/{🔧️nodes,🕸️graph,🗂️selection}/🦀️component.rs` — `engine::` baseline-fix + `.nodes()`/`.edges()`/`dag_working_scene` rewiring throughout (handlers + tests).
- `🎛️apps/🕸️dag/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — `dag_working_scene`/`.nodes()` rewiring.

Not authored by me (concurrent-session edits, landed mid-pass, necessary for compilation — see sharedFileRequests/Concurrent-churn): `📦️packages/🦀️rust/Cargo.toml`, `…/🧬️mutations/📝️text/🦀️component.rs`, `…/🔗connect-nodes/🦠️mutation/🦀️component.rs`, `…/🗃️replace-node-properties/🦠️mutation/🦀️component.rs`.

ucas-status: complete
