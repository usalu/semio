//! 📐 FEM 2D document model and element library on `vcs`.

#[cfg(test)]
use fem_core::ElementResult;
use fem_core::{Bar2, BeamEb2, Dof, Element, MemberUdl, Model, NodalLoad, Node, Support};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use vcs::create_document_vcs_envelope;
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const FEM_2D_SCHEMA: &str = "fem.2d";

// #region 🔖Document
/// 📍 A structural node in plan (x, y in meters).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
}

/// 🔩 A 2-node structural member — axial-only `Bar` or axial+bending `Beam`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemElement {
    #[serde(rename_all = "camelCase")]
    Bar { id: String, start: String, end: String, material_id: String, section_id: String },
    #[serde(rename_all = "camelCase")]
    Beam { id: String, start: String, end: String, material_id: String, section_id: String },
}

/// 🪪 A `FemElement`'s stable id, across both variants.
pub fn element_id(element: &FemElement) -> &str {
    match element {
        FemElement::Bar { id, .. } | FemElement::Beam { id, .. } => id,
    }
}

/// 🧱 An isotropic material — Young's modulus `e` in Pascals, Poisson's ratio `nu`, density `rho`
/// in kg/m³ (the latter two required for continuum `FemRegion` elements and self-weight).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemMaterial {
    pub id: String,
    pub name: String,
    pub e: f64,
    pub nu: f64,
    pub rho: f64,
}

/// 📏 A cross-section — area in m², strong-axis moment of inertia `iy` in m⁴.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSection {
    pub id: String,
    pub name: String,
    pub area: f64,
    pub iy: f64,
}

/// 🔒 A support: the subset of a node's DOFs restrained to zero displacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemSupport {
    pub id: String,
    pub node_id: String,
    pub fixed: Vec<Dof>,
}

/// 🏋️ A load — a concentrated nodal force/moment, a member UDL, or a normal pressure (Pa) over a
/// meshed `FemRegion`, simplified as a uniform global `-Y` nodal load (see `area_load_nodal_loads`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FemLoad {
    #[serde(rename_all = "camelCase")]
    Nodal { id: String, node_id: String, dof: Dof, value: f64 },
    #[serde(rename_all = "camelCase")]
    MemberUdl { id: String, element_id: String, wx: f64, wy: f64 },
    #[serde(rename_all = "camelCase")]
    Area { id: String, region_id: String, pressure: f64 },
}

/// 🪪 A `FemLoad`'s stable id, across every variant.
pub fn load_id(load: &FemLoad) -> &str {
    match load {
        FemLoad::Nodal { id, .. } | FemLoad::MemberUdl { id, .. } | FemLoad::Area { id, .. } => id,
    }
}

/// 📦 A named set of loads applied together for one analysis run, optionally including self-weight.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemLoadCase {
    pub id: String,
    pub name: String,
    pub loads: Vec<FemLoad>,
    pub self_weight: bool,
}

/// 🟩 A meshed continuum region — a polygon (with optional holes) filled with `Tri3Cst` elements at
/// solve time (see `build_nodes_and_elements`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemRegion {
    pub id: String,
    pub name: String,
    pub outline: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
    pub thickness: f64,
    pub material_id: String,
    pub mesh_size: f64,
}

/// 🧮 A linear combination of load cases — `(case_id, factor)` terms superposed by `fem2d_solve_all`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemCombination {
    pub id: String,
    pub name: String,
    pub terms: Vec<(String, f64)>,
}

/// ⚙️ Analysis settings — modal/buckling mode counts and the viewport deformation scale factor.
/// `deformation_scale` exaggerates the STATIC results view's real (meter-scale) displacements only;
/// modal/buckling mode shapes are dimensionless (mass/Kg-orthonormalized) and the viewer normalizes
/// them to a fixed fraction of the model's own extent instead of using this factor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemAnalysisSettings {
    pub modal_count: usize,
    pub buckling_count: usize,
    pub deformation_scale: f64,
}

impl Default for FemAnalysisSettings {
    fn default() -> Self {
        Self { modal_count: 3, buckling_count: 3, deformation_scale: 50.0 }
    }
}

/// 🎥 The canvas camera (pan/zoom) for the plugin viewport.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FemCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for FemCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🧾 Persistent fem-2d document — nodes, members, meshed regions, materials/sections, supports,
/// load cases/combinations, analysis settings and camera.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dDocument {
    pub nodes: Vec<FemNode>,
    pub elements: Vec<FemElement>,
    pub regions: Vec<FemRegion>,
    pub materials: Vec<FemMaterial>,
    pub sections: Vec<FemSection>,
    pub supports: Vec<FemSupport>,
    pub load_cases: Vec<FemLoadCase>,
    pub combinations: Vec<FemCombination>,
    pub analysis: FemAnalysisSettings,
    pub camera: FemCamera,
}
// #endregion 🔖Document

// #region 🔖Collections
/// 🪪 Stable-id accessor shared by every id-keyed document collection entry.
trait HasId {
    fn id(&self) -> &str;
}

impl HasId for FemNode {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemElement {
    fn id(&self) -> &str {
        element_id(self)
    }
}
impl HasId for FemMaterial {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemSection {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemSupport {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemLoadCase {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemRegion {
    fn id(&self) -> &str {
        &self.id
    }
}
impl HasId for FemCombination {
    fn id(&self) -> &str {
        &self.id
    }
}

/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id
/// already exists, else insert at the recorded index).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemNode)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemElement)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemMaterial)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemSection)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemSupport)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadCasesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemLoadCase)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemRegion)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, FemCombination)>,
}

/// 🩹 Applies a sparse id-keyed diff to a collection in place — remove-by-id, then replace-by-id
/// or insert-at-index for each `set` entry.
fn apply_collection_diff<T: HasId + Clone>(items: &mut Vec<T>, removed: &[String], set: &[(usize, T)]) {
    for id in removed {
        items.retain(|item| item.id() != id);
    }
    for (index, item) in set {
        if let Some(pos) = items.iter().position(|entry| entry.id() == item.id()) {
            items[pos] = item.clone();
        } else {
            items.insert((*index).min(items.len()), item.clone());
        }
    }
}

fn index_of<T: HasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}
// #endregion 🔖Collections

// #region 🔖Operations
/// 🩹 Sparse fem-2d diff over every document collection plus the scalar camera.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fem2dDiff {
    /// 🌍 Whole-document replacement (example import / reset); wins over every granular field below.
    pub document: Option<Fem2dDocument>,
    pub nodes: NodesDiff,
    pub elements: ElementsDiff,
    pub regions: RegionsDiff,
    pub materials: MaterialsDiff,
    pub sections: SectionsDiff,
    pub supports: SupportsDiff,
    pub load_cases: LoadCasesDiff,
    pub combinations: CombinationsDiff,
    pub analysis: Option<FemAnalysisSettings>,
    pub camera: Option<FemCamera>,
}

impl OperationDiff<Fem2dDocument> for Fem2dDiff {
    fn apply(&self, projection: &Fem2dDocument) -> Fem2dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        apply_collection_diff(&mut next.nodes, &self.nodes.removed, &self.nodes.set);
        apply_collection_diff(&mut next.elements, &self.elements.removed, &self.elements.set);
        apply_collection_diff(&mut next.regions, &self.regions.removed, &self.regions.set);
        apply_collection_diff(&mut next.materials, &self.materials.removed, &self.materials.set);
        apply_collection_diff(&mut next.sections, &self.sections.removed, &self.sections.set);
        apply_collection_diff(&mut next.supports, &self.supports.removed, &self.supports.set);
        apply_collection_diff(&mut next.load_cases, &self.load_cases.removed, &self.load_cases.set);
        apply_collection_diff(&mut next.combinations, &self.combinations.removed, &self.combinations.set);
        if let Some(analysis) = &self.analysis {
            next.analysis = analysis.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = Fem2dDiff { document: other.document, ..Default::default() };
            return;
        }
        self.nodes.removed.extend(other.nodes.removed);
        self.nodes.set.extend(other.nodes.set);
        self.elements.removed.extend(other.elements.removed);
        self.elements.set.extend(other.elements.set);
        self.regions.removed.extend(other.regions.removed);
        self.regions.set.extend(other.regions.set);
        self.materials.removed.extend(other.materials.removed);
        self.materials.set.extend(other.materials.set);
        self.sections.removed.extend(other.sections.removed);
        self.sections.set.extend(other.sections.set);
        self.supports.removed.extend(other.supports.removed);
        self.supports.set.extend(other.supports.set);
        self.load_cases.removed.extend(other.load_cases.removed);
        self.load_cases.set.extend(other.load_cases.set);
        self.combinations.removed.extend(other.combinations.removed);
        self.combinations.set.extend(other.combinations.set);
        if other.analysis.is_some() {
            self.analysis = other.analysis;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

/// 🧮 Fem-2d operation: id-keyed document-collection edits plus the scalar camera, each with a
/// true inverse computed from the pre-operation projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Fem2dOperation {
    SetNode { index: usize, node: FemNode },
    RemoveNode { id: String },
    SetElement { index: usize, element: FemElement },
    RemoveElement { id: String },
    SetMaterial { index: usize, material: FemMaterial },
    RemoveMaterial { id: String },
    SetSection { index: usize, section: FemSection },
    RemoveSection { id: String },
    SetSupport { index: usize, support: FemSupport },
    RemoveSupport { id: String },
    SetLoadCase { index: usize, load_case: FemLoadCase },
    RemoveLoadCase { id: String },
    SetRegion { index: usize, region: FemRegion },
    RemoveRegion { id: String },
    SetCombination { index: usize, combination: FemCombination },
    RemoveCombination { id: String },
    SetAnalysisSettings { settings: FemAnalysisSettings },
    SetCamera { camera: FemCamera },
    /// 🌍 Replaces the whole document (example import / reset).
    SetDocument { document: Fem2dDocument },
}

impl Operation<Fem2dDocument> for Fem2dOperation {
    type Diff = Fem2dDiff;

