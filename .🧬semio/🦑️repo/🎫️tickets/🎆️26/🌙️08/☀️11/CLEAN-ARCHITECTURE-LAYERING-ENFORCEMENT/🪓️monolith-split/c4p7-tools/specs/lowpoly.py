D = '✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'
SPEC = dict(
    orig='lowpoly-editor.rs.txt',
    file=D + '/🦀️.rs',
    wiring='each',
    unwrap_removed=['mod args_bridge {', '}'],
    children=[
        dict(folder='🌉️args-bridge', mod='args_bridge', unwrap='args_bridge', glob=False, reexport=False,
             ranges=[(228, 416)],
             doc='//! 🌉️ Lowpoly action bridge: folds the host `{action, args}` vocabulary into the snake_case `FromValue` payloads of the command nodes and builds the typed `LowpolyCommand` from an action name.'),
        dict(folder='🧵️retained-commands', mod='retained_commands',
             ranges=[(420, 995)],
             doc='//! 🧵️ Lowpoly retained commands: the migrated tool ids, the retained payload limits, the command disposition and admission, the resumable reducer, the command work cursor and the owned tool job factory.'),
        dict(folder='📬️store-preparation', mod='store_preparation',
             ranges=[(999, 1273)],
             doc='//! 📬️ Lowpoly store preparation: the exact retained-byte accounting, the one-item admission footprints and the artifact and config store preparation factories.'),
        dict(folder='🕹️play-app', mod='play_app', wire_at=1335,
             ranges=[(1277, 1675)],
             doc='//! 🕹️ Lowpoly play app: the `LowpolyPlayApp` editor with its media export, its render and the `ArtifactEditor` implementation.'),
        dict(folder='📋️manifest', mod='manifest',
             ranges=[(1680, 1966)],
             doc='//! 📋️ Lowpoly manifest: the utility declarations and the app manifest stitch of `create_lowpoly_app`.'),
    ],
    external=[
        D + '/📚️examples/🎬️demo-session/🦀️.rs',
        D + '/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs',
    ],
)
