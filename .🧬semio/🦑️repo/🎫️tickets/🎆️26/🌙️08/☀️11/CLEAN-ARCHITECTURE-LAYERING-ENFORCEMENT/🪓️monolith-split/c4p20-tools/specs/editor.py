from specs.lib import *

SPEC = dict(
    orig='editor.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    external=[],
    children=[
        ch('🔌', 'io-and-commands', 'io_and_commands', [(98, 232)],
           'Media I/O and commands of the remodeling editor: the workflow photo stream id, the typed photo and mesh ports and the aggregated command enum with its import routing.'),
        ch('🌉', 'action-bridge', 'action_bridge', [(275, 460)],
           'Action bridge of the remodeling editor: the typed argument decoding that turns a manifest action invocation into a remodeling command.'),
        ch('🚧', 'retained-commands', 'retained_commands', [(469, 740)],
           'Retained commands of the remodeling editor: the retained tool ids and publication contracts, the routes and the window-config command work with its job factory.'),
        ch('🎛', 'artifact-editor', 'artifact_editor', [(744, 1048)],
           'Artifact editor of the remodeling editor: the dispatch-only implementation of the artifact editor trait for the remodeling play app.'),
        ch('📜', 'manifest', 'manifest', [(1053, 1363)],
           'Manifest of the remodeling editor: the app definition stitched from one definition per taxonomy node.'),
    ],
)
