//! 🚦️ Shared Generation native codec: the literal eight-field document mirrors with cumulative construction controls, generic over the artifact snapshot of generation 2d and 3d.
//!
//! Each artifact names its snapshot, envelope and pack discriminator once by implementing [`GenerationNative`] on a local marker type.
#![allow(clippy::result_large_err)]
use semio_framework_dsl_record::{RecordSpecProducer, RecordValue};
use semio_framework_os_flow_contract::FlowHostSnapshot;
use semio_framework_os_playbook_contract::{GenerationPlayRoot, GenerationPlayState};
use semio_framework_value::ValueError;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::collections::BTreeMap;

#[path = "🧬️records/🦀️.rs"]
mod records;
#[path = "🧱️document/🦀️.rs"]
mod document;
#[path = "🌱️value/🦀️.rs"]
mod values;
#[path = "📏️rows/🦀️.rs"]
mod rows;

pub use document::{decode, encode};
pub use records::{
    camera_from_dsl, camera_to_dsl, dictionary_to_value_dsl_entries, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, value_dsl_entries_to_dictionary, widget_from_dsl, widget_to_dsl, CameraJsonDsl,
    DictEntryDsl, FormGenerationDsl, SynapseSpecDsl, ValueDsl, WidgetDsl, WidgetLayoutDsl,
};

/// 🧾️ The eight document fields of a decoded record, in the order the artifact grammar declares them.
pub struct GenerationFields {
    pub schema: String,
    pub camera: CameraJsonDsl,
    pub widgets: Vec<WidgetDsl>,
    pub synapses: Vec<SynapseSpecDsl>,
    pub layout: BTreeMap<String, WidgetLayoutDsl>,
    pub selected_generation_id: Option<String>,
    pub preview_text: Option<String>,
    pub generations: Vec<FormGenerationDsl>,
}

/// 🔌️ What an artifact tells the shared codec about its own snapshot: the envelope, the pack discriminator, the record grammar and how the snapshot splits into host and play state.
pub trait GenerationNative {
    type Snapshot;
    const ENVELOPE_ID: &'static str;
    const PACK_PREFIX: Option<&'static [u8]>;
    fn spec_producer() -> RecordSpecProducer;
    fn fields(record: &RecordValue, control: &mut NativeDecodeControl<'_>) -> Result<GenerationFields, ValueError>;
    fn host(snapshot: &Self::Snapshot) -> &FlowHostSnapshot;
    fn state(snapshot: &Self::Snapshot) -> &GenerationPlayState;
    fn assemble(host_snapshot: FlowHostSnapshot, generation: GenerationPlayRoot) -> Self::Snapshot;
}
