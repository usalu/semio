#[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let fixture = semio_framework_pack_json::parse(include_str!("../../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("brep").and_then(|entry| entry.get("pluginId")).and_then(semio_framework_pack_json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("brep").and_then(|entry| entry.get("flowId")).and_then(semio_framework_pack_json::Value::as_str));
        }
        bundle.begin_close();
        for _ in 0..100000 {
            let grant=geometry_test_close_grant(&bundle,1,64);
            let step=bundle.close_step(grant).unwrap();
            assert!(step.progress().unwrap().fits(grant));
            if matches!(step,PluginLifecycleStep::Complete(_)) {assert!(bundle.terminal_is_empty());return;}
        }
        panic!("BREP guest bundle resource retirement did not finish");
    }

    #[test]
    fn extension_guest_retires_actual_session_geometry_and_inflight_tessellation() {
        let imports=semio_framework_pack_json::parse(include_str!("../../🥽️mesh/🧫️fixtures/⏱️output-budget/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for (index,phase) in ["mesh-to-brep-tessellate","mesh-to-brep-retire-source","mesh-to-brep-admit","mesh-to-brep-triangles","mesh-to-brep-shell","mesh-to-brep-ready"].into_iter().enumerate() {
            let (mut owner,scope)=scoped_bundle();let input=semio_framework_pack_json::object([
                ("mesh".into(),semio_framework_pack_json::object([("$schema".into(),semio_framework_pack_json::Value::from("mesh")),("data".into(),semio_framework_pack_json::Value::from(semio_framework_pack_json::to_string(imports.get("toBrep").unwrap().get("mesh").unwrap())))])),
                ("tolerance".into(),semio_framework_pack_json::object([("$schema".into(),semio_framework_pack_json::Value::from("number")),("value".into(),semio_framework_pack_json::Value::from(0.001))]))]);
            let request=semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("operatorId".into(),semio_framework_pack_json::Value::from("brep.mesh.toBrep")),("retained".into(),super::tests::geometry_test_retained()),("inputJson".into(),semio_framework_pack_json::Value::from(semio_framework_pack_json::to_string(&input))),("nodeHash".into(),semio_framework_pack_json::Value::from(0x6D_60_00+index as u64)),("budget".into(),semio_framework_pack_json::Value::from(1u64)),("roundUnits".into(),semio_framework_pack_json::Value::from(1u64))]));let compact=super::tests::compact_evaluation_request(&request);let mut trips=0;
            loop {let answer=geometry_test_envelope(&owner,if trips==0{&request}else{&compact});assert_eq!(answer.get("done").and_then(semio_framework_pack_json::Value::as_bool),Some(false));trips+=1;assert!(trips<10000,"missing {phase}");if answer.get("phase").and_then(semio_framework_pack_json::Value::as_str)==Some(phase) {break;}}
            owner.begin_close();if flow_extension_sdk::evaluation_retirement_pending(&scope) {let grant=geometry_test_close_grant(&owner,1,8);assert_eq!(owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Some(Default::default()));assert_eq!(owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,..grant}).unwrap().progress(),Some(Default::default()));}
            let mut trips=0;while flow_extension_sdk::evaluation_retirement_pending(&scope) {let grant=geometry_test_close_grant(&owner,1,8);let small=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_release_bytes:8,..grant};let step=owner.close_step(small).unwrap();assert!(step.progress().unwrap().fits(small));if step.progress()==Some(Default::default()){assert!(owner.close_step(grant).unwrap().progress().unwrap().fits(grant));}trips+=1;assert!(trips<10000);}
            drop(scope);loop {let grant=geometry_test_close_grant(&owner,1,64);let step=owner.close_step(grant).unwrap();trips+=1;assert!(trips<100000);assert!(step.progress().unwrap().fits(grant));if matches!(step,PluginLifecycleStep::Complete(_)){assert!(owner.terminal_is_empty());break;}}
            eprintln!("[DEBUG] Existing BRep bundle drained inflight toBrep at {phase} before Session close,steps={trips}");
        }
        let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let grant = fixture.get("grant").and_then(semio_framework_pack_json::Value::as_array).unwrap();
        let items = grant[0].as_f64().unwrap() as usize;
        let bytes = grant[1].as_f64().unwrap() as usize;
        let mut bundle = bundle();
        let evaluate = semio_framework_pack_json::to_string(fixture.get("evaluate").unwrap());
        let mut answer = semio_framework_pack_json::parse_bytes(&bundle.invoke("evaluate", evaluate.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let compact=super::tests::compact_evaluation_request(&evaluate);let mut turns=0;
        while answer.get("done").and_then(semio_framework_pack_json::Value::as_bool)==Some(false) {turns+=1;assert!(turns<fixture.get("steps").unwrap().as_u64().unwrap());answer=geometry_test_envelope(&bundle,&compact);}
        assert_eq!(answer.get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(true));
        let output = semio_framework_pack_json::parse(answer.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let solid = output.get("solid").unwrap();
        assert_eq!(solid.get("$schema").and_then(semio_framework_pack_json::Value::as_str),fixture.get("geometrySchema").and_then(semio_framework_pack_json::Value::as_str));
        let handle = solid.get("handle").and_then(semio_framework_pack_json::Value::as_str).unwrap();
        let request = fixture.get("tessellate").unwrap();
        let tessellate = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("handle".into(),semio_framework_pack_json::Value::from(handle)),
            ("tolerance".into(),request.get("tolerance").unwrap().clone()),
            ("budget".into(),request.get("budget").unwrap().clone()),
            ("wallMicros".into(),request.get("wallMicros").unwrap().clone()),
            ("chunk".into(),request.get("chunk").unwrap().clone()),
        ]));
        let mesh = semio_framework_pack_json::parse_bytes(&bundle.invoke("tessellate",tessellate.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(mesh.get("done").and_then(semio_framework_pack_json::Value::as_bool),Some(false));
        assert!(mesh.get("unitsDone").and_then(semio_framework_pack_json::Value::as_f64).unwrap() > 0.0);
        let probe=geometry_test_close_grant(&bundle,0,bytes);assert_eq!(bundle.close_step(probe).unwrap().progress(),Some(Default::default()));
        bundle.begin_close();
        assert_eq!(bundle.invoke("evaluate",evaluate.as_bytes()).unwrap_err().code.0.as_str(),"extension.closing");
        bundle.cancel_close();
        let paused=geometry_test_close_grant(&bundle,items,bytes);assert!(matches!(bundle.close_step(paused).unwrap(),PluginLifecycleStep::Blocked { .. }));
        bundle.resume_close();
        let mut released = 0;
        let mut released_bytes_total = 0;
        for _ in 0..fixture.get("steps").and_then(semio_framework_pack_json::Value::as_f64).unwrap() as usize {
            let grant=geometry_test_close_grant(&bundle,items,bytes);let step=bundle.close_step(grant).unwrap();
            let receipt=step.progress().unwrap_or_else(||panic!("unexpected BREP close status {step:?}"));assert!(receipt.fits(grant));released+=receipt.copied_items;released_bytes_total+=receipt.released_bytes;
            if matches!(step,PluginLifecycleStep::Complete(_)){assert!(bundle.terminal_is_empty());assert!(released>4);assert!(released_bytes_total>0);return;}
        }
        panic!("actual BREP geometry and tessellation did not retire");
    }


fn geometry_test_request(operator: &str, input: semio_framework_pack_json::Value, node_hash: u64, budget: u64, wall: u64) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("operatorId".into(), semio_framework_pack_json::Value::from(operator)),("retained".into(),super::tests::geometry_test_retained()),
        ("inputJson".into(), semio_framework_pack_json::Value::from(semio_framework_pack_json::to_string(&input))),
        ("nodeHash".into(), semio_framework_pack_json::Value::from(node_hash)),
        ("budget".into(), semio_framework_pack_json::Value::from(budget)),
        ("wallMicros".into(), semio_framework_pack_json::Value::from(wall)),
        ("operatorVersion".into(), semio_framework_pack_json::Value::from("1")),
        ("dependencyJson".into(), semio_framework_pack_json::Value::from("{\"box\":[4,4,2],\"boxOffset\":[-2,-2,-0.5],\"cylinder\":[0.5,4],\"cylinderOffset\":[0,0,-2],\"operation\":\"brep.bool.cut\"}")),
    ]))
}

