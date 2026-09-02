//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::vdi3805::Vdi3805Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &["manufacturer_file", "catalog", "edition_profile", "correction_as_of", "strict_mode", "index", "geometry", "curves", "limits"];

/// 🧾️ `Vdi3805` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Vdi3805Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Vdi3805Outline {
    pub fn compute(snapshot: &Vdi3805Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.edition_profile.len() + snapshot.geometry.len() + snapshot.curves.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Vdi3805Outline {
    fn default() -> Self {
        Self::compute(&Vdi3805Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = Vdi3805Outline::compute(&Vdi3805Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = Vdi3805Snapshot::default();
        assert_eq!(Vdi3805Outline::compute(&snapshot), Vdi3805Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
