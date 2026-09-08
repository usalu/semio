
use super::*;
use semio_framework_plugin::{Locale, ViewModel};

#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_by_default_and_in_german() {
    let english = ViewModel { locale: Locale::En, ..ViewModel::default() };
    assert_eq!(process3d_labels(&english).stock.as_str(), "Stock");
    let german = ViewModel { locale: Locale::De, ..ViewModel::default() };
    assert_eq!(process3d_labels(&german).stock.as_str(), "Rohteil");
}
