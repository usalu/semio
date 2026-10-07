//! ⚖️ Process3d artifact — binary operation wire codec surface + laws (constitutional: spr, renamed
//! from the old `📡️protocol` module — no `📡️protocol` path segment may survive under `✏️s/🔌️plugins/`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Process3dMutation;
use store::{ArtifactEnvelopeMutationFieldAuthority as _, ArtifactEnvelopeSnapshotFieldAuthority as _};





/// 📦️ Encodes a `Process3dMutation` to its binary command form.
pub fn encode_op(operation: &Process3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    use Process3dMutation::*;
    let mut out = Vec::with_capacity(PROCESS3D_OWNER_BYTES);
    out.push(PROCESS3D_MUTATION_BINARY_FORMAT);
    match operation {
        CreateStep(value) => {
            out.push(0);
            store::pack_rt::write_varint_u64(&mut out, value.index as u64);
            write_str_lp(&mut out, &value.step.id);
            write_str_lp(&mut out, &value.step.label);
            out.push(u8::from(value.step.enabled));
            out.push(u8::from(value.step.origin.is_some()));
            if let Some(origin) = &value.step.origin {
                write_str_lp(&mut out, &origin.machine_id);
                write_str_lp(&mut out, &origin.capability_id);
            }
            write_measure(&mut out, &value.step.measure);
        }
        DeleteStep(value) => {
            out.push(1);
            write_str_lp(&mut out, &value.id);
        }
        RenameStep(value) => {
            out.push(2);
            write_str_lp(&mut out, &value.id);
            write_str_lp(&mut out, &value.new_label);
        }
        ChangeStepEnabled(value) => {
            out.push(3);
            write_str_lp(&mut out, &value.id);
            out.push(u8::from(value.new_enabled));
        }
        ChangeStepOrigin(value) => {
            out.push(4);
            write_str_lp(&mut out, &value.id);
            out.push(u8::from(value.new_origin.is_some()));
            if let Some(origin) = &value.new_origin {
                write_str_lp(&mut out, &origin.machine_id);
                write_str_lp(&mut out, &origin.capability_id);
            }
        }
        ReplaceStepMeasure(value) => {
            out.push(5);
            write_str_lp(&mut out, &value.id);
            write_measure(&mut out, &value.new_measure);
        }
        ReorderSteps(value) => {
            out.push(6);
            write_str_lp(&mut out, &value.id);
            store::pack_rt::write_varint_u64(&mut out, value.to_index as u64);
        }
        CreateMachine(value) => {
            out.push(7);
            store::pack_rt::write_varint_u64(&mut out, value.index as u64);
            write_machine(&mut out, &value.machine);
        }
        DeleteMachine(value) => {
            out.push(8);
            write_str_lp(&mut out, &value.id);
        }
        RenameMachine(value) => {
            out.push(9);
            write_str_lp(&mut out, &value.id);
            write_str_lp(&mut out, &value.new_label);
        }
        ChangeMachineIcon(value) => {
            out.push(10);
            write_str_lp(&mut out, &value.id);
            write_str_lp(&mut out, &value.new_icon_id);
        }
        ReplaceMachineCapabilities(value) => {
            out.push(11);
            write_str_lp(&mut out, &value.id);
            store::pack_rt::write_varint_u64(&mut out, value.new_capabilities.len() as u64);
            for capability in &value.new_capabilities {
                write_capability(&mut out, capability);
            }
        }
        MoveStock(value) => {
            out.push(12);
            write_pose(&mut out, &value.new_pose);
        }
        ChangeStockLabel(value) => {
            out.push(13);
            write_str_lp(&mut out, &value.new_label);
        }
        ReplaceStockSolid(value) => {
            out.push(14);
            write_child(&mut out, &value.new_solid);
        }
    }
    if out.len() > PROCESS3D_OWNER_BYTES {
        return Err(protocol::ProtocolError::LimitExceeded("process3d mutation exceeds fixed binary owner"));
    }
    Ok(out)
}

