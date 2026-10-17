E = '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine'
SPLITS = [
    '    /// 🎯️ One fuel-bounded luma/detect/describe slice; returns whether more frames remain.\n',
    '    /// 📦️ Moves the final sparse snapshot, observation table, and dense-camera index through bounded\n',
    '    /// 📈️ Coarse `[0, 1]` progress from the current stage\'s ordinal alone',
]


def pre(text):
    for a in SPLITS:
        anchor = '\n\n' + a
        assert text.count(anchor) == 1, (a, text.count(anchor))
        text = text.replace(anchor, '\n}\n\nimpl ReconstructionEngine {\n' + a)
    return text


SPEC = dict(
    orig='recon.rs.txt',
    file=E + '/🏭️reconstruction/🦀️.rs',
    nest=False,
    pre=pre,
    keep_empty=['🔖️Tests'],
    pre_extra=['}'] * 3 + ['impl ReconstructionEngine {'] * 3,
    children=[
        dict(folder='📥️frame-ingest', mod='frame_ingest', ranges=[(17, 228)],
             doc='//! 📥️ Frame ingest of the reconstruction engine: the ingest parameters, the frame acceptance with its blur gate and the frame source. See [`super`].'),
        dict(folder='🎛️engine-vocabulary', mod='engine_vocabulary', ranges=[(230, 488)],
             doc='//! 🎛️ Vocabulary of the reconstruction engine: the engine parameters, the stage, status and observation enums and the reconstruction frame helpers. See [`super`].'),
        dict(folder='🏗️engine-core', mod='engine_core', ranges=[(490, 555), (869, 1017), (1919, 2012)],
             doc='//! 🏗️ Core of the reconstruction engine: the engine state, its construction and observation and the budgeted advance loop that drives the stages. See [`super`].'),
        dict(folder='🔬️sparse-stages', mod='sparse_stages', ranges=[(567, 669), (746, 868), (1019, 1550)],
             doc='//! 🔬️ Sparse stages of the reconstruction engine: the feature extraction, pair matching and verification, track building, pose estimation and bundle adjustment steps with their preparations. See [`super`].'),
        dict(folder='🧊️dense-and-meshing-stages', mod='dense_and_meshing_stages', ranges=[(557, 565), (671, 744), (1552, 1917), (2014, 2019)],
             doc='//! 🧊️ Dense and meshing stages of the reconstruction engine: the finalisation, dense stereo, volume bounds, fusion and meshing steps with their preparations. See [`super`].'),
        dict(folder='👁️preview-and-products', mod='preview_and_products', ranges=[(2023, 2292)],
             doc='//! 👁️ Previews and products of the reconstruction engine: the scene preview, the terminal sparse, quality and geo chunks and the geo products. See [`super`].'),
    ],
    external=[E + '/🏭️reconstruction/🧪️tests/🔬️terminal-sparse-chunk/🦀️.rs'],
)
