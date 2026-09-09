//! 🖱️ Ephemeral drag state for one concrete Wires canvas.

#[cfg(test)]
use protocol::Mutation;

/// 🖱️ The frozen viewport and total pointer span held by one canvas drag.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "reasoning.wirescanvastransient")]
#[dsl(id = "wires.canvas-window-transient")]
#[dsl(layout = "lines")]
pub struct WiresCanvasTransient {
    pub drag_node_id: Option<String>,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
    pub drag_last_x: f64,
    pub drag_last_y: f64,
    pub drag_zoom: f64,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for WiresCanvasTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for WiresCanvasTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl Default for WiresCanvasTransient {
    fn default() -> Self {
        Self { drag_node_id: None, drag_start_x: 0.0, drag_start_y: 0.0, drag_last_x: 0.0, drag_last_y: 0.0, drag_zoom: 1.0 }
    }
}

impl protocol::MutationDiff<WiresCanvasTransient> for WiresCanvasTransient {
    fn apply(&self, _base: &WiresCanvasTransient) -> protocol::MutationApplyResult<WiresCanvasTransient> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

/// 🪟️ The concrete canvas owns its drag lifecycle independently of document/config streams.
pub struct WiresCanvasTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for WiresCanvasTransientOwner {
    const WINDOW_KIND_ID: &'static str = crate::editor::wires::WIRES_PLAY_WINDOW_CANVAS;
    type State = WiresCanvasTransient;
    type Mutation = WiresCanvasTransientMutation;

    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_transient_preparation_factory::<Self>()
    }

    fn build_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::State>> {
        semio_framework_plugin::bounded_window_transient_root_retirement_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_transient_store_disposer::<Self>()
    }
}
