//! 🔍️ Complete typed row references and consumed flags have admitted concrete Vec backing.
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow};
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};
const TABLES:&[(&str,usize,bool)]=&[("lowpoly_document",2,false),("lowpoly_object",7,true),("lowpoly_transform",28,false),("lowpoly_mesh_child",6,false),("lowpoly_paint_layer",9,true),("lowpoly_paint_octet",4,true),("lowpoly_mesh_state",1,false),("lowpoly_mesh_vertex",22,true),("lowpoly_mesh_halfedge",13,true),("lowpoly_mesh_face",6,true),("lowpoly_mesh_seam",4,true),("lowpoly_mesh_attribute",8,true),("lowpoly_mesh_attribute_index",4,true),("lowpoly_mesh_attribute_sample",4,true),("lowpoly_mesh_material",5,true),("lowpoly_mesh_texture",5,true),("lowpoly_mesh_texture_octet",4,true),("lowpoly_mesh_value",2,false),("lowpoly_mesh_boolean",2,false),("lowpoly_mesh_unsigned",2,false),("lowpoly_mesh_signed",2,false),("lowpoly_mesh_float",4,false),("lowpoly_mesh_text",2,false),("lowpoly_mesh_bytes",1,false),("lowpoly_mesh_bytes_octet",4,true),("lowpoly_mesh_array_member",4,true),("lowpoly_mesh_object_member",5,true)];
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
pub fn sort<T>(values:&mut[T],control:&mut NativeDecodeControl<'_>,mut compare:impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,control:&mut NativeDecodeControl<'_>,compare:&mut impl FnMut(&T,&T)->Result<std::cmp::Ordering,ValueError>)->Result<(),ValueError>{loop{control.step()?;let Some(left)=root.checked_mul(2).and_then(|value|value.checked_add(1)).filter(|value|*value<end)else{return Ok(())};let mut child=left;if left+1<end&&compare(&values[left],&values[left+1])?.is_lt(){child=left+1;}if !compare(&values[root],&values[child])?.is_lt(){return Ok(())}values.swap(root,child);root=child;}}
 control.scoped_stage(|control|{control.begin_stage(0)?;for root in(0..values.len()/2).rev(){sift(values,root,values.len(),control,&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,control,&mut compare)?;}control.checkpoint()})
}
struct Table<'a>{name:&'static str,identities:Vec<&'a SqliteRow>,groups:Vec<&'a SqliteRow>,used:Vec<bool>}
pub struct Rows<'a>{tables:Vec<Table<'a>>}
impl<'a> Rows<'a>{
 /// 🛂️ Capture all concrete row owners and validate exact identities before grouping them.
 pub fn new(database:&'a SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
  let mut tables=control.allocate_vec(TABLES.len())?;
  for&(name,width,grouped)in TABLES{let source=database.table(name)?;let mut identities=control.allocate_vec(source.rows.len())?;control.scoped_stage(|control|{control.begin_stage(source.rows.len())?;for row in&source.rows{control.step()?;if row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("managed mesh row identity or width differs"))}if grouped&&(row.integer(2)?<0){return Err(invalid("managed mesh ordinal is negative"))}identities.push(row);}Ok::<(),ValueError>(())})?;
   sort(&mut identities,control,|a,b|Ok(a.rowid.cmp(&b.rowid)))?;for pair in identities.windows(2){control.checkpoint()?;if pair[0].rowid==pair[1].rowid{return Err(invalid("managed mesh duplicate row identity"))}}
   let mut used=control.allocate_vec(identities.len())?;used.resize(identities.len(),false);let mut groups=control.allocate_vec(if grouped{identities.len()}else{0})?;if grouped{groups.extend_from_slice(&identities);sort(&mut groups,control,|a,b|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;}tables.push(Table{name,identities,groups,used});
  }Ok(Self{tables})
 }
 fn table(&mut self,name:&str)->Result<&mut Table<'a>,ValueError>{self.tables.iter_mut().find(|table|table.name==name).ok_or_else(||invalid("managed mesh owned table absent"))}
 /// 🪪️ Preserve full signed SQLite identity and consume an entity exactly once.
 pub fn take(&mut self,name:&str,id:i64,control:&mut NativeDecodeControl<'_>)->Result<&'a SqliteRow,ValueError>{let table=self.table(name)?;let mut first=0;let mut last=table.identities.len();while first<last{control.checkpoint()?;let middle=first+(last-first)/2;match table.identities[middle].rowid.cmp(&id){std::cmp::Ordering::Less=>first=middle+1,std::cmp::Ordering::Greater=>last=middle,std::cmp::Ordering::Equal=>{if std::mem::replace(&mut table.used[middle],true){return Err(invalid("managed mesh entity has shared or cyclic ownership"))}return Ok(table.identities[middle])}}}Err(invalid("managed mesh relation target absent"))}
 /// 🔗️ Read dense ordinals through paid references without cloning rows or literal cells.
 pub fn group(&mut self,name:&str,parent:i64,control:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
  let table=self.table(name)?;let mut first=0;let mut last=table.groups.len();while first<last{control.checkpoint()?;let middle=first+(last-first)/2;if table.groups[middle].integer(1)?<parent{first=middle+1}else{last=middle}}let start=first;last=table.groups.len();while first<last{control.checkpoint()?;let middle=first+(last-first)/2;if table.groups[middle].integer(1)?<=parent{first=middle+1}else{last=middle}}let end=first;let mut output=control.allocate_vec(end-start)?;
  control.scoped_stage(|control|{control.begin_stage(end-start)?;for row in&table.groups[start..end]{control.step()?;if row.integer(2)?!=i64::try_from(output.len()).map_err(|_|invalid("managed mesh ordinal exceeds integer64"))?{return Err(invalid("managed mesh ordinals are not contiguous"))}let at=table.identities.binary_search_by_key(&row.rowid,|row|row.rowid).map_err(|_|invalid("managed mesh grouped identity absent"))?;if std::mem::replace(&mut table.used[at],true){return Err(invalid("managed mesh occurrence has shared ownership"))}output.push(*row);}Ok::<(),ValueError>(())})?;Ok(output)
 }
 /// ✅️ Refuse every unconsumed scalar or orphan relationship in the complete mesh namespace.
 pub fn finish(&self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for table in&self.tables{control.scoped_stage(|control|{control.begin_stage(table.used.len())?;for used in&table.used{control.step()?;if !used{return Err(invalid("managed mesh unowned semantic entity"))}}Ok::<(),ValueError>(())})?;}Ok(())}
 /// 🔢️ Expose exact root cardinality from the admitted identity slots.
 pub fn len(&self,name:&str)->Result<usize,ValueError>{self.tables.iter().find(|table|table.name==name).map(|table|table.identities.len()).ok_or_else(||invalid("Lowpoly owned table absent"))}
 /// 🪪️ Consume a singleton without assuming any particular signed surrogate identity.
 pub fn sole(&mut self,name:&str,control:&mut NativeDecodeControl<'_>)->Result<&'a SqliteRow,ValueError>{let table=self.table(name)?;if table.identities.len()!=1{return Err(invalid("Lowpoly owned singleton cardinality differs"))}let id=table.identities[0].rowid;self.take(name,id,control)}
 /// 🧭️ Observe optional roots without consuming any owned entity.
 pub fn contains(&self,name:&str,id:i64)->bool{self.tables.iter().find(|table|table.name==name).is_some_and(|table|table.identities.binary_search_by_key(&id,|row|row.rowid).is_ok())}
}
