from specs.lib import *
F = FLOW + '/🖥️host/🌳️host-evaluation-history/🦀️.rs'
H = 'impl FlowHost {'
CUTS = [
    ('pub fn compiled_wire_literal', H, 'keep'),
    ('pub fn selected_widget_ids', H),
    ('pub(super) fn sync_from_dag', H),
    ('pub fn collapse_selection', H),
    ('pub fn history_store_from_baseline', H),
]
TAIL = '    // #endregion History\n}'


def pre(t):
    t, _ = cut_impls(t, CUTS)
    assert t.count(TAIL) == 1
    return t.replace(TAIL, '}\n// #endregion History')


CH = [
    ch('🧮', 'evaluation-steps-and-tree-building', 'evaluation_steps_and_tree_building', [(6, 280)],
       'Evaluation steps and tree building of the flow host: the bounded evaluation steps, blocked ports, dirty widget ids, the status trees and the tree and cluster tree built from the DAG.'),
    ch('🔄', 'seeds-outputs-and-dag-sync', 'seeds_outputs_and_dag_sync', [(282, 415)],
       'Seeds, outputs and DAG sync of the flow host: the compiled wire literal, the evaluation seeds, preview and export outputs and the DAG display, ghost and rebuild synchronisation.'),
    ch('🎯', 'selection-hover-and-preview-toggles', 'selection_hover_and_preview_toggles', [(417, 527)],
       'Selection, hover and preview toggles of the flow host: the selected widget ids, hover facts, the selection alignment and the preview off set.'),
    ch('📝', 'dag-sync-widget-properties-and-canvas-settings', 'dag_sync_widget_properties_and_canvas_settings', [(529, 829)],
       'DAG sync, widget properties and canvas settings of the flow host: syncing from the DAG, the host snapshot, slider, note, variable and image edits and the canvas theme, level of detail and grid settings.'),
    ch('🗜', 'clusters', 'clusters', [(831, 1094)],
       'Clusters of the flow host: collapsing a selection into a cluster and exploding a cluster.'),
    ch('⏪', 'undo-redo-history', 'undo_redo_history', [(1097, 1365)],
       'Undo and redo history of the flow host: the history store, the change leaves and journal, the edit recording, gestures and the undo and redo steps.'),
]
SPEC = dict(orig='hhist.rs', nest=True, file=F, children=CH, wiring='block', external=mounted(F, orig='hhist.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
