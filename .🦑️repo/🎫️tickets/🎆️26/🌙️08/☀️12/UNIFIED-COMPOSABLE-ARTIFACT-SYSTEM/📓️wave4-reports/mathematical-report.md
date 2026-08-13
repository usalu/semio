# W4 batch B — `mathematical` composes stdio `text`, `table`, `value`

**ucas-status: complete — 72/73 tests passing (stable across 2 consecutive full runs), 0 compile errors, 1 failure independently traced to a pre-ticket commit (evidence below); degenerate `"a"` kind id from the design brief NOT found despite an exhaustive search (documented honestly, see `## The "id \`a\` dies" finding`).**

## Baseline (before any edit)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-mathematical --all-targets
```
Result: **0 errors**, only pre-existing style warnings (ambiguous `testkit` glob import between `os_spr`/`os_pack`, a handful of "unnecessary qualification" / unused-import lints). Baseline was green, unlike writer's/cad's red baselines.

## What mathematical was duplicating

`MathematicalSnapshot` (`🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`) held two inline fields: `graph: MathematicalGraph` (a node-graph playground: `directed`, `nodes: Vec<{id,label,x,y}>`, `edges: Vec<{id,source,target}>`, `algorithm`, `algorithm_seed`) and `geometry: MathematicalGeometry` (`points: Vec<{x,y}>`, a convex-hull/centroid playground). Neither type reused any stdio subset — this was hand-rolled duplication of exactly the shapes `text`/`table`/`value` already generalize:

- **text** (`s.stdio.semio.text`, `SemioTextSnapshot{ schema, runs: Vec<SemioTextRun{language,content,marks}> }`) ← node **labels** (the prose/notation half of a node).
- **table** (`s.stdio.semio.table`, `SemioTableSnapshot{ schema, columns, rows: Vec<SemioTableRow{cells: Vec<SemioValue>}> }`) ← node **id/x/y** (tabulated results — one row per node, positionally aligned with the text runs).
- **value** (`s.stdio.semio.value`, `SemioValueSnapshot{ schema, root: SemioValue, nodes }`) ← everything else: `directed`/`algorithm`/`algorithmSeed` plus the full `edges` list and the geometry `points` list, all folded into one `SemioValue::Map` — genuinely "scalar/structured computed values" per the migration brief, not independently-authored prose.

Splitting one node's `label` into `text` and its `id`/`x`/`y` into `table` (rather than duplicating the whole node into both) keeps the two children non-redundant while staying real and lossless: both are always regenerated together from the SAME `graph.nodes` iteration order, so a round trip zips them back 1:1.

## What changed

### Composition machinery (new, artifact root)

`🗿️artifacts/➗️mathematical/🦀️component.rs`, new `🔖️Composition` region (`🔖️ChildTypes`/`🔖️Converters`/`🔖️WorkingScene`):
- `MathematicalNotationChild`/`MathematicalResultsChild`/`MathematicalComputedChild` — `store::ArtifactChild<SemioTextSnapshot|SemioTableSnapshot|SemioValueSnapshot>` type aliases.
- **Real bidirectional converters**: `mathematical_notation_from_graph`/`mathematical_results_from_graph`/`mathematical_computed_from_state` (graph+geometry → the three subsets) and `mathematical_graph_geometry_from_children` (the inverse — zips `results` rows with `notation` runs back into `MathematicalNode`s by index, and walks the `computed` value's `Map` entries back into `directed`/`algorithm`/`algorithmSeed`/`edges`/`points`). Degrades honestly (empty id/label, `0.0` coordinate) on a short/missing row rather than panicking — documented in the converter's own doc comment, per the recipe's honesty requirement.
- **Working scene**: `MathematicalWorkingScene{ graph, geometry }` in a `thread_local! MATH_SCRATCH: RefCell<HashMap<String, MathematicalWorkingScene>>` — never persisted, matches the `EngineRep` contract (same shape as flow's `FLOW_SCRATCH`/writer's `WRITER_SCRATCH`). Because `notation`/`table`/`value` for a given `(graph, geometry)` pair are always minted TOGETHER by `mathematical_children_from_state`, all three share ONE content-addressed `scene_id` (`mathematical-scene-<hash>`) as their `child_id` — only their `target.dialect.subset` differs (`text`/`table`/`value`) — so one cache entry serves all three reads. `mathematical_graph(&snapshot)`/`mathematical_geometry(&snapshot)` are the two read accessors every render/inference/export/command call site in the plugin now funnels through instead of the old `.graph`/`.geometry` field access; both fail soft (empty graph/geometry) on a cache miss, never panic — same documented staleness gap as every prior exemplar (store-level undo/redo bypasses `ArtifactApp::handle`).
- `mathematical_snapshot_with_state(graph, geometry) -> MathematicalSnapshot` — the fixture/import constructor replacing the old 2-field struct literal.

### Snapshot / composed children

`📸️snapshot/🦀️component.rs`: `MathematicalSnapshot.graph`/`.geometry` → `notation: MathematicalNotationChild #[child(kind="s.stdio.semio.text")]`, `results: MathematicalResultsChild #[child(kind="s.stdio.semio.table")]`, `computed: MathematicalComputedChild #[child(kind="s.stdio.semio.value")]`. Hand-rolled `ArtifactDsl`/`ArtifactPack` directly on `MathematicalSnapshot` (writer/cad's upgrade path) — `🔖️ChildCodecPrimitives` (hex/bracket handle codec), `🔖️TextPrimitives`/`🔖️BinaryPrimitives` (one `key=[childId,target]` line per field / LEB128 binary), `🔖️HandcraftedArtifactCodecs`. `Default` now calls `mathematical_snapshot_with_state(MathematicalGraph::default(), MathematicalGeometry::default())`.

`MathematicalArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical 3-field swap; `to_snapshot`/`from_snapshot`/`set_snapshot`/`default_ui` updated.

### DSL-mirror cleanup (`📸️snapshot/📝️text/🦀️component.rs`)

The former `MathematicalSnapshotDsl`/`impl ArtifactDsl for MathematicalSnapshotDsl`/`impl ArtifactPack for MathematicalSnapshotDsl`/`mathematical_snapshot_to_dsl`/`mathematical_snapshot_from_dsl` are **removed** — that indirection was the snapshot's OWN codec, now hand-rolled directly per above. `MathematicalGraphDsl`/`MathematicalEdgeDsl`/`math_graph_to_dsl`/`math_graph_from_dsl`/`math_edge_to_dsl`/`math_edge_from_dsl` are **kept** — they are the `SetArtifact` app command's own DSL payload shape (`🎮️commands/📄️artifact/🦀️component.rs`'s `SetArtifact{ graph: MathematicalGraphDsl, geometry: MathematicalGeometry }`, unrelated to the snapshot's persisted representation).

### Diff

`🔺️diff/🦀️component.rs`: `graph`/`geometry` fields → `notation`/`results`/`computed: Option<ArtifactChild<S>>` (single-Option, always-present slot — writer's `document` shape, not lowpoly's double-Option optional-slot shape, since a mathematical snapshot's three children are never absent). The former `artifact: Option<Box<MathematicalArtifact>>` whole-snapshot-replace slot is **removed** — it was dead code (grepped: never constructed by any app command; `SetArtifact` already routed through the granular `ReplaceGraph`/`ReplacePoints` mutations, not a whole-snapshot replace) and would otherwise be exactly the banned `SetSnapshot` vocabulary.

`🔺️diff/📝️text/🦀️component.rs`: `apply`/`apply_to_artifact`/`absorb` rewired to whole-handle replace on the three new fields (dropped the dead `artifact` branch). New `diff_from_state(graph, geometry) -> MathematicalDiff` builder — mints+caches all three children via `mathematical_children_from_state` and wraps them as `Some(...)`; replaces the unused `diff_set_graph`/`diff_set_geometry` helpers (also dead — grepped, no callers).

### Mutation triads (14 kinds — every `diff.rs`/`inverse.rs` that touched `.graph`/`.geometry`)

`change-graph-directed`, `update-graph-algorithm`, `replace-graph`, `create-node`, `delete-node`, `delete-nodes`, `change-node-label`, `move-node`, `connect-nodes`, `disconnect-nodes`, `replace-points`, `insert-point`, `remove-point`, `move-point`. Every one of these 28 files (`🔺️diff` + `↩️inverse`) is mechanically the SAME shape as before — the same node/edge/point CRUD algorithm, byte-for-byte identical — with exactly two changes: `base.graph.clone()`/`base.geometry.clone()` → `mathematical_graph(base)`/`mathematical_geometry(base)` (cache read instead of field read), and `MathematicalDiff{ graph: Some(graph), .. }` → `mathematical_children_from_state(&graph, &geometry)` fed into `MathematicalDiff{ notation: Some(_), results: Some(_), computed: Some(_), .. }` (mint-all-three instead of whole-slot replace, since text/table/value are three co-derived projections of the SAME `(graph, geometry)` pair — a graph-only or geometry-only mutation still regenerates all three, by design, to keep the invariant "all three children of one snapshot share one scene id" trivially true). `📝️text/🦀️component.rs` (the mutation payload's own `OpText`/`OpBinary` wire codec) needed **zero changes** — it encodes payload structs (`CreateNode{id,label,x,y}`, `ReplaceGraph{graph: MathematicalGraph}`, …) directly, never the snapshot, and `MathematicalGraph`/`MathematicalPoint`/etc. are unchanged Rust types (they moved from being snapshot fields to being cache-only working-scene types, but the types themselves are untouched).

### Inference / app layer

`💡️inferences/🦀️component.rs`: `compute_mathematical_topology(&snapshot.graph)` → `compute_mathematical_topology(&mathematical_graph(snapshot))`; `InferenceFieldSpec.reads` updated from `&["graph"]` to `&["notation","results","computed"]`.

`🎛️apps/➗️mathematical/🦀️component.rs`: `render()`'s `graph_window::render(&doc.snapshot.graph, ..)`/`geometry_window::render(&doc.snapshot.geometry)` and `export_media("result:out", ..)`'s `algorithm_overlay(&doc.snapshot.graph)` rewired through `mathematical_graph`/`mathematical_geometry`. `🎮️commands/{🕸️graph,📐️geometry,📄️artifact}/🦀️component.rs` — every command handler (`SetAlgorithm`, `SetDirected`, `NodeGraphEdit`, `SetArtifact`) that read `doc.snapshot.graph`/`.geometry` directly now reads through the same two accessors; ~20 test call sites across these files (`app.snapshot().graph...`) updated identically.

### Whole-document replace — nothing to remove

Checked (grep, before any edit): `ArtifactApp for MathematicalPlayApp` never overrode `whole_document_operation` — already the trait default `None`. `SetArtifact` was already routing through the granular `ReplaceGraph`/`ReplacePoints` mutations, never a whole-snapshot replace — no `reset_document_effect`/`HostEffect::LoadDocument` conversion was needed here (unlike writer/cad). The only whole-replace-shaped thing in this plugin was the dead `MathematicalDiff.artifact`/`diff_set_snapshot` pair described above, removed as dead code.

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was in the pre-migration `graph{...}geometry{...}` DSL-record grammar, incompatible with the new hand-rolled `notation=[…]\nresults=[…]\ncomputed=[…]` codec. The fixture's content (default graph: 4 nodes `a`-`d`, 4 edges, `topo` algorithm, directed; default geometry: 6 points) is exactly `MathematicalGraph::default()`/`MathematicalGeometry::default()`, so it's also exactly `MathematicalSnapshot::default()`'s own DSL text. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` (`cargo test … debug_fixture_regen -- --nocapture`), captured the real `print_dsl()` output, wrote it as the new fixture, removed the temporary module cleanly (verified: `grep -rn debug_fixture_regen` → nothing).

