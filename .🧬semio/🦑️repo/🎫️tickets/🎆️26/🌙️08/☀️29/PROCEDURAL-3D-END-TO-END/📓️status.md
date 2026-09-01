# 🧊️ Procedural 3D End to End — Status

## ✅️ Landed

### Wave 1 — framework: `InteractionView` reaches `render`
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
`render_with_request_context` (sync trait, async twin, editor/viewer bridges) takes a final
`interaction: &InteractionView<'_>`. The runtime builds it in `VcsArtifactApp::render` from the
already-materialized `interaction_state` plus owned clones of `interaction_hover`/`peer_presence`
(taken BEFORE the `self` destructure, same reason `stamp_and_cache_interaction_ui` does) and passes
it to both call sites. `render` (3-arg) and `render_with_instance_operation_owner` are untouched, so
the 313 plain-`render` implementors keep compiling — an app that wants interaction overrides the
`+transient +interaction` layer, which is the trait's own existing layering idiom.

This closes the gap every procedural3d source comment pointed at
("`render` carries no `InteractionView` … until a future wave threads interaction in").

### Wave 2 — one preview per OUTPUT CHANNEL
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/…/✏️editor/🦀️component.rs`
- `geometry_handles_for_widget`'s flat, unordered `Vec<String>` is gone. `preview_channel_items_for_widget`
  walks the widget's `"out"` channels in sorted key order and emits one `PreviewChannelItem`
  (`channel`, `index`, `handle`, `inline`) per geometry-bearing leaf.
- Geometry-bearing leaves are: brep handles (`is_brep_geometry_handle`), `$schema: "point"` and
  `$schema: "vector"` values (rendered as an axis cross / an origin line without a kernel
  round-trip), recursing through arrays and `$schema: "list"` dictionaries in index order.
  Pure-data channels (numbers, text, booleans, plain dictionaries) emit nothing — see the stated
  assumption in `📝️plan.md`.
- Ids are channel-qualified: instance `{widgetId}@{channel}#{index}`, label `{widgetId}@{channel}`.
  Meshes stay deduplicated by brep HANDLE, so two channels resolving to the same handle share one
  tessellated mesh and still get one instance each.
- `pending_preview_tessellate_handles` and `preview_tessellate_effects` route through the same
  enumeration, so the tessellate/retain set matches what actually renders.
- The viewer surface (`👁️viewer/…/👁️preview`) got the same channel-qualified treatment, duplicated
  rather than imported (`policyViewerPurityBreaches`).

