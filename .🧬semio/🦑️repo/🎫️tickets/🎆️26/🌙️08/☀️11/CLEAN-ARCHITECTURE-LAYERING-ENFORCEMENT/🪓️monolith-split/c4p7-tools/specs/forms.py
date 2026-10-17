import os
R = '/Users/ueli/Documents/semio'
D = '✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'


def _external():
    out = []
    for dp, dn, fn in os.walk(os.path.join(R, D)):
        for f in fn:
            if f.endswith('.rs'):
                rel = os.path.relpath(os.path.join(dp, f), R)
                if rel != D + '/🦀️.rs':
                    out.append(rel)
    return sorted(out)


SPEC = dict(
    orig='forms-editor.rs.txt',
    file=D + '/🦀️.rs',
    wiring='each',
    unwrap_removed=['mod args_bridge {', '}'],
    children=[
        dict(folder='🧩️contributions', mod='contributions',
             ranges=[(202, 294)],
             doc='//! 🧩️ Forms contributions: the host-declared plugin contributions parsed from the configuration, the question kind routes and the extension question rendering and catalogue kinds shared by the builder, the try wizard and the inspection panel.'),
        dict(folder='🌉️args-bridge', mod='args_bridge', unwrap='args_bridge', glob=False, reexport=False,
             ranges=[(298, 436)],
             doc='//! 🌉️ Forms action bridge: folds the shell `{action, args}` vocabulary (camelCase keys, block-list verbs, merged control values) into the typed `FormsCommand`.'),
        dict(folder='🧵️retained-commands', mod='retained_commands',
             ranges=[(518, 780)],
             doc='//! 🧵️ Forms retained commands: the retained tool ids and limits, the window command work cursor and the bounded command job factory.'),
        dict(folder='📬️store-preparation', mod='store_preparation',
             ranges=[(785, 918)],
             doc='//! 📬️ Forms store preparation: the exact retained-byte accounting, the one-item admission footprint and the store preparation factory.'),
        dict(folder='🕹️play-app', mod='play_app', wire_at=922,
             ranges=[(922, 1237)],
             doc='//! 🕹️ Forms play app: the `FormsPlayApp` editor and its `ArtifactEditor` implementation.'),
        dict(folder='📋️manifest', mod='manifest',
             ranges=[(1241, 1419)],
             doc='//! 📋️ Forms manifest: the app manifest stitch of `create_forms_app`.'),
    ],
    external=_external(),
)
