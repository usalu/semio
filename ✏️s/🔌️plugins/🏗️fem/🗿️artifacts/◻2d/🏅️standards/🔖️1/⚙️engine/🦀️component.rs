//! ⚙️ FEM 2D artifact — headless compute (constitutional: engine). Sibling files
//! `🕸️meshing`/`🎵️modal-buckling`/`🗺️mesh-preview` hold the region-meshing, modal/buckling and
//! mesh-preview bridges respectively; this file keeps the `Errors` region, the `Io` region, and the
//! top-level `fem2d_io()`/solve entry points that aren't specific to any of those three.

use crate::artifacts::fem2d::engine::meshing::{area_load_nodal_loads, build_nodes_and_elements, self_weight_nodal_loads, GRAVITY_G};
use crate::artifacts::fem2d::{Fem2dSnapshot, FemLoad};
use crate::model::{MemberUdl, NodalLoad, Support};
use std::collections::HashMap;

// #region 🔖️Register
/// 🗂️ Registers `Fem2dSnapshot`'s pack↔dsl codec under `FEM_2D_SCHEMA` so `framework/sync`'s
/// `FolderEndpoint` (and any other schema-string-keyed caller) can print/parse fem2d documents without
/// depending on its concrete `Projection`/`Mutation` types. Reached from the plugin root's
/// `semio_plugin!{ setup: … }` via `crate::model::register_all_engines`.
pub fn register() {
    crate::artifacts::fem2d::io_registry::register();

    register_pilot_languages();
    register_artifact_schema();
    register_artifact_inferences();
    crate::apps::fem2d::config::schema::register_app_schema();

    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::fem2d::Fem2dPlayApp>(crate::artifacts::fem2d::FEM_2D_SCHEMA);
}



/// 📎 Registers the fem2d artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::fem2d::schema::fem2d_artifact_schema_descriptor());
}

