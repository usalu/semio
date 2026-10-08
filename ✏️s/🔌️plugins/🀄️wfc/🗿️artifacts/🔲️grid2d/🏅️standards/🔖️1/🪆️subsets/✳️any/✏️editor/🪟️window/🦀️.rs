//! 🪟️ Exact-instance persisted and ephemeral ownership for the two 2D-grid panes.
//!
//! One `Grid2dWindowConfig` instance PER pane: the grid pane owns the camera, the grid chrome and
//! the ACTIVE TILE the pin utility paints; the preview pane owns its own camera and `solve_json` —
//! the last committed `Grid2dInferenceCommit`, written ONLY by the `solve` command. That cache is
//! window config, never document state: the law "the solve is an inference, never persisted in the
//! artifact" is about `Grid2dSnapshot`, which never gains an assignment field.

use crate::editor::grid2d::modes::edit::windows::{grid, preview};

/// 🎚️ ONE exact pane's persisted-local options. `WindowConfigOwner::State` requires
/// `dsl::DslField`, which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*` helpers the
/// record-backed `ArtifactDsl`/`ArtifactPack` below are written against.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.wfc.grid2d.windowconfig", extension = "wfcgrid2dwindowcfg")]
pub struct Grid2dWindowConfig {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
    pub grid_visible: bool,
    pub grid_snap_enabled: bool,
    pub grid_factor: f64,
    /// 🀄️ The tile id the `pin` utility writes into a clicked cell. Empty means "the first tile".
    pub active_tile_id: String,
    /// 🏁 The last `Grid2dInferenceCommit`, JSON-encoded — a derived cache the `solve` command
    /// refreshes, bounded by `MAXIMUM_PUBLICATION_BYTES`, never part of the document.
    pub solve_json: String,
}

impl Default for Grid2dWindowConfig {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, grid_visible: true, grid_snap_enabled: true, grid_factor: 1.0, active_tile_id: String::new(), solve_json: String::new() }
    }
}

/// 🎥️ The camera a pane actually draws and hit-tests with. An untouched camera sits at the world
/// ORIGIN, which parks the whole grid in the viewport's lower-right quadrant — the same
/// "an unset cursor must fall back to a real entry" law the active tile obeys — so a pane whose
/// camera has never been written centres on the authored grid instead. Both the renderer and
/// `grid::cell_at` read this, so a click and the pixel under it can never disagree.
pub fn effective_camera(document: &crate::schema::snapshot::Grid2dSnapshot, config: &Grid2dWindowConfig) -> (f64, f64, f64) {
    let zoom = if config.camera_zoom > 0.0 { config.camera_zoom } else { 1.0 };
    if config.camera_x == 0.0 && config.camera_y == 0.0 {
        return (f64::from(document.width) * document.cell_width * 0.5, f64::from(document.height) * document.cell_height * 0.5, zoom);
    }
    (config.camera_x, config.camera_y, zoom)
}

/// 🫧️ Per-pane scratch that must never reach the document: the hovered cell under the pointer.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid2dWindowTransient {
    pub hovered_cell: String,
}

/// 🔺️ Field-sparse diff of [`Grid2dWindowConfig`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid2dWindowConfigDiff {
    pub camera_x: Option<f64>,
    pub camera_y: Option<f64>,
    pub camera_zoom: Option<f64>,
    pub grid_visible: Option<bool>,
    pub grid_snap_enabled: Option<bool>,
    pub grid_factor: Option<f64>,
    pub active_tile_id: Option<String>,
    pub solve_json: Option<String>,
}

