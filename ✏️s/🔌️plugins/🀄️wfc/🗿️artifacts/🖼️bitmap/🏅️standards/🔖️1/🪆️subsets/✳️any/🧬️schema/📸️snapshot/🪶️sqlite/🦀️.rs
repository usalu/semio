//! 🖼️ Authored overlapping-model bitmap fields and ordered problem entities.
use super::{BitmapSnapshot,BitmapInput,BitmapOutputSpec,BitmapOverlappingModel,BitmapColor,BitmapPinnedPixel};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Cell,RowWriter,Reconstruction,NativeEncodingBound}}};
use std::collections::BTreeSet;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn index(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"bitmap ordinal overflow"))}
fn identity(r:&SqliteRow,n:usize)->Result<(),ValueError>{if r.values.len()!=n||r.integer(0)?!=r.rowid||r.rowid<=0{return Err(invalid("bitmap entity identity or field count differs"))}Ok(())}
fn unsigned(r:&SqliteRow,i:usize)->Result<u32,ValueError>{u32::try_from(r.integer(i)?).map_err(|_|invalid("bitmap scalar exceeds unsigned32"))}
fn boolean(r:&SqliteRow,i:usize)->Result<bool,ValueError>{match r.integer(i)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("bitmap boolean differs"))}}
fn one<'a>(d:&'a SqliteDatabase,name:&str,n:usize)->Result<&'a SqliteRow,ValueError>{let r=d.table(name)?.single_row()?;identity(r,n)?;if r.rowid!=1{return Err(invalid("bitmap document identity must be one"))}Ok(r)}
fn decimal(mut value:u64,buffer:&mut[u8;20])->&str{let mut start=buffer.len();loop{start-=1;buffer[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}std::str::from_utf8(&buffer[start..]).expect("decimal digits are ASCII")}
impl BitmapSnapshot{
 pub(crate) fn admit_sqlite_values(&self,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{let mut p=RowWriter::borrowed(c,phase)?;self.write_sqlite_rows(&mut p)?;p.finish_borrowed()}
 fn write_sqlite_rows(&self,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let mut seed=[0;20];
 p.insert_key("wfc_bitmap_document",1,&[Cell::Text(&self.schema),Cell::Text(decimal(self.seed,&mut seed))])?;
 p.insert_key("wfc_bitmap_input",1,&[Cell::Integer(i64::from(self.input.width)),Cell::Integer(i64::from(self.input.height)),Cell::Text(&self.input.pixels)])?;
 p.insert_key("wfc_bitmap_output",1,&[Cell::Integer(i64::from(self.output.width)),Cell::Integer(i64::from(self.output.height)),Cell::Integer(i64::from(self.output.periodic))])?;
 p.insert_key("wfc_bitmap_model",1,&[Cell::Integer(i64::from(self.model.pattern_size)),Cell::Integer(i64::from(self.model.symmetry)),Cell::Integer(i64::from(self.model.periodic_input)),self.model.ground.map_or(Cell::Null,|n|Cell::Integer(i64::from(n)))])?;
 for(i,color)in self.input.palette.iter().enumerate(){p.insert("wfc_bitmap_palette",&[Cell::Integer(1),Cell::Integer(index(i)?),Cell::Integer(i64::from(color.r)),Cell::Integer(i64::from(color.g)),Cell::Integer(i64::from(color.b)),Cell::Integer(i64::from(color.a))])?;}
 for(i,pin)in self.pinned.iter().enumerate(){p.insert("wfc_bitmap_pin",&[Cell::Integer(1),Cell::Integer(index(i)?),Cell::Integer(i64::from(pin.x)),Cell::Integer(i64::from(pin.y)),Cell::Integer(i64::from(pin.color))])?;}
 Ok(())
 }
}
impl ArtifactSqliteSnapshot for BitmapSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::schema::snapshot::text::decode_sqlite_snapshot_native(payload,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{crate::schema::snapshot::text::encode_sqlite_snapshot_native(self,encoding,c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut p=RowWriter::new(Self::SQLITE_SCHEMA,c)?;self.write_sqlite_rows(&mut p)?;p.finish()}
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
 validate_sqlite_database_schema(d,Self::SQLITE_SCHEMA,c.limits())?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=one(d,"wfc_bitmap_document",3)?;let seed=document.text(2)?.parse::<u64>().map_err(|_|invalid("bitmap seed exceeds unsigned64"))?;if seed.to_string()!=document.text(2)?{return Err(invalid("bitmap seed requires canonical decimal"))}
 let input=one(d,"wfc_bitmap_input",4)?;let output=one(d,"wfc_bitmap_output",4)?;let model=one(d,"wfc_bitmap_model",5)?;
 let ground=match model.values[4]{SqliteValue::Null=>None,_=>Some(unsigned(model,4)?)};
 let mut owned=Reconstruction::new(c)?;let schema=owned.text(document.text(1)?)?;let pixels=owned.text(input.text(3)?)?;let mut palette=Vec::new();let mut ids=BTreeSet::new();
 for row in d.table("wfc_bitmap_palette")?.ordered_rows(2)?{identity(row,7)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err(invalid("bitmap palette ownership differs"))}for _ in 0..4{owned.scalar()?;}palette.push(BitmapColor{r:unsigned(row,3)?,g:unsigned(row,4)?,b:unsigned(row,5)?,a:unsigned(row,6)?});}
 ids.clear();let mut pinned=Vec::new();for row in d.table("wfc_bitmap_pin")?.ordered_rows(2)?{identity(row,6)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err(invalid("bitmap pin ownership differs"))}for _ in 0..3{owned.scalar()?;}pinned.push(BitmapPinnedPixel{x:unsigned(row,3)?,y:unsigned(row,4)?,color:unsigned(row,5)?});}
 for _ in 0..10{owned.scalar()?;}owned.checkpoint()?;
 Ok(Self{schema,seed,input:BitmapInput{width:unsigned(input,1)?,height:unsigned(input,2)?,palette,pixels},output:BitmapOutputSpec{width:unsigned(output,1)?,height:unsigned(output,2)?,periodic:boolean(output,3)?},model:BitmapOverlappingModel{pattern_size:unsigned(model,1)?,symmetry:unsigned(model,2)?,periodic_input:boolean(model,3)?,ground},pinned})
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let mut b=NativeEncodingBound::new(c)?;b.add(4096)?;b.check_rows(4usize.checked_add(self.input.palette.len()).and_then(|n|n.checked_add(self.pinned.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"bitmap native entity overflow"))?)?;b.repeated(self.schema.len(),16)?;b.repeated(self.input.pixels.len(),16)?;for _ in &self.input.palette{b.add(512)?;}for _ in &self.pinned{b.add(512)?;}b.finish()}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let io=semio_framework_os_kernel::io_schema::IoError::from_value_error;c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(io)?;if dialect.artifact_kind!="s.wfc.bitmap"||dialect.standard!="1"||dialect.subset!="*"{return Err(io(invalid("bitmap dialect differs from its owned wildcard")))}if one(d,"wfc_bitmap_document",3).map_err(io)?.text(1).map_err(io)?!=self.schema{return Err(io(invalid("bitmap projected schema differs")))}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(io)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}
}
