# Declared Fixture Deltas R20

The roof feature declares exactlythree new native source products: one IfcOpeningElement OP-SC-VIEW and two IfcBeam members SC-SILL/SC-HEAD. The existing canonical class-count fixture is therefore changed from36 to37 openings and61 to63 beams; other current counts are preserved. This is a declared feature delta, not automatic baseline recapture. Further cellar additions require their own explicit delta after publication.

The highest dimension fixture changes from assumed9.40m to sourced9.325m, independently legible in both structural Annex5 and Annex6. The exact datum is retained; no rounding back to a nominal building box is used. The published roof source uses separate body/cap/aperture levels. Areas and historical reference values have not been adjusted merely to make authored geometry fit them.


## Cellar Feature Delta

The cellar author declares9 new products:2 physical hollow assemblies,4 native openings,2 independent covers and1 metal jib. Canonical counts therefore become7 slabs,41 openings and19 building-element proxies;63 beams and67 columns stay unchanged. Both slab hatches have explicit native IfcRelFillsElement relations to their IfcSlab covers. The generic host tests now validate wall versus slab classes and include any authored filling product, preserving the exactly-one-host and optional-filling checks. HOIST-JIB is deliberately shared among architecture,structure andservices; the model-membership fixture now tests that exact shared identity instead of forbidding all cross-domain service membership. The material association oracle accepts explicit type inheritance for CW-1. Type subclasses now participate in IFC-root registry checks.


## Focused Native Graph Verification

All6 focused main contracts pass after these declared changes: historical dimensions/basis, native wall-or-slab hosts and fillings, IFC-root registry including types, actual void/fill relationships, material association including inherited type ownership, and exact shared mechanical model identity. The386-product source hash stayed identical before and after the22.664s run:3eeaab3c39ff6cc263eb9a76e3b31a88f4fb592947618bba8c8696e60ee8bf91. Subsequent original-services publication is a new source cohort and requires an explicit new fixture delta.
