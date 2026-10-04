//! 🎨️ Explicit indexed bitmap entities retain every current native literal without a codec carrier.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,Projection,RowWriter}};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub struct Tables{pub bitmap:&'static str,pub pixel:&'static str,pub literal:&'static str}
pub trait BitmapRows{
 fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>;
 fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>;
 fn checkpoint(&mut self)->Result<(),ValueError>;
}
impl BitmapRows for Projection<'_,'_>{
 fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{Projection::insert(self,table,cells)}
 fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{Projection::insert_key(self,table,key,cells)}
 fn checkpoint(&mut self)->Result<(),ValueError>{Projection::checkpoint(self)}
}
impl BitmapRows for RowWriter<'_,'_>{
 fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{RowWriter::insert(self,table,cells)}
 fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{RowWriter::insert_key(self,table,key,cells)}
 fn checkpoint(&mut self)->Result<(),ValueError>{RowWriter::checkpoint(self)}
}
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn overflow()->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,"bitmap occurrence count exceeds address space")}
fn digit(value:u8)->Option<u8>{match value{b'A'..=b'Z'=>Some(value-b'A'),b'a'..=b'z'=>Some(value-b'a'+26),b'0'..=b'9'=>Some(value-b'0'+52),b'+'=>Some(62),b'/'=>Some(63),_=>None}}
/// 🔤️ Canonical recognition only chooses an explicit storage branch; it never refuses an owning literal.
pub fn canonical_count(text:&str,mut checkpoint:impl FnMut()->Result<(),ValueError>)->Result<Option<usize>,ValueError>{
 let bytes=text.as_bytes();if bytes.len()%4!=0{return Ok(None)}if bytes.is_empty(){return Ok(Some(0))}
 let padding=if bytes.ends_with(b"=="){2}else if bytes.ends_with(b"="){1}else{0};
 let end=bytes.len()-padding;
 for(index,byte)in bytes.iter().enumerate(){if index%1024==0{checkpoint()?}if if index<end{digit(*byte).is_none()}else{*byte!=b'='}{return Ok(None)}}
 let Some(last)=digit(bytes[end-1])else{return Ok(None)};
 if padding==2&&last&15!=0||padding==1&&last&3!=0{return Ok(None)}
 Ok(Some(bytes.len()/4*3-padding))
}
/// 📤️ Canonical bytes and literal Unicode scalar occurrences have separate editable relationships.
pub fn project(p:&mut impl BitmapRows,tables:&Tables,id:i64,width:u32,height:u32,text:&str)->Result<(),ValueError>{
 let count=canonical_count(text,||p.checkpoint())?;
 p.insert_key(tables.bitmap,id,&[Cell::Integer(i64::from(width)),Cell::Integer(i64::from(height)),Cell::Text(if count.is_some(){"indices"}else{"literal"})])?;
 if let Some(count)=count{let mut ordinal=0usize;for chunk in text.as_bytes().chunks_exact(4){
  let mut word=0u32;for byte in chunk{word=(word<<6)|u32::from(digit(*byte).unwrap_or(0));}
  for shift in[16,8,0]{if ordinal==count{break}p.insert(tables.pixel,&[Cell::Integer(id),Cell::Integer(i64::try_from(ordinal).map_err(|_|overflow())?),Cell::Integer(i64::from((word>>shift)&255))])?;ordinal+=1;}
 }}else{for(index,value)in text.chars().enumerate(){let mut buffer=[0;4];p.insert(tables.literal,&[Cell::Integer(id),Cell::Integer(i64::try_from(index).map_err(|_|overflow())?),Cell::Text(value.encode_utf8(&mut buffer))])?;}}
 Ok(())
}
#[derive(Clone,Copy)]
struct Occurrence<'a>{owner:i64,ordinal:usize,row:&'a SqliteRow}
fn sort<T,K:Ord+Copy>(values:&mut[T],key:impl Fn(&T)->K,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 fn sift<T,K:Ord+Copy>(values:&mut[T],mut root:usize,end:usize,key:&impl Fn(&T)->K,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  while root<end/2{native.step()?;let mut child=root*2+1;if child+1<end&&key(&values[child])<key(&values[child+1]){child+=1}if key(&values[root])>=key(&values[child]){break}values.swap(root,child);root=child;}Ok(())
 }
 native.scoped_stage(|native|{native.begin_stage(0)?;let count=values.len();for root in(0..count/2).rev(){sift(values,root,count,&key,native)?;}for end in(1..count).rev(){values.swap(0,end);sift(values,0,end,&key,native)?;}Ok(())})
}
fn occurrences<'a>(rows:&'a[SqliteRow],owners:&[i64],native:&mut NativeDecodeControl<'_>)->Result<Vec<Occurrence<'a>>,ValueError>{
 native.scoped_stage(|native|{
  let mut result=native.allocate_vec(rows.len())?;let mut ids=native.allocate_vec(rows.len())?;native.begin_stage(rows.len())?;
  for row in rows{if row.rowid<=0||row.values.len()!=4||row.integer(0)?!=row.rowid{return Err(invalid("bitmap occurrence identity or columns differ"))}
   let owner=row.integer(1)?;if owners.binary_search(&owner).is_err(){return Err(invalid("bitmap occurrence owner is absent"))}
   let ordinal=usize::try_from(row.integer(2)?).map_err(|_|invalid("bitmap occurrence ordinal is negative"))?;
   ids.push(row.rowid);result.push(Occurrence{owner,ordinal,row});native.step()?;
  }
  sort(&mut ids,|id|*id,native)?;native.begin_stage(ids.len())?;for pair in ids.windows(2){if pair[0]==pair[1]{return Err(invalid("bitmap occurrence identity is duplicated"))}native.step()?;}
  sort(&mut result,|row|(row.owner,row.ordinal),native)?;Ok(result)
 })
}
fn group<'a,'b>(rows:&'b[Occurrence<'a>],owner:i64)->&'b[Occurrence<'a>]{let start=rows.partition_point(|row|row.owner<owner);let end=rows.partition_point(|row|row.owner<=owner);&rows[start..end]}
fn text(rows:&[Occurrence<'_>],canonical:bool,native:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{
 native.begin_stage(rows.len())?;for(index,row)in rows.iter().enumerate(){if row.ordinal!=index{return Err(invalid("bitmap occurrence ordinal is not contiguous"))}native.step()?;}
 let length=if canonical{rows.len().checked_add(2).and_then(|n|n.checked_div(3)).and_then(|n|n.checked_mul(4)).ok_or_else(overflow)?}else{let mut bytes=0usize;for row in rows{let scalar=row.row.text(3)?;let mut characters=scalar.chars();if characters.next().is_none()||characters.next().is_some(){return Err(invalid("bitmap literal requires exactly one Unicode scalar"))}bytes=bytes.checked_add(scalar.len()).ok_or_else(overflow)?;}bytes};
 let mut out=native.allocate_vec::<u8>(length)?;native.begin_stage(rows.len())?;
 if canonical{for chunk in rows.chunks(3){let mut word=0u32;for(index,row)in chunk.iter().enumerate(){let value=u8::try_from(row.row.integer(3)?).map_err(|_|invalid("bitmap palette index exceeds unsigned byte"))?;word|=u32::from(value)<<(16-index*8);}
  out.push(ALPHABET[((word>>18)&63)as usize]);out.push(ALPHABET[((word>>12)&63)as usize]);out.push(if chunk.len()>1{ALPHABET[((word>>6)&63)as usize]}else{b'='});out.push(if chunk.len()>2{ALPHABET[(word&63)as usize]}else{b'='});for _ in chunk{native.step()?;}
 }}else{for row in rows{out.extend_from_slice(row.row.text(3)?.as_bytes());native.step()?;}}
 String::from_utf8(out).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"bitmap authored output lost valid UTF8"))
}
/// 📥️ Reconstructs real owning strings through the caller's exact cumulative native allocation frontier.
pub fn reconstruct(d:&SqliteDatabase,tables:&Tables,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<(i64,String)>,ValueError>{
 let maximum=c.reconstruction_remaining_bytes()?;let mut owned=0;
 let result=c.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint|{
  let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total);
  let mut native=NativeDecodeControl::new(maximum.min(remaining),&mut progress);
  let result=(||{
   let bitmaps=&d.table(tables.bitmap)?.rows;let pixels=&d.table(tables.pixel)?.rows;let literals=&d.table(tables.literal)?.rows;
   let mut owners=native.allocate_vec(bitmaps.len())?;native.begin_stage(bitmaps.len())?;for row in bitmaps{owners.push(row.rowid);native.step()?;}sort(&mut owners,|owner|*owner,&mut native)?;
   let pixels=occurrences(pixels,&owners,&mut native)?;let literals=occurrences(literals,&owners,&mut native)?;
   let mut result=native.allocate_vec(bitmaps.len())?;
   for bitmap in bitmaps{let pixel=group(&pixels,bitmap.rowid);let literal=group(&literals,bitmap.rowid);
    let canonical=match bitmap.text(3)?{"indices"=>true,"literal"=>false,_=>return Err(invalid("bitmap storage branch differs"))};
    if canonical&&!literal.is_empty()||!canonical&&!pixel.is_empty(){return Err(invalid("bitmap storage branches conflict"))}
    result.push((bitmap.rowid,text(if canonical{pixel}else{literal},canonical,&mut native)?));
   }Ok(result)
  })();owned=native.owned_bytes();(result,owned)
 });c.admit_reconstruction_bytes(owned)?;result?
}
/// 📦️ Transfers the one admitted pixel field into its actual bitmap owner.
pub fn take(values:&mut Vec<(i64,String)>,owner:i64)->Result<String,ValueError>{let index=values.iter().position(|value|value.0==owner).ok_or_else(||invalid("bitmap native pixel field is missing"))?;Ok(values.swap_remove(index).1)}
