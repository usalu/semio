#[allow(unused_imports)] use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)] use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::patch_document::*;
use semio_s_artifact_layout_layout_editor_commands::{patch_frame, patch_page};
use semio_s_artifact_layout_layout_editor::unit_tests::context::{dispatch, layout_app};
use semio_s_artifact_layout_layout_editor::LayoutCommand;

#[semio_framework_async_macros::async_test]
async fn patch_document_renames_sets_print_target_and_clears_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "name".into(), value: "Press sheet".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "cmyk".into() })).await;
    for (field, value) in [("addDataField", "true"), ("data.0.key", "title"), ("data.0.value", "A")] {
        dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: field.into(), value: value.into() })).await;
    }
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.name, "Press sheet");
    assert_eq!(snapshot.print_target.as_deref(), Some("cmyk"));
    assert_eq!(snapshot.data_fields,Some(semio_framework_value_dictionary::QuestionDictionary{entries:vec![semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"title".into(),value:semio_framework_value::DslValue::String("A".into())}]}));
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "  ".into() })).await;
    assert_eq!(app.snapshot().expect("projection").print_target, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_updates_the_baseline_grid() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "baselineGrid".into(), value: "18".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "baselineOffset".into(), value: "4".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "snapToBaseline".into(), value: "false".into() })).await;
    let grid = app.snapshot().expect("projection").grid;
    assert_eq!(grid.baseline_grid, 18.0);
    assert_eq!(grid.baseline_offset, 4.0);
    assert!(!grid.snap_to_baseline);
}

#[semio_framework_async_macros::async_test]
async fn patch_page_reorders_and_deletes_and_patch_frame_sets_flags() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "moveLater".into(), value: "true".into() })).await;
    let ids: Vec<String> = app.snapshot().expect("projection").pages.iter().map(|page| page.id.clone()).collect();
    assert_eq!(ids, vec!["page-2".to_string(), "page-1".to_string()]);
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "locked".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "visible".into(), value: "false".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let frame = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-1").unwrap();
    assert!(frame.locked());
    assert!(!frame.visible());
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-2".into()), field: "delete".into(), value: "true".into() })).await;
    let left: Vec<String> = app.snapshot().expect("projection").pages.iter().map(|page| page.id.clone()).collect();
    assert_eq!(left, vec!["page-1".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_updates_a_paragraph_style() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "paragraph.body.fontSize".into(), value: "18".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "paragraph.body.alignment".into(), value: "center".into() })).await;
    let style = app.snapshot().expect("projection").paragraph_styles.into_iter().find(|style| style.id == "paragraph.body").unwrap();
    assert_eq!(style.font_size, 18.0);
    assert_eq!(style.alignment, "center");
    assert_eq!(style.name, "Body");
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_sets_text_inset_and_patch_page_updates_a_layer() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "insetX".into(), value: "8".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "layer-1.name".into(), value: "Art".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "layer-1.locked".into(), value: "true".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let frame = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap();
    let semio_s_artifact_layout_layout_model::Frame::Text { inset, .. } = frame else { panic!("text frame") };
    assert_eq!(inset.x, 8.0);
    let layer = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().layers.iter().find(|layer| layer.id == "layer-1").unwrap();
    assert_eq!(layer.name, "Art");
    assert!(layer.locked);
    assert!(layer.visible);
}

#[semio_framework_async_macros::async_test]
async fn patch_document_adds_a_character_style_and_renames_the_spread() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "addCharacterStyle".into(), value: "Emphasis".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "character-1.italic".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "spread-1.name".into(), value: "Opening".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "parentPageId".into(), value: "".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.character_styles[0].name.as_deref(), Some("Emphasis"));
    assert_eq!(snapshot.character_styles[0].italic, Some(true));
    assert_eq!(snapshot.spreads[0].name, "Opening");
    assert_eq!(snapshot.pages[0].parent_page_id, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_page_adds_a_guide_and_moves_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "addGuide".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "guide.0.y".into(), value: "48".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let guide = &snapshot.pages.iter().find(|page| page.id == "page-1").unwrap().guides[0];
    assert_eq!(guide.y, 48.0);
    assert_eq!(guide.height, 0.0);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_applies_a_character_style_to_the_story() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "addCharacterStyle".into(), value: "Emphasis".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "character-1.fontSize".into(), value: "24".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "clearStyleRuns".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "addStyleRun".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "run.0.characterStyle".into(), value: "character-1".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let run = &snapshot.stories.iter().find(|story| story.id == "story-1").unwrap().style_runs[0];
    assert_eq!(run.character_style_id.as_deref(), Some("character-1"));
    assert_eq!(run.start, 0);
    assert_eq!(run.end, u64::try_from(snapshot.stories[0].content.len()).unwrap());
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "run.0.characterStyle".into(), value: "".into() })).await;
    assert_eq!(app.snapshot().expect("projection").stories[0].style_runs[0].character_style_id, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_limits_a_character_style_to_a_span() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "addCharacterStyle".into(), value: "Emphasis".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "character-1.fontSize".into(), value: "24".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "clearStyleRuns".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "addStyleRun".into(), value: "true".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "run.0.characterStyle".into(), value: "character-1".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "run.0.end".into(), value: "5".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let story = snapshot.stories.iter().find(|story| story.id == "story-1").unwrap();
    let run = &story.style_runs[0];
    assert_eq!(run.start, 0);
    assert_eq!(run.end, u64::try_from(story.content.char_indices().nth(5).unwrap().0).unwrap());
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &snapshot, &snapshot.pages[0], "", &[], None, false);
    let glyphs = &list.text_runs.iter().find(|run| run.object_id == "frame-text-1").expect("story").glyphs;
    assert!(glyphs.iter().any(|glyph| (glyph.font_size - 24.0).abs() < 0.1), "{glyphs:?}");
    assert!(glyphs.iter().any(|glyph| (glyph.font_size - 12.0).abs() < 0.1), "{glyphs:?}");
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_sets_link_resolution_and_color_profile() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "dpi".into(), value: "150".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "colorProfile".into(), value: "CMYK".into() })).await;
    let link = &app.snapshot().expect("projection").links[0];
    assert_eq!(link.dpi, 150);
    assert_eq!(link.color_profile.as_deref(), Some("CMYK"));
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-image-1".into(), page_id: Some("page-1".into()), field: "colorProfile".into(), value: "".into() })).await;
    assert_eq!(app.snapshot().expect("projection").links[0].color_profile, None);
}

#[semio_framework_async_macros::async_test]
async fn patch_frame_overrides_an_inherited_frame() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-inherited".into(), page_id: Some("page-1".into()), field: "x".into(), value: "90".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.pages[0].overrides[0].bounds.as_ref().unwrap().x, 90.0);
    assert_eq!(snapshot.parent_pages[0].frames[0].bounds().x, 50.0);
}

#[semio_framework_async_macros::async_test]
async fn patch_page_adds_a_layer_and_patch_frame_moves_onto_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "addLayer".into(), value: "Notes".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "layerId".into(), value: "layer-2".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    let page = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap();
    assert_eq!(page.layers.last().unwrap().name, "Notes");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-1").unwrap().layer_id(), "layer-2");
    assert!(page.layers.iter().find(|layer| layer.id == "layer-1").unwrap().object_ids.iter().all(|id| id != "frame-1"));
}

