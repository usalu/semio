//! 🫧️ Lane payload schema: drag preview, ghost preview, evaluation progress and peer presence; document truth never lives here.

use semio_framework_value::{DslValue, Number};
use semio_s_plugin_norm_structural_scene::{EntityKind, EntityRef, Handle, Vec3};

macro_rules! records {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

macro_rules! variants {
    ($($item:item)*) => {$(
        #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
        #[value(tag = "kind", content = "data", rename_all = "camelCase", rename_all_fields = "camelCase")]
        $item
    )*};
}

macro_rules! choices {
    ($($item:item)*) => {$(
        #[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
        #[value(rename_all = "camelCase")]
        $item
    )*};
}

records! {
    /// ✍️ One provisional field value, addressed by the same path syntax as `setField`.
    pub struct FieldWrite {
        pub path: String,
        pub value: DslValue,
    }

    /// ✋️ An open handle drag, previewed by every window and broadcast on presence; zero trace when aborted.
    pub struct DragPreview {
        pub handle: String,
        pub writes: Vec<FieldWrite>,
        pub started_seq: u64,
        pub origin: f64,
        pub s: f64,
        pub entry: String,
    }

    /// 👻️ A provisional what-if shown as a ghost: a remedy under the pointer or a hovered alternative.
    pub struct GhostPreview {
        pub writes: Vec<FieldWrite>,
        pub source: GhostSource,
    }

    /// ⏳️ Progress of the incremental evaluation; `running` holds check-unit ids.
    pub struct EvalProgress {
        pub running: Vec<String>,
        pub done: u32,
        pub total: u32,
        pub cancelled: bool,
    }

    /// 🫧️ Ephemeral local-only state of one editor instance.
    pub struct NormTransient {
        pub drag: Option<DragPreview>,
        pub ghost: Option<GhostPreview>,
        pub eval: EvalProgress,
        pub peers: Vec<PeerDrag>,
        pub view: ViewLane,
    }

    /// 🎞️ Ephemeral local-only view of one viewport: the chosen colour field, the selected parameter track with its value (`None` is the track start), the play flag, the animation clock in seconds and the playback speed the unit system of the inspector (`si` or `engineering`, `None` is engineering), and whether the touch Edit menu is open, whether the camera frames the selection (a double tap toggles it) the selected combination or variant index (`None` follows the governing one) the check index a stepping shortcut selected (`None` follows the stored selection) and the traffic group index (`None` follows the active combination).
    pub struct ViewLane {
        pub active_field: Option<String>,
        pub track: Option<String>,
        pub value: Option<f64>,
        pub playing: bool,
        pub clock: f64,
        pub speed: f64,
        pub units: Option<String>,
        pub item: Option<u32>,
        pub check: Option<u32>,
        pub group: Option<u32>,
        pub menu: bool,
        pub frame: bool,
    }

    /// 🖐️ The open drag of another user, mirrored from the shared presence for painting.
    pub struct PeerDrag {
        pub user: String,
        pub drag: DragPreview,
    }

    /// 🎥️ A peer's camera, shown as a ghost frustum.
    pub struct CameraPose {
        pub eye: Vec3,
        pub target: Vec3,
        pub up: Vec3,
        pub fov_y: f64,
    }

    /// 👥️ Ephemeral shared state of one user: selection, hover, camera ghost and drag preview.
    pub struct NormPresence {
        pub user: String,
        pub selection: Vec<EntityRef>,
        pub hover: Option<EntityRef>,
        pub camera: Option<CameraPose>,
        pub drag: Option<DragPreview>,
    }
}

choices! {
    /// 🧭️ The four persistence and sharing lanes of the editors.
    pub enum Lane {
        PersistedShared,
        PersistedLocal,
        EphemeralShared,
        EphemeralLocal,
    }
}

variants! {
    /// 🎯️ What caused a ghost preview.
    pub enum GhostSource {
        Remedy { check_id: String, index: u32 },
        Hover,
        Insert { spec: String, kind: EntityKind, at: Vec3 },
    }
}

impl Default for EvalProgress {
    fn default() -> Self {
        Self { running: Vec::new(), done: 0, total: 0, cancelled: false }
    }
}

impl Default for NormTransient {
    fn default() -> Self {
        Self { drag: None, ghost: None, eval: EvalProgress::default(), peers: Vec::new(), view: ViewLane::default() }
    }
}

impl Default for ViewLane {
    fn default() -> Self {
        Self { active_field: None, track: None, value: None, playing: false, clock: 0.0, speed: 1.0, units: None, item: None, check: None, group: None, menu: false, frame: false }
    }
}

impl FieldWrite {
    pub fn number(path: impl Into<String>, value: f64) -> Self {
        Self { path: path.into(), value: DslValue::Number(Number::Float(value)) }
    }
}

impl DragPreview {
    /// 🕹️ Preview of `handle` dragged to parameter `s`, with every bound field value resolved.
    pub fn at(handle: &Handle, s: f64, started_seq: u64) -> Self {
        Self { handle: handle.id.clone(), writes: handle.writes_at(s).into_iter().map(|w| match w.token {
            Some(token) => FieldWrite { path: w.path, value: DslValue::String(token) },
            None => FieldWrite::number(w.path, w.value),
        }).collect(), started_seq, origin: s, s, entry: String::new() }
    }
}

impl EvalProgress {
    /// 📊️ Completed share in `0..=1`; an empty plan counts as done.
    pub fn fraction(&self) -> f64 {
        if self.total == 0 { 1.0 } else { f64::from(self.done.min(self.total)) / f64::from(self.total) }
    }

    pub fn is_idle(&self) -> bool {
        self.running.is_empty()
    }
}

impl NormTransient {
    pub fn with_view(self, view: ViewLane) -> Self {
        Self { view, ..self }
    }

    pub fn with_drag(self, drag: Option<DragPreview>) -> Self {
        Self { drag, ..self }
    }

    pub fn with_ghost(self, ghost: Option<GhostPreview>) -> Self {
        Self { ghost, ..self }
    }

    pub fn with_eval(self, eval: EvalProgress) -> Self {
        Self { eval, ..self }
    }

    pub fn with_peers(self, peers: Vec<PeerDrag>) -> Self {
        Self { peers, ..self }
    }

    /// 🧹️ True when no provisional state remains, so an aborted drag leaves zero trace.
    pub fn is_clear(&self) -> bool {
        self.drag.is_none() && self.ghost.is_none() && self.eval == EvalProgress::default() && self.peers.is_empty()
    }

    /// ✍️ Every provisional write that windows overlay on the document: drag first, then ghost.
    pub fn overlay(&self) -> impl Iterator<Item = &FieldWrite> {
        self.drag.iter().flat_map(|d| d.writes.iter()).chain(self.ghost.iter().flat_map(|g| g.writes.iter()))
    }
}

impl NormPresence {
    pub fn new(user: impl Into<String>) -> Self {
        Self { user: user.into(), selection: Vec::new(), hover: None, camera: None, drag: None }
    }
}

impl Lane {
    pub const ALL: [Lane; 4] = [Self::PersistedShared, Self::PersistedLocal, Self::EphemeralShared, Self::EphemeralLocal];

    pub fn persisted(self) -> bool {
        matches!(self, Self::PersistedShared | Self::PersistedLocal)
    }

    pub fn shared(self) -> bool {
        matches!(self, Self::PersistedShared | Self::EphemeralShared)
    }
}
