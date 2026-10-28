use super::*;
#[test]
fn layout_inspection_summary_matches_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️summary.json")).expect("neutral inspector vectors");
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.name = fixture["name"].as_str().expect("document name").into();
    snapshot.pages.clear();
    assert_eq!(snapshot.pages.len(), fixture["pageCount"].as_u64().expect("page count") as usize);
    for row in fixture["cases"].as_array().expect("locales") {
        let config = LayoutWindowConfig { active_page_id: fixture["activePage"].as_str().expect("active page").into(), ..LayoutWindowConfig::default() };
        let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(row["locale"].as_str().expect("locale")).expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native) };
        let node = render(&snapshot, &config, &semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot::default(), crate::terminology::layout_labels(&view_state), &semio_framework_plugin::TreeWindows::for_body(&view_state, LAYOUT_PLAY_BODY_INSPECTION)).expect("semantic inspector");
        let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project and retire inspector");
        let actual: serde_json::Value = serde_json::from_str(&projection).expect("independent semantic JSON oracle");
        let section = &actual["children"][0];
        assert_eq!(section["component"]["label"], row["heading"]);
        let lines: Vec<_> = section["children"]
            .as_array()
            .expect("summary lines")
            .iter()
            .map(|child| {
                let label = child["component"]["label"].as_str().expect("summary row label");
                let description = child["component"]["description"].as_str().expect("summary row description");
                serde_json::Value::String(format!("{label}: {description}"))
            })
            .collect();
        assert_eq!(lines, *row["lines"].as_array().expect("expected lines"));
    }
}

#[test]
fn every_run_and_structural_operation_has_an_accessible_localized_control() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️controls.json")).expect("neutral controls");
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.data_fields = Some(semio_framework_pack_json::from_json_str(r#"{"entries":[{"questionId":"enabled","value":true}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed dictionary"));
    let frame_id = snapshot.pages[0].frames.iter().find_map(|frame| match frame { Frame::Text { id, .. } => Some(id.clone()), _ => None }).expect("text frame");
    let story_id = snapshot.pages[0].frames.iter().find_map(|frame| match frame { Frame::Text { story_id, .. } => Some(story_id.clone()), _ => None }).expect("story");
    let story = snapshot.stories.iter_mut().find(|story| story.id == story_id).expect("story");
    story.style_runs = (0..fixture["runCount"].as_u64().unwrap()).map(|_| semio_s_artifact_layout_layout_model::TextStyleRun { start: 0, end: 0, paragraph_style_id: None, character_style_id: None }).collect();
    for locale in fixture["locales"].as_array().unwrap() {
        let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::from_language_tag(locale.as_str().unwrap()).unwrap(), semio_framework_ui_locale::Terminology::Native);
        let labels = crate::terminology::layout_labels(&view);
        let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
        let mut projections = String::new();
        for id in [frame_id.clone(), "frame-1".into()] {
            let interaction = LayoutInteractionSnapshot { ids: vec![id], ..Default::default() };
            let node = render(&snapshot, &config, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), LAYOUT_PLAY_BODY_INSPECTION)).expect("inspector");
            projections.push_str(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project"));
        }
        assert!(!projections.contains("patchDocument.dataFields"), "data values use typed controls");
        for control in fixture["controls"].as_array().unwrap() {
            assert!(projections.contains(&format!("layout-play-inspector.{}", control.as_str().unwrap())), "missing {control} in {locale}");
        }
    }
}

#[test]
fn large_stories_render_as_complete_unicode_safe_windows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️capacity.json")).unwrap();
    for case in fixture["texts"].as_array().unwrap() {
        let content = case["unit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
        let ranges = text_chunks::ranges(&content);
        assert_eq!(ranges.iter().map(|range| &content[range.bytes.clone()]).collect::<String>(), content);
        assert!(ranges.iter().all(|range| range.bytes.len() <= 256));
        let actual: Vec<_> = ranges.iter().map(|range| serde_json::json!({ "start": range.start, "end": range.end, "byteStart": range.bytes.start, "byteEnd": range.bytes.end })).collect();
        assert_eq!(serde_json::to_value(actual).unwrap(), case["spans"]);
        let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
        snapshot.stories[0].content = content;
        let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let labels = crate::terminology::layout_labels(&view);
        let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
        for ids in [Vec::new(), vec!["frame-text-1".into()]] {
            let interaction = LayoutInteractionSnapshot { ids, ..Default::default() };
            let node = render(&snapshot, &config, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION)).expect("large text inspector");
            let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
            let _: serde_json::Value = serde_json::from_str(&json).unwrap();
        }
    }
}

