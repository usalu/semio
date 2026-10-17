from specs.lib import *

CH = [
    ch('🤝', 'media-contract', 'media_contract', [(20, 41)],
       'Media contract of the workflow graph: the connect-time negotiated wire contract and its placeholder.'),
    ch('🔢', 'media-enum-ordinals', 'media_enum_ordinals', [(45, 154), (433, 478)],
       'Media enum ordinals of the workflow graph: stable ordinals, inverse lookups and variant tables for media class, form, port direction and port multiplicity.'),
    ch('🫳', 'borrowed-record-specs', 'borrowed_record_specs', [(158, 231)],
       'Borrowed record specs of the workflow graph: the borrowed carrier macro and its three invocations, the label tables and the controlled native field helpers.'),
    ch('📐', 'media-contract-record', 'media_contract_record', [(233, 418)],
       'Media contract record codec of the workflow graph: the controlled spec, record projection, record admission and the field trait impls of the media contract.'),
    ch('🔌', 'media-port', 'media_port', [(422, 432), (480, 591)],
       'Media port of the workflow graph: the media port record, its spec, record projection, record admission and the field trait impl.'),
    ch('🧱', 'graph-model', 'graph_model', [(594, 691)],
       'Graph model of the workflow graph: positions, nodes, edges and the workflow itself with its constructors.'),
    ch('✅', 'validation-and-planning', 'validation_and_planning', [(694, 807)],
       'Validation and planning of the workflow graph: the structural validator, the delivery record, the topological order and the delivery plan.'),
    ch('🎛', 'parameters', 'parameters', [(813, 1095)],
       'Parameters of the workflow graph: parameter ids, types, bindings and patches, port synchronisation and config binding validation.'),
    ch('📥', 'inputs', 'inputs', [(1098, 1186)],
       'Inputs of the workflow graph: the declared collection-level input slot with its spec and field trait impl.'),
]

SPEC = dict(
    orig='gr.rs',
    file=GR + '/🦀️.rs',
    children=CH,
    external=mounted(GR + '/🦀️.rs', orig='gr.rs'),
)