impl protocol::DiffAlgebra<Grid2dWindowConfig> for Grid2dWindowConfigDiff {
    fn inverse(&self, base: &Grid2dWindowConfig) -> Self {
        Self {
            camera_x: self.camera_x.as_ref().map(|_| base.camera_x.clone()),
            camera_y: self.camera_y.as_ref().map(|_| base.camera_y.clone()),
            camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom.clone()),
            grid_visible: self.grid_visible.as_ref().map(|_| base.grid_visible.clone()),
            grid_snap_enabled: self.grid_snap_enabled.as_ref().map(|_| base.grid_snap_enabled.clone()),
            grid_factor: self.grid_factor.as_ref().map(|_| base.grid_factor.clone()),
            active_tile_id: self.active_tile_id.as_ref().map(|_| base.active_tile_id.clone()),
            solve_json: self.solve_json.as_ref().map(|_| base.solve_json.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none() && self.grid_visible.is_none() && self.grid_snap_enabled.is_none() && self.grid_factor.is_none() && self.active_tile_id.is_none() && self.solve_json.is_none()
    }
}

impl protocol::MutationDiff<Grid2dWindowConfig> for Grid2dWindowConfigDiff {
    fn apply(&self, base: &Grid2dWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid2dWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x.clone_from(value);
        }
        if let Some(value) = &self.camera_y {
            next.camera_y.clone_from(value);
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom.clone_from(value);
        }
        if let Some(value) = &self.grid_visible {
            next.grid_visible.clone_from(value);
        }
        if let Some(value) = &self.grid_snap_enabled {
            next.grid_snap_enabled.clone_from(value);
        }
        if let Some(value) = &self.grid_factor {
            next.grid_factor.clone_from(value);
        }
        if let Some(value) = &self.active_tile_id {
            next.active_tile_id.clone_from(value);
        }
        if let Some(value) = &self.solve_json {
            next.solve_json.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.camera_x.is_some() {
            self.camera_x = later.camera_x;
        }
        if later.camera_y.is_some() {
            self.camera_y = later.camera_y;
        }
        if later.camera_zoom.is_some() {
            self.camera_zoom = later.camera_zoom;
        }
        if later.grid_visible.is_some() {
            self.grid_visible = later.grid_visible;
        }
        if later.grid_snap_enabled.is_some() {
            self.grid_snap_enabled = later.grid_snap_enabled;
        }
        if later.grid_factor.is_some() {
            self.grid_factor = later.grid_factor;
        }
        if later.active_tile_id.is_some() {
            self.active_tile_id = later.active_tile_id;
        }
        if later.solve_json.is_some() {
            self.solve_json = later.solve_json;
        }
    }
}

/// 🪟️ The 2D grid window configuration's mutation vocabulary: one absolute setter per field group.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid2dWindowConfigMutation {
    SetActiveTile { tile_id: String },
    SetCamera { x: f64, y: f64, zoom: f64 },
    SetGridVisible { visible: bool },
    SetGridSnapEnabled { enabled: bool },
    SetGridFactor { factor: f64 },
    SetSolveJson { solve_json: String },
}

