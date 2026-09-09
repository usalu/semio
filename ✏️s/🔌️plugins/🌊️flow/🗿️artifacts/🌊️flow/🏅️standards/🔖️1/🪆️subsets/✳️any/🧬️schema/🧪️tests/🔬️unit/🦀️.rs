use super::*;

#[semio_framework_async_macros::async_test]
async fn widget_id_and_kind_label_agree_across_variants() {
    let widget = Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 3.0, min: 0.0, max: 10.0, step: 0.1 };
    assert_eq!(widget_id(&widget), "slider");
    assert_eq!(widget_kind_label(&widget), "inputSlider");
    assert_eq!(widget_tree_label(&widget), "slider (slider)");
}
