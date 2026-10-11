//! 🧬️ En1999 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

use crate::En1999Snapshot;

use protocol::list_delta::Keyed as _;

/// 🩹️ Sparse patch of the `materials` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MaterialsPatch {
    pub designation: Option<String>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::AluminiumMaterial> for En1999MaterialsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::AluminiumMaterial, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.designation {
            row.designation = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.designation.is_some() {
            self.designation = later.designation;
        }
    }

    fn inverse(&self, row: &crate::snapshot::AluminiumMaterial) -> Self {
        Self {
            designation: self.designation.as_ref().map(|_| row.designation.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.designation.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `materials` list (rows keyed by `id`).
    pub En1999MaterialsRows { removal: En1999MaterialsRemoved, insertion: En1999MaterialsInserted, relocation: En1999MaterialsMoved, modification: En1999MaterialsModified, row: crate::snapshot::AluminiumMaterial, patch: En1999MaterialsPatch, key: id }
}

impl En1999MaterialsRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::AluminiumMaterial], new: &[crate::snapshot::AluminiumMaterial]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999MaterialsRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999MaterialsInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `elements` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsElementsPatch {
    pub thickness: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::PlateElement> for En1999SectionsElementsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::PlateElement, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.thickness {
            row.thickness = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.thickness.is_some() {
            self.thickness = later.thickness;
        }
    }

    fn inverse(&self, row: &crate::snapshot::PlateElement) -> Self {
        Self {
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.thickness.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `elements` list (rows keyed by `id`).
    pub En1999SectionsElementsRows { removal: En1999SectionsElementsRemoved, insertion: En1999SectionsElementsInserted, relocation: En1999SectionsElementsMoved, modification: En1999SectionsElementsModified, row: crate::snapshot::PlateElement, patch: En1999SectionsElementsPatch, key: id }
}

/// 🩹️ Sparse patch of the `sections` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsPatch {
    pub kind: Option<String>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub flange_thickness: Option<f64>,
    pub web_thickness: Option<f64>,
    pub outer_diameter: Option<f64>,
    pub elements: Option<En1999SectionsElementsRows>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::AluminiumSection> for En1999SectionsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::AluminiumSection, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind {
            row.kind = value.clone();
        }
        if let Some(value) = &self.height {
            row.height = value.clone();
        }
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.flange_thickness {
            row.flange_thickness = value.clone();
        }
        if let Some(value) = &self.web_thickness {
            row.web_thickness = value.clone();
        }
        if let Some(value) = &self.outer_diameter {
            row.outer_diameter = value.clone();
        }
        if let Some(rows) = &self.elements {
            row.elements = rows.commit_onto(&row.elements, capability).map_err(|error| error.under(["elements"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() {
            self.kind = later.kind;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.flange_thickness.is_some() {
            self.flange_thickness = later.flange_thickness;
        }
        if later.web_thickness.is_some() {
            self.web_thickness = later.web_thickness;
        }
        if later.outer_diameter.is_some() {
            self.outer_diameter = later.outer_diameter;
        }
        if let Some(theirs) = later.elements {
            match self.elements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.elements = Some(theirs),
            }
        }
        self.elements = self.elements.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &crate::snapshot::AluminiumSection) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            height: self.height.as_ref().map(|_| row.height.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            flange_thickness: self.flange_thickness.as_ref().map(|_| row.flange_thickness.clone()),
            web_thickness: self.web_thickness.as_ref().map(|_| row.web_thickness.clone()),
            outer_diameter: self.outer_diameter.as_ref().map(|_| row.outer_diameter.clone()),
            elements: self.elements.as_ref().map(|rows| rows.inverse(&row.elements)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.height.is_none() && self.width.is_none() && self.flange_thickness.is_none() && self.web_thickness.is_none() && self.outer_diameter.is_none() && self.elements.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `sections` list (rows keyed by `id`).
    pub En1999SectionsRows { removal: En1999SectionsRemoved, insertion: En1999SectionsInserted, relocation: En1999SectionsMoved, modification: En1999SectionsModified, row: crate::snapshot::AluminiumSection, patch: En1999SectionsPatch, key: id }
}

impl En1999SectionsRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::AluminiumSection], new: &[crate::snapshot::AluminiumSection]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999SectionsRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999SectionsInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersActionsPatch {
    pub n_k: Option<f64>,
    pub m_y_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::MemberAction> for En1999MembersActionsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::MemberAction, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.n_k {
            row.n_k = value.clone();
        }
        if let Some(value) = &self.m_y_k {
            row.m_y_k = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.n_k.is_some() {
            self.n_k = later.n_k;
        }
        if later.m_y_k.is_some() {
            self.m_y_k = later.m_y_k;
        }
    }

    fn inverse(&self, row: &crate::snapshot::MemberAction) -> Self {
        Self {
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.n_k.is_none() && self.m_y_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1999MembersActionsRows { removal: En1999MembersActionsRemoved, insertion: En1999MembersActionsInserted, relocation: En1999MembersActionsMoved, modification: En1999MembersActionsModified, row: crate::snapshot::MemberAction, patch: En1999MembersActionsPatch, key: id }
}

/// 🩹️ Sparse patch of the `members` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersPatch {
    pub buckling_length_y: Option<f64>,
    pub buckling_length_z: Option<f64>,
    pub buckling_length_t: Option<f64>,
    pub ltb_length: Option<f64>,
    pub actions: Option<En1999MembersActionsRows>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::AluminiumMember> for En1999MembersPatch {
    fn commit_into(&self, row: &mut crate::snapshot::AluminiumMember, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.buckling_length_y {
            row.buckling_length_y = value.clone();
        }
        if let Some(value) = &self.buckling_length_z {
            row.buckling_length_z = value.clone();
        }
        if let Some(value) = &self.buckling_length_t {
            row.buckling_length_t = value.clone();
        }
        if let Some(value) = &self.ltb_length {
            row.ltb_length = value.clone();
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.buckling_length_y.is_some() {
            self.buckling_length_y = later.buckling_length_y;
        }
        if later.buckling_length_z.is_some() {
            self.buckling_length_z = later.buckling_length_z;
        }
        if later.buckling_length_t.is_some() {
            self.buckling_length_t = later.buckling_length_t;
        }
        if later.ltb_length.is_some() {
            self.ltb_length = later.ltb_length;
        }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &crate::snapshot::AluminiumMember) -> Self {
        Self {
            buckling_length_y: self.buckling_length_y.as_ref().map(|_| row.buckling_length_y.clone()),
            buckling_length_z: self.buckling_length_z.as_ref().map(|_| row.buckling_length_z.clone()),
            buckling_length_t: self.buckling_length_t.as_ref().map(|_| row.buckling_length_t.clone()),
            ltb_length: self.ltb_length.as_ref().map(|_| row.ltb_length.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.buckling_length_y.is_none() && self.buckling_length_z.is_none() && self.buckling_length_t.is_none() && self.ltb_length.is_none() && self.actions.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `members` list (rows keyed by `id`).
    pub En1999MembersRows { removal: En1999MembersRemoved, insertion: En1999MembersInserted, relocation: En1999MembersMoved, modification: En1999MembersModified, row: crate::snapshot::AluminiumMember, patch: En1999MembersPatch, key: id }
}

impl En1999MembersRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::AluminiumMember], new: &[crate::snapshot::AluminiumMember]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999MembersRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999MembersInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `connections` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ConnectionsPatch {
    pub welds_throat: Option<f64>,
    pub bolts_rows: Option<u32>,
    pub bolts_bolts_per_row: Option<u32>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::AluminiumConnection> for En1999ConnectionsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::AluminiumConnection, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.welds_throat {
            row.welds.throat = value.clone();
        }
        if let Some(value) = &self.bolts_rows {
            row.bolts.rows = value.clone();
        }
        if let Some(value) = &self.bolts_bolts_per_row {
            row.bolts.bolts_per_row = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.welds_throat.is_some() {
            self.welds_throat = later.welds_throat;
        }
        if later.bolts_rows.is_some() {
            self.bolts_rows = later.bolts_rows;
        }
        if later.bolts_bolts_per_row.is_some() {
            self.bolts_bolts_per_row = later.bolts_bolts_per_row;
        }
    }

    fn inverse(&self, row: &crate::snapshot::AluminiumConnection) -> Self {
        Self {
            welds_throat: self.welds_throat.as_ref().map(|_| row.welds.throat.clone()),
            bolts_rows: self.bolts_rows.as_ref().map(|_| row.bolts.rows.clone()),
            bolts_bolts_per_row: self.bolts_bolts_per_row.as_ref().map(|_| row.bolts.bolts_per_row.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.welds_throat.is_none() && self.bolts_rows.is_none() && self.bolts_bolts_per_row.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `connections` list (rows keyed by `id`).
    pub En1999ConnectionsRows { removal: En1999ConnectionsRemoved, insertion: En1999ConnectionsInserted, relocation: En1999ConnectionsMoved, modification: En1999ConnectionsModified, row: crate::snapshot::AluminiumConnection, patch: En1999ConnectionsPatch, key: id }
}

impl En1999ConnectionsRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::AluminiumConnection], new: &[crate::snapshot::AluminiumConnection]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999ConnectionsRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999ConnectionsInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `fire_scenarios` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FireScenariosPatch {
    pub member_id: Option<String>,
    pub theta_a: Option<f64>,
    pub duration_s: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::FireScenario> for En1999FireScenariosPatch {
    fn commit_into(&self, row: &mut crate::snapshot::FireScenario, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id {
            row.member_id = value.clone();
        }
        if let Some(value) = &self.theta_a {
            row.theta_a = value.clone();
        }
        if let Some(value) = &self.duration_s {
            row.duration_s = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() {
            self.member_id = later.member_id;
        }
        if later.theta_a.is_some() {
            self.theta_a = later.theta_a;
        }
        if later.duration_s.is_some() {
            self.duration_s = later.duration_s;
        }
    }

    fn inverse(&self, row: &crate::snapshot::FireScenario) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            theta_a: self.theta_a.as_ref().map(|_| row.theta_a.clone()),
            duration_s: self.duration_s.as_ref().map(|_| row.duration_s.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.theta_a.is_none() && self.duration_s.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fire_scenarios` list (rows keyed by `id`).
    pub En1999FireScenariosRows { removal: En1999FireScenariosRemoved, insertion: En1999FireScenariosInserted, relocation: En1999FireScenariosMoved, modification: En1999FireScenariosModified, row: crate::snapshot::FireScenario, patch: En1999FireScenariosPatch, key: id }
}

impl En1999FireScenariosRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::FireScenario], new: &[crate::snapshot::FireScenario]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999FireScenariosRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999FireScenariosInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `fatigue_details` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FatigueDetailsPatch {
    pub member_id: Option<String>,
    pub detail_category: Option<String>,
    pub delta_sigma_c: Option<f64>,
    pub delta_sigma_ed: Option<f64>,
    pub n_cycles: Option<f64>,
    pub m1: Option<f64>,
    pub m2: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::FatigueDetail> for En1999FatigueDetailsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::FatigueDetail, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id {
            row.member_id = value.clone();
        }
        if let Some(value) = &self.detail_category {
            row.detail_category = value.clone();
        }
        if let Some(value) = &self.delta_sigma_c {
            row.delta_sigma_c = value.clone();
        }
        if let Some(value) = &self.delta_sigma_ed {
            row.delta_sigma_ed = value.clone();
        }
        if let Some(value) = &self.n_cycles {
            row.n_cycles = value.clone();
        }
        if let Some(value) = &self.m1 {
            row.m1 = value.clone();
        }
        if let Some(value) = &self.m2 {
            row.m2 = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() {
            self.member_id = later.member_id;
        }
        if later.detail_category.is_some() {
            self.detail_category = later.detail_category;
        }
        if later.delta_sigma_c.is_some() {
            self.delta_sigma_c = later.delta_sigma_c;
        }
        if later.delta_sigma_ed.is_some() {
            self.delta_sigma_ed = later.delta_sigma_ed;
        }
        if later.n_cycles.is_some() {
            self.n_cycles = later.n_cycles;
        }
        if later.m1.is_some() {
            self.m1 = later.m1;
        }
        if later.m2.is_some() {
            self.m2 = later.m2;
        }
    }

    fn inverse(&self, row: &crate::snapshot::FatigueDetail) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            detail_category: self.detail_category.as_ref().map(|_| row.detail_category.clone()),
            delta_sigma_c: self.delta_sigma_c.as_ref().map(|_| row.delta_sigma_c.clone()),
            delta_sigma_ed: self.delta_sigma_ed.as_ref().map(|_| row.delta_sigma_ed.clone()),
            n_cycles: self.n_cycles.as_ref().map(|_| row.n_cycles.clone()),
            m1: self.m1.as_ref().map(|_| row.m1.clone()),
            m2: self.m2.as_ref().map(|_| row.m2.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.detail_category.is_none() && self.delta_sigma_c.is_none() && self.delta_sigma_ed.is_none() && self.n_cycles.is_none() && self.m1.is_none() && self.m2.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fatigue_details` list (rows keyed by `id`).
    pub En1999FatigueDetailsRows { removal: En1999FatigueDetailsRemoved, insertion: En1999FatigueDetailsInserted, relocation: En1999FatigueDetailsMoved, modification: En1999FatigueDetailsModified, row: crate::snapshot::FatigueDetail, patch: En1999FatigueDetailsPatch, key: id }
}

impl En1999FatigueDetailsRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::FatigueDetail], new: &[crate::snapshot::FatigueDetail]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999FatigueDetailsRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999FatigueDetailsInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `cold_formed` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ColdFormedPatch {
    pub material_id: Option<String>,
    pub thickness: Option<f64>,
    pub width: Option<f64>,
    pub span: Option<f64>,
    pub welded: Option<bool>,
    pub actions: Option<Vec<crate::snapshot::MemberAction>>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::ColdFormedSheet> for En1999ColdFormedPatch {
    fn commit_into(&self, row: &mut crate::snapshot::ColdFormedSheet, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.material_id {
            row.material_id = value.clone();
        }
        if let Some(value) = &self.thickness {
            row.thickness = value.clone();
        }
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.span {
            row.span = value.clone();
        }
        if let Some(value) = &self.welded {
            row.welded = value.clone();
        }
        if let Some(value) = &self.actions {
            row.actions = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.material_id.is_some() {
            self.material_id = later.material_id;
        }
        if later.thickness.is_some() {
            self.thickness = later.thickness;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.span.is_some() {
            self.span = later.span;
        }
        if later.welded.is_some() {
            self.welded = later.welded;
        }
        if later.actions.is_some() {
            self.actions = later.actions;
        }
    }

    fn inverse(&self, row: &crate::snapshot::ColdFormedSheet) -> Self {
        Self {
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            span: self.span.as_ref().map(|_| row.span.clone()),
            welded: self.welded.as_ref().map(|_| row.welded.clone()),
            actions: self.actions.as_ref().map(|_| row.actions.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.material_id.is_none() && self.thickness.is_none() && self.width.is_none() && self.span.is_none() && self.welded.is_none() && self.actions.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `cold_formed` list (rows keyed by `id`).
    pub En1999ColdFormedRows { removal: En1999ColdFormedRemoved, insertion: En1999ColdFormedInserted, relocation: En1999ColdFormedMoved, modification: En1999ColdFormedModified, row: crate::snapshot::ColdFormedSheet, patch: En1999ColdFormedPatch, key: id }
}

impl En1999ColdFormedRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::ColdFormedSheet], new: &[crate::snapshot::ColdFormedSheet]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999ColdFormedRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999ColdFormedInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🩹️ Sparse patch of the `shells` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ShellsPatch {
    pub material_id: Option<String>,
    pub radius: Option<f64>,
    pub thickness: Option<f64>,
    pub length: Option<f64>,
    pub actions: Option<Vec<crate::snapshot::MemberAction>>,
}

impl protocol::list_delta::RowPatch<crate::snapshot::AluminiumShell> for En1999ShellsPatch {
    fn commit_into(&self, row: &mut crate::snapshot::AluminiumShell, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.material_id {
            row.material_id = value.clone();
        }
        if let Some(value) = &self.radius {
            row.radius = value.clone();
        }
        if let Some(value) = &self.thickness {
            row.thickness = value.clone();
        }
        if let Some(value) = &self.length {
            row.length = value.clone();
        }
        if let Some(value) = &self.actions {
            row.actions = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.material_id.is_some() {
            self.material_id = later.material_id;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.thickness.is_some() {
            self.thickness = later.thickness;
        }
        if later.length.is_some() {
            self.length = later.length;
        }
        if later.actions.is_some() {
            self.actions = later.actions;
        }
    }

    fn inverse(&self, row: &crate::snapshot::AluminiumShell) -> Self {
        Self {
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            radius: self.radius.as_ref().map(|_| row.radius.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            length: self.length.as_ref().map(|_| row.length.clone()),
            actions: self.actions.as_ref().map(|_| row.actions.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.material_id.is_none() && self.radius.is_none() && self.thickness.is_none() && self.length.is_none() && self.actions.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `shells` list (rows keyed by `id`).
    pub En1999ShellsRows { removal: En1999ShellsRemoved, insertion: En1999ShellsInserted, relocation: En1999ShellsMoved, modification: En1999ShellsModified, row: crate::snapshot::AluminiumShell, patch: En1999ShellsPatch, key: id }
}

impl En1999ShellsRows {
        /// 🧮️ The delta of a whole-list setter: every base row leaves at its base index and every payload row enters at its payload index.
        pub fn setting(base: &[crate::snapshot::AluminiumShell], new: &[crate::snapshot::AluminiumShell]) -> Self {
            Self {
                removed: base.iter().enumerate().map(|(index, row)| En1999ShellsRemoved { id: row.key().to_string(), index }).collect(),
                inserted: new.iter().enumerate().map(|(index, row)| En1999ShellsInserted { index, row: row.clone() }).collect(),
                moved: Vec::new(),
                modified: Vec::new(),
            }
        }
    }

    /// 🔺️ Keyed sparse diff of the En1999 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Option<En1999MaterialsRows>,
    #[state(artifact)]
    pub sections: Option<En1999SectionsRows>,
    #[state(artifact)]
    pub members: Option<En1999MembersRows>,
    #[state(artifact)]
    pub connections: Option<En1999ConnectionsRows>,
    #[state(artifact)]
    pub fire_scenarios: Option<En1999FireScenariosRows>,
    #[state(artifact)]
    pub fatigue_details: Option<En1999FatigueDetailsRows>,
    #[state(artifact)]
    pub cold_formed: Option<En1999ColdFormedRows>,
    #[state(artifact)]
    pub shells: Option<En1999ShellsRows>,
}

impl protocol::MutationDiff<En1999Snapshot> for En1999Diff {
    fn apply(&self, base: &En1999Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1999Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(rows) = &self.materials {
            next.materials = rows.commit_onto(&base.materials, capability).map_err(|error| error.under(["materials"]))?;
        }
        if let Some(rows) = &self.sections {
            next.sections = rows.commit_onto(&base.sections, capability).map_err(|error| error.under(["sections"]))?;
        }
        if let Some(rows) = &self.members {
            next.members = rows.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?;
        }
        if let Some(rows) = &self.connections {
            next.connections = rows.commit_onto(&base.connections, capability).map_err(|error| error.under(["connections"]))?;
        }
        if let Some(rows) = &self.fire_scenarios {
            next.fire_scenarios = rows.commit_onto(&base.fire_scenarios, capability).map_err(|error| error.under(["fire_scenarios"]))?;
        }
        if let Some(rows) = &self.fatigue_details {
            next.fatigue_details = rows.commit_onto(&base.fatigue_details, capability).map_err(|error| error.under(["fatigue_details"]))?;
        }
        if let Some(rows) = &self.cold_formed {
            next.cold_formed = rows.commit_onto(&base.cold_formed, capability).map_err(|error| error.under(["cold_formed"]))?;
        }
        if let Some(rows) = &self.shells {
            next.shells = rows.commit_onto(&base.shells, capability).map_err(|error| error.under(["shells"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if let Some(theirs) = other.materials {
            match self.materials.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.materials = Some(theirs),
            }
            self.materials = self.materials.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.sections {
            match self.sections.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.sections = Some(theirs),
            }
            self.sections = self.sections.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.members {
            match self.members.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.members = Some(theirs),
            }
            self.members = self.members.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.connections {
            match self.connections.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.connections = Some(theirs),
            }
            self.connections = self.connections.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.fire_scenarios {
            match self.fire_scenarios.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.fire_scenarios = Some(theirs),
            }
            self.fire_scenarios = self.fire_scenarios.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.fatigue_details {
            match self.fatigue_details.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.fatigue_details = Some(theirs),
            }
            self.fatigue_details = self.fatigue_details.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.cold_formed {
            match self.cold_formed.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.cold_formed = Some(theirs),
            }
            self.cold_formed = self.cold_formed.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.shells {
            match self.shells.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.shells = Some(theirs),
            }
            self.shells = self.shells.take().filter(|rows| !rows.is_empty());
        }
    }
}

impl protocol::DiffAlgebra<En1999Snapshot> for En1999Diff {
    fn inverse(&self, base: &En1999Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            materials: self.materials.as_ref().map(|rows| rows.inverse(&base.materials)).filter(|rows| !rows.is_empty()),
            sections: self.sections.as_ref().map(|rows| rows.inverse(&base.sections)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse(&base.members)).filter(|rows| !rows.is_empty()),
            connections: self.connections.as_ref().map(|rows| rows.inverse(&base.connections)).filter(|rows| !rows.is_empty()),
            fire_scenarios: self.fire_scenarios.as_ref().map(|rows| rows.inverse(&base.fire_scenarios)).filter(|rows| !rows.is_empty()),
            fatigue_details: self.fatigue_details.as_ref().map(|rows| rows.inverse(&base.fatigue_details)).filter(|rows| !rows.is_empty()),
            cold_formed: self.cold_formed.as_ref().map(|rows| rows.inverse(&base.cold_formed)).filter(|rows| !rows.is_empty()),
            shells: self.shells.as_ref().map(|rows| rows.inverse(&base.shells)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.materials.as_ref().map_or(true, |rows| rows.is_empty()) && self.sections.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.connections.as_ref().map_or(true, |rows| rows.is_empty()) && self.fire_scenarios.as_ref().map_or(true, |rows| rows.is_empty()) && self.fatigue_details.as_ref().map_or(true, |rows| rows.is_empty()) && self.cold_formed.as_ref().map_or(true, |rows| rows.is_empty()) && self.shells.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
