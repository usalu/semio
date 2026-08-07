//! ⚡️ FEM 2D artifact — operation enum + laws (constitutional: op).

use crate::artifacts::fem2d::diff::{index_of, Fem2dDiff};
use crate::artifacts::fem2d::{element_id, Fem2dDocument, FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use protocol::Operation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


// #region 🔖️Operation
/// 🧮️ Fem-2d operation: id-keyed document-collection edits, each with a true inverse computed from
/// the pre-operation projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Fem2dOperation {
    SetNode {
        index: usize,
        #[dsl(block)]
        node: FemNode,
    },
    RemoveNode {
        id: String,
    },
    SetElement {
        index: usize,
        #[dsl(statements)]
        element: Box<FemElement>,
    },
    RemoveElement {
        id: String,
    },
    SetMaterial {
        index: usize,
        #[dsl(block)]
        material: FemMaterial,
    },
    RemoveMaterial {
        id: String,
    },
    SetSection {
        index: usize,
        #[dsl(block)]
        section: FemSection,
    },
    RemoveSection {
        id: String,
    },
    SetSupport {
        index: usize,
        #[dsl(block)]
        support: FemSupport,
    },
    RemoveSupport {
        id: String,
    },
    SetLoadCase {
        index: usize,
        #[dsl(block)]
        load_case: FemLoadCase,
    },
    RemoveLoadCase {
        id: String,
    },
    SetRegion {
        index: usize,
        #[dsl(block)]
        region: FemRegion,
    },
    RemoveRegion {
        id: String,
    },
    SetCombination {
        index: usize,
        #[dsl(block)]
        combination: FemCombination,
    },
    RemoveCombination {
        id: String,
    },
    SetAnalysisSettings {
        #[dsl(block)]
        settings: FemAnalysisSettings,
    },
    /// 🌍️ Replaces the whole document (example import / reset).
    SetDocument {
        #[dsl(block)]
        document: Fem2dDocument,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for Fem2dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for Fem2dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


impl Operation<Fem2dDocument> for Fem2dOperation {
    type Diff = Fem2dDiff;

    fn diff(&self, _projection: &Fem2dDocument) -> Fem2dDiff {
        let mut diff = Fem2dDiff::default();
        match self {
            Fem2dOperation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem2dOperation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem2dOperation::SetElement { index, element } => diff.elements.set.push((*index, (**element).clone())),
            Fem2dOperation::RemoveElement { id } => diff.elements.removed.push(id.clone()),
            Fem2dOperation::SetMaterial { index, material } => diff.materials.set.push((*index, material.clone())),
            Fem2dOperation::RemoveMaterial { id } => diff.materials.removed.push(id.clone()),
            Fem2dOperation::SetSection { index, section } => diff.sections.set.push((*index, section.clone())),
            Fem2dOperation::RemoveSection { id } => diff.sections.removed.push(id.clone()),
            Fem2dOperation::SetSupport { index, support } => diff.supports.set.push((*index, support.clone())),
            Fem2dOperation::RemoveSupport { id } => diff.supports.removed.push(id.clone()),
            Fem2dOperation::SetLoadCase { index, load_case } => diff.load_cases.set.push((*index, load_case.clone())),
            Fem2dOperation::RemoveLoadCase { id } => diff.load_cases.removed.push(id.clone()),
            Fem2dOperation::SetRegion { index, region } => diff.regions.set.push((*index, region.clone())),
            Fem2dOperation::RemoveRegion { id } => diff.regions.removed.push(id.clone()),
            Fem2dOperation::SetCombination { index, combination } => diff.combinations.set.push((*index, combination.clone())),
            Fem2dOperation::RemoveCombination { id } => diff.combinations.removed.push(id.clone()),
            Fem2dOperation::SetAnalysisSettings { settings } => diff.analysis = Some(settings.clone()),
            Fem2dOperation::SetDocument { document } => diff.document = Some(document.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Fem2dDocument) -> Vec<Self> {
        match self {
            Fem2dOperation::SetNode { node, .. } => match index_of(&projection.nodes, &node.id) {
                Some(index) => vec![Fem2dOperation::SetNode { index, node: projection.nodes[index].clone() }],
                None => vec![Fem2dOperation::RemoveNode { id: node.id.clone() }],
            },
            Fem2dOperation::RemoveNode { id } => index_of(&projection.nodes, id).map(|index| vec![Fem2dOperation::SetNode { index, node: projection.nodes[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetElement { element, .. } => match index_of(&projection.elements, element_id(element)) {
                Some(index) => vec![Fem2dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }],
                None => vec![Fem2dOperation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dOperation::RemoveElement { id } => index_of(&projection.elements, id).map(|index| vec![Fem2dOperation::SetElement { index, element: Box::new(projection.elements[index].clone()) }]).unwrap_or_default(),
            Fem2dOperation::SetMaterial { material, .. } => match index_of(&projection.materials, &material.id) {
                Some(index) => vec![Fem2dOperation::SetMaterial { index, material: projection.materials[index].clone() }],
                None => vec![Fem2dOperation::RemoveMaterial { id: material.id.clone() }],
            },
            Fem2dOperation::RemoveMaterial { id } => index_of(&projection.materials, id).map(|index| vec![Fem2dOperation::SetMaterial { index, material: projection.materials[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetSection { section, .. } => match index_of(&projection.sections, &section.id) {
                Some(index) => vec![Fem2dOperation::SetSection { index, section: projection.sections[index].clone() }],
                None => vec![Fem2dOperation::RemoveSection { id: section.id.clone() }],
            },
            Fem2dOperation::RemoveSection { id } => index_of(&projection.sections, id).map(|index| vec![Fem2dOperation::SetSection { index, section: projection.sections[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetSupport { support, .. } => match index_of(&projection.supports, &support.id) {
                Some(index) => vec![Fem2dOperation::SetSupport { index, support: projection.supports[index].clone() }],
                None => vec![Fem2dOperation::RemoveSupport { id: support.id.clone() }],
            },
            Fem2dOperation::RemoveSupport { id } => index_of(&projection.supports, id).map(|index| vec![Fem2dOperation::SetSupport { index, support: projection.supports[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetLoadCase { load_case, .. } => match index_of(&projection.load_cases, &load_case.id) {
                Some(index) => vec![Fem2dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }],
                None => vec![Fem2dOperation::RemoveLoadCase { id: load_case.id.clone() }],
            },
            Fem2dOperation::RemoveLoadCase { id } => index_of(&projection.load_cases, id).map(|index| vec![Fem2dOperation::SetLoadCase { index, load_case: projection.load_cases[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetRegion { region, .. } => match index_of(&projection.regions, &region.id) {
                Some(index) => vec![Fem2dOperation::SetRegion { index, region: projection.regions[index].clone() }],
                None => vec![Fem2dOperation::RemoveRegion { id: region.id.clone() }],
            },
            Fem2dOperation::RemoveRegion { id } => index_of(&projection.regions, id).map(|index| vec![Fem2dOperation::SetRegion { index, region: projection.regions[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetCombination { combination, .. } => match index_of(&projection.combinations, &combination.id) {
                Some(index) => vec![Fem2dOperation::SetCombination { index, combination: projection.combinations[index].clone() }],
                None => vec![Fem2dOperation::RemoveCombination { id: combination.id.clone() }],
            },
            Fem2dOperation::RemoveCombination { id } => index_of(&projection.combinations, id).map(|index| vec![Fem2dOperation::SetCombination { index, combination: projection.combinations[index].clone() }]).unwrap_or_default(),
            Fem2dOperation::SetAnalysisSettings { .. } => vec![Fem2dOperation::SetAnalysisSettings { settings: projection.analysis.clone() }],
            Fem2dOperation::SetDocument { .. } => vec![Fem2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Fem2dEnvelope = DocumentEnvelope<Fem2dDocument, Fem2dOperation>;
pub type Fem2dStore = DocumentStore<Fem2dDocument, Fem2dOperation>;
// #endregion 🔖️Operation

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️Fixtures
    fn simply_supported_beam_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![crate::artifacts::fem2d::FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }

    fn rectangle_region_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
        }
    }
    // #endregion 🔖️Fixtures

    // #region 🔖️OpRoundTrip
    fn round_trip(projection: &Fem2dDocument, operation: &Fem2dOperation) -> Fem2dDocument {
        let forward = vcs::apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn node_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOperation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 1.0 } });
        assert_eq!(after.nodes[0].x, 1.0);
        round_trip(&base, &Fem2dOperation::RemoveNode { id: "n1".into() });
    }

    #[test]
    fn element_op_round_trips() {
        let base = simply_supported_beam_doc();
        let updated = FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() };
        round_trip(&base, &Fem2dOperation::SetElement { index: 0, element: Box::new(updated) });
        round_trip(&base, &Fem2dOperation::RemoveElement { id: "e1".into() });
    }

    #[test]
    fn material_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "steel".into(), e: 200e9, nu: 0.3, rho: 7850.0 } });
        round_trip(&base, &Fem2dOperation::RemoveMaterial { id: "steel".into() });
    }

    #[test]
    fn section_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.01, iy: 1e-4 } });
        round_trip(&base, &Fem2dOperation::RemoveSection { id: "ipe300".into() });
    }

    #[test]
    fn support_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Ty] } });
        round_trip(&base, &Fem2dOperation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetLoadCase { index: 0, load_case: FemLoadCase { id: "dead".into(), name: "dead 2".into(), loads: vec![], self_weight: true } });
        round_trip(&base, &Fem2dOperation::RemoveLoadCase { id: "dead".into() });
    }

    #[test]
    fn region_op_round_trips() {
        let base = rectangle_region_doc();
        let updated = FemRegion { id: "r1".into(), name: "slab v2".into(), outline: vec![[0.0, 0.0], [5.0, 0.0], [5.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.03, material_id: "steel".into(), mesh_size: 0.5 };
        let after = round_trip(&base, &Fem2dOperation::SetRegion { index: 0, region: updated });
        assert_eq!(after.regions[0].thickness, 0.03);
        round_trip(&base, &Fem2dOperation::RemoveRegion { id: "r1".into() });
    }

    #[test]
    fn combination_op_round_trips() {
        let mut base = simply_supported_beam_doc();
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }] });
        let updated = FemCombination { id: "uls".into(), name: "ULS v2".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.4 }] };
        let after = round_trip(&base, &Fem2dOperation::SetCombination { index: 0, combination: updated });
        assert_eq!(after.combinations[0].terms[0].factor, 1.4);
        round_trip(&base, &Fem2dOperation::RemoveCombination { id: "uls".into() });
    }

    #[test]
    fn analysis_settings_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        assert_eq!(after.analysis.modal_count, 5);
    }

    #[test]
    fn document_op_round_trips() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let after = round_trip(&base, &Fem2dOperation::SetDocument { document: replacement.clone() });
        assert_eq!(after, replacement);
    }
    // #endregion 🔖️OpRoundTrip

    // #region 🔖️OpText
    #[test]
    fn fem2d_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetNode { index: 0, node: FemNode { id: "n1".into(), x: 1.0, y: 2.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveNode { id: "n1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetElement { index: 0, element: Box::new(FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }) });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetElement { index: 0, element: Box::new(FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }) });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveElement { id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetMaterial { index: 0, material: FemMaterial { id: "steel".into(), name: "Steel S235".into(), e: 210e9, nu: 0.3, rho: 7850.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveMaterial { id: "steel".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetSection { index: 0, section: FemSection { id: "ipe300".into(), name: "IPE 300".into(), area: 0.005381, iy: 8.356e-5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveSection { id: "ipe300".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveSupport { id: "s1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetLoadCase {
            index: 0,
            load_case: FemLoadCase {
                id: "dead".into(),
                name: "Dead Load".into(),
                loads: vec![
                    crate::artifacts::fem2d::FemLoad::Nodal { id: "l1".into(), node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -1000.0 },
                    crate::artifacts::fem2d::FemLoad::MemberUdl { id: "l2".into(), element_id: "e1".into(), wx: 0.0, wy: -5000.0 },
                    crate::artifacts::fem2d::FemLoad::Area { id: "l3".into(), region_id: "r1".into(), pressure: 800.0 },
                ],
                self_weight: true,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveLoadCase { id: "dead".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetRegion {
            index: 0,
            region: FemRegion {
                id: "r1".into(),
                name: "Slab".into(),
                outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]],
                holes: vec![vec![[1.0, 1.0], [2.0, 1.0], [2.0, 1.5]]],
                thickness: 0.02,
                material_id: "steel".into(),
                mesh_size: 0.5,
            },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveRegion { id: "r1".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetCombination {
            index: 0,
            combination: FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }] },
        });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::RemoveCombination { id: "uls".into() });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetAnalysisSettings { settings: FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
        store::test_support::assert_op_line_round_trip(&Fem2dOperation::SetDocument { document: simply_supported_beam_doc() });
    }
    // #endregion 🔖️OpText
}
// #endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

