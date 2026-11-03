//! 🔩️ Beam-column joints of a ductile frame: the capacity design ratio `ΣM_Rc / (1.3 ΣM_Rb)` per joint row (EN 1998-1 4.4.2.3) and the storey level the joint sits at.

use super::part_1;
use semio_s_artifact_norm_en1998_model::{En1998Building, En1998Joint};

/// 🔩 The joint rows that belong to building `building_id`.
pub fn joints_of<'a>(joints: &'a [En1998Joint], building_id: &str) -> Vec<&'a En1998Joint> {
    joints.iter().filter(|j| j.building_id == building_id).collect()
}

/// 🏛️ Demand `1.3 ΣM_Rb` [N·m] on the column resistance of a joint (EN 1998-1 4.4.2.3(4)).
pub fn demand_nm(joint: &En1998Joint) -> f64 {
    part_1::STRONG_COLUMN_FACTOR * joint.beam_moment_rd_nm
}

/// 🏛️ Capacity design ratio `ΣM_Rc / (1.3 ΣM_Rb)` of a joint; at least 1 satisfies the rule, infinite without beam resistance.
pub fn ratio(joint: &En1998Joint) -> f64 {
    part_1::strong_column_ratio(joint.column_moment_rd_nm, joint.beam_moment_rd_nm)
}

/// ✅ Whether the joint satisfies `ΣM_Rc ≥ 1.3 ΣM_Rb`.
pub fn is_satisfied(joint: &En1998Joint) -> bool {
    joint.column_moment_rd_nm >= demand_nm(joint)
}

/// 🏬 Index of the storey of `building` the joint sits at; `None` for an unknown storey id.
pub fn storey_index(building: &En1998Building, joint: &En1998Joint) -> Option<usize> {
    building.storeys.iter().position(|s| s.id == joint.storey_id)
}

/// 📏 Level [m] above the building base of the top of the storey the joint sits at; `None` for an unknown storey id.
pub fn elevation_m(building: &En1998Building, joint: &En1998Joint) -> Option<f64> {
    storey_index(building, joint).map(|index| building.storeys.iter().take(index + 1).map(|s| s.height_m).sum())
}

/// 🏁 Whether the joint is exempt from the capacity design rule: the joints of the top storey of a multi-storey building need not satisfy ΣM_Rc ≥ 1.3 ΣM_Rb (EN 1998-1 4.4.2.3(4); the exemption is stated in the target spec `en1997-en1998-target.md` line 482, SEI-50, "exempt: top floor"; the code text itself is not checked, unverified).
pub fn is_top_storey_exempt(building: &En1998Building, joint: &En1998Joint) -> bool {
    building.storeys.len() > 1 && storey_index(building, joint) == Some(building.storeys.len() - 1)
}
