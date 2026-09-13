use super::*;
use crate::editor::generation3d::commands::{add_generation, update_generation_values};
use crate::editor::generation3d::unit_tests::context::{app, dispatch, render as render_body, snapshot, Generation3dApp};
use crate::editor::generation3d::Generation3dCommand;

#[semio_framework_async_macros::async_test]
async fn generate_form_hints_without_a_selected_generation() {
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
}

/// 📝️ The first slider question the bundled fixture's form spec exposes, plus the generation it
/// belongs to — the exact pair a user types into.
async fn first_slider_question(app: &mut Generation3dApp) -> (String, String, f64) {
    dispatch(app, Generation3dCommand::AddGeneration(add_generation::AddGeneration {})).await;
    let read = snapshot(app);
    let generation_id = read.generation.generations.first().expect("addGeneration seeds one generation").id.clone();
    let spec = flow_fixture_to_form_spec(&read.fixture);
    let question = spec
        .steps
        .iter()
        .flat_map(|step| step.blocks.iter())
        .find(|block| block.kind == "slider")
        .expect("the bundled fixture must expose at least one slider input to generate from");
    let question_id = question.id.clone();
    let current = read.generation.generations[0].values.get(&question_id).and_then(dsl::DslValue::as_f64).unwrap_or_default();
    (generation_id, question_id, current)
}

/// ⚖️ LAW: with a generation selected the Form window stops hinting and renders REAL controls, each
/// one bound to `updateGenerationValues` — the verb the window kind declares. Before this law the
/// only cited evidence for this window was "renders hint or real form chrome"
/// (`📓️audit-user-journey-gaps-2026-09-13.md` gap #8); nothing asserted the change binding exists.
#[semio_framework_async_macros::async_test]
async fn a_selected_generation_turns_the_form_into_bound_controls() {
    let mut app = app().await;
    let (generation_id, question_id, _) = first_slider_question(&mut app).await;
    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await;
    assert!(!body.contains("Add a generation"), "a selected generation must retire the hint: {body}");
    assert!(body.contains("updateGenerationValues"), "every form control must dispatch updateGenerationValues: {body}");
    assert!(body.contains(&format!("generate.form.{question_id}")), "the slider question must render its own field: {body}");
    assert!(body.contains(&generation_id), "each control must carry the generation it edits: {body}");
    eprintln!("[DEBUG] generate form generation={generation_id} question={question_id}");
}

/// ⚖️ LAW: editing a form value round-trips into the PATCHED fixture the generate preview evaluates.
/// `updateGenerationValues` writes the roster entry, and `generation_fixture_for` — the one input the
/// generate preview's mesh is computed from (`flow_eval_tick::evaluate`) — carries the new number onto
/// the matching widget. That is the whole mechanism by which typing into the Form window changes the
/// preview mesh; the browser step in `🐍️generate-mode-probe.mjs` proves the mesh itself.
#[semio_framework_async_macros::async_test]
async fn editing_a_form_value_repatches_the_generate_preview_fixture() {
    let mut app = app().await;
    let (generation_id, question_id, before) = first_slider_question(&mut app).await;
    let next = before + 3.0;
    dispatch(
        &mut app,
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
            generation_id: Some(generation_id.clone()),
            question_id: question_id.clone(),
            value: dsl::DslValue::float(next),
        }),
    )
    .await;

    let read = snapshot(&app);
    let stored = read.generation.generations[0].values.get(&question_id).and_then(dsl::DslValue::as_f64);
    assert_eq!(stored, Some(next), "updateGenerationValues must persist the typed number on the generation");
    let patched = crate::standards::v1::subsets::any::schema::generation_fixture_for(&read.fixture, &read.generation, Some(generation_id.as_str()));
    let patched_value = patched
        .widgets
        .iter()
        .find_map(|widget| match widget {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id, value, .. } if *id == question_id => Some(*value),
            _ => None,
        })
        .expect("the patched generate fixture must still carry the edited slider widget");
    patched.retire_cold();
    assert_eq!(patched_value, next, "the generate preview evaluates the PATCHED fixture, so the edited value has to reach it");
    eprintln!("[DEBUG] form edit question={question_id} before={before} after={patched_value}");

    let body = render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).await;
    assert!(body.contains(&next.to_string()), "the form must render back the value it just committed: {body}");
}
