# 📐 Visual Editor Architecture

Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` (goal `norm`). Design only; no source was modified.
Scope: one shared, domain-neutral structural visual editor layer for all ten Eurocode artifacts (`⚖️en1990 … 🪶️en1999`) replacing the JSON-dump editors.

Shorthands used below:
- `NORM` = `✏️s/🔌️plugins/📕️norm`
- `SUBSET` = `🏅️standards/🔖️1/🪆️subsets/✳️any` (under `NORM/🗿️artifacts/<emoji>enXXXX/`)
- `SURFACE` = `NORM/📇️registry/🧬️contract/🖥️app-surface`
- `FW` = `🧰️framework`, `UI` = `FW/🔨️modules/🖱️ui`, `KITS` = `FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits`

---

## 0. Executive summary

1. The current editors are a generic value-tree walker (`render_document_editor` in `SURFACE/📝️document-editors/🦀️.rs`) over `Vec<…>` tables, plus a text report table. There is no geometry, no units beyond a suffix string, no selection link between a check and the thing it checks, and the whole report is recomputed (and the whole document re-serialised to JSON just to build a cache key) per change.
2. The framework already ships almost everything a visual layer needs: `World3d` scenes with scalar fields/legends, annotation layers (dimension/angle/marker/leader), sections with caps, highlight styles, a host-side raycast/marquee/gumball driven by an *interaction domain*, a per-object instance **delta lane** (`instances_delta_json`), `Canvas2d` scenes, a `GestureTool`/`ToolTransaction` statechart for streamed drags, transient/presence lanes, `latest-wins` async, operation progress and cancellation. fem3d is a working template (`🏗️fem/🗿️artifacts/🧊️3d/…/🎬️scene`, `✏️editor/🕹️interaction`).
3. What is missing is a **domain-neutral structural scene vocabulary** (members, plates, solids, fasteners, load glyphs, supports, utilisation, dimensions, diagrams), a **projection trait** (`snapshot + report → scene`), **typed unit-aware field specs**, **incremental evaluation** keyed by entity, and **handles that bind to field paths**.
4. Design: one new neutral plugin node `NORM/🎬️structural-scene` (schema-first, pure, no framework dependency except value derives), one new node `NORM/⚖️compliance/⚡️incremental-evaluation`, and a new set of shared windows under `SURFACE/🪟️visual-windows`. Each Eurocode artifact adds only a `🎬️scene` node (its projection) and wires four windows.
5. Realtime: edit/drag → transient preview (geometry-only, <8 ms) → commit as one `setFields` event → diff-driven selective re-evaluation (cheap checks <50 ms, rest streamed with progress and cancellation) → scene delta lane. Latency budgets in §5.7.
6. 15-agent plan in §9: two contract agents first, nine infrastructure agents in parallel, one EN 1993 reference vertical, then nine artifact verticals fanned out (agents reused), with non-overlapping directory ownership.

---

## 1. Current state (what was actually read)

| Area | Finding | Consequence |
|---|---|---|
| `SURFACE/📝️document-editors/🦀️.rs` | `render_document_editor<D: ToValue>` walks `DslValue`; objects/arrays become `tree_window_section_or_placeholder`; numbers are `ui::input(Number)` with `"{value} {unit}"` display text; `render_document_json` is explicitly "legacy". | Replace wholesale. No legacy kept (AGENTS.md). |
| `SURFACE/🔖️field-meta/🦀️.rs` | `NormFieldMeta { label_en, label_de, unit: Option<&str>, choices }`, path templates with `[]` wildcards, longest-prefix lookup. | Keep the path-template mechanism (it is good), extend the struct (§3.4). |
| `SURFACE/✏️edit-rules/🦀️.rs` | `NormEdit::{SetField, InsertItem, RemoveItem}` addressed by camelCase value path (`members[id=member-b1].length`), resolved per family to typed mutations via `SetFieldRule { path, kind, selectors, value }`. | The handle→field-path binding in §6 reuses this unchanged: a handle emits `NormEdit::SetField`. |
| `SURFACE/🪟️manifest-and-media/🦀️.rs` | `window_definition(id,label,body,icon)` hard-codes `SurfaceKind::Canvas2d`, no interactions/utilities. | Add `viewport_window_definition` (World3d + interaction ref + transform utility) and `diagram_window_definition`. |
| `SURFACE/🌳️report-tree` `render_report` | Tree of checks by part. | Superseded by checks window (§4.4). |
| EN 1993 `SUBSET/✏️editor/🎭️modes/✏️edit/🪟️windows/{📥️inputs,📊️results}` | `inputs::render` = `render_document_editor(document, locale, controller, Some(en1993_field_meta), windows)`; `results::render` = `render_report`. | Replace by `🧊️viewport`, `📈️diagrams`, `🧾️properties`, `✅️checks`. |
| EN 1993 `SUBSET/👁️viewer/…/🪟️windows/📊️report` | `TableWindowKit` over `report_table_rows`; viewer purity policy forbids importing editor modules. | Viewer windows must come from the **shared** surface crate, not from the editor (§4.6). |
| EN 1993 `SUBSET/✏️editor/🦀️.rs` | `ArtifactEditor` with `Transient = NoTransient`, `Presence = NoPresence`, six commands (`evaluate`, `setSelectedCheckIndex`, `setField`, `insertItem`, `removeItem`, `applyRemedy`), `NormHost::<En1993Family>::from_artifact(doc.snapshot.clone())` on **every render**. | Per-render snapshot clone; replace by a retained evaluation session (§5). |
| `NORM/⚖️compliance/🏛️norm-error-and-family` | `NormFamily::evaluate(&Document) -> CheckReport` is monolithic. `document_revision_key` JSON-serialises the whole document and hashes it per call; report cache is a process-global `Mutex<HashMap>` cleared at >64 entries; `NormHost::apply` blanks the report. | The real blocker for realtime. Replaced by mutation-sequence revisions + per-unit memo (§5). |
| `NORM/⚖️compliance/✅️check-results` | `CheckResult { id, part, clause: ClauseId, subject: SubjectRef{entity_id, path, label}, status, title, explanation, computed: Quantity, limit: Quantity, utilization: f64, annex, remedies: Vec<Remedy> }`; `Quantity{kind,value}` SI. | Already carries the entity link, path and clause that the visual layer needs. `subject.path` = field path of the governing input; used for colouring *and* for remedy ghosting. |
| EN 1993 snapshot (`SUBSET/🧬️schema/📸️snapshot`) | 819-line typed record: `materials, sections, members (length, buckling lengths, ltbRestraintSpacing, …), loadCases, memberActions (N,Vy,Vz,My,Mz,T), joints (boltClass, boltDiameter, boltRows …), fatigueDetails, fireExposures, coldFormedMembers, platedPanels, siloShells, tensionComponents, bridgeFatigue, towerLegs, piles, craneRunways`. SI units. | Plenty of geometry already (length, bolt diameter/rows, section dimensions); missing node positions/orientation — see §3.7 for the one schema addition (`placement`). |
| Entity collections of all ten (`grep Vec<` on snapshots) | 1990: permanents, variables, accidentals, seismics, members, bridgeSls, effects. 1991: floors, selfWeightElements, roofs, windFaces, accidentalCases, impact, explosion. 1992: concreteGrades, reinforcementGrades, prestressSteels, members, anchors, longitudinal. 1993: see above. 1994: beams, columns, slabs. 1995: members, connections. 1996: walls, concentrated, openings, loadCases. 1997: layers, footings, piles, retainingWalls, slopes, upliftCases, loadCases, testProfiles. 1998: buildings, bridges, assessments, silos, tanks, foundations, retainingWalls, towers, storeys, members, systems. 1999: elements, materials, sections, members, connections, fireScenarios, fatigueDetails, coldFormed, shells. | Mapping table in §3.6. |
| Framework `World3dScene` (`UI/🎬️scene/🎬️scenes/🌍️world3d`) | `camera_json, meshes_json, instances_json, instances_delta_json, selection_json, interaction_json, engagement_preview_json, domain_id, domain_granularity_id, snapshot lease, lanes`; builders `world3d_scene(...)`, `world3d_meshes_json_from_kinds`, `World3dScalarField`, `World3dAnnotationLayer`, `World3dSection`, `World3dHighlight`, `World3dModellingOptions`. | The 3D back-end. The structural scene is *lowered* into it (§3.5). |
| Framework interaction (`FW/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction`, fem3d `✏️editor/🕹️interaction`) | `InteractionDefinition { granularities, hierarchy, hover, selection }`; the **host** raycasts and dispatches `interactionSelect`/`interactionHover` with the raw instance id; transform gumball is armed by the `selection_json` record; `GestureTool` + `ToolMachineRunner` + `ToolTransaction` accumulate ticks in a transient and commit a net leaf once. | Direct manipulation (§6) is the fem3d pattern generalised. |
| `Canvas2dScene { camera, zoom, layers_json, framing, snapshot lease }` | 2D scene host. | Diagram window back-end (§4.2). |
| Taxonomy | Placeholders `🫧️transient/📌️.empty.md`, `👥️presence/📌️.empty.md`, `🎚️config/`, `☑️options/`, `🎬️actions/` exist under every mode and window node. | The slots for transient/presence/config are already laid out; fill, do not invent. |
| Path length | Deepest planned new file is 152 UTF-16 units / 220 UTF-8 bytes from repo root. | Rule for this plan: every new path ≤ 230 UTF-8 bytes (repo-relative). No tickets-folder copies of source. |

Open items I could not verify (flagged for the contract agents, not assumed): (a) whether `FW/🔨️modules/💡️inference` supports memoised incremental inference (the design in §5 works without it, via our own `CheckUnit` memo, and can adopt it later); (b) the exact `custom mesh` JSON accepted in `meshes_json` for non-primitive meshes (fem3d's `fem3d_solid_mesh_entries` is the working reference); (c) whether `Canvas2d` layers support text with locale-aware measurement (fem2d results window is the reference).

---

## 2. Layering and dependency graph

```
                    ┌────────────────────────────────────────────┐
                    │ NORM/🎬️structural-scene   (pure, leaf)      │
                    │  🧬️schema  🧮️tessellation  🔺️diff  🎨️colour │
                    │  🎯️projection (trait + builder + cache)     │
                    └───────────────┬────────────────────────────┘
                                    │
 ┌──────────────────────────────────┴──────────────────┐
 │ NORM/⚖️compliance (existing)                         │
 │   + ⚡️incremental-evaluation (CheckUnit, planner)   │
 └──────────────────────────────────┬──────────────────┘
                                    │
 ┌──────────────────────────────────┴──────────────────┐
 │ SURFACE (existing app-surface crate)                │
 │   🪟️visual-windows  🕹️direct-manipulation          │
 │   🫧️live-lanes  🔖️field-meta (extended)  🧾️inspector│
 └───────┬───────────────────────────────┬─────────────┘
         │                               │
  ✏️editor (per artifact)          👁️viewer (per artifact)
  imports 🎬️scene of same subset   imports 🎬️scene of same subset
         └──────────────┬────────────────┘
              🗿️artifacts/<x>/SUBSET/🎬️scene  (projection only)
