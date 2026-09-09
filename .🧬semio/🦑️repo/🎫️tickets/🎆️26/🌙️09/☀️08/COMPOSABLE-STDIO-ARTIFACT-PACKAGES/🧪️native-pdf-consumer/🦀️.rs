use semio_s_artifact_stdio_pdf as pdf;

/// 📦️ Links restored PDF deliverables and compares their public metadata with a neutral fixture.
fn main() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("neutral PDF metadata fixture");
    let kind = pdf::artifact_kind();
    let factories = pdf::native_codecs();
    pdf::definition().expect("standalone PDF definition");
    let actual = serde_json::json!({
        "artifactIdentity": pdf::contribution().identity,
        "kindId": kind.id,
        "factoryId": factories.first().expect("PDF factory").id,
        "factoryCount": factories.len(),
    });
    assert_eq!(actual, expected);
    println!("[DEBUG] restored PDF library consumer: metadata matches serde_json fixture; standalone definition builds");
}
