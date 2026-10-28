/// 🧪️ Real job integration with independent image, text, transform and alpha observations.
#[test]
fn pdf_export_streams_large_transparent_images_and_keeps_positioned_unicode() {
    let case: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/🧫️fixtures/📕️pdf/integration.json")).unwrap();
    let width = case["image"]["width"].as_u64().unwrap() as u32;
    let height = case["image"]["height"].as_u64().unwrap() as u32;
    let pattern: Vec<[u8; 4]> = case["image"]["pattern"].as_array().unwrap().iter().map(|pixel| std::array::from_fn(|channel| pixel[channel].as_u64().unwrap() as u8)).collect();
    let pixels: Vec<_> = (0..width as usize * height as usize).flat_map(|at| pattern[at % pattern.len()]).collect();
    let image = semio_framework_pixels::RasterImage { width, height, pixels };
    let png = semio_framework_pixels::encode_png(&image).unwrap();
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.stories[0].content = case["text"].as_str().unwrap().into();
    snapshot.stories[0].style_runs.clear();
    snapshot.links[0].state = Some("ready".into());
    snapshot.links[0].width = width;
    snapshot.links[0].height = height;
    snapshot.links[0].proxy_data_url = Some(format!("data:image/png;base64,{}", base64_encode(&png)));
    let frame = snapshot.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").unwrap();
    let semio_s_artifact_layout_layout_model::Frame::Rect { bounds, fill, stroke, .. } = frame else { panic!("rectangle fixture") };
    bounds.rotation = case["rect"]["rotation"].as_f64().unwrap();
    *fill = Some(std::array::from_fn(|channel| case["rect"]["fill"][channel].as_f64().unwrap() as f32));
    *stroke = Some(std::array::from_fn(|channel| case["rect"]["stroke"][channel].as_f64().unwrap() as f32));
    let exported = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).unwrap();
    let bytes = decode_base64(&exported.data).unwrap();
    assert_eq!(assert_pdf_structure(&bytes), 6);
    let expected_rgb: Vec<_> = image.pixels.chunks_exact(4).flat_map(|pixel| pixel[..3].iter().copied()).collect();
    let expected_alpha: Vec<_> = image.pixels.chunks_exact(4).map(|pixel| pixel[3]).collect();
    assert_eq!(pdf_rgb_images(&bytes), vec![(expected_rgb, expected_alpha)]);
    assert!(pdf_text_compact(&bytes).contains("Grüße🌍"));
    assert!(pdf_semantic_styles(&bytes).iter().any(|(family, weight, italic)| family == &snapshot.paragraph_styles[0].font_family && *weight == i64::from(snapshot.paragraph_styles[0].font_weight) && !italic));
    for (key, operator) in [("move", "m"), ("line", "l"), ("fill", "rg"), ("stroke", "RG")] {
        let count = if matches!(key, "fill" | "stroke") { 3 } else { 2 };
        let operands: Vec<_> = case["rect"][key].as_array().unwrap().iter().take(count).map(|number| number.as_f64().unwrap()).collect();
        assert!(pdf_has_operation(&bytes, operator, &operands), "{key}");
    }
    let document = lopdf::Document::load_mem(&bytes).unwrap();
    assert!(document.objects.values().filter_map(|object| object.as_dict().ok()).filter_map(|dict| dict.get(b"Resources").ok()).filter_map(|object| object.as_dict().ok()).filter_map(|dict| dict.get(b"ExtGState").ok()).filter_map(|object| object.as_dict().ok()).any(|states| states.iter().any(|(_, object)| object.as_dict().is_ok_and(|state| state.get(b"ca").and_then(lopdf::Object::as_float).is_ok_and(|alpha| (alpha - 0.375).abs() < 0.0001) && state.get(b"CA").and_then(lopdf::Object::as_float).is_ok_and(|alpha| (alpha - 0.625).abs() < 0.0001)))));
}
