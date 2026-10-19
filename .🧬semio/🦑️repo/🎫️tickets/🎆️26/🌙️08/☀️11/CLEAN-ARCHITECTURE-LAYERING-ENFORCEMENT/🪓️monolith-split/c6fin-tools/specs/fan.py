from specs.lib import *
F_ = FA = '✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
CH = [
    ch('🔌', 'presentation-io-and-frame-helpers', 'presentation_io_and_frame_helpers', [(91, 132), (136, 147)],
       'Presentation IO and frame helpers: the presentation media IO surface, the frame import grid columns, the next frame tile id and crop and the frame media name.'),
    ch('🧵', 'retained-command-tool-jobs', 'retained_command_tool_jobs', [(183, 342)],
       'Retained command tool jobs: the retained tool ids, payload schema and capacities, the publication contracts, the retained contract, selection and extent reductions and the retained command job factory.'),
    ch('📬', 'config-store-preparation', 'config_store_preparation', [(346, 518)],
       'Config store preparation: the config preparation factory and byte counter, the config edit bytes and fault and the one item publication implementations of the config store.'),
    ch('📥', 'reserved-presentation-import', 'reserved_presentation_import', [(522, 748)],
       'Reserved presentation import: the import tool id and port, the import job with its actions, its interactive job implementation and its reserved job implementation.'),
    ch('🎛', 'play-app-editor', 'play_app_editor', [(752, 1058)],
       'Play app editor: the animate presentation play app and its artifact editor implementation.'),
    ch('🧾', 'manifest-and-app-constructor', 'manifest_and_app_constructor', [(1062, 1215)],
       'Manifest and app constructor: the figure tile frame argument fields and the animate presentation app constructor with its manifest stitch.'),
]
SPEC = dict(orig='fan.rs', file=FA, children=CH, wiring='block', external=mounted(FA, orig='fan.rs'), external_prod=family_prod(F_, up=5, exclude=[c['folder'] for c in CH]), keep_empty=('Interaction', 'TaxonomyMounts'))