#[test]
fn a_choice_after_the_fixed_select_limit_remains_reachable_and_selected() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️capacity.json")).unwrap();
    let count = fixture["choiceCount"].as_u64().unwrap() as usize;
    let offset = fixture["choiceOffset"].as_u64().unwrap() as u32;
    let rows = fixture["choiceRows"].as_u64().unwrap() as u32;
    let field = "run.0.paragraphStyle";
    let key = format!("layout-play-inspector.patchFrame.{field}");
    let choices: Vec<_> = (0..count).map(|index| format!("style-{index}")).collect();
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: LAYOUT_PLAY_BODY_INSPECTION.into(), node_key: key, open: Some(true), offset, rows }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let labels = crate::terminology::layout_labels(&view);
    let node = choice_row(&semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION), "patchFrame", field, labels.paragraph_style, "style-79", choices.iter().map(|choice| (choice.as_str(), choice.as_str())), Some("frame-text-1"), Some("page-1")).unwrap();
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(actual["component"]["window"]["total"], count);
    assert_eq!(actual["component"]["window"]["offset"], offset);
    assert!(json.contains("style-79"));
    assert!(json.contains("\"selected\":true"));
}

#[test]
fn every_large_relationship_collection_renders_without_fixed_select_admission() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️capacity.json")).unwrap();
    let count = fixture["choiceCount"].as_u64().unwrap() as usize;
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let paragraph = snapshot.paragraph_styles[0].clone();
    let parent = snapshot.parent_pages[0].clone();
    let spread = snapshot.spreads[0].clone();
    let story = snapshot.stories[0].clone();
    let layer = snapshot.pages[0].layers[0].clone();
    let frame = snapshot.pages[0].frames.iter().find(|frame| matches!(frame, Frame::Text { .. })).unwrap().clone();
    for index in 0..count {
        let mut row = paragraph.clone(); row.id = format!("style-{index}"); snapshot.paragraph_styles.push(row);
        snapshot.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle { id: format!("character-{index}"), name: Some(format!("Character {index}")), font_family: None, font_size: None, font_weight: None, italic: None, color: None, tracking: None });
        let mut row = parent.clone(); row.id = format!("capacity-parent-{index}"); snapshot.parent_pages.push(row);
        let mut row = spread.clone(); row.id = format!("capacity-spread-{index}"); snapshot.spreads.push(row);
        let mut row = story.clone(); row.id = format!("capacity-story-{index}"); snapshot.stories.push(row);
        let mut row = layer.clone(); row.id = format!("capacity-layer-{index}"); snapshot.pages[0].layers.push(row);
        let mut row = frame.clone(); if let Frame::Text { id, .. } = &mut row { *id = format!("capacity-text-{index}"); } snapshot.pages[0].frames.push(row);
    }
    snapshot.stories[0].style_runs.push(semio_s_artifact_layout_layout_model::TextStyleRun { start: 0, end: 0, paragraph_style_id: None, character_style_id: None });
    let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
    let interaction = LayoutInteractionSnapshot { ids: vec!["frame-text-1".into()], ..Default::default() };
    let node = render(&snapshot, &config, &interaction, crate::terminology::layout_labels(&view), &semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION)).expect("all relationships exceed 32");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
}

#[test]
fn inherited_configuration_is_an_explicit_shared_parent_editor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️capacity.json")).unwrap();
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let frame = snapshot.pages[0].frames.iter().find(|frame| matches!(frame, Frame::Text { .. })).unwrap().clone();
    snapshot.parent_pages[0].layers = snapshot.pages[0].layers.clone();
    snapshot.parent_pages[0].frames = vec![frame];
    snapshot.pages[0].parent_page_id = Some(snapshot.parent_pages[0].id.clone());
    snapshot.pages[0].frames.clear();
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        let view = semio_framework_plugin::ViewModel::new(locale, semio_framework_ui_locale::Terminology::Native);
        let labels = crate::terminology::layout_labels(&view);
        let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
        let interaction = LayoutInteractionSnapshot { ids: vec!["frame-text-1".into()], ..Default::default() };
        let node = render(&snapshot, &config, &interaction, labels, &semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION)).unwrap();
        let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
        assert!(json.contains(labels.shared_parent_frame.as_str()));
        for field in fixture["parentFields"].as_array().unwrap().iter().filter(|field| field.as_str() != Some("parent.fill")) { assert!(json.contains(field.as_str().unwrap())); }
        assert!(!json.contains("patchFrame.columns.input"));
    }
}

#[test]
fn selected_and_global_story_editors_have_distinct_surface_keys() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️concurrency.json")).unwrap();
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
    let interaction = LayoutInteractionSnapshot { ids: vec!["frame-text-1".into()], ..Default::default() };
    let node = render(&snapshot, &config, &interaction, crate::terminology::layout_labels(&view), &semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION)).unwrap();
    let mut keys = std::collections::HashSet::new();
    let mut pending = vec![&node];
    while let Some(row) = pending.pop() { assert!(keys.insert(row.key.as_str().to_string()), "duplicate UI key: {}", row.key.as_str()); pending.extend(row.children.iter()); }
    for location in fixture["locations"].as_array().unwrap() { assert!(keys.contains(location.as_str().unwrap()), "missing editor location"); }
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
}

