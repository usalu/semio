use super::*;
use crate::editor::generation3d::commands::{add_generation, remove_generation, rename_generation, select_generation};
use crate::editor::generation3d::unit_tests::context::{app, dispatch, render as render_body, snapshot, Generation3dApp};
use crate::editor::generation3d::Generation3dCommand;

/// 🎯️ Reads the selection back the way a USER sees it — off the rendered tree, through the one
/// affordance only the selected row carries (the inline rename editor). Deliberately NOT off the
/// artifact snapshot: the generate-mode bodies render against the CONFIG's selection, and the whole
/// point of `schema::generation_by_id` is that those two used to disagree.
async fn rendered_selected_id(app: &mut Generation3dApp, roster: &[String]) -> Option<String> {
    let body = render_body(app, GENERATION_3D_PLAY_BODY_GENERATIONS).await;
    roster.iter().find(|id| body.contains(&format!("procedural3d-play-generate.generation.{id}.rename"))).cloned()
}

/// 🗂️ Seeds `count` generations through the SAME `addGeneration` the tree row dispatches, and returns
/// their ids in roster order — never a hand-built `GenerationPlayState`, because the roster a user
/// sees is the one the retained command actually published.
async fn seed_generations(app: &mut Generation3dApp, count: usize) -> Vec<String> {
    for _ in 0..count {
        dispatch(app, Generation3dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    }
    let ids: Vec<String> = snapshot(app).generation.generations.iter().map(|entry| entry.id.clone()).collect();
    assert_eq!(ids.len(), count, "addGeneration must publish one roster entry per dispatch");
    ids
}

#[semio_framework_async_macros::async_test]
async fn generate_mode_renders_surfaces() {
    let mut app = app().await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATIONS).await;
    assert!(body.contains("addGeneration"), "the generate-mode generations window must offer the add action: {body}");
}

/// ⚖️ LAW: every one of the Generations window's four row verbs is REACHABLE from the rendered tree,
/// not merely declared on the window kind. The row itself activates `selectGeneration`, the row
/// carries a `removeGeneration` row-action button, and the SELECTED row carries the inline rename
/// editor that dispatches `renameGeneration`.
///
/// 🐛️ Before this law `renameGeneration`/`removeGeneration` were `RowActionPlacement::Menu` —
/// right-click-only — and rename dispatched a hardcoded `"{name} copy"`, so the only browser-proven
/// verb in this window was `addGeneration` (`📓️audit-user-journey-gaps-2026-09-13.md` gap #7).
#[semio_framework_async_macros::async_test]
async fn every_generation_row_verb_is_reachable_from_the_rendered_tree() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 2).await;
    dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: ids[1].clone() })).await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATIONS).await;
    for verb in ["selectGeneration", "renameGeneration", "removeGeneration", "addGeneration"] {
        assert!(body.contains(verb), "the generations tree must emit {verb}: {body}");
    }
    assert!(!body.contains("\"placement\":\"menu\""), "row actions must paint ON the row, never only in its right-click menu: {body}");
    assert!(body.contains(&format!("procedural3d-play-generate.generation.{}.rename", ids[1])), "the selected row must carry the inline rename editor: {body}");
    assert!(!body.contains(&format!("procedural3d-play-generate.generation.{}.rename", ids[0])), "only the selected row carries the rename editor: {body}");
    eprintln!("[DEBUG] generations rows={} selected={}", ids.len(), ids[1]);
}

/// ⚖️ LAW: the inline rename editor's typed text reaches `renameGeneration`. A scalar `Trigger::Commit`
/// payload is named `value` by the framework, never `name` (`uiIntentPayload`, `🛠️ShellHelpers/🟦️.tsx`),
/// so the guest's action bridge has to read BOTH spellings or every inline rename silently renames to
/// the empty string.
#[test]
fn an_inline_rename_commit_carries_its_typed_text_as_value() {
    let args: dsl::DslValue = serde_json::json!({ "id": "generation-1", "value": "Balcony Study" }).into();
    let command = <crate::editor::generation3d::Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::command_from_action("renameGeneration", Some(&args)).expect("renameGeneration bridges");
    let Generation3dCommand::RenameGeneration(payload) = command else { panic!("renameGeneration must bridge to its own command row") };
    assert_eq!(payload, rename_generation::RenameGeneration { id: "generation-1".into(), name: "Balcony Study".into() });
}

/// ⚖️ LAW: the roster a user edits through the three row verbs converges — select the second
/// generation, rename it, remove the first, and both the surviving name and the selection are exactly
/// what the gestures asked for. The browser step in `🐍️generate-mode-probe.mjs` drives the same
/// sequence through the real tree rows.
#[semio_framework_async_macros::async_test]
async fn select_rename_and_remove_converge_on_the_roster_the_user_asked_for() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 2).await;
    dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: ids[1].clone() })).await;
    assert_eq!(rendered_selected_id(&mut app, &ids).await, Some(ids[1].clone()), "selectGeneration must move the selection the generate-mode windows render against");

    dispatch(&mut app, Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: ids[1].clone(), name: "Balcony Study".into() })).await;
    dispatch(&mut app, Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: ids[0].clone() })).await;

    let names: Vec<String> = {
        let after = snapshot(&app);
        after.generation.generations.iter().map(|entry| entry.name.clone()).collect()
    };
    assert_eq!(names, vec!["Balcony Study".to_string()], "the removed generation must be gone and the renamed one must keep its new name");
    assert_eq!(rendered_selected_id(&mut app, &ids).await, Some(ids[1].clone()), "removing another row must not steal the selection");

    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATIONS).await;
    assert!(body.contains("Balcony Study"), "the rendered roster must show the new name: {body}");
    assert!(!body.contains(&format!("generation.{}\"", ids[0])), "the removed row must be gone from the rendered roster: {body}");
    eprintln!("[DEBUG] roster after select/rename/remove names={names:?}");
}

/// ⚖️ LAW: the Generations window speaks both declared languages with no default — the German locale
/// must not fall back to an English row-action label or an English rename placeholder.
#[semio_framework_async_macros::async_test]
async fn generation_row_affordances_are_localized_in_german() {
    let mut app = app().await;
    let ids = seed_generations(&mut app, 1).await;
    dispatch(&mut app, Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: ids[0].clone() })).await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let body = crate::editor::generation3d::unit_tests::context::render_with_view(&mut app, GENERATION_3D_PLAY_BODY_GENERATIONS, &view_state).await;
    for german in ["Entfernen", "Generierung umbenennen", "Generierungen", "Generierung hinzufügen"] {
        assert!(body.contains(german), "the German generations tree must carry {german}: {body}");
    }
    assert!(!body.contains("Add Generation"), "no English fallback may survive in the German tree: {body}");
}
