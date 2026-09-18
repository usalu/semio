# 🧊️ A5 — the `wfc3d` artifact (3d arbitrary slot graph)

Crate `semio-s-artifact-wfc-3d` at `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d`. Dialect `s.wfc.wfc3d`,
OS kind `3d.wfc3d`, apps `s.wfc.wfc3d@1/*#editor` / `…#viewer`, breadcrumb `["semio", "wfc", "3d"]`,
nx project `@semio-tech/wfc-3d-rs`.

> ⚠️ **Folder**: authored under `🌊️wfc` until 12:41, when the whole plugin tree was consolidated into
> `🀄️wfc`; the coordinator ruled that move deliberate (🌊️ collides with the sibling `🌊️flow` plugin).
> Every path in this report is under `🀄️wfc`, and every `🌊️wfc` string inside this artifact's own
> files was rewritten (the 15 mutation-descriptor `owner` fields were the only ones).

---

## 1. Verification — every command run, and what it said

All from `/Users/ueli/Documents/semio`, foreground, `-j 4`, logs under `🗑️generated/wfc3d/`.

| # | command | result | log |
|---|---|---|---|
| 1 | `cargo check -p semio-s-artifact-wfc-3d --lib -j 4` (schema half, no feature) | ✅ green | `check-lib-1.log` |
| 2 | `cargo check -p semio-s-artifact-wfc-3d --features component-app-assembly --lib -j 4` | ✅ green | `check-feature-5.log` |
| 3 | `cargo check … --features component-app-assembly --lib --tests -j 4` | ✅ green | `check-tests-4.log` |
| 4 | `cargo clippy -p semio-s-artifact-wfc-3d --features component-app-assembly --lib --tests -j 4` | ✅ green, **0 warnings in this crate** | `clippy-6.log` |
| 5 | `RUST_MIN_STACK=33554432 cargo test … --lib -j 4 -- --test-threads=4` | ✅ **229 passed, 0 failed, 2 ignored** in 0.06 s | `test-11.log` |
| 6 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check … --target wasm32-wasip2 -j 4` | ✅ green | `wasm-3.log` |
| 7 | `NX_DAEMON=false bun nx run @semio-tech/wfc-js:test` | ⚠️ **147/148**, the one failure is `◻️2d`'s (slice A3) `ships one fixture directory per declared kind`; **all 9 wfc3d-owned files green** | `ts-test-7.log` |
| 8 | `python3 $T/🐍️wfc3d-oracle-runner.py` | ✅ **15/15 vectors, 0 problems; mount contract ok** | `oracle-3.log` |

Rows 3–8 are the audit follow-up re-run (fuel/budget raise, the two docstring fixes, the strict
preview instance lane, the comment-convention clean-up). Row 7's single failure is NOT in this
artifact: `◻️2d`'s suite compares its fixture DIRECTORY names against its declared kinds and reads
`↔️move-slot` where it wants `move-slot`, which is slice A3's to fix.

Re-run after the 15:0x switch from deny-list to ALLOW-LIST rule semantics (coordinator ruling): the
three examples' rule sets were rewritten to state their admitted pairs explicitly, the
`🚦️create-rule` / `🚫️delete-rule` fixture cases were renamed
(`🚦️forbids-two-rooms-side-by-side`, `🚫️drops-the-room-corridor-pairing`), and every fixture and
example asset was regenerated. All three examples still solve, and the contradiction test still
contradicts.

The two ignored tests are the two `#[ignore]` GENERATORS (`regenerate_committed_fixture_quintets`,
`regenerate_committed_example_assets`); they author files, they do not assert. Re-run them with
`cargo test … --lib -- --ignored regenerate`.

⚠️ `bun nx run @semio-tech/wfc-js:test` fails under the nx DAEMON with
`The "@repo/emoji-project-json" plugin threw an error … Source file "✏️s/🔌️plugins/🀄️wfc/📦️packages/🦀️rust/Cargo.toml" does not exist`.
The file exists; the daemon's project graph is stale after the folder move. `NX_DAEMON=false` is
green. W2 should `bun nx reset` once before trusting any nx target.

---

## 2. Document model as authored

`🧬️schema/📸️snapshot/🦀️.rs`. Field order below IS the canonical JSON key order.

