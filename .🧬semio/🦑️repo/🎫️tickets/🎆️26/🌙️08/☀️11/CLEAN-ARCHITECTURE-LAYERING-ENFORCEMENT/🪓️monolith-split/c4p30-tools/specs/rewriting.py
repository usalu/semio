from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
SPEC = dict(
    orig='rewriting-editor.rs.txt',
    file=F,
    wiring='each',
    unwrap_removed=['mod args_bridge {', '}'],
    children=[
        dict(folder='🌉️args-bridge', mod='args_bridge', unwrap='args_bridge', glob=False, reexport=False, nowiden=True,
             ranges=[(448, 522)],
             doc='//! 🌉️ Rewriting action bridge: folds the shell `{action, args}` vocabulary (camelCase keys, JSON floats, control values, the node-graph host operations array) into the typed `TrinityRewritingCommand`.'),
        dict(folder='🧵️retained-document-commands', mod='retained_document_commands',
             ranges=[(526, 681)],
             doc='//! 🧵️ Rewriting retained document commands: the document tool ids and limits, the bounded document job factory and the document tool job builder.'),
        dict(folder='🕹️play-app', mod='play_app',
             ranges=[(685, 961)],
             doc='//! 🕹️ Rewriting play app: the `TrinityRewritingPlayApp` editor and its `ArtifactEditor` implementation.'),
        dict(folder='📋️manifest', mod='manifest',
             ranges=[(967, 1122)],
             doc='//! 📋️ Rewriting manifest: the app manifest stitch of `create_rewriting_app`.'),
    ],
    external=_ext.mounted(F),
)
