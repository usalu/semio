use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_framework_plugin::{Emit, NoConfigMutation};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_gestures::editor::bim::gestures::keep_entry;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_gestures::editor::bim::commands::engagement_input::{handle, EngagementInput};
use semio_s_artifact_bim_model_tests::editor::bim::presence::BimPresenceMutation;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::transient::BimWindowTransient;
use semio_s_artifact_bim_model_bim::editor::bim::fixture::context::view;
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{demo, run};
use semio_framework_ui_locale::Locale;

fn addressed() -> BimDispatchCtx {
    let view = view(Locale::En, &[("bim-plan", semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID)], Some("bim-plan"));
    BimDispatchCtx::new(Vec::new(), Vec::new(), Some(&view), None, None)
}

fn type_into(ctx: &mut BimDispatchCtx, value: &str) -> Emit<ModelMutation, NoConfigMutation> {
    run(&demo(), |doc, cfg| handle(&EngagementInput { value: value.into() }, doc, cfg, ctx)).expect("keeps the line")
}

#[semio_framework_async_macros::async_test]
async fn the_typed_line_goes_to_the_window_transient_and_the_presence_and_never_to_the_document() {
    let mut ctx = addressed();
    let emit = type_into(&mut ctx, "3, 4");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
    assert_eq!(ctx.transient_out.as_ref().map(|transient| transient.engagement_input.as_str()), Some("3, 4"));
    assert!(matches!(ctx.presence_out.as_slice(), [BimPresenceMutation::Set { engagement_input, .. }] if engagement_input == "3, 4"));
}

#[semio_framework_async_macros::async_test]
async fn a_line_that_is_already_kept_writes_nothing_again() {
    let mut ctx = addressed();
    ctx.window_transient = BimWindowTransient { engagement_input: "3, 4".into(), ..Default::default() };
    ctx.presence.engagement_input = "3, 4".into();
    type_into(&mut ctx, "3, 4");
    assert!(ctx.transient_out.is_none() && ctx.presence_out.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn without_an_addressed_window_there_is_nothing_to_keep_it_in() {
    let mut ctx = BimDispatchCtx::default();
    type_into(&mut ctx, "3, 4");
    assert!(ctx.transient_out.is_none() && ctx.presence_out.is_empty());
}
