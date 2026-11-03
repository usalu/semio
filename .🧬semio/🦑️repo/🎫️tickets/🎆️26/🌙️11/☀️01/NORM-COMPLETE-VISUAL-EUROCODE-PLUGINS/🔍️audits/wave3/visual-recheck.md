# 🔍️ Visual Recheck, Wave 3 (code, not reports)

Read-only recheck of `visual-audit.md` by W3-VISUAL-RECHECK (Haiku). No source edits, no builds, no repo runs, no git writes.

`ART(code)` = `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 🧭️ Method and limits

- A full rerun of `measure.py` and `checkbind.py` was queued behind `heavy.sh` (graph-plan gate, several builds running). It was stopped and not used. Its scratch copies are in `🗑️generated/visual-recheck/`.
- Evidence below comes from targeted greps of the code at `ART(code)` (kit registries, diagram modules, grip sources, showcase files, check-binding literals). Grip counts re-run the audit's name-based proxy for the flagged collections only.
- The name-based diagram and grip checks are upper bounds, and the literal check-id binding check is weak where the scene binds checks by prefix (en1998) or through an owner table (en1991, en1997). Treat "closed" as "wired in code, not proven in the viewport".
- Not re-derived: the full per-family check binding for en1990, en1993, en1995 and en1996, and the 3D/palette status of every code beyond the rows listed. The audit values stand for those.

## ✅ Verdicts on the audit's 25 gaps

| # | Gap (audit) | Verdict | Evidence |
|---|---|---|---|
| 1 | en1998 check binding, 29 of 69 families unbound | partial | `ART(en1998)/🎬️scene` has 47 `binding_prefix`/`binding_exact` call sites (utilisation bindings). Still no scene literal for `en1998.1.*.modal.massParticipation`, `elevationRegularity`, `regularity.claim`/`derived`, `site.aGr.derived` (schema-produced ids, see `ART(en1998)/🧬️schema/💡️inferences/🌍️full-seismic-check/…`). Modal, combination and limitState families stay unbound. |
| 2 | en1997 check binding, 43 of 54 unbound | closed | `ART(en1997)/🎬️scene/🧰️kit/🦀️.rs` ~l.460-475: prefix table (`("en1997.anc-01.", "anchors")`, `dew-*`, `gi-*`, `g-02`, `geo-f03`, …). |
| 3 | en1992 check binding, 26 of 36 unbound | open | `ART(en1992)/🎬️scene`: `en1992.m-02`, `m-08`, `integrity`, `b-01`, `annexC`, `a-16`, `en1992-2.6`, `2.7` have 0 literal hits. `en1992-4.*` and `en1992.3` have 1 hit each. No `report_owner`-style table in en1992. |
| 4 | en1998 unregistered `tieBeams`, `slopes`, `placements` | closed | `ART(en1998)/🎬️scene/🧰️kit/🦀️.rs` l.126-128 `kind("tieBeam"…)`, `kind("slope"…)`, `kind("placement"…)`. Labels EN/DE present. |
| 5 | en1997 `characteristicBases` no 3D, grip, palette | partial | 3D and palette: registered at `🧰️kit/🦀️.rs` l.75 and l.97. Diagram: snake-case hit in `🎬️scene/📈️diagrams/🦀️.rs`. Grip: 0 hits in `🕹️handles`. Still no grip. |
| 6 | en1998 diagrams missing for 16 of 17 | partial | Now produced: `buildings` (`📈️diagrams`), `assessments` (`🏚️assessment/🦀️.rs` `diagrams`), `foundations` and `slopes` (`🌍️geotechnical/🦀️.rs` `diagrams`), `diaphragms` and `accelerograms` (`🧱️horizontal-elements/🦀️.rs`, name match). Still open: `bridges`, `silos`, `tanks`, `towers`, `retainingWalls`, `piers`, `bearings`, `joints`, `tieBeams`, `placements`; `soilLayers` only inside the foundation soil profile. |
| 7 | en1998 numeric grips not bound | open | Proxy re-run: `foundations` 5/30, `buildings` 6/21, `piers` 7/12, `bridges` 7/13, `silos` 3/8, `tanks` 4/8, `retainingWalls` 3/8, `soilLayers` 1/5 (all unchanged). `assessments` 7→8/25. |
| 8 | en1993 `joints` numeric grips 12/54 | open | Unchanged (12/54) in `ART(en1993)/🎬️scene-members/🕹️handles/🦀️.rs`. |
| 9 | en1993 `platedPanels` 15/45, `jointPlates` 11/19, `hollowJoints` 5/17 | partial | `platedPanels` 15→18/45. `jointPlates` and `hollowJoints` unchanged. |
| 10 | en1993 other numeric grips (`coldFormedMembers`, `siloShells`, `towerLegs`, `tensionComponents`, `craneRunways`, `bridgeFatigue`, `piles`, `builtUpMembers`) | open | `coldFormedMembers` 5→7/26 and `tensionComponents` 1→2/20 (small). The rest unchanged. |
| 11 | en1993 `sections` and `memberActions` not in palette or 3D | open | `ART(en1993)/🎬️scene-members/🧰️kit/🦀️.rs` ENTITY_KINDS (l.58-85) has no `sections` or `memberActions`. `sections` is drawn in diagrams (`📈️entity-diagrams`, `📈️design-diagrams`). |
| 12 | en1993 check binding 12 of 23 | partial | `en1993.1-3`, `3-1` have one hit each. `en1993.4-1`, `sections`, `1-11` have zero hits. |
| 13 | en1990 `members` 5/30, `bridgeSls` 2/14, `fatigueSpectra` 0/3, `effects` 0/1 | partial | `members` 5→13/30. `fatigueSpectra` 0→2/3. `bridgeSls` 2/14 unchanged. `effects` 0/1 and no `🕹️handles` hit. |
| 14 | en1990 check binding 8 of 17 | partial | `en1990.e10`, `en1990.members`, `en1990.table-2` have 0 hits. `sls`, `equ`, `6` have one hit each. |
| 15 | en1996 `walls` 7/31 grips; check binding 15/29 | open | `walls` 7/31 unchanged. `en1996.2` and `en1996.m-16` have 0 hits. |
| 16 | en1995 typed grips: `tapers` 0/7, `cutouts` 0/7, `decks` 2/16, `floors` 2/10, `diaphragms` 1/10, `trusses` 2/10, `bracings` 1/8, `members` 8/21 | open | All unchanged. No `🕹️handles` or `🧱️entities/🕹️grips` hit for `tapers` or `cutouts`. |
| 17 | en1995 check binding 9 of 17 | open | `en1995.material` and `en1995.1-2` have 0 hits. `en1995.t-18` has one hit. Not fully rechecked. |
| 18 | en1997 numeric grips `piles` 4/24, `layers` 2/12, `retainingWalls` 5/17, `labTests` 1/9, `anchors` 4/10, `embeddedWalls` 4/10, `pileGroups` 3/9 | partial | `retainingWalls` 5→10/17. `embeddedWalls` 4→6/10. `layers` 2→3/12. `piles` 4/24, `labTests` 1/9, `anchors` 4/10, `pileGroups` 3/9 unchanged. |
| 19 | `placements` not registered in en1991, en1992, en1993, en1994, en1995, en1996 | closed (3D, palette, labels) | Registered at: en1990 `🧰️kit/🦀️.rs` l.60, en1991 `🎬️scene-members/🧰️kit` l.169, en1992 l.69, en1993 `🎬️scene-members/🧰️kit` l.85, en1994 l.63, en1995 l.66, en1996 l.66, en1997 l.76, en1998 l.128, en1999 l.69. Diagrams for `placements` are still missing in most codes (see rows 20, 24, 25 and below). |
| 20 | en1991 diagrams missing for 9 of 28 | open | No `Diagram2d` hit for `roofSurfaces`, `specialVehicles`, `craneHoists`, `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines` in `ART(en1991)/🎬️scene*`. `placements` only as a layout word. |
| 21 | en1991 showcase missing for `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines` | open | 0 hits in `ART(en1991)/🖼️assets` and `📚️examples` for those five. Also 0 for `craneHoists`, `roofSurfaces`, `specialVehicles`. |
| 22 | en1992 grades not bound to pick, grip, diagram | partial | `concreteGrades` now has a grade legend diagram (`🎨️grades/🦀️.rs` `legend_diagram`). Still no grip for `concreteGrades`, `reinforcementGrades`, `prestressSteels` (0 hits in `🕹️handles`). `RcMember.sectionPoints` no grip. |
| 23 | en1992 `cases` and `grades` no diagram | partial | `grades` covered by the legend diagram. `cases` still has no `Diagram2d` hit. |
| 24 | en1995 diagrams missing for `cutouts` and `placements` | open | `cutouts` is drawn as a case row (`🧱️entities/🏷️typed/🦀️.rs`), not a `Diagram2d`. `placements` has no diagram. |
| 25 | en1999 diagrams missing for `placements`, `connections`, `fatigueCategories`, `ringStiffeners` | partial | `connections` now in `📈️diagrams/🔩️connection-diagrams/🦀️.rs`. `fatigueCategories` (`🔥️fire-and-fatigue`) and `ringStiffeners` (`🥫️sheets-and-shells`) have no `Diagram2d`. `placements` only as a word hit, unverified. |

Also open from the audit's "below the top 25" list:
- en1993 `frameMemberBases` diagram and handles: no `Diagram2d` hit and no `🕹️handles` hit (`frameBars`, `fatigueDetails` also no handles hit).
- en1990 `trafficGroups`: no handles and no numeric grip (not gripped by design, to confirm).
- en1994 `placements` palette: registered (closed). Diagram: still a word hit only.
- en1998 `bridges` and `assessments` diagram: `assessments` closed, `bridges` open.

## 🧭️ Live editing

Each of the 10 codes has one live test file under `ART(code)/🎬️scene/🧪️tests/🔬️live/🦀️.rs`. en1998 has explicit drag tests for silos, piers and towers (`🎬️scene/🧪️tests/🔬️live/🦀️.rs` l.197). Those tests are not re-run here, so live editing is wired for the families that have a grip, and not verified for the rest.

## 🧾 Check families with no visual

Families with no literal scene binding after this recheck (candidate list, not exhaustive): en1992 `m-02`, `m-08`, `integrity`, `b-01`, `annexC`, `a-16`, `2.6`, `2.7`; en1990 `e10`, `members`, `table-2`; en1993 `4-1`, `sections`, `1-11`; en1995 `material`, `1-2`; en1998 `modal`, `limitState`, `combination`, `cyclicShear`, `regularity`. en1996 `m-16`, `2`. en1997 family table now complete for the listed families.

## 🏆 Top 15 open gaps by user impact

1. en1998 numeric grips on `buildings` (6/21), `foundations` (5/30), `piers` (7/12), `bridges` (7/13), `silos`, `tanks`, `retainingWalls`, `soilLayers`. Largest seismic input set with no drag.
2. en1993 `joints` numeric grips 12/54, `jointPlates` 11/19, `hollowJoints` 5/17. Connection design inputs are the most edited.
3. en1992 check binding gaps (`annexC`, `integrity`, `m-02`, `m-08`, `b-01`, `a-16`, `2.6`, `2.7`). Checks with no visual result.
4. en1991 diagrams missing for `specialVehicles`, `craneHoists`, `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines`, `roofSurfaces`.
5. en1991 showcase missing for `footbridges`, `railDynamics`, `loadDispersals`, `bulkSolids`, `rotatingMachines`, `craneHoists`, `roofSurfaces`, `specialVehicles`. Makes the new collections invisible to the examples gallery.
6. en1998 diagrams missing for `bridges`, `silos`, `tanks`, `towers`, `retainingWalls`, `piers`, `bearings`, `joints`, `tieBeams`, `placements`.
7. en1993 `sections` and `memberActions` not in ENTITY_KINDS (no 3D pick, no palette, no EN/DE labels).
8. en1995 typed grips `tapers` 0/7 and `cutouts` 0/7, and `decks` 2/16, `floors` 2/10, `trusses` 2/10.
9. en1990 `effects` (no grip, 0/1), `bridgeSls` 2/14, `members` 13/30 (improved).
10. en1996 `walls` numeric grips 7/31, and check families `m-16`, `en1996.2`.
11. en1992 `concreteGrades`, `reinforcementGrades`, `prestressSteels`, `RcMember.sectionPoints` have no grips. Material inputs have no drag.
12. en1997 `characteristicBases` has no grip. `piles` 4/24, `pileGroups` 3/9, `labTests` 1/9, `anchors` 4/10 numeric grips still low.
13. en1999 `fatigueCategories` and `ringStiffeners` have no diagram.
14. en1993 `frameMemberBases` (no diagram, no grip), `frameBars` and `fatigueDetails` (no grips).
15. en1995 `cutouts` diagram (drawn as a case row only) and en1992 `cases` diagram.

## 📌 Summary of closures since wave 2

- Closed: en1997 check binding table; en1998 registry for `tieBeams`, `slopes`, `placements`; `placements` registry in all ten codes; en1992 `concreteGrades` legend diagram; en1999 `connections` diagram; en1998 diagrams for `assessments`, `foundations`, `slopes`.
- Partial: en1990 `members` and `fatigueSpectra` grips; en1997 `retainingWalls`, `embeddedWalls` grips; en1993 `platedPanels`, `coldFormedMembers` grips; en1997 `characteristicBases` (3D and palette only).
- Still open: en1998 numeric grips, en1993 joints and plated panels, en1991 diagrams and showcase, en1993 `sections` and `memberActions` registry, en1995 typed grips.
