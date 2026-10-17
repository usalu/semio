from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'gol.rs'), encoding='utf-8').read()
CUTS = [(568, 568, [598, 690, 761])]

CH = [
    ch('🚨', 'errors', 'errors', [(31, 95)],
       'Errors of the goals: the goal error with its display and its conversion from the model error.'),
    ch('🪪', 'id-scheme', 'id_scheme', [(97, 197)],
       'Id scheme of the goals: the `GOAL/SUBGOAL` identifier depth, parent, root, filesystem and compose id conversions and the trailing number parsers.'),
    ch('🗄', 'store-and-codec', 'store_and_codec', [(199, 385)],
       'Store and codec of the goals: the goal document codec, the goal store port and its memory and filesystem stores.'),
    ch('🧩', 'management-and-emission', 'management_and_emission', [(387, 556)],
       'Management and emission of the goals: the management port with its null and recording implementations and the memory emitter.'),
    ch('🏛', 'goals-aggregate', 'goals_aggregate', [(558, 597)],
       'Goals aggregate: the goal aggregate bound to its ports with its construction, list, read and search.'),
    ch('✨', 'goal-create', 'goal_create', [(599, 691)],
       'Goal create: the creation of a goal and the opening of its management milestone or issue.'),
    ch('✏', 'goal-change-and-close', 'goal_change_and_close', [(693, 764)],
       'Goal change and close: the change and close operations of the goal aggregate.'),
    ch('🔄', 'goal-reopen-delete-and-sync', 'goal_reopen_delete_and_sync', [(766, 876)],
       'Goal reopen, delete and sync: the reopen and delete operations, the management synchronisation, retitle, relocate and the event emission.'),
    ch('🌱', 'tree-seeds', 'tree_seeds', [(878, 971)],
       'Tree seeds of the goals: the flat goal and ticket records the tree is assembled from, the tree format and the plain tree lines.'),
    ch('🌳', 'tree-build', 'tree_build', [(973, 1113)],
       'Tree build of the goals: the goal tree assembly, ticket nesting and node ordering.'),
    ch('🖨', 'tree-render', 'tree_render', [(1115, 1199)],
       'Tree render of the goals: the open counts and the rendering of the goal tree as lines.'),
]

SPEC = dict(
    orig='gol.rs',
    file=GOL + '/🦀️.rs',
    children=CH,
    pre=split_impls_h(CUTS),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[],
)
