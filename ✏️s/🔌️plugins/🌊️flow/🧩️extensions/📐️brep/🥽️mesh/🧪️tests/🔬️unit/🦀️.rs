//! 🧪️ The mesh contract is shared with the independent Three.js oracle.
use super::*;
use semio_framework_value::serde_json;

fn drain_cancelled_mesh_evaluation(registry:&TestGeometryRegistry,request:&str,operator:&str,node:u64) {
    if flow_extension_sdk::evaluation_progress(&registry,operator,node).is_some() {let request=super::super::tests::compact_evaluation_request(request);let answer=semio_framework_pack_json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(registry,request.as_bytes()).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(answer.get("outputJson").and_then(|value|value.as_str()),Some(""));}
    if flow_extension_sdk::evaluation_retirement_pending(&registry) {assert!(matches!(flow_extension_sdk::retire_cancelled_evaluations_close_step(&registry,0,8),neural_engine::ValueRetirementStep::Blocked));assert!(matches!(flow_extension_sdk::retire_cancelled_evaluations_close_step(&registry,1,0),neural_engine::ValueRetirementStep::Blocked));}
    let mut turns=0;while flow_extension_sdk::evaluation_retirement_pending(&registry) {let remaining=flow_extension_sdk::retire_cancelled_evaluations_close_step(&registry,1,8);assert!(matches!(remaining,neural_engine::ValueRetirementStep::Pending {released_items:0..=1,released_bytes:0..=8}|neural_engine::ValueRetirementStep::Complete));turns+=1;assert!(turns<100000);}
    assert!(flow_extension_sdk::evaluation_progress(&registry,operator,node).is_none());eprintln!("[DEBUG] Same graph evaluation owner drained cancelled mesh, operator={operator}, boundedCloseTurns={turns}");
}

#[test]
fn mesh_output_fault_retires_owned_payload_before_terminal_error() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let law=&fixture["faultRetirement"];
    let payload=" ".repeat(law["payloadBytes"].as_u64().unwrap()as usize);let text=serde_json::to_string(&payload).unwrap();assert_eq!(serde_json::from_str::<String>(&text).unwrap(),payload);
    for cancel in [false,true] {
        let mut job=MeshOperatorJob::json(HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap()).unwrap();let Some(MeshJobOutput::Mesh(state))=&mut job.output else {unreachable!()};state.data=payload.clone();
        assert!(matches!(job.step(1),Ok(OperatorJobStep::Working(_))),"fault must retain its source payload");assert_eq!(job.progress().phase,law["phase"].as_str().unwrap());
        let initial=job.progress();assert!(matches!(job.close_step(1,0).unwrap(),OperatorJobStep::Working(_)));assert_eq!(job.progress().units_done,initial.units_done);
        if cancel {job.cancel();}
        let mut turns=0;
        loop {turns+=1;assert!(turns<10000);match job.close_step(1,law["grantBytes"].as_u64().unwrap()as usize) {
            Ok(OperatorJobStep::Working(_))=>{},
            Ok(OperatorJobStep::Cancelled(_)) if cancel=>break,
            Err(error) if !cancel=>{assert!(error.to_string().contains(law["error"].as_str().unwrap()));break;},
            _=>panic!("faulting payload published output or wrong terminal"),
        }}
        assert!(turns>law["payloadBytes"].as_u64().unwrap()as usize/law["grantBytes"].as_u64().unwrap()as usize);assert!(job.output.is_none() && job.retirement.is_none() && job.pending_output.is_none());
        eprintln!("[DEBUG] Owned output fault retired payload before terminal, cancel={cancel}, turns={turns}");
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_reconstruction_fault_retires_source_and_input_before_terminal_error() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let law=&fixture["reconstructionFault"];let registry=TestGeometryRegistry::new();let text=law["mesh"].to_string();let parsed:serde_json::Value=serde_json::from_str(&text).unwrap();assert_eq!(parsed,law["mesh"]);
    let input=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(text))))).insert("offset",Value::Dictionary(vector_dictionary([1.0,2.0,3.0]))));
    for cancel in [false,true] {
        let mut job=MeshOperatorJob::preparation("translate",registry.session.clone(),&input).unwrap();let mut source=parse_polygon_mesh_source(&law["mesh"].to_string()).unwrap();source.faces=serde_json::from_value(law["ownedFaces"].clone()).unwrap();let state=job.preparation.as_mut().unwrap();let previous=state.source.take().unwrap();state.reconstruction=Some(HalfedgeMesh::polygon_source_job(source).unwrap());let mut retirement=semio_framework_value::retirement::owned_retirement(previous);while !retirement.terminal_is_empty() {retirement.close_step(1,8).unwrap();}let mut turns=0;
        while job.progress().phase!=law["phase"].as_str().unwrap() {turns+=1;assert!(turns<10000);let step=job.step(1);if let Ok(OperatorJobStep::Done(value))=step {drop(neural_engine::ColdOwner::new(value));panic!("source fault published output");}assert!(matches!(step,Ok(OperatorJobStep::Working(_))),"source fault must retain reconstruction ownership");}
        let before=job.progress();assert!(matches!(job.close_step(1,0).unwrap(),OperatorJobStep::Working(_)));assert_eq!(job.progress().units_done,before.units_done);
        if cancel {job.cancel();}
        let mut close_turns=0;loop {close_turns+=1;assert!(close_turns<10000);match job.close_step(1,law["grantBytes"].as_u64().unwrap()as usize) {Ok(OperatorJobStep::Working(_))=>{},Err(_) if !cancel=>break,Ok(OperatorJobStep::Cancelled(_)) if cancel=>break,_=>panic!("source fault published or reached wrong terminal")}}
        assert!(close_turns>1);eprintln!("[DEBUG] Same reconstruction owner retired invalid topology, cancel={cancel}, closeTurns={close_turns}");
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_import_admission_fault_retains_all_typed_transfer_bytes() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let law=&fixture["importAdmissionFault"];let mesh=HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();let transfer=mesh.tessellate().unwrap();let words=transfer.positions.len()+transfer.normals.len()+transfer.colors.len()+transfer.indices.len()+transfer.face_ids.len()+transfer.vertex_ids.len()+transfer.edge_positions.len()+transfer.edge_ids.len()+transfer.uvs.len()+transfer.edge_uvs.len();let expected=words*4+transfer.edge_is_seam.len();assert_eq!(transfer.positions.len()%law["positionStride"].as_u64().unwrap()as usize,0);assert_eq!(transfer.indices.len()%law["triangleStride"].as_u64().unwrap()as usize,0);
    let registry=TestGeometryRegistry::new();let mut job=MeshOperatorJob::to_brep(registry.session.clone(),mesh,law["tolerance"].as_f64().unwrap()).unwrap();let mut turns=0;
    while job.import.as_ref().unwrap().fault.is_none() {turns+=1;assert!(turns<10000);assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_)));}
    let mut retirement=job.import.as_mut().unwrap().transfer_retirement.take().unwrap();let mut bytes=0;while !retirement.terminal_is_empty() {match retirement.close_step(1,law["grantBytes"].as_u64().unwrap()as usize).unwrap() {semio_framework_value::SnapshotRetirementStep::Pending {released_bytes,..}=>{assert!(released_bytes<=8);bytes+=released_bytes;},_=>{}}}
    loop {match job.step(1) {Ok(OperatorJobStep::Working(_))=>{},Err(_)=>break,_=>panic!("invalid import admitted or published")}}
    assert_eq!(bytes,expected,"refused triangle buffers must remain in the existing owned transfer retirement");eprintln!("[DEBUG] Existing mesh-I/O admission refusal retained all transfer payload bytes={bytes}");
}

