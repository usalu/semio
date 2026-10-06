//! 🌳️ Borrowed native primitives admit complete Value entities, ownership links and resolved references.
use super::{SemioValueSnapshot,SqliteDatabaseLimits,ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding as native;
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::ArtifactSqliteSnapshot;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Semio value native primitive differs from its authored shape")}
pub(crate)fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio value semantic extent overflow"))}
/// 🏛️ Admits the own document, nodes, values and two structural edge entities.
pub(crate)fn layout(limits:SqliteDatabaseLimits)->Result<()>{
 let mut bytes=0;for statement in SemioValueSnapshot::SQLITE_SCHEMA.split(';'){bytes=add(bytes,statement.trim().len())?;}for name in["semio_value_document","semio_value_node","semio_value_value","semio_value_list_element","semio_value_map_entry"]{bytes=add(bytes,name.len())?;}
 if bytes.max(SemioValueSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio value schema exceeds caller bytes"))}
 if limits.max_tables<5||limits.max_columns<8||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio value authored layout exceeds copied limits"))}Ok(())
}
/// 🧮️ Counts only actual non-NULL SQL cells and structural entities.
pub(crate)struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 pub(crate)fn new(limits:SqliteDatabaseLimits)->Self{Self{limits,rows:0,bytes:0}}
 pub(crate)fn row(&mut self,bytes:usize)->Result<()>{self.rows=add(self.rows,1)?;self.bytes=add(self.bytes,bytes)?;if self.rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio value complete rows exceed caller limit"))}if self.bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio value complete SQL cells exceed caller bytes"))}Ok(())}
 pub(crate)fn future(&self,count:usize)->Result<()>{if add(self.rows,count)?>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Semio value declared entities exceed caller limit"))}Ok(())}
}
/// 🔣️ Validates each decoded scalar with the common fixed four-byte scratch.
pub(crate)fn hex_text(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{native::hex_text_extent(value,control)}
fn octet(value:&str,at:usize)->Result<u8>{u8::from_str_radix(value.get(at..at+2).ok_or_else(invalid)?,16).map_err(|_|invalid())}
/// 🫙️ Validates raw octets without treating a byte value as UTF-8.
fn hex_blob(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize>{
 if value.len()%2!=0{return Err(invalid())}for at in(0..value.len()).step_by(2){octet(value,at)?;if at%512==0{control.checkpoint()?;}}Ok(value.len()/2)
}
fn compare_hex(a:&str,b:&str,control:&mut NativeDecodeControl<'_>)->Result<std::cmp::Ordering>{
 for at in(0..a.len().min(b.len())).step_by(2){if at%512==0{control.checkpoint()?;}let order=octet(a,at)?.cmp(&octet(b,at)?);if order!=std::cmp::Ordering::Equal{return Ok(order)}}control.checkpoint()?;Ok(a.len().cmp(&b.len()))
}
fn sift(ids:&mut[&str],mut root:usize,end:usize,control:&mut NativeDecodeControl<'_>)->Result<()>{
 loop{let left=root*2+1;if left>=end{return Ok(())}let mut child=left;if left+1<end&&compare_hex(ids[left],ids[left+1],control)?==std::cmp::Ordering::Less{child=left+1}if compare_hex(ids[root],ids[child],control)?!=std::cmp::Ordering::Less{return Ok(())}ids.swap(root,child);root=child;control.checkpoint()?;}
}
fn sort_ids(ids:&mut[&str],control:&mut NativeDecodeControl<'_>)->Result<()>{
 let length=ids.len();for start in(0..length/2).rev(){sift(ids,start,length,control)?;}for end in(1..length).rev(){ids.swap(0,end);sift(ids,0,end,control)?;}for pair in ids.windows(2){if compare_hex(pair[0],pair[1],control)?==std::cmp::Ordering::Equal{return Err(invalid())}}Ok(())
}
fn reference(value:&str,ids:&[&str],control:&mut NativeDecodeControl<'_>)->Result<()>{
 let mut lo=0;let mut hi=ids.len();while lo<hi{let mid=lo+(hi-lo)/2;match compare_hex(ids[mid],value,control)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(())}}Err(invalid())
}
/// 🌿️ Counts actual text variants, retaining lexemes, map duplicates and reference role.
pub(crate)fn value_text(value:&str,ids:Option<&[&str]>,census:&mut Census,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 control.scoped_depth(64,|control|{
  control.checkpoint()?;if value=="Z"{return census.row(12)}let(tag,body)=value.split_at_checked(1).ok_or_else(invalid)?;let body=body.strip_prefix('[').and_then(|body|body.strip_suffix(']')).ok_or_else(invalid)?;
  match tag{
   "B"=>{if body!="0"&&body!="1"{return Err(invalid())}census.row(20)?;},
   "I"|"F"|"S"=>census.row(add(if tag=="F"{13}else{11},hex_text(body,control)?)?)?,
   "Y"=>census.row(add(13,hex_blob(body,control)?)?)?,
   "R"=>{let size=hex_text(body,control)?;let bytes=if let Some(ids)=ids{reference(body,ids,control)?;8}else{size};census.row(add(11,bytes)?)?;},
   "L"|"M"=>{
    census.row(if tag=="L"{12}else{11})?;let mut items=native::Items::new(&value[1..])?;let count=items.count(control,limits.max_rows)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;
    control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=items.next(control)?{let item=if tag=="M"{let(key,value)=item.split_once(':').ok_or_else(invalid)?;census.row(add(32,hex_text(key,control)?)?)?;value}else{census.row(32)?;item};value_text(item,ids,census,control,limits)?;control.step()?;}Ok::<_,ValueError>(())})?;
   },
   _=>return Err(invalid())
  }Ok(())
 })
}
/// 🧬️ Counts the nested binary value domain with literal TEXT references for Table consumers.
pub(crate)fn value_binary(reader:&mut store::ByteReader<'_>,census:&mut Census,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 control.scoped_depth(64,|control|{
  control.checkpoint()?;let tag=reader.read_u8().map_err(|_|invalid())?;
  match tag{
   0=>census.row(12)?,
   1=>{if reader.read_u8().map_err(|_|invalid())?>1{return Err(invalid())}census.row(20)?;},
   2|3|4|8=>{let size=control.borrow_text(native::bytes(reader)?)?.len();census.row(add(if tag==3{13}else{11},size)?)?;},
   5=>{let size=native::bytes(reader)?.len();census.row(add(13,size)?)?;},
   6|7=>{
    census.row(if tag==6{12}else{11})?;let count=native::length(reader)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;
    control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{let bytes=if tag==7{add(32,control.borrow_text(native::bytes(reader)?)?.len())?}else{32};census.row(bytes)?;value_binary(reader,census,control,limits)?;control.step()?;}Ok::<_,ValueError>(())})?;
   },
   _=>return Err(invalid())
  }Ok(())
 })
}
/// 📃️ Builds only the real paid borrowed identity frontier before the typed graph owner.
pub(crate)fn document(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{
 layout(limits)?;control.scoped_stage(|control|{
  let[schema,root,nodes]=native::record(body,control)?;let mut census=Census::new(limits);census.row(add(16,hex_text(schema,control)?)?)?;
  let mut entries=native::Items::new(nodes)?;let count=entries.count(control,limits.max_rows)?;census.future(count.checked_mul(2).ok_or_else(invalid)?)?;let mut ids=control.allocate_vec::<&str>(count)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=entries.next(control)?{let(id,_)=item.split_once(':').ok_or_else(invalid)?;census.row(add(32,hex_text(id,control)?)?)?;ids.push(id);control.step()?;}Ok::<_,ValueError>(())})?;sort_ids(&mut ids,control)?;
  value_text(root,Some(&ids),&mut census,control,limits)?;let mut entries=native::Items::new(nodes)?;
  control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(item)=entries.next(control)?{let(_,value)=item.split_once(':').ok_or_else(invalid)?;value_text(value,Some(&ids),&mut census,control,limits)?;control.step()?;}Ok::<_,ValueError>(())})?;control.checkpoint()
 })
}
/// 📦️ Value's own binary body is the same authored UTF-8 document grammar.
pub(crate)fn binary(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<()>{let body=control.borrow_text(body)?;document(body,control,limits)}