```rust
Wfc3dSnapshot { schema: String, seed: u64, slots: Vec<Slot3d>, edges: Vec<SlotEdge>, tiles: Vec<Tile>, rules: Vec<GraphRule> }

Slot3d  { id, x, y, z: f64, width, height, depth: f64, pinned_tile_id: Option<String> }
SlotEdge{ id, from_slot_id, to_slot_id, relation: String }
Tile    { id, label: Option<String>, weight: f64, media: TileMedia3d }
GraphRule { id, tile_a_id, tile_b_id, relation: Option<String>, allowed: bool }
Color   { r, g, b, a: u8 }
TileMedia3d = Mesh { positions: Vec<f64>, indices: Vec<u32>, color: Option<Color> }
            | MeshChild { child: store::ArtifactChild<SemioMeshSnapshot> }   // internally tagged on "kind"
```

Decisions a reader should not have to reverse-engineer:

- **`x`/`y`/`z` is the box's MINIMUM corner**, `width`/`height`/`depth` its extent. Tile media is
  authored in the unit box `0..1`, so a preview instance is `position = slot origin`,
  `scale = slot extent` — which is what lets `tower-stack`'s differently-sized storeys share one
  mesh and cost one draw call.
- **`relation` is a real constraint channel**, not a label. Every distinct edge-relation string
  becomes its own `RelationId`, so `above` and `beside` can admit different tile pairs over the same
  topology. `MAX_WFC3D_RELATIONS = 32`.
- **Rules ARE the compatibility table — an ALLOW-LIST over an UNORDERED tile pair.** They compile
  onto an EMPTY `ModelBuilder`: `allowed: true` pushes `allow`, `allowed: false` pushes `deny`, and
  deny always beats allow at compile time regardless of call order. A pair no rule mentions is
  FORBIDDEN, `relation: None` states the rule for every relation at once, and both directed orders
  are pushed because a rule is written for an unordered pair. An empty rule set is therefore
  unsatisfiable the moment two slots are adjacent. This is the same law grid2d/grid3d/wfc2d state
  (coordinator ruling, plan §7); wfc3d originally shipped a deny-list and was switched.
  *(Assembly, the ancestor, was internally inconsistent: its engine wiring read an allow-list while
  its own example-outcome checker read a deny-list.)*
- **Collections are kept in canonical ascending-id order.** `canonical_slot_index` /
  `canonical_edge_index` / `canonical_tile_index` / `canonical_rule_index` count strictly smaller
  ids; every editor `create-*` inserts there, so a delete followed by its inverse restores the row's
  *position*, not just its content.