    fn diff(&self, _projection: &Fem2dDocument) -> Fem2dDiff {
        let mut diff = Fem2dDiff::default();
        match self {
            Fem2dOperation::SetNode { index, node } => diff.nodes.set.push((*index, node.clone())),
            Fem2dOperation::RemoveNode { id } => diff.nodes.removed.push(id.clone()),
            Fem2dOperation::SetElement { index, element } => diff.elements.set.push((*index, element.clone())),
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
            Fem2dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
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
                Some(index) => vec![Fem2dOperation::SetElement { index, element: projection.elements[index].clone() }],
                None => vec![Fem2dOperation::RemoveElement { id: element_id(element).to_string() }],
            },
            Fem2dOperation::RemoveElement { id } => index_of(&projection.elements, id).map(|index| vec![Fem2dOperation::SetElement { index, element: projection.elements[index].clone() }]).unwrap_or_default(),
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
            Fem2dOperation::SetCamera { .. } => vec![Fem2dOperation::SetCamera { camera: projection.camera.clone() }],
            Fem2dOperation::SetDocument { .. } => vec![Fem2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Fem2dEnvelope = DocumentVcsEnvelope<Fem2dDocument, Fem2dOperation>;
pub type Fem2dStore = DocumentVcsStore<Fem2dDocument, Fem2dOperation>;

pub fn empty_fem2d_projection() -> Fem2dDocument {
    Fem2dDocument::default()
}
// #endregion 🔖Operations

// #region 🔖Dsl
/// 📜 Hand-rolled lexer/printer for `Fem2dDocument`'s `.fem2d` DSL (`🔖Dsl`) and `Fem2dOperation`'s
/// one-line op text (`🔖OpText`) — every entity (node/element/material/section/support/region/load
/// case/combination) has ONE `print_*_fields`/`parse_*` pair shared verbatim by both a document line
/// (`@node ...`) and its matching operation line (`setNode index=.. ...`), so the two stay in lock-step
/// by construction. Grammar mirrors `vcs`'s own structural-line grammar and `writer`'s `writer_dsl`
/// module (`@marker key=value ... "trailing text"`), hand-rolled locally since `vcs`'s escaping helpers
/// are private to that crate. ASSUMES ids never contain `,` `:` `;` `|` `-` (mirrors `vcs`'s own
/// `split_ids`/`join_ids` precedent) — always true for this domain's short slug-like ids.
mod fem2d_dsl {
    use super::{Fem2dDocument, Fem2dOperation, FemAnalysisSettings, FemCamera, FemCombination, FemElement, FemLoad, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
    use fem_core::Dof;
    use std::collections::HashMap;
    use vcs::{TextError, TextSpan};

    //#region Lexer
    /// 🔐 Escapes `\`, `"` and newlines so arbitrary text (a name, or a whole nested `setDocument`
    /// document) fits inside one quoted field.
    fn escape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out
    }

    fn unescape_text(value: &str) -> String {
        let mut out = String::with_capacity(value.len());
        let mut chars = value.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(ch);
            }
        }
        out
    }

    /// 🔎 Finds the char index of the unescaped opening `"` of a trailing quoted field, mirroring
    /// `vcs`'s private `find_unescaped_trailing_quote` (kept in lock-step, see that doc comment).
    fn find_unescaped_trailing_quote(chars: &[char]) -> Option<usize> {
        if chars.is_empty() || *chars.last().unwrap() != '"' {
            return None;
        }
        let last = chars.len() - 1;
        let mut i = last;
        while i > 0 {
            i -= 1;
            if chars[i] == '"' {
                let mut backslashes = 0;
                let mut j = i;
                while j > 0 && chars[j - 1] == '\\' {
                    backslashes += 1;
                    j -= 1;
                }
                if backslashes % 2 == 0 {
                    return Some(i);
                }
            }
        }
        None
    }

    /// 🧾 One parsed `@marker key=value ...` line plus its optional trailing quoted text field.
    struct KvLine {
        marker: String,
        fields: HashMap<String, String>,
        text: Option<String>,
    }

    fn parse_kv_line(line: &str, line_no: u32) -> Result<KvLine, TextError> {
        let chars: Vec<char> = line.chars().collect();
        let (head, text) = match find_unescaped_trailing_quote(&chars) {
            Some(open) => {
                let content: String = chars[open + 1..chars.len() - 1].iter().collect();
                let head: String = chars[..open].iter().collect();
                (head.trim_end().to_string(), Some(unescape_text(&content)))
            }
            None => (line.to_string(), None),
        };
        let mut tokens = head.split_whitespace();
        let marker = tokens
            .next()
            .ok_or_else(|| TextError::new("expected a marker or operation name", TextSpan::at(line_no, 1)))?
            .to_string();
        let mut fields = HashMap::new();
        for token in tokens {
            let (key, value) = token
                .split_once('=')
                .ok_or_else(|| TextError::new(format!("expected key=value token, got '{token}'"), TextSpan::at(line_no, 1)))?;
            fields.insert(key.to_string(), value.to_string());
        }
        Ok(KvLine { marker, fields, text })
    }

    fn field<'a>(fields: &'a HashMap<String, String>, key: &str, line_no: u32) -> Result<&'a str, TextError> {
        fields
            .get(key)
            .map(|value| value.as_str())
            .ok_or_else(|| TextError::new(format!("missing field '{key}'"), TextSpan::at(line_no, 1)))
    }

    fn parse_f64(value: &str, key: &str, line_no: u32) -> Result<f64, TextError> {
        value.parse::<f64>().map_err(|_| TextError::new(format!("expected number for '{key}', got '{value}'"), TextSpan::at(line_no, 1)))
    }

    fn parse_usize(value: &str, key: &str, line_no: u32) -> Result<usize, TextError> {
        value.parse::<usize>().map_err(|_| TextError::new(format!("expected integer for '{key}', got '{value}'"), TextSpan::at(line_no, 1)))
    }

    fn parse_bool(value: &str, key: &str, line_no: u32) -> Result<bool, TextError> {
        match value {
            "true" => Ok(true),
            "false" => Ok(false),
            other => Err(TextError::expected(format!("expected bool for '{key}', got '{other}'"), TextSpan::at(line_no, 1), "true|false")),
        }
    }

    /// 🔢 Prints an `f64` via Rust's shortest round-trippable `Display` form (`"0"`, not `"0.0"`).
    fn fmt_num(value: f64) -> String {
        value.to_string()
    }
    //#endregion Lexer

    //#region Composite
    /// 📐 `x,y;x,y;...` polygon vertex list, or `-` for empty.
    fn print_points(points: &[[f64; 2]]) -> String {
        if points.is_empty() {
            return "-".to_string();
        }
        points.iter().map(|p| format!("{},{}", fmt_num(p[0]), fmt_num(p[1]))).collect::<Vec<_>>().join(";")
    }

    fn parse_points(spec: &str, key: &str, line_no: u32) -> Result<Vec<[f64; 2]>, TextError> {
        if spec == "-" {
            return Ok(Vec::new());
        }
        spec.split(';')
            .map(|pair| {
                let (x, y) = pair.split_once(',').ok_or_else(|| TextError::new(format!("expected '{key}' point 'x,y', got '{pair}'"), TextSpan::at(line_no, 1)))?;
                Ok([parse_f64(x, key, line_no)?, parse_f64(y, key, line_no)?])
            })
            .collect()
    }

    /// 🕳️ `points|points|...` hole list (each hole itself a `print_points` polygon), or `-` for none.
    fn print_holes(holes: &[Vec<[f64; 2]>]) -> String {
        if holes.is_empty() {
            return "-".to_string();
        }
        holes.iter().map(|hole| print_points(hole)).collect::<Vec<_>>().join("|")
    }

    fn parse_holes(spec: &str, key: &str, line_no: u32) -> Result<Vec<Vec<[f64; 2]>>, TextError> {
        if spec == "-" {
            return Ok(Vec::new());
        }
        spec.split('|').map(|hole| parse_points(hole, key, line_no)).collect()
    }

    fn dof_name(dof: Dof) -> &'static str {
        match dof {
            Dof::Tx => "Tx",
            Dof::Ty => "Ty",
            Dof::Tz => "Tz",
            Dof::Rx => "Rx",
            Dof::Ry => "Ry",
            Dof::Rz => "Rz",
        }
    }

