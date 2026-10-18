from specs.lib import *
F = FLOW + '/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs'
CH = [
    ch('🧱', 'document-model', 'document_model', [(22, 292)],
       'Document model of the flow snapshot: the serde defaults, the layout, artifact, GUI, channel, camera, synapse and widget records and the host components.'),
    ch('🪪', 'collection-identity-and-widget-chrome', 'collection_identity_and_widget_chrome', [(295, 377)],
       'Collection identity and widget chrome of the flow snapshot: the identified and patchable widget and synapse impls, the widget chrome and the default flow artifact.'),
    ch('🏷', 'widget-labels-and-io-ports', 'widget_labels_and_io_ports', [(379, 635)],
       'Widget labels and IO ports of the flow snapshot: the widget label and display meta, the variable and cluster layouts and the default neuron and widget IO ports.'),
    ch('🗺', 'dag-node-conversion', 'dag_node_conversion', [(637, 802)],
       'DAG node conversion of the flow snapshot: the property bag conversions, the widget properties and operator kind, the widget to DAG node conversion and the port endpoint parser.'),
    ch('🎚', 'slider-parameters-and-preview-content', 'slider_parameters_and_preview_content', [(804, 984)],
       'Slider parameters and preview content of the flow snapshot: the slider range constants and rounding, the slider value setter and the DAG preview content summaries.'),
    ch('🧩', 'widget-descriptors', 'widget_descriptors', [(986, 1136)],
       'Widget descriptors of the flow snapshot: the widget descriptor, its explicit and generated ids and the widget construction from a descriptor.'),
]
SPEC = dict(orig='snap.rs', nest=True, file=F, children=CH, wiring='block', external=mounted(F, orig='snap.rs'))
