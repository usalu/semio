//! 🏠️ Generation3d publication, retained replay and store ownership.
#![allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::change_generation_value::ChangeGenerationValue;
use crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema;
use crate::standards::v1::subsets::any::schema::mutations::connect_synapse::ConnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::create_generation::CreateGeneration;
use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
use crate::standards::v1::subsets::any::schema::mutations::delete_generation::DeleteGeneration;
use crate::standards::v1::subsets::any::schema::mutations::delete_widget::DeleteWidget;
use crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_synapse::DisconnectSynapse;
use crate::standards::v1::subsets::any::schema::mutations::move_widget::MoveWidget;
use crate::standards::v1::subsets::any::schema::mutations::rename_generation::RenameGeneration;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::update_camera::UpdateCamera;
use crate::standards::v1::subsets::any::schema::mutations::update_synapse::UpdateSynapse;
use crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget;
use crate::standards::v1::subsets::any::io::text::snapshot::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl,
    WidgetLayoutDsl,
};
use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::ChangeSliderValue;
use crate::standards::v1::subsets::any::schema::mutations::drag_transforms::DragTransforms;
use crate::standards::v1::subsets::any::schema::mutations::rotate_transforms::RotateTransforms;
use crate::standards::v1::subsets::any::schema::mutations::scale_transforms::ScaleTransforms;
use crate::standards::v1::subsets::any::schema::mutations::move_nodes::MoveNodes;
use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::{ChangeWidgetInput, WidgetInputValue};
use crate::standards::v1::subsets::any::schema::mutations::change_generation_preview::ChangeGenerationPreview;
use crate::standards::v1::subsets::any::schema::mutations::select_generation::SelectGeneration;
use protocol::OpBinary;
use store::ErasedSnapshotRetirement;
//#region 🔖️RetainedMountedIngress
const GENERATION3D_OWNER_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
const GENERATION3D_RETAINED_STACK_CAPACITY: usize = 64;
const GENERATION3D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;
const GENERATION3D_MAXIMUM_DOMAIN_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const GENERATION3D_MUTATION_VARIANT_COUNT: usize = 22;
pub const GENERATION3D_MOUNTED_OUTPUT_CHANNELS: usize = 4;
pub const GENERATION3D_MOUNTED_CONTROL_CREDITS: usize = 1;
const GENERATION3D_PUBLICATION_SLOTS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Generation3dPublicationLease {
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

impl semio_framework_job::FixedOperationOwner for Generation3dPublicationLease {
    fn retained_bytes(&self) -> usize {
        size_of::<Self>()
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.terminal {
            return InteractiveJobCloseStep::Complete { progress: Default::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_release_bytes < size_of::<Self>() {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        self.terminal = true;
        InteractiveJobCloseStep::Complete { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: size_of::<Self>(), ..Default::default() } }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal
    }
}

type Generation3dPublicationRegistry = semio_framework_job::FixedOperationRegistry<Generation3dPublicationLease, GENERATION3D_PUBLICATION_SLOTS>;

fn generation3d_publication_leases() -> &'static std::sync::Mutex<Generation3dPublicationRegistry> {
    static LEASES: std::sync::OnceLock<std::sync::Mutex<semio_framework_job::FixedOperationRegistry<Generation3dPublicationLease, GENERATION3D_PUBLICATION_SLOTS>>> = std::sync::OnceLock::new();
    LEASES.get_or_init(|| std::sync::Mutex::new(Generation3dPublicationRegistry::new(GENERATION3D_PUBLICATION_SLOTS * size_of::<Generation3dPublicationLease>())))
}

fn generation3d_publication_key(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::FixedOperationKey {
    semio_framework_job::FixedOperationKey::new(operation, generation)
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation3dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Generation3dPublicationHostileLease {
    operation: u64,
    hostile: Generation3dPublicationHostile,
    observed: Option<&'static str>,
}

#[cfg(test)]
fn generation3d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Generation3dPublicationHostileLease>; GENERATION3D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Generation3dPublicationHostileLease>; GENERATION3D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; GENERATION3D_PUBLICATION_SLOTS]))
}

#[cfg(test)]
pub fn generation3d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Generation3dPublicationHostile) {
    let mut hostiles = generation3d_publication_hostiles().try_lock().expect("Generation3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Generation3d hostile publication authority has a fixed slot");
    *slot = Some(Generation3dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(test)]
pub fn generation3d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = generation3d_publication_hostiles().try_lock().expect("Generation3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}

/// 🧮️ Domain credits admitted together for one publication authority.
#[derive(Clone, Copy, Debug)]
pub struct Generation3dPublicationCredits {
    pub maximum_items: usize,
    pub maximum_output_pages: usize,
    pub maximum_controls: usize,
}

pub fn generation3d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    credits: Generation3dPublicationCredits,
) -> Result<(), &'static str> {
    let Generation3dPublicationCredits { maximum_items, maximum_output_pages, maximum_controls } = credits;
    if generation.0 != live_revision || base_revision != live_revision || parent_revision != base_revision {
        return Err("generation3d-publication.initial-freshness");
    }
    let mut leases = generation3d_publication_leases().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("generation3d-publication.operation-duplicate");
    }
    if maximum_items == 0 || maximum_items > GENERATION3D_MAXIMUM_DOMAIN_ITEMS || maximum_output_pages != GENERATION3D_MOUNTED_OUTPUT_CHANNELS || maximum_controls != GENERATION3D_MOUNTED_CONTROL_CREDITS {
        return Err("generation3d-publication.domain-credits");
    }
    leases
        .admit(
            generation3d_publication_key(operation, generation),
            Generation3dPublicationLease { operation: operation.0, generation: generation.0, base_revision, parent_revision, live_revision, maximum_items, maximum_output_pages, maximum_controls, closing: false, terminal: false },
        )
        .map_err(|_| "generation3d-publication.saturated")
}

/// 🔐️ The ONE lease the app grants ITSELF for a replacement the host began (`loadDocumentArchive`, a whole-document load
/// → `build_document_store_initialization_job`): base, parent and live revision are all the generation the host started
/// the replacement on — the only publication that commit can accept — with the domain's own credits. A holder that
/// admitted a HOST lease for the same operation first keeps it (`Err`). The twin of `process3d_app_publication_lease`.
///
/// A self-grant is NOT a host publication and does not live in the host's fixed direct-mapped table, where it would refuse
/// an unrelated host publication mapping to the same slot. The host drives one replacement per instance at a time, so this
/// authority holds exactly one lease and a new self-grant supersedes the load the host already abandoned; it is released
/// through `generation3d_release_app_publication_authority` at validation, cancellation, fault and initializer close.
fn generation3d_app_publication_lease() -> &'static std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, Generation3dPublicationLease)>> {
    static LEASE: std::sync::OnceLock<std::sync::Mutex<Option<(semio_framework_job::FixedOperationKey, Generation3dPublicationLease)>>> = std::sync::OnceLock::new();
    LEASE.get_or_init(|| std::sync::Mutex::new(None))
}

/// 🔎️ One lease by its exact key, from the host table first and the app self-grant second.
fn generation3d_publication_lease_by_key(key: semio_framework_job::FixedOperationKey) -> Result<Option<Generation3dPublicationLease>, &'static str> {
    let leases = generation3d_publication_leases().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if let Some(lease) = leases.get(key) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = generation3d_app_publication_lease().try_lock().map_err(|_| "generation3d-publication.contended")?;
    Ok(app.as_ref().filter(|(held, _)| *held == key).map(|(_, lease)| *lease))
}

/// 🔎️ One lease by operation, from the host table first and the app self-grant second.
fn generation3d_publication_lease_by_operation(operation: semio_framework_job::OperationId) -> Result<Option<Generation3dPublicationLease>, &'static str> {
    let leases = generation3d_publication_leases().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if let Some((_, lease)) = leases.get_operation(operation) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = generation3d_app_publication_lease().try_lock().map_err(|_| "generation3d-publication.contended")?;
    Ok(app.as_ref().filter(|(_, lease)| lease.operation == operation.0).map(|(_, lease)| *lease))
}

/// 🔐️ Grants the app's own lease for `operation` — see [`generation3d_app_publication_lease`].
pub fn generation3d_admit_app_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let leases = generation3d_publication_leases().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("generation3d-publication.operation-duplicate");
    }
    drop(leases);
    let mut app = generation3d_app_publication_lease().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if app.as_ref().is_some_and(|(_, lease)| lease.operation == operation.0) {
        return Err("generation3d-publication.operation-duplicate");
    }
    *app = Some((
        generation3d_publication_key(operation, generation),
        Generation3dPublicationLease {
            operation: operation.0,
            generation: generation.0,
            base_revision: generation.0,
            parent_revision: generation.0,
            live_revision: generation.0,
            maximum_items: GENERATION3D_MAXIMUM_DOMAIN_ITEMS,
            maximum_output_pages: GENERATION3D_MOUNTED_OUTPUT_CHANNELS,
            maximum_controls: GENERATION3D_MOUNTED_CONTROL_CREDITS,
            closing: false,
            terminal: false,
        },
    ));
    Ok(())
}

/// 🔐️ Releases the app-admitted lease of `operation` (a host-admitted one is the host's to release).
pub fn generation3d_release_app_publication_authority(operation: semio_framework_job::OperationId) -> bool {
    let Ok(mut app) = generation3d_app_publication_lease().try_lock() else { return false };
    if app.as_ref().is_none_or(|(_, lease)| lease.operation != operation.0) {
        return false;
    }
    app.take().is_some()
}

pub fn generation3d_refresh_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_revision: u64) -> Result<(), &'static str> {
    let key = generation3d_publication_key(operation, generation);
    let mut leases = generation3d_publication_leases().try_lock().map_err(|_| "generation3d-publication.contended")?;
    if let Some(lease) = leases.get_mut(key) {
        lease.live_revision = live_revision;
        return Ok(());
    }
    drop(leases);
    let mut app = generation3d_app_publication_lease().try_lock().map_err(|_| "generation3d-publication.contended")?;
    let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key) else { return Err("generation3d-publication.stale-authority") };
    lease.live_revision = live_revision;
    Ok(())
}

pub fn generation3d_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {
    let lease = generation3d_publication_lease_by_key(generation3d_publication_key(operation, generation))?.ok_or("generation3d-publication.stale-authority")?;
    if lease.generation != generation.0 || lease.live_revision != generation.0 || lease.base_revision != lease.live_revision || lease.parent_revision != lease.base_revision {
        return Err("generation3d-publication.stale-aba-parent");
    }
    Ok((lease.base_revision, lease.parent_revision))
}

fn generation3d_validate_atomic_lease(lease: Generation3dPublicationLease, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    if lease.operation != operation.0 {
        return Err("generation3d-publication.wrong-operation");
    }
    if lease.generation != generation.0 {
        return Err("generation3d-publication.wrong-generation");
    }
    if lease.live_revision != live_generation.0 || lease.base_revision != lease.live_revision {
        return Err("generation3d-publication.wrong-base");
    }
    if lease.parent_revision != lease.base_revision {
        return Err("generation3d-publication.wrong-parent");
    }
    if lease.maximum_items == 0 || lease.maximum_output_pages != GENERATION3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION3D_MOUNTED_CONTROL_CREDITS {
        return Err("generation3d-publication.authority-credits");
    }
    Ok(())
}

/// 🔐️ Fail-closed Generation3d authority used by the shared atomic replacement branch.
pub fn generation3d_validate_atomic_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let lease = generation3d_publication_lease_by_operation(operation)?.ok_or("generation3d-publication.authority-missing")?;
    #[cfg(test)]
    let lease = {
        let mut lease = lease;
        let mut hostiles = generation3d_publication_hostiles().try_lock().map_err(|_| "generation3d-publication.hostile-contended")?;
        if let Some(hostile) = hostiles.iter_mut().flatten().find(|value| value.operation == operation.0) {
            hostile.observed = Some(match hostile.hostile {
                Generation3dPublicationHostile::Missing => "generation3d-publication.authority-missing",
                Generation3dPublicationHostile::WrongOperation => "generation3d-publication.wrong-operation",
                Generation3dPublicationHostile::WrongGeneration => "generation3d-publication.wrong-generation",
                Generation3dPublicationHostile::WrongBase => "generation3d-publication.wrong-base",
                Generation3dPublicationHostile::WrongParent => "generation3d-publication.wrong-parent",
            });
            match hostile.hostile {
                Generation3dPublicationHostile::Missing => return Err("generation3d-publication.authority-missing"),
                Generation3dPublicationHostile::WrongOperation => lease.operation = lease.operation.wrapping_add(1),
                Generation3dPublicationHostile::WrongGeneration => lease.generation = lease.generation.wrapping_add(1),
                Generation3dPublicationHostile::WrongBase => lease.base_revision = lease.base_revision.wrapping_add(1),
                Generation3dPublicationHostile::WrongParent => lease.parent_revision = lease.parent_revision.wrapping_add(1),
            }
        }
        lease
    };
    generation3d_validate_atomic_lease(lease, operation, generation, live_generation)
}

pub fn generation3d_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {
    let lease = generation3d_publication_lease_by_key(generation3d_publication_key(operation, generation))?.ok_or("generation3d-publication.stale-authority")?;
    if lease.maximum_output_pages != GENERATION3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION3D_MOUNTED_CONTROL_CREDITS {
        return Err("generation3d-publication.domain-credits-lost");
    }
    Ok(lease.maximum_items)
}

pub fn generation3d_release_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> bool {
    let Ok(mut leases) = generation3d_publication_leases().try_lock() else { return false };
    leases.take(generation3d_publication_key(operation, generation)).is_some()
}

