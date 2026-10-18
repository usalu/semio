from specs.lib import *
RM = '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot'
CH = [
    ch('🧩', 'asset-handles-and-durable-content', 'asset_handles_and_durable_content', [(106, 367)],
       'Asset handles and durable content of the remodeling snapshot: the composed image and mesh child types, the durable artifact store, the asset child handle minting and the content kind, digest and chunk laws.'),
    ch('🕸', 'mesh-handles', 'mesh_handles', [(372, 567)],
       'Mesh handles of the remodeling snapshot: the mesh child handle, the chunk assembly, the content handles, the resolution envelope and the bounded mesh chunks with the test mesh workspace.'),
    ch('🧮', 'buffers', 'buffers', [(572, 659)],
       'Buffers of the remodeling snapshot: the float sample buffer and the byte buffer with their DSL field bindings.'),
    ch('🎞', 'media-and-calibration', 'media_and_calibration', [(663, 835)],
       'Media and calibration of the remodeling snapshot: the image asset, media kinds, video sources, frame references, media streams, camera calibration, rig extrinsics and ground control points.'),
    ch('⚙', 'reconstruction-params', 'reconstruction_params', [(837, 1109)],
       'Reconstruction parameters of the remodeling snapshot: the ingest, feature, match, structure, dense, mesh, motion and geo parameters, the camera pose preview and the mesh source.'),
    ch('📊', 'reconstruction-results', 'reconstruction_results', [(1111, 1283)],
       'Reconstruction results of the remodeling snapshot: the watertight report, the mesh record, the sparse and dense clouds, the trajectory, the motion tracks, the geo products and the quality report.'),
]
SPEC = dict(orig='rm.rs', file=RM + '/🦀️.rs', children=CH, external=[], nest=True, leaf_concat=True, source_concat=True, oracle_root=RM + '/📦️packages/🦀️rust/🦀️.rs', oracle_copy=[RM])
