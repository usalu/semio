from specs.lib import *
MP = B3 + '/💡️queries/📏mass-properties'
CR = B3 + '/💡️queries/🦀️.rs'
CH = [
    ch('📐', 'solid-mass-properties', 'solid_measures', [(47, 397)],
       'Solid mass properties: signed volume, surface area, center of mass, bounding box and the full mass properties of a solid with their moment accumulation, the analytic sphere and box shortcuts and the inertia rotation.'),
    ch('📍', 'face-edge-and-solid-distance', 'face_edge_and_solid_distance', [(403, 564), (846, 863)],
       'Face, edge and solid distance: face area, edge length, the solid to solid distance and overlap tests and the closest point on a solid.'),
    ch('🧮', 'moment-quadrature-and-triangulation', 'moment_quadrature_and_triangulation', [(579, 840)],
       'Moment quadrature and triangulation: the moment component layout, the adaptive triangle quadrature and the planar polygon ear clipping.'),
    ch('🔁', 'loop-areas-and-volume-moments', 'loop_areas_and_volume_moments', [(865, 1066)],
       'Loop areas and volume moments: the straight loop area, the face and loop volume contributions and moments and the Newell area.'),
    ch('🗺', 'uv-polygons', 'uv_polygons', [(1068, 1334)],
       'UV polygons: the face loop UV polygons with their coedge sampling, the chord deviation segment count, the signed area and the point in polygon test.'),
    ch('🧭', 'face-samples-and-closest-point', 'face_samples_and_closest_point', [(1340, 1504)],
       'Face samples and closest point: the face sample points, the surface bounding box expansion, the analytic sphere volume and the closest point on a face, its plane and its boundary.'),
]
SPEC = dict(orig='mp.rs', file=MP + '/🦀️.rs', children=CH, wiring='block', nest=False, external=mounted(MP + '/🦀️.rs', orig='mp.rs'),
            crate_files=crate_files(CR, MP + '/🦀️.rs'))
