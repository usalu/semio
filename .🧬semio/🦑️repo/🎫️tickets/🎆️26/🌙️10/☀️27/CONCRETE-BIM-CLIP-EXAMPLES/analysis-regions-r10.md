# Physical Analysis Propagation R10

## Scope and Native Contracts

Analysis now selects primary slabs by source semantic roles floor and roof through building_geometry.roles. It does not infer the primary roof from the first ROOF object, so the distinct kiosk canopy cannot accidentally replace the main roof. The core geometry provider must expose the source roles mapping.

Every slab geometry/composite declares compression_layer, rib_width, rib_pitch, concrete and ceramic. A strict JSON Schema definition accepts only those fields, native metre quantities and material tags. Source authors provide5/6/25cm sourced geometry and assumed concrete25/hollow-unit-envelope8kN/m³ density inputs. The derived slab geometry retains the composite object. A hollow-unit envelope is not solid clay.

## Bearings, Loads and Translation

Support selection intersects each real column profile section with each occupied regional footprint at that region's own underside. A reference top minus maximum depth cannot fabricate a bearing surface when regional soffits differ.

Composite self-weight integrates concrete-equivalent depth t+(b/p)(h−t) and remaining ceramic-unit-envelope depth separately. For17/20/23cm structure, RC-equivalent depth is.0788/.086/.0932m; assumed regular-field weight is2.6996/3.062/3.4244kN/m². This estimate excludes separately modelled edge/transfer beams and must not be presented as a measured whole-building material takeoff.

Roof paving/weathering reserve is layer.paving reference thickness minus local structural depth above the nominal structural reference. With17cm nominal structure and9cm reference reserve, actual reserve is9cm above17cm structure and3cm above23cm structure. A paving event adds the same thickness increment locally to every structural region. No blanket soil or nominal full-depth RC load is counted.

Tributary loads intersect actual regional parts with Voronoi cells and surface zones. External zone IDs remain slab for the first floor and source roof/solarium on the roof; each exposes nested regions with actual occupied pieces, depth, breakdown and force contribution. Aggregate load breakdowns carry area-weighted composite components and equivalent depths. Slab-load links use mean occupied structural depth. Piloti-section links follow their actual linked object GUID and report only dimensions/areas that apply to that object.

Structural shell nodes lie at each region's true mid-surface and retain local thickness and region identity. Coincident XY nodes at different heights expose explicit assumed offset links. Column members retain their physical profile sections, areas and elevations; only an actual one-section circle gets a diameter. The analysis is an explicitly idealised load-path model, not a solved finite-element certification.

## Physical Clearance

Service clearance computes minimum pipe-centreline distance to the actual vertical profile prism and subtracts the actual pipe radius. It includes side faces and end caps and works when a noncircular column has no radius field. Convex distance minimisation uses stdlib numerical search; nonconvex profiles are partitioned through the existing internal polygon kernel. Vertical service segments use a direct minimising station. An intersecting centreline reports overlap through negative pipe-radius clearance rather than an invented circular column envelope.

## Independent Evidence

The test-first regional-bearing regression failed against the old root-envelope soffit. Eight focused regressions now pass in the Python/WASM validation host. Independent Shapely/GEOS and SciPy checks cover:

- Mixed-soffit support inclusion/exclusion from actual profile intersections.
- Segment-to-prism distance against SciPy scalar minimisation of independent GEOS plan distance plus vertical cap separation.
- Composite material volume balance and analytical regular-field weights.
- First-floor and roof reactions from independent GEOS cell intersections and analytical local area loads.
- Section-link identity and absence of false diameters for rectangles.
- Regional shell occupied area/mid-surface geometry and member section area.
- Public clearance classifications without column radius metadata.
- Paving-reserve event sensitivity:5cm over40m² at20kN/m³ adds exactly40kN, with no soil term.

The three existing source-independent sparse-support tests also pass: SciPy cKDTree Voronoi sampling, nonconvex subtraction and Shapely profile bearing. Existing analysis fixtures now use the native slab parts protocol rather than root-only approximations.

## Files and Integration

- 🧱️model/🧮️analysis/🐍️.py.
- 🧱️model/🧮️analysis/🧪️tests/📏️physical.json and 📏️physical.py.
- 🧱️model/🧮️analysis/🧪️tests/🐍️.py.
- 🧱️model/🧬️schema/🔣️.schema.json: geometry.composite only for this phase.
- 🧱️model/🐍️.py: preserve composite metadata in slab geometry only for this phase.

Source-wide tests require the concurrently authored measured source and source roles provider. Full integrated validation remains the coordinator's gate after source publication. No source JSON or drawings were edited by this agent.
