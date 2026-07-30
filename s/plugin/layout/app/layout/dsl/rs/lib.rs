//! 📜 Layout app — textual document grammar surface + laws (constitutional: dsl).

use layout::LayoutDocument;

/// 📄 The bundled sample fixture, handcrafted in the `.layout` DSL.
pub const LAYOUT_SAMPLE_TEXT: &str = include_str!("../../../../example/sample.layout");

/// 📖 Parses `.layout` DSL text into a `LayoutDocument`.
pub fn parse_dsl(text: &str) -> Result<LayoutDocument, store::TextError> {
    <LayoutDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LayoutDocument` back to `.layout` DSL text.
pub fn print_dsl(document: &LayoutDocument) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use layout::{Frame, GridSettings, Layer, LayoutBounds, LayoutCamera, Page, PageColumns, PageMargins, PageOverride, LAYOUT_FIXTURE_SCHEMA};

    fn minimal_document_with_character_style() -> LayoutDocument {
        LayoutDocument {
            schema: LAYOUT_FIXTURE_SCHEMA.into(),
            name: "Empty".into(),
            camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: vec![serde_json::json!({"id": "char.emph", "italic": true})],
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
        }
    }

    fn overrides_frame_flags_document() -> LayoutDocument {
        LayoutDocument {
            schema: LAYOUT_FIXTURE_SCHEMA.into(),
            name: "Flags".into(),
            camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: LayoutCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: vec![Page {
                id: "page-1".into(),
                name: "Page".into(),
                spread_id: "spread-1".into(),
                parent_page_id: None,
                width: 100.0,
                height: 100.0,
                margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
                columns: PageColumns { count: 1, gutter: 0.0 },
                guides: Vec::new(),
                layer_ids: vec!["layer-1".into()],
                layers: vec![Layer { id: "layer-1".into(), name: "Content".into(), visible: true, locked: false, object_ids: vec!["frame-locked".into(), "frame-unlocked".into()] }],
                frames: vec![
                    Frame::Rect { id: "frame-locked".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: Some(true), visible: Some(false), fill: None, stroke: None },
                    Frame::Rect { id: "frame-unlocked".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: Some(false), visible: Some(true), fill: None, stroke: None },
                ],
                overrides: vec![
                    PageOverride { object_id: "frame-locked".into(), bounds: Some(LayoutBounds { x: 1.0, y: 2.0, width: 3.0, height: 4.0, rotation: 5.0 }), visible: Some(true), locked: Some(false) },
                    PageOverride { object_id: "frame-unlocked".into(), bounds: None, visible: None, locked: None },
                ],
            }],
            print_target: None,
        }
    }

    #[test]
    fn sample_fixture_dsl_round_trips() {
        let doc = parse_dsl(LAYOUT_SAMPLE_TEXT).expect("parse sample layout fixture");
        assert_eq!(doc.schema, LAYOUT_FIXTURE_SCHEMA);
        assert_eq!(doc.pages.len(), 2);
        store::test_support::assert_dsl_round_trip(&doc);
    }

    #[test]
    fn dsl_round_trips_minimal_document_with_character_style() {
        store::test_support::assert_dsl_round_trip(&minimal_document_with_character_style());
    }

    #[test]
    fn dsl_round_trips_overrides_frame_flags_and_absent_print_target() {
        store::test_support::assert_dsl_round_trip(&overrides_frame_flags_document());
    }

    #[test]
    fn parse_dsl_reports_engine_parser_errors() {
        // The hand-rolled lexer/parser (and its bespoke error messages) is gone — parsing now goes
        // through the `dsl::` derive engine directly, so these assert only on the public
        // `store::DocumentDsl` surface, generically on failure rather than on exact internal wording
        // that no longer exists.
        assert!(parse_dsl("").is_err(), "empty text must fail: a document has required fields");
        assert!(parse_dsl("not a document at all").is_err(), "unrecognized leading token must fail");
        assert!(parse_dsl("schema=\"layout.fixture\" name=\"t\"").is_err(), "quoted schema must fail: schema is a bare ident");
        assert!(parse_dsl("schema=layout.fixture name=unquoted").is_err(), "unquoted name must fail: name is a quoted string");
        assert!(
            parse_dsl("schema=layout.fixture name=\"t\" camera { x=notanumber y=0 zoom=1 }").is_err(),
            "non-numeric camera field must fail"
        );
        let bad_bool = "schema=layout.fixture name=\"t\" camera { x=0 y=0 zoom=1 } previewCamera { x=0 y=0 zoom=1 } grid { baselineGrid=12 baselineOffset=0 snapToBaseline=maybe }";
        assert!(parse_dsl(bad_bool).is_err(), "non-boolean grid flag must fail");
    }
}
//#endregion 🧪Tests
