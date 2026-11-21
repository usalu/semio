# EN 1998 bridge and pipeline design cases

## Continuous bridge deck

The first-generation bridge catalogue now screens the continuous deck in §3.3 and computes Set A by support. Set B computes interior and end support displacement magnitudes for alternating signs. A third case combines the larger absolute pseudo-static structural effect with the inertia result in the same direction using SRSS. The analyst enters the applicable national thresholds and decorrelation distance, selects the governing strain across adjacent soil types, and provides the results of the three structural analyses. All 29 formula and boundary cases reproduced independently in SQLite and Rust. [EN 1998-2:2005+A2:2011 §3.3](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.2.2005.pdf)

## Buried welded steel pipeline

The 2006 pipeline case uses independently entered longitudinal and transverse design waves. It verifies the perfect-bond basis before comparing a conservative envelope of axial and bending strain with the tensile and compressive limits. New cases start with the stability, material and wave basis unconfirmed. All 18 cases reproduced independently in SQLite and Rust. ST-03 still covers global buckling, permanent ground movement, concrete pipes, unwelded joints and fault crossings. [EN 1998-4:2006 §6.5.2 and Annex B](https://www.phd.eng.br/wp-content/uploads/2014/12/en.1998.4.2006.pdf)

Native integrated tests and signed browser verification remain pending.

The properties editor now exposes saved case rows, case measurements and evidence. Snapshots, case changes, nested diffs and SQLite archive rows carry the same values. The archive suite passed 30 tests with 4,946 assertions. The wire harness also validated the snapshot, all seven case mutations and a nested case diff through the production TypeScript reader and Ajv. The independent archive fixture now expects 218 scalar words, as confirmed by SQLite and the canonical schema.