## Working-scene design

See `MathematicalWorkingScene`'s own doc comment (`🗿️artifacts/➗️mathematical/🦀️component.rs`, `🔖️WorkingScene` region). Summary: a `thread_local! HashMap<child_id, MathematicalWorkingScene>` cache, matching flow's `FLOW_SCRATCH`/writer's `WRITER_SCRATCH` pattern exactly, scaled to three co-derived children sharing one scene id. Populated at mutation-diff-build time (`mathematical_children_from_state`, called from every one of the 14 diff functions) and at fixture-construction time (`mathematical_snapshot_with_state`, used by `Default` and every test fixture builder). No `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle` yet — checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only), same standing gap every prior wave's report documents.

## Converters (real, not stubs)

`mathematical_notation_from_graph`/`mathematical_results_from_graph`/`mathematical_computed_from_state`/`mathematical_graph_geometry_from_children` (`🗿️artifacts/➗️mathematical/🦀️component.rs`, `🔖️Converters` region) — a real, lossless, positionally-aligned round trip for the node data (`text` runs ↔ `table` rows ↔ `graph.nodes`, zipped by index) plus a real structured-value round trip for everything else (`directed`/`algorithm`/`algorithmSeed`/`edges`/`points` ↔ one `SemioValue::Map`). Exercised indirectly by every mutation round-trip test (`create_then_delete_node_round_trips`, `move_point_inverse_restores_old_position`, etc. — all now read back through `mathematical_graph`/`mathematical_geometry`, which round-trip through these converters on every diff).

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-mathematical --all-targets
```
**0 errors**, confirmed on the final run. Remaining warnings are pre-existing/cosmetic (ambiguous `testkit` glob import from framework glue, several "unnecessary qualification"/unused-import style lints, a `hidden_glob_reexports` warning on the pre-existing `MathematicalDiff` import in `diff/text` — none touched by this pass, none block compilation).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-mathematical --no-fail-fast
```
**72/73 passed**, reproduced stable across 2 consecutive full runs (same single failure both times, not flaky). No test was deleted; one test (`every_printed_op_line_starts_with_the_rows_wire_keyword`) had its hardcoded exception list extended by one row (see below); one test (`insert_point_inverse_is_remove_point_at_same_index`) is left failing with full provenance (see below).

