from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'cbs.rs'), encoding='utf-8').read()
CUTS = [(92, 92, [124, 333, 409, 484]), (811, 811, [863, 1049])]
ANCHORS = [(
    "    }\n\n    //#endregion 🪪️Artifact Ids\n}\n",
    "    }\n}\n\n//#endregion 🪪️Artifact Ids\n",
), (
    "    }\n\n    //#endregion 🧱️Aggregates\n}\n\n/// 🌳️ Walks a chain of folder segments, creating nothing.\n",
    "    }\n}\n\n//#endregion 🧱️Aggregates\n\n//#region 📚️Context\n\n/// 🌳️ Walks a chain of folder segments, creating nothing.\n",
)]

CH = [
    ch('🙈', 'generated-folders-and-bundle-labels', 'generated_folders_and_bundle_labels', [(41, 78)],
       'Generated folders and bundle labels of the codebase: the generated output folder test and the bundle label normalisation.'),
    ch('🏠', 'codebase-handle', 'codebase_handle', [(84, 123)],
       'Codebase handle: the codebase bound to one repository root with its construction, root accessors and cache invalidation.'),
    ch('🏗', 'technology-detection', 'technology_detection', [(127, 334)],
       'Technology detection of the codebase: the technology and bundle detection that gives every path its owning bundle.'),
    ch('🚫', 'ignore-rules', 'ignore_rules', [(340, 412)],
       'Ignore rules of the codebase: the compiled gitignore, the repo exclusion and the filtering of considered files.'),
    ch('🚶', 'walk', 'walk', [(418, 489)],
       'Walk of the codebase: the lexical subtree walk, the extension glob and the scope to file resolution.'),
    ch('🪪', 'artifact-ids', 'artifact_ids', [(495, 625)],
       'Artifact ids of the codebase: the folder kind derivation and the folder, file and section id and uri builders.'),
    ch('🧭', 'scope-and-helpers', 'scope_and_helpers', [(635, 801)],
       'Scope and helpers of the codebase: the walk scope and the directory, bundle name, ignore pattern, section, definition and contributor helpers.'),
    ch('📚', 'codebase-context', 'codebase_context', [(803, 870)],
       'Codebase context: everything one codebase projection is built from, with its construction and loading.'),
    ch('🧱', 'bundle-folder-and-file-aggregates', 'bundle_folder_and_file_aggregates', [(876, 1058)],
       'Bundle, folder and file aggregates of the codebase context.'),
    ch('🌳', 'section-definition-breach-and-tree-aggregates', 'section_definition_breach_and_tree_aggregates', [(1060, 1236)],
       'Section, definition, breach and tree aggregates of the codebase context and the folder tree helpers.'),
]

SPEC = dict(
    orig='cbs.rs',
    file=CBS + '/🦀️.rs',
    children=CH,
    pre=chain(anchored(ANCHORS), split_impls_h(CUTS)),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[],
)
