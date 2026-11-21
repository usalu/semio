#[test]
fn nested_channel_pages_publish_only_requested_exact_text_with_the_original_hover_target() {
    let _arena = ui_arena_fixture::UiArenaFixture::new();
    let data = semio_framework_pack_json::parse(
        include_str!(PREVIEW_FIXTURE),
        semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap();
    let short = &data["cases"][1]["text"];
    let long_case = &data["cases"][3];
    let long = semio_framework_pack_json::object([("value".into(), long_case["text"].clone())]);
    fn row<'a>(id: &str, input: Option<&'a semio_framework_pack_json::Value>, output: Option<&'a semio_framework_pack_json::Value>) -> channel_values::ChannelValueRow<'a> {
        channel_values::ChannelValueRow { id: id.into(), label: id.into(), input: input.is_some(), value: input.or(output).unwrap() }
    }
    let rows = [row("source@out", None, Some(&long)), row("sink@in", Some(short), None)];
    let labels = channel_values::ChannelValueLabels::new(true, "Eingang", "Ausgang");
    let body = "preview";
    let request = |segments: &[&str], offset, rows| TreeWindowRequest {
        body_key: body.into(),
        node_key: segments.join("␟"),
        open: Some(true),
        offset,
        rows,
    };
    let project = |page: usize, out: usize, inside: usize| {
        let page_id = (page + 1).to_string();
        let view = ViewModel {
            tree_windows: [
                request(&["values", "source@out"], 0, 1),
                request(&["values", "source@out", "out"], 0, 1),
                request(&["values", "source@out", "out", "1"], if out < 4 { 0 } else { page as u32 }, if out < 4 { 3 } else { 1 }),
                request(&["values", "source@out", "out", "1", &page_id], 0, 1),
            ].into_iter().take(out).chain([
                request(&["values", "sink@in"], 0, 1),
                request(&["values", "sink@in", "in"], 0, 1),
            ].into_iter().take(inside)).collect(),
            ..ViewModel::new(Locale::De, Terminology::Native)
        };
        let placeholder = ui_build(text(ui_label("Scene").unwrap())).unwrap();
        channel_values::scene_with_channel_values(placeholder, &TreeWindows::for_body(&view, body), "procedural3d-view", "test-values", &labels, &rows, &[]).unwrap()
    };
    fn child<'a>(node: &'a BuiltNode, index: usize) -> &'a BuiltNode { &node.children[index] }
    fn read_surface(node: &BuiltNode) -> String {
        let Component::Surface(props) = &node.component else { panic!("text surface") };
        let mut scene: semio_ui_scene::TextEditorScene = semio_ui_scene::decode(props).unwrap();
        assert_eq!(scene.settings_json.as_deref(), Some(r#"{"readOnly":true}"#));
        fn collect(node: &BuiltNode, text: &mut String) {
            if let Component::Text(value) = &node.component { text.push_str(&value.packed_payload()); }
            for child in &node.children { collect(child, text); }
        }
        for carrier in &node.children {
            let mut text = String::new();
            collect(carrier, &mut text);
            assert!(semio_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), text));
        }
        assert_eq!(scene.lanes[0].bytes as usize, scene.buffer.len());
        scene.buffer
    }
    for index in [2, 0, 1] {
        let tree = project(index, 4, 2);
        assert_ne!(child(&tree, 0).key, child(&tree, 1).key);
        let values = child(child(child(&tree, 1), 0), 0);
        let port = child(values, 0);
        let output = child(port, 0);
        let value = child(output, 0);
        let page = child(value, 0);
        assert_eq!(value.children.len(), 1);
        assert_eq!(page.key.as_str(), (index + 1).to_string());
        let Component::TreeItem(props) = &value.component else { panic!("text pages") };
        let window = props.window.as_ref().unwrap();
        assert_eq!((window.total, window.offset), (3, index as u32));
        let hover = |node: &BuiltNode| serde_json::to_value(node.bindings.iter().next().unwrap()).unwrap();
        assert_eq!(hover(page), hover(port));
        let restored = read_surface(child(page, 0));
        assert_eq!(restored, long_case["pages"][index].as_str().unwrap());
        let input = child(child(values, 1), 0);
        assert_eq!(read_surface(child(input, 0)), short.as_str().unwrap());
    }
    let key_cases = semio_framework_pack_json::parse(
        include_str!(KEY_FIXTURE),
        semio_framework_pack_json::JsonMemberPolicy::Reject,
    ).unwrap();
    let key_cases = key_cases["cases"].as_array().unwrap();
    let dictionary = semio_framework_pack_json::object(key_cases.iter().map(|entry| {
        (entry["key"].as_str().unwrap().to_owned(), entry["value"].clone())
    }));
    let entries = [row("source@out", None, Some(&dictionary))];
    let project_dictionary = |tree_windows| {
        let view = ViewModel { tree_windows, ..ViewModel::new(Locale::De, Terminology::Native) };
        let placeholder = ui_build(text(ui_label("Scene").unwrap())).unwrap();
        channel_values::scene_with_channel_values(placeholder, &TreeWindows::for_body(&view, body), "procedural3d-view", "test-values", &labels, &entries, &[]).unwrap()
    };
    let mut requests = vec![
        request(&["values", "source@out"], 0, 1),
        request(&["values", "source@out", "out"], 4, 2),
    ];
    for id in ["5", "6"] {
        requests.extend([
            request(&["values", "source@out", "out", id], 0, 2),
            request(&["values", "source@out", "out", id, "key"], 2, 1),
            request(&["values", "source@out", "out", id, "key", "3"], 0, 1),
            request(&["values", "source@out", "out", id, "value"], 0, 1),
        ]);
    }
    let tree = project_dictionary(requests);
    let port = child(child(child(child(&tree, 1), 0), 0), 0);
    let output = child(port, 0);
    assert_eq!(output.children.len(), 2);
    let part = |node: &BuiltNode| {
        let Component::TreeItem(props) = &node.component else { panic!("dictionary entry") };
        serde_json::to_value(props).unwrap()
    };
    assert_eq!(part(child(output, 0))["label"], part(child(output, 1))["label"]);
    for (index, suffix) in ["#A", "#B"].into_iter().enumerate() {
        let member = child(output, index);
        assert_eq!(member.key.as_str(), (index + 5).to_string());
        let key = child(member, 0);
        let value = child(member, 1);
        assert_eq!(part(key)["label"], "Schlüssel");
        assert_eq!(part(value)["label"], "Wert");
        assert_eq!(key.children.len(), 1);
        let page = child(key, 0);
        assert_eq!(page.key.as_str(), "3");
        assert_eq!(read_surface(child(page, 0)), suffix);
        assert!(key_cases[index + 4]["key"].as_str().unwrap().ends_with(suffix));
        let expected = key_cases[index + 4]["value"]["shape"].to_string();
        assert_eq!(part(child(value, 0))["label"], "shape");
        assert_eq!(part(child(value, 0))["description"], expected);
        assert_eq!(serde_json::to_value(&page.bindings).unwrap(), serde_json::to_value(&port.bindings).unwrap());
    }
    drop(tree);
    if let Some(path) = std::env::var_os("SEMIO_CHANNEL_PREVIEW_BROWSER_TREE") {
        fn snapshot(node: &BuiltNode) -> serde_json::Value {
            serde_json::json!({
                "key": node.key.as_str(), "component": node.component,
                "layout": node.layout, "style": node.style, "accessibility": node.accessibility,
                "bindings": node.bindings, "disabled": node.disabled,
                "children": node.children.iter().map(snapshot).collect::<Vec<_>>(),
            })
        }
        let mut states = serde_json::Map::new();
        for (out, inside) in [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (0, 1), (0, 2)] {
            states.insert(format!("{out}.{inside}"), snapshot(&project(2, out, inside)));
        }
        for depth in 0..=5 {
            let tree_windows = [
                    request(&["values", "source@out"], 0, 1),
                    request(&["values", "source@out", "out"], 4, 1),
                    request(&["values", "source@out", "out", "5"], 0, 2),
                    request(&["values", "source@out", "out", "5", "key"], 2, 1),
                    request(&["values", "source@out", "out", "5", "key", "3"], 0, 1),
            ].into_iter().take(depth).collect();
            states.insert(format!("dictionary.{depth}.0"), snapshot(&project_dictionary(tree_windows)));
        }
        std::fs::write(path, serde_json::to_string(&states).unwrap()).unwrap();
    }
    for _ in 0..4096 {
        if built_node_pages_are_terminal_empty() { break; }
        close_built_node_page_one();
    }
    assert!(built_node_pages_are_terminal_empty());
}
