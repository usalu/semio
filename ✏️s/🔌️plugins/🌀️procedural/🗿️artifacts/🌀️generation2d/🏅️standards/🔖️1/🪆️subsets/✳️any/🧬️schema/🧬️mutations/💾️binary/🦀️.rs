//! ⚖️ Generation2d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::generation2d::dsl::{
    CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl, camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl,
    widget_to_dsl,
};
use crate::artifacts::generation2d::schema::mutations::text::Generation2dMutation;
use crate::artifacts::generation2d::schema::snapshot::Generation2dSnapshot;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
/// ⚡️ Local twin of `Generation2dMutation` — one flattened, `#[derive(dsl::DslEnum)]`-friendly
/// keyword variant per semantic mutation (each payload struct embeds a foreign `flow` type —
/// `Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson`/`FormGeneration` — that can't itself derive
/// `dsl::DslRecord`, so this mirror + the existing `*_to_dsl`/`*_from_dsl` bridge functions do the
/// wire conversion instead of deriving the codec straight off the payload structs).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Generation2dOperationDsl {
    CreateWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    ReplaceWidget {
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    DeleteWidget {
        id: String,
    },
    ConnectSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    ReplaceSynapse {
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    DisconnectSynapse {
        id: String,
    },
    MoveWidget {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    ClearWidgetLayout {
        id: String,
    },
    UpdateCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    ChangeSchema {
        schema: String,
    },
    CreateGeneration {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    DeleteGeneration {
        id: String,
    },
    RenameGeneration {
        id: String,
        name: String,
    },
    ChangeGenerationValue {
        id: String,
        question_id: String,
        value: dsl::DslValue,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Generation2dOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl OpBinary for Generation2dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn generation2d_operation_to_dsl(operation: &Generation2dMutation) -> Generation2dOperationDsl {
    match operation {
        Generation2dMutation::CreateWidget(payload) => Generation2dOperationDsl::CreateWidget { index: payload.index, widget: Box::new(widget_to_dsl(&payload.widget)) },
        Generation2dMutation::ReplaceWidget(payload) => Generation2dOperationDsl::ReplaceWidget { widget: Box::new(widget_to_dsl(&payload.widget)) },
        Generation2dMutation::DeleteWidget(payload) => Generation2dOperationDsl::DeleteWidget { id: payload.id.clone() },
        Generation2dMutation::ConnectSynapse(payload) => Generation2dOperationDsl::ConnectSynapse { index: payload.index, synapse: synapse_to_dsl(&payload.synapse) },
        Generation2dMutation::ReplaceSynapse(payload) => Generation2dOperationDsl::ReplaceSynapse { synapse: synapse_to_dsl(&payload.synapse) },
        Generation2dMutation::DisconnectSynapse(payload) => Generation2dOperationDsl::DisconnectSynapse { id: payload.id.clone() },
        Generation2dMutation::MoveWidget(payload) => Generation2dOperationDsl::MoveWidget { id: payload.id.clone(), layout: layout_to_dsl(&payload.layout) },
        Generation2dMutation::ClearWidgetLayout(payload) => Generation2dOperationDsl::ClearWidgetLayout { id: payload.id.clone() },
        Generation2dMutation::UpdateCamera(payload) => Generation2dOperationDsl::UpdateCamera { camera: camera_to_dsl(&payload.camera) },
        Generation2dMutation::ChangeSchema(payload) => Generation2dOperationDsl::ChangeSchema { schema: payload.schema.clone() },
        Generation2dMutation::CreateGeneration(payload) => Generation2dOperationDsl::CreateGeneration { generation: form_generation_to_dsl(&payload.generation) },
        Generation2dMutation::DeleteGeneration(payload) => Generation2dOperationDsl::DeleteGeneration { id: payload.id.clone() },
        Generation2dMutation::RenameGeneration(payload) => Generation2dOperationDsl::RenameGeneration { id: payload.id.clone(), name: payload.name.clone() },
        Generation2dMutation::ChangeGenerationValue(payload) => {
            Generation2dOperationDsl::ChangeGenerationValue { id: payload.id.clone(), question_id: payload.question_id.clone(), value: dsl::DslValue::from(&payload.value) }
        }
    }
}

fn generation2d_operation_from_dsl(operation: Generation2dOperationDsl) -> Result<Generation2dMutation, store::TextError> {
    use crate::artifacts::generation2d::mutations::{
        change_generation_value, change_schema, clear_widget_layout, connect_synapse, create_generation, create_widget, delete_generation, delete_widget, disconnect_synapse, move_widget, rename_generation, replace_synapse, replace_widget,
        update_camera,
    };
    Ok(match operation {
        Generation2dOperationDsl::CreateWidget { index, widget } => create_widget(index, widget_from_dsl(*widget)?),
        Generation2dOperationDsl::ReplaceWidget { widget } => replace_widget(widget_from_dsl(*widget)?),
        Generation2dOperationDsl::DeleteWidget { id } => delete_widget(id),
        Generation2dOperationDsl::ConnectSynapse { index, synapse } => connect_synapse(index, synapse_from_dsl(synapse)),
        Generation2dOperationDsl::ReplaceSynapse { synapse } => replace_synapse(synapse_from_dsl(synapse)),
        Generation2dOperationDsl::DisconnectSynapse { id } => disconnect_synapse(id),
        Generation2dOperationDsl::MoveWidget { id, layout } => move_widget(id, layout_from_dsl(&layout)),
        Generation2dOperationDsl::ClearWidgetLayout { id } => clear_widget_layout(id),
        Generation2dOperationDsl::UpdateCamera { camera } => update_camera(camera_from_dsl(&camera)),
        Generation2dOperationDsl::ChangeSchema { schema } => change_schema(schema),
        Generation2dOperationDsl::CreateGeneration { generation } => create_generation(form_generation_from_dsl(generation)),
        Generation2dOperationDsl::DeleteGeneration { id } => delete_generation(id),
        Generation2dOperationDsl::RenameGeneration { id, name } => rename_generation(id, name),
        Generation2dOperationDsl::ChangeGenerationValue { id, question_id, value } => change_generation_value(id, question_id, serde_json::Value::from(&value)),
    })
}

/// ⚡️ `Generation2dMutation`'s compact single-line op encoding — derive-engine grammar via
/// `Generation2dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Generation2dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Generation2dOperationDsl as protocol::OpText>::parse_op(line)?;
        generation2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Generation2dOperationDsl as protocol::OpText>::print_op(&generation2d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Generation2dOperationDsl` already implements
/// `OpBinary`, so this is a pure to/from-dsl forward.
impl OpBinary for Generation2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        generation2d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Generation2dOperationDsl::decode_op(bytes)?;
        generation2d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "generation2d mutation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Generation2dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Generation2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Generation2dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Generation2dMutation, protocol::ProtocolError> {
    Generation2dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::generation2d::mutations::{change_schema, connect_synapse, create_generation, create_widget, delete_widget};
    use crate::artifacts::generation2d::{GENERATION_2D_SCHEMA, Generation2dSnapshot};
    use flow::{SynapseSpec, Widget};
    use protocol::OpText;
    use semio_framework_os_kernel::os_store::test_support;
    use store::{ArtifactCommand, create_document_envelope};

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_create_widget() {
        test_support::assert_op_line_round_trip(&create_widget(2, Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() }));
    }

    #[test]
    fn op_text_round_trip_delete_widget() {
        test_support::assert_op_line_round_trip(&delete_widget("note-9".into()));
    }

    #[test]
    fn op_text_round_trip_connect_synapse() {
        test_support::assert_op_line_round_trip(&connect_synapse(1, SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() }));
    }

    #[test]
    fn op_text_round_trip_change_schema() {
        test_support::assert_op_line_round_trip(&change_schema("flow.fixture".into()));
    }

    #[test]
    fn op_text_round_trip_create_generation() {
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&create_generation(generation));
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️OpTextErrorTests
    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Generation2dMutation::parse_op("bogus-op id=\"x\"").unwrap_err();
        assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
    }

    #[test]
    fn op_text_parse_rejects_non_integer_index() {
        let error = Generation2dMutation::parse_op("create-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
        assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖️OpTextErrorTests

    #[test]
    fn op_binary_round_trips_via_wrapper_fns() {
        let operation = change_schema("flow.fixture".into());
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = store::ArtifactStore::<Generation2dSnapshot, Generation2dMutation>::new(create_document_envelope(GENERATION_2D_SCHEMA, "generation2d", Generation2dSnapshot::default(), None)).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![create_widget(3, Widget::InputNote { id: "note-9".into(), text: String::new() })], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
//#region 🔖️RetainedMountedIngress
const GENERATION2D_OWNER_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const GENERATION2D_RETAINED_STACK_CAPACITY: usize = 64;
const GENERATION2D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;
const GENERATION2D_MAXIMUM_DOMAIN_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const GENERATION2D_MAXIMUM_OUTPUT_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const GENERATION2D_MUTATION_VARIANT_COUNT: usize = 14;
pub const GENERATION2D_MOUNTED_OUTPUT_CHANNELS: usize = 4;
pub const GENERATION2D_MOUNTED_CONTROL_CREDITS: usize = 1;
const GENERATION2D_PUBLICATION_SLOTS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Generation2dPublicationLease {
    operation: u64,
    generation: u64,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
    closing: bool,
    terminal: bool,
}

impl semio_framework_job::FixedOperationOwner for Generation2dPublicationLease {
    fn retained_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 || maximum_bytes < std::mem::size_of::<Self>() {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.terminal {
            self.terminal = true;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: std::mem::size_of::<Self>() };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal
    }
}

type Generation2dPublicationRegistry = semio_framework_job::FixedOperationRegistry<Generation2dPublicationLease, GENERATION2D_PUBLICATION_SLOTS>;

fn generation2d_publication_leases() -> &'static std::sync::Mutex<Generation2dPublicationRegistry> {
    static LEASES: std::sync::OnceLock<std::sync::Mutex<semio_framework_job::FixedOperationRegistry<Generation2dPublicationLease, GENERATION2D_PUBLICATION_SLOTS>>> = std::sync::OnceLock::new();
    LEASES.get_or_init(|| std::sync::Mutex::new(Generation2dPublicationRegistry::new(GENERATION2D_PUBLICATION_SLOTS * std::mem::size_of::<Generation2dPublicationLease>())))
}

fn generation2d_publication_key(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::FixedOperationKey {
    semio_framework_job::FixedOperationKey::new(operation, generation)
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation2dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Generation2dPublicationHostileLease {
    operation: u64,
    hostile: Generation2dPublicationHostile,
    observed: Option<&'static str>,
}

#[cfg(test)]
fn generation2d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Generation2dPublicationHostileLease>; GENERATION2D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; GENERATION2D_PUBLICATION_SLOTS]))
}

#[cfg(test)]
pub fn generation2d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Generation2dPublicationHostile) {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Generation2d hostile publication authority has a fixed slot");
    *slot = Some(Generation2dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(test)]
pub fn generation2d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = generation2d_publication_hostiles().try_lock().expect("Generation2d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}

pub fn generation2d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
) -> Result<(), &'static str> {
    if generation.0 != live_revision || base_revision != live_revision || parent_revision != base_revision {
        return Err("generation2d-publication.initial-freshness");
    }
    let mut leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("generation2d-publication.operation-duplicate");
    }
    if maximum_items == 0 || maximum_items > GENERATION2D_MAXIMUM_DOMAIN_ITEMS || maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.domain-credits");
    }
    leases
        .admit(
            generation2d_publication_key(operation, generation),
            Generation2dPublicationLease { operation: operation.0, generation: generation.0, base_revision, parent_revision, live_revision, maximum_items, maximum_output_pages, maximum_controls, closing: false, terminal: false },
        )
        .map_err(|_| "generation2d-publication.saturated")
}

pub fn generation2d_refresh_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_revision: u64) -> Result<(), &'static str> {
    let mut leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    let lease = leases.get_mut(generation2d_publication_key(operation, generation)).ok_or("generation2d-publication.stale-authority")?;
    lease.live_revision = live_revision;
    Ok(())
}

pub fn generation2d_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    let lease = leases.get(generation2d_publication_key(operation, generation)).ok_or("generation2d-publication.stale-authority")?;
    if lease.generation != generation.0 || lease.live_revision != generation.0 || lease.base_revision != lease.live_revision || lease.parent_revision != lease.base_revision {
        return Err("generation2d-publication.stale-aba-parent");
    }
    Ok((lease.base_revision, lease.parent_revision))
}

fn generation2d_validate_atomic_lease(lease: Generation2dPublicationLease, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    if lease.operation != operation.0 {
        return Err("generation2d-publication.wrong-operation");
    }
    if lease.generation != generation.0 {
        return Err("generation2d-publication.wrong-generation");
    }
    if lease.live_revision != live_generation.0 || lease.base_revision != lease.live_revision {
        return Err("generation2d-publication.wrong-base");
    }
    if lease.parent_revision != lease.base_revision {
        return Err("generation2d-publication.wrong-parent");
    }
    if lease.maximum_items == 0 || lease.maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.authority-credits");
    }
    Ok(())
}

/// 🔐️ Fail-closed Generation2d authority used by the shared atomic replacement branch.
pub fn generation2d_validate_atomic_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    let mut lease = leases.get_operation(operation).map(|(_, lease)| *lease).ok_or("generation2d-publication.authority-missing")?;
    #[cfg(test)]
    {
        let mut hostiles = generation2d_publication_hostiles().try_lock().map_err(|_| "generation2d-publication.hostile-contended")?;
        if let Some(hostile) = hostiles.iter_mut().flatten().find(|value| value.operation == operation.0) {
            hostile.observed = Some(match hostile.hostile {
                Generation2dPublicationHostile::Missing => "generation2d-publication.authority-missing",
                Generation2dPublicationHostile::WrongOperation => "generation2d-publication.wrong-operation",
                Generation2dPublicationHostile::WrongGeneration => "generation2d-publication.wrong-generation",
                Generation2dPublicationHostile::WrongBase => "generation2d-publication.wrong-base",
                Generation2dPublicationHostile::WrongParent => "generation2d-publication.wrong-parent",
            });
            match hostile.hostile {
                Generation2dPublicationHostile::Missing => return Err("generation2d-publication.authority-missing"),
                Generation2dPublicationHostile::WrongOperation => lease.operation = lease.operation.wrapping_add(1),
                Generation2dPublicationHostile::WrongGeneration => lease.generation = lease.generation.wrapping_add(1),
                Generation2dPublicationHostile::WrongBase => lease.base_revision = lease.base_revision.wrapping_add(1),
                Generation2dPublicationHostile::WrongParent => lease.parent_revision = lease.parent_revision.wrapping_add(1),
            }
        }
    }
    generation2d_validate_atomic_lease(lease, operation, generation, live_generation)
}

pub fn generation2d_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    let lease = leases.get(generation2d_publication_key(operation, generation)).ok_or("generation2d-publication.stale-authority")?;
    if lease.maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.domain-credits-lost");
    }
    Ok(lease.maximum_items)
}

pub fn generation2d_release_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> bool {
    let Ok(mut leases) = generation2d_publication_leases().try_lock() else { return false };
    leases.take(generation2d_publication_key(operation, generation)).is_some()
}

/// 🧭️ Fixed ownership grammar for every Generation2d retained domain and lifecycle owner.
/// The 2d-only ClearWidgetLayout entry is deliberately explicit: a 3d mutation catalog cannot satisfy this table.
pub const GENERATION2D_RETAINED_OWNER_CATALOG: &[&str] = &[
    "snapshot.fixture.schema",
    "snapshot.fixture.camera.x",
    "snapshot.fixture.camera.y",
    "snapshot.fixture.camera.zoom",
    "snapshot.fixture.widgets.length",
    "snapshot.fixture.widgets.item.neuron",
    "snapshot.fixture.widgets.item.input-slider",
    "snapshot.fixture.widgets.item.input-note",
    "snapshot.fixture.widgets.item.input-image",
    "snapshot.fixture.widgets.item.variable",
    "snapshot.fixture.widgets.item.output-preview",
    "snapshot.fixture.widgets.item.output-action",
    "snapshot.fixture.widgets.item.output-export",
    "snapshot.fixture.widgets.item.cluster",
    "snapshot.fixture.widgets.item.strings",
    "snapshot.fixture.widgets.item.dictionary.entries",
    "snapshot.fixture.widgets.item.tree",
    "snapshot.fixture.widgets.item.flow",
    "snapshot.fixture.synapses.length",
    "snapshot.fixture.synapses.item.id",
    "snapshot.fixture.synapses.item.from",
    "snapshot.fixture.synapses.item.to",
    "snapshot.fixture.synapses.item.from-port",
    "snapshot.fixture.synapses.item.to-port",
    "snapshot.fixture.layout.length",
    "snapshot.fixture.layout.item.id",
    "snapshot.fixture.layout.item.x",
    "snapshot.fixture.layout.item.y",
    "snapshot.generation.generations.length",
    "snapshot.generation.generations.item.id",
    "snapshot.generation.generations.item.name",
    "snapshot.generation.generations.item.values.length",
    "snapshot.generation.generations.item.values.key",
    "snapshot.generation.generations.item.values.scalar",
    "snapshot.generation.selected-generation-id",
    "snapshot.generation.preview-text",
    "mutation.create-widget",
    "mutation.replace-widget",
    "mutation.delete-widget",
    "mutation.connect-synapse",
    "mutation.replace-synapse",
    "mutation.disconnect-synapse",
    "mutation.move-widget",
    "mutation.clear-widget-layout.2d-only",
    "mutation.update-camera",
    "mutation.change-schema",
    "mutation.create-generation",
    "mutation.delete-generation",
    "mutation.rename-generation",
    "mutation.change-generation-value",
    "history.edit.id",
    "history.edit.actor",
    "history.edit.forward",
    "history.edit.inverse",
    "history.edit.mutation-meta",
    "history.cursor.applied",
    "history.cursor.redo",
    "history.cursor.checkpoint",
    "conflict.rejected-fresh",
    "child.widget.cluster.tree",
    "child.widget.cluster.flow",
    "control.cancel",
    "control.retry",
    "control.close",
    "output.progress",
    "output.checkpoint",
    "output.preview",
    "output.terminal",
];

pub const GENERATION2D_RETAINED_MUTATION_OWNERS: [&str; GENERATION2D_MUTATION_VARIANT_COUNT] = [
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
];

/// 📐️ One structural opportunity is admitted per grant; combined nesting remains fixed.
pub const GENERATION2D_RETAINED_COMBINED_DEPTH: usize = 12;
pub const GENERATION2D_RETAINED_SCHEMA_DISCRIMINATOR: [u8; 4] = *b"P2D2";
pub const GENERATION2D_FORBIDDEN_3D_DISCRIMINATOR: [u8; 4] = *b"P3D3";

pub fn generation2d_retained_catalog_is_complete() -> bool {
    GENERATION2D_RETAINED_MUTATION_OWNERS == crate::artifacts::generation2d::schema::mutations::KINDS
        && GENERATION2D_RETAINED_OWNER_CATALOG.contains(&"mutation.clear-widget-layout.2d-only")
        && !GENERATION2D_RETAINED_OWNER_CATALOG.iter().any(|owner| owner.contains("process3d"))
}

enum Generation2dReplayDisplaced {
    Widget(flow::Widget),
    Synapse(flow::SynapseSpec),
    Layout(std::sync::Arc<flow::WidgetLayout>),
    Camera(flow::CameraJson),
    Text(String),
    Generation(flow::playbook::FormGeneration),
    Json(dsl::DslValue),
}

struct Generation2dReplayRetirement {
    value: std::mem::ManuallyDrop<Option<Generation2dReplayDisplaced>>,
    domain: flow::retained::FlowRetirement,
}

impl store::ErasedSnapshotRetirement for Generation2dReplayRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if !self.domain.is_empty() { return self.domain.close_step(maximum_items, maximum_bytes); }
        if maximum_items == 0 || maximum_bytes < GENERATION2D_OWNER_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(value) = self.value.take() {
            match value {
                Generation2dReplayDisplaced::Widget(value) => self.domain.push(flow::retained::FlowOwner::Widget(value)),
                Generation2dReplayDisplaced::Synapse(value) => self.domain.push(flow::retained::FlowOwner::Specs(vec![value])),
                Generation2dReplayDisplaced::Layout(value) => drop(value),
                Generation2dReplayDisplaced::Camera(value) => drop(value),
                Generation2dReplayDisplaced::Text(value) => self.domain.text(value),
                Generation2dReplayDisplaced::Generation(value) => drop(value),
                Generation2dReplayDisplaced::Json(value) => drop(value),
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: GENERATION2D_OWNER_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.domain.is_empty()
    }
}

impl Drop for Generation2dReplayRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "Generation2d replay displacement reached Drop before terminal-empty close");
    }
}

fn generation2d_retire_displaced(value: Generation2dReplayDisplaced) -> Option<Box<dyn store::ErasedSnapshotRetirement>> {
    Some(Box::new(Generation2dReplayRetirement { value: std::mem::ManuallyDrop::new(Some(value)), domain: flow::retained::FlowRetirement::default() }))
}

/// 🔁️ Direct semantic replay table. It consumes the retained mutation and writes only the
/// addressed field or collection; no VCS diff/apply or whole-snapshot replacement is reachable.
fn generation2d_apply_initialization_mutation(snapshot: &mut Generation2dSnapshot, mutation: &Generation2dMutation) -> Result<Option<Box<dyn store::ErasedSnapshotRetirement>>, &'static str> {
    let retired = match mutation {
        Generation2dMutation::CreateWidget(payload) => {
            if snapshot.fixture.widgets.iter().any(|entry| crate::artifacts::generation2d::widget_id(entry) == crate::artifacts::generation2d::widget_id(&payload.widget)) {
                return Err("generation2d-replay.widget-duplicate");
            }
            let index = payload.index.min(snapshot.fixture.widgets.len());
            snapshot.fixture.widgets.insert(index, generation2d_copy_widget(&payload.widget)?);
            None
        }
        Generation2dMutation::ReplaceWidget(payload) => {
            let id = crate::artifacts::generation2d::widget_id(&payload.widget);
            let index = snapshot.fixture.widgets.iter().position(|entry| crate::artifacts::generation2d::widget_id(entry) == id).ok_or("generation2d-replay.widget-missing")?;
            generation2d_retire_displaced(Generation2dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.fixture.widgets[index], generation2d_copy_widget(&payload.widget)?)))
        }
        Generation2dMutation::DeleteWidget(payload) => {
            let index = snapshot.fixture.widgets.iter().position(|entry| crate::artifacts::generation2d::widget_id(entry) == payload.id).ok_or("generation2d-replay.widget-missing")?;
            generation2d_retire_displaced(Generation2dReplayDisplaced::Widget(snapshot.fixture.widgets.remove(index)))
        }
        Generation2dMutation::ConnectSynapse(payload) => {
            if snapshot.fixture.synapses.iter().any(|entry| entry.id == payload.synapse.id) {
                return Err("generation2d-replay.synapse-duplicate");
            }
            let index = payload.index.min(snapshot.fixture.synapses.len());
            snapshot.fixture.synapses.insert(index, generation2d_copy_synapse(&payload.synapse)?);
            None
        }
        Generation2dMutation::ReplaceSynapse(payload) => {
            let index = snapshot.fixture.synapses.iter().position(|entry| entry.id == payload.synapse.id).ok_or("generation2d-replay.synapse-missing")?;
            generation2d_retire_displaced(Generation2dReplayDisplaced::Synapse(std::mem::replace(&mut snapshot.fixture.synapses[index], generation2d_copy_synapse(&payload.synapse)?)))
        }
        Generation2dMutation::DisconnectSynapse(payload) => {
            let index = snapshot.fixture.synapses.iter().position(|entry| entry.id == payload.id).ok_or("generation2d-replay.synapse-missing")?;
            generation2d_retire_displaced(Generation2dReplayDisplaced::Synapse(snapshot.fixture.synapses.remove(index)))
        }
        Generation2dMutation::MoveWidget(payload) => {
            if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
                return Err("generation2d-replay.layout-nonfinite");
            }
            snapshot.fixture.layout.insert(generation2d_copy_string(&payload.id)?, flow::WidgetLayout { x: payload.layout.x, y: payload.layout.y }).map(Generation2dReplayDisplaced::Layout).and_then(generation2d_retire_displaced)
        }
        Generation2dMutation::ClearWidgetLayout(payload) => snapshot.fixture.layout.remove(&payload.id).map(Generation2dReplayDisplaced::Layout).and_then(generation2d_retire_displaced),
        Generation2dMutation::UpdateCamera(payload) => {
            if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
                return Err("generation2d-replay.camera-nonfinite");
            }
            generation2d_retire_displaced(Generation2dReplayDisplaced::Camera(std::mem::replace(&mut snapshot.fixture.camera, flow::CameraJson { x: payload.camera.x, y: payload.camera.y, zoom: payload.camera.zoom })))
        }
        Generation2dMutation::ChangeSchema(payload) => generation2d_retire_displaced(Generation2dReplayDisplaced::Text(std::mem::replace(&mut snapshot.fixture.schema, generation2d_copy_string(&payload.schema)?))),
        Generation2dMutation::CreateGeneration(payload) => {
            if snapshot.generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
                return Err("generation2d-replay.generation-duplicate");
            }
            let mut selected = String::new();
            selected.try_reserve_exact(payload.generation.id.len()).map_err(|_| "generation2d-replay.selected-generation-preflight")?;
            for character in payload.generation.id.chars() {
                selected.push(character);
            }
            snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(generation2d_copy_generation(&payload.generation)?);
            snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some(selected);
            None
        }
        Generation2dMutation::DeleteGeneration(payload) => {
            let index = snapshot.generation.generations.iter().position(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            let removed = snapshot.generation.cold_builder_mut()?.generations.remove(index);
            if snapshot.generation.selected_generation_id.as_deref() == Some(payload.id.as_str()) {
                let mut selected = None;
                if let Some(first) = snapshot.generation.generations.first() {
                    let mut id = String::new();
                    id.try_reserve_exact(first.id.len()).map_err(|_| "generation2d-replay.selected-generation-preflight")?;
                    for character in first.id.chars() {
                        id.push(character);
                    }
                    selected = Some(id);
                }
                snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = selected;
            }
            generation2d_retire_displaced(Generation2dReplayDisplaced::Generation(removed))
        }
        Generation2dMutation::RenameGeneration(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            generation2d_retire_displaced(Generation2dReplayDisplaced::Text(std::mem::replace(&mut entry.name, generation2d_copy_string(&payload.name)?)))
        }
        Generation2dMutation::ChangeGenerationValue(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation2d-replay.generation-missing")?;
            let copied = generation2d_copy_json(&dsl::DslValue::from(&payload.value), 0)?;
            entry.values.insert(generation2d_copy_string(&payload.question_id)?, serde_json::Value::from(&copied)).map(|displaced| Generation2dReplayDisplaced::Json(dsl::DslValue::from(&displaced))).and_then(generation2d_retire_displaced)
        }
    };
    Ok(retired)
}
//#endregion 🔖️RetainedMountedIngress

//#region 🔖️TypedOwnedEnvelopeCatalog
const GENERATION2D_ENVELOPE_SNAPSHOT_PACK_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

struct Generation2dRetainedSnapshotRetirement {
    value: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    flow: flow::retained::FlowRetirement,
    generation: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl store::ErasedSnapshotRetirement for Generation2dRetainedSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(generation) = self.generation.as_mut() {
            let step = generation.close_step(maximum_items, maximum_bytes)?;
            if matches!(step, store::SnapshotRetirementStep::Complete) { self.generation.take(); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: match step { store::SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, _ => 0 } });
        }
        if !self.flow.is_empty() {
            return self.flow.close_step(maximum_items, maximum_bytes);
        }
        if let Some(value) = self.value.take() {
            self.flow.push(flow::retained::FlowOwner::Fixture(value.fixture));
            *self.generation = Some(Box::new(value.generation.into_retirement()));
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.flow.is_empty() && self.generation.is_none()
    }
}

impl Drop for Generation2dRetainedSnapshotRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() { assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Generation2d snapshot reached Drop before typed retirement"); }
    }
}

struct Generation2dRetainedSnapshotRetirementFactory;

pub(crate) fn generation2d_retire_owned_snapshot(value: Generation2dSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
    Box::new(Generation2dRetainedSnapshotRetirement { value: std::mem::ManuallyDrop::new(Some(value)), flow: Default::default(), generation: std::mem::ManuallyDrop::new(None) })
}

impl store::ArtifactOwnedValueRetirementFactory<Generation2dSnapshot> for Generation2dRetainedSnapshotRetirementFactory {
    fn retire_owned(&self, value: Generation2dSnapshot) -> Box<dyn store::ErasedSnapshotRetirement> {
        generation2d_retire_owned_snapshot(value)
    }
}

struct Generation2dRetainedSnapshotArcRetirement {
    value: std::mem::ManuallyDrop<Option<std::sync::Arc<Generation2dSnapshot>>>,
    owned: Generation2dRetainedSnapshotRetirement,
}

impl store::ErasedSnapshotRetirement for Generation2dRetainedSnapshotArcRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(value) = self.value.take() {
            *self.owned.value = std::sync::Arc::into_inner(value);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.owned.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.owned.terminal_is_empty()
    }
}

impl Drop for Generation2dRetainedSnapshotArcRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() { assert!(store::ErasedSnapshotRetirement::terminal_is_empty(self), "Generation2d Arc snapshot reached Drop before retained close"); }
    }
}

impl store::SnapshotRetirementFactory<Generation2dSnapshot> for Generation2dRetainedSnapshotRetirementFactory {
    fn retire(&self, snapshot: std::sync::Arc<Generation2dSnapshot>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Generation2dRetainedSnapshotArcRetirement {
            value: std::mem::ManuallyDrop::new(Some(snapshot)),
            owned: Generation2dRetainedSnapshotRetirement { value: std::mem::ManuallyDrop::new(None), flow: Default::default(), generation: std::mem::ManuallyDrop::new(None) },
        })
    }
}

pub fn generation2d_document_store_owners() -> store::MemberStoreOwners<Generation2dSnapshot, Generation2dMutation> {
    store::MemberStoreOwners::new(
        std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory),
        std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory),
        std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory),
        Box::new(store::ArtifactStoreCursorDisposer::<Generation2dSnapshot, Generation2dMutation>::new()),
    )
}

struct Generation2dRetainedMutationRetirement {
    value: std::mem::ManuallyDrop<Option<Generation2dMutation>>,
}

impl store::ErasedSnapshotRetirement for Generation2dRetainedMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.value.is_none() {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < store::ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        drop(self.value.take());
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }
}

impl Drop for Generation2dRetainedMutationRetirement {
    fn drop(&mut self) {
        assert!(self.value.is_none(), "fresh Generation2d mutation retirement fail-closed with an impossible populated-history owner");
    }
}

struct Generation2dRetainedMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<Generation2dMutation> for Generation2dRetainedMutationRetirementFactory {
    fn retire_owned(&self, value: Generation2dMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Generation2dRetainedMutationRetirement { value: std::mem::ManuallyDrop::new(Some(value)) })
    }
}

#[derive(Default)]
struct Generation2dMutationWidgetOwner {
    keyword: String,
    strings: [String; 4],
    numbers: [f64; 4],
    boolean: bool,
    lists: [Vec<String>; 2],
    dictionaries: [flow::neural::Dictionary; 2],
    dynamic: [Option<dsl::DslValue>; 2],
}

#[derive(Default)]
struct Generation2dMutationSynapseOwner {
    id: String,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
}

#[derive(Default)]
struct Generation2dMutationDictionaryEntryOwner {
    key: String,
    value: Option<flow::neural::Value>,
}

#[derive(Clone, Copy)]
enum Generation2dMutationDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

enum Generation2dMutationFrame {
    Root { field: Option<u16> },
    Statements { keyword: Option<String> },
    Widget { field: Option<u16>, owner: Generation2dMutationWidgetOwner },
    Synapse { field: Option<u16>, owner: Generation2dMutationSynapseOwner },
    Layout { field: Option<u16>, value: flow::WidgetLayout },
    Camera { field: Option<u16>, value: flow::CameraJson },
    Generation { field: Option<u16>, id: String, name: String, values: Vec<(String, dsl::DslValue)> },
    Dictionary { destination: Generation2dMutationDictionaryDestination, rows: Vec<Generation2dMutationDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    NeuralValue { table: usize, row: usize, field: Option<u16>, value: Option<flow::neural::Value> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Wire { parent: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural(store::mounted_pack_rt::RetainedValueContainer),
}

#[derive(Clone, Copy)]
enum Generation2dMutationStringTarget {
    Root(u16),
    Widget(usize, u16),
    Generation(usize, u16),
    DictionaryKey(usize, usize),
    NeuralText(usize),
    Sequence(usize),
    Statement(usize),
    SynapseId(usize),
    Wire(usize, u8),
    JsonKey,
    JsonValue,
    DslKey,
    DslValue,
}

enum Generation2dMutationJsonFrame {
    Array(Vec<dsl::DslValue>),
    Object { values: Vec<(String, dsl::DslValue)>, key: Option<String> },
}

enum Generation2dMutationDslFrame {
    Array(Vec<dsl::DslValue>),
    Object { values: Vec<(String, dsl::DslValue)>, key: Option<String> },
}

#[derive(Clone, Copy)]
enum Generation2dMutationJsonDestination {
    ChangeValue,
    Generation(usize),
}

struct Generation2dMutationStringOwner {
    target: Generation2dMutationStringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

/// 🧬️ Fixed-depth typed owner for the exact fourteen Generation2d mutation records.
/// Dynamic JSON is admitted only at the one ChangeGenerationValue value leaf.
struct Generation2dRetainedMutationOwner {
    ordinal: u8,
    stack: Vec<Generation2dMutationFrame>,
    string: Option<Generation2dMutationStringOwner>,
    strings: [String; 3],
    index: usize,
    widget: Option<flow::Widget>,
    synapse: Option<flow::SynapseSpec>,
    layout: Option<flow::WidgetLayout>,
    camera: Option<flow::CameraJson>,
    generation: Option<flow::playbook::FormGeneration>,
    json: dsl::DslValue,
    json_stack: Vec<Generation2dMutationJsonFrame>,
    json_destination: Option<Generation2dMutationJsonDestination>,
    dsl_stack: Vec<Generation2dMutationDslFrame>,
    dsl_destination: Option<(usize, usize)>,
    pending_table_rows: Option<u64>,
    value: std::mem::ManuallyDrop<Option<Generation2dMutation>>,
    complete: bool,
    handed_back: bool,
}

impl Generation2dRetainedMutationOwner {
    fn new(ordinal: u8) -> Result<Self, &'static str> {
        if usize::from(ordinal) >= GENERATION2D_MUTATION_VARIANT_COUNT {
            return Err("generation2d-mutation.variant");
        }
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION2D_RETAINED_COMBINED_DEPTH).map_err(|_| "generation2d-mutation.stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION2D_RETAINED_COMBINED_DEPTH).map_err(|_| "generation2d-mutation.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION2D_RETAINED_COMBINED_DEPTH).map_err(|_| "generation2d-mutation.dsl-stack-preflight")?;
        Ok(Self {
            ordinal,
            stack,
            string: None,
            strings: std::array::from_fn(|_| String::new()),
            index: 0,
            widget: None,
            synapse: None,
            layout: None,
            camera: None,
            generation: None,
            json: dsl::DslValue::Null,
            json_stack,
            json_destination: None,
            dsl_stack,
            dsl_destination: None,
            pending_table_rows: None,
            value: std::mem::ManuallyDrop::new(None),
            complete: false,
            handed_back: false,
        })
    }

    fn push(&mut self, frame: Generation2dMutationFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation2d-mutation.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    fn root_field(&self) -> Option<u16> {
        self.stack.iter().find_map(|frame| match frame {
            Generation2dMutationFrame::Root { field } => *field,
            _ => None,
        })
    }

    fn string_target(&mut self) -> Result<Generation2dMutationStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.string-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation2dMutationJsonFrame::Object { key: None, .. }) => Generation2dMutationStringTarget::JsonKey,
                _ => Generation2dMutationStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation2dMutationDslFrame::Object { key: None, .. }) => Generation2dMutationStringTarget::DslKey,
                _ => Generation2dMutationStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation2dMutationFrame::Root { field: Some(field) } => Ok(Generation2dMutationStringTarget::Root(*field)),
            Generation2dMutationFrame::Statements { keyword: None } => Ok(Generation2dMutationStringTarget::Statement(index)),
            Generation2dMutationFrame::Widget { field: Some(field), .. } => Ok(Generation2dMutationStringTarget::Widget(index, *field)),
            Generation2dMutationFrame::Generation { field: Some(field), .. } => Ok(Generation2dMutationStringTarget::Generation(index, *field)),
            Generation2dMutationFrame::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mutation.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation2dMutationStringTarget::DictionaryKey(index, row))
            }
            Generation2dMutationFrame::NeuralValue { field: Some(4), .. } => Ok(Generation2dMutationStringTarget::NeuralText(index)),
            Generation2dMutationFrame::Strings { .. } => Ok(Generation2dMutationStringTarget::Sequence(index)),
            Generation2dMutationFrame::Synapse { field: Some(0), .. } => Ok(Generation2dMutationStringTarget::SynapseId(index)),
            Generation2dMutationFrame::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation2dMutationStringTarget::Wire(index, target))
            }
            _ => Err("generation2d-mutation.string-role"),
        }
    }

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation2d-mutation.string-overlap");
        }
        self.string = Some(Generation2dMutationStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let characters = body.symbol_chars(symbol).map_err(|_| "generation2d-mutation.symbol")?;
        let owner = self.string.as_mut().expect("P2 mutation string retained");
        owner.value.try_reserve_exact(characters).map_err(|_| "generation2d-mutation.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, characters));
        if characters == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    fn grant_symbol(&mut self, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, characters)) = owner.symbol else { return Ok(false) };
        owner.value.push(body.symbol_char(symbol, index).map_err(|_| "generation2d-mutation.symbol-char")?.ok_or("generation2d-mutation.symbol-short")?);
        if index + 1 == characters {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P2 mutation symbol retained").symbol = Some((symbol, index + 1, characters));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation2d-mutation.string-handoff")?;
        match owner.target {
            Generation2dMutationStringTarget::Root(field) => {
                let slot = match (self.ordinal, field) {
                    (2 | 5 | 7 | 9 | 11, 0) => 0,
                    (12 | 13, 0) => 0,
                    (12 | 13, 1) => 1,
                    _ => return Err("generation2d-mutation.root-string-field"),
                };
                self.strings[slot] = owner.value;
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMutationStringTarget::Statement(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Statements { keyword }) => *keyword = Some(owner.value),
                _ => return Err("generation2d-mutation.statement-owner"),
            },
            Generation2dMutationStringTarget::Widget(index, field) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Widget { field: active, owner: widget }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation2d-mutation.widget-string")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation2d-mutation.widget-owner"),
            },
            Generation2dMutationStringTarget::Generation(index, field) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Generation { field: active, id, name, .. }) => {
                    if field == 0 {
                        *id = owner.value;
                    } else if field == 1 {
                        *name = owner.value;
                    } else {
                        return Err("generation2d-mutation.generation-string");
                    }
                    *active = None;
                }
                _ => return Err("generation2d-mutation.generation-owner"),
            },
            Generation2dMutationStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mutation.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation2d-mutation.dictionary-key-owner"),
            },
            Generation2dMutationStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(flow::neural::Value::Atom(flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.neural-text-owner"),
            },
            Generation2dMutationStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation2d-mutation.sequence-owner"),
            },
            Generation2dMutationStringTarget::SynapseId(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Synapse { field, owner: synapse }) => {
                    synapse.id = owner.value;
                    *field = None;
                }
                _ => return Err("generation2d-mutation.synapse-owner"),
            },
            Generation2dMutationStringTarget::Wire(index, role) => {
                let parent = match self.stack.get(index) {
                    Some(Generation2dMutationFrame::Wire { parent, .. }) => *parent,
                    _ => return Err("generation2d-mutation.wire-owner"),
                };
                let synapse = match self.stack.get_mut(parent) {
                    Some(Generation2dMutationFrame::Synapse { owner, .. }) => owner,
                    _ => return Err("generation2d-mutation.wire-parent"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    1 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    4 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            Generation2dMutationStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation2dMutationJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mutation.json-key-owner"),
            },
            Generation2dMutationStringTarget::JsonValue => self.assign_json(dsl::DslValue::String(owner.value))?,
            Generation2dMutationStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation2dMutationDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mutation.dsl-key-owner"),
            },
            Generation2dMutationStringTarget::DslValue => self.assign_dsl(dsl::DslValue::String(owner.value))?,
        }
        Ok(())
    }

    fn assign_json(&mut self, value: dsl::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation2dMutationJsonFrame::Array(values)) => values.push(value),
            Some(Generation2dMutationJsonFrame::Object { values, key }) => {
                let key = key.take().ok_or("generation2d-mutation.json-value-key")?;
                values.push((key, value));
            }
            None => match self.json_destination.take().ok_or("generation2d-mutation.json-destination")? {
                Generation2dMutationJsonDestination::ChangeValue => {
                    self.json = value;
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::Root { field }) if *field == Some(2) => *field = None,
                        _ => return Err("generation2d-mutation.change-value-owner"),
                    }
                }
                Generation2dMutationJsonDestination::Generation(index) => {
                    let values = match value {
                        dsl::DslValue::Object(values) => values,
                        _ => return Err("generation2d-mutation.generation-values-shape"),
                    };
                    match self.stack.get_mut(index) {
                        Some(Generation2dMutationFrame::Generation { field, values: target, .. }) => {
                            *target = values;
                            *field = None;
                        }
                        _ => return Err("generation2d-mutation.generation-values-owner"),
                    }
                }
            },
        }
        Ok(())
    }

    fn begin_json(&mut self, destination: Generation2dMutationJsonDestination) -> Result<(), &'static str> {
        if self.json_destination.is_some() {
            return Err("generation2d-mutation.json-overlap");
        }
        self.json_destination = Some(destination);
        Ok(())
    }

    fn assign_dsl(&mut self, value: dsl::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation2dMutationDslFrame::Array(values)) => values.push(value),
            Some(Generation2dMutationDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation2d-mutation.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation2d-mutation.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation2dMutationFrame::Widget { field, owner }) if *field == Some((slot + 2) as u16) => {
                        owner.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation2d-mutation.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    fn begin_dsl(&mut self) -> Result<bool, &'static str> {
        if self.dsl_destination.is_some() {
            return Ok(true);
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return Ok(false) };
        let field = match self.stack.get(parent) {
            Some(Generation2dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
            _ => return Ok(false),
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        Ok(true)
    }

    fn end_dsl(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation2d-mutation.dsl-end")? {
            Generation2dMutationDslFrame::Array(values) if kind == store::mounted_pack_rt::RetainedValueContainer::List => dsl::DslValue::Array(values),
            Generation2dMutationDslFrame::Object { values, key: None } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => dsl::DslValue::Object(values),
            _ => return Err("generation2d-mutation.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    fn begin_dictionary(&mut self, destination: Generation2dMutationDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation2d-mutation.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation2d-mutation.dictionary-preflight")?;
        values.resize_with(rows, Generation2dMutationDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation2d-mutation.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation2dMutationFrame::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Generation2dMutationDictionaryDestination, rows: Vec<Generation2dMutationDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation2d-mutation.dictionary-value")?);
        }
        match destination {
            Generation2dMutationDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation2dMutationFrame::Widget { field: active, owner }) if *active == Some(field) => {
                    owner.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation2d-mutation.dictionary-widget-owner"),
            },
            Generation2dMutationDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    fn finish_widget(owner: Generation2dMutationWidgetOwner) -> Result<flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => flow::Widget::InputSlider { id, label: second, value, min, max, step },
            "input-note" => flow::Widget::InputNote { id, text: second },
            "input-image" => flow::Widget::InputImage { id, src: second },
            "variable" => flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = flow::OrderedSet::new();
                for entry in first_list {
                    expanded.insert(entry);
                }
                flow::Widget::OutputPreview { id, preview: second_dictionary, expanded }
            }
            "output-action" => flow::Widget::OutputAction { id, action: second },
            "output-export" => flow::Widget::OutputExport { id, format: second },
            "cluster" => flow::Widget::Cluster {
                id,
                name: second,
                tree: dsl::from_dsl_value(first_dynamic.ok_or("generation2d-mutation.cluster-tree")?).map_err(|_| "generation2d-mutation.cluster-tree-shape")?,
                flow: dsl::from_dsl_value(second_dynamic.ok_or("generation2d-mutation.cluster-flow")?).map_err(|_| "generation2d-mutation.cluster-flow-shape")?,
            },
            _ => return Err("generation2d-mutation.widget-variant"),
        })
    }

    fn begin_record(&mut self) -> Result<(), &'static str> {
        if self.stack.is_empty() {
            return self.push(Generation2dMutationFrame::Root { field: None });
        }
        let table = self.stack.len() - 1;
        if let Some(Generation2dMutationFrame::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
            let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mutation.dictionary-value-row")?;
            *next = row + 1;
            return self.push(Generation2dMutationFrame::NeuralValue { table, row, field: None, value: None });
        }
        let root = self.root_field();
        let frame = match self.stack.last_mut() {
            Some(Generation2dMutationFrame::Statements { keyword }) => {
                let keyword = keyword.take().ok_or("generation2d-mutation.widget-keyword")?;
                Generation2dMutationFrame::Widget { field: None, owner: Generation2dMutationWidgetOwner { keyword, ..Default::default() } }
            }
            _ => match (self.ordinal, root) {
                (3, Some(1)) | (4, Some(0)) => Generation2dMutationFrame::Synapse { field: None, owner: Default::default() },
                (6, Some(1)) => Generation2dMutationFrame::Layout { field: None, value: flow::WidgetLayout { x: 0.0, y: 0.0 } },
                (8, Some(0)) => Generation2dMutationFrame::Camera { field: None, value: flow::CameraJson::default() },
                (10, Some(0)) => Generation2dMutationFrame::Generation { field: None, id: String::new(), name: String::new(), values: Vec::new() },
                _ => Generation2dMutationFrame::Structural(store::mounted_pack_rt::RetainedValueContainer::Record),
            },
        };
        self.push(frame)
    }

    fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        use store::mounted_pack_rt::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: 0x11, .. } => {
                if self.dsl_destination.is_some() {
                    self.begin_dsl()?;
                } else if self.json_destination.is_none() {
                    if self.ordinal == 13 && self.root_field() == Some(2) {
                        self.begin_json(Generation2dMutationJsonDestination::ChangeValue)?;
                    } else {
                        self.begin_dsl()?;
                    }
                }
            }
            Token::Begin { kind: Container::List, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.dsl-count")?).map_err(|_| "generation2d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation2dMutationDslFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.dsl-count")?).map_err(|_| "generation2d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation2dMutationDslFrame::Object { values, key: None });
            }
            Token::Begin { kind: Container::List, count } if self.json_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.json-list-count")?).map_err(|_| "generation2d-mutation.json-list-preflight")?;
                self.json_stack.push(Generation2dMutationJsonFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, .. } if self.json_destination.is_some() => {
                self.json_stack.push(Generation2dMutationJsonFrame::Object { values: Vec::new(), key: None });
            }
            Token::Begin { kind: Container::Map, .. } => {
                let index = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.generation-values-owner")?;
                if matches!(self.stack.get(index), Some(Generation2dMutationFrame::Generation { field: Some(2), .. })) {
                    self.begin_json(Generation2dMutationJsonDestination::Generation(index))?;
                    self.json_stack.push(Generation2dMutationJsonFrame::Object { values: Vec::new(), key: None });
                } else {
                    self.push(Generation2dMutationFrame::Structural(Container::Map))?;
                }
            }
            Token::Begin { kind: Container::Table, count } => {
                if self.pending_table_rows.take() != Some(count) {
                    return Err("generation2d-mutation.table-row-count");
                }
                let parent = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Generation2dMutationFrame::Widget { field: Some(field @ (1 | 5)), .. }) => Generation2dMutationDictionaryDestination::Widget { parent, field: *field },
                    Some(Generation2dMutationFrame::NeuralValue { field: Some(5), .. }) => Generation2dMutationDictionaryDestination::Value { parent },
                    _ => {
                        self.push(Generation2dMutationFrame::Structural(Container::Table))?;
                        return Ok(());
                    }
                };
                self.begin_dictionary(destination, count)?;
            }
            Token::Begin { kind: Container::Record, .. } => self.begin_record()?,
            Token::Begin { kind: Container::Statements, .. } => self.push(Generation2dMutationFrame::Statements { keyword: None })?,
            Token::Begin { kind: Container::List | Container::Tuple, count } => {
                let (parent, field) = match self.stack.last() {
                    Some(Generation2dMutationFrame::Widget { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    _ => {
                        self.push(Generation2dMutationFrame::Structural(Container::List))?;
                        return Ok(());
                    }
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "generation2d-mutation.sequence-preflight")?;
                self.push(Generation2dMutationFrame::Strings { parent, field, values })?;
            }
            Token::Begin { kind: Container::Wire, .. } => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.wire-parent")?;
                self.push(Generation2dMutationFrame::Wire { parent, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })?;
            }
            Token::Begin { kind, .. } => self.push(Generation2dMutationFrame::Structural(kind))?,
            Token::Unsigned { role: Role::FieldId, value } if value <= u16::MAX as u64 => match self.stack.last_mut() {
                Some(
                    Generation2dMutationFrame::Root { field }
                    | Generation2dMutationFrame::Widget { field, .. }
                    | Generation2dMutationFrame::Synapse { field, .. }
                    | Generation2dMutationFrame::Layout { field, .. }
                    | Generation2dMutationFrame::Camera { field, .. }
                    | Generation2dMutationFrame::Generation { field, .. }
                    | Generation2dMutationFrame::NeuralValue { field, .. },
                ) if field.is_none() => *field = Some(value as u16),
                _ => return Err("generation2d-mutation.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { field, present, next, .. }) => {
                    *field = Some(u16::try_from(value).map_err(|_| "generation2d-mutation.dictionary-field")?);
                    present.fill(false);
                    *next = 0;
                }
                _ => {}
            },
            Token::Unsigned { role: Role::Unsigned, value } if self.json_destination.is_none() && self.dsl_destination.is_none() => {
                self.index = usize::try_from(value).map_err(|_| "generation2d-mutation.index")?;
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation2d-mutation.string-length")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation2d-mutation.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(character) => {
                let owner = self.string.as_mut().ok_or("generation2d-mutation.string-char")?;
                owner.value.push(character);
                let remaining = owner.remaining.as_mut().ok_or("generation2d-mutation.string-width")?;
                *remaining = remaining.checked_sub(character.len_utf8() as u64).ok_or("generation2d-mutation.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, body)?,
            Token::F64(bits) => {
                if self.dsl_destination.is_some() {
                    self.assign_dsl(dsl::DslValue::float(f64::from_bits(bits)))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(dsl::DslValue::float(f64::from_bits(bits)))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(3) && value.is_none() => {
                            *value = Some(flow::neural::Value::Atom(flow::neural::Atom::Decimal(f64::from_bits(bits))));
                            *field = None;
                        }
                        Some(Generation2dMutationFrame::Widget { field, owner }) => {
                            let field = field.take().ok_or("generation2d-mutation.widget-number")?;
                            *owner.numbers.get_mut(field.checked_sub(2).ok_or("generation2d-mutation.widget-number-field")? as usize).ok_or("generation2d-mutation.widget-number-field")? = f64::from_bits(bits);
                        }
                        Some(Generation2dMutationFrame::Layout { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            _ => return Err("generation2d-mutation.layout-field"),
                        },
                        Some(Generation2dMutationFrame::Camera { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            Some(2) => value.zoom = f64::from_bits(bits),
                            _ => return Err("generation2d-mutation.camera-field"),
                        },
                        _ => return Err("generation2d-mutation.number-owner"),
                    }
                }
            }
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(dsl::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::NeuralValue { field, value: target, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(flow::neural::Value::Atom(flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::uint(value))?,
            Token::Tag { value: 0x01 | 0x02, .. } => {
                let boolean = matches!(token, Token::Tag { value: 0x02, .. });
                if self.dsl_destination.is_some() {
                    self.assign_dsl(dsl::DslValue::Bool(boolean))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(dsl::DslValue::Bool(boolean))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::Widget { field, owner }) => {
                            owner.boolean = boolean;
                            *field = None;
                        }
                        Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if matches!(*field, Some(0 | 1)) && value.is_none() => {
                            *value = Some(if *field == Some(0) { flow::neural::Value::Atom(flow::neural::Atom::Null) } else { flow::neural::Value::Atom(flow::neural::Atom::Boolean(boolean)) });
                            *field = None;
                        }
                        _ => return Err("generation2d-mutation.boolean-owner"),
                    }
                }
            }
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(dsl::DslValue::Null)?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(dsl::DslValue::Null)?,
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Wire { roles, roles_len, nodes, .. }) => {
                    let base = if *nodes == 0 { 0 } else { 3 };
                    roles[*roles_len] = base;
                    *roles_len += 1;
                    if presence & 1 != 0 {
                        roles[*roles_len] = base + 1;
                        *roles_len += 1;
                    }
                    if presence & 2 != 0 {
                        roles[*roles_len] = base + 2;
                        *roles_len += 1;
                    }
                    *nodes += 1;
                }
                _ => return Err("generation2d-mutation.wire-node"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len() => {
                    if value == 0 {
                        present.fill(true);
                    }
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { present, .. }) => {
                    for bit in 0..8 {
                        let row = first_row as usize + bit;
                        if row < present.len() {
                            present[row] = value & (1 << bit) != 0;
                        }
                    }
                }
                _ => {}
            },
            Token::End(kind) => {
                if self.end_dsl(kind)? {
                    return Ok(());
                }
                if self.json_destination.is_some() && matches!(kind, Container::List | Container::Map) {
                    let value = match self.json_stack.pop().ok_or("generation2d-mutation.json-end-owner")? {
                        Generation2dMutationJsonFrame::Array(values) if kind == Container::List => dsl::DslValue::Array(values),
                        Generation2dMutationJsonFrame::Object { values, key: None } if kind == Container::Map => dsl::DslValue::Object(values),
                        _ => return Err("generation2d-mutation.json-end-mismatch"),
                    };
                    self.assign_json(value)?;
                    return Ok(());
                }
                let frame = self.stack.pop().ok_or("generation2d-mutation.end-owner")?;
                match frame {
                    Generation2dMutationFrame::Root { field: None } if kind == Container::Record => {}
                    Generation2dMutationFrame::Widget { field: None, owner } if kind == Container::Record => self.widget = Some(Self::finish_widget(owner)?),
                    Generation2dMutationFrame::Synapse { field: None, owner } if kind == Container::Record => {
                        self.synapse = Some(flow::SynapseSpec { id: owner.id, from: owner.from, to: owner.to, from_port: owner.from_port, to_port: owner.to_port });
                    }
                    Generation2dMutationFrame::Layout { field: None, value } if kind == Container::Record => self.layout = Some(value),
                    Generation2dMutationFrame::Camera { field: None, value } if kind == Container::Record => self.camera = Some(value),
                    Generation2dMutationFrame::Generation { field: None, id, name, values } if kind == Container::Record => {
                        self.generation = Some(flow::playbook::FormGeneration { id, name, values: values.into_iter().map(|(key, value)| (key, serde_json::Value::from(&value))).collect() });
                    }
                    Generation2dMutationFrame::NeuralValue { table, row, field: None, value: Some(value) } if kind == Container::Record => match self.stack.get_mut(table) {
                        Some(Generation2dMutationFrame::Dictionary { rows, field: Some(1), .. }) => {
                            rows.get_mut(row).ok_or("generation2d-mutation.dictionary-value-row")?.value = Some(value);
                        }
                        _ => return Err("generation2d-mutation.dictionary-value-table"),
                    },
                    Generation2dMutationFrame::Dictionary { destination, rows, field: Some(1), .. } if kind == Container::Table => self.finish_dictionary(destination, rows)?,
                    Generation2dMutationFrame::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                        Some(Generation2dMutationFrame::Widget { field: active, owner }) => {
                            owner.lists[if field == 4 { 1 } else { 0 }] = values;
                            *active = None;
                        }
                        _ => return Err("generation2d-mutation.sequence-parent"),
                    },
                    Generation2dMutationFrame::Wire { parent, .. } if kind == Container::Wire => {
                        if let Some(Generation2dMutationFrame::Synapse { field, .. }) = self.stack.get_mut(parent) {
                            *field = None;
                        }
                    }
                    Generation2dMutationFrame::Statements { .. } if kind == Container::Statements => {}
                    Generation2dMutationFrame::Structural(expected) if expected == kind => {}
                    _ => return Err("generation2d-mutation.end-mismatch"),
                }
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() || self.json_destination.is_some() || !self.json_stack.is_empty() || self.dsl_destination.is_some() || !self.dsl_stack.is_empty() || self.pending_table_rows.is_some() {
                    return Err("generation2d-mutation.terminal-populated");
                }
                use crate::artifacts::generation2d::mutations::*;
                let strings = std::mem::replace(&mut self.strings, std::array::from_fn(|_| String::new()));
                let [first, second, _third] = strings;
                let mutation = match self.ordinal {
                    0 => create_widget(self.index, self.widget.take().ok_or("generation2d-mutation.create-widget")?),
                    1 => replace_widget(self.widget.take().ok_or("generation2d-mutation.replace-widget")?),
                    2 => delete_widget(first),
                    3 => connect_synapse(self.index, self.synapse.take().ok_or("generation2d-mutation.connect-synapse")?),
                    4 => replace_synapse(self.synapse.take().ok_or("generation2d-mutation.replace-synapse")?),
                    5 => disconnect_synapse(first),
                    6 => move_widget(first, self.layout.take().ok_or("generation2d-mutation.move-widget")?),
                    7 => clear_widget_layout(first),
                    8 => update_camera(self.camera.take().ok_or("generation2d-mutation.update-camera")?),
                    9 => change_schema(first),
                    10 => create_generation(self.generation.take().ok_or("generation2d-mutation.create-generation")?),
                    11 => delete_generation(first),
                    12 => rename_generation(first, second),
                    13 => change_generation_value(first, second, serde_json::Value::from(&std::mem::replace(&mut self.json, dsl::DslValue::Null))),
                    _ => return Err("generation2d-mutation.variant"),
                };
                *self.value = Some(mutation);
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Signed(_) | Token::Byte(_) | Token::WireLabelPresence(_) | Token::TablePresence { .. } | Token::TableBitmap { .. } => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Generation2dMutation> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    fn close_step(&mut self) -> bool {
        self.string = None;
        if self.stack.pop().is_some() {
            return false;
        }
        drop(self.value.take());
        drop(self.widget.take());
        drop(self.synapse.take());
        drop(self.layout.take());
        drop(self.camera.take());
        drop(self.generation.take());
        self.json_stack.clear();
        self.json_destination = None;
        self.dsl_stack.clear();
        self.dsl_destination = None;
        self.pending_table_rows = None;
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back
            && self.value.is_none()
            && self.stack.is_empty()
            && self.string.is_none()
            && self.widget.is_none()
            && self.synapse.is_none()
            && self.layout.is_none()
            && self.camera.is_none()
            && self.generation.is_none()
            && self.json_stack.is_empty()
            && self.json_destination.is_none()
            && self.dsl_stack.is_empty()
            && self.dsl_destination.is_none()
            && self.pending_table_rows.is_none()
    }
}

impl Drop for Generation2dRetainedMutationOwner {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Generation2d retained mutation owner reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation2dMutationSessionPhase {
    Format,
    Ordinal,
    Body,
    Ready,
    Published,
    Closing,
    Closed,
}

struct Generation2dMutationSession {
    phase: Generation2dMutationSessionPhase,
    expected_bytes: usize,
    maximum_items: usize,
    admitted: usize,
    pending: Option<(u64, u8)>,
    ordinal: u64,
    ordinal_shift: u32,
    ordinal_bytes: u8,
    body_bytes: u64,
    sealed: bool,
    body: std::mem::ManuallyDrop<Option<store::mounted_pack_rt::RetainedRecordBodyCursor>>,
    owner: std::mem::ManuallyDrop<Option<Generation2dRetainedMutationOwner>>,
}

impl Generation2dMutationSession {
    fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if expected_bytes < 3 || expected_bytes > GENERATION2D_OWNER_BYTES || maximum_items == 0 {
            return Err("generation2d-mutation.exact-credits");
        }
        Ok(Self {
            phase: Generation2dMutationSessionPhase::Format,
            expected_bytes,
            maximum_items,
            admitted: 0,
            pending: None,
            ordinal: 0,
            ordinal_shift: 0,
            ordinal_bytes: 0,
            body_bytes: 0,
            sealed: false,
            body: std::mem::ManuallyDrop::new(None),
            owner: std::mem::ManuallyDrop::new(None),
        })
    }

    fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if self.pending.is_some() || self.sealed || self.admitted == self.expected_bytes {
            return Err(value);
        }
        self.pending = Some((self.admitted as u64, value));
        self.admitted += 1;
        Ok(())
    }

    fn ingress_ready(&self) -> bool {
        self.pending.is_none()
    }

    fn seal(&mut self) -> Result<(), &'static str> {
        if self.pending.is_some() || self.admitted != self.expected_bytes || self.phase != Generation2dMutationSessionPhase::Body {
            return Err("generation2d-mutation.exact-byte-seal");
        }
        self.body.as_mut().ok_or("generation2d-mutation.body-owner")?.seal(self.body_bytes).map_err(|_| "generation2d-mutation.body-seal")?;
        self.sealed = true;
        Ok(())
    }

    fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Generation2dMutationSessionPhase::Ready | Generation2dMutationSessionPhase::Published) {
            return Ok(true);
        }
        if let (Some(owner), Some(body)) = (self.owner.as_mut(), self.body.as_ref()) {
            if owner.grant_symbol(body)? {
                return Ok(false);
            }
        }
        match self.phase {
            Generation2dMutationSessionPhase::Format => {
                let (_, byte) = self.pending.take().ok_or("generation2d-mutation.format-input")?;
                if byte != dsl::variants_binary::OP_BINARY_FORMAT {
                    return Err("generation2d-mutation.format");
                }
                self.phase = Generation2dMutationSessionPhase::Ordinal;
            }
            Generation2dMutationSessionPhase::Ordinal => {
                let (_, byte) = self.pending.take().ok_or("generation2d-mutation.ordinal-input")?;
                if self.ordinal_bytes >= 10 || (self.ordinal_bytes == 9 && (byte & 0xfe) != 0) {
                    return Err("generation2d-mutation.ordinal-overflow");
                }
                self.ordinal |= u64::from(byte & 0x7f) << self.ordinal_shift;
                self.ordinal_shift += 7;
                self.ordinal_bytes += 1;
                if byte & 0x80 == 0 {
                    if self.ordinal_bytes > 1 && byte & 0x7f == 0 {
                        return Err("generation2d-mutation.ordinal-noncanonical");
                    }
                    let ordinal = u8::try_from(self.ordinal).map_err(|_| "generation2d-mutation.variant")?;
                    let limits = store::mounted_pack_rt::PackLimits {
                        max_file_len: self.expected_bytes as u64,
                        max_segment_len: self.expected_bytes as u64,
                        max_symbols: self.maximum_items.min(GENERATION2D_MAXIMUM_DOMAIN_ITEMS) as u32,
                        max_depth: GENERATION2D_RETAINED_COMBINED_DEPTH as u16,
                        max_items: self.maximum_items.min(GENERATION2D_MAXIMUM_DOMAIN_ITEMS) as u64,
                        max_total_alloc: GENERATION2D_MAXIMUM_DOMAIN_BYTES as u64,
                    };
                    *self.body = Some(store::mounted_pack_rt::RetainedRecordBodyCursor::try_new(limits).map_err(|_| "generation2d-mutation.body-preflight")?);
                    *self.owner = Some(Generation2dRetainedMutationOwner::new(ordinal)?);
                    self.phase = Generation2dMutationSessionPhase::Body;
                }
            }
            Generation2dMutationSessionPhase::Body => {
                if let Some((_, byte)) = self.pending.take() {
                    self.body.as_mut().ok_or("generation2d-mutation.body-owner")?.admit_byte(self.body_bytes, byte).map_err(|(_, byte)| if byte == 0 { "generation2d-mutation.body-handback-zero" } else { "generation2d-mutation.body-handback" })?;
                    self.body_bytes += 1;
                }
                if let Some(event) = self.body.as_mut().ok_or("generation2d-mutation.body-owner")?.grant().map_err(|_| "generation2d-mutation.body-malformed")? {
                    if let store::mounted_pack_rt::RetainedRecordBodyToken::Value(token) = event {
                        let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                        let body = self.body.as_ref().expect("P2 retained mutation body");
                        self.owner.as_mut().expect("P2 retained mutation owner").accept(token, body)?;
                        if complete {
                            self.phase = Generation2dMutationSessionPhase::Ready;
                            return Ok(true);
                        }
                    }
                }
            }
            _ => return Err("generation2d-mutation.session-state"),
        }
        Ok(false)
    }

    fn take(&mut self) -> Option<Generation2dMutation> {
        if self.phase != Generation2dMutationSessionPhase::Ready {
            return None;
        }
        let value = self.owner.as_mut()?.take()?;
        self.phase = Generation2dMutationSessionPhase::Published;
        Some(value)
    }

    fn close_step(&mut self, maximum_items: usize) -> bool {
        self.phase = Generation2dMutationSessionPhase::Closing;
        self.pending = None;
        if maximum_items == 0 {
            return false;
        }
        if let Some(owner) = self.owner.as_mut() {
            if !owner.close_step() {
                return false;
            }
            drop(self.owner.take());
            return false;
        }
        if let Some(body) = self.body.as_mut() {
            if body.close_step(1) != store::mounted_pack_rt::RetainedPackCloseStep::Complete {
                return false;
            }
            drop(self.body.take());
            return false;
        }
        self.phase = Generation2dMutationSessionPhase::Closed;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == Generation2dMutationSessionPhase::Closed && self.owner.is_none() && self.body.is_none() && self.pending.is_none()
    }
}

impl Drop for Generation2dMutationSession {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Generation2d mutation session reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation2dPackSnapshotState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation2dPackSnapshotAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Generation2dPackSnapshotState,
    token: Option<store::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    session: std::mem::ManuallyDrop<Option<crate::artifacts::generation2d::snapshot::binary::Generation2dMountedPackSession>>,
    value: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Generation2dPackSnapshotAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation2dPackSnapshotState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation2dPackSnapshotState::Published | Generation2dPackSnapshotState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot> for Generation2dPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation2d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation2d-envelope.snapshot-pack-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::AwaitToken {
            if !terminal || token.kind != store::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = generation2d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(crate::artifacts::generation2d::snapshot::binary::Generation2dMountedPackSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation2dPackSnapshotState::Ingest;
        }
        if self.state == Generation2dPackSnapshotState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation2d-envelope.snapshot-token-replayed", token.start));
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation2d-envelope.snapshot-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P2 mounted pack session retained").seal().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Generation2dPackSnapshotState::Drive;
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation2d-envelope.snapshot-pack-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P2 mounted pack session retained").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-handback", retained.start + self.relative as u64))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::Drive {
            cx.set_stage("generation2d-retained-canonical-pack");
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 mounted pack session retained").grant().map_err(|_| self.diagnostic("generation2d-envelope.snapshot-pack-malformed", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P2 mounted pack session retained").take().ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-pack-handoff", token.start))?);
            self.session.as_mut().expect("P2 mounted pack session retained").request_cancel();
            self.state = Generation2dPackSnapshotState::CloseSession;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dPackSnapshotState::CloseSession {
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 mounted pack session retained").close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|_| self.diagnostic("generation2d-envelope.snapshot-session-close", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dPackSnapshotState::Ready;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation2d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<Generation2dSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, Generation2dPackSnapshotState::Ready) {
            return Err(self.diagnostic("generation2d-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation2d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Generation2dPackSnapshotState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str| store::OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path };
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(session) = self.session.as_mut() {
            session.request_cancel();
            if !session.close_step(maximum_items.min(1), maximum_bytes).map_err(|_| diagnostic("generation2d-envelope.snapshot-session-close"))? {
                self.state = Generation2dPackSnapshotState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dPackSnapshotState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&Generation2dRetainedSnapshotRetirementFactory, value));
                self.state = Generation2dPackSnapshotState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = Generation2dPackSnapshotState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement = self.retirement.as_mut().expect("Generation2d snapshot retirement remains retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| diagnostic("generation2d-envelope.snapshot-retirement-fault"))? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = Generation2dPackSnapshotState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(diagnostic("generation2d-envelope.snapshot-retirement-false-terminal")),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation2dPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(self.owners_terminal_empty(), "Generation2d pack snapshot authority reached Drop before publication or bounded retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation2dMutationDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation2dMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Generation2dMutationDecodeState,
    token: Option<store::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    drive_ingress: bool,
    session: std::mem::ManuallyDrop<Option<Generation2dMutationSession>>,
    value: std::mem::ManuallyDrop<Option<Generation2dMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Generation2dMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation2dMutationDecodeState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            drive_ingress: false,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path }
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation2dMutationDecodeState::Published | Generation2dMutationDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<Generation2dMutation> for Generation2dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation2d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation2d-envelope.mutation-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::AwaitToken {
            if !terminal || token.kind != store::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation2d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation2d-envelope.mutation-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation2d-envelope.mutation-pack-length", token.start))?;
            let maximum_items = generation2d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation2d-envelope.mutation-item-authority", token.start))?;
            *self.session = Some(Generation2dMutationSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation2d-envelope.mutation-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation2dMutationDecodeState::Ingest;
        }
        if self.state == Generation2dMutationDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation2d-envelope.mutation-token-replayed", token.start));
            }
            if self.drive_ingress {
                self.session.as_mut().expect("P2 retained mutation session").grant().map_err(|_| self.diagnostic("generation2d-envelope.mutation-ingress-malformed", retained.start + self.relative as u64))?;
                self.drive_ingress = !self.session.as_ref().expect("P2 retained mutation session").ingress_ready();
                cx.consume_fuel(1);
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation2d-envelope.mutation-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P2 retained mutation session").seal().map_err(|_| self.diagnostic("generation2d-envelope.mutation-seal", retained.end))?;
                self.state = Generation2dMutationDecodeState::Drive;
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation2d-envelope.mutation-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P2 retained mutation session").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation2d-envelope.mutation-handback", retained.start + self.relative as u64))?;
                self.drive_ingress = true;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::Drive {
            cx.set_stage("generation2d-retained-mutation");
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 retained mutation session").grant().map_err(|_| self.diagnostic("generation2d-envelope.mutation-malformed", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P2 retained mutation session").take().ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-handoff", token.start))?);
            self.state = Generation2dMutationDecodeState::CloseSession;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation2dMutationDecodeState::CloseSession {
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P2 retained mutation session").close_step(1) {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dMutationDecodeState::Ready;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation2d-envelope.mutation-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<Generation2dMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation2dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation2d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation2d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation2dMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(session) = self.session.as_mut() {
            if !session.close_step(1) {
                self.state = Generation2dMutationDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation2dMutationDecodeState::Closing;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                *self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&Generation2dRetainedMutationRetirementFactory, value));
                self.state = Generation2dMutationDecodeState::Closing;
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.state = Generation2dMutationDecodeState::Complete;
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        let retirement_fault = self.diagnostic("generation2d-envelope.mutation-retirement-fault", 0);
        let retirement_false_terminal = self.diagnostic("generation2d-envelope.mutation-retirement-false-terminal", 0);
        let retirement = self.retirement.as_mut().expect("P2 mutation retirement retained");
        match retirement.close_step(maximum_items, maximum_bytes).map_err(|_| retirement_fault)? {
            store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                drop(self.retirement.take());
                self.state = Generation2dMutationDecodeState::Complete;
                Ok(store::SnapshotRetirementStep::Complete)
            }
            store::SnapshotRetirementStep::Complete => Err(retirement_false_terminal),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation2dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(self.owners_terminal_empty(), "Generation2d mutation authority reached Drop before publication or terminal-empty close");
    }
}

struct Generation2dRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for Generation2dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "generation2d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT })
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, store::OwnedSchemaDecodeDiagnostic> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.terminal = true;
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

/// 🎭️ Owner-local exact catalog for the Generation2d fresh-envelope decode cohort.
pub struct Generation2dEnvelopeOwnedFieldCatalog;

/// 📦️ Installs Generation2d's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn generation2d_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<Generation2dSnapshot, Generation2dMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Generation2dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory), std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory))
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<Generation2dSnapshot, Generation2dMutation> for Generation2dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Generation2dSnapshot, Generation2dMutation>> {
        Box::new(store::ArtifactEnvelopeFreshVcsAuthority::new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(Generation2dRetainedSnapshotRetirementFactory),
            std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory),
            self.edit_history_decoder(),
        ))
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Generation2dSnapshot>> {
        Box::new(Generation2dPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<Generation2dMutation>> {
        Box::new(Generation2dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Generation2dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Generation2dMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(Generation2dRetainedMutationRetirementFactory))
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog

//#region 🔖️RetainedStoreInitialization
fn generation2d_copy_string(source: &str) -> Result<String, &'static str> {
    let mut target = String::new();
    target.try_reserve_exact(source.len()).map_err(|_| "generation2d-initializer.string-preflight")?;
    for character in source.chars() {
        target.push(character);
    }
    Ok(target)
}

fn generation2d_copy_json(source: &dsl::DslValue, depth: usize) -> Result<dsl::DslValue, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.json-depth");
    }
    Ok(match source {
        dsl::DslValue::Null => dsl::DslValue::Null,
        dsl::DslValue::Bool(value) => dsl::DslValue::Bool(*value),
        dsl::DslValue::Number(value) => dsl::DslValue::Number(value.clone()),
        dsl::DslValue::String(value) => dsl::DslValue::String(generation2d_copy_string(value)?),
        dsl::DslValue::Array(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation2d-initializer.json-array-preflight")?;
            for value in values {
                target.push(generation2d_copy_json(value, depth + 1)?);
            }
            dsl::DslValue::Array(target)
        }
        dsl::DslValue::Object(values) => {
            let mut target = Vec::new();
            for (key, value) in values {
                target.push((generation2d_copy_string(key)?, generation2d_copy_json(value, depth + 1)?));
            }
            dsl::DslValue::Object(target)
        }
    })
}

fn generation2d_copy_neural_value(source: &flow::neural::Value, depth: usize) -> Result<flow::neural::Value, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.neural-depth");
    }
    Ok(match source {
        flow::neural::Value::Atom(flow::neural::Atom::Null) => flow::neural::Value::Atom(flow::neural::Atom::Null),
        flow::neural::Value::Atom(flow::neural::Atom::Boolean(value)) => flow::neural::Value::Atom(flow::neural::Atom::Boolean(*value)),
        flow::neural::Value::Atom(flow::neural::Atom::Integer(value)) => flow::neural::Value::Atom(flow::neural::Atom::Integer(*value)),
        flow::neural::Value::Atom(flow::neural::Atom::Decimal(value)) => flow::neural::Value::Atom(flow::neural::Atom::Decimal(*value)),
        flow::neural::Value::Atom(flow::neural::Atom::String(value)) => flow::neural::Value::Atom(flow::neural::Atom::String(generation2d_copy_string(value)?)),
        flow::neural::Value::Dictionary(value) => flow::neural::Value::Dictionary(generation2d_copy_dictionary(value, depth + 1)?),
    })
}

fn generation2d_copy_dictionary(source: &flow::neural::Dictionary, depth: usize) -> Result<flow::neural::Dictionary, &'static str> {
    let mut target = flow::neural::Dictionary::new();
    for key in source.keys() {
        let value = source.get(key).ok_or("generation2d-initializer.dictionary-owner")?;
        target = target.insert(generation2d_copy_string(key)?, generation2d_copy_neural_value(value, depth + 1)?);
    }
    Ok(target)
}

fn generation2d_copy_tree(source: &flow::neural::Tree, depth: usize) -> Result<flow::neural::Tree, &'static str> {
    if depth >= GENERATION2D_RETAINED_STACK_CAPACITY {
        return Err("generation2d-initializer.tree-depth");
    }
    let mut neurons = Vec::new();
    neurons.try_reserve_exact(source.neurons.len()).map_err(|_| "generation2d-initializer.neurons-preflight")?;
    for neuron in &source.neurons {
        neurons.push(flow::neural::Neuron {
            id: generation2d_copy_string(&neuron.id)?,
            kind: generation2d_copy_string(&neuron.kind)?,
            params: generation2d_copy_dictionary(&neuron.params, depth + 1)?,
            tree: match neuron.tree.as_deref() {
                Some(tree) => Some(Box::new(generation2d_copy_tree(tree, depth + 1)?)),
                None => None,
            },
        });
    }
    let mut synapses = Vec::new();
    synapses.try_reserve_exact(source.synapses.len()).map_err(|_| "generation2d-initializer.tree-synapses-preflight")?;
    for synapse in &source.synapses {
        synapses.push(flow::neural::Synapse {
            id: generation2d_copy_string(&synapse.id)?,
            from: generation2d_copy_string(&synapse.from)?,
            to: generation2d_copy_string(&synapse.to)?,
            from_port: generation2d_copy_string(&synapse.from_port)?,
            to_port: generation2d_copy_string(&synapse.to_port)?,
        });
    }
    Ok(flow::neural::Tree { neurons, synapses })
}

fn generation2d_copy_flow_ui(source: &flow::FlowGui) -> Result<flow::FlowGui, &'static str> {
    let mut nodes = flow::OrderedMap::new();
    for (id, node) in &source.nodes {
        let chrome = match &node.chrome {
            flow::NodeChrome::Plain { preview } => flow::NodeChrome::Plain { preview: *preview },
            flow::NodeChrome::Slider { label, min, max, step, value } => flow::NodeChrome::Slider { label: generation2d_copy_string(label)?, min: *min, max: *max, step: *step, value: *value },
            flow::NodeChrome::Note { text } => flow::NodeChrome::Note { text: generation2d_copy_string(text)? },
            flow::NodeChrome::Image { src } => flow::NodeChrome::Image { src: generation2d_copy_string(src)? },
            flow::NodeChrome::Variable { name, schema } => flow::NodeChrome::Variable { name: generation2d_copy_string(name)?, schema: generation2d_copy_string(schema)? },
        };
        nodes.insert(generation2d_copy_string(id)?, flow::FlowNodeGui { layout: flow::WidgetLayout { x: node.layout.x, y: node.layout.y }, chrome });
    }
    let mut previews = Vec::new();
    previews.try_reserve_exact(source.previews.len()).map_err(|_| "generation2d-initializer.previews-preflight")?;
    for preview in &source.previews {
        let source = match &preview.source {
            Some(source) => Some(flow::FlowChannelRef { neuron: generation2d_copy_string(&source.neuron)?, channel: generation2d_copy_string(&source.channel)? }),
            None => None,
        };
        let mut expanded = flow::OrderedSet::new();
        for value in &preview.expanded {
            expanded.insert(generation2d_copy_string(value)?);
        }
        previews.push(flow::FlowPreviewGui {
            id: generation2d_copy_string(&preview.id)?,
            source,
            mode: generation2d_copy_string(&preview.mode)?,
            preview: generation2d_copy_dictionary(&preview.preview, 0)?,
            expanded,
            layout: preview.layout.as_ref().map(|layout| flow::WidgetLayout { x: layout.x, y: layout.y }),
        });
    }
    Ok(flow::FlowUi { camera: flow::CameraJson { x: source.camera.x, y: source.camera.y, zoom: source.camera.zoom }, nodes, previews })
}

fn generation2d_copy_widget(source: &flow::Widget) -> Result<flow::Widget, &'static str> {
    Ok(match source {
        flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            let mut inputs = Vec::new();
            inputs.try_reserve_exact(input_ports.len()).map_err(|_| "generation2d-initializer.inputs-preflight")?;
            for value in input_ports {
                inputs.push(generation2d_copy_string(value)?);
            }
            let mut outputs = Vec::new();
            outputs.try_reserve_exact(output_ports.len()).map_err(|_| "generation2d-initializer.outputs-preflight")?;
            for value in output_ports {
                outputs.push(generation2d_copy_string(value)?);
            }
            flow::Widget::Neuron { id: generation2d_copy_string(id)?, neuron_kind: generation2d_copy_string(neuron_kind)?, params: generation2d_copy_dictionary(params, 0)?, input_ports: inputs, output_ports: outputs, preview: *preview }
        }
        flow::Widget::InputSlider { id, label, value, min, max, step } => flow::Widget::InputSlider { id: generation2d_copy_string(id)?, label: generation2d_copy_string(label)?, value: *value, min: *min, max: *max, step: *step },
        flow::Widget::InputNote { id, text } => flow::Widget::InputNote { id: generation2d_copy_string(id)?, text: generation2d_copy_string(text)? },
        flow::Widget::InputImage { id, src } => flow::Widget::InputImage { id: generation2d_copy_string(id)?, src: generation2d_copy_string(src)? },
        flow::Widget::Variable { id, name, schema } => flow::Widget::Variable { id: generation2d_copy_string(id)?, name: generation2d_copy_string(name)?, schema: generation2d_copy_string(schema)? },
        flow::Widget::OutputPreview { id, preview, expanded } => {
            let mut next_expanded = flow::OrderedSet::new();
            for value in expanded {
                next_expanded.insert(generation2d_copy_string(value)?);
            }
            flow::Widget::OutputPreview { id: generation2d_copy_string(id)?, preview: generation2d_copy_dictionary(preview, 0)?, expanded: next_expanded }
        }
        flow::Widget::OutputAction { id, action } => flow::Widget::OutputAction { id: generation2d_copy_string(id)?, action: generation2d_copy_string(action)? },
        flow::Widget::OutputExport { id, format } => flow::Widget::OutputExport { id: generation2d_copy_string(id)?, format: generation2d_copy_string(format)? },
        flow::Widget::Cluster { id, name, tree, flow } => flow::Widget::Cluster { id: generation2d_copy_string(id)?, name: generation2d_copy_string(name)?, tree: generation2d_copy_tree(tree, 0)?, flow: generation2d_copy_flow_ui(flow)? },
    })
}

fn generation2d_copy_synapse(source: &flow::SynapseSpec) -> Result<flow::SynapseSpec, &'static str> {
    Ok(flow::SynapseSpec {
        id: generation2d_copy_string(&source.id)?,
        from: generation2d_copy_string(&source.from)?,
        to: generation2d_copy_string(&source.to)?,
        from_port: generation2d_copy_string(&source.from_port)?,
        to_port: generation2d_copy_string(&source.to_port)?,
    })
}

fn generation2d_copy_generation(source: &flow::playbook::FormGeneration) -> Result<flow::playbook::FormGeneration, &'static str> {
    let mut values = serde_json::Map::new();
    for (key, value) in &source.values {
        let copied = generation2d_copy_json(&dsl::DslValue::from(value), 0)?;
        values.insert(generation2d_copy_string(key)?, serde_json::Value::from(&copied));
    }
    Ok(flow::playbook::FormGeneration { id: generation2d_copy_string(&source.id)?, name: generation2d_copy_string(&source.name)?, values })
}

struct Generation2dSnapshotCopyCursor {
    target: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    phase: u8,
    index: usize,
    handed_back: bool,
}

impl Generation2dSnapshotCopyCursor {
    fn new(source: &Generation2dSnapshot) -> Result<Self, &'static str> {
        let mut target = Generation2dSnapshot {
            fixture: flow::FlowFixture { schema: String::new(), camera: flow::CameraJson::default(), widgets: Vec::new(), synapses: Vec::new(), layout: flow::OrderedMap::new() },
            generation: flow::playbook::GenerationPlayRoot::default(),
        };
        target.fixture.widgets.try_reserve_exact(source.fixture.widgets.len()).map_err(|_| "generation2d-initializer.widgets-preflight")?;
        target.fixture.synapses.try_reserve_exact(source.fixture.synapses.len()).map_err(|_| "generation2d-initializer.synapses-preflight")?;
        target.generation.cold_builder_mut()?.generations.try_reserve_exact(source.generation.generations.len()).map_err(|_| "generation2d-initializer.generations-preflight")?;
        Ok(Self { target: std::mem::ManuallyDrop::new(Some(target)), phase: 0, index: 0, handed_back: false })
    }

    fn step(&mut self, source: &Generation2dSnapshot, digest: &mut store::ArtifactStoreInitializationDigest) -> Result<bool, &'static str> {
        let target = self.target.as_mut().ok_or("generation2d-initializer.copy-owner")?;
        match self.phase {
            0 => {
                target.fixture.schema = generation2d_copy_string(&source.fixture.schema)?;
                digest.observe(source.fixture.schema.as_bytes());
                self.phase = 1;
            }
            1 => {
                target.fixture.camera.x = source.fixture.camera.x;
                digest.observe(&source.fixture.camera.x.to_bits().to_be_bytes());
                self.phase = 2;
            }
            2 => {
                target.fixture.camera.y = source.fixture.camera.y;
                digest.observe(&source.fixture.camera.y.to_bits().to_be_bytes());
                self.phase = 3;
            }
            3 => {
                target.fixture.camera.zoom = source.fixture.camera.zoom;
                digest.observe(&source.fixture.camera.zoom.to_bits().to_be_bytes());
                self.phase = 4;
            }
            4 if self.index < source.fixture.widgets.len() => {
                target.fixture.widgets.push(generation2d_copy_widget(&source.fixture.widgets[self.index])?);
                digest.observe(crate::artifacts::generation2d::widget_id(&source.fixture.widgets[self.index]).as_bytes());
                self.index += 1;
            }
            4 => {
                self.phase = 5;
                self.index = 0;
            }
            5 if self.index < source.fixture.synapses.len() => {
                target.fixture.synapses.push(generation2d_copy_synapse(&source.fixture.synapses[self.index])?);
                digest.observe(source.fixture.synapses[self.index].id.as_bytes());
                self.index += 1;
            }
            5 => {
                self.phase = 6;
                self.index = 0;
            }
            6 if self.index < source.fixture.layout.len() => {
                let (id, layout) = source.fixture.layout.iter().nth(self.index).ok_or("generation2d-initializer.layout-owner")?;
                target.fixture.layout.insert(generation2d_copy_string(id)?, flow::WidgetLayout { x: layout.x, y: layout.y });
                digest.observe(id.as_bytes());
                self.index += 1;
            }
            6 => {
                self.phase = 7;
                self.index = 0;
            }
            7 if self.index < source.generation.generations.len() => {
                target.generation.cold_builder_mut()?.generations.push(generation2d_copy_generation(&source.generation.generations[self.index])?);
                digest.observe(source.generation.generations[self.index].id.as_bytes());
                self.index += 1;
            }
            7 => {
                target.generation.cold_builder_mut()?.selected_generation_id = match source.generation.selected_generation_id.as_deref() {
                    Some(value) => Some(generation2d_copy_string(value)?),
                    None => None,
                };
                self.phase = 8;
            }
            8 => {
                target.generation.cold_builder_mut()?.preview_text = match source.generation.preview_text.as_deref() {
                    Some(value) => Some(generation2d_copy_string(value)?),
                    None => None,
                };
                self.phase = 9;
            }
            _ => return Ok(true),
        }
        Ok(self.phase == 9)
    }

    fn take(&mut self) -> Option<Generation2dSnapshot> {
        if self.phase != 9 || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.target.take()
    }

    fn close_step(&mut self, maximum_items: usize) -> bool {
        if maximum_items == 0 {
            return false;
        }
        drop(self.target.take());
        self.handed_back = true;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.target.is_none()
    }
}

impl Drop for Generation2dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Generation2d snapshot copy cursor reached Drop before handoff or terminal-empty close");
    }
}

fn generation2d_observe_json(digest: &mut store::ArtifactStoreInitializationDigest, value: &dsl::DslValue) {
    match value {
        dsl::DslValue::Null => digest.observe(b"null"),
        dsl::DslValue::Bool(value) => digest.observe(&[b'b', u8::from(*value)]),
        dsl::DslValue::Number(_) => {
            digest.observe(b"number");
            digest.observe(dsl::json::to_json_string(value).as_bytes());
        }
        dsl::DslValue::String(value) => {
            digest.observe(b"string");
            digest.observe(value.as_bytes());
        }
        dsl::DslValue::Array(values) => {
            digest.observe(b"array");
            digest.observe(&values.len().to_be_bytes());
            for value in values {
                generation2d_observe_json(digest, value);
            }
        }
        dsl::DslValue::Object(values) => {
            digest.observe(b"object");
            digest.observe(&values.len().to_be_bytes());
            for (key, value) in values {
                digest.observe(key.as_bytes());
                generation2d_observe_json(digest, value);
            }
        }
    }
}

fn generation2d_observe_dictionary(digest: &mut store::ArtifactStoreInitializationDigest, value: &flow::neural::Dictionary) {
    digest.observe(&value.len().to_be_bytes());
    for key in value.keys() {
        digest.observe(key.as_bytes());
        match value.get(key).expect("P2 dictionary key remains owned") {
            flow::neural::Value::Atom(flow::neural::Atom::Null) => digest.observe(b"null"),
            flow::neural::Value::Atom(flow::neural::Atom::Boolean(value)) => digest.observe(&[b'b', u8::from(*value)]),
            flow::neural::Value::Atom(flow::neural::Atom::Integer(value)) => digest.observe(&value.to_be_bytes()),
            flow::neural::Value::Atom(flow::neural::Atom::Decimal(value)) => digest.observe(&value.to_bits().to_be_bytes()),
            flow::neural::Value::Atom(flow::neural::Atom::String(value)) => digest.observe(value.as_bytes()),
            flow::neural::Value::Dictionary(value) => generation2d_observe_dictionary(digest, value),
        }
    }
}

fn generation2d_observe_tree(digest: &mut store::ArtifactStoreInitializationDigest, tree: &flow::neural::Tree) {
    digest.observe(&tree.neurons.len().to_be_bytes());
    for neuron in &tree.neurons {
        digest.observe(neuron.id.as_bytes());
        digest.observe(neuron.kind.as_bytes());
        generation2d_observe_dictionary(digest, &neuron.params);
        match neuron.tree.as_deref() {
            Some(tree) => generation2d_observe_tree(digest, tree),
            None => digest.observe(b"no-tree"),
        }
    }
    digest.observe(&tree.synapses.len().to_be_bytes());
    for synapse in &tree.synapses {
        for value in [&synapse.id, &synapse.from, &synapse.to, &synapse.from_port, &synapse.to_port] {
            digest.observe(value.as_bytes());
        }
    }
}

fn generation2d_observe_widget(digest: &mut store::ArtifactStoreInitializationDigest, widget: &flow::Widget) {
    match widget {
        flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            digest.observe(b"neuron");
            digest.observe(id.as_bytes());
            digest.observe(neuron_kind.as_bytes());
            generation2d_observe_dictionary(digest, params);
            for value in input_ports.iter().chain(output_ports) {
                digest.observe(value.as_bytes());
            }
            digest.observe(&[u8::from(*preview)]);
        }
        flow::Widget::InputSlider { id, label, value, min, max, step } => {
            digest.observe(b"input-slider");
            digest.observe(id.as_bytes());
            digest.observe(label.as_bytes());
            for value in [value, min, max, step] {
                digest.observe(&value.to_bits().to_be_bytes());
            }
        }
        flow::Widget::InputNote { id, text } => {
            digest.observe(b"input-note");
            digest.observe(id.as_bytes());
            digest.observe(text.as_bytes());
        }
        flow::Widget::InputImage { id, src } => {
            digest.observe(b"input-image");
            digest.observe(id.as_bytes());
            digest.observe(src.as_bytes());
        }
        flow::Widget::Variable { id, name, schema } => {
            digest.observe(b"variable");
            digest.observe(id.as_bytes());
            digest.observe(name.as_bytes());
            digest.observe(schema.as_bytes());
        }
        flow::Widget::OutputPreview { id, preview, expanded } => {
            digest.observe(b"output-preview");
            digest.observe(id.as_bytes());
            generation2d_observe_dictionary(digest, preview);
            for value in expanded {
                digest.observe(value.as_bytes());
            }
        }
        flow::Widget::OutputAction { id, action } => {
            digest.observe(b"output-action");
            digest.observe(id.as_bytes());
            digest.observe(action.as_bytes());
        }
        flow::Widget::OutputExport { id, format } => {
            digest.observe(b"output-export");
            digest.observe(id.as_bytes());
            digest.observe(format.as_bytes());
        }
        flow::Widget::Cluster { id, name, tree, flow } => {
            digest.observe(b"cluster");
            digest.observe(id.as_bytes());
            digest.observe(name.as_bytes());
            generation2d_observe_tree(digest, tree);
            for (id, node) in &flow.nodes {
                digest.observe(id.as_bytes());
                digest.observe(&node.layout.x.to_bits().to_be_bytes());
                digest.observe(&node.layout.y.to_bits().to_be_bytes());
            }
            for preview in &flow.previews {
                digest.observe(preview.id.as_bytes());
                digest.observe(preview.mode.as_bytes());
                generation2d_observe_dictionary(digest, &preview.preview);
            }
        }
    }
}

fn generation2d_observe_generation(digest: &mut store::ArtifactStoreInitializationDigest, generation: &flow::playbook::FormGeneration) {
    digest.observe(generation.id.as_bytes());
    digest.observe(generation.name.as_bytes());
    for (key, value) in &generation.values {
        digest.observe(key.as_bytes());
        generation2d_observe_json(digest, &dsl::DslValue::from(value));
    }
}

fn generation2d_observe_mutation(digest: &mut store::ArtifactStoreInitializationDigest, mutation: &Generation2dMutation) {
    match mutation {
        Generation2dMutation::CreateWidget(value) => {
            digest.observe(b"create-widget");
            digest.observe(&value.index.to_be_bytes());
            generation2d_observe_widget(digest, &value.widget);
        }
        Generation2dMutation::ReplaceWidget(value) => {
            digest.observe(b"replace-widget");
            generation2d_observe_widget(digest, &value.widget);
        }
        Generation2dMutation::DeleteWidget(value) => {
            digest.observe(b"delete-widget");
            digest.observe(value.id.as_bytes());
        }
        Generation2dMutation::ConnectSynapse(value) => {
            digest.observe(b"connect-synapse");
            digest.observe(&value.index.to_be_bytes());
            for value in [&value.synapse.id, &value.synapse.from, &value.synapse.to, &value.synapse.from_port, &value.synapse.to_port] {
                digest.observe(value.as_bytes());
            }
        }
        Generation2dMutation::ReplaceSynapse(value) => {
            digest.observe(b"replace-synapse");
            for value in [&value.synapse.id, &value.synapse.from, &value.synapse.to, &value.synapse.from_port, &value.synapse.to_port] {
                digest.observe(value.as_bytes());
            }
        }
        Generation2dMutation::DisconnectSynapse(value) => {
            digest.observe(b"disconnect-synapse");
            digest.observe(value.id.as_bytes());
        }
        Generation2dMutation::MoveWidget(value) => {
            digest.observe(b"move-widget");
            digest.observe(value.id.as_bytes());
            digest.observe(&value.layout.x.to_bits().to_be_bytes());
            digest.observe(&value.layout.y.to_bits().to_be_bytes());
        }
        Generation2dMutation::ClearWidgetLayout(value) => {
            digest.observe(b"clear-widget-layout.2d-only");
            digest.observe(value.id.as_bytes());
        }
        Generation2dMutation::UpdateCamera(value) => {
            digest.observe(b"update-camera");
            digest.observe(&value.camera.x.to_bits().to_be_bytes());
            digest.observe(&value.camera.y.to_bits().to_be_bytes());
            digest.observe(&value.camera.zoom.to_bits().to_be_bytes());
        }
        Generation2dMutation::ChangeSchema(value) => {
            digest.observe(b"change-schema");
            digest.observe(value.schema.as_bytes());
        }
        Generation2dMutation::CreateGeneration(value) => {
            digest.observe(b"create-generation");
            generation2d_observe_generation(digest, &value.generation);
        }
        Generation2dMutation::DeleteGeneration(value) => {
            digest.observe(b"delete-generation");
            digest.observe(value.id.as_bytes());
        }
        Generation2dMutation::RenameGeneration(value) => {
            digest.observe(b"rename-generation");
            digest.observe(value.id.as_bytes());
            digest.observe(value.name.as_bytes());
        }
        Generation2dMutation::ChangeGenerationValue(value) => {
            digest.observe(b"change-generation-value");
            digest.observe(value.id.as_bytes());
            digest.observe(value.question_id.as_bytes());
            generation2d_observe_json(digest, &dsl::DslValue::from(&value.value));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation2dStoreInitializationPhase {
    ValidateEnvelope,
    ValidateEditPair { left: usize, right: usize },
    CensusHistory { edit: usize, mutation: usize },
    CopyInitial,
    BuildRuntime,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FindApplied { position: usize, scan: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    HashInverse { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize, scan: usize },
    HashRedoForward { position: usize, edit: usize, mutation: usize },
    HashRedoInverse { position: usize, edit: usize, mutation: usize },
    CommitRedo { position: usize, edit: usize },
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

struct Generation2dStoreInitializationAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    history_items: usize,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>>>,
    copy: std::mem::ManuallyDrop<Option<Generation2dSnapshotCopyCursor>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Generation2dSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    active_terminal: bool,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Generation2dSnapshot, Generation2dMutation>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,
    phase: Generation2dStoreInitializationPhase,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl Generation2dStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Self {
        let (base_revision, parent_revision) = generation2d_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));
        Self {
            operation,
            generation,
            base_revision,
            parent_revision,
            history_items: 0,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            copy: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            active_terminal: false,
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            initial_digest: std::mem::ManuallyDrop::new(Some(store::ArtifactStoreInitializationDigest::new(b"generation2d.initial"))),
            edit_digest: std::mem::ManuallyDrop::new(None),
            phase: Generation2dStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn fail(&mut self, code: &'static [u8]) {
        let mut value = Vec::new();
        if value.try_reserve_exact(code.len()).is_ok() {
            value.extend_from_slice(code);
        }
        self.fault = Some(value);
        self.phase = Generation2dStoreInitializationPhase::RetireFault;
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn pump_active(&mut self) -> Result<bool, String> {
        if self.active_terminal {
            drop(self.active.take());
            self.active_terminal = false;
            return Ok(true);
        }
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, GENERATION2D_OWNER_BYTES)? {
            store::SnapshotRetirementStep::Complete if active.terminal_is_empty() => self.active_terminal = true,
            store::SnapshotRetirementStep::Complete => return Err("generation2d-initializer.active-false-terminal".into()),
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= GENERATION2D_OWNER_BYTES => {}
            store::SnapshotRetirementStep::Pending { .. } => return Err("generation2d-initializer.active-exceeded-grant".into()),
            store::SnapshotRetirementStep::Blocked => {}
        }
        Ok(true)
    }

    fn pump_retirement(&mut self, maximum_bytes: usize) -> Result<bool, String> {
        if self.pump_active()? {
            return Ok(false);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            use semio_framework_plugin::ArtifactOwnedDisposer;
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(false);
            }
            let disposer = self.candidate_disposer.as_mut().expect("P2 candidate disposer retained");
            return match disposer.close_step(candidate, 1, maximum_bytes).map_err(|_| "generation2d-initializer.candidate-close".to_string())? {
                semio_framework_plugin::PluginCloseStep::Complete if disposer.terminal_is_empty(candidate) => {
                    drop(self.candidate_disposer.take());
                    drop(self.candidate.take());
                    Ok(false)
                }
                semio_framework_plugin::PluginCloseStep::Complete => Err("generation2d-initializer.candidate-false-terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(&Generation2dRetainedSnapshotRetirementFactory, 1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    drop(self.runtime.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("generation2d-initializer.runtime-false-terminal".into()),
                _ => Ok(false),
            };
        }
        if let Some(copy) = self.copy.as_mut() {
            if copy.close_step(1) {
                drop(self.copy.take());
            }
            return Ok(false);
        }
        if self.envelope_retirement.is_none() {
            if let Some(envelope) = self.envelope.take() {
                *self.envelope_retirement = Some(generation2d_envelope_decode_owner_bundle().retire_envelope(envelope));
                return Ok(false);
            }
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            return match retirement.close_step(1, maximum_bytes)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    drop(self.envelope_retirement.take());
                    Ok(false)
                }
                store::SnapshotRetirementStep::Complete => Err("generation2d-initializer.envelope-false-terminal".into()),
                _ => Ok(false),
            };
        }
        Ok(true)
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.copy.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.candidate_disposer.is_none()
            && self.envelope_retirement.is_none()
            && self.initial_digest.is_none()
            && self.edit_digest.is_none()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Generation2dSnapshot, Generation2dMutation> for Generation2dStoreInitializationAuthority {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"generation2d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Generation2dStoreInitializationPhase::RetireCancelled | Generation2dStoreInitializationPhase::Cancelled) {
            self.phase = Generation2dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return semio_framework_job::StepOutcome::Yield;
        }
        match self.pump_active() {
            Ok(true) => {
                cx.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_bytes());
                self.phase = Generation2dStoreInitializationPhase::RetireFault;
            }
        }
        match self.phase {
            Generation2dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == crate::artifacts::generation2d::GENERATION_2D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= GENERATION2D_OWNER_BYTES);
                if valid {
                    self.phase = Generation2dStoreInitializationPhase::ValidateEditPair { left: 0, right: 1 };
                } else {
                    self.fail(b"generation2d-store.initializer-envelope-invalid");
                }
            }
            Generation2dStoreInitializationPhase::ValidateEditPair { left, right } => {
                let envelope = self.envelope.as_ref().expect("P2 envelope retained");
                if left >= envelope.vcs.edits.len() {
                    self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit: 0, mutation: 0 };
                } else if right >= envelope.vcs.edits.len() {
                    self.phase = Generation2dStoreInitializationPhase::ValidateEditPair { left: left + 1, right: left + 2 };
                } else if envelope.vcs.edits[left].id == envelope.vcs.edits[right].id {
                    self.fail(b"generation2d-store.initializer-duplicate-edit");
                } else {
                    self.phase = Generation2dStoreInitializationPhase::ValidateEditPair { left, right: right + 1 };
                }
            }
            Generation2dStoreInitializationPhase::CensusHistory { edit, mutation } => {
                let envelope = self.envelope.as_ref().expect("P2 envelope retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    match Generation2dSnapshotCopyCursor::new(&envelope.vcs.initial_snapshot) {
                        Ok(copy) => {
                            *self.copy = Some(copy);
                            self.phase = Generation2dStoreInitializationPhase::CopyInitial;
                        }
                        Err(code) => self.fail(code.as_bytes()),
                    }
                    return semio_framework_job::StepOutcome::Yield;
                };
                if entry.forwards.get(mutation).is_some() {
                    self.history_items = match self.history_items.checked_add(1) {
                        Some(value) if value <= GENERATION2D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"generation2d-store.initializer-history-capacity");
                            return semio_framework_job::StepOutcome::Yield;
                        }
                    };
                    self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation2dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
                }
            }
            Generation2dStoreInitializationPhase::CopyInitial => {
                let source = &self.envelope.as_ref().expect("P2 initializer envelope").vcs.initial_snapshot;
                match self.copy.as_mut().expect("P2 copy retained").step(source, self.initial_digest.as_mut().expect("P2 initial digest retained")) {
                    Ok(true) => self.phase = Generation2dStoreInitializationPhase::BuildRuntime,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Generation2dStoreInitializationPhase::BuildRuntime => {
                let initial = self.copy.as_mut().expect("P2 copy retained").take().expect("P2 copy handoff");
                drop(self.copy.take());
                let digest = self.initial_digest.take().expect("P2 initial digest retained").finish();
                let envelope = self.envelope.as_ref().expect("P2 initializer envelope");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, initial, digest));
                self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
            }
            Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("P2 history retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation2dStoreInitializationPhase::FindApplied { position: 0, scan: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                let runtime = self.runtime.as_mut().expect("P2 runtime retained");
                match lane {
                    0 => match runtime.seed_mutation(protocol::MutationId(generation2d_copy_string(&entry.id).unwrap_or_default())) {
                        Ok(()) => {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                        Err(error) => {
                            self.fault = Some(error.into_bytes());
                            self.phase = Generation2dStoreInitializationPhase::RetireFault;
                        }
                    },
                    1 if index < entry.forwards.len() => {
                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref())
                            .map(|id| protocol::MutationId(generation2d_copy_string(&id.0).unwrap_or_default()))
                            .unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        match runtime.seed_mutation(id) {
                            Ok(()) => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 },
                            Err(error) => {
                                self.fault = Some(error.into_bytes());
                                self.phase = Generation2dStoreInitializationPhase::RetireFault;
                            }
                        }
                    }
                    1 => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = Generation2dStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
            }
            Generation2dStoreInitializationPhase::FindApplied { position, scan } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self
                        .envelope
                        .as_ref()
                        .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                        .and_then(|id| generation2d_copy_string(id).ok());
                    self.runtime.as_mut().expect("P2 runtime retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Generation2dStoreInitializationPhase::FindRedo { position: 0, scan: 0 };
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                };
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation2d.edit");
                        digest.observe(edit.id.as_bytes());
                        digest.observe(&edit.sequence_number.to_be_bytes());
                        digest.observe(edit.started_at.as_bytes());
                        *self.edit_digest = Some(digest);
                        self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                    }
                    Some(_) => self.phase = Generation2dStoreInitializationPhase::FindApplied { position, scan: scan + 1 },
                    None => self.fail(b"generation2d-store.initializer-applied-missing"),
                }
            }
            Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                let Some(operation) = operation else {
                    self.phase = Generation2dStoreInitializationPhase::HashInverse { position, edit, mutation: 0 };
                    return semio_framework_job::StepOutcome::Yield;
                };
                generation2d_observe_mutation(self.edit_digest.as_mut().expect("P2 edit digest retained"), operation);
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("P2 runtime current retained");
                match generation2d_apply_initialization_mutation(current, operation) {
                    Ok(retired) => {
                        *self.active = retired;
                        self.phase = Generation2dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    }
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Generation2dStoreInitializationPhase::HashInverse { position, edit, mutation } => {
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.inverse.get(mutation));
                if let Some(operation) = operation {
                    generation2d_observe_mutation(self.edit_digest.as_mut().expect("P2 edit digest retained"), operation);
                    self.phase = Generation2dStoreInitializationPhase::HashInverse { position, edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation2dStoreInitializationPhase::CommitApplied { position, edit };
                }
            }
            Generation2dStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P2 applied edit retained");
                let id = generation2d_copy_string(&entry.id).unwrap_or_default();
                let actor = entry.actor.as_deref().and_then(|value| generation2d_copy_string(value).ok());
                let digest = self.edit_digest.take().expect("P2 edit digest retained").finish();
                let runtime = self.runtime.as_mut().expect("P2 runtime retained");
                match runtime.push_applied(id, digest) {
                    Ok(()) => {
                        runtime.observe_sequence(entry.sequence_number);
                        runtime.set_local_actor_id(actor);
                        self.phase = Generation2dStoreInitializationPhase::FindApplied { position: position + 1, scan: 0 };
                    }
                    Err(_) => self.fail(b"generation2d-store.initializer-applied-capacity"),
                }
            }
            Generation2dStoreInitializationPhase::FindRedo { position, scan } => {
                let Some(id) = self.redo_id(position) else {
                    self.phase = Generation2dStoreInitializationPhase::BuildCandidate;
                    cx.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                };
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation2d.redo");
                        digest.observe(edit.id.as_bytes());
                        digest.observe(&edit.sequence_number.to_be_bytes());
                        digest.observe(edit.started_at.as_bytes());
                        *self.edit_digest = Some(digest);
                        self.phase = Generation2dStoreInitializationPhase::HashRedoForward { position, edit: scan, mutation: 0 };
                    }
                    Some(_) => self.phase = Generation2dStoreInitializationPhase::FindRedo { position, scan: scan + 1 },
                    None => self.fail(b"generation2d-store.initializer-redo-missing"),
                }
            }
            Generation2dStoreInitializationPhase::HashRedoForward { position, edit, mutation } => {
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                if let Some(operation) = operation {
                    generation2d_observe_mutation(self.edit_digest.as_mut().expect("P2 redo digest retained"), operation);
                    self.phase = Generation2dStoreInitializationPhase::HashRedoForward { position, edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation2dStoreInitializationPhase::HashRedoInverse { position, edit, mutation: 0 };
                }
            }
            Generation2dStoreInitializationPhase::HashRedoInverse { position, edit, mutation } => {
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.inverse.get(mutation));
                if let Some(operation) = operation {
                    generation2d_observe_mutation(self.edit_digest.as_mut().expect("P2 redo digest retained"), operation);
                    self.phase = Generation2dStoreInitializationPhase::HashRedoInverse { position, edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation2dStoreInitializationPhase::CommitRedo { position, edit };
                }
            }
            Generation2dStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P2 redo edit retained");
                let id = generation2d_copy_string(&entry.id).unwrap_or_default();
                let digest = self.edit_digest.take().expect("P2 redo digest retained").finish();
                match self.runtime.as_mut().expect("P2 runtime retained").push_redo(id, digest) {
                    Ok(()) => self.phase = Generation2dStoreInitializationPhase::FindRedo { position: position + 1, scan: 0 },
                    Err(_) => self.fail(b"generation2d-store.initializer-redo-capacity"),
                }
            }
            Generation2dStoreInitializationPhase::BuildCandidate => {
                let authority = generation2d_validate_publication_authority(self.operation, self.generation);
                let fresh =
                    cx.operation() == self.operation && cx.generation() == self.generation && authority == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
                let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
                    self.fail(b"generation2d-store.initializer-generation-exhausted");
                    return semio_framework_job::StepOutcome::Yield;
                };
                if !fresh {
                    self.fail(b"generation2d-store.initializer-parent-stale-aba");
                    return semio_framework_job::StepOutcome::Yield;
                }
                let envelope = self.envelope.take().expect("P2 envelope retained until atomic publication");
                let runtime = self.runtime.take().expect("P2 runtime retained until atomic publication");
                *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, generation2d_document_store_owners()));
                self.phase = Generation2dStoreInitializationPhase::Complete;
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            Generation2dStoreInitializationPhase::RetireCancelled | Generation2dStoreInitializationPhase::RetireFault => match self.pump_retirement(GENERATION2D_OWNER_BYTES) {
                Ok(false) => return semio_framework_job::StepOutcome::Yield,
                Ok(true) => {
                    drop(self.initial_digest.take());
                    drop(self.edit_digest.take());
                    self.terminal_handoff = true;
                    if self.phase == Generation2dStoreInitializationPhase::RetireCancelled {
                        self.phase = Generation2dStoreInitializationPhase::Cancelled;
                        return semio_framework_job::StepOutcome::Cancelled;
                    }
                    self.phase = Generation2dStoreInitializationPhase::Fault;
                    let detail = cx
                        .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, self.fault.as_deref().unwrap_or(b"generation2d-store.initializer-fault"))
                        .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail });
                }
                Err(_) => self.fail(b"generation2d-store.initializer-close"),
            },
            Generation2dStoreInitializationPhase::Complete => {
                return semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            Generation2dStoreInitializationPhase::Cancelled => return semio_framework_job::StepOutcome::Cancelled,
            Generation2dStoreInitializationPhase::Fault => {
                let detail = cx
                    .payload_from_bytes(semio_framework_job::JobPayloadStream::Fault, self.fault.as_deref().unwrap_or(b"generation2d-store.initializer-fault"))
                    .unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault));
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail });
            }
        }
        cx.consume_fuel(1);
        semio_framework_job::StepOutcome::Yield
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<Generation2dSnapshot, Generation2dMutation>> {
        if self.phase != Generation2dStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        drop(self.initial_digest.take());
        drop(self.edit_digest.take());
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Generation2dStoreInitializationPhase::Cancelled | Generation2dStoreInitializationPhase::Fault) {
            self.phase = Generation2dStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, semio_framework::Fault> {
        self.begin_close();
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        match self.pump_retirement(maximum_bytes.min(GENERATION2D_OWNER_BYTES)) {
            Ok(false) => Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }),
            Ok(true) => {
                drop(self.initial_digest.take());
                drop(self.edit_digest.take());
                self.terminal_handoff = true;
                Ok(semio_framework_plugin::PluginCloseStep::Complete)
            }
            Err(error) => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("artifact-store.initializer-close"), error)),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for Generation2dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty_inner(), "Generation2d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn generation2d_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<Generation2dSnapshot, Generation2dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Generation2dSnapshot, Generation2dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Generation2dStoreInitializationAuthority::new(envelope, operation, generation)))
}
//#endregion 🔖️RetainedStoreInitialization

#[cfg(test)]
pub fn generation2d_all_retained_mutation_fixtures_for_test() -> Vec<Generation2dMutation> {
    use crate::artifacts::generation2d::mutations::*;
    let synapse = flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-a".into(), to: "retained-b".into(), from_port: "out".into(), to_port: "in".into() };
    let mut values = serde_json::Map::new();
    values.insert("nested".into(), serde_json::json!({"array": [true, null, 3.5], "text": "retained"}));
    let params = flow::neural::Dictionary::new()
        .insert("integer", flow::neural::Value::Atom(flow::neural::Atom::Integer(7)))
        .insert("nested", flow::neural::Value::Dictionary(flow::neural::Dictionary::new().insert("text", flow::neural::Value::Atom(flow::neural::Atom::String("retained".into())))));
    vec![
        create_widget(0, flow::Widget::Neuron { id: "retained-a".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
        replace_widget(flow::Widget::Cluster { id: "retained-a".into(), name: "Replaced".into(), tree: Default::default(), flow: Default::default() }),
        delete_widget("retained-a".into()),
        connect_synapse(0, generation2d_copy_synapse(&synapse).expect("P2 synapse fixture copy")),
        replace_synapse(flow::SynapseSpec { to_port: "alternate".into(), ..synapse }),
        disconnect_synapse("retained-synapse".into()),
        move_widget("retained-a".into(), flow::WidgetLayout { x: 11.0, y: -7.0 }),
        clear_widget_layout("retained-a".into()),
        update_camera(flow::CameraJson { x: 3.0, y: 4.0, zoom: 1.5 }),
        change_schema("flow.fixture.retained".into()),
        create_generation(flow::playbook::FormGeneration { id: "retained-generation".into(), name: "Retained Generation".into(), values }),
        delete_generation("retained-generation".into()),
        rename_generation("retained-generation".into(), "Renamed Generation".into()),
        change_generation_value("retained-generation".into(), "deep-answer".into(), serde_json::json!({"object": {"array": [1.0, false, "value"]}})),
    ]
}

#[cfg(test)]
pub fn generation2d_apply_retained_mutations_for_test(snapshot: &mut Generation2dSnapshot, mutations: &[Generation2dMutation]) {
    for mutation in mutations {
        if let Some(mut retirement) = generation2d_apply_initialization_mutation(snapshot, mutation).expect("P2 production fixture retained replay") {
            for _ in 0..GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
                match retirement.close_step(1, GENERATION2D_OWNER_BYTES).expect("P2 production fixture displacement close") {
                    store::SnapshotRetirementStep::Complete => {
                        assert!(retirement.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= GENERATION2D_OWNER_BYTES);
                    }
                    store::SnapshotRetirementStep::Blocked => {}
                }
            }
            assert!(retirement.terminal_is_empty());
        }
    }
}

#[cfg(test)]
mod retained_authority_laws {
    use super::*;

    fn close_session(session: &mut Generation2dMutationSession) {
        for _ in 0..GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
            if session.close_step(1) {
                assert!(session.terminal_is_empty());
                return;
            }
        }
        panic!("P2 retained mutation session did not close");
    }

    #[test]
    fn every_fourteen_variant_decodes_through_retained_structural_grants() {
        let mutations = generation2d_all_retained_mutation_fixtures_for_test();
        assert_eq!(mutations.len(), GENERATION2D_MUTATION_VARIANT_COUNT);
        for mutation in mutations {
            let bytes = encode_op(&mutation).expect("P2 retained mutation fixture encode");
            let mut session = Generation2dMutationSession::new(bytes.len(), GENERATION2D_MAXIMUM_DOMAIN_ITEMS).expect("P2 retained mutation preflight");
            for byte in bytes {
                assert!(session.ingress_ready());
                session.admit_byte(byte).expect("one retained mutation byte");
                for _ in 0..GENERATION2D_OWNER_BYTES {
                    session.grant().expect("one retained mutation ingress grant");
                    if session.ingress_ready() {
                        break;
                    }
                }
                assert!(session.ingress_ready(), "symbol expansion must hand input ownership back before the next byte");
            }
            session.seal().expect("exact retained mutation seal");
            let mut ready = false;
            for _ in 0..100_000 {
                if session.grant().expect("one retained semantic grant") {
                    ready = true;
                    break;
                }
            }
            assert!(ready, "retained P2 mutation owner must converge");
            assert_eq!(session.take().expect("typed P2 mutation handoff"), mutation);
            close_session(&mut session);
        }
    }

    #[test]
    fn deterministic_all_field_ledger_includes_the_2d_only_variant() {
        let mutations = generation2d_all_retained_mutation_fixtures_for_test();
        let mut left = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
        let mut right = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
        for mutation in &mutations {
            generation2d_observe_mutation(&mut left, mutation);
            generation2d_observe_mutation(&mut right, mutation);
        }
        assert_eq!(left.finish(), right.finish());
        assert!(mutations.iter().any(|mutation| matches!(mutation, Generation2dMutation::ClearWidgetLayout(_))));
    }
}
