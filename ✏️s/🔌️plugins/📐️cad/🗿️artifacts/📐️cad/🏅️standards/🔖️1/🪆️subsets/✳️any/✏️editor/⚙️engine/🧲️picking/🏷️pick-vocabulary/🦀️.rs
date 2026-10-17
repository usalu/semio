//! 🏷️ Vocabulary of the CAD pick engine: the pick kinds, targets, toggles, kernel kind mapping and hover keys. See [`super`].

use super::*;

//#region 🔖️Types
/// 🐁️ The two pointer gestures a pick event can carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialPickKind {
    PointerDown,
    PointerMove,
}

impl SpatialPickKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PointerDown => "pointer.down",
            Self::PointerMove => "pointer.move",
        }
    }
}

/// 🎯️ Renderer-facing pick kinds; kernel-private granularity rides on
/// [`SpatialPickTarget::geometry_kind`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpatialPickTargetKind {
    Object,
    Face,
    Edge,
    Vertex,
}

pub const SPATIAL_PICK_TARGET_KINDS: &[SpatialPickTargetKind] = &[SpatialPickTargetKind::Object, SpatialPickTargetKind::Face, SpatialPickTargetKind::Edge, SpatialPickTargetKind::Vertex];

impl SpatialPickTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::Face => "face",
            Self::Edge => "edge",
            Self::Vertex => "vertex",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        Some(match raw {
            "object" => Self::Object,
            "face" => Self::Face,
            "edge" => Self::Edge,
            "vertex" => Self::Vertex,
            _ => return None,
        })
    }

    /// 🥽️ Pick generality, coarsest first — the `CanvasPickMenu` ordering React sorts by.
    pub fn generality(self) -> u8 {
        match self {
            Self::Object => 0,
            Self::Face => 1,
            Self::Edge => 2,
            Self::Vertex => 3,
        }
    }
}

/// 🧲️ One snap/select target the editor offers for a kernel entity or a typology object row.
#[derive(Clone, Debug, PartialEq)]
pub struct SpatialPickTarget {
    pub kind: SpatialPickTargetKind,
    pub id: String,
    pub point: [f64; 3],
    pub points: Vec<[f64; 3]>,
    pub geometry_kind: Option<ModelEntityKind>,
    pub typology_id: Option<String>,
}

/// 🖱️ A committed selection row on the document (`{kind, id, editable}` in React).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectionTarget {
    pub kind: ModelEntityKind,
    pub id: String,
    pub editable: bool,
}

/// 👁️ Persisted hide/lock flags for one entity id.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpatialEntityFlags {
    pub hidden: bool,
    pub locked: bool,
}

impl SpatialEntityFlags {
    /// 👁️ `worldEntitySelectable` — a hidden or locked entity never takes a canvas pick.
    pub fn selectable(self) -> bool {
        !self.hidden && !self.locked
    }
}

/// 👁️ Per-kind on/off map for visibility filters or selection/hover gates (`Some(false)` disables;
/// `None` means "not set", which React spells as `!== false`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SpatialPickKindToggles {
    pub object: Option<bool>,
    pub face: Option<bool>,
    pub edge: Option<bool>,
    pub vertex: Option<bool>,
}

impl SpatialPickKindToggles {
    pub fn get(&self, kind: SpatialPickTargetKind) -> Option<bool> {
        match kind {
            SpatialPickTargetKind::Object => self.object,
            SpatialPickTargetKind::Face => self.face,
            SpatialPickTargetKind::Edge => self.edge,
            SpatialPickTargetKind::Vertex => self.vertex,
        }
    }

    pub fn set(&mut self, kind: SpatialPickTargetKind, value: Option<bool>) {
        match kind {
            SpatialPickTargetKind::Object => self.object = value,
            SpatialPickTargetKind::Face => self.face = value,
            SpatialPickTargetKind::Edge => self.edge = value,
            SpatialPickTargetKind::Vertex => self.vertex = value,
        }
    }

    /// 👁️ React's `toggles[kind] !== false`.
    pub fn enabled(&self, kind: SpatialPickTargetKind) -> bool {
        self.get(kind) != Some(false)
    }
}

/// 👁️ Per-typology on/off map for play chrome.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialTypologyToggles(pub BTreeMap<String, bool>);

impl SpatialTypologyToggles {
    pub fn enabled(&self, typology_id: &str) -> bool {
        self.0.get(typology_id) != Some(&false)
    }

    pub fn with(mut self, typology_id: impl Into<String>, value: bool) -> Self {
        self.0.insert(typology_id.into(), value);
        self
    }
}

/// 👁️ Per-primitive on/off map for play chrome (kernel entity kinds, not pick kinds).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialPrimitiveToggles(pub BTreeMap<ModelEntityKind, bool>);

impl SpatialPrimitiveToggles {
    pub fn enabled(&self, kind: ModelEntityKind) -> bool {
        self.0.get(&kind) != Some(&false)
    }

    pub fn with(mut self, kind: ModelEntityKind, value: bool) -> Self {
        self.0.insert(kind, value);
        self
    }
}

/// ☑️ Aggregate enabled state for a fixed-key boolean toggle group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialToggleGroupState {
    All,
    None,
    Partial,
}

