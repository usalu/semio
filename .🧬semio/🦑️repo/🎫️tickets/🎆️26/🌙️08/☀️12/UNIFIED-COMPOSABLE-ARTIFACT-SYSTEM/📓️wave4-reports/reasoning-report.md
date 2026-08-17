# W4 — `reasoning` (wires) composes stdio `graph`

**ucas-status: complete — 78/79 tests passing (reproduced identically across two consecutive clean full `cargo nextest run` runs), 0 compile errors on two independent consecutive `cargo check -p semio-s-plugin-reasoning-mindmap --all-targets` runs; the 1 remaining failure is independently traced by commit hash + `git log --date=iso` to a pre-existing, pre-migration mutation-semantics defect in `🚩set-node-root`'s inverse (authored by SMO's fanout wave, commit `880c37b4be`, 2026-08-13 01:03:02) unrelated to composition — evidence below.**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-reasoning-mindmap --all-targets` was run BEFORE touching any file. It was already **red**: 22 errors (lib) / 24 errors (lib-test), saved in full to `scratch-reasoning-baseline.txt`. Breakdown (`grep "^error\[" | sort | uniq -c`):
- `E0252` × 10 — "the name `create_node` is defined multiple times" (and 9 sibling triads) in `🧬️mutations/🦀️component.rs`: a bare `use super::create_node;` (needed for the enum body's module-path resolution) collided with `pub use create_node::mutation::create_node` (the builder FN of the same name) in the value namespace.
- `E0433` — `semio_framework::kernel::HostEffect` unresolved: the app root (`🎛️apps/🔌️wires/🦀️component.rs`) already had `reset_wires_document_effect` written (a `HostEffect::LoadDocument` builder, exactly the recipe's §5 pattern), but the plugin's own `Cargo.toml` never gained the `semio-framework` dependency — a genuinely incomplete prior attempt at this exact migration task, left mid-way.
- `E0599` × 8 — `WiresMutation::AddNode`/`RemoveNode`/`PatchNode`/`AddRelationship`/`RemoveEdge` referenced across 6 app-command files: these six generic mutation variants were deleted by an earlier SMO mutation-vocabulary fan-out pass (the file's own doc comment says so explicitly), but the app layer was never updated to the new `CreateNode`/`DeleteNode`/`MoveNode`/`ConnectNodes`/`DisconnectNodes` vocabulary.
- `E0560`/`E0609` × 4 — stdio's `CsvSnapshot` was independently reshaped (`headers`/`rows` → `has_header`/`records`) by unrelated concurrent churn; this plugin's own CSV import/export leaves were never updated.
- `E0308` — `JsonSnapshot.value: JsonValue` vs `serde_json::Value` type mismatch (stdio's own JSON wrapper type, unrelated to composition).

All were fixed in-pass since every affected file was already being rewritten for composition (E0252/E0433 directly, E0599 as part of the app-layer accessor rewrite, E0560/E0609/E0308 as part of getting the whole crate green).

## What changed

### Snapshot (`🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)

`WiresSnapshot.board_fixture: DslValue` (a duplicated nodes/edges/camera/meta graph blob) → `content: store::ArtifactChild<SemioGraphSnapshot>` (`#[child(kind = "s.stdio.semio.graph")]`), plus two small persisted fields that stay OUTSIDE the composed child: `camera: DslValue` (pan/zoom view state) and `meta: DslValue` (kind-catalog / allowed-identity app config, `Null` when absent). `wires_fixture: DslValue` (the identities/relationships semantic layer, including its own pre-existing internal `board` mirror) is **unchanged** — see "Scoping decision" below.

