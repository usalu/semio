from specs.lib import *
F = FILES['fe']
CH = [
    ch('🧾', 'retained-command-contracts-and-work', 'retained_command_contracts_and_work', [(162, 356)],
       'Retained command contracts and work: the retained tool ids, the publication contracts, the document item reducers and the retained command work.'),
    ch('🏭', 'retained-job-factories-and-preparation', 'retained_job_factories_and_preparation', [(358, 541)],
       'Retained job factories and preparation: the retained command job factory and the artifact preparation with its one item store publication.'),
    ch('📊', 'result-json-and-ports', 'result_json_and_ports', [(552, 678)],
       'Result json and ports: the result export helpers, the io ports and the dof action argument helpers.'),
    ch('🎛', 'play-app-editor', 'play_app_editor', [(682, 1275)],
       'Play app editor: the fem2d play app and its `ArtifactEditor` implementation.'),
    ch('🚀', 'ui-helpers-and-app-assembly', 'ui_helpers_and_app_assembly', [(1278, 1580)],
       'UI helpers and app assembly: the utility helpers, the patch action and the app creation with its manifest stitch.'),
]
SPEC = dict(orig='fe.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fe.rs'), keep_empty=('\U0001F9EA️UnitTests', '\U0001F516️Constants'))
