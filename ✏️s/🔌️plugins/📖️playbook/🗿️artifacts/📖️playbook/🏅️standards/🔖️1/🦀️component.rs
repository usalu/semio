//! 🏅️ Standard root — `standard() -> StandardDeclaration` (`terra-fleet-trinity-recipe` recipe,
//! `📓️terra-fleet-trinity-recipe-report.md`), mounts subset `any`. `mimes`/`extensions`: no real
//! MIME registration exists for `s.playbook.playbook` outside the old `ArtifactDefinition`'s
//! capability rows (`definition()`, kept — debt D1) — that channel only ever claimed a codec id
//! (`playbook.playbook:playbook`) and an extension (`playbook`), never a mime type.
//! `mimes: ["application/vnd.semio.playbook+json"]` is a documented synthesis (matches `🗒️note`/
//! `🖍️draw`/`🔱️trinity`'s identical documented deviation); `extensions: ["playbook"]` is the real,
//! carried-over value from `definition()`'s `s.playbook.codec.document.v1` row.

use crate::artifacts::playbook::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use semio_framework_plugin::StandardId;

pub async fn standard() -> StandardDeclaration {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.playbook+json"], extensions: &["playbook"] }, subsets: vec![subsets::any::subset()] }
}
