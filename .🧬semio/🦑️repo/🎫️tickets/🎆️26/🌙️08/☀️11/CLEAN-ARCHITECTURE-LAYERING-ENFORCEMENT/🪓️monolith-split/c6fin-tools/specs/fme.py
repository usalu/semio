from specs.lib import *
F_ = FE = '✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
CH = [
    ch('📄', 'document-io-algorithm-and-geometry-json', 'document_io_algorithm_and_geometry_json', [(70, 82), (88, 109), (113, 237)],
       'Document IO, algorithm and geometry JSON: the math app id and equation document reset, the equation IO surface, the algorithm overlay, the workflow JSON and the geometry layers JSON.'),
    ch('📏', 'retained-command-limits-and-work-state', 'retained_command_limits_and_work_state', [(261, 423)],
       'Retained command limits and work state: the equation tool ids and retained capacities, the publication contracts, the identities, the graph shape and edit preflight, the command extent, the work phases and close units and the retained command work struct.'),
    ch('🛠', 'retained-command-work-and-job-factory', 'retained_command_work_and_job_factory', [(425, 878)],
       'Retained command work and job factory: the stepping of the retained command work, its artifact command work implementation, the command job factory and its tool job factory implementations.'),
    ch('📬', 'store-preparation', 'store_preparation', [(882, 1084)],
       'Store preparation: the equation store preparation factory and preparation with the one item publication implementations of the store.'),
    ch('🎛', 'play-app-editor-and-constructor', 'play_app_editor_and_constructor', [(1088, 1335), (1339, 1422)],
       'Play app editor and constructor: the equation play app with its artifact editor implementation and the equation app constructor with its manifest stitch.'),
]
SPEC = dict(orig='fme.rs', file=FE, children=CH, wiring='block', external=mounted(FE, orig='fme.rs'), external_prod=family_prod(F_, up=5, exclude=[c['folder'] for c in CH]), keep_empty=('TaxonomyMounts', 'Interaction'))
