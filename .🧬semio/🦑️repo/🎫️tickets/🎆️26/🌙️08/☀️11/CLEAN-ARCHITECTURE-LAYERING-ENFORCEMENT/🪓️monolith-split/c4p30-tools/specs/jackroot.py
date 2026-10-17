from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️.rs'
SPEC = dict(
    orig='jack-root.rs.txt',
    file=F,
    wiring='each',
    children=[
        dict(folder='🔨️modules/⚠️errors', mod='errors',
             ranges=[(64, 205)],
             doc='//! ⚠️ Jack errors: the `TrinityRamError` of graph snapshots, manifest validation and mutations with its `Display`, `Error` and `From` conversions.'),
        dict(folder='🔨️modules/🔺️graph-model', mod='graph_model',
             ranges=[(212, 527)],
             doc='//! 🔺️ Jack graph model: the runtime `Port`, `Node`, `Edge`, `Camera` and `Graph` records, the scene validation, the entity references, the port key helpers, the graph constants and the artifact kind.'),
        dict(folder='🔨️modules/🧩️registration', mod='registration',
             ranges=[(531, 664)],
             doc='//! 🧩️ Jack registration: the pilot language specs, the `ArtifactApps` bound, the artifact definition and the artifact declaration.'),
    ],
    external=_ext.mounted(F),
)
