//! 📦️ Layout artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::layout::LayoutSnapshot;
use store::PackError;

/// 📦️ Encodes a `LayoutSnapshot` to its binary pack form.
pub fn encode(document: &LayoutSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `LayoutSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<LayoutSnapshot, PackError> {
    <LayoutSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::layout::dsl;
    use crate::artifacts::layout::{CharacterStyle, Frame, GridSettings, Layer, LayoutBounds, Page, PageColumns, PageMargins, PageOverride, LAYOUT_DOCUMENT_SCHEMA};

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dsl::parse_dsl(dsl::LAYOUT_SAMPLE_TEXT).expect("parse sample layout fixture");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_minimal_document_with_character_style() {
        let document = LayoutSnapshot {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "Empty".into(),
            grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: vec![CharacterStyle { id: "char.emph".into(), name: None, font_family: None, font_size: None, font_weight: None, italic: Some(true), color: None, tracking: None }],
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
            background_drawing: None,
            referenced_model: None,
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn pack_round_trips_overrides_frame_flags_and_absent_print_target() {
        let document = LayoutSnapshot {
            schema: LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "Flags".into(),
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
            data_fields_json: None,
            background_drawing: None,
            referenced_model: None,
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