/// 🧭️ Fixed ownership grammar for every Generation3d retained domain and lifecycle owner.
/// The 3d-only DeleteWidgetPosition entry is deliberately explicit: a 2d mutation catalog cannot satisfy this table.
pub const GENERATION3D_RETAINED_OWNER_CATALOG: &[&str] = &[
    "snapshot.host_snapshot.schema",
    "snapshot.host_snapshot.camera.x",
    "snapshot.host_snapshot.camera.y",
    "snapshot.host_snapshot.camera.zoom",
    "snapshot.host_snapshot.widgets.length",
    "snapshot.host_snapshot.widgets.item.neuron",
    "snapshot.host_snapshot.widgets.item.input-slider",
    "snapshot.host_snapshot.widgets.item.input-note",
    "snapshot.host_snapshot.widgets.item.input-image",
    "snapshot.host_snapshot.widgets.item.variable",
    "snapshot.host_snapshot.widgets.item.output-preview",
    "snapshot.host_snapshot.widgets.item.output-action",
    "snapshot.host_snapshot.widgets.item.output-export",
    "snapshot.host_snapshot.widgets.item.cluster",
    "snapshot.host_snapshot.widgets.item.strings",
    "snapshot.host_snapshot.widgets.item.dictionary.entries",
    "snapshot.host_snapshot.widgets.item.tree",
    "snapshot.host_snapshot.widgets.item.flow",
    "snapshot.host_snapshot.synapses.length",
    "snapshot.host_snapshot.synapses.item.id",
    "snapshot.host_snapshot.synapses.item.from",
    "snapshot.host_snapshot.synapses.item.to",
    "snapshot.host_snapshot.synapses.item.from-port",
    "snapshot.host_snapshot.synapses.item.to-port",
    "snapshot.host_snapshot.layout.length",
    "snapshot.host_snapshot.layout.item.id",
    "snapshot.host_snapshot.layout.item.x",
    "snapshot.host_snapshot.layout.item.y",
    "snapshot.generation.generations.length",
    "snapshot.generation.generations.item.id",
    "snapshot.generation.generations.item.name",
    "snapshot.generation.generations.item.values.length",
    "snapshot.generation.generations.item.values.key",
    "snapshot.generation.generations.item.values.scalar",
    "snapshot.generation.selected-generation-id",
    "snapshot.generation.preview-text",
    "mutations.create-widget",
    "mutations.update-widget",
    "mutations.delete-widget",
    "mutations.connect-synapse",
    "mutations.update-synapse",
    "mutations.disconnect-synapse",
    "mutations.move-widget",
    "mutations.delete-widget-position.3d-only",
    "mutations.update-camera",
    "mutations.change-schema",
    "mutations.create-generation",
    "mutations.delete-generation",
    "mutations.rename-generation",
    "mutations.change-generation-value",
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

pub const GENERATION3D_RETAINED_MUTATION_OWNERS: [&str; GENERATION3D_MUTATION_VARIANT_COUNT] = [
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "drag-transforms",
    "rotate-transforms",
    "scale-transforms",
    "move-nodes",
    "change-widget-input",
    "select-generation",
    "change-generation-preview",
];

pub const GENERATION3D_RETAINED_SCHEMA_DISCRIMINATOR: [u8; 4] = *b"P3D3";
pub const GENERATION3D_FORBIDDEN_2D_DISCRIMINATOR: [u8; 4] = *b"P2D2";

pub fn generation3d_retained_catalog_is_complete() -> bool {
    GENERATION3D_RETAINED_MUTATION_OWNERS == crate::standards::v1::subsets::any::schema::mutations::KINDS
        && GENERATION3D_RETAINED_OWNER_CATALOG.contains(&"mutations.delete-widget-position.3d-only")
        && !GENERATION3D_RETAINED_OWNER_CATALOG.iter().any(|owner| owner.contains("process2d"))
}

#[derive(semio_framework_value::RetireOwned)]
enum Generation3dReplayDisplaced {
    Widget(semio_framework_artifact_flow_flow::Widget),
    Widgets(Vec<semio_framework_artifact_flow_flow::Widget>),
    Layouts(semio_framework_artifact_flow_flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>),
    Synapse(semio_framework_artifact_flow_flow::SynapseSpec),
    Layout(semio_framework_artifact_flow_flow::WidgetLayout),
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Text(String),
    Generation(semio_framework_artifact_playbook_playbook::FormGeneration),
    Json(semio_framework_value::DslValue),
}

/// 🧊️ Retires one displaced replay owner at a cold boundary by paying every currency its own close quote names.
#[cfg(test)]
fn generation3d_retire_displaced_cold(value: Generation3dReplayDisplaced) {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Generation3dReplayDisplaced>(), maximum_release_bytes: 0, maximum_depth: 2 };
    let Ok((mut owner, _)) = semio_framework_value::retirement::admit_owned_retirement(value, birth) else { panic!("cold displaced replay owner refused its own birth") };
    while !owner.terminal_is_empty() {
        let copy = owner.next_copy_byte_demand().expect("cold displaced copy demand").max(GENERATION3D_OWNER_BYTES);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy).expect("cold displaced capacity demand"), maximum_release_bytes: owner.next_release_byte_demand().expect("cold displaced release demand"), maximum_depth: owner.next_depth_demand().expect("cold displaced depth demand").max(1) };
        owner.close_step(grant).expect("cold displaced replay close");
    }
}

/// 🪜️ Pays one Flow frontier turn out of the frontier's own quoted currencies — the mounted-pack session ladder this serves still counts items and bytes, so it cannot forward a caller grant.
fn generation3d_close_flow_frontier(flow: &mut semio_framework_artifact_flow_flow::retained::FlowRetirement) -> Result<(), semio_framework_value::ValueError> {
    let copy = flow.next_copy_byte_demand()?;
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: flow.next_capacity_byte_demand(copy)?, maximum_release_bytes: flow.next_release_byte_demand()?, maximum_depth: flow.next_depth_demand()?.max(1) };
    flow.step(grant).map(|_| ())
}

/// 🅿️ Parks one string list on an EMPTY Flow frontier; the callers only park once the previous owner is fully retired.
fn generation3d_park_flow_strings(flow: &mut semio_framework_artifact_flow_flow::retained::FlowRetirement, values: Vec<String>) {
    if flow.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Strings(values)).is_err() {
        unreachable!("an empty Flow frontier admits one owner");
    }
}

/// 🧮️ Replays a relative gesture into addressed operators and retains only displaced widget owners.
fn generation3d_replay_transforms(snapshot: &mut Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_value::DslValue) -> Option<Vec<(&'static str, semio_framework_value::DslValue)>>) -> Result<Option<Generation3dReplayDisplaced>, &'static str> {
    use crate::standards::v1::subsets::any::schema::mutations::{generation3d_targets_invariant,generation3d_with_params};

    generation3d_targets_invariant(targets)?;
    if targets.len() > GENERATION3D_MAXIMUM_DOMAIN_ITEMS { return Err("generation3d-replay.target-limit"); }
    let mut updates = Vec::new();
    updates.try_reserve_exact(targets.len()).map_err(|_| "generation3d-replay.transform-preflight")?;
    for id in targets {
        let Some(index) = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == id) else { continue };
        let widget = &snapshot.host_snapshot.widgets[index];
        let semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } = widget else { continue };
        if !kinds.contains(&neuron_kind.as_str()) { continue; }
        let params = semio_framework_value::ToValue::to_value(widget).get("params").cloned().unwrap_or(semio_framework_value::DslValue::Null);
        if let Some(next) = compose(&params).and_then(|entries| generation3d_with_params(widget, entries)) { updates.push((index, next)); }
    }
    if updates.is_empty() { return Err("generation3d-replay.transform-target"); }
    if identity {
        for (_, widget) in updates { widget.retire_cold(); }
        return Ok(None);
    }
    let mut displaced = Vec::new();
    displaced.try_reserve_exact(updates.len()).map_err(|_| "generation3d-replay.displaced-preflight")?;
    for (index, next) in updates { displaced.push(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next)); }
    Ok(Some(Generation3dReplayDisplaced::Widgets(displaced)))
}

/// 🔁️ Direct semantic replay table. It consumes the retained mutation and writes only the
/// addressed field or collection; no VCS diff/apply or whole-snapshot replacement is reachable.
fn generation3d_apply_initialization_mutation(snapshot: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<Option<Generation3dReplayDisplaced>, &'static str> {
    let retired = match mutation {
        Generation3dMutation::CreateWidget(payload) => {
            if snapshot.host_snapshot.widgets.iter().any(|entry| crate::widget_id(entry) == crate::widget_id(&payload.widget)) {
                return Err("generation3d-replay.widget-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.widgets.len());
            snapshot.host_snapshot.widgets.insert(index, generation3d_copy_widget(&payload.widget)?);
            None
        }
        Generation3dMutation::UpdateWidget(payload) => {
            let id = crate::widget_id(&payload.widget);
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == id).ok_or("generation3d-replay.widget-missing")?;
            for (index, synapse) in crate::standards::v1::subsets::any::schema::mutations::update_widget::variable_synapses(&snapshot.host_snapshot.widgets[index], &payload.widget, &snapshot.host_snapshot.synapses) { snapshot.host_snapshot.synapses[index] = synapse; }
            Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], generation3d_copy_widget(&payload.widget)?)))
        }
        Generation3dMutation::DeleteWidget(payload) => {
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            Some(Generation3dReplayDisplaced::Widget(snapshot.host_snapshot.widgets.remove(index)))
        }
        Generation3dMutation::ConnectSynapse(payload) => {
            if snapshot.host_snapshot.synapses.iter().any(|entry| entry.id == payload.synapse.id) {
                return Err("generation3d-replay.synapse-duplicate");
            }
            let index = payload.index.min(snapshot.host_snapshot.synapses.len());
            snapshot.host_snapshot.synapses.insert(index, generation3d_copy_synapse(&payload.synapse)?);
            None
        }
        Generation3dMutation::UpdateSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.synapse.id).ok_or("generation3d-replay.synapse-missing")?;
            Some(Generation3dReplayDisplaced::Synapse(std::mem::replace(&mut snapshot.host_snapshot.synapses[index], generation3d_copy_synapse(&payload.synapse)?)))
        }
        Generation3dMutation::DisconnectSynapse(payload) => {
            let index = snapshot.host_snapshot.synapses.iter().position(|entry| entry.id == payload.id).ok_or("generation3d-replay.synapse-missing")?;
            Some(Generation3dReplayDisplaced::Synapse(snapshot.host_snapshot.synapses.remove(index)))
        }
        Generation3dMutation::MoveWidget(payload) => {
            if !payload.layout.x.is_finite() || !payload.layout.y.is_finite() {
                return Err("generation3d-replay.layout-nonfinite");
            }
            snapshot
                .host_snapshot
                .layout
                .insert(generation3d_copy_string(&payload.id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: payload.layout.x, y: payload.layout.y })
                .map(Generation3dReplayDisplaced::Layout)
                
        }
        Generation3dMutation::DeleteWidgetPosition(payload) => snapshot.host_snapshot.layout.remove(&payload.id).map(Generation3dReplayDisplaced::Layout),
        Generation3dMutation::UpdateCamera(payload) => {
            if !payload.camera.x.is_finite() || !payload.camera.y.is_finite() || !payload.camera.zoom.is_finite() {
                return Err("generation3d-replay.camera-nonfinite");
            }
            Some(Generation3dReplayDisplaced::Camera(std::mem::replace(&mut snapshot.host_snapshot.camera, semio_framework_artifact_flow_flow::CameraJson { x: payload.camera.x, y: payload.camera.y, zoom: payload.camera.zoom })))
        }
        Generation3dMutation::ChangeSchema(payload) => Some(Generation3dReplayDisplaced::Text(std::mem::replace(&mut snapshot.host_snapshot.schema, generation3d_copy_string(&payload.new_schema)?))),
        Generation3dMutation::CreateGeneration(payload) => {
            let generation = snapshot.generation.cold_builder_mut()?;
            if generation.generations.iter().any(|entry| entry.id == payload.generation.id) {
                return Err("generation3d-replay.generation-duplicate");
            }
            let mut selected = String::new();
            selected.try_reserve_exact(payload.generation.id.len()).map_err(|_| "generation3d-replay.selected-generation-preflight")?;
            for character in payload.generation.id.chars() {
                selected.push(character);
            }
            let at = payload.index.unwrap_or(generation.generations.len()).min(generation.generations.len());
            generation.generations.insert(at, generation3d_copy_generation(&payload.generation)?);
            generation.selected_generation_id = Some(selected);
            None
        }
        Generation3dMutation::DeleteGeneration(payload) => {
            let generation = snapshot.generation.cold_builder_mut()?;
            let index = generation.generations.iter().position(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            let removed = generation.generations.remove(index);
            if generation.selected_generation_id.as_deref() == Some(payload.id.as_str()) {
                let mut selected = None;
                if let Some(first) = generation.generations.first() {
                    let mut id = String::new();
                    id.try_reserve_exact(first.id.len()).map_err(|_| "generation3d-replay.selected-generation-preflight")?;
                    for character in first.id.chars() {
                        id.push(character);
                    }
                    selected = Some(id);
                }
                generation.selected_generation_id = selected;
            }
            Some(Generation3dReplayDisplaced::Generation(removed))
        }
        Generation3dMutation::RenameGeneration(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            Some(Generation3dReplayDisplaced::Text(std::mem::replace(&mut entry.name, generation3d_copy_string(&payload.new_name)?)))
        }
        Generation3dMutation::ChangeGenerationValue(payload) => {
            let entry = snapshot.generation.cold_builder_mut()?.generations.iter_mut().find(|entry| entry.id == payload.id).ok_or("generation3d-replay.generation-missing")?;
            entry.values.insert(generation3d_copy_string(&payload.question_id)?, generation3d_copy_json(&payload.new_value, 0)?).map(Generation3dReplayDisplaced::Json)
        }
        Generation3dMutation::ChangeSliderValue(payload) => {
            if !payload.value.is_finite() { return Err("generation3d-replay.slider-nonfinite"); }
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            let mut next = generation3d_copy_widget(&snapshot.host_snapshot.widgets[index])?;
            if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut next, payload.value) { next.retire_cold(); return Err("generation3d-replay.slider-target"); }
            Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next)))
        }
        Generation3dMutation::DragTransforms(payload) => {
            use crate::standards::v1::subsets::any::schema::mutations::{generation3d_param_vector,generation3d_vector_literal,GENERATION3D_TRANSLATE_KINDS};

            let delta = [payload.dx, payload.dy, payload.dz];
            if delta.iter().any(|value| !value.is_finite()) { return Err("generation3d-replay.drag-nonfinite"); }
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_TRANSLATE_KINDS, delta == [0.0; 3], |params| {
                let current = generation3d_param_vector(params, "offset", [0.0; 3]);
                let next: [f64; 3] = std::array::from_fn(|axis| current[axis] + delta[axis]);
                next.iter().all(|value| value.is_finite()).then(|| vec![("offset", generation3d_vector_literal("vector", next))])
            })?
        }
        Generation3dMutation::RotateTransforms(payload) => {
            use crate::standards::v1::subsets::any::schema::mutations::{generation3d_param_vector,generation3d_param_number,generation3d_vector_literal,generation3d_number_literal,GENERATION3D_ROTATE_KINDS};

            use crate::standards::v1::subsets::any::schema::transforms::AxisAngle;
            let axis = [payload.ax, payload.ay, payload.az];
            if axis == [0.0; 3] || axis.iter().chain([&payload.angle]).any(|value| !value.is_finite()) { return Err("generation3d-replay.rotation-invariant"); }
            let delta = AxisAngle { axis, angle: payload.angle };
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_ROTATE_KINDS, payload.angle == 0.0, |params| {
                let current = AxisAngle { axis: generation3d_param_vector(params, "axis", [0.0, 0.0, 1.0]), angle: generation3d_param_number(params, "angle", 0.0) };
                let next = current.then(delta).ok()?;
                Some(vec![("axis", generation3d_vector_literal("vector", next.axis)), ("angle", generation3d_number_literal(next.angle))])
            })?
        }
        Generation3dMutation::ScaleTransforms(payload) => {
            use crate::standards::v1::subsets::any::schema::mutations::{generation3d_param_vector,generation3d_vector_literal,GENERATION3D_SCALE_KINDS};

            use crate::standards::v1::subsets::any::schema::transforms::compose_scale;
            let factors = [payload.sx, payload.sy, payload.sz];
            if factors.iter().any(|value| !value.is_finite() || *value <= 0.0) { return Err("generation3d-replay.scale-invariant"); }
            generation3d_replay_transforms(snapshot, &payload.targets, &GENERATION3D_SCALE_KINDS, factors == [1.0; 3], |params| {
                let next = compose_scale(generation3d_param_vector(params, "factor", [1.0; 3]), factors).ok()?;
                Some(vec![("factor", generation3d_vector_literal("vector", next)), ("center", generation3d_vector_literal("point", [0.0; 3]))])
            })?
        }
        Generation3dMutation::MoveNodes(payload) => {
            crate::standards::v1::subsets::any::schema::mutations::generation3d_targets_invariant(&payload.ids)?;
            if !payload.dx.is_finite() || !payload.dy.is_finite() || payload.ids.len() > GENERATION3D_MAXIMUM_DOMAIN_ITEMS { return Err("generation3d-replay.nodes-invariant"); }
            let mut updates = Vec::new();
            updates.try_reserve_exact(payload.ids.len()).map_err(|_| "generation3d-replay.layout-preflight")?;
            for id in &payload.ids {
                if !snapshot.host_snapshot.widgets.iter().any(|entry| crate::widget_id(entry) == id) { continue; }
                if let Some(layout) = snapshot.host_snapshot.layout.get(id) {
                    let next = semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x + payload.dx, y: layout.y + payload.dy };
                    if !next.x.is_finite() || !next.y.is_finite() { return Err("generation3d-replay.layout-nonfinite"); }
                    updates.push((generation3d_copy_string(id)?, next));
                }
            }
            if updates.is_empty() { return Err("generation3d-replay.nodes-target"); }
            if (payload.dx, payload.dy) == (0.0, 0.0) { None } else {
                let displaced = snapshot.host_snapshot.layout.clone();
                for (id, next) in updates { snapshot.host_snapshot.layout.insert(id, next); }
                Some(Generation3dReplayDisplaced::Layouts(displaced))
            }
        }
        Generation3dMutation::SelectGeneration(payload) => {
            if let Some(id) = &payload.generation_id {
                if !snapshot.generation.generations.iter().any(|entry| &entry.id == id) { return Err("generation3d-replay.generation-missing"); }
            }
            let next = payload.generation_id.as_deref().map(generation3d_copy_string).transpose()?;
            std::mem::replace(&mut snapshot.generation.cold_builder_mut()?.selected_generation_id, next).map(Generation3dReplayDisplaced::Text)
        }
        Generation3dMutation::ChangeGenerationPreview(payload) => {
            let next = payload.text.as_deref().map(generation3d_copy_string).transpose()?;
            std::mem::replace(&mut snapshot.generation.cold_builder_mut()?.preview_text, next).map(Generation3dReplayDisplaced::Text)
        }
        Generation3dMutation::ChangeWidgetInput(payload) => {
            if !payload.admissible() { return Err("generation3d-replay.input-invariant"); }
            let index = snapshot.host_snapshot.widgets.iter().position(|entry| crate::widget_id(entry) == payload.id).ok_or("generation3d-replay.widget-missing")?;
            let wired = snapshot.host_snapshot.synapses.iter().any(|synapse| synapse.to == payload.id && synapse.to_port == payload.channel);
            match payload.landing(&snapshot.host_snapshot.widgets[index], wired).map_err(|_| "generation3d-replay.input-target")? {
                None => None,
                Some(next) => Some(Generation3dReplayDisplaced::Widget(std::mem::replace(&mut snapshot.host_snapshot.widgets[index], next))),
            }
        }
    };
    Ok(retired)
}
//#endregion 🔖️RetainedMountedIngress