- **Tile weights live on the tile**, not in a side table (assembly's `weights` collection is gone);
  `weight > 0.0` is a fatal invariant on `create-tile` and `change-tile-weight`.
- `TileMedia3d::MeshChild` is a real composed-child handle (`crate::mesh_child_handle`,
  `child_id == target.artifact_id == tile id`). The preview resolves it through
  `local_owner::<SemioMeshSnapshot>()` and **falls soft** to the shared placeholder box when the host
  has not materialized it. Known gap: the snapshot does **not** declare `#[child(kind = …)]`, because
  the handle sits inside an enum inside a `Vec<Tile>` and the `ArtifactSchema` derive only discovers
  a child on a direct field. Every bundled example therefore uses inline `Mesh`.

---

## 3. Mutation vocabulary — 15 kinds

`KINDS` order = enum declaration order = the oracle catalog order.

| kebab kind | variant | verb / entity | folder | guard highlights |
|---|---|---|---|---|
| `create-slot` | `CreateSlot { index, slot }` | create / slot | `🧩️create-slot` | fatal duplicate-id, fatal degenerate box, fatal unknown pinned tile |
| `delete-slot` | `DeleteSlot { id }` | delete / slot | `🕳️delete-slot` | error missing; **cascades every incident edge**, info `wfc3d.slot.edges-cascaded` |
| `move-slot` | `MoveSlot { id, x, y, z }` | move / slot | `🚚️move-slot` | error missing, warn unchanged |
| `resize-slot` | `ResizeSlot { id, width, height, depth }` | resize / slot | `📐️resize-slot` | error missing, fatal degenerate, warn unchanged |
| `connect-slots` | `ConnectSlots { index, edge }` | connect / slots | `🔗️connect-slots` | fatal duplicate-id, error missing slot, fatal self-loop, warn duplicate relation |
| `disconnect-slots` | `DisconnectSlots { id }` | disconnect / slots | `✂️disconnect-slots` | error missing |
| `pin-slot` | `PinSlot { id, tile_id }` | **fix** / slot (`Fixed`) | `📌️pin-slot` | error missing slot, error missing tile, warn unchanged |
| `unpin-slot` | `UnpinSlot { id }` | **clear** / slot (`Cleared`) | `📍️unpin-slot` | error missing, warn no pin |
| `create-tile` | `CreateTile { index, tile }` | create / tile | `🀄️create-tile` | fatal duplicate-id, fatal non-positive weight |
| `delete-tile` | `DeleteTile { id }` | delete / tile | `🗑️delete-tile` | error missing; **cascades every rule naming it AND releases every pin on it**, info `wfc3d.tile.references-cascaded` |
| `change-tile-weight` | `ChangeTileWeight { id, weight }` | change / tile-weight | `⚖️change-tile-weight` | error missing, fatal non-positive, warn unchanged |
| `change-tile-media` | `ChangeTileMedia { id, media }` | change / tile-media | `🖼️change-tile-media` | error missing, warn unchanged |
| `create-rule` | `CreateRule { index, rule }` | create / rule | `🚦️create-rule` | fatal duplicate-id, fatal unknown tile |
| `delete-rule` | `DeleteRule { id }` | delete / rule | `🚫️delete-rule` | error missing |
| `change-seed` | `ChangeSeed { seed }` | change / seed | `🎲️change-seed` | warn unchanged |

**Approved-verb mapping for the other slices** (`pin`/`unpin` are NOT in
`protocol::APPROVED_VERBS`): the kebab kinds stay `pin-slot`/`unpin-slot`, and
`SemanticDescriptor` carries `verb: "fix", record: "Fixed"` and `verb: "clear", record: "Cleared"` —
the coordinator's convention, matching the engine's own `.fix(node, pattern)` API. The
`dsl::Mutations` derive only const-asserts `kind == kebab(variant)` and `is_approved_verb(verb)`; it
does NOT require the verb to be the kind's first segment.

Diagnostic codes are namespaced `wfc3d.<entity>.<fault>` (never the generic `mutation.*`), so the
"one code, two contradictory readings" collision the assembly oracle recorded cannot happen here.

---

## 4. Window and scene design

Two editor windows, `row` 50/50 (`create_default_layout`), one viewer window full-pane.

**`wfc-graph`** (`SurfaceKind::NodeGraph`, body `wfc.graph`) — **copied byte-for-byte from wfc2d**
(`◻️2d/…/🪟️windows/🕸️graph/🦀️.rs`, plus its `🧪️tests/🔬️unit/🦀️.rs`); `diff` between the two files is
empty. It knows nothing about either snapshot: its whole view is the local `SlotGraphView` trait over
`GraphSlotView`/`GraphEdgeView` plus `GraphCamera`. wfc3d supplies one newtype,
`Wfc3dGraphView<'a>(&'a Wfc3dSnapshot)`, in `✏️editor/🦀️.rs`, projecting `x`/`y` and **dropping `z`**
(the canvas is a plan of an arbitrary graph; the third axis is the preview's business). One node per
slot at its authored rectangle, one `adjacent-in` + one `adjacent-out` port per node, one edge per
adjacency labelled with its relation, nine actions all `Migrated`.

**`wfc-3d-preview`** (`SurfaceKind::World3d`, body `wfc.3d.preview`) — the solved assembly.
`meshes_json` is the SMALL catalogue: one `{id, data:{positions, normals, colors, indices, uvs}}`
entry per distinct TILE plus one shared `tile:__placeholder` unit box. `instances_json` is the many
lane: one record per **SOLVED** slot, `{id, meshId, position, rotation, scale, label}`. A slot the
solve did not assign is OMITTED, never drawn as a placeholder box — the same strict contract `grid3d`
keeps, and the honest one: this window paints the SOLUTION, so an unsatisfiable document yields ZERO
instances and lets `status_json`'s verdict say why (`an_unsatisfiable_document_places_no_instances_and_says_so`).
The shared placeholder survives for the one case it is actually for: a SOLVED slot whose tile media
resolves to no geometry still gets a body
(`a_solved_slot_whose_tile_has_no_geometry_borrows_the_placeholder`). `instances_delta_json`
rides alongside as a full generation (`base: 0`, all `changed`, no `removed`) because this window
rebuilds its projection per render rather than keeping residency — `instances_json` stays
authoritative, which is the contract. `camera_json` frames the document's real bounds so the pane
never opens on an all-zero pose. `status_json` carries one short line (solved / not solved /
contradiction) — deliberately short, because one oversized owned text fails the WHOLE surface
refresh, not just that panel.

**`wfc-3d-view`** (viewer, body `wfc.3d.view`) — reuses the editor preview's own projection functions
verbatim, so both surfaces place the same instances over the same catalogue.

**State tiers.** Per-pane `Wfc3dConfig` (`camera_x/y/zoom`, `active_tile_id`) with its own
`ArtifactDsl`/`ArtifactPack` envelope (`wfc.wfc3d.config`, extension `wfc3dcfg`) and three VCS'd
operations (`replace-config`, `change-camera`, `change-active-tile`), each with a real inverse and a
committed descriptor + payload schema. `change-camera`/`change-active-tile` emit ONLY config
mutations, so a camera move can never enter the document's undo history.

> **Divergence from A3, deliberate**: wfc3d's editor uses `NoTransient`/`NoTransientMutation`. The
> solve is recomputed inline per preview render (`solved_transient(document)`) rather than cached in
> an app transient lane. `Wfc3dTransient` exists as a plain value type that `solved_transient`
> produces and the preview consumes. Cheaper to get right, and honest — the solve is derived, never
> stored either way — but it does mean the preview pays for a solve on every repaint. Wiring the
> transient lane (as wfc2d does) is the obvious follow-up if a profile shows it.

---

## 5. Inference — `s.wfc.wfc3d.solve`

`🧬️schema/💡️inferences/🦀️.rs`. Routed cold job (`semio.infer` / `s.wfc.wfc3d.solve`, payload schema
`s.wfc.wfc3d.inference.request.v1`, classification `Migrated`, `ToolExecutionContract::resumable`)
plus three `store::InferredField`s (`Wfc3dSolve`, `Wfc3dContradiction`, `Wfc3dEntropy`), all driven
by one headless `solve_with_job`. Stages, one fuel unit each:

`Tiles → Relations → Rules → Model → Compile → Slots → Edges → Topology → Fixed → {Restore|Solve} → MapCommit → EncodeCommit → Complete`

- **Tiles** → one `PatternId` per tile (index order), weights straight off the tile.
- **Relations** → the distinct edge-relation strings, in first-seen order, one `RelationId` each.
- **Rules** → each authored rule lowered to `(allowed, relation or None = all, tile a, tile b)`. A
  rule naming a relation this document never authored binds to no arc and constrains nothing; it is
  kept in the document and skipped here.
- **Model** → `engine::model::ModelBuilder`, stepped ONE authored RULE per fuel unit onto the EMPTY
  builder: `allow`/`deny` for both directed orders, over the named relation or over every relation
  when the rule names none. **Compile** then calls `builder.compile()` once, where deny wins.
  `GraphModelBuild` was NOT used: it compiles a single `RelationId(0)` and cannot express
  per-relation rules. `TiledModelBuilder` would also have worked (engine.md recommends it for this
  artifact) and is the natural follow-up — with it `PatternId` and `TileId` coincide exactly as they
  already do here.
- **Slots/Edges/Topology** → `engine::topology::GraphTopologyBuild`, `add_arc` in BOTH directions per
  edge with that edge's own relation, stepped to a `GraphTopology`.
- **Fixed** → `pinned_tile_id` becomes a `(NodeId, PatternId)` pin.
- **Solve** → `engine::job::WfcJob<GraphTopology>` (or `WfcRestore` from a checkpoint), stepped by
  the session. Commit = `assignments {slotId: tileId}`; the contradiction verdict is
  `solve_with_job(...).is_ok()`; entropy is the PRIOR Shannon entropy over the tile weights
  (`0.0` for a pinned slot) — explicitly NOT the post-propagation entropy.

Admission caps: 256 tiles, 32 relations, 65 536 slots, 262 144 edges, 262 144 rules, 1 KiB per id,
1 MiB commit output. The tile and relation universes are the two that actually bound compile work
(`relations × tiles²` bits); the rest bound linear passes.

### 🐛️ Four live-solve bugs found and fixed here (the first three latent in assembly's inference)

1. **The child `WfcJob` was dropped without its close ladder.** Its admitted pages stay registered in
   the session's payload ledger, so `worker_job_close_step` returns `Blocked` forever and the whole
   headless solve hangs in the CLOSE, not in the search. Fix: `engine::job::close_job(&mut child)`
   (and the same for `WfcRestore`) before the option is cleared. **Measured**: before the fix the
   example-asset generator ran >10 minutes on a 3-slot problem; after it, 0.04 s for all three
   examples plus all fifteen fixtures.
2. **`InteractiveJob::terminal_is_empty` must not gate on a private `closing` flag.** It reports
   OWNERSHIP ("this job holds nothing retainable"), and the framework's own close ladder decides when
   to call `begin_close`.
3. **A `CommitCandidate` carries TWO retained payloads** (`state` and `output`). The child's `state`
   becomes this job's commit state; its `output` is spent (the assignment comes from
   `take_completed_commit()`), so it is retired with `retire_payload`, never dropped — a dropped
   payload with outstanding pages trips `RetainedJobPayload`'s Drop assertion and aborts the process.

4. **A `StepContext` grants exactly ONE payload page per STEP** (`payload_page_granted`), so every
   stage that admits a page must END its step. `encode_one` admits one page per call and the encode
   arm looped over pages inside a single step; with `fuel_per_step: 1` that never showed, because the
   fuel ended the step anyway. Raising the fuel exposed it: the second admission in a step is refused
   as `OpportunityExhausted`, and the resulting error's OWN fault detail cannot be admitted either, so
   the job dies as a `StepOutcome::Fault` carrying an EMPTY page — a completely mute failure.
   **Measured**: with the fuel raised and no other change, 10 of 229 tests failed and every solve
   returned `Unsolved`; probing the outcome stream showed `Fault { pages: 0, len: 0 }`. Fix: the
   encode arm returns `StepOutcome::Yield` after each admitted page. The encode stage is therefore
   still one page per step by design, and only the compile/map/solve stages spend the new fuel.

This is why assembly's own `📚️examples/🧪️tests/🧩️outcome` says the live solver "aborts the process"
and falls back to a pure checker. wfc3d runs the real solve in its tests.

> ⚠️ **For the sibling artifacts**: bug 4 is a trap for ANY artifact that raises `fuel_per_step` above
> `1` while a stage loops over payload pages. `grid3d` and `bitmap` already ship `65_536` / `250_000`;
> if either encodes its commit page-by-page inside the step loop rather than yielding per page, it has
> this bug and its solves fail silently. Worth one grep each.
>
> ⏱️ Also noted while chasing it: `250_000 µs` is ~31× `semio_framework_trace`'s shared
> `INTERACTIVE_STEP_CEILING_US` (8 ms), against which EVERY step is measured whatever budget its
> config asked for, and four consecutive over-ceiling steps quarantine the session with the same mute
> empty-page fault. wfc3d is safe because its step length is bounded by the preview cadence (16 units)
> and by the one-page-per-step encode, not by this deadline — but the deadline is not what keeps steps
> short, and an artifact that lets one step run to it will be quarantined.

---

## 6. Tests — 229 Rust, 148 TypeScript (project-wide), 15 Python vectors

- **15 × 7 per-case fixture tests** (`🧬️mutations/<kind>/🧪️tests/<case>/🦀️.rs`): applies-to-after,
  inverse-restores-before, both snapshots canonical, declared outcome holds, produces-committed-diff,
  committed-diff-canonical, committed-diff-applies-to-after.
- **Aggregate laws** (`🧬️mutations/🧪️tests/🔬️unit`): approved verbs, `KINDS` == dispatch roster,
  **inverse restores base exactly for all 15**, collections stay sorted after a canonical insert,
  op-text and op-binary round trip for all 15, refusal changes nothing, duplicate id is fatal.
- **Inference** (`💡️inferences/🧪️tests/🔬️unit`): trivially-satisfiable assigns every slot;
  contradiction agrees with solve; **an unsatisfiable spec is reported, not panicked or hung**;
  identical seed+spec ⇒ identical solution (the determinism law `DepHash` caching rests on); a new
  seed still solves; pins are respected; **a forbidding rule beats an admitting one**; **a
  relation-scoped rule only reaches its own relation**;
  empty document solves trivially; entropy (`ln 2` uniform, `0.0` pinned, skewed < uniform);
  routed metadata; idempotent factory registration; oversized catalogue refused at admission.
- **Schema/diff/snapshot/text/binary**: 20 non-empty descriptor leaves, mirrors name the same six
  fields, builder/analyzer facets, DSL + pack round trips for all three examples, tile media survives
  the `DslValue` bridge, bounded snapshot retirement ladder, diff index validation (invalid-index,
  missing-target, structural absorb).
- **Editor/viewer/windows**: canonical app ids and breadcrumbs, both window kinds, **every one of the
  15 document commands dispatches to exactly one mutation and touches no config**, the two view verbs
  touch only the config, armed-tile fallback, zero-extent normalisation, canonical insert position,
  **both window bodies render non-empty for all three examples**, viewer body renders for all three,
  unknown body key degrades to a label.
- **Mount contract + store fixture**: the committed fixture is the one statement of app ids, window
  kinds, mutation roster, inference route and example roster (Rust + TypeScript + Python all read
  it); a live `ArtifactStore` opens on an example, applies an edit, round-trips through document text
  and pack, and closes through its exact bounded owners.
- **Examples**: each example's `🗣️.dsl.semio` asset IS the print of its Rust builder; every collection
  sorted; the committed outcome is checked twice — by a pure consistency checker AND against the LIVE
  solve.
- **Python second implementation** (`🧪️tests/🧩️mutate-wfc3d-1/🐍️.py`, 15/15 green): re-implements the
  document, all 15 mutations with their own guards and both cascades, the sparse delta shape and
  every inverse, from the JSON Schemas — then requires the produced delta, the after-snapshot, the
  diagnostics AND its own inverse to agree with the committed quintets. It imports nothing from this
  repository.

---

## 7. Known gaps (for W2/W3)

1. **No Rust subject adapter for `🧪️tests/🧩️mutate-wfc3d-1`.** The oracle manifest
   (`🔮️oracles/🔣️.json`), the `🥒️.feature` and the Python reference are all committed; the
   `semio_repo_test_host` adapter (`🦀️.rs`) that `bun ./📜️script.ts contract` drives is NOT. It needs
   a generated host and could not be verified here. The Python half is runnable today:
   `python3 $T/🐍️wfc3d-oracle-runner.py`.
2. **Taxonomy registrations are not made** (slice P owns `🔣️taxonomy.json`). Needed:
   `members-of-tests` += the 15 case dir names, `🧩️mount-contract`, `🧩️mutate-wfc3d-1`,
   `🗄️store-fixture`, `🔬️unit`, `🧩️example`, `🧩️outcome`; `members-of-examples` +=
   `🚪️two-room-corridor`, `🧱️wall-roof-facade-strip`, `🗼️tower-stack`; `members-of-windows` +=
   `🕸️graph`, `🧊️preview`; `members-of-fixtures` += `🧩️mount-contract`, `🧬️mutations`. A missing
   memberName makes discovery return zero for that owner while the gate reads green.
3. **`bun ./📜️script.ts verify taxonomy|artifact-field-parity|contract` were not run** — taxonomy
   loading was broken repo-wide for part of this slice (the `▦️grid2d` fault, since fixed by P) and
   the nx daemon graph is still stale after the folder move. W2 should run them after a `bun nx reset`.
4. **`TileMedia3d::MeshChild` declares no `#[child(kind)]`** — see §2. A document that actually used
   mesh children would not publish them as composed child refs.
5. **The preview solves inline per render** — see §4.
6. **`ModelBuilder` rather than `TiledModelBuilder`** — see §5.
7. The `🔺️diff/📝️text` and `🔺️diff/💾️binary` TypeScript leaves exist because the plugin's TS barrel
   imports them; they state honestly that the diff has NO dedicated carrier (it rides the document's
   own). There is no Rust module under either directory.

---

## 8. What the plugin root (W2) needs from this artifact

```rust
semio_s_artifact_wfc_3d::artifact::<WfcApps>()                                    // .declare_artifact(…)
semio_s_artifact_wfc_3d::inferences::wfc3d_inference_metadata()                   // .routed_inference(…)
semio_s_artifact_wfc_3d::inferences::register_wfc3d_inference_factory(&ActionBus::production())?
semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor   / create_wfc3d_editor()
semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer   / create_wfc3d_viewer()
semio_s_artifact_wfc_3d::artifact_kind().id            // "3d.wfc3d" for .activation(OnArtifactKind)
semio_s_artifact_wfc_3d::examples::sources()           // 3 ExampleSources
```

`Cargo.toml` member + alias `semio-s-artifact-wfc-3d` are already in the root workspace (P).
Playground row: variant `wfc3d`, ports 6045 (react) / 6145 (wgpu),
`app = "s.wfc.wfc3d@1/*#editor"` — the empty-`VITE_SEMIO_APP_ID` fault is fault #1 in the pipeline
report and this artifact's app id is the value it needs.