#[semio_framework_async_macros::async_test]
async fn mesh_source_preparation_retains_json_and_typed_projection_under_work_grants() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let law=&fixture["sourceParsing"];let label=law["textUnit"].as_str().unwrap().repeat(law["textRepeats"].as_u64().unwrap()as usize);let mut source=law["mesh"].clone();source["materials"]=serde_json::json!({"paint":{"label":label}});let text=source.to_string();assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),source);let registry=TestGeometryRegistry::new();let input=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(text))))).insert("offset",Value::Dictionary(vector_dictionary([1.0,2.0,3.0]))));
    let mut job=registry.dispatch_job("brep.mesh.translate",&input).unwrap().unwrap();assert_eq!(job.progress().phase,law["initialPhase"].as_str().unwrap(),"whole JSON must remain inside retained preparation");let mut phases=std::collections::BTreeSet::new();let mut turns=0;let output=loop {phases.insert(job.progress().phase);let before=job.progress();assert!(matches!(job.close_step(1,0).unwrap(),OperatorJobStep::Working(_)));assert_eq!(job.progress().units_done,before.units_done);turns+=1;assert!(turns<100000);match job.step(1).unwrap() {OperatorJobStep::Working(progress)=>assert_eq!(progress.units_done,before.units_done+1),OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.translate",value).unwrap()),_=>panic!("valid parser preparation cancelled")}};
    for phase in law["phases"].as_array().unwrap() {assert!(phases.contains(phase.as_str().unwrap()));let mut cancelled=registry.dispatch_job("brep.mesh.translate",&input).unwrap().unwrap();while cancelled.progress().phase!=phase.as_str().unwrap() {assert!(matches!(cancelled.step(1).unwrap(),OperatorJobStep::Working(_)));}cancelled.step(1).unwrap();cancelled.cancel();drain_cancelled_mesh_job(&mut *cancelled);}
    let data=output.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap();let parsed:serde_json::Value=serde_json::from_str(data).unwrap();assert_eq!(parsed["materials"],source["materials"]);let positions:Vec<[f32;3]>=serde_json::from_value(parsed["vertices"].clone()).unwrap();assert_eq!(positions,vec![[1.,2.,3.],[2.,2.,3.],[1.,3.,3.]]);eprintln!("[DEBUG] Existing source owner retained JSON and typed projection, phases={phases:?}, workTurns={turns}");
}

#[semio_framework_async_macros::async_test]
async fn mesh_affine_operator_owns_matrix_contract_and_retained_output() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();let fixture=&fixture["affineJobs"];let registry=TestGeometryRegistry::new();let info=registry.operator_info("brep.mesh.transform").unwrap();let channel=info.inputs.iter().find(|channel|channel.name=="matrix").unwrap();assert_eq!(channel.value_types,vec!["list"]);assert_eq!(channel.item_types,vec!["number"]);assert_eq!(channel.cardinality,neural_engine::Cardinality::Exactly(16));let mesh=serde_json::json!({"vertices":fixture["positions"],"faces":fixture["faces"]}).to_string();
    let input=|matrix:&serde_json::Value|{let list=matrix.as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"),|list,(index,value)|list.insert(index.to_string(),Value::Dictionary(number_dictionary(value.as_f64().unwrap_or(f64::NAN)))));neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(mesh.clone()))))).insert("matrix",Value::Dictionary(list)))};
    for case in fixture["cases"].as_array().unwrap() {let input=input(&case["matrix"]);let mut job=registry.dispatch_job("brep.mesh.transform",&input).unwrap().unwrap();assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let mut phases=std::collections::BTreeSet::new();let output=loop {phases.insert(job.progress().phase);let before=job.progress().units_done;match job.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.transform",value).unwrap()),OperatorJobStep::Working(progress)=>assert_eq!(progress.units_done,before+1),_=>unreachable!()}};let data=output.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap();let polygon:serde_json::Value=serde_json::from_str(data).unwrap();let actual:Vec<[f64;3]>=serde_json::from_value(polygon["vertices"].clone()).unwrap();let expected:Vec<[f64;3]>=serde_json::from_value(case["positions"].clone()).unwrap();assert_eq!(actual.len(),expected.len());for (a,b) in actual.into_iter().zip(expected) {for axis in 0..3 {assert!((a[axis]-b[axis]).abs()<1e-6);}}assert!(phases.contains("transform-vertices"));assert!(phases.contains("mesh-output-pack"));for phase in phases {let mut cancelled=registry.dispatch_job("brep.mesh.transform",&input).unwrap().unwrap();while cancelled.progress().phase!=phase {assert!(matches!(cancelled.step(1).unwrap(),OperatorJobStep::Working(_)));}cancelled.cancel();drain_cancelled_mesh_job(&mut *cancelled);}eprintln!("[DEBUG] Named affine matrix retained through output: case={} units={}",case["name"],job.progress().units_done);}
    let shared:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();let coplanar=&shared["coplanarJob"];let source=serde_json::json!({"vertices":coplanar["positions"],"faces":coplanar["faces"]}).to_string();let request=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(source))))));let mut job=registry.dispatch_job("brep.mesh.mergeCoplanar",&request).unwrap().unwrap();let output=loop {match job.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.mergeCoplanar",value).unwrap()),OperatorJobStep::Working(_)=>{},_=>unreachable!()}};let mesh=read_mesh(&output,"meshOut").unwrap();assert_eq!(mesh.vertex_count(),coplanar["vertices"].as_u64().unwrap() as usize);assert_eq!(mesh.face_count(),coplanar["outputFaces"].as_u64().unwrap() as usize);assert_eq!(mesh.halfedge_count(),coplanar["corners"].as_u64().unwrap() as usize);eprintln!("[DEBUG] Named coplanar merge retained through complete JSON/pack/base64: units={} corners={}",job.progress().units_done,mesh.halfedge_count());
    let attributed=&shared["coplanarAttributeJob"];let assets:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🎨️attributes/🔣️.json")).unwrap();let faces=coplanar["faces"].as_array().unwrap();let corners=faces.iter().flat_map(|face|face.as_array().unwrap()).map(|id|{let point=&coplanar["positions"][id.as_u64().unwrap()as usize];serde_json::json!([point[0],point[1]])}).collect::<Vec<_>>();let source=serde_json::json!({"vertices":coplanar["positions"],"faces":coplanar["faces"],"attributes":{"uv":{"domain":"corner","semantic":"uv","interpolation":"linear","values":corners},"linear":{"domain":"face","semantic":"custom","interpolation":"linear","values":attributed["faceValues"]},"nearest":{"domain":"face","semantic":"custom","interpolation":"nearest","values":[0,1,2]}},"materials":assets["indexedMesh"]["materials"],"textures":assets["indexedMesh"]["textures"]});let request=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(source.to_string()))))));let mut job=registry.dispatch_job("brep.mesh.mergeCoplanar",&request).unwrap().unwrap();let output=loop {match job.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.mergeCoplanar",value).unwrap()),OperatorJobStep::Working(_)=>{},_=>unreachable!()}};let data=output.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap();let polygon:serde_json::Value=serde_json::from_str(data).unwrap();assert_eq!(polygon["faces"].as_array().unwrap().len(),1);assert_eq!(polygon["faces"][0].as_array().unwrap().len(),attributed["corners"].as_u64().unwrap()as usize);assert_eq!(polygon["materials"],source["materials"]);assert_eq!(polygon["textures"],source["textures"]);let value_at=|name:&str|{let channel=&polygon["attributes"][name];let index=channel.get("indices").map_or(0,|indices|indices[0].as_u64().unwrap()as usize);channel["values"][index].clone()};assert_eq!(value_at("linear").as_f64(),attributed["faceLinear"].as_f64());assert_eq!(value_at("nearest"),attributed["faceNearest"]);assert!(output.get("meshOut").unwrap().as_dictionary().unwrap().get("preview").unwrap().as_atom().unwrap().as_str().is_some());eprintln!("[DEBUG] Named coplanar provenance retained through JSON/pack/base64: units={} Corner UV, Face interpolation, materials/textures",job.progress().units_done);
    for case in fixture["invalid"].as_array().unwrap() {assert!(registry.dispatch_job("brep.mesh.transform",&input(&case["matrix"])).is_err());}let mut short=fixture["cases"][0]["matrix"].clone();short.as_array_mut().unwrap().pop();assert!(registry.dispatch_job("brep.mesh.transform",&input(&short)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn mesh_json_export_retains_polygon_metadata_without_preview_work() {
    use neural_engine::OperatorJobStep;
    let budget=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let fixture=semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🎨️attributes/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let source=fixture.get("indexedMesh").unwrap().to_string();let registry=TestGeometryRegistry::new();let input=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(source.clone()))))));
    for phase in budget.get("jsonExportPhases").unwrap().as_array().unwrap() {let mut job=registry.dispatch_job("brep.mesh.exportJson",&input).unwrap().unwrap();assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let mut units=0;while job.progress().phase!=phase.as_str().unwrap() {let before=job.progress().units_done;assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_)));assert_eq!(job.progress().units_done,before+1);units+=1;assert!(units<100000);}job.cancel();let before=job.progress().units_done;drain_cancelled_mesh_job(&mut *job);assert_eq!(job.progress().units_done,before);}
    let mut job=registry.dispatch_job("brep.mesh.exportJson",&input).unwrap().unwrap();let mut phases=std::collections::BTreeSet::new();let output=loop {phases.insert(job.progress().phase);match job.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.exportJson",value).unwrap()),OperatorJobStep::Working(_)=>{},_=>panic!("unexpected JSON cancellation")}};let text=read_text(&output,budget.get("jsonExportChannel").unwrap().as_str().unwrap()).unwrap();let mut actual:serde_json::Value=serde_json::from_str(&text).unwrap();let mut expected:serde_json::Value=serde_json::from_str(&source).unwrap();let positions=|value:&mut serde_json::Value|serde_json::from_value::<Vec<[f32;3]>>(value.as_object_mut().unwrap().remove("vertices").unwrap()).unwrap();assert_eq!(positions(&mut actual),positions(&mut expected));assert_eq!(actual,expected);assert!(!phases.contains("mesh-output-tessellate"));assert!(!phases.contains("mesh-output-pack"));assert!(!phases.contains("mesh-output-base64"));eprintln!("[DEBUG] Retained JSON-only export completedUnits={} bytes={} authored metadata preserved",job.progress().units_done,text.len());
    let case=budget.get("objExport").unwrap();let source=case.get("mesh").unwrap().to_string();let request=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(source))))));let mut job=registry.dispatch_job("brep.mesh.exportObj",&request).unwrap().unwrap();let mut phases=std::collections::BTreeMap::new();assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let output=loop {let before=job.progress();match job.step(1).unwrap() {OperatorJobStep::Working(_)=>{phases.entry(before.phase).or_insert(before.units_done);},OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.exportObj",value).unwrap()),_=>unreachable!()}};let text=read_text(&output,"text").unwrap();assert_eq!(text,case.get("expected").unwrap().as_str().unwrap());for (phase,_) in &phases {let mut cancelled=registry.dispatch_job("brep.mesh.exportObj",&request).unwrap().unwrap();while cancelled.progress().phase!=*phase {assert!(matches!(cancelled.step(1).unwrap(),OperatorJobStep::Working(_)));}let before=cancelled.progress();cancelled.cancel();drain_cancelled_mesh_job(&mut *cancelled);assert_eq!(cancelled.progress().units_done,before.units_done);}eprintln!("[DEBUG] Retained OBJ export authored UV,exactneutraltext,allobservedphasecancellation,units={}",job.progress().units_done);

}

#[test]
fn mesh_output_metadata_retains_bounded_values_and_cancellation() {
    use neural_engine::OperatorJob;
    let fixture=semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🎨️attributes/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh=decode_mesh(&fixture.get("indexedMesh").unwrap().to_string()).unwrap();
    let mut job=MeshOperatorJob::output(mesh.clone()).unwrap();
    let mut metadata_steps=0;
    let completed=loop {
        let (phase,before)=match job.output.as_ref() {Some(MeshJobOutput::Mesh(state))=>(state.phase(),state.data.len()),None=>("mesh-output-retire",0),_=>unreachable!()};
        let result=job.step(1).unwrap();
        if phase=="mesh-output-metadata" {
            metadata_steps+=1;
            if let Some(MeshJobOutput::Mesh(state))=&job.output {assert!(state.data.len().saturating_sub(before)<=2048);}
        }
        match result {neural_engine::OperatorJobStep::Done(output)=>break neural_engine::ColdOwner::new(output),neural_engine::OperatorJobStep::Working(_)=>{},_=>panic!("unexpected metadata cancellation")}
        assert!(metadata_steps<100000);
    };
    assert!(metadata_steps>4096);
    let data=completed.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap();
    let mut actual:serde_json::Value=serde_json::from_str(data).unwrap();let mut expected:serde_json::Value=serde_json::from_str(&fixture.get("indexedMesh").unwrap().to_string()).unwrap();let positions=|value:&mut serde_json::Value|serde_json::from_value::<Vec<[f32;3]>>(value.as_object_mut().unwrap().remove("vertices").unwrap()).unwrap();assert_eq!(positions(&mut actual),positions(&mut expected));assert_eq!(actual,expected);
    let mut cancelled=MeshOperatorJob::output(mesh).unwrap();
    while cancelled.progress().phase!="mesh-output-metadata" {assert!(matches!(cancelled.step(1).unwrap(),neural_engine::OperatorJobStep::Working(_)));}
    cancelled.step(1).unwrap();cancelled.cancel();
    drain_cancelled_mesh_job(&mut cancelled);
}

#[semio_framework_async_macros::async_test]
async fn mesh_output_jobs_bound_work_and_cancel_every_output_phase() {
    use neural_engine::OperatorJob;
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let mesh = decode_mesh(&semio_framework_pack_json::to_string(case.get("mesh").unwrap())).unwrap();
        let transfer = mesh.tessellate().unwrap();
        let preview = semio_framework_mesh_engine::MeshData { positions: transfer.positions, normals: transfer.normals, indices: transfer.indices, face_ids: transfer.face_ids, vertex_ids: transfer.vertex_ids, edge_positions: transfer.edge_positions, edge_ids: transfer.edge_ids, uvs: transfer.uvs, edge_uvs: transfer.edge_uvs, edge_is_seam: transfer.edge_is_seam, ..Default::default() };
        let expected = neural_engine::ColdOwner::new(channel_output("meshOut",Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(encode_mesh(&mesh).unwrap()))).insert("preview",Value::Atom(Atom::String(encode_base64(&encode_mesh_pack(&preview).unwrap()))))));
        assert_eq!(preview.indices.len()/3,case.get("triangles").unwrap().as_u64().unwrap() as usize);
        let mut area = 0.0f64;
        for triangle in preview.indices.chunks_exact(3) {
            let points = [triangle[0],triangle[1],triangle[2]].map(|index| MeshVector([0,1,2].map(|axis| preview.positions[index as usize*3+axis])));
            area += points[1].sub(points[0]).cross(points[2].sub(points[0])).length() as f64/2.0;
        }
        assert!((area-case.get("area").unwrap().as_f64().unwrap()).abs()<1e-6);
        let mut output = MeshOperatorJob::output(mesh.clone()).unwrap();
        let initial = output.progress();
        assert!(matches!(output.step(0).unwrap(), neural_engine::OperatorJobStep::Working(_)));
        assert_eq!(output.progress().units_done, initial.units_done);
        let result = loop {
            let before = output.progress().units_done;
            match output.step(1).unwrap() {
                neural_engine::OperatorJobStep::Working(progress) => assert_eq!(progress.units_done, before + 1),
                neural_engine::OperatorJobStep::Done(result) => break neural_engine::ColdOwner::new(result),
                _ => panic!("unexpected cancellation"),
            }
        };
        assert_eq!(semio_framework_pack_json::to_json_string(&*result), semio_framework_pack_json::to_json_string(&*expected));
        eprintln!("[DEBUG] retained mesh output case={} completedUnits={} cancellationPhases={}",case.get("name").unwrap().as_str().unwrap(),output.progress().units_done,fixture.get("phases").unwrap().as_array().unwrap().len());
        for phase in fixture.get("phases").unwrap().as_array().unwrap() {
            let phase = phase.as_str().unwrap();
            let mut output = MeshOperatorJob::output(mesh.clone()).unwrap();
            while output.progress().phase != phase { assert!(matches!(output.step(1).unwrap(), neural_engine::OperatorJobStep::Working(_))); }
            assert!(matches!(output.step(1).unwrap(), neural_engine::OperatorJobStep::Working(_)));
            output.cancel();
            let frozen = output.progress().units_done;
            drain_cancelled_mesh_job(&mut output);for _ in 0..2 {assert!(matches!(output.step(1).unwrap(),neural_engine::OperatorJobStep::Cancelled(_)));assert_eq!(output.progress().units_done,frozen);}
        }
    }
    let segments = fixture.get("large").unwrap().get("segments").unwrap().as_u64().unwrap() as u32;
    let mesh = HalfedgeMesh::cylinder_prim(1.0, 1.0, segments).unwrap();
    let mut output = MeshOperatorJob::output(mesh).unwrap();
    while output.progress().phase != "mesh-output-tessellate" { assert!(matches!(output.step(1).unwrap(), neural_engine::OperatorJobStep::Working(_))); }
    for _ in 0..segments { let before = output.progress().units_done; assert!(matches!(output.step(1).unwrap(), neural_engine::OperatorJobStep::Working(_))); assert_eq!(output.progress().units_done, before + 1); }
    assert_eq!(output.progress().phase, "mesh-output-tessellate");
    output.cancel();
    let mesh = HalfedgeMesh::from_faces(&vec![[0.0,0.0,0.0]; LIMIT+1],&[vec![0,1,2]]).unwrap();
    assert!(MeshOperatorJob::output(mesh).is_err());
    let transfer = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap().tessellate().unwrap();
    let encoder = MeshPackEncodingJob::new(semio_framework_mesh_engine::MeshData { positions: transfer.positions, ..Default::default() },32).err().expect("preview admission must reject oversized buffers");
    assert!(encoder.contains("exceeds"));
    let mut limited = MeshTessellationJob::with_preview_capacity(HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap(),32);
    let mut steps = 0;
    loop { steps += 1; assert!(steps<4096); match limited.step(1) { Err(error)=>{ assert!(error.to_string().contains("preview exceeds")); break; },Ok(MeshTessellationStep::Working(_))=>{},_=>panic!("preview exceeded admission without a fault") } }
}

