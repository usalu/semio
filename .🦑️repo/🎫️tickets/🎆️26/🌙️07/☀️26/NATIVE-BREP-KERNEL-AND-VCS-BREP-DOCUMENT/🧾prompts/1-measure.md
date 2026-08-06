# Lane 1-measure

Owned file: ✏️s/🔨️modules/🧊️3d/📐️brep/📏measure/🦀️component.rs

Mass properties and queries on Body solids/faces/edges.

Public API:
- solid_volume / solid_surface_area / solid_center_of_mass / solid_bounding_box
- face_area / edge_length
- distance_solid_solid / closest_point_on_solid / classify_point_on_solid (may call into classify module if present, else implement ray-cast stub with clear TODO for Wave 3)

Use divergence theorem surface quadrature for volume. Tests: unit box volume=1 (or w*d*h), area=6 for unit cube; sphere volume ~4/3 pi r^3 within 1e-2 with coarse tessellation of analytic faces.

Read wave1-common.md.