fn geometry_test_envelope(bundle: &ExtensionBundle, request: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::parse_bytes(&bundle.invoke("evaluate", request.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

fn geometry_test_output(envelope: &semio_framework_pack_json::Value) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::parse(envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

fn geometry_test_mesh_pack(bundle: &ExtensionBundle, output: &semio_framework_pack_json::Value) -> Vec<String> {
    let handle = output.get("solid").unwrap().get("handle").unwrap().as_str().unwrap();
    let mut chunks = Vec::new();
    let mut chunk = 0;
    for _ in 0..10000 {
        let request = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("handle".into(), semio_framework_pack_json::Value::from(handle)),
            ("tolerance".into(), semio_framework_pack_json::Value::from(0.05)),
            ("budget".into(), semio_framework_pack_json::Value::from(1000000u64)),
            ("wallMicros".into(), semio_framework_pack_json::Value::from(1000000u64)),
            ("chunk".into(), semio_framework_pack_json::Value::from(chunk)),
        ]));
        let answer = semio_framework_pack_json::parse_bytes(&bundle.invoke("tessellate", request.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        if answer.get("done").and_then(semio_framework_pack_json::Value::as_bool) != Some(true) { continue; }
        chunks.push(answer.get("meshPack").unwrap().as_str().unwrap().to_string());
        chunk += 1;
        if chunk >= answer.get("chunks").and_then(semio_framework_pack_json::Value::as_u64).unwrap() { return chunks; }
    }
    panic!("geometry parity tessellation failed to complete");
}

fn geometry_test_mesh_signature(chunks:Vec<String>)->(Vec<[i64;6]>,Vec<[i64;6]>,usize,f64,f64) {
    let mut corners=Vec::new();let mut edges=Vec::new();let mut triangles=0;let mut area=0.0;let mut volume=0.0;
    let quantize=|value:f32|{assert!(value.is_finite());(value as f64*1_000_000.0).round()as i64};
    for chunk in chunks {
        let mesh=super::decode_mesh_pack(&super::decode_base64(&chunk).unwrap()).unwrap();assert_eq!(mesh.positions.len(),mesh.normals.len());
        for (point,normal) in mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)) {corners.push([quantize(point[0]),quantize(point[1]),quantize(point[2]),quantize(normal[0]),quantize(normal[1]),quantize(normal[2])]);}
        for line in mesh.edge_positions.chunks_exact(6) {let a=[quantize(line[0]),quantize(line[1]),quantize(line[2])];let b=[quantize(line[3]),quantize(line[4]),quantize(line[5])];let (a,b)=if a<=b {(a,b)}else {(b,a)};edges.push([a[0],a[1],a[2],b[0],b[1],b[2]]);}
        let point=|id:u32|{let offset=id as usize*3;semio_framework_3d::brep::representation::vector::Vec3::new(mesh.positions[offset]as f64,mesh.positions[offset+1]as f64,mesh.positions[offset+2]as f64)};
        for ids in mesh.indices.chunks_exact(3) {let (a,b,c)=(point(ids[0]),point(ids[1]),point(ids[2]));area+=(b-a).cross(c-a).norm()/2.0;volume+=a.dot(b.cross(c))/6.0;triangles+=1;}
    }
    corners.sort_unstable();edges.sort_unstable();(corners,edges,triangles,area,volume)
}

fn geometry_test_close_grant(bundle:&ExtensionBundle,items:usize,copy:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    let copy=copy.max(bundle.retirement_demands(copy).unwrap().copy_bytes);let demand=bundle.retirement_demands(copy).unwrap();
    semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
fn close_geometry_test_bundle(bundle: &mut ExtensionBundle) {
    bundle.begin_close();
    for _ in 0..100000 {let grant=geometry_test_close_grant(bundle,1,64);let step=bundle.close_step(grant).unwrap();assert!(step.progress().unwrap().fits(grant));if matches!(step,PluginLifecycleStep::Complete(_)){assert!(bundle.terminal_is_empty());return;}}
    panic!("geometry inference owner failed to close");
}

#[test]
fn extension_guest_close_retires_only_its_existing_session_evaluation_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();let law=&fixture["ownerIsolation"];let mut request=fixture["evaluate"].clone();request["nodeHash"]=law["sharedNodeHash"].clone();request["budget"]=law["roundUnits"].clone();request["roundUnits"]=law["roundUnits"].clone();let request=serde_json::to_string(&request).unwrap();let compact=super::tests::compact_evaluation_request(&request);
    let mut retired=bundle();let mut live=bundle();assert_eq!(geometry_test_envelope(&retired,&request).get("done").and_then(semio_framework_pack_json::Value::as_bool),Some(false));assert_eq!(geometry_test_envelope(&live,&request).get("done").and_then(semio_framework_pack_json::Value::as_bool),Some(false));close_geometry_test_bundle(&mut retired);assert!(retired.terminal_is_empty());
    let mut answer=geometry_test_envelope(&live,&compact);let pending=answer.get("done").and_then(semio_framework_pack_json::Value::as_bool)==Some(false);if !pending {close_geometry_test_bundle(&mut live);}assert_eq!(pending,law["liveOwnerMustRemainPendingAfterOtherClose"].as_bool().unwrap());let mut turns=0;while answer.get("done").and_then(semio_framework_pack_json::Value::as_bool)==Some(false) {turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());answer=geometry_test_envelope(&live,&compact);}
    let signature=geometry_test_mesh_signature(geometry_test_mesh_pack(&live,&geometry_test_output(&answer)));let half=law["halfExtents"].as_array().unwrap();let oracle=parry3d::shape::Shape::mass_properties(&parry3d::shape::Cuboid::new(parry3d::math::Vector::new(half[0].as_f64().unwrap()as f32,half[1].as_f64().unwrap()as f32,half[2].as_f64().unwrap()as f32)),1.0).mass();assert!((signature.4.abs()-f64::from(oracle)).abs()<1e-8);close_geometry_test_bundle(&mut live);assert!(live.terminal_is_empty());println!("[DEBUG] Existing guest owners isolate same-node-hash cancellation and preserve live Parry volume={},turns={turns}",signature.4.abs());
}

