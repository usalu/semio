from specs.lib import *
F = FILES['we']
CH = [
    ch('🧾', 'command-enum-and-retained-routes', 'command_enum_and_retained_routes', [(68, 374)],
       'Command enum and retained routes: the wfc2d editor command with its op binary and ids, the retained tool ids and publication contracts, the command work and the retained job factory.'),
    ch('🕸', 'graph-view-args-and-gestures', 'graph_view_args_and_gestures', [(378, 621)],
       'Graph view, args and gestures: the graph view, the editor, the command argument readers, the graph gestures and node graph edit, the example document and the guards.'),
    ch('🧭', 'dispatch-and-artifact-editor', 'dispatch_and_artifact_editor', [(623, 1059)],
       'Dispatch and artifact editor: the command dispatch, the render body and the `ArtifactEditor` implementation with its transient render.'),
    ch('🚀', 'editor-assembly', 'editor_assembly', [(1063, 1179)],
       'Editor assembly: the action and example argument helpers and the editor creation with its manifest stitch.'),
]
SPEC = dict(orig='we.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='we.rs'), keep_empty=empties('we'))
