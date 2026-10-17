from specs.common import ch
E = '✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'

SPEC = dict(
    orig='gisedit.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    children=[
        ch('📋', 'constants-and-io', 'constants_and_io', [(78, 310)], 'Constants and typed media I/O of the GIS 2D play app: the app id, the interaction domain and snapshot, the layer ids, the UI value helpers and the I/O ports.'),
        ch('🕹', 'commands-and-retained-jobs', 'commands_and_retained_jobs', [(314, 338), (349, 355), (358, 524)], 'Commands of the GIS 2D play app: the aggregated command enum, the app type and the retained tool job factories with their publication contracts.'),
        ch('📬', 'one-item-preparation', 'one_item_preparation', [(528, 892)], 'One-item preparation of the GIS 2D play app: the stamped and unstamped preparation factories and the one-item store publication of a map edit.'),
        ch('🧭', 'play-app-editor', 'play_app_editor', [(895, 1272)], 'Editor impl of the GIS 2D play app: the context menu, the dispatch-only artifact editor surface and the window actions.'),
        ch('🧾', 'manifest', 'manifest', [(1276, 1480)], 'Manifest of the GIS 2D play app: the layer argument options and the app definition that stitches every command, panel and window.'),
    ],
    external=[E + '/🧪️tests/🔬️unit/🦀️.rs'],
)
