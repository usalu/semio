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

store::artifact_retire_struct!(WiresCanvasTransient { drag_node_id, drag_start_x, drag_start_y, drag_last_x, drag_last_y, drag_zoom });
store::artifact_retire_struct!(SetDrag { node_id, start_x, start_y, last_x, last_y, zoom });

impl store::retirement::RetireOwned for WiresCanvasTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::SetDrag(value) => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(value)]),
        }
    }
}

const WIRES_CANVAS_TRANSIENT_FIXED_BYTES: usize = 5 * size_of::<f64>() + 32;

fn wires_canvas_transient_footprint(mutation: &WiresCanvasTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let WiresCanvasTransientMutation::SetDrag(value) = mutation;
    if !value.start_x.is_finite() || !value.start_y.is_finite() || !value.last_x.is_finite() || !value.last_y.is_finite() || !value.zoom.is_finite() || value.zoom <= 0.0 {
        return Err("Wires canvas drag contains a non-finite coordinate or invalid zoom".into());
    }
    let retained_bytes = WIRES_CANVAS_TRANSIENT_FIXED_BYTES.checked_add(value.node_id.as_ref().map_or(0, String::len)).ok_or_else(|| "Wires canvas drag footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Wires canvas drag exceeds its retained publication envelope".into())
}

fn wires_canvas_transient_transfer(mutation: WiresCanvasTransientMutation) -> WiresCanvasTransient {
    let WiresCanvasTransientMutation::SetDrag(value) = mutation;
    WiresCanvasTransient { drag_node_id: value.node_id, drag_start_x: value.start_x, drag_start_y: value.start_y, drag_last_x: value.last_x, drag_last_y: value.last_y, drag_zoom: value.zoom }
}

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

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(wires_canvas_transient_footprint, wires_canvas_transient_transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}