#[semio_framework_async_macros::async_test]
async fn mesh_existing_synchronous_routes_record_runtime_costs() {
    let source = HalfedgeMesh::ico_sphere_prim(1.0,5).unwrap();
    let start=std::time::Instant::now();let cloned=source.clone();let clone_time=start.elapsed().as_micros();assert_eq!(cloned.vertex_count(),source.vertex_count());let encoded=encode_mesh(&source).unwrap();let start=std::time::Instant::now();let parsed=parse_polygon_mesh_source(&encoded).unwrap();let parse_time=start.elapsed().as_micros();assert_eq!(parsed.faces.len(),source.face_count());let start=std::time::Instant::now();semio_framework_mesh_engine::validate_polygon_mesh_attributes(parsed.vertices.len(),parsed.faces.len(),parsed.faces.iter().map(Vec::len).sum(),&parsed.attributes,&parsed.materials,&parsed.textures).unwrap();let admission_time=start.elapsed().as_micros();let start=std::time::Instant::now();let mut construct=HalfedgeMesh::polygon_source_job(parsed).unwrap();let capture_time=start.elapsed().as_micros();let start=std::time::Instant::now();let constructed=loop {if let MeshModelingStep::Done(mesh)=construct.step(4096).unwrap() {break mesh;}};let build_time=start.elapsed().as_micros();assert_eq!(constructed.vertex_count(),source.vertex_count());assert_eq!(constructed.face_count(),source.face_count());eprintln!("[DEBUG] Mesh source boundary vertices={} faces={} bytes={} cloneMicros={clone_time} parseAndAdmissionMicros={parse_time} metadataReadmissionMicros={admission_time} ownedCaptureMicros={capture_time} retainedBuildTotalMicros={build_time} retainedBuildUnits={}",source.vertex_count(),source.face_count(),encoded.len(),construct.progress().units_done);
    for operation in ["merge-distance","mirror","subdivide","analyze"] {
        let mut mesh = source.clone();
        let vertices = (0..2000).map(VertexId).collect::<Vec<_>>();
        let faces = (0..1024).map(FaceId).collect::<Vec<_>>();
        if operation == "mirror" { mesh.translate(MeshVector([2.0,0.0,0.0])).unwrap(); }
        let start = std::time::Instant::now();
        match operation {
            "merge-distance" => mesh.merge_vertices(&vertices,WeldMode::ByDistance,1e-5).unwrap(),
            "mirror" => mesh.mirror(MirrorAxis::X,1e-5).unwrap(),
            "subdivide" => mesh.subdivide_faces(&faces).unwrap(),
            _ => { let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap()); assert!(read_channel_number(&report,"area").unwrap()>0.0); }
        }
        eprintln!("[DEBUG] synchronous mesh route={operation} vertices={} faces={} elapsedMicros={}",source.vertex_count(),source.face_count(),start.elapsed().as_micros());
        assert!(mesh.vertex_count()>0 && mesh.face_count()>0);
    }
    let mut mesh = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
    let start = std::time::Instant::now(); mesh.loop_cut(&[EdgeId(0)],256).unwrap();
    eprintln!("[DEBUG] synchronous mesh route=loop-cut cuts=256 elapsedMicros={}",start.elapsed().as_micros());
    assert!(mesh.face_count()>6);
    for operation in ["translate","rotate","scale","knife","inset","extrude","orient","fill-holes","from-brep-indexed"] {
        let mut mesh = source.clone();
        let faces = (0..1024).map(FaceId).collect::<Vec<_>>();
        let triangle = mesh.face_vertex_ids(FaceId(0)).unwrap().into_iter().map(|id|mesh.vertex_position(id).unwrap()).collect::<Vec<_>>();
        let transfer = if operation == "from-brep-indexed" { Some(mesh.tessellate().unwrap()) } else { None };
        if operation == "fill-holes" { let data = parse_polygon_mesh_source(&encode_mesh(&mesh).unwrap()).unwrap(); mesh=HalfedgeMesh::from_faces(&data.vertices,&data.faces[1024..]).unwrap(); }
        let start = std::time::Instant::now();
        match operation {
            "translate" => mesh.translate(MeshVector([1.0,2.0,3.0])).unwrap(),
            "rotate" => mesh.rotate(MeshVector([1.0,2.0,3.0]),0.7).unwrap(),
            "scale" => mesh.scale(MeshVector([2.0,3.0,4.0])).unwrap(),
            "knife" => mesh.knife_cut(FaceId(0),triangle[0].lerp(triangle[1],0.5),triangle[0].lerp(triangle[2],0.5)).unwrap(),
            "inset" => mesh.inset_faces(&faces,1e-5).unwrap(),
            "extrude" => mesh.extrude_faces(&faces,0.01).unwrap(),
            "orient" => { mesh.orient_faces_consistently().unwrap(); },
            "fill-holes" => { mesh.fill_holes().unwrap(); },
            _ => { let transfer=transfer.unwrap(); mesh=indexed_triangle_mesh(&transfer.positions,&transfer.indices).unwrap(); },
        }
        eprintln!("[DEBUG] synchronous mesh route={operation} vertices={} faces={} elapsedMicros={}",source.vertex_count(),source.face_count(),start.elapsed().as_micros());
        assert!(mesh.vertex_count()>0 && mesh.face_count()>0);
    }
    let mut mesh=HalfedgeMesh::cylinder_prim(1.0,1.0,1024).unwrap();
    let start=std::time::Instant::now(); mesh.loop_cut(&[EdgeId(2048)],95).unwrap();
    eprintln!("[DEBUG] synchronous mesh route=loop-cut-admitted cuts=95 vertices={} faces={} elapsedMicros={}",mesh.vertex_count(),mesh.face_count(),start.elapsed().as_micros());
    assert!(mesh.vertex_count()<=LIMIT && mesh.face_count()<=LIMIT);
}