impl protocol::Mutation<Grid2dWindowConfig> for Grid2dWindowConfigMutation {
    type Diff = Grid2dWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-active-tile",
            display_name: "Set Active Tile",
            emoji: "🪟️",
            aggregate_variant: "SetActiveTile",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "🪟️",
            aggregate_variant: "SetCamera",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-grid-visible",
            display_name: "Set Grid Visible",
            emoji: "🪟️",
            aggregate_variant: "SetGridVisible",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-grid-snap-enabled",
            display_name: "Set Grid Snap Enabled",
            emoji: "🪟️",
            aggregate_variant: "SetGridSnapEnabled",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-grid-factor",
            display_name: "Set Grid Factor",
            emoji: "🪟️",
            aggregate_variant: "SetGridFactor",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-solve-json",
            display_name: "Set Solve Json",
            emoji: "🪟️",
            aggregate_variant: "SetSolveJson",
            payload_schema: "wfc.grid2dwindowconfig",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetActiveTile { .. } => &Self::DESCRIPTORS[0],
            Self::SetCamera { .. } => &Self::DESCRIPTORS[1],
            Self::SetGridVisible { .. } => &Self::DESCRIPTORS[2],
            Self::SetGridSnapEnabled { .. } => &Self::DESCRIPTORS[3],
            Self::SetGridFactor { .. } => &Self::DESCRIPTORS[4],
            Self::SetSolveJson { .. } => &Self::DESCRIPTORS[5],
        }
    }
    fn diff(&self, base: &Grid2dWindowConfig) -> protocol::MutationOutcome<Grid2dWindowConfigDiff> {
        let diff = match self {
            Self::SetActiveTile { tile_id } => Grid2dWindowConfigDiff { active_tile_id: (base.active_tile_id != *tile_id).then(|| tile_id.clone()), ..Default::default() },
            Self::SetCamera { x, y, zoom } => Grid2dWindowConfigDiff { camera_x: (base.camera_x != *x).then(|| x.clone()), camera_y: (base.camera_y != *y).then(|| y.clone()), camera_zoom: (base.camera_zoom != *zoom).then(|| zoom.clone()), ..Default::default() },
            Self::SetGridVisible { visible } => Grid2dWindowConfigDiff { grid_visible: (base.grid_visible != *visible).then(|| visible.clone()), ..Default::default() },
            Self::SetGridSnapEnabled { enabled } => Grid2dWindowConfigDiff { grid_snap_enabled: (base.grid_snap_enabled != *enabled).then(|| enabled.clone()), ..Default::default() },
            Self::SetGridFactor { factor } => Grid2dWindowConfigDiff { grid_factor: (base.grid_factor != *factor).then(|| factor.clone()), ..Default::default() },
            Self::SetSolveJson { solve_json } => Grid2dWindowConfigDiff { solve_json: (base.solve_json != *solve_json).then(|| solve_json.clone()), ..Default::default() },
        };
        if protocol::DiffAlgebra::<Grid2dWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds that value.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Grid2dWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetActiveTile { .. } => Self::SetActiveTile { tile_id: base.active_tile_id.clone() },
            Self::SetCamera { .. } => Self::SetCamera { x: base.camera_x.clone(), y: base.camera_y.clone(), zoom: base.camera_zoom.clone() },
            Self::SetGridVisible { .. } => Self::SetGridVisible { visible: base.grid_visible.clone() },
            Self::SetGridSnapEnabled { .. } => Self::SetGridSnapEnabled { enabled: base.grid_snap_enabled.clone() },
            Self::SetGridFactor { .. } => Self::SetGridFactor { factor: base.grid_factor.clone() },
            Self::SetSolveJson { .. } => Self::SetSolveJson { solve_json: base.solve_json.clone() },
        }])
    }
}
/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope.
impl store::ArtifactDsl for Grid2dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid 2D grid window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` would fail every retained load of these
/// window kinds with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for Grid2dWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ConfigRecord for Grid2dWindowConfig {}

macro_rules! mutation_wire {
    ($mutation:ty) => {
        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String {
                semio_framework_pack_json::to_json_string(self)
            }
            fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
            }
        }
        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                Ok(protocol::OpText::print_op(self).into_bytes())
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
                semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
            }
        }
    };
}

mutation_wire!(Grid2dWindowConfigMutation);

semio_framework_value::artifact_retire_struct!(Grid2dWindowTransient { hovered_cell });

macro_rules! owners {
    ($config:ident, $transient:ident, $kind:expr) => {
        pub struct $config;
        impl semio_framework_plugin::WindowConfigOwner for $config {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "wfc.grid2dwindowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = Grid2dWindowConfig;
            type Mutation = Grid2dWindowConfigMutation;
            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
                semio_framework_plugin::bounded_window_config_store_owners::<Self>()
            }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
            }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }
        pub struct $transient;
        impl semio_framework_plugin::WindowTransientOwner for $transient {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = Grid2dWindowTransient;
            type Mutation = Grid2dWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(Grid2dWindowTransientMutation::footprint, Grid2dWindowTransientMutation::into_state, state.clone(), mutation.clone()));
                semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
            }
        }
    };
}

owners!(Grid2dGridWindowConfigOwner, Grid2dGridWindowTransientOwner, grid::WINDOW_KIND_ID);
owners!(Grid2dPreviewWindowConfigOwner, Grid2dPreviewWindowTransientOwner, preview::WINDOW_KIND_ID);

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Grid2dGridWindowConfigOwner>()?;
    registry.register::<Grid2dPreviewWindowConfigOwner>()
}

pub fn register_transient(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Grid2dGridWindowTransientOwner>()?;
    registry.register::<Grid2dPreviewWindowTransientOwner>()
}

/// 🎚️ The config of the pane being rendered or dispatched — grid first, preview second, default
/// last, mirroring the per-window-instance lookup every sibling multi-pane editor uses.
pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Grid2dWindowConfig {
    view.window::<Grid2dGridWindowConfigOwner>().or_else(|| view.window::<Grid2dPreviewWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn config_from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> Grid2dWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<Grid2dGridWindowConfigOwner>().or_else(|| snapshot.get::<Grid2dPreviewWindowConfigOwner>())).cloned().unwrap_or_default()
}

pub fn transient_from_view(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> Grid2dWindowTransient {
    view.window::<Grid2dGridWindowTransientOwner>().or_else(|| view.window::<Grid2dPreviewWindowTransientOwner>()).cloned().unwrap_or_default()
}

fn window_kind(view: &semio_framework_plugin::ViewModel) -> Option<(&str, &str)> {
    let id = view.window_id.as_deref()?;
    let kind = view.window_instances.iter().find(|window| window.id == id)?.window_kind_id.as_str();
    Some((id, kind))
}

/// 🪟️ Which pane kind is being rendered or dispatched right now.
pub fn kind_for_view(view: &semio_framework_plugin::ViewModel) -> Option<&str> {
    window_kind(view).map(|(_, kind)| kind)
}

/// 📮️ Addresses a config write at the EXACT window instance the command was dispatched in — a
/// config write that names the wrong pane silently lands in the other pane's store.
pub fn addressed_config(view: &semio_framework_plugin::ViewModel, mutation: Grid2dWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let (id, kind) = window_kind(view).ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.grid2d.window.required"), "wfc.grid2d.window.required"))?;
    match kind {
        grid::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid2dGridWindowConfigOwner>(id, mutation)),
        preview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid2dPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.grid2d.window.kind-required"), "wfc.grid2d.window.kind-required")),
    }
}

semio_framework_plugin::transient_root! {
    state: Grid2dWindowTransient,
    mutation: Grid2dWindowTransientMutation,
    diff: Grid2dWindowTransientDiff,
    owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
    kind: "set-window-transient",
    display_name: "Set 2D Grid Window Transient",
    payload_schema: "wfc.grid2dwindowtransient",
    envelope: "s.wfc.grid2d.windowtransient",
    extension: "wfcgrid2dwindowtransient",
    fields: { hovered_cell: String },
}

semio_framework_plugin::window_transient_transfer! {
    state: Grid2dWindowTransient,
    mutation: Grid2dWindowTransientMutation,
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
