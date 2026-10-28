# Original Faceted Topology and Planarity Audit

The saved original source rosters were analyzed directly; no external geometry run was repeated. Original hashes remain basic `b247026e85ef163acd5a165177d8293e606e7b8f83348a9897a3a04967fe4fc1` and basin `d2bdf9d5f485ff77ce9723a24ceb75a5532b51d1788b9a5f3f5145944d32065f`.

| Original | Point IDs | Undirected edges | Source edge incidence | Incident faces per vertex | Face components |
|---|---:|---:|---|---|---|
| basic #1021 | 8 | 12 | every edge twice, opposing directions | all 8 vertices: 3 | one, 6 faces |
| basin #1152 | 160 | 320 | every edge twice, opposing directions | all 160 vertices: 4 | one, 163 faces |

Neither source has equal coordinate tuples under distinct point IDs. Neither contains a vertex incident to fewer than three distinct faces. Incidence uses exact source point IDs, applying each bound Orientation before comparing directed edges. Connectivity uses shared source-ID edges, not approximate geometric welding. These are original observations, not proof that identical topology constraints suffice for arbitrary solids.

Basic face residuals are exactly zero. Basin's largest plane residual is face #627 (40 points), 9.947598300641403e−14 source units over span 319.669090422956, relative 3.1118424016158956e−16. Its two-bound face #584 has 80 total points (outer and hole), span 609.759980332881 and zero measured plane residual. The independent computation chooses a maximum-area triangle relative to the first point, then measures every bound point against that plane. Exact floating equality would incorrectly reject #627. A dimensionless, scale-normalized roundoff tolerance must account for this measured residual; it must not use a large physical tolerance to accept genuinely bent faces. These measurements alone do not establish a universal rounding-error certificate.

Released ADD2 EXPRESS: `IfcBoolean = BOOLEAN` lines 27–28; `IfcLengthMeasure = REAL` lines 178–179. FaceBound.Orientation is IfcBoolean at 6628. CartesianPoint.Coordinates is LIST[1:3] OF IfcLengthMeasure at 4755, with CP2Dor3D at 4757. PolyLoop.Polygon is LIST[3:?] OF UNIQUE CartesianPoint, AllPointsSameDim at 8617–8619. Coplanarity is stated in the pinned primary PolyLoop HTML: ordered coplanar points and every polygon point coplanar. The EXPRESS rule alone checks common dimension, not coplanarity. Implicit closure connects the last point to the first.

These alias declarations do not authorize arbitrary typed-wrapper publication. Actual ordinary physical records use direct `.T.`/`.F.` Boolean and real coordinate values. Existing internal alias decoding and physical Part21 admission remain separate concerns. No schema widening, coordinate welding, or blanket wrapper acceptance is proposed here.

Authored input: `faceted-original-topology.py`; full per-face residuals, source edge counterexamples (empty for these originals), vertex census and components: `generated/faceted-original-topology.json`. No native compiler/runtime or full-view claim.
