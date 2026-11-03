//! 🧬️ En1999 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

/// 🔑️ Key extractor of the foreign `AluminiumMaterial` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::AluminiumMaterial> for MaterialKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::AluminiumMaterial) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `PlateElement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlateElementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::PlateElement> for PlateElementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::PlateElement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `AluminiumSection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::AluminiumSection> for SectionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::AluminiumSection) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `MemberAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemberActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::MemberAction> for MemberActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::MemberAction) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `AluminiumMember` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MemberKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::AluminiumMember> for MemberKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::AluminiumMember) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `AluminiumConnection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConnectionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::AluminiumConnection> for ConnectionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::AluminiumConnection) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `FireScenario` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FireScenarioKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::FireScenario> for FireScenarioKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::FireScenario) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `FatigueDetail` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FatigueDetailKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::FatigueDetail> for FatigueDetailKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::FatigueDetail) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ColdFormedSheet` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColdFormedKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::ColdFormedSheet> for ColdFormedKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::ColdFormedSheet) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `AluminiumShell` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::AluminiumShell> for ShellKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::AluminiumShell) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::Placement> for PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::Placement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WeldRun` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeldRunKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::WeldRun> for WeldRunKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::WeldRun) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `SectionVertex` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SectionVertexKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::SectionVertex> for SectionVertexKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::SectionVertex) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `FatigueCategory` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FatigueCategoryKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::FatigueCategory> for FatigueCategoryKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::FatigueCategory) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `PatchLoad` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatchLoadKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::PatchLoad> for PatchLoadKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::PatchLoad) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RingStiffener` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingStiffenerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::RingStiffener> for RingStiffenerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::RingStiffener) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `TransverseStiffener` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransverseStiffenerKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1999_model::TransverseStiffener> for TransverseStiffenerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1999_model::TransverseStiffener) -> String {
        row.id.clone()
    }
}


use protocol::list_delta::Keyed as _;