/// 📖️ Decodes a `Process3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Process3dMutation, protocol::ProtocolError> {
    use crate::mutations::{
        change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps, replace_machine_capabilities,
        replace_step_measure, replace_stock_solid,
    };
    if bytes.len() > PROCESS3D_OWNER_BYTES {
        return Err(protocol::ProtocolError::LimitExceeded("process3d mutation exceeds fixed binary owner"));
    }
    let mut reader = store::ByteReader::new(bytes);
    if reader.read_u8().map_err(protocol::ProtocolError::from)? != PROCESS3D_MUTATION_BINARY_FORMAT {
        return Err(process3d_protocol_error("unsupported process3d mutation binary format".into()));
    }
    let tag = reader.read_u8().map_err(protocol::ProtocolError::from)?;
    let mutation = match tag {
        0 => {
            let index = reader.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize;
            let id = read_str_lp(&mut reader).map_err(process3d_protocol_error)?;
            let label = read_str_lp(&mut reader).map_err(process3d_protocol_error)?;
            let enabled = reader.read_u8().map_err(protocol::ProtocolError::from)? != 0;
            let origin = match reader.read_u8().map_err(protocol::ProtocolError::from)? {
                0 => None,
                1 => Some(StepOrigin { machine_id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, capability_id: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
                _ => return Err(process3d_protocol_error("invalid step origin tag".into())),
            };
            Process3dMutation::CreateStep(create_step::CreateStep { index, step: ProcessStep { id, label, enabled, origin, measure: read_measure(&mut reader).map_err(process3d_protocol_error)? } })
        }
        1 => Process3dMutation::DeleteStep(delete_step::DeleteStep { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        2 => Process3dMutation::RenameStep(rename_step::RenameStep { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, new_label: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        3 => Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, new_enabled: reader.read_u8().map_err(protocol::ProtocolError::from)? != 0 }),
        4 => {
            let id = read_str_lp(&mut reader).map_err(process3d_protocol_error)?;
            let new_origin = match reader.read_u8().map_err(protocol::ProtocolError::from)? {
                0 => None,
                1 => Some(StepOrigin { machine_id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, capability_id: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
                _ => return Err(process3d_protocol_error("invalid step origin tag".into())),
            };
            Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id, new_origin })
        }
        5 => Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, new_measure: read_measure(&mut reader).map_err(process3d_protocol_error)? }),
        6 => Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, to_index: reader.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize }),
        7 => Process3dMutation::CreateMachine(create_machine::CreateMachine { index: reader.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize, machine: read_machine(&mut reader).map_err(process3d_protocol_error)? }),
        8 => Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        9 => Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, new_label: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        10 => Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: read_str_lp(&mut reader).map_err(process3d_protocol_error)?, new_icon_id: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        11 => {
            let id = read_str_lp(&mut reader).map_err(process3d_protocol_error)?;
            let count = reader.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize;
            if count > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                return Err(protocol::ProtocolError::LimitExceeded("process3d mutation capability count exceeds fixed catalog"));
            }
            let mut new_capabilities = Vec::with_capacity(count);
            for _ in 0..count {
                new_capabilities.push(read_capability(&mut reader).map_err(process3d_protocol_error)?);
            }
            Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities })
        }
        12 => Process3dMutation::MoveStock(move_stock::MoveStock { new_pose: read_pose(&mut reader).map_err(process3d_protocol_error)? }),
        13 => Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: read_str_lp(&mut reader).map_err(process3d_protocol_error)? }),
        14 => Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid { new_solid: read_child(&mut reader).map_err(process3d_protocol_error)? }),
        _ => return Err(process3d_protocol_error("unknown process3d mutation tag".into())),
    };
    if reader.remaining() != 0 {
        return Err(process3d_protocol_error("process3d mutation has trailing bytes".into()));
    }
    Ok(mutation)
}

