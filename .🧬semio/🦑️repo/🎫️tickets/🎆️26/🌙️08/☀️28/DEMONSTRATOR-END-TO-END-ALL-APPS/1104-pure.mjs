import{readFileSync,writeFileSync}from"node:fs";
const root="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/",subset="/🏅️standards/🔖️1/🪆️subsets/✳️any/";
function edit(path,changes){let source=readFileSync(path,"utf8");for(const[from,to]of changes){if(!source.includes(from))throw Error("Missing exact source in "+path+": "+from);source=source.replace(from,to)}writeFileSync(path,source)}
for(const variant of["🌀️generation2d","🧊️generation3d"]){const dim=variant.includes("2d")?"2d":"3d";edit(root+variant+subset+"✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs",[
["use semio_s_artifact_procedural_"+dim+"_model::with_host;\n",""],
["use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};","use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession, FlowHost};"],
['    let (nodes, edges, operators) = with_host(host_snapshot, |host| {\n        let (nodes, edges) = dag_host_snapshot_to_workflow(&host.dag.graph.host_snapshot);\n        let operators = document_operator_records(&host.dag.graph.host_snapshot, &nodes);\n        (nodes, edges, operators)\n    }).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("flow.retained", error.into_message()))?;',
'    let dag = FlowHost::project_dag_snapshot(host_snapshot, &semio_framework_os_flow::flow_neuron_kind_info_map());\n    let (nodes, edges) = dag_host_snapshot_to_workflow(&dag);\n    let operators = document_operator_records(&dag, &nodes);']
])}
edit(root+"🧊️generation3d"+subset+"✏️editor/📌️panels/🗿️artifact/🦀️.rs",[
["use semio_s_artifact_procedural_3d_model::with_host;\n",""],
["use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};","use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession, FlowHost};"],
['    let (nodes, edges) = with_host(host_snapshot, |host| dag_host_snapshot_to_workflow(&host.dag.graph.host_snapshot)).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("flow.retained", error.into_message()))?;',
'    let dag = FlowHost::project_dag_snapshot(host_snapshot, &semio_framework_os_flow::flow_neuron_kind_info_map());\n    let (nodes, edges) = dag_host_snapshot_to_workflow(&dag);']
]);
edit(root+"🧊️generation3d"+subset+"👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs",[
["    let Ok((graph_nodes, _)) = semio_s_artifact_procedural_3d_model::with_host(host_snapshot, |host| semio_s_artifact_procedural_3d_preview_eval::dag_host_snapshot_to_workflow(&host.dag.graph.host_snapshot)) else { return std::collections::BTreeMap::new() };",
"    let dag = semio_framework_os_flow::FlowHost::project_dag_snapshot(host_snapshot, &semio_framework_os_flow::flow_neuron_kind_info_map());\n    let (graph_nodes, _) = semio_s_artifact_procedural_3d_preview_eval::dag_host_snapshot_to_workflow(&dag);"]
]);
edit(root+"🧊️generation3d"+subset+"✏️editor/🪟️artifact-editor-impl/🦀️.rs",[
['generation3d_port_ids_by_node(&doc.snapshot.host_snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained"), error.into_message()))?',
"generation3d_port_ids_by_node(&doc.snapshot.host_snapshot)"],
["generation3d_port_ids_by_node(&doc.snapshot.host_snapshot)?;","generation3d_port_ids_by_node(&doc.snapshot.host_snapshot);"]
]);
edit(root+"🧊️generation3d"+subset+"✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs",[
["/// 🧾️ The law fixture's document tree, projected through the same retiring projection the flow","/// 🧾️ The law fixture's document tree, projected through the same borrowed projection the flow"],
["    let (nodes, edges) = with_host(&fixture, |host| dag_host_snapshot_to_workflow(&host.dag.graph.host_snapshot)).unwrap();",
"    let dag = FlowHost::project_dag_snapshot(&fixture, &semio_framework_os_flow::flow_neuron_kind_info_map());\n    let (nodes, edges) = dag_host_snapshot_to_workflow(&dag);"]
]);
console.log("Updated five procedural read surfaces and the existing exact document-row law.");
