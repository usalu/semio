//! 🚪️ IO s.animate.presentation (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec
//! plus every foreign hop, aggregated from the typed `Serializer<PresentationSnapshot>`/
//! `Deserializer<PresentationSnapshot>` leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`.
//! Replaces the old hand-rolled `ArtifactComposition`/`ComposerEntry` dispatch chain outright — all
//! io now goes exclusively through the `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `PresentationSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `PresentationMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are computed,
//! never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
//! binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a real, supported
//! shape per that type's own doc, matching the stdio pilot's/`🎬️sequence`'s identical documented
//! deviation; the underlying codec impls these would point at are unchanged and independently
//! tested either way.

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::presentation::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::presentation::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::presentation::{PresentationMutation, PresentationSnapshot, ANIMATE_DIALECT, PRESENTATION_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::json::v_rfc8259::any::PresentationIntoJson>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::json::v_rfc8259::any::JsonIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::md::v_commonmark::any::PresentationIntoMd>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::md::v_commonmark::any::MdIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::pdf::v1_4::any::PresentationIntoPdf>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::pdf::v1_4::any::PdfIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::pptx::v_ecma_376::any::PresentationIntoPptx>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::pptx::v_ecma_376::any::PptxIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::svg::v1_1::any::PresentationIntoSvg>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::svg::v1_1::any::SvgIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::png::v1_2::any::PresentationIntoPng>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::png::v1_2::any::PngIntoPresentation>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(serializer_entry::<PresentationSnapshot, export::txt::v_utf_8::any::PresentationIntoTxt>(ANIMATE_DIALECT)),
                    semio_framework_plugin::resolve_ready(deserializer_entry::<PresentationSnapshot, import::txt::v_utf_8::any::TxtIntoPresentation>(ANIMATE_DIALECT)),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<PresentationSnapshot, PresentationMutation>(PRESENTATION_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

//#region 🔖️MediaCodec
/// 🖼️ Relocated verbatim from the former artifact-tree `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): title-card SVG export for the app catalogue/
/// thumbnail surface. There is no real drawing content to route through semio/drawing here (this is a
/// generic placeholder title card, not a geometry export), so the shared framework helper stays — but
/// its output is round-tripped through stdio's own real SVG codec (`parse_svg_xml`/`write_svg_xml`)
/// before being returned, which both validates it is genuinely spec-conformant SVG and exercises the
/// real stdio engine rather than returning the framework helper's raw string untouched.
pub fn animate_presentation_document_json_to_svg(value: &semio_framework_os_kernel::json::Value) -> Result<(String, u32, u32), String> {
    use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, write_svg_xml};
    let (svg, width, height) = semio_framework_os::title_card_svg(value, "Animate Presentation", 1280, 720)?;
    let doc = parse_svg_xml(&svg)?;
    Ok((write_svg_xml(&doc), width, height))
}

/// 📥️ Builds a degenerate-but-valid one-slide deck from a rasterized DWG drawing, for the DWG import
/// path. `stdio_gap`: this plugin's write scope explicitly forbids inventing a converter inside
/// animate — there is no bridge anywhere in stdio/framework from the legacy
/// `semio_s_plugin_stdio::artifacts::dwg::DwgDrawing` (11 geometry variants: Line/Point/Circle/Arc/Ellipse/LwPolyline/
/// Spline/Text/Face3d/Polyline3d/PolyfaceMesh) to semio's `SemioDrawingSnapshot`/`DrawNode` tree.
/// Hand-rolling that conversion here would duplicate `semio_framework_os::dwg_drawing_to_svg`'s
/// existing, correct, shared geometry logic for a hand-rolled struct — reported in
/// `w5a--report.md`'s stdio_gaps rather than invented. The framework helpers stay (shared,
/// non-duplicative utilities, not local ad-hoc codec code); the SVG they produce is still round-
/// tripped through stdio's real SVG codec before rasterization, same as the title-card path.
pub fn animate_presentation_document_json_from_dwg(drawing: &semio_s_plugin_stdio::artifacts::dwg::DwgDrawing) -> Result<dsl::DslValue, String> {
    use semio_s_plugin_stdio::artifacts::svg::schema::snapshot::{parse_svg_xml, write_svg_xml};
    let (svg, width, height) = semio_framework_os::dwg_drawing_to_svg(drawing)?;
    let validated_svg = write_svg_xml(&parse_svg_xml(&svg)?);
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&validated_svg, width, height)?;
    let frame = crate::artifacts::presentation::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let source = crate::artifacts::presentation::FigureTileSource { src: format!("data:image/png;base64,{png_base64}"), kind: "image".into(), frame: frame.clone(), source_aspect: Some(width as f64 / height.max(1) as f64), pdf_page: None };
    let tiles = vec![crate::artifacts::presentation::FigureTileDraft { id: "imported-drawing".into(), name: "Imported Drawing".into(), crop: frame }];
    let deck = crate::artifacts::presentation::presentation_snapshot_with_tiles(&source, &tiles);
    Ok(dsl::ToValue::to_value(&deck))
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os_kernel::json::{object, Object, Value};

    #[test]
    fn animate_presentation_document_json_to_svg_embeds_title() {
        let document = object([("title".to_string(), Value::from("My Deck"))]);
        let (svg, width, height) = animate_presentation_document_json_to_svg(&document).expect("svg");
        assert!(svg.contains("My Deck"));
        assert_eq!((width, height), (1280, 720));
    }

    #[test]
    fn animate_presentation_document_json_to_svg_falls_back_to_app_label_without_title() {
        let document = Value::Object(Object::new());
        let (svg, _, _) = animate_presentation_document_json_to_svg(&document).expect("svg fallback");
        assert!(svg.contains("Animate Presentation"));
    }

    #[test]
    fn from_dwg_builds_single_slide_deck_from_entity() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing {
            layers: vec![semio_s_plugin_stdio::artifacts::dwg::DwgLayer::default()],
            entities: vec![semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
                layer: 0,
                color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
                geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
            }],
            extmin: [0.0, 0.0, 0.0],
            extmax: [10.0, 10.0, 0.0],
        };
        let document = animate_presentation_document_json_from_dwg(&drawing).expect("from_dwg");
        let deck: crate::artifacts::presentation::PresentationSnapshot = dsl::FromValue::from_value(document).expect("deck");
        assert_eq!(deck.schema, crate::artifacts::presentation::PRESENTATION_DOCUMENT_SCHEMA);
        let (source, tiles) = crate::artifacts::presentation::presentation_working_scene(&deck);
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0].name, "Imported Drawing");
        assert!(source.src.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn from_dwg_never_errors_on_empty_drawing() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let document = animate_presentation_document_json_from_dwg(&drawing).expect("from_dwg on empty drawing");
        let deck: crate::artifacts::presentation::PresentationSnapshot = dsl::FromValue::from_value(document).expect("deck");
        let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(&deck);
        assert_eq!(tiles.len(), 1);
    }
}
//#endregion 🔖️MediaCodec
