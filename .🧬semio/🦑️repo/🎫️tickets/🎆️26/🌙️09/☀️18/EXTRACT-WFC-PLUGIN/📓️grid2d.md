# 🔲️ `s.wfc.grid2d` — the 2D-grid artifact, as authored

Slice **A2**. Crate `semio-s-artifact-wfc-grid2d`, folder
`✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d`, dialect `s.wfc.grid2d`, OS kind `2d.wfcgrid2d`, ident
`grid2d`, app id `s.wfc.grid2d@1/*#editor`, labels "2D Grid" / "2D-Raster", ports 6042/6142.

Two renames landed mid-slice and are fully absorbed: `▦️grid2d` → `🔲️grid2d` (▦ is not
Extended_Pictographic, so the taxonomy loader refused it) and `🌊️wfc` → `🀄️wfc` (🌊 folds to the
same grapheme as the sibling `🌊️flow` plugin). No `▦️`/`🌊️wfc` string survives under this artifact.

---

## 1. Document model as authored

`Grid2dSnapshot` (`🧬️schema/📸️snapshot/🦀️.rs`), camelCase on the wire, field order as declared:

| field | type | meaning |
|---|---|---|
| `schema` | `String` | `"s.wfc.grid2d"` |
| `seed` | `u64` | the deterministic solve seed — PERSISTED, authored only by `change-seed`, never ambient |
| `width` / `height` | `u32` | grid extent in cells (both ≥ 1) |
| `cellWidth` / `cellHeight` | `f64` | world size of one cell; the box every tile's media is scaled into, and the grid pane's `grid_factor` |
| `periodicX` / `periodicY` | `bool` | per-axis wrap → `Boundary::Wrap`, else `Boundary::Open` |
| `tiles` | `[WfcTile2d]` | the pattern universe, kept sorted by id |
| `rules` | `[WfcAdjacencyRule2d]` | the adjacency whitelist, kept sorted by id |
| `pinned` | `[WfcPinnedCell2d]` | hard pre-assignments, kept row-major |
| `masked` | `[WfcCell2d]` | holes cut out of the problem, kept row-major |