    fn parse_dof(token: &str, line_no: u32) -> Result<Dof, TextError> {
        match token {
            "Tx" => Ok(Dof::Tx),
            "Ty" => Ok(Dof::Ty),
            "Tz" => Ok(Dof::Tz),
            "Rx" => Ok(Dof::Rx),
            "Ry" => Ok(Dof::Ry),
            "Rz" => Ok(Dof::Rz),
            other => Err(TextError::expected(format!("unknown dof '{other}'"), TextSpan::at(line_no, 1), "Tx|Ty|Tz|Rx|Ry|Rz")),
        }
    }

    fn print_dofs(dofs: &[Dof]) -> String {
        if dofs.is_empty() {
            return "-".to_string();
        }
        dofs.iter().map(|dof| dof_name(*dof)).collect::<Vec<_>>().join(",")
    }

    fn parse_dofs(spec: &str, line_no: u32) -> Result<Vec<Dof>, TextError> {
        if spec == "-" {
            return Ok(Vec::new());
        }
        spec.split(',').map(|token| parse_dof(token, line_no)).collect()
    }

    /// 🧮 `id:factor,id:factor,...` combination term list, or `-` for empty.
    fn print_terms(terms: &[(String, f64)]) -> String {
        if terms.is_empty() {
            return "-".to_string();
        }
        terms.iter().map(|(id, factor)| format!("{id}:{}", fmt_num(*factor))).collect::<Vec<_>>().join(",")
    }

    fn parse_terms(spec: &str, line_no: u32) -> Result<Vec<(String, f64)>, TextError> {
        if spec == "-" {
            return Ok(Vec::new());
        }
        spec.split(',')
            .map(|entry| {
                let (id, factor) = entry.split_once(':').ok_or_else(|| TextError::new(format!("expected 'id:factor' term, got '{entry}'"), TextSpan::at(line_no, 1)))?;
                Ok((id.to_string(), parse_f64(factor, "term.factor", line_no)?))
            })
            .collect()
    }