//#region 🔖️TypedOwnedEnvelopeCatalog

#[derive(Default)]
struct Generation3dMutationWidgetOwner {
    keyword: String,
    strings: [String; 4],
    numbers: [f64; 4],
    boolean: bool,
    lists: [Vec<String>; 2],
    dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
struct Generation3dMutationSynapseOwner {
    id: String,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
}

#[derive(Default)]
struct Generation3dMutationDictionaryEntryOwner {
    key: String,
    value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
enum Generation3dMutationDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded neural value belongs. A `Dictionary` reaches the wire either as a columnar
/// `Table` (many rows) or as a `List` of one-entry records, and the retained owner has to write the
/// value back into whichever of the two shapes it is standing in.
enum Generation3dMutationNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

enum Generation3dMutationFrame {
    Root { field: Option<u16> },
    Statements { keyword: Option<String> },
    Widget { field: Option<u16>, owner: Generation3dMutationWidgetOwner },
    Synapse { field: Option<u16>, owner: Generation3dMutationSynapseOwner },
    Layout { field: Option<u16>, value: semio_framework_artifact_flow_flow::WidgetLayout },
    Camera { field: Option<u16>, value: semio_framework_artifact_flow_flow::CameraJson },
    Generation { field: Option<u16>, id: String, name: String, values: Vec<(String, semio_framework_value::DslValue)> },
    Dictionary { destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    DictionaryEntries { destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner> },
    DictionaryEntry { entries: usize, row: usize, field: Option<u16> },
    NeuralValue { owner: Generation3dMutationNeuralOwner, field: Option<u16>, value: Option<semio_framework_artifact_flow_flow::neural::Value> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Wire { parent: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural(store::mounted_pack_rt::RetainedValueContainer),
}

#[derive(Clone, Copy)]
enum Generation3dMutationStringTarget {
    Root(u16),
    Widget(usize, u16),
    Generation(usize, u16),
    DictionaryKey(usize, usize),
    DictionaryEntryKey(usize),
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

enum Generation3dMutationJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

enum Generation3dMutationDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

#[derive(Clone, Copy)]
enum Generation3dMutationJsonDestination {
    ChangeValue,
    Generation(usize),
}

struct Generation3dMutationStringOwner {
    target: Generation3dMutationStringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

/// 🧬️ Fixed-depth typed owner for the exact twenty-two Generation3d mutation records.
/// Dynamic JSON is admitted only at the ChangeGenerationValue value and the ChangeWidgetInput input leaves.
struct Generation3dRetainedMutationOwner {
    ordinal: u8,
    stack: Vec<Generation3dMutationFrame>,
    string: Option<Generation3dMutationStringOwner>,
    strings: [String; 3],
    targets: Vec<String>,
    closing_targets: semio_framework_artifact_flow_flow::retained::FlowRetirement,
    numbers: [f64; 4],
    index: usize,
    index_present: bool,
    widget: Option<semio_framework_artifact_flow_flow::Widget>,
    synapse: Option<semio_framework_artifact_flow_flow::SynapseSpec>,
    layout: Option<semio_framework_artifact_flow_flow::WidgetLayout>,
    camera: Option<semio_framework_artifact_flow_flow::CameraJson>,
    generation: Option<semio_framework_artifact_playbook_playbook::FormGeneration>,
    json: semio_framework_value::DslValue,
    json_stack: Vec<Generation3dMutationJsonFrame>,
    json_destination: Option<Generation3dMutationJsonDestination>,
    dsl_stack: Vec<Generation3dMutationDslFrame>,
    dsl_destination: Option<(usize, usize)>,
    pending_table_rows: Option<u64>,
    value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    complete: bool,
    handed_back: bool,
}

impl Generation3dRetainedMutationOwner {
    fn new(ordinal: u8) -> Result<Self, &'static str> {
        if usize::from(ordinal) >= GENERATION3D_MUTATION_VARIANT_COUNT {
            return Err("generation3d-mutation.variant");
        }
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.dsl-stack-preflight")?;
        Ok(Self {
            ordinal,
            stack,
            string: None,
            strings: std::array::from_fn(|_| String::new()),
            targets: Vec::new(),
            closing_targets: Default::default(),
            numbers: [0.0; 4],
            index: 0,
            index_present: false,
            widget: None,
            synapse: None,
            layout: None,
            camera: None,
            generation: None,
            json: semio_framework_value::DslValue::Null,
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

    fn push(&mut self, frame: Generation3dMutationFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation3d-mutation.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    fn root_field(&self) -> Option<u16> {
        self.stack.iter().find_map(|frame| match frame {
            Generation3dMutationFrame::Root { field } => *field,
            _ => None,
        })
    }

    fn string_target(&mut self) -> Result<Generation3dMutationStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.string-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation3dMutationJsonFrame::Object { key: None, .. }) => Generation3dMutationStringTarget::JsonKey,
                _ => Generation3dMutationStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation3dMutationDslFrame::Object { key: None, .. }) => Generation3dMutationStringTarget::DslKey,
                _ => Generation3dMutationStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation3dMutationFrame::Root { field: Some(field) } => Ok(Generation3dMutationStringTarget::Root(*field)),
            Generation3dMutationFrame::Statements { keyword: None } => Ok(Generation3dMutationStringTarget::Statement(index)),
            Generation3dMutationFrame::Widget { field: Some(field), .. } => Ok(Generation3dMutationStringTarget::Widget(index, *field)),
            Generation3dMutationFrame::Generation { field: Some(field), .. } => Ok(Generation3dMutationStringTarget::Generation(index, *field)),
            Generation3dMutationFrame::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mutation.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation3dMutationStringTarget::DictionaryKey(index, row))
            }
            Generation3dMutationFrame::NeuralValue { field: Some(4), .. } => Ok(Generation3dMutationStringTarget::NeuralText(index)),
            Generation3dMutationFrame::DictionaryEntry { field: Some(0), .. } => Ok(Generation3dMutationStringTarget::DictionaryEntryKey(index)),
            Generation3dMutationFrame::Strings { .. } => Ok(Generation3dMutationStringTarget::Sequence(index)),
            Generation3dMutationFrame::Synapse { field: Some(0), .. } => Ok(Generation3dMutationStringTarget::SynapseId(index)),
            Generation3dMutationFrame::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation3dMutationStringTarget::Wire(index, target))
            }
            _ => Err("generation3d-mutation.string-role"),
        }
    }

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation3d-mutation.string-overlap");
        }
        self.string = Some(Generation3dMutationStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let characters = body.symbol_chars(symbol).map_err(|_| "generation3d-mutation.symbol")?;
        let owner = self.string.as_mut().expect("P3 mutation string retained");
        owner.value.try_reserve_exact(characters).map_err(|_| "generation3d-mutation.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, characters));
        if characters == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    fn grant_symbol(&mut self, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, characters)) = owner.symbol else { return Ok(false) };
        owner.value.push(body.symbol_char(symbol, index).map_err(|_| "generation3d-mutation.symbol-char")?.ok_or("generation3d-mutation.symbol-short")?);
        if index + 1 == characters {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P3 mutation symbol retained").symbol = Some((symbol, index + 1, characters));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation3d-mutation.string-handoff")?;
        match owner.target {
            Generation3dMutationStringTarget::Root(field) => {
                let slot = match (self.ordinal, field) {
                    (2 | 5 | 6 | 7 | 9 | 11 | 14, 0) => 0,
                    (12 | 13 | 19, 0) => 0,
                    (12 | 13 | 19, 1) => 1,
                    (20 | 21, 0) => {
                        self.index = 1;
                        0
                    }
                    _ => return Err("generation3d-mutation.root-string-field"),
                };
                self.strings[slot] = owner.value;
                if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMutationStringTarget::Statement(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Statements { keyword }) => *keyword = Some(owner.value),
                _ => return Err("generation3d-mutation.statement-owner"),
            },
            Generation3dMutationStringTarget::Widget(index, field) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Widget { field: active, owner: widget }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation3d-mutation.widget-string")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation3d-mutation.widget-owner"),
            },
            Generation3dMutationStringTarget::Generation(index, field) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Generation { field: active, id, name, .. }) => {
                    if field == 0 {
                        *id = owner.value;
                    } else if field == 1 {
                        *name = owner.value;
                    } else {
                        return Err("generation3d-mutation.generation-string");
                    }
                    *active = None;
                }
                _ => return Err("generation3d-mutation.generation-owner"),
            },
            Generation3dMutationStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mutation.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation3d-mutation.dictionary-key-owner"),
            },
            Generation3dMutationStringTarget::DictionaryEntryKey(index) => {
                let (entries, row) = match self.stack.get_mut(index) {
                    Some(Generation3dMutationFrame::DictionaryEntry { entries, row, field }) => {
                        *field = None;
                        (*entries, *row)
                    }
                    _ => return Err("generation3d-mutation.dictionary-entry-owner"),
                };
                match self.stack.get_mut(entries) {
                    Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mutation.dictionary-entry-row")?.key = owner.value,
                    _ => return Err("generation3d-mutation.dictionary-entries-owner"),
                }
            }
            Generation3dMutationStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.neural-text-owner"),
            },
            Generation3dMutationStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation3d-mutation.sequence-owner"),
            },
            Generation3dMutationStringTarget::SynapseId(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Synapse { field, owner: synapse }) => {
                    synapse.id = owner.value;
                    *field = None;
                }
                _ => return Err("generation3d-mutation.synapse-owner"),
            },
            Generation3dMutationStringTarget::Wire(index, role) => {
                let parent = match self.stack.get(index) {
                    Some(Generation3dMutationFrame::Wire { parent, .. }) => *parent,
                    _ => return Err("generation3d-mutation.wire-owner"),
                };
                let synapse = match self.stack.get_mut(parent) {
                    Some(Generation3dMutationFrame::Synapse { owner, .. }) => owner,
                    _ => return Err("generation3d-mutation.wire-parent"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    2 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    5 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            Generation3dMutationStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation3dMutationJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mutation.json-key-owner"),
            },
            Generation3dMutationStringTarget::JsonValue => self.assign_json(semio_framework_value::DslValue::String(owner.value))?,
            Generation3dMutationStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation3dMutationDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mutation.dsl-key-owner"),
            },
            Generation3dMutationStringTarget::DslValue => self.assign_dsl(semio_framework_value::DslValue::String(owner.value))?,
        }
        Ok(())
    }

    fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation3dMutationJsonFrame::Array(values)) => values.push(value),
            Some(Generation3dMutationJsonFrame::Object { values, key }) => {
                let key = key.take().ok_or("generation3d-mutation.json-value-key")?;
                values.push((key, value));
            }
            None => match self.json_destination.take().ok_or("generation3d-mutation.json-destination")? {
                Generation3dMutationJsonDestination::ChangeValue => {
                    self.json = value;
                    match self.stack.last_mut() {
                        Some(Generation3dMutationFrame::Root { field }) if *field == Some(2) => *field = None,
                        _ => return Err("generation3d-mutation.change-value-owner"),
                    }
                }
                Generation3dMutationJsonDestination::Generation(index) => {
                    let values = match value {
                        semio_framework_value::DslValue::Object(values) => values,
                        _ => return Err("generation3d-mutation.generation-values-shape"),
                    };
                    match self.stack.get_mut(index) {
                        Some(Generation3dMutationFrame::Generation { field, values: target, .. }) => {
                            *target = values;
                            *field = None;
                        }
                        _ => return Err("generation3d-mutation.generation-values-owner"),
                    }
                }
            },
        }
        Ok(())
    }

    fn begin_json(&mut self, destination: Generation3dMutationJsonDestination) -> Result<(), &'static str> {
        if self.json_destination.is_some() {
            return Err("generation3d-mutation.json-overlap");
        }
        self.json_destination = Some(destination);
        Ok(())
    }

    fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation3dMutationDslFrame::Array(values)) => values.push(value),
            Some(Generation3dMutationDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation3d-mutation.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation3d-mutation.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation3dMutationFrame::Widget { field, owner }) if *field == Some((slot + 2) as u16) => {
                        owner.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation3d-mutation.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    fn begin_dsl(&mut self) -> bool {
        if self.dsl_destination.is_some() {
            return true;
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return false };
        let field = match self.stack.get(parent) {
            Some(Generation3dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
            _ => return false,
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        true
    }

    fn end_dsl(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation3d-mutation.dsl-end")? {
            Generation3dMutationDslFrame::Array(values) if kind == store::mounted_pack_rt::RetainedValueContainer::List => semio_framework_value::DslValue::Array(values),
            Generation3dMutationDslFrame::Object { values, key: None } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => semio_framework_value::DslValue::Object(values),
            _ => return Err("generation3d-mutation.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    fn begin_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation3d-mutation.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-preflight")?;
        values.resize_with(rows, Generation3dMutationDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation3dMutationFrame::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = semio_framework_artifact_flow_flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation3d-mutation.dictionary-value")?);
        }
        match destination {
            Generation3dMutationDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation3dMutationFrame::Widget { field: active, owner }) if *active == Some(field) => {
                    owner.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation3d-mutation.dictionary-widget-owner"),
            },
            Generation3dMutationDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    fn finish_widget(owner: Generation3dMutationWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => semio_framework_artifact_flow_flow::Widget::InputSlider { id, label: second, value, min, max, step },
            "input-note" => semio_framework_artifact_flow_flow::Widget::InputNote { id, text: second },
            "input-image" => semio_framework_artifact_flow_flow::Widget::InputImage { id, src: second },
            "variable" => semio_framework_artifact_flow_flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
                for entry in first_list {
                    expanded.insert(entry);
                }
                semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview: second_dictionary, expanded }
            }
            "output-action" => semio_framework_artifact_flow_flow::Widget::OutputAction { id, action: second },
            "output-export" => semio_framework_artifact_flow_flow::Widget::OutputExport { id, format: second },
            "cluster" => semio_framework_artifact_flow_flow::Widget::Cluster {
                id,
                name: second,
                tree: semio_framework_value::FromValue::from_value(first_dynamic.ok_or("generation3d-mutation.cluster-tree")?).map_err(|_| "generation3d-mutation.cluster-tree-shape")?,
                flow: semio_framework_value::FromValue::from_value(second_dynamic.ok_or("generation3d-mutation.cluster-flow")?).map_err(|_| "generation3d-mutation.cluster-flow-shape")?,
            },
            _ => return Err("generation3d-mutation.widget-variant"),
        })
    }

    fn begin_record(&mut self) -> Result<(), &'static str> {
        if self.stack.is_empty() {
            return self.push(Generation3dMutationFrame::Root { field: None });
        }
        let table = self.stack.len() - 1;
        if let Some(Generation3dMutationFrame::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
            let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mutation.dictionary-value-row")?;
            *next = row + 1;
            return self.push(Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::TableRow { table, row }, field: None, value: None });
        }
        if let Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) = self.stack.get_mut(table) {
            let row = rows.len();
            rows.try_reserve(1).map_err(|_| "generation3d-mutation.dictionary-entry-preflight")?;
            rows.push(Generation3dMutationDictionaryEntryOwner::default());
            return self.push(Generation3dMutationFrame::DictionaryEntry { entries: table, row, field: None });
        }
        if let Some(Generation3dMutationFrame::DictionaryEntry { entries, row, field: Some(1) }) = self.stack.get_mut(table) {
            let owner = Generation3dMutationNeuralOwner::EntryRow { entries: *entries, row: *row };
            return self.push(Generation3dMutationFrame::NeuralValue { owner, field: None, value: None });
        }
        let root = self.root_field();
        let frame = match self.stack.last_mut() {
            Some(Generation3dMutationFrame::Statements { keyword }) => {
                let keyword = keyword.take().ok_or("generation3d-mutation.widget-keyword")?;
                Generation3dMutationFrame::Widget { field: None, owner: Generation3dMutationWidgetOwner { keyword, ..Default::default() } }
            }
            _ => match (self.ordinal, root) {
                (3, Some(1)) | (4, Some(0)) => Generation3dMutationFrame::Synapse { field: None, owner: Default::default() },
                (6, Some(1)) => Generation3dMutationFrame::Layout { field: None, value: semio_framework_artifact_flow_flow::WidgetLayout { x: 0.0, y: 0.0 } },
                (8, Some(0)) => Generation3dMutationFrame::Camera { field: None, value: semio_framework_artifact_flow_flow::CameraJson::default() },
                (10, Some(0)) => Generation3dMutationFrame::Generation { field: None, id: String::new(), name: String::new(), values: Vec::new() },
                _ => Generation3dMutationFrame::Structural(store::mounted_pack_rt::RetainedValueContainer::Record),
            },
        };
        self.push(frame)
    }

    fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        use store::mounted_pack_rt::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: 0x11, .. } => {
                if self.dsl_destination.is_some() {
                    self.begin_dsl();
                } else if self.json_destination.is_none() {
                    if matches!(self.ordinal, 13 | 19) && self.root_field() == Some(2) {
                        self.begin_json(Generation3dMutationJsonDestination::ChangeValue)?;
                    } else {
                        self.begin_dsl();
                    }
                }
            }
            Token::Begin { kind: Container::List, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dsl-count")?).map_err(|_| "generation3d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation3dMutationDslFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dsl-count")?).map_err(|_| "generation3d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation3dMutationDslFrame::Object { values, key: None });
            }
            Token::Begin { kind: Container::List, count } if self.json_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.json-list-count")?).map_err(|_| "generation3d-mutation.json-list-preflight")?;
                self.json_stack.push(Generation3dMutationJsonFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, .. } if self.json_destination.is_some() => {
                self.json_stack.push(Generation3dMutationJsonFrame::Object { values: Vec::new(), key: None });
            }
            Token::Begin { kind: Container::Map, .. } => {
                let index = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.generation-values-owner")?;
                if matches!(self.stack.get(index), Some(Generation3dMutationFrame::Generation { field: Some(2), .. })) {
                    self.begin_json(Generation3dMutationJsonDestination::Generation(index))?;
                    self.json_stack.push(Generation3dMutationJsonFrame::Object { values: Vec::new(), key: None });
                } else {
                    self.push(Generation3dMutationFrame::Structural(Container::Map))?;
                }
            }
            Token::Begin { kind: Container::Table, count } => {
                if self.pending_table_rows.take() != Some(count) {
                    return Err("generation3d-mutation.table-row-count");
                }
                let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Generation3dMutationFrame::Widget { field: Some(field @ (1 | 5)), .. }) => Generation3dMutationDictionaryDestination::Widget { parent, field: *field },
                    Some(Generation3dMutationFrame::NeuralValue { field: Some(5), .. }) => Generation3dMutationDictionaryDestination::Value { parent },
                    _ => {
                        self.push(Generation3dMutationFrame::Structural(Container::Table))?;
                        return Ok(());
                    }
                };
                self.begin_dictionary(destination, count)?;
            }
            Token::Begin { kind: Container::Record, .. } => self.begin_record()?,
            Token::Begin { kind: Container::Statements, .. } => self.push(Generation3dMutationFrame::Statements { keyword: None })?,
            Token::Begin { kind: Container::List | Container::Tuple, count } => {
                let (parent, field) = match self.stack.last() {
                    Some(Generation3dMutationFrame::NeuralValue { field: Some(5), value: None, .. }) => {
                        let parent = self.stack.len() - 1;
                        let mut rows = Vec::new();
                        rows.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dictionary-entry-count")?).map_err(|_| "generation3d-mutation.dictionary-entry-preflight")?;
                        return self.push(Generation3dMutationFrame::DictionaryEntries { destination: Generation3dMutationDictionaryDestination::Value { parent }, rows });
                    }
                    Some(Generation3dMutationFrame::Widget { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    Some(Generation3dMutationFrame::Root { field: Some(0) }) if (15..=18).contains(&self.ordinal) => (self.stack.len() - 1, 0),
                    _ => {
                        self.push(Generation3dMutationFrame::Structural(Container::List))?;
                        return Ok(());
                    }
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "generation3d-mutation.sequence-preflight")?;
                self.push(Generation3dMutationFrame::Strings { parent, field, values })?;
            }
            Token::Begin { kind: Container::Wire, .. } => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.wire-parent")?;
                self.push(Generation3dMutationFrame::Wire { parent, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })?;
            }
            Token::Begin { kind, .. } => self.push(Generation3dMutationFrame::Structural(kind))?,
            Token::Unsigned { role: Role::FieldId, value } if value <= u16::MAX as u64 => match self.stack.last_mut() {
                Some(
                    Generation3dMutationFrame::Root { field }
                    | Generation3dMutationFrame::Widget { field, .. }
                    | Generation3dMutationFrame::Synapse { field, .. }
                    | Generation3dMutationFrame::Layout { field, .. }
                    | Generation3dMutationFrame::Camera { field, .. }
                    | Generation3dMutationFrame::Generation { field, .. }
                    | Generation3dMutationFrame::DictionaryEntry { field, .. }
                    | Generation3dMutationFrame::NeuralValue { field, .. },
                ) if field.is_none() => *field = Some(value as u16),
                _ => return Err("generation3d-mutation.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => if let Some(Generation3dMutationFrame::Dictionary { field, present, next, .. }) = self.stack.last_mut() {
                *field = Some(u16::try_from(value).map_err(|_| "generation3d-mutation.dictionary-field")?);
                present.fill(false);
                *next = 0;
            },
            Token::Unsigned { role: Role::Unsigned, value } if self.json_destination.is_none() && self.dsl_destination.is_none() => {
                self.index = usize::try_from(value).map_err(|_| "generation3d-mutation.index")?;
                self.index_present = true;
                if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation3d-mutation.string-length")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation3d-mutation.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(character) => {
                let owner = self.string.as_mut().ok_or("generation3d-mutation.string-char")?;
                owner.value.push(character);
                let remaining = owner.remaining.as_mut().ok_or("generation3d-mutation.string-width")?;
                *remaining = remaining.checked_sub(character.len_utf8() as u64).ok_or("generation3d-mutation.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, body)?,
            Token::F64(bits) => {
                if self.dsl_destination.is_some() {
                    self.assign_dsl(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(3) && value.is_none() => {
                            *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(f64::from_bits(bits))));
                            *field = None;
                        }
                        Some(Generation3dMutationFrame::Widget { field, owner }) => {
                            let field = field.take().ok_or("generation3d-mutation.widget-number")?;
                            *owner.numbers.get_mut(field.checked_sub(2).ok_or("generation3d-mutation.widget-number-field")? as usize).ok_or("generation3d-mutation.widget-number-field")? = f64::from_bits(bits);
                        }
                        Some(Generation3dMutationFrame::Layout { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            _ => return Err("generation3d-mutation.layout-field"),
                        },
                        Some(Generation3dMutationFrame::Root { field }) if (14..=18).contains(&self.ordinal) => {
                            let slot = field.take().filter(|value| (1..=4).contains(value)).ok_or("generation3d-mutation.semantic-number-field")?;
                            self.numbers[usize::from(slot - 1)] = f64::from_bits(bits);
                        }
                        Some(Generation3dMutationFrame::Camera { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            Some(2) => value.zoom = f64::from_bits(bits),
                            _ => return Err("generation3d-mutation.camera-field"),
                        },
                        _ => return Err("generation3d-mutation.number-owner"),
                    }
                }
            }
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::NeuralValue { field, value: target, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::uint(value))?,
            Token::Tag { value: 0x01 | 0x02, .. } => {
                let boolean = matches!(token, Token::Tag { value: 0x02, .. });
                if self.dsl_destination.is_some() {
                    self.assign_dsl(semio_framework_value::DslValue::Bool(boolean))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(semio_framework_value::DslValue::Bool(boolean))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation3dMutationFrame::Widget { field, owner }) => {
                            owner.boolean = boolean;
                            *field = None;
                        }
                        Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if matches!(*field, Some(0 | 1)) && value.is_none() => {
                            *value = Some(if *field == Some(0) {
                                semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null)
                            } else {
                                semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(boolean))
                            });
                            *field = None;
                        }
                        _ => return Err("generation3d-mutation.boolean-owner"),
                    }
                }
            }
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Null)?,
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Wire { roles, roles_len, nodes, .. }) => {
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
                _ => return Err("generation3d-mutation.wire-node"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len() && value == 0 => {
                    present.fill(true);
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => if let Some(Generation3dMutationFrame::Dictionary { present, .. }) = self.stack.last_mut() {
                for bit in 0..8 {
                    let row = first_row as usize + bit;
                    if row < present.len() {
                        present[row] = value & (1 << bit) != 0;
                    }
                }
            },
            Token::End(kind) => {
                if self.end_dsl(kind)? {
                    return Ok(());
                }
                if self.json_destination.is_some() && matches!(kind, Container::List | Container::Map) {
                    let value = match self.json_stack.pop().ok_or("generation3d-mutation.json-end-owner")? {
                        Generation3dMutationJsonFrame::Array(values) if kind == Container::List => semio_framework_value::DslValue::Array(values),
                        Generation3dMutationJsonFrame::Object { values, key: None } if kind == Container::Map => semio_framework_value::DslValue::Object(values),
                        _ => return Err("generation3d-mutation.json-end-mismatch"),
                    };
                    self.assign_json(value)?;
                    return Ok(());
                }
                let frame = self.stack.pop().ok_or("generation3d-mutation.end-owner")?;
                match frame {
                    Generation3dMutationFrame::Root { field: None } if kind == Container::Record => {}
                    Generation3dMutationFrame::Widget { field: None, owner } if kind == Container::Record => self.widget = Some(Self::finish_widget(owner)?),
                    Generation3dMutationFrame::Synapse { field: None, owner } if kind == Container::Record => {
                        self.synapse = Some(semio_framework_artifact_flow_flow::SynapseSpec { id: owner.id, from: owner.from, to: owner.to, from_port: owner.from_port, to_port: owner.to_port });
                    }
                    Generation3dMutationFrame::Layout { field: None, value } if kind == Container::Record => self.layout = Some(value),
                    Generation3dMutationFrame::Camera { field: None, value } if kind == Container::Record => self.camera = Some(value),
                    Generation3dMutationFrame::Generation { field: None, id, name, values } if kind == Container::Record => {
                        self.generation = Some(semio_framework_artifact_playbook_playbook::FormGeneration { id, name, values: values.into_iter().collect() });
                    }
                    Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::TableRow { table, row }, field: None, value: Some(value) } if kind == Container::Record => match self.stack.get_mut(table) {
                        Some(Generation3dMutationFrame::Dictionary { rows, field: Some(1), .. }) => {
                            rows.get_mut(row).ok_or("generation3d-mutation.dictionary-value-row")?.value = Some(value);
                        }
                        _ => return Err("generation3d-mutation.dictionary-value-table"),
                    },
                    Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::EntryRow { entries, row }, field: None, value: Some(value) } if kind == Container::Record => {
                        match self.stack.get_mut(entries) {
                            Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) => {
                                rows.get_mut(row).ok_or("generation3d-mutation.dictionary-entry-row")?.value = Some(value);
                            }
                            _ => return Err("generation3d-mutation.dictionary-entry-list"),
                        }
                        if let Some(Generation3dMutationFrame::DictionaryEntry { field, .. }) = self.stack.last_mut() {
                            *field = None;
                        }
                    }
                    Generation3dMutationFrame::DictionaryEntry { field: None, .. } if kind == Container::Record => {}
                    Generation3dMutationFrame::DictionaryEntries { destination, rows } if matches!(kind, Container::List | Container::Tuple) => self.finish_dictionary(destination, rows)?,
                    Generation3dMutationFrame::Dictionary { destination, rows, field: Some(1), .. } if kind == Container::Table => self.finish_dictionary(destination, rows)?,
                    Generation3dMutationFrame::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                        Some(Generation3dMutationFrame::Root { field: active }) if (15..=18).contains(&self.ordinal) && field == 0 => { self.targets = values; *active = None; }
                        Some(Generation3dMutationFrame::Widget { field: active, owner }) => {
                            owner.lists[if field == 4 { 1 } else { 0 }] = values;
                            *active = None;
                        }
                        _ => return Err("generation3d-mutation.sequence-parent"),
                    },
                    Generation3dMutationFrame::Wire { parent, .. } if kind == Container::Wire => {
                        if let Some(Generation3dMutationFrame::Synapse { field, .. }) = self.stack.get_mut(parent) {
                            *field = None;
                        }
                    }
                    Generation3dMutationFrame::Statements { .. } if kind == Container::Statements => {}
                    Generation3dMutationFrame::Structural(expected) if expected == kind => {}
                    _ => return Err("generation3d-mutation.end-mismatch"),
                }
                if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() || self.json_destination.is_some() || !self.json_stack.is_empty() || self.dsl_destination.is_some() || !self.dsl_stack.is_empty() || self.pending_table_rows.is_some() {
                    return Err("generation3d-mutation.terminal-populated");
                }
                let strings = std::mem::replace(&mut self.strings, std::array::from_fn(|_| String::new()));
                let [first, second, _third] = strings;
                let mutation = match self.ordinal {
                    0 => Generation3dMutation::CreateWidget(CreateWidget { index: self.index, widget: self.widget.take().ok_or("generation3d-mutation.create-widget")? }),
                    1 => Generation3dMutation::UpdateWidget(UpdateWidget { widget: self.widget.take().ok_or("generation3d-mutation.update-widget")? }),
                    2 => Generation3dMutation::DeleteWidget(DeleteWidget { id: first }),
                    3 => Generation3dMutation::ConnectSynapse(ConnectSynapse { index: self.index, synapse: self.synapse.take().ok_or("generation3d-mutation.connect-synapse")? }),
                    4 => Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: self.synapse.take().ok_or("generation3d-mutation.update-synapse")? }),
                    5 => Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: first }),
                    6 => Generation3dMutation::MoveWidget(MoveWidget { id: first, layout: self.layout.take().ok_or("generation3d-mutation.move-widget")? }),
                    7 => Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: first }),
                    8 => Generation3dMutation::UpdateCamera(UpdateCamera { camera: self.camera.take().ok_or("generation3d-mutation.update-camera")? }),
                    9 => Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: first }),
                    10 => Generation3dMutation::CreateGeneration(CreateGeneration { generation: self.generation.take().ok_or("generation3d-mutation.create-generation")?, index: self.index_present.then_some(self.index) }),
                    11 => Generation3dMutation::DeleteGeneration(DeleteGeneration { id: first }),
                    12 => Generation3dMutation::RenameGeneration(RenameGeneration { id: first, new_name: second }),
                    13 => Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: first, question_id: second, new_value: std::mem::replace(&mut self.json, semio_framework_value::DslValue::Null) }),
                    14 => Generation3dMutation::ChangeSliderValue(ChangeSliderValue { id: first, value: self.numbers[0] }),
                    15 => Generation3dMutation::DragTransforms(DragTransforms { targets: std::mem::take(&mut self.targets), dx: self.numbers[0], dy: self.numbers[1], dz: self.numbers[2] }),
                    16 => Generation3dMutation::RotateTransforms(RotateTransforms { targets: std::mem::take(&mut self.targets), ax: self.numbers[0], ay: self.numbers[1], az: self.numbers[2], angle: self.numbers[3] }),
                    17 => Generation3dMutation::ScaleTransforms(ScaleTransforms { targets: std::mem::take(&mut self.targets), sx: self.numbers[0], sy: self.numbers[1], sz: self.numbers[2] }),
                    18 => Generation3dMutation::MoveNodes(MoveNodes { ids: std::mem::take(&mut self.targets), dx: self.numbers[0], dy: self.numbers[1] }),
                    19 => Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id: first, channel: second, input: <WidgetInputValue as semio_framework_value::FromValue>::from_value(std::mem::replace(&mut self.json, semio_framework_value::DslValue::Null)).map_err(|_| "generation3d-mutation.widget-input")? }),
                    20 => Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: (self.index == 1).then_some(first) }),
                    21 => Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: (self.index == 1).then_some(first) }),
                    _ => return Err("generation3d-mutation.variant"),
                };
                *self.value = Some(mutation);
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Generation3dMutation> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    fn close_step(&mut self) -> bool {
        self.string = None;
        if !self.closing_targets.terminal_is_empty() {
            generation3d_close_flow_frontier(&mut self.closing_targets).expect("admitted semantic target retirement");
            return false;
        }
        if let Some(frame) = self.stack.pop() {
            if let Generation3dMutationFrame::Strings { values, .. } = frame { generation3d_park_flow_strings(&mut self.closing_targets, values); }
            return false;
        }
        if !self.targets.is_empty() {
            generation3d_park_flow_strings(&mut self.closing_targets, std::mem::take(&mut self.targets));
            return false;
        }
        if let Some(mut mutation) = self.value.take() {
            let targets = match &mut mutation {
                Generation3dMutation::DragTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::RotateTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::ScaleTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::MoveNodes(value) => Some(std::mem::take(&mut value.ids)),
                _ => None,
            };
            if let Some(targets) = targets { generation3d_park_flow_strings(&mut self.closing_targets, targets); }
            mutation.retire_cold();
            return false;
        }
        drop(self.widget.take());
        drop(self.synapse.take());
        self.layout = None;
        self.camera = None;
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
            && self.targets.is_empty()
            && self.closing_targets.terminal_is_empty()
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

impl Drop for Generation3dRetainedMutationOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation3d retained mutation owner reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationSessionPhase {
    Format,
    Ordinal,
    Body,
    Ready,
    Published,
    Closing,
    Closed,
}

struct Generation3dMutationSession {
    phase: Generation3dMutationSessionPhase,
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
    owner: std::mem::ManuallyDrop<Option<Generation3dRetainedMutationOwner>>,
}

impl Generation3dMutationSession {
    fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if !(3..=GENERATION3D_OWNER_BYTES).contains(&expected_bytes) || maximum_items == 0 {
            return Err("generation3d-mutation.exact-credits");
        }
        Ok(Self {
            phase: Generation3dMutationSessionPhase::Format,
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
            && (self.phase != Generation3dMutationSessionPhase::Body
                || self.body.as_ref().is_some_and(store::mounted_pack_rt::RetainedRecordBodyCursor::ingress_ready))
    }

    fn seal(&mut self) -> Result<(), &'static str> {
        if self.pending.is_some() || self.admitted != self.expected_bytes || self.phase != Generation3dMutationSessionPhase::Body {
            return Err("generation3d-mutation.exact-byte-seal");
        }
        self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.seal(self.body_bytes).map_err(|_| "generation3d-mutation.body-seal")?;
        self.sealed = true;
        Ok(())
    }

    fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Generation3dMutationSessionPhase::Ready | Generation3dMutationSessionPhase::Published) {
            return Ok(true);
        }
        if let (Some(owner), Some(body)) = (self.owner.as_mut(), self.body.as_ref()) {
            if owner.grant_symbol(body)? {
                return Ok(false);
            }
        }
        match self.phase {
            Generation3dMutationSessionPhase::Format => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.format-input")?;
                if byte != dsl::variants_binary::OP_BINARY_FORMAT {
                    return Err("generation3d-mutation.format");
                }
                self.phase = Generation3dMutationSessionPhase::Ordinal;
            }
            Generation3dMutationSessionPhase::Ordinal => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.ordinal-input")?;
                if self.ordinal_bytes >= 10 || (self.ordinal_bytes == 9 && (byte & 0xfe) != 0) {
                    return Err("generation3d-mutation.ordinal-overflow");
                }
                self.ordinal |= u64::from(byte & 0x7f) << self.ordinal_shift;
                self.ordinal_shift += 7;
                self.ordinal_bytes += 1;
                if byte & 0x80 == 0 {
                    if self.ordinal_bytes > 1 && byte & 0x7f == 0 {
                        return Err("generation3d-mutation.ordinal-noncanonical");
                    }
                    let ordinal = u8::try_from(self.ordinal).map_err(|_| "generation3d-mutation.variant")?;
                    let limits = store::mounted_pack_rt::PackLimits {
                        max_file_len: self.expected_bytes as u64,
                        max_segment_len: self.expected_bytes as u64,
                        max_symbols: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u32,
                        max_depth: GENERATION3D_RETAINED_STACK_CAPACITY as u16,
                        max_items: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u64,
                        max_total_alloc: GENERATION3D_MAXIMUM_DOMAIN_BYTES as u64,
                    };
                    *self.body = Some(
                        store::mounted_pack_rt::RetainedRecordBodyCursor::try_new(
                            limits,
                            self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS),
                            self.expected_bytes,
                            self.expected_bytes,
                            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,
                        )
                        .map_err(|_| "generation3d-mutation.body-preflight")?,
                    );
                    *self.owner = Some(Generation3dRetainedMutationOwner::new(ordinal)?);
                    self.phase = Generation3dMutationSessionPhase::Body;
                }
            }
            Generation3dMutationSessionPhase::Body => {
                if self.pending.is_some() && self.body.as_ref().ok_or("generation3d-mutation.body-owner")?.ingress_ready() {
                    let (_, byte) = self.pending.take().ok_or("generation3d-mutation.body-input")?;
                    self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.admit_byte(self.body_bytes, byte).map_err(|(_, byte)| if byte == 0 { "generation3d-mutation.body-handback-zero" } else { "generation3d-mutation.body-handback" })?;
                    self.body_bytes += 1;
                }
                if let Some(store::mounted_pack_rt::RetainedRecordBodyToken::Value(token)) = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.grant().map_err(|_| "generation3d-mutation.body-malformed")? {
                    let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                    let body = self.body.as_ref().expect("P3 retained mutation body");
                    self.owner.as_mut().expect("P3 retained mutation owner").accept(token, body)?;
                    if complete {
                        self.phase = Generation3dMutationSessionPhase::Ready;
                        return Ok(true);
                    }
                }
            }
            _ => return Err("generation3d-mutation.session-state"),
        }
        Ok(false)
    }

    fn take(&mut self) -> Option<Generation3dMutation> {
        if self.phase != Generation3dMutationSessionPhase::Ready {
            return None;
        }
        let value = self.owner.as_mut()?.take()?;
        self.phase = Generation3dMutationSessionPhase::Published;
        Some(value)
    }

    fn next_retained_allocation_bytes(&mut self) -> Result<Option<usize>, &'static str> {
        let Some(body) = self.body.as_mut() else { return Ok(None) };
        body.next_allocation_bytes().map_err(|_| "generation3d-mutation.body-allocation")
    }

    fn reserve_retained_allocation(&mut self, maximum_bytes: usize) -> Result<(bool, usize), &'static str> {
        let body = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?;
        body.reserve_allocation(maximum_bytes)
            .map(|step| (step.progressed, step.allocated_bytes))
            .map_err(|_| "generation3d-mutation.body-allocation")
    }

    fn retained_allocated_bytes(&self) -> usize {
        self.body.as_ref().map_or(0, store::mounted_pack_rt::RetainedRecordBodyCursor::allocated_bytes)
    }

    fn next_retained_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() || self.owner.is_some() {
            return None;
        }
        self.body.as_ref()?.next_release_allocation_bytes().ok().flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::mounted_pack_rt::RetainedPackCloseStep, store::PackRefusal> {
        use store::mounted_pack_rt::RetainedPackCloseStep;
        if self.phase == Generation3dMutationSessionPhase::Closed {
            return Ok(RetainedPackCloseStep::Complete);
        }
        if maximum_items == 0 && (self.pending.is_some() || self.owner.is_some() || self.body.is_none()) {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closing;
        if maximum_items != 0 && self.pending.take().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.as_mut() {
            if !owner.close_step() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(self.owner.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(body) = self.body.as_mut() {
            match body.close_step(maximum_items, maximum_bytes)? {
                RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                    return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                }
                RetainedPackCloseStep::Complete => {
                    if maximum_items == 0 {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                }
            }
            drop(self.body.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closed;
        Ok(RetainedPackCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == Generation3dMutationSessionPhase::Closed && self.owner.is_none() && self.body.is_none() && self.pending.is_none()
    }
}

impl Drop for Generation3dMutationSession {
    /// 🔒️ Fail-closed on a live drop, but never while the thread is already unwinding — a second
    /// panic in a destructor during cleanup aborts the process instead of reporting the first
    /// failure. Same guard `OrderedMap::drop` and the framework's retirement roots carry.
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Generation3d mutation session reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dPackSnapshotState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation3dPackSnapshotAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Generation3dPackSnapshotState,
    token: Option<store::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    session: std::mem::ManuallyDrop<Option<crate::standards::v1::subsets::any::io::binary::snapshot::Generation3dMountedPackSession>>,
    value: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dPackSnapshotAuthority {
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.snapshot-close-demand", path).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refusal);
        }
        if self.value.is_some() {
            return store::artifact_retirement_owned_birth_demands(&*self.value).map_err(refusal);
        }
        Ok(Default::default())
    }

    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation3dPackSnapshotState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation3dPackSnapshotState::Published | Generation3dPackSnapshotState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
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

impl store::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot> for Generation3dPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::AwaitToken {
            if !terminal || token.kind != store::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(crate::standards::v1::subsets::any::io::binary::snapshot::generation3d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dPackSnapshotState::Ingest;
        }
        if self.state == Generation3dPackSnapshotState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start));
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 mounted pack session retained").seal().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Generation3dPackSnapshotState::Drive;
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-source-allocation-stalled", retained.start + self.relative as u64));
                }
                cx.consume_fuel(1);
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 mounted pack session retained").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-handback", retained.start + self.relative as u64))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::Drive {
            cx.set_stage("generation3d-retained-canonical-pack");
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 mounted pack session retained").grant().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-malformed", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 mounted pack session retained").take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-handoff", token.start))?);
            self.session.as_mut().expect("P3 mounted pack session retained").request_cancel();
            self.state = Generation3dPackSnapshotState::CloseSession;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 mounted pack session retained").next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(
                self.session.as_mut().expect("P3 mounted pack session retained").close_step(1, maximum_bytes).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-session-close", token.start))?,
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }
            ) {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dPackSnapshotState::Ready;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<Generation3dSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, Generation3dPackSnapshotState::Ready) {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Generation3dPackSnapshotState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let path = self.path;
        let diagnostic = move |code: &'static str, error: semio_framework_value::ValueError| store::OwnedSchemaDecodeDiagnostic::before(code, path).with_native(error);
        let empty = RetainedCloneProgress::default();
        if self.session.is_none() && self.retirement.is_none() && self.value.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            session.request_cancel();
            self.state = Generation3dPackSnapshotState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.snapshot-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted pack session refused its close")))? {
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
            self.state = Generation3dPackSnapshotState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d pack snapshot authority reached Drop before publication or bounded retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation3dMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: Generation3dMutationDecodeState,
    token: Option<store::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    drive_ingress: bool,
    session: std::mem::ManuallyDrop<Option<Generation3dMutationSession>>,
    value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dMutationDecodeAuthority {
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.mutation-close-demand", path).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refusal);
        }
        if self.value.is_some() {
            return store::artifact_retirement_owned_birth_demands(&*self.value).map_err(refusal);
        }
        Ok(Default::default())
    }

    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation3dMutationDecodeState::AwaitToken,
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
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
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
        matches!(self.state, Generation3dMutationDecodeState::Published | Generation3dMutationDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation> for Generation3dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.mutation-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::AwaitToken {
            if !terminal || token.kind != store::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.mutation-item-authority", token.start))?;
            *self.session = Some(Generation3dMutationSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.mutation-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dMutationDecodeState::Ingest;
        }
        if self.state == Generation3dMutationDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start));
            }
            if self.drive_ingress {
                if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))? {
                    let (progressed, _) = self
                        .session
                        .as_mut()
                        .expect("P3 retained mutation session")
                        .reserve_retained_allocation(exact)
                        .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))?;
                    if !progressed {
                        return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", retained.start + self.relative as u64));
                    }
                    cx.consume_fuel(1);
                    return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-ingress-malformed", retained.start + self.relative as u64))?;
                self.drive_ingress = !self.session.as_ref().expect("P3 retained mutation session").ingress_ready();
                cx.consume_fuel(1);
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 retained mutation session").seal().map_err(|_| self.diagnostic("generation3d-envelope.mutation-seal", retained.end))?;
                self.state = Generation3dMutationDecodeState::Drive;
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.mutation-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 retained mutation session").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.mutation-handback", retained.start + self.relative as u64))?;
                self.drive_ingress = true;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::Drive {
            cx.set_stage("generation3d-retained-mutation");
            if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))? {
                let (progressed, _) = self
                    .session
                    .as_mut()
                    .expect("P3 retained mutation session")
                    .reserve_retained_allocation(exact)
                    .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))?;
                if !progressed {
                    return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-malformed", token.start))? {
                return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 retained mutation session").take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-handoff", token.start))?);
            self.state = Generation3dMutationDecodeState::CloseSession;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 retained mutation session").next_retained_release_allocation_bytes().unwrap_or(0);
            match self
                .session
                .as_mut()
                .expect("P3 retained mutation session")
                .close_step(1, maximum_bytes)
                .map_err(|_| self.diagnostic("generation3d-envelope.mutation-session-close", token.start))?
            {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { .. } => return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {}
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dMutationDecodeState::Ready;
            return Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<Generation3dMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation3dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation3dMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let path = self.path;
        let diagnostic = move |code: &'static str, error: semio_framework_value::ValueError| store::OwnedSchemaDecodeDiagnostic::before(code, path).with_native(error);
        let empty = RetainedCloneProgress::default();
        if self.session.is_none() && self.retirement.is_none() && self.value.is_none() {
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            self.state = Generation3dMutationDecodeState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.mutation-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted mutation session refused its close")))? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
            self.state = Generation3dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d mutation authority reached Drop before publication or terminal-empty close");
    }
}

