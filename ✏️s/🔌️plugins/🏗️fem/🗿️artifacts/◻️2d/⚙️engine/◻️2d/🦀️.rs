//! 📐️ FEM document resolution and solving owned by the two-dimensional artifact.

use crate::fem2d_engine::meshing::build_nodes_and_elements;
use crate::fem2d_engine::meshing::self_weight_nodal_loads;
use crate::fem2d_engine::meshing::translate_loads;
use crate::fem2d_engine::meshing::GRAVITY_G;
use semio_s_fem_model::Support;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use std::collections::HashMap;

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem2dSnapshot`.
#[derive(Clone, Debug, PartialEq)]
pub enum Fem2dError {
    UnknownNodeId(String),
    UnknownMaterialId(String),
    UnknownSectionId(String),
    UnknownRegionId(String),
    MeshFailed { region_id: String, reason: String },
    LoadCaseNotFound(String),
    ModeIndexOutOfRange(usize),
    Fem(semio_s_fem_model::FemError),
    ResultSource(semio_s_fem_analyses_model::ResultSourceIdentityError),
}

impl std::fmt::Display for Fem2dError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownNodeId(id) => write!(formatter, "unknown node id: {id}"),
            Self::UnknownMaterialId(id) => write!(formatter, "unknown material id: {id}"),
            Self::UnknownSectionId(id) => write!(formatter, "unknown section id: {id}"),
            Self::UnknownRegionId(id) => write!(formatter, "unknown region id: {id}"),
            Self::MeshFailed { region_id, reason } => write!(formatter, "region {region_id} failed to mesh: {reason}"),
            Self::LoadCaseNotFound(id) => write!(formatter, "load case not found: {id}"),
            Self::ModeIndexOutOfRange(index) => write!(formatter, "mode index out of range: {index}"),
            Self::Fem(error) => std::fmt::Display::fmt(error, formatter),
            Self::ResultSource(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for Fem2dError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fem(error) => Some(error),
            Self::ResultSource(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_s_fem_model::FemError> for Fem2dError {
    fn from(error: semio_s_fem_model::FemError) -> Self {
        Self::Fem(error)
    }
}
// #endregion 🔖️Errors

// #region 🔖️Solve
/// 🌉️ Resolves a `Fem2dSnapshot` plus a named load case into a `semio_s_fem_static::Model`, erroring
/// descriptively on any dangling material/section/node/region reference.
pub fn build_model(doc: &Fem2dSnapshot, case_id: &str) -> Result<semio_s_fem_static::Model, Fem2dError> {
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();

    let (mut nodal_loads, member_loads) = translate_loads(&load_case.loads, &regions)?;
    if load_case.self_weight {
        nodal_loads.extend(self_weight_nodal_loads(doc, &regions));
    }

    Ok(semio_s_fem_static::Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🌉️ Frozen public entry point: solves a `Fem2dSnapshot`'s named load case for linear-static
/// equilibrium. Signature is a contract consumed directly by the plugin host; do not rename or
/// change it.
pub fn fem2d_solve(doc: &Fem2dSnapshot, case_id: &str) -> Result<semio_s_fem_model::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    semio_s_fem_static::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Richer entry point: resolves EVERY `doc.load_cases`/`doc.combinations` entry at once (regions
/// meshed via the same `build_nodes_and_elements` resolution as `build_model`) and solves them all
/// together via `semio_s_fem_analyses::solve_multi_case` — self-weight honored per-case through
/// `doc.materials`' `rho` (see `self_weight_nodal_loads`'s doc for the `Tri3Cst` caveat), gravity
/// fixed at `[0.0, -9.81, 0.0]`. Canonical typed source identities preserve both namespaces.
pub fn fem2d_solve_all(doc: &Fem2dSnapshot) -> Result<HashMap<String, semio_s_fem_model::StaticResult>, Fem2dError> {
    for (kind, id) in doc.load_cases.iter().map(|case| ("loadCase", case.id.as_str())).chain(doc.combinations.iter().map(|combination| ("combination", combination.id.as_str()))) {
        semio_s_fem_analyses_model::validate_result_source_parts(kind, id).map_err(Fem2dError::ResultSource)?;
    }
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = semio_s_fem_analyses_model::AnalysisModel { nodes, elements, supports };

    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for load_case in &doc.load_cases {
        let (nodal_loads, member_loads) = translate_loads(&load_case.loads, &regions)?;
        cases.push(semio_s_fem_analyses_model::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight });
    }

    let combinations: Vec<semio_s_fem_analyses_model::Combination> = doc.combinations.iter().map(|c| semio_s_fem_analyses_model::Combination { id: c.id.clone(), terms: c.terms.iter().map(|t| (t.case_id.clone(), t.factor)).collect() }).collect();

    semio_s_fem_analyses::solve_multi_case(&model, &cases, &combinations, [0.0, -GRAVITY_G, 0.0]).map_err(Fem2dError::from)?.into_iter().map(|(source, result)| semio_s_fem_analyses_model::encode_result_source_identity(&source).map(|identity| (identity, result)).map_err(Fem2dError::ResultSource)).collect()
}
// #endregion 🔖️Solve

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