#[test]
fn a_257_byte_dictionary_value_uses_typed_windowed_hex_inputs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️concurrency.json")).unwrap();
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.data_fields = Some(semio_framework_pack_json::from_json_str(r#"{"entries":[{"questionId":"bytes","value":null}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let byte_spans: Vec<_> = text_chunks::byte_ranges(fixture["byteValues"].as_array().unwrap().len()).iter().map(|span| serde_json::json!({ "start": span.start, "end": span.end })).collect();
    assert_eq!(serde_json::to_value(byte_spans).unwrap(), fixture["byteSpans"]);
    snapshot.data_fields.as_mut().unwrap().entries[0].value = semio_framework_value::DslValue::Bytes(fixture["byteValues"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect());
    let view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let config = LayoutWindowConfig { active_page_id: snapshot.pages[0].id.clone(), ..Default::default() };
    let node = render(&snapshot, &config, &LayoutInteractionSnapshot::default(), crate::terminology::layout_labels(&view), &semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION)).expect("typed byte inspector");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("data.0.value.bytes"));
}

#[test]
fn story_editor_locations_scroll_independently_over_the_same_semantic_field() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️concurrency.json")).unwrap();
    let locations: Vec<_> = fixture["locations"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
    let view = semio_framework_plugin::ViewModel { tree_windows: locations.iter().enumerate().map(|(index, location)| semio_framework_plugin::TreeWindowRequest { body_key: LAYOUT_PLAY_BODY_INSPECTION.into(), node_key: (*location).into(), open: Some(true), offset: (index * 2) as u32, rows: 1 }).collect(), ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let windows = semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION);
    let labels = crate::terminology::layout_labels(&view);
    let content = "a".repeat(600);
    let stamp = text_chunks::fingerprint(content.as_bytes());
    for (index, location) in locations.iter().enumerate() {
        let node = chunked_text(&windows, location, "story.story-1.content", &content, labels.story, &stamp).unwrap();
        let semio_framework_plugin::Component::TreeItem(props) = &node.component else { panic!("text window") };
        assert_eq!(props.window.as_ref().map(|window| (window.total, window.offset)), Some((3, (index * 2) as u32)));
        assert_eq!(node.children.len(), 1);
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    }
}

#[test]
fn the_nonfirst_typed_byte_window_remains_reachable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️concurrency.json")).unwrap();
    let bytes: Vec<_> = fixture["byteValues"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let location = "layout-play-inspector.dictionary.data.0.value.bytes";
    let view = semio_framework_plugin::ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: LAYOUT_PLAY_BODY_INSPECTION.into(), node_key: location.into(), open: Some(true), offset: 1, rows: 1 }], ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let node = chunked_bytes(&semio_framework_plugin::TreeWindows::for_body(&view, LAYOUT_PLAY_BODY_INSPECTION), location, "data.0.value", &bytes, crate::terminology::layout_labels(&view).data_bytes, "expected-stamp").unwrap();
    let semio_framework_plugin::Component::TreeItem(props) = &node.component else { panic!("byte window") };
    assert_eq!(props.window.as_ref().map(|window| (window.total, window.offset)), Some((3, 1)));
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    assert!(json.contains("data.0.value.bytes.128.256.expected-stamp"));
}

#[test]
fn a_long_story_materializes_only_its_requested_last_text_window(){let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🎮️commands/📄️patch-document/🧵️text-chunks/🔣️.json")).unwrap();let row=&law["cases"][2];let content=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let location="layout-play-inspector.long-story";let count=text_chunks::ranges(&content).len();let view=semio_framework_plugin::ViewModel{tree_windows:vec![semio_framework_plugin::TreeWindowRequest{body_key:LAYOUT_PLAY_BODY_INSPECTION.into(),node_key:location.into(),open:Some(true),offset:(count-1)as u32,rows:1}],..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De,semio_framework_ui_locale::Terminology::Native)};let labels=crate::terminology::layout_labels(&view);let node=chunked_text(&semio_framework_plugin::TreeWindows::for_body(&view,LAYOUT_PLAY_BODY_INSPECTION),location,"story.long.content",&content,labels.story,&text_chunks::fingerprint(content.as_bytes())).unwrap();assert_eq!(node.children.len(),1);let semio_framework_plugin::Component::TreeItem(props)=&node.component else{panic!("text window")};assert_eq!(props.window.as_ref().map(|window|(window.total,window.offset)),Some((count as u32,(count-1)as u32)));semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();}