struct Generation3dRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for Generation3dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "generation3d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

/// 🎭️ Owner-local exact catalog for the Generation3d fresh-envelope decode cohort.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Generation3dEnvelopeOwnedFieldCatalog;

/// 📦️ Installs Generation3d's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn generation3d_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<Generation3dSnapshot, Generation3dMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Generation3dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
}

impl store::ArtifactEnvelopeOwnedFieldCatalog<Generation3dSnapshot, Generation3dMutation> for Generation3dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>> {
        Box::new(Generation3dPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation>> {
        Box::new(Generation3dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Generation3dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Generation3dMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog

//#region 🔖️RetainedStoreInitialization
fn generation3d_copy_string(source: &str) -> Result<String, &'static str> {
    let mut target = String::new();
    target.try_reserve_exact(source.len()).map_err(|_| "generation3d-initializer.string-preflight")?;
    for character in source.chars() {
        target.push(character);
    }
    Ok(target)
}

fn generation3d_copy_json(source: &semio_framework_value::DslValue, depth: usize) -> Result<semio_framework_value::DslValue, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.json-depth");
    }
    Ok(match source {
        semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
        semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
        semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
        semio_framework_value::DslValue::String(value) => semio_framework_value::DslValue::String(generation3d_copy_string(value)?),
        semio_framework_value::DslValue::Bytes(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation3d-initializer.json-bytes-preflight")?;
            target.extend_from_slice(values);
            semio_framework_value::DslValue::Bytes(target)
        }
        semio_framework_value::DslValue::Array(values) => {
            let mut target = Vec::new();
            target.try_reserve_exact(values.len()).map_err(|_| "generation3d-initializer.json-array-preflight")?;
            for value in values {
                target.push(generation3d_copy_json(value, depth + 1)?);
            }
            semio_framework_value::DslValue::Array(target)
        }
        semio_framework_value::DslValue::Object(values) => {
            let mut target = Vec::new();
            for (key, value) in values {
                target.push((generation3d_copy_string(key)?, generation3d_copy_json(value, depth + 1)?));
            }
            semio_framework_value::DslValue::Object(target)
        }
    })
}

fn generation3d_copy_neural_value(source: &semio_framework_artifact_flow_flow::neural::Value, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Value, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.neural-depth");
    }
    Ok(match source {
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null) => semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null),
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(*value))
        }
        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(value)) => {
            semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(generation3d_copy_string(value)?))
        }
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(value) => semio_framework_artifact_flow_flow::neural::Value::Dictionary(generation3d_copy_dictionary(value, depth + 1)?),
    })
}