```

Rules:
- `🎬️structural-scene` depends on nothing in the framework except `value_derive` (so it stays testable headless, hashable, and portable to TS/other languages via the schema files). The lowering to `World3dScene`/`Canvas2dScene` lives in `SURFACE/🪟️visual-windows`, not in the scene crate.
- Domain-neutral framework / domain-specific extension: scene crate knows *no* Eurocode. Eurocode knowledge lives only in the per-artifact `🎬️scene` projection and in `FieldSpec` tables. It is deliberately reusable by fem/bim/block later (promotion path: move node to `🧰️framework` or a shared plugin when a second consumer exists; no change in schema).
- Viewer purity: viewer and editor both import `🎬️scene` (a sibling of both) and the shared windows; neither imports the other.

---

## 3. Schema-first: the structural scene

All schemas are authored once per the existing snapshot convention: `🧬️schema/{🛰️.proto, 🔗️.graphql, 🟦️.ts, 🦀️.rs}` with generated/hand-synced twins and `🧪️tests` parity. Units: SI base only inside the schema (m, N, Pa, rad, K, s). Coordinates: right-handed, **Z up**, global frame; members carry a local frame. Text is `LocalizedCopy {en, de}` (never a single string; no default language), so scenes are **locale-free and cacheable**; the window picks the locale at lowering time.

### 3.1 Module tree (new)

```
NORM/🎬️structural-scene/
├── 🦀️.rs                       mounts, re-exports (explicit re-export policy for clients)
├── 📦️packages/🦀️rust/Cargo.toml  crate semio_s_plugin_norm_structural_scene
├── 🧬️schema/
│   ├── 🦀️.rs  🛰️.proto  🔗️.graphql  🟦️.ts
│   ├── 📐️geometry/🦀️.rs          Vec3, Frame, Polyline3, Polygon2/3 (with holes), Arc, Transform, Aabb
│   ├── 🧱️section-shapes/🦀️.rs    ProfileKind + params → Polygon2; Composite, RebarLayout
│   ├── 🧩️entities/🦀️.rs          SceneEntity, EntityKind, Geometry (the sum type), EntityRef
│   ├── 🏹️load-glyphs/🦀️.rs       LoadGlyph, Support
│   ├── 🎨️appearance/🦀️.rs        MaterialRole, ColourChannel, StyleState, Layer, ColourMapRef
│   ├── 🏷️annotations/🦀️.rs       Dimension, Label, Leader, Legend, FailureGlyph
│   ├── 🕹️handles/🦀️.rs           Handle, HandleKind, FieldBinding, HandleDrag
│   ├── 📈️diagram2d/🦀️.rs        Diagram2d, Axis, Series, Marker
│   └── 🔺️scene-diff/🦀️.rs        SceneDelta, EntityPatch
├── 🧮️tessellation/ (see §3.8)
├── 🎯️projection/ (see §3.9)
├── 🔺️diff/🦀️.rs                 diff(a,b), apply(delta, a)
├── 🎨️colour-maps/🦀️.rs          ramps, domains, CVD-safe palettes, pattern redundancy
├── 🔮️oracles/🐍️.py               independent Python scene/geometry oracles
└── 🧪️tests/ 🥒️.feature, 🔬️unit, 🧫️fixtures
```

### 3.2 Core types (Rust form of the schema; proto/graphql/ts mirror 1:1)

```rust
/// 🎬️ A complete, locale-free picture of one artifact at one document+report revision.
pub struct StructuralScene {
    pub schema: &'static str,                    // "semio.norm.structural-scene/v1"
    pub revision: SceneRevision,                 // (document_seq, report_seq)
    pub bounds: Aabb,
    pub entities: Vec<SceneEntity>,              // stable order = document order
    pub annotations: Vec<Annotation>,
    pub legends: Vec<Legend>,
    pub layers: Vec<Layer>,                      // toggleable groups (members, loads, supports, dims, soil …)
    pub description: LocalizedCopy,              // one-paragraph text equivalent of the whole scene (a11y)
}

/// 🆔 Stable identity == document identity so a pick resolves straight to a field-path prefix.
pub struct EntityRef { pub kind: EntityKind, pub id: String, pub path: String }   // path e.g. "members[id=member-b1]"

pub struct SceneEntity {
    pub entity: EntityRef,
    pub geometry: Geometry,
    pub appearance: Appearance,
    pub utilisation: Option<UtilisationBinding>, // worst check id, ratio, status, per-station samples
    pub label: Option<LocalizedCopy>,
    pub accessible_name: LocalizedCopy,          // mandatory, validated non-empty en+de
    pub accessible_detail: LocalizedCopy,        // "HEB 240, S355, 4.00 m, utilisation 0.64"
    pub layer: LayerId,
    pub pick: PickPolicy,                        // Whole | SubParts(vertex|edge|face) | None
    pub handles: Vec<HandleRef>,                 // ids into the handle table (see §6)
    pub content_hash: u64,                       // over geometry+appearance+utilisation (diff key)
}

pub enum Geometry {
    Member(MemberGeometry),         // axis + section extrusion (beams, columns, bars, piles, tower legs, runways, stringers)
    Plate(PlateGeometry),           // flat plate/slab/panel: outline(+holes), thickness, offset, ribs/stiffeners
    Shell(ShellGeometry),           // silo/tank/cooling shell: generatrix revolved or parametric patch, thickness
    Solid(SolidGeometry),           // Prism | Box | Cylinder | Wedge | Mesh | StrataColumn | FootingBody | WallAssembly
    Fastener(FastenerGeometry),     // Bolt|Screw|Nail|Dowel|Stud|Rebar|Anchor|Weld|Tendon (+ pattern)
    Support(SupportGeometry),       // Fixed|Pinned|Roller|Spring|ElasticBed|Pile-head
    Load(LoadGlyph),                // Point|Line|Area|Pressure|Moment|Thermal|Inertial|WindField|Impact|Explosion
    Surface(SurfaceGeometry),       // soil surface, slip circle, interaction surface, failure plane
    Group(Vec<EntityRef>),          // combined selection unit (e.g. a bolt group, a joint)
}

pub struct MemberGeometry {
    pub axis: Path3,                              // Polyline3 | Arc | Spline (SI)
    pub frame: Frame,                             // x along axis, y,z section axes; roll angle
    pub section: SectionShape,                    // outline polygon source
    pub section_taper: Option<SectionShape>,      // tapered/haunched: linear interpolation along axis
    pub offsets: (Vec2, Vec2),                    // eccentricities at ends
    pub releases: [EndRelease; 2],                // visual glyph only
    pub stations: Vec<Station>,                   // optional sampled x ∈ [0,1] for scalar overlays
}

pub enum SectionShape {
    Parametric { profile: ProfileKind, params: Vec<(String, f64)> },  // IShape{h,b,tw,tf,r}, Rhs, Chs, Angle, Channel, Tee, Rect, Circle, Zed/Cee (cold-formed), AluExtruded(Polygon2)
    Polygon { outer: Polygon2, holes: Vec<Polygon2> },
    Composite { parts: Vec<(SectionShape, MaterialRole, Vec2)>, rebar: Vec<RebarLayout> },   // EN1992/1994
    ThinWalled { centreline: Polyline2, thickness: Vec<f64> },                               // EN1993-1-3, EN1999 cold-formed
}
```

`SectionShape::outline()` is the single canonical source for: the extruded mesh (3D), the 2D section sketch (stress-block diagram), the Python oracle comparison (area, I, W), and the inspector thumbnail.

### 3.3 Load, support, fastener, annotation vocabulary

```rust
pub enum LoadGlyph {
    Point   { at: Vec3, direction: Vec3, magnitude: Quantity, case: LoadCaseRef },
    Moment  { at: Vec3, axis: Vec3, magnitude: Quantity, case: LoadCaseRef },
    Line    { path: Path3, direction: Vec3, profile: LoadProfile /*Uniform|Linear|Stepped|Sampled*/, case: LoadCaseRef },
    Area    { polygon: Polygon3, direction: Vec3, intensity: Quantity, case: LoadCaseRef },
    Pressure{ face: EntityRef, zones: Vec<PressureZone{polygon, value: Quantity, label}> },     // EN1991-1-4 Cpe zones A–J, snow drift
    Thermal { target: EntityRef, delta_t: Quantity },
    Inertial{ origin: Vec3, direction: Vec3, masses: Vec<(Vec3, Quantity)> },                    // EN1998 equivalent lateral forces
    Impact  { at: Vec3, direction: Vec3, energy_or_force: Quantity },                           // EN1991-1-7
    Explosion{ centre: Vec3, pressure: Quantity, shape: PressureWave },
}
pub struct LoadCaseRef { pub id: String, pub kind: LoadKind, pub psi: Option<(f64,f64,f64)> }  // colour by kind
pub enum LoadScaleRule { LogLengthByMagnitude, Fixed } // glyph size is screen-space, magnitude only changes shaft length

