
use super::*;
use crate::editor::process3d::config::Process3dConfig;

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_by_default_and_in_german() {
    let mut config = Process3dConfig::default();
    assert_eq!(process3d_labels(&config).stock.as_str(), "Stock");
    config.locale = "de".into();
    assert_eq!(process3d_labels(&config).stock.as_str(), "Rohteil");
}
