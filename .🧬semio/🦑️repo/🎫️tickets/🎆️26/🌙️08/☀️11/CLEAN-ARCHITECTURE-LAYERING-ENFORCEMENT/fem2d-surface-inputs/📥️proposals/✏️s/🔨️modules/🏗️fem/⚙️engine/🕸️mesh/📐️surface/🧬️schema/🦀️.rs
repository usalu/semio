//! 📐️ Binary64 region identity and indexed volume/surface records.

use super::super::{MeshOpts, PlanarDomain};

/// 🪪️ One explicit region input transferred by its domain owner.
#[derive(Clone, Debug, PartialEq)]
pub struct RegionSurfaceInputV1 {
    pub region_id: String,
    pub domain: PlanarDomain,
    pub thickness: f64,
    pub options: MeshOpts,
}

/// 🧱️ A region's binary64 positions, volume tetrahedra and outward boundary triangles.
#[derive(Clone, Debug, PartialEq)]
pub struct RegionVolumeSurfaceV1 {
    pub region_id: String,
    pub points: Vec<[f64; 3]>,
    pub tetrahedra: Vec<[u32; 4]>,
    pub triangles: Vec<[u32; 3]>,
}

/// 📦️ Domain-ordered successfully meshed regions; an empty inventory is complete geometry.
#[derive(Clone, Debug, PartialEq)]
pub struct RegionVolumeSurfaceBatchV1 {
    pub regions: Vec<RegionVolumeSurfaceV1>,
}

/// 🚧️ Explicit surface input and operation refusals remain distinct from failed triangulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionSurfaceFaultV1 {
    RegionIdentity,
    NonFiniteCoordinate,
    NonFiniteThickness,
    NonFiniteOptions,
    DomainCapacity,
    IndexCapacity,
    OwnershipAuthority,
    AllocationRefused,
    OperationIdentity,
}
