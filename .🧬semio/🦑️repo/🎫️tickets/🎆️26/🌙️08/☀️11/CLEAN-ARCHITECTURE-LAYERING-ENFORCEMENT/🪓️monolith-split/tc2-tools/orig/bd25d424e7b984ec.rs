//! 🔏️ Drawing retained mutation laws over the semantic digest and the workset admission.

use super::*;

#[test]
fn retained_drawing_schema_digest_distinguishes_every_nested_semantic_field() {
    let baseline = rich_layer();
    let baseline_digest = create_digest(baseline.clone());
    let mut variants = Vec::new();

    let modifiers: &[fn(&mut DrawingLayerNode)] = &[
        |value| if let DrawingLayerNode::Group(group)=value {group.isolation=true;},
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill_rule = semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).id = "different-id".into(),
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).name = "different-name".into(),
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).transform.x = 9.0,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).transform.y = 9.0,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).transform.scale_x = 9.0,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).transform.scale_y = 9.0,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill = None,
        |value| {
            if let Some(FillStyle::RadialGradient { cx, .. }) = &mut semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill {
                *cx = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { cy, .. }) = &mut semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill {
                *cy = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { r, .. }) = &mut semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill {
                *r = 9.0;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill {
                stops[0].offset = 0.75;
            }
        },
        |value| {
            if let Some(FillStyle::RadialGradient { stops, .. }) = &mut semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.fill {
                stops[0].color[2] = 0.9;
            }
        },
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke = None,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").color[0] = 0.9,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").width = 9.0,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").cap = semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Square,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").join = semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Round,
        |value| semio_s_artifact_draw_drawing_model::layer_base_mut(value).attributes.stroke.as_mut().expect("stroke").dash.as_mut().expect("dash")[0] = 9.0,
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.rect.as_mut().expect("rect").width = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.ellipse = Some(semio_s_artifact_draw_drawing_model::DrawingEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.circle = Some(semio_s_artifact_draw_drawing_model::DrawingCircle { cx: 1.0, cy: 2.0, r: 3.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.line = Some(semio_s_artifact_draw_drawing_model::DrawingLine { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0 });
            }
        },
        |value| {
            if let DrawingLayerNode::Shape(shape) = rich_child(value, 0) {
                shape.polygon = Some(semio_s_artifact_draw_drawing_model::DrawingPolygon { points: vec![[1.0, 2.0], [3.0, 4.0]].into() });
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[0] = PathSegment::Line { to: [1.0, 2.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[1] = PathSegment::Line { to: [9.0, 4.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[2] = PathSegment::Quad { ctrl: [9.0, 6.0], to: [7.0, 8.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Path(path) = rich_child(value, 1) {
                path.segments[3] = PathSegment::Cubic { ctrl1: [9.0, 10.0], ctrl2: [11.0, 12.0], to: [13.0, 20.0] };
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.x = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.y = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.content = "different text".into();
            }
        },
        |value| {
            if let DrawingLayerNode::Text(text) = rich_child(value, 2) {
                text.size = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Image(image) = rich_child(value, 3) {
                image.height = 9.0;
            }
        },
        |value| {
            if let DrawingLayerNode::Trace(trace) = rich_child(value, 5) {
                trace.params.simplify_epsilon = 9.0;
            }
        },
        |value| {
            let DrawingLayerNode::Group(group) = value else { unreachable!() };
            group.children.swap(0, 1);
        },
    ];
    for modifier in modifiers {
        let mut value = baseline.clone();
        modifier(&mut value);
        variants.push(value);
    }

    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).visible = true;
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).locked = false;
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).opacity = 0.5;
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).blend_mode = "screen".into();
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).transform.rotation = 0.75;
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).transform.shear = 1.25;
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::Solid { color: [0.9, 0.2, 0.3, 0.4] });
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).attributes.fill = Some(FillStyle::LinearGradient { x1: 1.0, y1: 2.0, x2: 3.0, y2: 4.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.8, 0.4] }].into() });
    variants.push(value);
    let mut value = baseline.clone();
    semio_s_artifact_draw_drawing_model::layer_base_mut(&mut value).attributes.stroke = Some(StrokeStyle { color: [0.9, 0.6, 0.7, 0.8], width: 3.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Square, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Round, dash: Some(vec![2.0, 3.0].into()) });
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Path(path) = &mut group.children[1] {
            path.segments[4] = PathSegment::Arc { rx: 15.0, ry: 16.0, rotation: 18.0, large_arc: false, sweep: true, to: [20.0, 19.0] };
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Image(image) = &mut group.children[3] {
            image.image_key = "other-asset-reference".into();
            image.width = 2.0;
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Boolean(boolean) = &mut group.children[4] {
            boolean.operation = "difference".into();
            boolean.children.swap(0, 1);
        }
    }
    variants.push(value);
    let mut value = baseline.clone();
    if let DrawingLayerNode::Group(group) = &mut value {
        if let DrawingLayerNode::Trace(trace) = &mut group.children[5] {
            trace.source_key = "other-trace-source".into();
            trace.params.threshold = 0.25;
        }
    }
    variants.push(value);

    for variant in variants {
        assert_ne!(create_digest(variant), baseline_digest, "every Drawing layer scalar, style, geometry, order, and asset reference changes the SHA-256 semantic authority");
    }

    let id = semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from("layer");
    let all_payloads = [
        DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: id.clone(), visible: false }),
        DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: id.clone(), locked: true }),
        DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: id.clone(), opacity: 0.25 }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: id.clone(), blend_mode: "screen".into() }),
        DrawingMutation::RenameLayer(RenameLayer { layer_id: id.clone(), new_name: "renamed".into() }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: id.clone(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: id.clone(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: id.clone(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: id.clone(), boolean_operation: "intersection".into() }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: id.clone(), params: semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(2), layer: Box::new(baseline.clone()) }),
        DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment{source:(id.clone()),target:"copied-layer".into()}].into(), layer_id: id.clone() }),
        DrawingMutation::DeleteLayer(DeleteLayer { layer_id: id.clone() }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: id, parent_id: Some("parent".into()), index: 3 }),
    ];
    let mut digests = std::collections::HashSet::new();
    for payload in all_payloads {
        assert!(digests.insert(digest(&payload).expect("all fourteen Drawing mutation payloads hash distinctly")));
        drain_mutation(payload);
    }
    assert_mutation_digest_distinct(DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: false }), DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: "layer".into(), visible: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: false }), DrawingMutation::SetLayerLocked(SetLayerLocked { layer_id: "layer".into(), locked: true }));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:"layer".into(),fill_rule:crate::FillRule::Evenodd}),DrawingMutation::SetLayerFillRule(SetLayerFillRule {layer_id:"layer".into(),fill_rule:crate::FillRule::Nonzero}));
    assert_mutation_digest_distinct(DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.25 }), DrawingMutation::SetLayerOpacity(SetLayerOpacity { layer_id: "layer".into(), opacity: 0.5 }));
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "multiply".into() }),
        DrawingMutation::SetLayerBlendMode(SetLayerBlendMode { layer_id: "layer".into(), blend_mode: "screen".into() }),
    );
    assert_mutation_digest_distinct(DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "left".into() }), DrawingMutation::RenameLayer(RenameLayer { layer_id: "layer".into(), new_name: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x: 1.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x: 6.0, y: 2.0, scale_x: 3.0, scale_y: 4.0, rotation: 5.0, shear: 0.0 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x:1.0,y:2.0,scale_x:3.0,scale_y:4.0,rotation:5.0,shear:0.0 } }),
        DrawingMutation::UpdateLayerTransform(UpdateLayerTransform { layer_id: "layer".into(), transform: semio_s_artifact_draw_drawing_model::DrawingTransform { x:1.0,y:2.0,scale_x:3.0,scale_y:4.0,rotation:5.0,shear:0.5 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: None }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 0.4] }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 0.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.5, color: [0.1, 0.2, 0.3, 0.4] }].into() }) }),
        DrawingMutation::ReplaceLayerFill(ReplaceLayerFill { layer_id: "layer".into(), fill: Some(FillStyle::LinearGradient { x1: 9.0, y1: 1.0, x2: 2.0, y2: 3.0, stops: vec![GradientStop { offset: 0.75, color: [0.1, 0.2, 0.8, 0.4] }].into() }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: None }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.1, 0.2, 0.3, 0.4], width: 1.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Round, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Bevel, dash: Some(vec![1.0].into()) }) }),
        DrawingMutation::ReplaceLayerStroke(ReplaceLayerStroke { layer_id: "layer".into(), stroke: Some(StrokeStyle { color: [0.9, 0.2, 0.3, 0.4], width: 2.0, cap: semio_s_artifact_draw_drawing_model::stroke::StrokeCap::Square, join: semio_s_artifact_draw_drawing_model::stroke::StrokeJoin::Round, dash: Some(vec![2.0].into()) }) }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "union".into() }),
        DrawingMutation::SetLayerBooleanOperation(SetLayerBooleanOperation { layer_id: "layer".into(), boolean_operation: "difference".into() }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: 0.25, simplify_epsilon: 0.5 } }),
        DrawingMutation::UpdateLayerTraceParams(UpdateLayerTraceParams { layer_id: "layer".into(), params: semio_s_artifact_draw_drawing_model::DrawingTraceParams { threshold: 0.75, simplify_epsilon: 1.5 } }),
    );
    assert_mutation_digest_distinct(
        DrawingMutation::CreateLayer(CreateLayer { parent_id: None, index: None, layer: Box::new(baseline.clone()) }),
        DrawingMutation::CreateLayer(CreateLayer { parent_id: Some("parent".into()), index: Some(1), layer: Box::new(baseline.clone()) }),
    );
    assert_mutation_digest_distinct(DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment{source:("left".into()),target:"copied-layer".into()}].into(), layer_id: "left".into() }), DrawingMutation::DuplicateLayer(DuplicateLayer { identities: vec![semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment{source:("right".into()),target:"copied-layer".into()}].into(), layer_id: "right".into() }));
    assert_mutation_digest_distinct(DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "left".into() }), DrawingMutation::DeleteLayer(DeleteLayer { layer_id: "right".into() }));
    assert_mutation_digest_distinct(
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: None, index: 0 }),
        DrawingMutation::ReorderLayer(ReorderLayer { layer_id: "layer".into(), parent_id: Some("parent".into()), index: 1 }),
    );
    drain_snapshot(DrawingSnapshot { layers: vec![baseline].into(), ..semio_s_artifact_draw_drawing_model::default_drawing_document("digest-owner", None) });
}

