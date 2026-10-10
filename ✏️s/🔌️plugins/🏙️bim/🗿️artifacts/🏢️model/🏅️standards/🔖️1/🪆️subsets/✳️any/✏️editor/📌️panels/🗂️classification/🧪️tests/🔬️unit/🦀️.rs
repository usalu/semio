use super::*;
use crate::ClassificationItem;
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn item(code: &str, title: &str, parent: Option<&str>) -> ClassificationItem {
    ClassificationItem { code: code.into(), title: title.into(), parent: parent.map(str::to_string) }
}

fn model() -> ModelSnapshot {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let entries = vec![item("EF", "Elements and functions", None), item("EF_25", "Walls and barriers", Some("EF")), item("EF_25_10", "Walls", Some("EF_25")), item("Pr", "Products", None), item("Pr_20", "Structure", Some("Pr"))];
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass 2015".into(), edition: "2024".into(), source: None, entries });
    snapshot.classifications.insert("w-south".into(), std::collections::BTreeMap::from([("cs-uni".to_string(), "EF_25_10".to_string())]));
    snapshot
}

fn text(snapshot: &ModelSnapshot, elements: &[&str], library: &[&str], locale: Locale) -> String {
    let own = |ids: &[&str]| ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let view = ViewModel::new(locale, Terminology::Native);
    let node = render(snapshot, crate::editor::bim::terminology::bim_labels(&view), &own(elements), &own(library), &TreeWindows::for_body(&view, BODY_KEY)).expect("the classification browser renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn the_whole_table_is_shown_in_tree_order_indented_by_depth() {
    let rendered = text(&model(), &[], &[], Locale::En);
    for expected in ["Uniclass 2015 2024", "EF  Elements and functions", "· EF_25  Walls and barriers", "· · EF_25_10  Walls", "Pr  Products", "· Pr_20  Structure", "bim-classification.cs-uni.search.input"] {
        assert!(rendered.contains(expected), "the browser shows '{expected}': {rendered}");
    }
    let (tree, flat) = (rendered.find("EF_25_10").expect("a leaf"), rendered.find("Pr_20").expect("another branch"));
    assert!(tree < flat, "the tree runs depth first in table order");
}

#[semio_framework_async_macros::async_test]
async fn a_search_narrows_the_tree_to_the_matches_and_their_ancestors() {
    let rendered = text(&model(), &[], &["cs-uni:EF_25_10"], Locale::En);
    assert!(rendered.contains("EF_25_10  Walls") && rendered.contains("EF_25  Walls and barriers") && rendered.contains("EF  Elements and functions"), "{rendered}");
    assert!(!rendered.contains("Pr_20") && !rendered.contains("Products"), "the other branch is gone: {rendered}");
    assert!(rendered.contains("Matches"), "the number of matches is shown: {rendered}");
    let other_system = text(&model(), &[], &["cs-din:331"], Locale::En);
    assert!(other_system.contains("Pr_20"), "a selection that names another system leaves this one whole");
}

#[semio_framework_async_macros::async_test]
async fn a_check_marks_the_code_every_selected_holder_carries_and_the_labels_follow_the_locale() {
    let marked = text(&model(), &["w-south"], &[], Locale::En);
    let plain = text(&model(), &["w-east"], &[], Locale::En);
    assert_ne!(marked, plain, "the classified wall marks its entry");
    let german = text(&model(), &[], &[], Locale::De);
    assert!(german.contains("Code oder Titel suchen") && german.contains("zuweisen"), "{german}");
    let empty = text(&ModelSnapshot::default(), &[], &[], Locale::En);
    assert!(empty.contains("No classification system in the library."), "{empty}");
}

#[semio_framework_async_macros::async_test]
async fn found_reads_the_codes_of_one_system_out_of_the_library_selection() {
    let selection = ["cs-uni:EF_25".to_string(), "cs-din:331".to_string(), "m-brick".to_string(), "cs-uni:Pr".to_string()];
    assert_eq!(found("cs-uni", &selection).into_iter().collect::<Vec<_>>(), ["EF_25", "Pr"]);
    assert!(found("cs-none", &selection).is_empty());
}
