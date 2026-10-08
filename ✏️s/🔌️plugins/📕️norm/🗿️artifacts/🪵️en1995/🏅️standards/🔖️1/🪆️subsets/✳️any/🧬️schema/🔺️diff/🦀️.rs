//! 🧬️ EN1995 diff schema — sparse scalar fields plus positional row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};

use crate::En1995Snapshot;

//#region 🔖️Rows
protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `CharacteristicAction`.
    pub En1995MemberActionPatch of crate::CharacteristicAction { set { kind: String, category: String, load_duration: String, q_line_n_per_m: f64, f_point_n: f64, m_k_nm: f64, v_k_n: f64, n_k_n: f64, n_t_k_n: f64, f_c90_k_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `CharacteristicAction` list.
    pub En1995MemberActionDelta { removal: En1995MemberActionRemoval, insertion: En1995MemberActionInsertion, relocation: En1995MemberActionRelocation, modification: En1995MemberActionModification, row: crate::CharacteristicAction, patch: En1995MemberActionPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConnectionAction`.
    pub En1995ConnectionActionPatch of crate::ConnectionAction { set { kind: String, load_duration: String, f_k_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConnectionAction` list.
    pub En1995ConnectionActionDelta { removal: En1995ConnectionActionRemoval, insertion: En1995ConnectionActionInsertion, relocation: En1995ConnectionActionRelocation, modification: En1995ConnectionActionModification, row: crate::ConnectionAction, patch: En1995ConnectionActionPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TimberMember`.
    pub En1995MemberPatch of crate::TimberMember { set { label_en: String, label_de: String, role: crate::MemberRole, strength_class: String, service_class: u8, support: crate::SupportType, b_m: f64, h_m: f64, span_m: f64, support_length_m: f64, bearing_length_m: f64, buckling_length_y_m: f64, buckling_length_z_m: f64, lateral_restraint_spacing_m: f64, notch_depth_m: f64, notch_distance_m: f64, m_crit_nm: f64, mass_kg_per_m: f64, mass_kg_per_m2: f64, damping_xi: f64, fire_duration_s: f64, bridge_n_obs: f64, bridge_t_l_years: f64, bridge_beta: f64, bridge_a: f64, bridge_b: f64, bridge_crowd_per_m2: f64 } nest { actions: En1995MemberActionDelta } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `TimberMember` list.
    pub En1995MemberDelta { removal: En1995MemberRemoval, insertion: En1995MemberInsertion, relocation: En1995MemberRelocation, modification: En1995MemberModification, row: crate::TimberMember, patch: En1995MemberPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TimberConnection`.
    pub En1995ConnectionPatch of crate::TimberConnection { set { label_en: String, label_de: String, fastener_type: String, strength_class: String, service_class: u8, diameter_m: f64, number: u32, rows: u32, spacing_m: f64, edge_distance_m: f64, end_distance_m: f64, t1_m: f64, t2_m: f64, steel_plate: bool, steel_plate_thickness_m: f64, shear_planes: u32, f_u_k: f64 } nest { actions: En1995ConnectionActionDelta } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `TimberConnection` list.
    pub En1995ConnectionDelta { removal: En1995ConnectionRemoval, insertion: En1995ConnectionInsertion, relocation: En1995ConnectionRelocation, modification: En1995ConnectionModification, row: crate::TimberConnection, patch: En1995ConnectionPatch, key: id }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1995 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub members: En1995MemberDelta,
    #[state(artifact)]
    pub connections: En1995ConnectionDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1995Snapshot> for En1995Diff {
    fn apply(&self, base: &En1995Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1995Snapshot> {
        Ok(En1995Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            members: self.members.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?,
            connections: self.connections.commit_onto(&base.connections, capability).map_err(|error| error.under(["connections"]))?,
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        self.members.absorb(other.members);
        self.connections.absorb(other.connections);
    }
}

impl DiffAlgebra<En1995Snapshot> for En1995Diff {
    fn inverse(&self, base: &En1995Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            members: self.members.inverse(&base.members),
            connections: self.connections.inverse(&base.connections),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.members.is_empty()
            && self.connections.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