    fn next_part<'a>(parts: &mut impl Iterator<Item = &'a str>, what: &str, line_no: u32) -> Result<&'a str, TextError> {
        parts.next().ok_or_else(|| TextError::new(format!("expected {what}"), TextSpan::at(line_no, 1)))
    }

    /// 🏋️ One `FemLoad` as `kind:id:...kind-specific fields...` — see `parse_load` for each kind's shape.
    fn print_load(load: &FemLoad) -> String {
        match load {
            FemLoad::Nodal { id, node_id, dof, value } => format!("nodal:{id}:{node_id}:{}:{}", dof_name(*dof), fmt_num(*value)),
            FemLoad::MemberUdl { id, element_id, wx, wy } => format!("memberUdl:{id}:{element_id}:{}:{}", fmt_num(*wx), fmt_num(*wy)),
            FemLoad::Area { id, region_id, pressure } => format!("area:{id}:{region_id}:{}", fmt_num(*pressure)),
        }
    }

    fn parse_load(token: &str, line_no: u32) -> Result<FemLoad, TextError> {
        let mut parts = token.split(':');
        let kind = next_part(&mut parts, "a load kind", line_no)?;
        match kind {
            "nodal" => {
                let id = next_part(&mut parts, "load id", line_no)?.to_string();
                let node_id = next_part(&mut parts, "load node id", line_no)?.to_string();
                let dof = parse_dof(next_part(&mut parts, "load dof", line_no)?, line_no)?;
                let value = parse_f64(next_part(&mut parts, "load value", line_no)?, "value", line_no)?;
                Ok(FemLoad::Nodal { id, node_id, dof, value })
            }
            "memberUdl" => {
                let id = next_part(&mut parts, "load id", line_no)?.to_string();
                let element_id = next_part(&mut parts, "load element id", line_no)?.to_string();
                let wx = parse_f64(next_part(&mut parts, "load wx", line_no)?, "wx", line_no)?;
                let wy = parse_f64(next_part(&mut parts, "load wy", line_no)?, "wy", line_no)?;
                Ok(FemLoad::MemberUdl { id, element_id, wx, wy })
            }
            "area" => {
                let id = next_part(&mut parts, "load id", line_no)?.to_string();
                let region_id = next_part(&mut parts, "load region id", line_no)?.to_string();
                let pressure = parse_f64(next_part(&mut parts, "load pressure", line_no)?, "pressure", line_no)?;
                Ok(FemLoad::Area { id, region_id, pressure })
            }
            other => Err(TextError::expected(format!("unknown load kind '{other}'"), TextSpan::at(line_no, 1), "nodal|memberUdl|area")),
        }
    }

    /// 🏋️ `load|load|...` load list, or `-` for empty.
    fn print_loads(loads: &[FemLoad]) -> String {
        if loads.is_empty() {
            return "-".to_string();
        }
        loads.iter().map(print_load).collect::<Vec<_>>().join("|")
    }

    fn parse_loads(spec: &str, line_no: u32) -> Result<Vec<FemLoad>, TextError> {
        if spec == "-" {
            return Ok(Vec::new());
        }
        spec.split('|').map(|token| parse_load(token, line_no)).collect()
    }
    //#endregion Composite

    //#region Entities
    fn print_node_fields(node: &FemNode) -> String {
        format!("id={} x={} y={}", node.id, fmt_num(node.x), fmt_num(node.y))
    }

    fn parse_node(fields: &HashMap<String, String>, line_no: u32) -> Result<FemNode, TextError> {
        Ok(FemNode { id: field(fields, "id", line_no)?.to_string(), x: parse_f64(field(fields, "x", line_no)?, "x", line_no)?, y: parse_f64(field(fields, "y", line_no)?, "y", line_no)? })
    }

    fn element_kind(element: &FemElement) -> &'static str {
        match element {
            FemElement::Bar { .. } => "bar",
            FemElement::Beam { .. } => "beam",
        }
    }

    fn print_element_fields(element: &FemElement) -> String {
        let (id, start, end, material_id, section_id) = match element {
            FemElement::Bar { id, start, end, material_id, section_id } | FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
        };
        format!("id={id} start={start} end={end} material={material_id} section={section_id}")
    }

    fn parse_element(kind: &str, fields: &HashMap<String, String>, line_no: u32) -> Result<FemElement, TextError> {
        let id = field(fields, "id", line_no)?.to_string();
        let start = field(fields, "start", line_no)?.to_string();
        let end = field(fields, "end", line_no)?.to_string();
        let material_id = field(fields, "material", line_no)?.to_string();
        let section_id = field(fields, "section", line_no)?.to_string();
        match kind {
            "bar" => Ok(FemElement::Bar { id, start, end, material_id, section_id }),
            "beam" => Ok(FemElement::Beam { id, start, end, material_id, section_id }),
            other => Err(TextError::expected(format!("unknown element kind '{other}'"), TextSpan::at(line_no, 1), "bar|beam")),
        }
    }

    fn print_material_fields(material: &FemMaterial) -> String {
        format!("id={} e={} nu={} rho={}", material.id, fmt_num(material.e), fmt_num(material.nu), fmt_num(material.rho))
    }

    fn parse_material(fields: &HashMap<String, String>, name: String, line_no: u32) -> Result<FemMaterial, TextError> {
        Ok(FemMaterial {
            id: field(fields, "id", line_no)?.to_string(),
            name,
            e: parse_f64(field(fields, "e", line_no)?, "e", line_no)?,
            nu: parse_f64(field(fields, "nu", line_no)?, "nu", line_no)?,
            rho: parse_f64(field(fields, "rho", line_no)?, "rho", line_no)?,
        })
    }

    fn print_section_fields(section: &FemSection) -> String {
        format!("id={} area={} iy={}", section.id, fmt_num(section.area), fmt_num(section.iy))
    }

    fn parse_section(fields: &HashMap<String, String>, name: String, line_no: u32) -> Result<FemSection, TextError> {
        Ok(FemSection { id: field(fields, "id", line_no)?.to_string(), name, area: parse_f64(field(fields, "area", line_no)?, "area", line_no)?, iy: parse_f64(field(fields, "iy", line_no)?, "iy", line_no)? })
    }

    fn print_support_fields(support: &FemSupport) -> String {
        format!("id={} node={} fixed={}", support.id, support.node_id, print_dofs(&support.fixed))
    }

    fn parse_support(fields: &HashMap<String, String>, line_no: u32) -> Result<FemSupport, TextError> {
        Ok(FemSupport { id: field(fields, "id", line_no)?.to_string(), node_id: field(fields, "node", line_no)?.to_string(), fixed: parse_dofs(field(fields, "fixed", line_no)?, line_no)? })
    }

    fn print_region_fields(region: &FemRegion) -> String {
        format!(
            "id={} material={} thickness={} mesh={} outline={} holes={}",
            region.id,
            region.material_id,
            fmt_num(region.thickness),
            fmt_num(region.mesh_size),
            print_points(&region.outline),
            print_holes(&region.holes)
        )
    }

    fn parse_region(fields: &HashMap<String, String>, name: String, line_no: u32) -> Result<FemRegion, TextError> {
        Ok(FemRegion {
            id: field(fields, "id", line_no)?.to_string(),
            name,
            outline: parse_points(field(fields, "outline", line_no)?, "outline", line_no)?,
            holes: parse_holes(field(fields, "holes", line_no)?, "holes", line_no)?,
            thickness: parse_f64(field(fields, "thickness", line_no)?, "thickness", line_no)?,
            material_id: field(fields, "material", line_no)?.to_string(),
            mesh_size: parse_f64(field(fields, "mesh", line_no)?, "mesh", line_no)?,
        })
    }

    fn print_load_case_fields(load_case: &FemLoadCase) -> String {
        format!("id={} selfweight={} loads={}", load_case.id, load_case.self_weight, print_loads(&load_case.loads))
    }

    fn parse_load_case(fields: &HashMap<String, String>, name: String, line_no: u32) -> Result<FemLoadCase, TextError> {
        Ok(FemLoadCase {
            id: field(fields, "id", line_no)?.to_string(),
            name,
            loads: parse_loads(field(fields, "loads", line_no)?, line_no)?,
            self_weight: parse_bool(field(fields, "selfweight", line_no)?, "selfweight", line_no)?,
        })
    }

    fn print_combination_fields(combination: &FemCombination) -> String {
        format!("id={} terms={}", combination.id, print_terms(&combination.terms))
    }

    fn parse_combination(fields: &HashMap<String, String>, name: String, line_no: u32) -> Result<FemCombination, TextError> {
        Ok(FemCombination { id: field(fields, "id", line_no)?.to_string(), name, terms: parse_terms(field(fields, "terms", line_no)?, line_no)? })
    }
    //#endregion Entities

    //#region Document
    /// 📤 Prints a full `.fem2d` document: nodes, elements (bar/beam, own line-kind per variant),
    /// regions, materials, sections, supports, load cases, combinations, one `@analysis` line and one
    /// `@camera` line — field order mirrors `Fem2dDocument`'s own struct field order.
    pub fn print_document(doc: &Fem2dDocument) -> String {
        let mut lines = Vec::new();
        for node in &doc.nodes {
            lines.push(format!("@node {}", print_node_fields(node)));
        }
        for element in &doc.elements {
            lines.push(format!("@{} {}", element_kind(element), print_element_fields(element)));
        }
        for region in &doc.regions {
            lines.push(format!("@region {} \"{}\"", print_region_fields(region), escape_text(&region.name)));
        }
        for material in &doc.materials {
            lines.push(format!("@material {} \"{}\"", print_material_fields(material), escape_text(&material.name)));
        }
        for section in &doc.sections {
            lines.push(format!("@section {} \"{}\"", print_section_fields(section), escape_text(&section.name)));
        }
        for support in &doc.supports {
            lines.push(format!("@support {}", print_support_fields(support)));
        }
        for load_case in &doc.load_cases {
            lines.push(format!("@loadcase {} \"{}\"", print_load_case_fields(load_case), escape_text(&load_case.name)));
        }
        for combination in &doc.combinations {
            lines.push(format!("@combination {} \"{}\"", print_combination_fields(combination), escape_text(&combination.name)));
        }
        lines.push(format!("@analysis modal={} buckling={} scale={}", doc.analysis.modal_count, doc.analysis.buckling_count, fmt_num(doc.analysis.deformation_scale)));
        lines.push(format!("@camera x={} y={} zoom={}", fmt_num(doc.camera.x), fmt_num(doc.camera.y), fmt_num(doc.camera.zoom)));
        lines.join("\n")
    }

    /// 📥 Parses a full `.fem2d` document back into a `Fem2dDocument` (see `print_document`). Every
    /// line kind is order-independent (dispatched purely by its `@marker`), so a hand-edited fixture may
    /// interleave entity kinds freely.
    pub fn parse_document(text: &str) -> Result<Fem2dDocument, TextError> {
        let mut doc = Fem2dDocument::default();
        for (index, raw_line) in text.lines().enumerate() {
            let line_no = index as u32 + 1;
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }
            let parsed = parse_kv_line(line, line_no)?;
            match parsed.marker.as_str() {
                "@node" => doc.nodes.push(parse_node(&parsed.fields, line_no)?),
                "@bar" | "@beam" => doc.elements.push(parse_element(&parsed.marker[1..], &parsed.fields, line_no)?),
                "@region" => doc.regions.push(parse_region(&parsed.fields, parsed.text.clone().unwrap_or_default(), line_no)?),
                "@material" => doc.materials.push(parse_material(&parsed.fields, parsed.text.clone().unwrap_or_default(), line_no)?),
                "@section" => doc.sections.push(parse_section(&parsed.fields, parsed.text.clone().unwrap_or_default(), line_no)?),
                "@support" => doc.supports.push(parse_support(&parsed.fields, line_no)?),
                "@loadcase" => doc.load_cases.push(parse_load_case(&parsed.fields, parsed.text.clone().unwrap_or_default(), line_no)?),
                "@combination" => doc.combinations.push(parse_combination(&parsed.fields, parsed.text.clone().unwrap_or_default(), line_no)?),
                "@analysis" => {
                    doc.analysis = FemAnalysisSettings {
                        modal_count: parse_usize(field(&parsed.fields, "modal", line_no)?, "modal", line_no)?,
                        buckling_count: parse_usize(field(&parsed.fields, "buckling", line_no)?, "buckling", line_no)?,
                        deformation_scale: parse_f64(field(&parsed.fields, "scale", line_no)?, "scale", line_no)?,
                    };
                }
                "@camera" => {
                    doc.camera = FemCamera {
                        x: parse_f64(field(&parsed.fields, "x", line_no)?, "x", line_no)?,
                        y: parse_f64(field(&parsed.fields, "y", line_no)?, "y", line_no)?,
                        zoom: parse_f64(field(&parsed.fields, "zoom", line_no)?, "zoom", line_no)?,
                    };
                }
                other => return Err(TextError::expected(format!("unknown fem2d dsl marker '{other}'"), TextSpan::at(line_no, 1), "@node|@bar|@beam|@region|@material|@section|@support|@loadcase|@combination|@analysis|@camera")),
            }
        }
        Ok(doc)
    }
    //#endregion Document

    //#region Operation
    /// 📤 Prints a single one-line `Fem2dOperation` — every `Set*` variant reuses the SAME
    /// `print_*_fields` helper its matching `@marker` document line uses, plus an `index=` field; a
    /// `setDocument` embeds `print_document`'s full multi-line output escaped into one quoted field.
    pub fn print_operation(operation: &Fem2dOperation) -> String {
        match operation {
            Fem2dOperation::SetNode { index, node } => format!("setNode index={index} {}", print_node_fields(node)),
            Fem2dOperation::RemoveNode { id } => format!("removeNode id={id}"),
            Fem2dOperation::SetElement { index, element } => format!("setElement index={index} kind={} {}", element_kind(element), print_element_fields(element)),
            Fem2dOperation::RemoveElement { id } => format!("removeElement id={id}"),
            Fem2dOperation::SetMaterial { index, material } => format!("setMaterial index={index} {} \"{}\"", print_material_fields(material), escape_text(&material.name)),
            Fem2dOperation::RemoveMaterial { id } => format!("removeMaterial id={id}"),
            Fem2dOperation::SetSection { index, section } => format!("setSection index={index} {} \"{}\"", print_section_fields(section), escape_text(&section.name)),
            Fem2dOperation::RemoveSection { id } => format!("removeSection id={id}"),
            Fem2dOperation::SetSupport { index, support } => format!("setSupport index={index} {}", print_support_fields(support)),
            Fem2dOperation::RemoveSupport { id } => format!("removeSupport id={id}"),
            Fem2dOperation::SetLoadCase { index, load_case } => format!("setLoadCase index={index} {} \"{}\"", print_load_case_fields(load_case), escape_text(&load_case.name)),
            Fem2dOperation::RemoveLoadCase { id } => format!("removeLoadCase id={id}"),
            Fem2dOperation::SetRegion { index, region } => format!("setRegion index={index} {} \"{}\"", print_region_fields(region), escape_text(&region.name)),
            Fem2dOperation::RemoveRegion { id } => format!("removeRegion id={id}"),
            Fem2dOperation::SetCombination { index, combination } => format!("setCombination index={index} {} \"{}\"", print_combination_fields(combination), escape_text(&combination.name)),
            Fem2dOperation::RemoveCombination { id } => format!("removeCombination id={id}"),
            Fem2dOperation::SetAnalysisSettings { settings } => format!("setAnalysisSettings modal={} buckling={} scale={}", settings.modal_count, settings.buckling_count, fmt_num(settings.deformation_scale)),
            Fem2dOperation::SetCamera { camera } => format!("setCamera x={} y={} zoom={}", fmt_num(camera.x), fmt_num(camera.y), fmt_num(camera.zoom)),
            Fem2dOperation::SetDocument { document } => format!("setDocument \"{}\"", escape_text(&print_document(document))),
        }
    }

    /// 📥 Parses a single one-line `Fem2dOperation` (see `print_operation`). Always parsed as "line 1" —
    /// the caller (`vcs::parse_document_text`) remaps the error span onto the op log's real line number.
    pub fn parse_operation(line: &str) -> Result<Fem2dOperation, TextError> {
        let parsed = parse_kv_line(line, 1)?;
        match parsed.marker.as_str() {
            "setNode" => Ok(Fem2dOperation::SetNode { index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?, node: parse_node(&parsed.fields, 1)? }),
            "removeNode" => Ok(Fem2dOperation::RemoveNode { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setElement" => Ok(Fem2dOperation::SetElement {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                element: parse_element(field(&parsed.fields, "kind", 1)?, &parsed.fields, 1)?,
            }),
            "removeElement" => Ok(Fem2dOperation::RemoveElement { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setMaterial" => Ok(Fem2dOperation::SetMaterial {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                material: parse_material(&parsed.fields, parsed.text.clone().unwrap_or_default(), 1)?,
            }),
            "removeMaterial" => Ok(Fem2dOperation::RemoveMaterial { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setSection" => Ok(Fem2dOperation::SetSection {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                section: parse_section(&parsed.fields, parsed.text.clone().unwrap_or_default(), 1)?,
            }),
            "removeSection" => Ok(Fem2dOperation::RemoveSection { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setSupport" => Ok(Fem2dOperation::SetSupport { index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?, support: parse_support(&parsed.fields, 1)? }),
            "removeSupport" => Ok(Fem2dOperation::RemoveSupport { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setLoadCase" => Ok(Fem2dOperation::SetLoadCase {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                load_case: parse_load_case(&parsed.fields, parsed.text.clone().unwrap_or_default(), 1)?,
            }),
            "removeLoadCase" => Ok(Fem2dOperation::RemoveLoadCase { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setRegion" => Ok(Fem2dOperation::SetRegion {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                region: parse_region(&parsed.fields, parsed.text.clone().unwrap_or_default(), 1)?,
            }),
            "removeRegion" => Ok(Fem2dOperation::RemoveRegion { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setCombination" => Ok(Fem2dOperation::SetCombination {
                index: parse_usize(field(&parsed.fields, "index", 1)?, "index", 1)?,
                combination: parse_combination(&parsed.fields, parsed.text.clone().unwrap_or_default(), 1)?,
            }),
            "removeCombination" => Ok(Fem2dOperation::RemoveCombination { id: field(&parsed.fields, "id", 1)?.to_string() }),
            "setAnalysisSettings" => Ok(Fem2dOperation::SetAnalysisSettings {
                settings: FemAnalysisSettings {
                    modal_count: parse_usize(field(&parsed.fields, "modal", 1)?, "modal", 1)?,
                    buckling_count: parse_usize(field(&parsed.fields, "buckling", 1)?, "buckling", 1)?,
                    deformation_scale: parse_f64(field(&parsed.fields, "scale", 1)?, "scale", 1)?,
                },
            }),
            "setCamera" => Ok(Fem2dOperation::SetCamera {
                camera: FemCamera { x: parse_f64(field(&parsed.fields, "x", 1)?, "x", 1)?, y: parse_f64(field(&parsed.fields, "y", 1)?, "y", 1)?, zoom: parse_f64(field(&parsed.fields, "zoom", 1)?, "zoom", 1)? },
            }),
            "setDocument" => {
                let text = parsed.text.ok_or_else(|| TextError::new("setDocument requires a quoted document field", TextSpan::at(1, 1)))?;
                Ok(Fem2dOperation::SetDocument { document: parse_document(&text)? })
            }
            other => Err(TextError::expected(
                format!("unknown fem2d operation '{other}'"),
                TextSpan::at(1, 1),
                "setNode|removeNode|setElement|removeElement|setMaterial|removeMaterial|setSection|removeSection|setSupport|removeSupport|setLoadCase|removeLoadCase|setRegion|removeRegion|setCombination|removeCombination|setAnalysisSettings|setCamera|setDocument",
            )),
        }
    }
    //#endregion Operation
}

impl vcs::DocumentDsl for Fem2dDocument {
    const EXTENSION: &'static str = "fem2d";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        fem2d_dsl::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        fem2d_dsl::print_document(self)
    }
}
// #endregion 🔖Dsl

// #region 🔖OpText
impl vcs::OpText for Fem2dOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        fem2d_dsl::parse_operation(line)
    }

    fn print_op(&self) -> String {
        fem2d_dsl::print_operation(self)
    }
}
// #endregion 🔖OpText

// #region 🔖Bridge

// #region 🔖Errors
/// ⚠️ Everything that can go wrong resolving or solving a `Fem2dDocument`.
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
    Fem(#[from] fem_core::FemError),
}
// #endregion 🔖Errors

// #region 🔖RegionMeshing
/// ⚖️ Gravitational acceleration (m/s²) used both by the document-bridge's own lumped self-weight
/// translation (`self_weight_nodal_loads`, feeding the frozen `fem2d_solve`) and as the `gravity`
/// argument to `fem_core::analyses::solve_multi_case` (`fem2d_solve_all`).
const GRAVITY_G: f64 = 9.81;

/// 🌐 One meshed `FemRegion` — resolved node ids (mesh point-index → doc/synthesized node id, ONE
/// per unique mesh point, matching `points`/`tris` index order), reused by `build_nodes_and_elements`'s
/// caller for area-load tributary-area and self-weight computation.
struct MeshedRegion {
    region_id: String,
    material_id: String,
    thickness: f64,
    node_ids: Vec<String>,
    points: Vec<[f64; 2]>,
    tris: Vec<[u32; 3]>,
}

/// 🧩 `build_nodes_and_elements`'s resolved `(nodes, elements, meshed regions)` triple.
type ResolvedGeometry = (Vec<Node>, Vec<Box<dyn Element>>, Vec<MeshedRegion>);

/// 📐 Unsigned area of triangle `(p0, p1, p2)` via the shoelace formula.
fn triangle_area(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> f64 {
    (0.5 * ((p1[0] - p0[0]) * (p2[1] - p0[1]) - (p2[0] - p0[0]) * (p1[1] - p0[1]))).abs()
}

/// 🌉 Shared node/element resolution for `build_model` and `fem2d_solve_all`: base document
/// nodes/elements (`Bar2`/`BeamEb2`, `density: material.rho`) plus every `FemRegion` meshed into
/// `Tri3Cst` elements (plane-stress, `PlaneKind::Stress`) — a region boundary point that coincides
/// (within `1e-9`, both x and y) with an existing document node reuses that node's id, so supports and
/// loads placed on that node reach the mesh; otherwise a node is synthesized once per unique mesh
/// point as `{region_id}_m{point_index}`.
fn build_nodes_and_elements(doc: &Fem2dDocument) -> Result<ResolvedGeometry, Fem2dError> {
    let node_exists = |id: &str| doc.nodes.iter().any(|n| n.id == id);
    let mut nodes: Vec<Node> = doc.nodes.iter().map(|n| Node { id: n.id.clone(), pos: [n.x, n.y, 0.0] }).collect();

    let mut elements: Vec<Box<dyn Element>> = Vec::with_capacity(doc.elements.len());
    for element in &doc.elements {
        let (id, start, end, material_id, section_id) = match element {
            FemElement::Bar { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
            FemElement::Beam { id, start, end, material_id, section_id } => (id, start, end, material_id, section_id),
        };
        if !node_exists(start) {
            return Err(Fem2dError::UnknownNodeId(start.clone()));
        }
        if !node_exists(end) {
            return Err(Fem2dError::UnknownNodeId(end.clone()));
        }
        let material = doc.materials.iter().find(|m| &m.id == material_id).ok_or_else(|| Fem2dError::UnknownMaterialId(material_id.clone()))?;
        let section = doc.sections.iter().find(|s| &s.id == section_id).ok_or_else(|| Fem2dError::UnknownSectionId(section_id.clone()))?;
        match element {
            FemElement::Bar { .. } => {
                elements.push(Box::new(Bar2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, density: material.rho }));
            }
            FemElement::Beam { .. } => {
                elements.push(Box::new(BeamEb2 { id: id.clone(), start: start.clone(), end: end.clone(), e: material.e, area: section.area, iy: section.iy, density: material.rho }));
            }
        }
    }

    let mut meshed_regions = Vec::with_capacity(doc.regions.len());
    for region in &doc.regions {
        let domain = fem_core::mesh::PlanarDomain { outer: region.outline.clone(), holes: region.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem2dError::MeshFailed { region_id: region.id.clone(), reason: e.to_string() })?;
        let material = doc.materials.iter().find(|m| m.id == region.material_id).ok_or_else(|| Fem2dError::UnknownMaterialId(region.material_id.clone()))?;

        let mut node_ids = Vec::with_capacity(tri_mesh.points.len());
        for (point_index, p) in tri_mesh.points.iter().enumerate() {
            let id = match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => {
                    let synthetic_id = format!("{}_m{}", region.id, point_index);
                    nodes.push(Node { id: synthetic_id.clone(), pos: [p[0], p[1], 0.0] });
                    synthetic_id
                }
            };
            node_ids.push(id);
        }

        for (tri_index, tri) in tri_mesh.tris.iter().enumerate() {
            let tri_nodes = [node_ids[tri[0] as usize].clone(), node_ids[tri[1] as usize].clone(), node_ids[tri[2] as usize].clone()];
            elements.push(Box::new(fem_core::elements2d::Tri3Cst { id: format!("{}_t{}", region.id, tri_index), nodes: tri_nodes, e: material.e, nu: material.nu, thickness: region.thickness, kind: fem_core::elements2d::PlaneKind::Stress, density: material.rho }));
        }

        meshed_regions.push(MeshedRegion { region_id: region.id.clone(), material_id: region.material_id.clone(), thickness: region.thickness, node_ids, points: tri_mesh.points, tris: tri_mesh.tris });
    }

    Ok((nodes, elements, meshed_regions))
}