pub enum SupportKind { Fixed, Pinned, Roller{normal: Vec3}, Spring{k: [Quantity;6]}, ElasticBed{k: Quantity, extent: Polygon3}, PileHead }
pub struct FastenerGeometry {
    pub kind: FastenerKind,                 // Bolt{class, preload}, Screw, Nail, Dowel, Stud{h,d}, Rebar{bar_dia, bend: Option<BendShape>}, Anchor, Weld{a_throat, kind: Fillet|Butt}, Tendon{profile: Path3, duct: Option<f64>}
    pub axis: Segment3,
    pub diameter: f64,
    pub head: HeadShape,                    // hex, countersunk, headed stud …
    pub pattern: Option<Pattern>,           // Grid{nx,ny,sx,sy} | Row{n,s} | Along{path, spacing} | Cage{stirrup spacing}
    pub hole_clearance: Option<f64>,
}
pub enum Annotation {
    Dimension { id, from: Vec3, to: Vec3, offset: Vec3, value: Quantity, binding: Option<FieldBinding> },  // dimension lines may be editable (§6)
    Label     { id, anchor: Vec3, text: LocalizedCopy, tone: Tone },
    Leader    { id, anchor: Vec3, text: LocalizedCopy, offset_px: [i16;2] },
    Clause    { id, anchor: EntityRef, clause: ClauseId },     // clause badge near the governing detail
    FailureGlyph { id, at: EntityRef, mode: FailureMode, extent: Option<Polygon3> }, // LTB wave, flexural kink, punching cone, slip circle, shear cone, block tear-out
}
pub struct DeformedShape { pub base: EntityRef, pub field: DisplacementField /*sampled per station/vertex*/, pub scale: f64 }
pub struct ModeShape    { pub base: EntityRef, pub index: u32, pub frequency: Quantity, pub field: DisplacementField, pub participation: f64 }
```

`DeformedShape`/`ModeShape` are `Geometry` modifiers on a base entity (`SceneEntity.appearance.deformation: Option<DeformationOverlay>`), so the undeformed ghost and the deformed shape coexist without duplicating entities. EN 1993 buckling modes, EN 1998 modal shapes, EN 1990 SLS bridge deflections, EN 1992 deflection use this.

### 3.4 Appearance, utilisation colouring, themes

```rust
pub struct Appearance {
    pub role: MaterialRole,       // Steel, Concrete, Rebar, Timber, Masonry, Soil{class}, Aluminium, Glass, Weld, Bolt, Load{kind}, Support, Neutral …
    pub channel: ColourChannel,   // Role | Utilisation | Status | Scalar{field_id} | Custom(rgb)
    pub style: StyleState,        // Normal | Hover | Selected | Dimmed | Ghost(remedy preview) | Error | Stale
    pub opacity: f32,
    pub hatch: Option<Hatch>,     // redundant non-colour cue (fail = diagonal hatch)
}
pub struct UtilisationBinding {
    pub check_id: String,         // governing (max-utilisation) CheckResult.id
    pub ratio: f64,               // CheckResult.utilization
    pub status: CheckStatus,
    pub per_station: Option<Vec<(f64 /*x/L*/, f64)>>,   // along-member utilisation → World3dScalarField on the member mesh
    pub stale: bool,              // evaluation pending for this entity (greyed ring, not hidden)
}
```
Colour mapping is **not** in the scene: scene carries ratio and role; `ColourMap` (§8) maps `ratio→rgb` per theme. Default utilisation map: perceptually uniform sequential 0.0–1.0 with a distinct diverging break at 1.0 (fail) and glyph + hatch redundancy so colour is never the only channel. `StructuralTheme` (config, per window) selects map, role palette, lighting, edge style, label density.

### 3.5 Lowering to the framework (in `SURFACE/🪟️visual-windows/🧊️viewport3d`)

| Scene concept | Framework target |
|---|---|
| Member/Plate/Solid/Fastener mesh | `meshes_json` (instanced by `MeshKey = hash(section outline, kind)`; members of equal section share one mesh, scaled along the axis), `instances_json` (transform, tint, id = raw entity id as in fem3d) |
| Instance delta | `instances_delta_json` `{base, revision, count, changed[], removed[]}` produced directly from `SceneDelta` |
| Utilisation along member / on shell | `World3dScalarField` (+ `World3dScalarLegend`, `World3dColorRamp` from `ColourMap`) |
| Dimension/Label/Leader/Marker/Clause badge | `World3dAnnotationLayer` (`World3dDimension`, `World3dMarker`, `World3dLeader`), `World3dTone` from Tone |
| Section cuts (soil, wall, slab) | `World3dSection` with `World3dSectionCap` |
| Hover/selection/ghost | `World3dHighlight`, `selection_json` (incl. gumball arming), `engagement_preview_json` for remedy ghost |
| Picking | `interaction_json` + `domain_id`/`domain_granularity_id`; sub-part targets via the pick-target lane (`SpatialPickTarget`) |
| 2D diagrams | `Canvas2dScene.layers_json` (+ table twin, §4.2) |

### 3.6 Per-artifact entity → visual map (what each projection owes)

| Artifact | 3D entities | 2D diagrams | Failure/overlay |
|---|---|---|---|
| EN 1990 | members/effects as frame with combination load glyphs (G, Q, W, S, A, seismic arrows coloured by kind, ψ-weighted), bridge SLS deflected shape | combination matrix bar chart (γ·ψ per action), ψ-factor table plot, SLS limit lines | governing combination highlight |
| EN 1991 | floors (imposed area loads, category colour), roofs (snow shape coefficient zones), wind faces (c_pe zones A–E as Pressure zones, q_p profile arrows), self-weight elements (volume colour by γ), accidental impact/explosion glyphs | q_p(z) profile, c_pe zone map, snow shape factors vs pitch, Load-duration/combination ψ chart | zone labels with clause badges |
| EN 1992 | beams/columns/slabs as Composite sections with rebar cage (longitudinal bars, stirrups, links), anchors (headed, group, cone), prestress tendons | M–N interaction polygon with demand point and utilisation ray, section strain/stress block (rectangular/parabola-rectangle), shear truss (cot θ), crack-width vs stress, deflection | punching cone (u1), shear plane, strut-and-tie, crack pattern |
| EN 1993 | members (I/RHS/CHS/angle/channel extrusions, haunches), joints (bolt groups, welds, end plates, gussets), plated panels (stiffeners), silo/tank shells, tension components (cables/rods), tower legs, piles, crane runways | buckling curves a0–d with λ̄ marker, M–N / My–Mz interaction, S–N (detail categories, log–log) with damage fraction, stress at fire temperature (k_y,θ vs θ), cross-section class chart, effective-width sketch | LTB wave, flexural kink, block tearing, plate buckling modes |
| EN 1994 | composite beam: steel section + concrete slab + profiled deck + studs (Stud pattern), columns (encased/filled), slabs (deck ribs) | plastic stress blocks (full/partial interaction), M–N polygon, shear connection degree η | slip arrows at interface, deck rib failure |
| EN 1995 | timber members (solid/glulam rects), connections: nails/screws/dowels/bolts patterns, steel plates, spacings and edge/end distances as editable dimensions | k_mod/load-duration chart, embedment strength, effective number n_ef, spacing diagram | splitting plane, row shear block |
| EN 1996 | walls (elevation with bond pattern, openings, lintels, flanges, concentrated load patches with 45° spreading), load cases | eccentricity diagram (top/mid/bottom), slenderness reduction Φ(λ), strength–eccentricity envelope | cracked zones, stress block under concentrated load |
| EN 1997 | soil `StrataColumn`s (layer colour by class, φ'/c'/γ labels), footings (bearing pressure patch, effective area), piles, retaining walls (active/passive wedges, anchors), slopes (slip circle), uplift cases, test profiles (CPT/SPT as depth plots) | bearing capacity chart (q vs B), earth-pressure diagram, CPT profile with layer overlay, pile settlement curve | slip surface, Meyerhof/Prandtl wedge, hydraulic heave zone |
| EN 1998 | buildings (storeys as slabs with mass glyphs and equivalent lateral forces), bridges, silos/tanks/towers (shells), foundations, retaining walls, mode shapes | elastic/design response spectra Type 1/2 + DE-NA with T1 marker, storey drift/force profile, behaviour-factor q chart, capacity curve | mode shape overlay, soft-storey highlight |
| EN 1999 | extruded aluminium sections (arbitrary `Polygon`), members with **HAZ bands** (reduced-strength regions along the axis), connections, cold-formed profiles, shells, fire scenarios | buckling curves for alu (classes A/B), HAZ factor ρ_haz chart, section effective thickness, S–N alu | HAZ extent, local buckling mode |

### 3.7 One required schema addition: placement

Several snapshots describe members by `length` only. To draw a structure (not a bag of members), each spatial entity needs a placement. Add to each artifact snapshot a typed `Placement { origin: Vec3, rotation: Rot, parent: Option<EntityId> }` per spatial entity (member, wall, footing, storey, …). `Placement` is added through the normal mutation vocabulary (`update-<entity>-placement`) in each artifact's `🧬️schema/🧬️mutations`. Defaults are **derived**, not stored, when absent (`LayoutRule`: members of a snapshot are laid out left-to-right with gap, walls on a line, storeys stacked) so every existing fixture still projects deterministically; as soon as the user drags an entity, a `Placement` is persisted. The projection documents its default layout in a golden test. (AGENTS.md: no migration scripts — absence is a valid state, so no migration is needed; existing fixtures are not rewritten.)

### 3.8 Tessellation (`🧮️tessellation/`, pure, no framework)

```
🧮️tessellation/
├── 🦀️.rs
├── 🧱️section-outline/🦀️.rs   SectionShape::outline() with fillets (r) tessellated by chord tolerance
├── 📏️extrude/🦀️.rs          outline × Path3 × Frame → MeshData (indexed, normals, optional per-vertex scalar slot)
├── 🔩️fasteners/🦀️.rs        bolt/screw/stud/rebar/weld/tendon primitives, instanced
├── 🪨️solids/🦀️.rs          prism/cylinder/wedge/strata/wall-with-openings (CSG-lite: polygon-with-holes extrusion)
├── 🏹️glyphs/🦀️.rs          arrow/moment-arc/pressure-wave/support meshes (unit meshes, screen-space scaled by host)
├── 🌐️shells/🦀️.rs          revolve generatrix, conical/cylindrical shells
└── 🧪️tests/…
```
`pub struct MeshData { positions: Vec<[f32;3]>, normals, indices: Vec<u32>, scalar_slot: Option<Vec<f32>>, aabb }`. `trait Tessellate { fn mesh(&self, tol: Tolerance, lod: Lod) -> MeshData; fn key(&self) -> MeshKey; }`. LOD ladder (Lod0 box impostor … Lod3 filleted) selected by device class (§8) and entity count (budget: ≤ 400 k triangles desktop, ≤ 120 k mobile).

### 3.9 Projection trait (`🎯️projection/`)

```rust
/// 🎯 Pure per-artifact mapping: snapshot + evaluation → scene. No I/O, no locale, no framework types.
pub trait StructuralProjection {
    type Document;
    fn schema() -> &'static str;                                                 // "semio.norm.en1993/v1"
    fn entity_kinds() -> &'static [EntityKindSpec];                              // kind id, EN/DE label, icon, collection path, pick granularity
    fn project_entity(&self, doc: &Self::Document, entity: &EntityRef, eval: &EntityEvaluation, ctx: &ProjectionContext, out: &mut SceneBuilder);
    fn project_global(&self, doc: &Self::Document, eval: &Evaluation, ctx: &ProjectionContext, out: &mut SceneBuilder);   // ground, axes, legends, global annotations
    fn diagrams(&self, doc: &Self::Document, eval: &Evaluation, focus: &Focus, ctx: &ProjectionContext) -> Vec<Diagram2d>;
    fn handles(&self, doc: &Self::Document, selection: &[EntityRef], ctx: &ProjectionContext) -> Vec<Handle>;
    fn entity_dependencies(&self, doc: &Self::Document, entity: &EntityRef) -> Vec<EntityRef>;   // for incremental re-projection (e.g. member → section, material)
}

pub struct ProjectionContext { pub detail: Lod, pub deformation_scale: f64, pub load_case_filter: LoadFilter, pub visible_layers: LayerSet }
pub struct EntityEvaluation<'a> { pub checks: &'a [CheckResult], pub worst: Option<&'a CheckResult>, pub stale: bool, pub stations: Option<&'a Stations> }