#[test]
fn extension_guest_cancellation_preserves_the_original_u64_node_identity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();
    let law = &fixture["exactCancellation"];
    let identity = law["nodeHashLiteral"].as_str().unwrap().parse::<u64>().unwrap();
    let mut request = fixture["evaluate"].clone();
    request["nodeHash"] = serde_json::Value::from(identity);
    request["budget"] = law["roundUnits"].clone();
    request["roundUnits"] = law["roundUnits"].clone();
    let request = serde_json::to_string(&request).unwrap();
    let oracle: serde_json::Value = serde_json::from_str(&request).unwrap();
    assert_eq!(oracle["nodeHash"].as_u64(), Some(identity));
    let mut owner = bundle();
    let started = geometry_test_envelope(&owner, &request);
    let cancellation = serde_json::to_string(&serde_json::json!({"operatorId": oracle["operatorId"], "nodeHash": identity})).unwrap();
    let answer: serde_json::Value = serde_json::from_slice(&owner.invoke("evaluateCancel", cancellation.as_bytes()).unwrap()).unwrap();
    close_geometry_test_bundle(&mut owner);
    assert!(owner.terminal_is_empty());
    assert_eq!(started.get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(false));
    assert_eq!(answer["retired"], law["retired"]);
    assert_eq!(answer["pending"], law["pending"]);
    println!("[DEBUG] Existing guest cancellation preserves independent Serde u64 node identity={identity}");
}

