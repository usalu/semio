# r5 audit: BIM model inference tree and its consumers

Ticket `26/10/08/BIM-PLUGIN`. Read-only audit by static reading of the sources. Nothing was compiled or run; timings quoted below are the ones recorded in the r4 execution reports and were not re-measured.

Abbreviations used in paths:

- `I` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences`
- `E` = `I/🧊️element-solids`
- `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `F` = `🧰️framework/🛍️products/💻️os/🔨️modules`

Line numbers refer to the files as they are on disk at the time of the audit.

## 0. Verdict

The storey-rooted design holds up on the points that matter most for correctness. The snapshot stores authored values only, dependency inputs are honest for every field I traced, the join graph and the family dependencies are complete, and the parametric laws are tested. Style rules pass: all 347 doc blocks start with an emoji, there are no comments inside definitions, and there is no `[DEBUG]`, TODO or stub.

The tree is not yet clean on performance, duplication and consistency:

1. Wall layout is recomputed O(W) per wall inside four consumers and in the field's own dependency input. That makes each pass O(W²) per storey.
2. The same derivations (wall layout, host extent, top constraint, opening frame, room, stair run, rectangle) exist in two to five places each.
3. Production never uses the per-entity cache or the diff gate. The editor recomputes whole fields. The design's "one wall, not all" claim is not in effect.
4. Only storey and wall/opening host edges are real DAG parents. Solids, plan, spaces and diagnostics parent on storeys only.
5. Openings are judged valid by the frames, but the solids silently drop some holes that the frames call valid. Quantities subtract openings the solids never cut, and double-count overlapping ones. IFC export uses a third formula.
6. Three inference access paths coexist: the editor session, the viewer render memo and IFC export. Each computes its own copy.

## 1. Findings

Priority: P1 = correctness or a design claim that is false today; P2 = duplication, cost or missing guard; P3 = hygiene.

