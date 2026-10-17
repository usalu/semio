from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'gwb.rs'), encoding='utf-8').read()
CUTS = [(52, 52, [312, 404, 605, 845])]

CH = [
    ch('🏗', 'open-and-discovery', 'open_and_discovery', [(52, 311)],
       'Open and discovery of the headless workspace: construction, the folder and hub openers, the origin, catalog discovery, artifact byte reads and the settled hub snapshots.'),
    ch('🔬', 'probe-documents', 'probe_documents', [(313, 405)],
       'Probe documents of the headless workspace: ensuring the probe artifact and its apply, undo and redo mutations.'),
    ch('🔌', 'plugin-activation-and-bindings', 'plugin_activation_and_bindings', [(407, 608)],
       'Plugin activation and bindings of the headless workspace: activation attempts, artifact channel opening, shell route and action adapter binding, cancellable activation jobs and installed artifact bindings.'),
    ch('🌐', 'hub-session-documents', 'hub_session_documents', [(610, 850)],
       'Hub session documents of the headless workspace: binding a hub document, closing hub links, the live wait, the codec registration, the document actor and the document watch.'),
    ch('🧩', 'artifact-creation-and-routing', 'artifact_creation_and_routing', [(852, 963)],
       'Artifact creation and routing of the headless workspace: plugin artifact creation, media export, capability resolution, the routing channel and the action adapter.'),
    ch('🏛', 'gateway-backend-impl', 'gateway_backend_impl', [(965, 1168)],
       'Gateway backend implementation of the headless workspace: context, search, describe, prepare, invoke, resource read and resource listing.'),
    ch('🪪', 'arc-delegate', 'arc_delegate', [(1170, 1204)],
       'Arc delegate of the gateway backend: the delegating implementation for a shared workspace.'),
]

SPEC = dict(
    orig='gwb.rs',
    file=GWB + '/🦀️.rs',
    children=CH,
    pre=split_impls(CUTS),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[],
)