#[test]
fn extension_guest_compact_hops_keep_the_original_distinct_execution_readers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();
    let law = &fixture["ownerIsolation"];
    let operator = fixture["evaluate"]["operatorId"].as_str().unwrap();
    let hash = law["sharedNodeHash"].as_u64().unwrap();
    let mut request = fixture["evaluate"].clone();
    request["nodeHash"] = serde_json::Value::from(hash);
    request["budget"] = law["roundUnits"].clone();
    request["roundUnits"] = law["roundUnits"].clone();
    let request = serde_json::to_string(&request).unwrap();
    let compact = super::tests::compact_evaluation_request(&request);
    let (mut first, first_reader) = scoped_bundle();
    let (mut second, second_reader) = scoped_bundle();
    let first_identity = first_reader.owner_identity();
    let second_identity = second_reader.owner_identity();
    assert_ne!(first_identity, second_identity);
    geometry_test_envelope(&first, &request);
    geometry_test_envelope(&second, &request);
    for _ in 0..law["originalReaderHops"].as_u64().unwrap() {
        assert!(flow_extension_sdk::evaluation_progress(&first_reader, operator, hash).is_some());
        assert!(flow_extension_sdk::evaluation_progress(&second_reader, operator, hash).is_some());
        assert_eq!(first_reader.owner_identity(), first_identity);
        assert_eq!(second_reader.owner_identity(), second_identity);
        assert_eq!(geometry_test_envelope(&first, &compact).get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(false));
        assert_eq!(geometry_test_envelope(&second, &compact).get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(false));
    }
    drop(first_reader);
    close_geometry_test_bundle(&mut first);
    assert!(flow_extension_sdk::evaluation_progress(&second_reader, operator, hash).is_some());
    assert_eq!(geometry_test_envelope(&second, &compact).get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(false));
    drop(second_reader);
    close_geometry_test_bundle(&mut second);
    assert!(first.terminal_is_empty() && second.terminal_is_empty());
    println!("[DEBUG] Two original guest execution readers retain distinct source identities through eight compact hops and isolated close");
}

#[test]
fn extension_guest_malformed_cancellation_preserves_the_original_live_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();
    let law = &fixture["exactCancellation"];
    let operator = fixture["evaluate"]["operatorId"].as_str().unwrap();
    let hash = fixture["ownerIsolation"]["sharedNodeHash"].as_u64().unwrap();
    let mut request = fixture["evaluate"].clone();
    request["nodeHash"] = serde_json::Value::from(hash);
    request["budget"] = law["roundUnits"].clone();
    request["roundUnits"] = law["roundUnits"].clone();
    let request = serde_json::to_string(&request).unwrap();
    for malformed in law["malformed"].as_array().unwrap() {
        let text = malformed.as_str().unwrap();
        let oracle: serde_json::Value = serde_json::from_str(text).unwrap();
        assert!(oracle["operatorId"].as_str().is_none() || oracle["nodeHash"].as_u64().is_none());
        let (mut owner, reader) = scoped_bundle();
        geometry_test_envelope(&owner, &request);
        let result = owner.invoke("evaluateCancel", text.as_bytes());
        let cancelled = flow_extension_sdk::evaluation_retirement_pending(&reader);
        let live = flow_extension_sdk::evaluation_progress(&reader, operator, hash).is_some();
        drop(reader);
        close_geometry_test_bundle(&mut owner);
        assert!(owner.terminal_is_empty());
        assert_eq!(result.err().map(|fault| fault.code.0).as_deref(), law["faultCode"].as_str());
        assert!(!cancelled && live);
    }
    println!("[DEBUG] Four independent Serde malformed identities refuse cancellation and preserve original live owners");
}

