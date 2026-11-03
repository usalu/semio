//! 🧬️ En1992 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
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

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

/// 🔑️ Key extractors of the foreign model rows (`id`), local markers for `list_delta!`.
macro_rules! row_key {
    ($($key:ident for $row:ident;)+) => {$(
        #[doc = concat!("🔑️ Key extractor of the foreign `", stringify!($row), "` rows (`id`).")]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $key;
        impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1992_model::$row> for $key {
            type Key = String;
            fn key_of(row: &semio_s_artifact_norm_en1992_model::$row) -> String { row.id.clone() }
        }
    )+};
}

row_key! {
    ConcreteGradeKey for ConcreteGrade;
    ReinforcementGradeKey for ReinforcementGrade;
    PrestressSteelKey for PrestressSteel;
    BarLayerKey for BarLayer;
    SectionPointKey for SectionPoint;
    BarPointKey for BarPoint;
    TendonPointKey for TendonPoint;
    StudRowKey for StudRow;
    OpeningKey for Opening;
    LoadCaseActionsKey for LoadCaseActions;
    RcMemberKey for RcMember;
    AnchorKey for Anchor;
    PlacementKey for Placement;
}

/// 🩹️ Sparse scalar patch of one keyed row: the struct, its commit, coalescing, inverse and emptiness.
macro_rules! scalar_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { $($f:ident: $t:ty),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>),+ }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })+
                Ok(())
            }
            fn absorb(&mut self, later: Self) { $(if later.$f.is_some() { self.$f = later.$f; })+ }
            fn inverse(&self, row: &$row) -> Self { Self { $($f: self.$f.as_ref().map(|_| row.$f.clone())),+ } }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())+ }
        }
    };
}