#[semio_framework_async_macros::async_test]
async fn typed_editing_vectors_reach_native_layout_state() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️editing.json")).expect("neutral editing vectors");
    let mut app = layout_app().await;
    for step in fixture["steps"].as_array().expect("steps") {
        let field = step[1].as_str().unwrap().to_string();
        let value = step[2].as_str().unwrap().to_string();
        let command = match step[0].as_str().unwrap() {
            "document" => LayoutCommand::PatchDocument(PatchDocument { field, value }),
            "page" => LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field, value }),
            kind => LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: if kind == "text" { "frame-text-1".into() } else { "frame-1".into() }, page_id: Some("page-1".into()), field, value }),
        };
        dispatch(&mut app, command).await;
    }
    let snapshot = app.snapshot().expect("projection");
    let page = snapshot.pages.iter().find(|page| page.id == "page-1").unwrap();
    let semio_s_artifact_layout_layout_model::Frame::Rect { fill, .. } = page.frames.iter().find(|frame| frame.id() == "frame-1").unwrap() else { panic!("rect") };
    let story = snapshot.stories.iter().find(|story| story.id == "story-1").unwrap();
    let run = &story.style_runs[5];
    let dictionary = snapshot.data_fields.as_ref().unwrap();
    let semio_framework_value::DslValue::Object(members) = &dictionary.entries[0].value else { panic!("object") };
    let actual = serde_json::json!({ "runStart": run.start, "runEnd": run.end, "runCount": story.style_runs.len(), "fillAlpha": fill.unwrap()[3], "layerFirst": page.layers[0].id, "spreadId": page.spread_id, "dataKey": dictionary.entries[0].question_id, "dataNestedKey": members[0].0, "dataValue": matches!(members[0].1, semio_framework_value::DslValue::Bool(true)), "paragraphStyle": run.paragraph_style_id, "characterStyle": run.character_style_id });
    assert_eq!(actual, fixture["expected"]);
}

#[test]
fn inherited_frames_dispatch_shared_content_and_explicit_parent_edits() {
    use semio_s_artifact_layout_layout_model::Frame;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let text = snapshot.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap().clone();
    let image = snapshot.pages[0].frames.iter().find(|frame| frame.id() == "frame-image-1").unwrap().clone();
    snapshot.parent_pages[0].layers = snapshot.pages[0].layers.clone();
    snapshot.parent_pages[0].frames = vec![text, image];
    snapshot.pages[0].parent_page_id = Some(snapshot.parent_pages[0].id.clone());
    snapshot.pages[0].frames.clear();
    snapshot.links[0].artifact_kind = "s.stdio.pdf".into();
    snapshot.links[0].artifact_ref = "source-pdf".into();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = |frame_id: &str, field: &str, value: &str| patch_frame::handle(&patch_frame::PatchFrame { frame_id: frame_id.into(), page_id: Some(snapshot.pages[0].id.clone()), field: field.into(), value: value.into() }, &doc, &cfg).unwrap();
    assert!(matches!(emit("frame-text-1", "storyContent", "Shared text").artifact_mutations[0], LayoutMutation::EditStory(_)));
    assert!(matches!(emit("frame-text-1", "addStyleRun", "true").artifact_mutations[0], LayoutMutation::SetStoryRuns(_)));
    assert!(!emit("frame-image-1", "open", "true").effects.is_empty());
    let parent = emit("frame-text-1", "parent.columns", "2");
    let LayoutMutation::SetParentPageContent(content) = &parent.artifact_mutations[0] else { panic!("parent mutation") };
    assert!(matches!(&content.frames[0], Frame::Text { columns: 2, .. }));
}

#[test]
fn bounded_story_commits_preserve_text_outside_the_addressed_unicode_span() {
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    for content in ["a".repeat(513), "😀é".repeat(300)] {
        let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
        snapshot.stories[0].content = content.clone();
        let story_id = snapshot.stories[0].id.clone();
        let ranges = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::ranges(&content);
        let span=ranges.get(1).unwrap();
        let config = NoConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg = ConfigView { snapshot: &config, window: None };
        let stamp = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::fingerprint(content.as_bytes());
        let payload = PatchDocument { field: format!("story.{story_id}.content.chunk.{}.{}.{stamp}", span.start, span.end), value: "Edited 😀".into() };
        let emit = semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &doc, &cfg).unwrap();
        let LayoutMutation::EditStory(edit) = &emit.artifact_mutations[0] else { panic!("native text splice") };
        let actual: String = content.chars().take(edit.offset).chain(edit.insert.chars()).chain(content.chars().skip(edit.offset + edit.delete)).collect();
        let expected = format!("{}{}{}", &content[..span.start], "Edited 😀", &content[span.end..]);
        assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(expected).unwrap());
    }
}