/// 💡️ Registers the fem2d artifact 💡️inference schema descriptor into the OS-wide inference
/// catalog — sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(
        crate::artifacts::fem2d::standards::v1::subsets::any::schema::inferences::fem2d_artifact_inference_descriptor(),
    );
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d",
        extension: Some("fem2d"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("fem.fem2d"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("fem.fem2d.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem.fem2d.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::fem2d::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("fem.fem2d.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem2d.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("fem2d.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "fem2d.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::fem2d::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("fem2d.spr"),
    });
}


// #endregion 🔖️Register

pub fn empty_fem2d_snapshot() -> Fem2dSnapshot {
    Fem2dSnapshot::default()
}

// #region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.2d` × 2D-Vector) plus `geometry:in` (importing an externally authored 2D outline as a new
/// `FemRegion` — see `crate::apps::fem2d::import_media`) and `results:out` (every load case/combination's
/// solved `crate::model::StaticResult`, pinned to the `computation.fem2d` artifact kind declared in
/// `crate::artifacts::fem2d::computation_artifact_kind` — see `crate::apps::fem2d::export_media`).
pub fn fem2d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::fem2d::FEM_2D_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        ports: vec![fem2d_geometry_in_port(), fem2d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.fem".into(), name: "FEM 2D".into(), dimension: "2d".into(), component_kind: "fem2d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored 2D polygon-with-holes outline, imported as a new
/// `FemRegion`.
pub fn fem2d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem2d` artifact kind.
pub fn fem2d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        kind_id: Some("computation.fem2d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
// #endregion 🔖️Io

// #region 🔖️Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem2dSnapshot`.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Fem2dError {
    #[error("unknown node id: {0}")]
    UnknownNodeId(String),
    #[error("unknown material id: {0}")]
    UnknownMaterialId(String),
    #[error("unknown section id: {0}")]
    UnknownSectionId(String),
    #[error("unknown region id: {0}")]
    UnknownRegionId(String),
    #[error("region {region_id} failed to mesh: {reason}")]
    MeshFailed { region_id: String, reason: String },
    #[error("load case not found: {0}")]
    LoadCaseNotFound(String),
    #[error("mode index out of range: {0}")]
    ModeIndexOutOfRange(usize),
    #[error(transparent)]
    Fem(#[from] crate::model::FemError),
}
// #endregion 🔖️Errors

// #region 🔖️Solve
/// 🌉️ Resolves a `Fem2dSnapshot` plus a named load case into a `crate::model::Model`, erroring
/// descriptively on any dangling material/section/node/region reference.
pub fn build_model(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::model::Model, Fem2dError> {
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }));
            }
            FemLoad::Area { region_id, pressure, .. } => {
                let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads(region, *pressure));
            }
        }
    }
    if load_case.self_weight {
        nodal_loads.extend(self_weight_nodal_loads(doc, &regions));
    }

    Ok(crate::model::Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🌉️ Frozen public entry point: solves a `Fem2dSnapshot`'s named load case for linear-static
/// equilibrium. Signature is a contract consumed directly by the plugin host; do not rename or
/// change it.
pub fn fem2d_solve(doc: &Fem2dSnapshot, case_id: &str) -> Result<crate::model::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    crate::model::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉️ Richer entry point: resolves EVERY `doc.load_cases`/`doc.combinations` entry at once (regions
/// meshed via the same `build_nodes_and_elements` resolution as `build_model`) and solves them all
/// together via `crate::analyses::solve_multi_case` — self-weight honored per-case through
/// `doc.materials`' `rho` (see `self_weight_nodal_loads`'s doc for the `Tri3Cst` caveat), gravity
/// fixed at `[0.0, -9.81, 0.0]`. Returns results keyed by case id ∪ combination id.
pub fn fem2d_solve_all(doc: &Fem2dSnapshot) -> Result<HashMap<String, crate::model::StaticResult>, Fem2dError> {
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.iter().map(|d| (*d).into()).collect() }).collect();
    let model = crate::analyses::AnalysisModel { nodes, elements, supports };

    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for load_case in &doc.load_cases {
        let mut nodal_loads = Vec::new();
        let mut member_loads = Vec::new();
        for load in &load_case.loads {
            match load {
                FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: (*dof).into(), value: *value }),
                FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                    member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }));
                }
                FemLoad::Area { region_id, pressure, .. } => {
                    let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                    nodal_loads.extend(area_load_nodal_loads(region, *pressure));
                }
            }
        }
        cases.push(crate::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight });
    }

    let combinations: Vec<crate::analyses::Combination> = doc.combinations.iter().map(|c| crate::analyses::Combination { id: c.id.clone(), terms: c.terms.iter().map(|t| (t.case_id.clone(), t.factor)).collect() }).collect();

    crate::analyses::solve_multi_case(&model, &cases, &combinations, [0.0, -GRAVITY_G, 0.0]).map_err(Fem2dError::from)
}
// #endregion 🔖️Solve

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::fem2d::{FemAnalysisSettings, FemCombination, FemCombinationTerm, FemDof, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
    use crate::model::{Dof, ElementResult};

    // #region 🔖️Io
    /// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
    /// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
    /// enforcement is exercised at the run-crate level.
    #[test]
    fn fem2d_io_declares_geometry_in_and_results_out_ports() {
        let io = fem2d_io();
        assert_eq!(io.document_schema, crate::artifacts::fem2d::FEM_2D_SCHEMA);
        assert_eq!(io.document_media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(io.document_media_type.form, semio_framework_plugin::MediaForm::Vector);
        assert_eq!(io.artifact.id, "2d.fem");
        assert_eq!(io.artifact.component_kind, "fem2d");

        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.required, "geometry:in is a required input port");
        assert_eq!(geometry_in.media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(geometry_in.media_type.form, semio_framework_plugin::MediaForm::Vector);
        assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

        let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
        assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert!(!results_out.required, "results:out is optional");
        assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem2d"));
        assert_eq!(results_out.media_type.class, semio_framework_plugin::MediaClass::Data);
        assert_eq!(results_out.media_type.form, semio_framework_plugin::MediaForm::Value);
    }
    // #endregion 🔖️Io

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn simply_supported_beam_two_span_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 3.0, y: 0.0 }, FemNode { id: "n3".into(), x: 6.0, y: 0.0 }],
            elements: vec![
                FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() },
                FemElement::Beam { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "ipe300".into() },
            ],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n3".into(), fixed: vec![FemDof::Ty] }],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }, FemLoad::MemberUdl { id: "l2".into(), element_id: "e2".into(), wx: 0.0, wy: -10000.0 }],
                self_weight: false,
            }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn truss_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 4.0, y: 0.0 }, FemNode { id: "n3".into(), x: 4.0, y: 3.0 }],
            elements: vec![
                FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "rod".into(), area: 0.001, iy: 0.0 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: FemDof::Ty, value: -1000.0 }, FemLoad::Nodal { id: "l2".into(), node_id: "n3".into(), dof: FemDof::Tx, value: -500.0 }],
                self_weight: false,
            }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🟩️ A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes (so `build_nodes_and_elements`'s exact-position reuse binds the mesh boundary
    /// to them) — 2 adjacent corners fully pinned, enough to remove all 3 in-plane rigid-body modes.
    fn rectangle_region_doc() -> Fem2dSnapshot {
        Fem2dSnapshot {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![FemDof::Tx, FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    /// 🕳️ Same rectangle as `rectangle_region_doc` but with a small square hole near the center.
    fn rectangle_with_hole_region_doc() -> Fem2dSnapshot {
        let mut doc = rectangle_region_doc();
        doc.regions[0].holes = vec![vec![[1.5, 0.75], [2.5, 0.75], [2.5, 1.25], [1.5, 1.25]]];
        doc
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️BuildModel
    #[test]
    fn build_model_reports_dangling_material() {
        let mut doc = simply_supported_beam_doc();
        doc.materials.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.to_string().contains("material"), "unexpected error: {err}");
    }

    #[test]
    fn build_model_reports_dangling_section() {
        let mut doc = simply_supported_beam_doc();
        doc.sections.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.to_string().contains("section"), "unexpected error: {err}");
    }

    #[test]
    fn build_model_reports_dangling_node() {
        let mut doc = simply_supported_beam_doc();
        doc.nodes.clear();
        let err = build_model(&doc, "dead").unwrap_err();
        assert!(err.to_string().contains("node"), "unexpected error: {err}");
    }
    // #endregion 🔖️BuildModel

    // #region 🔖️Regions
    #[test]
    fn build_model_meshes_region_and_solves() {
        let doc = rectangle_region_doc();
        let result = fem2d_solve(&doc, "self").expect("region solves");
        assert!(result.checks.residual_norm < 1e-6, "residual {}", result.checks.residual_norm);
    }

    #[test]
    fn region_with_hole_meshes_and_solves() {
        let doc = rectangle_with_hole_region_doc();
        let result = fem2d_solve(&doc, "self").expect("region with hole solves");
        assert!(result.checks.residual_norm < 1e-6, "residual {}", result.checks.residual_norm);
    }

    #[test]
    fn area_load_on_region_produces_reactions() {
        let mut doc = rectangle_region_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), region_id: "r1".into(), pressure: 5000.0 }], self_weight: false }];
        let result = fem2d_solve(&doc, "pressure").expect("area load solves");
        assert!(result.checks.residual_norm < 1e-6, "residual {}", result.checks.residual_norm);

        let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
        let region_area = 4.0 * 2.0;
        let expected = 5000.0 * region_area;
        assert!((total_ty_reaction - expected).abs() / expected < 0.02, "reaction sum {total_ty_reaction} vs expected {expected}");
    }
    // #endregion 🔖️Regions

    // #region 🔖️SelfWeight
    #[test]
    fn self_weight_case_produces_nonzero_reactions() {
        let mut doc = simply_supported_beam_doc();
        doc.load_cases = vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }];
        let result = fem2d_solve(&doc, "self").expect("self-weight solves");

        let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
        let (area, length) = (0.005381, 6.0);
        let expected = 7850.0 * area * length * 9.81;
        assert!(total_ty_reaction.abs() > 1e-3, "expected nonzero reactions from self-weight, got {total_ty_reaction}");
        assert!((total_ty_reaction - expected).abs() / expected < 0.01, "reaction sum {total_ty_reaction} vs expected {expected}");
    }

    /// ⚖️ Region self-weight through the NATIVE `fem2d_solve_all` path (now possible since `Tri3Cst`
    /// gained `mass()`): total vertical reaction must equal `ρ·thickness·Area·g` — exercised on the SAME
    /// `rectangle_region_doc` fixture `build_model_meshes_region_and_solves` only checks for a small
    /// residual on, now checked for the exact expected total instead. This also guards against
    /// double-counting: `fem2d_solve_all` must NOT also apply the lumped `self_weight_nodal_loads`
    /// translation (that helper is exclusive to `build_model`/`fem2d_solve` — see its doc comment).
    #[test]
    fn region_self_weight_via_solve_all_matches_total_mass_times_gravity() {
        let doc = rectangle_region_doc();
        let results = fem2d_solve_all(&doc).expect("region self-weight solves via solve_all");
        let result = results.get("self").unwrap();
        let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
        let (thickness, area, rho) = (0.02, 4.0 * 2.0, 7850.0);
        let expected = rho * thickness * area * GRAVITY_G;
        assert!((total_ty_reaction - expected).abs() / expected < 0.01, "reaction sum {total_ty_reaction} vs expected {expected}");
    }
    // #endregion 🔖️SelfWeight

    // #region 🔖️SolveAll
    #[test]
    fn fem2d_solve_all_returns_case_and_combination_results() {
        let mut doc = simply_supported_beam_doc();
        doc.load_cases.push(FemLoadCase { id: "live".into(), name: "live".into(), loads: vec![FemLoad::Nodal { id: "l2".into(), node_id: "n1".into(), dof: FemDof::Ty, value: -2000.0 }], self_weight: false });
        doc.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] });

        let results = fem2d_solve_all(&doc).expect("solves all");
        assert_eq!(results.len(), 3, "expected 2 cases + 1 combination, got keys {:?}", results.keys().collect::<Vec<_>>());
        assert!(results.contains_key("dead"));
        assert!(results.contains_key("live"));
        assert!(results.contains_key("uls"));

        let dead = results.get("dead").unwrap().clone();
        let live = results.get("live").unwrap().clone();
        let combo = results.get("uls").unwrap();

        for cd in &combo.displacements {
            let dd = dead.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            let ld = live.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * dd.values[k] + 1.5 * ld.values[k];
                assert!((cd.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}", cd.node_id);
            }
        }
    }
    // #endregion 🔖️SolveAll

    // #region 🔖️AnalyticalBenchmark
    #[test]
    fn simply_supported_beam_matches_analytical_udl_solution() {
        let doc = simply_supported_beam_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Tx) {
            assert!(reaction.value.abs() < 1e-6, "horizontal reaction {reaction:?} should be ~0 (no horizontal load)");
        }
        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Ty) {
            assert!((reaction.value - 30000.0).abs() < 1.0, "vertical reaction {reaction:?} not near 30000N");
        }

        let (_, ElementResult::Beam { stations }) = &result.elements[0] else { panic!("expected beam result") };
        let midspan = stations.iter().min_by(|a, b| (a.x - 3.0).abs().partial_cmp(&(b.x - 3.0).abs()).unwrap()).unwrap();
        assert!((midspan.m - 45000.0).abs() / 45000.0 < 0.01, "midspan moment {} not near 45000", midspan.m);
    }

    #[test]
    fn two_span_beam_matches_analytical_midspan_deflection_and_moment() {
        let doc = simply_supported_beam_two_span_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Tx) {
            assert!(reaction.value.abs() < 1e-6, "horizontal reaction {reaction:?} should be ~0 (no horizontal load)");
        }
        for reaction in result.reactions.iter().filter(|r| r.dof == Dof::Ty) {
            assert!((reaction.value - 30000.0).abs() < 1.0, "vertical reaction {reaction:?} not near 30000N");
        }

        let midspan_disp = result.displacements.iter().find(|d| d.node_id == "n2").unwrap();
        let expected = -0.009617;
        assert!((midspan_disp.values[Dof::Ty.index()] - expected).abs() / expected.abs() < 0.02, "midspan deflection {} not near {expected}", midspan_disp.values[Dof::Ty.index()]);

        let (_, ElementResult::Beam { stations }) = &result.elements[0] else { panic!("expected beam result") };
        let end_moment = stations.last().unwrap();
        assert!((end_moment.m - 45000.0).abs() / 45000.0 < 0.01, "end moment at midspan node {} not near 45000", end_moment.m);
    }
    // #endregion 🔖️AnalyticalBenchmark

    // #region 🔖️Truss
    #[test]
    fn truss_is_in_equilibrium_with_finite_bar_forces() {
        let doc = truss_doc();
        let result = fem2d_solve(&doc, "dead").expect("solves");

        assert!(result.checks.reaction_sum[Dof::Tx.index()].abs() < 1e-6);
        assert!((result.checks.reaction_sum[Dof::Ty.index()]).abs() < 1e-6);

        for (_, element_result) in &result.elements {
            let ElementResult::Bar { n } = element_result else { panic!("expected bar result") };
            assert!(n.is_finite() && *n != 0.0, "bar force {n} should be finite and nonzero");
        }
    }
    // #endregion 🔖️Truss

    // #region 🔖️UnknownCase
    #[test]
    fn unknown_load_case_returns_descriptive_error() {
        let doc = simply_supported_beam_doc();
        let err = fem2d_solve(&doc, "missing").unwrap_err();
        assert!(err.contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖️UnknownCase

    // #region 🔖️ExampleFixture
    #[test]
    fn example_fixture_parses_and_solves() {
        use store::ArtifactDsl;
        let doc: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).expect("example fixture parses");
        assert_eq!(doc.nodes.len(), 12);
        assert_eq!(doc.elements.len(), 9);
        assert_eq!(doc.regions.len(), 1);
        assert_eq!(doc.combinations.len(), 1);

        let result = fem2d_solve(&doc, "dead").expect("example fixture solves");
        assert!(result.checks.residual_norm < 1e-6);

        let results = fem2d_solve_all(&doc).expect("example fixture solves all");
        assert!(results.contains_key("dead"), "missing dead case result");
        assert!(results.contains_key("live"), "missing live case result");
        assert!(results.contains_key("uls"), "missing uls combination result");
        assert!(results.get("dead").unwrap().checks.residual_norm < 1e-6);

        let nodal_von_mises = crate::artifacts::fem2d::engine::mesh_preview::fem2d_nodal_von_mises(&doc, "dead").expect("nodal von mises resolves");
        assert!(!nodal_von_mises.is_empty(), "the region carries a real area load in the dead case");

        let buckling = crate::artifacts::fem2d::engine::modal_buckling::fem2d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
        assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
    }
    // #endregion 🔖️ExampleFixture
}
// #endregion 🧪️Tests


