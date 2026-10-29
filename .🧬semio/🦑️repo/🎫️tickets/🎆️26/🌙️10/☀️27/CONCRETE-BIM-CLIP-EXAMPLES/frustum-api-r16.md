# Tapered Foundation Geometry R16

The schema now accepts optional top_outline vertices on a solid part. Corresponding lower/upper sections must be convex and have compatible parallel edges; incompatible counts, twisted correspondence and concave lofts are rejected. Existing concave ordinary prisms retain their original contour behavior.

Shared model helpers derive true vertices, interpolated sections, footprint, inclined sections and elevation silhouettes. Occupied volume keeps its exact constant-section union path for ordinary prisms and uses adaptive slice-union integration when a tapered solid is present. The same generic part resolves source-local elevations into world coordinates. No stepped pyramid approximation or runtime geometry library was introduced.

Semantic representations preserve actual tapered solids in 3D and use tapered extrusion body metadata; sections and elevations retain inclined sides. Quantity integration includes actual volume and unions overlapping parts. Five portable regressions were written before implementation. They pass against independent SciPy convex hull volume, SciPy quadrature with GEOS slice unions, NumPy interpolation and Shapely section/elevation polygons, including invalid geometry cases.

Drawing integration is owned by the frustum projection executor: source-derived plan caps and cuts, inclined sections and elevations, true side-face normals, detail views and axon. The executor reports8new production recording tests plus22existing drawing tests passing; native Manim frames remain a separate unverified gate.

New buried column sections are split at the diagram terrain elevation0 for painter ordering. Opaque terrain is inserted after buried pieces and before above-ground pieces in ordinary material views. Structure-only/exploded views can reveal foundations with ground disabled. The new ground-occlusion regression and two existing axon physical contracts pass.

Foundation source publication and its independent material/contact checks are the next gate, documented in foundations-r16.md. The isolated pad outline/depth traces remain explicitly inferred where archival measurements are unavailable; these tests do not establish survey precision.

Post-publication: the382-element source passes strict AJV and resolves45foundations with67columns. The integrated geometry/drawing suite passes44tests against that source. Source-author foundation/contact/material tests pass5tests on the published source; the parent separately verifies the5frustum API contracts. Full native clip playback remains blocked by the launcher registry issue documented in runtime-verification-r7.md.
