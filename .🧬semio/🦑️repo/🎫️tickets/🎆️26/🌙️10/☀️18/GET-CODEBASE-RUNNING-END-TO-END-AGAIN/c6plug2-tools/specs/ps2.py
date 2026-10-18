from specs.lib import *
F = FILES['ps2']
def pre(t):
    old = '\n//#region 🧩️EditorContract\n'
    assert t.count(old) == 1
    return t.replace(old, '\n//#endregion 🧵️RetainedCommands\n' + old)
CH = [
    ch('🌱', 'play-snapshot-authority', 'play_snapshot_authority', [(8, 260)],
       'Play snapshot authority: the immutable typed puzzle authority with its host view, mutation and diff algebra, retirement and the pack, dsl and sqlite codecs.'),
    ch('🔖', 'constants-examples-and-scene', 'constants_examples_and_scene', [(292, 568)],
       'Constants, examples and scene: the controller and granularity constants, the shipped example json, the scene and interaction snapshot and the selection writes.'),
    ch('📇', 'kind-catalogs-and-shipped-kinds', 'kind_catalogs_and_shipped_kinds', [(570, 872)],
       'Kind catalogs and shipped kinds: the manifest and document kind catalogs, the inferred kind entries and the shipped node kinds with their options.'),
    ch('🏷', 'node-labels-ids-and-placement-geometry', 'node_labels_ids_and_placement_geometry', [(874, 1293)],
       'Node labels, ids and placement geometry: the display labels, the id allocation, the target regions, the handle geometry and kind compatibility and the clipboard and lock helpers.'),
    ch('🔁', 'host-sync-and-ui-scopes', 'host_sync_and_ui_scopes', [(1298, 1411)],
       'Host sync and ui scopes: the board host content and runtime sync and the window and selection ui dirty scopes.'),
    ch('🧵', 'command-variants', 'command_variants', [(1413, 1553)],
       'Command variants: the command variant macro, the puzzle2d command enum it declares and its binary op implementation.'),
    ch('🧰', 'retained-tool-ids-and-scene-builders', 'retained_tool_ids_and_scene_builders', [(1556, 1706)],
       'Retained tool ids and scene builders: the retained, host configuration, job and generic tool ids and the scene builders.'),
]
PP = [('mod command_variants;', 'mod command_variants;\npub use command_variants::Puzzle2dCommand;')]
SPEC = dict(orig='ps2.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='ps2.rs'), pre=pre, patch_parent=PP,
            expect_extra=['pub use command_variants::Puzzle2dCommand;', '//#endregion 🧵️RetainedCommands'])
