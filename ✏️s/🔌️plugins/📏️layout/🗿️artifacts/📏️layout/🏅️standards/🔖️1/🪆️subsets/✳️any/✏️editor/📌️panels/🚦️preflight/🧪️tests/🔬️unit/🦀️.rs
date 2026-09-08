
use super::*;
use crate::editor::layout::testkit::{layout_app, render as render_body};
use semio_framework_plugin::AppLabels;

#[semio_framework_async_macros::async_test]
async fn preflight_finds_missing_asset() {
    let issues = run_layout_preflight(&crate::standards::v1::subsets::any::schema::default_document(), LayoutLabels::labels(semio_framework_plugin::Locale::En, semio_framework_plugin::Terminology::Native));
    assert!(issues.iter().any(|issue| issue.code == "asset.missing"));
    let mut app = layout_app().await;
    let json = render_body(&mut app, LAYOUT_PLAY_BODY_PREFLIGHT).await;
    assert!(json.contains("asset.missing") || json.contains("Linked asset missing"));
}

#[semio_framework_async_macros::async_test]
async fn preflight_reports_all_expected_issue_codes() {
    let json = r#"{
            "schema": "layout.layout",
            "name": "Preflight Fixture",
            "grid": {"baselineGrid":12,"baselineOffset":0,"snapToBaseline":false},
            "paragraphStyles": [{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],
            "characterStyles": [
                {"id":"character.small","fontFamily":"Layout Sans","fontSize":6},
                {"id":"character.exotic","fontFamily":"Comic Sans","fontSize":10}
            ],
            "stories": [
                {"id":"story-small","content":"Small caption text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.small"}]},
                {"id":"story-exotic","content":"Exotic font text.","styleRuns":[{"start":0,"end":10,"paragraphStyleId":"paragraph.body","characterStyleId":"character.exotic"}]},
                {"id":"story-overset","content":"placeholder","styleRuns":[]}
            ],
            "links": [
                {"id":"link-missing","path":"a.png","hash":"sha256:missing","width":100,"height":100,"dpi":300,"state":"missing"},
                {"id":"link-modified","path":"b.png","hash":"sha256:abc","width":100,"height":100,"dpi":300,"state":"modified"},
                {"id":"link-lowres","path":"c.png","hash":"sha256:def","width":100,"height":100,"dpi":72},
                {"id":"link-rgb","path":"d.png","hash":"sha256:ghi","width":100,"height":100,"dpi":300,"colorProfile":"RGB"}
            ],
            "parentPages": [],
            "spreads": [{"id":"spread-1","name":"Spread 1","pageIds":["page-1"]}],
            "pages": [{
                "id":"page-1","name":"Page 1","spreadId":"spread-1","width":200,"height":200,
                "margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},
                "guides":[], "layerIds":["layer-1"],
                "layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-oob","frame-missing","frame-modified","frame-lowres","frame-no-story","frame-small","frame-exotic","frame-overset"]}],
                "frames":[
                    {"id":"frame-oob","layerId":"layer-1","kind":"rect","bounds":{"x":150,"y":150,"w":100,"h":100,"rotation":0},"fill":[0,0,0,1]},
                    {"id":"frame-missing","layerId":"layer-1","kind":"image","bounds":{"x":0,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-missing"},
                    {"id":"frame-modified","layerId":"layer-1","kind":"image","bounds":{"x":20,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-modified"},
                    {"id":"frame-lowres","layerId":"layer-1","kind":"image","bounds":{"x":40,"y":0,"w":20,"h":20,"rotation":0},"linkId":"link-lowres"},
                    {"id":"frame-no-story","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":40,"w":50,"h":20,"rotation":0},"storyId":"story-absent","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-small","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":60,"w":50,"h":20,"rotation":0},"storyId":"story-small","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-exotic","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":80,"w":50,"h":20,"rotation":0},"storyId":"story-exotic","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"},
                    {"id":"frame-overset","layerId":"layer-1","kind":"text","bounds":{"x":0,"y":100,"w":50,"h":20,"rotation":0},"storyId":"story-overset","columns":1,"inset":{"x":0,"y":0,"w":50,"h":20},"wrapMode":"none"}
                ],
                "overrides":[]
            }],
            "printTarget":"print"
        }"#;
    let mut doc: LayoutSnapshot = dsl::os_pack::from_json_str(json).expect("preflight fixture");
    if let Some(story) = doc.stories.iter_mut().find(|story| story.id == "story-overset") {
        story.content = "a".repeat(450);
    }
    let issues = run_layout_preflight(&doc, LayoutLabels::labels(semio_framework_plugin::Locale::En, semio_framework_plugin::Terminology::Native));
    let codes: Vec<&str> = issues.iter().map(|issue| issue.code.as_str()).collect();
    for expected in ["object.out_of_bounds", "asset.missing", "asset.modified", "asset.low_resolution", "image.empty_frame", "text.missing_story", "text.below_minimum_size", "font.missing", "text.overset", "asset.rgb_in_print"] {
        assert!(codes.contains(&expected), "missing preflight code: {expected}");
    }
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_preflight_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), LAYOUT_PLAY_PREFLIGHT_TAB_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LAYOUT_PLAY_BODY_PREFLIGHT));
}
