"""✍️ Stages the mounted regression for font reassignment with retained raw codes."""

from pathlib import Path
import json
import sys

ticket = Path(__file__).resolve().parent
base = Path(sys.argv[1]).resolve()
stage = ticket / "🗑️generated/text-font-edit"
previous = ticket / "🗑️generated/mixed/editor-tests.rs"
source_file = base / "✏️editor/🧪️tests/🔬️unit/🦀️.rs"
fixture_file = base / "🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json"
case = json.loads((ticket / "🗑️generated/font-reassignment.json").read_text())
fixture = json.loads(fixture_file.read_text())
fixture["fontReassignment"] = case
(stage / "natural-file-fixture.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n")
source = (previous if previous.exists() else source_file).read_text()
source += r'''
#[semio_framework_async_macros::async_test]
async fn registered_pdf_font_change_preserves_encoded_text_and_the_next_line() {
    use crate::editor::page::{objects, ObjectKind};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfTextArrayItem};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_framework_value::DslValue;
    use serde_json::json;

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let case = &fixture["fontReassignment"];
    let byte = case["sourceCode"].as_u64().unwrap();
    let adjust = case["kerning"].as_f64().unwrap();
    let alternate = case["sourceAlternateCode"].as_u64().unwrap();
    let program = format!("BT /Fsrc 20 Tf 10 50 Td 24 TL [<{byte:02X}> {adjust} <{byte:02X}>] TJ T* (Tail) Tj ET");
    let input = pdf_fixture_file(vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 180 90] /Resources << /Font << /Fsrc 5 0 R /Fdst 6 0 R >> >> /Contents 4 0 R >>".to_vec(),
        pdf_fixture_stream(program.as_bytes()),
        format!("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [{alternate} /eacute] >> >>").into_bytes(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /MacRomanEncoding >>".to_vec(),
    ]);
    let mut scope = laws::OriginalFixtureScope::admit();
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut app = open_registered_pdf(input, 1, &mut scope).await;
    let text_objects = |snapshot: &PdfSnapshot| objects(snapshot).into_iter().filter(|object| object.kind == ObjectKind::Text).collect::<Vec<_>>();
    let before = text_objects(app.snapshot().unwrap());
    assert_eq!(before[0].text.as_str(), case["firstLine"].as_str().unwrap());
    assert_eq!(before[1].text.as_str(), case["secondLine"].as_str().unwrap());
    let args = DslValue::from(json!({"page": 0, "object": before[0].id, "text": case["targetFont"]}));
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "set-font", Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();

    let verify = |snapshot: &PdfSnapshot, from_original: bool| {
        let after = text_objects(snapshot);
        assert_eq!(after[0].text, before[0].text);
        assert_eq!((after[0].x, after[0].y), (before[0].x, before[0].y));
        assert_eq!(after[1].text, before[1].text);
        assert_eq!((after[1].x, after[1].y), (before[1].x, before[1].y));
        for (object, expected) in after.iter().zip(["targetFont", "sourceFont"]) {
            assert_eq!(snapshot.font(&object.font).unwrap().base_font(), case[expected].as_str().unwrap());
        }
        let PdfOp::ShowTextArray { items } = &snapshot.pages[0].content[after[0].end] else { panic!("the edited text array retains its spacing"); };
        assert!(items.iter().any(|item| matches!(item, PdfTextArrayItem::Adjust { amount } if *amount == adjust)));
        if from_original {
            assert!(matches!(&items[0], PdfTextArrayItem::Text { text } if text == "é"));
        }
    };
    verify(app.snapshot().unwrap(), true);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.unwrap().unwrap();
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).unwrap();
    let mut reopened = open_registered_pdf(saved.data, 1, &mut scope).await;
    verify(reopened.snapshot().unwrap(), false);
    laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).unwrap();
}
'''
(stage / "editor-tests.rs").write_text(source)