## Fixed outright (trivial, unambiguous, independently traced)

`apps::mathematical::component::tests::every_printed_op_line_starts_with_the_rows_wire_keyword` failed with `left: "set-artifact" right: "set-document"`. Root cause: the test computes its expected wire keyword by kebab-casing the manifest action id, with only `"setLocale"` hardcoded as an exception — but the `app_commands!` row `"setDocument" as "set-artifact" => set_artifact::SetArtifact` (and `SetArtifact`'s own `#[dsl(keyword = "set-artifact")]`) declares a SECOND, undeclared-in-the-test divergence. `git log -1 --date=iso -- 🎮️commands/📄️artifact/🦀️component.rs` → `31209e7a… 2026-08-13 00:13:16` — this row predates any edit I made to this file (I only touched `render`/`export_media`, verified via `git diff` before editing). Fixed by adding `"setDocument" => "set-artifact".to_string()` as a second documented match arm, mirroring the existing `"setLocale"` exception.

## Honest gap — 1 pre-existing failure, not fixed, full provenance

`artifacts::mathematical::…mutations::component::tests::insert_point_inverse_is_remove_point_at_same_index` fails: after `insert-point(index=1)` then undo, the restored geometry has 5 points instead of the original 6 (missing the point that was originally at index 1).

**Root cause (verified by hand, not guessed)**: `remove-point`'s `diff` function computes `geometry = mathematical_geometry(base).clone(); geometry.points.remove(index)` — i.e. it clones the ORIGINAL `base` geometry and removes an index from it, then the diff's `apply` REPLACES the whole slot on `state` unconditionally. When `state` (the post-insert 7-point geometry) differs from `base` (the original 6-point geometry), removing index 1 from `base` produces `base` **minus one of its own original points**, not `state` **minus the newly-inserted point** — a different, wrong 5-point result. This is a structural bug in the diff's own math, independent of storage: I hand-traced the exact same arithmetic against the PRE-migration field-based code (`base.geometry.clone()` instead of `mathematical_geometry(base)` — otherwise byte-identical logic) and it reproduces IDENTICALLY (5 points, missing the same element) — this migration changed WHERE the data is read from (cache vs. field), never the diff algorithm itself, which I preserved verbatim per mutation.

**Dating**: `git log -1 --date=iso -- …/➖️remove-point/🔺️diff/🦀️component.rs` and `…/➕️insert-point/↩️inverse/🦀️component.rs` → both `16619a96… 2026-08-12 11:09:41` — **before this ticket opened** (`2026-08-12 15:02:49`, per `📌️important.md`). Confirmed via `%ad`, not the commit message's fake `🎆️26🌙️06☀️04` glyphs.

**Why not fixed outright**: this is the SAME class of bug `📌️important.md`'s own "D2 — Concern B" flags for stdio's `✳️text`/`✳️table`/`✳️graph` subsets — a whole-collection-slot diff computed from `base` instead of the live `state`, which only round-trips correctly for mutations that patch a value already present at that base position (move/label-change/toggle — all of which DO pass) and silently corrupts on any mutation that changes collection LENGTH (insert/remove — this one fails). `important.md` explicitly defers this class of fix to a dedicated DiffKit rework (`IndexedTripleDiff`/`NamedTripleDiff`) "before this reaches the 33-plugin fan-out," i.e. it is a known, ticket-level, deliberately-out-of-scope architectural concern, not a migration-introduced regression — fixing it here would mean redesigning the diff-application contract for all 14 of this plugin's structural mutations, well beyond "migrate to composed children." Flagged here with full derivation so a future DiffKit pass (or a dedicated bug ticket) can pick it up with zero re-investigation cost.

## Concurrent-churn observations

One transient `cargo check` failure during final re-verification: `semio-framework-plugin` (a W1-owned framework dependency, outside this plugin's boundary) failed to compile with an E0432 error on one run. Immediately retried in the foreground (no background wait, per `📌️important.md`'s dispatch rule) — the very next `cargo check -p semio-s-plugin-mathematical --all-targets` came back clean (0 errors, "Finished") with no changes made on my side, confirming it was another session's in-flight edit to `semio-framework-plugin` settling mid-build, not a defect in this plugin. Re-ran `cargo nextest` afterward to reconfirm the stable 72/73 result.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/➗️mathematical/**` (including this plugin's own fixture asset), never touching `📦️glue.rs`/`📦️index.ts` or any `🗄️stdio/**` file (only read for schema reference: `SemioTextSnapshot`/`SemioTableSnapshot`/`SemioValueSnapshot` and their leaf types).

