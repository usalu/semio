# Typed enums (wave 2)

Input: rule-guard pass 18, check 21 (521 raw hits, per fixture copy). Deduped per schema field: 121 distinct non-id String fields in 10 artifacts (en1992 12, en1999 14, en1998 12, en1993 29, en1991 14, en1994 9, en1990 10, en1997 7, en1995 8, en1996 2) plus the id fields below. Scanner output reduced with the script in the session scratchpad (not kept). Nothing was compiled; no cargo/rustc was run.

## Result

- Converted end to end: en1992 `BarLayer.bond_condition` -> `BondCondition { Good, Poor }` (pilot, template for the rest).
- Classified: all 121 fields below.
- Not converted: everything else. Reason: one field touches about 30 source files in 8 twins plus 100 to 330 fixtures, the owners are active in every artifact, and without a build each conversion must be proven with the schema-sync checker only. The remaining conversions are mechanical with the template script `🧰️scripts/typed-enums/en1992_bond_condition.py` (one asserted replacement per site) and should be taken per artifact by the owner or in a follow-up wave.
- Skipped on purpose: en1995 `orientation` and `grade` (en1995 v7 is converting them).

## What the pilot changed (en1992, BondCondition)

Wire tokens `Good`/`Poor` (same convention as `FireRating`, `MemberKind`: plain variant names, `CanonicalJsonTree` safe). Labels EN/DE per variant: schema `$defs/BondCondition` `x-semio-ui.options` ("Good bond conditions"/"Gute Verbundbedingungen", "Poor bond conditions"/"Schlechte Verbundbedingungen"); the field-meta `BOND` choices already carried the same labels.

Files (root `🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any`):
- `🧬️schema/📸️snapshot/🦀️.rs`: enum (derives like `FireRating`, plus `DslScalar` so the DSL text codec reads `bond-condition=good` as kebab case), field type, 6 literals.
- Twins, artifact and snapshot facet: `🟦️.ts` (type, `normWireLiteral` parser, parser use), `🔗️.graphql` (enum, field), `🛰️.proto` (enum `BOND_GOOD/BOND_POOR`, field), `🔣️.json` (`$defs/BondCondition`, property `$ref`).
- Diff: `🔺️diff/🦀️.rs` (patch field type), `🔺️diff/🟦️.ts` (type, parser, import); diff json/proto/graphql unchanged (they reference the snapshot property, same as `exposure`).
- Leaf `⭕️change-bar-layer-bond-condition`: payload `new_bond_condition: BondCondition`, diff (no clone), inverse (copy), ts twin, json twin (`$ref` to the snapshot def, widget `text` removed).
- Codecs: sqlite rs (`bond_condition`/`read_bond_condition`, tokens `Good`/`Poor`), sqlite ts (`BONDS`, `choice`), `🗄️.sql` `CHECK(bond_condition IN ('Good','Poor'))`. The text codec (`dec_json`/`enc_json`) and the DSL assets need no change (value traits derive from the enum).
- Edit rule and kind: unchanged (`SetFieldRule` already points at `newBondCondition`); the editor choice list `BOND` already existed.
- Checks: 4 sites compare against `BondCondition::Poor` instead of the string `"poor"`.
- Fixtures: 320 JSON files rewritten by `🧰️scripts/typed-enums/en1992_bond_condition_fixtures.py`: `good` -> `Good`; sqlite roundtrip `arbitrary` -> `Poor`; apply fixture before=`Poor`, mutation/after/diff=`Good` (the former `good-alt` behaved as good, so the check-outcome numbers `fail-08/09/13` stay valid). No migration code remains in the repo; the scripts live in the ticket.
- Schema-sync (`check.py --artifact en1992`): no new mismatch from the enum in the snapshot lanes; one mismatch added in the leaf graphql twin (`newBondCondition: String!` vs json `BondCondition`), identical to the existing `ChangeMemberFireRating` pattern. The diff-lane count (about 330) is churn from the concurrent schema-sync work, not from this change (the same `ref json=support ts=SupportCondition` message exists for old enums).
- Found while reading: `En1992ReinforcementGradesPatch` declares `ductility: String` but the leaf diff passes a `DuctilityClass` (compile error unless fixed); routed to the en1992 owner in requests.md.

## Classification

Legend: ENUM = closed set, make a schema enum with EN/DE labels; CAT = open reference into an existing catalogue (id validated against it); REF = entity id into a collection of the same document (keep `String`, already validated by `mutation.target-missing`); FREE = free text, keep; DEAD = unused, delete.

### Id fields (all artifacts, about 330 of the 521 hits)
`member_id`, `layer_id`, `anchor_id`, `grade_id`, ... : REF. Keep String. The mutation payloads already reject unknown ids. Nothing to convert; the scanner counted them because fixtures use 2 or 3 ids.

