from specs.lib import *

CH = [
    ch('🔁', 'host-lifecycle', 'host_lifecycle', [(21, 316)],
       'Host lifecycle of the plugin host: hot-swap events, loaded programs, the plugin host with its registry, quarantine and action kernel, the hot-swap rollback and the manifest validator.'),
    ch('🗃', 'backbone-document', 'backbone_document', [(319, 603)],
       'Backbone document of the plugin host: the backbone document and its value impls, the space and workflow document aliases, the envelope helpers and the pack, text and payload codecs.'),
    ch('🩹', 'workflow-reconcile', 'workflow_reconcile', [(607, 809)],
       'Workflow reconcile of the plugin host: the graph reconcile that drops orphan edges, type mismatches, duplicate wires, cycles and unbound parameter bindings.'),
    ch('🏪', 'workflow-store', 'workflow_store', [(813, 935)],
       'Workflow store of the plugin host: the workflow store over the backbone with its authoring, undo, redo and catalog wire.'),
    ch('🔗', 'backbone-ports', 'backbone_ports', [(939, 1011)],
       'Backbone ports of the plugin host: the backbone port trait, its implementation, the port union and the document sync.'),
    ch('📚', 'space-catalog', 'space_catalog', [(1025, 1209)],
       'Space catalog of the plugin host: the space catalog entries and their backbone URI tracking, creation, deletion, import, export, loading and seeding.'),
    ch('🧩', 'extension-install', 'extension_install', [(1213, 1327)],
       'Extension install of the plugin host: installed extensions, the install error with its conversions and the extension install and query methods of the plugin host.',
       cfg_mod='#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]'),
]

SPEC = dict(
    orig='ph.rs',
    file=PH + '/🦀️.rs',
    children=CH,
    external=[T + '/orig/hu.rs'],
)
