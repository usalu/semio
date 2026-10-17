from specs.lib import *


def pre(text):
    assert text.endswith('}\n')
    return text + '// #endregion 🔖️TwoViewFivePointTests\n'


SPEC = dict(
    orig='sfmunit.rs.txt',
    file=EN + '/📸️sfm/🧪️tests/🔬️unit/🦀️.rs',
    nest=False,
    pre=pre,
    pre_extra=['// #endregion 🔖️TwoViewFivePointTests'],
    external=[],
    children=[
        ch('🔀', 'geometry-tests', 'geometry_tests', [(8, 264)],
           'Geometry tests of the incremental structure from motion: the two-view model estimation, the triangulation and the perspective-n-point solvers against planted ground truth.', reexport=False),
        ch('🧭', 'incremental-tests', 'incremental_tests', [(268, 505)],
           'Incremental reconstruction tests of the structure from motion: the bounded step fixtures and ceilings, the full synthetic multi-camera run and the low-parallax initial pair.', reexport=False),
        ch('🌐', 'global-and-prior-tests', 'global_and_prior_tests', [(509, 670)],
           'Global averaging, loop closure and pose prior tests of the structure from motion: the rotation and translation averaging, the loop detection, the pose-graph optimisation and the similarity and ground-control priors.', reexport=False),
        ch('🎯', 'five-point-and-diagnostics', 'five_point_and_diagnostics', [(676, 1116)],
           'Five-point, bundle adjustment and diagnostic tests of the structure from motion: the shared pose helpers, the eight-point and five-point essential estimation, the true-pose checks, the bundle convergence and the ignored cost diagnostic.'),
    ],
)
