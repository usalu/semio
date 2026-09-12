use super::*;
use crate::viewer::generation3d::unit_tests::context::{app, dispatch};
use crate::viewer::generation3d::Generation3dViewCommand;
use crate::viewer::generation3d::unit_tests::context;

/// 👁️ Dispatches through the REAL interactive-job pipeline — an action that is not `Migrated`, or
/// whose bounded reducer proof is missing, faults here instead of silently doing nothing.
#[semio_framework_async_macros::async_test]
async fn set_sun_azimuth_dispatches_live_and_leaves_the_document_untouched() {
    let _serial = crate::viewer::generation3d::unit_tests::context::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    let command = Generation3dViewCommand::SetSunAzimuth(SetSunAzimuth { value: 120.0 });
    assert_eq!(command.command_id(), "setSunAzimuth");
    dispatch(&mut app, command).await;
    assert_eq!(context::snapshot(&app), before, "a viewer command must never mutate the document");
}
