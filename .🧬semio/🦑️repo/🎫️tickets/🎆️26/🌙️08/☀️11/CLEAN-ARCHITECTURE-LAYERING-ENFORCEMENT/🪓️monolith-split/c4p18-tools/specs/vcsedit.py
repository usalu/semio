from specs.common import ch
E = '✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor'

SPEC = dict(
    orig='vcsedit.rs.txt',
    file=E + '/🦀️.rs',
    nest=False,
    children=[
        ch('📋', 'constants-and-ui-helpers', 'constants_and_ui_helpers', [(55, 135)], 'Constants and UI value helpers of the VCS play app: the app ids, the interaction domain, the action and UI value constructors and the example document effect.'),
        ch('🕹', 'command-surface-and-job-factories', 'command_surface_and_job_factories', [(139, 161), (180, 185), (188, 261), (549, 660)], 'Command surface of the VCS play app: the aggregated command enum, the app type, the bounded and resumable publication contracts and the retained tool job factories.'),
        ch('✍', 'edit-command-work', 'edit_command_work', [(263, 547)], 'Edit command work of the VCS play app: the text edit phases, the remnant and the resumable edit command work that drives them.'),
        ch('📬', 'store-preparation-and-proofs', 'store_preparation_and_proofs', [(664, 812)], 'Store preparation of the VCS play app: the one-item preparation factory and the bounded and resumable proof catalogs.'),
        ch('🧭', 'editor-and-manifest', 'editor_and_manifest', [(815, 1138)], 'Editor impl and manifest of the VCS play app: the dispatch-only artifact editor surface and the app definition that stitches every command, panel and window.'),
    ],
    external=[E + '/🧪️tests/🔬️unit/🦀️.rs', E + '/📚️examples/🎬️demo-session/🦀️.rs', E + '/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs'],
)
