from specs.lib import *
DC = PL + '/📜️declaration'
CH = [
    ch('📋', 'declaration-and-builder-states', 'declaration_and_builder_states', [(53, 203)],
       'Declaration and builder states: the artifact declaration, its runtime capability requirements and the declaration builder with its schema and ready states.'),
    ch('🏗', 'ready-declaration-builder', 'ready_declaration_builder', [(205, 488)],
       'Ready declaration builder: the operations of a declaration builder that already has its schema.'),
    ch('🧮', 'codec-spec-and-capability-sources', 'codec_spec_and_capability_sources', [(490, 759)],
       'Codec spec and capability sources: the document codec spec, the capability sources and the declaration operations over them.'),
    ch('🎬', 'host-media-handlers', 'host_media_handlers', [(761, 947), (1094, 1179)],
       'Host media handlers: the mesh import and two dimensional svg export handler types, their descriptors, declarations, requests and results, the runtime error and the handler registry.'),
    ch('🌊', 'flow-extensions', 'flow_extensions', [(949, 1092), (1181, 1218)],
       'Flow extensions: the extension executable identity, contributions, manifest, descriptor and declaration and the flow extension registry.'),
]
SPEC = dict(orig='dc.rs', file=DC + '/🦀️.rs', children=CH, wiring='block', external=mounted(DC + '/🦀️.rs', orig='dc.rs'))
