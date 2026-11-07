//! 🧪️ Every donated Flow tool belongs to exactly one declared job factory.

#[test]
fn evaluation_factory_owns_its_exact_declared_routes_once(){
 use crate::editor::flow::{FlowEvaluationJobFactory,FlowHostEffectJobFactory,FlowGraphOperationJobFactory,FlowDirectStoreJobFactory,FlowChildGroupJobFactory};use semio_plugin::ArtifactOwnedToolJobFactory;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🔣️donation.json")).unwrap();let factories=[("FlowEvaluationJobFactory",FlowEvaluationJobFactory::TOOL_IDS),("FlowHostEffectJobFactory",FlowHostEffectJobFactory::TOOL_IDS),("FlowGraphOperationJobFactory",FlowGraphOperationJobFactory::TOOL_IDS),("FlowDirectStoreJobFactory",FlowDirectStoreJobFactory::TOOL_IDS),("FlowChildGroupJobFactory",FlowChildGroupJobFactory::TOOL_IDS)];
 for row in fixture["cases"].as_array().unwrap(){let tool=row["tool"].as_str().unwrap();let mut owner=None;let mut count=0;for(name,ids)in factories{if ids.contains(&tool){count+=1;owner=Some(name)}}assert_eq!(count,1);assert_eq!(owner,row["factory"].as_str());}
 for tool in FlowEvaluationJobFactory::TOOL_IDS{assert_eq!(FlowEvaluationJobFactory::PUBLICATION_CONTRACTS.iter().filter(|contract|contract.tool_id==*tool&&contract.lanes==&[semio_plugin::ArtifactToolPublicationLane::HostOnly]).count(),1);assert!(!FlowHostEffectJobFactory::TOOL_IDS.contains(tool));}
}
