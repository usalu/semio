#[allow(unused_imports)]
use crate::editor::bim::gestures::session::{Modifiers, ToolEvent};
#[allow(unused_imports)]
use crate::editor::bim::gestures::Raw;
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::world_pointer_down::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn the_ground_point_is_the_first_two_finite_coordinates_and_a_press_without_one_does_nothing() {
    assert_eq!(ground_of(&[4.0, 5.0, 0.0]), Some([4.0, 5.0]));
    assert_eq!(ground_of(&[4.0]), None);
    assert_eq!(ground_of(&[f64::NAN, 1.0, 0.0]), None);
    let payload = WorldPointerDown { position: Vec::new(), ..Default::default() };
    let emit = run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).expect("nothing to do");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn the_host_modifiers_are_shift_ctrl_and_meta_key_and_the_ray_hit_is_the_ground() {
    let payload = WorldPointerDown { position: vec![1.0, 2.0, 0.0], shift_key: true, meta_key: true, ..Default::default() };
    let raw = payload.raw();
    assert_eq!((raw.ground, raw.modifiers.shift, raw.modifiers.ctrl, raw.modifiers.meta), (Some([1.0, 2.0]), true, false, true));
}