### en1992 (12)
| field | host | values seen | class | proposed type / note |
|---|---|---|---|---|
| bond_condition | BarLayer | good, poor | ENUM | DONE `BondCondition` |
| position | BarLayer | bottom, top, tension, compression (field-meta `BAR_POS`) | ENUM | `BarPosition`; code tests "top" and "bottom"/"tension" |
| cement_type | En1992Artifact | N, S (R used by code) | ENUM | `CementClass { S, N, R }` (EN 1992-1-1 3.1.2(6)); code default arm `_ => N` goes away; many `&str` engine functions (`cement_s`, `creep_*`, `eps_cd_0`, `shrinkage`) and oracle inputs take the token |
| column_position | PunchingSpec | interior, edge, corner (field-meta) | ENUM | `ColumnPosition { Interior, Edge, Corner, WallEnd }`; `punching_beta` also handles `wall_end`, so the set has four members; scene `punching` and `control_perimeter` match on strings |
| column_method | FireSpec | A, B (field-meta) | DEAD | parameter is `_column_method` (unused) in `required_for`; delete the field or wire Table 5.2a/b method A/B and then `ColumnFireMethod { A, B }` |
| slab_system | FireSpec | one-way, two-way (field-meta also flat, ribbed) | ENUM | `SlabFireSystem { OneWay, TwoWay }`; flat and ribbed are already `MemberKind::FlatSlab/RibbedSlab`, drop them from the choice list |
| kind | LoadCaseActions | permanent, imposed, snow, wind, accidental, prestress; fixtures also `Beam`, `permanent-alt` | ENUM | `ActionKind`; fixtures `Beam` (a bogus member kind) must be fixed to a valid variant |
| category | LoadCaseActions | self, office, residential, shopping, storage, snow, wind, accidental; fixtures `unlisted` | ENUM or CAT | psi category: closed in field-meta, but `psi_factors(&category)` reads an EN 1990 Table A1.1 catalogue; prefer CAT into the en1990 category registry |
| source | LoadCaseActions | udl, point, external | ENUM | `ActionSource` |
| bend | BarPoint | auto, straight, hook, u-hook | ENUM | `BarBend` (scene `bend_mode` maps strings) |
| new_kind / new_source / new_category | leaf payloads | follow the host field | ENUM | change together with the host |
| new_name | rename leaves | free | FREE | |

### en1999 (14; the model has a single enum, everything is a String)
| field | host | values seen | class | proposed type / note |
|---|---|---|---|---|
| designation | Materials | aw6082-t6, aw6060-t6, ... | CAT | id into the alloy-and-temper table (`alloys-and-sections`); validate |
| product_form | Materials | extrudedProfile, extrudedTube, sheet | ENUM | `ProductForm` |
| weld_process, process | SectionsElements, WeldRuns | mig, tig | ENUM | `WeldProcess` (one enum for both) |
| weld_direction, direction | SectionsElements, WeldRuns | longitudinal, transverse | ENUM | `WeldDirection` |
| detail_type | FatigueCategories | butt-weld, fillet-weld | CAT | detail category table of EN 1999-1-3 (Annex J) |
| fabrication_class | Shells | B | ENUM | shell fabrication quality class read by `shell_quality_q` (A, B, C) |
| filler_alloy | WeldGroup | 4043, 5356 | CAT | filler alloy table (Table 8.8) |
| material | BoltGroup | 8.8, 10.9 | CAT | bolt grade catalogue (shared with en1993 `bolt_class`) |
| position | PatchLoads | end, interior | ENUM | `PatchLoadPosition` |
| ring | OutlineVertices | outer, hole | ENUM | `OutlineRing` |
| kind (member, connection, action) | various | combined, extrudedI, permanent, ... | ENUM per host | `MemberKind`/`ConnectionKind`/`ActionKind`; leaf payloads `new_kind` follow |
| new_source | ChangeConnActionSource | external, udl | ENUM | `ActionSource` |

### en1998 (12)
annex (de, en): ENUM, there is `AnnexChoice` in compliance, reuse it; content_category: ENUM; device (elastomeric, sliding): ENUM; direction (x, y): ENUM; drift_limit_class (brittle, ductile): ENUM; ductility_class (dcl, dcm, dch): ENUM; en_ground_type (A to E): ENUM; en_spectrum_type (type1, type2): ENUM; importance_class (I to IV): ENUM; knowledge_level (kl1 to kl3): ENUM; limit_state (nc, sd, dl): ENUM; system_type (frame, wall, masonry, ...): ENUM; t1_method (ct, given, rayleigh): ENUM; samples: FREE (numeric series as text; better a numeric list, tracked separately).

### en1993 (29)
ENUM: analysis (elastic, plastic), assumed_class (rigid, ...), chord_shape, end_support/start_support (fork, ...), environment, fabrication_class, lattice_kind (leg, ...), load_application, load_distribution, member_type (beam, beamColumn), method (safe_life, damage_tolerant), moment_diagram, patch_type, pattern (twoLines, ...), rating (r30, r60, ...; reuse a fire-rating enum), release_end/release_start (rigid, ...), row_type, shape, stainless_forming, stiffener_shape, support (fixed, free, ...), torsion_load, new_kind (leaf payload).
CAT: bolt_class (8.8, 10.9; bolt grade catalogue), grade (S355, S460, 1.4301; steel grade catalogue), subgrade (J2, K2; quality sub-grade of the grade), weld_grade (S355; same steel grade catalogue), designation (HEB 240, welded I 600x200x6x8: profile catalogue id or a parametric section, two shapes, keep FREE until the profile catalogue ids are typed).
FREE: stress_history (numeric series as text; make a numeric list).

### en1991 (14)
ENUM: bridge_load_group (gr1a, gr1b, ...), construction_activity (formwork, scaffolding), crane_class/hoist_class (HC1 to HC4; one `HoistClass` enum, the crane class field takes the same set), fire_occupancy (office, shopping, ...), roof_type (flat, monopitch, duopitch, ...), silo_kind (silo, tank), thermal_element_type (bridge, building), material (cement_screed, reinforced_concrete, steel: CAT into the EN 1991-1-1 Annex A density table), category (B1, C2, E1: ENUM, EN 1991-1-1 Table 6.1 categories), snow_zone (2, 3; ENUM, NA zones differ per annex: CAT into NDP zone table), shape/face/activity (`x` placeholders in fixtures: derive from the code first).

### en1994 (9)
ENUM: buckling_curve (a, b, c, ...), construction (propped, unpropped), fatigue_detail (flange_butt_weld, reinforcement, stud_welded: CAT into the detail category table), fire_rating (r60, r90: reuse a fire-rating enum), structure_kind (bridge, building), stage (construction, composite), new_category/new_kind/new_stage (leaf payloads follow). designation (HEB300, ...): CAT (profile catalogue) or FREE for the custom plate girder.

### en1990 (10)
ENUM: action_distribution, resistance_distribution, distribution (normal, lognormal, gumbel, ...; one `DistributionKind`), factor (gamma, psi0, k_fi, ...: the factor symbol, ENUM), inspection_level (IL1 to IL3), supervision_level (DSL1 to DSL3), structure_kind (building, rail_bridge, custom; compare en1991/en1994 `structure_kind`: one shared enum in `en1990`), quantity (strength, yield strength: ENUM of material quantities). FREE: justification, target (a combination or action id: REF).

### en1997 (7)
ENUM: design_approach (da1, da2, da3), design_situation (persistent, transient, accidental, seismic; fixtures use bsP/bsT codes from DIN 1054: ENUM with both), earth_pressure_mode (active, atRest, passive), pile_type (bored, driven, cfa, ...), wall_movement (free, rigid), tendon (strand, bar). CAT: soil_type (gravel, sand, ...: soil classification catalogue).

### en1995 (8)
ENUM: face (top, bottom), tension_side (tension, compression), uls_equation (6.10, 6.10ab), load_duration (permanent, long, medium, short, instantaneous), category (A, B, ...: EN 1990 category, shared with en1991), fastener_type (bolt, screw, nail, dowel, ...). SKIPPED (en1995 v7): orientation, grade (CAT into the strength-class registry).

### en1996 (2)
ENUM: imposed_category (A to E, shared with en1991 `category`), new_design_situation (accidental, seismic, persistent, transient).

## Cross-artifact observation

The same closed sets recur across artifacts and should live once in a shared crate under compliance, not once per artifact: action category (EN 1990 Table A1.1), structure kind, fire rating, bolt grade, steel grade, weld process and direction, design situation, soil classification. A per-artifact enum with the same variants would repeat code (AGENTS.md: repeated code must be close) and drift.

## Next steps (order)

1. en1992: position, column_position (4 variants), slab_system, cement_type, bend, action kind/source/category, delete `column_method`.
2. en1999: product_form, process, direction, ring, position, fabrication_class, then the CAT references.
3. Run `check.py` per artifact and `SEMIO_LANGUAGE=en repo ./📜️script.ts verify build-owners sync` after the crate edits (not run here: no builds were started and the `repo` MCP server was unreachable in this session).

Ready for rebuild: typed enums (en1992 BondCondition).