#[semio_framework_async_macros::async_test]
async fn mesh_mutation_preparation_retains_source_reconstruction_before_modeling() {
    use neural_engine::OperatorJobStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let case=&fixture["inputPreparation"];let registry=TestGeometryRegistry::new();let source=registry.dispatch_cold("brep.mesh.box",["width","height","depth"].into_iter().fold(Dictionary::new(),|input,name|input.insert(name,Value::Dictionary(number_dictionary(1.0))))).unwrap();let input=neural_engine::ColdOwner::new(Dictionary::new().insert("mesh",source.get("meshOut").unwrap().clone()).insert("offset",Value::Dictionary(vector_dictionary(std::array::from_fn(|axis|case["offset"][axis].as_f64().unwrap())))));let mut job=registry.dispatch_job("brep.mesh.translate",&input).unwrap().unwrap();assert_eq!(job.progress().phase,case["initialPhase"].as_str().unwrap(),"input reconstruction must remain in the same retained job");assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let mut phases=std::collections::BTreeMap::new();let output=loop {let before=job.progress();match job.step(1).unwrap() {OperatorJobStep::Working(_)=>{phases.entry(before.phase).or_insert(before.units_done);},OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.translate",value).unwrap()),_=>unreachable!()}};
    for phase in case["phases"].as_array().unwrap() {assert!(phases.contains_key(phase.as_str().unwrap()),"missing {phase}");}let mesh=read_mesh(&output,"meshOut").unwrap();assert_eq!(mesh.vertex_count(),case["vertices"].as_u64().unwrap() as usize);assert_eq!(mesh.face_count(),case["faces"].as_u64().unwrap() as usize);let report=neural_engine::ColdOwner::new(analyze(&mesh).unwrap());assert!((read_channel_number(&report,"area").unwrap()-case["area"].as_f64().unwrap()).abs()<1e-6);assert!((read_channel_number(&report,"volume").unwrap()-case["volume"].as_f64().unwrap()).abs()<1e-6);
    for (phase,_) in phases {let mut cancelled=registry.dispatch_job("brep.mesh.translate",&input).unwrap().unwrap();let mut preparation_steps=0;while cancelled.progress().phase!=phase {assert!(matches!(cancelled.step(1).unwrap(),OperatorJobStep::Working(_)));preparation_steps+=1;assert!(preparation_steps<100000);}cancelled.cancel();let before=cancelled.progress();assert!(matches!(cancelled.close_step(1,0).unwrap(),OperatorJobStep::Working(progress) if progress.units_done==before.units_done));let mut trips=0;loop {match cancelled.close_step(1,8).unwrap() {OperatorJobStep::Working(_)=>{trips+=1;assert!(trips<100000);},OperatorJobStep::Cancelled(_)=>break,_=>panic!("cancelled source preparation published")}}}
    eprintln!("[DEBUG] Same mesh mutation job retained source reconstruction and translation,all-phase eight-byte cancellation,units={}",job.progress().units_done);
}

#[semio_framework_async_macros::async_test]
async fn mesh_analysis_retains_topology_tessellation_and_measurement() {
    use neural_engine::{OperatorJob,OperatorJobStep};
    let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh=HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
    for phase in fixture.get("analysisPhases").unwrap().as_array().unwrap() {
        let mut job=MeshOperatorJob::analysis(mesh.clone()).unwrap();
        assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));
        let mut trips=0;
        while job.progress().phase!=phase.as_str().unwrap() { let before=job.progress().units_done; assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_))); assert_eq!(job.progress().units_done,before+1); trips+=1; assert!(trips<4096); }
        assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_))); job.cancel(); let frozen=job.progress();
        drain_cancelled_mesh_job(&mut job); assert_eq!(job.progress().units_done,frozen.units_done);
    }
    let mut job=MeshOperatorJob::analysis(mesh).unwrap();
    let output=loop { if let OperatorJobStep::Done(value)=job.step(1).unwrap() { break neural_engine::ColdOwner::new(value); } };
    for key in ["area","volume","edges","triangles"] { assert!((read_channel_number(&output,key).unwrap()-fixture.get("analysis").unwrap().get(key).unwrap().as_f64().unwrap()).abs()<1e-12); }
}

#[semio_framework_async_macros::async_test]
async fn mesh_generation_refuses_expansion_before_reconstruction() {
    let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let limit=fixture.get("limits").unwrap().get("vertices").unwrap().as_u64().unwrap() as usize;
    let source=HalfedgeMesh::from_faces(&vec![[1.0,0.0,0.0];limit],&[vec![0,1,2]]).unwrap();
    for mut job in [source.mirror_job(MirrorAxis::X,0.0).unwrap(),source.subdivide_faces_job(&[FaceId(0)]).unwrap()] {
        loop { match job.step(4096) { Err(error)=>{ assert!(error.to_string().contains("capacity"));assert!(!job.progress().phase.ends_with("reconstruct"));break; },Ok(MeshModelingStep::Working(_))=>{},_=>panic!("expansion crossed its admission limit") } }
    }
    let mut source=HalfedgeMesh::cylinder_prim(1.0,1.0,1024).unwrap();
    let before=(source.vertex_count(),source.face_count(),source.halfedge_count());
    assert!(source.loop_cut(&[EdgeId(2048)],256).err().unwrap().to_string().contains("capacity"));
    assert_eq!((source.vertex_count(),source.face_count(),source.halfedge_count()),before);
}

