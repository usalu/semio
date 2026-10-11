//! 🧬️ En1997 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

use crate::En1997Snapshot;

/// 🩹️ Sparse patch of the `layers` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997LayersPatch {
    pub oedometric_modulus: Option<f64>,
    pub phi_prime_deg: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::SoilLayer> for En1997LayersPatch {
    fn commit_into(&self, row: &mut crate::SoilLayer, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.oedometric_modulus {
            row.oedometric_modulus = value.clone();
        }
        if let Some(value) = &self.phi_prime_deg {
            row.phi_prime_deg = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.oedometric_modulus.is_some() {
            self.oedometric_modulus = later.oedometric_modulus;
        }
        if later.phi_prime_deg.is_some() {
            self.phi_prime_deg = later.phi_prime_deg;
        }
    }

    fn inverse(&self, row: &crate::SoilLayer) -> Self {
        Self {
            oedometric_modulus: self.oedometric_modulus.as_ref().map(|_| row.oedometric_modulus.clone()),
            phi_prime_deg: self.phi_prime_deg.as_ref().map(|_| row.phi_prime_deg.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.oedometric_modulus.is_none() && self.phi_prime_deg.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `layers` list (rows keyed by `id`).
    pub En1997LayersRows { removal: En1997LayersRemoved, insertion: En1997LayersInserted, relocation: En1997LayersMoved, modification: En1997LayersModified, row: crate::SoilLayer, patch: En1997LayersPatch, key: id }
}

/// 🩹️ Sparse patch of the `footings` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997FootingsPatch {
    pub width: Option<f64>,
    pub embedment: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::SpreadFoundation> for En1997FootingsPatch {
    fn commit_into(&self, row: &mut crate::SpreadFoundation, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.width {
            row.width = value.clone();
        }
        if let Some(value) = &self.embedment {
            row.embedment = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.embedment.is_some() {
            self.embedment = later.embedment;
        }
    }

    fn inverse(&self, row: &crate::SpreadFoundation) -> Self {
        Self {
            width: self.width.as_ref().map(|_| row.width.clone()),
            embedment: self.embedment.as_ref().map(|_| row.embedment.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.width.is_none() && self.embedment.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `footings` list (rows keyed by `id`).
    pub En1997FootingsRows { removal: En1997FootingsRemoved, insertion: En1997FootingsInserted, relocation: En1997FootingsMoved, modification: En1997FootingsModified, row: crate::SpreadFoundation, patch: En1997FootingsPatch, key: id }
}

/// 🩹️ Sparse patch of the `piles` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997PilesPatch {
    pub length: Option<f64>,
    pub count: Option<u32>,
}

impl protocol::list_delta::RowPatch<crate::Pile> for En1997PilesPatch {
    fn commit_into(&self, row: &mut crate::Pile, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.length {
            row.length = value.clone();
        }
        if let Some(value) = &self.count {
            row.count = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.length.is_some() {
            self.length = later.length;
        }
        if later.count.is_some() {
            self.count = later.count;
        }
    }

    fn inverse(&self, row: &crate::Pile) -> Self {
        Self {
            length: self.length.as_ref().map(|_| row.length.clone()),
            count: self.count.as_ref().map(|_| row.count.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.length.is_none() && self.count.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `piles` list (rows keyed by `id`).
    pub En1997PilesRows { removal: En1997PilesRemoved, insertion: En1997PilesInserted, relocation: En1997PilesMoved, modification: En1997PilesModified, row: crate::Pile, patch: En1997PilesPatch, key: id }
}

/// 🩹️ Sparse patch of the `retaining_walls` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997RetainingWallsPatch {
    pub base_width: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::RetainingWall> for En1997RetainingWallsPatch {
    fn commit_into(&self, row: &mut crate::RetainingWall, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.base_width {
            row.base_width = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.base_width.is_some() {
            self.base_width = later.base_width;
        }
    }

    fn inverse(&self, row: &crate::RetainingWall) -> Self {
        Self {
            base_width: self.base_width.as_ref().map(|_| row.base_width.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.base_width.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `retaining_walls` list (rows keyed by `id`).
    pub En1997RetainingWallsRows { removal: En1997RetainingWallsRemoved, insertion: En1997RetainingWallsInserted, relocation: En1997RetainingWallsMoved, modification: En1997RetainingWallsModified, row: crate::RetainingWall, patch: En1997RetainingWallsPatch, key: id }
}

/// 🩹️ Sparse patch of the `slopes` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997SlopesPatch {
    pub angle_deg: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::Slope> for En1997SlopesPatch {
    fn commit_into(&self, row: &mut crate::Slope, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.angle_deg {
            row.angle_deg = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.angle_deg.is_some() {
            self.angle_deg = later.angle_deg;
        }
    }

    fn inverse(&self, row: &crate::Slope) -> Self {
        Self {
            angle_deg: self.angle_deg.as_ref().map(|_| row.angle_deg.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.angle_deg.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `slopes` list (rows keyed by `id`).
    pub En1997SlopesRows { removal: En1997SlopesRemoved, insertion: En1997SlopesInserted, relocation: En1997SlopesMoved, modification: En1997SlopesModified, row: crate::Slope, patch: En1997SlopesPatch, key: id }
}

/// 🩹️ Sparse patch of the `uplift_cases` row addressed by its key.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1997UpliftCasesPatch {
    pub permanent_stabilizing: Option<f64>,
    pub permanent_destabilizing: Option<f64>,
    pub variable_destabilizing: Option<f64>,
    pub pore_pressure: Option<f64>,
    pub total_stress: Option<f64>,
}

impl protocol::list_delta::RowPatch<crate::UpliftCase> for En1997UpliftCasesPatch {
    fn commit_into(&self, row: &mut crate::UpliftCase, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &self.permanent_stabilizing {
            row.permanent_stabilizing = value.clone();
        }
        if let Some(value) = &self.permanent_destabilizing {
            row.permanent_destabilizing = value.clone();
        }
        if let Some(value) = &self.variable_destabilizing {
            row.variable_destabilizing = value.clone();
        }
        if let Some(value) = &self.pore_pressure {
            row.pore_pressure = value.clone();
        }
        if let Some(value) = &self.total_stress {
            row.total_stress = value.clone();
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        if later.permanent_stabilizing.is_some() {
            self.permanent_stabilizing = later.permanent_stabilizing;
        }
        if later.permanent_destabilizing.is_some() {
            self.permanent_destabilizing = later.permanent_destabilizing;
        }
        if later.variable_destabilizing.is_some() {
            self.variable_destabilizing = later.variable_destabilizing;
        }
        if later.pore_pressure.is_some() {
            self.pore_pressure = later.pore_pressure;
        }
        if later.total_stress.is_some() {
            self.total_stress = later.total_stress;
        }
    }

    fn inverse(&self, row: &crate::UpliftCase) -> Self {
        Self {
            permanent_stabilizing: self.permanent_stabilizing.as_ref().map(|_| row.permanent_stabilizing.clone()),
            permanent_destabilizing: self.permanent_destabilizing.as_ref().map(|_| row.permanent_destabilizing.clone()),
            variable_destabilizing: self.variable_destabilizing.as_ref().map(|_| row.variable_destabilizing.clone()),
            pore_pressure: self.pore_pressure.as_ref().map(|_| row.pore_pressure.clone()),
            total_stress: self.total_stress.as_ref().map(|_| row.total_stress.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.permanent_stabilizing.is_none() && self.permanent_destabilizing.is_none() && self.variable_destabilizing.is_none() && self.pore_pressure.is_none() && self.total_stress.is_none()
    }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of the `uplift_cases` list (rows keyed by `id`).
    pub En1997UpliftCasesRows { removal: En1997UpliftCasesRemoved, insertion: En1997UpliftCasesInserted, relocation: En1997UpliftCasesMoved, modification: En1997UpliftCasesModified, row: crate::UpliftCase, patch: En1997UpliftCasesPatch, key: id }
}

/// 🔺️ Keyed sparse diff of the En1997 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Diff {
    #[state(artifact)]
    pub structure_id: Option<String>,
    #[state(artifact)]
    pub geotechnical_category: Option<u8>,
    #[state(artifact)]
    pub design_situation: Option<String>,
    #[state(artifact)]
    pub design_approach: Option<String>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub groundwater_level: Option<f64>,
    #[state(artifact)]
    pub investigation_depth: Option<f64>,
    #[state(artifact)]
    pub layers: Option<En1997LayersRows>,
    #[state(artifact)]
    pub footings: Option<En1997FootingsRows>,
    #[state(artifact)]
    pub piles: Option<En1997PilesRows>,
    #[state(artifact)]
    pub retaining_walls: Option<En1997RetainingWallsRows>,
    #[state(artifact)]
    pub slopes: Option<En1997SlopesRows>,
    #[state(artifact)]
    pub uplift_cases: Option<En1997UpliftCasesRows>,
}

impl protocol::MutationDiff<En1997Snapshot> for En1997Diff {
    fn apply(&self, base: &En1997Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1997Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.structure_id {
            next.structure_id = value.clone();
        }
        if let Some(value) = &self.geotechnical_category {
            next.geotechnical_category = value.clone();
        }
        if let Some(value) = &self.design_situation {
            next.design_situation = value.clone();
        }
        if let Some(value) = &self.design_approach {
            next.design_approach = value.clone();
        }
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.groundwater_level {
            next.groundwater_level = value.clone();
        }
        if let Some(value) = &self.investigation_depth {
            next.investigation_depth = value.clone();
        }
        if let Some(rows) = &self.layers {
            next.layers = rows.commit_onto(&base.layers, capability).map_err(|error| error.under(["layers"]))?;
        }
        if let Some(rows) = &self.footings {
            next.footings = rows.commit_onto(&base.footings, capability).map_err(|error| error.under(["footings"]))?;
        }
        if let Some(rows) = &self.piles {
            next.piles = rows.commit_onto(&base.piles, capability).map_err(|error| error.under(["piles"]))?;
        }
        if let Some(rows) = &self.retaining_walls {
            next.retaining_walls = rows.commit_onto(&base.retaining_walls, capability).map_err(|error| error.under(["retaining_walls"]))?;
        }
        if let Some(rows) = &self.slopes {
            next.slopes = rows.commit_onto(&base.slopes, capability).map_err(|error| error.under(["slopes"]))?;
        }
        if let Some(rows) = &self.uplift_cases {
            next.uplift_cases = rows.commit_onto(&base.uplift_cases, capability).map_err(|error| error.under(["uplift_cases"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.structure_id.is_some() {
            self.structure_id = other.structure_id;
        }
        if other.geotechnical_category.is_some() {
            self.geotechnical_category = other.geotechnical_category;
        }
        if other.design_situation.is_some() {
            self.design_situation = other.design_situation;
        }
        if other.design_approach.is_some() {
            self.design_approach = other.design_approach;
        }
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.groundwater_level.is_some() {
            self.groundwater_level = other.groundwater_level;
        }
        if other.investigation_depth.is_some() {
            self.investigation_depth = other.investigation_depth;
        }
        if let Some(theirs) = other.layers {
            match self.layers.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.layers = Some(theirs),
            }
            self.layers = self.layers.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.footings {
            match self.footings.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.footings = Some(theirs),
            }
            self.footings = self.footings.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.piles {
            match self.piles.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.piles = Some(theirs),
            }
            self.piles = self.piles.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.retaining_walls {
            match self.retaining_walls.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.retaining_walls = Some(theirs),
            }
            self.retaining_walls = self.retaining_walls.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.slopes {
            match self.slopes.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.slopes = Some(theirs),
            }
            self.slopes = self.slopes.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.uplift_cases {
            match self.uplift_cases.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.uplift_cases = Some(theirs),
            }
            self.uplift_cases = self.uplift_cases.take().filter(|rows| !rows.is_empty());
        }
    }
}

impl protocol::DiffAlgebra<En1997Snapshot> for En1997Diff {
    fn inverse(&self, base: &En1997Snapshot) -> Self {
        Self {
            structure_id: self.structure_id.as_ref().map(|_| base.structure_id.clone()),
            geotechnical_category: self.geotechnical_category.as_ref().map(|_| base.geotechnical_category.clone()),
            design_situation: self.design_situation.as_ref().map(|_| base.design_situation.clone()),
            design_approach: self.design_approach.as_ref().map(|_| base.design_approach.clone()),
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            groundwater_level: self.groundwater_level.as_ref().map(|_| base.groundwater_level.clone()),
            investigation_depth: self.investigation_depth.as_ref().map(|_| base.investigation_depth.clone()),
            layers: self.layers.as_ref().map(|rows| rows.inverse(&base.layers)).filter(|rows| !rows.is_empty()),
            footings: self.footings.as_ref().map(|rows| rows.inverse(&base.footings)).filter(|rows| !rows.is_empty()),
            piles: self.piles.as_ref().map(|rows| rows.inverse(&base.piles)).filter(|rows| !rows.is_empty()),
            retaining_walls: self.retaining_walls.as_ref().map(|rows| rows.inverse(&base.retaining_walls)).filter(|rows| !rows.is_empty()),
            slopes: self.slopes.as_ref().map(|rows| rows.inverse(&base.slopes)).filter(|rows| !rows.is_empty()),
            uplift_cases: self.uplift_cases.as_ref().map(|rows| rows.inverse(&base.uplift_cases)).filter(|rows| !rows.is_empty()),
        }
    }

    fn is_empty(&self) -> bool {
        self.structure_id.is_none() && self.geotechnical_category.is_none() && self.design_situation.is_none() && self.design_approach.is_none() && self.annex.is_none() && self.groundwater_level.is_none() && self.investigation_depth.is_none() && self.layers.as_ref().map_or(true, |rows| rows.is_empty()) && self.footings.as_ref().map_or(true, |rows| rows.is_empty()) && self.piles.as_ref().map_or(true, |rows| rows.is_empty()) && self.retaining_walls.as_ref().map_or(true, |rows| rows.is_empty()) && self.slopes.as_ref().map_or(true, |rows| rows.is_empty()) && self.uplift_cases.as_ref().map_or(true, |rows| rows.is_empty())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
