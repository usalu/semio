use super::*;
use crate::editor::bim::modes::edit::windows::plan;
use crate::editor::bim::presence::BimPresenceMutation;
use crate::editor::bim::transient::BimWindowTransient;
use crate::editor::bim::unit_tests::context::view;
use crate::editor::bim::unit_tests::support::{demo, run};
use semio_framework_ui_locale::Locale;

fn addressed() -> BimDispatchCtx {
    let view = view(Locale::En, &[("bim-plan", plan::WINDOW_KIND_ID)], Some("bim-plan"));
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