Vocabulary, all defined LOCALLY (an artifact crate may never import another app plugin's crate, so
`🖍️draw`'s `PathSegment` is mirrored, not depended on):

- `WfcColor { r, g, b, a: u32 }` (0–255).
- `WfcPoint2 { x, y: f64 }` in the tile's own `0..1` unit space.
- `WfcPathSegment` — tagged `kind`: `moveTo` / `lineTo` / `quadTo` / `cubicTo` / `close`.
- `WfcVectorPath { segments, fill?, stroke?, strokeWidth }`.
- `WfcTileMedia2d` — tagged `kind`:
  `bitmap { width, height, palette: [WfcColor], pixels: base64 palette indices, row-major }` |
  `vector { paths }` | `image { child: ArtifactChild<SemioImageSnapshot> }`
  (`s.stdio.semio@v1/image`).
- `WfcTile2d { id, label?, weight: f64 > 0, media }`.
- `WfcDirection2d` — `LEFT` / `RIGHT` / `TOP` / `BOTTOM`, `y` growing DOWNWARD, so `TOP` is `y - 1`.
- `WfcAdjacencyRule2d { id, tileAId, tileBId, direction, allowed }` — "B lies in `direction` of A".

**The rule default, stated once and enforced everywhere: a `(A, B, direction)` pair with no authored
row is FORBIDDEN.** The rule set IS the complete whitelist, so an empty rule set is unsatisfiable the
moment two adjacent unmasked cells exist (a committed test asserts exactly that). `allowed = false`
rows exist so an editor can carry an explicit refusal and toggle a rule without losing its id.

Two decisions worth naming:

- **`WfcDirection2d` spells its wire tokens per variant** (`#[value(rename = "LEFT")]`, …). The value
  derive supports only `camelCase`/`kebab-case`/`lowercase`/`snake_case` and **silently ignores**
  anything else — a `rename_all = "SCREAMING_SNAKE_CASE"` left the wire on PascalCase while the JSON
  Schema and the GraphQL enum declared `SCREAMING_SNAKE_CASE`. That mismatch cost 87 test failures
  before it was found; every sibling wfc artifact with an uppercase enum has the same trap.
- **`Bitmap.pixels` uses `format: "base64"`, never `contentEncoding`** — `OwnedJsonSchemaValidator`
  hard-errors on keywords outside its allowlist.

### Files

```
🔲️grid2d/
  🦀️.rs                                   crate root: WFC_GRID2D_{DOCUMENT_SCHEMA,DIALECT}, artifact_kind(),
                                          definition() (15 capability rows), ArtifactApps, artifact<PA>(),
                                          the whole #[path] mount tree, flat shims, 3 test mounts
  🟦️.ts                                   scaffolder stub (artifact root TS facade)
  📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts}
  🧪️tests/{🔬️unit,🔬️store-fixture}/🦀️.rs
  🏅️standards/🔖️1/🦀️.rs                    standard() — mimes/extensions, mounts subset any
  🏅️standards/🔖️1/🪆️subsets/🔣️.json        subset policy (scaffolder)
  🏅️standards/🔖️1/🪆️subsets/✳️any/
    🦀️.rs                                 subset() — SchemaDeclaration{descriptor,inferences,services}, io(), surfaces, examples
    🧬️schema/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}        artifact facet + the 20-leaf descriptor
    🧬️schema/📸️snapshot/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}
    🧬️schema/📸️snapshot/📝️text/{🦀️.rs,📖️.grammar.semio,🟦️.ts}   Dsl twin + ArtifactDsl/ArtifactPack
    🧬️schema/📸️snapshot/💾️binary/{🦀️.rs,📡️.protocol.semio,🟦️.ts} encode/decode + the bounded retirement
    🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts,📝️text/🟦️.ts,💾️binary/🟦️.ts}
    🧬️schema/🧬️mutations/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}  + 📝️text/ + 💾️binary/ (op codecs)
    🧬️schema/🧬️mutations/<14 kinds>/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json,🔺️diff/🦀️.rs,↩️inverse/🦀️.rs,🧪️tests/<case>/🦀️.rs}
    🧬️schema/💡️inferences/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}
    🧬️schema/🧪️tests/🧩️suite/🟦️.ts          the cross-language fixture oracle (vitest)
    🚪️io/{🦀️.rs,🟦️.ts} + 📤️export/…/{🔤️txt,🔣️json} + 📥️import/…/{🔤️txt,🔣️json}
    ✏️editor/{🦀️.rs,🪟️window/🦀️.rs,🎭️modes/✏️edit/{🦀️.rs,🪟️windows/{🔲️grid,👁️preview}/🦀️.rs}}
    👁️viewer/{🦀️.rs,🎭️modes/👁️view/{🦀️.rs,🪟️windows/👁️preview/🦀️.rs}}
    📚️examples/{🚰️pipes,🏜️terrain}/🦀️.rs
    🧫️fixtures/🧬️mutations/<14>/<case>/{📸️snapshot/⬅️before,➡️after,🦠️mutation,🎯️outcome,🔺️diff}/🔣️.json
    🔮️oracles/🔣️.json
    🧪️tests/{🧩️mount-contract/🦀️.rs, 🔲️mutate-grid2d-1/{🥒️.feature,🐍️.py,🦀️.rs}}
```

---

## 2. The fourteen mutations

Kind / directory / payload / what its diff touches. Every collection insert lands at its CANONICAL
sorted position (`ordered_tile_index` / `ordered_rule_index` / `ordered_cell_index`) — never an
append — so a delete's inverse restores POSITION as well as value.

| kind | dir | payload | diff lanes | inverse |
|---|---|---|---|---|
| `change-seed` | 🎲️ | `seed` | `seed` | `change-seed(base)` |
| `resize-grid` | 📐️ | `width, height` | `width`,`height`,`pinnedRemoved`,`maskedRemoved` | resize back + re-`mask`/`pin` every cascaded cell |
| `change-cell-size` | 📏️ | `cellWidth, cellHeight` | both scalars | restore both |
| `change-periodicity` | 🔁️ | `periodicX, periodicY` | both flags | restore both |
| `create-tile` | 🌱️ | `tile` | `tilesUpserted[(sorted, tile)]` | `delete-tile(id)` |
| `delete-tile` | 🗑️ | `id` | `tilesRemoved` + every rule naming it + every pin to it | re-create tile, then rules, then pins |
| `change-tile-weight` | ⚖️ | `id, weight` | `tilesUpserted[(existing index, tile)]` | restore weight |
| `change-tile-media` | 🎨️ | `id, media` | `tilesUpserted[(existing index, tile)]` | restore media |
| `create-rule` | 🚦️ | `rule` | `rulesUpserted[(sorted, rule)]` | `delete-rule(id)` |
| `delete-rule` | ❌ | `id` | `rulesRemoved` | re-create the exact row |
| `pin-cell` | 📌️ | `x, y, tileId` | `pinnedUpserted` (existing index on re-pin, else row-major) | re-pin the old tile, or unpin |
| `unpin-cell` | 📍️ | `x, y` | `pinnedRemoved` | re-pin |
| `mask-cell` | 🕳️ | `x, y` | `maskedUpserted` + `pinnedRemoved` (a masked cell carries no pin) | unmask, then restore the pin |
| `unmask-cell` | 🔳️ | `x, y` | `maskedRemoved` | mask again |

Refusals, by code: `mutation.duplicate-id` (tile/rule id already taken), `mutation.invariant` (empty
id, non-positive weight, non-positive cell size, zero extent, out-of-bounds cell, unknown tile, a pin
on a masked cell, a second rule on the same `(A,B,direction)`), `mutation.target-missing` (delete /
unpin / unmask of something that is not there), `mutation.no-op` (warning; the value is already what
the payload asks for), `mutation.cascade` (info; names how many dependent rows went with it).

⚠️ **`pin` / `unpin` / `mask` / `unmask` are NOT in `protocol::APPROVED_VERBS`** and
`#[derive(dsl::Mutations)]` asserts membership at COMPILE time. The four cell mutations keep their
kebab KINDS (`pin-cell`, …) and carry the approved verbs `fix` / `clear` / `remove` / `restore`
(records `FixedCell` / `ClearedCellPin` / `RemovedCell` / `RestoredCell`). Sibling slices with
`pin-slot`/`mask-*` verbs hit the same wall.

---

## 3. The solve inference — `s.wfc.grid2d.solve`

`🧬️schema/💡️inferences/🦀️.rs`. Tool id `s.wfc.grid2d.solve`, job kind `semio.infer`, payload schema
`s.wfc.grid2d.inference.request.v1`, owner `wfc`, classification `Migrated`.

Stages (`Grid2dInferenceStage`), one fuel unit each:

1. **Tiles** — one `TiledModelBuilder::tile(weight)` per authored tile, in document order; a
   non-positive or non-finite weight is neutralised to `1.0` rather than refused (the mutation layer
   already refuses authoring one). An empty catalogue short-circuits to a contradiction commit.
   On exhaustion: `declare_stencil_relations_tiled(&mut builder, &Stencil2d::VonNeumann)`.
2. **Rules** — each `allowed = true` row calls `builder.allow_mirrored(relations[slot], a, b)`, so a
   `RIGHT` row already states the matching `LEFT` one. Relation slots follow
   `Stencil2d::VonNeumann::offsets()` order `[(1,0),(-1,0),(0,1),(0,-1)]` = RIGHT, LEFT, BOTTOM, TOP.
3. **Model** — `builder.compile()`.
4. **Topology** — `Grid2dTopology::new(width, height, &VonNeumann, relations, boundary_x,
   boundary_y, mask)`; `mask[y*w+x] = false` for every masked cell, `Boundary::Wrap` per periodic
   axis else `Boundary::Open`. The SAME `Vec<RelationId>` the declaration returned is handed on.
5. **Fixed** — every pinned cell becomes `(NodeId, PatternId)`; every MASKED cell is additionally
   pinned to the placeholder pattern 0, because an inactive cell still owns a domain in the job's
   dense state — and is then omitted from the commit.
6. **Restore | Solve** — `WfcRestore::new(…checkpoint)` stepped to completion then `take_job()`, or
   `WfcJob::new(operation, model, topology, WfcJobConfig::default(), None, fixed)`.
7. **MapCommit** — one row per cell, masked cells skipped, pattern index → tile id.
8. **EncodeCommit** — pages `{"assignments":[[x,y,"tileId"],…],"contradiction":bool,
   "entropy":[[x,y,value],…]}` one chunk per admitted page.

`Grid2dInferenceCommit { assignments: Vec<(u32,u32,String)>, contradiction: bool,
entropy: Vec<(u32,u32,f64)> }`. Entropy is the PRIOR Shannon entropy of the tile-weight distribution,
`0.0` for a pinned or masked cell — honestly the pre-propagation value, not the narrowed per-cell
domain entropy a live "which cell collapses next" overlay would want.

Three `store::InferredField` impls ride the same job headlessly: `Grid2dSolve`
(`s.wfc.grid2d.inference.solve`), `Grid2dContradiction`, `Grid2dEntropy` (one step per cell).

**Two load-bearing findings about driving `WfcJob` from a parent job** — both cost real debugging:

- **An unsatisfiable grid is published as a FAULT, not as a commit.** `WfcJob`'s `Complete` stage
  emits `Publication::new(PublicationKind::Fault, b"wfc-unsatisfiable", …)`. A parent that only
  matches `StepOutcome::Complete` turns a legitimate contradiction into an error. This facet matches
  the fault detail and converts it into the `contradiction: true` verdict its commit is contracted to
  carry. (Assembly's `AssemblyContradiction` instead reads `solve_with_job(..).is_ok()` — same fact,
  read the other way round.)
- **`InteractiveJob::terminal_is_empty` must report OWNERSHIP only, never gate on the job's own
  `closing` flag** (A3's finding, applied here): the framework calls `begin_close()` itself at close
  stage 0 and answers `Blocked` for as long as `terminal_is_empty()` is false
  (`🧵️job/🦀️.rs:2724`), so a `closing`-gated predicate makes every `while !terminal_is_empty()`
  driver spin. The `closing` field is gone from this job; both headless close ladders carry a
  bounded guard assert instead.
- **Every `RetainedJobPayload` and every child job must be closed before it is dropped.** Dropping
  the child's `CommitCandidate.output`, or dropping the `WfcJob` itself with `self.child = None`,
  panics inside `RetainedJobPayload::drop` with *"requires one-page close to terminal-empty"* — and
  the panic then wedged the whole test binary (four tests "running for over 60 seconds"). The fix
  uses the engine's own production helpers: `job::retire_outcome(&mut outcome)` then
  `job::close_job(&mut child)`. **Assembly's `AssemblyInferenceJob` has the identical latent bug at
  `🧬️schema/💡️inferences/🦀️.rs` Solve/Restore arms** — worth fixing there, or at least not copying
  forward into wfc2d/wfc3d/grid3d/bitmap.

---

## 4. Windows and scenes

`edit` mode, `create_default_layout([grid, preview], "row", [50, 50])`.

**`wfc-grid2d-grid`** — `SurfaceKind::Board2d`, icon `layout-grid`, body key `wfc.grid2d.grid`.
`fixture_json` carries one rectangle node per cell (`cell-<x>-<y>`, centred in its cell,
`nodeKind` ∈ `open`/`pinned`/`masked`, `text` = the pinned tile id); `glyph_catalogs_json` carries one
`nodeKinds` row per state so a hole and a pin never read as the same square;
`grid_snap_enabled: true` and `grid_factor` = the authored `cellWidth` (falling back to it when the
window config still holds `0.0`), so a board click is a pure integer division into one cell.
Utilities `select` / `pin` / `mask` (icons `mouse-pointer` / `lock` / `eraser`, all verified against
`IconName`), armed per WINDOW INSTANCE through the five-step
`active_utility_by_window_id` → focused → flat → `select` fallback.

**`wfc-grid2d-preview`** — `SurfaceKind::Canvas2d`, icon `preview`. Draws each SOLVED cell's tile
media into its cell rect: a vector tile becomes one `segments` layer per path with a
`[cellWidth, 0, 0, cellHeight, originX, originY]` transform (unit tile space → cell rect); a bitmap
tile becomes one filled rect per pixel while `width*height ≤ 64`, else one swatch of the
palette-weighted average colour; an `image` child becomes a labelled placeholder, because a composed
child is not resolvable inside the guest render pass. With no cached solve it draws the bare cell
outlines — the honest "nothing inferred yet" state.

The layer contract is the host's, not a plausible-looking shape: `Canvas2dHost` branches on
`layer.segments` first, then `layer.kind === "image"`, then `polyline`, then `layerBounds`, and a
bounds layer needs explicit `x`/`y`/`width`/`height` plus a `#`-prefixed `color` or it falls through
to a hue ramp.

**Viewer** — one read-only `wfc-grid2d-view-preview` Canvas2d pane: every unmasked cell's outline
plus every PINNED cell's media. It boots the same committed example the editor does.

### State tiers

- **Document** — the problem only.
- **Per-window-instance config** (`Grid2dWindowConfig`, one owner per pane): camera, grid chrome,
  `active_tile_id` (what `pin` paints), and `solve_json` — the last `Grid2dInferenceCommit`, written
  ONLY by the `solve` command. It is a derived cache in WINDOW CONFIG; `Grid2dSnapshot` never gains
  an assignment field, so the "the solve is an inference, never persisted" law holds.
- **Per-window transient** (`Grid2dWindowTransient { hovered_cell }`) — per-frame scratch that never
  reaches the document.

### Commands (21, all `Migrated`)

The 14 mutation verbs, plus `pick-cell` (one board click resolved against the ARMED utility: `pin`
writes the active tile, `mask` toggles the hole, `select` emits nothing), `set-active-tile`,
`set-camera`, `set-grid-visible`, `set-grid-snap-enabled`, `set-grid-factor`, `solve`.
`command_from_action` is typed against `dsl::DslValue` (never `serde_json::Value`) and refuses an
unknown id rather than silently defaulting.

---

## 5. Examples

- **`pipes`** (`🚰️pipes`, icon `network`, seed 7) — 6×6, 32×32 cells, five VECTOR tiles whose rotation
  variants are separate tiles (`empty`, `pipe-h`, `pipe-v`, `elbow-ne`, `elbow-sw`), each declaring a
  `[left, right, top, bottom]` connector mask. Rules are generated from the masks for every ordered
  pair whose facing connectors agree, in the two canonical directions only (the inference mirrors
  them). One pinned cell, one masked cell.
- **`terrain`** (`🏜️terrain`, icon `map`, seed 11) — 8×8, 24×24 cells, periodic X, three BITMAP tiles
  (`grass` / `sand` / `water`, each a 4×4 two-colour indexed bitmap), transition law "water only ever
  touches sand", two pins.

Both state their document in Rust and PRINT the DSL text the picker loads — no second, drifting
`🗣️.dsl.semio` asset.

---

## 6. Tests

| suite | what it holds |
|---|---|
| `🧪️tests/🔬️unit` (artifact root) | identity, the OS-kind/dialect namespace split, `definition()`, dsl/pack/json round trips |
| `🧪️tests/🔬️store-fixture` | every example survives the pack envelope; `decode_into` hands back a bounded retirement that reaches terminal-empty; a starved step makes no progress but never fails |
| `🧪️tests/🧩️mount-contract` | every kind owns a complete triad directory, every fixture case owns a mounted test, every facet states all five language leaves, the oracle manifest and the replay triplet exist |
| `📸️snapshot/🧪️tests/🔬️unit` | addressing, bounds, cell rects, row-major ordering, direction algebra, base64 palette round trip + malformed-prefix behaviour |
| `🔺️diff/🧪️tests/🔬️unit` | sparse apply, index-exact insert, refusals, cell keying, absorb precedence both ways, every scalar lane |
| `🧬️mutations/🧪️tests/🔬️unit` | the kind roster against the enum, canonical-position helpers, op text+binary round trip, both cascades and their inverses, four refusal laws |
| 14 × `🧬️mutations/<kind>/🧪️tests/<case>` | the seven-assertion fixture shape (apply, inverse, canonical ×2, outcome, diff ×2) |
| `💡️inferences/🧪️tests/🔬️unit` | same seed → same assignment, a different seed still solves, empty rule set contradicts, no tiles contradicts, masked cells omitted, pins honoured, entropy map shape + exact prior value, the three `InferredField`s agree, routed metadata, five descriptor leaves |
| `✏️editor/🧪️tests/🔬️unit` | manifest role/dialect, both panes, 50/50 layout, localization, every mutation kind clickable AND `Migrated`, the `solve` verb on the preview pane, three utilities, every authored action id maps to a typed command, an unknown action is refused, command binary round trip, boot document, the five-step utility fallback |
| `🪟️window/🧪️tests/🔬️unit` | defaults, two distinct owners, config text+pack round trip and `record_spec`, transient round trip + absorb, config inverse, an unaddressed config write is refused |
| `🎭️modes/✏️edit/🧪️tests/🔬️unit` | mode localization, no unreferenced tool, layout names both panes, three distinct utilities |
| `🪟️windows/🔲️grid/🧪️tests/🔬️unit` | Board2d surface + registered icon, one node per cell for BOTH examples, cell-state classification, pin text vs masked, snap factor fallback, non-empty render for both examples |
| `🪟️windows/👁️preview/🧪️tests/🔬️unit` | Canvas2d surface + icon, unsolved → one outline per unmasked cell, solved vector → real path layers with 6-tuple transforms, solved bitmap → 64×16 pixel rects with `#`-hex colours, contradiction → bare grid, malformed cache ignored, non-empty render for both examples |
| `👁️viewer/🧪️tests/🔬️unit` | viewer role, one window, no mutation verb, same boot scene, inert command, non-empty render for both examples, pinned draws / masked does not |
| `📚️examples/*/🧪️tests/🧩️example` | shape, sorted collections, no dangling reference, media shape, text-is-a-print-of-Rust, deterministic solve (35 / 64 assignments) |
| `📚️examples/🧪️tests/🧩️outcome` | two distinct examples, each decodes with tiles and rules |
| `🧪️tests/🔲️mutate-grid2d-1/🦀️.rs` | walks the fixture tree OFF DISK, replays forward + inverse, asserts canonical JSON |
| `🧪️tests/🔲️mutate-grid2d-1/{🥒️.feature,🐍️.py}` | the declared `asset://` vector table and the Python second implementation that replays it |
| `🧬️schema/🧪️tests/🧩️suite/🟦️.ts` | the cross-language oracle: discovers the vectors on disk, re-applies every committed diff in TypeScript, checks tags/order/vocabularies |

The Python oracle (`$T/🐍️grid2d-oracle.py`) is a real second implementation of the document model and
of all fourteen diff/apply/inverse semantics; it never imports the Rust crate. `$T/🐍️grid2d-fixtures.py`
emits the quintets, the mounted tests, the feature file, the replay script, the oracle manifest and
the crate-root mount block from ONE case table. `$T/📜️grid2d-mutation-leaves.py` emits the 14 payload
/ manifest / payload-schema leaves. A repeat of the 12:41 tree move costs three commands.

---

## 7. Every command run

All foreground (detached with `nohup`+`disown` only to free the tool slot, never backgrounded past a
turn), `-j 4`, logs under `$T/🗑️generated/grid2d/`.

| command | result | log |
|---|---|---|
| `cargo check -p semio-s-artifact-wfc-grid2d --lib -j 4` | RED → 6 errors (4 unapproved verbs, 2 borrow) | `check-1.txt` |
| same, after the verb + borrow fixes | **GREEN** | `check-2.txt` |
| `cargo check … --features component-app-assembly --lib` | RED, peer breakage in `semio-framework-os-infinite` (`🌍️world/🦀️.rs:291` borrow) — not this crate | `check-3.txt` |
| `cargo check … --lib --tests -j 4` (no feature) | **GREEN** | `check-4.txt` |
| `RUST_MIN_STACK=33554432 cargo test … --lib -- --test-threads=4` | RED: 87 failures, ONE cause (direction wire spelling); 4 solve tests wedged by the payload-drop panic | `test-1.txt`, `test-2.txt` |
| single-test drive of `a_masked_cell_is_omitted` | surfaced `RetainedJobPayload requires one-page close to terminal-empty` | `test-solve-1.txt` |
| `cargo test … --lib -- --test-threads=4` (after both fixes) | **GREEN — 147 passed / 0 failed** | `test-3.txt` |
| `cargo check … --features component-app-assembly --lib --tests` | RED → 12 mechanical errors, then 4, then **GREEN** | `check-6/7/8.txt` |
| `RUST_MIN_STACK=33554432 cargo test … --features component-app-assembly --lib -- --test-threads=4` | RED 3, then **GREEN — 201 passed / 0 failed, 0 warnings** | `test-4/5/6.txt` |
| `cargo clippy … --features component-app-assembly --lib --tests` | 2 warnings, then **GREEN — 0 warnings from this crate** | `clippy-1.txt`, `clippy-2.txt` |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check … --target wasm32-wasip2` | **GREEN**, 0 errors / 0 warnings from this crate | `check-wasm.txt` |
| `python3 …/🧪️tests/🔲️mutate-grid2d-1/🐍️.py` | **GREEN — 14 vectors replayed, 0 failures** | — |
| `bun nx run @semio-tech/wfc-js:test` | **GREEN — 5 files, 113 tests; 22 of them this artifact's** | `ts-test-1.txt` |
| `bun ./📜️script.ts verify taxonomy report --scope "…/🔲️grid2d"` | **RED, pre-existing and unrelated** | `taxonomy-1.txt` |
| re-run of test + clippy + wasm after slice A3's three close-ladder findings and the example-icon fix | **GREEN — 201 / 0, 0 clippy warnings, wasm 0 errors** | `test-7.txt`, `test-8.txt`, `clippy-3.txt`, `check-wasm-2.txt` |

The taxonomy failure is the one slice P already logged: the gitignored, stale
`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📤️distribution/🧾️manifest.json` has unsorted
`inputs[]` pairs → *"Current compiler input manifest compiler inputs are not path-sorted"*. It throws
before any wfc row is examined and mentions no wfc path; regenerating it is a `framework-os-dev`
build owned elsewhere.

---

## 8. Known gaps

1. **The `image` tile media variant is declared but not composed.** `WfcTileMedia2d::Image` carries a
   real `store::ArtifactChild<SemioImageSnapshot>`, but the snapshot declares no `#[child(kind = …)]`
   field, so the host never hydrates it and both preview panes draw a labelled placeholder. Closing
   it needs the `Composed Child Load Requirements` contract (a `SemioMembers` roster,
   `genesis_child_pack`, `child_id == target.artifact_id`) — a packet of its own. Bitmap and vector
   tiles are fully real today.
2. **No `🔺️diff` Rust text/binary facet.** The diff rides the pack/JSON carriers; its `LanguageSpec`
   declares `grammar: None, protocol: None`, the same documented shape `🕸️dag` and `🎬️sequence` use.
3. **The preview's solve cache is per-pane window config**, so opening a second preview pane shows an
   unsolved grid until `solve` runs in it. An app-level config (or the `EphemeralEmit` transient
   lane) would share it; the window-config tier was chosen because it needs no new dispatch path.
4. **Bitmap tiles larger than 64 pixels draw as one averaged swatch** in both preview panes. A real
   raster path would need a `data:` URL (an `image` layer) minted in Rust.
5. **`🐍️.py` replay and `🥒️.feature` are ticket-local infrastructure**, not a committed repo gate:
   nothing in CI runs them yet. The Rust `🧪️tests/🔲️mutate-grid2d-1/🦀️.rs` walks the same tree and
   IS in `cargo test`.
6. **No playground boot.** Servers are the coordinator's to start; the `activate-wfc-react-dev` +
   serve + probe ladder for variant `grid2d` (ports 6042/6142) has not been run from this slice.
7. **The examples ship no `🟦️.ts` twin**, so the vitest `📚️examples/**/🧪️tests/🧩️example/🟦️.ts` glob
   matches nothing for grid2d (the `🧬️schema/🧪️tests/🧩️suite/🟦️.ts` oracle does run). Sibling
   artifacts do provide example twins, so the glob is satisfied plugin-wide.