## The "id `a` dies" finding

Per the dispatch brief (`📓️design-full-plan.md` §4: `mathematical→C:text,table,value (id \`a\` dies)`), I searched exhaustively for a degenerate artifact-kind id literally `"a"` (or similarly short) in this plugin's artifact-kind registration: `artifact_kind()` (`🗿️artifacts/➗️mathematical/🦀️component.rs`, `id: "computation.mathematical"`), `ArtifactDeclaration::builder("s.mathematical")`, every `#[artifact_schema(id = "s.mathematical.mathematical"[.config|.presence|.inference])]`, the `Dialect`/`ArtifactPresentation`/`MediaPortSpec` literals in `mathematical_io()`, `MATH_APP_ID`, and every body-key constant — grepped both `id: "a"` and bare `"a"` across every file type (`.rs`/`.json`/`.graphql`/`.proto`/`.ts`). The only literal `"a"` strings in the whole plugin are the fixture graph's node-id (`MathematicalNode{id:"a",..}`, the same `"a"`/`"b"`/`"c"`/`"d"` demo node ids used throughout the mutation tests) — not a kind id anywhere. All current kind ids already look canonical and match the pattern established by writer (`text.document`/`s.writer.writer`) and cad. **I did not find and therefore did not fabricate a fix for this** — consistent with `📌️important.md`'s own caution against unverified counts/claims (the "20 dangling mounts, actually 2" story). If this refers to a registration outside this plugin's boundary (e.g. a stdio catalog row or the W6-owned taxonomy), it is out of my scope to touch and I have no evidence such a row exists.

## Files touched this pass

- `🗿️artifacts/➗️mathematical/🦀️component.rs` — new `🔖️Composition` region (child types, converters, working scene, `mathematical_children_from_state`/`mathematical_graph`/`mathematical_geometry`/`mathematical_snapshot_with_state`).
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `MathematicalSnapshot` field swap, hand-rolled codecs.
- `…/🧬️schema/🦀️component.rs` — `MathematicalArtifact` field swap, conversions.
- `…/🧬️schema/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs` — `MathematicalDiff` field swap (dead `artifact` slot removed), apply/absorb, `diff_from_state`, tests.
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — `MathematicalSnapshotDsl` removed (kept `MathematicalGraphDsl`/`MathematicalEdgeDsl` for `SetArtifact`), fixture regen, test fixes.
- `…/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — test fixture fix.
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `compute_mathematical_topology` call site, `InferenceFieldSpec.reads`.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch-level test fixes (11 tests).
- `…/🧬️schema/🧬️mutations/{✂️disconnect-nodes,❌️delete-node,➕️insert-point,➖️remove-point,🌀️replace-points,🎯️move-point,🏷️change-node-label,🔀️change-graph-directed,🔁️replace-graph,🔗️connect-nodes,🕹️move-node,🗑️delete-nodes,🟢️create-node,🧮️update-graph-algorithm}/{🔺️diff,↩️inverse}/🦀️component.rs` — 28 files, mechanical rewiring per `## Mutation triads` above.
- `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` — `mathematical_graph` call site.
- `🎛️apps/➗️mathematical/🦀️component.rs` — `render`/`export_media` rewiring, `every_printed_op_line_starts_with_the_rows_wire_keyword` pre-existing-bug fix.
- `🎛️apps/➗️mathematical/🎮️commands/{🕸️graph,📐️geometry,📄️artifact}/🦀️component.rs` — handler + test rewiring.

ucas-status: complete