#[semio_framework_async_macros::async_test]
async fn mesh_expensive_routes_step_through_the_existing_operator_owner() {
    use neural_engine::OperatorJobStep;
    let registry=TestGeometryRegistry::new();let source=encode_mesh(&HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap()).unwrap();
    for operation in ["transform","mergeCoplanar","weld","translate","rotate","scale","loopCut","knifeCut","inset","extrude","orient","fillHoles","translateComponents","rotateComponents","scaleComponents","moveVertices","triangulate","snapVertices","moveProportional"] {
        let id=format!("brep.mesh.{operation}");let input=registry.operator_info(&id).unwrap().inputs.iter().fold(Dictionary::new(),|input,channel|match &channel.default {Some(value)=>input.insert(channel.name.clone(),value.clone()),None=>input});
        let input=neural_engine::ColdOwner::new(input.insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(source.clone()))))));
        let mut job=registry.dispatch_job(&id,&input).unwrap().expect("expensive mesh route requires retained work");assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let before=job.progress().units_done;assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_)));assert_eq!(job.progress().units_done,before+1);
        eprintln!("[DEBUG] retained mesh operator={id} phase={} actualUnits={}",job.progress().phase,job.progress().units_done);job.cancel();let frozen=job.progress().units_done;drain_cancelled_mesh_job(&mut *job);assert_eq!(job.progress().units_done,frozen);
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_from_brep_retains_tessellation_import_normals_and_output() {
    let _serial=super::super::tests::test_serial().await;
    use neural_engine::OperatorJobStep;
    let registry=TestGeometryRegistry::new();let defaults=|id:&str|registry.operator_info(id).unwrap().inputs.iter().fold(Dictionary::new(),|input,channel|match &channel.default {Some(value)=>input.insert(channel.name.clone(),value.clone()),None=>input});
    let solid=registry.dispatch_cold("brep.prim3d.box",defaults("brep.prim3d.box")).unwrap();let input=neural_engine::ColdOwner::new(defaults("brep.mesh.fromBrep").insert("geometry",solid.get("solid").unwrap().clone()));
    for phase in ["samplingEdges","mesh-import-vertices","mesh-import-normals","mesh-output-vertices","mesh-output-pack"] {
        let mut job=registry.dispatch_job("brep.mesh.fromBrep",&input).unwrap().unwrap();let mut trips=0;
        while job.progress().phase!=phase {let before=job.progress().units_done;assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_)));assert!(job.progress().units_done>=before);trips+=1;assert!(trips<10000,"missing {phase}");}
        assert!(matches!(job.step(1).unwrap(),OperatorJobStep::Working(_)));job.cancel();let frozen=job.progress().units_done;drain_cancelled_mesh_job(&mut *job);assert_eq!(job.progress().units_done,frozen);
    }
    let mut job=registry.dispatch_job("brep.mesh.fromBrep",&input).unwrap().unwrap();let output=loop {let before=job.progress().units_done;match job.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.fromBrep",value).unwrap()),OperatorJobStep::Working(_)=>assert!(job.progress().units_done>=before),_=>panic!("unexpected cancellation")}};
    let mesh=read_mesh(&output,"meshOut").unwrap();assert_eq!(mesh.attributes()["normal"].values.len(),mesh.halfedge_count());let report=neural_engine::ColdOwner::new(analyze(&mesh).unwrap());assert_eq!(read_channel_number(&report,"boundaryEdges").unwrap(),0.0);
    eprintln!("[DEBUG] retained fromBrep completedUnits={} corners={} retainedNormals={}",job.progress().units_done,mesh.halfedge_count(),mesh.attributes()["normal"].values.len());
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json")).unwrap();let case=&fixture["toBrep"];let request=neural_engine::ColdOwner::new(defaults("brep.mesh.toBrep").insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(case["mesh"].to_string()))))));let counts=||registry.session.with_kernel_read(|kernel|{let body=kernel.tessellation_body();Ok([body.vertices.len(),body.edges.len(),body.coedges.len(),body.loops.len(),body.faces.len(),body.shells.len(),body.solids.len(),body.curves3.len(),body.curves2.len(),body.surfaces.len(),kernel.live_handles().len()])}).unwrap();let initial=counts();let mut job=registry.dispatch_job("brep.mesh.toBrep",&request).unwrap().expect("toBrep requires the existing retained operator owner");assert!(matches!(job.step(0).unwrap(),OperatorJobStep::Working(_)));let mut phases=std::collections::BTreeMap::new();let output=loop {let before=job.progress();match job.step(1).unwrap() {OperatorJobStep::Working(_)=>{phases.entry(before.phase).or_insert(before.units_done);},OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.toBrep",value).unwrap()),_=>unreachable!()}};let handle=read_geometry(&output,"geometry").unwrap();registry.session.with_kernel_read(|kernel|{kernel.validate_gate_sync(&handle).map_err(|errors|invalid(format!("{errors:?}")))?;assert!((kernel.area_sync(&handle).map_err(|error|map_kernel_error(&error))?-case["area"].as_f64().unwrap()).abs()<1e-6);Ok(())}).unwrap();let current=counts();assert_eq!(current[4],initial[4]+case["triangles"].as_u64().unwrap()as usize);assert_eq!(current[6],initial[6]+case["solids"].as_u64().unwrap() as usize);assert_eq!(output.get("geometry").and_then(Value::as_dictionary).unwrap().get("kind").and_then(Value::as_atom).and_then(Atom::as_str),case["kind"].as_str());
    let closed_request=neural_engine::ColdOwner::new(defaults("brep.mesh.toBrep").insert("mesh",Value::Dictionary(Dictionary::with_schema("mesh").insert("data",Value::Atom(Atom::String(case["closedMesh"].to_string()))))));let mut closed=registry.dispatch_job("brep.mesh.toBrep",&closed_request).unwrap().unwrap();let output=loop {match closed.step(1).unwrap() {OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.toBrep",value).unwrap()),OperatorJobStep::Working(_)=>{},_=>unreachable!()}};let solid=read_geometry(&output,"geometry").unwrap();registry.session.with_kernel_read(|kernel|{kernel.validate_gate_sync(&solid).map_err(|errors|invalid(format!("{errors:?}")))?;assert!((kernel.volume_sync(&solid).map_err(|error|map_kernel_error(&error))?-case["closedVolume"].as_f64().unwrap()).abs()<1e-6);Ok(())}).unwrap();let stable=counts();for (phase,_) in &phases {let mut cancelled=registry.dispatch_job("brep.mesh.toBrep",&request).unwrap().unwrap();while cancelled.progress().phase!=*phase {assert!(matches!(cancelled.step(1).unwrap(),OperatorJobStep::Working(_)));}cancelled.cancel();let frozen=cancelled.progress().units_done;assert!(matches!(cancelled.close_step(1,case["retirementBytes"][0].as_u64().unwrap() as usize).unwrap(),OperatorJobStep::Working(_)));assert_eq!(cancelled.progress().units_done,frozen);let mut retired=0;loop {match cancelled.step(1).unwrap() {OperatorJobStep::Working(_)=>{retired+=1;assert!(retired<10000);},OperatorJobStep::Cancelled(_)=>break,_=>panic!("cancelled import published a handle")}}assert_eq!(counts(),stable);}eprintln!("[DEBUG] Retained toBrep valid triangle-derived topology,allobservedphasecancellation exact{} entity/handle counters,units={}",stable.len(),job.progress().units_done);
    for (phase_index,phase) in phases.keys().filter(|phase|Some(**phase)!=case["privateInitialPhase"].as_str()).enumerate() {
        for all in [false,true] {
            let node=0x6D_50_00u64+phase_index as u64*2+u64::from(all);
            let parked_request=semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
                ("operatorId".into(),semio_framework_pack_json::Value::from("brep.mesh.toBrep")),("inputJson".into(),semio_framework_pack_json::Value::from(semio_framework_pack_json::to_json_string(&*request))),
                ("nodeHash".into(),semio_framework_pack_json::Value::from(node)),("budget".into(),semio_framework_pack_json::Value::from(1u64)),("roundUnits".into(),semio_framework_pack_json::Value::from(1u64))]));
            let compact_request=super::super::tests::compact_evaluation_request(&parked_request);let mut trips=0;
            loop {let request=if trips==0 {&parked_request}else{&compact_request};let response=semio_framework_pack_json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry,request.as_bytes()).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(response.get("done").and_then(|value|value.as_bool()),Some(false),"guest phase {phase}");trips+=1;assert!(trips<200000);if trips==1 {assert_eq!(response.get("phase").and_then(|value|value.as_str()),case["guestFirstPhase"].as_str());}if response.get("phase").and_then(|value|value.as_str())==Some(*phase) {break;}}
            if all {assert_eq!(flow_extension_sdk::cancel_all_evaluations(&registry),1);}else {assert!(flow_extension_sdk::cancel_evaluation(&registry,"brep.mesh.toBrep",node));}
            let mut retired=0;
            while flow_extension_sdk::evaluation_retirement_pending(&registry) {flow_extension_sdk::retire_cancelled_evaluations_close_step(&registry,1,case["retirementBytes"][1].as_u64().unwrap() as usize);retired+=1;assert!(retired<10000);}
            assert!(flow_extension_sdk::evaluation_progress(&registry,"brep.mesh.toBrep",node).is_none());assert_eq!(counts(),stable,"guest cancelled at {phase}");
        }
    }
    let mut fault=semio_framework_3d::brep::engine::MeshImportCursor::new(vec![0.,0.,0.,1.,0.,0.,0.,1.,0.,2.,0.,0.],Vec::new(),vec![0,1,2,0,1,3],0.001).unwrap();
    let mut trips=0;loop {match registry.session.step_mesh_import(&mut fault,1) {Err(_)=>break,Ok(Some(_))=>panic!("faulted import published"),_=>{trips+=1;assert!(trips<10000);}}}assert_eq!(counts(),stable,"fault rollback preserves exactly the existing body");
    let mut interleaved=registry.dispatch_job("brep.mesh.toBrep",&request).unwrap().unwrap();let mut trips=0;let output=loop {match interleaved.step(1).unwrap() {
        OperatorJobStep::Done(value)=>break neural_engine::ColdOwner::new(registry.finish_job("brep.mesh.toBrep",value).unwrap()),
        OperatorJobStep::Working(progress)=>{trips+=1;assert!(trips<10000);if matches!(progress.phase,"mesh-to-brep-triangles"|"mesh-to-brep-shell"|"mesh-to-brep-solid"|"mesh-to-brep-ready") {registry.session.with_kernel(|kernel|{let live=kernel.live_handles();kernel.retain(&live);Ok(())}).unwrap();}},_=>panic!("unexpected cancellation")}};
    let protected=read_geometry(&output,"geometry").unwrap();registry.session.with_kernel_read(|kernel|{kernel.validate_gate_sync(&protected).map_err(|errors|invalid(format!("{errors:?}")))?;Ok(())}).unwrap();
    eprintln!("[DEBUG] Guest toBrep per-phase cancel_evaluation and cancel_all_evaluations retirement preserved all11 initial entity/handle counts; invalidsecondtriangle fault rollback passed");


}

#[semio_framework_async_macros::async_test]
async fn mesh_output_encoding_cancels_through_the_existing_graph_owner() {
    let _serial = super::super::tests::test_serial().await;
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let input = neural_engine::ColdOwner::new(Dictionary::new().insert("data",Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(fixture.get("cases").unwrap().as_array().unwrap()[0].get("mesh").unwrap())))));
    let operator = "brep.mesh.construct"; let node_hash = 0x6D_04_00_i64;
    let request = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("operatorId".into(),semio_framework_pack_json::Value::from(operator)),
        ("inputJson".into(),semio_framework_pack_json::Value::from(semio_framework_pack_json::to_json_string(&*input))),
        ("nodeHash".into(),semio_framework_pack_json::Value::from(node_hash)),
        ("budget".into(),semio_framework_pack_json::Value::from(1_i64)),
        ("wallMicros".into(),semio_framework_pack_json::Value::from(1_i64)),
    ]));
    let compact_request=super::super::tests::compact_evaluation_request(&request);
    for phase in ["mesh-output-pack","mesh-output-base64"] {
        let mut trips = 0;
        loop {
            let current=if trips==0 {&request}else{&compact_request};let result = semio_framework_pack_json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry,current.as_bytes()).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert_eq!(result.get("done").and_then(|value|value.as_bool()),Some(false),"[DEBUG] compact output phase {phase}: {result:?}");
            assert_eq!(result.get("cancellable").and_then(|value|value.as_bool()),Some(true));
            assert_eq!(result.get("outputJson").and_then(|value|value.as_str()),Some(""));
            trips += 1; assert!(trips < 200000);
            if result.get("phase").and_then(|value|value.as_str()) == Some(phase) { break; }
        }
        assert!(flow_extension_sdk::cancel_evaluation(&registry,operator,node_hash as u64));
        drain_cancelled_mesh_evaluation(&registry,&request,operator,node_hash as u64);
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_mirror_uses_the_existing_retained_modeling_owner() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let fixture = fixture.get("mirroring").unwrap();
    let mut source = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
    source.translate(MeshVector([0,1,2].map(|axis|fixture.get("translation").unwrap().as_array().unwrap()[axis].as_f64().unwrap() as f32))).unwrap();
    for phase in fixture.get("phases").unwrap().as_array().unwrap() {
        let mut job = source.mirror_job(MirrorAxis::X,0.0001).unwrap();
        while job.progress().phase != phase.as_str().unwrap() { assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_))); }
        assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_)));
        job.cancel(); let frozen = job.progress();
        assert!(matches!(job.step(usize::MAX).unwrap(),MeshModelingStep::Cancelled(progress) if progress == frozen));
    }
    let mut job = source.mirror_job(MirrorAxis::X,0.0001).unwrap();
    let result = loop { let before = job.progress().units_done; match job.step(1).unwrap() { MeshModelingStep::Working(progress) => assert_eq!(progress.units_done,before+1), MeshModelingStep::Done(mesh) => break mesh, _ => panic!("unexpected cancellation") } };
    let report = neural_engine::ColdOwner::new(analyze(&result).unwrap());
    for key in ["vertices","faces","area","volume"] { assert!((read_channel_number(&report,key).unwrap()-fixture.get(key).unwrap().as_f64().unwrap()).abs()<1e-6); }
    assert_eq!(source.vertex_count(),8); assert_eq!(source.face_count(),6);
}