/// ⚖️ Lumped self-weight nodal loads (downward, global `-Y`) — `ρ·A·L` split evenly at a bar/beam's
/// two end nodes, `ρ·thickness·triangleArea` split evenly at each region triangle's 3 nodes, summed
/// per node. A simple document-bridge translation feeding ONLY the frozen `fem2d_solve`/`build_model`
/// path (which has no native self-weight concept) — `fem2d_solve_all` never calls this helper, since it
/// gets self-weight natively through `fem_core::analyses`' own `element.mass()`-based pipeline for
/// EVERY massed element (`Bar2`/`BeamEb2` and now `Tri3Cst` regions too), so the two paths never overlap.
fn self_weight_nodal_loads(doc: &Fem2dDocument, regions: &[MeshedRegion]) -> Vec<NodalLoad> {
    let mut totals: HashMap<String, f64> = HashMap::new();

    for element in &doc.elements {
        let (start, end, material_id, section_id) = match element {
            FemElement::Bar { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
            FemElement::Beam { start, end, material_id, section_id, .. } => (start, end, material_id, section_id),
        };
        let (Some(material), Some(section), Some(n0), Some(n1)) =
            (doc.materials.iter().find(|m| &m.id == material_id), doc.sections.iter().find(|s| &s.id == section_id), doc.nodes.iter().find(|n| &n.id == start), doc.nodes.iter().find(|n| &n.id == end))
        else {
            continue;
        };
        let length = ((n1.x - n0.x).powi(2) + (n1.y - n0.y).powi(2)).sqrt();
        let weight = material.rho * section.area * length * GRAVITY_G;
        *totals.entry(start.clone()).or_insert(0.0) += weight / 2.0;
        *totals.entry(end.clone()).or_insert(0.0) += weight / 2.0;
    }

    for region in regions {
        let Some(material) = doc.materials.iter().find(|m| m.id == region.material_id) else { continue };
        for tri in &region.tris {
            let area = triangle_area(region.points[tri[0] as usize], region.points[tri[1] as usize], region.points[tri[2] as usize]);
            let weight = material.rho * region.thickness * area * GRAVITY_G;
            for &idx in tri {
                *totals.entry(region.node_ids[idx as usize].clone()).or_insert(0.0) += weight / 3.0;
            }
        }
    }

    totals.into_iter().map(|(node_id, weight)| NodalLoad { node_id, dof: Dof::Ty, value: -weight }).collect()
}

/// 🌬️ Converts a `FemLoad::Area` (uniform pressure, Pa) into per-node global `-Y` nodal loads —
/// `pressure * tributaryArea` at each region node, where tributary area is `(1/3)` of the summed area
/// of every triangle touching that node.
fn area_load_nodal_loads(region: &MeshedRegion, pressure: f64) -> Vec<NodalLoad> {
    let mut tributary: HashMap<String, f64> = HashMap::new();
    for tri in &region.tris {
        let area = triangle_area(region.points[tri[0] as usize], region.points[tri[1] as usize], region.points[tri[2] as usize]);
        for &idx in tri {
            *tributary.entry(region.node_ids[idx as usize].clone()).or_insert(0.0) += area / 3.0;
        }
    }
    tributary.into_iter().map(|(node_id, trib)| NodalLoad { node_id, dof: Dof::Ty, value: -pressure * trib }).collect()
}
// #endregion 🔖RegionMeshing

/// 🌉 Resolves a `Fem2dDocument` plus a named load case into a `fem_core::Model`, erroring
/// descriptively on any dangling material/section/node/region reference.
pub fn build_model(doc: &Fem2dDocument, case_id: &str) -> Result<Model, Fem2dError> {
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: *dof, value: *value }),
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

    Ok(Model { nodes, elements, supports, nodal_loads, member_loads })
}

/// 🌉 Frozen public entry point: solves a `Fem2dDocument`'s named load case for linear-static
/// equilibrium. Signature is a contract consumed directly by `fem-plugin` — do not rename or
/// change it.
pub fn fem2d_solve(doc: &Fem2dDocument, case_id: &str) -> Result<fem_core::StaticResult, String> {
    let model = build_model(doc, case_id).map_err(|e| e.to_string())?;
    fem_core::solve_linear_static(&model).map_err(|e| e.to_string())
}

/// 🌉 Richer entry point: resolves EVERY `doc.load_cases`/`doc.combinations` entry at once (regions
/// meshed via the same `build_nodes_and_elements` resolution as `build_model`) and solves them all
/// together via `fem_core::analyses::solve_multi_case` — self-weight honored per-case through
/// `doc.materials`' `rho` (see `self_weight_nodal_loads`'s doc for the `Tri3Cst` caveat), gravity
/// fixed at `[0.0, -9.81, 0.0]`. Returns results keyed by case id ∪ combination id.
pub fn fem2d_solve_all(doc: &Fem2dDocument) -> Result<HashMap<String, fem_core::StaticResult>, Fem2dError> {
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };

    let mut cases = Vec::with_capacity(doc.load_cases.len());
    for load_case in &doc.load_cases {
        let mut nodal_loads = Vec::new();
        let mut member_loads = Vec::new();
        for load in &load_case.loads {
            match load {
                FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: *dof, value: *value }),
                FemLoad::MemberUdl { element_id, wx, wy, .. } => {
                    member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 }));
                }
                FemLoad::Area { region_id, pressure, .. } => {
                    let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                    nodal_loads.extend(area_load_nodal_loads(region, *pressure));
                }
            }
        }
        cases.push(fem_core::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight });
    }

    let combinations: Vec<fem_core::analyses::Combination> = doc.combinations.iter().map(|c| fem_core::analyses::Combination { id: c.id.clone(), terms: c.terms.clone() }).collect();

    fem_core::analyses::solve_multi_case(&model, &cases, &combinations, [0.0, -GRAVITY_G, 0.0]).map_err(Fem2dError::from)
}

// #region 🔖ModalBuckling
/// 🔢 Node-major, active-DOF-filtered ordering matching `fem_core::analyses::ModalResult`/
/// `BucklingResult`'s documented shape-vector layout — a small local reimplementation (mirrors
/// `analyses::build_dof_map`, which isn't `pub`, following the same precedent that module's own doc
/// comment sets for `lib.rs`'s private `build_dof_map`) used to unpack a raw mode-shape `VecD` back
/// into per-node `[f64;6]` values.
fn mode_dof_order(nodes: &[Node], elements: &[Box<dyn Element>]) -> Vec<(String, Dof)> {
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in elements {
            if element.node_ids().iter().any(|id| id == &node.id) {
                for &dof in element.dofs_per_node() {
                    if !active.contains(&dof) {
                        active.push(dof);
                    }
                }
            }
        }
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    order
}

/// 🎵 Modal analysis: lowest `doc.analysis.modal_count` natural frequencies/mode shapes.
pub fn fem2d_modal(doc: &Fem2dDocument) -> Result<fem_core::analyses::ModalResult, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };
    fem_core::analyses::modal(&model, doc.analysis.modal_count).map_err(Fem2dError::from)
}

/// 🌉 Richer modal entry point: solves the same modal analysis as `fem2d_modal` but also unpacks mode
/// `mode_index`'s shape `VecD` into a friendly per-node `[f64;6]` displacement map (see `mode_dof_order`),
/// ready to feed the same deformed-shape rendering `fem-plugin` already uses for static results. Returns
/// `(frequency_hz, node_id -> displacement values)`.
pub fn fem2d_modal_mode_values(doc: &Fem2dDocument, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let order = mode_dof_order(&nodes, &elements);
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };
    let result = fem_core::analyses::modal(&model, doc.analysis.modal_count)?;
    let freq = *result.frequencies_hz.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((freq, values))
}

/// 🧩 `buckling_inputs`'s resolved `(nodes, elements, supports, load case)` quadruple.
type BucklingInputs = (Vec<Node>, Vec<Box<dyn Element>>, Vec<Support>, fem_core::analyses::LoadCase);

/// 🌉 Shared buckling-case resolution for `fem2d_buckling`/`fem2d_buckling_mode_values`: builds the
/// geometry plus the ONE named `case_id`'s `analyses::LoadCase`, mirroring `fem2d_solve_all`'s
/// per-case load translation (nodal/member-UDL/area loads), erroring `"load case not found: {case_id}"`
/// if `case_id` isn't in `doc.load_cases`.
fn buckling_inputs(doc: &Fem2dDocument, case_id: &str) -> Result<BucklingInputs, Fem2dError> {
    let (nodes, elements, regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();
    let load_case = doc.load_cases.iter().find(|lc| lc.id == case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;

    let mut nodal_loads = Vec::new();
    let mut member_loads = Vec::new();
    for load in &load_case.loads {
        match load {
            FemLoad::Nodal { node_id, dof, value, .. } => nodal_loads.push(NodalLoad { node_id: node_id.clone(), dof: *dof, value: *value }),
            FemLoad::MemberUdl { element_id, wx, wy, .. } => member_loads.push((element_id.clone(), MemberUdl { wx: *wx, wy: *wy, wz: 0.0 })),
            FemLoad::Area { region_id, pressure, .. } => {
                let region = regions.iter().find(|r| &r.region_id == region_id).ok_or_else(|| Fem2dError::UnknownRegionId(region_id.clone()))?;
                nodal_loads.extend(area_load_nodal_loads(region, *pressure));
            }
        }
    }
    let case = fem_core::analyses::LoadCase { id: load_case.id.clone(), nodal_loads, member_loads, self_weight: load_case.self_weight };
    Ok((nodes, elements, supports, case))
}

/// 🏛️ Linear buckling: lowest `doc.analysis.buckling_count` load factors/mode shapes for `case_id`.
pub fn fem2d_buckling(doc: &Fem2dDocument, case_id: &str) -> Result<fem_core::analyses::BucklingResult, Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };
    fem_core::analyses::buckling(&model, &case, doc.analysis.buckling_count).map_err(Fem2dError::from)
}

