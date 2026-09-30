#[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let fixture = pack::json::parse(include_str!("../../../🧫️fixtures/🔣️.json")).unwrap();
        let mut bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("brep").and_then(|entry| entry.get("pluginId")).and_then(pack::json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("brep").and_then(|entry| entry.get("flowId")).and_then(pack::json::Value::as_str));
        }
        bundle.begin_close();
        for _ in 0..100000 {
            let bytes = 65536.max(bundle.next_close_byte_demand());
            let step = bundle.close_step(1, bytes).unwrap();
            if let PluginCloseStep::Pending { released_items,released_bytes } = step { assert!(released_items <= 1); assert!(released_bytes <= bytes); }
            if step == PluginCloseStep::Complete { assert!(bundle.terminal_is_empty()); return; }
        }
        panic!("BREP guest bundle resource retirement did not finish");
    }

    #[test]
    fn extension_guest_retires_actual_session_geometry_and_inflight_tessellation() {
        let fixture = pack::json::parse(include_str!("../../🧫️fixtures/🚪️retirement/🔣️.json")).unwrap();
        let grant = fixture.get("grant").and_then(pack::json::Value::as_array).unwrap();
        let items = grant[0].as_f64().unwrap() as usize;
        let bytes = grant[1].as_f64().unwrap() as usize;
        let mut bundle = bundle();
        let evaluate = pack::json::to_string(fixture.get("evaluate").unwrap());
        let answer = pack::json::parse_bytes(&bundle.invoke("evaluate", evaluate.as_bytes()).unwrap()).unwrap();
        assert_eq!(answer.get("done").and_then(pack::json::Value::as_bool), Some(true));
        let output = pack::json::parse(answer.get("outputJson").and_then(pack::json::Value::as_str).unwrap()).unwrap();
        let solid = output.get("solid").unwrap();
        assert_eq!(solid.get("$schema").and_then(pack::json::Value::as_str),fixture.get("geometrySchema").and_then(pack::json::Value::as_str));
        let handle = solid.get("handle").and_then(pack::json::Value::as_str).unwrap();
        let request = fixture.get("tessellate").unwrap();
        let tessellate = pack::json::to_string(&pack::json::object([
            ("handle".into(),pack::json::Value::from(handle)),
            ("tolerance".into(),request.get("tolerance").unwrap().clone()),
            ("budget".into(),request.get("budget").unwrap().clone()),
            ("wallMicros".into(),request.get("wallMicros").unwrap().clone()),
            ("chunk".into(),request.get("chunk").unwrap().clone()),
        ]));
        let mesh = pack::json::parse_bytes(&bundle.invoke("tessellate",tessellate.as_bytes()).unwrap()).unwrap();
        assert_eq!(mesh.get("done").and_then(pack::json::Value::as_bool),Some(false));
        assert!(mesh.get("unitsDone").and_then(pack::json::Value::as_f64).unwrap() > 0.0);
        assert_eq!(bundle.close_step(0,bytes).unwrap(),PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        bundle.begin_close();
        assert_eq!(bundle.invoke("evaluate",evaluate.as_bytes()).unwrap_err().code.0.as_str(),"extension.closing");
        bundle.cancel_close();
        assert!(matches!(bundle.close_step(items,bytes).unwrap(),PluginCloseStep::Blocked { .. }));
        bundle.resume_close();
        let mut released = 0;
        let mut released_bytes_total = 0;
        for _ in 0..fixture.get("steps").and_then(pack::json::Value::as_f64).unwrap() as usize {
            let supplied_bytes = bytes.max(bundle.next_close_byte_demand());
            match bundle.close_step(items,supplied_bytes).unwrap() {
                PluginCloseStep::Pending { released_items, released_bytes } => { assert!(released_items <= items); assert!(released_bytes <= supplied_bytes); released += released_items; released_bytes_total += released_bytes; }
                PluginCloseStep::Complete => { assert!(bundle.terminal_is_empty()); assert!(released > 4); assert!(released_bytes_total > 0); return; }
                step => panic!("unexpected BREP close status {step:?}"),
            }
        }
        panic!("actual BREP geometry and tessellation did not retire");
    }
