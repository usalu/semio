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
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.wfc.grid2d.windowconfig", extension = "wfcgrid2dwindowcfg", layout = "lines")]
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

/// 🫧️ Per-pane scratch that must never reach the document: the hovered cell under the pointer.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Grid2dWindowTransient {
    pub hovered_cell: String,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid2dWindowConfigMutation {
    Snapshot { config: Grid2dWindowConfig },
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid2dWindowTransientMutation {
    Snapshot { transient: Grid2dWindowTransient },
}

impl protocol::Mutation<Grid2dWindowConfig> for Grid2dWindowConfigMutation {
    type Diff = Grid2dWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-config",
        display_name: "Set 2D Grid Window Configuration",
        emoji: "🪟️",
        aggregate_variant: "Snapshot",
        payload_schema: "wfc.grid2dwindowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, _base: &Grid2dWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }
    fn inverse(&self, base: &Grid2dWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

impl protocol::Mutation<Grid2dWindowTransient> for Grid2dWindowTransientMutation {
    type Diff = Grid2dWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-transient",
        display_name: "Set 2D Grid Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "wfc.grid2dwindowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, _base: &Grid2dWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &Grid2dWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope.
impl store::ArtifactDsl for Grid2dWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(Grid2dWindowConfig);

impl store::ArtifactDsl for Grid2dWindowTransient {
    const EXTENSION: &'static str = "wfcgrid2dwindowtransient";
    fn envelope_id() -> &'static str {
        "s.wfc.grid2d.windowtransient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        dsl::json::to_json_string(self)
    }
}

impl store::ArtifactPack for Grid2dWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

macro_rules! mutation_wire {
    ($mutation:ty) => {
        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String {
                dsl::json::to_json_string(self)
            }
            fn parse_op(line: &str) -> Result<Self, store::TextError> {
                dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
            }
        }
        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                Ok(protocol::OpText::print_op(self).into_bytes())
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
                dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
            }
        }
    };
}

mutation_wire!(Grid2dWindowConfigMutation);
mutation_wire!(Grid2dWindowTransientMutation);

impl protocol::MutationDiff<Grid2dWindowTransient> for Grid2dWindowTransient {
    fn apply(&self, _base: &Grid2dWindowTransient) -> protocol::MutationApplyResult<Grid2dWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

store::artifact_retire_struct!(Grid2dWindowTransient { hovered_cell });

impl store::retirement::RetireOwned for Grid2dWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn grid2d_window_transient_preflight(mutation: &Grid2dWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Grid2dWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = size_of::<Grid2dWindowTransient>().checked_add(transient.hovered_cell.capacity()).ok_or_else(|| "2D grid window transient footprint overflowed".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn grid2d_window_transient_transfer(mutation: Grid2dWindowTransientMutation) -> Grid2dWindowTransient {
    match mutation {
        Grid2dWindowTransientMutation::Snapshot { transient } => transient,
    }
}

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
                let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(grid2d_window_transient_preflight, grid2d_window_transient_transfer, state.clone(), mutation.clone()));
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
pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Grid2dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let (id, kind) = window_kind(view).ok_or_else(|| semio_framework_plugin::Fault::from("wfc-grid2d-window-required"))?;
    let mutation = Grid2dWindowConfigMutation::Snapshot { config };
    match kind {
        grid::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid2dGridWindowConfigOwner>(id, mutation)),
        preview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid2dPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("wfc-grid2d-window-kind-required")),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
