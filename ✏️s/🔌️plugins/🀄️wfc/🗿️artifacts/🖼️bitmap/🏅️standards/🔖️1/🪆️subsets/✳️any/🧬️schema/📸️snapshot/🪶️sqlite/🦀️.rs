//! 🖼️ Authored overlapping-model bitmap fields and ordered problem entities.
use super::{BitmapSnapshot,BitmapInput,BitmapOutputSpec,BitmapOverlappingModel,BitmapColor,BitmapPinnedPixel};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue,SnapshotEncoding,validate_sqlite_database_schema,artifact::{Cell,Projection,Reconstruction,NativeEncodingBound}}};
use std::collections::BTreeSet;
fn index(n:usize)->Result<i64,String>{i64::try_from(n).map_err(|_|"bitmap ordinal overflow".into())}
fn identity(r:&SqliteRow,n:usize)->Result<(),String>{if r.values.len()!=n||r.integer(0)?!=r.rowid||r.rowid<=0{return Err("bitmap entity identity or field count differs".into())}Ok(())}
fn unsigned(r:&SqliteRow,i:usize)->Result<u32,String>{u32::try_from(r.integer(i)?).map_err(|_|"bitmap scalar exceeds unsigned32".into())}
fn boolean(r:&SqliteRow,i:usize)->Result<bool,String>{match r.integer(i)?{0=>Ok(false),1=>Ok(true),_=>Err("bitmap boolean differs".into())}}
fn one<'a>(d:&'a SqliteDatabase,name:&str,n:usize)->Result<&'a SqliteRow,String>{let r=d.table(name)?.single_row()?;identity(r,n)?;if r.rowid!=1{return Err("bitmap document identity must be one".into())}Ok(r)}
impl ArtifactSqliteSnapshot for BitmapSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 let mut p=Projection::new(Self::SQLITE_SCHEMA,c)?;p.check_rows(4usize.checked_add(self.input.palette.len()).and_then(|n|n.checked_add(self.pinned.len())).ok_or("bitmap row count overflow")?)?;
 p.insert_key("wfc_bitmap_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.seed.to_string())])?;
 p.insert_key("wfc_bitmap_input",1,&[Cell::Integer(i64::from(self.input.width)),Cell::Integer(i64::from(self.input.height)),Cell::Text(&self.input.pixels)])?;
 p.insert_key("wfc_bitmap_output",1,&[Cell::Integer(i64::from(self.output.width)),Cell::Integer(i64::from(self.output.height)),Cell::Integer(i64::from(self.output.periodic))])?;
 p.insert_key("wfc_bitmap_model",1,&[Cell::Integer(i64::from(self.model.pattern_size)),Cell::Integer(i64::from(self.model.symmetry)),Cell::Integer(i64::from(self.model.periodic_input)),self.model.ground.map_or(Cell::Null,|n|Cell::Integer(i64::from(n)))])?;
 for(i,color)in self.input.palette.iter().enumerate(){p.insert("wfc_bitmap_palette",&[Cell::Integer(1),Cell::Integer(index(i)?),Cell::Integer(i64::from(color.r)),Cell::Integer(i64::from(color.g)),Cell::Integer(i64::from(color.b)),Cell::Integer(i64::from(color.a))])?;}
 for(i,pin)in self.pinned.iter().enumerate(){p.insert("wfc_bitmap_pin",&[Cell::Integer(1),Cell::Integer(index(i)?),Cell::Integer(i64::from(pin.x)),Cell::Integer(i64::from(pin.y)),Cell::Integer(i64::from(pin.color))])?;}
 p.finish()
 }
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
 validate_sqlite_database_schema(d,Self::SQLITE_SCHEMA,c.limits()).map_err(|e|e.to_string())?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
 let document=one(d,"wfc_bitmap_document",3)?;let seed=document.text(2)?.parse::<u64>().map_err(|_|"bitmap seed exceeds unsigned64")?;if seed.to_string()!=document.text(2)?{return Err("bitmap seed requires canonical decimal".into())}
 let input=one(d,"wfc_bitmap_input",4)?;let output=one(d,"wfc_bitmap_output",4)?;let model=one(d,"wfc_bitmap_model",5)?;
 let ground=match model.values[4]{SqliteValue::Null=>None,_=>Some(unsigned(model,4)?)};
 let mut owned=Reconstruction::new(c)?;let schema=owned.text(document.text(1)?)?;let pixels=owned.text(input.text(3)?)?;let mut palette=Vec::new();let mut ids=BTreeSet::new();
 for row in d.table("wfc_bitmap_palette")?.ordered_rows(2)?{identity(row,7)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err("bitmap palette ownership differs".into())}for _ in 0..4{owned.scalar()?;}palette.push(BitmapColor{r:unsigned(row,3)?,g:unsigned(row,4)?,b:unsigned(row,5)?,a:unsigned(row,6)?});}
 ids.clear();let mut pinned=Vec::new();for row in d.table("wfc_bitmap_pin")?.ordered_rows(2)?{identity(row,6)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err("bitmap pin ownership differs".into())}for _ in 0..3{owned.scalar()?;}pinned.push(BitmapPinnedPixel{x:unsigned(row,3)?,y:unsigned(row,4)?,color:unsigned(row,5)?});}
 for _ in 0..10{owned.scalar()?;}owned.checkpoint()?;
 Ok(Self{schema,seed,input:BitmapInput{width:unsigned(input,1)?,height:unsigned(input,2)?,palette,pixels},output:BitmapOutputSpec{width:unsigned(output,1)?,height:unsigned(output,2)?,periodic:boolean(output,3)?},model:BitmapOverlappingModel{pattern_size:unsigned(model,1)?,symmetry:unsigned(model,2)?,periodic_input:boolean(model,3)?,ground},pinned})
 }
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=NativeEncodingBound::new(c)?;b.add(4096)?;b.check_rows(4usize.checked_add(self.input.palette.len()).and_then(|n|n.checked_add(self.pinned.len())).ok_or("bitmap native entity overflow")?)?;b.repeated(self.schema.len(),16)?;b.repeated(self.input.pixels.len(),16)?;for _ in &self.input.palette{b.add(512)?;}for _ in &self.pinned{b.add(512)?;}b.finish()}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;if dialect.artifact_kind!="s.wfc.bitmap"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("bitmap dialect differs from its owned wildcard").into())}if one(d,"wfc_bitmap_document",3)?.text(1)?!=self.schema{return Err(String::from("bitmap projected schema differs").into())}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}
}
