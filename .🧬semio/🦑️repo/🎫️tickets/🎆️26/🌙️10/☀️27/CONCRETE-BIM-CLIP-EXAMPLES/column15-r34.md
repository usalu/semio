# Ground Column15 Source Correction R34

[Primary UVa book,p145](https://uvadoc.uva.es/bitstream/10324/68427/1/9788413202839.pdf) identifies ground column15 as cylindrical with28cm diameter beside the helical stair and the15–20beam. The official indexed passage was independently retrieved. This corrects the source's mistaken TY-SQUARE22 assignment. The existing archive tag15/axis[4.737,8.23] and numbered plan/stair context establish the mapping; the primary statement concerns the ground storey.

Column15 now uses existingTY-CIRC28-FINISHED. Its GUID,axis,base0,top2.91 and source memberships are preserved. The type relationship was moved to the existing circular type. Column15-FIRST independently remainsTY-CIRC22. Source evidence and the unresolved core/render boundary are explicit; neither column nor foundation was moved or arbitrarily resized to accommodate a camera. No source count or new type was added.

A language-neutral source-profile case first failed on the old square profile. The full columns runner now passes its source profile/unchanged upper/analytic volume checks, independentNumPy256-segment closed-mesh volume and nativeIFCCIRCLEPROFILEDEF assertion, plus the six existing generic circle/rectangle/ellipse/chamfer/concave/staged cases. UTF-8 reads are explicit for native cross-platform execution.

Independent GEOS/SciPy recomputation of all97registered primary beam/column pairs found exactly one changed value: column15/BEAM-PRIMARY-F-15-20/part0,.007260000000→.008168701169920m³. The old oracle failed at that precise pair before its expected value was updated. Every other96pair remained unchanged. Pairwise total is now.530529321090400m³ (not a global union claim). Three focused tests pass2.803s: independent pair values, exhaustive nonauxiliary beam/column register and actualF15–20full/exclusive/shared material partition.

Changed files: modelJSON element15/type association/property labels; portable model/tests/📏️columns.json and .py; model/tests/🏗️joints.json. No production geometry-kernel change was needed. New source field/memberships remain part of the later ground cohort; R30andR31frozen proofs retain their original source hashes.

The full curtain-cohort JSONSchema check caught a metadata-label shape I added incorrectly: ProfileEvidence was initially flat{en,de}. It was corrected with a fresh source read to the canonical{name:{en,de},unit:""} descriptor. DimensionScope was already canonical. The geometry/source-profile proof was unaffected; whole-source validation now includes the corrected label contract.