/// 🩹️ Sparse patch of the `materials` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MaterialsPatch {
    pub designation: Option<String>,
    pub product_form: Option<semio_s_artifact_norm_en1999_model::ProductForm>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::AluminiumMaterial> for En1999MaterialsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::AluminiumMaterial, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.designation { row.designation = value.clone(); }
        if let Some(value) = &self.product_form { row.product_form = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.designation.is_some() { self.designation = later.designation; }
        if later.product_form.is_some() { self.product_form = later.product_form; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::AluminiumMaterial) -> Self {
        Self {
            designation: self.designation.as_ref().map(|_| row.designation.clone()),
            product_form: self.product_form.as_ref().map(|_| row.product_form.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.designation.is_none() && self.product_form.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `materials` list (rows keyed by `id`).
    pub En1999MaterialsRows { removal: En1999MaterialsRemoved, insertion: En1999MaterialsInserted, relocation: En1999MaterialsMoved, modification: En1999MaterialsModified, row: semio_s_artifact_norm_en1999_model::AluminiumMaterial, patch: En1999MaterialsPatch, list: Vec<semio_s_artifact_norm_en1999_model::AluminiumMaterial>, key: String = by MaterialKey }
}


/// 🩹️ Sparse patch of the `elements` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsElementsPatch {
    pub width: Option<f64>,
    pub thickness: Option<f64>,
    pub outstand: Option<bool>,
    pub welded: Option<bool>,
    pub weld_position: Option<f64>,
    pub weld_process: Option<semio_s_artifact_norm_en1999_model::WeldProcess>,
    pub weld_t_junction: Option<bool>,
    pub weld_direction: Option<semio_s_artifact_norm_en1999_model::WeldDirection>,
    pub edge_stiffener_depth: Option<f64>,
    pub edge_stiffener_thickness: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::PlateElement> for En1999SectionsElementsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::PlateElement, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.width { row.width = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.outstand { row.outstand = value.clone(); }
        if let Some(value) = &self.welded { row.welded = value.clone(); }
        if let Some(value) = &self.weld_position { row.weld_position = value.clone(); }
        if let Some(value) = &self.weld_process { row.weld_process = value.clone(); }
        if let Some(value) = &self.weld_t_junction { row.weld_t_junction = value.clone(); }
        if let Some(value) = &self.weld_direction { row.weld_direction = value.clone(); }
        if let Some(value) = &self.edge_stiffener_depth { row.edge_stiffener_depth = value.clone(); }
        if let Some(value) = &self.edge_stiffener_thickness { row.edge_stiffener_thickness = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.width.is_some() { self.width = later.width; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.outstand.is_some() { self.outstand = later.outstand; }
        if later.welded.is_some() { self.welded = later.welded; }
        if later.weld_position.is_some() { self.weld_position = later.weld_position; }
        if later.weld_process.is_some() { self.weld_process = later.weld_process; }
        if later.weld_t_junction.is_some() { self.weld_t_junction = later.weld_t_junction; }
        if later.weld_direction.is_some() { self.weld_direction = later.weld_direction; }
        if later.edge_stiffener_depth.is_some() { self.edge_stiffener_depth = later.edge_stiffener_depth; }
        if later.edge_stiffener_thickness.is_some() { self.edge_stiffener_thickness = later.edge_stiffener_thickness; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::PlateElement) -> Self {
        Self {
            width: self.width.as_ref().map(|_| row.width.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            outstand: self.outstand.as_ref().map(|_| row.outstand.clone()),
            welded: self.welded.as_ref().map(|_| row.welded.clone()),
            weld_position: self.weld_position.as_ref().map(|_| row.weld_position.clone()),
            weld_process: self.weld_process.as_ref().map(|_| row.weld_process.clone()),
            weld_t_junction: self.weld_t_junction.as_ref().map(|_| row.weld_t_junction.clone()),
            weld_direction: self.weld_direction.as_ref().map(|_| row.weld_direction.clone()),
            edge_stiffener_depth: self.edge_stiffener_depth.as_ref().map(|_| row.edge_stiffener_depth.clone()),
            edge_stiffener_thickness: self.edge_stiffener_thickness.as_ref().map(|_| row.edge_stiffener_thickness.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.width.is_none() && self.thickness.is_none() && self.outstand.is_none() && self.welded.is_none() && self.weld_position.is_none() && self.weld_process.is_none() && self.weld_t_junction.is_none() && self.weld_direction.is_none() && self.edge_stiffener_depth.is_none() && self.edge_stiffener_thickness.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `elements` list (rows keyed by `id`).
    pub En1999SectionsElementsRows { removal: En1999SectionsElementsRemoved, insertion: En1999SectionsElementsInserted, relocation: En1999SectionsElementsMoved, modification: En1999SectionsElementsModified, row: semio_s_artifact_norm_en1999_model::PlateElement, patch: En1999SectionsElementsPatch, list: Vec<semio_s_artifact_norm_en1999_model::PlateElement>, key: String = by PlateElementKey }
}

/// 🩹️ Sparse patch of the `sections` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999SectionsPatch {
    pub kind: Option<semio_s_artifact_norm_en1999_model::SectionKind>,
    pub height: Option<f64>,
    pub width: Option<f64>,
    pub flange_thickness: Option<f64>,
    pub web_thickness: Option<f64>,
    pub outer_diameter: Option<f64>,
    pub elements: Option<En1999SectionsElementsRows>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::AluminiumSection> for En1999SectionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::AluminiumSection, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.height { row.height = value.clone(); }
        if let Some(value) = &self.width { row.width = value.clone(); }
        if let Some(value) = &self.flange_thickness { row.flange_thickness = value.clone(); }
        if let Some(value) = &self.web_thickness { row.web_thickness = value.clone(); }
        if let Some(value) = &self.outer_diameter { row.outer_diameter = value.clone(); }
        if let Some(rows) = &self.elements {
            row.elements = rows.commit_onto(&row.elements, capability).map_err(|error| error.under(["elements"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() { self.kind = later.kind; }
        if later.height.is_some() { self.height = later.height; }
        if later.width.is_some() { self.width = later.width; }
        if later.flange_thickness.is_some() { self.flange_thickness = later.flange_thickness; }
        if later.web_thickness.is_some() { self.web_thickness = later.web_thickness; }
        if later.outer_diameter.is_some() { self.outer_diameter = later.outer_diameter; }
        if let Some(theirs) = later.elements {
            match self.elements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.elements = Some(theirs),
            }
        }
        self.elements = self.elements.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::AluminiumSection) -> Self {
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
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `sections` list (rows keyed by `id`).
    pub En1999SectionsRows { removal: En1999SectionsRemoved, insertion: En1999SectionsInserted, relocation: En1999SectionsMoved, modification: En1999SectionsModified, row: semio_s_artifact_norm_en1999_model::AluminiumSection, patch: En1999SectionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::AluminiumSection>, key: String = by SectionKey }
}


/// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersActionsPatch {
    pub kind: Option<semio_s_artifact_norm_en1999_model::ActionKind>,
    pub category: Option<semio_s_artifact_norm_en1999_model::ActionCategory>,
    pub source: Option<semio_s_artifact_norm_en1999_model::ActionSource>,
    pub g_k_line: Option<f64>,
    pub q_k_line: Option<f64>,
    pub n_k: Option<f64>,
    pub v_y_k: Option<f64>,
    pub v_z_k: Option<f64>,
    pub m_y_k: Option<f64>,
    pub m_z_k: Option<f64>,
    pub torsion_k: Option<f64>,
    pub local_load_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::MemberAction> for En1999MembersActionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::MemberAction, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.category { row.category = value.clone(); }
        if let Some(value) = &self.source { row.source = value.clone(); }
        if let Some(value) = &self.g_k_line { row.g_k_line = value.clone(); }
        if let Some(value) = &self.q_k_line { row.q_k_line = value.clone(); }
        if let Some(value) = &self.n_k { row.n_k = value.clone(); }
        if let Some(value) = &self.v_y_k { row.v_y_k = value.clone(); }
        if let Some(value) = &self.v_z_k { row.v_z_k = value.clone(); }
        if let Some(value) = &self.m_y_k { row.m_y_k = value.clone(); }
        if let Some(value) = &self.m_z_k { row.m_z_k = value.clone(); }
        if let Some(value) = &self.torsion_k { row.torsion_k = value.clone(); }
        if let Some(value) = &self.local_load_k { row.local_load_k = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() { self.kind = later.kind; }
        if later.category.is_some() { self.category = later.category; }
        if later.source.is_some() { self.source = later.source; }
        if later.g_k_line.is_some() { self.g_k_line = later.g_k_line; }
        if later.q_k_line.is_some() { self.q_k_line = later.q_k_line; }
        if later.n_k.is_some() { self.n_k = later.n_k; }
        if later.v_y_k.is_some() { self.v_y_k = later.v_y_k; }
        if later.v_z_k.is_some() { self.v_z_k = later.v_z_k; }
        if later.m_y_k.is_some() { self.m_y_k = later.m_y_k; }
        if later.m_z_k.is_some() { self.m_z_k = later.m_z_k; }
        if later.torsion_k.is_some() { self.torsion_k = later.torsion_k; }
        if later.local_load_k.is_some() { self.local_load_k = later.local_load_k; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::MemberAction) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            category: self.category.as_ref().map(|_| row.category.clone()),
            source: self.source.as_ref().map(|_| row.source.clone()),
            g_k_line: self.g_k_line.as_ref().map(|_| row.g_k_line.clone()),
            q_k_line: self.q_k_line.as_ref().map(|_| row.q_k_line.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_y_k: self.v_y_k.as_ref().map(|_| row.v_y_k.clone()),
            v_z_k: self.v_z_k.as_ref().map(|_| row.v_z_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
            m_z_k: self.m_z_k.as_ref().map(|_| row.m_z_k.clone()),
            torsion_k: self.torsion_k.as_ref().map(|_| row.torsion_k.clone()),
            local_load_k: self.local_load_k.as_ref().map(|_| row.local_load_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.category.is_none() && self.source.is_none() && self.g_k_line.is_none() && self.q_k_line.is_none() && self.n_k.is_none() && self.v_y_k.is_none() && self.v_z_k.is_none() && self.m_y_k.is_none() && self.m_z_k.is_none() && self.torsion_k.is_none() && self.local_load_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1999MembersActionsRows { removal: En1999MembersActionsRemoved, insertion: En1999MembersActionsInserted, relocation: En1999MembersActionsMoved, modification: En1999MembersActionsModified, row: semio_s_artifact_norm_en1999_model::MemberAction, patch: En1999MembersActionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::MemberAction>, key: String = by MemberActionKey }
}

/// 🩹️ Sparse patch of the `members` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999MembersPatch {
    pub section_id: Option<String>,
    pub material_id: Option<String>,
    pub length: Option<f64>,
    pub support: Option<semio_s_artifact_norm_en1999_model::SupportCondition>,
    pub buckling_length_y: Option<f64>,
    pub buckling_length_z: Option<f64>,
    pub buckling_length_t: Option<f64>,
    pub ltb_length: Option<f64>,
    pub c1: Option<f64>,
    pub restrained_ltb: Option<bool>,
    pub bearing_length: Option<f64>,
    pub transverse_weld: Option<bool>,
    pub transverse_weld_offset: Option<f64>,
    pub actions: Option<En1999MembersActionsRows>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::AluminiumMember> for En1999MembersPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::AluminiumMember, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.section_id { row.section_id = value.clone(); }
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.length { row.length = value.clone(); }
        if let Some(value) = &self.support { row.support = value.clone(); }
        if let Some(value) = &self.buckling_length_y { row.buckling_length_y = value.clone(); }
        if let Some(value) = &self.buckling_length_z { row.buckling_length_z = value.clone(); }
        if let Some(value) = &self.buckling_length_t { row.buckling_length_t = value.clone(); }
        if let Some(value) = &self.ltb_length { row.ltb_length = value.clone(); }
        if let Some(value) = &self.c1 { row.c1 = value.clone(); }
        if let Some(value) = &self.restrained_ltb { row.restrained_ltb = value.clone(); }
        if let Some(value) = &self.bearing_length { row.bearing_length = value.clone(); }
        if let Some(value) = &self.transverse_weld { row.transverse_weld = value.clone(); }
        if let Some(value) = &self.transverse_weld_offset { row.transverse_weld_offset = value.clone(); }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.section_id.is_some() { self.section_id = later.section_id; }
        if later.material_id.is_some() { self.material_id = later.material_id; }
        if later.length.is_some() { self.length = later.length; }
        if later.support.is_some() { self.support = later.support; }
        if later.buckling_length_y.is_some() { self.buckling_length_y = later.buckling_length_y; }
        if later.buckling_length_z.is_some() { self.buckling_length_z = later.buckling_length_z; }
        if later.buckling_length_t.is_some() { self.buckling_length_t = later.buckling_length_t; }
        if later.ltb_length.is_some() { self.ltb_length = later.ltb_length; }
        if later.c1.is_some() { self.c1 = later.c1; }
        if later.restrained_ltb.is_some() { self.restrained_ltb = later.restrained_ltb; }
        if later.bearing_length.is_some() { self.bearing_length = later.bearing_length; }
        if later.transverse_weld.is_some() { self.transverse_weld = later.transverse_weld; }
        if later.transverse_weld_offset.is_some() { self.transverse_weld_offset = later.transverse_weld_offset; }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::AluminiumMember) -> Self {
        Self {
            section_id: self.section_id.as_ref().map(|_| row.section_id.clone()),
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            length: self.length.as_ref().map(|_| row.length.clone()),
            support: self.support.as_ref().map(|_| row.support.clone()),
            buckling_length_y: self.buckling_length_y.as_ref().map(|_| row.buckling_length_y.clone()),
            buckling_length_z: self.buckling_length_z.as_ref().map(|_| row.buckling_length_z.clone()),
            buckling_length_t: self.buckling_length_t.as_ref().map(|_| row.buckling_length_t.clone()),
            ltb_length: self.ltb_length.as_ref().map(|_| row.ltb_length.clone()),
            c1: self.c1.as_ref().map(|_| row.c1.clone()),
            restrained_ltb: self.restrained_ltb.as_ref().map(|_| row.restrained_ltb.clone()),
            bearing_length: self.bearing_length.as_ref().map(|_| row.bearing_length.clone()),
            transverse_weld: self.transverse_weld.as_ref().map(|_| row.transverse_weld.clone()),
            transverse_weld_offset: self.transverse_weld_offset.as_ref().map(|_| row.transverse_weld_offset.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.section_id.is_none() && self.material_id.is_none() && self.length.is_none() && self.support.is_none() && self.buckling_length_y.is_none() && self.buckling_length_z.is_none() && self.buckling_length_t.is_none() && self.ltb_length.is_none() && self.c1.is_none() && self.restrained_ltb.is_none() && self.bearing_length.is_none() && self.transverse_weld.is_none() && self.transverse_weld_offset.is_none() && self.actions.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `members` list (rows keyed by `id`).
    pub En1999MembersRows { removal: En1999MembersRemoved, insertion: En1999MembersInserted, relocation: En1999MembersMoved, modification: En1999MembersModified, row: semio_s_artifact_norm_en1999_model::AluminiumMember, patch: En1999MembersPatch, list: Vec<semio_s_artifact_norm_en1999_model::AluminiumMember>, key: String = by MemberKey }
}


/// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ConnectionsActionsPatch {
    pub kind: Option<semio_s_artifact_norm_en1999_model::ActionKind>,
    pub category: Option<semio_s_artifact_norm_en1999_model::ActionCategory>,
    pub source: Option<semio_s_artifact_norm_en1999_model::ActionSource>,
    pub g_k_line: Option<f64>,
    pub q_k_line: Option<f64>,
    pub n_k: Option<f64>,
    pub v_y_k: Option<f64>,
    pub v_z_k: Option<f64>,
    pub m_y_k: Option<f64>,
    pub m_z_k: Option<f64>,
    pub torsion_k: Option<f64>,
    pub local_load_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::MemberAction> for En1999ConnectionsActionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::MemberAction, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.category { row.category = value.clone(); }
        if let Some(value) = &self.source { row.source = value.clone(); }
        if let Some(value) = &self.g_k_line { row.g_k_line = value.clone(); }
        if let Some(value) = &self.q_k_line { row.q_k_line = value.clone(); }
        if let Some(value) = &self.n_k { row.n_k = value.clone(); }
        if let Some(value) = &self.v_y_k { row.v_y_k = value.clone(); }
        if let Some(value) = &self.v_z_k { row.v_z_k = value.clone(); }
        if let Some(value) = &self.m_y_k { row.m_y_k = value.clone(); }
        if let Some(value) = &self.m_z_k { row.m_z_k = value.clone(); }
        if let Some(value) = &self.torsion_k { row.torsion_k = value.clone(); }
        if let Some(value) = &self.local_load_k { row.local_load_k = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() { self.kind = later.kind; }
        if later.category.is_some() { self.category = later.category; }
        if later.source.is_some() { self.source = later.source; }
        if later.g_k_line.is_some() { self.g_k_line = later.g_k_line; }
        if later.q_k_line.is_some() { self.q_k_line = later.q_k_line; }
        if later.n_k.is_some() { self.n_k = later.n_k; }
        if later.v_y_k.is_some() { self.v_y_k = later.v_y_k; }
        if later.v_z_k.is_some() { self.v_z_k = later.v_z_k; }
        if later.m_y_k.is_some() { self.m_y_k = later.m_y_k; }
        if later.m_z_k.is_some() { self.m_z_k = later.m_z_k; }
        if later.torsion_k.is_some() { self.torsion_k = later.torsion_k; }
        if later.local_load_k.is_some() { self.local_load_k = later.local_load_k; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::MemberAction) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            category: self.category.as_ref().map(|_| row.category.clone()),
            source: self.source.as_ref().map(|_| row.source.clone()),
            g_k_line: self.g_k_line.as_ref().map(|_| row.g_k_line.clone()),
            q_k_line: self.q_k_line.as_ref().map(|_| row.q_k_line.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_y_k: self.v_y_k.as_ref().map(|_| row.v_y_k.clone()),
            v_z_k: self.v_z_k.as_ref().map(|_| row.v_z_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
            m_z_k: self.m_z_k.as_ref().map(|_| row.m_z_k.clone()),
            torsion_k: self.torsion_k.as_ref().map(|_| row.torsion_k.clone()),
            local_load_k: self.local_load_k.as_ref().map(|_| row.local_load_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.category.is_none() && self.source.is_none() && self.g_k_line.is_none() && self.q_k_line.is_none() && self.n_k.is_none() && self.v_y_k.is_none() && self.v_z_k.is_none() && self.m_y_k.is_none() && self.m_z_k.is_none() && self.torsion_k.is_none() && self.local_load_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1999ConnectionsActionsRows { removal: En1999ConnectionsActionsRemoved, insertion: En1999ConnectionsActionsInserted, relocation: En1999ConnectionsActionsMoved, modification: En1999ConnectionsActionsModified, row: semio_s_artifact_norm_en1999_model::MemberAction, patch: En1999ConnectionsActionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::MemberAction>, key: String = by MemberActionKey }
}

/// 🩹️ Sparse patch of the `connections` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ConnectionsPatch {
    pub member_id: Option<String>,
    pub material_id: Option<String>,
    pub kind: Option<semio_s_artifact_norm_en1999_model::ConnectionKind>,
    pub bolts_material: Option<String>,
    pub bolts_diameter: Option<f64>,
    pub bolts_rows: Option<u32>,
    pub bolts_bolts_per_row: Option<u32>,
    pub bolts_edge_distance: Option<f64>,
    pub bolts_pitch: Option<f64>,
    pub bolts_gauge: Option<f64>,
    pub bolts_plate_thickness: Option<f64>,
    pub welds_filler_alloy: Option<String>,
    pub welds_throat: Option<f64>,
    pub welds_length: Option<f64>,
    pub welds_beta_w: Option<f64>,
    pub welds_haz_extent: Option<f64>,
    pub actions: Option<En1999ConnectionsActionsRows>,
    pub load_eccentricity: Option<f64>,
    pub hole_stagger: Option<f64>,
    pub force_method: Option<semio_s_artifact_norm_en1999_model::FastenerForceMethod>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::AluminiumConnection> for En1999ConnectionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::AluminiumConnection, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.bolts_material { row.bolts.material = value.clone(); }
        if let Some(value) = &self.bolts_diameter { row.bolts.diameter = value.clone(); }
        if let Some(value) = &self.bolts_rows { row.bolts.rows = value.clone(); }
        if let Some(value) = &self.bolts_bolts_per_row { row.bolts.bolts_per_row = value.clone(); }
        if let Some(value) = &self.bolts_edge_distance { row.bolts.edge_distance = value.clone(); }
        if let Some(value) = &self.bolts_pitch { row.bolts.pitch = value.clone(); }
        if let Some(value) = &self.bolts_gauge { row.bolts.gauge = value.clone(); }
        if let Some(value) = &self.bolts_plate_thickness { row.bolts.plate_thickness = value.clone(); }
        if let Some(value) = &self.welds_filler_alloy { row.welds.filler_alloy = value.clone(); }
        if let Some(value) = &self.welds_throat { row.welds.throat = value.clone(); }
        if let Some(value) = &self.welds_length { row.welds.length = value.clone(); }
        if let Some(value) = &self.welds_beta_w { row.welds.beta_w = value.clone(); }
        if let Some(value) = &self.welds_haz_extent { row.welds.haz_extent = value.clone(); }
        if let Some(value) = &self.load_eccentricity { row.load_eccentricity = value.clone(); }
        if let Some(value) = &self.hole_stagger { row.hole_stagger = value.clone(); }
        if let Some(value) = &self.force_method { row.force_method = value.clone(); }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.material_id.is_some() { self.material_id = later.material_id; }
        if later.kind.is_some() { self.kind = later.kind; }
        if later.bolts_material.is_some() { self.bolts_material = later.bolts_material; }
        if later.bolts_diameter.is_some() { self.bolts_diameter = later.bolts_diameter; }
        if later.bolts_rows.is_some() { self.bolts_rows = later.bolts_rows; }
        if later.bolts_bolts_per_row.is_some() { self.bolts_bolts_per_row = later.bolts_bolts_per_row; }
        if later.bolts_edge_distance.is_some() { self.bolts_edge_distance = later.bolts_edge_distance; }
        if later.bolts_pitch.is_some() { self.bolts_pitch = later.bolts_pitch; }
        if later.bolts_gauge.is_some() { self.bolts_gauge = later.bolts_gauge; }
        if later.bolts_plate_thickness.is_some() { self.bolts_plate_thickness = later.bolts_plate_thickness; }
        if later.welds_filler_alloy.is_some() { self.welds_filler_alloy = later.welds_filler_alloy; }
        if later.welds_throat.is_some() { self.welds_throat = later.welds_throat; }
        if later.welds_length.is_some() { self.welds_length = later.welds_length; }
        if later.welds_beta_w.is_some() { self.welds_beta_w = later.welds_beta_w; }
        if later.welds_haz_extent.is_some() { self.welds_haz_extent = later.welds_haz_extent; }
        if later.load_eccentricity.is_some() { self.load_eccentricity = later.load_eccentricity; }
        if later.hole_stagger.is_some() { self.hole_stagger = later.hole_stagger; }
        if later.force_method.is_some() { self.force_method = later.force_method; }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::AluminiumConnection) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            bolts_material: self.bolts_material.as_ref().map(|_| row.bolts.material.clone()),
            bolts_diameter: self.bolts_diameter.as_ref().map(|_| row.bolts.diameter.clone()),
            bolts_rows: self.bolts_rows.as_ref().map(|_| row.bolts.rows.clone()),
            bolts_bolts_per_row: self.bolts_bolts_per_row.as_ref().map(|_| row.bolts.bolts_per_row.clone()),
            bolts_edge_distance: self.bolts_edge_distance.as_ref().map(|_| row.bolts.edge_distance.clone()),
            bolts_pitch: self.bolts_pitch.as_ref().map(|_| row.bolts.pitch.clone()),
            bolts_gauge: self.bolts_gauge.as_ref().map(|_| row.bolts.gauge.clone()),
            bolts_plate_thickness: self.bolts_plate_thickness.as_ref().map(|_| row.bolts.plate_thickness.clone()),
            welds_filler_alloy: self.welds_filler_alloy.as_ref().map(|_| row.welds.filler_alloy.clone()),
            welds_throat: self.welds_throat.as_ref().map(|_| row.welds.throat.clone()),
            welds_length: self.welds_length.as_ref().map(|_| row.welds.length.clone()),
            welds_beta_w: self.welds_beta_w.as_ref().map(|_| row.welds.beta_w.clone()),
            welds_haz_extent: self.welds_haz_extent.as_ref().map(|_| row.welds.haz_extent.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
            load_eccentricity: self.load_eccentricity.as_ref().map(|_| row.load_eccentricity.clone()),
            hole_stagger: self.hole_stagger.as_ref().map(|_| row.hole_stagger.clone()),
            force_method: self.force_method.as_ref().map(|_| row.force_method.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.material_id.is_none() && self.kind.is_none() && self.bolts_material.is_none() && self.bolts_diameter.is_none() && self.bolts_rows.is_none() && self.bolts_bolts_per_row.is_none() && self.bolts_edge_distance.is_none() && self.bolts_pitch.is_none() && self.bolts_gauge.is_none() && self.bolts_plate_thickness.is_none() && self.welds_filler_alloy.is_none() && self.welds_throat.is_none() && self.welds_length.is_none() && self.welds_beta_w.is_none() && self.welds_haz_extent.is_none() && self.actions.as_ref().map_or(true, |rows| rows.is_empty()) && self.load_eccentricity.is_none() && self.hole_stagger.is_none() && self.force_method.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `connections` list (rows keyed by `id`).
    pub En1999ConnectionsRows { removal: En1999ConnectionsRemoved, insertion: En1999ConnectionsInserted, relocation: En1999ConnectionsMoved, modification: En1999ConnectionsModified, row: semio_s_artifact_norm_en1999_model::AluminiumConnection, patch: En1999ConnectionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::AluminiumConnection>, key: String = by ConnectionKey }
}


/// 🩹️ Sparse patch of the `scenarios` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FireScenariosPatch {
    pub member_id: Option<String>,
    pub theta_a: Option<f64>,
    pub duration_s: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::FireScenario> for En1999FireScenariosPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::FireScenario, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.theta_a { row.theta_a = value.clone(); }
        if let Some(value) = &self.duration_s { row.duration_s = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.theta_a.is_some() { self.theta_a = later.theta_a; }
        if later.duration_s.is_some() { self.duration_s = later.duration_s; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::FireScenario) -> Self {
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
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fire_scenarios` list (rows keyed by `id`).
    pub En1999FireScenariosRows { removal: En1999FireScenariosRemoved, insertion: En1999FireScenariosInserted, relocation: En1999FireScenariosMoved, modification: En1999FireScenariosModified, row: semio_s_artifact_norm_en1999_model::FireScenario, patch: En1999FireScenariosPatch, list: Vec<semio_s_artifact_norm_en1999_model::FireScenario>, key: String = by FireScenarioKey }
}


/// 🩹️ Sparse patch of the `details` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
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

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::FatigueDetail> for En1999FatigueDetailsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::FatigueDetail, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.detail_category { row.detail_category = value.clone(); }
        if let Some(value) = &self.delta_sigma_c { row.delta_sigma_c = value.clone(); }
        if let Some(value) = &self.delta_sigma_ed { row.delta_sigma_ed = value.clone(); }
        if let Some(value) = &self.n_cycles { row.n_cycles = value.clone(); }
        if let Some(value) = &self.m1 { row.m1 = value.clone(); }
        if let Some(value) = &self.m2 { row.m2 = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.detail_category.is_some() { self.detail_category = later.detail_category; }
        if later.delta_sigma_c.is_some() { self.delta_sigma_c = later.delta_sigma_c; }
        if later.delta_sigma_ed.is_some() { self.delta_sigma_ed = later.delta_sigma_ed; }
        if later.n_cycles.is_some() { self.n_cycles = later.n_cycles; }
        if later.m1.is_some() { self.m1 = later.m1; }
        if later.m2.is_some() { self.m2 = later.m2; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::FatigueDetail) -> Self {
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
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fatigue_details` list (rows keyed by `id`).
    pub En1999FatigueDetailsRows { removal: En1999FatigueDetailsRemoved, insertion: En1999FatigueDetailsInserted, relocation: En1999FatigueDetailsMoved, modification: En1999FatigueDetailsModified, row: semio_s_artifact_norm_en1999_model::FatigueDetail, patch: En1999FatigueDetailsPatch, list: Vec<semio_s_artifact_norm_en1999_model::FatigueDetail>, key: String = by FatigueDetailKey }
}


/// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ColdFormedActionsPatch {
    pub kind: Option<semio_s_artifact_norm_en1999_model::ActionKind>,
    pub category: Option<semio_s_artifact_norm_en1999_model::ActionCategory>,
    pub source: Option<semio_s_artifact_norm_en1999_model::ActionSource>,
    pub g_k_line: Option<f64>,
    pub q_k_line: Option<f64>,
    pub n_k: Option<f64>,
    pub v_y_k: Option<f64>,
    pub v_z_k: Option<f64>,
    pub m_y_k: Option<f64>,
    pub m_z_k: Option<f64>,
    pub torsion_k: Option<f64>,
    pub local_load_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::MemberAction> for En1999ColdFormedActionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::MemberAction, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.category { row.category = value.clone(); }
        if let Some(value) = &self.source { row.source = value.clone(); }
        if let Some(value) = &self.g_k_line { row.g_k_line = value.clone(); }
        if let Some(value) = &self.q_k_line { row.q_k_line = value.clone(); }
        if let Some(value) = &self.n_k { row.n_k = value.clone(); }
        if let Some(value) = &self.v_y_k { row.v_y_k = value.clone(); }
        if let Some(value) = &self.v_z_k { row.v_z_k = value.clone(); }
        if let Some(value) = &self.m_y_k { row.m_y_k = value.clone(); }
        if let Some(value) = &self.m_z_k { row.m_z_k = value.clone(); }
        if let Some(value) = &self.torsion_k { row.torsion_k = value.clone(); }
        if let Some(value) = &self.local_load_k { row.local_load_k = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() { self.kind = later.kind; }
        if later.category.is_some() { self.category = later.category; }
        if later.source.is_some() { self.source = later.source; }
        if later.g_k_line.is_some() { self.g_k_line = later.g_k_line; }
        if later.q_k_line.is_some() { self.q_k_line = later.q_k_line; }
        if later.n_k.is_some() { self.n_k = later.n_k; }
        if later.v_y_k.is_some() { self.v_y_k = later.v_y_k; }
        if later.v_z_k.is_some() { self.v_z_k = later.v_z_k; }
        if later.m_y_k.is_some() { self.m_y_k = later.m_y_k; }
        if later.m_z_k.is_some() { self.m_z_k = later.m_z_k; }
        if later.torsion_k.is_some() { self.torsion_k = later.torsion_k; }
        if later.local_load_k.is_some() { self.local_load_k = later.local_load_k; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::MemberAction) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            category: self.category.as_ref().map(|_| row.category.clone()),
            source: self.source.as_ref().map(|_| row.source.clone()),
            g_k_line: self.g_k_line.as_ref().map(|_| row.g_k_line.clone()),
            q_k_line: self.q_k_line.as_ref().map(|_| row.q_k_line.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_y_k: self.v_y_k.as_ref().map(|_| row.v_y_k.clone()),
            v_z_k: self.v_z_k.as_ref().map(|_| row.v_z_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
            m_z_k: self.m_z_k.as_ref().map(|_| row.m_z_k.clone()),
            torsion_k: self.torsion_k.as_ref().map(|_| row.torsion_k.clone()),
            local_load_k: self.local_load_k.as_ref().map(|_| row.local_load_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.category.is_none() && self.source.is_none() && self.g_k_line.is_none() && self.q_k_line.is_none() && self.n_k.is_none() && self.v_y_k.is_none() && self.v_z_k.is_none() && self.m_y_k.is_none() && self.m_z_k.is_none() && self.torsion_k.is_none() && self.local_load_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1999ColdFormedActionsRows { removal: En1999ColdFormedActionsRemoved, insertion: En1999ColdFormedActionsInserted, relocation: En1999ColdFormedActionsMoved, modification: En1999ColdFormedActionsModified, row: semio_s_artifact_norm_en1999_model::MemberAction, patch: En1999ColdFormedActionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::MemberAction>, key: String = by MemberActionKey }
}

/// 🩹️ Sparse patch of the `formed` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ColdFormedPatch {
    pub material_id: Option<String>,
    pub thickness: Option<f64>,
    pub width: Option<f64>,
    pub span: Option<f64>,
    pub welded: Option<bool>,
    pub actions: Option<En1999ColdFormedActionsRows>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::ColdFormedSheet> for En1999ColdFormedPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::ColdFormedSheet, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.width { row.width = value.clone(); }
        if let Some(value) = &self.span { row.span = value.clone(); }
        if let Some(value) = &self.welded { row.welded = value.clone(); }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.material_id.is_some() { self.material_id = later.material_id; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.width.is_some() { self.width = later.width; }
        if later.span.is_some() { self.span = later.span; }
        if later.welded.is_some() { self.welded = later.welded; }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::ColdFormedSheet) -> Self {
        Self {
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            span: self.span.as_ref().map(|_| row.span.clone()),
            welded: self.welded.as_ref().map(|_| row.welded.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.material_id.is_none() && self.thickness.is_none() && self.width.is_none() && self.span.is_none() && self.welded.is_none() && self.actions.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `cold_formed` list (rows keyed by `id`).
    pub En1999ColdFormedRows { removal: En1999ColdFormedRemoved, insertion: En1999ColdFormedInserted, relocation: En1999ColdFormedMoved, modification: En1999ColdFormedModified, row: semio_s_artifact_norm_en1999_model::ColdFormedSheet, patch: En1999ColdFormedPatch, list: Vec<semio_s_artifact_norm_en1999_model::ColdFormedSheet>, key: String = by ColdFormedKey }
}


/// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ShellsActionsPatch {
    pub kind: Option<semio_s_artifact_norm_en1999_model::ActionKind>,
    pub category: Option<semio_s_artifact_norm_en1999_model::ActionCategory>,
    pub source: Option<semio_s_artifact_norm_en1999_model::ActionSource>,
    pub g_k_line: Option<f64>,
    pub q_k_line: Option<f64>,
    pub n_k: Option<f64>,
    pub v_y_k: Option<f64>,
    pub v_z_k: Option<f64>,
    pub m_y_k: Option<f64>,
    pub m_z_k: Option<f64>,
    pub torsion_k: Option<f64>,
    pub local_load_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::MemberAction> for En1999ShellsActionsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::MemberAction, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.kind { row.kind = value.clone(); }
        if let Some(value) = &self.category { row.category = value.clone(); }
        if let Some(value) = &self.source { row.source = value.clone(); }
        if let Some(value) = &self.g_k_line { row.g_k_line = value.clone(); }
        if let Some(value) = &self.q_k_line { row.q_k_line = value.clone(); }
        if let Some(value) = &self.n_k { row.n_k = value.clone(); }
        if let Some(value) = &self.v_y_k { row.v_y_k = value.clone(); }
        if let Some(value) = &self.v_z_k { row.v_z_k = value.clone(); }
        if let Some(value) = &self.m_y_k { row.m_y_k = value.clone(); }
        if let Some(value) = &self.m_z_k { row.m_z_k = value.clone(); }
        if let Some(value) = &self.torsion_k { row.torsion_k = value.clone(); }
        if let Some(value) = &self.local_load_k { row.local_load_k = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.kind.is_some() { self.kind = later.kind; }
        if later.category.is_some() { self.category = later.category; }
        if later.source.is_some() { self.source = later.source; }
        if later.g_k_line.is_some() { self.g_k_line = later.g_k_line; }
        if later.q_k_line.is_some() { self.q_k_line = later.q_k_line; }
        if later.n_k.is_some() { self.n_k = later.n_k; }
        if later.v_y_k.is_some() { self.v_y_k = later.v_y_k; }
        if later.v_z_k.is_some() { self.v_z_k = later.v_z_k; }
        if later.m_y_k.is_some() { self.m_y_k = later.m_y_k; }
        if later.m_z_k.is_some() { self.m_z_k = later.m_z_k; }
        if later.torsion_k.is_some() { self.torsion_k = later.torsion_k; }
        if later.local_load_k.is_some() { self.local_load_k = later.local_load_k; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::MemberAction) -> Self {
        Self {
            kind: self.kind.as_ref().map(|_| row.kind.clone()),
            category: self.category.as_ref().map(|_| row.category.clone()),
            source: self.source.as_ref().map(|_| row.source.clone()),
            g_k_line: self.g_k_line.as_ref().map(|_| row.g_k_line.clone()),
            q_k_line: self.q_k_line.as_ref().map(|_| row.q_k_line.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_y_k: self.v_y_k.as_ref().map(|_| row.v_y_k.clone()),
            v_z_k: self.v_z_k.as_ref().map(|_| row.v_z_k.clone()),
            m_y_k: self.m_y_k.as_ref().map(|_| row.m_y_k.clone()),
            m_z_k: self.m_z_k.as_ref().map(|_| row.m_z_k.clone()),
            torsion_k: self.torsion_k.as_ref().map(|_| row.torsion_k.clone()),
            local_load_k: self.local_load_k.as_ref().map(|_| row.local_load_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.kind.is_none() && self.category.is_none() && self.source.is_none() && self.g_k_line.is_none() && self.q_k_line.is_none() && self.n_k.is_none() && self.v_y_k.is_none() && self.v_z_k.is_none() && self.m_y_k.is_none() && self.m_z_k.is_none() && self.torsion_k.is_none() && self.local_load_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1999ShellsActionsRows { removal: En1999ShellsActionsRemoved, insertion: En1999ShellsActionsInserted, relocation: En1999ShellsActionsMoved, modification: En1999ShellsActionsModified, row: semio_s_artifact_norm_en1999_model::MemberAction, patch: En1999ShellsActionsPatch, list: Vec<semio_s_artifact_norm_en1999_model::MemberAction>, key: String = by MemberActionKey }
}

/// 🩹️ Sparse patch of the `shells` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999ShellsPatch {
    pub material_id: Option<String>,
    pub radius: Option<f64>,
    pub thickness: Option<f64>,
    pub length: Option<f64>,
    pub welded: Option<bool>,
    pub fabrication_class: Option<semio_s_artifact_norm_en1999_model::FabricationClass>,
    pub actions: Option<En1999ShellsActionsRows>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::AluminiumShell> for En1999ShellsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::AluminiumShell, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.material_id { row.material_id = value.clone(); }
        if let Some(value) = &self.radius { row.radius = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.length { row.length = value.clone(); }
        if let Some(value) = &self.welded { row.welded = value.clone(); }
        if let Some(value) = &self.fabrication_class { row.fabrication_class = value.clone(); }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.material_id.is_some() { self.material_id = later.material_id; }
        if later.radius.is_some() { self.radius = later.radius; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.length.is_some() { self.length = later.length; }
        if later.welded.is_some() { self.welded = later.welded; }
        if later.fabrication_class.is_some() { self.fabrication_class = later.fabrication_class; }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::AluminiumShell) -> Self {
        Self {
            material_id: self.material_id.as_ref().map(|_| row.material_id.clone()),
            radius: self.radius.as_ref().map(|_| row.radius.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            length: self.length.as_ref().map(|_| row.length.clone()),
            welded: self.welded.as_ref().map(|_| row.welded.clone()),
            fabrication_class: self.fabrication_class.as_ref().map(|_| row.fabrication_class.clone()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.material_id.is_none() && self.radius.is_none() && self.thickness.is_none() && self.length.is_none() && self.welded.is_none() && self.fabrication_class.is_none() && self.actions.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `shells` list (rows keyed by `id`).
    pub En1999ShellsRows { removal: En1999ShellsRemoved, insertion: En1999ShellsInserted, relocation: En1999ShellsMoved, modification: En1999ShellsModified, row: semio_s_artifact_norm_en1999_model::AluminiumShell, patch: En1999ShellsPatch, list: Vec<semio_s_artifact_norm_en1999_model::AluminiumShell>, key: String = by ShellKey }
}


/// 🩹️ Sparse patch of the `placements` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999PlacementsPatch {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
    pub yaw: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::Placement> for En1999PlacementsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::Placement, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.x { row.x = value.clone(); }
        if let Some(value) = &self.y { row.y = value.clone(); }
        if let Some(value) = &self.z { row.z = value.clone(); }
        if let Some(value) = &self.yaw { row.yaw = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.x.is_some() { self.x = later.x; }
        if later.y.is_some() { self.y = later.y; }
        if later.z.is_some() { self.z = later.z; }
        if later.yaw.is_some() { self.yaw = later.yaw; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::Placement) -> Self {
        Self {
            x: self.x.as_ref().map(|_| row.x.clone()),
            y: self.y.as_ref().map(|_| row.y.clone()),
            z: self.z.as_ref().map(|_| row.z.clone()),
            yaw: self.yaw.as_ref().map(|_| row.yaw.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.z.is_none() && self.yaw.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `placements` list (rows keyed by `id`).
    pub En1999PlacementsRows { removal: En1999PlacementsRemoved, insertion: En1999PlacementsInserted, relocation: En1999PlacementsMoved, modification: En1999PlacementsModified, row: semio_s_artifact_norm_en1999_model::Placement, patch: En1999PlacementsPatch, list: Vec<semio_s_artifact_norm_en1999_model::Placement>, key: String = by PlacementKey }
}


/// 🩹️ Sparse patch of the `runs` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999WeldRunsPatch {
    pub member_id: Option<String>,
    pub element_id: Option<String>,
    pub start: Option<f64>,
    pub length: Option<f64>,
    pub offset: Option<f64>,
    pub throat: Option<f64>,
    pub process: Option<semio_s_artifact_norm_en1999_model::WeldProcess>,
    pub t_junction: Option<bool>,
    pub direction: Option<semio_s_artifact_norm_en1999_model::WeldDirection>,
    pub haz_extent: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::WeldRun> for En1999WeldRunsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::WeldRun, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.element_id { row.element_id = value.clone(); }
        if let Some(value) = &self.start { row.start = value.clone(); }
        if let Some(value) = &self.length { row.length = value.clone(); }
        if let Some(value) = &self.offset { row.offset = value.clone(); }
        if let Some(value) = &self.throat { row.throat = value.clone(); }
        if let Some(value) = &self.process { row.process = value.clone(); }
        if let Some(value) = &self.t_junction { row.t_junction = value.clone(); }
        if let Some(value) = &self.direction { row.direction = value.clone(); }
        if let Some(value) = &self.haz_extent { row.haz_extent = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.element_id.is_some() { self.element_id = later.element_id; }
        if later.start.is_some() { self.start = later.start; }
        if later.length.is_some() { self.length = later.length; }
        if later.offset.is_some() { self.offset = later.offset; }
        if later.throat.is_some() { self.throat = later.throat; }
        if later.process.is_some() { self.process = later.process; }
        if later.t_junction.is_some() { self.t_junction = later.t_junction; }
        if later.direction.is_some() { self.direction = later.direction; }
        if later.haz_extent.is_some() { self.haz_extent = later.haz_extent; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::WeldRun) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            element_id: self.element_id.as_ref().map(|_| row.element_id.clone()),
            start: self.start.as_ref().map(|_| row.start.clone()),
            length: self.length.as_ref().map(|_| row.length.clone()),
            offset: self.offset.as_ref().map(|_| row.offset.clone()),
            throat: self.throat.as_ref().map(|_| row.throat.clone()),
            process: self.process.as_ref().map(|_| row.process.clone()),
            t_junction: self.t_junction.as_ref().map(|_| row.t_junction.clone()),
            direction: self.direction.as_ref().map(|_| row.direction.clone()),
            haz_extent: self.haz_extent.as_ref().map(|_| row.haz_extent.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.element_id.is_none() && self.start.is_none() && self.length.is_none() && self.offset.is_none() && self.throat.is_none() && self.process.is_none() && self.t_junction.is_none() && self.direction.is_none() && self.haz_extent.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `weld_runs` list (rows keyed by `id`).
    pub En1999WeldRunsRows { removal: En1999WeldRunsRemoved, insertion: En1999WeldRunsInserted, relocation: En1999WeldRunsMoved, modification: En1999WeldRunsModified, row: semio_s_artifact_norm_en1999_model::WeldRun, patch: En1999WeldRunsPatch, list: Vec<semio_s_artifact_norm_en1999_model::WeldRun>, key: String = by WeldRunKey }
}


/// 🩹️ Sparse patch of the `vertices` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999OutlineVerticesPatch {
    pub section_id: Option<String>,
    pub ring: Option<semio_s_artifact_norm_en1999_model::OutlineRing>,
    pub element_id: Option<String>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::SectionVertex> for En1999OutlineVerticesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::SectionVertex, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.section_id { row.section_id = value.clone(); }
        if let Some(value) = &self.ring { row.ring = value.clone(); }
        if let Some(value) = &self.element_id { row.element_id = value.clone(); }
        if let Some(value) = &self.y { row.y = value.clone(); }
        if let Some(value) = &self.z { row.z = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.section_id.is_some() { self.section_id = later.section_id; }
        if later.ring.is_some() { self.ring = later.ring; }
        if later.element_id.is_some() { self.element_id = later.element_id; }
        if later.y.is_some() { self.y = later.y; }
        if later.z.is_some() { self.z = later.z; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::SectionVertex) -> Self {
        Self {
            section_id: self.section_id.as_ref().map(|_| row.section_id.clone()),
            ring: self.ring.as_ref().map(|_| row.ring.clone()),
            element_id: self.element_id.as_ref().map(|_| row.element_id.clone()),
            y: self.y.as_ref().map(|_| row.y.clone()),
            z: self.z.as_ref().map(|_| row.z.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.section_id.is_none() && self.ring.is_none() && self.element_id.is_none() && self.y.is_none() && self.z.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `outline_vertices` list (rows keyed by `id`).
    pub En1999OutlineVerticesRows { removal: En1999OutlineVerticesRemoved, insertion: En1999OutlineVerticesInserted, relocation: En1999OutlineVerticesMoved, modification: En1999OutlineVerticesModified, row: semio_s_artifact_norm_en1999_model::SectionVertex, patch: En1999OutlineVerticesPatch, list: Vec<semio_s_artifact_norm_en1999_model::SectionVertex>, key: String = by SectionVertexKey }
}
/// 🩹️ Sparse patch of the `categories` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999FatigueCategoriesPatch {
    pub member_id: Option<String>,
    pub element_id: Option<String>,
    pub detail_category: Option<String>,
    pub detail_type: Option<semio_s_artifact_norm_en1999_model::DetailType>,
    pub location_x: Option<f64>,
    pub m1: Option<f64>,
    pub m2: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::FatigueCategory> for En1999FatigueCategoriesPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::FatigueCategory, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.member_id { row.member_id = value.clone(); }
        if let Some(value) = &self.element_id { row.element_id = value.clone(); }
        if let Some(value) = &self.detail_category { row.detail_category = value.clone(); }
        if let Some(value) = &self.detail_type { row.detail_type = value.clone(); }
        if let Some(value) = &self.location_x { row.location_x = value.clone(); }
        if let Some(value) = &self.m1 { row.m1 = value.clone(); }
        if let Some(value) = &self.m2 { row.m2 = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.member_id.is_some() { self.member_id = later.member_id; }
        if later.element_id.is_some() { self.element_id = later.element_id; }
        if later.detail_category.is_some() { self.detail_category = later.detail_category; }
        if later.detail_type.is_some() { self.detail_type = later.detail_type; }
        if later.location_x.is_some() { self.location_x = later.location_x; }
        if later.m1.is_some() { self.m1 = later.m1; }
        if later.m2.is_some() { self.m2 = later.m2; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::FatigueCategory) -> Self {
        Self {
            member_id: self.member_id.as_ref().map(|_| row.member_id.clone()),
            element_id: self.element_id.as_ref().map(|_| row.element_id.clone()),
            detail_category: self.detail_category.as_ref().map(|_| row.detail_category.clone()),
            detail_type: self.detail_type.as_ref().map(|_| row.detail_type.clone()),
            location_x: self.location_x.as_ref().map(|_| row.location_x.clone()),
            m1: self.m1.as_ref().map(|_| row.m1.clone()),
            m2: self.m2.as_ref().map(|_| row.m2.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.member_id.is_none() && self.element_id.is_none() && self.detail_category.is_none() && self.detail_type.is_none() && self.location_x.is_none() && self.m1.is_none() && self.m2.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `fatigue_categories` list (rows keyed by `id`).
    pub En1999FatigueCategoriesRows { removal: En1999FatigueCategoriesRemoved, insertion: En1999FatigueCategoriesInserted, relocation: En1999FatigueCategoriesMoved, modification: En1999FatigueCategoriesModified, row: semio_s_artifact_norm_en1999_model::FatigueCategory, patch: En1999FatigueCategoriesPatch, list: Vec<semio_s_artifact_norm_en1999_model::FatigueCategory>, key: String = by FatigueCategoryKey }
}
/// 🩹️ Sparse patch of the `loads` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999PatchLoadsPatch {
    pub subject_id: Option<String>,
    pub position: Option<semio_s_artifact_norm_en1999_model::PatchLoadPosition>,
    pub x: Option<f64>,
    pub bearing_length: Option<f64>,
    pub web_height: Option<f64>,
    pub web_angle: Option<f64>,
    pub corner_radius: Option<f64>,
    pub load: Option<f64>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::PatchLoad> for En1999PatchLoadsPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::PatchLoad, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.subject_id { row.subject_id = value.clone(); }
        if let Some(value) = &self.position { row.position = value.clone(); }
        if let Some(value) = &self.x { row.x = value.clone(); }
        if let Some(value) = &self.bearing_length { row.bearing_length = value.clone(); }
        if let Some(value) = &self.web_height { row.web_height = value.clone(); }
        if let Some(value) = &self.web_angle { row.web_angle = value.clone(); }
        if let Some(value) = &self.corner_radius { row.corner_radius = value.clone(); }
        if let Some(value) = &self.load { row.load = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.subject_id.is_some() { self.subject_id = later.subject_id; }
        if later.position.is_some() { self.position = later.position; }
        if later.x.is_some() { self.x = later.x; }
        if later.bearing_length.is_some() { self.bearing_length = later.bearing_length; }
        if later.web_height.is_some() { self.web_height = later.web_height; }
        if later.web_angle.is_some() { self.web_angle = later.web_angle; }
        if later.corner_radius.is_some() { self.corner_radius = later.corner_radius; }
        if later.load.is_some() { self.load = later.load; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::PatchLoad) -> Self {
        Self {
            subject_id: self.subject_id.as_ref().map(|_| row.subject_id.clone()),
            position: self.position.as_ref().map(|_| row.position.clone()),
            x: self.x.as_ref().map(|_| row.x.clone()),
            bearing_length: self.bearing_length.as_ref().map(|_| row.bearing_length.clone()),
            web_height: self.web_height.as_ref().map(|_| row.web_height.clone()),
            web_angle: self.web_angle.as_ref().map(|_| row.web_angle.clone()),
            corner_radius: self.corner_radius.as_ref().map(|_| row.corner_radius.clone()),
            load: self.load.as_ref().map(|_| row.load.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.subject_id.is_none() && self.position.is_none() && self.x.is_none() && self.bearing_length.is_none() && self.web_height.is_none() && self.web_angle.is_none() && self.corner_radius.is_none() && self.load.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `patch_loads` list (rows keyed by `id`).
    pub En1999PatchLoadsRows { removal: En1999PatchLoadsRemoved, insertion: En1999PatchLoadsInserted, relocation: En1999PatchLoadsMoved, modification: En1999PatchLoadsModified, row: semio_s_artifact_norm_en1999_model::PatchLoad, patch: En1999PatchLoadsPatch, list: Vec<semio_s_artifact_norm_en1999_model::PatchLoad>, key: String = by PatchLoadKey }
}
/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999RingStiffenersPatch {
    pub shell_id: Option<String>,
    pub x: Option<f64>,
    pub depth: Option<f64>,
    pub thickness: Option<f64>,
    pub internal: Option<bool>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::RingStiffener> for En1999RingStiffenersPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::RingStiffener, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.shell_id { row.shell_id = value.clone(); }
        if let Some(value) = &self.x { row.x = value.clone(); }
        if let Some(value) = &self.depth { row.depth = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.internal { row.internal = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.shell_id.is_some() { self.shell_id = later.shell_id; }
        if later.x.is_some() { self.x = later.x; }
        if later.depth.is_some() { self.depth = later.depth; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.internal.is_some() { self.internal = later.internal; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::RingStiffener) -> Self {
        Self {
            shell_id: self.shell_id.as_ref().map(|_| row.shell_id.clone()),
            x: self.x.as_ref().map(|_| row.x.clone()),
            depth: self.depth.as_ref().map(|_| row.depth.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            internal: self.internal.as_ref().map(|_| row.internal.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.shell_id.is_none() && self.x.is_none() && self.depth.is_none() && self.thickness.is_none() && self.internal.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `ring_stiffeners` list (rows keyed by `id`).
    pub En1999RingStiffenersRows { removal: En1999RingStiffenersRemoved, insertion: En1999RingStiffenersInserted, relocation: En1999RingStiffenersMoved, modification: En1999RingStiffenersModified, row: semio_s_artifact_norm_en1999_model::RingStiffener, patch: En1999RingStiffenersPatch, list: Vec<semio_s_artifact_norm_en1999_model::RingStiffener>, key: String = by RingStiffenerKey }
}
/// 🩹️ Sparse patch of the `stiffeners` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1999TransverseStiffenersPatch {
    pub girder_id: Option<String>,
    pub position: Option<f64>,
    pub width: Option<f64>,
    pub thickness: Option<f64>,
    pub sides: Option<semio_s_artifact_norm_en1999_model::StiffenerSides>,
}

impl protocol::list_delta::RowPatch<semio_s_artifact_norm_en1999_model::TransverseStiffener> for En1999TransverseStiffenersPatch {
    fn commit_into(&self, row: &mut semio_s_artifact_norm_en1999_model::TransverseStiffener, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.girder_id { row.girder_id = value.clone(); }
        if let Some(value) = &self.position { row.position = value.clone(); }
        if let Some(value) = &self.width { row.width = value.clone(); }
        if let Some(value) = &self.thickness { row.thickness = value.clone(); }
        if let Some(value) = &self.sides { row.sides = value.clone(); }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.girder_id.is_some() { self.girder_id = later.girder_id; }
        if later.position.is_some() { self.position = later.position; }
        if later.width.is_some() { self.width = later.width; }
        if later.thickness.is_some() { self.thickness = later.thickness; }
        if later.sides.is_some() { self.sides = later.sides; }
    }

    fn inverse(&self, row: &semio_s_artifact_norm_en1999_model::TransverseStiffener) -> Self {
        Self {
            girder_id: self.girder_id.as_ref().map(|_| row.girder_id.clone()),
            position: self.position.as_ref().map(|_| row.position.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            thickness: self.thickness.as_ref().map(|_| row.thickness.clone()),
            sides: self.sides.as_ref().map(|_| row.sides.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.girder_id.is_none() && self.position.is_none() && self.width.is_none() && self.thickness.is_none() && self.sides.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `transverse_stiffeners` list (rows keyed by `id`).
    pub En1999TransverseStiffenersRows { removal: En1999TransverseStiffenersRemoved, insertion: En1999TransverseStiffenersInserted, relocation: En1999TransverseStiffenersMoved, modification: En1999TransverseStiffenersModified, row: semio_s_artifact_norm_en1999_model::TransverseStiffener, patch: En1999TransverseStiffenersPatch, list: Vec<semio_s_artifact_norm_en1999_model::TransverseStiffener>, key: String = by TransverseStiffenerKey }
}


    /// 🔺️ Keyed sparse diff of the En1999 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
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
    #[state(artifact)]
    pub placements: Option<En1999PlacementsRows>,
    #[state(artifact)]
    pub weld_runs: Option<En1999WeldRunsRows>,
    #[state(artifact)]
    pub outline_vertices: Option<En1999OutlineVerticesRows>,
    #[state(artifact)]
    pub fatigue_categories: Option<En1999FatigueCategoriesRows>,
    #[state(artifact)]
    pub patch_loads: Option<En1999PatchLoadsRows>,
    #[state(artifact)]
    pub ring_stiffeners: Option<En1999RingStiffenersRows>,
    #[state(artifact)]
    pub transverse_stiffeners: Option<En1999TransverseStiffenersRows>,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
}

impl protocol::MutationDiff<En1999Snapshot> for En1999Diff {}

impl protocol::ColdMutationDiff<En1999Snapshot> for En1999Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
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
        if let Some(theirs) = other.placements {
            match self.placements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.placements = Some(theirs),
            }
            self.placements = self.placements.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.weld_runs {
            match self.weld_runs.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.weld_runs = Some(theirs),
            }
            self.weld_runs = self.weld_runs.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.outline_vertices {
            match self.outline_vertices.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.outline_vertices = Some(theirs),
            }
            self.outline_vertices = self.outline_vertices.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.fatigue_categories {
            match self.fatigue_categories.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.fatigue_categories = Some(theirs),
            }
            self.fatigue_categories = self.fatigue_categories.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.patch_loads {
            match self.patch_loads.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.patch_loads = Some(theirs),
            }
            self.patch_loads = self.patch_loads.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.ring_stiffeners {
            match self.ring_stiffeners.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.ring_stiffeners = Some(theirs),
            }
            self.ring_stiffeners = self.ring_stiffeners.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.transverse_stiffeners {
            match self.transverse_stiffeners.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.transverse_stiffeners = Some(theirs),
            }
            self.transverse_stiffeners = self.transverse_stiffeners.take().filter(|rows| !rows.is_empty());
        }
    }

    fn apply(&self, unpublished: &mut En1999Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.reliability_class {
            next.reliability_class = value.clone();
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
        if let Some(rows) = &self.placements {
            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        }
        if let Some(rows) = &self.weld_runs {
            next.weld_runs = rows.commit_onto(&base.weld_runs, capability).map_err(|error| error.under(["weld_runs"]))?;
        }
        if let Some(rows) = &self.outline_vertices {
            next.outline_vertices = rows.commit_onto(&base.outline_vertices, capability).map_err(|error| error.under(["outline_vertices"]))?;
        }
        if let Some(rows) = &self.fatigue_categories {
            next.fatigue_categories = rows.commit_onto(&base.fatigue_categories, capability).map_err(|error| error.under(["fatigue_categories"]))?;
        }
        if let Some(rows) = &self.patch_loads {
            next.patch_loads = rows.commit_onto(&base.patch_loads, capability).map_err(|error| error.under(["patch_loads"]))?;
        }
        if let Some(rows) = &self.ring_stiffeners {
            next.ring_stiffeners = rows.commit_onto(&base.ring_stiffeners, capability).map_err(|error| error.under(["ring_stiffeners"]))?;
        }
        if let Some(rows) = &self.transverse_stiffeners {
            next.transverse_stiffeners = rows.commit_onto(&base.transverse_stiffeners, capability).map_err(|error| error.under(["transverse_stiffeners"]))?;
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<En1999Snapshot> for En1999Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.reliability_class.is_none() && self.materials.as_ref().map_or(true, |rows| rows.is_empty()) && self.sections.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.connections.as_ref().map_or(true, |rows| rows.is_empty()) && self.fire_scenarios.as_ref().map_or(true, |rows| rows.is_empty()) && self.fatigue_details.as_ref().map_or(true, |rows| rows.is_empty()) && self.cold_formed.as_ref().map_or(true, |rows| rows.is_empty()) && self.shells.as_ref().map_or(true, |rows| rows.is_empty()) && self.placements.as_ref().map_or(true, |rows| rows.is_empty()) && self.weld_runs.as_ref().map_or(true, |rows| rows.is_empty()) && self.outline_vertices.as_ref().map_or(true, |rows| rows.is_empty()) && self.fatigue_categories.as_ref().map_or(true, |rows| rows.is_empty()) && self.patch_loads.as_ref().map_or(true, |rows| rows.is_empty()) && self.ring_stiffeners.as_ref().map_or(true, |rows| rows.is_empty()) && self.transverse_stiffeners.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

impl protocol::ColdDiffAlgebra<En1999Snapshot> for En1999Diff {
    fn inverse(&self, base: &En1999Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class.clone()),
            materials: self.materials.as_ref().map(|rows| rows.inverse(&base.materials)).filter(|rows| !rows.is_empty()),
            sections: self.sections.as_ref().map(|rows| rows.inverse(&base.sections)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse(&base.members)).filter(|rows| !rows.is_empty()),
            connections: self.connections.as_ref().map(|rows| rows.inverse(&base.connections)).filter(|rows| !rows.is_empty()),
            fire_scenarios: self.fire_scenarios.as_ref().map(|rows| rows.inverse(&base.fire_scenarios)).filter(|rows| !rows.is_empty()),
            fatigue_details: self.fatigue_details.as_ref().map(|rows| rows.inverse(&base.fatigue_details)).filter(|rows| !rows.is_empty()),
            cold_formed: self.cold_formed.as_ref().map(|rows| rows.inverse(&base.cold_formed)).filter(|rows| !rows.is_empty()),
            shells: self.shells.as_ref().map(|rows| rows.inverse(&base.shells)).filter(|rows| !rows.is_empty()),
            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),
            weld_runs: self.weld_runs.as_ref().map(|rows| rows.inverse(&base.weld_runs)).filter(|rows| !rows.is_empty()),
            outline_vertices: self.outline_vertices.as_ref().map(|rows| rows.inverse(&base.outline_vertices)).filter(|rows| !rows.is_empty()),
            fatigue_categories: self.fatigue_categories.as_ref().map(|rows| rows.inverse(&base.fatigue_categories)).filter(|rows| !rows.is_empty()),
            patch_loads: self.patch_loads.as_ref().map(|rows| rows.inverse(&base.patch_loads)).filter(|rows| !rows.is_empty()),
            ring_stiffeners: self.ring_stiffeners.as_ref().map(|rows| rows.inverse(&base.ring_stiffeners)).filter(|rows| !rows.is_empty()),
            transverse_stiffeners: self.transverse_stiffeners.as_ref().map(|rows| rows.inverse(&base.transverse_stiffeners)).filter(|rows| !rows.is_empty()),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1999_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1999_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1999_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1999_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves
