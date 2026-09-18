//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the single `✳️any` subset.
//! `extensions` carries this artifact's real `ArtifactDsl::EXTENSION` (`wfcgrid2d`, the same string
//! `definition()`'s `s.wfc.grid2d.codec.document-1` row claims); `mimes` is the documented
//! `application/vnd.semio.<artifact>+json` synthesis every sibling artifact uses, since no real
//! MIME registration exists for a semio-native dialect.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcgrid2d+json"], extensions: &["wfcgrid2d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}
