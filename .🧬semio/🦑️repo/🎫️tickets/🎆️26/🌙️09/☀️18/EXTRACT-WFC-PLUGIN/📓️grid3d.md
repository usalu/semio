# 🧱️ A4 — `s.wfc.grid3d`, the WFC 3D grid artifact

Crate `semio-s-artifact-wfc-grid3d` at `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d` (233 files, 100 Rust).
Dialect `s.wfc.grid3d@1/*`, OS kind `3d.wfcgrid3d`, apps `s.wfc.grid3d@1/*#editor` / `#viewer`,
labels "3D Grid" / "3D-Raster", module ident `grid3d`.

---

## 1. Verification — every command run, with its result

| # | command | result |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --tests -j 4` | ✅️ exit 0 (`🗑️generated/grid3d/check-7.log`) |
| 2 | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib -j 4 -- --test-threads=4` | ✅️ **201 passed, 0 failed, 2 ignored**, 0.14 s (`test-final.log`) |
| 3 | `cargo clippy -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --tests -j 4` | ✅️ exit 0, **zero warnings in this crate** (`clippy-4.log`; the remaining warnings in the log are all framework crates) |
| 4 | `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-artifact-wfc-grid3d --features component-app-assembly --target wasm32-wasip2 -j 4` | ✅️ exit 0 in 25 s (`wasm-1.log`) |
| 5 | `python3 …/🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py` (the second implementation, stdlib only) | ✅️ `replayed 14 vector(s) across 14 kinds`, exit 0 |
| 6 | `cargo test … -- --ignored debug_emit` (fixture + example-asset generators) | ✅️ exit 0 (`gen-4.log`) |
| 7 | `bun nx run @semio-tech/wfc-js:test` | ❌️ **blocked, not by this artifact**: the Nx project graph refuses to load — `Native dependency has no Nx project owner: ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust` (that crate has no `📋️project.json`; grid3d's own is present). See §7. |
| 8 | `bun ./📜️script.ts verify taxonomy enforce --scope "…/🧱️grid3d"` | ❌️ **blocked, pre-existing and unrelated**: `Current compiler input manifest compiler inputs are not path-sorted` — the stale, gitignored `🧰️framework/…/🧑‍💻dev/📤️distribution/🧾️manifest.json`, already recorded by slice P. Zero wfc rows in it. |

Scripts that (re)produce every generated file, all in this ticket folder and all idempotent:
`🐍️grid3d-mutation-scaffold.py` (14 mutation folders), `🐍️grid3d-schema-leaves.py` (the 20 + 5 facet
leaves), `🐍️grid3d-case-tests.py` (14 fixture-replay modules + the crate-root mounts),
`🐍️grid3d-oracles.py` (the oracle manifest), `🐍️grid3d-grammar-sidecars.py` (`.ebnf`/`.g4` from the
grammar, `.ksy`/`.spicy`/`.abnf` from the protocol).

---

## 2. Document model as authored

`Grid3dSnapshot` (`🧬️schema/📸️snapshot/🦀️.rs`), JSON Schema normative at `📸️snapshot/🔣️.json`:

```
schema: String            = "s.wfc.grid3d"
seed: u64                   deterministic solve seed, authored only by change-seed
width, height, depth: u32   grid extent, each ≥ 1
cellSizesX/Y/Z: [f64]       NON-UNIFORM: one size per column/row/layer; len == the axis' extent
periodicX/Y/Z: bool         Boundary::Wrap when set, Boundary::Open otherwise
tiles:  [Grid3dTile]        sorted by id
rules:  [Grid3dRule]        sorted by id
pinned: [Grid3dPinnedCell]  sorted by the cell key "x:y:z"
masked: [Grid3dCell]        sorted by the cell key "x:y:z"
```

- `Grid3dTile { id, label?, weight, media }`; `weight` is the sampler bias, always finite and positive
  (every mutation refuses otherwise).
- `Grid3dTileMedia` = `Mesh { mesh: Grid3dMesh }` | `MeshChild { child: ArtifactChild<SemioMeshSnapshot> }`,
  tagged by `kind` (`"mesh"` / `"meshChild"`). `Grid3dMesh { positions: [f64] (xyz triples in the
  tile-space unit box 0..1), indices: [u32], color? }`. A `MeshChild` the host has not hydrated renders
  as a unit-box placeholder rather than failing the surface.
- `Grid3dRule { id, tileAId, tileBId, direction, allowed }`, `direction ∈ LEFT RIGHT FRONT BACK BOTTOM TOP`
  meaning "B lies in `direction` of A". **The rule set is a CLOSED ALLOW-LIST: a tile pair no rule
  mentions for a direction is NOT allowed**, and an `allowed: false` rule is an explicit DENY that wins
  over any allow of the same directed pair.
- `Grid3dColor { r, g, b, a }` 0–255, `Grid3dAxis ∈ x y z`.
- **Deviations from plan §3, both deliberate**: (a) the solve's commit row is a RECORD
  `{x, y, z, tileId}` and not a 4-tuple — `semio_framework_value_derive` implements `ToValue`/`FromValue`
  for 2-tuples but not 4-tuples, so `[[x,y,z,tile]]` is unrepresentable on this wire; (b) the commit also
  carries `satisfiable: bool` (see §5).

### Geometry

`cell_origin` is the cumulative sum of the sizes before the cell on each axis; `cell_extent` is the
cell's own three sizes. There is no framework grid-geometry helper — positions are freeform floats and
the artifact computes them (`🧬️schema/🌐️scene-internals/🦀️.rs`).

---

## 3. Mutations (14, every one point-invertible)

Every collection insert lands at the CANONICAL SORTED position via `mutations::ordered_index`, never at
the end, so a delete and its inverse create round-trip at the same index. `ordered_index` answers the
EXISTING index when the key is already present, which is what lets one diff lane serve both
upsert-in-place and insert.

| kind | folder | payload | refuses | inverse |
|---|---|---|---|---|
| `change-seed` | `🎲️change-seed` | `seed` | no-op warn | `change-seed(base.seed)` |
| `resize-grid` | `📐️resize-grid` | `width/height/depth` | empty axis; a pin or mask left outside | `resize-grid(base…)` + `change-cell-sizes` ×3 |
| `change-cell-sizes` | `📏️change-cell-sizes` | `axis, sizes` | wrong length; non-finite/non-positive | `change-cell-sizes(axis, base sizes)` |
| `change-periodicity` | `🔁️change-periodicity` | three flags | no-op warn | the base flags |
| `create-tile` | `🧱️create-tile` | `tile` | empty/duplicate id; bad weight | `delete-tile(id)` |
| `delete-tile` | `🕳️delete-tile` | `id` | missing target | base-derived `create-tile` + every cascaded `create-rule`/`pin-cell` |
| `change-tile-weight` | `⚖️change-tile-weight` | `tileId, weight` | missing tile; bad weight; no-op | the base weight |
| `change-tile-media` | `🖼️change-tile-media` | `tileId, media` | missing tile; no-op | the base media |
| `create-rule` | `🚦️create-rule` | `rule` | empty/duplicate id; unknown tile | `delete-rule(id)` |
| `delete-rule` | `❌️delete-rule` | `id` | missing target | base-derived `create-rule` |
| `pin-cell` | `📌️pin-cell` | `pinned` | outside grid; unknown tile; masked cell; no-op | the previous pin, else `unpin-cell` |
| `unpin-cell` | `📍️unpin-cell` | `x, y, z` | no pin there | base-derived `pin-cell` |
| `mask-cell` | `🚫️mask-cell` | `cell` | outside grid; PINNED cell; already masked | `unmask-cell` |
| `unmask-cell` | `🔓️unmask-cell` | `x, y, z` | not masked | base-derived `mask-cell` |

`resize-grid` REFUSES rather than cascading a pin or mask away, and `mask-cell` REFUSES a pinned cell
rather than silently unpinning it — both kept atomic on purpose, so each stays a single-step inverse.
`delete-tile` is the one deliberate cascade (rules and pins that named the tile), and its inverse is
base-derived and restores each cascaded row at its own canonical position.

Semantic verbs follow the coordinator's cross-artifact convention (`APPROVED_VERBS` has no
pin/unpin/mask/unmask): `pin-cell → fix`, `unpin-cell → clear`, `mask-cell → remove`,
`unmask-cell → restore`; kebab kinds unchanged.

---

## 4. Window and scene design

Mode `edit`, layout `row` 50 / 50, both surfaces `SurfaceKind::World3d`.

**`wfc-grid3d-grid`** (`✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️grid`) — one box instance per cell at its
own non-uniform origin, scaled to its own box. The catalogue holds ONE mesh per distinct look
(neutral translucent cage, dark masked cage, one tile-tinted cage per tile some cell is pinned to) —
3 meshes for 48 cells in the `blocks` example, never one mesh per cell. **The instance id IS the cell
key `x:y:z`**, so a pick comes back addressable with no second lookup table. It declares its own
interaction domain `wfc.grid3d.cells` (one granularity, `cell`) — a window kind referencing an
undeclared domain is refused outright by the plugin builder. Utilities `select` / `pin` / `mask`;
`pickCell` reads the utility ARMED FOR THE DISPATCHED WINDOW (`active_utility_by_window_id` first,
focused window second — `🗒️note`'s five-step chain) and resolves to `pin-cell`, `mask-cell`, or nothing.

**`wfc-grid3d-preview`** (`…/🪟️windows/👁️preview`) — one mesh entry per TILE, one instance per SOLVED
cell scaled into that cell's box. `instances_json` is authoritative on every publication;
`instances_delta_json` rides alongside it only when it names less than half the set (puzzle 3d's
`delta_is_worth_publishing` rule). The `base` it applies to comes from a PROCESS-LOCAL residency keyed
by window instance (`static RESIDENCY: Mutex<BTreeMap<String, Grid3dPreviewResidency>>`): app-local
scratch, never a document field, never a checkpoint. `status_json` carries `{state, solved, cells, seed}`
and is asserted ≤ 512 bytes, because one oversized `UiText` fails the whole surface's refresh.

Per-window-instance persisted config `Grid3dWindowConfig` (`✏️editor/🪟️window`): camera position/target/
zoom, the armed tile, the masked-cell toggle — one owner per window kind, addressed by instance id, so a
pane never writes its sibling's pose. An all-zero camera (position == target) is the "never posed"
reading; such a pane opens framed on what the document actually spans.

The viewer (`👁️viewer`, window `wfc-grid3d-view`) renders the same solved scene through the SAME pure
`🧬️schema/🌐️scene-internals` functions. Those live in the schema tree, not under either surface, so the
viewer never imports through the mutation-capable editor (`policyViewerPurityBreaches`) and the two
surfaces cannot drift.

---

## 5. Inference `s.wfc.grid3d.solve`

`🧬️schema/💡️inferences/🦀️.rs`. Tool id `s.wfc.grid3d.solve`, job kind `semio.infer`, payload schema
`s.wfc.grid3d.inference.request.v1`, owner `wfc`. Registered as a routed cold job
(`register_grid3d_inference_factory`) plus three synchronous `store::InferredField`s
(`Grid3dSolve` / `Grid3dContradiction` / `Grid3dEntropy`) that drive the SAME job headlessly.

Stages: `Tiles → Rules → Model → Mask → Topology → Fixed → {Restore | Solve} → MapCommit → EncodeCommit → Complete`.

- **Tiles** — one `TileId` per tile in index order, weight `1.0` for a non-positive/non-finite one.
- **Rules** — `TiledModelBuilder` + `declare_stencil_relations_3d_tiled(Stencil3d::Face6)`; the returned
  relation vector (offsets order `[+x −x +y −y +z −z]`) is handed to `Grid3dTopology::new` unchanged.
  `allowed` → `allow(forward, a, b)` AND `allow(opposite, b, a)`; `!allowed` → the matching pair of
  `deny`s, which always win.
- **Model** — one `compile()`; **Mask** — one `mask[z·w·h + y·w + x] = false` per masked cell;
  **Topology** — `Grid3dTopology::new(w, h, d, Face6, relations, bx, by, bz, mask)`, boundaries
  `Wrap`/`Open` per periodicity flag; **Fixed** — one `(NodeId, PatternId)` per pin.
- **Solve** — the shared `job::WfcJob<Grid3dTopology>`, or `WfcRestore` from a checkpoint.
- **MapCommit** — one `{x, y, z, tileId}` row per unmasked cell; **EncodeCommit** — the paged commit
  payload `{"satisfiable":…,"assignments":[…]}`.

Determinism: every input is read off the snapshot (seed included), no ambient randomness, so `DepHash`
caching over the three fields is sound. `the_same_seed_and_spec_always_produce_the_same_assignment`
asserts it on the 48-cell example.

### Four real faults found and fixed here

1. **One payload page per STEP.** `StepContext::admit_payload_page` grants exactly one page per step
   (`payload_page_granted`). The encode stage originally looped inside one step, so the second page was
   refused as `OpportunityExhausted` — and the refusal's own fault detail could not be admitted either,
   which surfaces as an EMPTY fault string with no clue in it. Fixed by yielding after every committed
   page.
2. **A `CommitCandidate` carries TWO retained payloads** (`state` and `output`). Keeping only `state` and
   letting the candidate drop trips `RetainedJobPayload`'s Drop assertion and aborts the process. Both
   the Solve and the Restore arm now retire what they do not keep. (Assembly has this latent in both
   arms — do not copy it.)
3. **A child `WfcJob`/`WfcRestore` must go through the close ladder before it is dropped.**
   `self.child = None` after a completed solve aborts the process. `close_owned` runs
   `begin_close` + `close_step` to terminal-empty first.
4. **`wfc-unsatisfiable` is published as a job FAULT**, not as a commit-less completion
   (`⚙️engine/💼️job/🦀️.rs`, `PublicationKind::Fault`). That exact detail is intercepted, its payload
   retired, the child closed, and the verdict answered as `satisfiable: false` with no rows — a grid with
   no consistent assignment is an ANSWER, never an inference failure. This is why the commit grew a
   `satisfiable` field.

Also: `InteractiveJob::terminal_is_empty` reports OWNERSHIP ONLY and is never gated on a `closing` flag
(the framework calls `begin_close` itself at close stage 0 and answers `Blocked` forever otherwise);
the headless close loop carries the `assert_ne!(…, Blocked)` guard instead.

**Performance.** The headless adapter runs `fuel_per_step = 65 536`, `step_budget_us = 250 000`, not
assembly's `1`/`2000`: at one unit per step the 48-cell `blocks` example spent **over ten minutes** in
session bookkeeping. With the bigger budget the entire 201-test suite — which solves both examples
several times — runs in **0.14 s**.

---

## 6. Examples, fixtures, oracles

- `📚️examples/🧱️blocks` — the real-world case: `air`/`floor`/`roof`/`wall` on a NON-UNIFORM 4×3×4 grid
  (`cellSizesX [1, 2, 2, 1]`, `Y [3, 1.5, 1.5]`, `Z [1, 1, 1, 2]`), 38 rules (all 16 horizontal pairs for
  RIGHT and BACK, six vertical stack pairs for TOP), one pinned floor cell, one masked cell. 47 cells to
  fill; solvable for any seed.
- `📚️examples/🪠️pipes-3d` — the tiny case: `empty`/`pipe-x`/`pipe-z` on a 3×3×3 grid with a PERIODIC x
  axis, 14 rules, one pinned `pipe-x`. The wrap forces the whole pinned row to `pipe-x`, which the
  `a_periodic_axis_wraps_rather_than_ending_the_run` test measures.
- Both `🗣️.dsl.semio` assets are PRINTS of the Rust builder (generated by the `debug_emit_example_assets`
  ignored test), so the asset can never drift from the authority.
- `🧫️fixtures/🧬️mutations/**` — 14 quintets (before/after/mutation/outcome/diff), all generated from ONE
  base scene by `debug_emit_mutation_fixtures`. Floats canonical `N.0`; keys in struct-declaration order
  (the generator re-INDENTS the canonical text instead of round-tripping a `serde_json::Value`, which
  would re-sort every object's keys).
- `🔮️oracles/🔣️.json` — a `verified-native-second-implementation` entry for the Python reference plus a
  recorded `noOracleDecisions` refusal naming every third-party WFC implementation surveyed and why each
  was declined (they all oracle a SOLVER, which the engine crate already differential-tests; none of them
  has this artifact's document model, mutation algebra or non-uniform geometry).
- `🧪️tests/🧩️mutate-wfc-grid3d-1/{🥒️.feature, 🐍️.py, 🦀️.rs}` — declared `asset://` vector table, the
  stdlib-only Python second implementation, and the Rust exhaustiveness case that keeps the enum, the
  `KINDS` roster, the 14 on-disk manifests, the 14 payload schemas, the oracle catalog, the TypeScript
  twin, the feature table and the Python oracle all naming the same 14 kinds.

---

## 7. Known gaps / what the plugin root and W2 need to know

1. **`bun nx run @semio-tech/wfc-js:test` cannot run** until `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/📦️packages/🦀️rust`
   gets a `📋️project.json` — the Nx graph plugin refuses the whole workspace over it
   (`Native dependency has no Nx project owner`). grid3d's own `📋️project.json` + `📜️script.ts` are in
   place (`@semio-tech/wfc-grid3d-rs`). Nothing in grid3d can fix it.
2. **`verify taxonomy` is blocked** on the pre-existing unsorted `🧾️manifest.json` slice P already
   recorded; grid3d was never reached by that check, so its taxonomy conformance is UNVERIFIED by tooling
   (it does follow the reference tree shape file for file).
3. **The plugin root must call** `standards::v1::subsets::any::subset::<PA>()` through
   `crate::artifact::<PA>()`, `.routed_inference(schema::inferences::grid3d_inference_metadata())` and
   `register_grid3d_inference_factory(&ActionBus::production())`. `PA` must satisfy
   `crate::ArtifactApps` (`From<VcsArtifactApp<EditorApp<Grid3dEditor>>>` + the `ViewerApp<Grid3dViewer>`
   twin).
4. **`io::io()` ships `entries: &[]`** — the native DSL/pack pair is real and complete; grid3d contributes
   no foreign-format hop of its own. The stdio text round trip is the plugin-level composer.
5. **The preview solves on the render path.** `ArtifactEditor::render` is handed only document + config +
   view state, so there is no transient lane to cache a solve in. It is fast (a whole example solves in
   well under a millisecond), but a document far larger than the bundled examples would pay that cost per
   frame. The honest fix is a retained tool run, which this slice did not need.
6. **Not exercised here**: the playground boot and the browser probes (no server is started from a slice)
   and the storybook gate. `describe`/`plugin-registry:generate` belong to the plugin root.
7. `🧬️schema/🌐️scene-internals` is a Rust-only internals module under `🧬️schema`, following remodeling's
   `➕️algebra-internals`/`🗺️spatial-internals` precedent. It carries no UI dependency (plain `dsl::json`
   + `semio_framework::MeshData`), which is what lets both surfaces share it.