fn generation3d_copy_dictionary(source: &semio_framework_artifact_flow_flow::neural::Dictionary, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Dictionary, &'static str> {
    let mut target = semio_framework_artifact_flow_flow::neural::Dictionary::new();
    for key in source.keys() {
        let value = source.get(key).ok_or("generation3d-initializer.dictionary-owner")?;
        target = target.insert(generation3d_copy_string(key)?, generation3d_copy_neural_value(value, depth + 1)?);
    }
    Ok(target)
}

fn generation3d_copy_tree(source: &semio_framework_artifact_flow_flow::neural::Tree, depth: usize) -> Result<semio_framework_artifact_flow_flow::neural::Tree, &'static str> {
    if depth >= GENERATION3D_RETAINED_STACK_CAPACITY {
        return Err("generation3d-initializer.tree-depth");
    }
    let mut neurons = Vec::new();
    neurons.try_reserve_exact(source.neurons.len()).map_err(|_| "generation3d-initializer.neurons-preflight")?;
    for neuron in &source.neurons {
        neurons.push(semio_framework_artifact_flow_flow::neural::Neuron {
            id: generation3d_copy_string(&neuron.id)?,
            kind: generation3d_copy_string(&neuron.kind)?,
            params: generation3d_copy_dictionary(&neuron.params, depth + 1)?,
            tree: match neuron.tree.as_deref() {
                Some(tree) => Some(Box::new(generation3d_copy_tree(tree, depth + 1)?)),
                None => None,
            },
        });
    }
    let mut synapses = Vec::new();
    synapses.try_reserve_exact(source.synapses.len()).map_err(|_| "generation3d-initializer.tree-synapses-preflight")?;
    for synapse in &source.synapses {
        synapses.push(semio_framework_artifact_flow_flow::neural::Synapse {
            id: generation3d_copy_string(&synapse.id)?,
            from: generation3d_copy_string(&synapse.from)?,
            to: generation3d_copy_string(&synapse.to)?,
            from_port: generation3d_copy_string(&synapse.from_port)?,
            to_port: generation3d_copy_string(&synapse.to_port)?,
        });
    }
    Ok(semio_framework_artifact_flow_flow::neural::Tree { neurons, synapses })
}

