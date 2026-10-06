#[test]
fn playground_renderers_and_examples_select_actual_nx_targets() {
    let root = temp_root("playgrounds");
    fs::create_dir_all(crate::catalog::generated_dir(&root)).unwrap();
    fs::write(crate::catalog::generated_dir(&root).join("🎠️playgrounds.json"), "[]\n").unwrap();
    fs::write(crate::catalog::generated_dir(&root).join("🚀️playgrounds.json"), r#"[{"variant":"demo","pluginId":"plugin","cratePath":"plugin/rs","aliases":[],"ports":{"react":3100,"wgpu":3200},"examples":["🎬️first","🎬️second"]}]"#).unwrap();
    let tree = discover(&root);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗣️launch-axes/🔣️.json")).unwrap();
    for renderer in fixture["renderers"].as_array().unwrap() {
        for example in ["all", fixture["example"].as_str().unwrap()] {
            let mut selection = &tree;
            for key in ["dev", "plugin", "demo", renderer["id"].as_str().unwrap()] { selection = selection.children.iter().find(|child| child.key == key).unwrap(); }
            if example != "all" { selection = selection.children.iter().find(|child| child.key == "examples").unwrap(); }
            selection = selection.children.iter().find(|child| child.key == example).unwrap();
            assert!(selection.leaf.is_none(), "language authority cannot be implicit");
            let language = selection.children.iter().find(|child| child.key == "language").unwrap();
            assert_eq!(language.children.iter().map(|child|child.key.as_str()).collect::<Vec<_>>(),fixture["locales"].as_array().unwrap().iter().map(|locale|locale.as_str().unwrap()).collect::<Vec<_>>());
            assert!(language.children.iter().all(|child|child.leaf.is_none()));
            for locale in fixture["locales"].as_array().unwrap() {
                for terminology in fixture["terminologies"].as_array().unwrap() {
                    let mut node = selection;
                    for key in ["language", locale.as_str().unwrap(), "terminology", terminology.as_str().unwrap()] { node = node.children.iter().find(|child| child.key == key).unwrap_or_else(|| panic!("missing explicit {key}")); }
                    let Some(CommandLeaf::Process(spec)) = &node.leaf else { panic!("missing process") };
                    assert!(node.children.is_empty());
                    let mut args = vec!["nx".to_string(), "run".into(), format!("@semio-tech/framework-os-dev:{}", renderer["target"].as_str().unwrap())];
                    if renderer["id"] == "wgpu-native" && example != "all" { args.extend(["--".into(), "--example".into(), example.into()]); }
                    assert_eq!(spec.args, args);
                    assert!(spec.env.contains(&("SEMIO_LOCKED_LOCALE".into(), locale.as_str().unwrap().into())));
                    assert!(spec.env.contains(&("SEMIO_LOCKED_TERMINOLOGY".into(), terminology.as_str().unwrap().into())));
                    assert!(spec.env.contains(&("S_OS_PORT".into(), renderer["port"].as_str().unwrap().into())));
                    assert_eq!(spec.env.iter().find(|(key,_)| key=="PLAYGROUND_LOCKED_EXAMPLE_ID").map(|(_,value)|value.as_str()), (example!="all").then_some(example));
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}
