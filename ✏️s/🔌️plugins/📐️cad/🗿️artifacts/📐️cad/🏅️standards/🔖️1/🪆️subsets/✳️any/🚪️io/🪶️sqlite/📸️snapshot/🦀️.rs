//! 🏗️ Complete CAD relational ownership under the actual caller ledger.
use crate::{CadSnapshot, CadReference, CadNode, CadModelChild, CadDrawingChild};
use crate::CadReferenceIndex;
use crate::standards::v1::subsets::any::schema::references::{close, invalid};
use crate::standards::v1::subsets::any::schema::references::frontiers as semantic_frontiers;
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_value::{NativeDecodeControl, ValueError, ValueRefusalKind};
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding, validate_sqlite_database_schema_controlled, artifact::{Projection, Cell, FloatColumn, insert_ieee754, read_binary64, ieee754_is_null}}};

const NUMBERS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15), FloatColumn::Binary64(18)];


fn slots(value: &CadSnapshot) -> [(&'static str, Option<&CadModelChild>); 4] {
    [("shapeModel", value.shape_model.as_ref()), ("buildingModel", value.building_model.as_ref()), ("energyModel", value.energy_model.as_ref()), ("structureClassicModel", value.structure_classic_model.as_ref())]
}
fn ordinal(value: usize) -> Result<Cell<'static>, ValueError> { i64::try_from(value).map(Cell::Integer).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit, "CAD ordinal exceeds signed SQL identity")) }
fn optional(value: Option<f64>) -> Cell<'static> { value.map(Cell::Real).unwrap_or(Cell::Null) }
fn validate_child<S>(value: &store::ArtifactChild<S>, subset: &str) -> Result<(), ValueError> {
    let dialect = &value.target.dialect;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != subset { return Err(invalid("CAD composed child has another model/drawing/topology domain")); }
    Ok(())
}
fn child_cells<S>(value: &store::ArtifactChild<S>) -> [Cell<'_>; 5] {
    let target = &value.target;
    let dialect = &target.dialect;
    [Cell::Text(&value.child_id), Cell::Text(&target.artifact_id), Cell::Text(&dialect.artifact_kind), Cell::Text(&dialect.standard), Cell::Text(&dialect.subset)]
}
fn forecast(value: &CadSnapshot, mut checkpoint: impl FnMut(usize) -> Result<(), ValueError>) -> Result<usize, ValueError> {
    let mut count = 1usize;
    for size in [value.drawings.len(), value.breps.len(), value.nodes.len(), slots(value).iter().filter(|(_, child)| child.is_some()).count(), value.references_by_model_definition_id.len()] {
        count = count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "CAD semantic row count overflow"))?;
    }
    checkpoint(count)?;
    for rows in value.references_by_model_definition_id.values() {
        count = count.checked_add(rows.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "CAD reference row count overflow"))?;
        checkpoint(count)?;
    }
    Ok(count)
}
trait Rows {
    fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>;
    fn insert_reference(&mut self,cells:&[Cell<'_>])->Result<(),ValueError>;
    fn check_rows(&self,count:usize)->Result<(),ValueError>;
    fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>;
}
impl Rows for Projection<'_,'_>{
    fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{Projection::insert(self,table,cells)}
    fn insert_reference(&mut self,cells:&[Cell<'_>])->Result<(),ValueError>{insert_ieee754(self,"cad_reference",cells,NUMBERS).map(|_|())}
    fn check_rows(&self,count:usize)->Result<(),ValueError>{Projection::check_rows(self,count)}
    fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{Projection::checkpoint_total(self,total)}
}
struct Census<'c,'p>{control:&'c mut SqliteSnapshotControl<'p>,count:usize,bytes:usize,phase:SqliteSnapshotPhase}
impl Census<'_,'_>{
    fn row(&mut self,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
        let mut bytes=8usize;
        for cell in cells{let size=match cell{Cell::Null=>0,Cell::Integer(_)|Cell::Real(_)|Cell::Float32(_)=>8,Cell::Text(value)=>value.len(),Cell::PagedText(value)=>value.text_bytes(),Cell::Blob(value)=>value.len()};bytes=bytes.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CAD semantic cell byte count overflow"))?;}
        for column in columns{let index=match column{FloatColumn::Binary64(index)|FloatColumn::Binary32(index)=>*index};let cell=cells.get(index.checked_sub(1).ok_or_else(||invalid("CAD IEEE identity column is invalid"))?).ok_or_else(||invalid("CAD IEEE field is missing"))?;let value=match cell{Cell::Null=>continue,Cell::Real(value)=>*value,_=>return Err(invalid("CAD IEEE field has an invalid storage class"))};let class=if value.is_nan(){bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};bytes=bytes.checked_add(8+class.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CAD IEEE semantic byte count overflow"))?;}
        let total=self.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"CAD semantic byte count overflow"))?;self.control.check_value_bytes(total)?;let count=self.count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CAD row count overflow"))?;self.control.check_rows(count)?;self.bytes=total;self.count=count;i64::try_from(count).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"CAD row identity exceeds i64"))
    }
}
impl Rows for Census<'_,'_>{
    fn insert(&mut self,_table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{self.row(cells,&[])}
    fn insert_reference(&mut self,cells:&[Cell<'_>])->Result<(),ValueError>{self.row(cells,NUMBERS).map(|_|())}
    fn check_rows(&self,count:usize)->Result<(),ValueError>{self.control.check_rows(count)}
    fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{self.control.checkpoint(self.phase,self.count,total)}
}
fn semantic(value:&CadSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
    control.checkpoint(phase,0,0)?;let mut output=Census{control,count:0,bytes:0,phase};write_rows(value,&mut output)
}
fn write_rows(value:&CadSnapshot,output:&mut impl Rows)->Result<(),ValueError>{
    let total = forecast(value, |count| output.check_rows(count))?;
    output.checkpoint_total(total)?;
    output.insert("cad_document", &[Cell::Text(&value.schema), Cell::Text(&value.id)])?;
    for (slot, child) in slots(value) {
        if let Some(child) = child {
            validate_child(child, "model")?;
            let cells = child_cells(child);
            output.insert("cad_model_child", &[Cell::Integer(1), Cell::Text(slot), cells[0], cells[1], cells[2], cells[3], cells[4]])?;
            output.checkpoint_total(total)?;
        }
    }
    for (index, child) in value.drawings.iter().enumerate() {
        validate_child(child, "drawing")?;
        let cells = child_cells(child);
        output.insert("cad_drawing_child", &[Cell::Integer(1), ordinal(index)?, cells[0], cells[1], cells[2], cells[3], cells[4]])?;
        output.checkpoint_total(total)?;
    }
    for (index, child) in value.breps.iter().enumerate() {
        validate_child(child, "brep")?;
        let cells = child_cells(child);
        output.insert("cad_brep_child", &[Cell::Integer(1), ordinal(index)?, cells[0], cells[1], cells[2], cells[3], cells[4]])?;
        output.checkpoint_total(total)?;
    }
    for (key, rows) in &value.references_by_model_definition_id {
        let group = output.insert("cad_reference_group", &[Cell::Integer(1), Cell::Text(key)])?;
        output.checkpoint_total(total)?;
        for (index, row) in rows.iter().enumerate() {
            let orientation = row.orientation.map(|values| values.map(|value| optional(Some(value)))).unwrap_or([Cell::Null; 4]);
            let cells = [Cell::Integer(group), ordinal(index)?, Cell::Text(&row.id), Cell::Text(&row.source_url), Cell::Text(&row.media_kind), Cell::Real(row.origin[0]), Cell::Real(row.origin[1]), Cell::Real(row.origin[2]), Cell::Integer(i64::from(row.orientation.is_some())), orientation[0], orientation[1], orientation[2], orientation[3], optional(row.scale), Cell::Real(row.width_world), Cell::Integer(i64::from(row.hidden)), Cell::Integer(i64::from(row.locked)), optional(row.opacity)];
            output.insert_reference(&cells)?;
            output.checkpoint_total(total)?;
        }
    }
    for (index, row) in value.nodes.iter().enumerate() {
        output.insert("cad_node", &[Cell::Integer(1), ordinal(index)?, Cell::Text(&row.id), Cell::Text(&row.label), Cell::Text(&row.kind)])?;
        output.checkpoint_total(total)?;
    }
    Ok(())
}
fn project(value:&CadSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
    let mut output=Projection::new(<CadSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,control)?;write_rows(value,&mut output)?;output.finish()
}
fn flag(row: &SqliteRow, index: usize) -> Result<bool, ValueError> {
    match row.integer(index)? { 0 => Ok(false), 1 => Ok(true), _ => Err(invalid("CAD boolean is undeclared")) }
}
fn optional_number(row: &SqliteRow, index: usize) -> Result<Option<f64>, ValueError> {
    if ieee754_is_null(row, index, NUMBERS)? { Ok(None) } else { read_binary64(row, index, NUMBERS).map(Some) }
}
fn child<S: Send + 'static>(row: &SqliteRow, start: usize, subset: &str, control: &mut NativeDecodeControl<'_>) -> Result<store::ArtifactChild<S>, ValueError> {
    let mut value = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef {
        artifact_id: String::new(),
        dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() },
    }), close::<store::ArtifactChild<S>>);
    value.as_mut().child_id = control.copy_text(row.text(start)?)?;
    value.as_mut().target.artifact_id = control.copy_text(row.text(start + 1)?)?;
    value.as_mut().target.dialect.artifact_kind = control.copy_text(row.text(start + 2)?)?;
    value.as_mut().target.dialect.standard = control.copy_text(row.text(start + 3)?)?;
    value.as_mut().target.dialect.subset = control.copy_text(row.text(start + 4)?)?;
    validate_child(value.as_mut(), subset)?;
    Ok(value.take())
}
fn reference(row: &SqliteRow, control: &mut NativeDecodeControl<'_>) -> Result<CadReference, ValueError> {
    let present = flag(row, 9)?;
    let orientation = [optional_number(row, 10)?, optional_number(row, 11)?, optional_number(row, 12)?, optional_number(row, 13)?];
    if orientation.iter().any(|value| value.is_some() != present) { return Err(invalid("CAD orientation presence must match all four numeric fields")); }
    let mut value = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(CadReference {
        id: String::new(), source_url: String::new(), media_kind: String::new(),
        origin: [read_binary64(row, 6, NUMBERS)?, read_binary64(row, 7, NUMBERS)?, read_binary64(row, 8, NUMBERS)?],
        orientation: if present { Some([orientation[0].unwrap(), orientation[1].unwrap(), orientation[2].unwrap(), orientation[3].unwrap()]) } else { None },
        scale: optional_number(row, 14)?, width_world: read_binary64(row, 15, NUMBERS)?,
        hidden: flag(row, 16)?, locked: flag(row, 17)?, opacity: optional_number(row, 18)?,
    }, close::<CadReference>);
    value.as_mut().id = control.copy_text(row.text(3)?)?;
    value.as_mut().source_url = control.copy_text(row.text(4)?)?;
    value.as_mut().media_kind = control.copy_text(row.text(5)?)?;
    Ok(value.take())
}
fn reconstruct(database: &SqliteDatabase, control: &mut NativeDecodeControl<'_>) -> Result<CadSnapshot, ValueError> {
    let documents = frontiers::keyed(&database.table("cad_document")?.rows, 3, None, control)?;
    if documents.len() != 1 || documents[0].rowid != 1 { return Err(invalid("CAD requires one document")); }
    let models = frontiers::keyed(&database.table("cad_model_child")?.rows, 8, Some(1), control)?;
    let drawings = frontiers::keyed(&database.table("cad_drawing_child")?.rows, 8, Some(1), control)?;
    let breps = frontiers::keyed(&database.table("cad_brep_child")?.rows, 8, Some(1), control)?;
    let groups = frontiers::keyed(&database.table("cad_reference_group")?.rows, 3, Some(1), control)?;
    let references = frontiers::keyed(&database.table("cad_reference")?.rows, 39, None, control)?;
    let nodes = frontiers::keyed(&database.table("cad_node")?.rows, 6, Some(1), control)?;
    let mut value = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(CadSnapshot {
        schema: String::new(), id: String::new(),
        shape_model: None, building_model: None, energy_model: None, structure_classic_model: None,
        drawings: Vec::new(), breps: Vec::new(), references_by_model_definition_id: CadReferenceIndex::new(), nodes: Vec::new(),
    }, close::<CadSnapshot>);
    value.as_mut().schema = control.copy_text(documents[0].text(1)?)?;
    value.as_mut().id = control.copy_text(documents[0].text(2)?)?;
    let mut occupied = [false; 4];
    control.begin_stage(models.len())?;
    for row in models {
        let slot = match row.text(2)? { "shapeModel" => 0, "buildingModel" => 1, "energyModel" => 2, "structureClassicModel" => 3, _ => return Err(invalid("CAD model slot is undeclared")) };
        if std::mem::replace(&mut occupied[slot], true) { return Err(invalid("CAD model slots must be unique")); }
        let owned: CadModelChild = control.scoped_stage(|control| child(row, 3, "model", control))?;
        match slot { 0 => value.as_mut().shape_model = Some(owned), 1 => value.as_mut().building_model = Some(owned), 2 => value.as_mut().energy_model = Some(owned), 3 => value.as_mut().structure_classic_model = Some(owned), _ => unreachable!() }
        control.step()?;
    }
    let drawings = frontiers::dense(&drawings, control)?;
    value.as_mut().drawings = semantic_frontiers::collect(drawings.len(), control, |index, control| child(drawings[index], 3, "drawing", control))?;
    let breps = frontiers::dense(&breps, control)?;
    value.as_mut().breps = semantic_frontiers::collect(breps.len(), control, |index, control| child(breps[index], 3, "brep", control))?;
    let references = frontiers::references(&references, &groups, control)?;
    let mut index = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(CadReferenceIndex::owned_slots(groups.len(), control)?, close::<CadReferenceIndex>);
    let mut position = 0;
    control.begin_stage(groups.len())?;
    for group in groups {
        let key = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(group.text(2)?)?, close::<String>);
        let start = position;
        while position < references.len() && references[position].integer(1)? == group.rowid { position += 1; control.scoped_stage(|control| { control.begin_stage(1)?; control.step() })?; }
        let rows = semantic_frontiers::collect(position - start, control, |offset, control| reference(references[start + offset], control))?;
        index.as_mut().append(key.take(), rows)?;
        control.step()?;
    }
    if position != references.len() { return Err(invalid("CAD has unconsumed references")); }
    value.as_mut().references_by_model_definition_id = index.take().finish(control)?;
    let nodes = frontiers::dense(&nodes, control)?;
    value.as_mut().nodes = semantic_frontiers::collect(nodes.len(), control, |index, control| {
        let row = nodes[index];
        let mut node = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(CadNode { id: String::new(), label: String::new(), kind: String::new() }, close::<CadNode>);
        node.as_mut().id = control.copy_text(row.text(3)?)?;
        node.as_mut().label = control.copy_text(row.text(4)?)?;
        node.as_mut().kind = control.copy_text(row.text(5)?)?;
        Ok(node.take())
    })?;
    Ok(value.take())
}
fn restore(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<CadSnapshot, ValueError> {
    validate_sqlite_database_schema_controlled(database, <CadSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA, SqliteSnapshotPhase::ReconstructSnapshot, control)?;
    control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
    control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot, |remaining, callback| {
        let mut progress = |event: semio_framework_value::native_decoding::NativeDecodeProgress| callback(event.completed, event.total);
        let mut native = NativeDecodeControl::new(remaining, &mut progress);
        let result = reconstruct(database, &mut native);
        (result, native.owned_bytes())
    })?
}
fn native_list(value: Option<&semio_framework_dsl_record::FieldValue>) -> Result<&[semio_framework_dsl_record::FieldValue],ValueError> {
    match value { None | Some(semio_framework_dsl_record::FieldValue::Absent) => Ok(&[]), Some(semio_framework_dsl_record::FieldValue::List(value)) => Ok(value), _ => Err(invalid("CAD requires an ordered native collection")) }
}
fn native_rows(record: &semio_framework_dsl_record::RecordValue, native: &mut NativeDecodeControl<'_>, maximum: usize) -> Result<(),ValueError> {
    let mut count = 1usize;
    let mut add = |size: usize| {
        count = count.checked_add(size).filter(|count| *count <= maximum).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "CAD native semantic rows exceed caller limit"))?;
        Ok::<(),ValueError>(())
    };
    for id in [2, 3, 4, 5] { if !matches!(record.fields.get(&id), None | Some(semio_framework_dsl_record::FieldValue::Absent)) { add(1)?; } }
    add(native_list(record.fields.get(&6))?.len())?;
    add(native_list(record.fields.get(&7))?.len())?;
    add(native_list(record.fields.get(&9))?.len())?;
    let groups = match record.fields.get(&8) { None | Some(semio_framework_dsl_record::FieldValue::Absent) => &[][..], Some(semio_framework_dsl_record::FieldValue::Map(groups)) => groups.as_slice(), _ => return Err(invalid("CAD requires its literal native reference map")) };
    add(groups.len())?;
    native.scoped_stage(|native| {
        native.begin_stage(groups.len())?;
        for (_, value) in groups { add(native_list(Some(value))?.len())?; native.step()?; }
        native.checkpoint()
    })
}
fn validate_owned(value: &CadSnapshot, mut checkpoint: impl FnMut() -> Result<(), ValueError>) -> Result<(), ValueError> {
    checkpoint()?;
    for (_, child) in slots(value) { if let Some(child) = child { validate_child(child, "model")?; } }
    for child in &value.drawings { checkpoint()?; validate_child(child, "drawing")?; }
    for child in &value.breps { checkpoint()?; validate_child(child, "brep")?; }
    checkpoint()
}
impl ArtifactSqliteSnapshot for CadSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn retire_sqlite_snapshot(self) { close(self); }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { project(self, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> { restore(database, control) }
    fn decode_sqlite_snapshot_native(payload: &store::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        control.check_rows(1)?;
        let maximum = control.limits().max_rows;
        let value=store::decode_sqlite_snapshot_record_native(payload, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |record, native| {
            native_rows(record, native, maximum)?;
            let mut value = semio_framework_dsl_record::__rt::DecodedFieldOwner::new(Self::__dsl_from_record_controlled(record, native)?, close::<Self>);
            validate_owned(value.as_mut(), || native.scoped_stage(|native| { native.begin_stage(1)?; native.step() }))?;
            Ok(value.take())
        }, control)?;
        let mut value=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,close::<Self>);
        semantic(value.as_mut(),control,SqliteSnapshotPhase::DecodeNative)?;Ok(value.take())
    }
    fn encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<store::io_schema::IoPayload, ValueError> {
        control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0)?;
        forecast(self, |count| control.check_rows(count))?;
        validate_owned(self, || control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0))?;
        semantic(self,control,SqliteSnapshotPhase::EncodeNative)?;
        store::encode_sqlite_snapshot_record_native(encoding, <Self as store::ArtifactDsl>::envelope_id(), Self::__dsl_spec_producer(), |native| self.__dsl_to_record_controlled(native), control)
    }
}
semio_framework_value::artifact_retire_struct!(crate::CadNode { id, label, kind });
semio_framework_value::artifact_retire_struct!(crate::CadSnapshot { schema, id, shape_model, building_model, energy_model, structure_classic_model, drawings, breps, references_by_model_definition_id, nodes });
semio_framework_value::artifact_retire_struct!(crate::schema::CadArtifact { schema, id, shape_model, building_model, energy_model, structure_classic_model, drawings, breps, references_by_model_definition_id, nodes });


#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path="🚦️frontiers/🦀️.rs"]
pub(crate) mod frontiers;
