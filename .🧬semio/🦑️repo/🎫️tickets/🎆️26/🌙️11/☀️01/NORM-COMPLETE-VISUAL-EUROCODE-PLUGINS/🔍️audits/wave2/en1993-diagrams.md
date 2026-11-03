# EN 1993 wave 2 diagrams (artifact owner): gaps 2, 3 and 18 of editor-ux-audit

Nothing compiled (rule). No git writes, no `repo run`. Rebuild line appended to `🗑️generated/build-en1993/latest.md`. Gates: handle-paths `en1993 0 uncovered`; source-boundary scan was started through `heavy.sh` but had not finished at report time (the new file is 19.7 KB, the diagrams closure stays far below 230 KB / 28 files / 3500 lines).

## Verification first (the audit was stale)
- Diagrams already present before this task: members (buckling, LTB, M-N, class), fatigueDetails (S-N + rainflow), fireExposures, joints (M-phi with stiffness classification bands, gusset/block-tearing/haunch sketches), restraints, weldGroups (stress vectors), hollowJoints, parametricSections, frameBars (buckling modes), siloShells, platedPanels (corrugated shear, effective width sketch), bridgeFatigue (deck strip), coldFormedMembers.
- Gap 3 (profile grips): already present as `section_handles` in `🎬️scene-members/🕹️handles` (sections h, b, t_w, t_f on the member that references the section; derived properties are not gripped). No change.
- Gap 18 memberActions grips (N, V_z, M_y magnitude arrows) and 3D load glyphs on the members: already present (`glyph_handles`, `🏹️loads-and-supports`); loadCases already had a 3D card, an outliner/insert kind and a move grip.

## Added
- New module `🎬️scene/📈️design-diagrams/🦀️.rs` (mounted in `📈️diagrams` next to entity/annex diagrams, pure projection of `(document, report)`):
  - `pile-mv:<id>` sheet pile M-V interaction polygon after corrosion with the (V_Ed, M_Ed) design point (EN 1993-5 5.2.2).
  - `bridge-sn:<id>` S-N curve of the detail category (with size effect) and the damage-equivalent range gamma_Mf Delta sigma_E,2 at 2e6 cycles (EN 1993-2 9.5.2), log axes.
  - `crane-wheel:<id>` bending moment envelope over the wheel position on the runway span with the governing midspan wheel marker.
  - `panel-rho:<id>` plate buckling reduction rho(lambda_p) with the panel's design point (uniform compression).
  - `load-case-n:<id>` and `load-case-my:<id>` per load case bars of N and M_y over its member actions.
  - `ladder:<collection>:<id>` utilisation ladder (bar per check, limit line at 1.0) for towerLegs, tensionComponents, storeys, builtUpMembers, craneRunways, jointRows, frameBars, frameNodes, jointPlates, foundationBlocks, piles; empty when the entity has no finite check.
  - Focus (`Some(entity id)`) narrows to that entity like all other diagram modules.
- Handles: `case_handles` on loadCases: choice grips for action kind and psi category (paths `loadCases[id=..].kind|category`, both have edit rules; the card now advertises its handle ids).

## Tests
- unit: `the_design_point_diagrams_exist_validate_and_carry_the_operating_point`, `every_check_carrying_collection_gets_a_utilisation_ladder` (validate, table twin, marker positions, log axes).
- live: `design_point_diagrams_follow_edits_and_a_focused_projection_equals_the_full_one` (30 random edits of pile moment, bridge range, wheel load, panel thickness, member action; each focused diagram equals the same diagram in the full set; pile marker follows the edit).
- Existing `every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws` covers all new diagrams on every showcase.

## Not done / open
- towerLegs N-M: a tower leg row has no section/axial capacity leaves (only lattice slenderness, wind and guy data), so no N-M polygon is honest; it gets the ladder. frameNodes/frameBars internal-force diagrams need a frame solver result that no check exposes; only the ladder (and existing buckling-mode diagram) exists.
- Crane wheel diagram models one wheel on a simply supported span (no wheel spacing leaf exists). Plate rho uses psi = 1.
- Unverified by compiler: all of the above (read-compile only), including `Diagram2d: PartialEq` for the live test and the `Limit` with `AxisRef::X` for the ladder.
