//! 🏗️ IFC4 headers, ordered value ownership and exact entity references.
use super::{IfcSnapshot,IfcHeader,IfcEntity,IfcComplexType,IfcValue};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,FloatColumn},*}};
#[path="📏️encoding/💰️backing/🦀️.rs"]
mod native_backing;
#[path="🛬️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
const REAL:&[FloatColumn]=&[FloatColumn::Binary64(3)];
struct UnsignedWord{bytes:[u8;20],start:usize}
impl UnsignedWord{
 fn new(mut value:u64)->Self{let mut out=Self{bytes:[0;20],start:20};loop{out.start-=1;out.bytes[out.start]=b'0'+(value%10)as u8;value/=10;if value==0{return out;}}}
 fn text(&self)->&str{std::str::from_utf8(&self.bytes[self.start..]).expect("decimal digits are UTF-8")}
}
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 ordinal exceeds signed64"))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid IFC4 row identity or columns"))}else{Ok(())}}
fn unsigned(text:&str)->Result<u64,ValueError>{let value=text.parse::<u64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"IFC4 identity exceeds unsigned64"))?;if UnsignedWord::new(value).text()!=text{Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC4 identity must be canonical decimal text"))}else{Ok(value)}}
fn null(row:&SqliteRow,index:usize)->Result<(),ValueError>{if row.values.get(index)==Some(&SqliteValue::Null){Ok(())}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected IFC4 inactive value field"))}}
fn sort_paid<T>(values:&mut[T],mut compare:impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,compare:&mut impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{loop{let Some(child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let mut next=child;if child+1<end&&compare(&values[child],&values[child+1])?.is_lt(){next=child+1;}if !compare(&values[root],&values[next])?.is_lt(){return Ok(())}values.swap(root,next);root=next;}}
 for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut compare)?;}Ok(())
}
enum Owner{Header(&'static str,usize),Argument(Option<i64>,Option<i64>,usize),Aggregate(i64,usize),Typed(i64,usize)}
fn project_value(value:&IfcValue,owner:Owner,entities:&[(u64,i64)],projection:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let mut pending=projection.allocate_frontier(1)?;pending.push((value,owner));
 while let Some((value,owner))=pending.pop(){projection.checkpoint()?;let mut cells=[Cell::Null;8];let reference;
 cells[0]=Cell::Text(match value{IfcValue::Unset=>"unset",IfcValue::Derived=>"derived",IfcValue::Integer(_)=>"integer",IfcValue::Real(_)=>"real",IfcValue::String(_)=>"string",IfcValue::Enum(_)=>"enum",IfcValue::Reference(_)=>"reference",IfcValue::Aggregate(_)=>"aggregate",IfcValue::TypedValue{..}=>"typedValue"});
 match value{IfcValue::Integer(v)=>cells[1]=Cell::Integer(*v),IfcValue::Real(v)=>cells[2]=Cell::Real(*v),IfcValue::String(v)=>cells[3]=Cell::Text(v),IfcValue::Enum(v)=>cells[4]=Cell::Text(v),IfcValue::Reference(v)=>{reference=UnsignedWord::new(*v);cells[5]=Cell::Text(reference.text());cells[6]=entities.binary_search_by_key(v,|entry|entry.0).ok().map(|index|entities[index].1).map_or(Cell::Null,Cell::Integer);},IfcValue::TypedValue{name,..}=>cells[7]=Cell::Text(name),_=>{}}
 let id=projection.insert_float("ifc_value",&cells,REAL)?;
 match owner{Owner::Header(table,index)=>{projection.insert(table,&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::Argument(entity,complex,index)=>{projection.insert("ifc_argument",&[entity.map_or(Cell::Null,Cell::Integer),complex.map_or(Cell::Null,Cell::Integer),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::Aggregate(parent,index)=>{projection.insert("ifc_aggregate_element",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;},Owner::Typed(parent,index)=>{projection.insert("ifc_typed_argument",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;}}
 if let IfcValue::Aggregate(items)|IfcValue::TypedValue{items,..}=value{projection.check_rows(pending.len().checked_add(items.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 pending count overflow"))?)?;for(index,item)in items.iter().enumerate().rev(){projection.push_frontier(&mut pending,(item,if matches!(value,IfcValue::Aggregate(_)){Owner::Aggregate(id,index)}else{Owner::Typed(id,index)}))?;if index%256==0{projection.checkpoint()?;}}}
 }Ok(())
}
fn visit_rows(snapshot:&IfcSnapshot,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let mut predicted=2usize;for size in[snapshot.entities.len(),snapshot.header.file_description.len(),snapshot.header.file_name.len(),snapshot.header.file_schema.len()]{predicted=predicted.checked_add(size).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 projected count overflow"))?;}p.check_rows(predicted)?;p.checkpoint()?;
 let mut identities=p.allocate_frontier(snapshot.entities.len())?;for(index,entity)in snapshot.entities.iter().enumerate(){identities.push((entity.id,ordinal(index.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 identity count overflow"))?)?));p.checkpoint()?;}
 sort_paid(&mut identities,|left,right|{p.checkpoint()?;Ok(left.0.cmp(&right.0))})?;for pair in identities.windows(2){if pair[0].0==pair[1].0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate IFC4 instance identifier"));}p.checkpoint()?;}
 p.insert_key("ifc_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(1)])?;p.insert_key("ifc_header",1,&[])?;
 for(table,values)in[("ifc_file_description_argument",&snapshot.header.file_description),("ifc_file_name_argument",&snapshot.header.file_name),("ifc_file_schema_argument",&snapshot.header.file_schema)]{for(index,value)in values.iter().enumerate(){project_value(value,Owner::Header(table,index),&identities,p)?;}}
 for(index,entity)in snapshot.entities.iter().enumerate(){let id=identities.binary_search_by_key(&entity.id,|entry|entry.0).map(|i|identities[i].1).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"missing IFC4 identity"))?;let word=UnsignedWord::new(entity.id);p.insert_key("ifc_entity",id,&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(word.text()),Cell::Text(&entity.name)])?;
 p.check_rows(entity.args.len().checked_add(entity.complex.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 member count overflow"))?)?;for(index,value)in entity.args.iter().enumerate(){project_value(value,Owner::Argument(Some(id),None,index),&identities,p)?;}
 for(index,part)in entity.complex.iter().enumerate(){let complex=p.insert("ifc_complex_type",&[Cell::Integer(id),Cell::Integer(ordinal(index)?),Cell::Text(&part.name)])?;p.check_rows(part.args.len())?;for(index,value)in part.args.iter().enumerate(){project_value(value,Owner::Argument(None,Some(complex),index),&identities,p)?;}}}Ok(())
 }
impl IfcSnapshot{pub(super)fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::extent(control.limits())?;let mut writer=RowWriter::borrowed(control,phase)?;visit_rows(self,&mut writer)?;writer.finish_borrowed()}pub(super)fn admit_sqlite_record(value:&semio_framework_dsl_record::RecordValue,limits:SqliteDatabaseLimits,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{semantic::admit_record(value,limits,native)}}
impl ArtifactSqliteSnapshot for IfcSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn retire_sqlite_snapshot(self){super::native::close(self)}
 fn decode_sqlite_snapshot_native(payload:&semio_framework_os_kernel::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::native::decode(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_os_kernel::io_schema::IoPayload,ValueError>{super::native::encode(self,encoding,control)}
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{native_backing::preflight(self,control)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::extent(control.limits())?;let mut p=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut p)?;p.finish()}
    fn from_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{validate_sqlite_database_schema(db,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;reconstruction::read(db,control)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_os_kernel::io_schema::ArtifactDialect,db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.ifc"||dialect.standard!="4"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown owned IFC4 snapshot dialect"));}let root=db.table("ifc_document")?.single_row()?;if root.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"IFC4 document schema differs from its projection"));}Ok(semio_framework_os_kernel::io_schema::IoOutcome{value:(),diagnostics:Vec::new()})})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}
}
