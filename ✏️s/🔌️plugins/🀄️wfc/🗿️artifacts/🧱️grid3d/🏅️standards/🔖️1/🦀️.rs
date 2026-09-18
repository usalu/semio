//! 🏅️ Standard root for `s.wfc.grid3d@1` — `standard() -> StandardDeclaration`, mounting the one
//! subset this artifact has (`✳️any`). `extensions` is the real `Grid3dSnapshot::EXTENSION`;
//! `mimes` follows the documented `application/vnd.semio.<kind>+json` synthesis every sibling
//! artifact carries, since no external MIME registration exists for this dialect.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfcgrid3d+json"], extensions: &["wfcgrid3d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}
