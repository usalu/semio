# 🧗️ WP-08 wall depth — hand-off for the editor and IO sub-agents (label `w2-wp08-walldepth`)

T = `C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️10\☀️08\BIM-PLUGIN`, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Read first: `T/r12-wave-brief.md`, `T/r11-finish-brief.md` (ALL hard rules: no Grep tool, no modifying git, gate for every cargo call, 256-char paths, laws, en+de labels,
accessible UI, concise code, no comments inside definitions, emoji docstrings, `[DEBUG] ` temp logs), `C:\git\semio\AGENTS.md`. The framework os-kernel / plugin crates may still
fail to compile (owner commit 677, agent `r11-store`); write code blind and verify with `cargo check … --lib` / `cargo test … --lib` as soon as it compiles. Files mapped by a
running build reject truncating writes on Windows (Errno 22 / os error 1224): use the Edit tool or retry; scripts must not `open(...,'w')` big shared files without retry.

## What exists now (the core is written; do not redo it)
Snapshot (generated from `T/r12-w2-wp08-model.ts` through `T/r3-f1-gen-model.ts`):
* `ModelSnapshot.wall_sweeps: BTreeMap<String, WallSweep>`; `WallSweep { host: wall id, side: WallSide{Left|Right} (looking along the axis start→end; left = interior face), profile: Profile,
  height: f64 (lowest point above the wall base), inset: f64 (moved into the wall), material: material id, name }`. Profile coordinates: `(out of the wall, up)` centred on the origin
  (`element_solids::profile_polygon`); a baseboard is `Rectangle{width:0.02 (out), depth:0.1 (up)}`.
* `Wall.base_slab: Option<String>` (base follows the TOP of that slab), `TopConstraint::{Roof{roof,offset}, Slab{slab,offset}, Ceiling{ceiling,offset}}` (top follows the UNDERSIDE of that
  element plus offset; only walls may use them, all other kinds refuse them), `Opening.reveal_depth: Option<f64>` (frame set back from the FRONT face; front = left face unless
  `flip_facing`), `Opening.reveal_material: Option<String>` (material of the jambs/head/sill inside the reveal).
* Mutations (leaf dirs under `S/🧬️schema/🧬️mutations/`): `create-wall-sweep{id, wall_sweep}`, `set-wall-sweep{id, host?, side?, profile?, height?, inset?, material?, name?}`,
  `delete-wall-sweep{id}`, `set-wall-base-slab{id, slab: Option<String>}`; extended `set-opening{.., reveal_depth: Option<Assigned<Option<f64>>>, reveal_material: Option<Assigned<Option<String>>>}`,
  `create-opening`, `create-wall`, `set-wall-top` (attach validation `mutations::wall_depth::{attach_flaw, sweep_flaw, reveal_flaw}`), deletes of roof/slab/ceiling refuse while a wall attaches
  (`mutation.target-referenced`), `delete-wall`/`delete-storey`/`delete-elements` cascade the sweeps, `split-wall`/`copy-elements` repeat them. Payload structs are in each leaf's `🦠️mutation/🦀️.rs`
  (`ModelMutation::CreateWallSweep(create_wall_sweep::CreateWallSweep{..})` etc., paths via `crate::mutations::<snake>::<Variant>`).
* Inference (all in `S/🧬️schema/💡️inferences/`):
  * `🧱️wall-layout/🧲️attach/🦀️.rs`: `AttachSurface` (graph node `ModelNode::Surface(id)`), `ElevationPoint{s,z}`, `AttachState{target,found,covered,clamped,cycle}`, `sample`, `apply`, `find_cycle`.
  * `WallLayout` (field `inference.wall_layout[wall]`) gained `base_profile`, `top_profile: Vec<ElevationPoint>` (empty = flat; absolute z in building coordinates over arc length `s` of the axis),
    `top_attach`, `base_attach: Option<AttachState>`, methods `top_at(s)`, `base_at(s)`; `base_z` = lowest base, `top_z` = highest top, `height` = their difference, `side_area/left_area/right_area/volume` exact.
  * `🧊️element-solids/🧷️wall-sweeps/🦀️.rs`: `inference.element_solids[sweep_id]` (family `SolidFamily::WallSweep`, one group `body`), `runs`, `path_length`, `section_of`, `extents_of`, `section_area`, `visible_perimeter`.
  * wall solids now follow `top_profile/base_profile`; `Cut` carries `reveal/material/facing_right`; groups with part `"reveal"` hold the jambs of an authored reveal.
  * `OpeningFrame` gained `setback: Option<f64>` (authored reveal depth clamped to the thickness), `reveal_material`, `facing_right`; the filler sits `face_front - setback - frame_depth/2`.
  * quantities: `QuantityKind::WallSweep` (key `wall-sweep`): `length` (path less interruptions), `width` = out of the wall, `height` = up, `perimeter` = visible outline, `gross_area` = cross-section,
    `net_area == surface_area` = visible outline × length, `gross_volume` = section × length, `net_volume` = solid volume, `mass`; attached walls have exact side areas/volumes.
  * diagnostics (en+de in `⚠️diagnostics/💬️messages`): `RefWallSweepHost`, `RefAttachTarget`, `WallAttachCycle`, `WallAttachUnreached`, `WallAttachCollapsed`, `WallSweepAboveWall`, `WallSweepNoRun`, `OpeningRevealDepth`.
