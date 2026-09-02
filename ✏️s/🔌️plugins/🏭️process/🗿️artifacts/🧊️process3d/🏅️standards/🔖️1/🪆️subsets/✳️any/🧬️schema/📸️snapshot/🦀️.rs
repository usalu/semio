//! 🧬️ Process3d snapshot schema — artifact-lane fields only.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `stock`/`steps` are no longer
//! inline (`Stock`/`Vec<ProcessStep>`, duplicating `SolidSpec` geometry) — they compose real
//! `s.stdio.semio.brep`/`s.stdio.semio.flow` CHILD HANDLES. `#[derive(dsl::DslRecord)]` is dropped
//! (an `ArtifactChild<S>` field has no `dsl::DslField` impl reachable from this crate, same wall
//! `📐️cad`/`✳️object`/`✳️kit` hit) in favor of a hand-rolled `ArtifactDsl`/`ArtifactPack` — see
//! `🔖️HandcraftedArtifactCodecs` below, matching `📐️cad`'s own snapshot facet exactly.

use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, StepOrigin, Stock, StockQuantity, WorkingSolid, Workshop, WorkshopMachine};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use semio_framework_os_kernel::{FromValue, ToValue};

//#region 🔖️Snapshot
/// 📸️ Persisted process3d document snapshot (persistent fields of the artifact). `stock_solid`/
/// `steps`/`tool_solids` are composed CHILD slots — `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Children must sit directly
/// on this struct (not nested inside a helper record) for the derive to see them — confirmed against
/// `🧬️schema/✨️derive/🦀️.rs`'s field-walk, which only iterates a struct's own direct fields.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dSnapshot {
    #[state(artifact)]
    pub workshop: Workshop,
    #[state(artifact)]
    pub stock_id: String,
    #[state(artifact)]
    pub stock_label: String,
    #[state(artifact)]
    pub stock_pose: Pose,
    #[state(artifact)]
    pub stock_payload: Stock,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub stock_solid: store::ArtifactChild<SemioBrepSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub steps: store::ArtifactChild<SemioFlowSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub step_payloads: Vec<ProcessStep>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    #[value(default)]
    pub tool_solids: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[value(default)]
    pub resolved_up_to: Option<usize>,
}

