# Halfspace and Official CSG Peer Review

Read-only actual TS kernel probes plus native/source and official provenance audit. No production edits/native execution.

## Proven Accepted Wrong Empty Result

Actual subtractHalfSpace(unit cube[0,1]^3,{origin:[1e-10,0,0],normal:[1,0,0]}) returns0triangles, volume0. Exact subtraction of positiveX halfspace retains slab0<=x<=1e-10, volume1e-10. The fast minDistance>=−EPS branch treats a real thin retained region as fully removed. At1e-9 same empty result;1e-8 currently refuses self-intersecting-solid. Numeric inability to retain a thin feature should produce an explicit limited/ambiguous failure, not a complete wrong empty solid. Main and High were notified. Use exact full/empty signs where mathematically representable, or explicitly refuse tolerance-band ambiguity. Own generated probe module cleaned.

## Source Side and Transform Review

Official pinned IFC4 IfcHalfSpaceSolid primary definition: https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcgeometricmodelresource/lexical/ifchalfspacesolid.htm . AgreementFlagTRUE material lies opposite the base normal; FALSE material lies toward it. Current geometry reader sign logic is consistent: transformed plane covector is obtained from cross of transformed tangent columns, corrected by sign relative to transformed third column, then AgreementFlag selects material side. This preserves reflection and nonuniform affine mapping better than transforming a normal as a vector. Singular/nonfinite transforms explicitly fail. Supported exact base IfcHalfSpaceSolid/IfcPlane arities and first-operand restrictions are explicit; bounded/boxed halfspace subtypes remain unsupported rather than guessed.

The finite cutter is intersection of host bounding box with halfspace, avoiding arbitrary guessed extension distance; convex cap is sorted in its own plane. Mandatory existing solid reconstruction/validation can retain holed host caps. All nested operations share one Control object, preserving cumulative ticks/depth/polygon checks rather than resetting Boolean budget. Empty/full fast paths must retain original final callback and truthful precision result. Current EPS classification is the concrete defect above; no additional accepted-invalid mesh was reproduced in this focused review.

## Official CSG Fixture Identity

Retained permanent official-bath.ifc verified3290bytes/SHA256afb2fc8f0702c3d90ab9395ea0897301c98499160bb69dcda88484fec0375fbd, matching its metadata source https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/bath-csg-solid.ifc . Metadata identifies IFC4 ADD2TC1, pinned commit and CC-BY-ND4.0; original source bytes retained unchanged. Expected sampled volume548147011.3103606mm³ and72arc segments are explicitly sampled geometry, not exact analytic CSG completeness. This bath tests Block/BooleanResult/roundedextrusion, not halfspace; current halfspace fixture plane-clipping.ifc is authored, not an unchanged official clipping example. Do not transfer official bath provenance to the clipping fixture.

Native port/syntax checks and TS tests are separate evidence. No Manifold WASM or native Boolean execution was available to this audit.

## Fresh Precision Fix Verified

Re-executed latest actual TS source: plane offsets1e−12,1e−10,1e−9 now explicitly reject boolean-ambiguous-halfspace-domain-intersection. Exact tangent offsets0and1 return respectively empty and full unitcube; midpoint.5 gives volume0.49999999999999994 with20triangles. Native source has same exact fastsigns and explicit nonzero tolerance-band refusal. Prior accepted-wrong-empty defect is resolved. Own generated transformed module removed.
