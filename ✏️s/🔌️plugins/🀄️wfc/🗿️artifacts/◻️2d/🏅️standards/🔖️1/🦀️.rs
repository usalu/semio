//! 🏅️ Standard root — `standard() -> StandardDeclaration`, mounting the one subset this artifact
//! has. `mimes`/`extensions` are the artifact's own: `.wfc2d` is the extension the native codec
//! claims (`ArtifactDsl::EXTENSION`), and the vendor MIME follows the same synthesis every sibling
//! artifact documents, since no MIME registration exists for a semio artifact kind.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub fn standard<PA: crate::ArtifactApps>() -> StandardDeclaration<PA> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.wfc2d+json"], extensions: &["wfc2d"] }, subsets: vec![subsets::any::subset::<PA>()] }
}
