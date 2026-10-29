# Research: Nakagin Capsule Tower Teaching Model

Ticket: GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS
Scope: read-only research for Grasshopper and Excel teaching videos.
Status: Research only. No repo files were changed.

> **Superseded values.** The coordinator's decisions and the shared model `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json` replace the "Recommended teaching values" below where they differ: capsule 2.5 × 4.0 × 2.5 m; types A 36, B 35, C 34, D 35; tower A 9 capsule storeys × 3 facades × 2 = 54, tower B 10 × 4 × 2 + 1 × 3 × 2 = 86; podium 5.5 m; Tokyo HDD18 1,585.7 K·d (JMA normals 1991–2020). Audits check against the model, not against this table.

Conventions used below:
- `[R]` = repo evidence (path given).
- `[W]` = web source (URL given).
- `[K]` = author knowledge, not verified in this session. Treat as uncertain.
- Values marked **uncertain** conflict between sources or were not confirmed.

---

## 0. Summary

- The repo has one Nakagin model: an IFC4 export of the repo's own "Metabolism" kit (`nakagin-capsule-tower.ifc`). It is a generated model of the repo's design, not an independent architectural source.
- The repo IFC contains 154 capsules in 6 orientation variants (`\`, `/`, `L`, `J`, `s`, `p`), 2 cores, 16 tambours, 1 base, 2 capitals, 3 bridges. The historical count is 140.
- Historical sources agree on: 2 towers, 140 prefabricated capsules, 2.5 × 2.5 × 4.0 m capsule (Wikipedia), circular window about 1.3 m, four high-tension bolts per capsule, built 1970–1972, Ginza, demolition started 12 April 2022.
- Sources conflict on tower storeys (11 and 13 are the most consistent) and on interior capsule size (2.3 × 3.8 × 2.1 m internal vs 2.5 × 4.0 m nominal).
- Recommended teaching model: 140 capsules, Tower A 11 storeys with 9 capsule floors, Tower B 13 storeys with 11 capsule floors, 7 capsules per floor (63 + 77 = 140), core 5.5 × 5.5 m, storey pitch 2.75 m, 4 capsule types.

---

## 1. Repo inventory

### 1.1 Files found (`git ls-files | grep -i nakagin`, 110 matches total)

| Area | Path | Notes |
|---|---|---|
| IFC fixture | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc` | 2,496,437 bytes. IFC4, `FILE_NAME` from IfcOpenShell 0.8.4.post1, project named "Metabolism". Units: millimetres (`IFCSIUNIT` MILLI METRE). |
| Semio kit DSL (mutate fixture) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧪️tests/🧫️fixtures/🧰️mutate-semio-kit/🏢️nakagin-capsule-tower/🗣️.dsl.semio` | 7 lines, 78 KB. Types are hex-encoded; decoded names: Base, Bridge, First Storey, Capital, `\`, Tambour, `/`, `J`, `L`, `p`, `s`, Last Storey. |
| Semio model DSL (mutate fixture) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🧪️tests/🧫️fixtures/🏛️mutate-semio-model/📝️nakagin-capsule-tower.dsl.semio` | 4 lines, 119 KB. Spatial tree Site > Building > Storey, elements of class IfcBuildingElementProxy with `ComposePieceAttributes`. |
| Semio graph DSL | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧪️tests/🧫️fixtures/🌳️mutate-semio-graph/🏢️nakagin-capsule-tower/🗣️.dsl.semio` | 136 KB. Not inspected in detail. |
| Semio flow fixture | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧪️tests/🧫️fixtures/🌊️mutate-semio-flow/📝️nakagin-capsule-tower.dsl.semio` | 131 KB. Not inspected in detail. |
| Puzzle 3D tower (assets) | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio` | 249 lines. Objects: Base, Bridge, Capital, Tambour, plus capsule and roof objects. Vortex (port) coordinates in metres. Best single source for port geometry. |
| Puzzle 3D tower (assets, other) | same folder: `📡️tower.spr.semio`, `🔧️tower.op.semio`, `🎒️.pack.semio`, `🛂️manifest.json` | Not inspected. |
| Puzzle 2D/5D/3D examples | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻️2d,🖐️5d,🧊️3d}/.../📚️examples/🏗️nakagin-capsule-tower/` | TS and Rust examples and tests for each dimension. Not inspected in detail. |
| Block example (single capsule) | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio` | 27 lines. Object "Capsule J", 1 representation `capsule_J.glb`, 1 door vortex at `@0,-1.6,1.2` direction `^0,-1,0`, radius 0.3. |
| Block capsule JSON | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/🏢️nakagin-capsule.json` | Not inspected. |
| Repo note | `.🧬semio/🦑️repo/✍️notes/nakagin-examples.md` | Explains how to recreate the Nakagin SVG (flat layout, 367.416 × 2169.141 canvas, 12 piece symbols, 181 placements). It is about a 2D SVG, not the 3D geometry. |
| Repo note | `.🧬semio/🦑️repo/✍️notes/nakagin-shared.ts` | Helper for loading that SVG. 68 lines. |
| Repo image | `🧰️framework/🔨️modules/🖼️assets/🖼️images/🏙️architecture/🏢️nakagin-capsule-tower.png` | Preview image. Not inspected. |
| Tickets (NAKAGIN) | about 45 files under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/...` | Relevant summaries: `🎆️26/🌙️03/☀️20/EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS/🎫️ticket.json`, `🎆️26/🌙️03/☀️25/UPDATE-SCENE-STORIES-TO-USE-NAKAGIN-CAPSULE-TOWER-DESIGN/`, `🎆️26/🌙️05/☀️16/NAKAGIN-BOARD-RECTANGLE-FOOTPRINT-MATCHES-CIRCLE-DIAMETER/🎫️ticket.json`. |

Ticket notes that matter:
- `EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS/🎫️ticket.json` says the IFC is generated from the Metabolism kit by `export_design_model(format='.ifc')` and contains "180 building element proxies". The fixture has 180 `IFCBUILDINGELEMENTPROXY` entities, consistent with that. The `📌️important.md` in that folder is empty.
- `FIXTURES-ARE-TESTING-EXAMPLES-ONLY/📓️nakagin-fixture-separation-2026-09-09.md` is a repo decision that fixtures are testing examples only. Use the historical values here as the teaching source, not the fixture.

### 1.2 IFC geometry (measured with a script in the scratchpad)

I parsed the IFC, applied the mapped-item transforms and placements, and computed world-space bounding boxes. Script: `C:\Users\Ueli\AppData\Local\Temp\claude\C--git-semio\0a2a834f-4899-4447-9012-59e0272e94be\scratchpad\ifc_world.py` (its helper is `ifc_summary.py` in the same folder). Output: `ifc_world_out.txt` in the same folder.

Element type counts (`IFCRELDEFINESBYTYPE`):

| IFC type name (decoded) | Count | Local bbox (mm, x × y × z) | Meaning (from `ComposeTypeMetadata` description) |
|---|---|---|---|
| `\` | 31 | 2628 × 2528 × 4246 | Apartment capsule, entrance back, view front |
| `/` | 40 | 2629 × 2528 × 4246 | Capsule, entrance back, view front, mirrored |
| `L` | 26 | 2632 × 2528 × 4246 | Capsule, entrance side, view front, mirrored |
| `J` | 33 | 2632 × 2528 × 4246 | Capsule, entrance side, view front |
| `s` | 13 | 2632 × 2528 × 4246 | Capsule, entrance side, view side |
| `p` | 11 | 2628 × 2528 × 4246 | Capsule, entrance back, view side, mirrored |
| Tambour | 16 | 4588 × 5506 × 5506 (world) | One-storey tambour for the rectangular shaft core, elevator in centre, stairs and doors on each corner |
| First Storey | 2 | 4588 × 5506 × 5506 (world) | Lowest core element |
| Last Storey | 2 | 4588 × 5506 × 5506 (world) | Top core element |
| Capital | 2 | 5500 × 11500 × 5500 | "A roof that has the skin of a cut extruded cuboid" |
| Base | 1 | 31396 × 7500 × 20724 | "Polygonal two-storey office building with an entrance hall and access to the staircases and elevators of the rectangular cores on the groundfloor... On the second floor are the office rooms." |
| Bridge | 3 | 5250 × 2678 × 4800 | "A bridge with two segments an addition on the west along the main path." |

Total capsules: 31 + 40 + 26 + 33 + 13 + 11 = **154**.

Capsule floors, per tower (counted from placement names `cs_sl<n>_d<n>_t_f<floor>_b_c<tower>`):

| Tower (IFC `c`) | Capsule floors (`f`) | Capsules | Per floor |
|---|---|---|---|
| c0 (tower at x ≈ -7.5 m) | f0–f8 (9 floors) | 69 | 7 or 8 |
| c1 (tower at x ≈ -18.6 m) | f0–f10 (11 floors) | 85 | 7 or 8 |

Other measured values:
- Storey pitch along the floor axis: 2.75 m (tambour and core origins at 7.5, 10.25, 13.0, ... m). Capsule origins also step by 0.917 m (one third of the pitch).
- Core positions: c0 at (-7.5, -7.7) m, c1 at (-18.6, -7.7) m. Centre-to-centre spacing 11.1 m (in IFC x).
- Tambour plan: 5.5 × 5.5 m (`door tambour` ports at ±2.75 m from centre).
- Bridge spacing: 8.25 m between the three bridges (3 floors × 2.75 m).

**Axis caveat (uncertain):** the IFC axes are not consistent. Capsule world extents show the 4.25 m length in z, and the same z axis carries the floor stacking. Two capsules one floor apart would overlap in that axis. I did not resolve whether the real vertical is the IFC Y or Z axis, so do not use the IFC axis for vertical dimensions. Use the plan dimensions (2.5 m wide, 4.0 m long) and the storey pitch only, and check orientation visually in Grasshopper.

### 1.3 Puzzle 3D tower port data (`🖼️assets/🏢️tower/🗣️.dsl.semio`)

Values in metres, direction vectors unitless.

| Object | Port (vortex-kind) | Point | Direction | Notes |
|---|---|---|---|---|
| Base | core rectangular bottom (c0), (c1) | (-7.5, -7.7, 7.5) | (0, 0, 1) | Base ports sit at the core bottoms. |
| Base | platform right (e) | (5.6, -1.3, 0) | (1, 0, 0) | |
| Roof/Capital | roof rectangular top | (0, 0, 0) | (0, 0, -1) | |
| Capsule (object 1) | door capsule right | (0.45, -2.1, 0) | (0, -1, 0) | Two doors per capsule: one on the side and one on the back. |
| Capsule (object 1) | door capsule right | (-1.3, -1.25, 0) | (-1, 0, 0) | Side door (lateral) |
| Capsule (object 1) | door capsule left | (1.3, -1.25, 0) | (1, 0, 0) | Side door (lateral) |
| Tambour | door tambour right (sl*_d0), left (sl*_d1), etc. | (2.75, 0.9, 0.2) and (0.9, 2.75, 0.2) | (1, 0, 0) and (0, 1, 0) | Doors on the four core faces. |
| Tambour | tambour rectangular bottom (b) | (0, 0, 0.917) | (0, 0, -1) | |
| Tambour | tambour rectangular top (t) | (0, 0, 3.667) | (0, 0, 1) | Height 2.75 m between bottom and top. |

Capsule ports at x = ±1.3 m match the 2.6 m capsule width in IFC.

---

## 2. Historical facts (with sources)

Each row lists the source. Conflicts are shown.

| Fact | Value | Source | Confidence |
|---|---|---|---|
| Architect | Kisho Kurokawa (Metabolist movement) | [W] https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower | High |
| Completed | 1972 (construction 1970–1972) | [W] Wikipedia (fetch, technical panel); [W] https://archeyes.com/nakagin-capsule-tower-kisho-kurokawa/ | High |
| Location | Ginza, Tokyo, Japan | [W] Wikipedia | High |
| Demolition | Began 12 April 2022; Wikipedia infobox says "Demolished 2022" | [W] Wikipedia; [W] https://sah.org/about-sah/news/news-detail/2022/06/07/sah-statement-on-the-demolition-of-the-nakagin-capsule-tower-in-tokyo | High for start date. Some sources say 14 April (Japan Property Central); treat as uncertain. |
| Towers | Two interconnected concrete towers | [W] Wikipedia | High |
| Storeys | Tower 1 = 11 floors, Tower 2 = 13 floors | [W] Wikipedia body ("eleven and thirteen floors"); Wikipedia infobox says 13 | High for 11 and 13. Some search summaries say 14 or 13 only. Treat as 11 and 13. |
| Heights | 13-floor tower 53.5 m, 11-floor tower 47.4 m | [W] Wikipedia footnote | Medium (single source) |
| Bridge decks | 3, on floors 6, 9, 12, linking both towers | [W] Wikipedia | Medium |
| Cores | Central lift and stair shafts, one per tower; core dimensions not published in Wikipedia | [W] Wikipedia | Medium |
| Concrete | Ordinary concrete basement to floor 2; lightweight concrete above | [W] Wikipedia | Medium |
| Capsules | 140 self-contained prefabricated capsules | [W] Wikipedia; [W] https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower | High |
| Capsules per floor | "Most floors had eight capsules per tower, with a few exceptions" | [W] Wikipedia | Medium. This conflicts with 140 total unless only some floors carry capsules. |
| Capsule side split | Side L 83, side R 57 | [W] Wikipedia type table (derived by the fetch) | Medium |
| Capsule size (nominal) | 2.5 m × 2.5 m × 4.0 m (8.2 × 8.2 × 13.1 ft) | [W] Wikipedia | High for the source; see conflict below |
| Capsule size (internal) | 2.3 × 3.8 × 2.1 m (width × length × height) | [W] https://www.archiweb.cz/en/b/nakagin-capsule-tower | Medium. Another source gives 2.4 × 3.7 × 2.1 m. |
| Capsule size (other) | 2.5 × 4.0 m with window (Archeyes); about 9 m² per unit (secondary source) | [W] Archeyes; [W] https://www.meer.com/en/106782-nakagin-capsule-tower-a-future-that-didnt-last | Medium |
| Window | Circular, 1.3 m diameter, at one end of the capsule | [W] Wikipedia; [W] Archeyes | High |
| Attachment | Each capsule fixed to one of the two shafts by four high-tension bolts | [W] Wikipedia; [W] Archeyes | High |
| Umbilical | About 1 m long | [W] Wikipedia | Medium |
| Installation | 5 to 8 capsules per day; about 30 days; 3 hours per capsule | [W] Wikipedia | Medium |
| Fabrication | Factory about 450 km from Tokyo; Archeyes text says "built in Osaka" | [W] Wikipedia; [W] Archeyes | Low. The factory name is not confirmed here, and it is uncertain whether Osaka is the factory location. Leave the factory name blank in teaching material. |
| Structure per capsule | Steel frame; two days steel frame, two days precast concrete | [W] Archeyes | Medium |
| Fire protection | 45 mm asbestos on major structural elements; 30 mm on exterior panels | [W] Wikipedia | Medium. Mention only as history if at all. |
| Base / lower building | Two-storey office podium with entrance hall, staircases and lifts of the cores on ground floor, offices on floor 2, setback and colonnade towards the street | [R] IFC `Base` description; the real building's podium is not confirmed by a web source here | Low. Uncertain for the real building. |
| Roof / capital | Capsule-free roof box on top of each core ("cut extruded cuboid" in IFC) | [R] IFC `Capital` description; no web source here confirms the real roof | Low |
| Demolition approval | 80 % of owners, first achieved 15 April 2007 | [W] Archeyes | Medium |
| Preserved capsules | 23 of 140 | [W] Wikipedia; [W] WebSearch summary | Medium |
| Floor area | 3,091.23 m² (infobox) | [W] Wikipedia infobox | Medium. Not stated whether for one tower or both. |

Conflicts summary:
1. Tower storeys: 11 and 13 (Wikipedia body) vs "13" only (infobox) vs "14" (one search summary). Use 11 and 13.
2. Capsule size: 2.5 × 2.5 × 4.0 m (Wikipedia, Archeyes, nominal) vs 2.3 × 3.8 × 2.1 m (internal, Archiweb). Use nominal outer size for the model and internal size for the Excel calculation.
3. Capsules per floor: "eight per tower per floor" (Wikipedia) cannot be reconciled with 140 total across 11 + 13 floors. The repo IFC has 7 or 8 per floor on 9 and 11 floors, which gives 154. Use 7 per floor for the teaching model (see section 3).

---

## 3. Recommended teaching values

All lengths in metres. The "Source" column names the basis: `R` = repo IFC/DSL measurement, `W` = web source, `K` = author judgement.

### 3.1 Capsule (outer dimensions and features)

| Parameter | Value | Source | Notes |
|---|---|---|---|
| Width (w) | 2.5 | W (Wikipedia 2.5 m); R (IFC 2.63 m bbox, includes bolts/brackets) | Use 2.5 |
| Length (l) (projection from core) | 4.0 | W (Wikipedia 4.0 m); R (IFC 4.25 m bbox) | Use 4.0 |
| Height (h) | 2.5 | W (Wikipedia 2.5 m, height from nominal); R (IFC 2.53 m) | Use 2.5 |
| Corner radius (vertical edges) | 0.25 | K | Teaching choice. Historical capsule is a rounded box with a soft shell; 0.2–0.3 looks right. |
| Window diameter | 1.3 | W (Wikipedia, Archeyes) | Circular |
| Window position | Centred on the outward end face (2.5 × 2.5 m face), centre height about 1.25 m above floor | K | Historically "at one end" (Wikipedia). Exact height not sourced. |
| Attachment | 4 bolts at the corners of the core-side face | W | Model as 4 small cylinders, radius 0.03 |
| Umbilical | 1.0 m, rectangular section 0.2 | W (1 m) | Optional, for detail shots |

### 3.2 Capsule types (4 named types)

The repo has 6 IFC variants. Historically the capsules are the same shape with different entrance and view positions. For the teaching model, group the 6 variants into 4 types.

| Type ID | Name | Repo IFC variants | Entrance | View | Mirrored | Count in teaching model (140 total) |
|---|---|---|---|---|---|---|
| T1 | Single residence, standard | `\` | back | front | no | 40 |
| T2 | Single residence, mirrored | `/`, `p` (mirrored) | back | front (`/`) or side (`p`) | yes | 36 |
| T3 | Office or study, side entrance | `J`, `L` | side | front | `J` no, `L` yes | 44 |
| T4 | Rotated (side view) | `s` | side | side | no | 20 |
| | Total | 6 variants | | | | **140** |

Count check: 40 + 36 + 44 + 20 = 140.

Note: T3 uses the room program "office or study" as a teaching proposal, not a historical fact. Historically all Nakagin capsules were residential.

### 3.3 Core

| Parameter | Value | Source | Notes |
|---|---|---|---|
| Core plan | Square, 5.5 × 5.5 m (outer) | R (tambour bbox 5.5 m, doors at ±2.75 m) | Historically rectangular shaft core |
| Core storey height | 2.75 | R (floor pitch) | Each core storey carries one tambour |
| Core height per tower | 11 or 13 storeys × 2.75 = 30.25 / 35.75 m | K from R pitch | Compare with W heights 47.4 / 53.5 m; the IFC pitch does not match the real building height. Use only for teaching, flag it. |
| Core centre spacing (two towers) | 11.1 | R (IFC c0 vs c1) | Historical spacing not confirmed (uncertain). |
| Tower A and B storeys | Tower A 11, Tower B 13 | W (Wikipedia body) | Use these. The IFC has 9 and 11 capsule floors. |

### 3.4 Capsule count and layout

| Parameter | Tower A (11 storeys) | Tower B (13 storeys) | Total | Source |
|---|---|---|---|---|
| Capsule floors | 9 | 11 | | R (IFC c0 f0–f8 and c1 f0–f10) |
| Capsules per capsule floor | 7 | 7 | | R (7 or 8 per floor in IFC); W ("eight per floor, with exceptions") |
| Capsules | 63 | 77 | **140** | W (140 total) |
| Capsules per facade (4 facades around core, 2 per side per floor) | 7 spread as 2+2+2+1 | 7 spread as 2+2+2+1 | | K |
| Bridges | 3 linking both towers, at floors 6, 9, 12 (Wikipedia) | same bridges serve both towers | 3 | W (Wikipedia). Repo IFC bridges are at IFC floors 2, 5, 8 (conflict, see section 5). |

Capsule rotation around the core (teaching values):

| Facade | Angle (degrees about core vertical axis) | Types on facade | Source |
|---|---|---|---|
| North (+Y) | 0 | T1, T3 (entrance toward core) | R (IFC yaw 0 on `\` at c0) |
| South (−Y) | 180 | T2, T3 (mirrored) | R (IFC yaw 180 on `\` at c1) |
| East (+X) | 90 | T4 (view side) | R (IFC `rotation` 90 on tambour doors; `s` variant) |
| West (−X) | 270 | T4 (view side) | R (IFC `rotation` 270) |

### 3.5 Lower building (podium)

| Parameter | Value | Source | Notes |
|---|---|---|---|
| Storeys | 2 | R (Base description: two storeys) | |
| Height | 7.5 | R (Base bbox 7.5 m) | |
| Footprint (overall) | 31.4 × 20.7 m | R (Base bbox) | Polygonal in IFC; use rectangle for teaching |
| Ground floor | Entrance hall, lift and stair access to cores | R | |
| Floor 2 | Offices | R | |
| Setback and colonnade | Setback towards street | R (description) | K |

### 3.6 Capital (roof box)

| Parameter | Value | Source |
|---|---|---|
| Plan | 5.5 × 5.5 m | R (Capital bbox) |
| Height | 11.5 m (as IFC bbox; top of tower core) | R |
| Top | Cut extruded cuboid skin (roof top port at centre) | R (`roof rectangular top` port) |

Note: the real Nakagin roof and water/machinery arrangement is not confirmed by a web source in this research. Use the IFC values as teaching placeholders and mark them as such in the video.

---

## 4. Energy values for an Excel heating-demand calculation

### 4.1 Model

Per capsule, steady-state heating demand for one heating season:

Q_season = (UA_envelope + 0.34 × n × V) × (T_in − T_out_mean) × t_heating  [W·h or Wh per season, with conversion]

Where UA_envelope is the sum of U × A over the envelope, n the air change rate, V the net volume, T_in = 20 °C, T_out_mean the mean outdoor temperature during heating hours, and t_heating the heating hours. For degree-day use: Q_season = 24 × HDD18 × (UA + 0.34·n·V) × 1 Wh/(W·K·d) in kWh (see 4.5).

### 4.2 Envelope values (teaching proposal)

| Parameter | Value | Unit | Source / basis |
|---|---|---|---|
| Interior set point | 20 | °C | K (common teaching value; state it) |
| Wall U-value (steel shell + insulation) | 0.60 | W/(m²·K) | K (proposed insulated steel shell with about 50 mm mineral wool, λ 0.035). Historically the shell was uninsulated steel with a thin panel (estimate 2.5 to 3.0 W/(m²·K)). Show both cases. |
| Roof / ceiling U-value | 0.50 | W/(m²·K) | K |
| Floor U-value (over crawl or bridged) | 0.60 | W/(m²·K) | K |
| Window U-value (single glazed, historical) | 5.0 | W/(m²·K) | K (uncertain, single glazing) |
| Window U-value (double low-E, retrofit) | 2.3 | W/(m²·K) | W: 2019 Tokyo guideline example, "2.33 W/m²K or less" (aluminium-resin, Low-E double glazing), source https://www.spt.metro.tokyo.lg.jp/tosei/hodohappyo/press/2019/01/29/documents/10.pdf (from WebSearch result, not fetched) |
| Window area | 1.33 | m² | W: 1.3 m diameter circle, area π × 0.65² |
| Air change rate | 0.5 | h⁻¹ | K (typical infiltration, uncertain) |
| Net volume (capsule) | 2.3 × 3.8 × 2.1 = 18.3 | m³ | W (Archiweb internal dimensions) |

Envelope areas for one capsule (outer, nominal 2.5 × 2.5 × 4.0 m):
- Roof: 2.5 × 4.0 = 10.0 m²
- Floor: 2.5 × 4.0 = 10.0 m² (over interior slab, ground floor not applicable in tower; treat as internal, so exclude from UA if floor is between conditioned storeys. Decision for the Excel: count floor as a loss only for the lowest capsule row.)
- Two long walls: 2 × (4.0 × 2.5) = 20.0 m² (minus window area for the end wall only)
- Two end walls: 2 × (2.5 × 2.5) = 12.5 m² (one end has the window, 1.33 m²; the other is the entrance door, assumed 2.0 m²)

Net opaque envelope (wall + roof, no floor): 20.0 + 10.0 + (12.5 − 1.33 − 2.0) ≈ 39.2 m²

### 4.3 Example UA for one capsule (teaching values)

| Case | UA_wall | UA_roof | UA_window | UA_total (W/K) | Ventilation 0.34·n·V (W/K) | Total (W/K) |
|---|---|---|---|---|---|---|
| Insulated (proposed) | 0.60 × 29.2 = 17.5 | 0.50 × 10.0 = 5.0 | 2.3 × 1.33 = 3.1 | 25.6 | 0.34 × 0.5 × 18.3 = 3.1 | 28.7 |
| Uninsulated steel, single glazed | 2.8 × 29.2 = 81.8 | 2.8 × 10.0 = 28.0 | 5.0 × 1.33 = 6.7 | 116.5 | 3.1 | 119.6 |

Wall area 29.2 m² = 2 long walls (20.0 m²) + 2 end walls net of window and door (12.5 − 1.33 − 2.0 = 9.2 m²). Floor is excluded from UA (internal slab between conditioned capsules), except for the lowest capsule row, which needs a separate calculation.

### 4.4 Tokyo climate (heating)

| Parameter | Value | Source | Confidence |
|---|---|---|---|
| Heating degree days, base 18 °C | About 1,000 to 1,300 K·d per year | K (uncertain). WebSearch did not return a Tokyo-specific figure. | Low. Compute from JMA daily data for Tokyo station (番号 47662) before publishing. |
| Mean January outdoor temperature | About 5 °C | K | Medium. Verify with JMA normals. |
| Heating design outdoor temperature (winter, Tokyo) | About 0 to 3 °C | K | Low. Verify with JMA or ASHRAE design data. |
| Heating season (teaching) | November to March, 151 days | K | Medium |
| Heating hours per day | 24 (conservative) or 12 | K | Teaching choice |

Source to check: JMA climate statistics https://www.data.jma.go.jp/stats/etrn/index.php (not fetched in this session).

### 4.5 Excel formula (per capsule, season)

- Degree-day method: Q_season [kWh] = 24 × HDD18 × (UA_total [W/K]) / 1000
  - Example, insulated, HDD18 = 1,100: 24 × 1,100 × 28.7 / 1000 = 758 kWh per season per capsule.
  - Example, uninsulated, single glazed: 24 × 1,100 × 119.6 / 1000 = 3,160 kWh per season.
- Excel layout: one sheet "Capsule_types" (rows = T1–T4, columns = wall A, roof A, window A, U values, n, V); one sheet "Climate" (HDD18 by month, JMA source); one sheet "Result" (per type, per tower, per building, kWh).

Caveat: these are steady-state teaching numbers. They do not include solar gain, internal gain, or the shared-wall effect between neighbouring capsules. Mark this on the slide.

### 4.6 Room program (teaching proposal)

| Capsule type | Net floor area (m²) | Occupants | Use | Source |
|---|---|---|---|---|
| T1 Single residence, standard | 2.3 × 3.8 = 8.7 | 1 | Residence (bed, bath unit, built-in storage) | W (interior size); W (historical use) |
| T2 Single residence, mirrored | 8.7 | 1 | Residence | W; R (mirror variant) |
| T3 Office or study, side entrance | 8.7 | 1 | Study or office (teaching variant, not historical) | K |
| T4 Rotated, side view | 8.7 | 1 | Residence (view to side) | W; R (`s` variant) |

Built-in furniture (bath unit, bed, storage) is historical per Wikipedia and Archeyes but not measured here. Leave furniture dimensions as K placeholders (bath unit about 1.2 × 1.0 m, bed 2.0 × 1.0 m).

---

## 5. Open uncertainties

1. IFC axis convention: which IFC axis is vertical. Capsule local length (4.25 m) lies along the floor stacking axis. Confirm visually before modelling.
2. Capsule count per floor: 7 vs 8 per floor and which floors are empty. 140 total is historical; the IFC has 154.
3. Tower A and B storeys: 11 and 13 (Wikipedia body). Heights (47.4 and 53.5 m) imply about 4.3 m per storey, which conflicts with the IFC 2.75 m pitch. Storey height in the real building is unknown here.
4. Capsule size: nominal 2.5 × 2.5 × 4.0 m vs internal 2.3 × 3.8 × 2.1 m. Which is "outer" for the model.
5. Core dimensions and plan in the real building: not found in the web sources. The 5.5 × 5.5 m core is from the repo IFC only.
6. Core spacing and bridge floors: IFC bridges sit at storeys 2, 5, 8 (by IFC floor index), Wikipedia says floors 6, 9, 12. Conflict.
7. Base (podium): 2 storeys, 7.5 m, 31.4 × 20.7 m is from the repo IFC only. The real podium is not verified.
8. Roof / capital: 5.5 × 11.5 × 5.5 m is from the repo IFC only. Not verified.
9. Factory location and name: Wikipedia says about 450 km from Tokyo; Archeyes says Osaka. Not confirmed.
10. Window position on the capsule: historical end-face position is from Wikipedia wording ("at one end"); exact face not confirmed.
11. Heating degree days and design temperatures for Tokyo: not verified by web search. Must be taken from JMA before the Excel is published.
12. U-values: steel-shell U-values in this file are teaching proposals, not measured. The 2019 Tokyo window figure (2.33) is the only sourced envelope value.
13. Japanese 2013 energy-saving standard region 6 values: not found in search results. Treat the region 6 limits as open. Check MLIT documents.
14. Demolition completion: Wikipedia says demolition "began" 12 April 2022; completion date not confirmed.
15. Room program: T3 "office or study" is a teaching invention. Historical capsules were residential.

---

## Sources

- [W] https://en.wikipedia.org/wiki/Nakagin_Capsule_Tower (fetched, technical details and body text)
- [W] https://archeyes.com/nakagin-capsule-tower-kisho-kurokawa/ (fetched)
- [W] https://www.archiweb.cz/en/b/nakagin-capsule-tower (search summary; internal dimensions 2.3 × 3.8 × 2.1 m)
- [W] https://www.meer.com/en/106782-nakagin-capsule-tower-a-future-that-didnt-last (search summary)
- [W] https://sah.org/about-sah/news/news-detail/2022/06/07/sah-statement-on-the-demolition-of-the-nakagin-capsule-tower-in-tokyo (search summary)
- [W] https://www.spt.metro.tokyo.lg.jp/tosei/hodohappyo/press/2019/01/29/documents/10.pdf (search summary, window U-value 2.33)
- [R] `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc`
- [R] `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio`
- [R] `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🖼️assets/🏢️nakagin-capsule/🗣️.dsl.semio`
- [R] `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧪️tests/🧫️fixtures/🧰️mutate-semio-kit/🏢️nakagin-capsule-tower/🗣️.dsl.semio`
- [R] `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️03/☀️20/EXPORT-NAKAGIN-CAPSULE-TOWER-IFC-FILE-TO-REPORTS/🎫️ticket.json`
- [K] Author knowledge for the energy proposals in section 4, to be verified against JMA and MLIT.
