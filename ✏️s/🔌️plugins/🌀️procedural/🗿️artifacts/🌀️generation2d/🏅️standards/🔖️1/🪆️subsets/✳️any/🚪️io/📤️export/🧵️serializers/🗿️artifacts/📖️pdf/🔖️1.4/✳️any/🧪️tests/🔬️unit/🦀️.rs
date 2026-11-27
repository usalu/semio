use super::*;

/// 🔮️ lopdf (third-party, test-only) opens the bundled example's bytes as a one-page PDF 1.4.
#[test]
fn the_bundled_example_is_a_one_page_pdf_1_4() {
    let pdf = serialize_bytes(&semio_s_artifact_procedural_2d_io_text::snapshot::default_snapshot()).expect("pdf");
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let document = lopdf::Document::load_mem(&pdf).expect("independent PDF reader opens the page");
    assert_eq!(document.version, "1.4");
    assert_eq!(document.get_pages().len(), 1);
}

#[test]
fn the_declaration_carries_the_pdf_route() {
    let declaration = declaration().expect("generation2d declaration with its pdf route");
    assert!(declaration.definition().capabilities().any(|capability| capability.identity().as_str() == "s.procedural.generation2d.composer.pdf"));
    assert!(!semio_s_artifact_procedural_generation2d::definition().expect("base definition").capabilities().any(|capability| capability.identity().as_str() == "s.procedural.generation2d.composer.pdf"));
}