#[semio_framework_async_macros::async_test]
async fn mesh_merge_uses_retained_pair_and_corner_work() {
    let fixtures = semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let jobs = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixtures.get("merges").unwrap().as_array().unwrap() {
        let source = decode_mesh(&semio_framework_pack_json::to_string(case.get("mesh").unwrap())).unwrap();
        let ids = case.get("selection").unwrap().as_array().unwrap().iter().map(|id|VertexId(id.as_u64().unwrap() as u32)).collect::<Vec<_>>();
        let mode = if case.get("mode").unwrap().as_str()==Some("distance") { WeldMode::ByDistance } else { WeldMode::First };
        let threshold = case.get("threshold").unwrap().as_f64().unwrap() as f32;
        let mut job = source.merge_vertices_job(&ids,mode,threshold).unwrap();
        let mesh = loop { let before = job.progress().units_done; match job.step(1).unwrap() { MeshModelingStep::Working(progress) => assert_eq!(progress.units_done,before+1), MeshModelingStep::Done(mesh) => break mesh, _=>panic!("unexpected cancellation") } };
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for key in ["vertices","faces","area"] { assert!((read_channel_number(&report,key).unwrap()-case.get(key).unwrap().as_f64().unwrap()).abs()<1e-6); }
        if mode == WeldMode::ByDistance {
            for phase in jobs.get("merging").unwrap().get("phases").unwrap().as_array().unwrap() {
                let mut job = source.merge_vertices_job(&ids,mode,threshold).unwrap();
                while job.progress().phase != phase.as_str().unwrap() { assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_))); }
                assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_))); job.cancel(); let frozen = job.progress();
                assert!(matches!(job.step(usize::MAX).unwrap(),MeshModelingStep::Cancelled(progress) if progress==frozen));
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_subdivision_retains_concave_triangulation_and_centroid_work() {
    let fixtures = semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let jobs = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let polygon = &fixtures.get("polygons").unwrap().as_array().unwrap()[0];
    let source = decode_mesh(&semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("vertices".into(),polygon.get("vertices").unwrap().clone()),("faces".into(),polygon.get("faces").unwrap().clone())]))).unwrap();
    for phase in jobs.get("subdivision").unwrap().get("phases").unwrap().as_array().unwrap() {
        let mut job = source.subdivide_faces_job(&[FaceId(0)]).unwrap();
        while job.progress().phase != phase.as_str().unwrap() { assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_))); }
        assert!(matches!(job.step(1).unwrap(),MeshModelingStep::Working(_))); job.cancel(); let frozen = job.progress();
        assert!(matches!(job.step(usize::MAX).unwrap(),MeshModelingStep::Cancelled(progress) if progress==frozen));
    }
    let mut job = source.subdivide_faces_job(&[FaceId(0)]).unwrap();
    let result = loop { let before = job.progress().units_done; match job.step(1).unwrap() { MeshModelingStep::Working(progress)=>assert_eq!(progress.units_done,before+1),MeshModelingStep::Done(mesh)=>break mesh,_=>panic!("unexpected cancellation") } };
    assert_eq!(result.vertex_count(),polygon.get("expectedVertices").unwrap().as_u64().unwrap() as usize);
    assert_eq!(result.face_count(),polygon.get("expectedFaces").unwrap().as_u64().unwrap() as usize);
    let report = neural_engine::ColdOwner::new(analyze(&result).unwrap());
    assert!((read_channel_number(&report,"area").unwrap()-polygon.get("area").unwrap().as_f64().unwrap()).abs()<1e-6);
}

struct TestGeometryRegistry {
    registry: Option<flow_extension_sdk::ExtensionEvaluationResources>,
    session: Session,
}

impl TestGeometryRegistry {
    fn new() -> Self {
        let session = Session::new();
        let registry = flow_extension_sdk::ExtensionEvaluationResources::new(super::super::module_registry(&session));
        Self { registry: Some(registry), session }
    }
}

impl std::ops::Deref for TestGeometryRegistry {
    type Target = neural_engine::SharedRegistry;
    fn deref(&self) -> &Self::Target { self.registry.as_ref().expect("open fixture registry").registry() }
}

