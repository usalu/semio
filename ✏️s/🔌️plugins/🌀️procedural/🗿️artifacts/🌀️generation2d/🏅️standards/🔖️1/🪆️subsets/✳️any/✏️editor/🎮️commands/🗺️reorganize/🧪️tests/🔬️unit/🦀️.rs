use super::*;
use crate::editor::generation2d::unit_tests::context::{app, close, dispatch, snapshot_read};
use crate::editor::generation2d::Generation2dCommand;

#[semio_framework_async_macros::async_test]
async fn reorganize_emits_operations() {
    let mut app = app().await;
    let placements = |app: &crate::editor::generation2d::unit_tests::context::Generation2dApp| -> Vec<(String, u64, u64)> {
        snapshot_read(app).fixture.layout.iter().map(|(id, layout)| (id.clone(), layout.x.to_bits(), layout.y.to_bits())).collect()
    };
    let before = placements(&app);
    dispatch(&mut app, Generation2dCommand::Reorganize(Reorganize {})).await;
    let after = placements(&app);
    close(app);
    assert_ne!(before, after);
}
