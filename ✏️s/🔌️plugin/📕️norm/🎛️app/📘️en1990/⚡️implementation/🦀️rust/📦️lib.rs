//! ⚖️ EN 1990 basis of structural design — document entities (constitutional: general).

use norm_core::AnnexChoice;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// 📊️ One variable action category/value pair for `Document.q_k` — a plain, un-tagged
/// `Vec<QkEntry>` list element (order-preserving: index determines "leading" in the combination
/// logic), reached only through that list so it needs no keyword of its own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct QkEntry {
    #[dsl(positional)]
    pub category: String,
    #[dsl(positional)]
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "en1990", layout = "lines")]
pub struct Document {
    pub g_k: f64,
    #[dsl(table)]
    pub q_k: Vec<QkEntry>,
    #[dsl(unit = "kN")]
    pub resistance_kn: f64,
    pub consequence_class: u8,
    pub annex: AnnexChoice,
    /// 🌍️ Seismic accidental action A_Ed [kN] combined per Eq. 6.12b; 0.0 disables the seismic situation.
    #[dsl(unit = "kN")]
    pub seismic_a_ed_kn: f64,
}

impl Default for Document {
    fn default() -> Self {
        Self { g_k: 100.0, q_k: vec![QkEntry { category: "office".into(), value: 50.0 }, QkEntry { category: "wind".into(), value: 30.0 }], resistance_kn: 300.0, consequence_class: 2, annex: AnnexChoice::De, seismic_a_ed_kn: 40.0 }
    }
}
//#endregion 🔖️Types
