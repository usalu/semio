//! 🪪️ Session constants and owner catalog: the session capacities and limits, the mounted owner class and claim and the process owner catalog. See [`super`].

use super::*;

//#region 🔖️Contract
pub const FEM2D_MOUNTED_JOB_KIND: &str = "semio.fem2d.mounted-analysis";

pub(super) const SESSION_ACTIVE_CAPACITY: usize = 32;
pub(super) const SESSION_CONSUMER_CAPACITY: usize = 128;
pub(super) type RequestKey = semio_framework_value::ContentRequestKey;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsumerKey { Model, Window(semio_framework_plugin::WindowConfigAuthorityIdentity) }

pub(super) const SESSION_SHELL_CAPACITY: usize = 64;

pub(super) const SESSION_MAXIMUM_INPUT_ITEMS: usize = 4_096;

pub(super) const SESSION_MAXIMUM_INPUT_BYTES: usize = 4 * 1_024 * 1_024;

/// 📏️ Shared paged model admission covers the complete accumulated document and region geometry.
pub(super) const SESSION_MAXIMUM_NODES: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_NODE_SLOTS;

pub(super) const SESSION_MAXIMUM_ELEMENTS: usize = semio_s_fem_analyses_model::MOUNTED_ANALYSIS_ELEMENT_SLOTS;

pub(super) const SESSION_MAXIMUM_SUPPORTS: usize = 64;

pub(super) const SESSION_MAXIMUM_MESH_POINTS: usize = 80;

pub(super) const SESSION_MAXIMUM_MESH_TRIANGLES: usize = 40;

pub(super) const SESSION_MAXIMUM_REGION_HOLES: usize = 16;

pub(super) const SESSION_MAXIMUM_BOUNDARY_POINTS: usize = 64;

pub(super) const SESSION_MAXIMUM_OUTPUT_BYTES: usize = 16 * 1_024;

pub(super) const SESSION_MAXIMUM_VISUAL_LOADS: usize = 64;

pub(super) const SESSION_MAXIMUM_FAULT_BYTES: usize = 4_096;

pub(super) const SESSION_OWNER_PAGE_BYTES: usize = 16_384;

pub(super) const SESSION_MAXIMUM_STRING_BYTES: usize = SESSION_OWNER_PAGE_BYTES;

/// 🧮️ Census units one snapshot-read opportunity may charge before it yields to the next refresh —
/// every unit is one owner or scalar of constant cost, so a whole demo-sized census (a few hundred
/// units) admits in the opportunity that opened it instead of waiting one host refresh per unit.
pub(super) const SESSION_PREFLIGHT_UNITS_PER_OPPORTUNITY: usize = 4_096;

/// ⏱️ Wall-clock ceiling of one host job step (the interactive 8 ms contract) and the most session
/// units the job bridge drives inside it — each unit is one exact stage step, so a bounded drive
/// replaces one host round trip per unit with one per ceiling.
pub(super) const SESSION_STEP_CEILING_MS: u64 = 8;

pub(super) const SESSION_UNITS_PER_STEP: usize = 65_536;

pub(super) const INPUT_BYTES: usize = 63;

pub(super) const FEM2D_JOB_TAG: u64 = 0xf2d0_0000_0000_0000;

pub(super) const FEM2D_JOB_COUNTER_MAXIMUM: u64 = 0x000f_ffff_ffff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MountedOwnerClass {
    GraphPlans,
    DomainOuter,
    DomainHoleList,
    DomainHoleVectors,
    ModelNodes,
    ModelElements,
    ModelSupports,
    ModelRegionNodeIds,
    ModelStrings,
    SupportDofs,
    MeshPreparationVectors,
    MeshPreparationIndexVector,
    MeshPreparedVectors,
    MeshTriangulationVectors,
    MeshTriangulationWorkspaceVectors,
    MeshConstraintVector,
    MeshEdgeIndexVectors,
    MeshOutputVectors,
    AssemblyDofOrderVector,
    AssemblyDofStrings,
    AssemblyPlanVectors,
    AssemblyPartitionVectors,
    AssemblyMergeVectors,
    AssemblyPendingVectors,
    CsrVectors,
    PcgVectors,
    VisualVectors,
    VisualStrings,
    OutputPages,
    FaultPages,
}

#[derive(Clone, Copy)]
struct MountedOwnerClaim {
    pub(super) class: MountedOwnerClass,
    pub(super) roots: usize,
    pub(super) items: usize,
}

#[derive(Clone, Copy)]
pub(super) struct MountedProcessOwnerCatalog {
    pub(super) claims: [MountedOwnerClaim; 30],
}

