# 🧬 FEM 2D schema, mutations, analysis outputs, results storage, demo, schema-first process (2026-09-16)

Read-only exploration (Sonnet). Paths relative to the repo root.

## 1. Document schema

Entities in `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs`; snapshot in `🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/{🦀️.rs,🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`. Per-subset `🧬️schema/🔣️.json` files `allOf`-ref the shared `fem/fem2d/artifact.json`.

| Entity | Type (line) | Fields |
|---|---|---|
| Node | `FemNode` (62) | `id`, `x` m, `y` m |
| DOF | `FemDof` (79) | Tx,Ty,Tz,Rx,Ry,Rz (2d: Tx,Ty,Rz) |
| Element | `FemElement` (161, tag `kind`) | `Bar{id,start,end,materialId,sectionId}` / `Beam{…}` |
| Material | `FemMaterial` (179) | `id,name,e Pa,nu,rho kg/m³` |
| Section | `FemSection` (192) | `id,name,area m²,iy m⁴` |
| Support | `FemSupport` (204) | `id,nodeId,fixed: Vec<FemDof>` |
| Load | `FemLoad` (214, tag `kind`) | `Nodal{id,nodeId,dof,value}`, `MemberUdl{id,elementId,wx,wy}`, `Area{id,regionId,pressure}` |
| Load case | `FemLoadCase` (233) | `id,name,loads,selfWeight` |
| Region | `FemRegion` (245) | `id,name,outline,holes,thickness,materialId,meshSize` |
| Combination | `FemCombination` (268) / `FemCombinationTerm` (260) | `id,name,terms[{caseId,factor}]` |
| Analysis | `FemAnalysisSettings` (285) | `modalCount,bucklingCount,deformationScale` default {3,3,50} |

`Fem2dSnapshot` (`📸️snapshot/🦀️.rs:12`): `nodes, elements, regions, materials, sections, supports, loadCases, combinations, analysis`. Camera, result display and active example are NOT document fields (window config / `Effect::LoadDocument`).

## 2. Mutations (`🧬️mutations/🦀️.rs:30`, `KINDS` at :391, 25 variants)

| Entity | create | delete | replace/update |
|---|---|---|---|
| Node | ✅ | ✅ (guard `element_references`) | ❌ — no move/replace-node |
| Element | ✅ | ✅ | ✅ `replace-element` |
| Material / Section / Support / Region | ✅ | ✅ | ✅ `replace-*` |
| Load case | ✅ | ✅ (guard `load_case_referrers`) | ❌ (rename impossible) + `change-load-case-self-weight` |
| Load | `add-load{caseId, load}` | `remove-load{caseId, loadId}` | ❌ — no replace-load |
| Combination | ✅ | ✅ (guard `combination_referrers`) | ❌ |
| Analysis | — | — | ✅ `update-analysis-settings{settings}` |

Guards `🧬️mutations/🦀️.rs:124+`: `identity_matches` (143) forbids renaming via replace; `referenced` (155) blocks orphaning deletes; `node_geometry`, `material_plausibility`, `section_plausibility`, `region_geometry`, `analysis_bounds`, `*_reference`, `*_referrers`.

Inspector consequence: element/material/section/support/region/analysis are editable with existing `replace-*`/`update-*` (send the whole record with one field changed). Node position and load fields need NEW mutations (`replace-node`, `replace-load`; also `replace-load-case` for rename and `replace-combination` for terms).

## 3. Analysis outputs

Engine `✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs` (`solve_linear_static` :462 → `StaticResult` :324 `{displacements: Vec<NodeDisplacement{node_id, values:[f64;6]}>, reactions, elements: Vec<(String, ElementResult)>, checks}`; `ElementResult` :306 `Bar{n}`, `Beam{stations[{x,n,v,m}]}`, `Plane{gauss}` …), `⚙️engine/🧮️analyses/🦀️.rs` (`solve_multi_case` :2877, `modal` :2973 → `ModalResult{frequencies_hz, shapes}`, `buckling` :3031 → `BucklingResult{factors, shapes}`), 2d bridge `⚙️engine/◻️2d/🦀️.rs` (`fem2d_solve_all` :102 → `HashMap<String, StaticResult>` keyed by case/combination id) and `◻️2d/🎵️modal-buckling/🦀️.rs` (`fem2d_modal_mode_values` :48, `fem2d_buckling_mode_values` :101).