/// 🌉 Richer buckling entry point: mirrors `fem2d_modal_mode_values` — solves the same buckling
/// analysis as `fem2d_buckling` but also unpacks mode `mode_index`'s shape into a per-node
/// displacement map. Returns `(load_factor, node_id -> displacement values)`.
pub fn fem2d_buckling_mode_values(doc: &Fem2dDocument, case_id: &str, mode_index: usize) -> Result<(f64, HashMap<String, [f64; 6]>), Fem2dError> {
    let (nodes, elements, supports, case) = buckling_inputs(doc, case_id)?;
    let order = mode_dof_order(&nodes, &elements);
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };
    let result = fem_core::analyses::buckling(&model, &case, doc.analysis.buckling_count)?;
    let factor = *result.factors.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let shape = result.shapes.get(mode_index).ok_or(Fem2dError::ModeIndexOutOfRange(mode_index))?;
    let mut values: HashMap<String, [f64; 6]> = HashMap::new();
    for (i, (node_id, dof)) in order.iter().enumerate() {
        values.entry(node_id.clone()).or_insert([0.0; 6])[dof.index()] = shape.get(i);
    }
    Ok((factor, values))
}
// #endregion 🔖ModalBuckling

// #region 🔖MeshPreview
/// 🗺️ One meshed region's cheap preview geometry — mesh points plus triangle vertex indices, WITHOUT
/// building any `fem_core::Element`. Used purely by `fem-plugin` for a mesh-edge preview overlay in the
/// model window and to correlate `fem2d_solve_all`'s `Tri3Cst` results (ids `"{region_id}_t{tri_index}"`,
/// see `build_nodes_and_elements`) back to screen-space triangles for contour rendering.
pub struct RegionMesh {
    pub region_id: String,
    pub points: Vec<[f64; 2]>,
    pub tris: Vec<[u32; 3]>,
    /// 🪪 Per-point node id, SAME coincident-node resolution `build_nodes_and_elements` uses (existing
    /// doc node within `1e-9` reused, else synthesized `"{region_id}_m{point_index}"`) — lets a caller
    /// (`fem-plugin`'s nodal-averaged contour rendering) map `fem2d_nodal_von_mises`'s node-keyed map
    /// straight onto this mesh's triangles.
    pub node_ids: Vec<String>,
}

/// 🗺️ Triangulates every `FemRegion` in `doc` (same `fem_core::mesh::triangulate` call as
/// `build_nodes_and_elements`, so triangle indices/ids line up deterministically with solved results)
/// and returns just the geometry — cheap enough to call on every render.
pub fn fem2d_mesh_preview(doc: &Fem2dDocument) -> Result<Vec<RegionMesh>, Fem2dError> {
    let mut out = Vec::with_capacity(doc.regions.len());
    for region in &doc.regions {
        let domain = fem_core::mesh::PlanarDomain { outer: region.outline.clone(), holes: region.holes.clone() };
        let opts = fem_core::mesh::MeshOpts { max_edge: region.mesh_size, min_angle_deg: 20.0 };
        let tri_mesh = fem_core::mesh::triangulate(&domain, &opts).map_err(|e| Fem2dError::MeshFailed { region_id: region.id.clone(), reason: e.to_string() })?;
        let node_ids = tri_mesh
            .points
            .iter()
            .enumerate()
            .map(|(point_index, p)| match doc.nodes.iter().find(|n| (n.x - p[0]).abs() < 1e-9 && (n.y - p[1]).abs() < 1e-9) {
                Some(n) => n.id.clone(),
                None => format!("{}_m{}", region.id, point_index),
            })
            .collect();
        out.push(RegionMesh { region_id: region.id.clone(), points: tri_mesh.points, tris: tri_mesh.tris, node_ids });
    }
    Ok(out)
}

/// 🎨 Nodal-averaged von Mises stress for `case_id`'s solved result (via `fem2d_solve_all`, so `case_id`
/// may name either a `FemLoadCase` or a `FemCombination`), keyed by node id — the document-layer bridge
/// to `fem_core::analyses::nodal_averaged_scalar`, feeding `fem-plugin`'s banded contour rendering.
pub fn fem2d_nodal_von_mises(doc: &Fem2dDocument, case_id: &str) -> Result<HashMap<String, f64>, Fem2dError> {
    let (nodes, elements, _regions) = build_nodes_and_elements(doc)?;
    let supports: Vec<Support> = doc.supports.iter().map(|s| Support { node_id: s.node_id.clone(), fixed: s.fixed.clone() }).collect();
    let model = fem_core::analyses::AnalysisModel { nodes, elements, supports };
    let results = fem2d_solve_all(doc)?;
    let result = results.get(case_id).ok_or_else(|| Fem2dError::LoadCaseNotFound(case_id.to_string()))?;
    Ok(fem_core::analyses::nodal_averaged_scalar(&model, result, fem_core::analyses::StressScalar::VonMises))
}
// #endregion 🔖MeshPreview
// #endregion 🔖Bridge

