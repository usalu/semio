//! ✏️ Drawing retained mutation laws over path geometry, text and blend edits: appearance preservation, cancellation, digest distinction and sparse diffs.

use super::*;

#[test]
fn retained_path_geometry_mutation_preserves_appearance_and_retires_segments() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧪️tests/🧫️fixtures/🧬️mutations/✏️update-path-geometry/🔣️.json"))).unwrap();
    let before: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}> = serde_json::from_value(fixture["before"].clone()).unwrap();
    let after: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}> = serde_json::from_value(fixture["after"].clone()).unwrap();
    let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", before);
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).opacity = 0.4;
    let id = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let mutation = semio_s_artifact_draw_drawing_transform::update_path_geometry::mutation::update_path_geometry(id, after.clone());
    let result = apply(source, &mutation).expect("retained geometry mutation applies");
    let DrawingLayerNode::Path(path) = &result.layers[0] else { panic!("Expected path") };
    assert_eq!(path.segments, after);
    assert_eq!(path.base.opacity, 0.4);
    drain_snapshot(result);
    drain_mutation(mutation);
}

#[test]
fn retained_path_edit_cancellation_preserves_the_entire_document() {
    initialize_drawing_mutation_arena_pool_for_test();
    let layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Curve")).to_string().into()).expect("nonempty authored identity"), "Curve", vec![PathSegment::Move { to: [0.0,0.0] }, PathSegment::Line { to: [10.0,0.0] }].into());
    let id = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let mut source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let expected = source.clone();
    let mutation = semio_s_artifact_draw_drawing_transform::update_path_geometry::mutation::update_path_geometry(id, vec![PathSegment::Move { to: [5.0,5.0] }, PathSegment::Line { to: [20.0,20.0] }].into());
    let operation = semio_framework_job::OperationId(8_090);
    let generation = semio_framework_job::Generation(90);
    let mut authority = borrowed_candidate(operation,generation).unwrap();
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    for _ in 0..100_000 {
        if authority.segments_clone.as_ref().is_some_and(|clone| clone.index == 1) { break; }
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation,generation,semio_framework_job::StepBudget::new(1,u64::MAX,UNBOUNDED_STEP_GRANT),cancel.clone(),semio_framework_job::default_now_us,&mut sequence,&mut context_retained);
        assert!(!authority.step(&mut source,&mutation,&mut context).unwrap());
    }
    assert_eq!(authority.segments_clone.as_ref().unwrap().index,1);
    cancel.cancel_now();
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(operation,generation,semio_framework_job::StepBudget::new(1,u64::MAX,UNBOUNDED_STEP_GRANT),cancel,semio_framework_job::default_now_us,&mut sequence,&mut context_retained);
    assert_eq!(authority.step(&mut source,&mutation,&mut context),Err("drawing-store.mutation-candidate-cancelled"));
    close_candidate(&mut authority,Some(&mut source));
    assert_eq!(source,expected);
    drain_snapshot(source);
    drain_snapshot(expected);
    drain_mutation(mutation);
}

#[test]
fn retained_path_geometry_digest_distinguishes_control_points() {
    assert_mutation_digest_distinct(
        semio_s_artifact_draw_drawing_transform::update_path_geometry::mutation::update_path_geometry("path".into(),vec![PathSegment::Cubic { ctrl1: [1.0,2.0], ctrl2: [3.0,4.0], to: [5.0,6.0] }].into()),
        semio_s_artifact_draw_drawing_transform::update_path_geometry::mutation::update_path_geometry("path".into(),vec![PathSegment::Cubic { ctrl1: [2.0,2.0], ctrl2: [3.0,4.0], to: [5.0,6.0] }].into()),
    );
}

#[test]
fn retained_text_edit_preserves_identity_and_appearance() {
    initialize_drawing_mutation_arena_pool_for_test();
    let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Caption")).to_string().into()).expect("nonempty authored identity"), "Caption");
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer).opacity = 0.4;
    let id = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let mutation = semio_s_artifact_draw_drawing_style::update_text::mutation::update_text(id.clone(), "Grüße 🌍\n123".into(), 36.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta);
    let result = apply(source, &mutation).expect("retained text mutation applies");
    let DrawingLayerNode::Text(text) = &result.layers[0] else { panic!("Expected text") };
    assert_eq!(text.content, "Grüße 🌍\n123");
    assert_eq!(text.size, 36.0);
    assert_eq!(text.base.id, id);
    assert_eq!(text.base.opacity, 0.4);
    drain_snapshot(result);
    drain_mutation(mutation);
}

#[test]
fn retained_text_digest_distinguishes_content_and_size() {
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::update_text::mutation::update_text("text".into(), "A".into(), 24.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta), semio_s_artifact_draw_drawing_style::update_text::mutation::update_text("text".into(), "B".into(), 24.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::update_text::mutation::update_text("text".into(), "A".into(), 24.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta), semio_s_artifact_draw_drawing_style::update_text::mutation::update_text("text".into(), "A".into(), 36.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta));
}