//#region 🔖️MutationWirePrimitives
/// 🧵️ Field codecs and one-byte-grant cursors of this facet's handcrafted mutation wire.
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    if len > 4096 {
        return Err("process3d pack string exceeds fixed capacity".into());
    }
    let source = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(len).map_err(|_| "process3d pack string admission failed".to_string())?;
    bytes.extend_from_slice(source);
    Ok(bytes)
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &semio_framework_artifact_reference::ArtifactRef) {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<semio_framework_artifact_reference::ArtifactRef, String> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    semio_framework_artifact_reference::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
fn write_pose(out: &mut Vec<u8>, pose: &Pose) {
    for value in pose.position.iter().chain(pose.axis.iter()).chain(std::iter::once(&pose.angle)) {
        out.extend_from_slice(&value.to_le_bytes());
    }
}

fn read_pose(reader: &mut store::ByteReader<'_>) -> Result<Pose, String> {
    Ok(Pose {
        position: [reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?],
        axis: [reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?],
        angle: reader.read_f64_le().map_err(|e| e.to_string())?,
    })
}

fn write_solid(out: &mut Vec<u8>, solid: &WorkingSolid) {
    match solid {
        WorkingSolid::Box { width, depth, height } => {
            out.push(0);
            for value in [width, depth, height] {
                out.extend_from_slice(&value.to_le_bytes());
            }
        }
        WorkingSolid::Cylinder { radius, height } => {
            out.push(1);
            out.extend_from_slice(&radius.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
        }
        WorkingSolid::Sphere { radius } => {
            out.push(2);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        WorkingSolid::ImportedMesh { mesh_url } => {
            out.push(3);
            write_str_lp(out, mesh_url);
        }
        WorkingSolid::ImportedSolid { solid_handle } => {
            out.push(4);
            write_str_lp(out, solid_handle);
        }
        WorkingSolid::Reference { reference_id } => {
            out.push(5);
            write_str_lp(out, reference_id);
        }
    }
}

fn read_solid(reader: &mut store::ByteReader<'_>) -> Result<WorkingSolid, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(WorkingSolid::Box { width: reader.read_f64_le().map_err(|e| e.to_string())?, depth: reader.read_f64_le().map_err(|e| e.to_string())?, height: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(WorkingSolid::Cylinder { radius: reader.read_f64_le().map_err(|e| e.to_string())?, height: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(WorkingSolid::Sphere { radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(WorkingSolid::ImportedMesh { mesh_url: read_str_lp(reader)? }),
        4 => Ok(WorkingSolid::ImportedSolid { solid_handle: read_str_lp(reader)? }),
        5 => Ok(WorkingSolid::Reference { reference_id: read_str_lp(reader)? }),
        _ => Err("process3d pack solid tag is invalid".into()),
    }
}

fn write_measure(out: &mut Vec<u8>, measure: &ProcessMeasure) {
    match measure {
        ProcessMeasure::Cut { tool, pose } => {
            out.push(0);
            write_solid(out, tool);
            write_pose(out, pose);
        }
        ProcessMeasure::Drill { radius, depth, pose } => {
            out.push(1);
            out.extend_from_slice(&radius.to_le_bytes());
            out.extend_from_slice(&depth.to_le_bytes());
            write_pose(out, pose);
        }
        ProcessMeasure::Attach { component, pose } => {
            out.push(2);
            write_solid(out, component);
            write_pose(out, pose);
        }
    }
}

fn read_measure(reader: &mut store::ByteReader<'_>) -> Result<ProcessMeasure, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(ProcessMeasure::Cut { tool: read_solid(reader)?, pose: read_pose(reader)? }),
        1 => Ok(ProcessMeasure::Drill { radius: reader.read_f64_le().map_err(|e| e.to_string())?, depth: reader.read_f64_le().map_err(|e| e.to_string())?, pose: read_pose(reader)? }),
        2 => Ok(ProcessMeasure::Attach { component: read_solid(reader)?, pose: read_pose(reader)? }),
        _ => Err("process3d pack measure tag is invalid".into()),
    }
}

fn write_recipe(out: &mut Vec<u8>, recipe: &MeasureRecipe) {
    let (tag, fields): (u8, [&str; 3]) = match recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => (0, [diameter, kerf, ""]),
        MeasureRecipe::BladeCut { kerf, length, depth } => (1, [kerf, length, depth]),
        MeasureRecipe::PocketCut { diameter, depth } => (2, [diameter, depth, ""]),
        MeasureRecipe::BoreDrill { radius, depth } => (3, [radius, depth, ""]),
        MeasureRecipe::CylinderAttach { radius, length } => (4, [radius, length, ""]),
        MeasureRecipe::BoxAttach { width, depth, height } => (5, [width, depth, height]),
    };
    out.push(tag);
    for field in fields {
        write_str_lp(out, field);
    }
}

