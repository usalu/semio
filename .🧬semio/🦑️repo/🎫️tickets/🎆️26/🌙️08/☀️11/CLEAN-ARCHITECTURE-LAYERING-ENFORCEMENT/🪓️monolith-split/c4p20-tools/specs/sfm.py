from specs.lib import *

SPLITS = [
    '    /// 🌱️ Seeds the reconstruction from an initial pair: estimates the essential matrix + relative pose via\n',
    '    /// 🎯️ Re-estimates every registered camera via PnP against the current triangulated cloud',
]


def pre(text):
    for a in SPLITS:
        anchor = '\n\n' + a
        assert text.count(anchor) == 1, (a, text.count(anchor))
        text = text.replace(anchor, '\n}\n\nimpl IncrementalSfm {\n' + a)
    return text


SPEC = dict(
    orig='sfm.rs.txt',
    file=EN + '/📸️sfm/🦀️.rs',
    nest=False,
    pre=pre,
    pre_extra=['}'] * 2 + ['impl IncrementalSfm {'] * 2,
    external=[orig('sfmunit')],
    children=[
        ch('🛰', 'synthetic-fixtures-and-priors', 'synthetic_fixtures_and_priors', [(114, 315)],
           'Synthetic fixtures and pose priors of the incremental structure from motion: the seeded synthetic scene with its observations and textured renderer, and the similarity alignment and ground-control residual of the pose priors.'),
        ch('🧱', 'incremental-state', 'incremental_state', [(67, 110), (321, 737)],
           'State of the incremental structure from motion: the interactive limits, the configuration, the reconstruction, the bundle problem, the phase and preparation records and the accessors of the incremental reconstruction.'),
        ch('🧭', 'seed-and-registration', 'seed_and_registration', [(739, 1315)],
           'Seed and registration steps of the incremental structure from motion: the initial pair, the two-view fallback registration, the track triangulation and the bounded seed-pair and registration advances.'),
        ch('🔩', 'bundle-and-snapshot', 'bundle_and_snapshot', [(1317, 1678)],
           'Bundle adjustment and snapshot steps of the incremental structure from motion: the PnP pose refinement, the local and global bundle adjustment, the outlier pruning and the bounded reconstruction snapshot.'),
    ],
)