* Terms used in labels: en "Wall sweep" / de "Wandprofil"; "Attach to roof" / "An Dach anbinden"; "Reveal" / "Laibung".

## Editor agent (label `w2-wp08-editor`) — report `T/r12-exec-w2-wp08-editor.md`
Everything under `S/✏️editor/**` (+ the mount rows it needs in the artifact root `A/🦀️.rs`, `A` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`). Pattern files: `🧩️entities/🔲️ceilings`, `🧩️entities/🛝️ramps`,
`🧵️gestures/🪟️opening` (host picking), `🪛️utilities/🦀️.rs`, `🎮️commands/🔃️flip-walls`, `📌️panels/🔍️properties`, `🗣️terminology/🦀️.rs` (`app_labels!`, every text en+de), `🧪️tests/⌨️completeness`.
1. Entities (`🧩️entities/🦀️.rs` + a new `🧩️entities/🧷️wall-sweeps/🦀️.rs` + tests): `top_text`/`parse_top` for the attach variants (`roof r-main 0.1`, `slab sl 0`, `ceiling ce 0`; the match is exhaustive and breaks the build today);
   wall rows `base_slab` (choices = slabs of the building plus "free", write `SetWallBaseSlab`) and `top_attach` (picker: roofs, slabs and ceilings of the building, plus free = back to storey top; keeps the offset; writes `SetWallTop`);
   opening rows `reveal_depth` (number, empty clears via `Assigned::new(None)`) and `reveal_material` (material choices + none); a new entity kind `wall-sweep` (library false, parent = host wall id, group label "Wall sweeps",
   fields name/host/side/profile/height/inset/material with text profile syntax `rect 0.02 x 0.1`, `circle 0.04`, `custom x,y; x,y; …`, create default = first wall + first material + baseboard profile, delete, rename, inferred rows length/area/volume).
   The existing law tests (`every_kind_creates_a_mutation_that_applies…` parent table, `every_create_and_delete_mutation_of_the_model_has_its_row`) must pass: add the parent arm for the new kind.
2. Sweep tool: utility row `"sweep"` (icon `minus`/`baseline`, group `structure`, `PLAN_WORLD`, key `shift+b`, arm `armSweep`), `ArmSweep` in `🎮️commands/🛠️arm-utility`, command table row + bridge + labels, a gesture
   `🧵️gestures/🧷️sweep/🦀️.rs` registered in `build_tool` (host search like the opening tool; side from the pointer; profile and material from the library selection with the baseboard default; ghost = the swept path along the face;
   one click writes `CreateWallSweep`; refusals through `ctx.accepts`), tests in the style of `🧵️gestures/🪟️opening/🧪️tests`.
3. Command `attachWalls` / kebab `attach-walls` (label "Attach to Roof", description en+de) payload `AttachWalls { ids: Vec<String>, target: String }`: walls among `ids` (else the selection) get `SetWallTop` to the roof / slab /
   ceiling `target` (else the one roof, slab or ceiling in the selection; refuse with a notice code `bim.attach.target-missing` when none), offset 0, one history row; plus the properties-panel action row "Attach to roof" on walls when
   the selection also holds a roof/slab/ceiling, and "Free the top" (back to `StoreyTop{0}`). Reachable by hotkey `shift+r` if free.
4. Outliner / properties / library / selection: sweeps appear under their wall (or in a "Wall sweeps" group), reveal and attach rows are accessible (labels on every control), the 3D window picks sweeps through `render::element_ids`
   (already includes them), `bim.*` fault notices for new refusal codes in en+de.
5. Verify: `cargo test … --lib editor::` once it compiles; the completeness laws; report counts.

## IO agent (label `w2-wp08-io`) — report `T/r12-exec-w2-wp08-io.md`
Everything under `S/🚪️io/**` and the IFC/glTF/SVG oracles in `S/🧪️tests/🏗️export-bim-1-ifc`, `🧊️export-bim-1-gltf`, `🎨️export-bim-1-svg` (+ their fixtures under `S/🧫️fixtures/🏗️ifc`). Pattern: `🚪️io/📤️export/🏗️ifc/🔲️ceilings`,
`🛝️ramps`, `🧭️frames`, `🚪️io/📥️import/🏗️ifc/🔲️ceilings`, `T/r3-recipe-io.md`, `T/r7-exec-z-codecs.md`.
1. IFC export (`IFC4`/`IFC2X3` as the exporter does today): walls whose `top_profile`/`base_profile` is non-empty export with a tessellated body (`IfcTriangulatedFaceSet`/the exporter's existing tessellated helper, as sloped ceilings do)
   instead of the extruded footprint, keep `Qto_WallBaseQuantities` from the exact layout values (`GrossSideArea`, `GrossVolume`, `Height` = max height) and add a Pset `Semio_WallAttach` { `TopTarget`, `TopKind` (Roof|Slab|Ceiling), `TopOffset`, `BaseSlab` }
   plus an `IfcRelConnectsElements` from the wall to the attached roof/slab/ceiling (description `TopAttach` / `BaseAttach`); sweeps -> `IfcMember` (PredefinedType `USERDEFINED`/`MULLION`-free: use `IfcMember` with ObjectType `WallSweep`),
   one per run, body = `IfcExtrudedAreaSolid` of the profile along a straight run, else the tessellated solid; Pset `Semio_WallSweep` { `Host`, `Side`, `Height`, `Inset`, `Material`, `Profile` } and `Qto` from the quantity row (`Length`, `CrossSectionArea`, `GrossVolume`),
   material via the existing layer/material mechanism; `IfcRelAggregates`/containment like other members. Openings with an authored reveal get Pset `Semio_OpeningReveal` { `RevealDepth`, `RevealMaterial` } and their
   filler placement already follows `OpeningFrame.setback`.
2. IFC import: `IfcMember` with `Semio_WallSweep` -> `WallSweep` (merge runs of one sweep id by the Pset `SweepId`), `Semio_WallAttach` -> `TopConstraint` / `base_slab`, `Semio_OpeningReveal` -> opening reveal fields. Round trip
   (export -> import) reproduces sweeps, attaches and reveals for the examples; validated with ifcopenshell (`.venv/Scripts/python.exe`, never `python -` on stdin) in the differential oracle `🏗️export-bim-1-ifc/🐍️.py`
   (counts per class, Psets, Qto vs the quantity table, kernel volume of an attached wall within 1e-6 of the layout volume).
3. glTF/SVG: solids flow through `SolidFamily::WallSweep` (names already added); add the sweeps to the SVG/plan export only if the plan linework draws them (it does not yet; leave).
4. Text/binary codecs: derived from the snapshot types; extend the codec round-trip tests with a model that has a sweep, an attached wall and a reveal. Regenerate the committed binary/text assets only through their bless tests
   (`BIM_BLESS=1`), and never touch fixtures of other packages.
5. Verify as soon as the crate compiles: `cargo test … --lib io::`, wasm32-wasip2 check, the ifc oracle.

## Shared rules for both
Do not edit the files owned by the core author unless a compile error blocks you (then a surgical fix, reported): `🧬️schema/💡️inferences/{🧱️wall-layout,🧊️element-solids/🧱️walls,🧊️element-solids/🧷️wall-sweeps,🕸️model-graph,⚠️diagnostics}`,
`🧬️schema/🧬️mutations/{🧲️wall-depth,🌊️cascade,🧵️elements}`, the four new leaf dirs. Peers (other W2/W1 agents, humans) edit shared files concurrently: re-read before every Edit, small edits, keep files compile-atomic.
