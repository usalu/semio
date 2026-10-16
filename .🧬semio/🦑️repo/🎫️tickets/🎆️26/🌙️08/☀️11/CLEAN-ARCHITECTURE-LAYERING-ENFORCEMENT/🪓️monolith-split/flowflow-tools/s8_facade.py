"""🚪 Step 8: the thin facade root and manifest. Work tree only."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
MODEL="semio_framework_artifact_flow_flow_model"
DOC="semio_framework_artifact_flow_flow_document"
COPY="semio_framework_artifact_flow_flow_copy"
model_fns="cluster_io_layout, cluster_tree_from_property, cluster_tree_property, dag_preview_content_from_dict, default_neuron_input_ports, default_neuron_output_ports, default_neuron_params_from_info, descriptor_explicit_id, generated_widget_id, neuron_io_layout, parse_port_endpoint, preview_content_summary, sensible_slider_range, set_widget_slider_value, variable_io_ports, widget_display_meta, widget_from_descriptor, widget_from_descriptor_with_info, widget_io_ports, widget_label, widget_to_dag_node"
model_types="CameraJson, FlowArtifact, FlowChannelRef, FlowGui, FlowHostComponents, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, FlowUi, NodeChrome, SynapseSpec, Widget, WidgetDescriptor, WidgetLayout, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP"
doc_artifact="FlowGraphStep, FlowHostSnapshot, keyboard_order, keyboard_step, tree_from_host_snapshot"
doc_vcs="AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, empty_flow_snapshot, FlowCollectionDelta, FlowDelta, FlowDiff, FlowEnvelope, FlowMutation, FlowSnapshotRetirementFactory, FlowStore, FLOW_DOCUMENT_SCHEMA, FLOW_HISTORY_STORE_FACTORY, MoveSynapse, MoveWidget, original_flow_history_envelope, original_flow_history_store, prepared_operation_wire_source, RemoveSynapse, RemoveWidget, retire_flow_mutation, retire_flow_store_cold"
copy_names="FlowCopyAllocations, FlowHostSnapshotCopy, FlowSnapshotCopyIntakeSource, FlowSnapshotIntakeProducer, FlowSynapseCopy, FlowWidgetCopy"
root=f'''//! 🌊️ Independently composable Flow document, mutation, and ownership contracts: the facade over the model, document and copy producers.

pub use {MODEL}::{{neural, os_dsl, os_pack, os_spr, os_store, widget_id_for, OrderedMap, OrderedSet}};

pub use {MODEL}::{{
    {model_fns},
    {model_types},
}};

pub use {DOC}::{{
    {doc_artifact},
    {doc_vcs},
}};

pub mod artifact {{
    pub use super::{{
        {model_fns},
        {model_types},
        {doc_artifact},
    }};
}}

pub mod graph_parameter {{
    pub use {MODEL}::graph_parameter::{{SetGraphParameter, SetGraphParameterRetirement}};
}}

pub mod retained {{
    pub use {MODEL}::retained::{{FlowOwner, FlowRetirement}};
    pub use {COPY} as copy;
    pub use {COPY}::{{{copy_names}}};
}}

pub mod vcs {{
    pub use super::{{
        {doc_vcs},
        FlowLayoutEntry,
    }};
}}

pub mod io {{
    pub mod binary {{
        pub mod snapshot {{
            pub use {DOC}::io::binary::snapshot::FlowSnapshotPackProducer;
        }}
    }}
}}
'''
write("🦀️.rs",root)
cargo='''[package]
workspace = "../../../../../../../../.."
name = "semio-framework-artifact-flow-flow"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Composable Flow document, mutation, and ownership contracts: the facade over the model, document and copy producers"

[package.metadata.semio]
role = "artifact"
id = "flow-flow"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[features]
protocol-laws = []

[dependencies]
semio-framework-artifact-flow-flow-model = { path = "../../🧱️model/📦️packages/🦀️rust" }
semio-framework-artifact-flow-flow-document = { path = "../../🧾️document/📦️packages/🦀️rust" }
semio-framework-artifact-flow-flow-copy = { path = "../../🧵️retained/📑️copy/📦️packages/🦀️rust" }
'''
write("📦️packages/🦀️rust/Cargo.toml",cargo)
print("ok")