// #region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem2dDocumentVcs {
        store: RefCell<Fem2dStore>,
    }

    #[wasm_bindgen]
    impl Fem2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem2dStore::new(envelope)
                }
                None => Fem2dStore::new(create_document_vcs_envelope(FEM_2D_SCHEMA, "fem2d", empty_fem2d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
// #endregion 🔖WasmBridge

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::apply_operation;

    // #region 🔖Fixtures
    fn simply_supported_beam_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 6.0, y: 0.0 }],
            elements: vec![FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![Dof::Ty] }],
            load_cases: vec![FemLoadCase { id: "dead".into(), name: "dead".into(), loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }], self_weight: false }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }

    fn simply_supported_beam_two_span_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 3.0, y: 0.0 }, FemNode { id: "n3".into(), x: 6.0, y: 0.0 }],
            elements: vec![
                FemElement::Beam { id: "e1".into(), start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() },
                FemElement::Beam { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "ipe300".into() },
            ],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "ipe300".into(), name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] }, FemSupport { id: "s2".into(), node_id: "n3".into(), fixed: vec![Dof::Ty] }],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::MemberUdl { id: "l1".into(), element_id: "e1".into(), wx: 0.0, wy: -10000.0 }, FemLoad::MemberUdl { id: "l2".into(), element_id: "e2".into(), wx: 0.0, wy: -10000.0 }],
                self_weight: false,
            }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }

    fn truss_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "n1".into(), x: 0.0, y: 0.0 }, FemNode { id: "n2".into(), x: 4.0, y: 0.0 }, FemNode { id: "n3".into(), x: 4.0, y: 3.0 }],
            elements: vec![
                FemElement::Bar { id: "e1".into(), start: "n1".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
                FemElement::Bar { id: "e2".into(), start: "n2".into(), end: "n3".into(), material_id: "steel".into(), section_id: "rod".into() },
            ],
            regions: vec![],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![FemSection { id: "rod".into(), name: "rod".into(), area: 0.001, iy: 0.0 }],
            supports: vec![FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Tx, Dof::Ty] }, FemSupport { id: "s2".into(), node_id: "n2".into(), fixed: vec![Dof::Tx, Dof::Ty] }],
            load_cases: vec![FemLoadCase {
                id: "dead".into(),
                name: "dead".into(),
                loads: vec![FemLoad::Nodal { id: "l1".into(), node_id: "n3".into(), dof: Dof::Ty, value: -1000.0 }, FemLoad::Nodal { id: "l2".into(), node_id: "n3".into(), dof: Dof::Tx, value: -500.0 }],
                self_weight: false,
            }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }

    /// 🟩 A 4x2m rectangular region (steel, 0.02m thick, 1m mesh) whose 4 corners are pre-placed as
    /// document nodes (so `build_nodes_and_elements`'s exact-position reuse binds the mesh boundary
    /// to them) — 2 adjacent corners fully pinned, enough to remove all 3 in-plane rigid-body modes.
    fn rectangle_region_doc() -> Fem2dDocument {
        Fem2dDocument {
            nodes: vec![FemNode { id: "c0".into(), x: 0.0, y: 0.0 }, FemNode { id: "c1".into(), x: 4.0, y: 0.0 }, FemNode { id: "c2".into(), x: 4.0, y: 2.0 }, FemNode { id: "c3".into(), x: 0.0, y: 2.0 }],
            elements: vec![],
            regions: vec![FemRegion { id: "r1".into(), name: "slab".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], thickness: 0.02, material_id: "steel".into(), mesh_size: 1.0 }],
            materials: vec![FemMaterial { id: "steel".into(), name: "steel".into(), e: 210e9, nu: 0.3, rho: 7850.0 }],
            sections: vec![],
            supports: vec![FemSupport { id: "s1".into(), node_id: "c0".into(), fixed: vec![Dof::Tx, Dof::Ty] }, FemSupport { id: "s2".into(), node_id: "c1".into(), fixed: vec![Dof::Tx, Dof::Ty] }],
            load_cases: vec![FemLoadCase { id: "self".into(), name: "self weight".into(), loads: vec![], self_weight: true }],
            combinations: vec![],
            analysis: FemAnalysisSettings::default(),
            camera: FemCamera::default(),
        }
    }

    /// 🕳️ Same rectangle as `rectangle_region_doc` but with a small square hole near the center.
    fn rectangle_with_hole_region_doc() -> Fem2dDocument {
        let mut doc = rectangle_region_doc();
        doc.regions[0].holes = vec![vec![[1.5, 0.75], [2.5, 0.75], [2.5, 1.25], [1.5, 1.25]]];
        doc
    }
    // #endregion 🔖Fixtures

    // #region 🔖OpRoundTrip
    fn round_trip(projection: &Fem2dDocument, operation: &Fem2dOperation) -> Fem2dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
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
        round_trip(&base, &Fem2dOperation::SetElement { index: 0, element: updated });
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
        round_trip(&base, &Fem2dOperation::SetSupport { index: 0, support: FemSupport { id: "s1".into(), node_id: "n1".into(), fixed: vec![Dof::Ty] } });
        round_trip(&base, &Fem2dOperation::RemoveSupport { id: "s1".into() });
    }

    #[test]
    fn load_case_op_round_trips() {
        let base = simply_supported_beam_doc();
        round_trip(&base, &Fem2dOperation::SetLoadCase { index: 0, load_case: FemLoadCase { id: "dead".into(), name: "dead 2".into(), loads: vec![], self_weight: true } });
        round_trip(&base, &Fem2dOperation::RemoveLoadCase { id: "dead".into() });
    }

    #[test]
    fn camera_op_round_trips() {
        let base = simply_supported_beam_doc();
        let after = round_trip(&base, &Fem2dOperation::SetCamera { camera: FemCamera { x: 7.0, y: 8.0, zoom: 2.0 } });
        assert_eq!(after.camera.zoom, 2.0);
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
        base.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![("dead".into(), 1.35)] });
        let updated = FemCombination { id: "uls".into(), name: "ULS v2".into(), terms: vec![("dead".into(), 1.4)] };
        let after = round_trip(&base, &Fem2dOperation::SetCombination { index: 0, combination: updated });
        assert_eq!(after.combinations[0].terms[0].1, 1.4);
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

    #[test]
    fn document_diff_absorb_wins_over_granular_changes() {
        let base = simply_supported_beam_doc();
        let replacement = rectangle_region_doc();
        let mut diff = Fem2dOperation::SetCamera { camera: FemCamera { x: 1.0, y: 2.0, zoom: 3.0 } }.diff(&base);
        diff.absorb(Fem2dOperation::SetDocument { document: replacement.clone() }.diff(&base));
        assert_eq!(diff.apply(&base), replacement);
    }
    // #endregion 🔖OpRoundTrip

    // #region 🔖BuildModel
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
    // #endregion 🔖BuildModel

    // #region 🔖Regions
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
    // #endregion 🔖Regions

    // #region 🔖SelfWeight
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
    // #endregion 🔖SelfWeight

    // #region 🔖SolveAll
    #[test]
    fn fem2d_solve_all_returns_case_and_combination_results() {
        let mut doc = simply_supported_beam_doc();
        doc.load_cases.push(FemLoadCase { id: "live".into(), name: "live".into(), loads: vec![FemLoad::Nodal { id: "l2".into(), node_id: "n1".into(), dof: Dof::Ty, value: -2000.0 }], self_weight: false });
        doc.combinations.push(FemCombination { id: "uls".into(), name: "ULS".into(), terms: vec![("dead".into(), 1.35), ("live".into(), 1.5)] });

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
    // #endregion 🔖SolveAll

    // #region 🔖AnalyticalBenchmark
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
    // #endregion 🔖AnalyticalBenchmark

    // #region 🔖Truss
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
    // #endregion 🔖Truss

    // #region 🔖UnknownCase
    #[test]
    fn unknown_load_case_returns_descriptive_error() {
        let doc = simply_supported_beam_doc();
        let err = fem2d_solve(&doc, "missing").unwrap_err();
        assert!(err.contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖UnknownCase

    // #region 🔖ModalBuckling
    #[test]
    fn fem2d_modal_returns_requested_mode_count() {
        let doc = rectangle_region_doc();
        let result = fem2d_modal(&doc).expect("modal solves");
        assert_eq!(result.frequencies_hz.len(), doc.analysis.modal_count);
        for w in result.frequencies_hz.windows(2) {
            assert!(w[0] <= w[1], "frequencies should be ascending: {:?}", result.frequencies_hz);
        }
        for &f in &result.frequencies_hz {
            assert!(f.is_finite() && f >= 0.0, "frequency should be finite and non-negative: {f}");
        }
    }

    #[test]
    fn fem2d_modal_mode_values_returns_node_displacements() {
        let doc = simply_supported_beam_doc();
        let (freq, values) = fem2d_modal_mode_values(&doc, 0).expect("modal mode values solves");
        assert!(freq.is_finite() && freq >= 0.0);
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem2d_buckling_returns_requested_mode_count() {
        let doc = simply_supported_beam_doc();
        let result = fem2d_buckling(&doc, "dead").expect("buckling solves");
        assert_eq!(result.factors.len(), doc.analysis.buckling_count);
        for &f in &result.factors {
            assert!(f.is_finite(), "buckling factor should be finite: {f}");
        }
    }

    #[test]
    fn fem2d_buckling_mode_values_returns_node_displacements() {
        let doc = simply_supported_beam_doc();
        let (factor, values) = fem2d_buckling_mode_values(&doc, "dead", 0).expect("buckling mode values solves");
        assert!(factor.is_finite());
        assert!(values.contains_key("n1"));
        assert!(values.contains_key("n2"));
    }

    #[test]
    fn fem2d_buckling_unknown_case_errors() {
        let doc = simply_supported_beam_doc();
        let err = fem2d_buckling(&doc, "missing").err().expect("expected error");
        assert!(err.to_string().contains("load case not found"), "unexpected error: {err}");
    }
    // #endregion 🔖ModalBuckling

    // #region 🔖MeshPreview
    #[test]
    fn fem2d_mesh_preview_returns_region_triangles() {
        let doc = rectangle_region_doc();
        let meshes = fem2d_mesh_preview(&doc).expect("mesh preview succeeds");
        assert_eq!(meshes.len(), 1);
        assert_eq!(meshes[0].region_id, "r1");
        assert!(!meshes[0].tris.is_empty(), "expected at least one triangle");
        assert!(!meshes[0].points.is_empty(), "expected mesh points");
        assert_eq!(meshes[0].node_ids.len(), meshes[0].points.len(), "one node id per mesh point");
        // `rectangle_region_doc`'s 4 outline corners coincide with existing doc nodes `c0..c3`, so those
        // 4 mesh points must resolve to the doc's own node ids, not synthesized `r1_m*` ids.
        for corner_id in ["c0", "c1", "c2", "c3"] {
            assert!(meshes[0].node_ids.contains(&corner_id.to_string()), "expected corner {corner_id} to be reused, got {:?}", meshes[0].node_ids);
        }
    }
    // #endregion 🔖MeshPreview

    // #region 🔖NodalVonMises
    /// 🎨 `fem2d_nodal_von_mises` returns one value per mesh node, and a uniform area-pressure load
    /// (biaxial-ish but still smoothly varying membrane stress) produces FINITE values at every node —
    /// not a tight numeric benchmark (the region isn't a pure patch-test field), just a wiring check that
    /// the document-bridge correctly plumbs `fem_core::analyses::nodal_averaged_scalar`.
    #[test]
    fn fem2d_nodal_von_mises_returns_one_value_per_mesh_node() {
        let mut doc = rectangle_region_doc();
        doc.load_cases = vec![FemLoadCase { id: "pressure".into(), name: "pressure".into(), loads: vec![FemLoad::Area { id: "a1".into(), region_id: "r1".into(), pressure: 5000.0 }], self_weight: false }];
        let averaged = fem2d_nodal_von_mises(&doc, "pressure").expect("nodal von mises solves");
        let meshes = fem2d_mesh_preview(&doc).expect("mesh preview succeeds");
        for node_id in &meshes[0].node_ids {
            let v = *averaged.get(node_id).unwrap_or_else(|| panic!("missing averaged value for node {node_id}"));
            assert!(v.is_finite() && v >= 0.0, "node {node_id}: von mises {v} should be finite and non-negative");
        }
    }
    // #endregion 🔖NodalVonMises

    // #region 🔖ExampleFixture
    #[test]
    fn example_fixture_parses_and_solves() {
        let json = include_str!("../example/default.fem2d.json");
        let doc: Fem2dDocument = serde_json::from_str(json).expect("example fixture parses");
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

        let nodal_von_mises = fem2d_nodal_von_mises(&doc, "dead").expect("nodal von mises resolves");
        assert!(!nodal_von_mises.is_empty(), "the region carries a real area load in the dead case");

        let buckling = fem2d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
        assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
    }
    // #endregion 🔖ExampleFixture
}
// #endregion 🔖Tests
