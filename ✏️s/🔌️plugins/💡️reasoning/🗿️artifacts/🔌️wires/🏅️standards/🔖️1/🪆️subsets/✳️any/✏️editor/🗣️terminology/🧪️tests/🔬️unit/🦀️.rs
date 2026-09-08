
use super::*;
use semio_framework_plugin::{AppLabels, Locale, Terminology};

#[semio_framework_async_macros::async_test]
async fn relationship_kind_display_name_resolves_labels() {
    assert_eq!(relationship_kind_display_name("is", WiresLabels::labels(Locale::En, Terminology::Native)), "Is");
    assert_eq!(relationship_kind_display_name("unknown", WiresLabels::labels(Locale::En, Terminology::Native)), "unknown");
}
