use super::*;

#[test]
fn every_catalogue_row_activates_the_exact_drag_descriptor() {
    let _serial = crate::test_serial::lock();
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let mut keys = std::collections::BTreeSet::new();
    for item in sections.iter().flat_map(|section| &section.items) {
        let node = catalogue_row(&Generation3dLabels::NATIVE_EN, item).expect("catalogue row");
        let Component::TreeItem(props) = &node.component else { panic!("catalogue tree row") };
        let target = props.target.as_ref().expect("activation target");
        assert_eq!(target.activation.as_ref().unwrap().as_str(), "addWidget");
        let activation: serde_json::Value = serde_json::to_value(target.args.as_ref().expect("descriptor args")).unwrap();
        let drag = &props.drag_data.as_ref().unwrap().iter().find(|(key, _)| key.as_str() == GENERATION_3D_WIDGET_DRAG_MIME).unwrap().1;
        let descriptor: serde_json::Value = serde_json::from_str(drag.as_str()).unwrap();
        assert_eq!(activation, descriptor, "{}", item.name);
        if let Some(format) = &item.format {
            assert!(props.label.0.as_str().contains(&format.to_uppercase()), "format is visible in the catalogue label");
        }
        let payload = crate::editor::generation3d::commands::add_widget::AddWidget::from_catalogue(item);
        let typed: serde_json::Value = serde_json::from_str(&payload.descriptor_json().expect("registered descriptor fits the command schema")).unwrap();
        assert_eq!(typed, descriptor);
        assert!(keys.insert(node.key.as_str().to_owned()), "catalogue row identity is unique");
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::app::built_to_component_tree(node)).expect("retire row");
    }
}
