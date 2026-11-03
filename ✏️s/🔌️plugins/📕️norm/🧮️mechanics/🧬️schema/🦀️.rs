//! 🧬️ Mechanics schema: SI units, global Z up, local x along the member, six DOF per node `[ux, uy, uz, rx, ry, rz]`.

pub type Dof6 = [f64; 6];

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Node {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Material {
    pub id: String,
    pub e: f64,
    pub g: f64,
    pub rho: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Section {
    pub id: String,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
    pub iw: f64,
    pub asy: f64,
    pub asz: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Member {
    pub id: String,
    pub node_i: String,
    pub node_j: String,
    pub material: String,
    pub section: String,
    pub roll: f64,
    pub release_i: [bool; 6],
    pub release_j: [bool; 6],
    pub shear_deformable: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Support {
    pub node: String,
    pub fixed: [bool; 6],
    pub spring: Dof6,
    pub prescribed: Dof6,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NodalLoad {
    pub node: String,
    pub force: Dof6,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LoadFrame {
    #[default]
    Global,
    Local,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DistributedLoad {
    pub member: String,
    pub frame: LoadFrame,
    pub start: [f64; 3],
    pub end: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TemperatureLoad {
    pub member: String,
    pub alpha: f64,
    pub uniform: f64,
    pub gradient_y: f64,
    pub gradient_z: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LoadCase {
    pub id: String,
    pub nodal: Vec<NodalLoad>,
    pub distributed: Vec<DistributedLoad>,
    pub temperature: Vec<TemperatureLoad>,
    pub acceleration: [f64; 3],
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NodalMass {
    pub node: String,
    pub mass: Dof6,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub materials: Vec<Material>,
    pub sections: Vec<Section>,
    pub members: Vec<Member>,
    pub supports: Vec<Support>,
    pub load_cases: Vec<LoadCase>,
    pub masses: Vec<NodalMass>,
    pub self_mass: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NodeDisplacement {
    pub node: String,
    pub u: Dof6,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Reaction {
    pub node: String,
    pub force: Dof6,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DiagramPoint {
    pub x: f64,
    pub n: f64,
    pub vy: f64,
    pub vz: f64,
    pub t: f64,
    pub my: f64,
    pub mz: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MemberForces {
    pub member: String,
    pub length: f64,
    pub end_i: Dof6,
    pub end_j: Dof6,
    pub diagram: Vec<DiagramPoint>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StaticResult {
    pub case: String,
    pub displacements: Vec<NodeDisplacement>,
    pub reactions: Vec<Reaction>,
    pub members: Vec<MemberForces>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BucklingMode {
    pub alpha_cr: f64,
    pub shape: Vec<NodeDisplacement>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BucklingResult {
    pub case: String,
    pub modes: Vec<BucklingMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MassType {
    #[default]
    Consistent,
    Lumped,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ModalMode {
    pub omega: f64,
    pub frequency_hz: f64,
    pub participation: [f64; 3],
    pub effective_mass: [f64; 3],
    pub effective_mass_ratio: [f64; 3],
    pub shape: Vec<NodeDisplacement>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ModalResult {
    pub total_mass: [f64; 3],
    pub modes: Vec<ModalMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LtbEnd {
    pub lateral: bool,
    pub twist: bool,
    pub minor_rotation: bool,
    pub warping: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LtbRestraint {
    pub x: f64,
    pub lateral: bool,
    pub twist: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LtbMember {
    pub length: f64,
    pub e: f64,
    pub g: f64,
    pub iz: f64,
    pub iw: f64,
    pub j: f64,
    pub polar_radius_sq: f64,
    pub wagner: f64,
    pub axial: f64,
    pub end_i: LtbEnd,
    pub end_j: LtbEnd,
    pub restraints: Vec<LtbRestraint>,
    pub moment: Vec<(f64, f64)>,
    pub line_load: Vec<(f64, f64)>,
    pub point_loads: Vec<(f64, f64)>,
    pub load_height: f64,
    pub elements: usize,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LtbShapePoint {
    pub x: f64,
    pub lateral: f64,
    pub twist: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct LtbResult {
    pub alpha_cr: f64,
    pub mcr: f64,
    pub shape: Vec<LtbShapePoint>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionMethod {
    FiniteElement,
    ThinWalled,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SectionOptions {
    pub cells_across_wall: f64,
    pub max_edge: f64,
    pub max_triangles: usize,
}

impl SectionOptions {
    /// 🧱️ The documented mesh budget of the torsion solver: four cells across the wall, no edge length cap (`max_edge` 0), at most 40 000 triangles. Named at every call site, there is no implicit `Default`.
    pub fn standard() -> Self {
        Self { cells_across_wall: 4.0, max_edge: 0.0, max_triangles: 40_000 }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ThinWall {
    pub y1: f64,
    pub z1: f64,
    pub y2: f64,
    pub z2: f64,
    pub thickness: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SectionConstants {
    pub area: f64,
    pub centroid_y: f64,
    pub centroid_z: f64,
    pub iy: f64,
    pub iz: f64,
    pub iyz: f64,
    pub i_major: f64,
    pub i_minor: f64,
    pub j: f64,
    pub iw: f64,
    pub shear_centre_y: f64,
    pub shear_centre_z: f64,
    pub a_vy: f64,
    pub a_vz: f64,
    pub w_el_y_pos: f64,
    pub w_el_y_neg: f64,
    pub w_el_z_pos: f64,
    pub w_el_z_neg: f64,
    pub w_pl_y: f64,
    pub w_pl_z: f64,
    pub poisson: f64,
    pub method: SectionMethod,
    pub triangles: usize,
    pub nodes: usize,
    pub max_edge: f64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Progress {
    pub stage: String,
    pub fraction: f64,
}

/// 🌐️ A mechanics message in both languages; neither is the default.
#[derive(Clone, Debug, PartialEq)]
pub struct MechanicsText {
    pub en: String,
    pub de: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MechanicsError {
    Cancelled,
    Invalid(MechanicsText),
    Singular { dof: usize },
    Unstable { node: String, dof: usize },
    NoPositiveEigenvalue,
    NoConvergence,
}

impl MechanicsError {
    /// 🚫️ An invalid model or input with its explanation in both languages.
    pub fn invalid(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self::Invalid(MechanicsText { en: en.into(), de: de.into() })
    }
}

impl std::fmt::Display for MechanicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for MechanicsError {}

pub type MechanicsResult<T> = Result<T, MechanicsError>;
