//! 🎨️ Visual state and quality: the window constants, the region visual quality, the node live field, the fem visual state, the region quality slots and the live visual. See [`super`].

use super::*;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "fem2d-model";

pub const BODY_KEY: &str = "fem2d.play.model";

/// 📐️ Exaggeration factor for offsetting the moment-diagram polyline perpendicular to a member — single
/// consumer: the results window's static-results moment diagram
/// (`crate::editor::fem2d::modes::edit::windows::results::render`).
pub const MOMENT_SCALE_2D: f64 = 0.001;

/// 🎨️ Muted color for the mesh-edge preview overlay drawn under this window's members.
pub(super) const MESH_EDGE_COLOR: &str = "#475569";
//#endregion 🔖️Constants

//#region 👁️LiveVisualLanguage
/// 🎨️ Stable region-quality vocabulary shared by mesh-job previews and the 2D canvas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RegionVisualQuality {
    #[default]
    Unmeshed,
    Coarse,
    Refined,
    Final,
}

impl RegionVisualQuality {
    pub(crate) fn color(self) -> &'static str {
        match self {
            Self::Unmeshed => "#64748b",
            Self::Coarse => "#f59e0b",
            Self::Refined => "#38bdf8",
            Self::Final => "#22c55e",
        }
    }

    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Unmeshed => "unmeshed",
            Self::Coarse => "coarse",
            Self::Refined => "refined",
            Self::Final => "final",
        }
    }
}

/// 📈️ One node's replaceable iterative-solver field sample in model coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeLiveField {
    pub node_id: String,
    pub displacement: [f64; 2],
    pub residual: [f64; 2],
    pub reaction: [f64; 2],
    pub contour: f64,
    pub mode_shape: [f64; 2],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FemVisualState {
    #[default]
    Unmeshed,
    Coarse,
    Refined,
    Assembling,
    SolvingUnconverged,
    SolvingConverged,
    ValidatedFinal,
    FaultedCancelled,
}

impl FemVisualState {
    pub(super) fn id(self) -> &'static str {
        match self {
            Self::Unmeshed => "unmeshed",
            Self::Coarse => "coarse-mesh",
            Self::Refined => "refined-mesh",
            Self::Assembling => "assembling",
            Self::SolvingUnconverged => "solving-unconverged",
            Self::SolvingConverged => "solving-converged",
            Self::ValidatedFinal => "validated-final",
            Self::FaultedCancelled => "faulted-cancelled-last-valid",
        }
    }
}

/// 👁️ Replaceable, non-authoritative progress consumed by the 2D surface.
pub(super) const FEM2D_LIVE_REGION_CAPACITY: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Fem2dRegionQualitySlots {
    pub(super) slots: [Option<(String, RegionVisualQuality)>; FEM2D_LIVE_REGION_CAPACITY],
}

impl Fem2dRegionQualitySlots {
    pub fn insert(&mut self, id: String, quality: RegionVisualQuality) -> Result<Option<RegionVisualQuality>, String> {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.as_ref().is_some_and(|(current, _)| *current == id)) {
            let previous = slot.as_mut().map(|(_, current)| std::mem::replace(current, quality));
            return Ok(previous);
        }
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(id) };
        *slot = Some((id, quality));
        Ok(None)
    }

    pub fn get(&self, id: &str) -> Option<&RegionVisualQuality> {
        self.slots.iter().find_map(|slot| slot.as_ref().filter(|(current, _)| current == id).map(|(_, quality)| quality))
    }

    pub fn update(&mut self, id: &str, quality: RegionVisualQuality) -> bool {
        let Some((_, current)) = self.slots.iter_mut().find_map(|slot| slot.as_mut().filter(|(current, _)| current == id)) else { return false };
        *current = quality;
        true
    }

    pub fn take_one(&mut self) -> Option<(String, RegionVisualQuality)> {
        self.slots.iter_mut().find(|slot| slot.is_some()).and_then(Option::take)
    }

    pub fn is_empty(&self) -> bool {
        self.slots.iter().all(Option::is_none)
    }

    pub(crate) fn slot(&self, index: usize) -> Option<&(String, RegionVisualQuality)> {
        self.slots.get(index).and_then(Option::as_ref)
    }
}

impl Default for Fem2dRegionQualitySlots {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }
}

impl FromIterator<(String, RegionVisualQuality)> for Fem2dRegionQualitySlots {
    fn from_iter<T: IntoIterator<Item = (String, RegionVisualQuality)>>(iter: T) -> Self {
        let mut slots = Self::default();
        for (id, quality) in iter.into_iter().take(FEM2D_LIVE_REGION_CAPACITY) {
            let _ = slots.insert(id, quality);
        }
        slots
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fem2dLiveVisual {
    pub region_quality: Fem2dRegionQualitySlots,
    pub assembling_element_ids: Vec<String>,
    pub fields: Vec<NodeLiveField>,
    pub state: FemVisualState,
    pub residual_norm: f64,
    pub tolerance: f64,
    pub progress_completed: usize,
    pub progress_total: usize,
    pub converged: bool,
    pub validated_final: bool,
}
//#endregion 👁️LiveVisualLanguage
