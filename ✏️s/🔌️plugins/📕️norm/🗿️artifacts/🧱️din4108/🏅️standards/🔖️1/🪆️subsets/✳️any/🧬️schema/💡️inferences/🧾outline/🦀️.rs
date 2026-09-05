//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::din4108::Din4108Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "category",
    "layers",
    "climate",
    "airtightness_n50",
    "psi_times_l_sum",
    "rh_int",
    "catalog_id",
    "material_id",
    "airtightness_class",
    "t_int_c",
    "solar_absorptance",
    "irradiance_w_m2",
    "moisture_mu_exterior",
    "moisture_mu_interior",
    "envelope_area_m2",
    "bb2_details_conform",
    "application_type",
    "declared_application_class",
];

/// 🧾️ `Din4108` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din4108Outline {
    pub fn compute(snapshot: &Din4108Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = snapshot.layers.len() as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din4108Outline {
    fn default() -> Self {
        Self::compute(&Din4108Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = Din4108Outline::compute(&Din4108Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = Din4108Snapshot::default();
        assert_eq!(Din4108Outline::compute(&snapshot), Din4108Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
