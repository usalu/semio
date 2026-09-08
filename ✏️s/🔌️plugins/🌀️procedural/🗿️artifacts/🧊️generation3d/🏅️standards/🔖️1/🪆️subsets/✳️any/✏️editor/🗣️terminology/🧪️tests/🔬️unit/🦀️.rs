
use super::*;

#[test]
fn labels_resolve_native_english_and_german_from_the_config_locale() {
    assert_eq!(generation3d_labels(&Generation3dConfig::default()).widgets.as_str(), "Widgets");
    assert_eq!(generation3d_labels(&Generation3dConfig { locale: "de-DE".into(), ..Generation3dConfig::default() }).widgets.as_str(), "Elemente");
}

#[test]
fn unknown_catalog_kind_falls_back_to_the_id_itself() {
    let labels = generation3d_labels(&Generation3dConfig::default());
    assert_eq!(generation3d_catalog_label("bogusKind", labels), "bogusKind");
}
