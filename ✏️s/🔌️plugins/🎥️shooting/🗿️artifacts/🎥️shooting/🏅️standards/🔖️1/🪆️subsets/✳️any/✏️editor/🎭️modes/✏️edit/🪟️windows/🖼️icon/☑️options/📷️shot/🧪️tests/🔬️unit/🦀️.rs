use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
#[allow(unused_imports)]
use crate::ShootingSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::MeasureSelectItem;
use crate::editor::shooting::modes::edit::windows::icon::options::shot::*;
use crate::editor::shooting::terminology::shooting_play_labels;

#[semio_framework_async_macros::async_test]
async fn shot_measure_lists_every_shot() {
    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let labels = shooting_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    match measure(&snapshot, labels) {
        WindowMeasure::Select { items, .. } => assert_eq!(items.len(), snapshot.shots.len()),
        other => panic!("shot measure must be a select, got {other:?}"),
    }
}
