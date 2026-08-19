//! 📚️ Example `high-consequence-office`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "high-consequence-office";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("High Consequence Office", "High Consequence Office")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️high-consequence-office.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🏗️ The canonical snapshot `PRIMARY_TEXT` was printed from (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2, fixture regen per `📓️migration-recipe.md`
/// §7) — a CC3 (high-consequence) office building basis-of-design check with three variable-action
/// entries under the EN annex and the seismic accidental action disabled. Calling this mints the
/// `q_k` composed-child handle into the working-scene cache with the SAME content, so its
/// content-addressed `child_id` matches whatever `PRIMARY_TEXT`'s `qK=` line decodes to — the
/// standard way a caller (e.g. a test parsing `PRIMARY_TEXT` fresh) recovers the real entries
/// behind a parsed-from-text handle within this process.
pub async fn reference_snapshot() -> crate::artifacts::en1990::En1990Snapshot {
    let q_k = crate::artifacts::en1990::en1990_qk_child_from_entries(&[
        crate::artifacts::en1990::En1990QkEntry { category: "office".into(), value: 60.0 },
        crate::artifacts::en1990::En1990QkEntry { category: "partition-walls".into(), value: 12.0 },
        crate::artifacts::en1990::En1990QkEntry { category: "snow".into(), value: 18.0 },
    ]);
    crate::artifacts::en1990::En1990Snapshot { g_k: 250.0, q_k, resistance_kn: 420.0, consequence_class: 3, annex: crate::document::AnnexChoice::En, seismic_a_ed_kn: 0.0 }
}
