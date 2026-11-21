#[test]
fn published_form_buttons_and_page_requests_have_one_address() {
    let _arena = ui_arena_fixture::UiArenaFixture::new();
    let data: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    let body = data["body"].as_str().unwrap();
    let left = data["windows"][0].as_str().unwrap();
    let right = data["windows"][1].as_str().unwrap();
    let first = data["first"].as_u64().unwrap() as usize;
    let second = data["second"].as_u64().unwrap() as usize;
    let field = || ui_build(ui_id(text(ui_label("Field").unwrap()), "generate.form.one").unwrap()).unwrap();
    let rendered = |index, locale| {
        let mut node = ui_build(ui_child(ui_id(column(), "generate.form").unwrap(), field()).unwrap()).unwrap();
        node.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
        let Component::Container(props) = &mut node.component else { panic!("form") };
        props.role = ContainerRole::Form;
        GenerationFormPage { body: node, index, total: 2 }.present(left, locale).unwrap()
    };
    let first_page = rendered(first, Locale::En);
    let second_page = rendered(second, Locale::De);
    assert_eq!(first_page.children[0].children.len(), 3);
    assert!(first_page.children[0].children[0].disabled);
    assert!(second_page.children[0].children[2].disabled);
    assert_eq!(second_page.children[0].children[1].accessibility.live, Liveness::Polite);
    assert!(matches!(&second_page.children[1].layout, LayoutSpec::Scroll(_)));
    assert_ne!(first_page.children[1].key, second_page.children[1].key);
    let control = &first_page.children[0].children[2];
    let descriptor = serde_json::to_value(control.bindings.iter().next().unwrap()).unwrap();
    assert_eq!(descriptor["action"]["scope"], data["controller"]);
    assert_eq!(descriptor["action"]["name"], data["action"]);
    assert_eq!(descriptor["args"]["windowId"], left);
    assert_eq!(descriptor["args"]["page"].as_f64(), Some(second as f64));

    let mut scheduler = TreeWindowScheduler::default();
    scheduler.set_form_page(body, left, second as u32);
    assert_eq!(scheduler.due(0), vec![body.to_owned()]);
    scheduler.report(body, &[MeasuredWindow {key:"measured-list".into(), offset:8, rows:4}], 20, 1);
    scheduler.set_form_page(body, right, data["later"].as_u64().unwrap() as u32);
    assert!(scheduler.due(1).is_empty());
    scheduler.settled(body);
    assert_eq!(scheduler.due(1), vec![body.to_owned()]);
    let mut view = ViewModel::new(Locale::En, Terminology::Native);
    view.tree_windows = scheduler.view_state_fields().0;
    assert_eq!(GenerationFormPage::requested(&view, body, left), second);
    assert_eq!(GenerationFormPage::requested(&view, body, right), data["later"].as_u64().unwrap() as usize);
    assert_eq!(GenerationFormPage::requested(&view, "another-pane", left), first);
    assert!(view.tree_windows.iter().any(|request| request.node_key == "measured-list" && request.offset == 8));
    scheduler.set_form_page(body, left, first as u32);
    view.tree_windows = scheduler.view_state_fields().0;
    assert_eq!(GenerationFormPage::requested(&view, body, left), first);
    drop(first_page);
    drop(second_page);
    for _ in 0..4096 {
        if built_node_pages_are_terminal_empty() {
            break;
        }
        close_built_node_page_one();
    }
    assert!(built_node_pages_are_terminal_empty());
}
