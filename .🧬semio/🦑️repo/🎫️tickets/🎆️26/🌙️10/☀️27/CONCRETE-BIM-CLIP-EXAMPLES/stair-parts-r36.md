# Stair Parts R36

Root adds selected-product public stair_geometry and native IfcStair STEP support, and uses all stair.parts in public representations. Semantic steps/landings remain separately addressable; structural meshes participate in physical views and quantities. Mesh-containing3D/body outputs declare SolidModel and actual polyhedron/faceted_brep geometry.

Portable model/tests/🪜️stair-parts.json/.py independently defines adjacent tile tread/landing and a .9m³ RCmesh waist, total1.1m³. Initial three tests fail: public selected-stair helper absent; native IfcStair unsupported. After integration nativeSTEP passes, independentSciPy/GEOS volume checks expose1.099998932m³ instead of1.1. A new six-boundary-plane regression fails on a complete cube face.

Cause: _polyhedron_cut checks only endpointa of sortedmesh edges. The highest-index boundaryvertex may neverbeendpointa, silently dropping a face corner. Checks now retain both endpoints on coincident planes. No tolerance increased: all boundaryareas match at1e−12, all structuralview/quantity/nativeSTEP tests pass. _solid_geometry separately dropped authored role/material when replacingrawmesh withnormalizedgeometry; a rednativeproxy test exposes missingmaterial, fixed by updating normalized geometry into the nativepart record.

Five portable tests pass2.498s using independent NumPy/SciPy ConvexHull and GEOS slice unions; API/selectednativeSTEP EN/DE checks include exactpartcount, nativebrep/extrusion counts, materialconstituent inventory and preservationofmaterialdomain. Fullnative IFCengine validation is next; source449staircandidate notyetpublished.

Changed: modelAPI, model/exchangeAPI, model/tests/🪜️stair-parts.json/.py. Retained stair-native-r36.py authors independent-engine samples, generatedoutput undergenerated. RootdoesnotalterR36authorstaircoordinates/waistproducer orR38views.


Independent web-ifc0.0.78 native validation passes bothEN/DEsyntheticstair samples: trueIfcStair/half-turn type,3occupiedbodies,36triangles, placedbounds preserved and nativevolume1.099999979138m³ versus1.1m³ analytical (Float32engineerror~2.1e−8m³). The engine is test-only behind retained stair-native-r36.mjs, no runtime dependency added. Nativeoutput and verification JSONunder generated. Actualnewsource stairs will be sampled separately after publication.


Affected existingmesh coverage:6ramp contracts plus nativecollision-band contract run34.086s:6pass, one isolated recorder setup raisesmissingfillings afternewcataloguecontract. Its explicit syntheticgeometry dictionary now declaresemptyfillings; focused actualaxon+vehicle inventory test passes.808s with unchanged9ownedrampbodies. No productiongetfallback added. Newstructuralstairparts are not substituted for rampownership. SourceJSON still445/a444atthis checkpoint.
