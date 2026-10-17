use super::*;

#[test]
fn neutral_affine_matrices_roundtrip_without_losing_shear_or_collapsed_axes() {
    let cases: serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧮️geometry/↗️affine/🔣️.json")).unwrap();
    for sample in cases.as_array().unwrap() {
        let matrix: [f64;6]=serde_json::from_value(sample["matrix"].clone()).unwrap();
        let actual=drawing_transform_to_matrix(&drawing_matrix_to_transform(matrix));
        let magnitude=matrix[..4].iter().fold(f64::MIN_POSITIVE,|max,value| max.max(value.abs()));
        for index in 0..6 { assert!((actual[index]-matrix[index]).abs()/magnitude.max(matrix[index].abs())<1e-12,"{}: {actual:?}",sample["name"]); }
    }
}


#[semio_framework_async_macros::async_test]
async fn affine_edit_preserves_scene_matrix_and_has_an_exact_inverse() {
    let mut document=crate::default_drawing_document("affine-edit",None);
    document.layers=vec![crate::create_drawing_shape_layer_rect(crate::identity::DrawingIdentity::admit((("Affine rectangle")).to_string().into()).expect("nonempty authored identity"), "Affine rectangle")].into();
    let layer=crate::layer_id(&document.layers[0]).to_string();
    let before=document.clone();
    let wanted=[-2.0,1.0,3.0,0.5,7.0,8.0];
    let mutation=semio_s_artifact_draw_drawing_transform::update_layer_transform::mutation::update_layer_transform(layer.into(),drawing_matrix_to_transform(wanted));
    let inverse=semio_s_artifact_draw_drawing_mutations::inverse_drawing_mutation(&document,&mutation).unwrap();
    semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut document,&mutation).unwrap();
    let scene=semio_s_artifact_draw_drawing_scene::flatten_drawing_document_to_scene_nodes(&document);
    for index in 0..6 { assert!((scene[0].transform[index]-wanted[index]).abs()<1e-12); }
    for mutation in inverse { semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut document,&mutation).unwrap(); }
    assert_eq!(document,before);
}
