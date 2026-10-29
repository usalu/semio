# Beam/Column Joint Revalidation and Roof Bearing R42

Revalidated the current explicit beam/column material ownership after the ground-column15 correction and roof-stair continuations. No source geometry, source owner references, API, schema or existing fixture was changed in this round.

## Checkpoints and Scope

The initial447-product source was SHA25659520f42d21688639957e15574ceaf9d49ebd8a620ac5c289c5d645b70ecfb71. It supplied the production35joint contracts, separate six ramp contracts and ten native beam samples. During the requested extra bearing inspection, the parent published a448-product source, SHA2561dbab6d387cfce2cccc8df02fde1fe33ab446e6d8911b3e8699cafda09578113. A fresh source-only complete joint sweep on that later checkpoint gives the same result. No unrelated boiler publication was overwritten or treated as a joint fixture change.

## Independent Source-Only Joint Oracle

Durable joints-r42-oracle.py imports no model API and does not recapture production geometry as its expected result. It reads authored JSON directly, resolves numeric/storey elevations and section-specific axes, independently constructs rectangle/polygon/circle/ellipse outlines and rotated profile bores with NumPy, checks shoelace net area against GEOS, intersects raw beam profiles with actual column sections, and independently checks extruded common solids with SciPy ConvexHull. Circles use96segments to compare against the existing authored geometry contract; this is not an exact-circle/as-built measurement claim.

The exhaustive sweep covers63nonauxiliary beam products,67column products and100column sections. It finds exactly97positive section/beam-part pairs, matching all97existing fixture entries. There are zero new pairs, zero removed pairs, zero changed expected pair volumes at1e-9m³ and zero beam joint-owner reference differences. The current pairwise sum is0.5305293210904004m³, exactly the existing fixture total. This remains a sum of pairs, not an asserted global union error estimate.

The earlier source-backed ground15 change is already represented correctly: column15 is a finished28cm circle at4.737,8.23. Its F-15-20 part0 common area is0.027229003899734m² over2.61–2.91m, giving0.008168701169920m³. No second fixture correction was needed.

10-FIRST has a lower22cm square section over3.14–6.34m in the inspected source;15-FIRST has a lower22cm circular section. Both new upper square sections run6.60–9.03m. Their roof continuations add no positive raw beam joint. Existing10-FIRST/R-4-10 andR-10-11 common volumes remain0.00726m³ and0.00363m³, both over6.04–6.34m. 15-FIRST has no positive exclusive beam common solid. The nearby R-14-15/R-34-10 upstands top out at6.57m, below the new6.60m section starts. The complete sweep also checks the remote solarium and cellar beams rather than relying on that local elevation argument alone.

## Production Contracts and Separate Ramp Classification

All35actual BeamJointOwnershipContract tests pass on the447checkpoint in21.118s. These test conserved full-member partitions, exclusive material inventory, source immutability, valid ownership, native quantity scopes and the complete registered joint cohort against the fixture. The source-only oracle independently establishes that the fixture itself remains justified.

The six RampGeometryContract tests pass separately in6.314s. Their actual inclined source slabs, diagonal synthetic flight, shared landing ownership, production drawing/vehicle callbacks and native aggregation semantics remain unchanged by these column edits. No ramp result is counted as a beam/column intersection proof, and no ramp producer was edited.

## Independent Native Beam Verification

Pinned web-ifc0.0.78 independently decodes ten selected-product samples, English/German for each of F-15-20, R-4-10, R-10-11, R-14-15 andR-34-10. The cohort covers the changed finished ground15 footprint,10-FIRST's existing lower-storey joints and both roof-stair continuation neighborhoods, including beams requiring no new owner reference.

All native identity/localized names, actual placed bounds, occupied mesh volume, Pset_MaterialOwnership body/full/owned/shared scopes and explicit shared owner tags pass. Maximum volume error is approximately2.4e-8m³. One language gives:

|Beam|Native bodies|Triangles|Native volume m³|API owned volume m³|
|---|---:|---:|---:|---:|
|F-15-20|1|120|0.283089313651|0.283089299|
|R-4-10|1|104|0.296225720122|0.296225707|
|R-10-11|5|60|0.352352963739|0.352352940|
|R-14-15|1|12|0.116379999068|0.116380000|
|R-34-10|2|24|0.033022739146|0.033022740|

Durable joints-r42-native.py selects this cohort through the existing exchange interface; joints-r30-native.mjs supplies the existing independent pinned-reader interface. These are native selected beam body checks, not a native rendered-frame or whole-building structural acceptance claim.

## Additional Roof Post Bearing Finding

The parent requested a separate physical check of upper column bases against local RC structure. Durable joints-r42-bearing.py reuses only the independent source decoder and performs bounded local GEOS overlays of each upper22×22cm footprint against raw roof regions minus authored slab/native opening holes. A1e-10m test precision grid avoids near-coincident global overlay stalls. No product geometry is snapped or changed.

Both upper post footprints are0.0484m² and have complete structural coverage in plan, but their6.60m bottom faces have no direct reinforced-concrete top-face contact:

|Upper post|Structural area m²|Structural top m|Vertical separation to post base m|
|---|---:|---:|---:|
|10-FIRST|0.025520|6.51|0.09|
|10-FIRST|0.022880|6.57|0.03|
|15-FIRST|0.048400|6.57|0.03|

10-FIRST's underlying regions are all explicitly tagged MAT-RC beam strips. For15-FIRST,0.035585m² is tagged MAT-RC beam strips and0.012815m² is MAT-HOLLOW-SLAB with the source5cm concrete compression layer at the top. Neither post has unsupported plan area; the issue is the vertical material/bearing registration.

The source roof material LS-ROOF includes a9cm weathering/paving reserve over the17cm reference structure. The existing finish geometry brings local structural tops6.51/6.57 up to finished roof datum6.60, using MAT-ROOF-FINISH. Therefore a complete building drawing may show material touching the post at its base, but that material is finish/weathering rather than RC bearing. This is a structural bearing gap in the current occupied material contract, not proof of an entirely empty visual gap.

No automatic base correction was made. A flat6.57m base for10-FIRST would initially contact only the0.02288m² upper strip, while lowering it to6.51m would intersect the stepped upper RC region and require an explicit ownership/junction partition. There is no source basis here for an arbitrary3cm shim or a claim that finish is structural concrete. The current registered_roof_stair_support role does not explicitly identify an embedded bearing continuation below the exposed roof datum. A source-supported stepped/embedded junction or an explicitly bounded exposed-only scope needs resolution by the roof/stair producer. Existing material geometry and the97pair beam fixture must not be changed merely to hide this separate uncertainty.

The parent received the exact local region areas, elevations, material identities and no-edit recommendation before this report was finalized.

## Files and Handoff

Created durable ticket inputs/reports: joints-r42-oracle.py, joints-r42-bearing.py, joints-r42-native.py and this report. Generated pair/bearing JSON, native sample/output JSON and reader dependencies are under 🗑️generated for parent cleanup. No checked-in source/API/schema/test file was changed. The worker was disposed after verification.

No shell, Git mutation, worktree, IDE, AGENTS edit, checked-in source copy, migration or external runtime dependency was introduced. Parent owns the newly identified roof bearing resolution and the final source-wide cohort, notes and analytical-link refresh.