pub struct SceneBuilder { /* append-only, enforces unique ids, validates a11y names en+de, computes content_hash */ }
impl SceneBuilder {
    pub fn member(&mut self, e: &EntityRef) -> MemberBuilder;
    pub fn plate(&mut self, e: &EntityRef) -> PlateBuilder;
    pub fn solid(&mut self, e: &EntityRef) -> SolidBuilder;
    pub fn fastener(&mut self, e: &EntityRef) -> FastenerBuilder;
    pub fn load(&mut self, g: LoadGlyph);   pub fn support(&mut self, s: SupportGeometry);
    pub fn dimension(&mut self, d: Dimension);  pub fn label(&mut self, l: LabelSpec);
    pub fn utilise(&mut self, e: &EntityRef, ev: &EntityEvaluation);   // fills UtilisationBinding + Appearance.channel
}

/// 🧩 Runner: caches `EntityFragment` per (entity, content-key); produces the full scene or a delta.
pub struct SceneProjector<P: StructuralProjection> { /* fragment cache, dependency index */ }
impl<P: StructuralProjection> SceneProjector<P> {
    pub fn project(&mut self, doc: &P::Document, eval: &Evaluation, ctx: &ProjectionContext) -> StructuralScene;
    pub fn project_delta(&mut self, doc: &P::Document, changed: &ChangedSet, eval: &Evaluation, ctx: &ProjectionContext) -> (StructuralScene, SceneDelta);
}
```
Fragment cache key = `hash(entity value subtree, hashes of dependencies, EntityEvaluation hash, ctx.detail)`. Changing one member's length re-projects that member (plus entities declaring it as a dependency — e.g. its joints and dimension lines); all other fragments are reused, so the delta is minimal by construction. `SceneDelta { base: SceneRevision, revision, changed: Vec<EntityPatch>, removed: Vec<EntityRef>, annotations_changed, legends_changed }` maps 1:1 onto `instances_delta_json` for the 3D lane.

Entity-kind registry (`EntityKindSpec`) is the *single* source for: outliner tree, interaction-domain granularities (§4.1), `FieldSpec` groups, a11y names, and the "add entity" palette (insert command per collection).

---

## 4. Windows

All shared windows are in `SURFACE/🪟️visual-windows/` (one crate, existing `app-surface`), parameterised by an `ArtifactVisualSpec<P: StructuralProjection>` value that each artifact supplies (projection, field-spec table, interaction definition, controller id). The per-artifact window nodes are two-line shims (definition + render delegating to the shared function), exactly like today's `inputs.rs`/`results.rs` but 3 lines each.

```
SURFACE/🪟️visual-windows/
├── 🦀️.rs                          ArtifactVisualSpec, window ids/body keys, mount list
├── 🧊️viewport3d/🦀️.rs            render, lowering (§3.5), camera/section/layers measures, config owner
│   ├── 🎚️config/🧬️schema/…        Viewport3dWindowConfig (camera, sun, layers, colour map id, deformation scale, section planes, label density, units)
│   ├── 🧪️tests/…
├── 📈️diagrams2d/🦀️.rs           Canvas2d lowering, axis/ticks, hit-test, table twin
├── 🧾️properties/🦀️.rs            typed inspector (replaces render_document_editor)
├── ✅️checks/🦀️.rs                checks list + detail + remedies
├── 🌲️outliner/🦀️.rs              entity tree by EntityKindSpec (selection-synced)
└── 🧪️tests/…
```
Default layout (desktop): left column outliner (top) + properties (bottom); centre viewport3d; right checks (top) + diagrams2d (bottom). Mobile and tablet layouts in §8.4. Layout is declared through `WindowLayout`/`WindowLayoutStackNode` like `edit_mode::layout()` today, from one shared `visual_layout(device)` function.

### 4.1 3D viewport window — `norm-<variant>-viewport`

- `WindowKindDefinition` via new `viewport_window_definition(id, label, body_key, icon)` in `SURFACE/🪟️manifest-and-media`: `surface_kind: SurfaceKind::World3d`, `interactions: vec![InteractionRef(norm.<variant>)]`, `utilities: [transform]`, `actions`: `viewportSetCamera`, `viewportFrame`, `viewportToggleLayer`, `viewportSetColourMap`, `viewportSetSection`, `viewportSetDeformationScale`, `viewportSetLoadCase`.
- Body: `scene_surface(body_key, SurfaceKind::World3d, &world3d_scene)` (same call fem3d uses via `world_3d_surface`).
- Interaction domain `norm.<variant>` generated from `entity_kinds()`: granularities = entity kinds (+ `check` and `loadCase` for cross-selection), `HierarchyProvider::Flat` (tree comes from outliner), hover channels `["pointer","list"]` (so hovering a check row highlights the entity and vice versa), selection modes Single/Multiple, methods Pick/Rectangle, all four merge modes. This is verbatim the fem3d definition with generated granularities.
- Scene instances are keyed by **raw entity id** (host raycasts and dispatches `interactionSelect` with that id; no hit-testing in the guest).
- Overlays: legend (utilisation ramp with 100 % tick), dimension lines, clause badges, labels with density control, failure glyphs. A "stale" overlay shows a thin animated ring on entities whose evaluation is pending (not a spinner covering the model).
- Camera: orbit with `Viewport3dOrbit`; "frame selection" and "frame worst check" actions; saved views per window config.

### 4.2 2D diagrams window — `norm-<variant>-diagrams`

`SurfaceKind::Canvas2d` with `Canvas2dScene.layers_json` lowered from `Diagram2d`:

```rust
pub struct Diagram2d { pub id: String, pub title: LocalizedCopy, pub axes: [Axis; 2], pub series: Vec<Series>, pub markers: Vec<Marker>, pub annotations: Vec<DiagramNote>, pub focus: Option<EntityRef>, pub legend: bool, pub table_twin: DataTable }
pub struct Axis { pub label: LocalizedCopy, pub quantity: QuantityKind, pub unit: UnitRef, pub scale: Scale /*Linear|Log10|Sqrt*/, pub domain: Option<(f64,f64)>, pub ticks: TickRule }
pub enum Series {
    Polyline { id, points: Vec<[f64;2]>, style: Stroke, label: LocalizedCopy },       // buckling curves, response spectra, S–N, CPT profile
    Polygon  { id, ring: Vec<[f64;2]>, fill: Fill, hatch: Option<Hatch> },            // M–N interaction polygon, allowable regions
    Band     { id, lower: Vec<[f64;2]>, upper: Vec<[f64;2]> },                        // envelopes, HAZ, scatter bands
    Scatter  { id, points: Vec<Point{xy, tone, label}> },                             // demand points, test data
    Limit    { id, axis: AxisRef, value: f64, label: LocalizedCopy },                 // H/V limit lines (U = 1)
    Section  { id, shape: SectionShape, strain: Option<Profile>, stress: Option<Profile>, neutral_axis: Option<Line2> },  // stress/strain blocks over the cross section
    Ray      { id, origin: [f64;2], through: [f64;2], to_boundary: bool },            // utilisation ray origin→demand→boundary (U = |OA|/|OB|)
    Surface3 { id, mesh: MeshData },                                                  // optional 3D interaction surface N–My–Mz, shown in a small World3d sub-surface
}
pub struct Marker { pub id: String, pub at: [f64;2], pub shape: MarkerShape, pub tone: Tone, pub utilisation: Option<f64>, pub bound_check: Option<String> }
```
- Diagrams are a function of **(entity focus, report)**: selecting a member in 3D switches to its diagrams (interaction diagram, buckling curve, section stress block). A tab strip lists the diagrams available for the focus, ordered by governing utilisation.
- Direct manipulation in 2D is limited to *markers bound to fields* (e.g. drag the demand point is *not* allowed because demand is derived; dragging a spectrum corner period T_B is allowed when it is an NDP field). Binding declared via `Marker.binding: Option<FieldBinding>` and flows through the same handle machinery (§6).
- **Table twin**: every diagram carries a `DataTable` (series as columns) shown by a "Table" toggle and used as the accessible alternative (§8.2); the viewer's current `TableWindowKit` is reused for it.
- Hit-testing for 2D (hover tooltip, click series → select entity/check) uses `UI/🔨️modules/🎯️canvas-pick-targets`.

### 4.3 Properties / inspector window — `norm-<variant>-properties`

Replaces `render_document_editor`. Driven by the **selection** (interaction domain) and `FieldSpec` (§3.4/§4.3.1), not by a blind walk of the value tree. With nothing selected it shows the snapshot's top-level fields (annex, design situation) and the entity collection summary.

#### 4.3.1 `FieldSpec` (extends `NormFieldMeta`; same path-template lookup)

```rust
pub struct FieldSpec {
    pub label: LocalizedLabel,                       // EN/DE
    pub help: Option<LocalizedLabel>,                // one sentence; includes clause text pointer
    pub clause: Option<ClauseRef>,                   // opens clause panel
    pub kind: FieldKind,                             // Quantity{kind: QuantityKind, display_unit: UnitId, units: &[UnitId]} | Int | Bool | Choice(&[NormFieldChoice]) | EntityRef{to: EntityKind} | Text | Angle | Ratio | Percent
    pub range: Option<Range<f64>>,                   // hard bounds (SI) – validity range of the norm formula
    pub soft_range: Option<Range<f64>>,              // typical, drives slider extent
    pub step: Option<f64>, pub precision: u8,
    pub group: &'static str, pub order: u16,         // inspector section
    pub tier: Tier,                                  // Basic | Advanced | Expert
    pub visible_if: Option<Predicate>,               // e.g. only when jointKind == "bolted"
    pub ndp: Option<NdpRef>,                         // national-annex parameter badge (NA-DE value as default)
    pub handle: Option<HandleHint>,                  // how this field can be edited in the viewport (§6)
    pub affects: Affects,                            // Entities(kind list) | AllChecks – feeds incremental planner (§5.3)
}
```
- `UnitId`/`UnitSystem` live in `NORM/⚖️compliance/📐️quantities-clauses-and-text` (extend `Quantity`/`QuantityKind` with `UnitId`, conversion factors, parse/format). The document remains SI; conversion happens at the field boundary only (`FieldKind::Quantity` display unit → SI before the `setField` value is built). Users choose unit presets (SI engineering: mm/kN/MPa; SI base) in window config.
- `NormFieldMeta` is replaced by `FieldSpec`; the ten per-artifact `✏️editor/🏷️field-meta/🦀️.rs` tables are rewritten once (Phase 0 mechanical conversion by the owner of the field-spec WP, then content-completed by the artifact owners). No dual types.
- `FieldKind::EntityRef` gives reference integrity and the **entity graph**: `members[].sectionId → sections`, `memberActions[].memberId → members`, etc. The same table drives the incremental dependency graph (§5.3) and the "used by" chips.

#### 4.3.2 Controls (all via existing framework nodes: `ui::input`, `ui::select`, `Slider`, `Toggle`, `Stepper`, `Tabs`, `Table`, `Tree`, `Progress`)
- `QuantityInput` = number input + unit `Select`; commits on blur/Enter with `Trigger::Commit`; Esc reverts; arrow keys step (Shift ×10, Alt ×0.1); inline validation message + soft-range slider for the fields flagged `handle`; shows the SI value on hover.
- `ChoiceSelect` with localized labels (the current `choices` mechanism, mandatory EN+DE).
- `EntityRefPicker`: dropdown with thumbnail (section outline from `SectionShape::outline()`), "create new…" row.
- Arrays: editable grid (`Table` window kit, `set-cell` commands) with row selection ⇄ viewport selection; add/remove via `insertItem`/`removeItem`; reorder by drag.
- Per-field **live feedback**: if any `CheckResult.subject.path` equals/prefixes this field's path, render a mini utilisation bar and the governing check title (click → checks window). If a `Remedy.target.path` equals the field, show the `required` value chip ("≥ 22 mm") with one-click apply.
- NDP fields carry an "NA-DE" badge and a reset-to-annex-default action.
- Computed (inferred) values (from `💡️inferences`, e.g. A, I_y, section class) appear as read-only rows in the same group.

### 4.4 Checks window — `norm-<variant>-checks`

- Grouped by entity (default) / by clause / by part (toggle). Each row: status glyph (✓ ⚠ ✗ ∅ — never colour only), localized title, `ClauseId` link (opens clause panel; keyboard-activatable), `computed / limit` with units from `Quantity.kind`, **utilisation bar** (`Progress` element, 0–120 % scale, tick at 100 %, warn threshold marker from `warn_above`), stale shading while pending, spinner + cancel for running expensive units.
- Selecting a row sets selection = `SubjectRef.entity_id` in the interaction domain (viewport focuses, diagrams switch, inspector scrolls to `SubjectRef.path`). Hover bridges via the `list` hover channel.
- Remedies: chips per `Remedy` with the action text; **hover/focus previews a ghost** (apply `required` to a *copy* in the transient lane; viewport renders `StyleState::Ghost`; the delta check estimate appears in the chip: "U 1.12 → 0.91"); click raises existing `applyRemedy`.
- Summary header: `CheckReportSummary` (pass/warn/fail/NA, worst utilisation, per-part verdict chips) with filter toggles; "worst first" sort.
- Virtualised via the existing `NORM_LIST_VIRTUALIZE_THRESHOLD` windowing.

### 4.5 Outliner — `norm-<variant>-outliner`
Tree by `EntityKindSpec` (collections as groups, entities as rows with status glyph + worst utilisation). Row actions: add/duplicate/remove (`insertItem`/`removeItem`), focus, isolate (layer solo). Uses `tree_group`/`tree_item_with_action` as today.

### 4.6 Viewer
`👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report` is replaced by `🧊️viewport`, `📈️diagrams`, `✅️checks` shims calling the same shared render functions with `ReadOnly` (no handles, no gumball, no commands except camera/layer/colour-map). The table report survives as the *table twin* of the checks window and as the `report:out` media path.

---

## 5. Realtime loop

### 5.1 Lanes (AGENTS.md: persisted/ephemeral × local/shared)

| Lane | Content | Mechanism |
|---|---|---|
| Persisted shared | Document (snapshot), `Placement`, all field values | Store events (existing `*Mutation`, event-sourced) |
| Persisted local-only | Window config: camera, layers, colour map, units, theme, density | Window config owner (existing `norm_results_window_config_owner!` pattern → new `visual_window_config_owner!`) |
| Ephemeral shared | Presence: peers' selection, hover, camera ghost, **drag preview** | Presence lane (`👥️presence/🧬️schema`) |
| Ephemeral local | Hover, gizmo state, remedy ghost, in-flight evaluation state, pending drag ticks | Transient lane (`🫧️transient`) |

Schema (new, shared, in `SURFACE/🫧️live-lanes/🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}`):

```rust
pub struct NormTransient { pub drag: Option<DragPreview>, pub ghost: Option<GhostPreview>, pub eval: EvalProgress }
pub struct DragPreview   { pub handle: HandleId, pub writes: Vec<FieldWrite{path: String, value: DslValue}>, pub started_seq: u64 }
pub struct GhostPreview  { pub writes: Vec<FieldWrite>, pub source: GhostSource /*Remedy(check_id, index)|Hover*/ }
pub struct EvalProgress  { pub running: Vec<UnitId>, pub done: u32, pub total: u32, pub cancelled: bool }
pub struct NormPresence  { pub user: UserId, pub selection: Vec<EntityRef>, pub hover: Option<EntityRef>, pub camera: Option<CameraPose>, pub drag: Option<DragPreview> }
```
Each artifact's `ArtifactEditor` sets `type Transient = NormTransient; type Presence = NormPresence;` (replacing `NoTransient`/`NoPresence`). Peers render a coloured ghost of another user's `drag` — the multi-user story.

### 5.2 Pipeline

```
 pointer/keyboard/inspector edit
        │ (1) transient: DragPreview{writes}             ← no document event, no history
        ▼
 preview doc = apply_overrides(doc, writes)              ← in-memory, structural sharing
        │ (2) cheap path (<8 ms): SceneProjector.project_delta(preview doc, stale report)
        │     + Tier-0 CheckUnits (O(1) closed-form: section class, limits)    ← synchronous, ≤4 ms budget
        ▼
 viewport + inspector + checks (tier-0 rows live, rest "stale")
        │ (3) release / Enter: ONE command `setFields{writes}` → one Mutation event (one undo step)
        ▼
 store applies mutation → `SnapshotDiff` (🧬️schema/🔺️diff) → ChangedSet{paths, entities}
        │ (4) planner: affected CheckUnits = reverse-dependency closure(ChangedSet)
        ▼
 EvaluationSession (retained, one per open artifact)
        │ (5) run affected units: Tier-1 on worker, Tier-2 expensive streamed; latest-wins cancels superseded
        │     each finished unit → partial report merge → `EvalProgress` transient + report revision++
        ▼
 (6) SceneProjector.project_delta(new doc, ChangedSet ∪ entities of finished units) → SceneDelta
        → `instances_delta_json` lane + annotation/legend patches + checks/diagram re-render