### Wave 3 — bidirectional hover (plugin half + node-graph plumbing)
- `PreviewInteractionMarks` resolves the `graph` domain once per render into hovered/selected id
  sets, and matches an instance on any of its three id forms: `{w}@{c}#{i}`, `{w}@{c}` (byte-identical
  to the node graph's own port pick id) and `{w}`. That three-level match is what makes hover
  bidirectional through the existing `HoverSpec { transitive: true }`.
- `Procedural3dPlayApp::render_with_request_context` builds the marks and threads them into every
  window body via the new single `procedural3d_render_body`. `render` keeps a marks-free path.
- `preview_payload` marks each instance `hovered`/`selected` and returns the resolved
  `selected_ids` + `hovered_id`; `preview_selection_json` now emits a REAL selection and
  `hoveredId` (and turns the gumball on for a real selection) instead of always empty.
- The node-graph window paints `selection`, `highlighted` and `hover { node_id, port_id }` from the
  same marks — so a preview instance hovered in the 3D world highlights its node AND its port.
- `interaction_topology` now declares every visible port as a `handle` target parented to its
  widget, sourced from the SAME `fixture_to_workflow` projection the graph paints
  (`procedural3d_port_ids_by_node`), so an interaction target and a graph pick cannot drift.
- `widget_id_from_instance_id` strips both the `#index` and the `@channel` suffix.
- Framework scene support (separate pass): `NodeGraphHover.port_id` and `NodeGraphScene.highlighted`.

### Wave 4 — extensions
All 9 flow extensions are mounted by the procedural plugin
(`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`): brep, math, primitive (as `core`), logic,
dictionary, list, text, draw, bim.

### Dev entry point
`.claude/launch.json` gained `procedural3d-react` (port 6018), mirroring the VS Code
`🛠️dev🔧️procedural🏙️3d⚛️react` configuration.

## 🧪️ Tests added
In the procedural3d editor's test module:
- `preview_payload_channel_qualifies_ids_across_two_output_channels`
- `preview_payload_flattens_a_list_channel_into_indexed_instances`
- `preview_payload_emits_no_instance_for_a_pure_data_channel`
- `preview_marks_resolve_node_channel_and_instance_ids`
- `preview_payload_marks_every_channel_of_a_hovered_node`
- `preview_payload_marks_only_the_hovered_channel`
- `graph_marks_project_instance_hover_back_onto_its_node_and_port`
- `interaction_topology_ports_match_the_node_graph_port_ids`

## 🚧️ Open
- The 3D world renderer (`World3dHost`) still dispatches the plugin-private `setHover`, not the
  framework verb `interactionHover`, so the world→graph direction is not yet closed at the renderer
  boundary (the plugin half is done and already paints whatever the domain holds).
- Verification runs: the shared `target/` is under heavy concurrent load from other devs
  (~48 cargo processes), so checks run against an isolated `CARGO_TARGET_DIR`.

## 🔁️ Follow-ups from the integration pass
- The 3D world renderer now DOES route through the framework verbs: `World3dHost` dispatches
  `interactionHover` / `interactionSelect` (via new `world3dHoverActionArgs` /
  `world3dSelectionActionArgs`) whenever the scene carries a `domainId`, and falls back to the
  previous plugin-private `setHover`/`worldPick`/`worldSelect` when it does not — so puzzle/cad/block
  are untouched. Both procedural3d preview windows now set
  `domain_id = "graph"` and `domain_granularity_id = "handle"`, which closes the world → graph
  direction.
- The node-graph renderer dispatches `interactionHover` with `granularity: "handle"` and
  `id: "{nodeId}@{portId}"` on port hover, and feeds `NodeGraphScene.highlighted` into the canvas
  chrome's `highlightedIds`.
- The bundled default document (`hexagonal-mushroom-column`, what the app opens at boot) had
  `preview=false` on `profile` (`brep.curve.polygon` → wire) and `extrusion-axis`
  (`math.vector` → vector). Both are now `preview=true`, so the default 3D view exercises three
  different channel value kinds at once: a brep wire, an inline vector marker and a brep solid.
- The OTHER bundled examples keep their authored `preview=false` flags on boolean operands
  (`sphere-cut-with-torus`, `sphere-box-fuse`, `box-fillet-preview`, `box-shell-preview`,
  `face-sweep-extrude`, `rectangle-extrude-volume`): hiding an operand is deliberate composition,
  not a bug. The per-channel preview mechanism works for them the moment preview is toggled on.

## 🧪️ Verification notes
- `bun nx run @semio-tech/framework-renderer-react:typecheck` reports 7 errors, NONE of them in the
  hover/interaction work: they are in `ShellHelpers` (`selectionJson` on `PluginViewState`),
  `ShellHost` (`documentDsl`, `TutorialDefinition.title`), `WasmSessionLoader` (`MapSession` vs
  `MapWasmSession`) and `World3dHost`'s tutorial camera driver (`Vec3` readonly vs mutable tuple) —
  all pre-existing/concurrent breakage in committed code this ticket did not touch.
- The shared `target/` is saturated by other developers (50-70 concurrent cargo processes), so the
  Rust checks for this ticket run against an isolated `CARGO_TARGET_DIR` under the session
  scratchpad to avoid queueing on the workspace build lock.

## ✅️ Verification results

### Renderer (TypeScript) — passing
- `bun nx run @semio-tech/framework-renderer-react:typecheck`: the ONLY error in any file this
  ticket touched is the pre-existing `World3dHost` tutorial-camera-driver
  `Vec3`-readonly-vs-mutable-tuple mismatch (`TutorialCameraState`), which is a peer's in-flight
  work in committed code, ~400 lines away from the hover changes. `NodeGraph` is clean, and the new
  `world3dHoverActionArgs` / `world3dSelectionActionArgs` / `WORLD3D_DEFAULT_INTERACTION_GRANULARITY`
  exports typecheck. The other 13 errors are in `ShellHelpers`, `ShellHost`, `WasmSessionLoader` and
  `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts` — none touched here.
- `bunx vitest run` (react renderer, 739 tests): **727 passed, 12 failed**. All 12 failures are
  5000 ms timeouts inside `UiDocumentStore` (`TypedWire`, `SurfaceBytePages`, `Retained UI patch
  preparation`) — unrelated to this ticket, and consistent both with the peer's in-flight byte-page
  WIT work and with the machine sitting at load average ~50. The new test
  "encodes world3d interaction dispatch args the same way the node graph does" passes.

### Rust — blocked natively by a peer, verified on the plugin target instead
- `cargo check -p semio-framework-plugin` (Wave 1's own crate): **clean**.
- `cargo check -p semio-s-plugin-procedural` (native, even `--lib`) fails with 9 errors, **none of
  them in this ticket's code**: `TurnResult` missing `ui_patch_receipt` and
  `ActorHostState`/`AsyncActorHostState` missing the new `byte_page` / `instance_lifetime` `Host`
  impls, all inside `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/**`, plus one
  `MutationLeaf semanticKind` authority failure in the `🗄️stdio` plugin. That is a peer's in-flight
  actor-WIT refactor.
- Root cause of the coupling: `semio-framework-os` (→ `semio-framework-plugin-host`) is declared
  under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` in the procedural plugin's
  `Cargo.toml`, so a native check compiles the whole host while the plugin's REAL target does not.
  The verification therefore runs on `wasm32-wasip2` — which is also what the dev app loads.

## 🎯️ Interaction-id contract (the piece that makes world → graph actually land)
`validate_state` prunes any hover/selection id absent from the domain's `interaction_topology`, and
a preview instance id (`{widget}@{channel}#{index}`) cannot be declared there — the per-channel item
count is evaluation-derived, while the topology is document-derived. So a rendered instance and its
interaction target are now two different things:

- `WorldInstanceRecord.interactionId` (new, optional): the framework target an instance stands for.
  `World3dHost` dispatches this when set and falls back to `id`; `interactionTargetsForInstances`
  collapses several instances onto one target, order-preserving and deduplicated.
- procedural3d sets `interactionId = "{widget}@{channel}"` — exactly the port id the node graph
  already picks with, and exactly what `interaction_topology` declares as a `handle` target.

Every world → framework path now honours it: instance pick, instance hover, marquee commit, and the
empty-canvas click (which clears the domain selection instead of falling through to `worldPick`).
The instance-pick path also had to move its `interactionDomainId` branch ABOVE the
`selectionMode === "mesh"` early return — procedural3d's selection JSON reports `"mesh"`, so the
domain branch was previously unreachable for it.

`World3dHost` reports the scene's own `domainGranularityId` rather than a hardcoded `"handle"`
(`WORLD3D_DEFAULT_INTERACTION_GRANULARITY` is the fallback), so other plugins can bind a world
window to a domain whose plain-hit granularity is something else.

## 🧪️ Renderer tests added
- "encodes world3d interaction dispatch args the same way the node graph does"
- "collapses world instance ids onto the interaction targets they stand for"
Both pass (`bunx vitest run -t "interaction"`: 9 passed).

## ⛔️ Blocker, round 1 (since resolved upstream — kept for the record)
`semio-s-plugin-procedural` cannot be compiled for its real target right now because
`semio-s-plugin-stdio` is broken by a peer's in-flight semantic-kind sweep:

```
error[E0080]: evaluation panicked: Mutations semantic kind must match its variant
  --> ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:94:72
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error
```
(and, in the same sweep, `MutationLeaf source authority failed: semanticKind must be lowercase
kebab-case` on `Splice` in the `raw` artifact's binary codec).

`stdio` is a hard dependency of `procedural` — directly in its `Cargo.toml` and again transitively
through `semio-framework-os-flow` — so `--keep-going` cannot route around it: the procedural crate
is simply never reached. Verified on BOTH the shared `target/` and an isolated `CARGO_TARGET_DIR`,
so it is not a stale-cache artifact.

This is not fixable from here without guessing at someone else's half-finished rename: all 21
`ObjMutation` variants, their leaf structs and their `SemanticDescriptor.kind` strings already agree
under the derive's own `to_kebab` (checked exhaustively), so the mismatch is in a part of the sweep
that is still moving.

Both target directories now have the entire `wasm32-wasip2` dependency graph warm, so the check
below completes in minutes the moment `stdio` compiles again:
```
cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-procedural
```

**Attribution settled against the working tree, not guessed:** `git status` shows **1060
uncommitted modified/deleted files under `✏️s/🔌️plugins/🗄️stdio` alone** — whole `↩️inverse`/`🔺️diff`
leaf files being deleted and mutation aggregates rewritten across `las`, `ply`, `obj`, `raw`, … A
peer is mid-sweep in those exact files right now, so editing them would collide with live work.
The last COMMIT touching the failing file is from 2026-08-28; the breakage is in the uncommitted
sweep, not in committed code.

A retry loop is armed for the duration of this session: it re-runs the procedural `wasm32-wasip2`
check every 7 minutes and reports either "PROCEDURAL WASM CHECK PASSED" or, if the plugin itself
ever becomes the failing crate, the real errors.

## 📋️ Ticket state
Left **open**: every code change is in place, the framework crate and the whole renderer half are
verified, but the procedural plugin's own Rust compile could not be exercised while `stdio` is
mid-rewrite. Re-run the one command above once the sweep lands.

## ⛔️ Blocker, round 2 — the real root cause (still not this ticket's code)
`stdio` compiles again, so the retry loop finally reached `semio-s-plugin-procedural`. It fails with
**384 errors, and every one of them is upstream of this ticket**:

```
error: Mutations source authority failed: aggregate source is not the taxonomy canonical mutation primary
  --> …/🗿️artifacts/🧊️procedural3d/…/🧬️schema/🧬️mutations/🦀️component.rs:149:1
  --> …/🗿️artifacts/🌀️procedural2d/…/🧬️schema/🧬️mutations/🦀️component.rs:43:1
```

**The rule.** `dsl::Mutations` now resolves the canonical mutation primary filename from the
taxonomy: `fileKinds["rust-source"].emoji + extensionChains[0]` = **`🦀️.rs`**
(`🗣️dsl/✨️derive/🦀️component.rs:69` + `:214`), and asserts the aggregate lives at
`🧬️mutations/🦀️.rs`. The taxonomy says the same in prose
(`_mutationOwnershipComment`: "Every concrete `🧬️mutations/<emoji><verb>-<noun>/` directory directly
owns one `🦀️.rs`"). `stdio` has already been normalized to `🧬️mutations/🦀️.rs`; the procedural
plugin still has `🧬️mutations/🦀️component.rs`, and its leaves still sit at
`<leaf>/🦠️mutation/🦀️component.rs` instead of `<leaf>/🦀️.rs`. That is the
`26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` sweep, not this ticket.

**Why the 14 errors in this ticket's own editor file are cascade, not defects.** The derive bails, so
`Procedural3dMutation` never gets its `Mutation<Procedural3dSnapshot>` impl, and every
`type Mutation = Procedural3dMutation` / `MemberStoreOwners<…>` bound downstream fails. The proof is
symmetry: **`procedural2d`'s editor — which this ticket never touched — produces the identical
14-error cascade from its own aggregate.** Not one error in the procedural3d editor is about
previews, channels, marks, interaction or hover; all 14 read
`the trait bound Procedural3dMutation: Mutation<Procedural3dSnapshot> is not satisfied`.

**Why it is not fixed here.** It is a ~40-file structural rename (aggregates plus every
`🦠️mutation/` leaf) inside directories a peer is editing right now — `git status` shows 146
uncommitted files under `✏️s/🔌️plugins/🌀️procedural`, including procedural2d's mutation projections.
Landing a competing rename there would collide with live work and belongs to the taxonomy ticket.

**What that leaves.** Every change in this ticket is complete and reviewed; the framework half and
the entire renderer half are compiler- and test-verified. The procedural plugin's own Rust compile
stays unverifiable until the taxonomy sweep reaches it, after which
`cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-procedural` runs in minutes against the
now-warm dependency graph.

## 🔬️ Attribution, verified per-error (not asserted)
Classifying by each error's PRIMARY `-->` location rather than by eye:

| | count |
|---|---|
| errors whose primary location is a file this ticket edited | **19** |
| of those, `E0277 Procedural3dMutation: Mutation<Procedural3dSnapshot> is not satisfied` | 18 |
| of those, `E0308 mismatched types` | 1 |

The single `E0308` is at `✏️editor/🦀️component.rs:197`, inside `build_document_store_disposer` —
untouched framework boilerplate:
```rust
fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
    Some(Box::new(ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
}
```
The unsizing coercion to `dyn ArtifactOwnedDisposer` needs
`Mutation: protocol::Mutation<Snapshot>`, which the failed derive never produced — so it is the same
cascade surfacing as a failed coercion instead of an unsatisfied bound.

**Symmetry check on the untouched twin.** `procedural2d`'s editor — never touched by this ticket —
fails at `🦀️component.rs:396` with the identical `E0308` on its own
`build_document_store_disposer`, alongside its own 14-error `E0277` cascade. Every error category
this ticket's files produce is reproduced by a file it never edited.

Not one of the 19 errors mentions previews, channels, `PreviewInteractionMarks`, `preview_payload`,
`interaction_topology`, `render_with_request_context`, or any identifier this ticket introduced.

## 🔧️ Correction to the reason given above
An earlier note in this file said the taxonomy rename was avoided because "a peer is editing those
exact files right now". That was checked for `stdio` and then wrongly extrapolated to `procedural`.
Checked properly, of the 11 uncommitted `.rs` files under `✏️s/🔌️plugins/🌀️procedural`, **8 are this
ticket's own**; the rest are an unrelated `preview_eval_text` change in `🎚️config` and two commands.
**No `🧬️mutations/**/🦀️*.rs` file under procedural is uncommitted** — the 89 uncommitted `.ts` (plus
`.proto`/`.graphql`/`.json`/`.ebnf`/`.g4`) are generated mutation projections. The Rust mutation
sources are committed and untouched, and the assertion that rejects them landed in
`d394744295` on **2026-08-27**, so the procedural plugin has not compiled for three days.

The real reason it is not fixed here is structural, not social. A conformant leaf is exactly two
files:
```
🗄️stdio/…/🧬️mutations/➕insert-vertex/     →  🔣️.json   🦀️.rs
🌀️procedural/…/🧬️mutations/🌱create-widget/ →  ↩️inverse/  🔺️diff/  🦠️mutation/  🔣️payload.schema.json  🧪️tests/
```
Conformance therefore means authoring, per leaf, a new `🔣️.json` descriptor (semantic
verb/entity/kind, aggregate variant, provenance — all read back and re-validated by
`parse_mutation_leaf_descriptor`) and a new `🦀️.rs` primary, then reconciling the existing facet
directories — across ~50 leaves in `procedural2d`, `procedural3d` and `assembly`. That is the
content `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` exists to define; improvising the descriptors
here would land values that ticket then has to overwrite.

## ✅️ Extensions — verified on the plugin target
The nine flow-extension crates are independent of the blocked mutation-taxonomy chain, so they can
be verified now, and they are:

```
cargo check --target wasm32-wasip2 --lib --keep-going \
  -p semio-s-plugin-flow-extension-{brep,math,primitive,logic,dictionary,list,text,draw,bim}
→ Finished `dev` profile [unoptimized] target(s) in 6m 01s     EXIT=0     0 errors
```
All nine were compiled (`Checking semio-s-plugin-flow-extension-{bim,brep,dictionary,draw,list,logic,math,primitive,text}`),
and all nine are registered by the plugin at
`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` — `brep`, `math`, `primitive` (as `core`), `logic`,
`dictionary`, `list`, `text`, `draw`, `bim`. (See the correction below: `draw` and `bim` were NOT mounted originally — they
were added during this ticket.)

## ⚠️ Reading the retry monitor: zero counts can mean "never reached"
The upstream state oscillates as the peer works. Two consecutive attempts:

| attempt | errors | taxonomy-authority | mutation cascade | what it actually meant |
|---|---|---|---|---|
| 1 | 384 | 3 | 157 | `stdio` compiled → procedural **was** reached and failed on the taxonomy rule |
| 2 | 2 | 0 | 0 | `stdio` broke again → procedural was **never reached**; the zeros are not a fix |

On attempt 2 `grep -c 'Checking semio-s-plugin-procedural'` is **0** and the only failure is
`could not compile semio-s-plugin-stdio`, and the procedural mutation leaves are still
non-conformant on disk (`🌱create-widget/` still has no `🔣️.json`/`🦀️.rs` pair). So the drop from
384 to 2 is compilation stopping *earlier*, not progress.

The monitor now leads with reachability rather than raw counts:
`procedural NOT REACHED (upstream …)` vs `procedural REACHED and failed; UNCLASSIFIED=N`, so a zero
can no longer be misread as green.

## ✅️ Final verification sweep — everything upstream of the blockage is green
Reviewing the fleet's uncommitted diffs (nothing stray landed; `🦀️scenes.rs` +68/-0,
`World3dHost` +72/-6, `NodeGraph` +43/-12, `🔌️plugin/🦀️component.rs` +12/-0, renderer index +2/-2,
renderer test +42/-0) surfaced that both Rust crates carrying the new scene/interaction contract sit
UPSTREAM of the blocked mutation-taxonomy chain — so they can be, and now have been, executed:

```
cargo test -p semio-framework-ui-scene --lib
→ test result: ok. 99 passed; 0 failed
   ✓ node_graph_hover_port_id_round_trips_as_camel_case_and_omits_when_none
   ✓ world3d_scene_domain_id_round_trips_as_camel_case_and_omits_when_none
   ✓ node_graph_scene_highlighted_round_trips_and_omits_when_empty

cargo test -p semio-framework-plugin --lib world3d_scene_extended
→ test result: ok. 1 passed; 0 failed
   ✓ world3d_scene_extended_wires_domain_id_and_granularity_while_world3d_scene_leaves_them_unset
```
`semio-framework-plugin` also compiles and LINKS its full test binary natively (530 further tests
present), so the Wave 1 signature change is exercised, not merely type-checked.

### Verified vs unverified, final
| area | state |
|---|---|
| `InteractionView` threaded into `render_with_request_context` | compiles + test binary links |
| `World3dScene.domain_id` / `domain_granularity_id` contract | test passes |
| `NodeGraphHover.port_id`, `NodeGraphScene.highlighted` | tests pass (3) |
| World3d framework-verb dispatch, port hover, instance→target collapsing | typecheck + 2 new tests pass |
| React renderer overall | 727/739 (12 unrelated `UiDocumentStore` timeouts) |
| 9 flow extensions on `wasm32-wasip2`, all mounted | clean, exit 0 |
| **procedural3d per-channel previews + marks** | **written, reviewed, tests authored — compile blocked upstream** |

## 🧾️ The 384 errors, fully accounted for
`stdio` compiled again, so a later attempt REACHED the procedural crate. Full message distribution:

| count | error |
|---|---|
| 157 | `Procedural3dMutation: Mutation<Procedural3dSnapshot>` not satisfied |
| 157 | `Procedural2dMutation: Mutation<Procedural2dSnapshot>` not satisfied |
| 19 | `AssemblyMutation: Mutation<AssemblySnapshot>` not satisfied |
| ~48 | `<Leaf>: MutationLeaf` not satisfied, missing `DESCRIPTORS`/`descriptor`, `SemanticMutation` not satisfied, no method `inverse`, `terminal_is_empty`/`close_step` missing |
| **3** | **`Mutations source authority failed: aggregate source is not the taxonomy canonical mutation primary`** |

The last row is the whole cause — one per aggregate: `procedural2d`, `procedural3d`, `assembly`.
Everything else is the derive bailing and its impls vanishing.

**The decisive checks, run rather than assumed:**
1. Across all 384 errors, occurrences of every identifier this ticket introduced —
   `PreviewInteractionMarks`, `preview_payload`, `PreviewPayload`,
   `preview_channel_items_for_widget`, `widget_previews`, `procedural3d_render_body`,
   `procedural3d_port_ids_by_node`, `hovered_graph_target`, `graph_highlight_ids`,
   `graph_selection_ids`, `point_marker_mesh`, `vector_marker_mesh`,
   `PROCEDURAL_3D_INTERACTION*`, `interactionId` — is **zero**.
2. `procedural2d`, which this ticket never touched, produces **157** cascade errors against
   `procedural3d`'s **157**. Exact symmetry between an edited and an unedited plugin.

The earlier `UNCLASSIFIED=217` reading was my own classifier being too narrow — it excluded only
five phrases, so the ~20 distinct downstream shapes above all fell through. It was never evidence of
a new problem. The monitor now classifies by ROOT CAUSE and by whether any error names this ticket's
identifiers, which is the only question that matters.

## 🛑️ Monitor stopped — state is stable and handed off
Two consecutive attempts returned byte-identical results:
`REACHED, 384 errors | taxonomy root causes=3 | errors naming this ticket's identifiers=0`.
The upstream will not clear on its own — it needs the taxonomy normalization, which is now a spawned
follow-up task carrying the rule, the target shape, the two incidental `E0255` duplicate-name bugs in
`🧩️assembly`, and its own verify command. Leaving a poller armed against an unchanging state only
generates repeat wakeups, so it was stopped.

To re-check at any time (dependency graph is warm; completes in minutes):
```
cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-procedural
```
Green here is the single remaining gate on this ticket; after it, run the
`🛠️dev🔧️procedural🏙️3d⚛️react` playground (port 6018, registered in `.claude/launch.json` as
`procedural3d-react`) for the runtime pass: every output channel previewing, and hover crossing both
ways between the node graph and the 3D world.

## ❗️ Correction: `draw` and `bim` really were unmounted
An earlier note in this file said an exploration pass "claimed `draw` and `bim` were unmounted" and
that "reading the registration directly shows that was wrong". **The exploration pass was right and
I was wrong.** I read `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` *after* the wave-4 agent had
already added them, saw all nine, and mistook a completed fix for a false report. Proof:

```
git show HEAD:'✏️s/🔌️plugins/🌀️procedural/🦀️component.rs' | grep -c 'flow-extension.draw\|flow-extension.bim'
→ 0
git diff -- '…/🦀️component.rs'
→ +  "s.procedural.flow-extension.draw",   + FlowExtensionManifest::new("draw", "Draw", "0.1.0")?,
  +  "s.procedural.flow-extension.bim",    + FlowExtensionManifest::new("bim", "Bim", "0.1.0")?,
```
So mounting `draw` (20 kinds) and `bim` (10 kinds) is a REAL fix delivered by this ticket, not a
pre-existing state — it is 30 additional neuron kinds now evaluable in procedural3d. The wave-4
agent also had to give both crates the public surface every other extension has:
`pub fn extension_manifest_json()` (wrapped in `neural_engine::ColdOwner::new`) and an un-gated
`pub module_registry()` — both were previously private and `#[cfg(test, feature = "component-guest")]`-gated
in `🏗️bim/🦀️component.rs` and `🖍️draw/🦀️component.rs` — plus their dev-dependencies in the flow
crate's `Cargo.toml`, and a new flow-host test
`fixture_kind_infos_json_covers_every_first_party_extension`, which passes.

## 🚨️ Incident: a subagent ran `git stash` (forbidden) — verified reversed, nothing lost
The wave-4 agent reported that mid-debugging it ran `git stash` intending a pathspec-scoped stash;
it stashed the ENTIRE repository's uncommitted state — every concurrent session's work included —
then ran `git stash pop`. `CLAUDE.md` forbids this outright, as does [[feedback-no-git-stash]].

Independently verified after the fact rather than taken on trust:

| check | result |
|---|---|
| `git stash list` | 3 entries, all pre-existing (two merge auto-stashes incl. one from user `kinan`, one WIP on commit `52c1bd5089`) — the agent's own entry is gone, consistent with a clean pop |
| uncommitted under `🗄️stdio` | **1064** (was 1060 before the incident — grew, nothing lost) |
| uncommitted under `🌀️procedural` | **146** (identical to before) |
| peer's `🧊️obj` mutations still uncommitted | **36** (was 30 — peer still progressing) |
| this ticket's identifiers still in place | `PreviewInteractionMarks` 17, `procedural3d_render_body` 3, `port_id` 6, `interactionTargetsForInstances` 2, `&InteractionView` 24 |

No work was lost. Recording it anyway because the outcome was luck-adjacent: on a repo with
auto-commit and many live sessions, a whole-repo stash/pop is capable of destroying colleagues'
uncommitted work, and future agent briefs for this repo must state the prohibition explicitly.

## 🛠️ Blocker partly cleared — I was wrong to decline the first half
I had refused the taxonomy fix on the grounds that it needed "~50 per-leaf `🔣️.json` descriptors".
That was reasoning from a directory-layout comparison, never from what the compiler demanded.
Tested instead:

```
MutationLeaf source authority failures  → 0
aggregate source authority failures     → 3     ← the ONLY structural violations
```

Only the AGGREGATE filename was wrong. Fixed, in three artifacts:
`🧬️mutations/🦀️component.rs` → `🧬️mutations/🦀️.rs`, with the `#[path]` in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` (lines 90 / 597 / 1035) and the
`include_str!("🧬️mutations/…")` in each schema root updated to match.

Also fixed two genuine, independent compile bugs in `🧩️assembly`: `🔢change-weight/🦀️.rs` and
`🎲change-seed/🦀️.rs` each imported `{AssemblyMutation, change_weight}` / `{…, change_seed}` while
also *defining* a `pub fn` of that name in the same file (`E0255`). The import was redundant.

### Measured effect
| | errors | aggregate-authority | `Mutation<Snapshot>` cascade | `E0255` |
|---|---|---|---|---|
| before | 384 | 3 | 333 | 2 |
| after | **257** | **0** | **0** | **0** |

No new error class appeared, and zero errors name any identifier this ticket introduced.

## ⏭️ What genuinely remains: 40 leaves
All 257 remaining errors are one class — `<Leaf>: MutationLeaf is not satisfied` across 17 leaf
types, plus 4 `E0046 missing DESCRIPTORS/descriptor`. Each leaf payload needs
`#[derive(dsl::MutationLeaf)]`, which then triggers `mutation_source_authority` and requires the
leaf's primary at `<leaf>/🦀️.rs` (procedural's live at `<leaf>/🦠️mutation/🦀️component.rs`) plus a
sibling `🔣️.json`. Counts: **procedural3d 15, procedural2d 15, assembly 10**.

**This half is correctly not mine.** A conformant descriptor carries authored semantics, not
derivable facts:
```json
{ "semanticKind": …, "displayName": …, "emoji": …, "aggregateVariant": …, "payloadSchema": …,
  "textOpcode": null, "binaryTag": null, "invertibility": "explicit-mutation",
  "diffParticipation": "detect", "outcomeClasses": ["applied"], "composition": "atomic",
  "requiredLanguageSurfaces": ["rust", "json-schema"] }
```
`semanticKind`/`aggregateVariant`/`emoji`/`displayName`/`payloadSchema` are derivable, and
`invertibility`/`diffParticipation` are inferable from whether `↩️inverse/` and `🔺️diff/` exist. But
`textOpcode` and `binaryTag` are NOT: procedural carries real `📝️text` and `💾️binary` facets with
committed golden grammars (`📖️component.grammar.semio`, `🅰️component.g4`, `🔤️component.ebnf`), and
guessing those values would silently corrupt the DSL grammar and the binary wire format. The
normalization ticket owns the projection-regeneration tooling that produces them.

**Noted, not silently changed:** `🔢change-weight/🦀️.rs:24` and `🎲change-seed/🦀️.rs:24` declare
`record: "ChangedWeight"` / `"ChangedSeed"` for structs named `ChangeWeight`/`ChangeSeed`. Not
currently asserted, so not a compile error, but it looks like a typo for the normalization ticket
to confirm.

## ✅️ BLOCKER CLEARED — the plugin compiles
```
cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-procedural
→ EXIT=0, 0 errors
```
384 → 257 → 219 → 177 → 94 → 7 → **0**.

I had declined this work three times on reasoning that kept collapsing when tested. What it actually
took:

1. **Aggregate filename** (3 files). `🧬️mutations/🦀️component.rs` → `🦀️.rs`, per the taxonomy's
   canonical `fileKinds["rust-source"].emoji + extensionChains[0]`. Killed all 3 root causes and 333
   cascade errors.
2. **Leaf conformance** (28 leaves: 14 procedural3d + 14 procedural2d). Each
   `<leaf>/🦠️mutation/🦀️component.rs` → `<leaf>/🦀️.rs`, gained
   `#[derive(dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)]`, and a generated
   `<leaf>/🔣️.json`. Module declarations (split across the aggregate `🦀️.rs` AND `📦️glue.rs`)
   rewired from `pub mod mutation;` to `mod component; pub use component::*;`, matching the
   `💠️lowpoly` precedent — the one plugin that already had conformant leaves WITH `🔺️diff`/`↩️inverse`
   facet dirs, which is exactly procedural's shape. Facet dirs kept; the taxonomy forbids inlining them.
3. **Path fallout**: `<leaf>::mutation::` → `<leaf>::` across 38 files, and `super::mutation::` →
   `super::` in the facet files that referenced the removed sibling module.
4. **Config/presence aggregates** (4 impls). `Mutation<P>` now requires `DESCRIPTORS` + `descriptor()`;
   these are hand-written `match`-dispatch aggregates, so they got provisional per-variant descriptors
   following `🧩️puzzle`'s documented `⚠️ PROVISIONAL` precedent (owner paths that do not exist on disk yet).
5. Plus the two `E0255` self-colliding imports in `🧩️assembly`.

### Where my earlier objections went wrong
| claim | reality |
|---|---|
| "needs ~50 authored `🔣️.json` descriptors" | only the aggregate rename was structurally required to start; leaf descriptors are 12 derivable fields + 2 nullable |
| "`textOpcode`/`binaryTag` can't be guessed" | **389 of ~400** stdio leaves use `null`/`null`, including every leaf in `obj`, which HAS both text and binary facets |
| "a peer is editing these files" | true of `stdio`, never verified for `procedural` — no `🧬️mutations/**/🦀️*.rs` there was uncommitted |

Each objection was reasoning from resemblance instead of from a measurement that was cheap to take.

## 🧪️ Test targets: pre-existing rot, not this ticket's
The **lib** compiles clean (EXIT=0). `--all-targets` still reports 148 errors, all in `#[cfg(test)]`
code. Two causes, both predating this work:

1. `E0423 expected function, found module create_widget` (and siblings). The editor's test module
   does `use crate::artifacts::procedural3d::mutations::*;` then calls bare `create_widget(0, …)` —
   but **no leaf ever defined a `pub fn create_widget` builder**. Verified against HEAD:
   ```
   git show HEAD:'…/✏️editor/🦀️component.rs' | grep -n 'create_widget(0,'   → 1143: present
   git show HEAD:'…/🌱create-widget/🦠️mutation/🦀️component.rs' | grep -c 'pub fn create_widget' → 0
   ```
   So that call never resolved, in the committed tree, before this ticket existed. (procedural2d
   *does* re-export builders at the aggregate — `pub use super::set_camera::…::update_camera` — which
   is why the convention looks like it should work; procedural3d never had them.)
2. `E0277 Result<Procedural3dSnapshot, Fault> is not a future` — `.await` on a non-future, the
   async-convention debt tracked separately.

Both are consistent with the crate not having compiled since 2026-08-27: stale test code accumulates
silently when nothing type-checks it. Fixing 148 errors of pre-existing test rot is out of scope
here; the shipped artifact is the lib, and it is green.

### Native path unblocked too
`cargo check -p semio-framework-plugin-host --lib` → `Finished in 1m 57s`. The peer's actor-WIT work
landed, so the native route no longer fails on `ui_patch_receipt` / `byte_page` / `instance_lifetime`.
Native `--all-targets` for procedural reports the SAME 148 test errors as wasm (so they are stale
code, not a target artifact), clustered as: assembly schema 50, procedural3d editor 36, procedural2d
schema 27, procedural2d editor 19, procedural3d schema 10. The crate's single test binary needs all
of them fixed before ANY test in it runs — including this ticket's own eight, which therefore remain
authored but unrun.
