//! 🏭️ Explicit Process3d parent projection into its authored semantic entities.
use crate::standards::v1::subsets::any::schema::snapshot::Process3dSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::{CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, StockQuantity, WorkingSolid};
#[path="🔍️rows/🦀️.rs"]
mod rows;
#[path="🏗️ownership/🦀️.rs"]
mod ownership;
#[path="🛂️capability/🦀️.rs"]
mod capability;
use rows::Rows;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
use store::sqlite_snapshot::{
    artifact::{insert_ieee754, read_binary64, Cell, FloatColumn, Projection, Reconstruction},
    SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("🗄️.sql");
const TABLES: [(&str, usize); 32] = [
    ("process3_document", 3),
    ("process3_stock_pose", 23),
    ("process3_stock_payload", 4),
    ("process3_stock_payload_pose", 23),
    ("process3_solid", 2),
    ("process3_solid_box", 11),
    ("process3_solid_cylinder", 8),
    ("process3_solid_sphere", 5),
    ("process3_solid_imported_mesh", 3),
    ("process3_solid_imported_solid", 3),
    ("process3_solid_reference", 3),
    ("process3_stock_payload_solid", 3),
    ("process3_machine", 7),
    ("process3_capability", 7),
    ("process3_recipe_disc_cut", 4),
    ("process3_recipe_blade_cut", 5),
    ("process3_recipe_pocket_cut", 4),
    ("process3_recipe_bore_drill", 4),
    ("process3_recipe_cylinder_attach", 4),
    ("process3_recipe_box_attach", 5),
    ("process3_parameter", 8),
    ("process3_rule", 9),
    ("process3_step", 6),
    ("process3_step_origin", 4),
    ("process3_measure", 3),
    ("process3_measure_cut", 3),
    ("process3_measure_drill", 8),
    ("process3_measure_attach", 3),
    ("process3_measure_pose", 23),
    ("process3_stock_solid_child", 7),
    ("process3_steps_child", 7),
    ("process3_tool_solid_child", 8),
];
const POSE: [FloatColumn; 7] = [FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];
const THREE: [FloatColumn; 3] = [FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4)];
const TWO: [FloatColumn; 2] = [FloatColumn::Binary64(2), FloatColumn::Binary64(3)];
const ONE: [FloatColumn; 1] = [FloatColumn::Binary64(2)];
const PARAMETER: [FloatColumn; 1] = [FloatColumn::Binary64(5)];
const RULE: [FloatColumn; 1] = [FloatColumn::Binary64(6)];
fn columns(table: usize) -> &'static [FloatColumn] {
    match table {
        1 | 3 | 28 => &POSE,
        5 => &THREE,
        6 | 26 => &TWO,
        7 => &ONE,
        20 => &PARAMETER,
        21 => &RULE,
        _ => &[],
    }
}
fn ordinal(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Process3d ordinal overflow"))
}
fn optional(value: &Option<String>) -> Cell<'_> {
    value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)
}
fn recipe_variant(value: &MeasureRecipe) -> &'static str {
    match value {
        MeasureRecipe::DiscCut { .. } => "discCut",
        MeasureRecipe::BladeCut { .. } => "bladeCut",
        MeasureRecipe::PocketCut { .. } => "pocketCut",
        MeasureRecipe::BoreDrill { .. } => "boreDrill",
        MeasureRecipe::CylinderAttach { .. } => "cylinderAttach",
        MeasureRecipe::BoxAttach { .. } => "boxAttach",
    }
}
fn quantity(value: StockQuantity) -> &'static str {
    match value {
        StockQuantity::Width => "width",
        StockQuantity::Depth => "depth",
        StockQuantity::Height => "height",
        StockQuantity::MaxDimension => "maxDimension",
        StockQuantity::MinDimension => "minDimension",
    }
}
pub(super) fn forecast(value: &Process3dSnapshot, c: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<usize, ValueError> {
    c.checkpoint(phase, 0, 0)?;
    let limits = c.limits();
    if limits.max_tables < TABLES.len() || limits.max_columns < 23 || SQL.len() + TABLES.iter().map(|(name, _)| name.len()).sum::<usize>() > limits.max_schema_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Process3d schema limit"));
    }
    let mut total = 9usize;
    let mut units = 0usize;
    let mut add = |count: usize| {
        total = total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Process3d row count overflow"))?;
        c.check_rows(total)?;
        units = units.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Process3d forecast work overflow"))?;
        if units % 256 == 0 {
            c.checkpoint(phase, units, 0)?;
        }
        Ok::<(), ValueError>(())
    };
    add(value.tool_solids.len())?;
    for machine in &value.workshop.machines {
        add(1)?;
        for capability in &machine.capabilities {
            add(2)?;
            add(capability.parameters.len())?;
            add(capability.rules.len())?;
        }
    }
    for step in &value.step_payloads {
        add(4)?;
        add(usize::from(step.origin.is_some()))?;
        if !matches!(&step.measure, ProcessMeasure::Drill { .. }) {
            add(2)?;
        }
    }
    c.checkpoint(phase, units, units)?;
    Ok(total)
}
fn put(out: &mut Projection<'_, '_>, table: usize, cells: &[Cell<'_>]) -> Result<i64, ValueError> {
    if columns(table).is_empty() {
        out.insert(TABLES[table].0, cells)
    } else {
        insert_ieee754(out, TABLES[table].0, cells, columns(table))
    }
}
fn pose(out: &mut Projection<'_, '_>, table: usize, owner: i64, value: &Pose) -> Result<(), ValueError> {
    put(out, table, &[Cell::Integer(owner), Cell::Real(value.position[0]), Cell::Real(value.position[1]), Cell::Real(value.position[2]), Cell::Real(value.axis[0]), Cell::Real(value.axis[1]), Cell::Real(value.axis[2]), Cell::Real(value.angle)])?;
    Ok(())
}
fn child<S>(out: &mut Projection<'_, '_>, table: usize, owner: i64, value: &store::ArtifactChild<S>, order: Option<usize>) -> Result<(), ValueError> {
    let target = &value.target;
    let fields = [Cell::Text(&value.child_id), Cell::Text(&target.artifact_id), Cell::Text(&target.dialect.artifact_kind), Cell::Text(&target.dialect.standard), Cell::Text(&target.dialect.subset)];
    match order {
        None => {
            put(out, table, &[Cell::Integer(owner), fields[0], fields[1], fields[2], fields[3], fields[4]])?;
        }
        Some(order) => {
            put(out, table, &[Cell::Integer(owner), Cell::Integer(ordinal(order)?), fields[0], fields[1], fields[2], fields[3], fields[4]])?;
        }
    }
    Ok(())
}
fn solid(out: &mut Projection<'_, '_>, value: &WorkingSolid) -> Result<i64, ValueError> {
    let variant = match value {
        WorkingSolid::Box { .. } => "box",
        WorkingSolid::Cylinder { .. } => "cylinder",
        WorkingSolid::Sphere { .. } => "sphere",
        WorkingSolid::ImportedMesh { .. } => "importedMesh",
        WorkingSolid::ImportedSolid { .. } => "importedSolid",
        WorkingSolid::Reference { .. } => "reference",
    };
    let id = put(out, 4, &[Cell::Text(variant)])?;
    match value {
        WorkingSolid::Box { width, depth, height } => {
            put(out, 5, &[Cell::Integer(id), Cell::Real(*width), Cell::Real(*depth), Cell::Real(*height)])?;
        }
        WorkingSolid::Cylinder { radius, height } => {
            put(out, 6, &[Cell::Integer(id), Cell::Real(*radius), Cell::Real(*height)])?;
        }
        WorkingSolid::Sphere { radius } => {
            put(out, 7, &[Cell::Integer(id), Cell::Real(*radius)])?;
        }
        WorkingSolid::ImportedMesh { mesh_url } => {
            put(out, 8, &[Cell::Integer(id), Cell::Text(mesh_url)])?;
        }
        WorkingSolid::ImportedSolid { solid_handle } => {
            put(out, 9, &[Cell::Integer(id), Cell::Text(solid_handle)])?;
        }
        WorkingSolid::Reference { reference_id } => {
            put(out, 10, &[Cell::Integer(id), Cell::Text(reference_id)])?;
        }
    }
    Ok(id)
}
fn recipe(out: &mut Projection<'_, '_>, owner: i64, value: &MeasureRecipe) -> Result<(), ValueError> {
    match value {
        MeasureRecipe::DiscCut { diameter, kerf } => {
            put(out, 14, &[Cell::Integer(owner), Cell::Text(diameter), Cell::Text(kerf)])?;
        }
        MeasureRecipe::BladeCut { kerf, length, depth } => {
            put(out, 15, &[Cell::Integer(owner), Cell::Text(kerf), Cell::Text(length), Cell::Text(depth)])?;
        }
        MeasureRecipe::PocketCut { diameter, depth } => {
            put(out, 16, &[Cell::Integer(owner), Cell::Text(diameter), Cell::Text(depth)])?;
        }
        MeasureRecipe::BoreDrill { radius, depth } => {
            put(out, 17, &[Cell::Integer(owner), Cell::Text(radius), Cell::Text(depth)])?;
        }
        MeasureRecipe::CylinderAttach { radius, length } => {
            put(out, 18, &[Cell::Integer(owner), Cell::Text(radius), Cell::Text(length)])?;
        }
        MeasureRecipe::BoxAttach { width, depth, height } => {
            put(out, 19, &[Cell::Integer(owner), Cell::Text(width), Cell::Text(depth), Cell::Text(height)])?;
        }
    }
    Ok(())
}
pub(super) fn project(value: &Process3dSnapshot, c: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let total = forecast(value, c, SqliteSnapshotPhase::ProjectSnapshot)?;
    let mut out = Projection::new(SQL, c)?;
    let doc = put(&mut out, 0, &[Cell::Text(&value.stock_id), Cell::Text(&value.stock_label)])?;
    pose(&mut out, 1, doc, &value.stock_pose)?;
    let stock = put(&mut out, 2, &[Cell::Integer(doc), Cell::Text(&value.stock_payload.id), Cell::Text(&value.stock_payload.label)])?;
    pose(&mut out, 3, stock, &value.stock_payload.pose)?;
    let stock_solid = solid(&mut out, &value.stock_payload.solid)?;
    put(&mut out, 11, &[Cell::Integer(stock), Cell::Integer(stock_solid)])?;
    child(&mut out, 29, doc, &value.stock_solid, None)?;
    child(&mut out, 30, doc, &value.steps, None)?;
    for (order, machine) in value.workshop.machines.iter().enumerate() {
        let id = put(&mut out, 12, &[Cell::Integer(doc), Cell::Integer(ordinal(order)?), Cell::Text(&machine.id), Cell::Text(&machine.label), Cell::Text(&machine.icon_id), optional(&machine.catalog_id)])?;
        for (order, capability) in machine.capabilities.iter().enumerate() {
            let cap = put(&mut out, 13, &[Cell::Integer(id), Cell::Integer(ordinal(order)?), Cell::Text(&capability.id), Cell::Text(&capability.label), Cell::Text(&capability.icon_id), Cell::Text(recipe_variant(&capability.recipe))])?;
            recipe(&mut out, cap, &capability.recipe)?;
            for (order, parameter) in capability.parameters.iter().enumerate() {
                put(&mut out, 20, &[Cell::Integer(cap), Cell::Integer(ordinal(order)?), Cell::Text(&parameter.id), Cell::Text(&parameter.label), Cell::Real(parameter.value)])?;
            }
            for (order, rule) in capability.rules.iter().enumerate() {
                let (variant, stock_quantity, parameter, margin) = match rule {
                    CapabilityRule::Min { quantity, parameter, margin } => ("min", *quantity, parameter, *margin),
                    CapabilityRule::Max { quantity, parameter, margin } => ("max", *quantity, parameter, *margin),
                };
                put(&mut out, 21, &[Cell::Integer(cap), Cell::Integer(ordinal(order)?), Cell::Text(variant), Cell::Text(quantity(stock_quantity)), Cell::Text(parameter), Cell::Real(margin)])?;
            }
        }
        out.checkpoint_total(total)?;
    }
    for (order, step) in value.step_payloads.iter().enumerate() {
        let id = put(&mut out, 22, &[Cell::Integer(doc), Cell::Integer(ordinal(order)?), Cell::Text(&step.id), Cell::Text(&step.label), Cell::Integer(i64::from(step.enabled))])?;
        if let Some(origin) = &step.origin {
            put(&mut out, 23, &[Cell::Integer(id), Cell::Text(&origin.machine_id), Cell::Text(&origin.capability_id)])?;
        }
        let measure = put(&mut out, 24, &[Cell::Integer(id), Cell::Text(step.measure.kind_slug())])?;
        pose(&mut out, 28, measure, step.measure.pose())?;
        match &step.measure {
            ProcessMeasure::Cut { tool, .. } => {
                let solid = solid(&mut out, tool)?;
                put(&mut out, 25, &[Cell::Integer(measure), Cell::Integer(solid)])?;
            }
            ProcessMeasure::Drill { radius, depth, .. } => {
                put(&mut out, 26, &[Cell::Integer(measure), Cell::Real(*radius), Cell::Real(*depth)])?;
            }
            ProcessMeasure::Attach { component, .. } => {
                let solid = solid(&mut out, component)?;
                put(&mut out, 27, &[Cell::Integer(measure), Cell::Integer(solid)])?;
            }
        }
        out.checkpoint_total(total)?;
    }
    for (order, tool) in value.tool_solids.iter().enumerate() {
        child(&mut out, 31, doc, tool, Some(order))?;
    }
    out.checkpoint_total(total)?;
    out.finish()
}

fn text(row: &SqliteRow, index: usize, c: &mut SqliteSnapshotControl<'_>) -> Result<String, ValueError> {
    Reconstruction::new(c)?.text(row.text(index)?)
}
fn read_pose(row: &SqliteRow, c: &mut SqliteSnapshotControl<'_>) -> Result<Pose, ValueError> {
    let mut scalar = |index| {
        Reconstruction::new(c)?.scalar()?;
        read_binary64(row, index, &POSE)
    };
    Ok(Pose { position: [scalar(2)?, scalar(3)?, scalar(4)?], axis: [scalar(5)?, scalar(6)?, scalar(7)?], angle: scalar(8)? })
}
fn scalar(row: &SqliteRow, index: usize, fields: &[FloatColumn], c: &mut SqliteSnapshotControl<'_>) -> Result<f64, ValueError> {
    Reconstruction::new(c)?.scalar()?;
    read_binary64(row, index, fields)
}
fn read_solid(id: i64, rows: &mut Rows<'_>, c: &mut SqliteSnapshotControl<'_>) -> Result<WorkingSolid, ValueError> {
    let root = rows.row(4, id, c)?;
    Ok(match root.text(1)? {
        "box" => {
            let row = rows.one(5, id, c)?;
            WorkingSolid::Box { width: scalar(row, 2, &THREE, c)?, depth: scalar(row, 3, &THREE, c)?, height: scalar(row, 4, &THREE, c)? }
        }
        "cylinder" => {
            let row = rows.one(6, id, c)?;
            WorkingSolid::Cylinder { radius: scalar(row, 2, &TWO, c)?, height: scalar(row, 3, &TWO, c)? }
        }
        "sphere" => {
            let row = rows.one(7, id, c)?;
            WorkingSolid::Sphere { radius: scalar(row, 2, &ONE, c)? }
        }
        "importedMesh" => WorkingSolid::ImportedMesh { mesh_url: text(rows.one(8, id, c)?, 2, c)? },
        "importedSolid" => WorkingSolid::ImportedSolid { solid_handle: text(rows.one(9, id, c)?, 2, c)? },
        "reference" => WorkingSolid::Reference { reference_id: text(rows.one(10, id, c)?, 2, c)? },
        _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d unknown working solid")),
    })
}
fn read_recipe(kind: &str, parent: i64, rows: &mut Rows<'_>, c: &mut SqliteSnapshotControl<'_>) -> Result<MeasureRecipe, ValueError> {
    Ok(match kind {
        "discCut" => {
            let row = rows.one(14, parent, c)?;
            MeasureRecipe::DiscCut { diameter: text(row, 2, c)?, kerf: text(row, 3, c)? }
        }
        "bladeCut" => {
            let row = rows.one(15, parent, c)?;
            MeasureRecipe::BladeCut { kerf: text(row, 2, c)?, length: text(row, 3, c)?, depth: text(row, 4, c)? }
        }
        "pocketCut" => {
            let row = rows.one(16, parent, c)?;
            MeasureRecipe::PocketCut { diameter: text(row, 2, c)?, depth: text(row, 3, c)? }
        }
        "boreDrill" => {
            let row = rows.one(17, parent, c)?;
            MeasureRecipe::BoreDrill { radius: text(row, 2, c)?, depth: text(row, 3, c)? }
        }
        "cylinderAttach" => {
            let row = rows.one(18, parent, c)?;
            MeasureRecipe::CylinderAttach { radius: text(row, 2, c)?, length: text(row, 3, c)? }
        }
        "boxAttach" => {
            let row = rows.one(19, parent, c)?;
            MeasureRecipe::BoxAttach { width: text(row, 2, c)?, depth: text(row, 3, c)?, height: text(row, 4, c)? }
        }
        _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d unknown recipe")),
    })
}
fn read_quantity(value: &str) -> Result<StockQuantity, ValueError> {
    match value {
        "width" => Ok(StockQuantity::Width),
        "depth" => Ok(StockQuantity::Depth),
        "height" => Ok(StockQuantity::Height),
        "maxDimension" => Ok(StockQuantity::MaxDimension),
        "minDimension" => Ok(StockQuantity::MinDimension),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d unknown stock quantity")),
    }
}
fn read_child<S>(row: &SqliteRow, ordered: bool, subset: &str, c: &mut SqliteSnapshotControl<'_>) -> Result<store::ArtifactChild<S>, ValueError> {
    let offset = usize::from(ordered);
    if row.text(4 + offset)? != "s.stdio.semio" || row.text(5 + offset)? != "v1" || row.text(6 + offset)? != subset {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d child dialect"));
    }
    Ok(store::ArtifactChild::new(
        text(row, 2 + offset, c)?,
        store::io_schema::ArtifactRef { artifact_id: text(row, 3 + offset, c)?, dialect: store::io_schema::ArtifactDialect { artifact_kind: text(row, 4 + offset, c)?, standard: text(row, 5 + offset, c)?, subset: text(row, 6 + offset, c)? } },
    ))
}
pub(super) fn reconstruct(database: &SqliteDatabase, c: &mut SqliteSnapshotControl<'_>) -> Result<Process3dSnapshot, ValueError> {
    let mut rows = Rows::new(database, c)?;
    let document = database.table(TABLES[0].0)?.single_row()?;
    rows.row(0, document.rowid, c)?;
    let stock_id = ownership::field(text(document, 1, c)?);
    let stock_label = ownership::field(text(document, 2, c)?);
    let stock_pose = read_pose(rows.one(1, document.rowid, c)?, c)?;
    let stock = rows.one(2, document.rowid, c)?;
    let id = ownership::field(text(stock, 2, c)?);
    let label = ownership::field(text(stock, 3, c)?);
    let pose = read_pose(rows.one(3, stock.rowid, c)?, c)?;
    let solid_id = rows.one(11, stock.rowid, c)?.integer(2)?;
    let stock_payload = ownership::field(crate::Stock { id: id.take(), label: label.take(), pose, solid: read_solid(solid_id, &mut rows, c)? });
    let machines_rows = rows.children(12, document.rowid, true, c)?;
    let mut machines = ownership::guarded_collection::<crate::WorkshopMachine>(machines_rows.len(), c)?;
    for machine in machines_rows {
        let id = ownership::field(text(machine, 3, c)?);
        let label = ownership::field(text(machine, 4, c)?);
        let icon_id = ownership::field(text(machine, 5, c)?);
        let catalog_id = ownership::optional(machine.optional_text(6)?.map(|value| Reconstruction::new(c)?.text(value)).transpose()?);
        let capabilities_rows = rows.children(13, machine.rowid, true, c)?;
        let mut capabilities = ownership::guarded_collection::<crate::Capability>(capabilities_rows.len(), c)?;
        for capability in capabilities_rows {
            let id = ownership::field(text(capability, 3, c)?);
            let label = ownership::field(text(capability, 4, c)?);
            let icon_id = ownership::field(text(capability, 5, c)?);
            let recipe = ownership::field(read_recipe(capability.text(6)?, capability.rowid, &mut rows, c)?);
            let parameters_rows = rows.children(20, capability.rowid, true, c)?;
            let mut parameters = ownership::guarded_collection::<crate::CapabilityParameter>(parameters_rows.len(), c)?;
            for row in parameters_rows {
                parameters.as_mut().push(crate::CapabilityParameter { id: text(row, 3, c)?, label: text(row, 4, c)?, value: scalar(row, 5, &PARAMETER, c)? });
            }
            let rule_rows = rows.children(21, capability.rowid, true, c)?;
            let mut rules = ownership::guarded_variants::<CapabilityRule>(rule_rows.len(), c)?;
            for row in rule_rows {
                let quantity = read_quantity(row.text(4)?)?;
                let parameter = ownership::field(text(row, 5, c)?);
                let margin = scalar(row, 6, &RULE, c)?;
                rules.as_mut().push(match row.text(3)? {
                    "min" => CapabilityRule::Min { quantity, parameter: parameter.take(), margin },
                    "max" => CapabilityRule::Max { quantity, parameter: parameter.take(), margin },
                    _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d unknown capability rule")),
                });
            }
            capabilities.as_mut().push(crate::Capability { id: id.take(), label: label.take(), icon_id: icon_id.take(), recipe: recipe.take(), parameters: parameters.take(), rules: rules.take() });
        }
        machines.as_mut().push(crate::WorkshopMachine { id: id.take(), label: label.take(), icon_id: icon_id.take(), catalog_id: catalog_id.take(), capabilities: capabilities.take() });
    }
    let step_payloads_rows = rows.children(22, document.rowid, true, c)?;
    let mut step_payloads = ownership::guarded_collection::<crate::ProcessStep>(step_payloads_rows.len(), c)?;
    for step in step_payloads_rows {
        let id = ownership::field(text(step, 3, c)?);
        let label = ownership::field(text(step, 4, c)?);
        Reconstruction::new(c)?.scalar()?;
        let enabled = match step.integer(5)? {
            0 => false,
            1 => true,
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d invalid enabled boolean")),
        };
        let origins = rows.children(23, step.rowid, false, c)?;
        let origin = ownership::optional(origins.first().map(|row| Ok::<_, ValueError>(crate::StepOrigin { machine_id: text(row, 2, c)?, capability_id: text(row, 3, c)? })).transpose()?);
        let measure = rows.one(24, step.rowid, c)?;
        let pose = read_pose(rows.one(28, measure.rowid, c)?, c)?;
        let measure = match measure.text(2)? {
            "cut" => {
                let id = rows.one(25, measure.rowid, c)?.integer(2)?;
                ProcessMeasure::Cut { tool: read_solid(id, &mut rows, c)?, pose }
            }
            "drill" => {
                let row = rows.one(26, measure.rowid, c)?;
                ProcessMeasure::Drill { radius: scalar(row, 2, &TWO, c)?, depth: scalar(row, 3, &TWO, c)?, pose }
            }
            "attach" => {
                let id = rows.one(27, measure.rowid, c)?.integer(2)?;
                ProcessMeasure::Attach { component: read_solid(id, &mut rows, c)?, pose }
            }
            _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Process3d unknown measure")),
        };
        step_payloads.as_mut().push(crate::ProcessStep { id: id.take(), label: label.take(), enabled, origin: origin.take(), measure });
    }
    let stock_solid = ownership::field(read_child(rows.one(29, document.rowid, c)?, false, "brep", c)?);
    let steps = ownership::field(read_child(rows.one(30, document.rowid, c)?, false, "flow", c)?);
    let tool_solids_rows = rows.children(31, document.rowid, true, c)?;
    let mut tool_solids = ownership::guarded_collection::<store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot>>(tool_solids_rows.len(), c)?;
    for row in tool_solids_rows {
        tool_solids.as_mut().push(read_child(row, true, "brep", c)?);
    }
    rows.finish()?;
    let owner = ownership::guarded(Process3dSnapshot { workshop: crate::Workshop { machines: machines.take() }, stock_id: stock_id.take(), stock_label: stock_label.take(), stock_pose, stock_payload: stock_payload.take(), stock_solid: stock_solid.take(), steps: steps.take(), step_payloads: step_payloads.take(), tool_solids: tool_solids.take() });
    Reconstruction::new(c)?.checkpoint()?;
    Ok(owner.take())
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

