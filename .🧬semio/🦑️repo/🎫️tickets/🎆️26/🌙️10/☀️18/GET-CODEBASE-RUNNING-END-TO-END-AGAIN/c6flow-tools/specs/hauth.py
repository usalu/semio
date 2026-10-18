from specs.lib import *
F = FLOW + '/🖥️host/✏️host-authoring/🦀️.rs'
H = 'impl FlowHost {'
CUTS = [
    ('pub fn replace_host_snapshot', H),
    ('pub fn apply_eval_outputs_json', H),
    ('pub fn set_viewport', H),
    ('pub fn set_ghost_widget', H),
    ('pub fn connect(', H),
    ('pub fn insert_between', H),
    ('pub fn reorganize', H),
]


def pre(t):
    t, _ = cut_impls(t, CUTS)
    return t


CH = [
    ch('🏗', 'construction-and-registry', 'construction_and_registry', [(5, 100)],
       'Construction and registry of the flow host: the operator registry and geometry port builders, the history store factory and the host snapshot constructors.'),
    ch('🗂', 'snapshot-catalogue-and-kind-infos', 'snapshot_catalogue_and_kind_infos', [(102, 258)],
       'Snapshot, catalogue and kind infos of the flow host: replacing and applying host snapshots, the history store reset, the host catalogue and the neuron kind infos.'),
    ch('📊', 'evaluation-baseline-and-publication', 'evaluation_baseline_and_publication', [(260, 484)],
       'Evaluation baseline and publication of the flow host: applying evaluation outputs, the baseline install and currency, the baseline and evaluation publication steps and the computing chrome.'),
    ch('🎥', 'camera-and-pointer-planning', 'camera_and_pointer_planning', [(486, 676)],
       'Camera and pointer planning of the flow host: the viewport, the camera, the interaction projection, wheel and pointer planning and commit and the screen wheel gestures.'),
    ch('🪄', 'ghost-widget-and-widget-editing', 'ghost_widget_and_widget_editing', [(678, 754)],
       'Ghost widget and widget editing of the flow host: the ghost widget and adding, removing and moving widgets.'),
    ch('🔗', 'synapse-and-port-editing', 'synapse_and_port_editing', [(756, 1013)],
       'Synapse and port editing of the flow host: connecting, the synapse ids, adding and removing input and output ports and disconnecting.'),
    ch('🧲', 'gumball-editing', 'gumball_editing', [(1016, 1111)],
       'Gumball editing of the flow host: inserting a widget between two others, making space and setting the neuron parameters.'),
    ch('🖱', 'reorganize-and-screen-pointer', 'reorganize_and_screen_pointer', [(1114, 1267)],
       'Reorganize and screen pointer of the flow host: the tree layout reorganize, the screen pointer down, move, up and cancel, the graph edits drain and the selection.'),
]
SPEC = dict(orig='hauth.rs', nest=True, file=F, children=CH, wiring='block', external=mounted(F, orig='hauth.rs'), pre=pre, pre_extra=['}', H] * len(CUTS))
