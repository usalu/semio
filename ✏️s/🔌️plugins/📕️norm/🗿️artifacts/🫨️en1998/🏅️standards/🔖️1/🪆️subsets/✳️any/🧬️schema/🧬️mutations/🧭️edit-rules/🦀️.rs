//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the EN1998 editor: which value-tree path raises which concrete kind.

use crate::app_surface::{InsertItemRule, NormEditRules, RemoveItemRule, SelectorKey, SetFieldRule};

/// 📚 Every path an EN1998 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field: &[
        SetFieldRule { path: "towers[].mRdNm", kind: "change-tower-m-rd-nm", selectors: &[SelectorKey::Index("index")], value: "newMRdNm" },
        SetFieldRule { path: "buildings[].storeys[].permanentGkN", kind: "change-storey-permanent-gk-n", selectors: &[SelectorKey::Index("buildingIndex"), SelectorKey::Index("storeyIndex")], value: "newPermanentGkN" },
        SetFieldRule { path: "buildings[].members[].detailingCompatibleWithQ", kind: "change-member-detailing", selectors: &[SelectorKey::Index("buildingIndex"), SelectorKey::Index("memberIndex")], value: "newDetailingCompatibleWithQ" },
        SetFieldRule { path: "assessments[].rKN", kind: "change-assessment-rkn", selectors: &[SelectorKey::Index("index")], value: "newRKN" },
        SetFieldRule { path: "buildings[].systems[].baseShearResistanceN", kind: "change-system-v-rd-n", selectors: &[SelectorKey::Index("buildingIndex"), SelectorKey::Index("systemIndex")], value: "newBaseShearResistanceN" },
        SetFieldRule { path: "annex", kind: "change-annex", selectors: &[], value: "newAnnex" },
        SetFieldRule { path: "buildings[].elevationRegular", kind: "change-elevation-regular", selectors: &[SelectorKey::Index("buildingIndex")], value: "newElevationRegular" },
        SetFieldRule { path: "buildings[].storeys[].driftXM", kind: "change-storey-drift-xm", selectors: &[SelectorKey::Index("buildingIndex"), SelectorKey::Index("storeyIndex")], value: "newDriftXM" },
        SetFieldRule { path: "buildings[].storeys[].stiffnessX", kind: "change-storey-stiffness-x", selectors: &[SelectorKey::Index("buildingIndex"), SelectorKey::Index("storeyIndex")], value: "newStiffnessX" },
        SetFieldRule { path: "bridges[].vRdN", kind: "change-bridge-v-rd-n", selectors: &[SelectorKey::Index("index")], value: "newVRdN" },
        SetFieldRule { path: "buildings[].planRegular", kind: "change-building-plan-regular", selectors: &[SelectorKey::Index("buildingIndex")], value: "newPlanRegular" },
        SetFieldRule { path: "buildings[].masonryWallAreaRatio", kind: "change-masonry-wall-ratio", selectors: &[SelectorKey::Index("buildingIndex")], value: "newMasonryWallAreaRatio" },
    ],
    insert_item: &[
        InsertItemRule { path: "buildings", kind: "insert-building", selectors: &[], index: "index", item: "building" },
        InsertItemRule { path: "bridges", kind: "insert-bridge", selectors: &[], index: "index", item: "bridge" },
        InsertItemRule { path: "assessments", kind: "insert-assessment", selectors: &[], index: "index", item: "assessment" },
        InsertItemRule { path: "towers", kind: "insert-tower", selectors: &[], index: "index", item: "tower" },
        InsertItemRule { path: "tanks", kind: "insert-tank", selectors: &[], index: "index", item: "tank" },
        InsertItemRule { path: "retainingWalls", kind: "insert-retaining-wall", selectors: &[], index: "index", item: "wall" },
        InsertItemRule { path: "foundations", kind: "insert-foundation", selectors: &[], index: "index", item: "foundation" },
        InsertItemRule { path: "silos", kind: "insert-silo", selectors: &[], index: "index", item: "silo" },
    ],
    remove_item: &[
        RemoveItemRule { path: "assessments", kind: "remove-assessment", selectors: &[], index: "index" },
        RemoveItemRule { path: "bridges", kind: "remove-bridge", selectors: &[], index: "index" },
        RemoveItemRule { path: "buildings", kind: "remove-building", selectors: &[], index: "index" },
        RemoveItemRule { path: "foundations", kind: "remove-foundation", selectors: &[], index: "index" },
        RemoveItemRule { path: "retainingWalls", kind: "remove-retaining-wall", selectors: &[], index: "index" },
        RemoveItemRule { path: "silos", kind: "remove-silo", selectors: &[], index: "index" },
        RemoveItemRule { path: "tanks", kind: "remove-tank", selectors: &[], index: "index" },
        RemoveItemRule { path: "towers", kind: "remove-tower", selectors: &[], index: "index" },
    ],
};
