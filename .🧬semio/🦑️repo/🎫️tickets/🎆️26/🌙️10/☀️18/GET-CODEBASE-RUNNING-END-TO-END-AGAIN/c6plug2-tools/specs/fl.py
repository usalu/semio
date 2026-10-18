from specs.lib import *
F = FILES['fl']
CH = [
    ch('🙈', 'internal-actions-and-context-menu', 'internal_actions_and_context_menu', [(106, 109), (172, 263)],
       'Internal actions and context menu: the palette-hidden internal action, the context menu items and the content child.'),
    ch('🗃', 'direct-store-emit-and-work', 'direct_store_emit_and_work', [(266, 437)],
       'Direct store emit and work: the direct store tool ids, the emit, the generation preview, the generation window transient and the direct store work.'),
    ch('🧰', 'direct-store-command-work', 'direct_store_command_work', [(439, 865)],
       'Direct store command work: the artifact command work implementation of the direct store lane with its stepping, restore and close.'),
    ch('🏭', 'direct-store-factory-and-child-group-route', 'direct_store_factory_and_child_group_route', [(867, 1213)],
       'Direct store factory and child group route: the direct store job factory and the child group retained route with its work, factory and tool job implementations.'),
    ch('📤', 'host-only-retained-route', 'host_only_retained_route', [(1217, 1433)],
       'Host only retained route: the host only tool ids, the host effect payload, job and job factory and the scalar command and host wire views.'),
    ch('🕸', 'graph-operation-route', 'graph_operation_route', [(1437, 1770)],
       'Graph operation route: the graph operation tool ids, references, work and job factory and the job factory proofs of the retained routes.'),
    ch('🧩', 'contributions-route-and-instance-owner', 'contributions_route_and_instance_owner', [(1773, 2004)],
       'Contributions route and instance owner: the contributions tool ids, work and job factory and the flow instance operation owner.'),
    ch('🎛', 'flow-play-app', 'flow_play_app', [(2006, 2492)],
       'Flow play app: the flow play app and its `ArtifactEditor` implementation.'),
    ch('🚀', 'app-assembly', 'app_assembly', [(2496, 2700)],
       'App assembly: the app creation with its manifest stitch.'),
]
SPEC = dict(orig='fl.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='fl.rs'), keep_empty=('\U0001F9EA️UnitTests', '\U0001F9EA️Tests', '\U0001FAA2️TaxonomyMounts'))