```

### 5.3 Incremental evaluation (`NORM/⚖️compliance/⚡️incremental-evaluation/`)

```rust
pub struct CheckUnit { pub id: UnitId, pub subjects: Vec<EntityRef>, pub reads: ReadSet, pub tier: CostTier /*T0 closed-form <1ms | T1 <50ms | T2 heavy, progress*/, pub run: fn(&UnitInput, &mut dyn Progress, &CancelToken) -> Vec<CheckResult> }
pub trait IncrementalNormFamily: NormFamily {
    fn units(document: &Self::Document) -> Vec<CheckUnit>;        // `evaluate` becomes `units().flat_map(run)` (full = fold of units; tested equal)
}
pub struct EntityGraph { /* reverse references derived from FieldSpec.kind == EntityRef + `affects` */ }
pub struct EvaluationSession<F: IncrementalNormFamily> { /* memo: UnitId → (read-hash, Vec<CheckResult>), report, revision */ }
impl<F> EvaluationSession<F> {
    pub fn plan(&self, doc: &F::Document, changed: &ChangedSet) -> Plan;         // units to rerun, ordered by tier then proximity to selection
    pub fn run(&mut self, plan: Plan, cx: &mut JobContext) -> impl Stream<Item = UnitDone>;   // progress + cancellation via ⏳️operation-progress and ♻️cancellation
    pub fn report(&self) -> &CheckReport;                                          // merged, ordered, summary recomputed incrementally
}
```
- Unit boundaries follow what the artifacts already do: per-entity check functions inside `⚖️checks/🧱️member-checks`, `🔗️special-element-checks/🔩️joint-checks`, `…/⏳️fatigue-fire-and-thin-walled-checks`, `…/🏛️shell-and-structure-checks`. The artifact owner splits `evaluate` into units keyed `(check family, entity id)`; no unit may read a document path outside its declared `ReadSet` (enforced by a test harness that records accessed paths through a tracking `ToValue` view and fails on undeclared reads).
- Invalidation: unit re-runs iff its read-hash changed. Global fields (`annex`, design situation, NDP overrides) are in every unit's `ReadSet` by construction (cheap: unchanged most of the time).
- Replace `document_revision_key` (full-document JSON hash per call) with the store's **mutation sequence number** + per-unit read-hash. `NormHost::from_artifact` per paint is replaced by reading the retained session; `report_cache()` global mutex is removed.
- Cancellation: a new `setFields`/`setField` event or a drag tick cancels still-queued/running T1/T2 units that read changed paths (`latest-wins` combinator in `FW/🔨️modules/⏳️async/🥇️latest-wins`); finished independent units are kept.
- Progress: `EvalProgress{done,total,running}` surfaces in the checks window header, viewport "stale" rings, and the standard progress/cancel affordance (AGENTS.md: all expensive operations show progress and are cancellable). T2 examples: EN 1993 plate buckling / FE-like shell checks, EN 1997 slope slip-circle search, EN 1998 modal combination, EN 1992 M–N polygon generation at fine fibre resolution.
- Exposed as bounded tool jobs through the existing `norm_owned_tool_job_factory!` / `bounded_first_step_tool_proofs!` mechanism (new command `evaluateUnits` plus the existing `evaluate` = full plan).

### 5.4 Incremental scene diff
`SceneProjector` (§3.9) + `SceneDelta` (`🔺️diff`). Law (property-tested): `apply(diff(a,b), a) == b` and `diff(a,a)` is empty and contains zero entities for unrelated edits.

### 5.5 Coalescing and back-pressure
- Drag ticks are sampled at display rate, coalesced to the latest value per handle (drop intermediate), preview projection runs at most once per frame; the transient lane publishes at ≤30 Hz to peers.
- Evaluation requests are debounced 60 ms after the last tick for T1; T2 starts 250 ms after quiescence (or immediately on commit).
- Short connection loss: edits keep applying to the local store; presence/ghost lanes simply drop; no freeze (the UI never awaits the network). Reconnect replays events; long offline is not supported (AGENTS.md "SHOULD NOT accept long offline").

### 5.6 Frame budget
Projection and lowering must never allocate per-vertex for unchanged entities: fragments cache `MeshKey`s and instance transforms; only changed instances are re-emitted. Large meshes (soil strata, shells) use the framework's paged snapshot lease (`World3dSnapshotLease`) instead of 32 KiB surface docs (the failure mode fem3d documents in its `world_3d_surface` comment).

### 5.7 Target latencies (acceptance gates; benches via `repo bench`)

| Interaction | p50 | p95 | Notes |
|---|---|---|---|
| Drag preview → pixels (geometry only, T0 units) | 6 ms | 12 ms | desktop; mobile p95 24 ms; single entity changed, 1 000-entity scene |
| Field commit → inspector/check row updated (T0/T1) | 30 ms | 80 ms | one entity, typical snapshot |
| Field commit → all affected checks final | 120 ms | 400 ms | excludes T2 |
| T2 unit progress first tick | 100 ms | 250 ms | then streamed; cancellable within 50 ms |
| Selection (pick) → inspector/diagrams/checks synced | 16 ms | 40 ms | |
| Cold open of EN 1993 example → first scene frame | 250 ms | 600 ms | |
| Memory: scene fragments + meshes | ≤ 64 MB desktop, ≤ 24 MB mobile | | |

---

## 6. Direct manipulation: picking → selection → handles → commands

### 6.1 Picking and selection
Host raycast/marquee dispatches `interactionSelect`/`interactionHover` (domain `norm.<variant>`, granularity = entity kind, id = raw entity id; sub-part targets via pick-target lane for faces/edges of plates and bolt rows). `InteractionView::selection(domain)` feeds: outliner highlight, inspector contents, diagrams focus, checks filter, handle set. No second selection model.

### 6.2 Handles (`🕹️handles` schema, produced by `StructuralProjection::handles`)

```rust
pub struct Handle {
    pub id: HandleId,                       // "member-b1/length/end"
    pub entity: EntityRef,
    pub kind: HandleKind,                   // Translate{axis|plane} | ScalarAlong{axis} | Radial{centre} | Angular{axis,centre} | DimensionLine | CountStepper | FaceOffset
    pub anchor: Vec3,                       // world position of the grip
    pub screen_hint: GripStyle,             // arrow/diamond/ring; label shown while dragging ("L = 4.25 m")
    pub writes: Vec<FieldBinding>,          // what a drag does (below)
    pub snap: SnapRule,                     // grid, increments (50 mm), neighbour entities, standard catalogue values (bolt dia 12/16/20/24…)
    pub limits: Option<Range<f64>>,         // from FieldSpec.range, tightened by geometry (cannot pass the opposite end)
    pub accessible_name: LocalizedCopy,     // "Member B1 length handle"
}
pub struct FieldBinding {
    pub path: String,                       // concrete value path: "members[id=member-b1].length"
    pub map: Affine,                        // field = base + k · s   (s = scalar drag parameter in handle units)
    pub unit: UnitId,                       // display unit during drag
    pub quantise: Option<Quantise>,         // Round(step) | Choice(&[f64]) | Int
}
```
- `path` uses the same syntax as `NormEdit::SetField.path`; therefore it is validated by the existing `SetFieldRule` tables (`SURFACE/✏️edit-rules`). A property test asserts every handle's every binding resolves to a `SetFieldRule` and to an existing leaf in the snapshot.
- Handle sources per `FieldSpec.handle: HandleHint`: `LengthAlong(axis)`, `Offset(normal)`, `Count(stepper)`, `Radius`, `Angle`, `Thickness(face)`, `Spacing(axis)`, `PlacementOrigin` (gumball), `DimensionLine`. The projection author only *places* them; geometry-to-field mapping is in the binding.
- Multi-field handles (e.g. drag a plate edge: `width` + `placement.origin.x`; drag bolt-group spacing: `p1` and `rows`) list several `writes`.

### 6.3 Gesture → commands
- Statechart `HandleDragTool` (in `SURFACE/🕹️direct-manipulation`, built on `FW tool-machine` `GestureTool`/`ToolMachineRunner`, same as fem3d's gumball): `Idle → Hover(handle) → Armed → Dragging(s) → Commit | Abort`. Dragging updates `NormTransient.drag` (previewed by all windows and broadcast on presence); Commit raises **one** `setFields{writes}` (new bounded command) = one `Mutation` event = one undo step; Abort (Esc / lost capture) leaves zero trace.
- The transform utility (gumball) for whole-entity moves reuses fem3d's `selection_json` gumball arming; its net translation raises `setFields` on `placement.origin.*` for each selected entity.
- Keyboard equivalents: Tab cycles handles of the selection; arrow keys nudge by `snap` increment (Shift ×10, Alt ×0.1); Enter commits; Esc aborts; typed numeric entry while dragging (AutoCAD-style) overrides `s`.
- Insert: "Add member/wall/footing…" tools place entities by click (two clicks for members) → `insertItem` with a seeded `value` carrying `Placement` and catalogue defaults; the ghost follows the cursor in the transient lane.
- Dimension lines with `binding` are directly editable: click → inline numeric field in 3D overlay → `setFields`.
- Undo/redo: `mod+z`/`mod+shift+z` already registered; one drag = one entry.

### 6.4 Command additions (shared, in `SURFACE/⌨️command-args-and-dispatch` and `SURFACE/✏️edit-rules`)
`setFields { writes: [{path, value}] }`, `evaluateUnits { units?: [UnitId], reason }`, `cancelEvaluation`, `previewRemedy { checkId, remedyIndex }` (transient only), `clearPreview`, `setViewportOption`/`setDiagramOption` (window config). Each artifact's `app_commands!` enum appends rows (append is safe per the macro doc; no reordering) — done by the artifact owner during rollout.

---

## 7. Testing strategy (test-driven; language-agnostic; third-party oracles)

### 7.1 Layers

| Layer | What | Where |
|---|---|---|
| Schema parity | Rust ⇄ proto ⇄ graphql ⇄ ts describe the same scene schema; round-trip of fixtures | `🎬️structural-scene/🧬️schema/🧪️tests` |
| Geometry oracles | Section outline area/centroid/I vs `sectionproperties` (third-party Python) and catalogue values in snapshots; extruded mesh volume = A·L, watertight, normals outward, AABB (`trimesh`); polygon ops (`shapely`) | `🔮️oracles/🐍️.py`, `.feature` |
| Projection goldens | For each artifact fixture → `StructuralScene` canonical JSON equals committed golden; EN+DE a11y strings present | `<artifact>/SUBSET/🎬️scene/🧪️tests/🧫️fixtures/…/🔣️.json` |
| Scene laws (property) | ids unique; every entity `accessible_name` en+de non-empty; every check's `subject.entity_id` resolves to an entity or global; `apply(diff(a,b),a)==b`; project∘mutation delta minimal (only dependency closure changes) | `proptest` in Rust; `fast-check` TS twin using the same `.feature` data |
| Incremental ≡ full | For random mutation sequences, incremental report == full `evaluate` (bit-equal within f64 ε), cancellation leaves no partial/mixed state, undeclared-read detector | `⚡️incremental-evaluation/🧪️tests` |
| Handles | every `FieldBinding.path` resolves (snapshot leaf + `SetFieldRule`); `map` round-trips value↔s; limits respect `FieldSpec.range`; drag-commit-undo returns to the original snapshot | per-artifact + shared |
| Windows | UI-tree snapshot tests of each window for fixture + locale (en,de) (existing `…/🧪️tests/🔬️unit` pattern); keyboard-flow tests (Tab handles, nudge, commit) through `UiDriver`; targets-wgpu accessibility projection tests | shared + shim tests |
| Diagrams | Series numbers vs artifact Python oracles (`🔮️oracles/⚖️compliance/🐍️.py` already computes χ, resistances; add curve points), spectra vs EN 1998-1 §3.2.2.2 formula reimplemented independently, M–N polygon vs `shapely` containment of demand and ray-boundary ratio = reported utilisation | `.feature` |
| A11y/colour | Palettes CVD-safe (simulate deuteranopia/protanopia/tritanopia with `colorspacious`/`colour-science`), min ΔE between adjacent ramp stops, contrast ≥ 4.5:1 for text and ≥ 3:1 for graphical objects, no colour-only status | `🎨️colour-maps/🔮️oracles` |
| Perf | Benches for §5.7 numbers (`repo bench`), regression gate | `…/🧪️tests/⏱️bench` |
| E2E | Per artifact: open example → pick member → drag handle → check updated → undo; screenshot diff of viewport (headless wgpu) | `🧪️tests` of the artifact |

### 7.2 Language-agnostic feature (template; one per feature, tags follow the repo's `@capability-… @oracle-… @comparison-… @id-… @level-… @mode-…` convention)

```gherkin
@capability-structural-scene-projection
@oracle-python-trimesh-sectionproperties
@comparison-ordered-json-v1
Feature: Project an EN 1993 snapshot to a structural scene
  Scenario Outline: <fixture> member geometry matches the independent oracle
    Given the committed snapshot shared://…/🧫️fixtures/<fixture>/📸️snapshot/🔣️.json
    When the structural scene is projected with detail "lod2"
    Then the scene equals shared://…/🧫️fixtures/<fixture>/🎬️scene/🔣️.json
    And every entity has accessible names in "en" and "de"
    And the oracle reports for entity "member-b1" section area within 0.5% of catalogue area
    And the oracle reports the extruded mesh volume within 0.1% of area times length

  Scenario: Editing a member length changes only that member and its dimension
    Given the committed snapshot …
    When mutation "update-member-length" sets "members[id=member-b1].length" to 4.5
    Then the scene delta changes exactly the entities "member-b1,dim-b1-length"
    And utilisation of "member-b1" is stale until evaluation completes

  Scenario: A handle drag commits one undoable event
    Given a selection of "member-b1" and a drag of handle "member-b1/length/end" to s=4.5
    When the drag is released
    Then exactly one mutation event is recorded with write "members[id=member-b1].length" = 4.5
    And undo restores the original snapshot
