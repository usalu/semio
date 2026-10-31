# Nakagin Remodel — Final Shared Model Record

This record supersedes the earlier 70/70 and regular-facade reconstruction.

## Evidence and Scope

The A606 measured-survey publication inventories **A 76 / B 64**, total 140. The independently viewed floor chart gives A floors 3–13 as **2,7,7,7,8,8,7,8,8,7,7** and B floors 3–11 as **2,8,8,7,8,8,7,8,8**. Exact room suffixes must not be interpreted as counts.

Compact placement frames and six physical door/window families come from the existing repository DSL/CAD. Source core 1 is the tall A tower; source core 0 is the short B tower. The adapted frame maps source regular floor zero to actual floor 4, preserves three sublevels and bridge gaps at floors 6, 9 and 12, and shifts upper geometry 2.75 m so the sparse third floor begins above the 7.5 m office roof. The precise two floor-3 positions and additional A omissions on floors 4/5/13 remain explicitly assumed; their counts are documented but the complete measured room plan was not publicly available. This is a detailed, physically checked teaching reconstruction, not a fully surveyed as-built replica.

The source hierarchy is: [A606 survey inventory](https://readyfor.jp/projects/time_capsule_2072/announcements/241581), [floor-count chart](https://note.com/kotatsustories/n/nb6c501853f0f), [original 1973 Domus drawings](https://www.domusweb.it/en/from-the-archive/2010/12/21/kurokawa-s-capsules.html), [construction specification](https://capsule-architecture.com/1972nakagin.html), [M+ A806 accession](https://www.mplus.org.hk/en/collection/objects/unit-a806-nakagin-capsule-tower-19701972-tokyo-japan-2024104/), repository CAD/DSL and the GLUON visual archive examined by the exploration agent. M+ overall dimensions retain their published order without inventing axis labels.

## Implemented Contract

- Schema-first explicit floor/group/item records produce 74 facade branches: 66 pairs and 8 single capsules. Slot.path remains BUILDING/TOWER/STORY/ATTACHMENT FACADE; sublevel, actual floor, physical family, centre, orientation and actual door plane are independent fields.
- NamedPart and Assembly expose renderer-neutral, closed indexed solids. Six physical families implement actual end/long-wall apertures and end/side entries, handed fasteners and 1.3 m nominal glazing. The 1.2 mm steel skin is sourced separately from assumed truss and lining dimensions.
- Reference interiors include a front floor bed and four drawers, curved wetroom door/enclosure, bathtub, washbasin, toilet, TV, audio console, telephone, media cabinet, folding desk, lighting and detector. Handed layouts preserve the entrance route. Proposed A–D furniture programs require interior=study; they are independent from physical families.
- Concrete height derives from authored crowns plus an explicitly assumed 2.8 m service head. Final core heights are A 42.1333 m and B 36.6333 m; diagonal roof-head tips are 53.4933 m and 47.9933 m. The metal roof enclosure has an open rear, exposed bracing and water tank. Height and overhang are live parameters.
- The explicit 4/2/2 attachment cadence has a phase-0 U-shaped landing serving four ports, north-west phase-1 and south-west phase-2 landing pairs, and three stair flights per storey. B rotates this topology 180°. Actual segmented lift fronts open only at connected phase-0 stops; lift approaches begin outside the closed door leaf. Local landings replace continuous low-headroom floor rings. Ground doors and thresholds open to the pilotis. Service risers occupy a nonwalking pocket. Partial-height core overrides clip higher openings and landings.
- The irregular podium roof extent replaces its bounding rectangle. Ground-level pilotis are open; only the upper level is glazed. Every slab is clipped around both core holes. Basement circulation continues the same three-flight turning cadence through two local corner platforms; shaft walls continue below ground and ring footings leave the lift pit open.
- Bridge decks at actual floors 6/9/12 are at 15.95/24.2/32.45 m, using the compact source layout. All capsules begin at or above the podium roof.
- building_assembly supports AssemblyProgress events and cooperative cancellation through AssemblyCancelled, checked at core, flight, part and capsule boundaries. No external runtime library is required.

## Final Verification

**58 Python tests passed on the raised supported-equipment snapshot (34.248 seconds).** Independent NumPy/rhino3dm oracles cover all six closed positive-volume families, all 140 world placements and actual entry planes, six-family analytic glazing circles against transformed Rhino circles, real portholes, material layers, complete door approach volumes, top door heads inside concrete, 2 m stair centreline headroom, capsule/core/capsule/bridge/podium separation, podium shaft holes, pilotis, basement stair connections and full-depth outer-core wall load paths to ring footings, study/reference interior separation, furniture support by the finish slab and all 140 threshold collars touching the authored door ports, partial core overrides, progress completion and cancellation before/mid-generation.

**AJV passed:** authored schema accepted, all **29** invalid portable models rejected; **1,229** quantified leaves have valid provenance. Independent raw-JSON counting reproduces A76/B64 and every floor count. Program counts remain A35/B33/C35/D37. Climate and energy variant oracles continue to pass.

Files changed: shared JSON data, schema, Python geometry, portable JSON cases, Python tests, independent JavaScript tests and bilingual README. The eighteen native scene/language checks have durable success records; the final circulation snapshot passes the independent support-graph and mesh-clearance gates and is frozen for final visual captures.

The final isolated support correction adds a finish slab from the lining to the reference furniture feet and family-specific metal threshold collars over the nominal-envelope/source-port gap. It preserves all exterior poses, furnishing positions and energy inputs. The support geometry passed its 52-test gate.

The final study-only correction constructs every primary bed, bath, cabinet, desk and sofa footprint from its authored width, length and count. Local fitting frames adapt placement for end/side entrances; every study remains within the room, non-overlapping and clear of the entrance. An independent portable fixture validates exact dimensions and areas across all four studies and all six families. Reference interiors, exterior poses and energy inputs are unchanged. The latest complete aperture/ground-entry/bridge gate had 58 Python passes and AJV acceptance plus 29 rejected invalid models. The final independent circulation gate is recorded separately in circulation.md.

The late aperture correction decomposes side-entry shell and lining panels around the complete rectangle and clips only the matching low rounded corner. Independent finite segment/triangle tests sample five widths and three heights, including the low curved zone, with solid controls outside the opening. Porthole rays also check steel-truss clearance. These changes preserve every exterior capsule pose, furniture placement and energy input.

The independent circulation oracle exposed a bridge approach sign error: doorway/approach solids were centred at world Y=+.9 while all three decks were at Y=-.9. Both core openings and approaches now share the authored deck centreline; a portable expected coordinate and independent NumPy-bounds regression cover all six approaches. Capsule poses are unchanged.

Actual-mesh clearance testing isolated two outer roof lobby strips beneath the low edge of the oblique machinery enclosure. The final roof level retains the turning corners and a 1m inner service route; unused outer U arms stop at lower floors. Capsule, lift and bridge poses are unchanged. This avoids claiming a route underneath the low roof slope.

The inferred B roof water tank is placed opposite its rotated west service route; A retains the tank west of its east route. This removes the last identified actual-solid roof-route obstruction without changing capsules, circulation terminals or energy inputs.

Roof equipment now has explicit assumed load paths: each tank rests on a steel cradle with four checked disk-bearing points, two cantilever beams/braces and two posts bearing on the structural roof ring. Every support stays outside the central lift opening and opposite the flat service route. The central vent has a four-piece curb bearing on actual shaft-wall tops and retains an open 0.7×0.5m vent aperture. Portable cases and independent NumPy geometry checks validate contact and shaft clearance. Tank XY footprints and the vent top remain unchanged. The tank/frame datum is raised by an explicitly assumed 0.65m to clear the measured final-flight obstruction; posts still start at the roof ring. Expanded mesh clearance includes stair centres as well as flat connector strips.

The resumed gate reread the authoritative files and confirmed the raised stand survived interruption. The schema accepts all 1,229 provenanced quantities and rejects 29 invalid portable models. Equipment tests pin the 0.65m raise, multiple bearing points, frame-to-roof contact, shaft clearance, and vent-curb support/opening. The expanded circulation gate additionally checks continuous stair-centre paths against every physical mesh, beyond the flat connector strips.

## Accepted Raised-Equipment Snapshot

Final shared verification: **58 Python tests, 34.248 seconds, PASS**. A strengthened isolated NumPy contact test additionally verifies every post→beam→cradle→tank bearing interface (0.124 seconds, PASS). AJV accepts the authored model, rejects all 29 invalid portable models, and validates 1,229 provenance-bearing quantities.

The independent circulation suite passes **12 tests in 9.480 seconds**. Its actual-solid support graph reaches all 140 capsule landings, 26 lift approaches, six bridge approaches and three bridge decks from only the two exterior ground thresholds, with complete independent NumPy/NetworkX edge/reachability agreement. Actual-triangle/inside-solid clearance checks cover continuous flat walking strips and every continuous stair-tread centreline tube against all physical parts and conservative capsule envelopes. This is bounded geometric assurance for the authored paths and widths, not surveyed or code-compliance certification. See circulation.md for the independent portable fixtures and scope.

The geometry source is frozen. Final representative visual and Tower lifecycle captures are coordinated by root.
