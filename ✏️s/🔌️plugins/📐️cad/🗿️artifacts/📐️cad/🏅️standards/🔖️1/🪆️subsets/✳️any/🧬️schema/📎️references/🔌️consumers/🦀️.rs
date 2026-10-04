//! 🔌️ Unmounted replacement inside the actual forest inference module.
fn forest_references_for_model_definitions(reference_z: f64) -> crate::CadReferenceIndex {
    let mut output = crate::CadReferenceIndex::new();
    for pane in CadPaneId::all() {
        output.insert(pane.model_definition_id().into(), vec![CadReference {
            id: "ref-concrete-forest".into(), source_url: CAD_CONCRETE_FOREST_REFERENCE_URL.into(),
            media_kind: "image".into(), origin: forest_reference_origin(reference_z),
            orientation: None, scale: None, width_world: CAD_FOREST_REFERENCE_WIDTH_WORLD,
            hidden: false, locked: true, opacity: Some(1.0),
        }]);
    }
    output
}
