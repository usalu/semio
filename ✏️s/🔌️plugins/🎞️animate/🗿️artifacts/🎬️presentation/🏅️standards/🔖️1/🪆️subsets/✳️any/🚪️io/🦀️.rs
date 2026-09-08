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
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{PresentationMutation, PresentationSnapshot, ANIMATE_DIALECT, PRESENTATION_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<PresentationSnapshot, export::json::v_rfc8259::any::PresentationIntoJson>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::json::v_rfc8259::any::JsonIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::md::v_commonmark::any::PresentationIntoMd>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::md::v_commonmark::any::MdIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::pdf::v1_4::any::PresentationIntoPdf>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::pdf::v1_4::any::PdfIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::pptx::v_ecma_376::any::PresentationIntoPptx>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::pptx::v_ecma_376::any::PptxIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::svg::v1_1::any::PresentationIntoSvg>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::svg::v1_1::any::SvgIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::png::v1_2::any::PresentationIntoPng>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::png::v1_2::any::PngIntoPresentation>(ANIMATE_DIALECT),
                    serializer_entry::<PresentationSnapshot, export::txt::v_utf_8::any::PresentationIntoTxt>(ANIMATE_DIALECT),
                    deserializer_entry::<PresentationSnapshot, import::txt::v_utf_8::any::TxtIntoPresentation>(ANIMATE_DIALECT),
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
/// 🖼️ Encodes a portable title card through the shared XML/SVG model.
pub fn animate_presentation_document_json_to_svg(value: &semio_framework_os_kernel::json::Value) -> Result<(String, u32, u32), String> {
    use semio_s_artifact_stdio_svg::schema::snapshot::write_svg_xml;
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
    let title = value.get("title").and_then(|entry| entry.as_str()).or_else(|| value.get("id").and_then(|entry| entry.as_str())).unwrap_or("Animate Presentation");
    let attributes = |values: &[(&str, &str)]| values.iter().map(|(name, value)| XmlAttr { name: (*name).into(), value: (*value).into() }).collect();
    let background = XmlNode::Element { name: "rect".into(), attrs: attributes(&[("width", "100%"), ("height", "100%"), ("fill", "white")]), children: Vec::new() };
    let title = XmlNode::Element { name: "text".into(), attrs: attributes(&[("x", "32"), ("y", "64"), ("font-size", "32"), ("fill", "#111827")]), children: vec![XmlNode::Text { text: title.into() }] };
    let root = XmlNode::Element { name: "svg".into(), attrs: attributes(&[("xmlns", "http://www.w3.org/2000/svg"), ("viewBox", "0 0 1280 720"), ("width", "1280"), ("height", "720")]), children: vec![background, title] };
    Ok((write_svg_xml(&XmlDocument { root: Some(root), ..Default::default() }), 1280, 720))
}

/// 📥️ Rasterizes a DWG drawing through the native host into a one-slide deck.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn animate_presentation_document_json_from_dwg(drawing: &semio_s_artifact_stdio_dwg::DwgDrawing) -> Result<dsl::DslValue, String> {
    use semio_s_artifact_stdio_svg::schema::snapshot::{parse_svg_xml, write_svg_xml};
    let (svg, width, height) = semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::io::dwg_drawing_to_svg(drawing)?;
    let validated_svg = write_svg_xml(&parse_svg_xml(&svg)?);
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&validated_svg, width, height)?;
    let frame = crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    let source = crate::FigureTileSource { src: format!("data:image/png;base64,{png_base64}"), kind: "image".into(), frame: frame.clone(), source_aspect: Some(width as f64 / height.max(1) as f64), pdf_page: None };
    let tiles = vec![crate::FigureTileDraft { id: "imported-drawing".into(), name: "Imported Drawing".into(), crop: frame }];
    let deck = crate::presentation_snapshot_with_tiles(&source, &tiles);
    Ok(dsl::ToValue::to_value(&deck))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️MediaCodec
