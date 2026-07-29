//! 📦 Note app — binary document surface + laws (constitutional: pack).

use note::{NoteBlockNode, NoteCamera, NoteDocument, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use std::collections::BTreeMap;
use store::PackError;

/// 📦 Encodes a `NoteDocument` to its binary pack form.
pub fn encode(document: &NoteDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `NoteDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<NoteDocument, PackError> {
    <NoteDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = note_dsl::parse_dsl(note_dsl::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_representative_document() {
        let mut assets = BTreeMap::new();
        assets.insert(
            "asset-1".into(),
            NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into(), width: Some(10.0), height: Some(20.0) },
        );
        let document = NoteDocument {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            camera: NoteCamera { x: 12.5, y: -4.0, zoom: 1.5 },
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: None,
            grid_opacity: Some(0.35),
            snap_enabled: None,
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: None,
            assets,
            blocks: vec![
                NoteBlockNode::Text {
                    id: "text-1".into(),
                    name: "Text".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 200.0,
                    height: 80.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    paragraphs: vec![NoteTextParagraph {
                        runs: vec![NoteTextRun { text: "plain".into(), bold: None, italic: None, underline: None, link: None }],
                    }],
                    font_size: 16.0,
                    font_weight: "bold".into(),
                    align: "center".into(),
                },
                NoteBlockNode::Table {
                    id: "table-1".into(),
                    name: "Table".into(),
                    x: 20.0,
                    y: 20.0,
                    width: 320.0,
                    height: 120.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    columns: vec!["A".into(), "B".into()],
                    rows: vec![vec![NoteTableCell { content: "a1".into() }, NoteTableCell { content: "b1".into() }]],
                },
            ],
        };
        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪Tests
