# Physical Joinery R25

## Scope

Implemented real planar glazed sashes in the existing original-oak ribbon, metal-window, entrance-door and salon types. Each pane is bounded by four disjoint physical frame-material members, and its glass is trimmed to their clear opening. The existing two entrance leaves have two separate sash assemblies and meeting stiles; no fixed central mullion obstructs their passage. Added a physical intermediate-mullion contract for authored nonuniform panel stations. It does not manufacture a new historic station schedule.

The salon now has distinct top and bottom two-lane metal tracks. Each track is made from one thin web and three guide strips, preserving the two open channels rather than filling a rectangular metal envelope. Sliding pane/sash depth is placed in an actual channel, alternating lanes where additional pane stations are authored. This is closed-position construction geometry, not animation of the full travel mechanism. There is still only the existing one authored salon glass panel. It must not be described as a measured reconstruction of the complete original sliding-panel train.

## Primary Evidence and Limits

Read physical-architecture-audit-r24.md, facade-structure-r23.md, source-audit-r17.md and source-materials-r10.md. Independently inspected the local original FLC19414 ground-plan pixels: their scale and linework do not support a reliable glazing-bar register. The source book carpentry excerpts establish oak versus metal and the salon sliding operation, but its full p210–216 drawing pixels remain unavailable. Direct retrieval of [the original book](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1) again timed out through web access.

The [Fondation Le Corbusier owner page](https://www.fondationlecorbusier.fr/oeuvre-architecture/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-1931/) independently states that the original timber window frames were replaced with painted aluminium during restoration beginning1967. Downloaded and inspected owner photographs [5](https://www.fondationlecorbusier.fr/wp-content/uploads/2022/04/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-5.jpg), [8](https://www.fondationlecorbusier.fr/wp-content/uploads/2022/04/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-8.jpg) and [10](https://www.fondationlecorbusier.fr/wp-content/uploads/2022/04/realisations-villa-savoye-et-loge-du-jardinier-poissy-france-1928-10.jpg). They visibly establish multiple modern ribbon members and a salon division, but the salon photograph is heavily ramp-occluded and the page does not establish original1931 station identity. Consequently they do not justify transferring a uniform historic module schedule into all apertures. These photographs remain generated inspection inputs only.

Sash widths28mm oak/18mm metal, sash depths25/12mm, potential mullion widths35/22mm and track web3mm are expressly inferred and unmeasured. The outer existing frame envelope is preserved. No rebate, weather seal, drainage, roller or original hardware dimensions are claimed. The source key SAB2024-carpentry-construction-unmeasured deliberately distinguishes these constructed teaching details from measured book dimensions.

CW-1 remains the previously authored continuous glass band: no defensible source-bound pane station registration was recovered here. The optional CW station geometry was not added as unused speculative implementation. The long ribbons also retain their existing singleton pane schedules; the new mullion contract is verified with a deliberately nonuniform synthetic schedule, not passed off as an historic register. These remain significant physical fidelity gaps, independent of passing geometry/API tests.

## Test-First Verification

Created language-neutral 🪚️joinery.json before implementation with nonuniform stations0/.31/.72/1 and explicit dimensions. The initial Python/GEOS contract failed for missing physical mullions, as expected. Three final independent contracts pass: nonuniform mullions/sashes and glass nonintersection using GEOS; source salon two open track lanes and material identity; all source glazed filling solids pairwise nonoverlap and independent GEOS occupied volume agreement with model quantities, including two entrance leaf sashes without a central fixed mullion. The five earlier filling contracts also pass after the window glass-area oracle was updated to exclude physical sashes. Parent coordination owns broad model/schema/clip/native validation. Source element count is unchanged.

## Files

- Model 🐍️.py: narrow _filling_geometry joinery construction changes.
- Model 🔣️.json: inferred physical joinery type quantities only; no invented pane stations or added element records.
- Model 🧬️schema/🔣️.schema.json: optional attributed positive sash_width, sash_depth, mullion_width and track_web quantities.
- Model 🧪️tests/🪚️joinery.py and 🪚️joinery.json: three physical contracts and neutral fixture.
- Model 🧪️tests/🪟️fillings.py: sash-reduced glass-area oracle.

The integration author was asked to register the new module in the standard BIM load_tests hook. All research and results are preserved in this ticket report; no source copies, native GUI, shell commands, Git mutations or new runtime dependency were used.