fn generation3d_copy_flow_ui(source: &semio_framework_artifact_flow_flow::FlowGui) -> Result<semio_framework_artifact_flow_flow::FlowGui, &'static str> {
    let mut nodes = semio_framework_artifact_flow_flow::OrderedMap::new();
    for (id, node) in &source.nodes {
        let chrome = match &node.chrome {
            semio_framework_artifact_flow_flow::NodeChrome::Plain { preview } => semio_framework_artifact_flow_flow::NodeChrome::Plain { preview: *preview },
            semio_framework_artifact_flow_flow::NodeChrome::Slider { label, min, max, step, value } => {
                semio_framework_artifact_flow_flow::NodeChrome::Slider { label: generation3d_copy_string(label)?, min: *min, max: *max, step: *step, value: *value }
            }
            semio_framework_artifact_flow_flow::NodeChrome::Note { text } => semio_framework_artifact_flow_flow::NodeChrome::Note { text: generation3d_copy_string(text)? },
            semio_framework_artifact_flow_flow::NodeChrome::Image { src } => semio_framework_artifact_flow_flow::NodeChrome::Image { src: generation3d_copy_string(src)? },
            semio_framework_artifact_flow_flow::NodeChrome::Variable { name, schema } => semio_framework_artifact_flow_flow::NodeChrome::Variable { name: generation3d_copy_string(name)?, schema: generation3d_copy_string(schema)? },
        };
        nodes.insert(generation3d_copy_string(id)?, semio_framework_artifact_flow_flow::FlowNodeGui { layout: semio_framework_artifact_flow_flow::WidgetLayout { x: node.layout.x, y: node.layout.y }, chrome });
    }
    let mut previews = Vec::new();
    previews.try_reserve_exact(source.previews.len()).map_err(|_| "generation3d-initializer.previews-preflight")?;
    for preview in &source.previews {
        let source = match &preview.source {
            Some(source) => Some(semio_framework_artifact_flow_flow::FlowChannelRef { neuron: generation3d_copy_string(&source.neuron)?, channel: generation3d_copy_string(&source.channel)? }),
            None => None,
        };
        let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
        for value in &preview.expanded {
            expanded.insert(generation3d_copy_string(value)?);
        }
        previews.push(semio_framework_artifact_flow_flow::FlowPreviewGui {
            id: generation3d_copy_string(&preview.id)?,
            source,
            mode: generation3d_copy_string(&preview.mode)?,
            preview: generation3d_copy_dictionary(&preview.preview, 0)?,
            expanded,
            layout: preview.layout.as_ref().map(|layout| semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y }),
        });
    }
    Ok(semio_framework_artifact_flow_flow::FlowUi { camera: semio_framework_artifact_flow_flow::CameraJson { x: source.camera.x, y: source.camera.y, zoom: source.camera.zoom }, nodes, previews })
}

fn generation3d_copy_widget(source: &semio_framework_artifact_flow_flow::Widget) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
    Ok(match source {
        semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            let mut inputs = Vec::new();
            inputs.try_reserve_exact(input_ports.len()).map_err(|_| "generation3d-initializer.inputs-preflight")?;
            for value in input_ports {
                inputs.push(generation3d_copy_string(value)?);
            }
            let mut outputs = Vec::new();
            outputs.try_reserve_exact(output_ports.len()).map_err(|_| "generation3d-initializer.outputs-preflight")?;
            for value in output_ports {
                outputs.push(generation3d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::Neuron {
                id: generation3d_copy_string(id)?,
                neuron_kind: generation3d_copy_string(neuron_kind)?,
                params: generation3d_copy_dictionary(params, 0)?,
                input_ports: inputs,
                output_ports: outputs,
                preview: *preview,
            }
        }
        semio_framework_artifact_flow_flow::Widget::InputSlider { id, label, value, min, max, step } => {
            semio_framework_artifact_flow_flow::Widget::InputSlider { id: generation3d_copy_string(id)?, label: generation3d_copy_string(label)?, value: *value, min: *min, max: *max, step: *step }
        }
        semio_framework_artifact_flow_flow::Widget::InputNote { id, text } => semio_framework_artifact_flow_flow::Widget::InputNote { id: generation3d_copy_string(id)?, text: generation3d_copy_string(text)? },
        semio_framework_artifact_flow_flow::Widget::InputImage { id, src } => semio_framework_artifact_flow_flow::Widget::InputImage { id: generation3d_copy_string(id)?, src: generation3d_copy_string(src)? },
        semio_framework_artifact_flow_flow::Widget::Variable { id, name, schema } => {
            semio_framework_artifact_flow_flow::Widget::Variable { id: generation3d_copy_string(id)?, name: generation3d_copy_string(name)?, schema: generation3d_copy_string(schema)? }
        }
        semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview, expanded } => {
            let mut next_expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
            for value in expanded {
                next_expanded.insert(generation3d_copy_string(value)?);
            }
            semio_framework_artifact_flow_flow::Widget::OutputPreview { id: generation3d_copy_string(id)?, preview: generation3d_copy_dictionary(preview, 0)?, expanded: next_expanded }
        }
        semio_framework_artifact_flow_flow::Widget::OutputAction { id, action } => semio_framework_artifact_flow_flow::Widget::OutputAction { id: generation3d_copy_string(id)?, action: generation3d_copy_string(action)? },
        semio_framework_artifact_flow_flow::Widget::OutputExport { id, format } => semio_framework_artifact_flow_flow::Widget::OutputExport { id: generation3d_copy_string(id)?, format: generation3d_copy_string(format)? },
        semio_framework_artifact_flow_flow::Widget::Cluster { id, name, tree, flow } => {
            semio_framework_artifact_flow_flow::Widget::Cluster { id: generation3d_copy_string(id)?, name: generation3d_copy_string(name)?, tree: generation3d_copy_tree(tree, 0)?, flow: generation3d_copy_flow_ui(flow)? }
        }
    })
}

fn generation3d_copy_synapse(source: &semio_framework_artifact_flow_flow::SynapseSpec) -> Result<semio_framework_artifact_flow_flow::SynapseSpec, &'static str> {
    Ok(semio_framework_artifact_flow_flow::SynapseSpec {
        id: generation3d_copy_string(&source.id)?,
        from: generation3d_copy_string(&source.from)?,
        to: generation3d_copy_string(&source.to)?,
        from_port: generation3d_copy_string(&source.from_port)?,
        to_port: generation3d_copy_string(&source.to_port)?,
    })
}

fn generation3d_copy_generation(source: &semio_framework_artifact_playbook_playbook::FormGeneration) -> Result<semio_framework_artifact_playbook_playbook::FormGeneration, &'static str> {
    Ok(semio_framework_artifact_playbook_playbook::FormGeneration { id: generation3d_copy_string(&source.id)?, name: generation3d_copy_string(&source.name)?, values: source.values.clone() })
}

struct Generation3dSnapshotCopyCursor {
    target: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    phase: u8,
    index: usize,
    handed_back: bool,
}

impl Generation3dSnapshotCopyCursor {
    fn new(source: &Generation3dSnapshot) -> Result<Self, &'static str> {
        let mut target = Generation3dSnapshot {
            host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot {
                schema: String::new(),
                camera: semio_framework_artifact_flow_flow::CameraJson::default(),
                widgets: Vec::new(),
                synapses: Vec::new(),
                layout: semio_framework_artifact_flow_flow::OrderedMap::new(),
            },
            generation: semio_framework_artifact_playbook_playbook::GenerationPlayState::default().into(),
        };
        target.host_snapshot.widgets.try_reserve_exact(source.host_snapshot.widgets.len()).map_err(|_| "generation3d-initializer.widgets-preflight")?;
        target.host_snapshot.synapses.try_reserve_exact(source.host_snapshot.synapses.len()).map_err(|_| "generation3d-initializer.synapses-preflight")?;
        target.generation.cold_builder_mut()?.generations.try_reserve_exact(source.generation.generations.len()).map_err(|_| "generation3d-initializer.generations-preflight")?;
        Ok(Self { target: std::mem::ManuallyDrop::new(Some(target)), phase: 0, index: 0, handed_back: false })
    }

    fn step(&mut self, source: &Generation3dSnapshot) -> Result<bool, &'static str> {
        let target = self.target.as_mut().ok_or("generation3d-initializer.copy-owner")?;
        match self.phase {
            0 => {
                target.host_snapshot.schema = generation3d_copy_string(&source.host_snapshot.schema)?;
                self.phase = 1;
            }
            1 => {
                target.host_snapshot.camera.x = source.host_snapshot.camera.x;
                self.phase = 2;
            }
            2 => {
                target.host_snapshot.camera.y = source.host_snapshot.camera.y;
                self.phase = 3;
            }
            3 => {
                target.host_snapshot.camera.zoom = source.host_snapshot.camera.zoom;
                self.phase = 4;
            }
            4 if self.index < source.host_snapshot.widgets.len() => {
                target.host_snapshot.widgets.push(generation3d_copy_widget(&source.host_snapshot.widgets[self.index])?);
                self.index += 1;
            }
            4 => {
                self.phase = 5;
                self.index = 0;
            }
            5 if self.index < source.host_snapshot.synapses.len() => {
                target.host_snapshot.synapses.push(generation3d_copy_synapse(&source.host_snapshot.synapses[self.index])?);
                self.index += 1;
            }
            5 => {
                self.phase = 6;
                self.index = 0;
            }
            6 if self.index < source.host_snapshot.layout.len() => {
                let (id, layout) = source.host_snapshot.layout.iter().nth(self.index).ok_or("generation3d-initializer.layout-owner")?;
                target.host_snapshot.layout.insert(generation3d_copy_string(id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y });
                self.index += 1;
            }
            6 => {
                self.phase = 7;
                self.index = 0;
            }
            7 if self.index < source.generation.generations.len() => {
                target.generation.cold_builder_mut()?.generations.push(generation3d_copy_generation(&source.generation.generations[self.index])?);
                self.index += 1;
            }
            7 => {
                target.generation.cold_builder_mut()?.selected_generation_id = match source.generation.selected_generation_id.as_deref() {
                    Some(value) => Some(generation3d_copy_string(value)?),
                    None => None,
                };
                self.phase = 8;
            }
            8 => {
                target.generation.cold_builder_mut()?.preview_text = match source.generation.preview_text.as_deref() {
                    Some(value) => Some(generation3d_copy_string(value)?),
                    None => None,
                };
                self.phase = 9;
            }
            _ => return Ok(true),
        }
        Ok(self.phase == 9)
    }

    fn take(&mut self) -> Option<Generation3dSnapshot> {
        if self.phase != 9 || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.target.take()
    }

    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.target.is_none() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        store::artifact_retirement_owned_birth_demands(&*self.target)
    }

    fn abandon(&mut self, active: &mut Option<Box<dyn store::ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if self.target.is_none() {
            self.handed_back = true;
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let step = store::artifact_retirement_admit_owned(&mut self.target, active, grant)?;
        if self.target.is_none() {
            self.handed_back = true;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.target.is_none()
    }
}

impl Drop for Generation3dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation3d snapshot copy cursor reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CensusHistory { edit: usize, mutation: usize },
    CopyInitial,
    AdoptWorkspace,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildOwners,
    BuildCandidate,
    Complete,
    RetireCancelled,
    RetireFault,
    Cancelled,
    Fault,
}

struct Generation3dStoreInitializationAuthority {
    actor: protocol::ActorId,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    history_items: usize,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>>>,
    copy: std::mem::ManuallyDrop<Option<Generation3dSnapshotCopyCursor>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<Generation3dSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>>>,
    owners: std::mem::ManuallyDrop<Option<store::DocumentStoreOwners<Generation3dSnapshot, Generation3dMutation>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    displaced: std::mem::ManuallyDrop<Option<Generation3dReplayDisplaced>>,
    rejected: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    candidate_disposer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactDocumentStoreDisposer<Generation3dSnapshot, Generation3dMutation>>>,
    closer: std::mem::ManuallyDrop<Option<semio_framework_plugin::ArtifactStoreInitializationJob<Generation3dSnapshot, Generation3dMutation>>>,
    snapshot_factory: std::mem::ManuallyDrop<Option<std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<Generation3dSnapshot>>>>,
    factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    publication: semio_framework_job::RetainedJobPublication,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: Generation3dStoreInitializationPhase,
    resume_phase: Option<Generation3dStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl Generation3dStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        if generation3d_validate_publication_authority(operation, generation).is_err() {
            let _ = generation3d_admit_app_publication_authority(operation, generation);
        }
        let (base_revision, parent_revision) = generation3d_validate_publication_authority(operation, generation).unwrap_or((u64::MAX, u64::MAX));
        Self {
            actor,
            operation,
            generation,
            base_revision,
            parent_revision,
            history_items: 0,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            copy: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            owners: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            displaced: std::mem::ManuallyDrop::new(None),
            rejected: std::mem::ManuallyDrop::new(None),
            candidate_disposer: std::mem::ManuallyDrop::new(None),
            closer: std::mem::ManuallyDrop::new(None),
            snapshot_factory: std::mem::ManuallyDrop::new(Some(std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()))),
            factory_close: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job::RetainedJobPublication::new(),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: Generation3dStoreInitializationPhase::ValidateEnvelope,
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
        self.phase = Generation3dStoreInitializationPhase::RetireFault;
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

    fn pump_owned(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_value::ValueError> {
        if self.active.is_some() {
            let step = store::artifact_retirement_box_close_step(&mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.displaced.is_some() {
            let step = store::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.rejected.is_some() {
            let step = store::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        Ok(false)
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "generation3d initializer close depth overflow"))?;
            Ok(demand)
        };
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if let Some(owner) = self.active.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.displaced.is_some() {
            return store::artifact_retirement_owned_birth_demands(&*self.displaced);
        }
        if self.rejected.is_some() {
            return store::artifact_retirement_owned_birth_demands(&*self.rejected);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return match self.candidate_disposer.as_ref() {
                Some(disposer) => nested(disposer.retirement_demands(candidate, body)?),
                None => Ok(RetirementDemand { depth: 1, ..Default::default() }),
            };
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.constructor_is_complete() && owners.uninstalled_owners_terminal_is_empty() { Ok(RetirementDemand { depth: 1, ..Default::default() }) } else if owners.constructor_is_complete() { nested(owners.uninstalled_owners_demands(body)?) } else { nested(owners.constructor_demands(body)?) };
        }
        if let Some(copy) = self.copy.as_ref() {
            return copy.close_demands();
        }
        if let Some(closer) = self.closer.as_ref() {
            return nested(closer.retirement_demands(body)?);
        }
        if self.envelope.is_some() {
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if let Some(factory) = self.factory_close.as_ref() {
            return nested(factory.demands(body)?);
        }
        if self.snapshot_factory.is_some() {
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<std::sync::Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        use semio_framework_value::{retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress}, FactoryAuthority, ValueError, ValueRefusalKind};
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if demand == Default::default() {
            return Ok(Default::default());
        }
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "generation3d initializer close exceeds its admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(Default::default());
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| step.progress());
        }
        if self.active.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| step.progress());
        }
        if self.displaced.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant).map(|step| step.progress());
        }
        if self.rejected.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.rejected, &mut self.active, grant).map(|step| step.progress());
        }
        if self.candidate.is_some() {
            if self.candidate_disposer.is_none() {
                *self.candidate_disposer = Some(semio_framework_plugin::ArtifactDocumentStoreDisposer::new());
                return Ok(item);
            }
            let candidate = self.candidate.as_mut().expect("observed original candidate store");
            let disposer = self.candidate_disposer.as_mut().expect("observed original candidate disposer");
            let step = disposer.close_step(candidate, child).map_err(|error| ValueError::new(ValueRefusalKind::InvariantViolated, error.describe()))?;
            if disposer.terminal_is_empty(candidate) {
                *self.candidate_disposer = None;
                drop(self.candidate.take());
            }
            return Ok(step.progress().unwrap_or_default());
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory = self.snapshot_factory.as_ref().expect("initializer snapshot issuer outlives its runtime");
            let step = runtime.close_step(factory, child)?;
            let step = admit_retained_clone_close(child, step, runtime.terminal_is_empty(), "generation3d initializer runtime close")?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(step.progress());
        }
        if let Some(owners) = self.owners.as_mut() {
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(item);
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "generation3d initializer uninstalled owners close")?;
            return Ok(step.progress());
        }
        if let Some(copy) = self.copy.as_mut() {
            let step = copy.abandon(&mut self.active, grant)?;
            if copy.terminal_is_empty() && self.active.is_none() {
                drop(self.copy.take());
            }
            return Ok(step.progress());
        }
        if self.closer.is_some() {
            use semio_framework_job::InteractiveJob as _;
            let closer = self.closer.as_mut().expect("observed original envelope closer");
            return match closer.close_step(child) {
                semio_framework_job::InteractiveJobCloseStep::Pending { progress } => Ok(progress),
                semio_framework_job::InteractiveJobCloseStep::Complete { progress } => {
                    if semio_framework_job::InteractiveJob::terminal_is_empty(closer) {
                        drop(self.closer.take());
                    }
                    Ok(progress)
                }
                semio_framework_job::InteractiveJobCloseStep::Blocked => Ok(Default::default()),
                semio_framework_job::InteractiveJobCloseStep::Refused { kind, progress } => Err(ValueError::literal(kind, "generation3d initializer envelope close refused").with_retained_progress(progress)),
            };
        }
        if let Some(envelope) = self.envelope.take() {
            *self.closer = Some(semio_framework_plugin::bounded_document_store_initialization_job(envelope, crate::GENERATION_3D_SCHEMA, self.operation, self.generation, self.actor.clone()));
            return Ok(item);
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            let step = admit_retained_clone_close(child, step, factory.terminal_is_empty(), "generation3d initializer issuer close")?;
            if factory.terminal_is_empty() {
                *self.factory_close = None;
            }
            return Ok(step.progress());
        }
        if let Some(factory) = self.snapshot_factory.take() {
            let factory: std::sync::Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_is_empty(&self) -> bool {
        self.envelope.is_none()
            && self.copy.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.displaced.is_none()
            && self.rejected.is_none()
            && self.candidate_disposer.is_none()
            && self.closer.is_none()
            && self.snapshot_factory.is_none()
            && self.factory_close.is_none()
            && self.publication.terminal_is_empty()
    }

    fn close_is_complete(&self) -> bool {
        self.close_is_empty()
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_empty()
    }
}

const GENERATION3D_INITIALIZER_DEFAULT_FAULT: &[u8] = b"generation3d-store.initializer-fault";

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<Generation3dSnapshot, Generation3dMutation> for Generation3dStoreInitializationAuthority {
    fn retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        self.close_demands(body)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"generation3d-store.initializer-stale-aba");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::Cancelled) {
            self.phase = Generation3dStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(None);
        }
        match self.pump_owned(cx) {
            Ok(true) => {
                cx.consume_fuel(1);
                return Ok(None);
            }
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = Generation3dStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::RetireFault | Generation3dStoreInitializationPhase::Cancelled | Generation3dStoreInitializationPhase::Fault | Generation3dStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) => {}
                    Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => {
                        cx.consume_retained(progress)?;
                        cx.consume_fuel(1);
                        return Ok(None);
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = Generation3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
        }
        match self.phase {
            Generation3dStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), self.actor.clone()));
                self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation3dStoreInitializationPhase::ValidateEnvelope => {
                let valid = self.envelope.as_ref().is_some_and(|envelope| envelope.schema == crate::GENERATION_3D_SCHEMA && !envelope.id.is_empty() && envelope.id.len() <= GENERATION3D_OWNER_BYTES);
                if valid {
                    self.phase = Generation3dStoreInitializationPhase::ValidateEdit { index: 0 };
                } else {
                    self.fail(b"generation3d-store.initializer-envelope-invalid");
                }
            }
            Generation3dStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, usize::MAX) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit: 0, mutation: 0 },
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = Generation3dStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"generation3d-store.initializer-duplicate-edit"),
                }
            }
            Generation3dStoreInitializationPhase::CensusHistory { edit, mutation } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation3dStoreInitializationPhase::BindGenesis;
                    return Ok(None);
                };
                if entry.forwards.get(mutation).is_some() {
                    self.history_items = match self.history_items.checked_add(1) {
                        Some(value) if value <= GENERATION3D_MAXIMUM_DOMAIN_ITEMS => value,
                        _ => {
                            self.fail(b"generation3d-store.initializer-history-capacity");
                            return Ok(None);
                        }
                    };
                    self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit, mutation: mutation + 1 };
                } else {
                    self.phase = Generation3dStoreInitializationPhase::CensusHistory { edit: edit + 1, mutation: 0 };
                }
            }
            Generation3dStoreInitializationPhase::CopyInitial => {
                let source = &self.envelope.as_ref().expect("P3 initializer envelope").vcs.genesis.facts().snapshot();
                match self.copy.as_mut().expect("P3 copy retained").step(source) {
                    Ok(true) => self.phase = Generation3dStoreInitializationPhase::AdoptWorkspace,
                    Ok(false) => {}
                    Err(code) => self.fail(code.as_bytes()),
                }
            }
            Generation3dStoreInitializationPhase::AdoptWorkspace => {
                let initial = self.copy.as_mut().expect("P3 copy retained").take().expect("P3 copy handoff");
                drop(self.copy.take());
                let factory = std::sync::Arc::clone(self.snapshot_factory.as_ref().expect("P3 snapshot issuer retained"));
                match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, factory) {
                    Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                    Err(initial) => {
                        *self.rejected = Some(initial);
                        self.fail(b"initializer-owned-workspace-adoption");
                    }
                }
            }
            Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("P3 history retained");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = Generation3dStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("P3 runtime retained");
                match lane {
                    0 => match runtime.seed_mutation(protocol::MutationId(generation3d_copy_string(&entry.id).unwrap_or_default())) {
                        Ok(()) => {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                        Err(error) => {
                            self.fault = Some(error.into_bytes());
                            self.phase = Generation3dStoreInitializationPhase::RetireFault;
                        }
                    },
                    1 if index < entry.forwards.len() => {
                        let id = entry
                            .mutation_meta
                            .get(index)
                            .and_then(|meta| meta.mutation_id.as_ref()).map_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)), |id| protocol::MutationId(generation3d_copy_string(&id.0).unwrap_or_default()));
                        match runtime.seed_edit_operation(&entry.id, id) {
                            Ok(()) => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 },
                            Err(error) => {
                                self.fault = Some(error.into_bytes());
                                self.phase = Generation3dStoreInitializationPhase::RetireFault;
                            }
                        }
                    }
                    1 => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = Generation3dStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
            }
            Generation3dStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("P3 envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("P3 runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = Generation3dStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = Generation3dStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = Generation3dStoreInitializationPhase::RetireFault;
                    }
                }
            }
            Generation3dStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self
                        .envelope
                        .as_ref()
                        .and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.as_ref()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| &checkpoint.id)))
                        .and_then(|id| generation3d_copy_string(id).ok());
                    self.runtime.as_mut().expect("P3 runtime retained").set_current_checkpoint_id(checkpoint);
                    self.phase = Generation3dStoreInitializationPhase::FindRedo { position: 0 };
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                    }
                    Some(_) => self.fail(b"generation3d-store.initializer-applied-missing"),
                    None => self.fail(b"generation3d-store.initializer-applied-missing"),
                }
            }
            Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    match Generation3dSnapshotCopyCursor::new(self.envelope.as_ref().expect("retained genesis").vcs.genesis.facts().snapshot()) { Ok(copy) => *self.copy = Some(copy), Err(code) => { self.fail(code.as_bytes()); return Ok(None); } }
                    self.phase = Generation3dStoreInitializationPhase::CopyInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let operation = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).and_then(|entry| entry.forwards.get(mutation));
                let Some(operation) = operation else {
                    self.phase = Generation3dStoreInitializationPhase::CommitApplied { position, edit };
                    return Ok(None);
                };
                let envelope = self.envelope.as_ref().expect("P3 envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("P3 applied edit remains retained");
                let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("P3 applied forward remains retained");
                if effective.operation().is_none() {
                    drop(effective);
                    self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    return Ok(None);
                }
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("P3 runtime current retained");
                let applied = effective.operation().map(|operation| generation3d_apply_initialization_mutation(current, operation));
                drop(effective);
                match applied {
                    Some(Ok(displaced)) => {
                        *self.displaced = displaced;
                        self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    }
                    None => self.phase = Generation3dStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 },
                    Some(Err(code)) => self.fail(code.as_bytes()),
                }
            }
            Generation3dStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P3 applied edit retained");
                let runtime = self.runtime.as_mut().expect("P3 runtime retained");
                match runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => {
                        runtime.observe_sequence(entry.sequence_number);

                        self.phase = Generation3dStoreInitializationPhase::FindApplied { position: position + 1 };
                    }
                    Err(_) => self.fail(b"generation3d-store.initializer-applied-capacity"),
                }
            }
            Generation3dStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = Generation3dStoreInitializationPhase::BuildOwners;
                    cx.consume_fuel(1);
                    return Ok(None);
                };
                let scan = self.edit_index.position(id).unwrap_or(usize::MAX);
                match self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(scan)) {
                    Some(edit) if edit.id == id => {
                        self.phase = Generation3dStoreInitializationPhase::CommitRedo { position, edit: scan };
                    }
                    Some(_) => self.fail(b"generation3d-store.initializer-redo-missing"),
                    None => self.fail(b"generation3d-store.initializer-redo-missing"),
                }
            }
            Generation3dStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("P3 redo edit retained");
                match self.runtime.as_mut().expect("P3 runtime retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    Ok(()) => self.phase = Generation3dStoreInitializationPhase::FindRedo { position: position + 1 },
                    Err(_) => self.fail(b"generation3d-store.initializer-redo-capacity"),
                }
            }
            Generation3dStoreInitializationPhase::BuildOwners => {
                if self.owners.is_none() {
                    match store::bounded_artifact_store_owners::<Generation3dSnapshot, Generation3dMutation>(cx.retained_grant()) {
                        Ok((owners, progress)) => {
                            *self.owners = Some(owners);
                            cx.consume_retained(progress)?;
                        }
                        Err(refused) => {
                            *self.owners = refused.owners;
                            if refused.error.retained_progress() != Default::default() {
                                cx.consume_retained(refused.error.retained_progress())?;
                            }
                        }
                    }
                } else if let Some(owners) = self.owners.as_mut().filter(|owners| !owners.constructor_is_complete()) {
                    match owners.admit_constructor(cx.retained_grant()) {
                        Ok(progress) => cx.consume_retained(progress)?,
                        Err((error, progress)) => {
                            cx.consume_retained(progress)?;
                            return Err(error);
                        }
                    }
                } else {
                    self.phase = Generation3dStoreInitializationPhase::BuildCandidate;
                }
                cx.consume_fuel(1);
                return Ok(None);
            }
            Generation3dStoreInitializationPhase::BuildCandidate => {
                let authority = generation3d_validate_publication_authority(self.operation, self.generation);
                let fresh =
                    cx.operation() == self.operation && cx.generation() == self.generation && authority == Ok((self.base_revision, self.parent_revision)) && self.base_revision == self.parent_revision && self.parent_revision == self.generation.0;
                let Some(candidate_generation) = self.parent_revision.checked_add(1) else {
                    self.fail(b"generation3d-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                if !fresh {
                    self.fail(b"generation3d-store.initializer-parent-stale-aba");
                    return Ok(None);
                }
                let envelope = self.envelope.take().expect("P3 envelope retained until atomic publication");
                let runtime = self.runtime.take().expect("P3 runtime retained until atomic publication");
                let owners = self.owners.take().expect("P3 owners retained until atomic publication");
                *self.candidate = Some(store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners));
                self.phase = Generation3dStoreInitializationPhase::Complete;
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            Generation3dStoreInitializationPhase::RetireCancelled | Generation3dStoreInitializationPhase::RetireFault => match self.close_turn(cx.retained_grant()) {
                Ok(progress) => {
                    cx.consume_retained(progress)?;
                    if !self.close_is_complete() {
                        return Ok(None);
                    }
                    generation3d_release_app_publication_authority(self.operation);
                    self.terminal_handoff = true;
                    if self.phase == Generation3dStoreInitializationPhase::RetireCancelled {
                        self.phase = Generation3dStoreInitializationPhase::Cancelled;
                        return JobOutcomeBorrow::admit_cancelled(cx);
                    }
                    self.phase = Generation3dStoreInitializationPhase::Fault;
                    return self.publication.advance_from_source(semio_framework_job::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION3D_INITIALIZER_DEFAULT_FAULT), cx);
                }
                Err(_) => self.fail(b"generation3d-store.initializer-close"),
            },
            Generation3dStoreInitializationPhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None),
            Generation3dStoreInitializationPhase::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            Generation3dStoreInitializationPhase::Fault => return self.publication.advance_from_source(semio_framework_job::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(GENERATION3D_INITIALIZER_DEFAULT_FAULT), cx),
        }
        cx.consume_fuel(1);
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }
    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<Generation3dSnapshot, Generation3dMutation>> {
        if self.phase != Generation3dStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, Generation3dStoreInitializationPhase::Cancelled | Generation3dStoreInitializationPhase::Fault) {
            self.phase = Generation3dStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        self.begin_close();
        let progress = self.close_turn(grant)?;
        if self.close_is_complete() {
            generation3d_release_app_publication_authority(self.operation);
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(progress));
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for Generation3dStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty_inner()), "Generation3d initializer reached Drop before candidate handoff or terminal-empty close");
    }
}

pub fn generation3d_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<Generation3dSnapshot, Generation3dMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(Generation3dStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}
//#endregion 🔖️RetainedStoreInitialization

#[cfg(test)]
pub fn generation3d_all_retained_mutation_fixtures_for_test() -> Vec<Generation3dMutation> {
    let synapse = semio_framework_artifact_flow_flow::SynapseSpec { id: "retained-synapse".into(), from: "retained-a".into(), to: "retained-b".into(), from_port: "out".into(), to_port: "in".into() };
    let mut values: semio_framework_artifact_playbook_playbook::PlaybookValues = semio_framework_artifact_playbook_playbook::PlaybookValues::new();
    values.insert(
        "nested".into(),
        semio_framework_value::DslValue::object([("array".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Bool(true), semio_framework_value::DslValue::Null, semio_framework_value::DslValue::float(3.5)])), ("text".to_string(), semio_framework_value::DslValue::String("retained".to_string()))]),
    );
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("retained".into()))),
        ),
    );
    let mut mutations = vec![
        Generation3dMutation::CreateWidget(CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "retained-a".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(UpdateWidget { widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "retained-a".into(), name: "Updated".into(), tree: Default::default(), flow: Default::default() } }),
        Generation3dMutation::DeleteWidget(DeleteWidget { id: "retained-a".into() }),
        Generation3dMutation::ConnectSynapse(ConnectSynapse { index: 0, synapse: generation3d_copy_synapse(&synapse).expect("P3 synapse fixture copy") }),
        Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: semio_framework_artifact_flow_flow::SynapseSpec { to_port: "alternate".into(), ..synapse } }),
        Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: "retained-synapse".into() }),
        Generation3dMutation::MoveWidget(MoveWidget { id: "retained-a".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 11.0, y: -7.0 } }),
        Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "retained-a".into() }),
        Generation3dMutation::UpdateCamera(UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 3.0, y: 4.0, zoom: 1.5 } }),
        Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.host_snapshot.retained".into() }),
        Generation3dMutation::CreateGeneration(CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "retained-generation".into(), name: "Retained Generation".into(), values }, index: Some(0) }),
        Generation3dMutation::DeleteGeneration(DeleteGeneration { id: "retained-generation".into() }),
        Generation3dMutation::RenameGeneration(RenameGeneration { id: "retained-generation".into(), new_name: "Renamed Generation".into() }),
        Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue {
            id: "retained-generation".into(),
            question_id: "deep-answer".into(),
            new_value: semio_framework_value::DslValue::object([("object".to_string(), semio_framework_value::DslValue::object([("array".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(1.0), semio_framework_value::DslValue::Bool(false), semio_framework_value::DslValue::String("value".to_string())]))]))]),
        }),
        Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: Some("retained-generation".into()) }),
        Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: None }),
        Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: Some("Retained preview \u{1F600}".into()) }),
        Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: None }),
    ];
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧫️fixtures/🧬️semantic-wire/🔣️.json")).expect("semantic wire corpus");
    for case in corpus["cases"].as_array().expect("semantic wire cases") {
        mutations.push(<Generation3dMutation as semio_framework_value::FromValue>::from_value(case["mutation"].clone().into()).expect("authored semantic mutation"));
    }
    mutations
}

#[cfg(test)]
pub fn generation3d_apply_retained_mutations_for_test(snapshot: &mut Generation3dSnapshot, mutations: &[Generation3dMutation]) {
    for mutation in mutations {
        if let Some(displaced) = generation3d_apply_initialization_mutation(snapshot, mutation).expect("P3 production fixture retained replay") {
            generation3d_retire_displaced_cold(displaced);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-authority-laws/🦀️.rs"]
mod retained_authority_laws;

#[path = "📐️geometry-service/🦀️.rs"]
pub mod geometry_service;