impl Default for Process3dSnapshot {
    fn default() -> Self {
        crate::artifacts::process3d::empty_process3d_snapshot()
    }
}

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`/`✳️object`/`✳️kit`'s own) — a handle is
/// exactly two strings (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`), never the
/// child's own content.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) fn enc_child_list<S>(items: &[store::ArtifactChild<S>]) -> String {
    format!("[{}]", items.iter().map(enc_child).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_child_list<S>(s: &str) -> Result<Vec<store::ArtifactChild<S>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_child).collect()
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️JsonFieldPrimitives
/// 🧾️ `workshop`/`stock_pose` are structured but child-free — JSON-serialize then hex-encode
/// through the shared `enc_str`/`dec_str`, matching `📐️cad`'s established `enc_json`/`dec_json`
/// convention for structured fields that don't need a bespoke wire shape.
fn enc_json<T: ToValue>(value: &T) -> String {
    enc_str(&semio_framework_os_kernel::json::to_json_string(value))
}
fn dec_json<T: FromValue>(s: &str) -> Result<T, String> {
    semio_framework_os_kernel::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️JsonFieldPrimitives

//#region 🔖️TextPrimitives
fn print_process3d_snapshot_body(s: &Process3dSnapshot) -> String {
    format!(
        "workshop={}\nstockId={}\nstockLabel={}\nstockPose={}\nstockPayload={}\nstockSolid={}\nsteps={}\nstepPayloads={}\ntoolSolids={}\nresolvedUpTo={}",
        enc_json(&s.workshop),
        enc_str(&s.stock_id),
        enc_str(&s.stock_label),
        enc_json(&s.stock_pose),
        enc_json(&s.stock_payload),
        enc_child(&s.stock_solid),
        enc_child(&s.steps),
        enc_json(&s.step_payloads),
        enc_child_list(&s.tool_solids),
        enc_json(&s.resolved_up_to),
    )
}
fn parse_process3d_snapshot_body(body: &str) -> Result<Process3dSnapshot, String> {
    let mut snapshot = crate::artifacts::process3d::empty_process3d_snapshot();
    let mut saw_workshop = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("workshop=") {
            snapshot.workshop = dec_json(rest)?;
            saw_workshop = true;
        } else if let Some(rest) = line.strip_prefix("stockId=") {
            snapshot.stock_id = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("stockLabel=") {
            snapshot.stock_label = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("stockPose=") {
            snapshot.stock_pose = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("stockPayload=") {
            snapshot.stock_payload = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("stockSolid=") {
            snapshot.stock_solid = dec_child(rest)?;
        } else if let Some(rest) = line.strip_prefix("steps=") {
            snapshot.steps = dec_child(rest)?;
        } else if let Some(rest) = line.strip_prefix("stepPayloads=") {
            snapshot.step_payloads = dec_json(rest)?;
        } else if let Some(rest) = line.strip_prefix("toolSolids=") {
            snapshot.tool_solids = dec_child_list(rest)?;
        } else if let Some(rest) = line.strip_prefix("resolvedUpTo=") {
            snapshot.resolved_up_to = dec_json(rest)?;
        } else {
            return Err(format!("process3d snapshot: unknown line {line:?}"));
        }
    }
    if !saw_workshop {
        return Err("process3d snapshot: missing workshop line".to_string());
    }
    Ok(snapshot)
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
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
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
pub(crate) fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
pub(crate) fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
fn write_child_list<S>(out: &mut Vec<u8>, items: &[store::ArtifactChild<S>]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items {
        write_child(out, item);
    }
}
fn read_child_list<S>(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactChild<S>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    if count > 8192 {
        return Err("process3d pack child count exceeds fixed capacity".into());
    }
    let mut items = Vec::with_capacity(count as usize);
    for _ in 0..count {
        items.push(read_child(reader)?);
    }
    Ok(items)
}

pub(crate) fn write_pose(out: &mut Vec<u8>, pose: &Pose) {
    for value in pose.position.iter().chain(pose.axis.iter()).chain(std::iter::once(&pose.angle)) {
        out.extend_from_slice(&value.to_le_bytes());
    }
}

pub(crate) fn read_pose(reader: &mut store::ByteReader<'_>) -> Result<Pose, String> {
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
    }
}

fn read_solid(reader: &mut store::ByteReader<'_>) -> Result<WorkingSolid, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(WorkingSolid::Box { width: reader.read_f64_le().map_err(|e| e.to_string())?, depth: reader.read_f64_le().map_err(|e| e.to_string())?, height: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(WorkingSolid::Cylinder { radius: reader.read_f64_le().map_err(|e| e.to_string())?, height: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(WorkingSolid::Sphere { radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(WorkingSolid::ImportedMesh { mesh_url: read_str_lp(reader)? }),
        4 => Ok(WorkingSolid::ImportedSolid { solid_handle: read_str_lp(reader)? }),
        _ => Err("process3d pack solid tag is invalid".into()),
    }
}

pub(crate) fn write_measure(out: &mut Vec<u8>, measure: &ProcessMeasure) {
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

pub(crate) fn read_measure(reader: &mut store::ByteReader<'_>) -> Result<ProcessMeasure, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(ProcessMeasure::Cut { tool: read_solid(reader)?, pose: read_pose(reader)? }),
        1 => Ok(ProcessMeasure::Drill { radius: reader.read_f64_le().map_err(|e| e.to_string())?, depth: reader.read_f64_le().map_err(|e| e.to_string())?, pose: read_pose(reader)? }),
        2 => Ok(ProcessMeasure::Attach { component: read_solid(reader)?, pose: read_pose(reader)? }),
        _ => Err("process3d pack measure tag is invalid".into()),
    }
}

fn write_step(out: &mut Vec<u8>, step: &ProcessStep) {
    write_str_lp(out, &step.id);
    write_str_lp(out, &step.label);
    out.push(u8::from(step.enabled));
    out.push(u8::from(step.origin.is_some()));
    if let Some(origin) = &step.origin {
        write_str_lp(out, &origin.machine_id);
        write_str_lp(out, &origin.capability_id);
    }
    write_measure(out, &step.measure);
}

fn read_step(reader: &mut store::ByteReader<'_>) -> Result<ProcessStep, String> {
    let id = read_str_lp(reader)?;
    let label = read_str_lp(reader)?;
    let enabled = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let origin = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(StepOrigin { machine_id: read_str_lp(reader)?, capability_id: read_str_lp(reader)? }),
        _ => return Err("process3d pack origin tag is invalid".into()),
    };
    Ok(ProcessStep { id, label, enabled, origin, measure: read_measure(reader)? })
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

pub(crate) fn write_capability(out: &mut Vec<u8>, capability: &Capability) {
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

pub(crate) fn read_capability(reader: &mut store::ByteReader<'_>) -> Result<Capability, String> {
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

pub(crate) fn write_machine(out: &mut Vec<u8>, machine: &WorkshopMachine) {
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

pub(crate) fn read_machine(reader: &mut store::ByteReader<'_>) -> Result<WorkshopMachine, String> {
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

fn encode_process3d_snapshot_binary(s: &Process3dSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 2;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_count(&mut out, s.workshop.machines.len());
    for machine in &s.workshop.machines {
        write_machine(&mut out, machine);
    }
    write_str_lp(&mut out, &s.stock_id);
    write_str_lp(&mut out, &s.stock_label);
    write_pose(&mut out, &s.stock_pose);
    write_str_lp(&mut out, &s.stock_payload.id);
    write_str_lp(&mut out, &s.stock_payload.label);
    write_solid(&mut out, &s.stock_payload.solid);
    write_pose(&mut out, &s.stock_payload.pose);
    write_child(&mut out, &s.stock_solid);
    write_child(&mut out, &s.steps);
    write_count(&mut out, s.step_payloads.len());
    for step in &s.step_payloads {
        write_step(&mut out, step);
    }
    write_child_list(&mut out, &s.tool_solids);
    out.push(u8::from(s.resolved_up_to.is_some()));
    if let Some(cursor) = s.resolved_up_to {
        store::pack_rt::write_varint_u64(&mut out, cursor as u64);
    }
    out
}
fn decode_process3d_snapshot_binary(bytes: &[u8]) -> Result<Process3dSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 2;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let machine_count = read_count(&mut reader)?;
    let mut machines = Vec::with_capacity(machine_count);
    for _ in 0..machine_count {
        machines.push(read_machine(&mut reader)?);
    }
    let workshop = Workshop { machines };
    let stock_id = read_str_lp(&mut reader)?;
    let stock_label = read_str_lp(&mut reader)?;
    let stock_pose = read_pose(&mut reader)?;
    let stock_payload = Stock { id: read_str_lp(&mut reader)?, label: read_str_lp(&mut reader)?, solid: read_solid(&mut reader)?, pose: read_pose(&mut reader)? };
    let stock_solid = read_child(&mut reader)?;
    let steps = read_child(&mut reader)?;
    let step_count = read_count(&mut reader)?;
    let mut step_payloads = Vec::with_capacity(step_count);
    for _ in 0..step_count {
        step_payloads.push(read_step(&mut reader)?);
    }
    let tool_solids = read_child_list(&mut reader)?;
    let resolved_up_to = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize),
        _ => return Err("process3d pack cursor tag is invalid".into()),
    };
    if reader.remaining() != 0 {
        return Err("process3d pack has trailing bytes".into());
    }
    Ok(Process3dSnapshot { workshop, stock_id, stock_label, stock_pose, stock_payload, stock_solid, steps, step_payloads, tool_solids, resolved_up_to })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️RetainedStructuralReader
fn retained_advance<T>(bytes: &[u8], offset: &mut usize, read: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<T, String> {
    let mut reader = store::ByteReader::new(bytes.get(*offset..).ok_or_else(|| "process3d retained cursor offset escaped its backing".to_string())?);
    let value = read(&mut reader)?;
    *offset = offset.checked_add(reader.position()).ok_or_else(|| "process3d retained cursor offset overflow".to_string())?;
    Ok(value)
}

#[derive(Default)]
pub(crate) struct Process3dRetainedPoseCursor {
    index: usize,
    values: [f64; 7],
}

impl Process3dRetainedPoseCursor {
    pub(crate) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Pose>, String> {
        self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
        self.index += 1;
        Ok((self.index == self.values.len()).then(|| Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }))
    }

    pub(crate) fn take_partial(&mut self) -> Pose {
        self.index = 0;
        Pose { position: [self.values[0], self.values[1], self.values[2]], axis: [self.values[3], self.values[4], self.values[5]], angle: self.values[6] }
    }
}

#[derive(Default)]
pub(crate) struct Process3dRetainedSolidCursor {
    tag: Option<u8>,
    index: usize,
    values: [f64; 3],
    text: Option<String>,
    string: Process3dRetainedStringCursor,
}

impl Process3dRetainedSolidCursor {
    pub(crate) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<WorkingSolid>, String> {
        let Some(tag) = self.tag else {
            let tag = reader.read_u8().map_err(|error| error.to_string())?;
            if tag > 4 {
                return Err("process3d retained solid tag is invalid".into());
            }
            self.tag = Some(tag);
            return Ok(None);
        };
        let scalar_count = match tag {
            0 => 3,
            1 => 2,
            2 => 1,
            _ => 0,
        };
        if self.index < scalar_count {
            self.values[self.index] = reader.read_f64_le().map_err(|error| error.to_string())?;
            self.index += 1;
            if self.index < scalar_count {
                return Ok(None);
            }
        } else if tag >= 3 {
            let Some(text) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.text = Some(text);
        }
        Ok(Some(self.take_partial()))
    }

    pub(crate) fn take_partial(&mut self) -> WorkingSolid {
        let tag = self.tag.take().unwrap_or(0);
        self.index = 0;
        if tag >= 3 && self.text.is_none() {
            self.text = Some(self.string.take_partial());
        }
        match tag {
            0 => WorkingSolid::Box { width: self.values[0], depth: self.values[1], height: self.values[2] },
            1 => WorkingSolid::Cylinder { radius: self.values[0], height: self.values[1] },
            2 => WorkingSolid::Sphere { radius: self.values[0] },
            3 => WorkingSolid::ImportedMesh { mesh_url: self.text.take().unwrap_or_default() },
            4 => WorkingSolid::ImportedSolid { solid_handle: self.text.take().unwrap_or_default() },
            _ => unreachable!("validated retained solid tag"),
        }
    }
}

#[derive(Default)]
pub(crate) struct Process3dRetainedChildCursor {
    phase: u8,
    child_id: Option<String>,
    target: Option<store::os_io::ArtifactRef>,
    string: Process3dRetainedStringCursor,
}

impl Process3dRetainedChildCursor {
    pub(crate) fn step<S>(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
        if self.phase == 0 {
            let Some(child_id) = self.string.step(reader)? else { return Ok(None) };
            self.string = Process3dRetainedStringCursor::default();
            self.child_id = Some(child_id);
            self.phase = 1;
            return Ok(None);
        }
        let Some(uri) = self.string.step(reader)? else { return Ok(None) };
        self.string = Process3dRetainedStringCursor::default();
        let target = store::os_io::ArtifactRef::parse_uri(&uri).map_err(|error| error.to_string())?;
        self.target = Some(target);
        Ok(Some(store::ArtifactChild::new(self.child_id.take().unwrap_or_default(), self.target.take().expect("Process3d retained child target exists"))))
    }

    pub(crate) fn take_partial<S>(&mut self) -> store::ArtifactChild<S> {
        if self.phase == 0 && self.child_id.is_none() {
            self.child_id = Some(self.string.take_partial());
        } else if self.phase == 1 && self.target.is_none() {
            self.target = store::os_io::ArtifactRef::parse_uri(&self.string.take_partial()).ok();
        }
        store::ArtifactChild::new(
            self.child_id.take().unwrap_or_default(),
            self.target.take().unwrap_or(store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedMeasurePhase {
    Tag,
    Solid,
    Radius,
    Depth,
    Pose,
}

pub(crate) struct Process3dRetainedMeasureCursor {
    phase: Process3dRetainedMeasurePhase,
    tag: u8,
    radius: f64,
    depth: f64,
    solid: Option<WorkingSolid>,
    solid_cursor: Process3dRetainedSolidCursor,
    pose_cursor: Process3dRetainedPoseCursor,
}

impl Default for Process3dRetainedMeasureCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedMeasurePhase::Tag, tag: 0, radius: 0.0, depth: 0.0, solid: None, solid_cursor: Process3dRetainedSolidCursor::default(), pose_cursor: Process3dRetainedPoseCursor::default() }
    }
}

impl Process3dRetainedMeasureCursor {
    pub(crate) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessMeasure>, String> {
        match self.phase {
            Process3dRetainedMeasurePhase::Tag => {
                self.tag = reader.read_u8().map_err(|error| error.to_string())?;
                self.phase = match self.tag {
                    0 | 2 => Process3dRetainedMeasurePhase::Solid,
                    1 => Process3dRetainedMeasurePhase::Radius,
                    _ => return Err("process3d retained measure tag is invalid".into()),
                };
            }
            Process3dRetainedMeasurePhase::Solid => {
                if let Some(solid) = self.solid_cursor.step(reader)? {
                    self.solid = Some(solid);
                    self.phase = Process3dRetainedMeasurePhase::Pose;
                }
            }
            Process3dRetainedMeasurePhase::Radius => {
                self.radius = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Depth;
            }
            Process3dRetainedMeasurePhase::Depth => {
                self.depth = reader.read_f64_le().map_err(|error| error.to_string())?;
                self.phase = Process3dRetainedMeasurePhase::Pose;
            }
            Process3dRetainedMeasurePhase::Pose => {
                if let Some(pose) = self.pose_cursor.step(reader)? {
                    let solid = self.solid.take();
                    return Ok(Some(match self.tag {
                        0 => ProcessMeasure::Cut { tool: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
                        2 => ProcessMeasure::Attach { component: solid.unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose },
                        _ => unreachable!("validated retained measure tag"),
                    }));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn take_partial(&mut self) -> ProcessMeasure {
        let pose = self.pose_cursor.take_partial();
        let solid = self.solid.take().unwrap_or_else(|| self.solid_cursor.take_partial());
        match self.tag {
            1 => ProcessMeasure::Drill { radius: self.radius, depth: self.depth, pose },
            2 => ProcessMeasure::Attach { component: solid, pose },
            _ => ProcessMeasure::Cut { tool: solid, pose },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedStepPhase {
    Id,
    Label,
    Enabled,
    OriginTag,
    OriginMachine,
    OriginCapability,
    Measure,
}

pub(crate) struct Process3dRetainedStepCursor {
    phase: Process3dRetainedStepPhase,
    id: Option<String>,
    label: Option<String>,
    enabled: bool,
    origin_machine: Option<String>,
    origin_capability: Option<String>,
    string: Process3dRetainedStringCursor,
    measure_cursor: Process3dRetainedMeasureCursor,
}

impl Default for Process3dRetainedStepCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedStepPhase::Id, id: None, label: None, enabled: false, origin_machine: None, origin_capability: None, string: Process3dRetainedStringCursor::default(), measure_cursor: Process3dRetainedMeasureCursor::default() }
    }
}

impl Process3dRetainedStepCursor {
    pub(crate) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<ProcessStep>, String> {
        match self.phase {
            Process3dRetainedStepPhase::Id => {
                let Some(id) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.id = Some(id);
                self.phase = Process3dRetainedStepPhase::Label;
            }
            Process3dRetainedStepPhase::Label => {
                let Some(label) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.label = Some(label);
                self.phase = Process3dRetainedStepPhase::Enabled;
            }
            Process3dRetainedStepPhase::Enabled => {
                self.enabled = reader.read_u8().map_err(|error| error.to_string())? != 0;
                self.phase = Process3dRetainedStepPhase::OriginTag;
            }
            Process3dRetainedStepPhase::OriginTag => {
                self.phase = match reader.read_u8().map_err(|error| error.to_string())? {
                    0 => Process3dRetainedStepPhase::Measure,
                    1 => Process3dRetainedStepPhase::OriginMachine,
                    _ => return Err("process3d retained origin tag is invalid".into()),
                };
            }
            Process3dRetainedStepPhase::OriginMachine => {
                let Some(machine) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_machine = Some(machine);
                self.phase = Process3dRetainedStepPhase::OriginCapability;
            }
            Process3dRetainedStepPhase::OriginCapability => {
                let Some(capability) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.origin_capability = Some(capability);
                self.phase = Process3dRetainedStepPhase::Measure;
            }
            Process3dRetainedStepPhase::Measure => {
                if let Some(measure) = self.measure_cursor.step(reader)? {
                    return Ok(Some(ProcessStep {
                        id: self.id.take().unwrap_or_default(),
                        label: self.label.take().unwrap_or_default(),
                        enabled: self.enabled,
                        origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
                        measure,
                    }));
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn take_partial(&mut self) -> ProcessStep {
        let partial = self.string.take_partial();
        match self.phase {
            Process3dRetainedStepPhase::Id if self.id.is_none() => self.id = Some(partial),
            Process3dRetainedStepPhase::Label if self.label.is_none() => self.label = Some(partial),
            Process3dRetainedStepPhase::OriginMachine if self.origin_machine.is_none() => self.origin_machine = Some(partial),
            Process3dRetainedStepPhase::OriginCapability if self.origin_capability.is_none() => self.origin_capability = Some(partial),
            _ => drop(partial),
        }
        ProcessStep {
            id: self.id.take().unwrap_or_default(),
            label: self.label.take().unwrap_or_default(),
            enabled: self.enabled,
            origin: self.origin_machine.take().map(|machine_id| StepOrigin { machine_id, capability_id: self.origin_capability.take().unwrap_or_default() }),
            measure: self.measure_cursor.take_partial(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedStockPhase {
    Id,
    Label,
    Solid,
    Pose,
}

pub(crate) struct Process3dRetainedStockCursor {
    phase: Process3dRetainedStockPhase,
    id: Option<String>,
    label: Option<String>,
    solid: Option<WorkingSolid>,
    string: Process3dRetainedStringCursor,
    solid_cursor: Process3dRetainedSolidCursor,
    pose_cursor: Process3dRetainedPoseCursor,
}

impl Default for Process3dRetainedStockCursor {
    fn default() -> Self {
        Self { phase: Process3dRetainedStockPhase::Id, id: None, label: None, solid: None, string: Process3dRetainedStringCursor::default(), solid_cursor: Process3dRetainedSolidCursor::default(), pose_cursor: Process3dRetainedPoseCursor::default() }
    }
}

impl Process3dRetainedStockCursor {
    fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<Stock>, String> {
        match self.phase {
            Process3dRetainedStockPhase::Id => {
                let Some(id) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.id = Some(id);
                self.phase = Process3dRetainedStockPhase::Label;
            }
            Process3dRetainedStockPhase::Label => {
                let Some(label) = self.string.step(reader)? else { return Ok(None) };
                self.string = Process3dRetainedStringCursor::default();
                self.label = Some(label);
                self.phase = Process3dRetainedStockPhase::Solid;
            }
            Process3dRetainedStockPhase::Solid => {
                if let Some(solid) = self.solid_cursor.step(reader)? {
                    self.solid = Some(solid);
                    self.phase = Process3dRetainedStockPhase::Pose;
                }
            }
            Process3dRetainedStockPhase::Pose => {
                if let Some(pose) = self.pose_cursor.step(reader)? {
                    return Ok(Some(Stock { id: self.id.take().unwrap_or_default(), label: self.label.take().unwrap_or_default(), solid: self.solid.take().unwrap_or(WorkingSolid::Sphere { radius: 0.0 }), pose }));
                }
            }
        }
        Ok(None)
    }

    fn take_partial(&mut self) -> Stock {
        let partial = self.string.take_partial();
        match self.phase {
            Process3dRetainedStockPhase::Id if self.id.is_none() => self.id = Some(partial),
            Process3dRetainedStockPhase::Label if self.label.is_none() => self.label = Some(partial),
            _ => drop(partial),
        }
        Stock { id: self.id.take().unwrap_or_default(), label: self.label.take().unwrap_or_default(), solid: self.solid.take().unwrap_or_else(|| self.solid_cursor.take_partial()), pose: self.pose_cursor.take_partial() }
    }
}

pub(crate) struct Process3dRetainedStringCursor {
    length: usize,
    shift: u32,
    remaining: Option<usize>,
    bytes: Vec<u8>,
    maximum_bytes: usize,
}

impl Default for Process3dRetainedStringCursor {
    fn default() -> Self {
        Self { length: 0, shift: 0, remaining: None, bytes: Vec::new(), maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES }
    }
}

impl Process3dRetainedStringCursor {
    #[cfg(test)]
    fn with_maximum_bytes(maximum_bytes: usize) -> Self {
        let mut cursor = Self::default();
        cursor.maximum_bytes = maximum_bytes;
        cursor
    }

    fn finish(&mut self) -> Result<String, String> {
        self.length = 0;
        self.shift = 0;
        self.remaining = None;
        String::from_utf8(std::mem::take(&mut self.bytes)).map_err(|error| error.to_string())
    }

    pub(crate) fn step(&mut self, reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
        if let Some(remaining) = self.remaining {
            let byte = reader.read_u8().map_err(|error| error.to_string())?;
            self.bytes.push(byte);
            let remaining = remaining - 1;
            self.remaining = Some(remaining);
            return if remaining == 0 { Ok(Some(self.finish()?)) } else { Ok(None) };
        }
        let byte = reader.read_u8().map_err(|error| error.to_string())?;
        let payload = usize::from(byte & 0x7f);
        self.length = self.length.checked_add(payload.checked_shl(self.shift).ok_or_else(|| "process3d retained string length overflow".to_string())?).ok_or_else(|| "process3d retained string length overflow".to_string())?;
        if byte & 0x80 != 0 {
            self.shift = self.shift.checked_add(7).filter(|shift| *shift < usize::BITS).ok_or_else(|| "process3d retained string length overflow".to_string())?;
            return Ok(None);
        }
        if self.length > self.maximum_bytes {
            return Err("process3d retained string exceeded its fixed byte credit".into());
        }
        self.bytes.try_reserve_exact(self.length).map_err(|_| "process3d retained string admission failed".to_string())?;
        self.remaining = Some(self.length);
        if self.length == 0 {
            Ok(Some(self.finish()?))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn take_partial(&mut self) -> String {
        self.finish().unwrap_or_default()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.length == 0 && self.shift == 0 && self.remaining.is_none() && self.bytes.is_empty()
    }

    pub(crate) fn has_partial(&self) -> bool {
        self.length != 0 || self.shift != 0 || self.remaining.is_some() || !self.bytes.is_empty()
    }
}

impl Drop for Process3dRetainedStringCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained string cursor reached Drop before exact handback");
    }
}

fn retained_string_step(bytes: &[u8], offset: &mut usize, cursor: &mut Option<Process3dRetainedStringCursor>) -> Result<Option<String>, String> {
    retained_advance(bytes, offset, |reader| cursor.get_or_insert_with(Process3dRetainedStringCursor::default).step(reader))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedSnapshotPhase {
    Header,
    Format,
    MachineCount,
    MachineId,
    MachineLabel,
    MachineIcon,
    MachineCatalogTag,
    MachineCatalogId,
    MachineCapabilityCount,
    CapabilityId,
    CapabilityLabel,
    CapabilityIcon,
    CapabilityRecipeTag,
    CapabilityRecipeField(usize),
    CapabilityParameterCount,
    ParameterId,
    ParameterLabel,
    ParameterValue,
    CapabilityRuleCount,
    RuleTag,
    RuleQuantity,
    RuleParameter,
    RuleMargin,
    MachineComplete,
    StockId,
    StockLabel,
    StockPose,
    StockPayload,
    StockSolid,
    Steps,
    StepCount,
    Step(usize),
    ToolCount,
    Tool(usize),
    Cursor,
    Complete,
}

/// 🧵️ Retained structural reader used only by mounted envelope ingress. It borrows the
/// admitted fixed backing and constructs at most one field or collection item per grant.
pub(crate) struct Process3dRetainedSnapshotReader {
    phase: Process3dRetainedSnapshotPhase,
    offset: usize,
    expected: usize,
    remaining_items: usize,
    machine_index: usize,
    capability_index: usize,
    total_capabilities: usize,
    parameter_index: usize,
    total_parameters: usize,
    rule_index: usize,
    total_rules: usize,
    recipe_tag: u8,
    rule_tag: u8,
    rule_quantity: StockQuantity,
    candidate: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    active_machine: std::mem::ManuallyDrop<Option<WorkshopMachine>>,
    active_capability: std::mem::ManuallyDrop<Option<Capability>>,
    active_string: std::mem::ManuallyDrop<Option<Process3dRetainedStringCursor>>,
    active_recipe_fields: std::mem::ManuallyDrop<[Option<String>; 3]>,
    active_parameter_id: std::mem::ManuallyDrop<Option<String>>,
    active_parameter_label: std::mem::ManuallyDrop<Option<String>>,
    active_rule_parameter: std::mem::ManuallyDrop<Option<String>>,
    active_pose: std::mem::ManuallyDrop<Option<Process3dRetainedPoseCursor>>,
    active_stock: std::mem::ManuallyDrop<Option<Process3dRetainedStockCursor>>,
    active_child: std::mem::ManuallyDrop<Option<Process3dRetainedChildCursor>>,
    active_step: std::mem::ManuallyDrop<Option<Process3dRetainedStepCursor>>,
    terminal_handoff: bool,
}

impl Process3dRetainedSnapshotReader {
    pub(crate) fn new(maximum_items: usize) -> Self {
        let empty_brep = store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        let empty_flow = store::ArtifactChild::new(String::new(), store::os_io::ArtifactRef { artifact_id: String::new(), dialect: store::os_io::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            phase: Process3dRetainedSnapshotPhase::Header,
            offset: 0,
            expected: 0,
            remaining_items: maximum_items,
            machine_index: 0,
            capability_index: 0,
            total_capabilities: 0,
            parameter_index: 0,
            total_parameters: 0,
            rule_index: 0,
            total_rules: 0,
            recipe_tag: 0,
            rule_tag: 0,
            rule_quantity: StockQuantity::Width,
            candidate: std::mem::ManuallyDrop::new(Some(Process3dSnapshot {
                workshop: Workshop { machines: Vec::new() },
                stock_id: String::new(),
                stock_label: String::new(),
                stock_pose: Pose::default(),
                stock_payload: Stock { id: String::new(), label: String::new(), solid: WorkingSolid::Box { width: 0.0, depth: 0.0, height: 0.0 }, pose: Pose::default() },
                stock_solid: empty_brep,
                steps: empty_flow,
                step_payloads: Vec::new(),
                tool_solids: Vec::new(),
                resolved_up_to: None,
            })),
            active_machine: std::mem::ManuallyDrop::new(None),
            active_capability: std::mem::ManuallyDrop::new(None),
            active_string: std::mem::ManuallyDrop::new(None),
            active_recipe_fields: std::mem::ManuallyDrop::new([None, None, None]),
            active_parameter_id: std::mem::ManuallyDrop::new(None),
            active_parameter_label: std::mem::ManuallyDrop::new(None),
            active_rule_parameter: std::mem::ManuallyDrop::new(None),
            active_pose: std::mem::ManuallyDrop::new(None),
            active_stock: std::mem::ManuallyDrop::new(None),
            active_child: std::mem::ManuallyDrop::new(None),
            active_step: std::mem::ManuallyDrop::new(None),
            terminal_handoff: false,
        }
    }

    fn advance_reader<T>(&mut self, bytes: &[u8], read: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<T, String> {
        let mut reader = store::ByteReader::new(bytes.get(self.offset..).ok_or_else(|| "process3d retained reader offset escaped its backing".to_string())?);
        let value = read(&mut reader)?;
        self.offset = self.offset.checked_add(reader.position()).ok_or_else(|| "process3d retained reader offset overflow".to_string())?;
        Ok(value)
    }

    fn admit_items(&mut self, count: usize, kind: &'static str) -> Result<(), String> {
        self.remaining_items = self.remaining_items.checked_sub(count).ok_or_else(|| format!("process3d retained {kind} count exceeded its exact item credit"))?;
        Ok(())
    }

    fn finish_capability(&mut self) {
        let value = self.active_capability.take().expect("Process3d retained capability exists");
        self.active_machine.as_mut().expect("Process3d retained machine exists").capabilities.push(value);
        self.capability_index += 1;
        self.phase = if self.capability_index < self.total_capabilities { Process3dRetainedSnapshotPhase::CapabilityId } else { Process3dRetainedSnapshotPhase::MachineComplete };
    }

    fn finish_recipe(&mut self) -> Result<(), String> {
        let [first, second, third] = std::mem::take(&mut *self.active_recipe_fields);
        let first = first.unwrap_or_default();
        let second = second.unwrap_or_default();
        let third = third.unwrap_or_default();
        self.active_capability.as_mut().expect("Process3d retained capability exists").recipe = match self.recipe_tag {
            0 => MeasureRecipe::DiscCut { diameter: first, kerf: second },
            1 => MeasureRecipe::BladeCut { kerf: first, length: second, depth: third },
            2 => MeasureRecipe::PocketCut { diameter: first, depth: second },
            3 => MeasureRecipe::BoreDrill { radius: first, depth: second },
            4 => MeasureRecipe::CylinderAttach { radius: first, length: second },
            5 => MeasureRecipe::BoxAttach { width: first, depth: second, height: third },
            _ => return Err("process3d retained recipe tag is invalid".into()),
        };
        Ok(())
    }

    pub(crate) fn step(&mut self, bytes: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, String> {
        if cx.should_yield() || cx.fuel_remaining() == 0 || cx.is_cancelled() {
            return Ok(false);
        }
        match self.phase {
            Process3dRetainedSnapshotPhase::Header => {
                const TOKEN: &[u8] = b"process.process3d.pack v1";
                if bytes.len() < 12 || bytes[..8] != store::semio_format::BINARY_MAGIC {
                    return Err("process3d retained pack header is invalid".into());
                }
                let token_len = u32::from_le_bytes(bytes[8..12].try_into().map_err(|_| "process3d retained token length is invalid")?) as usize;
                let token_end = 12usize.checked_add(token_len).ok_or("process3d retained token length overflow")?;
                if bytes.get(12..token_end) != Some(TOKEN) {
                    return Err("process3d retained pack token is invalid".into());
                }
                self.offset = token_end;
                self.phase = Process3dRetainedSnapshotPhase::Format;
            }
            Process3dRetainedSnapshotPhase::Format => {
                let format = self.advance_reader(bytes, |reader| reader.read_u8().map_err(|e| e.to_string()))?;
                if format != 2 {
                    return Err("process3d retained pack format is unsupported".into());
                }
                self.phase = Process3dRetainedSnapshotPhase::MachineCount;
            }
            Process3dRetainedSnapshotPhase::MachineCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "machine")?;
                let machines = &mut self.candidate.as_mut().expect("Process3d retained shell exists").workshop.machines;
                machines.try_reserve_exact(count).map_err(|_| "process3d retained machine admission failed".to_string())?;
                if machines.capacity() > 8192 {
                    return Err("process3d retained machine observed capacity exceeded".into());
                }
                self.expected = count;
                self.phase = if count == 0 { Process3dRetainedSnapshotPhase::StockId } else { Process3dRetainedSnapshotPhase::MachineId };
            }
            Process3dRetainedSnapshotPhase::MachineId => {
                if let Some(id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    *self.active_machine = Some(WorkshopMachine { id, label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() });
                    self.phase = Process3dRetainedSnapshotPhase::MachineLabel;
                }
            }
            Process3dRetainedSnapshotPhase::MachineLabel => {
                if let Some(label) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_machine.as_mut().expect("Process3d retained machine exists").label = label;
                    self.phase = Process3dRetainedSnapshotPhase::MachineIcon;
                }
            }
            Process3dRetainedSnapshotPhase::MachineIcon => {
                if let Some(icon_id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_machine.as_mut().expect("Process3d retained machine exists").icon_id = icon_id;
                    self.phase = Process3dRetainedSnapshotPhase::MachineCatalogTag;
                }
            }
            Process3dRetainedSnapshotPhase::MachineCatalogTag => {
                let tag = self.advance_reader(bytes, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                self.phase = match tag {
                    0 => Process3dRetainedSnapshotPhase::MachineCapabilityCount,
                    1 => Process3dRetainedSnapshotPhase::MachineCatalogId,
                    _ => return Err("process3d retained catalog tag is invalid".into()),
                };
            }
            Process3dRetainedSnapshotPhase::MachineCatalogId => {
                if let Some(catalog_id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_machine.as_mut().expect("Process3d retained machine exists").catalog_id = Some(catalog_id);
                    self.phase = Process3dRetainedSnapshotPhase::MachineCapabilityCount;
                }
            }
            Process3dRetainedSnapshotPhase::MachineCapabilityCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "capability")?;
                let capabilities = &mut self.active_machine.as_mut().expect("Process3d retained machine exists").capabilities;
                capabilities.try_reserve_exact(count).map_err(|_| "process3d retained capability admission failed".to_string())?;
                self.capability_index = 0;
                self.total_capabilities = count;
                self.phase = if count == 0 { Process3dRetainedSnapshotPhase::MachineComplete } else { Process3dRetainedSnapshotPhase::CapabilityId };
            }
            Process3dRetainedSnapshotPhase::CapabilityId => {
                if let Some(id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    *self.active_capability = Some(Capability { id, label: String::new(), icon_id: String::new(), recipe: MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }, parameters: Vec::new(), rules: Vec::new() });
                    self.phase = Process3dRetainedSnapshotPhase::CapabilityLabel;
                }
            }
            Process3dRetainedSnapshotPhase::CapabilityLabel => {
                if let Some(label) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_capability.as_mut().expect("Process3d retained capability exists").label = label;
                    self.phase = Process3dRetainedSnapshotPhase::CapabilityIcon;
                }
            }
            Process3dRetainedSnapshotPhase::CapabilityIcon => {
                if let Some(icon_id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_capability.as_mut().expect("Process3d retained capability exists").icon_id = icon_id;
                    self.phase = Process3dRetainedSnapshotPhase::CapabilityRecipeTag;
                }
            }
            Process3dRetainedSnapshotPhase::CapabilityRecipeTag => {
                self.recipe_tag = self.advance_reader(bytes, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                if self.recipe_tag > 5 {
                    return Err("process3d retained recipe tag is invalid".into());
                }
                self.phase = Process3dRetainedSnapshotPhase::CapabilityRecipeField(0);
            }
            Process3dRetainedSnapshotPhase::CapabilityRecipeField(field) => {
                if let Some(value) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.active_recipe_fields[field] = Some(value);
                    if field < 2 {
                        self.phase = Process3dRetainedSnapshotPhase::CapabilityRecipeField(field + 1);
                    } else {
                        self.finish_recipe()?;
                        self.phase = Process3dRetainedSnapshotPhase::CapabilityParameterCount;
                    }
                }
            }
            Process3dRetainedSnapshotPhase::CapabilityParameterCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "capability parameter")?;
                let parameters = &mut self.active_capability.as_mut().expect("Process3d retained capability exists").parameters;
                parameters.try_reserve_exact(count).map_err(|_| "process3d retained capability parameter admission failed".to_string())?;
                self.parameter_index = 0;
                self.total_parameters = count;
                self.phase = if count == 0 { Process3dRetainedSnapshotPhase::CapabilityRuleCount } else { Process3dRetainedSnapshotPhase::ParameterId };
            }
            Process3dRetainedSnapshotPhase::ParameterId => {
                if let Some(id) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    *self.active_parameter_id = Some(id);
                    self.phase = Process3dRetainedSnapshotPhase::ParameterLabel;
                }
            }
            Process3dRetainedSnapshotPhase::ParameterLabel => {
                if let Some(label) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    *self.active_parameter_label = Some(label);
                    self.phase = Process3dRetainedSnapshotPhase::ParameterValue;
                }
            }
            Process3dRetainedSnapshotPhase::ParameterValue => {
                let value = self.advance_reader(bytes, |reader| reader.read_f64_le().map_err(|error| error.to_string()))?;
                let parameter = CapabilityParameter { id: self.active_parameter_id.take().unwrap_or_default(), label: self.active_parameter_label.take().unwrap_or_default(), value };
                self.active_capability.as_mut().expect("Process3d retained capability exists").parameters.push(parameter);
                self.parameter_index += 1;
                self.phase = if self.parameter_index < self.total_parameters { Process3dRetainedSnapshotPhase::ParameterId } else { Process3dRetainedSnapshotPhase::CapabilityRuleCount };
            }
            Process3dRetainedSnapshotPhase::CapabilityRuleCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "capability rule")?;
                let rules = &mut self.active_capability.as_mut().expect("Process3d retained capability exists").rules;
                rules.try_reserve_exact(count).map_err(|_| "process3d retained capability rule admission failed".to_string())?;
                self.rule_index = 0;
                self.total_rules = count;
                if count == 0 {
                    self.finish_capability();
                } else {
                    self.phase = Process3dRetainedSnapshotPhase::RuleTag;
                }
            }
            Process3dRetainedSnapshotPhase::RuleTag => {
                self.rule_tag = self.advance_reader(bytes, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                if self.rule_tag > 1 {
                    return Err("process3d retained capability rule tag is invalid".into());
                }
                self.phase = Process3dRetainedSnapshotPhase::RuleQuantity;
            }
            Process3dRetainedSnapshotPhase::RuleQuantity => {
                self.rule_quantity = match self.advance_reader(bytes, |reader| reader.read_u8().map_err(|error| error.to_string()))? {
                    0 => StockQuantity::Width,
                    1 => StockQuantity::Depth,
                    2 => StockQuantity::Height,
                    3 => StockQuantity::MaxDimension,
                    4 => StockQuantity::MinDimension,
                    _ => return Err("process3d retained stock quantity tag is invalid".into()),
                };
                self.phase = Process3dRetainedSnapshotPhase::RuleParameter;
            }
            Process3dRetainedSnapshotPhase::RuleParameter => {
                if let Some(parameter) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    *self.active_rule_parameter = Some(parameter);
                    self.phase = Process3dRetainedSnapshotPhase::RuleMargin;
                }
            }
            Process3dRetainedSnapshotPhase::RuleMargin => {
                let margin = self.advance_reader(bytes, |reader| reader.read_f64_le().map_err(|error| error.to_string()))?;
                let parameter = self.active_rule_parameter.take().unwrap_or_default();
                let value = match self.rule_tag {
                    0 => CapabilityRule::Min { quantity: self.rule_quantity, parameter, margin },
                    1 => CapabilityRule::Max { quantity: self.rule_quantity, parameter, margin },
                    _ => return Err("process3d retained capability rule tag is invalid".into()),
                };
                self.active_capability.as_mut().expect("Process3d retained capability exists").rules.push(value);
                self.rule_index += 1;
                if self.rule_index < self.total_rules {
                    self.phase = Process3dRetainedSnapshotPhase::RuleTag;
                } else {
                    self.finish_capability();
                }
            }
            Process3dRetainedSnapshotPhase::MachineComplete => {
                let value = self.active_machine.take().expect("Process3d retained machine exists");
                self.candidate.as_mut().expect("Process3d retained shell exists").workshop.machines.push(value);
                self.machine_index += 1;
                self.phase = if self.machine_index < self.expected { Process3dRetainedSnapshotPhase::MachineId } else { Process3dRetainedSnapshotPhase::StockId };
            }
            Process3dRetainedSnapshotPhase::StockId => {
                if let Some(value) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.candidate.as_mut().expect("Process3d retained shell exists").stock_id = value;
                    self.phase = Process3dRetainedSnapshotPhase::StockLabel;
                }
            }
            Process3dRetainedSnapshotPhase::StockLabel => {
                if let Some(value) = retained_string_step(bytes, &mut self.offset, &mut self.active_string)? {
                    drop(self.active_string.take());
                    self.candidate.as_mut().expect("Process3d retained shell exists").stock_label = value;
                    self.phase = Process3dRetainedSnapshotPhase::StockPose;
                }
            }
            Process3dRetainedSnapshotPhase::StockPose => {
                let cursor = self.active_pose.get_or_insert_with(Process3dRetainedPoseCursor::default);
                if let Some(value) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").stock_pose = value;
                    drop(self.active_pose.take());
                    self.phase = Process3dRetainedSnapshotPhase::StockPayload;
                }
            }
            Process3dRetainedSnapshotPhase::StockPayload => {
                let cursor = self.active_stock.get_or_insert_with(Process3dRetainedStockCursor::default);
                if let Some(value) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").stock_payload = value;
                    drop(self.active_stock.take());
                    self.phase = Process3dRetainedSnapshotPhase::StockSolid;
                }
            }
            Process3dRetainedSnapshotPhase::StockSolid => {
                let cursor = self.active_child.get_or_insert_with(Process3dRetainedChildCursor::default);
                if let Some(value) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").stock_solid = value;
                    drop(self.active_child.take());
                    self.phase = Process3dRetainedSnapshotPhase::Steps;
                }
            }
            Process3dRetainedSnapshotPhase::Steps => {
                let cursor = self.active_child.get_or_insert_with(Process3dRetainedChildCursor::default);
                if let Some(value) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").steps = value;
                    drop(self.active_child.take());
                    self.phase = Process3dRetainedSnapshotPhase::StepCount;
                }
            }
            Process3dRetainedSnapshotPhase::StepCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "step")?;
                let steps = &mut self.candidate.as_mut().expect("Process3d retained shell exists").step_payloads;
                steps.try_reserve_exact(count).map_err(|_| "process3d retained step admission failed".to_string())?;
                if steps.capacity() > 8192 {
                    return Err("process3d retained step observed capacity exceeded".into());
                }
                self.expected = count;
                self.phase = Process3dRetainedSnapshotPhase::Step(0);
            }
            Process3dRetainedSnapshotPhase::Step(index) if index < self.expected => {
                let cursor = self.active_step.get_or_insert_with(Process3dRetainedStepCursor::default);
                if let Some(step) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").step_payloads.push(step);
                    drop(self.active_step.take());
                    self.phase = Process3dRetainedSnapshotPhase::Step(index + 1);
                }
            }
            Process3dRetainedSnapshotPhase::Step(_) => self.phase = Process3dRetainedSnapshotPhase::ToolCount,
            Process3dRetainedSnapshotPhase::ToolCount => {
                let count = self.advance_reader(bytes, read_count)?;
                self.admit_items(count, "tool")?;
                let tools = &mut self.candidate.as_mut().expect("Process3d retained shell exists").tool_solids;
                tools.try_reserve_exact(count).map_err(|_| "process3d retained tool admission failed".to_string())?;
                if tools.capacity() > 8192 {
                    return Err("process3d retained tool observed capacity exceeded".into());
                }
                self.expected = count;
                self.phase = Process3dRetainedSnapshotPhase::Tool(0);
            }
            Process3dRetainedSnapshotPhase::Tool(index) if index < self.expected => {
                let cursor = self.active_child.get_or_insert_with(Process3dRetainedChildCursor::default);
                if let Some(child) = retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.candidate.as_mut().expect("Process3d retained shell exists").tool_solids.push(child);
                    drop(self.active_child.take());
                    self.phase = Process3dRetainedSnapshotPhase::Tool(index + 1);
                }
            }
            Process3dRetainedSnapshotPhase::Tool(_) => self.phase = Process3dRetainedSnapshotPhase::Cursor,
            Process3dRetainedSnapshotPhase::Cursor => {
                let cursor = self.advance_reader(bytes, |reader| match reader.read_u8().map_err(|e| e.to_string())? {
                    0 => Ok(None),
                    1 => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
                    _ => Err("process3d retained cursor tag is invalid".into()),
                })?;
                self.candidate.as_mut().expect("Process3d retained shell exists").resolved_up_to = cursor;
                if self.offset != bytes.len() {
                    return Err("process3d retained pack has trailing bytes".into());
                }
                self.phase = Process3dRetainedSnapshotPhase::Complete;
            }
            Process3dRetainedSnapshotPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dRetainedSnapshotPhase::Complete)
    }

    pub(crate) fn take(&mut self) -> Option<Process3dSnapshot> {
        if self.phase != Process3dRetainedSnapshotPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    pub(crate) fn take_rejected(&mut self) -> Option<Process3dSnapshot> {
        if self.terminal_handoff {
            return None;
        }
        if let Some(mut cursor) = self.active_string.take() {
            let partial = cursor.take_partial();
            match self.phase {
                Process3dRetainedSnapshotPhase::MachineId => {
                    *self.active_machine = Some(WorkshopMachine { id: partial, label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() });
                }
                Process3dRetainedSnapshotPhase::MachineLabel => self.active_machine.as_mut().expect("Process3d retained machine exists").label = partial,
                Process3dRetainedSnapshotPhase::MachineIcon => self.active_machine.as_mut().expect("Process3d retained machine exists").icon_id = partial,
                Process3dRetainedSnapshotPhase::MachineCatalogId => self.active_machine.as_mut().expect("Process3d retained machine exists").catalog_id = Some(partial),
                Process3dRetainedSnapshotPhase::CapabilityId => {
                    *self.active_capability = Some(Capability { id: partial, label: String::new(), icon_id: String::new(), recipe: MeasureRecipe::DiscCut { diameter: String::new(), kerf: String::new() }, parameters: Vec::new(), rules: Vec::new() });
                }
                Process3dRetainedSnapshotPhase::CapabilityLabel => self.active_capability.as_mut().expect("Process3d retained capability exists").label = partial,
                Process3dRetainedSnapshotPhase::CapabilityIcon => self.active_capability.as_mut().expect("Process3d retained capability exists").icon_id = partial,
                Process3dRetainedSnapshotPhase::CapabilityRecipeField(field) => self.active_recipe_fields[field] = Some(partial),
                Process3dRetainedSnapshotPhase::ParameterId => *self.active_parameter_id = Some(partial),
                Process3dRetainedSnapshotPhase::ParameterLabel => *self.active_parameter_label = Some(partial),
                Process3dRetainedSnapshotPhase::RuleParameter => *self.active_rule_parameter = Some(partial),
                Process3dRetainedSnapshotPhase::StockId => self.candidate.as_mut().expect("Process3d retained shell exists").stock_id = partial,
                Process3dRetainedSnapshotPhase::StockLabel => self.candidate.as_mut().expect("Process3d retained shell exists").stock_label = partial,
                _ => drop(partial),
            }
        }
        if self.active_recipe_fields.iter().any(Option::is_some) && self.active_capability.is_some() {
            self.finish_recipe().expect("Process3d retained partial recipe tag was admitted");
        }
        if (self.active_parameter_id.is_some() || self.active_parameter_label.is_some()) && self.active_capability.is_some() {
            let parameter = CapabilityParameter { id: self.active_parameter_id.take().unwrap_or_default(), label: self.active_parameter_label.take().unwrap_or_default(), value: 0.0 };
            self.active_capability.as_mut().expect("Process3d retained capability exists").parameters.push(parameter);
        }
        if let Some(parameter) = self.active_rule_parameter.take() {
            if let Some(capability) = self.active_capability.as_mut() {
                let rule = match self.rule_tag {
                    1 => CapabilityRule::Max { quantity: self.rule_quantity, parameter, margin: 0.0 },
                    _ => CapabilityRule::Min { quantity: self.rule_quantity, parameter, margin: 0.0 },
                };
                capability.rules.push(rule);
            }
        }
        if let Some(capability) = self.active_capability.take() {
            if let Some(machine) = self.active_machine.as_mut() {
                machine.capabilities.push(capability);
            }
        }
        if let Some(machine) = self.active_machine.take() {
            if let Some(candidate) = self.candidate.as_mut() {
                candidate.workshop.machines.push(machine);
            }
        }
        if let Some(mut pose) = self.active_pose.take() {
            if let Some(candidate) = self.candidate.as_mut() {
                candidate.stock_pose = pose.take_partial();
            }
        }
        if let Some(mut stock) = self.active_stock.take() {
            if let Some(candidate) = self.candidate.as_mut() {
                candidate.stock_payload = stock.take_partial();
            }
        }
        if let Some(mut child) = self.active_child.take() {
            if let Some(candidate) = self.candidate.as_mut() {
                match self.phase {
                    Process3dRetainedSnapshotPhase::StockSolid => candidate.stock_solid = child.take_partial(),
                    Process3dRetainedSnapshotPhase::Steps => candidate.steps = child.take_partial(),
                    Process3dRetainedSnapshotPhase::Tool(_) => candidate.tool_solids.push(child.take_partial()),
                    _ => candidate.stock_solid = child.take_partial(),
                }
            }
        }
        if let Some(mut step) = self.active_step.take() {
            if let Some(candidate) = self.candidate.as_mut() {
                candidate.step_payloads.push(step.take_partial());
            }
        }
        let candidate = self.candidate.take();
        self.terminal_handoff = true;
        candidate
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff
            && self.candidate.is_none()
            && self.active_machine.is_none()
            && self.active_capability.is_none()
            && self.active_string.is_none()
            && self.active_recipe_fields.iter().all(Option::is_none)
            && self.active_parameter_id.is_none()
            && self.active_parameter_label.is_none()
            && self.active_rule_parameter.is_none()
            && self.active_pose.is_none()
            && self.active_stock.is_none()
            && self.active_child.is_none()
            && self.active_step.is_none()
    }
}

impl Drop for Process3dRetainedSnapshotReader {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained snapshot reader reached Drop before handoff");
    }
}

#[cfg(test)]
mod retained_structural_laws {
    use super::*;

    fn capability(id: &str, recipe: MeasureRecipe) -> Capability {
        Capability {
            id: id.into(),
            label: id.into(),
            icon_id: "tool".into(),
            recipe,
            parameters: vec![
                CapabilityParameter { id: "first".into(), label: "First".into(), value: 1.0 },
                CapabilityParameter { id: "second".into(), label: "Second".into(), value: 2.0 },
                CapabilityParameter { id: "third".into(), label: "Third".into(), value: 3.0 },
            ],
            rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "first".into(), margin: 0.1 }, CapabilityRule::Max { quantity: StockQuantity::MaxDimension, parameter: "third".into(), margin: 0.2 }],
        }
    }

    fn complete_snapshot() -> Process3dSnapshot {
        let mut snapshot = crate::artifacts::process3d::empty_process3d_snapshot();
        snapshot.workshop = Workshop {
            machines: vec![WorkshopMachine {
                id: "machine".into(),
                label: "Machine".into(),
                icon_id: "machine".into(),
                catalog_id: Some("catalog".into()),
                capabilities: vec![
                    capability("disc", MeasureRecipe::DiscCut { diameter: "first".into(), kerf: "second".into() }),
                    capability("blade", MeasureRecipe::BladeCut { kerf: "first".into(), length: "second".into(), depth: "third".into() }),
                    capability("pocket", MeasureRecipe::PocketCut { diameter: "first".into(), depth: "second".into() }),
                    capability("bore", MeasureRecipe::BoreDrill { radius: "first".into(), depth: "second".into() }),
                    capability("cylinder", MeasureRecipe::CylinderAttach { radius: "first".into(), length: "second".into() }),
                    capability("box", MeasureRecipe::BoxAttach { width: "first".into(), depth: "second".into(), height: "third".into() }),
                ],
            }],
        };
        snapshot.stock_payload.solid = WorkingSolid::ImportedMesh { mesh_url: "mesh.glb".into() };
        snapshot.step_payloads = vec![
            ProcessStep {
                id: "box".into(),
                label: "Box".into(),
                enabled: true,
                origin: Some(StepOrigin { machine_id: "machine".into(), capability_id: "disc".into() }),
                measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 1.0, depth: 2.0, height: 3.0 }, pose: Pose::default() },
            },
            ProcessStep { id: "cylinder".into(), label: "Cylinder".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius: 1.0, height: 2.0 }, pose: Pose::default() } },
            ProcessStep { id: "sphere".into(), label: "Sphere".into(), enabled: true, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::Sphere { radius: 1.0 }, pose: Pose::default() } },
            ProcessStep { id: "solid".into(), label: "Solid".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::ImportedSolid { solid_handle: "solid-1".into() }, pose: Pose::default() } },
            ProcessStep { id: "drill".into(), label: "Drill".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.2, depth: 0.4, pose: Pose::default() } },
        ];
        snapshot.tool_solids.push(snapshot.stock_solid.clone());
        snapshot.resolved_up_to = Some(4);
        snapshot
    }

    fn retained_pack(snapshot: &Process3dSnapshot) -> Vec<u8> {
        const TOKEN: &[u8] = b"process.process3d.pack v1";
        let raw = encode_process3d_snapshot_binary(snapshot);
        let mut bytes = Vec::with_capacity(12 + TOKEN.len() + raw.len());
        bytes.extend_from_slice(&store::semio_format::BINARY_MAGIC);
        bytes.extend_from_slice(&(TOKEN.len() as u32).to_le_bytes());
        bytes.extend_from_slice(TOKEN);
        bytes.extend_from_slice(&raw);
        bytes
    }

    #[test]
    fn retained_string_cursor_enforces_one_byte_grants_and_hostile_boundaries() {
        fn grant(cursor: &mut Process3dRetainedStringCursor, byte: u8) -> Result<Option<String>, String> {
            let mut reader = store::ByteReader::new(std::slice::from_ref(&byte));
            let value = cursor.step(&mut reader);
            assert_eq!(reader.position(), 1, "one retained string grant consumes one byte opportunity");
            value
        }

        let mut exact = Process3dRetainedStringCursor::with_maximum_bytes(3);
        assert_eq!(grant(&mut exact, 3).expect("exact length admission"), None);
        assert_eq!(grant(&mut exact, b'a').expect("exact byte one"), None);
        assert_eq!(grant(&mut exact, b'b').expect("exact byte two"), None);
        assert_eq!(grant(&mut exact, b'c').expect("exact byte three"), Some("abc".into()));
        assert!(exact.terminal_is_empty());

        let mut plus_one = Process3dRetainedStringCursor::with_maximum_bytes(3);
        assert!(grant(&mut plus_one, 4).expect_err("maximum plus one must fail before producer copy").contains("fixed byte credit"));
        assert_eq!(plus_one.take_partial(), "", "maximum plus one returns its empty pre-copy owner");
        assert!(plus_one.terminal_is_empty());

        let mut malformed = Process3dRetainedStringCursor::with_maximum_bytes(3);
        assert_eq!(grant(&mut malformed, 1).expect("malformed length admission"), None);
        assert!(grant(&mut malformed, 0xff).expect_err("malformed UTF-8 must fail at its byte boundary").contains("utf-8"));
        assert!(malformed.terminal_is_empty());

        let mut truncated = Process3dRetainedStringCursor::with_maximum_bytes(3);
        assert_eq!(grant(&mut truncated, 2).expect("truncated length admission"), None);
        assert_eq!(grant(&mut truncated, b'x').expect("truncated first byte"), None);
        assert!(truncated.step(&mut store::ByteReader::new(&[])).is_err(), "truncation must remain a resumable read failure");
        assert_eq!(truncated.take_partial(), "x", "interrupted string bytes return through the exact handback owner");
        assert!(truncated.terminal_is_empty());

        let mut overflowing_length = Process3dRetainedStringCursor::with_maximum_bytes(3);
        for _ in 0..(usize::BITS / 7) {
            assert_eq!(grant(&mut overflowing_length, 0x80).expect("bounded length byte"), None);
        }
        assert!(grant(&mut overflowing_length, 0x80).is_err(), "overlong retained length must fail without payload allocation");
        assert_eq!(overflowing_length.take_partial(), "");
        assert!(overflowing_length.terminal_is_empty());
    }

    #[test]
    fn mounted_snapshot_region_has_zero_whole_string_reader_edges() {
        let source = include_str!("🦀️.rs");
        let retained = source.split_once("//#region 🔖️RetainedStructuralReader").expect("retained snapshot region start").1.split_once("//#endregion 🔖️RetainedStructuralReader").expect("retained snapshot region end").0;
        assert_eq!(retained.matches(concat!("read_str", "_lp")).count(), 0, "mounted snapshot reader must have no whole-string edge");
    }

    #[test]
    fn retained_snapshot_reader_yields_between_structural_fields_and_hands_off_exactly_once() {
        let expected = complete_snapshot();
        let bytes = retained_pack(&expected);
        let operation = semio_framework_job::OperationId(u64::MAX - 81);
        let generation = semio_framework_job::Generation(17);
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut reader = Process3dRetainedSnapshotReader::new(8_192);
        let mut grants = 0;
        loop {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            grants += 1;
            if reader.step(&bytes, &mut context).expect("retained structural step") {
                break;
            }
            assert_eq!(context.fuel_remaining(), 0);
            assert!(grants < 8_192, "retained structural cursor must converge");
        }
        assert!(grants >= 256, "snapshot fields, string bytes, and nested items must not collapse into one whole decode grant");
        assert_eq!(reader.take(), Some(expected));
        assert!(reader.take().is_none(), "retained snapshot handoff is exact");
        assert!(reader.terminal_is_empty());
    }

    #[test]
    fn deepest_nested_snapshot_cursor_closes_without_populated_drop() {
        let expected = complete_snapshot();
        let bytes = retained_pack(&expected);
        let operation = semio_framework_job::OperationId(u64::MAX - 82);
        let generation = semio_framework_job::Generation(18);
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut reader = Process3dRetainedSnapshotReader::new(8_192);
        for _ in 0..96 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if reader.step(&bytes, &mut context).expect("nested retained structural step") {
                break;
            }
        }
        let partial = reader.take_rejected().expect("partial snapshot handback");
        assert!(reader.terminal_is_empty());
        drop(reader);
        let mut retirement = store::SnapshotRetirementFactory::retire(&crate::artifacts::process3d::spr::Process3dSnapshotRetirementFactory, std::sync::Arc::new(partial));
        for _ in 0..8_192 {
            if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                break;
            }
        }
        assert!(retirement.terminal_is_empty());
        drop(retirement);
    }

    #[test]
    fn every_machine_capability_parameter_and_rule_substate_interrupts_into_exact_retirement() {
        use Process3dRetainedSnapshotPhase::*;
        let phases = [
            MachineId,
            MachineLabel,
            MachineIcon,
            MachineCatalogTag,
            MachineCatalogId,
            MachineCapabilityCount,
            CapabilityId,
            CapabilityLabel,
            CapabilityIcon,
            CapabilityRecipeTag,
            CapabilityRecipeField(0),
            CapabilityRecipeField(1),
            CapabilityRecipeField(2),
            CapabilityParameterCount,
            ParameterId,
            ParameterLabel,
            ParameterValue,
            CapabilityRuleCount,
            RuleTag,
            RuleQuantity,
            RuleParameter,
            RuleMargin,
            MachineComplete,
        ];
        let bytes = retained_pack(&complete_snapshot());
        for (index, target) in phases.into_iter().enumerate() {
            let operation = semio_framework_job::OperationId(u64::MAX - 1_000 - index as u64);
            let generation = semio_framework_job::Generation(100 + index as u64);
            let cancel = semio_framework_job::CancelToken::root_now();
            let mut preview_sequence = 0;
            let mut reader = Process3dRetainedSnapshotReader::new(8_192);
            for _ in 0..8_192 {
                if reader.phase == target {
                    break;
                }
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!reader.step(&bytes, &mut context).expect("drive to retained snapshot substate"), "target substate must occur before completion");
            }
            assert_eq!(reader.phase, target, "every catalogued nested substate must be reachable");
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut preview_sequence);
            assert!(!reader.step(&bytes, &mut context).expect("interrupt one retained snapshot substate"));
            assert_eq!(context.fuel_remaining(), 0, "one substate consumes exactly one grant");
            let partial = reader.take_rejected().expect("interrupted substate exact handback");
            assert!(reader.terminal_is_empty());
            drop(reader);
            let mut retirement = store::SnapshotRetirementFactory::retire(&crate::artifacts::process3d::spr::Process3dSnapshotRetirementFactory, std::sync::Arc::new(partial));
            for _ in 0..8_192 {
                if matches!(retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                    break;
                }
            }
            assert!(retirement.terminal_is_empty(), "interrupted nested substate must close incrementally to terminal-empty");
            drop(retirement);
        }
    }
}
//#endregion 🔖️RetainedStructuralReader

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Real hex/bracket text + LEB128 binary primitives — same upgrade `✳️object`/`✳️kit`/`📐️cad`
/// made when they gained real `ArtifactChild<S>` slots (the old `dsl::DslRecord`-derive-driven
/// `Self::__dsl_spec()` path cannot express a composed child slot).
impl store::ArtifactDsl for Process3dSnapshot {
    const EXTENSION: &'static str = "process3d";
    fn envelope_id() -> &'static str {
        "process.process3d"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_process3d_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_process3d_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Process3dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_process3d_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_process3d_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `process3d_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn process3d_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <Process3dSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <Process3dSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <Process3dSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <Process3dSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <Process3dSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_os_kernel::json::object([
        ("parsed".to_string(), semio_framework_os_kernel::json::from_dsl_value(&ToValue::to_value(&parsed))),
        ("reparsed".to_string(), semio_framework_os_kernel::json::from_dsl_value(&ToValue::to_value(&reparsed))),
        ("packDecoded".to_string(), semio_framework_os_kernel::json::from_dsl_value(&ToValue::to_value(&unpacked))),
        ("canonicalText".to_string(), semio_framework_os_kernel::json::Value::String(canonical.clone())),
        ("canonicalTextAgain".to_string(), semio_framework_os_kernel::json::Value::String(canonical_again.clone())),
    ]);
    Ok(semio_framework_os_kernel::json::to_string(&report))
}
//#endregion 🌉️IdentityBridge