fn read_recipe(reader: &mut store::ByteReader<'_>) -> Result<MeasureRecipe, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let first = read_str_lp(reader)?;
    let second = read_str_lp(reader)?;
    let third = read_str_lp(reader)?;
    match tag {
        0 => Ok(MeasureRecipe::DiscCut { diameter: first, kerf: second }),
        1 => Ok(MeasureRecipe::BladeCut { kerf: first, length: second, depth: third }),
        2 => Ok(MeasureRecipe::PocketCut { diameter: first, depth: second }),
        3 => Ok(MeasureRecipe::BoreDrill { radius: first, depth: second }),
        4 => Ok(MeasureRecipe::CylinderAttach { radius: first, length: second }),
        5 => Ok(MeasureRecipe::BoxAttach { width: first, depth: second, height: third }),
        _ => Err("process3d pack recipe tag is invalid".into()),
    }
}

fn write_count(out: &mut Vec<u8>, count: usize) {
    store::pack_rt::write_varint_u64(out, count as u64);
}

fn read_count(reader: &mut store::ByteReader<'_>) -> Result<usize, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    if count > 8192 {
        return Err("process3d pack item count exceeds fixed capacity".into());
    }
    Ok(count)
}

fn write_capability(out: &mut Vec<u8>, capability: &Capability) {
    write_str_lp(out, &capability.id);
    write_str_lp(out, &capability.label);
    write_str_lp(out, &capability.icon_id);
    write_recipe(out, &capability.recipe);
    write_count(out, capability.parameters.len());
    for parameter in &capability.parameters {
        write_str_lp(out, &parameter.id);
        write_str_lp(out, &parameter.label);
        out.extend_from_slice(&parameter.value.to_le_bytes());
    }
    write_count(out, capability.rules.len());
    for rule in &capability.rules {
        let (tag, quantity, parameter, margin) = match rule {
            CapabilityRule::Min { quantity, parameter, margin } => (0, quantity, parameter, margin),
            CapabilityRule::Max { quantity, parameter, margin } => (1, quantity, parameter, margin),
        };
        out.push(tag);
        out.push(match quantity {
            StockQuantity::Width => 0,
            StockQuantity::Depth => 1,
            StockQuantity::Height => 2,
            StockQuantity::MaxDimension => 3,
            StockQuantity::MinDimension => 4,
        });
        write_str_lp(out, parameter);
        out.extend_from_slice(&margin.to_le_bytes());
    }
}

fn read_capability(reader: &mut store::ByteReader<'_>) -> Result<Capability, String> {
    let id = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let icon_id = read_str_lp(reader)?;
    let recipe = read_recipe(reader)?;
    let parameter_count = read_count(reader)?;
    let mut parameters = Vec::with_capacity(parameter_count);
    for _ in 0..parameter_count {
        parameters.push(CapabilityParameter { id: read_str_lp(reader)?, label: read_str_lp(reader)?, value: reader.read_f64_le().map_err(|e| e.to_string())? });
    }
    let rule_count = read_count(reader)?;
    let mut rules = Vec::with_capacity(rule_count);
    for _ in 0..rule_count {
        let tag = reader.read_u8().map_err(|e| e.to_string())?;
        let quantity = match reader.read_u8().map_err(|e| e.to_string())? {
            0 => StockQuantity::Width,
            1 => StockQuantity::Depth,
            2 => StockQuantity::Height,
            3 => StockQuantity::MaxDimension,
            4 => StockQuantity::MinDimension,
            _ => return Err("process3d pack stock quantity tag is invalid".into()),
        };
        let parameter = read_str_lp(reader)?;
        let margin = reader.read_f64_le().map_err(|e| e.to_string())?;
        rules.push(match tag {
            0 => CapabilityRule::Min { quantity, parameter, margin },
            1 => CapabilityRule::Max { quantity, parameter, margin },
            _ => return Err("process3d pack capability rule tag is invalid".into()),
        });
    }
    Ok(Capability { id, label, icon_id, recipe, parameters, rules })
}

fn write_machine(out: &mut Vec<u8>, machine: &WorkshopMachine) {
    write_str_lp(out, &machine.id);
    write_str_lp(out, &machine.label);
    write_str_lp(out, &machine.icon_id);
    out.push(u8::from(machine.catalog_id.is_some()));
    if let Some(catalog_id) = &machine.catalog_id {
        write_str_lp(out, catalog_id);
    }
    write_count(out, machine.capabilities.len());
    for capability in &machine.capabilities {
        write_capability(out, capability);
    }
}

