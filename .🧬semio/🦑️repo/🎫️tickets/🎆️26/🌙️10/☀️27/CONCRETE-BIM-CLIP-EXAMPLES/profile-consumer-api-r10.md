# Profile Consumer API R10

## Contract

The schema now requires top-level roles with six string keys: column_example, column_type_example, floor, roof, entrance_glazing, ramp. The source author supplies actual tags; role(key,model) returns the referenced actual tag and rejects an unknown role or missing target. No aliases for fictional column identifiers are introduced. The car schema additionally requires a positive height quantity. The author was requested to distinguish assumed vehicle height from sourced building dimensions.

_special now resolves only the requested property. Unrelated properties no longer eagerly load an assumed column type. Column example diameter resolves the actual column_example role/type; occurrence dimensions retain their actual profile metadata. Curve radius calculation reads only the vehicle dimensions it needs, and explicit curtain radius no longer evaluates an unused fallback.

## Physical Vehicle Checks

Vehicle occupancy is the vertical interval from ground-storey elevation to ground plus car.height. Column parts intersecting that physical interval participate regardless of storey assignment; upper columns and basement portions below road are excluded. Circle distance is analytic oriented-rectangle-to-circle distance. Rectangles, chamfers and arbitrary concave polygon profiles use edge distance, edge crossing and containment tests. No bounding radius participates in physical clearance. Ellipse contours carry a conservative curve_error bound, subtracted from polygon distance, to avoid falsely declaring an inscribed approximation clear.

Path crossings resolve the actual support tags. Gap endpoints come from physical outline intersections with the crossing row rather than parsing a grid-letter tag or reusing one global diameter. clearance and clearance_limits expose obstruction_width (combined physical half-widths), and path_pinches exposes parts and bounding_radius for display halos. A supplied diameter in clearance_limits describes an explicit circular-profile teaching scenario rather than a source dimension.

A continuous-path clearance certificate subtracts a conservative sample motion bound: half of uniform path sample distance times (1 + vehicle corner radius / turning radius). This uses travelled distance instead of chord distance, so curved motion is bounded. collision_free is true only when sampled physical clearance exceeds that bound. A hypothetical spacing override never overrides a failed physical route certificate.

## Tests

Portable cases: clip/🧱️model/🧪️tests/🚘️profiles.json and matching Python test. Initial failing test demonstrated missing _part_pose_distance. Final independent Shapely tests passed rectangle face clearance, rotated car, contained obstacle, chamfer corner, concave notch, analytic circle distance over 28 headings, conservative ellipse bound against a 4096-vertex oracle, vertical occupancy, semantic role lookup and lazy property resolution after removing the old assumed type. All six R9 shape/mesh/STEP tests also passed after these changes.

A temporary in-memory source smoke checked car_path, clearance and path_pinches through the isolated Python/WASM validator. The source author was still authoring the new numbered source register, roles and height at that moment; no historical route conclusion is drawn from that transient source smoke. Source-specific validation must be rerun after the authored register is saved.

## Files

Modified model/🐍️.py; model/🧬️schema/🔣️.schema.json (roles, car.height only). Added model/🧪️tests/🚘️profiles.json and 🚘️profiles.py. R9 column tests remain present. Runtime remains stdlib only; Shapely is used solely as an independent test oracle.
