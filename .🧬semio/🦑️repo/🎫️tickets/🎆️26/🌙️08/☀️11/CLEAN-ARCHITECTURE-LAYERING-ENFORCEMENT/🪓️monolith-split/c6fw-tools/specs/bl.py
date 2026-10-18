from specs.lib import *
BL = B3 + '/🛠️operations/🎨️blend'
CR = B3 + '/🛠️operations/🦀️.rs'
CH = [
    ch('✅', 'blend-validation-and-geometry', 'blend_validation_and_geometry', [(65, 226)],
       'Blend validation and geometry: the solid and edge set checks, the blend request validation and the line, plane, curve parameter and face plane geometry helpers.'),
    ch('📈', 'blend-pcurves', 'blend_pcurves', [(232, 435)],
       'Blend p-curves: the closed-form affine, planar, blend and interpolated parameter curves with their deviation check and the surface and curve kind readers.'),
    ch('🩹', 'blend-patches', 'blend_patches', [(441, 750)],
       'Blend patches: the blend spec, the end trim, the patch record and the cylinder, chamfer plane, cone and cap patch builders.'),
    ch('🔺', 'blend-corners', 'blend_corners', [(756, 955)],
       'Blend corners: the corner record, the vertex star, the corner builder and surface and the conic end trim.'),
    ch('⏱', 'blend-job', 'blend_job', [(961, 1188)],
       'Blend job: the member and minted records, the pcurve branch alignment, the phase indices and the resumable blend job with its staged operation.'),
    ch('🔪', 'face-surgery', 'face_surgery', [(1190, 1410)],
       'Face surgery: the patch trimming, the carried edge minting, the original face rebuild and the patch and corner face assembly.'),
]
SPEC = dict(orig='bl.rs', file=BL + '/🦀️.rs', children=CH, wiring='block', external=mounted(BL + '/🦀️.rs', orig='bl.rs'),
            crate_files=crate_files(CR, BL + '/🦀️.rs'))