#[test]
fn retained_text_edit_cancellation_keeps_the_complete_document() {
    initialize_drawing_mutation_arena_pool_for_test();
    let layer = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Caption")).to_string().into()).expect("nonempty authored identity"), "Caption");
    let id = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let mut source = DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    let expected = source.clone();
    let mutation = semio_s_artifact_draw_drawing_style::update_text::mutation::update_text(id, "😀🙂Grüße".into(), 36.0,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta);
    let operation = semio_framework_job::OperationId(8_091);
    let generation = semio_framework_job::Generation(91);
    let mut authority = borrowed_candidate(operation, generation).unwrap();
    let cancel = semio_framework_job::root_cancel_token();
    let mut sequence = 0;
    for _ in 0..100_000 {
        if authority.text_clone.as_ref().is_some_and(|clone| clone.index == 4) { break; }
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut context_retained);
        assert!(!authority.step(&mut source, &mutation, &mut context).unwrap());
    }
    assert_eq!(authority.text_clone.as_ref().unwrap().index, 4);
    cancel.cancel_now();
    let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel, semio_framework_job::default_now_us, &mut sequence, &mut context_retained);
    assert_eq!(authority.step(&mut source, &mutation, &mut context), Err("drawing-store.mutation-candidate-cancelled"));
    close_candidate(&mut authority, Some(&mut source));
    assert_eq!(source, expected);
    drain_snapshot(source);
    drain_snapshot(expected);
    drain_mutation(mutation);
}

#[test]
fn isolation_mutation_digest_observes_the_boolean() {
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation("group".into(),false),semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation("group".into(),true));
}

#[test]
fn retained_blend_mutations_validate_vocabulary_and_return_unchanged_rejections() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🎛️field-patch/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap().iter().filter(|case| case["patch"]["field"] == "blendMode") {
        let source = nested_snapshot();
        let id = semio_s_artifact_draw_drawing_model::layer_id(&source.layers[0]).to_owned();
        let mode = case["patch"]["value"].as_str().unwrap();
        let operation = semio_s_artifact_draw_drawing_style::set_layer_blend_mode::mutation::set_layer_blend_mode(id,mode.into());
        let result = apply(nested_snapshot(),&operation);
        if case["accepted"] == true {
            let actual = result.expect("canonical mode applies");
            let mut expected = source.clone();
            semio_s_artifact_draw_drawing_model::layer_base_mut(&mut expected.layers[0]).blend_mode = mode.into();
            assert_eq!(actual,expected);
            drain_snapshot(actual);drain_snapshot(expected);
        } else {
            let (actual,error) = result.expect_err("invalid mode is refused");
            assert_eq!(error,"drawing-store.mutation-blend-mode-invalid");
            assert_eq!(actual,source);
            drain_snapshot(actual);
        }
        drain_snapshot(source);drain_mutation(operation);
    }
}

#[test]
fn retained_authored_shape_and_image_facets_match_sparse_diffs_and_digest_all_arguments() {
    use semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField;
    let shape=semio_s_artifact_draw_drawing_model::create_drawing_shape_layer_rect(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Shape")).to_string().into()).expect("nonempty authored identity"), "Shape");let id=semio_s_artifact_draw_drawing_model::layer_id(&shape).clone();
    let before=DrawingSnapshot{layers:vec![shape].into(),..Default::default()};
    let value=apply(before,&semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate(id,ShapeCoordinateField::RectWidth,None,64.0)).unwrap();
    let DrawingLayerNode::Shape(shape)=&value.layers[0] else{panic!()};assert_eq!(shape.rect.as_ref().unwrap().width,64.0);drain_snapshot(value);
    let image=semio_s_artifact_draw_drawing_model::create_drawing_image_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Image")).to_string().into()).expect("nonempty authored identity"), "Image","before");let id=semio_s_artifact_draw_drawing_model::layer_id(&image).clone();
    let before=DrawingSnapshot{layers:vec![image].into(),..Default::default()};
    let value=apply(before,&semio_s_artifact_draw_drawing_style::update_image::mutation::update_image(id,"after".into(),64.0,32.0)).unwrap();
    let DrawingLayerNode::Image(image)=&value.layers[0] else{panic!()};assert_eq!(image.image_key,"after");assert_eq!((image.width,image.height),(64.0,32.0));drain_snapshot(value);
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"before".into(),64.0,32.0),semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"after".into(),64.0,32.0));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"key".into(),64.0,32.0),semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"key".into(),65.0,32.0));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"key".into(),64.0,32.0),semio_s_artifact_draw_drawing_style::update_image::mutation::update_image("image".into(),"key".into(),64.0,33.0));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,1.0),semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectY,None,1.0));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::PolygonX,Some(1),1.0),semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::PolygonX,Some(2),1.0));
    assert_mutation_digest_distinct(semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,1.0),semio_s_artifact_draw_drawing_transform::shape_coordinate::mutation::set_shape_coordinate("shape".into(),ShapeCoordinateField::RectX,None,2.0));
}