//#region 🔖️ArtifactEngine
pub struct Fem2dEngine {
    artifact: crate::artifacts::fem2d::schema::Fem2dArtifact,
    snapshot: crate::artifacts::fem2d::Fem2dSnapshot,
}

impl Fem2dEngine {
    pub fn new(snapshot: crate::artifacts::fem2d::Fem2dSnapshot) -> Self {
        Self {
            artifact: crate::artifacts::fem2d::schema::Fem2dArtifact::from_snapshot(snapshot.clone()),
            snapshot,
        }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::Fem2dComposer as Fem2dAnyComposer;
    use crate::artifacts::fem2d::standards::v1::subsets::any::schema::Fem2dBuilder as Fem2dAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const FEM2D_DIALECT: Dialect = Dialect { artifact_kind: "s.fem2d", standard: StandardId("1"), subset: SubsetId("*") };
    const FEM2D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::fem2d::Fem2dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == FEM2D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Fem2dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Fem2dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Fem2dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == FEM2D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::fem2d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Fem2dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    fn compose_export_csv(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::fem2d::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    fn compose_export_md(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::fem2d::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::fem2d::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    /// 🌉️ `stl`/`obj` below are real geometry (bridged through the semio mesh subset — see
    /// `engine::meshing::build_semio_mesh_snapshot` — never hand-rolled bytes). `zip`/`png` export
    /// entries were REMOVED outright (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-
    /// MEDIA-FORMAT-RETIREMENT W5a): fem2d has no real archive-bundle or raster-visualization
    /// capability to honestly back a `.zip`/`.png` export — their old leaves wrote raw JSON bytes
    /// under a fabricated format name.
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::fem2d::io::export::serializers::artifacts::stl::v_ascii::any::export(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::fem2d::io::export::serializers::artifacts::obj::v3_0::any::export(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<Fem2dAnyComposer>(),
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[FEM2D_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[FEM2D_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[FEM2D_DIALECT], compose: compose_export_json },
            ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[FEM2D_DIALECT], compose: compose_export_stl },
            ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[FEM2D_DIALECT], compose: compose_export_obj },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