#[test]
fn queued_text_spans_reject_content_that_changed_before_their_address() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️concurrency.json")).unwrap();
    for case in fixture["texts"].as_array().unwrap() {
        let content = case["unit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
        let original = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::fingerprint(content.as_bytes());
        let span=semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::ranges(&content).get(1).unwrap();
        let field = format!("chunk.{}.{}.{original}", span.start, span.end);
        let changed = format!("{}{}", case["insert"].as_str().unwrap(), content);
        let current = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::fingerprint(changed.as_bytes());
        let result = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::splice(&changed, &field, "Edited", &current);
        let fault = result.expect_err("stale span is never applied");
        assert!(format!("{fault:?}").contains(fixture["staleCode"].as_str().unwrap()));
    }
}

#[test]
fn typed_byte_spans_edit_nonfirst_windows_and_refuse_reordered_members() {
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../📌️panels/🔍️inspection/🧪️tests/🧫️fixtures/🔣️concurrency.json")).unwrap();
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.data_fields = Some(semio_framework_pack_json::from_json_str(r#"{"entries":[{"questionId":"bytes","value":null},{"questionId":"other","value":null}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    snapshot.data_fields.as_mut().unwrap().entries[0].value = semio_framework_value::DslValue::Bytes(fixture["byteValues"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect());
    let stamp = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::dictionary_stamp(&snapshot);
    let payload = PatchDocument { field: format!("data.0.value.bytes.128.256.{stamp}"), value: fixture["byteEdit"]["hex"].as_str().unwrap().into() };
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let emit = semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &ArtifactView::new(&snapshot, &history), &ConfigView { snapshot: &config, window: None }).unwrap();
    let LayoutMutation::ChangeDataFields(change) = &emit.artifact_mutations[0] else { panic!("native typed dictionary mutation") };
    let semio_framework_value::DslValue::Bytes(bytes) = &change.new_fields.as_ref().unwrap().entries[0].value else { panic!("bytes remain typed") };
    assert_eq!(serde_json::to_value(bytes).unwrap(), fixture["byteEdit"]["expected"]);
    snapshot.data_fields.as_mut().unwrap().entries.swap(0, 1);
    assert!(semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &ArtifactView::new(&snapshot, &history), &ConfigView { snapshot: &config, window: None }).is_err());
}

#[test]
fn dictionary_text_spans_reject_remote_insertions_and_member_reordering() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.data_fields = Some(semio_framework_pack_json::from_json_str(r#"{"entries":[{"questionId":"first","value":null},{"questionId":"second","value":null}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    snapshot.data_fields.as_mut().unwrap().entries[0].value = semio_framework_value::DslValue::String("😀é".repeat(300));
    let stamp = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::dictionary_stamp(&snapshot);
    let payload = PatchDocument { field: format!("data.0.value.chunk.85.170.{stamp}"), value: "Local draft".into() };
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut changed = snapshot.clone();
    if let semio_framework_value::DslValue::String(value) = &mut changed.data_fields.as_mut().unwrap().entries[0].value { value.insert(0, 'X'); }
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &ArtifactView::new(&changed, &history), &cfg).is_err());
    snapshot.data_fields.as_mut().unwrap().entries.swap(0, 1);
    assert!(semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &ArtifactView::new(&snapshot, &history), &cfg).is_err());
}

#[test]
fn scalar_dictionary_controls_and_nested_member_order_share_the_span_guard() {
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.data_fields = Some(semio_framework_pack_json::from_json_str(r#"{"entries":[{"questionId":"object","value":{"a":"same","b":"same"}}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let stamp = semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::dictionary_stamp(&snapshot);
    let payload = PatchDocument { field: format!("data.0.0.kind.at.{stamp}"), value: "number".into() };
    if let semio_framework_value::DslValue::Object(entries) = &mut snapshot.data_fields.as_mut().unwrap().entries[0].value { entries.swap(0, 1); }
    assert_ne!(semio_s_artifact_layout_layout_editor_commands::patch_document::text_chunks::dictionary_stamp(&snapshot), stamp);
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    assert!(semio_s_artifact_layout_layout_editor_commands::patch_document::handle(&payload, &ArtifactView::new(&snapshot, &history), &ConfigView { snapshot: &config, window: None }).is_err());
}
