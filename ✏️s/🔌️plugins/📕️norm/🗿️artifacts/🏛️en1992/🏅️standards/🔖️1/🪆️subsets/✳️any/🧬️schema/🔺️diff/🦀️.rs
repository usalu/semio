//! 🧬️ En1992 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

/// 🩹️ Sparse patch of the `concrete_grades` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ConcreteGradesPatch {
    pub f_ck: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::ConcreteGrade> for En1992ConcreteGradesPatch {
    fn commit_into(&self, row: &mut crate::ConcreteGrade, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.f_ck {
            row.f_ck = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.f_ck.is_some() {
            self.f_ck = later.f_ck;
        }
    }

    fn inverse(&self, row: &crate::ConcreteGrade) -> Self {
        Self {
            f_ck: self.f_ck.as_ref().map(|_| row.f_ck.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.f_ck.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `concrete_grades` list (rows keyed by `id`).
    pub En1992ConcreteGradesRows { removal: En1992ConcreteGradesRemoved, insertion: En1992ConcreteGradesInserted, relocation: En1992ConcreteGradesMoved, modification: En1992ConcreteGradesModified, row: crate::ConcreteGrade, patch: En1992ConcreteGradesPatch, key: id }
}

/// 🩹️ Sparse patch of the `reinforcement_grades` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992ReinforcementGradesPatch {
    pub f_yk: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::ReinforcementGrade> for En1992ReinforcementGradesPatch {
    fn commit_into(&self, row: &mut crate::ReinforcementGrade, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.f_yk {
            row.f_yk = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.f_yk.is_some() {
            self.f_yk = later.f_yk;
        }
    }

    fn inverse(&self, row: &crate::ReinforcementGrade) -> Self {
        Self {
            f_yk: self.f_yk.as_ref().map(|_| row.f_yk.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.f_yk.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `reinforcement_grades` list (rows keyed by `id`).
    pub En1992ReinforcementGradesRows { removal: En1992ReinforcementGradesRemoved, insertion: En1992ReinforcementGradesInserted, relocation: En1992ReinforcementGradesMoved, modification: En1992ReinforcementGradesModified, row: crate::ReinforcementGrade, patch: En1992ReinforcementGradesPatch, key: id }
}

/// 🩹️ Sparse patch of the `prestress_steels` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992PrestressSteelsPatch {
    pub name: Option<String>,
    pub f_pk: Option<f64>,
    pub f_p0_1k: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::PrestressSteel> for En1992PrestressSteelsPatch {
    fn commit_into(&self, row: &mut crate::PrestressSteel, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.name {
            row.name = value.clone();
        }
        if let Some(value) = &self.f_pk {
            row.f_pk = value.clone();
        }
        if let Some(value) = &self.f_p0_1k {
            row.f_p0_1k = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.name.is_some() {
            self.name = later.name;
        }
        if later.f_pk.is_some() {
            self.f_pk = later.f_pk;
        }
        if later.f_p0_1k.is_some() {
            self.f_p0_1k = later.f_p0_1k;
        }
    }

    fn inverse(&self, row: &crate::PrestressSteel) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.name.clone()),
            f_pk: self.f_pk.as_ref().map(|_| row.f_pk.clone()),
            f_p0_1k: self.f_p0_1k.as_ref().map(|_| row.f_p0_1k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.name.is_none() && self.f_pk.is_none() && self.f_p0_1k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `prestress_steels` list (rows keyed by `id`).
    pub En1992PrestressSteelsRows { removal: En1992PrestressSteelsRemoved, insertion: En1992PrestressSteelsInserted, relocation: En1992PrestressSteelsMoved, modification: En1992PrestressSteelsModified, row: crate::PrestressSteel, patch: En1992PrestressSteelsPatch, key: id }
}

/// 🩹️ Sparse patch of the `longitudinal` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersLongitudinalPatch {
    pub diameter: Option<f64>,
    pub count: Option<u32>,
}

impl protocol::list_delta::RowPatch<crate::BarLayer> for En1992MembersLongitudinalPatch {
    fn commit_into(&self, row: &mut crate::BarLayer, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.diameter {
            row.diameter = value.clone();
        }
        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.diameter.is_some() {
            self.diameter = later.diameter;
        }
        if later.count.is_some() {
            self.count = later.count;
        }
    }

    fn inverse(&self, row: &crate::BarLayer) -> Self {
        Self {
            diameter: self.diameter.as_ref().map(|_| row.diameter.clone()),
            count: self.count.as_ref().map(|_| row.count.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.diameter.is_none() && self.count.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `longitudinal` list (rows keyed by `id`).
    pub En1992MembersLongitudinalRows { removal: En1992MembersLongitudinalRemoved, insertion: En1992MembersLongitudinalInserted, relocation: En1992MembersLongitudinalMoved, modification: En1992MembersLongitudinalModified, row: crate::BarLayer, patch: En1992MembersLongitudinalPatch, key: id }
}

/// 🩹️ Sparse patch of the `actions` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersActionsPatch {
    pub m_k: Option<f64>,
    pub n_k: Option<f64>,
    pub v_k: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::LoadCaseActions> for En1992MembersActionsPatch {
    fn commit_into(&self, row: &mut crate::LoadCaseActions, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.m_k {
            row.m_k = value.clone();
        }
        if let Some(value) = &self.n_k {
            row.n_k = value.clone();
        }
        if let Some(value) = &self.v_k {
            row.v_k = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.m_k.is_some() {
            self.m_k = later.m_k;
        }
        if later.n_k.is_some() {
            self.n_k = later.n_k;
        }
        if later.v_k.is_some() {
            self.v_k = later.v_k;
        }
    }

    fn inverse(&self, row: &crate::LoadCaseActions) -> Self {
        Self {
            m_k: self.m_k.as_ref().map(|_| row.m_k.clone()),
            n_k: self.n_k.as_ref().map(|_| row.n_k.clone()),
            v_k: self.v_k.as_ref().map(|_| row.v_k.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.m_k.is_none() && self.n_k.is_none() && self.v_k.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `actions` list (rows keyed by `id`).
    pub En1992MembersActionsRows { removal: En1992MembersActionsRemoved, insertion: En1992MembersActionsInserted, relocation: En1992MembersActionsMoved, modification: En1992MembersActionsModified, row: crate::LoadCaseActions, patch: En1992MembersActionsPatch, key: id }
}

/// 🩹️ Sparse patch of the `members` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992MembersPatch {
    pub exposure: Option<crate::ExposureClass>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub effective_depth: Option<f64>,
    pub cover: Option<f64>,
    pub span: Option<f64>,
    pub fire_rating: Option<crate::FireRating>,
    pub fire_axis_distance: Option<f64>,
    pub stirrups_spacing: Option<f64>,
    pub longitudinal: Option<En1992MembersLongitudinalRows>,
    pub actions: Option<En1992MembersActionsRows>,
}

impl protocol::list_delta::RowPatch<crate::RcMember> for En1992MembersPatch {
    fn commit_into(&self, row: &mut crate::RcMember, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.exposure {
            row.exposure = value.clone();
        }
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.height {
            row.height = value.clone();
        }
        if let Some(value) = &self.effective_depth {
            row.effective_depth = value.clone();
        }
        if let Some(value) = &self.cover {
            row.cover = value.clone();
        }
        if let Some(value) = &self.span {
            row.span = value.clone();
        }
        if let Some(value) = &self.fire_rating {
            row.fire.as_mut().ok_or_else(|| missing_target("fire"))?.rating = value.clone();
        }
        if let Some(value) = &self.fire_axis_distance {
            row.fire.as_mut().ok_or_else(|| missing_target("fire"))?.axis_distance = value.clone();
        }
        if let Some(value) = &self.stirrups_spacing {
            row.stirrups.as_mut().ok_or_else(|| missing_target("stirrups"))?.spacing = value.clone();
        }
        if let Some(rows) = &self.longitudinal {
            row.longitudinal = rows.commit_onto(&row.longitudinal, capability).map_err(|error| error.under(["longitudinal"]))?;
        }
        if let Some(rows) = &self.actions {
            row.actions = rows.commit_onto(&row.actions, capability).map_err(|error| error.under(["actions"]))?;
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.exposure.is_some() {
            self.exposure = later.exposure;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.effective_depth.is_some() {
            self.effective_depth = later.effective_depth;
        }
        if later.cover.is_some() {
            self.cover = later.cover;
        }
        if later.span.is_some() {
            self.span = later.span;
        }
        if later.fire_rating.is_some() {
            self.fire_rating = later.fire_rating;
        }
        if later.fire_axis_distance.is_some() {
            self.fire_axis_distance = later.fire_axis_distance;
        }
        if later.stirrups_spacing.is_some() {
            self.stirrups_spacing = later.stirrups_spacing;
        }
        if let Some(theirs) = later.longitudinal {
            match self.longitudinal.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.longitudinal = Some(theirs),
            }
        }
        if let Some(theirs) = later.actions {
            match self.actions.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.actions = Some(theirs),
            }
        }
        self.longitudinal = self.longitudinal.take().filter(|rows| !rows.is_empty());
        self.actions = self.actions.take().filter(|rows| !rows.is_empty());
    }

    fn inverse(&self, row: &crate::RcMember) -> Self {
        Self {
            exposure: self.exposure.as_ref().map(|_| row.exposure.clone()),
            width: self.width.as_ref().map(|_| row.width.clone()),
            height: self.height.as_ref().map(|_| row.height.clone()),
            effective_depth: self.effective_depth.as_ref().map(|_| row.effective_depth.clone()),
            cover: self.cover.as_ref().map(|_| row.cover.clone()),
            span: self.span.as_ref().map(|_| row.span.clone()),
            fire_rating: self.fire_rating.as_ref().and_then(|_| (|| Some(row.fire.as_ref()?.rating.clone()))()),
            fire_axis_distance: self.fire_axis_distance.as_ref().and_then(|_| (|| Some(row.fire.as_ref()?.axis_distance.clone()))()),
            stirrups_spacing: self.stirrups_spacing.as_ref().and_then(|_| (|| Some(row.stirrups.as_ref()?.spacing.clone()))()),
            longitudinal: self.longitudinal.as_ref().map(|rows| rows.inverse(&row.longitudinal)).filter(|rows| !rows.is_empty()),
            actions: self.actions.as_ref().map(|rows| rows.inverse(&row.actions)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.exposure.is_none() && self.width.is_none() && self.height.is_none() && self.effective_depth.is_none() && self.cover.is_none() && self.span.is_none() && self.fire_rating.is_none() && self.fire_axis_distance.is_none() && self.stirrups_spacing.is_none() && self.longitudinal.as_ref().map_or(true, |rows| rows.is_empty()) && self.actions.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `members` list (rows keyed by `id`).
    pub En1992MembersRows { removal: En1992MembersRemoved, insertion: En1992MembersInserted, relocation: En1992MembersMoved, modification: En1992MembersModified, row: crate::RcMember, patch: En1992MembersPatch, key: id }
}

/// 🩹️ Sparse patch of the `anchors` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1992AnchorsPatch {
    pub h_ef: Option<f64>,
    pub a_s: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::Anchor> for En1992AnchorsPatch {
    fn commit_into(&self, row: &mut crate::Anchor, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.h_ef {
            row.h_ef = value.clone();
        }
        if let Some(value) = &self.a_s {
            row.a_s = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.h_ef.is_some() {
            self.h_ef = later.h_ef;
        }
        if later.a_s.is_some() {
            self.a_s = later.a_s;
        }
    }

    fn inverse(&self, row: &crate::Anchor) -> Self {
        Self {
            h_ef: self.h_ef.as_ref().map(|_| row.h_ef.clone()),
            a_s: self.a_s.as_ref().map(|_| row.a_s.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.h_ef.is_none() && self.a_s.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `anchors` list (rows keyed by `id`).
    pub En1992AnchorsRows { removal: En1992AnchorsRemoved, insertion: En1992AnchorsInserted, relocation: En1992AnchorsMoved, modification: En1992AnchorsModified, row: crate::Anchor, patch: En1992AnchorsPatch, key: id }
}

/// 🔺️ Keyed sparse diff of the En1992 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub delta_c_dev: Option<f64>,
    #[state(artifact)]
    pub cement_type: Option<String>,
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
}

impl protocol::MutationDiff<En1992Snapshot> for En1992Diff {
    fn apply(&self, base: &En1992Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1992Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
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
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
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
    }
}

impl protocol::DiffAlgebra<En1992Snapshot> for En1992Diff {
    fn inverse(&self, base: &En1992Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            design_working_life_years: self.design_working_life_years.as_ref().map(|_| base.design_working_life_years.clone()),
            delta_c_dev: self.delta_c_dev.as_ref().map(|_| base.delta_c_dev.clone()),
            cement_type: self.cement_type.as_ref().map(|_| base.cement_type.clone()),
            concrete_grades: self.concrete_grades.as_ref().map(|rows| rows.inverse(&base.concrete_grades)).filter(|rows| !rows.is_empty()),
            reinforcement_grades: self.reinforcement_grades.as_ref().map(|rows| rows.inverse(&base.reinforcement_grades)).filter(|rows| !rows.is_empty()),
            prestress_steels: self.prestress_steels.as_ref().map(|rows| rows.inverse(&base.prestress_steels)).filter(|rows| !rows.is_empty()),
            members: self.members.as_ref().map(|rows| rows.inverse(&base.members)).filter(|rows| !rows.is_empty()),
            anchors: self.anchors.as_ref().map(|rows| rows.inverse(&base.anchors)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.title.is_none() && self.design_working_life_years.is_none() && self.delta_c_dev.is_none() && self.cement_type.is_none() && self.concrete_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.reinforcement_grades.as_ref().map_or(true, |rows| rows.is_empty()) && self.prestress_steels.as_ref().map_or(true, |rows| rows.is_empty()) && self.members.as_ref().map_or(true, |rows| rows.is_empty()) && self.anchors.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
