//! 🏅️ Standard root for `s.wfc.wfc3d@1` — `standard() -> StandardDeclaration`, mounting subset `any`,
//! this artifact's only subset. `mimes` is a documented synthesis (no MIME registration exists for
//! this kind outside the capability rows); `extensions` carries the real `wfc3d` value the codec
//! capability claims.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfc3d+json"], extensions: &["wfc3d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}
