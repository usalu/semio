use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_combination::*;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::NoConfig;
use crate::context::dispatch;
use crate::context::fem3d_demo_app;

fn refusal(text: &str) -> String {
    let snapshot = crate::context::Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot::default());
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&*snapshot, &history);
    let config = NoConfig::default();
    let fault = handle(&AddCombination { name: "ULS".into(), terms: text.into() }, &doc, &ConfigView { snapshot: &config, window: None }).err().expect("a term list the document cannot honour is refused with a fault");
    format!("{fault:?}")
}

/// 🔗️ LAW: `addCombination` refuses terms that are not JSON, that name no term, an unknown or repeated case, or a non-finite factor with the visible `fem3d.combination.terms` fault instead of publishing nothing.
#[semio_framework_async_macros::async_test]
async fn add_combination_refuses_terms_it_cannot_honour_with_a_fault() {
    for text in ["", "dead", "dead:1.35", "[]", "{}", "[[\"ghost\",1.0]]", "[[\"dead\",1.0],[\"dead\",2.0]]", "[[\"dead\"]]", "[[\"dead\",\"heavy\"]]", "7"] {
        assert!(refusal(text).contains("fem3d.combination.terms"), "{text}");
    }
}

/// 🔗️ LAW: every documented terms spelling — pairs, records and one case-to-factor object — authors the same combination through the real dispatch, so the action palette form can always author a valid value.
#[semio_framework_async_macros::async_test]
async fn add_combination_accepts_pairs_records_and_a_factor_object() {
    let mut app = fem3d_demo_app().await;
    let case = app.snapshot().expect("snapshot").load_cases[0].id.clone();
    let spellings = [format!("[[\"{case}\",1.35]]"), format!("[{{\"caseId\":\"{case}\",\"factor\":1.35}}]"), format!("{{\"{case}\":1.35}}")];
    for (index, terms) in spellings.into_iter().enumerate() {
        dispatch(&mut app, Fem3dCommand::AddCombination(AddCombination { name: format!("ULS{index}"), terms })).await;
        let snapshot = app.snapshot().expect("snapshot");
        let added = snapshot.combinations.iter().find(|combination| combination.name == format!("ULS{index}")).unwrap_or_else(|| panic!("spelling {index} authored a combination"));
        assert_eq!(added.terms.get(&case), Some(&1.35));
    }
}

/// 🔗️ LAW: the natural spellings a user types — `dead:1.35,live:1.5`, `dead=1.35, live=1.5`, `1.35 dead + 1.5 live`, `1.35*dead; 1.5*live`, a case NAME in any letter case, and JSON — all author the same combination.
#[semio_framework_async_macros::async_test]
async fn add_combination_accepts_the_natural_spellings_users_type_3d() {
    let mut app = fem3d_demo_app().await;
    let cases = app.snapshot().expect("snapshot").load_cases.iter().map(|case| (case.id.clone(), case.name.clone())).collect::<Vec<_>>();
    let ((dead, dead_name), (live, _)) = (cases.iter().find(|(id, _)| id == "dead").cloned().expect("dead"), cases.iter().find(|(id, _)| id == "live").cloned().expect("live"));
    let spellings = [
        format!("{dead}:1.35,{live}:1.5"),
        format!("{dead}=1.35, {live}=1.5"),
        format!("1.35 {dead} + 1.5 {live}"),
        format!("{dead} 1.35; {live} 1.5"),
        format!("1.35*{dead}\n1.5*{live}"),
        format!("{}:1.35, {live}:1.5", dead_name.to_uppercase()),
        format!("[[\"{dead}\",1.35],[\"{live}\",1.5]]"),
        format!("{{\"{dead}\":1.35,\"{live}\":1.5}}"),
    ];
    for (index, terms) in spellings.into_iter().enumerate() {
        dispatch(&mut app, Fem3dCommand::AddCombination(AddCombination { name: format!("NAT{index}"), terms: terms.clone() })).await;
        let snapshot = app.snapshot().expect("snapshot");
        let added = snapshot.combinations.iter().find(|combination| combination.name == format!("NAT{index}")).unwrap_or_else(|| panic!("'{terms}' authored a combination"));
        assert_eq!((added.terms.get(&dead), added.terms.get(&live), added.terms.len()), (Some(&1.35), Some(&1.5), 2), "{terms}");
    }
}

/// 🔗️ LAW: every refusal of `terms` carries a complete EN and DE notice whose placeholders its params fill, so the shell shows readable text instead of a code.
#[test]
fn add_combination_refusals_have_complete_localized_notices_3d() {
    let table = semio_framework::fault_notice_definitions(<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp as semio_plugin::ArtifactEditor>::fault_notices());
    assert!(semio_framework::validate_fault_notices(&table).is_empty(), "{:?}", semio_framework::validate_fault_notices(&table));
    let fault = |text: &str| -> semio_plugin::Fault { let snapshot = crate::context::Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot::default()); parse_terms(text, &snapshot).err().expect("refused") };
    for (text, code, english, german) in [("dead 1 2", "fem3d.combination.terms-unreadable", "could not be read", "nicht gelesen"), ("", "fem3d.combination.terms-empty", "at least one term", "mindestens einen Term"), ("ghost:1", "fem3d.combination.terms-case-unknown", "'ghost' is not a load case", "'ghost' ist kein Lastfall")] {
        let refused = fault(text);
        assert_eq!(refused.code.0.as_str(), code, "{text}");
        for (locale, expected) in [(semio_ui_locale::Locale::En, english), (semio_ui_locale::Locale::De, german)] {
            let shown = semio_framework::fault_notice_text(&table, code, refused.params.as_deref(), semio_ui_locale::Terminology::Native, locale).expect("notice");
            assert!(shown.contains(expected), "{locale:?}: {shown}");
        }
    }
}