| ID | Pri | Field | Issue | Location | Evidence | Fix |
|---|---|---|---|---|---|---|
| F01 | P1 | wall-layout, joins | `layout_of` rebuilds every band of the storey for each wall, and the field's `dep_input` repeats the same scan. Each pass is O(W) per wall, O(W²) per storey. The engine calls `dep_input` for every step even with the cache off. | `I/🧱️wall-layout/🦀️.rs:166`, `:188-195`, `:282`; `I/🧱️wall-layout/🔗️joins/🦀️.rs:264-304` (`join` scans all bands, and `contact` rescans); `F/💡️inference/🦀️.rs:455` | r4-exec-i-solids-walls: 500 walls, debug 1.6 s of which 1.5 s is `layout_of`; release 142 ms for the whole model, 145 ms of it wall-layout (reported, not re-measured) | Build the bands and the tip index once per storey (sweep on tips). Give the storey its own node or pass a bands map. `dep_input` should read the precomputed neighbour set. |
| F02 | P1 | all wall consumers | Wall layout is recomputed outside its field, each time per wall. That duplicates the `wall-layout` field and inherits F01. Design §5 wants layouts as parents. | `E/🧱️walls/🦀️.rs:175` (in `compute`); `S/🗺️plan-linework/🧱️walls/🦀️.rs:144`; `S/📦️bodies/🦀️.rs:300`; `S/⚠️diagnostics/📐️validity/🦀️.rs:79`; `S/🏠️spaces/🦀️.rs:102-105` (`joins::join` + `footprint` in `obstacles_of`) | Each call site re-derives footprint, height and joins from the same snapshot | Consumers take `WallLayout` from the wall-layout node (parent) or from `ModelInference.wall_layout`. Delete the `layout_of` calls outside the field. |
| F03 | P1 | opening-frames, solids, plan | Host extent is resolved five times, with two different mullion/curtain depths. The `Host` node does not take the `wall-layout` node as parent (design: "host wall layout"); it recomputes `top_of`, `offsets_of`, `thickness_of`. | `I/🪟️opening-frames/🦀️.rs:231` (`host_extent`), `:223` (`curtain_extent`), `:211` (`mullion_depth`), `:456` (Host node); `E/🧱️walls/🦀️.rs:28` (`wall_host`, calls `offsets_of` twice at `:37-38`); `E/🚪️fillers/🦀️.rs:104` (`host_of`); `E/🪟️curtain-walls/🦀️.rs:30` (`curtain_host`); `S/🏠️spaces/🦀️.rs:110` (mullion depth again via `profile_extents`) | Same HostExtent built in three modules; spaces and frames compute the mullion depth differently (Rectangle depth vs y-span of the polygon; equal today, not guarded) | One `host_extent` in one module. Its output is the `Host` node value. Solids and fillers read the `Host` node. Add a test that `wall_host == host_extent` and that mullion depth is one function. |
| F04 | P1 | storey-levels, solids, stairs, bodies | `TopConstraint` is resolved four times. The arithmetic is identical today. | `I/🧱️wall-layout/🦀️.rs:225` (`top_of`); `E/🦀️.rs:252` (`resolve_vertical`); `S/📦️bodies/🦀️.rs:146` (`vertical`); `I/🪜️stair-runs/🦀️.rs:173-177` (inline in `run_of`) | Four copies of the same match | Keep one `top_of` (wall-layout or storey-levels) and call it from all four. |
| F05 | P2 | solids, plan, bodies | Shared geometry helpers are copied: `rectangle` four times; stair flight outlines in three modules; profile outline in three modules; `segment_of` four times; `point`/`mark` six times. AGENTS.md wants repeated code kept close, and these are spread across modules. | `S/📦️bodies/🦀️.rs:257`, `:264-287`, `:124`, `:76`; `S/🗺️plan-linework/🪜️stairs/🦀️.rs:20`; `E/🪜️stairs/🦀️.rs:36`; `E/📐️plan-kit/🦀️.rs:52`; `E/🦀️.rs:290` (`profile_polygon`); `E/🏛️columns/🦀️.rs:19` (`profile_loop`); `I/🧱️wall-layout/🦀️.rs:121`; `E/🦀️.rs:244`; `I/🪟️opening-frames/🦀️.rs:267` | Copies diverge silently when one is fixed | One `geometry` helper per concept, placed next to its users, or in the framework geometry crate if it is domain-neutral. |
| F06 | P1 | element-solids (walls, fillers), opening-frames, diagnostics | Silent hole loss. Frames call an opening valid when `s_min >= -1e-9` and `z_max <= height + 1e-9`. The wall solid cuts only where `s_min > lo + 1e-9`, `z_max < height - 1e-9`, and inside the join-trimmed face extent. So a window flush with the wall end or top is `valid`, produces no diagnostic, and makes no hole. A window reaching into a trimmed corner is dropped the same way. Fillers check only `frame.valid`, so the frame is built with no hole in the wall. The r4 report says "judged invalid by the diagnostics agent"; the diagnostics do not judge it. | `I/🪟️opening-frames/🦀️.rs:339-347`; `E/🧱️walls/🦀️.rs:104` (`usable`), `:124` (`lo`/`hi` from joins), `:46-57` (`hosted_cuts`); `E/🚪️fillers/🦀️.rs:165`, `:174`; `I/⚠️diagnostics/📐️validity/🦀️.rs:157-160` (maps issues only); `LENGTH_EPS = 1e-9` in `F/📐️geometry/➗️vector/🦀️.rs` | Mismatch between two predicates for the same hole | One validity rule that includes the join-trimmed extent and the top, with one tolerance, computed in opening-frames as an issue (`OutsideTrimmedExtent`). Diagnostics report it. Walls, fillers and quantities use the same `valid`. |
| F07 | P1 | quantities | Quantities disagree with the solids on openings. `usable` keeps frames with `OutsideHostExtent`, `AboveHostTop`, `BelowHostBase` and `OverlapsSibling`, which the solids never cut. `opening_area` sums clipped areas per opening with no union, so overlapping siblings are subtracted twice. `net_volume` follows the same sum. | `I/🧮️quantities/🦀️.rs:225` (`usable`), `:234-236` (`openings_of`), `:241-251` (`wall_quantity`) | Test `an_opening_outside_its_host_is_clipped...` encodes the clipping, but not the mismatch with the solid; no test covers overlapping siblings | Quantities consume only the holes the solid cuts. Union the cut rectangles per host before summing. Add a test: two overlapping windows, and a window at the wall end. |
| F08 | P2 | IFC export (not the field) | Third formula for the same wall. `NetSideArea` and `NetVolume` use the raw `width*height` of every typed opening, with no clipping, no validity and no arc scaling. `NetVolume = layout.volume - opened*thickness`. Quantities use per-layer volumes with `scale_at` for arcs. | `S/🚪️io/📤️export/🏗️ifc/🧱️walls/🦀️.rs` `emit` (the `areas` loop over `model.openings`) and the `Quantity::Volume("NetVolume", …)` line in `wall` | Straight walls agree; arc walls, invalid openings and overlapping openings do not | IFC reads `inferred.quantities.elements[wall]` and the frames. Delete the local formulas. |
| F09 | P2 | IFC export | Export runs its own full inference: five independent engine runs (levels, wall layouts, solids lazily, spaces, curtain layouts), none shared with the editor. | `S/🚪️io/📤️export/🏗️ifc/🦀️.rs:111-112`, `:123`; `S/🚪️io/📤️export/🏗️ifc/🏠️spaces/🦀️.rs:15`; `S/🚪️io/📤️export/🏗️ifc/🪟️curtain/🦀️.rs:14` | Each `compute_*` is `infer_field(snapshot, None)`, so nothing is shared | Call `ModelInference::infer(model)` once per export and read the fields. |
| F10 | P1 | editor, viewer | Three inference access paths. The editor uses the `ModelInferenceSession` (`S/✏️editor/🧵️inference/🦀️.rs`). The viewer uses the thread-local `render::solids` and `render::plans` memos (`S/🖼️render/🦀️.rs:129`, `:134`). IFC export builds its own. Element solids and plan linework are computed separately by the editor and by the viewer. The `render` module doc says `render::solids`/`plans` are "the only inference reads a window needs", but the editor's world, plan and section windows read the session instead. | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️world/🦀️.rs:61-62`; `S/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️world/🦀️.rs:55`; `S/👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️plan/🦀️.rs:48` | Two sources of truth and duplicate compute | One cache type, one instance per snapshot, shared by editor, viewer and export. The render memo goes. |
| F11 | P1 | all fields | Incremental recompute is not in production. Every `compute_*` calls `infer_field(snapshot, None)`. `infer_field_after_diff` and `InferenceCache` have no production caller; only tests call them (37 sites). The editor session gates on field reads and then recomputes the whole field. Moving one wall recomputes every wall layout and every wall solid. The design (§0.2, §5) and the r1 note say the opposite. | `I/🪜️storey-levels/🦀️.rs:104`; `I/🧱️wall-layout/🦀️.rs:315`; `I/🪞️curtain-layout/🦀️.rs:121`; `I/🪜️stair-runs/🦀️.rs:258`; `I/🏠️spaces/🦀️.rs:372`; `I/🪟️opening-frames/🦀️.rs:467`; `E/🦀️.rs:409`; `I/🗺️plan-linework/🦀️.rs:334`; `I/⚠️diagnostics/🦀️.rs:227`; `S/✏️editor/🧵️inference/🦀️.rs:28`, `:39`, `:70` | Recompute is per field, not per entity | Either wire a per-field `InferenceCache` through the session (and add a cache-transparency test per field), or correct the design claim. |
| F12 | P1 | engine, diagnostics, bodies | `dep_input` runs for every step even with the cache off, because the merkle hash needs it. The `Model` key serialises the whole snapshot on every infer. Storey scope clones whole type maps per storey (`wall_types`, `slab_types`, `roof_types`, `window_types`, `door_types`, `column_types`, `beam_types`), so any type edit invalidates every storey. | `F/💡️inference/🦀️.rs:455-456`; `I/⚠️diagnostics/🦀️.rs:300` (`to_value(snapshot)`); `S/📦️bodies/🦀️.rs:365-371` | Full-model serialisation per pass, even on a no-op | Hash only when the cache is enabled. Make `Model` depend on the collections it reads (its `reads`). Scope the type maps to the types the storey's elements use. |
| F13 | P2 | solids, plan, spaces, diagnostics | DAG parents are storeys only, not the layout, frame or room nodes that design §5 names. Only storey to wall-layout and storey to host to opening are real designed edges. Consequences: no reuse between fields (F02, F03), and whole-storey recompute. | `E/🦀️.rs:368-372` (plan); `S/🗺️plan-linework/🦀️.rs:371-373`; `I/🏠️spaces/🦀️.rs:330-333`; `I/⚠️diagnostics/🦀️.rs:284-292` | Parents list storey keys | Parent solids on the wall-layout and opening-frames nodes, plan on layout, frames and solids. If the engine cannot read another field's value, document the limit in the design. |
| F14 | P2 | element-solids (roofs), diagnostics | Roof fallback is hidden. The roof module says "limits are documented, never silent", and reports `roof.fallback-flat.*` in `RoofGeometry::fallback`. No diagnostic code or message exists for it, so a curved, concave or invalid-pitch roof silently becomes flat. | `E/🏠️roofs/🦀️.rs:8`, `:26-41`, `:70`; `S/⚠️diagnostics/🦀️.rs` (no fallback code); `S/⚠️diagnostics/💬️messages/🦀️.rs` (no row) | Grep for `fallback` in the diagnostics tree returns nothing | Add a `Warning` diagnostic per fallback, from the roof solid's fallback, and an en/de row. |
| F15 | P2 | editor section window | Section cuts are computed at render time, outside the inference catalogue. `section_plane(&solid.mesh(), …)` runs for every crossing solid on every redraw, and `solid.mesh()` copies the whole triangle list. | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️section/🦀️.rs:72-94`, `:86`; `E/🦀️.rs:148` (`mesh()`) | Not in r2 §5 catalogue | Add a `section` inference (per storey and line) with the same DAG, or memoise by (snapshot, line). |
| F16 | P2 | mutation diffs | Two mutation diffs run the whole storey-levels engine to validate a column top. Leaves are meant to read base only, not run inference. | `S/🧬️schema/🧬️mutations/🏛️create-column/🔺️diff/🦀️.rs:5`, `:15-16`; `S/🧬️schema/🧬️mutations/🎛️set-column/🔺️diff/🦀️.rs:5`, `:20` | Full-model engine per mutation | Use a pure elevation function (`resolve` over `stacking(base, building)` for the one target storey). |
| F17 | P2 | plan-linework, spaces | Rooms are computed twice. The plan's `draw_spaces` calls `rooms_of`, which re-runs the boolean arrangement that the spaces field already produced. `finish()` also computes a ceiling the plan never draws. | `S/🗺️plan-linework/📍️annotations/🦀️.rs:35`; `I/🏠️spaces/🦀️.rs:238-246`, `:266-272` | Two boolean arrangements per storey | Plan reads `ModelInference.spaces` (outline, holes, point). |
| F18 | P2 | solids (walls), opening-frames | O(n·m) scans per pass. `hosted_cuts` scans every opening for each wall and calls `frame_of` for each, which scans every opening again (`sibling_cuts`). `sibling_cuts` runs once per opening in `dep_input` and once in `frame_of`. | `E/🧱️walls/🦀️.rs:46-67` (`hosted_cuts`, `hosted_dependency`); `I/🪟️opening-frames/🦀️.rs:203-205`, `:348`, `:447` | O(W·O + O²) per pass | Build one `BTreeMap<host, Vec<opening>>` per field run and pass it to the frame and dependency functions. |
| F19 | P2 | spaces | `obstacles_of` rebuilds every wall's join and footprint (F02). `finish` runs one region boolean per room and per overlapping obstacle, and `rooms_of` runs one more boolean per storey. | `I/🏠️spaces/🦀️.rs:101-108`, `:238-246`, `:266-272` | Booleans scale with rooms times obstacles | Reuse `wall-layout` footprints and index obstacles by bounding box. |
| F20 | P2 | oracles | Oracle gaps. (a) `curtain-layout` has no oracle table; its numbers are checked only through the closed forms in `🧊️infer-bim-1-wall-solids/🐍️.py:357` and the three.js meshes of `curtain-grid`. (b) Arc walls have only Rust closed forms: the kernel oracle covers "every straight wall" (`🧊️infer-bim-1-wall-solids/🥒️.feature`). (c) Fillers (frame, muntins, glass, leaves) have no kernel check. (d) Diagnostics: the third-party oracle adjudicates 13 `ADJUDICATED` codes (`I/⚠️diagnostics/🦀️.rs:232-246`); the other 41 of 54 codes have no third-party table. | `S/🧪️tests/🧊️infer-bim-1-wall-solids/🥒️.feature`; `S/🧪️tests/🗺️infer-bim-1-plan-and-diagnostics/🥒️.feature`; `I/⚠️diagnostics/🦀️.rs:232` | Listed | Add a curtain-layout table to the levels case, a curved-wall kernel scenario, a filler kernel scenario, and oracle rows for the remaining diagnostic codes. |
| F21 | P2 | tests | Missing law tests: no gating test in `storey-levels` or `quantities`; no determinism or default law in `stair-runs` or `spaces` (only the aggregate root law covers them); no cache-transparency test for any field except element-solids (`a_warm_cache_equals_a_cold_recompute`). The gating test of `opening-frames` is at field level and is present. | `I/🪜️storey-levels/🧪️tests/🔬️unit/🦀️.rs`; `I/🧮️quantities/🧪️tests/🔬️unit/🦀️.rs`; `I/🪜️stair-runs/🧪️tests/🔬️unit/🦀️.rs`; `I/🏠️spaces/🧪️tests/🔬️unit/🦀️.rs` | Test names listed above | Add the four missing laws per field. |
| F22 | P2 | opening-frames | `sibling_cuts` (F18) and the opening `dep_input` both scan all openings; the frame's `tangent` guard (`hypot() > 0.5`, else `(1,0)`) silently gives a direction to a zero-length host that is not reported. | `I/🪟️opening-frames/🦀️.rs:354` | Silent fallback | Report `HostDegenerate` or refuse the frame. |
| F23 | P2 | editor session | The session compares and clones the whole snapshot on every change (`previous == snapshot`, `previous = Some(snapshot.clone())`), and `between` diffs it again. | `S/✏️editor/🧵️inference/🦀️.rs:70`, `:73`, `:75` | O(N) per change, plus a clone | Hold a revision counter or a diff from the mutation applier instead. |
| F24 | P2 | render | The render memo uses a single thread-local entry per function and compares snapshots by equality. Editor and viewer on one thread overwrite each other's entry. | `S/🖼️render/🦀️.rs:129`, `:134` | Thrash when both are mounted | Removed with F10. |
| F25 | P3 | element-solids (taxonomy) | `SolidFamily` and `SolidSource` duplicate eight variants. Voids and doors that fail a check return `SolidFamily::Window` builders, so an empty placeholder carries the wrong family. Empty solids are dropped, so this is cosmetic. | `E/🦀️.rs:38-66`; `E/🚪️fillers/🦀️.rs:156-157`, `:181` | Two enums for one concept | One enum, one mapping from source to family. |
| F26 | P3 | railings, stairs | Railing section sizes are constants in code (`POST_SIZE`, `RAIL_WIDTH`, `RAIL_DEPTH`), not authored or reported. Stringers and nosings are not modelled. Both are documented as limits, but not in diagnostics. | `E/🛤️railings/🦀️.rs` (constants); r4-exec-i-solids-rest §4 | Silent placeholder geometry | Add authored profile fields (schema-first) or a `Warning` diagnostic that the section is a default. |
| F27 | P3 | opening sill semantics | `Opening.sill` is added to the type sill for windows (0 means type default) and used alone for doors and voids. The rule is documented but not validated. | `I/🪟️opening-frames/🦀️.rs:186-194` | r4-exec-i-openings §6.5 | Make the authored sill explicit (`sill_override: Option<f64>`) or reject zero for windows. |
| F28 | P3 | IFC import | Import converts absolute z to authored `base_offset` using `import.levels`, computed once at the top (`S/🚪️io/📥️import/🏗️ifc/🦀️.rs:90`). Storeys must be imported first. The walls use `i.levels` at `S/🚪️io/📥️import/🏗️ifc/🧱️walls/🦀️.rs:100` and `:156`. | as cited | Ordering assumption, not checked | Assert the storey set before levels are computed, or compute levels per storey after import. |
| F29 | P3 | out of scope | Doc comments of the create-roof leaves contain literal escape text (`\u{1f53a}️`, `¶CreateRoof¶`) instead of the emoji and backticks. Probably a generator bug. | `S/🧬️schema/🧬️mutations/🏠️create-roof/🔺️diff/🦀️.rs:1`, and the create-roof `🦠️mutation` doc | Visible in the source | Check the generator for escape handling. |

## 2. What was checked and passes

Snapshot (`S/🧬️schema/📸️snapshot/🧱️entities/🦀️.rs`): no derived field is stored. Heights, elevations, lengths, areas and footprints are absent. The only elevations are `Site.elevation` and `Building.elevation`, and they are authored.

Mutations: no leaf writes an inferred value. The only inference reads in leaves are the two validations in F16 and `wall-layout::axis_length` in `🧬️mutations/📍️placement`.

Dependency honesty (checked field by field against `compute`):

- storey-levels: `dependency` covers level, height, building and site elevations. The building id is not in the record, but the storey's parent chain carries it.
- wall-layout: `dep_input` is the wall, its layers and the layers and records of the join neighbours. The neighbour set is the same function `compute` uses (`joins::join(...).neighbours`), so it covers Miter, Butt, Through and Cross contacts (checked against `contact`, `node_trim`, `butt_trim`, `crossings`).
- curtain-layout: explicit field list; covers everything `curtain_layout_of` reads.
- opening-frames: opening, window and door records, and the cut rectangles of its siblings. Host node: wall or curtain record, layers, building placement.
- element-solids families: each `dependency` covers the record and its type; walls and fillers cover all hosted openings and their types; the top-level `dep_input` adds building origin and rotation.
- stair-runs: the stair record. `run_of` reads nothing else.
- spaces, plan-linework, diagnostics: `storey_scope` and `authored` cover every collection the storey's rooms, plan and findings read (checked against `obstacles_of`, `ceiling_of`, `draw_*`, `references::storey`, `validity::storey`, `clashes::building`).

`reads()` is a superset of what each `compute` reads for every field, so `fields()` gating is not unsafe. The `reads()` lists are over-broad for `plan-linework` and `diagnostics` (whole type maps), which only costs time.

DAG: all `plan()` lists are topological (storeys in stacking order, then elements). The stacking chain has no cycles. No field has a parent in another building.

Style: 347 doc blocks, none starting with ASCII; no comments inside definitions; no trailing comments; no `[DEBUG]`, TODO, FIXME, stub or placeholder in the inference tree.

Joins: the Butt rule and the T-end tie-break (lowest id) are consistent between `contact` and the neighbour set, so a wall that starts or stops touching another changes its own dependency hash.

Clash search: sorted sweep on x, so it is O(n log n + k), not O(n²).

## 3. Parametric and gating coverage (item 4)

| Law | Covered by |
|---|---|
| Storey height → walls, columns, beams, slabs, roofs, railings, stairs, plan cut, space clear height, openings, quantities, curtains, diagnostics | `wall-layout`, `element-solids` (`changing_a_storey_height_*`, `a_storey_edit_re_infers_*`), `stair-runs`, `plan-linework` (`changing_a_storey_height_moves_the_cut_of_the_storeys_above_only`), `spaces` (`the_clear_height_follows_the_storey_height`), `opening-frames`, `quantities` (`a_storey_height_edit_follows_into_the_quantities`), `curtain-layout`, `diagnostics` |
| Storey height → elevations of storeys above | Only indirectly (wall-layout tests); no direct test in `storey-levels` (F21) |
| Wall axis → openings, joins, spaces | `opening-frames` (`moving_and_curving_the_host_axis_re_derives_every_frame_along_it`), `wall-layout` (`moving_a_wall_into_contact_changes_the_dependency_of_the_other`), `spaces` (`moving_a_wall_changes_the_room_area`), `diagnostics` (`moving_a_wall_into_a_column_creates_a_clash_and_moving_it_back_removes_it`) |
| Type thickness → footprints | `wall-layout` (`changing_one_wall_type_thickness_moves_its_neighbours_footprints_but_not_unrelated_walls`) |
| Gating (`infer_field_after_diff`) | Present for `wall-layout`, `element-solids` (+ `plan-kit`), `spaces`, `plan-linework`, `diagnostics`, `stair-runs`, `curtain-layout`, `opening-frames`; absent for `storey-levels` (aggregate test only) and `quantities` (F21) |
| Determinism and default | Present in most modules and in the aggregate root law; missing per module for `stair-runs` and `spaces` (F21) |
| Cache transparency | Only `element-solids::a_warm_cache_equals_a_cold_recompute`; the cache is not used in production (F11) |

## 4. Third-party oracle matrix (item 6)

| Field | Oracle case(s) under `S/🧪️tests/` | Covered | Not covered |
|---|---|---|---|
| storey-levels | `🪜️infer-bim-1-levels-and-wall-heights` | elevations, running sums per building | storey gaps, negative levels only in the house fixture |
| wall-layout | `🧱️infer-bim-1-wall-joins`, `🪜️infer-bim-1-levels-and-wall-heights` | footprints, joins (L, T, X, 3- and 4-way nodes, arcs), location lines | higher-order arc joins beyond tangent and right-angle |
| curtain-layout | none own; closed forms in `🧊️infer-bim-1-wall-solids/🐍️.py:357`; three.js `curtain-grid` | indirect | no table (F20a) |
| opening-frames | `🪟️infer-bim-1-opening-frames` (`placed`, `invalid`) | frames, issues, cuts, on line and arc hosts | curtain-wall hosts in the oracle; rotated building in the invalid case |
| element-solids (walls) | `🧊️infer-bim-1-wall-solids`, `🧊️infer-bim-1-solids-three` | straight walls with openings, layers | arc walls (closed form in Rust only, F20b) |
| element-solids (fillers) | three.js meshes of `straight-openings` | window and door meshes as blessed | no kernel check of frame, muntins, glass (F20c) |
| element-solids (rest) | `🧊️infer-bim-1-solids-rest` | columns, beams, slabs, stairs, railings (planar), volumes and bounds | roofs with pitch, curved columns (flattened) |
| spaces | `🏠️infer-bim-1-spaces` | rooms, areas, clear height, bounding walls | explicit arc outlines |
| quantities | `🧮️infer-bim-1-quantities` | wall, slab, column, beam totals | net volume with invalid or overlapping openings (F07) |
| stair-runs | `🪜️infer-bim-1-stair-runs` | flights, landings, compliance | none notable |
| plan-linework | `🗺️infer-bim-1-plan-and-diagnostics` (`house`, `curved`) | poché areas, eight metrics | styles, dashed members, stairs |
| diagnostics | same case (`clean`, `defects`) | 13 adjudicated codes | 41 of 54 codes (F20d) |

Fields with no oracle table: curtain-layout.

## 5. Consumers (item 1 and the consumer half of item 5)

- Editor (`S/✏️editor`): reads the session (`🧵️inference`), panels and windows read `ModelInference` fields only. The entity inspector reads quantities and storey levels. No derivation is written back to the snapshot. Derived geometry is computed in the section window (F15).
- Viewer (`S/👁️viewer`): reads the render memo (F10).
- IO (`S/🚪️io`): export recomputes everything (F08, F09); import converts derived geometry into authored values (F28); the text snapshot projection calls `ModelInference::infer` for oracle tables only.
- Examples (`S/📚️examples/🧰️checks`): shared check helper; `infer` runs inference twice to assert determinism. Its mount is not visible from the tree I read, so I did not rate it.

## 6. Prioritised fix list

1. Per-storey wall layout as a single computation with tip-indexed joins; consumers read the wall-layout node (F01, F02, F19, F18 part).
2. One host resolver, one top resolver, one opening validity rule with the join-trimmed extent; quantities, solids, IFC and diagnostics consume it; report dropped holes and roof fallbacks (F03, F04, F06, F07, F08, F14).
3. Wire the per-field cache through the editor session (or correct the design claim); hash dependencies only when caching; scope `Model` and type maps (F11, F12, F23).
4. One inference session for editor, viewer and export; delete the render memo (F09, F10, F24).
5. Real DAG parents for solids, plan, spaces and diagnostics, or document the limit (F13).
6. Add the section inference (F15) and remove the engine call from the two column diffs (F16).
7. Oracles: curtain-layout table, curved-wall kernel case, filler kernel case, diagnostic codes beyond the 13 adjudicated (F20).
8. Missing law tests (F21) and the shared helper cleanup (F05, F25).
9. P3 items as time allows (F22, F26–F29).

## 7. Not verified

- The engine internals beyond `F/💡️inference/🦀️.rs:455` (diff gating, budget and cancellation) were read only where the tree calls them.
- `joins` correctness (geometry) was not re-derived; only its dependency honesty was checked.
- No build, test or oracle was run. Timings are the r4 reports' figures.
- `storey_levels::parent_of` and `stacking` were not checked for callers beyond the ones listed.