No transient/time-history and no load-stepping anywhere. `FemJobStage` (`analyses/🦀️.rs:464`) is solve progress, not timesteps. The deformation animation is therefore a phase/scale animation of one solved static field (0→1, or sinusoidal breathing) or of a unit-normalised mode shape; the per-node data is `StaticResult.displacements[node].values[Tx,Ty]` or the mode values after `normalize_mode_shape`.

## 4. Results storage and deformed-shape rendering

No persisted results cache: `fem2d_solve_all`/`fem2d_modal`/`fem2d_buckling` run fresh inside every results render (`📊️results/🦀️.rs:140, 243, 267`). Persisted per window: `Fem2dResultsWindowConfig{camera, result_source_id, result_mode, result_mode_index}` (`📊️results/🎚️config/🦀️.rs:8`, schema `🎚️config/🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`, fixture `🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json`, contract test `🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`), mutated by `Fem2dResultsWindowConfigMutation::Snapshot{config}` from `SetResultDisplay::handle_window` (`👁️set-result-display/🦀️.rs:26`).

`fem2d_deformed_shape_layers(doc, disp_map, deform_scale)` (`🧱️model/🦀️.rs:1114`): straight polyline between displaced endpoints, `dx = disp[Tx] * scale * SCALE_2D`, `dy = -disp[Ty] * scale * SCALE_2D`. Static uses `doc.analysis.deformation_scale`; modal/buckling use `normalize_mode_shape` (`app-surface/🦀️.rs:64`) × `fem2d_model_extent × MODE_SHAPE_AMPLITUDE_RATIO (0.1)`. Static also draws reaction labels (159-169) and the von-Mises contour (200-233). An animated frame reuses the helper with `scale × phase`.

## 5. Demo fixture (`🪆️subsets/🌐️any/🖼️assets/🎬️demo/🗣️.dsl.semio`, also the default boot document via `default_fem2d_snapshot`, `🌐️any/🧬️schema/🦀️.rs:177`)

Timber portal frame with concrete slab and steel foundation column: 12 nodes (`n1,n2,p8,p0_l1,p0_l2,p8_l1,p8_l2,ridge,rc0..rc3`), 9 beams (`e3` CHS column, `e4..e7` posts, `e8,e9` floor beams, `e10,e11` rafters), 1 region `r1` "First Floor Slab" (t 0.2 m, concrete, mesh 1 m), 3 materials (`steel` 210 GPa, `timber` 11 GPa, `concrete` 33 GPa), 4 sections (`chs76, post140, floorbeam, rafter`), 4 supports (`s1` n1 Tx,Ty; `s2` p8 Ty; `s3,s4` rc0/rc1 Tx,Ty), 2 load cases (`dead`: area 800 Pa on r1 + self weight; `live`: nodal −12000 N Ty on p8_l1 + area 1500 Pa), 1 combination `uls` = 1.35 dead + 1.5 live, analysis {3, 3, 300}.

## 6. Schema-first file set for one mutation (template `create-node`, `🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/⚪️create-node/`)

1. `🦀️.rs` payload + `impl MutationKind<Fem2dSnapshot, Fem2dMutation>` (SEMANTICS, `label()`, `target()`).
2. `🔺️diff/🦀️.rs` — `diff(payload, base) -> MutationOutcome<Fem2dDiff>`.
3. `↩️inverse/🦀️.rs` — `inverse(payload, base) -> Vec<Fem2dMutation>`.
4. `🧬️schema/🔣️.json` payload JSON schema (`mutation: "createNode"` tag).
5. `🔣️.json` leaf descriptor (`MutationLeafDescriptor`: `semanticKind`, `aggregateVariant`, `payloadSchema`, `textOpcode`, `invertibility`, …).
6. `🧪️tests/<slug>/🦀️.rs` one folder per scenario.
Shared: 7. `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` enum + `KINDS` (`kinds_conformance` test :654); 8. `🟦️.ts` union; 9. `🔣️.json` aggregate `oneOf`; 10. `📖️.grammar.semio` opcode (`semio-grammar-conformance`); 11. `🔗️.graphql`/`🛰️.proto` mirror the snapshot only; 12. guards; 13. commands/fixtures that surface it. Governing doc: `🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️derivation-rules.md`.