/// 🩹️ Sparse patch of one keyed row with direct leaves (`set`), leaves behind an optional object (`within`) and owned row lanes (`rows`): the struct, its commit, coalescing, inverse and emptiness.
macro_rules! nested_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { set { $($f:ident: $t:ty),* $(,)? } within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } optional { $($o:ident : $ot:ty => $oflag:ident),* $(,)? } rows { $($lane:ident : $rows:ty = $label:literal),* $(,)? } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>,)* $(pub $w: Option<$wt>,)* $(pub $o: Option<$ot>, pub $oflag: Option<bool>,)* $(pub $lane: Option<$rows>,)* }

        impl protocol::list_delta::RowPatch<$row> for $name {
            #[allow(unused_variables)]
            fn commit_into(&self, row: &mut $row, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })*
                $(if let Some(value) = &self.$w { row.$obj.as_mut().ok_or_else(|| missing_target(stringify!($obj)))?.$inner = value.clone(); })*
                $(if self.$oflag == Some(true) { row.$o = None; } else if let Some(value) = &self.$o { row.$o = Some(value.clone()); })*
                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto(&row.$lane, capability).map_err(|error| error.under([$label]))?; })*
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$f.is_some() { self.$f = later.$f; })*
                $(if later.$w.is_some() { self.$w = later.$w; })*
                $(if later.$oflag == Some(true) { self.$oflag = Some(true); self.$o = None; } else if later.$o.is_some() { self.$o = later.$o; self.$oflag = None; })*
                $(if let Some(theirs) = later.$lane {
                    match self.$lane.as_mut() {
                        Some(mine) => mine.absorb(theirs),
                        None => self.$lane = Some(theirs),
                    }
                    self.$lane = self.$lane.take().filter(|rows| !rows.is_empty());
                })*
            }
            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($f: self.$f.as_ref().map(|_| row.$f.clone()),)*
                    $($w: self.$w.as_ref().and_then(|_| (|| Some(row.$obj.as_ref()?.$inner.clone()))()),)*
                    $($o: if self.$o.is_some() || self.$oflag == Some(true) { row.$o.clone() } else { None }, $oflag: if (self.$o.is_some() || self.$oflag == Some(true)) && row.$o.is_none() { Some(true) } else { None },)*
                    $($lane: self.$lane.as_ref().map(|rows| rows.inverse(&row.$lane)).filter(|rows| !rows.is_empty()),)*
                }
            }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())* $(&& self.$w.is_none())* $(&& self.$o.is_none() && self.$oflag != Some(true))* $(&& self.$lane.as_ref().map_or(true, |rows| rows.is_empty()))* }
        }
    };
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `concrete_grades` row addressed by its key.
    En1992ConcreteGradesPatch for semio_s_artifact_norm_en1992_model::ConcreteGrade { name: String, f_ck: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `concrete_grades` list (rows keyed by `id`).
    pub En1992ConcreteGradesRows { removal: En1992ConcreteGradesRemoved, insertion: En1992ConcreteGradesInserted, relocation: En1992ConcreteGradesMoved, modification: En1992ConcreteGradesModified, row: semio_s_artifact_norm_en1992_model::ConcreteGrade, patch: En1992ConcreteGradesPatch, list: Vec<semio_s_artifact_norm_en1992_model::ConcreteGrade>, key: String = by ConcreteGradeKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `reinforcement_grades` row addressed by its key.
    En1992ReinforcementGradesPatch for semio_s_artifact_norm_en1992_model::ReinforcementGrade { name: String, f_yk: f64, e_s: f64, ductility: semio_s_artifact_norm_en1992_model::DuctilityClass, k: f64, eps_uk: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `reinforcement_grades` list (rows keyed by `id`).
    pub En1992ReinforcementGradesRows { removal: En1992ReinforcementGradesRemoved, insertion: En1992ReinforcementGradesInserted, relocation: En1992ReinforcementGradesMoved, modification: En1992ReinforcementGradesModified, row: semio_s_artifact_norm_en1992_model::ReinforcementGrade, patch: En1992ReinforcementGradesPatch, list: Vec<semio_s_artifact_norm_en1992_model::ReinforcementGrade>, key: String = by ReinforcementGradeKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `prestress_steels` row addressed by its key.
    En1992PrestressSteelsPatch for semio_s_artifact_norm_en1992_model::PrestressSteel { name: String, f_pk: f64, f_p0_1k: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `prestress_steels` list (rows keyed by `id`).
    pub En1992PrestressSteelsRows { removal: En1992PrestressSteelsRemoved, insertion: En1992PrestressSteelsInserted, relocation: En1992PrestressSteelsMoved, modification: En1992PrestressSteelsModified, row: semio_s_artifact_norm_en1992_model::PrestressSteel, patch: En1992PrestressSteelsPatch, list: Vec<semio_s_artifact_norm_en1992_model::PrestressSteel>, key: String = by PrestressSteelKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `longitudinal` row addressed by its key.
    En1992MembersLongitudinalPatch for semio_s_artifact_norm_en1992_model::BarLayer { diameter: f64, count: u32, position: semio_s_artifact_norm_en1992_model::BarPosition, anchorage_length: f64, lap_length: f64, bond_condition: semio_s_artifact_norm_en1992_model::BondCondition, aggregate_size: f64, lap_share: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `longitudinal` list (rows keyed by `id`).
    pub En1992MembersLongitudinalRows { removal: En1992MembersLongitudinalRemoved, insertion: En1992MembersLongitudinalInserted, relocation: En1992MembersLongitudinalMoved, modification: En1992MembersLongitudinalModified, row: semio_s_artifact_norm_en1992_model::BarLayer, patch: En1992MembersLongitudinalPatch, list: Vec<semio_s_artifact_norm_en1992_model::BarLayer>, key: String = by BarLayerKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
    En1992MembersActionsPatch for semio_s_artifact_norm_en1992_model::LoadCaseActions { kind: semio_s_artifact_norm_en1992_model::ActionKind, category: semio_s_artifact_norm_en1992_model::PsiCategory, source: semio_s_artifact_norm_en1992_model::ActionSource, g_k_line: f64, q_k_line: f64, point_force: f64, m_k: f64, n_k: f64, v_k: f64, t_k: f64, v_k_punch: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1992MembersActionsRows { removal: En1992MembersActionsRemoved, insertion: En1992MembersActionsInserted, relocation: En1992MembersActionsMoved, modification: En1992MembersActionsModified, row: semio_s_artifact_norm_en1992_model::LoadCaseActions, patch: En1992MembersActionsPatch, list: Vec<semio_s_artifact_norm_en1992_model::LoadCaseActions>, key: String = by LoadCaseActionsKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `section_points` row addressed by its key.
    En1992MembersSectionPointsPatch for semio_s_artifact_norm_en1992_model::SectionPoint { y: f64, z: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `section_points` list (rows keyed by `id`).
    pub En1992MembersSectionPointsRows { removal: En1992MembersSectionPointsRemoved, insertion: En1992MembersSectionPointsInserted, relocation: En1992MembersSectionPointsMoved, modification: En1992MembersSectionPointsModified, row: semio_s_artifact_norm_en1992_model::SectionPoint, patch: En1992MembersSectionPointsPatch, list: Vec<semio_s_artifact_norm_en1992_model::SectionPoint>, key: String = by SectionPointKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `bars` row addressed by its key.
    En1992MembersBarsPatch for semio_s_artifact_norm_en1992_model::BarPoint { layer_id: String, y: f64, z: f64, diameter: f64, length: f64, bend: semio_s_artifact_norm_en1992_model::BarBend }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `bars` list (rows keyed by `id`).
    pub En1992MembersBarsRows { removal: En1992MembersBarsRemoved, insertion: En1992MembersBarsInserted, relocation: En1992MembersBarsMoved, modification: En1992MembersBarsModified, row: semio_s_artifact_norm_en1992_model::BarPoint, patch: En1992MembersBarsPatch, list: Vec<semio_s_artifact_norm_en1992_model::BarPoint>, key: String = by BarPointKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `tendon_points` row addressed by its key.
    En1992MembersTendonPointsPatch for semio_s_artifact_norm_en1992_model::TendonPoint { x: f64, y: f64, z: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `tendon_points` list (rows keyed by `id`).
    pub En1992MembersTendonPointsRows { removal: En1992MembersTendonPointsRemoved, insertion: En1992MembersTendonPointsInserted, relocation: En1992MembersTendonPointsMoved, modification: En1992MembersTendonPointsModified, row: semio_s_artifact_norm_en1992_model::TendonPoint, patch: En1992MembersTendonPointsPatch, list: Vec<semio_s_artifact_norm_en1992_model::TendonPoint>, key: String = by TendonPointKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `stud_rows` row addressed by its key.
    En1992MembersStudRowsPatch for semio_s_artifact_norm_en1992_model::StudRow { angle: f64, count: u32, spacing_first: f64, spacing_radial: f64, diameter: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `stud_rows` list (rows keyed by `id`).
    pub En1992MembersStudRowsRows { removal: En1992MembersStudRowsRemoved, insertion: En1992MembersStudRowsInserted, relocation: En1992MembersStudRowsMoved, modification: En1992MembersStudRowsModified, row: semio_s_artifact_norm_en1992_model::StudRow, patch: En1992MembersStudRowsPatch, list: Vec<semio_s_artifact_norm_en1992_model::StudRow>, key: String = by StudRowKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `openings` row addressed by its key.
    En1992MembersOpeningsPatch for semio_s_artifact_norm_en1992_model::Opening { x: f64, y: f64, width: f64, depth: f64 }
}
protocol::list_delta! {
    /// 📋️ Positional row delta of the `openings` list (rows keyed by `id`).
    pub En1992MembersOpeningsRows { removal: En1992MembersOpeningsRemoved, insertion: En1992MembersOpeningsInserted, relocation: En1992MembersOpeningsMoved, modification: En1992MembersOpeningsModified, row: semio_s_artifact_norm_en1992_model::Opening, patch: En1992MembersOpeningsPatch, list: Vec<semio_s_artifact_norm_en1992_model::Opening>, key: String = by OpeningKey }
}
nested_patch! {
    /// 🩹️ Sparse patch of the `members` row addressed by its key.
    En1992MembersPatch for semio_s_artifact_norm_en1992_model::RcMember {
        set { label_en: String, label_de: String, kind: semio_s_artifact_norm_en1992_model::MemberKind, concrete_grade_id: String, reinforcement_grade_id: String, prestress_steel_id: String, exposure: semio_s_artifact_norm_en1992_model::ExposureClass, width: f64, height: f64, effective_depth: f64, cover: f64, span: f64, support: semio_s_artifact_norm_en1992_model::SupportCondition, buckling_length: f64, use_fem: bool, udl: f64, deflection_sensitive: bool, hd_over_h: f64, liquid_sigma_s: f64, liquid_rho_p_eff: f64, liquid_f_ct_eff: f64, liquid_s_r_max: f64, bridge_sigma_c: f64, bridge_delta_sigma_s: f64, relative_humidity: f64 , bearing_width: f64}
        within { stirrups_diameter => stirrups.diameter: f64, stirrups_spacing => stirrups.spacing: f64, stirrups_legs => stirrups.legs: u32, punching_column_width => punching.column_width: f64, punching_column_depth => punching.column_depth: f64, punching_column_position => punching.column_position: semio_s_artifact_norm_en1992_model::ColumnPosition, punching_asw => punching.asw: f64, punching_beta_method => punching.beta_method: semio_s_artifact_norm_en1992_model::PunchingBetaMethod, punching_moment_c1 => punching.moment_c1: f64, punching_moment_c2 => punching.moment_c2: f64, prestress_force => prestress.force: f64, prestress_area => prestress.area: f64, prestress_eccentricity => prestress.eccentricity: f64, prestress_loss_ratio => prestress.loss_ratio: f64, fire_rating => fire.rating: semio_s_artifact_norm_en1992_model::FireRating, fire_axis_distance => fire.axis_distance: f64, fire_slab_system => fire.slab_system: semio_s_artifact_norm_en1992_model::SlabFireSystem }
        optional { tightness: semio_s_artifact_norm_en1992_model::TightnessClass => clear_tightness }
        rows { longitudinal: En1992MembersLongitudinalRows = "longitudinal", actions: En1992MembersActionsRows = "actions", section_points: En1992MembersSectionPointsRows = "sectionPoints", bars: En1992MembersBarsRows = "bars", tendon_points: En1992MembersTendonPointsRows = "tendonPoints", stud_rows: En1992MembersStudRowsRows = "studRows", openings: En1992MembersOpeningsRows = "openings" }
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `members` list (rows keyed by `id`).
    pub En1992MembersRows { removal: En1992MembersRemoved, insertion: En1992MembersInserted, relocation: En1992MembersMoved, modification: En1992MembersModified, row: semio_s_artifact_norm_en1992_model::RcMember, patch: En1992MembersPatch, list: Vec<semio_s_artifact_norm_en1992_model::RcMember>, key: String = by RcMemberKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `actions` row addressed by its key.
    En1992AnchorsActionsPatch for semio_s_artifact_norm_en1992_model::LoadCaseActions { kind: semio_s_artifact_norm_en1992_model::ActionKind, category: semio_s_artifact_norm_en1992_model::PsiCategory, source: semio_s_artifact_norm_en1992_model::ActionSource, g_k_line: f64, q_k_line: f64, point_force: f64, m_k: f64, n_k: f64, v_k: f64, t_k: f64, v_k_punch: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1992AnchorsActionsRows { removal: En1992AnchorsActionsRemoved, insertion: En1992AnchorsActionsInserted, relocation: En1992AnchorsActionsMoved, modification: En1992AnchorsActionsModified, row: semio_s_artifact_norm_en1992_model::LoadCaseActions, patch: En1992AnchorsActionsPatch, list: Vec<semio_s_artifact_norm_en1992_model::LoadCaseActions>, key: String = by LoadCaseActionsKey }
}

nested_patch! {
    /// 🩹️ Sparse patch of the `anchors` row addressed by its key.
    En1992AnchorsPatch for semio_s_artifact_norm_en1992_model::Anchor {
        set { h_ef: f64, cracked: bool, f_uk: f64, f_yk: f64, a_s: f64, d: f64, c1: f64, f_ck: f64, head_diameter: f64 }
        within {  }
        optional {  }
        rows { actions: En1992AnchorsActionsRows = "actions" }
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `anchors` list (rows keyed by `id`).
    pub En1992AnchorsRows { removal: En1992AnchorsRemoved, insertion: En1992AnchorsInserted, relocation: En1992AnchorsMoved, modification: En1992AnchorsModified, row: semio_s_artifact_norm_en1992_model::Anchor, patch: En1992AnchorsPatch, list: Vec<semio_s_artifact_norm_en1992_model::Anchor>, key: String = by AnchorKey }
}

scalar_patch! {
    /// 🩹️ Sparse patch of the `placements` row addressed by its key.
    En1992PlacementsPatch for semio_s_artifact_norm_en1992_model::Placement { x: f64, y: f64, z: f64, yaw: f64 }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `placements` list (rows keyed by `id`).
    pub En1992PlacementsRows { removal: En1992PlacementsRemoved, insertion: En1992PlacementsInserted, relocation: En1992PlacementsMoved, modification: En1992PlacementsModified, row: semio_s_artifact_norm_en1992_model::Placement, patch: En1992PlacementsPatch, list: Vec<semio_s_artifact_norm_en1992_model::Placement>, key: String = by PlacementKey }
}

/// 🔺️ Keyed sparse diff of the En1992 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub delta_c_dev: Option<f64>,
    #[state(artifact)]
    pub cement_type: Option<semio_s_artifact_norm_en1992_model::CementClass>,
    #[state(artifact)]
    pub concrete_grades: Option<En1992ConcreteGradesRows>,
    #[state(artifact)]
    pub reinforcement_grades: Option<En1992ReinforcementGradesRows>,
    #[state(artifact)]
    pub prestress_steels: Option<En1992PrestressSteelsRows>,
    #[state(artifact)]
    pub members: Option<En1992MembersRows>,
    #[state(artifact)]
    pub anchors: Option<En1992AnchorsRows>,
    #[state(artifact)]
    pub placements: Option<En1992PlacementsRows>,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
}

impl protocol::MutationDiff<En1992Snapshot> for En1992Diff {}

impl protocol::ColdMutationDiff<En1992Snapshot> for En1992Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        if other.title.is_some() {
            self.title = other.title;
        }
        if other.design_working_life_years.is_some() {
            self.design_working_life_years = other.design_working_life_years;
        }
        if other.delta_c_dev.is_some() {
            self.delta_c_dev = other.delta_c_dev;
        }
        if other.cement_type.is_some() {
            self.cement_type = other.cement_type;
        }
        if let Some(theirs) = other.concrete_grades {
            match self.concrete_grades.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.concrete_grades = Some(theirs),
            }
            self.concrete_grades = self.concrete_grades.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.reinforcement_grades {
            match self.reinforcement_grades.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.reinforcement_grades = Some(theirs),
            }
            self.reinforcement_grades = self.reinforcement_grades.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.prestress_steels {
            match self.prestress_steels.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.prestress_steels = Some(theirs),
            }
            self.prestress_steels = self.prestress_steels.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.members {
            match self.members.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.members = Some(theirs),
            }
            self.members = self.members.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.anchors {
            match self.anchors.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.anchors = Some(theirs),
            }
            self.anchors = self.anchors.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.placements {
            match self.placements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.placements = Some(theirs),
            }
            self.placements = self.placements.take().filter(|rows| !rows.is_empty());
        }
        self.cases.absorb(other.cases);
    }

    fn apply(&self, unpublished: &mut En1992Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.reliability_class {
            next.reliability_class = value.clone();
        }
        if let Some(value) = &self.title {
            next.title = value.clone();
        }
        if let Some(value) = &self.design_working_life_years {
            next.design_working_life_years = value.clone();
        }
        if let Some(value) = &self.delta_c_dev {
            next.delta_c_dev = value.clone();
        }
        if let Some(value) = &self.cement_type {
            next.cement_type = value.clone();
        }
        if let Some(rows) = &self.concrete_grades {
            next.concrete_grades = rows.commit_onto(&base.concrete_grades, capability).map_err(|error| error.under(["concrete_grades"]))?;
        }
        if let Some(rows) = &self.reinforcement_grades {
            next.reinforcement_grades = rows.commit_onto(&base.reinforcement_grades, capability).map_err(|error| error.under(["reinforcement_grades"]))?;
        }
        if let Some(rows) = &self.prestress_steels {
            next.prestress_steels = rows.commit_onto(&base.prestress_steels, capability).map_err(|error| error.under(["prestress_steels"]))?;
        }
        if let Some(rows) = &self.members {
            next.members = rows.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?;
        }
        if let Some(rows) = &self.anchors {
            next.anchors = rows.commit_onto(&base.anchors, capability).map_err(|error| error.under(["anchors"]))?;
        }
        if let Some(rows) = &self.placements {
            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        }
        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<En1992Snapshot> for En1992Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.reliability_class.is_none() && self.title.is_none() && self.design_working_life_years.is_none() && self.delta_c_dev.is_none() && self.cement_type.is_none() && self.concrete_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.reinforcement_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.prestress_steels.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.anchors.as_ref().map_or(true, |rows| rows.is_empty()) && self.placements.as_ref().map_or(true, |rows| rows.is_empty()) && self.cases.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1992Snapshot> for En1992Diff {
    fn inverse(&self, base: &En1992Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            design_working_life_years: self.design_working_life_years.as_ref().map(|_| base.design_working_life_years.clone()),
            delta_c_dev: self.delta_c_dev.as_ref().map(|_| base.delta_c_dev.clone()),
            cement_type: self.cement_type.as_ref().map(|_| base.cement_type.clone()),
            concrete_grades: self.concrete_grades.as_ref().map(|rows| rows.inverse(&base.concrete_grades)).filter(|rows| !rows.is_empty()),
            reinforcement_grades: self.reinforcement_grades.as_ref().map(|rows| rows.inverse(&base.reinforcement_grades)).filter(|rows| !rows.is_empty()),
            prestress_steels: self.prestress_steels.as_ref().map(|rows| rows.inverse(&base.prestress_steels)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse(&base.members)).filter(|rows| !rows.is_empty()),
            anchors: self.anchors.as_ref().map(|rows| rows.inverse(&base.anchors)).filter(|rows| !rows.is_empty()),
            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),
            cases: self.cases.inverse(&base.cases),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1992_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1992_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1992_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1992_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves
