//! 📦️ Process3d mutations use one canonical Pack document in archive, command and envelope ingress.

use crate::Process3dMutation;
use dsl::os_dsl::original_receiving::{OpBinaryReceiving, OriginalPackValueReceiving};
use semio_framework_value::{FromValue, RetainedCloneGrant, RetainedCloneProgress, RetainedWorkOutcome, ToValue, ValueError, ValueRefusalKind};

/// 📡️ The authored binary protocol for this facet.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub const PROCESS3D_OWNER_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
pub const PROCESS3D_MAXIMUM_DOMAIN_ITEMS: usize = 8_192;

/// 📤️ Encodes the complete typed mutation value to its canonical Pack document.
pub fn encode_op(operation: &Process3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let bytes = store::pack_rt::encode_pack_value(&operation.to_value());
    check_extent(bytes.len())?;
    Ok(bytes)
}

/// 📥️ Decodes the same canonical mutation value that the original receiver admits.
pub fn decode_op(bytes: &[u8]) -> Result<Process3dMutation, protocol::ProtocolError> {
    check_extent(bytes.len())?;
    let value = store::pack_rt::decode_pack_value(bytes).map_err(protocol::ProtocolError::from)?;
    Process3dMutation::from_value(store::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| protocol::ProtocolError::Malformed { what: "process3d mutation", offset: 0, detail: error.to_string() })
}

fn check_extent(bytes: usize) -> Result<(), protocol::ProtocolError> {
    if bytes > PROCESS3D_OWNER_BYTES {
        return Err(protocol::ProtocolError::LimitExceeded("process3d mutation exceeds fixed binary owner"));
    }
    Ok(())
}

impl protocol::OpBinary for Process3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        decode_op(bytes)
    }
}

impl OpBinaryReceiving for Process3dMutation {
    type DomainFailure = ValueError;
    type Receiving = OriginalPackValueReceiving<Self>;

    fn admit_receiving(source: &[u8], range: (usize, usize), grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Self::Receiving, RetainedCloneProgress)>> {
        if range.1.saturating_sub(range.0) > PROCESS3D_OWNER_BYTES {
            return RetainedWorkOutcome { result: Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "process3d mutation exceeds fixed binary owner")), work: 0 };
        }
        Self::Receiving::admit(source, range, grant)
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn process3d_all_retained_mutation_fixtures_for_test() -> Vec<Process3dMutation> {
    use semio_s_artifact_process_process3d_model::snapshot::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, StepOrigin, StockQuantity, WorkingSolid, WorkshopMachine};
    use semio_s_artifact_process_process3d_mut_machines::{
        change_machine_icon::ChangeMachineIcon, create_machine::CreateMachine, delete_machine::DeleteMachine, rename_machine::RenameMachine, replace_machine_capabilities::ReplaceMachineCapabilities,
    };
    use semio_s_artifact_process_process3d_mut_steps::{
        change_step_enabled::ChangeStepEnabled, change_step_origin::ChangeStepOrigin, create_step::CreateStep, delete_step::DeleteStep, rename_step::RenameStep, reorder_steps::ReorderSteps, replace_step_measure::ReplaceStepMeasure,
    };
    use semio_s_artifact_process_process3d_mut_stock::{change_stock_label::ChangeStockLabel, move_stock::MoveStock, replace_stock_solid::ReplaceStockSolid};

    let pose = Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.5 };
    let capability = Capability {
        id: "deep-capability".into(),
        label: "Deep Capability".into(),
        icon_id: "deep-tool".into(),
        recipe: MeasureRecipe::BoxAttach { width: "width".into(), depth: "depth".into(), height: "height".into() },
        parameters: vec![
            CapabilityParameter { id: "width".into(), label: "Width".into(), value: 4.0 },
            CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 5.0 },
            CapabilityParameter { id: "height".into(), label: "Height".into(), value: 6.0 },
        ],
        rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "width".into(), margin: 0.25 }, CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "height".into(), margin: 0.5 }],
    };
    let step = ProcessStep {
        id: "deep-step".into(),
        label: "Deep Step".into(),
        enabled: true,
        origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: capability.id.clone() }),
        measure: ProcessMeasure::Cut { tool: WorkingSolid::ImportedMesh { mesh_url: "fixtures/deep-tool.glb".into() }, pose: pose.clone() },
    };
    let machine = WorkshopMachine { id: "transient-machine".into(), label: "Transient Machine".into(), icon_id: "saw".into(), catalog_id: Some("deep-catalog".into()), capabilities: vec![capability.clone()] };
    let child = semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot().stock_solid;
    vec![
        Process3dMutation::CreateStep(CreateStep { index: 0, step }),
        Process3dMutation::DeleteStep(DeleteStep { id: "obsolete-step".into() }),
        Process3dMutation::RenameStep(RenameStep { id: "deep-step".into(), new_label: "Deep Step Renamed".into() }),
        Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "deep-step".into(), new_enabled: false }),
        Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "deep-step".into(), new_origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: capability.id.clone() }) }),
        Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure {
            id: "deep-step".into(),
            new_measure: ProcessMeasure::Attach {
                component: semio_s_artifact_process_process3d_model::snapshot::pack_imported_brep(&semio_s_artifact_process_process3d_model::snapshot::brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 0.5 }))
                    .expect("small owned B-Rep fixture"),
                pose: pose.clone(),
            },
        }),
        Process3dMutation::ReorderSteps(ReorderSteps { id: "deep-step".into(), to_index: 0 }),
        Process3dMutation::CreateMachine(CreateMachine { index: 1, machine }),
        Process3dMutation::DeleteMachine(DeleteMachine { id: "transient-machine".into() }),
        Process3dMutation::RenameMachine(RenameMachine { id: "machine".into(), new_label: "Renamed Machine".into() }),
        Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "machine".into(), new_icon_id: "drill".into() }),
        Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "machine".into(), new_capabilities: vec![capability] }),
        Process3dMutation::MoveStock(MoveStock { new_pose: pose }),
        Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }),
        Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: child }),
        Process3dMutation::ResizeStock(semio_s_artifact_process_process3d_mut_stock::resize_stock::ResizeStock { field: "width".into(), size: 2.0 }),
    ]
}
