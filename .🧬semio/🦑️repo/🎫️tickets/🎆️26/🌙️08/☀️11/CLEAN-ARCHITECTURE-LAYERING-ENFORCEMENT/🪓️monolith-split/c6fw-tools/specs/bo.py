from specs.lib import *
BO = B3 + '/🛠️operations/🔀️boolean'
CH = [
    ch('⚡', 'fast-paths', 'fast_paths', [(132, 300)],
       'Fast paths: the trivial disjoint and contained topology fast path and the exact box fast path with their inside and box tests.'),
    ch('🖇', 'imprint-face-pairs', 'imprint_face_pairs', [(306, 446)],
       'Imprint face pairs: the pending imprint record, the face pair imprint through the surface intersection and the range splitting.'),
    ch('⏳', 'resumable-boolean-job', 'resumable_boolean_job', [(452, 929)],
       'Resumable boolean job: the boolean phases, progress, steps and admission, the boolean job with its stepping and the job constructor.'),
    ch('✂', 'scoping-and-clip-curves', 'scoping_and_clip_curves', [(934, 1275)],
       'Scoping and clip curves: the new solid issue scoping and the clipping of an intersection curve to both face trims with the periodic UV helpers and boundary curve lookups.'),
    ch('📌', 'clip-endpoint-snapping', 'clip_endpoint_snapping', [(1277, 1463)],
       'Clip endpoint snapping: the clip endpoint snap, the boundary touch parameters, the clip p-curves and the golden section minimum.'),
    ch('🔗', 'imprint-vertices-and-edges', 'imprint_vertices_and_edges', [(1469, 1625)],
       'Imprint vertices and edges: the welded imprint vertex, the imprint edge, the coincident boundary edge and the boundary vertex and sub-edge helpers.'),
    ch('🏷', 'classification-and-coincidence', 'classification_and_coincidence', [(1631, 1920)],
       'Classification and coincidence: the interior face point, the local point in solid test, the face classification and keep rules and the coincident face detection.'),
    ch('🧵', 'stitch-and-cleanup', 'stitch_and_cleanup', [(1926, 2123)],
       'Stitch and cleanup: the selected face stitching, the shell grouping and the removal of orphan solids, faces, edges and vertices.'),
    ch('👁', 'mesh-preview-and-shell-helpers', 'mesh_preview_and_shell_helpers', [(2131, 2313)],
       'Mesh preview and shell helpers: the opt-in mesh preview boolean and the outer face, vertex position and probe point helpers.'),
    ch('📏', 'aabb-math-and-stage', 'aabb_math_and_stage', [(2319, 2431)],
       'AABB math and stage: the bounding box arithmetic and the boolean stage as a staged operation.'),
]
SPEC = dict(orig='bo.rs', file=BO + '/🦀️.rs', children=CH, wiring='block', external=mounted(BO + '/🦀️.rs', orig='bo.rs'), crate_files=[])
