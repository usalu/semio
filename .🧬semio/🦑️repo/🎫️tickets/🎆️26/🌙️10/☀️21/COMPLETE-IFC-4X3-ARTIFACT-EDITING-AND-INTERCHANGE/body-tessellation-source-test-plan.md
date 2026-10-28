# Body Tessellation Released Source and Neutral Oracle

Pinned official release commit610ee3f95db9d0380b1eb8de4dc6e17833531518; exact source manifest/hashes and external HTML inputs are retained under generated/body-tessellation/oracle.json and0/1/2.html. Authored ticket inputs: body-tessellation-cases.json and body-tessellation-oracle.py. No production sources copied or edited.

## Authenticated clauses and applicability

[Released concept](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry/content.html), SHA256 b8f905fdf6d7bcb4afa69d20111600589e52f4bef43043b8a7814e0b5da78d32, lines117–122: the representation uses Body identifier,Tessellation type,and IfcTessellatedFaceSet items. Its generic diagram roots the usage at IfcElement; its usage table includes IfcElement. These three explicit predicates can be executed under an explicitly requested concept. The diagram/general-use text does not establish that every element must have a body or choose tessellation.

[Released IfcWall usage](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcWall.htm), SHA25618ad72b5db203b6df1909361555cb3bef98ffc0a50e7a3735660e8423118f90b, lines1794–1803 explicitly labels this inherited concept Reference View and describes optional single/multiple tessellated surface representations for IfcElement. It does not make all Body geometries tessellated. No equivalent full Alignment-basedReferenceView applicability graph was authenticated by these inputs.

[Released IfcTessellatedFaceSet](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcTessellatedFaceSet.htm), SHA2564ab6fa906f66cb20d2110d8501a66b6c647c25f8b5ff941224aa889a0e6e99ad, supplies the declared item hierarchy context. Actual independent schema membership is checked below.

No executable XML/mvdXML href was present in these three inspected released files. This is a file-local result, not proof that no constraint file exists elsewhere. The authoritative pinned artifacts here are released prose/diagram/usage clauses, not an invented machine template. Do not label a translated hand-authored predicate as official mvdXML.

## Executed independent oracle

Python3.13 with existing ticket IfcOpenShell0.9.0.post1 created physical IfcShapeRepresentation instances and concrete IfcTriangulatedFaceSet/IfcPolygonalFaceSet/IfcPolyline items. It read actual attribute values and used entity.is_a(IfcTessellatedFaceSet) independently of our catalog.9/9 neutral expected rows match: triangle,polygon and mixed tessellated items pass; wrong identifier,type,item or mixed invalid items fail; unrelated Curve3D Body with explicit nonapplicability is not-applicable; unknown applicability remains incomplete. Actual graph is generated/body-tessellation/graphs.ifc.

The positive triangle and polygon graphs are independently authored minimum physical examples, not claimed official example files. They are predicate fixtures, not whole-model full EXPRESS/MVD validation. No official positive whole graph was located in this narrow checkpoint. Schema Items nonempty/cardinality/reference integrity belongs to ordinary schema validation and should not be silently presented as a fourth concept predicate.

## Implementation boundary

Use neutral rows as first explicit concept contract, with caller-provided applicability and released source provenance. ReferenceView header alone is insufficient to assert every body must use this concept. Resolve representation/item targets and subtype membership progressively from original captured source. Add production cases for missing/ambiguous target,complex constituents,multiple representations and controlled cancellation before enabling native gate. A partial concept report must never become full ReferenceView or Alignment-basedReferenceView valid. Extend the source manifest with actual exchange applicability requirements before claiming automatic applicability.

Cross-platform handoff: oracle short-path ctypes is now guarded by sys.platform=='win32'; other platforms use normal Path. Fresh9/9 run writes oracle-v2.json with exact physical representationId per row, preserving old oracle.json. Root can select these IDs from graphs.ifc without entity-order inference. The earlier signed-zero oracle received the same Windows-only guard.
