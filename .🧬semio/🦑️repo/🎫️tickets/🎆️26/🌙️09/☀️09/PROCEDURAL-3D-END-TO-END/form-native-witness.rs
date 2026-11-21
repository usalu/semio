use semio_ui_contract_builder::*;

#[test]
fn generated_form_uses_the_native_form_and_field_builders() {
    for title in ["Form", "Formular"] {
        let label = |text| Label(UiText::try_from_str(text).unwrap());
        let control = input(InputKind::Text).try_id("generate.form.caption.control").ok().unwrap()
            .try_label("Caption").ok().unwrap();
        let field = field(label("Caption")).try_id("generate.form.caption").ok().unwrap()
            .try_child(control).ok().unwrap();
        let fields = column().try_id("generate.form.fields").ok().unwrap()
            .try_child(field).ok().unwrap();
        let mut panel = form(label(title)).try_id("generate.form").ok().unwrap()
            .try_child(fields).ok().unwrap().try_build().unwrap();
        panel.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, ..Default::default() });
        let Component::Container(ref props) = panel.component else { panic!("form container") };
        assert_eq!(props.role, ContainerRole::Form);
        assert_eq!(props.label.as_ref().unwrap().0.as_str(), title);
        let field = &panel.children[0].children[0];
        let control = &field.children[0];
        assert_eq!(control.key.as_str().strip_suffix(".control"), Some(field.key.as_str()));
        assert_eq!(control.accessibility.label.as_ref().unwrap().0.as_str(), "Caption");
        assert!(matches!(panel.layout, LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, sizing: Sizing::Fill, .. })));
    }
}
