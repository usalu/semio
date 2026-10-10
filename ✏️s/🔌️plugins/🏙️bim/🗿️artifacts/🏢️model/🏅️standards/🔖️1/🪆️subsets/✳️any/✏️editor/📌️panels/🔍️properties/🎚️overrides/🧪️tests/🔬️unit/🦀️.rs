use super::*;
use crate::editor::bim::entities::components::tests::placed;
use crate::ComponentOverride;
use semio_framework_plugin::{ui_node_list, PanelTreeBuilder, ViewModel};
use semio_framework_ui_locale::{Locale, Terminology};

fn with(overrides: &[(&str, &str)]) -> ModelSnapshot {
    let mut snapshot = placed();
    for (name, value) in overrides {
        snapshot.component_overrides.insert(format!("c-table.{name}"), ComponentOverride { component: "c-table".into(), name: (*name).into(), value: (*value).into() });
    }
    snapshot
}

fn labels(locale: Locale) -> &'static BimLabels {
    crate::editor::bim::terminology::bim_labels(&ViewModel::new(locale, Terminology::Native))
}

fn text(snapshot: &ModelSnapshot, locale: Locale) -> String {
    let nodes = render_rows(snapshot, "bim-properties", "c-table", labels(locale));
    let node = PanelTreeBuilder::new("bim-properties").and_then(|builder| builder.section("bim-properties.overrides", None, true, ui_node_list(nodes)?)).and_then(PanelTreeBuilder::build).expect("the rows build");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn every_parameter_of_the_family_is_a_row_in_dependency_order_with_its_family_formula() {
    let snapshot = with(&[]);
    let found = rows(&snapshot, "c-table", labels(Locale::En));
    let names: Vec<&str> = found.iter().map(|row| row.name.as_str()).collect();
    let family = snapshot.family_parameters.values().filter(|row| row.family == "fam-table").count();
    assert_eq!(found.len(), family);
    assert!(names.contains(&"width") && names.contains(&"depth"));
    let width = found.iter().find(|row| row.name == "width").expect("width");
    assert_eq!((width.family_formula.as_str(), width.formula.clone(), width.value.as_str(), width.overridden()), ("1.6 m", None, "1600 mm", false));
    assert!(found.iter().all(|row| row.issue.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn an_override_changes_the_value_of_its_parameter_and_of_the_ones_that_use_it() {
    let found = rows(&with(&[("width", "2 m")]), "c-table", labels(Locale::En));
    let width = found.iter().find(|row| row.name == "width").expect("width");
    assert_eq!((width.formula.as_deref(), width.value.as_str(), width.overridden()), (Some("2 m"), "2000 mm", true));
    assert_eq!(found.iter().filter(|row| row.overridden()).count(), 1);
    let plain = rows(&with(&[]), "c-table", labels(Locale::En));
    assert_eq!(plain.iter().find(|row| row.name == "depth"), found.iter().find(|row| row.name == "depth"), "an unrelated parameter keeps its value");
}

#[semio_framework_async_macros::async_test]
async fn an_override_that_fails_shows_its_issue_in_the_language_of_the_viewer() {
    let snapshot = with(&[("width", "missing_name * 2")]);
    let english = rows(&snapshot, "c-table", labels(Locale::En));
    let german = rows(&snapshot, "c-table", labels(Locale::De));
    let (en, de) = (english.iter().find(|row| row.name == "width").expect("width"), german.iter().find(|row| row.name == "width").expect("width"));
    assert!(!en.issue.is_empty() && !de.issue.is_empty());
    assert_ne!(en.issue, de.issue, "the issue text follows the locale");
    assert_eq!(en.value, "", "a parameter that fails has no value");
}

#[semio_framework_async_macros::async_test]
async fn a_component_that_does_not_exist_has_no_rows() {
    assert!(rows(&placed(), "c-ghost", labels(Locale::En)).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_panel_rows_offer_an_input_per_parameter_and_a_reset_only_for_an_overridden_one() {
    let plain = text(&with(&[]), Locale::En);
    assert!(plain.contains("bim-properties.override.width.input") && plain.contains("bim-properties.override.depth.input"), "{plain}");
    assert!(!plain.contains("Reset"), "nothing is overridden: {plain}");
    assert!(plain.contains("from the family"), "{plain}");
    let overridden = text(&with(&[("width", "2 m")]), Locale::En);
    assert!(overridden.contains("bim-properties.override.width.reset") && overridden.contains("Reset width to the family"), "{overridden}");
    assert_eq!(overridden.matches(".reset").count(), 1);
    assert!(overridden.contains("overridden"), "{overridden}");
}

#[semio_framework_async_macros::async_test]
async fn every_row_is_labelled_in_both_languages() {
    let german = text(&with(&[("width", "2 m")]), Locale::De);
    assert!(german.contains("Formel von width für diese Komponente") && german.contains("width auf die Familie zurücksetzen") && german.contains("abweichend"), "{german}");
    let english = text(&with(&[("width", "2 m")]), Locale::En);
    assert!(english.contains("Formula of width for this component"));
}
