from specs.lib import *
F = FILES['ps3']
CH = [
    ch('🌱', 'play-snapshot-authority-and-codecs', 'play_snapshot_authority_and_codecs', [(16, 194), (1862, 1914)],
       'Play snapshot authority and codecs: the immutable typed puzzle authority with its host view, mutation and diff algebra, retirement and the dsl, pack and sqlite codecs.'),
    ch('🔖', 'constants-examples-and-scene-types', 'constants_examples_and_scene_types', [(221, 455), (1837, 1852)],
       'Constants, examples and scene types: the controller, granularity and hover constants, the shipped example json, the vortex, reference, object, target volume, attraction and scene types and the engagement verbs.'),
    ch('🎬', 'scene-snapshot-conversion', 'scene_snapshot_conversion', [(457, 686)],
       'Scene snapshot conversion: the clipboard fragments, the scene snapshot from and to the document and host snapshots and the scene object conversions.'),
    ch('🧱', 'catalog-rows-mesh-index-and-labels', 'catalog_rows_mesh_index_and_labels', [(688, 1027)],
       'Catalog rows, mesh index and labels: the kind row normalization, the catalog upserts, the mesh urls and scales, the vortex world positions and the object display labels.'),
    ch('🪟', 'windows-utilities-and-interaction', 'windows_utilities_and_interaction', [(1029, 1300)],
       'Windows, utilities and interaction: the fill tool and window id helpers, the utility map hit, the interaction snapshot and the number edits and focus selection.'),
    ch('⚖', 'joint-weights-and-distribution', 'joint_weights_and_distribution', [(1302, 1603)],
       'Joint weights and distribution: the joint weight keys, the distribution weights, the kind weight groups and the joint vortex measures and children.'),
    ch('🧵', 'command-variants', 'command_variants', [(1605, 1835)],
       'Command variants: the command variant macro, the puzzle3d command enum it declares and its binary op implementation.'),
]
PP = [('mod command_variants;', 'mod command_variants;\npub use command_variants::Puzzle3dCommand;')]
SPEC = dict(orig='ps3.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='ps3.rs'), patch_parent=PP,
            expect_extra=['pub use command_variants::Puzzle3dCommand;'])