/// ☑️ The owned tri-state checkbox value a toggle group maps onto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialToggleCheckboxState {
    Checked,
    Unchecked,
    Indeterminate,
}

/// 👁️ Which scene layers stay visible for geometry edit vs typology object picking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpatialSceneVisibility {
    pub show_factory_wireframe: bool,
    pub show_committed_faces: bool,
    pub show_committed_edges: bool,
}
//#endregion 🔖️Types

//#region 🔖️KindMapping
/// 🧭️ Kernel entity kind → the coarser pick kind that represents it on the canvas.
pub fn geometry_kind_to_object_pick(kind: ModelEntityKind) -> Option<SpatialPickTargetKind> {
    Some(match kind {
        ModelEntityKind::Vertex | ModelEntityKind::Anchor => SpatialPickTargetKind::Vertex,
        ModelEntityKind::Edge | ModelEntityKind::Wire => SpatialPickTargetKind::Edge,
        ModelEntityKind::Face | ModelEntityKind::Shell => SpatialPickTargetKind::Face,
        ModelEntityKind::Solid => SpatialPickTargetKind::Object,
        _ => return None,
    })
}

/// 🧭️ Pick kind → the kernel entity kind a snap/selection payload must name.
pub fn kernel_geometry_kind_for_object_pick(kind: SpatialPickTargetKind, geometry_kind: Option<ModelEntityKind>) -> ModelEntityKind {
    if let Some(kind) = geometry_kind {
        return kind;
    }
    match kind {
        SpatialPickTargetKind::Vertex => ModelEntityKind::Vertex,
        SpatialPickTargetKind::Edge => ModelEntityKind::Edge,
        SpatialPickTargetKind::Face => ModelEntityKind::Face,
        SpatialPickTargetKind::Object => ModelEntityKind::Solid,
    }
}

/// 🎯️ Pick kinds an interaction's `selection.accept` list admits, or `None` for "accepts anything".
pub fn spatial_pick_kinds_for_selection_accept(accept: &[ModelEntityKind]) -> Option<BTreeSet<SpatialPickTargetKind>> {
    if accept.is_empty() {
        return None;
    }
    let mut out = BTreeSet::new();
    for kind in accept {
        if let Some(direct) = match kind {
            ModelEntityKind::Object => Some(SpatialPickTargetKind::Object),
            ModelEntityKind::Face => Some(SpatialPickTargetKind::Face),
            ModelEntityKind::Edge => Some(SpatialPickTargetKind::Edge),
            ModelEntityKind::Vertex => Some(SpatialPickTargetKind::Vertex),
            _ => None,
        } {
            out.insert(direct);
            continue;
        }
        if let Some(mapped) = geometry_kind_to_object_pick(*kind) {
            out.insert(mapped);
        }
    }
    Some(out)
}

/// 🧭️ The primitive entity kind of a pick target (typology object rows have none).
pub fn pick_target_primitive_kind(target: &SpatialPickTarget) -> Option<ModelEntityKind> {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return None;
    }
    Some(kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind))
}
//#endregion 🔖️KindMapping

//#region 🔖️Keys
/// 🪪️ Stable `kind:id` key for a pick target.
pub fn spatial_pick_target_key(target: &SpatialPickTarget) -> String {
    format!("{}:{}", target.kind.as_str(), target.id)
}

/// 🪪️ Stable hover/selection key for a committed selection row.
pub fn selection_target_hover_key(target: &SelectionTarget) -> String {
    format!("{}:{}", target.kind.as_str(), target.id)
}

/// 🪪️ Stable kernel-geometry member key for object-reveal lookup.
pub fn spatial_pick_target_member_key(target: &SpatialPickTarget) -> String {
    if target.kind == SpatialPickTargetKind::Object && target.geometry_kind.is_none() {
        return format!("object:{}", target.id);
    }
    format!("{}:{}", kernel_geometry_kind_for_object_pick(target.kind, target.geometry_kind).as_str(), target.id)
}

pub(super) fn pinned_pick_target_keys<'a>(keys: impl IntoIterator<Item = &'a str>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for key in keys {
        out.insert(key.to_string());
        let Some(colon) = key.find(':') else { continue };
        let (kind, id) = (&key[..colon], &key[colon + 1..]);
        if let Some(mapped) = ModelEntityKind::parse(kind).and_then(geometry_kind_to_object_pick) {
            out.insert(format!("{}:{id}", mapped.as_str()));
        }
        if kind == "object" {
            out.insert(format!("solid:{id}"));
        }
    }
    out
}

/// 🪪️ Expands a hover/selection key across geometry pick aliases (`solid:foo` ↔ `object:foo`).
pub fn spatial_hover_key_aliases(key: Option<&str>) -> BTreeSet<String> {
    match key.filter(|key| !key.is_empty()) {
        None => BTreeSet::new(),
        Some(key) => pinned_pick_target_keys([key]),
    }
}

/// 🪪️ True when two hover/selection keys name the same thing, aliases included.
pub fn spatial_hover_keys_match(left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left.filter(|k| !k.is_empty()), right.filter(|k| !k.is_empty())) else {
        return false;
    };
    left == right || spatial_hover_key_aliases(Some(left)).contains(right)
}
//#endregion 🔖️Keys
