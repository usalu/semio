//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the one subset `✳️any`.
//! `mimes`/`extensions` mirror `definition()`'s own codec capability row: the extension is
//! `BitmapSnapshot::EXTENSION` verbatim, the mime a documented synthesis (no registry entry exists
//! for a semio-native dialect), the same deviation every sibling artifact's standard root carries.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcbitmap+json"], extensions: &["wfcbitmap"] }, subsets: vec![subsets::any::subset::<PA>()] }
}