fn read_machine(reader: &mut store::ByteReader<'_>) -> Result<WorkshopMachine, String> {
    let id = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let icon_id = read_str_lp(reader)?;
    let catalog_id = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(read_str_lp(reader)?),
        _ => return Err("process3d pack catalog tag is invalid".into()),
    };
    let count = read_count(reader)?;
    let mut capabilities = Vec::with_capacity(count);
    for _ in 0..count {
        capabilities.push(read_capability(reader)?);
    }
    Ok(WorkshopMachine { id, label, icon_id, catalog_id, capabilities })
}




































//#endregion 🔖️MutationWirePrimitives

//#region 🔖️RetainedEnvelopeOwnership
use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, Process3dSnapshot, ProcessMeasure, ProcessStep, StepOrigin, Stock, StockQuantity, WorkingSolid, WorkshopMachine};












































































































//#endregion 🔖️RetainedEnvelopeOwnership

//#region 🔖️OwnedEnvelopeCatalog


















































//#endregion 🔖️OwnedEnvelopeCatalog

//#region 🔖️RetainedConstruction



































//#endregion 🔖️RetainedConstruction

//#region 🔖️RetainedStoreInitialization

















//#endregion 🔖️RetainedStoreInitialization

#[cfg(test)]
pub fn process3d_all_retained_mutation_fixtures_for_test() -> Vec<Process3dMutation> {
    use crate::mutations::{
        change_machine_icon::ChangeMachineIcon, change_step_enabled::ChangeStepEnabled, change_step_origin::ChangeStepOrigin, change_stock_label::ChangeStockLabel, create_machine::CreateMachine, create_step::CreateStep,
        delete_machine::DeleteMachine, delete_step::DeleteStep, move_stock::MoveStock, rename_machine::RenameMachine, rename_step::RenameStep, reorder_steps::ReorderSteps, replace_machine_capabilities::ReplaceMachineCapabilities,
        replace_step_measure::ReplaceStepMeasure, replace_stock_solid::ReplaceStockSolid,
    };

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
    let child = crate::empty_process3d_snapshot().stock_solid;
    vec![
        Process3dMutation::CreateStep(CreateStep { index: 0, step }),
        Process3dMutation::DeleteStep(DeleteStep { id: "obsolete-step".into() }),
        Process3dMutation::RenameStep(RenameStep { id: "deep-step".into(), new_label: "Deep Step Renamed".into() }),
        Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "deep-step".into(), new_enabled: false }),
        Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "deep-step".into(), new_origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: capability.id.clone() }) }),
        Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "deep-step".into(), new_measure: ProcessMeasure::Attach { component: WorkingSolid::ImportedSolid { solid_handle: "deep-solid-handle".into() }, pose: pose.clone() } }),
        Process3dMutation::ReorderSteps(ReorderSteps { id: "deep-step".into(), to_index: 0 }),
        Process3dMutation::CreateMachine(CreateMachine { index: 1, machine }),
        Process3dMutation::DeleteMachine(DeleteMachine { id: "transient-machine".into() }),
        Process3dMutation::RenameMachine(RenameMachine { id: "machine".into(), new_label: "Renamed Machine".into() }),
        Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "machine".into(), new_icon_id: "drill".into() }),
        Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "machine".into(), new_capabilities: vec![capability] }),
        Process3dMutation::MoveStock(MoveStock { new_pose: pose }),
        Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }),
        Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: child }),
    ]
}

//#region 🧪️RetainedLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️retained-laws/🦀️.rs"]
mod retained_laws;
//#endregion 🧪️RetainedLaws

mod native_codec {
use super::*;
use crate::schema::mutations::Process3dMutation;
use crate::schema::mutations::{
    change_machine_icon, change_step_enabled, change_step_origin, change_stock_label, create_machine, create_step, delete_machine, delete_step, move_stock, rename_machine, rename_step, reorder_steps, replace_machine_capabilities,
    replace_step_measure, replace_stock_solid,
};
use crate::{Capability, Pose, StepOrigin, WorkshopMachine};
use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{Process3dMutationDsl};

impl protocol::OpBinary for Process3dMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}

impl protocol::OpBinary for Process3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        crate::standards::v1::subsets::any::io::binary::mutations::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        crate::standards::v1::subsets::any::io::binary::mutations::decode_op(bytes)
    }
}
}
