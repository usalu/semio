from specs.lib import *
P3 = '✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot'
CH = [
    ch('🪚', 'machines-and-catalogs', 'machines_and_catalogs', [(83, 327), (328, 366), (1015, 1063)],
       'Machines and catalogs of the process3d snapshot: the stock quantity, measure kind and recipe, the capability rules and parameters, the workshop machine with its patch, the workshop, the generic machine set, the machine catalog trait with its closed dispatch enum (closed in the module of the trait because of macro textual scoping), the contributed catalog and the machines topic payload.',
       extra_defs={'MachineCatalogs': 'pub'}),
    ch('📐', 'document-records', 'document_records', [(369, 583)],
       'Document records of the process3d snapshot: the axis and flag defaults, the pose and step origin, the working solid, the stock, the process measure and step with their patches and the working scene.'),
    ch('🧱', 'brep-converters', 'brep_converters', [(589, 780)],
       'B-Rep converters of the process3d snapshot: the analytic B-Rep per working solid, the shipped reference solids and the box, cylinder and sphere topology builders.'),
    ch('🔀', 'flow-and-scene-converters', 'flow_and_scene_converters', [(787, 913), (920, 991)],
       'Flow and scene converters of the process3d snapshot: the genesis child pack and restore projection, the conversions between process steps and flow nodes, the working scene to and from the persisted snapshot and the empty snapshot.'),
]
SPEC = dict(orig='p3.rs', file=P3 + '/🦀️.rs', children=CH, external=[], nest=True, expect_missing=['include!("../../🚪️io/🪶️sqlite/📸️snapshot/🪆️native-fields/🦀️.rs");'], expect_extra=['include!("../../../🚪️io/🪶️sqlite/📸️snapshot/🪆️native-fields/🦀️.rs");'], leaf_concat=True, source_concat=True, oracle_root=os.path.normpath(P3 + '/../🦀️.rs'))