**Codec wall hit exactly as the recipe predicted**: `WiresSnapshot` previously bridged to a hand-derived mirror (`WiresSnapshotDsl` + the artifact root's whole `🔖️DslMirror` region — `CameraDsl`/`NodeDsl`/`EdgeDsl`/`BoardFixtureDsl`/`WiresFixtureDsl`/`IdentityDsl`/`RelationshipDsl`/`IdentityKindDsl`/`RelationshipKindDsl`/`KindCatalogsDsl`/`MetaWiresDsl`/`MetaDsl`/`SourceDsl` — 13 `#[derive(dsl::DslRecord)]` structs) solely to give the derive engine a typed path through the board's dynamically-shaped `DslValue`. Since `content` is now opaque, that whole 13-struct mirror is dead — confirmed via `grep` (zero real consumers beyond the codec itself) and deleted outright. Hand-rolled `ArtifactDsl`/`ArtifactPack` instead: text `wires=<hex>` / `nodes=<hex>` / `edges=<hex>` / `camera=<hex>` / `meta=<hex>`, five lines; binary length-prefixed mirror of the same five fields.

**A real bug found and fixed during this pass, not just latent risk the recipe warned about** (two, actually):
1. **Bare-handle codec risk** — avoided from the start by following `dag`'s precedent directly: `parse_dsl`/`decode_pack` decode the REAL `nodes`/`edges` JSON and mint+cache a fresh, deterministic, content-addressed handle from them every time, never persisting only `(child_id, target)`.
2. **Key-order-losing JSON round trip** — my first codec draft encoded each `DslValue` field via `crate::artifacts::wires::schema::fixture_json_string`/`dsl_to_json`, which round-trips through `serde_json::Value` (whose `Object` variant normalizes/sorts keys — no `preserve_order` feature). This silently REORDERED `wires_fixture`'s object keys on every round trip, which broke `DslValue::Object`'s order-sensitive (`Vec`-backed) `PartialEq` — caught immediately by 6 real test failures (`dsl_text_round_trips_empty`, `pack_round_trips_empty`, `document_text_round_trip_with_operation_applied`, `set_node_root_round_trip`, `inference_determinism_law`, `dsl_round_trip_empty_document`), NOT by `cargo check`. Fixed by encoding/decoding `DslValue` DIRECTLY via `serde_json::to_string(&value)`/`serde_json::from_str::<DslValue>(text)` — `DslValue`'s own hand-written `Serialize`/`Deserialize` impl (`dsl_value_serde.rs`) preserves entry order end-to-end; the bug was specifically in going through the lossy `serde_json::Value` intermediate. Documented prominently in the file's `🔖️CodecPrimitives` region so a future reader doesn't reintroduce it. This fixed 5 of the 6 failures outright (see "Remaining failure" below for the 6th).

### Scoping decision — `wires_fixture`'s own internal `board` duplicate

`wires_fixture: DslValue` (identities/relationships) contains its OWN nested `board` mirror (documented in the pre-migration module doc as "the same `BoardFixtureDsl` shape as the top-level `board_fixture`" — i.e. a second, independent duplicate of the graph, pre-dating this migration). This pass deliberately does **not** touch that internal shape or eliminate the duplication — the task's target field is `board_fixture` (the top-level graph blob that genuinely duplicated stdio's `graph` subset in the sense §4 means: nodes+edges+ports+properties). `wires_fixture.board` is a narrower, separate, plugin-internal duplication concern; touching it would require reworking `wires_fixture`'s whole shape (13 more DSL-mirror types, catalogue/inspection panel readers of `wires.get("board").get("meta")...`, the identity/relationship semantic layer itself) for no composition benefit — out of proportion to this ticket's scope. Every write site that used to sync `wires_fixture.board` from `board_fixture` (`handcrafted_metabolism_snapshot`'s final sync block) now syncs it from [`wires_working_board`] instead — same behavior, re-pointed at the new accessor. Flagged here explicitly as a known, intentionally-out-of-scope remaining duplication.

### `WiresArtifact` (`…/🧬️schema/🦀️component.rs`)

Identical field swap (`board_fixture` → `content: WiresContentChild`, `camera`, `meta`), matching `WiresSnapshot`. `to_snapshot`/`from_snapshot`/`set_snapshot`/`Default` updated. `DocumentHelpers` region (`array_mut`/`entity_id`/`dsl_id`/`dsl_to_json`/`fixture_json_string`/`fixture_camera`/`fixture_nodes`/`fixture_edges`/`wires_identities`/`wires_relationships`/`node_position`/`force_layout_board`) is **completely unchanged** — every one of these functions is generic over `&DslValue`, so they keep working unmodified against the RECONSTRUCTED board `wires_working_board` hands them. This is the main reason the blast radius stayed manageable: the working-scene accessor reconstructs the exact same board shape these helpers always expected.

### Composed-child bridge + converter + working scene (`🗿️artifacts/🔌️wires/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `WiresContentChild = store::ArtifactChild<SemioGraphSnapshot>`.
- **Real bidirectional converter** (not a stub): `wires_content_snapshot_from_scene(nodes, edges) -> SemioGraphSnapshot` / `scene_from_wires_content_snapshot(&SemioGraphSnapshot) -> (Vec<DslValue>, Vec<DslValue>)`. This app's board nodes/edges are dynamically-shaped `DslValue` objects (not a fixed Rust struct — unlike `dag`'s typed `DagNodeSpec`), so the "honest string boundary" pattern applies at the JSON level directly: the FULL raw node `DslValue` round-trips as JSON in one `SemioValueEntry` property (`wires.node`), the SOURCE OF TRUTH on decode; `id`/`label`(=`text`)/`kind`(=`nodeKind`)/`position`(=`x`,`y`) are ALSO projected onto `SemioGraphNode`'s own native fields for genuine graph-shape tooling. `SemioGraphEdge` has no `properties` slot (unlike nodes), so the FULL raw edge `DslValue` round-trips as JSON in the `label` field (never populated by this app's own edges otherwise); `id`/`source`/`target` are also projected onto native fields, `kind` from `edgeKind` when present. Every field of both directions round-trips — verified by `node_edge_content_round_trips_through_the_composed_child_snapshot`, which specifically includes `radius`/`root`/`edgeKind` (fields `SemioGraphNode`/`SemioGraphEdge` have no native slot for).
- `wires_content_child_handle(nodes, edges) -> WiresContentChild` — content-addressed (`DefaultHasher` over the converted `SemioGraphSnapshot`'s JSON), same pattern as `dag_content_child_handle`/`document_child_handle`. Verified deterministic + content-sensitive by `content_child_handle_is_content_addressed_and_deterministic`.
- `WiresWorkingScene { nodes: Vec<DslValue>, edges: Vec<DslValue> }` + `thread_local!` `WIRES_SCRATCH: RefCell<HashMap<child_id, WiresWorkingScene>>` — never persisted, matches `EngineRep`. `wires_working_scene(&WiresSnapshot) -> WiresWorkingScene` is the raw nodes/edges read; `wires_working_board(&WiresSnapshot) -> DslValue` reconstructs the FULL legacy board shape (`schema`/`camera`/`nodes`/`edges`/`meta`?/`wires`) from the scene plus `snapshot.camera`/`.meta` — **the single accessor** every render/panel/command/diff call site that used to read `snapshot.board_fixture` now goes through. `wires_content_child_handle_and_cache` is the single mint+cache call site every diff builder, codec decode, and fixture converter goes through.
- Same documented staleness gap as every other exemplar (store-level undo/redo bypasses `ArtifactApp::handle`; fails soft to an empty scene, never panics).

### Diff (`…/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs`)

`WiresDiff.board_fixture: Option<DslValue>` → single `content: Option<WiresContentChild>` (never-absent-only-replaced shape, matching `dag`'s/writer's `document`/`content`, not lowpoly's `Option<Option<_>>`) plus `camera: Option<DslValue>`/`meta: Option<DslValue>`. `artifact: Option<Box<WiresArtifact>>` (the whole-artifact-replace escape hatch) **kept, unchanged** — confirmed (`grep whole_document_operation`) that `ReasoningWiresPlayApp` never overrides `whole_document_operation`, and `diff_set_snapshot`/`WiresDiff.artifact` is a structural VCS-level escape hatch (like writer's/cad's), not a `WiresMutation` variant or `WiresCommand` — no forbidden-vocabulary violation, nothing to remove (same finding as `dag`'s report).

`apply`/`apply_to_artifact`/`absorb` updated for `content`/`camera`/`meta`. New shared builder region: `diff_board_fixture(board: DslValue) -> WiresDiff` now extracts `nodes`/`edges` out of the board and mints+caches a content handle (camera/meta in the passed-in `board` are intentionally ignored here — no triad in this plugin ever writes them); `diff_wires_and_board` layers `wires_fixture` on top. `board_after_add_node`/`board_after_remove_node`/`board_after_patch_node`/`fixtures_after_add_edge`/`fixtures_after_remove_edge` (the shared diff-building helpers every triad routes through) now read the CURRENT board via `wires_working_board(snapshot)` instead of `snapshot.board_fixture.clone()` — a one-line change per helper, **zero changes needed in the individual triad files that call them** (`create-node`, `delete-node`, `connect-nodes`, `disconnect-nodes`).

### Mutation vocabulary — kept, rewired (10 triads unchanged in shape)

`create-node`/`delete-node`/`move-node`/`resize-node`/`change-node-kind`/`change-node-shape`/`edit-node-text`/`set-node-root`/`connect-nodes`/`disconnect-nodes` — payload types unchanged, no forbidden vocabulary (confirmed by grep — `SetSnapshot`/`NoMutation`/`CollectionMutation` don't appear). What changed: 6 triads (`move-node`/`resize-node`/`change-node-kind`/`change-node-shape`/`edit-node-text`/`set-node-root`) that directly did `base.board_fixture.clone()` in their own `diff.rs` now call `wires_working_board(base)` instead — a one-line swap each. `find_board_node`/`find_board_edge` (`💡️inferences/🦀️component.rs`) changed return type from `Option<&'a DslValue>` (borrowed from the now-gone `board_fixture` field) to owned `Option<DslValue>` (read through `wires_working_board`) — this rippled into every triad's `↩️inverse` leaf that chained `.and_then(|node| node.get(...))` off the result (an `E0515` "cannot return value referencing function parameter" once the closure owns `node` instead of borrowing it) — fixed by terminating each `.and_then` chain to an owned value INSIDE the closure (`.and_then(|node| node.get("shape").and_then(|v| v.as_str()).map(str::to_string))`) rather than returning a reference out of it. Also fixed a baseline `E0252` in the dispatch enum (`🧬️mutations/🦀️component.rs`): the bare `use super::create_node;` (and 9 siblings) collided with `pub use super::create_node::mutation::create_node` in the value namespace — removed the bare imports, fully-qualified every enum-variant payload type and builder re-export as `super::<slug>::...` instead.

### `whole_document_operation` — nothing to remove

Same finding as `dag`: `ReasoningWiresPlayApp`'s `ArtifactApp` impl never overrides it — no cleanup needed.

### App-layer rewiring (11 files)

`🎛️apps/🔌️wires/🦀️component.rs` (root: canvas render call, 5 tests), `🎮️commands/{🔵️node,🔗️relationship,🗑️delete,🖱️pointer,🔄️layout,🧬️example}` (6 files), `📌️panels/{📄️artifact,🔍️inspection}` (2 files) — every `document.board_fixture`/`snapshot.board_fixture` direct field access rewritten to `wires_working_board(document)`. Separately, **6 dead mutation-variant references** (baseline `E0599`, pre-dating this migration — the old generic `AddNode`/`RemoveNode`/`PatchNode`/`AddRelationship`/`RemoveEdge` variants were deleted by an earlier SMO fan-out but the app layer was never updated) fixed to the current vocabulary in the same pass since every touched file needed the accessor rewrite anyway: `WiresMutation::AddNode{node}` → `mutations::create_node(node)`; `RemoveNode{node_id}` → `mutations::delete_node(node_id)`; `RemoveEdge{edge_id}` → `mutations::disconnect_nodes(edge_id)`; `AddRelationship{edge,relationship}` → `mutations::connect_nodes(edge,relationship)`; `PatchNode{node_id,patch:{x,y}}` (both occurrences, `canvas_pointer_move` drag and `force_layout_operations`) → `mutations::move_node(node_id,new_x,new_y)` (the patch was always x/y-only in practice, so this is a lossless, simpler replacement, not a semantics change).

`✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml`: added `semio-framework = { path = "…", package = "semio-framework" }` — `reset_wires_document_effect` (`🎛️apps/🔌️wires/🦀️component.rs`, the `HostEffect::LoadDocument` builder `set_active_example` already used) referenced `semio_framework::kernel::HostEffect` without the dependency ever being declared; this was a baseline `E0433`, not something I introduced, but I needed the crate to compile regardless.

### IO serializers (`🚪️io/📤️export`/`📥️import`)

`svg`/`md`/`png`/`json` (both directions, 8 files) are generic pack/dsl passthroughs (`ArtifactPack::encode_pack`/`decode_pack`, `ArtifactDsl::print_dsl`/`parse_dsl`) — **unaffected**, no board-fixture-shaped code inside them. `csv` (both directions) was baseline-broken (`E0560`/`E0609`): stdio's `CsvSnapshot` was independently reshaped (`headers`/`rows` → `has_header`/`records`) by unrelated concurrent churn, and — confirmed by tracing the pre-migration logic — this leaf's `serialize`/`deserialize` never actually produced a non-degenerate `CsvSnapshot` even before that rename (`WiresSnapshot`'s derived JSON never had a `"headers"`/`"rows"` shape to begin with, so `value.get("headers")` always returned `None`). Left as an honest no-op passthrough (documented in-line) with the current field names, pending a real wires↔csv tabular-mapping design — not this migration's scope. `json` export also had an unrelated baseline `E0308` (`JsonSnapshot.value: JsonValue` vs raw `serde_json::Value`) fixed with `.into()`.

### Fixture regeneration

The only `.dsl.semio` assets under this plugin's artifact tree are `🗿️artifacts/🔌️wires/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` and the app-level `🎛️apps/🔌️wires/…/🎬️demo-session/🖼️assets/🗣️example.dsl.semio`. Both were, BEFORE this migration, generic placeholder stub text (`"semio demo.dsl v1\nid=demo\nbody=demo"`) — not genuine `WiresSnapshot` DSL documents under either the old or new codec (confirmed: the old `WiresSnapshotDsl` grammar also required `wires=`/`board=` lines this stub never had). The app-level one is only checked for non-emptiness (`primary_asset_is_nonempty`) — untouched. The artifact-level one is ALSO `parse_dsl`'d by `📚️examples/🎬️demo/🧪️tests/🦀️test.rs`'s `inference_determinism_law` — this failed with `"wires snapshot: unknown line \"semio demo.dsl v1\""` under my new codec (and would have failed identically under the OLD codec too, since the stub never matched either grammar; traced by `git log --date=iso` to `fd01661f06`, 2026-08-12 18:08:12 — within this ticket's own window, likely earlier plugin-taxonomy scaffolding, and the crate never compiled before today so this test never actually ran to completion pre-migration). Regenerated for real via the temporary-debug-test technique: a `#[cfg(test)] mod debug_fixture_regen` added to `📚️examples/🎬️demo/🧪️tests/🦀️test.rs`, gated on `DUMP_WIRES_DEMO=1`, built a one-node `WiresSnapshot` via `create_node` and printed real `print_dsl()` output (`cargo test … dump_demo_dsl_when_requested -- --nocapture`), captured and written as the new fixture. Temporary module removed cleanly afterward — verified `grep -rn debug_fixture_regen` returns nothing.

## Working-scene design

`WiresWorkingScene { nodes: Vec<DslValue>, edges: Vec<DslValue> }`, `thread_local!` `WIRES_SCRATCH: RefCell<HashMap<String, WiresWorkingScene>>` keyed by `WiresContentChild::child_id`. Every mutation-diff builder, codec decode (`parse_dsl`/`decode_pack`), and fixture converter mints+caches through `wires_content_child_handle_and_cache`. Read through the single accessors `wires_working_scene`/`wires_working_board`. Documented staleness gap: store-level undo/redo bypasses `ArtifactApp::handle`; fails soft (empty scene), never panics — same as every other W3/W4 exemplar, since no `LinkResolver` exists yet (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned).

## Converter (real, not a stub)

`wires_content_snapshot_from_scene`/`scene_from_wires_content_snapshot` — see "Composed-child bridge" above. Round-trip-tested (`node_edge_content_round_trips_through_the_composed_child_snapshot`, exercising `radius`/`root`/`edgeKind` specifically — fields the neutral subset has no native slot for — passing).

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly, matching every prior exemplar's finding. Out of scope for a plugin-scoped agent.

## Verification (actual, run in the foreground)

Baseline (before any edit):
```
cargo check -p semio-s-plugin-reasoning-mindmap --all-targets
```
**22 errors (lib) / 24 errors (lib-test)** — see Baseline section above for the exact breakdown.

After migration:
```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-reasoning-mindmap --all-targets
```
**0 errors**, confirmed on two independent consecutive runs (warnings only — unused imports/qualifications, an `E0365`/`pub_use_of_private_extern_crate` future-incompat warning, an ambiguous-glob-imports `testkit` warning — all pre-existing, confirmed present in the baseline's own warning output before my edits; none introduced by this pass beyond the expected cosmetic ones from the region churn).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-reasoning-mindmap --no-fail-fast
```
Run 1 (mid-pass, before the key-order codec fix): **73 passed, 6 failed** — traced all 6 to the `serde_json::Value` key-order bug (see "What changed" above), fixed.
Run 2 (after the fix): **78 passed, 1 failed**.
Run 3 (immediately after, no further edits): **78 passed, 1 failed** — identical named failure both times, not flaky.

## The 1 remaining failure — independently traced, NOT introduced by this migration

`artifacts::wires::standards::v1::subsets::any::schema::mutations::component::tests::set_node_root_round_trip`.

**Root cause**: `create_node`'s test fixture (`node("node-1", "Alpha")`) never includes a `"root"` key. `set-node-root`'s `🚩set-node-root/↩️inverse` restores the OLD root value via `node.get("root").and_then(as_bool).unwrap_or(false)` — when the key was never present, this returns `false` and the inverse calls `set_node_root(id, false)`, which (via `set_node_field`) WRITES an explicit `"root": false` key. The restored node therefore has an EXTRA key (`"root": false`) the original never had — `set_node_root`'s payload type (`{ node_id: String, new_root: bool }`, a plain bool setter) has no way to express "remove the key" versus "set it to false", so the inverse can only approximate. This is a structural limitation of the mutation's own payload shape, not a composition-migration concern.

**Not introduced by this migration**: the SAME `DslValue::Object` structural equality (`Vec`-backed, order- and presence-sensitive `PartialEq`) applied identically BEFORE this migration, when `WiresSnapshot::PartialEq` compared `board_fixture: DslValue` directly field-by-field — an extra `"root": false` key would have failed that comparison exactly the same way. My migration only changed WHERE the inequality surfaces (via `WiresContentChild`'s content-addressed hash instead of a direct field compare); it did not change the underlying node-JSON construction logic in `set_node_field`/the inverse at all.

**Traced by commit, not by message-parsing**: `git log -1 --date=iso --format="%H %ad %s" -- .../🚩set-node-root/↩️inverse/🦀️component.rs` → commit `880c37b4be589c6952e7067871246fa837bb1da3`, `%ad` = **2026-08-13 01:03:02 +0200** — the SMO mutation-vocabulary authoring pass that introduced all 10 triads, landed hours before this migration touched the file today (well within this ticket's own window per `📌️important.md`'s auto-commit-date warning, but authored by a different, earlier wave — the inverse logic itself is unchanged, carried forward verbatim by this pass).

Not trivial to fix in-scope: a real fix needs `SetNodeRoot`'s payload to become `Option<bool>` (tri-state: unset/false/true) or a dedicated `unset-node-root` triad — a mutation-semantics change, not a composition-migration task, matching `dag`'s own precedent (`delete-node`'s append-only-inverse gap, left documented rather than silently patched around).

## sharedFileRequests

None. No `🗄️stdio/**` file was read-written — only read for reference (`SemioGraphSnapshot`/`SemioGraphNode`/`SemioGraphEdge`/`SemioGraphPort`/`GraphNodeId`/`GraphEdgeId`/`STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA` schema at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/🦀️component.rs`). No `📦️glue.rs`/`📦️index.ts` edit was needed — every module this migration touches already existed as a glue-mounted sibling; only file CONTENTS changed, not the mount tree. Searched for a framework/plugin fixture-coupling issue analogous to `dag`'s (a separate framework-owned parser reading this plugin's own fixture text) — found none (`grep -rln "include_str.*reasoning\|include_str.*wires" 🧰️framework` returns only unrelated generated icon-asset files).

## Concurrent-churn observations

The repo's auto-committer advanced from `🚩️501` (session start) to `🚩️503` mid-pass, sweeping my in-progress edits into commits `3550b3dc09`/`515271bf60` as it went (confirmed via `git status --porcelain` showing most touched files already staged `M ` rather than unstaged ` M` by the time of the final `git diff HEAD --stat`) — matches `📌️important.md`'s documented behavior exactly; no work was lost, nothing needed recovering, and no git-modifying command was run. `git status --porcelain -- ✏️s/🔌️plugins/💡️reasoning` was re-checked clean of any OTHER session's live edits both before starting (per the orchestrator's pre-check) and again just before writing this report (`git diff --stat -- ✏️s/🔌️plugins/💡️reasoning` was empty at both those checkpoints) — no `remodel`/`trinity`-style concurrent-edit collision found in this plugin's subtree.

## Files touched this pass

33 files changed (654 insertions, 431 deletions) under `✏️s/🔌️plugins/💡️reasoning/**`, `git diff HEAD --stat`:
- `🗿️artifacts/🔌️wires/🦀️component.rs` — new `🔖️ContentBridge`/`🔖️WorkingScene` regions (converter, content-addressed handle, thread-local cache, `wires_working_board` accessor), deleted the dead 13-struct `🔖️Dsl`/`🔖️DslMirror` region, `empty_camera`, round-trip + determinism tests, updated existing tests.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — full rewrite: `WiresSnapshot` field swap, hand-rolled codec (fixed mid-pass for the key-order bug), tests.
- `…/📸️snapshot/📝️text/🦀️component.rs` — module doc, 2 test fixes.
- `…/🧬️schema/🦀️component.rs` — `WiresArtifact` field swap + `Default`/conversions; `ExampleFixture` region (`metabolism_wires_example_snapshot`, `handcrafted_metabolism_snapshot`'s sync block) rewired.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `WiresDiff.content`/`.camera`/`.meta`.
- `…/🔺️diff/📝️text/🦀️component.rs` — `apply`/`apply_to_artifact`/`absorb`, `diff_board_fixture`/`diff_wires_and_board`/`board_after_*`/`fixtures_after_*` rewired onto `wires_working_board`, 2 tests fixed.
- `…/🧬️mutations/🦀️component.rs` — E0252 fix (`super::` qualification), 12 test call sites fixed (`wires_working_board`/owned-`find_board_node` adjustments).
- `…/🧬️mutations/{🧭move-node,📐resize-node,🏷️change-node-kind,🔷change-node-shape,✏️edit-node-text,🚩set-node-root}/🔺️diff/🦀️component.rs` — `base.board_fixture.clone()` → `wires_working_board(base)`.
- `…/🧬️mutations/{🏷️change-node-kind,🔷change-node-shape,✏️edit-node-text}/↩️inverse/🦀️component.rs` — E0515 fix (terminate `.and_then` chains to owned values inside the closure).
- `…/🧬️mutations/🧭move-node/↩️inverse/🦀️component.rs` — `node_position(node)` → `node_position(&node)`.
- `…/🧬️mutations/💾️binary/🦀️component.rs` — 1 test fixed.
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `find_board_node`/`find_board_edge` return type `Option<&'a DslValue>` → owned `Option<DslValue>`, `infer()` rewired, test fixture rewired.
- `…/🚪️io/📤️export/🧵️serializers/…/csv/…` and `…/🚪️io/📥️import/🧩️deserializers/…/csv/…` — baseline `CsvSnapshot` field-rename fix (honest no-op).
- `…/🚪️io/📤️export/🧵️serializers/…/json/…` — baseline `JsonValue`/`serde_json::Value` fix.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (real `WiresSnapshot` DSL text, was a generic placeholder stub before).
- `📦️packages/🦀️rust/Cargo.toml` — added `semio-framework` dependency.
- `🎛️apps/🔌️wires/🦀️component.rs` — canvas-render call site, 5 tests (2 old-mutation-vocabulary, 3 accessor rewires).
- `🎛️apps/🔌️wires/🎮️commands/{🔵️node,🔗️relationship,🗑️delete,🖱️pointer,🔄️layout,🧬️example}/🦀️component.rs` — accessor rewires + old-mutation-vocabulary fixes (`AddNode`/`RemoveNode`/`PatchNode`/`AddRelationship`/`RemoveEdge` → `create_node`/`delete_node`/`move_node`/`connect_nodes`/`disconnect_nodes`).
- `🎛️apps/🔌️wires/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — accessor rewires.

ucas-status: complete
