//! 📚️ LAW: every example a stdio editor seats through `setActiveExample` decodes into a NON-EMPTY document of that
//! editor's own snapshot type. The editors' example loaders used to answer an asset that no longer decodes with the
//! genesis document (`parse_dsl(PRIMARY_TEXT).unwrap_or_default()`), so a stale `🗣️.dsl.semio` shipped as an empty
//! editor nobody noticed (`📓️wp-lb2.md` item 7: the xml demo was the old five-field wire). This law names every
//! seated example that is stale, per editor, instead of letting one silently open empty.

use semio_framework_os_kernel::os_store::ArtifactDsl;

/// 📚️ One seated example: the editors whose loader seats it, its declared id, and the decode check against the
/// snapshot type those editors hold.
struct SeatedExample {
    editors: &'static str,
    id: &'static str,
    check: fn() -> Result<(), String>,
}

/// 🔬️ `Ok` when `text` decodes into a document other than the genesis one — an example that decodes to the genesis
/// document seats nothing.
fn decodes_non_empty<S: ArtifactDsl + Default + PartialEq>(text: &str) -> Result<(), String> {
    match S::parse_dsl(text) {
        Err(error) => Err(format!("undecodable: {error}")),
        Ok(snapshot) if snapshot == S::default() => Err("empty: decodes to the genesis document".to_string()),
        Ok(_) => Ok(()),
    }
}

