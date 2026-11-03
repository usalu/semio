# Open-items sweep wave 2: en1996 and en1999

Nothing compiled (no cargo, no builds). Rebuild lines appended to `🗑️generated/build-en1996/latest.md` and `build-en1999/latest.md`; request entries at the end of `📌️important/requests.md`. No git writes; mutation fixtures untouched (PATH-BUDGET freeze). Every file below is under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<🪨️en1996|🪶️en1999>/🏅️standards/🔖️1/🪆️subsets/✳️any/` ("ANY").

## 1. CHECK-HIGHLIGHT orphans (pre-empted by reading every emitted check family)

The law (`ownership_audit`) takes owners = scene entity base ids (en1996: walls; en1999: all 11 `ENTITY_KINDS` collections) + design case ids. `Owners::file` only sees top-level document arrays, so nested rows (openings, bars, ties ...) never own a check; they resolve through the wall id embedded in the check id or through the subject being the wall.

### en1996 (changed)
| family (check id) | subject before | owner now |
|---|---|---|
| wall evaluators (`en1996.<clause>.<wall>`...), `m-31/32/33/83/05/21/45` document cases (`row_case`, `wall_case`, bracing) | wall id (all `.bound(&wall.id..)`, bracing = first shear wall) | wall, unchanged |
| `en1996.8.6.chase-depth/extent.<chase>`, `en1996.8.5.joint-position.<joint>` | chase or joint id: NOT a scene entity base (scene entity is `wall:chase-<id>`), orphan | now `SubjectRef` entity = `wall_id`, path unchanged (`chases[id=..].depthM`); `🧬️schema/⚖️masonry/🪚️detailing/🦀️.rs` `chase_subject`, `joint_subject` |
| `en1996.8.chase-wall.<id>`, `en1996.8.joint-wall.<id>` (wall id does not exist) | chase/joint wall id of a missing wall | document-level prefix (reason: the owning wall does not exist) |
| `en1996.m-95.*` (behaviour factor, building) | entity "building" (not an entity) | document-level prefix `en1996.m-95.` (reason: concerns the whole building) |
| `en1996.integrity.*`, `en1996.subject.empty` | row or load case ids | already document-level |
| bar layout, lintel cover, bracing system, diaphragm, joint spacing | wall or bar id with wall id embedded in the check id | wall via embedded run, unchanged |
| design case rows `cases` | case id | owner and in `audit_owners` (declared or not) |
`OWNERS` (`🧬️schema/⚖️masonry/🧮️units/🦀️.rs:17`) is now `["walls","cases"]` + the four document-level prefixes; the previous list named nested collections that never exist at top level. Test: `chase_and_joint_checks_are_owned_by_their_wall_and_keep_the_row_path` (detailing tests).

### en1999 (changed once)
Read all `SubjectRef::new` sites of the 12 check modules: member, connection, sheet, shell, fire, fatigue detail/category, weld run, patch load, ring and transverse stiffener checks all use the row id of an owning collection; section element remedies and `3.2.alloy.<member>` carry the member via the embedded id; `en1999.3.2.form.<mat>` only fires for a material used by a member (redirect via `names_row`); `en1999.ref.*` and the six `*.empty` already document-level. Orphan class found: `en1999.6.1.outline.{props,match,torsion}.<section>` of a section no member references (subject = section, not an owner). Added the document-level prefix `en1999.6.1.outline.` in `🧬️schema/💡️inferences/⚖️checks/🧮️units/🦀️.rs:44`. (Used sections still redirect to the first member that names them.)

Still possible on the first run, by class (the test message lists the ids): a section/material used by no owner but giving other check ids than the above; en1999 `cases` is listed in `OWNERS` although the snapshot has no `cases` (harmless).

## 2. Collections: 3D + grips + outliner entry (verified by reading, the audit used literal matching)
- Outliner: both outliners render `ENTITY_KINDS` collections first and then "every other list of entities of the document" (`render_outliner_document`), so placements, sections, materials, outlineVertices, cases have entries (en1996 `✏️editor/🎭️modes/✏️edit/🪟️windows/🌲️outliner`, en1999 same path).
- en1996: walls, openings, concentrated, chases, joints gripped (`🎬️scene/🕹️handles`); lintels, piers, ties, prestress, confined, flanges, bars gripped in `🎬️scene/🧩️members/🦀️.rs:156-220` (dispatch `handles:140`); placements: move + rotate grips `handles:122-135` (writes `placements[id=WALL].x|y|yaw`, `insert-placement` kind exists). Gap closed now: diagrams `chase-depth:<wall>` (depth d vs remaining thickness, tabulated depth as limit line, marker drags `chases[id].depthM`) and `movement-joints:<wall>` (panel widths vs recommended spacing, markers drag `positionM`) in `🎬️scene/📈️diagrams/🦀️.rs`; test `chases_and_movement_joints_have_diagrams_whose_markers_drag_their_own_rows`. `cases` stays out of the insert palette by design (design cases are inserted from the catalogue browser, not the 3D palette).
- en1999: members, connections, sheets, shells, fire, fatigue, weld runs, stiffeners gripped; placements gripped through `move_handles` for members, sheets, shells (first drag inserts via `PlacementCommit`, `insert-placement` exists). Gap closed now: `materials` had no grip because it has no numeric leaf: new choice-stepper grip `<member>:product-form` on the member's material (`handles/🦀️.rs`, `choice_handle`, tokens = enum wire tokens), test `the_product_form_of_the_member_material_is_a_choice_stepper_on_the_member`. Alloy designation is a free catalogue string (not an enum), so no stepper (TYPED-ENUMS v3 territory).
- Real bug found and fixed on the way: en1999 field-meta choice tokens were kebab (`self-weight`, `butt-weld`, `fillet-weld`, `extruded-i`, `extruded-profile|tube|bar`, `drawn-tube`) while the enums, `🎲️field-metadata.json` and fixtures use camelCase wire tokens; the editor would have written unparsable values. Now camelCase; docstring of `DetailType` corrected.

## 3. Open items: state with evidence
### en1996
| item | state |
|---|---|
| placements per wall (121), openings/concentrated positions (122), cavity ties/flanges/confined/bars as entities (123) | done: `placements` rows + grips above; `WallOpening.position_m`, `ConcentratedLoad.positionM`, entity kinds tie/prestress/confined/flange/bar in `🎬️scene/🧰️kit:52-65` |
| `wall_profile`/`fire_state` exports (124/125) | superseded: scene has its own `🎬️scene/🧮️profile` (`profile::governing`) which the diagrams use; no public export added |
| reinforcement at real positions with `Pattern::Points` (229) | open, visual refinement (bars still use `Pattern::Along`, `🎬️scene/🔩️reinforcement:36,49`); not data-blocked |
| NDP K, f_vk0, f_vlt2 (285, 359) | done in code: `normative-tables:114-136`, registry rows `ndp.en1996.k.*` 170-205 match the public list incl. 0.80/0.65 CS thin; printed-standard check of `γ_M` row and unresolved E96-xx stays blocked (printed standard) |
| masonry 3886 lines (519), diff 3715 (786), mutations 3792 (873) | done: scan now masonry 1844, diff 480, mutations 1177, scene 3195 (80 %), max crate in en1996 = scene |
| diff JSON-schema crate headroom | done: `semio-s-artifact-norm-en1996-diff-json-schema` 119 lines / 52772 B (was 3804 lines); `mutations-json-schema` 1717 lines |
| oracle gap `phi-m` lambda, A_1, u (674) | done: `phi_m_terms` in `🧬️schema/📐️masonry-formulas/📏️factors-and-geometry/🦀️.rs` (`phi_m` delegates), adapter `🧬️schema/💡️inferences/🧪️tests/🎯️oracle-wiring/🦀️.rs` outputs lambda, A_1, u, Phi_m; blessed cases `en1996.G.phi-m-*` carry a sympy reference |
| "Invalid tie index." English-only text (864) | done: string no longer exists in the tree (grep) |
| NDP tagging review (813) | done earlier (`[done] ndp v3`, requests.md:965) |
| results-config (126) and hub manifests (127) naming `norm-en1996-inputs/results` | blocked: other owners (results-config, hub) |
| build/golden items (305, 401, 458, 547, 788) | blocked: build runner; goldens (annex E panel, chases, joint diagrams) must be blessed after the first green build |
| path budget | done: longest en1996 path 253 B, en1999 254 B (< 256) |

### en1999
| item | state |
|---|---|
| placements per member/sheet/shell, add-placement kinds (113, 447, 488) | done: `placements` model, `insert-placement`, `change-placement-*`, grips |
| free outline polygon/vertices (114), explicit weld runs (115), `member_resistances` (116), per-entity units (117) | done: `outlineVertices`, `weldRuns`, `member-checks:629 pub fn member_resistances`, `🧮️units` rows per collection |
| `k_sup` on buckling lengths (262) | done: applied only as default when length <= 0 (`member-checks:458-459, 659-660`) |
| S1 eta and HAZ plate (353) | done (v10 audit): eta only for webs, HAZ per E2-03 |
| over-gate crates (517, 783, 784, 809, 936) | done except bytes: diff 1818 lines, mutations 2141, model 2497, scene 3353, check-parts 1638. `semio-s-artifact-norm-en1999-checks` is 3223 lines but **261227 B of 262144 B (917 B headroom)**; next growth must split: move `🥫️thin-wall-checks` (14 KB, uses `use super::*`) into the `📏️parts` crate, needs a compiler loop, not done blind |
| boltsPerRow SetFieldRule (879) | done: `edit-rules:146` `change-conn-bolts-bolts-per-row` |
| `use mut_member::change_member_*` E0432 (927) | done: `🏗️member/🦀️.rs` mounts all nine, `🔗️connection` + `🔗️connection-fasteners` mount the connection leaves; collection-rows crate is 1 file now |
| umbrella `script.ts` (1268) | done: every `semio-s-artifact-norm-en1999-*` package name occurs in `📦️packages/🦀️rust/📜️script.ts` (python check) |
| oracle gaps (675): free outline It/Iw x3 | done: `section_polygon` adapter now outputs It and Iw via `mechanics::analyse_section` (same kernel as the free-outline torsion check) |
| oracle gaps: tee, elliptical hollow kinds, channel `y_sc`, angle `I_major/I_minor` | blocked: new geometry kinds/functions in `part_1_1::section_geometry` (checks crate at the byte gate, see above) and compile loop |
| fixtures > 256 B (441) | done by measurement (max 254); PATH-BUDGET flattening is the path owner's job, mutation fixtures frozen |
| web crippling coefficients | blocked: ten values null, printed EN 1999-1-4 6.1.7 table needed |
| materials/product form: unverified values | none invented; PSI oracle table literals still open (should come from `ndp.en1990.psi.*`) |

## 4. Coordination
- TYPED-ENUMS v3: en1999 field-meta tokens now camelCase (requests.md note).
- RC-THREADING v2: no field touched.
- Unverified normative values introduced: none. Existing unverified ones unchanged (chase 30 mm / 115 mm, joint spacing 12/8/6 m, M-95 q ranges, Annex E cells).

## 5. Likely first-compile fix-ups
`m::joint_spacing_limit_m` / `m::kind_words` via the scene `masonry` glob (re-exported through `⚖️masonry/🦀️.rs:23,27`); `choice_handle` options type `Vec<(String,&str,&str)>`; `Material` import in the en1999 oracle adapter; `FieldBinding.value_at` in the new diagram test; `Axis.unit` field in the same test.