#[test]
fn retained_drawing_workset_admits_exact_field_page_and_keeps_document_mutation_arena_and_clone_bounds_independent() {
    let mut exact_source = nested_snapshot();
    let exact_owner = exact_native_layer_backing(&source.layers);
    let exact_target = match exact_source.layers.last().expect("Drawing exact-boundary group") {
        DrawingLayerNode::Group(group) => semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone(),
        _ => unreachable!("Drawing exact-boundary group remains exact"),
    };
    let exact = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES).into() });
    let workset = live_workset(&mut exact_source, &exact).expect("an exact field page obtains its independent fixed workset");
    let (configured_arena_items, configured_arena_bytes) = DrawingMutationArenaOwner::configured_totals().expect("the fixed arena has one derived owner claim");
    assert_eq!((workset.arena_items, workset.arena_bytes), (configured_arena_items, configured_arena_bytes), "one arena owner is charged exactly once");
    assert_eq!((workset.clone_items, workset.clone_bytes), (0, 0), "an overlay-only rename does not invent clone credit");
    assert_eq!(workset.workset_items().expect("workset item total"), workset.arena_items + workset.authority_items);
    assert_eq!(workset.workset_bytes().expect("workset byte total"), workset.arena_bytes + workset.authority_bytes);
    assert_eq!(
        workset.architectural_maximum_bytes().expect("derived architectural maximum"),
        DRAWING_MAXIMUM_NESTED_BYTES * 2 + DRAWING_OWNED_FIELD_BYTES + workset.arena_bytes + workset.authority_bytes,
        "the simultaneous maximum is derived from 2D + P + A + Q",
    );
    assert!(workset.simultaneous_bytes().expect("actual simultaneous ownership") <= workset.architectural_maximum_bytes().expect("derived architectural maximum"));
    let exact_source = apply(exact_source, &exact).expect("an exact 4096-byte retained overlay page is admitted");
    assert_eq!(exact_native_layer_backing(&source.layers), exact_owner, "exact boundary publication retains the source native container owner");
    drain_mutation(exact);
    drain_snapshot(exact_source);

    let plus_source = nested_snapshot();
    let plus_owner = native_layer_backing(&plus_source.layers);
    let plus_target = match plus_source.layers.last().expect("Drawing +1 group") {
        DrawingLayerNode::Group(group) => semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone(),
        _ => unreachable!("Drawing +1 group remains exact"),
    };
    let plus_one = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1).into() });
    let (plus_source, error) = apply(plus_source, &plus_one).expect_err("4096 +1 retained overlay page rejects");
    assert_eq!(error, "drawing-store.mutation-field-capacity");
    assert_eq!(native_layer_backing(&plus_source.layers), plus_owner, "+1 rejection returns the exact source owner without partial publication");
    drain_mutation(plus_one);
    drain_snapshot(plus_source);

    let source = document_over_byte_bound_snapshot();
    let target = semio_s_artifact_draw_drawing_model::layer_id(&source.layers[0]).clone();
    let source_owner = native_layer_backing(&source.layers);
    let mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: target, visible: false });
    let (source, error) = apply(source, &mutation).expect_err("a D + 1 source rejects independently of a small mutation");
    assert_eq!(error, "drawing-store.preflight-byte-capacity");
    assert_eq!(native_layer_backing(&source.layers), source_owner, "document rejection returns the exact source authority before overlay handoff");
    drain_mutation(mutation);
    drain_snapshot(source);
}
