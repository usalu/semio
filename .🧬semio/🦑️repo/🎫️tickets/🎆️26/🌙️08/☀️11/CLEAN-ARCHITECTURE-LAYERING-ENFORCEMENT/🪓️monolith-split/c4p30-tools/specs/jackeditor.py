from . import _ext
F = '✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
SPEC = dict(
    orig='jack-editor.rs.txt',
    file=F,
    wiring='each',
    unwrap_removed=['mod args_bridge {', '}'],
    children=[
        dict(folder='🌉️args-bridge', mod='args_bridge', unwrap='args_bridge', glob=False, reexport=False, nowiden=True,
             ranges=[(325, 403)],
             doc='//! 🌉️ Jack action bridge: folds the shell `{action, args}` vocabulary into the typed `TrinityJackCommand`.'),
        dict(folder='🧵️retained-commands', mod='retained_commands',
             ranges=[(412, 694)],
             doc='//! 🧵️ Jack retained commands: the window configuration, transient and document tool ids and limits, the three bounded job factories and the transient command work.'),
        dict(folder='🕹️play-app', mod='play_app',
             ranges=[(407, 409), (697, 1072)],
             doc='//! 🕹️ Jack play app: the `TrinityJackPlayApp` editor and its `ArtifactEditor` implementation.'),
        dict(folder='📋️manifest', mod='manifest',
             ranges=[(1078, 1219)],
             doc='//! 📋️ Jack manifest: the app manifest stitch of `create_trinity_jack_app`.'),
    ],
    external=_ext.mounted(F),
)
