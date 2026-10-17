//! 🧪️ Shared image fixtures verify sparse deltas, history, and wire parity.
#[test]
fn image_edits_preserve_history_and_sparse_dimensions() {
    use protocol::Mutation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖼️update-image/🔣️.json")).unwrap();
    let mut layer=semio_s_artifact_draw_drawing_model::create_drawing_image_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image",fixture["before"]["imageKey"].as_str().unwrap());
    let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Image(image)=&mut layer else{panic!()};image.width=fixture["before"]["width"].as_f64().unwrap();image.height=fixture["before"]["height"].as_f64().unwrap();
    let id=semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let before=semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot{layers:vec![layer].into(),..Default::default()};
    let expected=&fixture["after"];
    let mutation=super::mutation::update_image(id.clone(),expected["imageKey"].as_str().unwrap().into(),expected["width"].as_f64().unwrap(),expected["height"].as_f64().unwrap());
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());
    let mut after=protocol::apply_diff(outcome.diff(),&before).unwrap();
    let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Image(image)=&after.layers[0] else{panic!()};
    assert_eq!(serde_json::json!({"imageKey":image.image_key.to_string_owner(),"width":image.width,"height":image.height}),*expected);
    for inverse in mutation.inverse(&before).unwrap(){semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after,&inverse).unwrap();}assert_eq!(after,before);
    let only_width=super::mutation::update_image(id.clone(),"before".into(),64.0,128.0).diff(&before);let patch=&only_width.diff().layers.as_ref().unwrap().modified[0].patch;
    assert_eq!(patch.image_width,Some(64.0));assert!(patch.image_key.is_none()&&patch.image_height.is_none());
    for invalid in fixture["invalid"].as_array().unwrap(){let mutation=super::mutation::update_image(id.clone(),invalid["imageKey"].as_str().unwrap().into(),invalid["width"].as_f64().unwrap(),invalid["height"].as_f64().unwrap());assert!(!mutation.diff(&before).messages().is_empty());}
}
