from specs.lib import *

SPEC = dict(
    orig='mesh.rs.txt',
    file=EN + '/🥽️mesh/🦀️.rs',
    nest=False,
    external=[orig('meshunit')],
    children=[
        ch('📤', 'interchange-preparations', 'interchange_preparations', [(52, 296)],
           'Interchange of the mesh pipeline: the resumable GLB interchange preparation and the bounded texture PNG encoding.'),
        ch('✅', 'bounded-validation', 'bounded_validation', [(302, 565)],
           'Bounded validation of the mesh pipeline: the resumable watertightness validation preparation with its phases.'),
        ch('🪄', 'bounded-postprocess', 'bounded_postprocess', [(571, 863)],
           'Bounded post-processing of the mesh pipeline: the resumable outward orientation, Taubin smoothing, simplification, UV unwrapping and texturing preparations.'),
        ch('🏭', 'pipeline', 'pipeline', [(869, 1395)],
           'Pipeline of the mesh surface extraction: the pipeline parameters and status, the stage order and the bounded step that drives the stages to a watertight result or an explicit failure.'),
    ],
)