/// 📋️ Every example the 62 stdio editor loaders seat, deduplicated by (snapshot type, example module).
fn seated_examples() -> Vec<SeatedExample> {
    vec![
        SeatedExample { editors: "aviEditor", id: semio_s_artifact_stdio_avi::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_avi::AviSnapshot>(semio_s_artifact_stdio_avi::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "bcf", id: semio_s_artifact_stdio_bcf::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_bcf::BcfSnapshot>(semio_s_artifact_stdio_bcf::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "bmpEditor", id: semio_s_artifact_stdio_bmp::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_bmp::BmpSnapshot>(semio_s_artifact_stdio_bmp::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "csv", id: semio_s_artifact_stdio_csv::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_csv::CsvSnapshot>(semio_s_artifact_stdio_csv::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "dwgAc1018Editor, dwgAc1024Editor",
            id: semio_s_artifact_stdio_dwg::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_dwg::DwgSnapshot>(semio_s_artifact_stdio_dwg::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample { editors: "dxfAnyEditor", id: semio_s_artifact_stdio_dxf::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_dxf::DxfSnapshot>(semio_s_artifact_stdio_dxf::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "gif87aEditor, gif89aEditor", id: semio_s_artifact_stdio_gif::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_gif::GifSnapshot>(semio_s_artifact_stdio_gif::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "gltfAnyEditor", id: semio_s_artifact_stdio_gltf::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_gltf::GltfSnapshot>(semio_s_artifact_stdio_gltf::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "html", id: semio_s_artifact_stdio_html::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_html::HtmlSnapshot>(semio_s_artifact_stdio_html::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "ifc4AnyEditor", id: semio_s_artifact_stdio_ifc::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_ifc::IfcSnapshot>(semio_s_artifact_stdio_ifc::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "ifc2x3AnyEditor",
            id: semio_s_artifact_stdio_ifc::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot>(semio_s_artifact_stdio_ifc::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "ifc2x3CobieEditor",
            id: semio_s_artifact_stdio_ifc::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::cobie::schema::snapshot::Ifc2x3Snapshot>(semio_s_artifact_stdio_ifc::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "ifc2x3Cv20Editor",
            id: semio_s_artifact_stdio_ifc::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::cv20::schema::snapshot::Ifc2x3Snapshot>(semio_s_artifact_stdio_ifc::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "ifc2x3SavEditor",
            id: semio_s_artifact_stdio_ifc::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::sav::schema::snapshot::Ifc2x3Snapshot>(semio_s_artifact_stdio_ifc::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "jpgAnyEditor, jpgBaselineEditor",
            id: semio_s_artifact_stdio_jpg::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_jpg::JpgSnapshot>(semio_s_artifact_stdio_jpg::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample { editors: "json_any", id: semio_s_artifact_stdio_json::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_json::JsonSnapshot>(semio_s_artifact_stdio_json::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "json_i_json",
            id: semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::i_json::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_json::JsonSnapshot>(semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::i_json::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample { editors: "lasAnyEditor", id: semio_s_artifact_stdio_las::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_las::LasSnapshot>(semio_s_artifact_stdio_las::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "md", id: semio_s_artifact_stdio_md::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_md::MdSnapshot>(semio_s_artifact_stdio_md::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "mp3Editor", id: semio_s_artifact_stdio_mp3::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_mp3::Mp3Snapshot>(semio_s_artifact_stdio_mp3::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "mp4Editor", id: semio_s_artifact_stdio_mp4::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_mp4::Mp4Snapshot>(semio_s_artifact_stdio_mp4::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "objAnyEditor", id: semio_s_artifact_stdio_obj::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_obj::ObjSnapshot>(semio_s_artifact_stdio_obj::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "plyAnyEditor", id: semio_s_artifact_stdio_ply::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_ply::PlySnapshot>(semio_s_artifact_stdio_ply::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "pngEditor", id: semio_s_artifact_stdio_png::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_png::PngSnapshot>(semio_s_artifact_stdio_png::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "semioAnyEditor", id: semio_s_artifact_stdio_semio::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_semio::SemioSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "semioAnimationEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioAudioEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioCadEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioDocumentEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioDrawingEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioFlowEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioGraphEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioImageEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioKitEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioModelEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioObjectEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioPresentationEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioTableEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioTextEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioValueEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "semioVideoEditor",
            id: semio_s_artifact_stdio_semio::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot>(semio_s_artifact_stdio_semio::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "stepAnyEditor, stepCc1Editor, stepCc2Editor, stepCc3Editor, stepCc4Editor, stepCc5Editor, stepCc6Editor",
            id: semio_s_artifact_stdio_step::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_step::StepSnapshot>(semio_s_artifact_stdio_step::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample { editors: "stlAnyEditor", id: semio_s_artifact_stdio_stl::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_stl::StlSnapshot>(semio_s_artifact_stdio_stl::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "svgAnyEditor, svgBasicEditor, svgTinyEditor",
            id: semio_s_artifact_stdio_svg::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_svg::SvgSnapshot>(semio_s_artifact_stdio_svg::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample {
            editors: "tiffAnyEditor, tiffBaselineEditor",
            id: semio_s_artifact_stdio_tiff::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_tiff::TiffSnapshot>(semio_s_artifact_stdio_tiff::examples::demo::PRIMARY_TEXT),
        },
        SeatedExample { editors: "tsv", id: semio_s_artifact_stdio_tsv::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_tsv::TsvSnapshot>(semio_s_artifact_stdio_tsv::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "txt", id: semio_s_artifact_stdio_txt::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_txt::TxtSnapshot>(semio_s_artifact_stdio_txt::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "wavEditor", id: semio_s_artifact_stdio_wav::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_wav::WavSnapshot>(semio_s_artifact_stdio_wav::examples::demo::PRIMARY_TEXT) },
        SeatedExample { editors: "xml_any", id: semio_s_artifact_stdio_xml::examples::demo::ID, check: || decodes_non_empty::<semio_s_artifact_stdio_xml::XmlSnapshot>(semio_s_artifact_stdio_xml::examples::demo::PRIMARY_TEXT) },
        SeatedExample {
            editors: "xml_valid",
            id: semio_s_artifact_stdio_xml::standards::v1_0::subsets::valid::examples::demo::ID,
            check: || decodes_non_empty::<semio_s_artifact_stdio_xml::XmlSnapshot>(semio_s_artifact_stdio_xml::standards::v1_0::subsets::valid::examples::demo::PRIMARY_TEXT),
        },
    ]
}

/// 📚️ LAW: every seated stdio example decodes into a non-empty document of its editors' snapshot type.
#[test]
fn every_seated_stdio_example_decodes_into_a_non_empty_document() {
    let stale: Vec<String> = seated_examples().into_iter().filter_map(|example| (example.check)().err().map(|reason| format!("{} [{}]: {reason}", example.id, example.editors))).collect();
    assert!(stale.is_empty(), "{} stale seated example(s):\n{}", stale.len(), stale.join("\n"));
}