#[semio_framework_async_macros::async_test]
async fn named_geometry_inference_owns_worker_invocation_and_dependency_identity() {
    let _serial = super::tests::test_serial().await;
    let fixture = semio_framework_pack_json::parse(include_str!("../../💡️inferences/📐️geometry/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut bundle = bundle();
    let service = semio_framework_plugin::artifact_inference_service(super::geometry_inference::GEOMETRY_ARTIFACT_KIND, super::geometry_inference::GEOMETRY_INFERENCE_SCHEMA).unwrap().unwrap();
    assert!(service.metadata().payload.is_some());
    let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: 1048576, work_units: 1, recursion_depth: 64 };
    let context_required = semio_framework_plugin::ArtifactInferenceExecutionRequest { policy: &[], budgets: &budgets, retained:semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(fixture.get("request").unwrap().get("retained").unwrap()),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), cancellation_id: "context-proof", previous_state: None, requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold, canonical_payload: b"{}", dependencies: &[] };
    assert_eq!(service.infer(&context_required).err().expect("context refusal").code, "artifact-inference.context-required");
    let mut registry = semio_framework_plugin::ArtifactInferenceServiceRegistry::default();
    registry.register(service).unwrap();
    registry.register(service).unwrap();
    fn pure_owner(_request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> { unreachable!() }
    assert!(registry.register(semio_framework_plugin::ArtifactInferenceService::new(service.metadata(), pure_owner)).is_err());
    assert_eq!(bundle.manifest.contributions.len(), 1);
    let manifest = semio_framework_pack_json::parse(bundle.manifest.topic_contributions[0].payload.get("manifestJson").and_then(|value| value.as_str()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for operator in manifest.get("contributes").unwrap().get("operators").and_then(|value| value.as_array()).unwrap() {
        if !operator.get("id").and_then(|value| value.as_str()).is_some_and(|id| id.starts_with("brep.")) { continue; }
        let summary = operator.get("summary").and_then(|value| value.as_str()).unwrap();
        if !summary.contains("[quality:") {
            assert!(operator.get("group").and_then(|value| value.as_array()).unwrap().iter().any(|group| group.as_str() == Some("Schemas")));
            let schema = operator.get("id").and_then(|value| value.as_str()).unwrap().rsplit('.').next().unwrap();
            assert!(manifest.get("contributes").unwrap().get("schemas").and_then(|value| value.as_array()).unwrap().iter().any(|item| item.get("id").and_then(|value| value.as_str()) == Some(schema)));
        }
        assert!(!summary.contains("[quality:Unsupported]"));
    }
    let calls = super::geometry_inference::GEOMETRY_INFERENCE_CALLS.load(std::sync::atomic::Ordering::SeqCst);
    let request = semio_framework_pack_json::to_string(fixture.get("request").unwrap());
    let answer = geometry_test_envelope(&bundle, &request);
    assert_eq!(answer.get("done"), fixture.get("expected").unwrap().get("done"));
    assert_eq!(super::geometry_inference::GEOMETRY_INFERENCE_CALLS.load(std::sync::atomic::Ordering::SeqCst), calls + 1);
    let output = geometry_test_output(&answer);
    assert!(output.get("solid").is_some());
    let volume_input = semio_framework_pack_json::object([("geometry".into(), output.get("solid").unwrap().clone())]);
    let volume = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.measure.volume", volume_input, 0, 1, 1)));
    assert_eq!(volume.get("volume").unwrap().get("value").and_then(semio_framework_pack_json::Value::as_f64), fixture.get("expectedVolume").and_then(semio_framework_pack_json::Value::as_f64));
    let bare_input = semio_framework_pack_json::object([("geometry".into(), output.get("solid").unwrap().clone())]);
    let mut bare_request = semio_framework_pack_json::parse(&geometry_test_request("brep.measure.volume", bare_input, 0, 1, 1), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    bare_request.as_object_mut().unwrap().insert("dependencyJson", semio_framework_pack_json::Value::from(""));
    let refused = bundle.invoke("evaluate", semio_framework_pack_json::to_string(&bare_request).as_bytes()).err().expect("geometry source dependency refusal");
    assert_eq!(refused.code.0.as_str(), "geometry-inference.dependencies-required");
    let mut changed_request = fixture.get("request").unwrap().clone();
    changed_request.as_object_mut().unwrap().insert("inputJson", fixture.get("changedInput").unwrap().get("inputJson").unwrap().clone());
    let changed_output = geometry_test_output(&geometry_test_envelope(&bundle, &semio_framework_pack_json::to_string(&changed_request)));
    let changed_volume_input = semio_framework_pack_json::object([("geometry".into(), changed_output.get("solid").unwrap().clone())]);
    let changed_volume = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.measure.volume", changed_volume_input, 0, 1, 1)));
    assert_eq!(changed_volume.get("volume").unwrap().get("value").and_then(semio_framework_pack_json::Value::as_f64), fixture.get("changedVolume").and_then(semio_framework_pack_json::Value::as_f64));
    let parsed: flow_extension_sdk::EvaluateRequest = semio_framework_pack_json::from_json_str(&request, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let original = flow_extension_sdk::evaluation_dependency_identity(&parsed);
    let mut reordered: flow_extension_sdk::EvaluateRequest = semio_framework_pack_json::from_json_str(&request, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    reordered.input_json = "{\"height\":{\"value\":4,\"$schema\":\"number\"},\"depth\":{\"value\":3,\"$schema\":\"number\"},\"width\":{\"value\":2,\"$schema\":\"number\"}}".into();
    assert_eq!(original, flow_extension_sdk::evaluation_dependency_identity(&reordered));
    for field in ["inputJson", "dependencyJson", "operatorVersion"] {
        let mut changed = fixture.get("request").unwrap().clone();
        changed.as_object_mut().unwrap().insert(field, semio_framework_pack_json::Value::from("changed"));
        let changed: flow_extension_sdk::EvaluateRequest = semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(&changed), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_ne!(original, flow_extension_sdk::evaluation_dependency_identity(&changed));
    }
    close_geometry_test_bundle(&mut bundle);
}

#[test]
fn named_geometry_bypass_preserves_original_node_identity_and_round_grants() {
    use semio_framework_plugin::{WireArtifactInferenceRequest,WireArtifactInferenceResult,WireArtifactInferenceBudget,WireArtifactInferenceCacheMode};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();let law=&fixture["bypassIdentity"];let (mut owner,reader)=scoped_bundle();let identity=reader.owner_identity();let metadata=super::geometry_inference::geometry_inference_service().metadata();let mut receipts=Vec::new();
    for hash in law["nodeHashes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()) {
        let mut payload=serde_json::json!({"operatorId":fixture["evaluate"]["operatorId"],"inputJson":fixture["evaluate"]["inputJson"],"nodeHash":hash,"dependencyJson":"","operatorVersion":"bypass-v1","budget":law["roundUnits"],"roundUnits":law["roundUnits"],"wallMicros":1,"resume":false,"retained":fixture["evaluate"]["retained"]});
        for hop in 0..=law["compactHops"].as_u64().unwrap() {
            if hop>0 {payload["inputJson"]=serde_json::json!("");payload["resume"]=serde_json::json!(true);}
            let request=WireArtifactInferenceRequest {wire_version:semio_framework_plugin::ARTIFACT_INFERENCE_WIRE_VERSION,owner:metadata.owner.into(),artifact_kind:metadata.artifact_kind.into(),artifact_schema:metadata.artifact_schema.into(),artifact_schema_version:1,inference_schema:metadata.inference_schema.into(),inference_schema_version:1,algorithm_version:1,policy_version:1,revision:1,generation:1,source_dialect:"s.flow.flow.standard.v1.dialect.canonical".into(),policy:vec![],budgets:WireArtifactInferenceBudget {allocation_bytes:1048576,work_units:law["roundUnits"].as_u64().unwrap(),recursion_depth:64},retained:semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(&super::tests::geometry_test_retained()),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),cancellation_id:law["cancellationGroup"].as_str().unwrap().into(),previous_state:None,requested_cache_mode:WireArtifactInferenceCacheMode::Bypass,canonical_payload:serde_json::to_vec(&payload).unwrap(),dependencies:vec![]};
            let result:WireArtifactInferenceResult=semio_framework_pack_json::from_json_str(std::str::from_utf8(&owner.artifact_infer(semio_framework_pack_json::to_json_string(&request).as_bytes()).unwrap()).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let envelope:serde_json::Value=serde_json::from_slice(&result.canonical_payload).unwrap();let progress=flow_extension_sdk::evaluation_progress(&reader,fixture["evaluate"]["operatorId"].as_str().unwrap(),hash);receipts.push((result.complete,result.actual_cache_mode,envelope,progress.map(|value|value.units_done),reader.owner_identity()));
        }
    }
    drop(reader);close_geometry_test_bundle(&mut owner);assert!(owner.terminal_is_empty());
    for (index,(complete,mode,envelope,units,original)) in receipts.into_iter().enumerate() {let hop=index as u64%(law["compactHops"].as_u64().unwrap()+1);assert_eq!(complete,law["complete"].as_bool().unwrap());assert_eq!(mode,WireArtifactInferenceCacheMode::Bypass);assert_eq!(envelope["done"],law["complete"]);assert_eq!(units,Some((hop+1)as usize));assert_eq!(original,identity);}
    println!("[DEBUG] Original bypass node identities retain independent one-unit compact frontiers under the same cancellation group");
}

#[semio_framework_async_macros::async_test]
async fn named_geometry_inference_resumes_with_progress_matches_output_and_cancels() {
    let _serial = super::tests::test_serial().await;
    let (mut bundle,scope) = scoped_bundle();
    let parse = |text: &str| semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let box_input = parse("{\"width\":{\"$schema\":\"number\",\"value\":4},\"depth\":{\"$schema\":\"number\",\"value\":4},\"height\":{\"$schema\":\"number\",\"value\":2}}");
    let cylinder_input = parse("{\"radius\":{\"$schema\":\"number\",\"value\":0.5},\"height\":{\"$schema\":\"number\",\"value\":4}}");
    let box_geometry = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.prim3d.box", box_input, 0, 1, 1))).get("solid").unwrap().clone();
    let cylinder_geometry = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.prim3d.cylinder", cylinder_input, 0, 1, 1))).get("solid").unwrap().clone();
    let translate = |geometry, offset| semio_framework_pack_json::object([("geometry".into(), geometry), ("offset".into(), parse(offset))]);
    let a = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.xform.translate", translate(box_geometry, "{\"$schema\":\"vector\",\"x\":-2,\"y\":-2,\"z\":-0.5}"), 0, 1, 1))).get("geometryOut").unwrap().clone();
    let b = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.xform.translate", translate(cylinder_geometry, "{\"$schema\":\"vector\",\"x\":0,\"y\":0,\"z\":-2}"), 0, 1, 1))).get("geometryOut").unwrap().clone();
    let input = semio_framework_pack_json::object([("a".into(), a), ("b".into(), b)]);
    let request = geometry_test_request("brep.bool.cut", input.clone(), 0xfeedcafe, 1, 1);
    let compact_request=super::tests::compact_evaluation_request(&request);
    let mut units = 0.0;
    let mut hops = 0;
    let stepped = loop {
        let answer = geometry_test_envelope(&bundle, if hops==0 {&request}else{&compact_request});
        let next = answer.get("unitsDone").and_then(semio_framework_pack_json::Value::as_f64).unwrap();
        assert!(next >= units);
        units = next;
        hops += 1;
        if answer.get("done").and_then(semio_framework_pack_json::Value::as_bool) == Some(true) { break geometry_test_output(&answer); }
        assert_eq!(answer.get("cancellable").and_then(semio_framework_pack_json::Value::as_bool), Some(true));
        assert!(hops < 100000);
    };
    assert!(hops > 1);
    let ordinary = geometry_test_output(&geometry_test_envelope(&bundle, &geometry_test_request("brep.bool.cut", input.clone(), 0, 1000000, 3600000000)));
    assert!(stepped.get("error").is_none());
    let a=geometry_test_mesh_signature(geometry_test_mesh_pack(&bundle,&stepped));let b=geometry_test_mesh_signature(geometry_test_mesh_pack(&bundle,&ordinary));assert_eq!(a.0,b.0,"retained and ordinary corner positions/normals");assert_eq!(a.1,b.1,"retained and ordinary boundary geometry");assert_eq!(a.2,b.2);assert!((a.3-b.3).abs()<1e-5);assert!((a.4-b.4).abs()<1e-5);eprintln!("[DEBUG] Named retained/ordinary tessellation parity: triangles={} area={} signedVolume={} order-independent authored corners/edges",a.2,a.3,a.4);
    let pending = geometry_test_envelope(&bundle, &request);
    assert_eq!(pending.get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(false));
    let initial_units = pending.get("unitsDone").unwrap().clone();
    let _progressed = geometry_test_envelope(&bundle, &compact_request);
    let mut invalidated = parse(&request);
    invalidated.as_object_mut().unwrap().insert("operatorVersion", semio_framework_pack_json::Value::from("2"));
    let fixture=semio_framework_pack_json::parse(include_str!("../../💡️inferences/📐️geometry/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let replacement=fixture.get("replacement").unwrap();
    let invalidated=semio_framework_pack_json::to_string(&invalidated);
    let compact_invalidated=super::tests::compact_evaluation_request(&invalidated);
    let mut replacement_rounds=0;
    let restarted = loop {
        let answer=geometry_test_envelope(&bundle,if replacement_rounds==0 {&invalidated}else{&compact_invalidated});
        replacement_rounds+=1;
        assert!(replacement_rounds<100000);
        if replacement.get("pendingPhases").and_then(semio_framework_pack_json::Value::as_array).unwrap().iter().any(|phase|Some(phase)==answer.get("phase")) {
            assert_eq!(answer.get("done").and_then(semio_framework_pack_json::Value::as_bool),Some(false));
            assert_eq!(answer.get("outputJson"),replacement.get("outputJson"));
            continue;
        }
        break answer;
    };
    assert!(replacement_rounds>1);
    assert_eq!(restarted.get("unitsDone"), Some(&initial_units));
    let cancel = "{\"operatorId\":\"brep.bool.cut\",\"nodeHash\":4276996862}";
    let cancelled = semio_framework_pack_json::parse_bytes(&bundle.invoke("evaluateCancel", cancel.as_bytes()).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(cancelled.get("retired").and_then(semio_framework_pack_json::Value::as_f64), Some(1.0));
    let mut cancellation_rounds=0;while flow_extension_sdk::evaluation_retirement_pending(&scope){cancellation_rounds+=1;assert!(cancellation_rounds<100000);let reply=semio_framework_pack_json::parse_bytes(&bundle.invoke("evaluateCancel",cancel.as_bytes()).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(reply.get("ok").and_then(semio_framework_pack_json::Value::as_bool),Some(true));}
    assert!(flow_extension_sdk::evaluation_progress(&scope,"brep.bool.cut", 0xfeedcafe).is_none());
    let metadata = super::geometry_inference::geometry_inference_service().metadata();
    let named = semio_framework_plugin::WireArtifactInferenceRequest {
        wire_version: semio_framework_plugin::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: metadata.owner.into(), artifact_kind: metadata.artifact_kind.into(), artifact_schema: metadata.artifact_schema.into(), artifact_schema_version: 1,
        inference_schema: metadata.inference_schema.into(), inference_schema_version: 1, algorithm_version: 1, policy_version: 1,
        revision: 1, generation: 1, source_dialect: "s.flow.flow.standard.v1.dialect.canonical".into(), policy: vec![],
        budgets: semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: 1048576, work_units: 1, recursion_depth: 64 }, retained:semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(&super::tests::geometry_test_retained()),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), cancellation_id: "geometry-retained-cancel".into(),
        previous_state: None, requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
        canonical_payload: request.into_bytes(), dependencies: vec![],
    };
    assert!(semio_framework_plugin::install_extension_bundle(&mut Some(bundle)).await.unwrap());
    semio_framework_plugin::extension_activate().await.unwrap();
    let named_result = semio_framework_plugin::wire_artifact_infer(semio_framework_pack_json::to_json_string(&named).as_bytes()).await;
    let cancellation = semio_framework_plugin::cancel_artifact_inference(&named.cancellation_id);
    while flow_extension_sdk::evaluation_retirement_pending(&scope){let copy=flow_extension_sdk::evaluation_retirement_demands(&scope,8).unwrap().copy_bytes.max(8);let demand=flow_extension_sdk::evaluation_retirement_demands(&scope,copy).unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let step=flow_extension_sdk::retire_cancelled_evaluations_close_step(&scope,grant).unwrap();assert!(step.progress().fits(grant));}
    let retired = flow_extension_sdk::evaluation_progress(&scope,"brep.bool.cut", 0xfeedcafe).is_none();drop(scope);
    semio_framework_plugin::plugin_runtime::extension_dispose_cold().unwrap();
    let answer = named_result.unwrap();
    let answer: semio_framework_plugin::WireArtifactInferenceResult = semio_framework_pack_json::from_json_str(std::str::from_utf8(&answer).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(!answer.complete);
    assert_eq!(answer.quality, "ExactNumericalWithinTolerance");
    cancellation.unwrap();
    assert!(retired);
}


#[semio_framework_async_macros::async_test]
async fn named_geometry_inference_standard_gateway_uses_registered_extension_context() {
    use semio_framework_plugin::{WireArtifactInferenceRequest, WireArtifactInferenceResult, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};
    let _serial = super::tests::test_serial().await;
    let fixture = semio_framework_pack_json::parse(include_str!("../../💡️inferences/📐️geometry/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let bundle = bundle();
    let metadata = super::geometry_inference::geometry_inference_service().metadata();
    let request = WireArtifactInferenceRequest {
        wire_version: semio_framework_plugin::ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: metadata.owner.into(), artifact_kind: metadata.artifact_kind.into(), artifact_schema: metadata.artifact_schema.into(), artifact_schema_version: 1,
        inference_schema: metadata.inference_schema.into(), inference_schema_version: 1, algorithm_version: 1, policy_version: 1,
        revision: 1, generation: 1, source_dialect: "s.flow.flow.standard.v1.dialect.canonical".into(), policy: vec![],
        budgets: WireArtifactInferenceBudget { allocation_bytes: 1048576, work_units: 1, recursion_depth: 64 }, retained:semio_framework_pack_json::from_json_str(&semio_framework_pack_json::to_string(&super::tests::geometry_test_retained()),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), cancellation_id: "named-geometry-standard-gateway".into(),
        previous_state: None, requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: semio_framework_pack_json::to_string(fixture.get("request").unwrap()).into_bytes(), dependencies: vec![],
    };
    let wire = semio_framework_pack_json::to_json_string(&request);
    let answer = bundle.artifact_infer(wire.as_bytes()).unwrap();
    let result: WireArtifactInferenceResult = semio_framework_pack_json::from_json_str(std::str::from_utf8(&answer).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(result.inference_schema, metadata.inference_schema);
    assert!(result.complete);
    assert_eq!(result.actual_cache_mode, request.requested_cache_mode);
    assert_eq!(result.provenance.owner, metadata.owner);
    assert_eq!(result.quality, "ExactAnalytic");
    let envelope = semio_framework_pack_json::parse_bytes(&result.canonical_payload, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert!(geometry_test_output(&envelope).get("solid").is_some());
    let mut schema_request = request.clone();
    schema_request.cancellation_id = "named-schema-value".into();
    schema_request.canonical_payload = geometry_test_request("brep.text", semio_framework_pack_json::object([("value".into(), semio_framework_pack_json::object([("$schema".into(), semio_framework_pack_json::Value::from("text")), ("value".into(), semio_framework_pack_json::Value::from("Mesh Label"))]))]), 0, 1, 1).into_bytes();
    let schema_answer = bundle.artifact_infer(semio_framework_pack_json::to_json_string(&schema_request).as_bytes()).unwrap();
    let schema_result: WireArtifactInferenceResult = semio_framework_pack_json::from_json_str(std::str::from_utf8(&schema_answer).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(schema_result.quality, "SchemaValue");
    assert_eq!(schema_result.validity, "valid");
    assert!(schema_result.complete);
    let mesh_fixture = semio_framework_pack_json::parse(include_str!("../../🥽️mesh/🧫️fixtures/⏱️output-budget/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mesh = semio_framework_pack_json::to_string(mesh_fixture.get("cases").unwrap().as_array().unwrap()[0].get("mesh").unwrap());
    let mesh_input = neural_engine::ColdOwner::new(super::Dictionary::new().insert("data", super::Value::Dictionary(super::text_dictionary(mesh))));
    let mut mesh_request = request.clone();
    mesh_request.cancellation_id = "named-polygon-mesh".into();
    mesh_request.canonical_payload = geometry_test_request("brep.mesh.construct", semio_framework_pack_json::parse(&semio_framework_pack_json::to_json_string(&*mesh_input), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(), 0x6d03, 1, 1).into_bytes();
    let mut previous_units = 0;
    let mut mesh_trips = 0;
    loop {
        let answer = bundle.artifact_infer(semio_framework_pack_json::to_json_string(&mesh_request).as_bytes()).unwrap();
        let output: WireArtifactInferenceResult = semio_framework_pack_json::from_json_str(std::str::from_utf8(&answer).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(output.quality, "PolygonMesh");
        let envelope = semio_framework_pack_json::parse_bytes(&output.canonical_payload, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let units = envelope.get("unitsDone").and_then(|value| value.as_u64()).unwrap();
        assert!(units >= previous_units);
        previous_units = units;
        mesh_trips += 1;
        if output.complete { assert!(geometry_test_output(&envelope).get("meshOut").is_some()); break; }
        assert!(mesh_trips < 4096);
        mesh_request.requested_cache_mode = WireArtifactInferenceCacheMode::Incremental;
        mesh_request.previous_state = Some(output.canonical_payload);
    }
    assert!(mesh_trips > 1);
    assert!(semio_framework_plugin::install_extension_bundle(&mut Some(bundle)).await.unwrap());
    semio_framework_plugin::extension_activate().await.unwrap();
    let answer = semio_framework_plugin::wire_artifact_infer(wire.as_bytes()).await;
    let automatic = answer.as_ref().map(|bytes| semio_framework_pack_json::from_json_str::<WireArtifactInferenceResult>(std::str::from_utf8(bytes).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let volume_answer = match automatic.as_ref() {
        Ok(automatic) => {
            let envelope = semio_framework_pack_json::parse_bytes(&automatic.canonical_payload, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let input = semio_framework_pack_json::object([("geometry".into(), geometry_test_output(&envelope).get("solid").unwrap().clone())]);
            let mut volume_request = request.clone();
            volume_request.cancellation_id = "named-geometry-volume".into();
            volume_request.canonical_payload = geometry_test_request("brep.measure.volume", input, 0, 1, 1).into_bytes();
            Some(semio_framework_plugin::wire_artifact_infer(semio_framework_pack_json::to_json_string(&volume_request).as_bytes()).await)
        }
        Err(_) => None,
    };
    semio_framework_plugin::plugin_runtime::extension_dispose_cold().unwrap();
    let automatic = automatic.unwrap();
    assert!(automatic.complete);
    assert_eq!(automatic.quality, result.quality);
    let volume_answer = volume_answer.unwrap().unwrap();
    let volume_result: WireArtifactInferenceResult = semio_framework_pack_json::from_json_str(std::str::from_utf8(&volume_answer).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let volume_envelope = semio_framework_pack_json::parse_bytes(&volume_result.canonical_payload, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(geometry_test_output(&volume_envelope).get("volume").unwrap().get("value").and_then(semio_framework_pack_json::Value::as_f64), fixture.get("expectedVolume").and_then(semio_framework_pack_json::Value::as_f64));
}
