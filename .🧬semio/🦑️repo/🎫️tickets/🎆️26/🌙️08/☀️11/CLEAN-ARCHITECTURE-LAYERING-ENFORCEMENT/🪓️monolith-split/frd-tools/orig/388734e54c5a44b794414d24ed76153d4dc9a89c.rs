use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_ui_contract::BuiltNode;
use crate::viewer::pdf14a::modes::view::windows::main::{page_summary};
use crate::viewer::pdf14a::modes::view::windows::main::*;
static CORPUS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../✏️editor/🧪️tests/🧫️fixtures/📄️resolved-page-domain/🔣️.json"));
fn fixture() -> crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot {
    use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
    let fixture: serde_json::Value = serde_json::from_str((*CORPUS)).unwrap();
    PdfSnapshot { schema: fixture["schema"].as_str().unwrap().into(), pages: fixture["pages"].as_array().unwrap().iter().map(|page| PageDoc { width: page["width"].as_f64().unwrap(), height: page["height"].as_f64().unwrap(), text: page["text"].as_str().unwrap().into() }).collect() }
}

#[test]
fn definition_declares_a_document_window() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    assert!(definition.actions.is_empty());
}
#[test]
fn render_lists_each_own_page_with_exact_dimensions_and_text() {
    let document = fixture();
    assert_eq!(document.pages.len(), 2);
    assert_eq!(page_summary(0, &document.pages[0]), "1 | 200 × 300\nSemio page one");
    assert_eq!(page_summary(1, &document.pages[1]), "2 | 400 × 500\nSeite zwei");
    let node = render(&document).unwrap();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&projection).unwrap();
    assert_eq!(scene.buffer, page_summary(0, &document.pages[0]));
    assert_eq!(scene.settings_json.as_deref(), Some("{\"readOnly\":true}"));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
