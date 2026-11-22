# EN 1998 bridge and pipeline design cases

## Continuous bridge deck

The first-generation bridge catalogue now screens the continuous deck in §3.3 and computes Set A by support. Set B computes interior and end support displacement magnitudes for alternating signs. A third case combines the larger absolute pseudo-static structural effect with the inertia result in the same direction using SRSS. The analyst enters the applicable national thresholds and decorrelation distance, selects the governing strain across adjacent soil types, and provides the results of the three structural analyses. All 29 formula and boundary cases reproduced independently in SQLite and Rust. [EN 1998-2:2005+A2:2011 §3.3](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.2.2005.pdf)

## Buried welded steel pipeline

The 2006 pipeline case uses independently entered longitudinal and transverse design waves. It verifies the perfect-bond basis before comparing a conservative envelope of axial and bending strain with the tensile and compressive limits. New cases start with the stability, material and wave basis unconfirmed. All 18 cases reproduced independently in SQLite and Rust. ST-03 still covers global buckling, permanent ground movement, concrete pipes, unwelded joints and fault crossings. [EN 1998-4:2006 §6.5.2 and Annex B](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

Native integrated tests and signed browser verification remain pending.

The properties editor now exposes saved case rows, case measurements and evidence. Snapshots, case changes, nested diffs and SQLite archive rows carry the same values. The archive suite passed 30 tests with 4,946 assertions. The wire harness also validated the snapshot, all seven case mutations and a nested case diff through the production TypeScript reader and Ajv. The independent archive fixture now expects 218 scalar words, as confirmed by SQLite and the canonical schema.

## Remaining buried-pipeline design provisions

Clauses 6.3.4 and 6.5.1 distinguish stable ground from segments subject to concentrated or permanent soil movements. Clause 6.5.3 uses the §6.5.2 welded-steel strain limits for the response of the affected segment, including both local and global buckling. The existing wave-passage case addresses a separate load mechanism. A differential-movement case needs an identified fault, landslide or lateral-spreading model and the resulting extreme tensile and compressive strains and global-buckling assessment over that segment. [EN 1998-4:2006 §§6.3.4 and 6.5](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

For concrete pipes, §6.5.2 requires the concrete and steel strain limits under the governing combined axial and bending response, plus a separate damage-limitation check on residual cracking and leakage. For an unwelded joint, the governing evidence is its axial movement and rotation together with the specified admissible damage. The catalogue needs the selected concrete design basis, joint capacity evidence and the combined response. [EN 1998-4:2006 §6.5.2](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

Clause 6.6 adds design considerations for fault crossings, including alignment, burial depth and soil restraint, wall thickness and backfill control on both sides of the fault, and avoiding geometry that anchors the pipe. Its 50 m and approximately 15 m provisions address different construction measures and need distinct evidence. ST-03 remains open for this workflow. [EN 1998-4:2006 §6.6](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

## Differential movement in a welded steel pipeline

C98-19 verifies the affected segment using the maximum combined tensile strain, maximum magnitude of compressive strain and global buckling utilization supplied by the project analysis. A missing or invalid input returns an unverified basis. The strain limits share the existing welded-steel criterion with C98-11. The assessment retains separate results for tension, local buckling and global buckling. The project analysis supplies the site deformation and global resistance. Concrete pipe strains and unwelded joint limit checks are covered below. Site-response analysis, project capacities and fault-crossing detailing remain in ST-03. [EN 1998-4:2006 §§6.5.2 and 6.5.3](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

All 15 boundary and evidence cases reproduced in the production evaluator and independent SQLite calculation. All 18 cases for the existing wave passage kernel also passed after sharing the strain limits. The native integrated catalogue regression is wired to the same fixture. The C98-19 example preserved its evidence and numeric values through the canonical archive and mutation/diff wire readers. The all-Norm archive suite passed 500 tests with 19,634 assertions.

## Concrete pipes and unwelded joints

C98-20 compares the governing combined concrete and reinforcement strains with the EN 1992-1-1 limits identified by the designer. It separately checks reinforcing-steel tension under damage limitation against the strain cap supported by project residual-cracking and leak-tightness evidence. C98-21 compares joint rotation with the documented rotation capacity at the entered axial opening or closing displacement. Equation inputs and compatibility evidence remain editable in the existing case editor. A missing or nonfinite demand, absent qualification or invalid limit produces an unverified basis. The production evaluators agreed with SQLite in 11 concrete and nine joint scenarios. [EN 1998-4:2006 §6.5.2](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

Clause 6.6 recommends relatively thick pipe and controlled backfill for 50 m on each side. The approximately 15 m oversize trench concerns substantially unsuitable existing soil. The alignment, cover depth, interface restraint and avoidance of anchoring features depend on the crossing. The available text supplies no universal crossing angle, burial depth or trench-length tolerance. [EN 1998-4:2006 §6.6](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)
## Fault-crossing protection lengths

C98-22 checks the extent of relatively thick-walled pipe and controlled backfill against 50 m on each side of a documented fault crossing. Results stay separate for the two sides. If existing soil differs substantially from the recommended backfill, the entered trench lengths are reported with the approximate 15 m reference. The application records insufficient evidence where a design input is missing. The quantitative case leaves the crossing alignment, burial depth, interface friction and anchorage decisions to project evidence. [EN 1998-4:2006 §6.6](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

The Rust evaluator and independent SQLite calculation matched all 13 fixture rows. The authored case also passed both wire validators through the snapshot and mutation suite. The native editor regression now requires the archived case to reach both report entry points.