impl Drop for TestGeometryRegistry {
    fn drop(&mut self) {
        if let Some(mut registry)=self.registry.take(){use semio_framework_plugin::ExtensionResourceOwner;registry.begin_close();while !registry.terminal_is_empty(){registry.close_step(4096,65536).unwrap();}}
        self.session.close();
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_knife_widget_preserves_shared_fixture_surfaces() {
    let registry = TestGeometryRegistry::new();
    let fixtures = semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/✂️knife-cut/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let info = registry.operator_info("brep.mesh.knifeCut").unwrap();
    assert!(info.inputs.iter().any(|channel| channel.name == "start" && channel.value_types.iter().any(|kind| kind == "point")));
    assert!(info.outputs.iter().any(|channel| channel.name == "meshOut" && channel.value_types.iter().any(|kind| kind == "mesh")));
    for case in fixtures.get("cases").unwrap().as_array().unwrap() {
        let source = decode_mesh(&semio_framework_pack_json::to_string(case.get("mesh").unwrap())).unwrap();
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let cut = case.get("cut").unwrap();
        let coordinates = |key: &str| [0, 1, 2].map(|axis| cut.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let output = registry.dispatch_cold("brep.mesh.knifeCut", Dictionary::new()
            .insert("mesh", seed.get("meshOut").unwrap().clone())
            .insert("face", Value::Dictionary(number_dictionary(cut.get("face").unwrap().as_f64().unwrap())))
            .insert("start", Value::Dictionary(point_dictionary(coordinates("start"))))
            .insert("end", Value::Dictionary(point_dictionary(coordinates("end"))))).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            let expected = expected.as_f64().unwrap();
            if key == "minimumFaces" { assert!(mesh.face_count() as f64 >= expected); continue; }
            let actual = read_channel_number(&report, key).unwrap();
            if expected == 0.0 { assert_eq!(actual, 0.0); } else { assert!((actual / expected - 1.0).abs() < 1e-6, "{key}: {actual} != {expected}"); }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_component_transforms_match_shared_fixtures() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧭️component-transform/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let source = decode_mesh(&semio_framework_pack_json::to_string(fixture.get("mesh").unwrap())).unwrap();
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let transform = case.get("transform").unwrap();
        let operation = transform.get("operation").unwrap().as_str().unwrap();
        let coordinates = |key: &str| [0, 1, 2].map(|axis| transform.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let pivot = transform.get("pivot").unwrap();
        let input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone())
            .insert("mode", Value::Dictionary(text_dictionary(transform.get("mode").unwrap().as_str().unwrap())))
            .insert("selection", Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(transform.get("selection").unwrap()))))
            .insert(match operation { "translate" => "offset", "rotate" => "axis", _ => "factor" }, Value::Dictionary(vector_dictionary(coordinates("vector"))))
            .insert("angle", Value::Dictionary(number_dictionary(transform.get("angle").unwrap().as_f64().unwrap())))
            .insert("pivot", Value::Dictionary(text_dictionary(if pivot.as_str() == Some("selection") { "selection" } else { "point" })))
            .insert("center", Value::Dictionary(point_dictionary(if pivot.as_array().is_some() { coordinates("pivot") } else { [0.0; 3] })));
        let output = registry.dispatch_cold(&format!("brep.mesh.{operation}Components"), input).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        assert_eq!(mesh.face_count(), source.face_count());
        for id in 0..mesh.face_count() { assert_eq!(mesh.face_vertex_ids(FaceId(id as u32)).unwrap(), source.face_vertex_ids(FaceId(id as u32)).unwrap()); }
        for (id, expected) in case.get("expected").unwrap().as_array().unwrap().iter().enumerate() {
            let actual = mesh.vertex_position(VertexId(id as u32)).unwrap().0;
            for axis in 0..3 { assert!((actual[axis] as f64 - expected.as_array().unwrap()[axis].as_f64().unwrap()).abs() < 1e-5); }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_loop_cut_consumes_preview_halfedge_ids() {
    let registry = TestGeometryRegistry::new();
    let source = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let preview = source.tessellate().unwrap();
    let edge = *preview.edge_ids.iter().find(|&&id| id as usize >= source.edge_count()).expect("preview has sparse halfedge ids");
    let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
    let output = registry.dispatch_cold("brep.mesh.loopCut", Dictionary::new()
        .insert("mesh", seed.get("meshOut").unwrap().clone())
        .insert("edges", Value::Dictionary(text_dictionary(format!("[{edge}]"))))
        .insert("cuts", Value::Dictionary(number_dictionary(2.0)))).unwrap();
    let mesh = read_mesh(&output, "meshOut").unwrap();
    assert_eq!(mesh.vertex_count(), 16);
    assert_eq!(mesh.face_count(), 14);
    let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
    assert_eq!(read_channel_number(&report, "boundaryEdges").unwrap(), 0.0);
    assert!((read_channel_number(&report, "volume").unwrap() - 1.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn brep_scale_preserves_each_axis_and_explicit_center() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture.get("brepScales").unwrap().as_array().unwrap() {
        let coordinates = |key: &str| [0, 1, 2].map(|axis| case.get(key).unwrap().as_array().unwrap()[axis].as_f64().unwrap());
        let input = Dictionary::new().insert("width", Value::Dictionary(number_dictionary(1.0))).insert("depth", Value::Dictionary(number_dictionary(1.0))).insert("height", Value::Dictionary(number_dictionary(1.0)));
        let source = registry.dispatch_cold("brep.prim3d.box", input).unwrap();
        let output = registry.dispatch_cold("brep.xform.scale", Dictionary::new().insert("geometry", source.get("solid").unwrap().clone()).insert("factor", Value::Dictionary(vector_dictionary(coordinates("factors")))).insert("center", Value::Dictionary(point_dictionary(coordinates("center"))))).unwrap();
        let handle = read_geometry(&output, "geometryOut").unwrap();
        registry.session.with_kernel_read(|kernel| {
            let mesh = kernel.tessellate(&handle, 0.1).map_err(|error| map_kernel_error(&error))?;
            let minimum = [0, 1, 2].map(|axis| mesh.position.chunks_exact(3).map(|point| point[axis] as f64).fold(f64::INFINITY, f64::min));
            let maximum = [0, 1, 2].map(|axis| mesh.position.chunks_exact(3).map(|point| point[axis] as f64).fold(f64::NEG_INFINITY, f64::max));
            let volume = kernel.volume(&handle).map_err(|error| map_kernel_error(&error))?;
            for axis in 0..3 {
                assert!((minimum[axis] - coordinates("minimum")[axis]).abs() < 1e-6);
                assert!((maximum[axis] - coordinates("maximum")[axis]).abs() < 1e-6);
            }
            assert!((volume - case.get("volume").unwrap().as_f64().unwrap()).abs() < 1e-6);
            Ok(())
        }).unwrap();
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_reflections_preserve_outward_winding_at_every_scale() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture.get("reflections").unwrap().as_array().unwrap() {
        let factors = case.get("factors").unwrap().as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        let seed = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
        let factor = Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(factors[0]))).insert("y", Value::Atom(Atom::Decimal(factors[1]))).insert("z", Value::Atom(Atom::Decimal(factors[2])));
        let output = registry.dispatch_cold("brep.mesh.scale", Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("factor", Value::Dictionary(factor))).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        let triangles = mesh.tessellate().unwrap();
        let mut volume = 0.0;
        for face in triangles.indices.chunks_exact(3) {
            let point = |index: u32| [0, 1, 2].map(|axis| triangles.positions[index as usize * 3 + axis] as f64);
            let [a, b, c] = [point(face[0]), point(face[1]), point(face[2])];
            volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) + a[1] * (b[2] * c[0] - b[0] * c[2]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
        }
        let expected = case.get("volume").unwrap().as_f64().unwrap();
        assert!((volume / expected - 1.0).abs() < 1e-6, "{}: signed volume {volume}", case.get("name").unwrap().as_str().unwrap());
    }
}

#[test]
fn indexed_mesh_contract_fixtures() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture.get("meshes").unwrap().as_array().unwrap() {
        let mesh = decode_mesh(&semio_framework_pack_json::to_string(case.get("mesh").unwrap())).unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            if let Some(number) = expected.as_f64() {
                let actual = report.get(key).unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_f64().unwrap();
                assert!((actual - number).abs() <= if number == 0.0 { 1e-12 } else { number.abs() * 1e-5 }, "{key}: {actual} != {number}");
            } else { assert!(report.get(key).is_none(), "open surfaces cannot report enclosed volume"); }
        }
        eprintln!("mesh fixture {}: area={}", case.get("name").unwrap().as_str().unwrap(), read_channel_number(&report, "area").unwrap());
        assert_eq!(mesh.to_obj().unwrap(), decode_mesh(&encode_mesh(&mesh).unwrap()).unwrap().to_obj().unwrap());
    }
    for case in fixture.get("invalid").unwrap().as_array().unwrap() {
        assert!(decode_mesh(&semio_framework_pack_json::to_string(case.get("mesh").unwrap())).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_widgets_are_registered_and_typed() {
    let registry = TestGeometryRegistry::new();
    for id in ["brep.mesh.box", "brep.mesh.construct", "brep.mesh.fromBrep", "brep.mesh.extrude", "brep.mesh.inset", "brep.mesh.loopCut", "brep.mesh.knifeCut", "brep.mesh.analyze", "brep.mesh.exportObj"] {
        let info = registry.operator_info(id).expect(id);
        assert!(info.group.iter().any(|group| group.starts_with("Mesh")));
        assert!(!info.outputs.is_empty());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_widget_workflow_fixtures_execute() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for case in fixture.get("workflows").unwrap().as_array().unwrap() {
        let operation = case.get("operation").unwrap().as_str().unwrap();
        let id = format!("brep.mesh.{operation}");
        let info = registry.operator_info(&id).unwrap();
        let mut input = Dictionary::new();
        for channel in &info.inputs {
            if let Some(value) = &channel.default { input = input.insert(channel.name.clone(), value.clone()); }
        }
        if info.inputs.iter().any(|channel| channel.name == "mesh") {
            let seed = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
            input = input.insert("mesh", seed.get("meshOut").unwrap().clone());
        }
        let output = registry.dispatch_cold(&id, input).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let mesh = output.get("meshOut").unwrap().as_dictionary().unwrap();
        let mesh = decode_mesh(mesh.get("data").unwrap().as_atom().unwrap().as_str().unwrap()).unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            let actual = report.get(key).unwrap().as_dictionary().unwrap().get("value").unwrap().as_atom().unwrap().as_f64().unwrap();
            assert!((actual - expected.as_f64().unwrap()).abs() < 1e-5, "{id} {key}: {actual} != {expected:?}");
        }
    }
}

#[test]
fn mesh_preview_preserves_topology_identifiers() {
    let output = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
    let dictionary = output.get("meshOut").unwrap().as_dictionary().unwrap();
    let body = dictionary.get("preview").unwrap().as_atom().unwrap().as_str().unwrap();
    let preview = decode_mesh_pack(&decode_base64(body).unwrap()).unwrap();
    assert_eq!(preview.indices.len(), 36);
    assert_eq!(preview.face_ids.len(), 12);
    assert!(preview.face_ids.iter().all(|id| *id < 6));
    assert!(preview.vertex_ids.iter().all(|id| *id < 8));
}

#[test]
fn brep_tessellation_seams_become_shared_mesh_vertices() {
    let source = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap().tessellate().unwrap();
    let welded = indexed_triangle_mesh(&source.positions, &source.indices).unwrap();
    assert_eq!(welded.vertex_count(), 8);
    let report = neural_engine::ColdOwner::new(analyze(&welded).unwrap());
    assert_eq!(read_channel_number(&report, "boundaryEdges").unwrap(), 0.0);
    assert!((read_channel_number(&report, "volume").unwrap() - 1.0).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn mesh_workbench_creates_edits_analyzes_and_converts() {
    let registry = TestGeometryRegistry::new();
    let defaults = |id: &str| registry.operator_info(id).unwrap().inputs.iter().fold(Dictionary::new(), |input, channel| match &channel.default {
        Some(value) => input.insert(channel.name.clone(), value.clone()),
        None => input,
    });
    let mut output = registry.dispatch_cold("brep.mesh.box", defaults("brep.mesh.box")).unwrap();
    for (id, key, value) in [("brep.mesh.inset", "amount", 0.1), ("brep.mesh.extrude", "distance", 0.5)] {
        let input = defaults(id).insert("mesh", output.get("meshOut").unwrap().clone()).insert(key, Value::Dictionary(number_dictionary(value)));
        output = registry.dispatch_cold(id, input).unwrap();
    }
    let analyzed = registry.dispatch_cold("brep.mesh.analyze", Dictionary::new().insert("mesh", output.get("meshOut").unwrap().clone())).unwrap();
    let area = read_channel_number(&analyzed, "area").unwrap();
    let volume = read_channel_number(&analyzed, "volume").unwrap();
    assert!((area - 7.6).abs() < 1e-5);
    assert!((volume - 1.32).abs() < 1e-5);
    let brep = registry.dispatch_cold("brep.mesh.toBrep", defaults("brep.mesh.toBrep").insert("mesh", output.get("meshOut").unwrap().clone())).unwrap();
    let handle = read_geometry(&brep, "geometry").unwrap();
    let transfer = registry.session.with_kernel_read(|kernel| kernel.tessellate(&handle, 0.1).map_err(|error| map_kernel_error(&error))).unwrap();
    assert!(!transfer.index.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn mesh_inspection_widgets_match_portable_fixtures() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔎️inspection/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh = decode_mesh(&semio_framework_pack_json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let seed = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let query = case.get("query").unwrap();
        let id = format!("brep.mesh.inspect{}", match query.get("kind").unwrap().as_str().unwrap() { "vertex" => "Vertex", "edge" => "Edge", _ => "Face" });
        let result = registry.dispatch_cold(&id, Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(query.get("index").unwrap().as_f64().unwrap())))).unwrap();
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() {
            if key == "vertices" { assert_eq!(semio_framework_pack_json::parse(&read_text(&result, key).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), *expected); }
            else if let Some(values) = expected.as_array() {
                let actual = read_xyz(&result, key).unwrap();
                for axis in 0..3 { assert!((actual[axis] - values[axis].as_f64().unwrap()).abs() < 1e-5); }
            } else { assert_eq!(read_channel_number(&result, key).unwrap(), expected.as_f64().unwrap()); }
        }
        let info = registry.operator_info(&id).unwrap();
        for output in &info.outputs { assert!(result.get(&output.name).is_some(), "{id} missing {}", output.name); }
    }
    for query in fixture.get("invalid").unwrap().as_array().unwrap() {
        let Some(kind) = query.get("kind").unwrap().as_str().filter(|kind| ["vertex", "edge", "face"].contains(kind)) else { continue; };
        let id = format!("brep.mesh.inspect{}", match kind { "vertex" => "Vertex", "edge" => "Edge", _ => "Face" });
        assert!(registry.dispatch_cold(&id, Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(query.get("index").unwrap().as_f64().unwrap())))).is_err());
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_modeling_widgets_match_kernel_portable_fixtures() {
    let registry = TestGeometryRegistry::new();
    let fixtures = semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🧫️fixtures/🛠️modeling/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for (key, operation) in [("bevels", "bevel"), ("dissolutions", "dissolveVertices"), ("merges", "mergeVertices"), ("mirrors", "mirror"), ("decimations", "decimate")] {
        for fixture in fixtures.get(key).unwrap().as_array().unwrap() {
            let mut mesh = if let Some(mesh) = fixture.get("mesh") { decode_mesh(&semio_framework_pack_json::to_string(mesh)).unwrap() } else { HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap() };
            if let Some(translation) = fixture.get("translation") { mesh.translate(MeshVector([0, 1, 2].map(|axis| translation.as_array().unwrap()[axis].as_f64().unwrap() as f32))).unwrap(); }
            let seed = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
            let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone());
            for parameter in ["amount", "segments", "ratio"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); } }
            for parameter in ["mode", "axis"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(text_dictionary(value.as_str().unwrap()))); } }
            if let Some(value) = fixture.get("threshold") { input = input.insert("tolerance", Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); }
            if let Some(value) = fixture.get("selection") { input = input.insert("selection", Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(value)))); }
            if let Some(pairs) = fixture.get("edges") {
                let ids: Vec<_> = pairs.as_array().unwrap().iter().map(|pair| {
                    let pair = pair.as_array().unwrap();
                    let a = pair[0].as_u64().unwrap() as u32; let b = pair[1].as_u64().unwrap() as u32;
                    (0..mesh.halfedge_count()).find(|&id| mesh.edge_endpoints(EdgeId(id as u32)).is_ok_and(|(x, y)| (x.0 == a && y.0 == b) || (x.0 == b && y.0 == a))).unwrap() as u32
                }).collect();
                input = input.insert("edges", Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(&semio_framework_pack_json::array(ids.into_iter().map(semio_framework_pack_json::Value::from))))));
            }
            let output = registry.dispatch_cold(&format!("brep.mesh.{operation}"), input).unwrap();
            let mesh = read_mesh(&output, "meshOut").unwrap();
            let report = neural_engine::ColdOwner::new(analyze(&mesh).unwrap());
            for parameter in ["vertices", "faces", "area", "volume"] {
                if let Some(expected) = fixture.get(parameter).and_then(|value| value.as_f64()) { let actual = read_channel_number(&report, parameter).unwrap(); assert!((actual - expected).abs() < 1e-5, "{operation} {parameter}: {actual} != {expected}"); }
            }
            if let Some(maximum) = fixture.get("maximumVertices") { assert!(mesh.vertex_count() as f64 <= maximum.as_f64().unwrap()); }
            if let Some(minimum) = fixture.get("minimumVolume") { assert!(read_channel_number(&report, "volume").unwrap() >= minimum.as_f64().unwrap()); }
        }
    }
    for (key, operation) in [("proportional", "moveProportional"), ("snapping", "snapVertices")] {
        let fixture = fixtures.get(key).unwrap();
        let source = decode_mesh(&semio_framework_pack_json::to_string(fixture.get("mesh").unwrap())).unwrap();
        let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
        let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone()).insert("selection", Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(fixture.get("selection").unwrap()))));
        for (parameter, target) in [("pivot", "center"), ("delta", "offset")] { if let Some(value) = fixture.get(parameter) { input = input.insert(target, Value::Dictionary(point_dictionary([0, 1, 2].map(|axis| value.as_array().unwrap()[axis].as_f64().unwrap())))); } }
        for parameter in ["radius", "grid"] { if let Some(value) = fixture.get(parameter) { input = input.insert(parameter, Value::Dictionary(number_dictionary(value.as_f64().unwrap()))); } }
        let output = registry.dispatch_cold(&format!("brep.mesh.{operation}"), input).unwrap();
        let mesh = read_mesh(&output, "meshOut").unwrap();
        for (id, expected) in fixture.get("expected").unwrap().as_array().unwrap().iter().enumerate() {
            let actual = mesh.vertex_position(VertexId(id as u32)).unwrap().0;
            for axis in 0..3 { assert!((actual[axis] as f64 - expected.as_array().unwrap()[axis].as_f64().unwrap()).abs() < 1e-6); }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_face_merge_widgets_preserve_fixture_surface() {
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧵️merge-faces/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let source = decode_mesh(&semio_framework_pack_json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let seed = neural_engine::ColdOwner::new(mesh_output(&source).unwrap());
    for case in fixture.get("cases").unwrap().as_array().unwrap() {
        let mut input = Dictionary::new().insert("mesh", seed.get("meshOut").unwrap().clone());
        if let Some(edges) = case.get("edges") { input = input.insert("edges", Value::Dictionary(text_dictionary(semio_framework_pack_json::to_string(edges)))); }
        let id = format!("brep.mesh.{}", case.get("operation").unwrap().as_str().unwrap());
        let output = registry.dispatch_cold(&id, input).unwrap();
        let result = read_mesh(&output, "meshOut").unwrap();
        let report = neural_engine::ColdOwner::new(analyze(&result).unwrap());
        for (key, expected) in case.get("expected").unwrap().as_object().unwrap() { assert!((read_channel_number(&report, key).unwrap() - expected.as_f64().unwrap()).abs() < 1e-6); }
    }
}

#[semio_framework_async_macros::async_test]
async fn reflected_preview_edge_ids_inspect_the_serialized_mesh() {
    let registry = TestGeometryRegistry::new();
    let fixtures = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔎️inspection/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let source = neural_engine::ColdOwner::new(mesh_output(&HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap()).unwrap());
    for transform in fixtures.get("previewTransforms").unwrap().as_array().unwrap() {
        let factor = transform.get("factor").unwrap().as_array().unwrap();
        let output = registry.dispatch_cold("brep.mesh.scale", Dictionary::new().insert("mesh", source.get("meshOut").unwrap().clone()).insert("factor", Value::Dictionary(vector_dictionary([0, 1, 2].map(|axis| factor[axis].as_f64().unwrap()))))).unwrap();
        let preview_text = output.get("meshOut").unwrap().as_dictionary().unwrap().get("preview").unwrap().as_atom().unwrap().as_str().unwrap();
        let preview = decode_mesh_pack(&decode_base64(preview_text).unwrap()).unwrap();
        for (&id, endpoints) in preview.edge_ids.iter().zip(preview.edge_positions.chunks_exact(6)) {
            let inspected = registry.dispatch_cold("brep.mesh.inspectEdge", Dictionary::new().insert("mesh", output.get("meshOut").unwrap().clone()).insert("index", Value::Dictionary(number_dictionary(id as f64)))).unwrap();
            let start = read_xyz(&inspected, "start").unwrap(); let end = read_xyz(&inspected, "end").unwrap();
            let a = [endpoints[0] as f64, endpoints[1] as f64, endpoints[2] as f64]; let b = [endpoints[3] as f64, endpoints[4] as f64, endpoints[5] as f64];
            assert!((start == a && end == b) || (start == b && end == a), "preview edge {id}: {start:?} {end:?} != {a:?} {b:?}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn mesh_modeling_yields_and_cancels_through_graph_evaluation_handler() {
    let _serial = super::super::tests::test_serial().await;
    let registry = TestGeometryRegistry::new();
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/⏱️modeling-budget/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh = decode_mesh(&semio_framework_pack_json::to_string(fixture.get("mesh").unwrap())).unwrap();
    let source = neural_engine::ColdOwner::new(mesh_output(&mesh).unwrap());
    for (index, case) in fixture.get("cases").unwrap().as_array().unwrap().iter().enumerate() {
        let operator = case.get("operator").unwrap().as_str().unwrap();
        let mut input = Dictionary::new().insert("mesh", source.get("meshOut").unwrap().clone());
        for (key, value) in case.get("parameters").unwrap().as_object().unwrap() {
            let value = if let Some(number) = value.as_f64() { number_dictionary(number) } else { text_dictionary(value.as_str().unwrap()) };
            input = input.insert(key, Value::Dictionary(value));
        }
        let input = neural_engine::ColdOwner::new(input);
        let input_json = semio_framework_pack_json::to_json_string(&*input);
        let node_hash = 0x6D_00_00 + index as i64;
        let request = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("operatorId".into(), semio_framework_pack_json::Value::from(operator)),
            ("inputJson".into(), semio_framework_pack_json::Value::from(input_json)),
            ("nodeHash".into(), semio_framework_pack_json::Value::from(node_hash)),
            ("budget".into(), semio_framework_pack_json::Value::from(1_i64)),
            ("wallMicros".into(), semio_framework_pack_json::Value::from(1_i64)),
        ]));
        let first = semio_framework_pack_json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, request.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(first.get("done").and_then(|value| value.as_bool()), Some(false), "{operator} must yield");
        assert_eq!(first.get("cancellable").and_then(|value| value.as_bool()), Some(true));
        assert_eq!(first.get("outputJson").and_then(|value| value.as_str()), Some(""));
        assert!(flow_extension_sdk::evaluation_progress(&registry,operator, node_hash as u64).is_some());
        assert!(flow_extension_sdk::cancel_evaluation(&registry,operator, node_hash as u64));
        assert!(!flow_extension_sdk::cancel_evaluation(&registry,operator, node_hash as u64));
        drain_cancelled_mesh_evaluation(&registry,&request,operator,node_hash as u64);
        let compact_request=super::super::tests::compact_evaluation_request(&request);let mut trips = 0; let mut previous = 0;
        let output = loop {
            trips += 1; assert!(trips < 200000, "{operator} did not terminate");
            let current=if trips==1 {&request}else{&compact_request};let result = semio_framework_pack_json::parse_bytes(&flow_extension_sdk::evaluate_invoke_json(&registry, current.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let done = result.get("unitsDone").unwrap().as_u64().unwrap(); let total = result.get("unitsTotal").unwrap().as_u64().unwrap();
            assert!(done >= previous && done <= total); previous = done;
            if result.get("done").unwrap().as_bool() == Some(true) { break semio_framework_pack_json::parse(result.get("outputJson").unwrap().as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); }
            assert_eq!(result.get("cancellable").and_then(|value| value.as_bool()), Some(true));
            assert_eq!(result.get("outputJson").and_then(|value| value.as_str()), Some(""));
        };
        assert!(trips >= case.get("minimumRoundTrips").unwrap().as_u64().unwrap() as usize);
        let expected = neural_engine::ColdOwner::new(registry.dispatch(operator, &input).unwrap());
        let expected = semio_framework_pack_json::to_json_string(&*expected);
        assert_eq!(output, semio_framework_pack_json::parse(&expected, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        assert_eq!(encode_mesh(&mesh).unwrap(), source.get("meshOut").unwrap().as_dictionary().unwrap().get("data").unwrap().as_atom().unwrap().as_str().unwrap());
    }
}
#[test]
fn polygon_attributes_share_portable_fixture_and_preserve_owned_assets() {
    let fixture = semio_framework_pack_json::parse(include_str!("../../../../../../../../🧰️framework/🔨️modules/🏗️mesh-engine/🧫️fixtures/🎨️attributes/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let input = fixture.get("mesh").unwrap();
    let mesh = decode_mesh(&input.to_string()).unwrap();
    assert_eq!(mesh.attributes().len(),7);
    let encoded = encode_mesh(&mesh).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap(),serde_json::from_str::<serde_json::Value>(&input.to_string()).unwrap());
    for invalid in fixture.get("invalidCases").unwrap().as_array().unwrap() {
        let mut input = input.clone();
        if let Some(channel) = invalid.get("channel").and_then(semio_framework_pack_json::Value::as_str) {
            input.get_mut("attributes").unwrap().get_mut(channel).unwrap().as_object_mut().unwrap().insert("values",invalid.get("values").unwrap().clone());
        }
        if let Some(material) = invalid.get("material").and_then(semio_framework_pack_json::Value::as_str) {
            input.get_mut("materials").unwrap().get_mut(material).unwrap().as_object_mut().unwrap().insert("baseColorTexture",invalid.get("baseColorTexture").unwrap().clone());
        }
        assert!(decode_mesh(&input.to_string()).is_err());
    }
}

fn drain_cancelled_mesh_job(job:&mut dyn neural_engine::OperatorJob) {
    use neural_engine::OperatorJobStep;
    let before=job.progress();
    assert!(matches!(job.close_step(1,0).unwrap(),OperatorJobStep::Working(progress) if progress.units_done==before.units_done));
    for _ in 0..100_000 {
        match job.close_step(1,8).unwrap() {
            OperatorJobStep::Working(_)=>{},
            OperatorJobStep::Cancelled(_)=>return,
            OperatorJobStep::Done(_)=>panic!("cancelled mesh candidate was published"),
        }
    }
    panic!("mesh cancellation did not drain within bounded grants");
}
