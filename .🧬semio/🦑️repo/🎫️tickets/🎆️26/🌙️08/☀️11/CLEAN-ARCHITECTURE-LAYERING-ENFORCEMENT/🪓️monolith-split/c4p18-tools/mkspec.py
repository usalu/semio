import os, sys, json
T = os.path.dirname(os.path.abspath(__file__))
REPO = '/Users/ueli/Documents/semio'
E = '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine'
V = '️'

def ch(emoji, kebab, mod, ranges, doc, **kw):
    return dict(folder=emoji + V + kebab, mod=mod, ranges=ranges, doc='//! %s%s %s See [`super`].' % (emoji, V, doc), **kw)

TABLE = {
 'feature': dict(file=E + '/🌟️feature/🦀️.rs', external=[E + '/🌟️feature/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🔎', 'detectors', 'detectors', [(21, 638)], 'Keypoint detectors of the remodeling engine features: the FAST and Harris corner responses, the oriented-FAST and Harris detectors and the resumable bounded detection preparation.'),
    ch('🧬', 'descriptors-matching-and-flow', 'descriptors_matching_and_flow', [(640, 975)], 'Descriptors, matching and flow of the remodeling engine features: the oriented rBRIEF description, the brute-force, epipolar-guided and ZNCC matching and the pyramidal Lucas-Kanade tracking.'),
    ch('🌀', 'akaze-scale-space', 'akaze_scale_space', [(977, 1295)], 'AKAZE of the remodeling engine features: the nonlinear diffusion scale space, the Hessian blob detector and the M-LDB description.'),
 ]),
 'dense': dict(file=E + '/🌫️dense/🦀️.rs', external=[E + '/🌫️dense/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🧩', 'patch-match-stereo', 'patch_match_stereo', [(114, 801)], 'Multi-view stereo of the dense reconstruction: PatchMatch, the plane sweep and the depth filters.'),
    ch('🧊', 'fusion-and-tsdf', 'fusion_and_tsdf', [(803, 1340)], 'Depth fusion and TSDF volumes of the dense reconstruction: the cross-view fusion, the volume and its resumable integration.'),
    ch('☁', 'point-cloud-analysis', 'point_cloud_analysis', [(1342, 1829)], 'Point-cloud analysis of the dense reconstruction: normals, downsampling, outlier removal, ground and plane classification and the cloud change distances.'),
 ]),
 'h264': dict(file=E + '/🎥️video/🎞️h264/🦀️.rs', external=[], children=[
    ch('📖', 'bitstream-and-headers', 'bitstream_and_headers', [(37, 404)], 'Bitstream syntax of the baseline H.264 decoder: the bit reader, RBSP extraction and the SPS, PPS and slice header parsing.'),
    ch('🔤', 'cavlc', 'cavlc', [(406, 664)], 'CAVLC of the baseline H.264 decoder: the coefficient token, total zeros and run before tables and the residual block decoding.'),
    ch('🧮', 'reconstruction', 'reconstruction', [(666, 1226)], 'Reconstruction of the baseline H.264 decoder: the inverse transform, the picture buffer and the intra and inter prediction.'),
    ch('🧼', 'deblocking-and-output', 'deblocking_and_output', [(1228, 1625)], 'Deblocking and output of the baseline H.264 decoder: the loop filter, the decoded picture buffer and the YUV conversion.'),
    ch('🍰', 'slice-data', 'slice_data', [(1627, 2380)], 'Slice data of the baseline H.264 decoder: the macroblock layer parsing and reconstruction of I and P slices.'),
    ch('✍', 'encoder', 'encoder', [(2508, 2713)], 'Encoder of the baseline H.264 codec: the bit writer and the I_PCM and P_SKIP stream writer.', glob=False),
 ]),
 'motion': dict(file=E + '/🏃️motion/🦀️.rs', external=[E + '/🏃️motion/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🎯', 'tracking-and-trajectories', 'tracking_and_trajectories', [(23, 311)], 'Tracking and trajectories of the videogrammetry: the vector helpers, the 2D and multi-object trackers and the 3D trajectory triangulation.'),
    ch('🎼', 'kinematics-modal-and-sync', 'kinematics_modal_and_sync', [(313, 593)], 'Kinematics, vibration and synchronisation of the videogrammetry: the strain and kinematic fields, the modal analysis and the camera sync.'),
    ch('🪞', 'camera-motion-compensation', 'camera_motion_compensation', [(595, 835)], 'Camera motion compensation of the videogrammetry: the rolling-shutter compensation, the path stabilisation and the deblurring.'),
    ch('🫧', 'non-rigid-and-pose', 'non_rigid_and_pose', [(837, 997)], 'Non-rigid capture and rigid-body pose tracking of the videogrammetry.'),
 ]),
 'camera': dict(file=E + '/📷️camera/🦀️.rs', external=[E + '/📷️camera/🧪️tests/🔬️unit/🦀️.rs'], children=[
    ch('🔭', 'intrinsics', 'intrinsics', [(18, 187)], 'Intrinsics of the camera model: the pinhole parameters and the lens distortion models.', glob=False),
    ch('🧭', 'poses-and-rigs', 'poses_and_rigs', [(189, 613)], 'Poses and rigs of the camera model: the extrinsics, the rolling-shutter model and the multi-camera rig with its refinement.'),
    ch('📐', 'calibration', 'calibration', [(639, 1046)], 'Calibration of the camera model: the matrix helpers, the planar calibration and the self calibration.'),
 ]),
 'geometry': dict(file=E + '/📸️sfm/📐️geometry/🦀️.rs', external=[], children=[
    ch('🧮', 'matrix-helpers', 'matrix_helpers', [(39, 139)], 'Matrix helpers of the multi-view geometry: the small vector and matrix functions every model shares.'),
    ch('🔀', 'two-view', 'two_view', [(171, 961)], 'Two-view models of the multi-view geometry: the fundamental, homography and essential estimation (including the five-point solver), the essential decomposition and refinement and the model selection.'),
    ch('📍', 'triangulation-and-pnp', 'triangulation_and_pnp', [(963, 1413)], 'Triangulation and perspective-n-point of the multi-view geometry: the linear and refined triangulation, the P3P, EPnP and RANSAC solvers and the pose refinement.'),
    ch('🛤', 'tracks-global-and-loop-closure', 'tracks_global_and_loop_closure', [(1415, 1796)], 'Tracks, global averaging and loop closure of the multi-view geometry: the track building, the rotation and translation averaging, the loop detection and the pose-graph optimisation.'),
 ]),
 'reconunit': dict(file=E + '/🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs', external=[], patch={'♻\ufe0fdense-replay': [('    pub(super) fn encode(', '    pub(in super::super) fn encode(')]}, children=[
    ch('📥', 'ingest-gate', 'ingest_gate', [(5, 85)], 'Frame ingest tests of the staged reconstruction engine: the blur gate, the stride and frame cap and the video push counts with the flat and checker frame fixtures.'),
    ch('🧊', 'synthetic-scene', 'synthetic_scene', [(89, 313)], 'Synthetic scene of the staged reconstruction engine tests: the vector helpers, the textured cube renderer, the orbiting frames and the tiny engine parameters.'),
    ch('🔗', 'pipeline-contracts', 'pipeline_contracts', [(315, 653)], 'Pipeline contract tests of the staged reconstruction engine: the chunking invariance, the orbit registration, the preview and frame minimum, the end-to-end contract and the bounded step ceilings.', reexport=False),
    ch('🔎', 'orbit-diagnostics', 'orbit_diagnostics', [(655, 1206)], 'Orbit diagnostics of the staged reconstruction engine: the ignored stage trace, geometry audit and dense audit against the ground truth of the synthetic orbit.', reexport=False),
    ch('♻', 'dense-replay', 'dense_replay', [(1208, 1477)], 'Dense replay diagnostic of the staged reconstruction engine: the dense dump codec and the replay of a dumped dense stage through fusion and meshing.'),
 ]),
}

if __name__ == '__main__':
    for k, v in TABLE.items():
        sp = dict(orig='%s.rs.txt' % k, file=v['file'], nest=False, children=v['children'], external=v['external'])
        if v.get('patch'):
            sp['patch'] = v['patch']
        with open(os.path.join(T, 'specs', k + '.py'), 'w', encoding='utf-8') as fh:
            fh.write('SPEC = %s\n' % json.dumps(sp, ensure_ascii=False, indent=1).replace('true', 'True').replace('false', 'False'))
        print('wrote', k)