impl MountedProcessOwnerCatalog {
    pub(super) const fn fixed() -> Self {
        Self {
            claims: [
                MountedOwnerClaim { class: MountedOwnerClass::GraphPlans, roots: 1, items: 8 },
                MountedOwnerClaim { class: MountedOwnerClass::DomainOuter, roots: 1, items: SESSION_MAXIMUM_BOUNDARY_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::DomainHoleList, roots: 1, items: SESSION_MAXIMUM_REGION_HOLES },
                MountedOwnerClaim { class: MountedOwnerClass::DomainHoleVectors, roots: SESSION_MAXIMUM_REGION_HOLES, items: SESSION_MAXIMUM_REGION_HOLES * SESSION_MAXIMUM_BOUNDARY_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelNodes, roots: (SESSION_MAXIMUM_NODES * size_of::<semio_s_fem_model::Node>() + 4095) / 4096, items: SESSION_MAXIMUM_NODES },
                MountedOwnerClaim { class: MountedOwnerClass::ModelElements, roots: (SESSION_MAXIMUM_ELEMENTS * size_of::<semio_s_fem_static::Elements>() + 4095) / 4096, items: SESSION_MAXIMUM_ELEMENTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelSupports, roots: (SESSION_MAXIMUM_SUPPORTS * size_of::<MountedAnalysisSupport>() + 4095) / 4096, items: SESSION_MAXIMUM_SUPPORTS },
                MountedOwnerClaim { class: MountedOwnerClass::ModelRegionNodeIds, roots: 1, items: SESSION_MAXIMUM_MESH_POINTS },
                MountedOwnerClaim {
                    class: MountedOwnerClass::ModelStrings,
                    roots: 2 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) + 3 * SESSION_MAXIMUM_ELEMENTS + 4 * SESSION_MAXIMUM_MESH_TRIANGLES + SESSION_MAXIMUM_SUPPORTS + 2,
                    items: 0,
                },
                MountedOwnerClaim { class: MountedOwnerClass::SupportDofs, roots: SESSION_MAXIMUM_SUPPORTS, items: 6 * SESSION_MAXIMUM_SUPPORTS },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparationVectors, roots: 2, items: SESSION_MAXIMUM_MESH_POINTS + 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparationIndexVector, roots: 1, items: SESSION_MAXIMUM_MESH_POINTS },
                MountedOwnerClaim { class: MountedOwnerClass::MeshPreparedVectors, roots: 2, items: SESSION_MAXIMUM_MESH_POINTS + 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshTriangulationVectors, roots: 3, items: SESSION_MAXIMUM_MESH_POINTS + 2 * (4 * SESSION_MAXIMUM_MESH_TRIANGLES + 1) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshTriangulationWorkspaceVectors, roots: 3, items: 3 * (12 * SESSION_MAXIMUM_MESH_TRIANGLES + 3) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshConstraintVector, roots: 1, items: 3 * SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::MeshEdgeIndexVectors, roots: 3, items: 3 * SESSION_MAXIMUM_MESH_TRIANGLES + 2 * (12 * SESSION_MAXIMUM_MESH_TRIANGLES + 3) },
                MountedOwnerClaim { class: MountedOwnerClass::MeshOutputVectors, roots: 3, items: 2 * SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_MESH_TRIANGLES },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyDofOrderVector, roots: 1, items: 3 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyDofStrings, roots: 3 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS), items: 0 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPlanVectors, roots: 4, items: 12 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS) },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPartitionVectors, roots: 3, items: 1 + 2 * 144 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyMergeVectors, roots: 4, items: 2 + 2 * 144 },
                MountedOwnerClaim { class: MountedOwnerClass::AssemblyPendingVectors, roots: 3, items: 6 + 3 + 36 },
                MountedOwnerClaim { class: MountedOwnerClass::CsrVectors, roots: 5, items: 512 },
                MountedOwnerClaim { class: MountedOwnerClass::PcgVectors, roots: 10, items: 512 },
                MountedOwnerClaim {
                    class: MountedOwnerClass::VisualVectors,
                    roots: 5,
                    items: 2 * (SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES)
                        + 64
                        + SESSION_MAXIMUM_ELEMENTS
                        + SESSION_MAXIMUM_MESH_TRIANGLES
                        + SESSION_MAXIMUM_NODES
                        + SESSION_MAXIMUM_MESH_POINTS,
                },
                MountedOwnerClaim { class: MountedOwnerClass::VisualStrings, roots: 64 + SESSION_MAXIMUM_NODES + SESSION_MAXIMUM_MESH_POINTS + SESSION_MAXIMUM_ELEMENTS + SESSION_MAXIMUM_MESH_TRIANGLES + 1, items: 0 },
                MountedOwnerClaim { class: MountedOwnerClass::OutputPages, roots: 3 * semio_framework_ui_scene::CANVAS2D_SNAPSHOT_PAGE_CAPACITY + 3 * (SESSION_MAXIMUM_OUTPUT_BYTES / SESSION_OWNER_PAGE_BYTES), items: 6 },
                MountedOwnerClaim { class: MountedOwnerClass::FaultPages, roots: 2, items: 0 },
            ],
        }
    }

    pub(super) const fn roots(self) -> usize {
        let mut roots = 0;
        let mut index = 0;
        while index < self.claims.len() {
            assert!(self.claims[index].class as usize == index, "owner catalog class order");
            roots += self.claims[index].roots;
            index += 1;
        }
        roots
    }

    pub(super) const fn items(self) -> usize {
        let mut items = 0;
        let mut index = 0;
        while index < self.claims.len() {
            items += self.claims[index].items;
            index += 1;
        }
        items
    }

    pub(super) fn credit(self, input_items: usize, input_bytes: usize) -> Result<(usize, usize), &'static [u8]> {
        if input_items > SESSION_MAXIMUM_INPUT_ITEMS || input_bytes > SESSION_MAXIMUM_INPUT_BYTES {
            return Err(b"fem2d.session-process-input-credit-exceeded");
        }
        let pages = self.roots();
        let items = input_items.checked_add(self.items()).and_then(|items| items.checked_add(pages)).ok_or(b"fem2d.session-process-item-overflow" as &'static [u8])?;
        let bytes = input_bytes.checked_add(pages.checked_mul(SESSION_OWNER_PAGE_BYTES).ok_or(b"fem2d.session-process-page-byte-overflow" as &'static [u8])?).ok_or(b"fem2d.session-process-byte-overflow" as &'static [u8])?;
        if items > SESSION_MAXIMUM_ITEMS || bytes > SESSION_MAXIMUM_BYTES {
            return Err(b"fem2d.session-process-admission-exceeded");
        }
        Ok((items, bytes))
    }
}
//#endregion 🔖️Contract