```

### 7.3 Oracles (Python, `-I`, per repo rules; no runtime dependency — test-time only)
`trimesh` (volume, watertight, bounds), `sectionproperties` (area, I, W_el, W_pl, centroid), `shapely` (polygon area/containment, M–N polygon), `numpy` (spectra, curves), `colorspacious` (CVD), `pytest-snapshot` semantics reproduced by committed goldens (no new tooling in the Rust build). All oracle scripts live in `🔮️oracles` nodes and are invoked through `repo` test targets.

---

## 8. Accessibility, i18n, customisation, devices

### 8.1 i18n
Every label, help, clause note, accessible name, diagram axis/series label, legend title is `LocalizedLabel::native(en, de)` / `LocalizedCopy`. No default language: scenes carry both, the window picks `ViewModel.locale`. Number formatting is locale-aware (decimal separator, thin-space grouping) in `QuantityInput` and labels. A census test (like the existing `🏷️mutation-label-census`) fails the build when any new string has an empty `en` or `de`. Units are labelled via `UnitId` tables with EN/DE names.

### 8.2 Accessibility
- **Text alternative of the 3D scene**: `StructuralScene.description` + per-entity `accessible_detail`; an always-available "Scene as list" mode (outliner is the keyboard-accessible model; selection in either updates both). Viewport has `role="application"` with an `aria-describedby` summary and live region announcing selection/utilisation changes ("Member B1 selected, utilisation 64 %, passes").
- Keyboard: full operation without pointer (Tab order outliner → viewport handles → properties → checks → diagrams; camera orbit/pan/zoom on arrow/+/− with documented keys; handles as in §6.3). Visible focus ring on entities and handles. Focus is never trapped.
- Colour: status = glyph + text + hatch + colour; palettes CVD-safe; high-contrast theme; forced-colors mode respected; user-selectable ramp (viridis-like, cividis-like, diverging red-blue, grayscale).
- Motion: ghost/stale animations obey `prefers-reduced-motion`; camera transitions off when set.
- Diagrams: table twin, series described in `title/desc`, tooltips reachable by keyboard (arrow through points).
- Targets sizes: ≥ 24 px (desktop) / ≥ 44 px (touch) hit areas for handles.
- Tests: wgpu accessibility projection tests exist in the framework (`🔬️targets-wgpu-accessibility-projection`); each shared window gets one.

### 8.3 Customisation
`Viewport3dWindowConfig` / `DiagramWindowConfig` (persisted local-only, schema-first in `🎚️config/🧬️schema`): colour map id and domain (e.g. 0–1.2, or clip at 1.0), role palette, background, lighting/sun (reuse `WorldSunConfig`), edge/outline style, label density, dimension visibility, load-glyph scale and per-load-kind visibility, deformation scale, section planes, unit preset, layer visibility/solo, saved camera views, inspector tier (Basic/Advanced/Expert), checks sort/filter. Themes = named config bundles (`StructuralTheme`) selectable and shareable as plain files. Windows can be docked/floated/hidden by the framework layout system; per-user layouts persist.

### 8.4 Multi-device (desktop → mobile → tablet)
| Device | Layout | Input |
|---|---|---|
| Desktop (primary) | Outliner+properties left, viewport centre, checks+diagrams right | mouse/keyboard, hover previews, gumball, handles |
| Mobile | One window at a time via bottom tab bar (Model, Properties, Checks, Diagrams); selection persists across tabs; viewport defaults to LOD1, labels declutter; bottom sheet for inspector over viewport | touch: tap select, long-press multi, two-finger orbit/pan/zoom, handles with 44 px targets and tap-to-nudge ±step buttons instead of free drag; no hover — ghost previews on press |
| Tablet | Two-pane: viewport + (properties or checks) toggle; diagrams as popover/sheet | touch + pencil: handles drag precisely, pencil hover (where available) acts as hover |
Device class from `UI/📱️device`; layout chosen by one `visual_layout(device_class)` function; LOD/budget by device class.

**8.4.1 Touch lane (wave 3).** Mobile and tablet layouts hold all five windows (the outliner is a tab, the list-based selection surface; on a phone a selection raises the properties bottom sheet). The shell feeds raw contacts and a clock through the domain-neutral `TouchSession` (`🕹️interaction/👆️gesture/🖐️touch-session`, slop 8 px, long press 450 ms, double tap 300 ms, replayed on the language-agnostic fixture `🖐️touch-session.json` by the Rust and TypeScript twins): the primary press is deferred until the finger has decided, a held finger is an additive press (shift) and a long press released in place asks the plugin (`PluginShellHooks::handle_touch`) to open the Edit menu, two fingers are the camera (`apply_pinch_to_orbit`, the Rust twin of the web orbit rig law, also expressed in the wheel and drag units of the native orbit controller), and where the plugin's `touch_profile` says a drag never edits a handle (tap-to-nudge) one finger orbits. Handles are edited there with the nudge rows of the viewport panel (minus and plus per grip, one schema step, one `setFields` commit). The Edit menu is the `render_touch_menu` tree in the viewport panel while the ephemeral view lane (`menu`) holds it open; copy, paste, duplicate, delete, select all and (through the Properties tab) bulk edit are reachable without a keyboard. Hit areas: markers of the diagrams pick within half the device minimum target (44 px on touch).

---

## 9. Phased implementation plan (15 agents)

Principles: schema before code; one owner per directory; shared files get a single owner and changes by others go through that owner's interface (no concurrent edit of the same file); each WP ships `.feature` + unit tests + oracle where applicable; all builds/tests via `repo`/MCP (never cargo directly); no git writes; path ≤ 230 UTF-8 bytes; no comments inside definitions; docstrings start with a unique emoji; EN+DE strings.

Agents: **A1 … A15**. Waves are gates ("W1 done" = schemas merged and compiling in the working tree).

### Wave 0 — contracts (A1, A2 in parallel; ~0.5 day) — blocks everything

**WP-01 (A1) Scene schema**
- Create: `NORM/🎬️structural-scene/{🦀️.rs, 📦️packages/🦀️rust/Cargo.toml}`; `🧬️schema/{🦀️.rs,🛰️.proto,🔗️.graphql,🟦️.ts}`; `🧬️schema/📐️geometry`, `🧱️section-shapes`, `🧩️entities`, `🏹️load-glyphs`, `🎨️appearance`, `🏷️annotations`, `🔺️scene-diff` (each `🦀️.rs`); `🧪️tests/🥒️.feature` (schema parity), `🧪️tests/🔬️unit/🦀️.rs`; register crate in `NORM/Cargo.toml` workspace.
- Contract output: all type definitions in §3.2–3.4 compile; `validate()` per type; canonical JSON writer (used for goldens).

**WP-02 (A2) Diagram, handle, field-spec, lane, config schemas**
- Create: `NORM/🎬️structural-scene/🧬️schema/📈️diagram2d/🦀️.rs`, `🧬️schema/🕹️handles/🦀️.rs` (+ proto/graphql/ts mirrors of those in the same four files owned by A1 → A2 supplies a `🧬️schema/🧩️parts/` include list; to avoid sharing the four top-level files, A1 owns top-level files and A2 delivers **separate** `📈️diagram2d/{🛰️.proto,🔗️.graphql,🟦️.ts}` and `🕹️handles/{…}` which A1's top-level files `import`).
- Create: `SURFACE/🫧️live-lanes/{🦀️.rs,🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}}` (`NormTransient`, `NormPresence`, `DragPreview`, `GhostPreview`, `EvalProgress`), `SURFACE/🔖️field-meta/🦀️.rs` rewritten to `FieldSpec` (§4.3.1) with the path-template lookup unchanged; `NORM/⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs` extended with `UnitId`, unit tables, conversion, parse/format (EN/DE unit names); conversion of the ten `SUBSET/✏️editor/🏷️field-meta/🦀️.rs` tables to compile against `FieldSpec` mechanically (struct update `..FieldSpec::EMPTY`), no content changes; adjust `SURFACE/📝️document-editors/🦀️.rs` only to keep the old walker compiling until WP-06 deletes it (time-boxed, ≤ the duration of Wave 1).
- Exit gate: workspace compiles; all ten artifacts' tests still pass.

### Wave 1 — infrastructure (A3–A11 in parallel after Wave 0)

**WP-03 (A3) Tessellation & geometry kernel** — `NORM/🎬️structural-scene/🧮️tessellation/**`: `🦀️.rs`, `🧱️section-outline`, `📏️extrude`, `🔩️fasteners`, `🪨️solids`, `🏹️glyphs`, `🌐️shells` (+ tests); `NORM/🎬️structural-scene/🔮️oracles/🐍️.py` + `🥒️.feature` geometry oracle (trimesh/sectionproperties/shapely). Done when profile catalogue (I, H, RHS, CHS, L, U, T, alu polygon, composite+rebar) matches oracle.

**WP-04 (A4) Projection framework + diff + a11y description** — `NORM/🎬️structural-scene/🎯️projection/{🦀️.rs,🧱️scene-builder/🦀️.rs,🧩️fragment-cache/🦀️.rs,🔗️dependency-index/🦀️.rs}`, `🔺️diff/🦀️.rs`, `🧪️tests` (laws in §7.1), a **mock artifact** projection in `🧪️tests/🧫️fixtures/` for tests (does not touch real artifacts).

**WP-05 (A5) Incremental evaluation** — `NORM/⚖️compliance/⚡️incremental-evaluation/{🦀️.rs,⚙️units/🦀️.rs,🕸️entity-graph/🦀️.rs,📋️planner/🦀️.rs,🧵️session/🦀️.rs,🔍️read-tracker/🦀️.rs,🧪️tests}`; edits to `NORM/⚖️compliance/🦀️.rs` (mount) and `🏛️norm-error-and-family/🦀️.rs` (add `IncrementalNormFamily`, remove `document_revision_key` + global report cache + `NormHost` per-paint caching once WP-12 lands — removal in WP-12). Bench for §5.7 evaluation rows.

**WP-06 (A6) Typed inspector** — `SURFACE/🪟️visual-windows/🧾️properties/{🦀️.rs,🎛️controls/🦀️.rs,✅️field-feedback/🦀️.rs,🧪️tests}`; delete `render_document_editor`, `render_object_editor`, `render_array_editor`, `render_value_editor`, `render_document_json` from `SURFACE/📝️document-editors/🦀️.rs` (keep `inputs_section_id`, summary) and fix `SURFACE/🧪️tests` that referenced them; `SURFACE/🌲️outliner/🦀️.rs` (new, mount in `SURFACE/🦀️.rs`). Owns `SURFACE/🦀️.rs` mount list edits for *all* new SURFACE modules (single-writer rule: other WPs hand A6 their mount line via the shared hand-off file `TICKET/📌️important/mounts.md`, A6 applies them within the hour).

**WP-07 (A7) 3D viewport window** — `SURFACE/🪟️visual-windows/{🦀️.rs,🧊️viewport3d/**}` (lowering §3.5, `Viewport3dWindowConfig` owner, measures, legend), `SURFACE/🪟️manifest-and-media/🦀️.rs` (+ `viewport_window_definition`, `diagram_window_definition`, `visual_layout(device)`), interaction-domain generator `SURFACE/🕹️direct-manipulation/🧭️domain/🦀️.rs`.

**WP-08 (A8) 2D diagrams window** — `SURFACE/🪟️visual-windows/📈️diagrams2d/{🦀️.rs,🖼️canvas-lowering/🦀️.rs,📊️table-twin/🦀️.rs,🎯️hit-test/🦀️.rs,🧪️tests}`; generic diagram builders reusable by projections: `NORM/🎬️structural-scene/📈️diagram-kit/{🦀️.rs,📉️curves/🦀️.rs,🧿️interaction-polygon/🦀️.rs,📜️spectra/🦀️.rs,🌀️sn-curves/🦀️.rs,🧱️stress-blocks/🦀️.rs}` (pure functions: polygon from capacity points, ray/utilisation, log-log S–N, spectrum sampling, stress-block geometry) with `shapely`/`numpy` oracle.

**WP-09 (A9) Checks window + viewer wiring** — `SURFACE/🪟️visual-windows/✅️checks/{🦀️.rs,📶️utilisation-bar/🦀️.rs,🩹️remedy-chips/🦀️.rs,🔗️clause-link/🦀️.rs,🧪️tests}`; supersedes `SURFACE/🌳️report-tree/🦀️.rs::render_report` (keeps `report_table_columns/rows` as table twin); `SURFACE/📚️catalogue-and-inspection/🦀️.rs` inspection panel updated to share row component.

**WP-10 (A10) Direct manipulation + live lanes** — `SURFACE/🕹️direct-manipulation/{🦀️.rs,🛠️handle-drag-tool/🦀️.rs,🧲️snap/🦀️.rs,⌨️keyboard/🦀️.rs,🎯️pick-targets/🦀️.rs,🧪️tests}`, `SURFACE/🫧️live-lanes/🧵️lane-runtime/🦀️.rs` (preview overlay, 30 Hz presence publisher, coalescer), new commands in `SURFACE/⌨️command-args-and-dispatch/🦀️.rs` and `SURFACE/✏️edit-rules/🦀️.rs` (`setFields`, `previewRemedy`, `clearPreview`, `cancelEvaluation`, `evaluateUnits`), `SURFACE/🧵️retained-commands/🦀️.rs` hooks for batch dispatch.

**WP-11 (A11) Theme, colour maps, a11y, devices, i18n audit** — `NORM/🎬️structural-scene/🎨️colour-maps/{🦀️.rs,🌈️ramps/🦀️.rs,🧪️tests,🔮️oracles/🐍️.py}`; `SURFACE/🪟️visual-windows/♿️accessibility/{🦀️.rs,📣️live-region/🦀️.rs,⌨️focus-order/🦀️.rs,🧪️tests}`; `SURFACE/🪟️visual-windows/📱️device-profiles/🦀️.rs` (LOD/budget/layout per device); i18n census test for all new strings; theme bundle schema `SURFACE/🪟️visual-windows/🎨️themes/🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}`.

### Wave 2 — EN 1993 vertical (reference template) (A12 + helpers A3, A4 for 3 days)

**WP-12 (A12) EN 1993 reference vertical** — owns the whole `NORM/🗿️artifacts/🔩️en1993/**` tree and proves every shared piece:
- Create `SUBSET/🎬️scene/{🦀️.rs,📦️packages/🦀️rust/Cargo.toml,🔩️members/🦀️.rs,🔗️joints/🦀️.rs,🧱️plates-and-shells/🦀️.rs,🏹️loads-and-supports/🦀️.rs,📈️diagrams/🦀️.rs,🕹️handles/🦀️.rs,🧪️tests/{🥒️.feature,🔬️unit/🦀️.rs,🧫️fixtures/**},🔮️oracles/🐍️.py}`.
- Split `SUBSET/🧬️schema/💡️inferences/⚖️checks/**` `evaluate` into `CheckUnit`s (files `🧱️member-checks`, `🔗️special-element-checks/*`) and implement `IncrementalNormFamily`; remove `NormHost::from_artifact` use in `SUBSET/✏️editor/🦀️.rs::render`, add retained `EvaluationSession`.
- Add `Placement` mutations + snapshot fields (`🧬️schema/📸️snapshot`, `🧬️mutations`, proto/graphql/ts, `🔺️diff`, `🚪️io` codecs, examples updated by hand).
- Replace windows: delete `SUBSET/✏️editor/🎭️modes/✏️edit/🪟️windows/{📥️inputs,📊️results}` and `SUBSET/👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report`; create `{🧊️viewport,📈️diagrams,🧾️properties,✅️checks}` shims in editor and `{🧊️viewport,📈️diagrams,✅️checks}` in viewer; update `SUBSET/✏️editor/🦀️.rs` (Transient/Presence types, commands rows, manifest, layout), `🏷️field-meta/🦀️.rs` content, `🎭️modes/✏️edit/🦀️.rs` layout, panels.
- Deliverable: a **rollout kit** `TICKET/📌️important/artifact-rollout-checklist.md` (not code; ≤ 1 page) listing the exact mechanical steps, so Wave 3 agents do not need clarification.
- Exit gate: §5.7 latencies and §7 laws pass on EN 1993; EN 1993 e2e feature passes.

### Wave 3 — fan-out (nine artifacts; each agent owns exactly its artifact tree `NORM/🗿️artifacts/<emoji>enXXXX/**`; same steps as WP-12's kit)

| WP | Agent | Artifact | Distinct 3D work | Distinct 2D work |
|---|---|---|---|---|
| WP-13 | A13 | `⚖️en1990` | load-glyph model of combinations, bridge SLS deformed shape | combination matrix, ψ-chart |
| WP-14 | A14 | `🏋️en1991` | pressure zones (wind c_pe, snow shape), area loads, impact/explosion glyphs | q_p(z), c_pe map, snow factors |
| WP-15 | A15 | `🏛️en1992` | composite sections with rebar cages, anchors, tendons, punching cone | M–N polygon, stress/strain block, shear truss |
| WP-16 | A5 (free after WP-05) | `🧩️en1994` | steel+deck+slab+studs | plastic stress blocks, shear connection |
| WP-17 | A6 (after WP-06) | `🪵️en1995` | timber members + connection patterns with editable spacings | k_mod, embedment, n_ef |
| WP-18 | A7 (after WP-07) | `🪨️en1996` | masonry walls with bond/openings/spreading | eccentricity, Φ(λ) |
| WP-19 | A8 (after WP-08) | `🌍️en1997` | strata solids, footings, retaining walls, slopes, piles | bearing, earth pressure, CPT, slip circle |
| WP-20 | A9 (after WP-09) | `🫨️en1998` | storeys/mass glyphs, mode shapes, towers/tanks/silos | response spectra, drift, capacity curve |
| WP-21 | A10 (after WP-10) | `🪶️en1999` | alu polygons, HAZ bands, connections | buckling curves alu, HAZ ρ |

(So agents A1, A2, A3, A4, A11 are the floating hardening pool: A1/A2 = schema evolution requests + golden review across artifacts; A3 = geometry gaps requested by artifact agents (new profiles/solids) via tickets in `TICKET/📌️important/requests.md`; A4 = projection-framework fixes and cross-artifact scene-law CI; A11 = a11y/i18n/perf audits and the cross-artifact bench gate.) That yields 15 agents with a maximum of 9 artifact agents + 5 floating.

### Wave 4 — hardening (all floating agents)
- Cross-artifact scene-law matrix (all ten fixtures × laws in §7.1), CVD + contrast audit, i18n census, bench gate, e2e screenshots, delete leftover references to removed APIs (`render_document_json`, `NormFieldMeta`, global report cache); close ticket with `ticket_close`.

### Ownership rules (non-overlap)
| Path set | Single owner |
|---|---|
| `NORM/🎬️structural-scene/🧬️schema/**` top-level four files + `📐️geometry,🧱️section-shapes,🧩️entities,🏹️load-glyphs,🎨️appearance,🏷️annotations,🔺️scene-diff` | A1 (A2 owns `📈️diagram2d`, `🕹️handles`) |
| `…/🧮️tessellation/**`, geometry oracle | A3 |
| `…/🎯️projection/**`, `…/🔺️diff/**` | A4 |
| `…/📈️diagram-kit/**` | A8 |
| `…/🎨️colour-maps/**` | A11 |
| `NORM/⚖️compliance/⚡️incremental-evaluation/**`, `🏛️norm-error-and-family` edits | A5 |
| `SURFACE/🔖️field-meta`, `SURFACE/🫧️live-lanes/🧬️schema`, `compliance/📐️quantities…` (units) | A2 (Wave 0 only), then read-only; later changes via A2 |
| `SURFACE/🧾️properties`, `🌲️outliner`, `📝️document-editors`, `SURFACE/🦀️.rs` mounts | A6 |
| `SURFACE/🧊️viewport3d`, `🪟️manifest-and-media`, `🕹️direct-manipulation/🧭️domain` | A7 |
| `SURFACE/📈️diagrams2d` | A8 |
| `SURFACE/✅️checks`, `🌳️report-tree`, `📚️catalogue-and-inspection` | A9 |
| `SURFACE/🕹️direct-manipulation/(rest)`, `🫧️live-lanes/🧵️lane-runtime`, `⌨️command-args-and-dispatch`, `✏️edit-rules`, `🧵️retained-commands` | A10 |
| `SURFACE/♿️accessibility`, `📱️device-profiles`, `🎨️themes` | A11 |
| `NORM/🗿️artifacts/<x>/**` | the WP's agent (A12 for en1993, Wave 3 table for the rest) |

Cross-owner change protocol: file a request line in `TICKET/📌️important/requests.md` (`<from> → <owner>: path, change, reason`); owners act within the same wave; never edit another owner's file.

### Risks and mitigations
| Risk | Mitigation |
|---|---|
| Per-entity check split changes numerical results | Differential test incremental ≡ full ≡ pre-split golden reports (existing Python oracles per artifact stay the arbiter) |
| Mesh JSON for custom meshes underspecified | WP-07 spike first day against fem3d's `fem3d_solid_mesh_entries`; fall back to primitive instancing + paged lease |
| Scene size for EN 1997 strata / EN 1998 shells | Paged snapshot lease + LOD ladder + budgets in device profiles |
| Missing placements in existing fixtures | Derived default layout (golden-tested) — no migration |
| Hosts other than React world (native wgpu) lack a feature (e.g. section caps) | Lowering degrades by capability flags; scene stays complete; table twin and outliner guarantee function |
| 10 crates × new `Cargo.toml` churn | One workspace edit by A1 in Wave 0; artifacts add one dependency line each in their own tree |

---

## 10. Definition of done (feature level)
1. Opening any of the ten artifacts shows: outliner, 3D viewport with utilisation colouring, typed properties, checks list with bars and clause links, and at least one diagram per governing check — no JSON text visible anywhere in the editor.
2. Dragging any declared handle previews live (<12 ms p95), commits as one undoable event, and the affected checks update within 400 ms p95 (T2 streams with progress and is cancellable).
3. Every fixture projects to a golden scene validated by independent oracles; incremental ≡ full evaluation holds on random mutation sequences.
4. EN and DE complete; keyboard-only and screen-reader paths exist for selection, editing and checks; CVD-safe palettes verified.
5. Desktop, mobile and tablet layouts implemented from the same windows.
6. Viewer shows the same scene and checks read-only; no editor import in the viewer tree.
7. All obsolete code removed (`render_document_editor` and friends, `NormFieldMeta`, global report cache, `document_revision_key`), no compatibility layers.
