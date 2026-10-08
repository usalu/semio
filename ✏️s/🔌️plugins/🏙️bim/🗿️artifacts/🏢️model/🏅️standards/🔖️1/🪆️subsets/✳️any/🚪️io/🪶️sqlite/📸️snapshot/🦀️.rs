//! 🏢️ Literal BIM fields, exact scalar words and owned authored relationships.
use crate::*;
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl};
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
pub const SCHEMA:&str=include_str!("🗄️.sql");
#[path="📏️cells/🦀️.rs"]mod semantic_cells;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"BIM extent overflow"))}
struct Cells<'a>{values:[Cell<'a>;64],len:usize}
impl<'a>Cells<'a>{
 fn new()->Self{Self{values:[Cell::Null;64],len:0}}
 fn push(&mut self,value:Cell<'a>)->Result<()>{if self.len==self.values.len(){return Err(invalid("BIM row exceeds authored stack width"))}self.values[self.len]=value;self.len+=1;Ok(())}
 fn nulls(&mut self,count:usize)->Result<()>{for _ in 0..count{self.push(Cell::Null)?;}Ok(())}
 fn emit(&self,p:&mut RowWriter<'_,'_>,table:&str)->Result<i64>{p.insert(table,&self.values[..self.len])}
}
fn class(value:f64)->&'static str{if value.is_nan(){"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"}}
fn next<'a>(row:&'a SqliteRow,index:&mut usize)->Result<&'a SqliteValue>{let value=row.values.get(*index).ok_or_else(||invalid("BIM scalar column is absent"))?;*index+=1;Ok(value)}
trait Scalar:Sized{
 const WIDTH:usize;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>;
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>;
}
impl Scalar for String{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{c.push(Cell::Text(self))}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{match next(row,index)?{SqliteValue::Text(value)=>n.copy_text(value),_=>Err(invalid("BIM requires literal text"))}}
}
macro_rules! integer{($t:ty)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{c.push(Cell::Integer(*self as i64))}
 fn read(row:&SqliteRow,index:&mut usize,_:&mut NativeDecodeControl<'_>)->Result<Self>{match next(row,index)?{SqliteValue::Integer(value)=><$t>::try_from(*value).map_err(|e|invalid(e.to_string())),_=>Err(invalid("BIM requires an integer word"))}}
}}}
integer!(u32);integer!(i32);
impl Scalar for bool{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{c.push(Cell::Integer(i64::from(*self)))}
 fn read(row:&SqliteRow,index:&mut usize,_:&mut NativeDecodeControl<'_>)->Result<Self>{match next(row,index)?{SqliteValue::Integer(0)=>Ok(false),SqliteValue::Integer(1)=>Ok(true),_=>Err(invalid("BIM requires a canonical Boolean"))}}
}
impl Scalar for f64{
 const WIDTH:usize=3;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{c.push(if self.is_finite(){Cell::Real(*self)}else{Cell::Null})?;c.push(Cell::Integer(self.to_bits()as i64))?;c.push(Cell::Text(class(*self)))}
 fn read(row:&SqliteRow,index:&mut usize,_:&mut NativeDecodeControl<'_>)->Result<Self>{let query=next(row,index)?;let value=match next(row,index)?{SqliteValue::Integer(bits)=>f64::from_bits(*bits as u64),_=>return Err(invalid("BIM binary64 word is absent"))};let agrees=match query{SqliteValue::Null=>!value.is_finite(),SqliteValue::Real(query)=>value.is_finite()&&*query==value,SqliteValue::Integer(query)=>value.is_finite()&&value>=-9223372036854775808.0&&value<9223372036854775808.0&&value.fract()==0.0&&value as i64==*query&&*query as f64==value,_=>false};if !agrees||!matches!(next(row,index)?,SqliteValue::Text(name)if name==class(value)){return Err(invalid("BIM binary64 query or class disagrees with its word"))}Ok(value)}
}
impl<T:Scalar>Scalar for Option<T>{
 const WIDTH:usize=T::WIDTH;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Some(value)=>value.append(c),None=>c.nulls(Self::WIDTH)}}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let end=add(*index,Self::WIDTH)?;let cells=row.values.get(*index..end).ok_or_else(||invalid("BIM optional scalar is absent"))?;if cells.iter().all(|value|matches!(value,SqliteValue::Null)){*index=end;Ok(None)}else{T::read(row,index,n).map(Some)}}
}
macro_rules! enumeration{($t:ty;$($v:ident),+)=>{impl Scalar for $t{
 const WIDTH:usize=1;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{c.push(Cell::Text(match self{$(Self::$v=>stringify!($v)),+}))}
 fn read(row:&SqliteRow,index:&mut usize,_:&mut NativeDecodeControl<'_>)->Result<Self>{match next(row,index)?{SqliteValue::Text(name)=>match name.as_str(){$(stringify!($v)=>Ok(Self::$v),)+_=>Err(invalid("BIM enum symbol is undeclared"))},_=>Err(invalid("BIM enum requires text"))}}
}}}
enumeration!(MaterialCategory;Concrete,Masonry,Wood,Metal,Glass,Insulation,Finish,Membrane,Other);
enumeration!(LayerFunction;Structure,Substrate,Insulation,Finish,Membrane,Core);
enumeration!(LocationLine;Center,Interior,Exterior,CoreCenter);
enumeration!(Phase;Existing,New,Demolished,Temporary);
enumeration!(DoorLeaves;Single,Double);enumeration!(Swing;Left,Right);enumeration!(Turn;Left,Right);
enumeration!(StringerKind;None,Closed,Open,Mono);enumeration!(RiserKind;Open,Closed);
macro_rules! scalar_record{($t:ty;$($field:ident:$kind:ty),+)=>{impl Scalar for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{$(self.$field.append(c)?;)+Ok(())}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{Ok(Self{$($field:<$kind as Scalar>::read(row,index,n)?,)+})}
}}}
scalar_record!(Point2;x:f64,y:f64);scalar_record!(Rgb;r:f64,g:f64,b:f64);
scalar_record!(Vertex;point:Point2,bulge:f64);scalar_record!(Slope;direction:f64,angle:f64);
scalar_record!(StairStringer;kind:StringerKind,width:f64,depth:f64);
scalar_record!(Layer;material:String,thickness:f64,function:LayerFunction);
struct Reader<'d,'n,'p>{database:&'d SqliteDatabase,native:&'n mut NativeDecodeControl<'p>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'d SqliteRow>>>,used:BTreeSet<(&'static str,i64)>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<()>{self.native.charge(96)?;if !self.used.insert((table,row.rowid)){return Err(invalid("BIM row is multiply owned"))}Ok(())}
 fn take(&mut self,table:&'static str,parent:i64,column:usize,width:usize,ordered:bool)->Result<Vec<&'d SqliteRow>>{
  let key=(table,column);if !self.groups.contains_key(&key){let rows=&self.database.table(table)?.rows;self.native.charge(rows.len().checked_mul(256).ok_or_else(||invalid("BIM ownership workspace overflow"))?)?;let mut groups:BTreeMap<i64,Vec<&SqliteRow>>=BTreeMap::new();let mut ids=BTreeSet::new();for row in rows{self.native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err(invalid("BIM relational row shape differs"))}match row.values.get(column){Some(SqliteValue::Integer(parent))=>groups.entry(*parent).or_default().push(row),Some(SqliteValue::Null)=>{},_=>return Err(invalid("BIM ownership key is invalid"))}}self.groups.insert(key,groups);}
  let rows=self.groups.get_mut(&key).and_then(|groups|groups.remove(&parent)).unwrap_or_default();if ordered{let mut ordered=BTreeMap::new();for row in rows{self.native.step()?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;if ordered.insert(ordinal,row).is_some(){return Err(invalid("BIM relationship ordinal is repeated"))}}let mut result=self.native.allocate_vec(ordered.len())?;for(index,(ordinal,row))in ordered.into_iter().enumerate(){self.native.step()?;if ordinal!=index{return Err(invalid("BIM relationship ordinals are not dense"))}self.use_row(table,row)?;result.push(row)}Ok(result)}else{for row in &rows{self.native.step()?;self.use_row(table,row)?;}Ok(rows)}
 }
 fn optional(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<Option<&'d SqliteRow>>{let rows=self.take(table,parent,column,width,false)?;if rows.len()>1{return Err(invalid("BIM owned singleton is repeated"))}Ok(rows.into_iter().next())}
 fn required(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<&'d SqliteRow>{self.optional(table,parent,column,width)?.ok_or_else(||invalid("BIM required owned singleton is absent"))}
 fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow>{let rows=&self.database.table(table)?.rows;if rows.len()!=1||rows[0].rowid<=0||rows[0].integer(0)?!=rows[0].rowid||rows[0].values.len()!=width{return Err(invalid("BIM document singleton shape differs"))}self.use_row(table,&rows[0])?;Ok(&rows[0])}
 fn list<T:Scalar>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>>{let rows=self.take(table,parent,1,3+T::WIDTH,true)?;let mut values=self.native.allocate_vec(rows.len())?;for row in rows{self.native.step()?;let mut index=3;values.push(T::read(row,&mut index,self.native)?)}Ok(values)}
 fn finish(self)->Result<()>{let mut total=0;for table in &self.database.tables{self.native.step()?;total=add(total,table.rows.len())?;}if total!=self.used.len(){return Err(invalid("BIM contains unowned rows"))}self.native.checkpoint()}
}
fn ordinal(index:usize)->Result<i64>{i64::try_from(index).map_err(|e|invalid(e.to_string()))}
fn list<T:Scalar>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&[T])->Result<()>{for(index,value)in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;value.append(&mut c)?;c.emit(p,table)?;}Ok(())}
fn active(row:&SqliteRow,start:usize,count:usize,selected:usize)->Result<()>{for index in start..start+count{if index==selected{if !matches!(row.values.get(index),Some(SqliteValue::Integer(_))){return Err(invalid("BIM active owner is absent"))}}else if !matches!(row.values.get(index),Some(SqliteValue::Null)){return Err(invalid("BIM variant has multiple owners"))}}Ok(())}
fn absent(row:&SqliteRow,start:usize,count:usize)->Result<()>{if row.values.get(start..start+count).is_some_and(|cells|cells.iter().all(|v|matches!(v,SqliteValue::Null))){Ok(())}else{Err(invalid("BIM inactive variant columns are populated"))}}
fn owners(c:&mut Cells<'_>,parent:i64,count:usize,slot:usize)->Result<()>{for index in 0..count{c.push(if index==slot{Cell::Integer(parent)}else{Cell::Null})?;}Ok(())}
fn write_axis(p:&mut RowWriter<'_,'_>,parent:i64,value:&Axis,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,2,slot)?;match value{Axis::Line{start,end}=>{c.push(Cell::Text("Line"))?;start.append(&mut c)?;end.append(&mut c)?;c.nulls(3)?;},Axis::Arc{start,end,bulge}=>{c.push(Cell::Text("Arc"))?;start.append(&mut c)?;end.append(&mut c)?;bulge.append(&mut c)?;}}c.emit(p,"bim_axis")?;Ok(())}
fn read_axis(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<Axis>{let row=r.required("bim_axis",parent,slot+1,19)?;active(row,1,2,slot+1)?;let mut i=4;let start=Point2::read(row,&mut i,r.native)?;let end=Point2::read(row,&mut i,r.native)?;match row.text(3)?{"Line"=>{absent(row,16,3)?;Ok(Axis::Line{start,end})},"Arc"=>Ok(Axis::Arc{start,end,bulge:f64::read(row,&mut i,r.native)?}),_=>Err(invalid("BIM axis kind is undeclared"))}}
fn write_top(p:&mut RowWriter<'_,'_>,parent:i64,value:&TopConstraint,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,4,slot)?;match value{TopConstraint::Unconnected{height}=>{c.push(Cell::Text("Unconnected"))?;height.append(&mut c)?;c.nulls(4)?;},TopConstraint::StoreyTop{offset}=>{c.push(Cell::Text("StoreyTop"))?;c.nulls(3)?;offset.append(&mut c)?;c.nulls(1)?;},TopConstraint::Storey{storey,offset}=>{c.push(Cell::Text("Storey"))?;c.nulls(3)?;offset.append(&mut c)?;storey.append(&mut c)?;}}c.emit(p,"bim_top")?;Ok(())}
fn read_top(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<TopConstraint>{let row=r.required("bim_top",parent,slot+1,13)?;active(row,1,4,slot+1)?;match row.text(5)?{"Unconnected"=>{absent(row,9,4)?;let mut i=6;Ok(TopConstraint::Unconnected{height:f64::read(row,&mut i,r.native)?})},"StoreyTop"=>{absent(row,6,3)?;absent(row,12,1)?;let mut i=9;Ok(TopConstraint::StoreyTop{offset:f64::read(row,&mut i,r.native)?})},"Storey"=>{absent(row,6,3)?;let mut i=9;let offset=f64::read(row,&mut i,r.native)?;Ok(TopConstraint::Storey{storey:String::read(row,&mut i,r.native)?,offset})},_=>Err(invalid("BIM top kind is undeclared"))}}
fn write_profile(p:&mut RowWriter<'_,'_>,parent:i64,value:&Profile,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,6,slot)?;match value{
 Profile::Rectangle{width,depth}=>{c.push(Cell::Text("Rectangle"))?;width.append(&mut c)?;depth.append(&mut c)?;c.nulls(9)?;},
 Profile::Circle{diameter}=>{c.push(Cell::Text("Circle"))?;c.nulls(6)?;diameter.append(&mut c)?;c.nulls(6)?;},
 Profile::IShape{width,depth,web,flange}=>{c.push(Cell::Text("IShape"))?;width.append(&mut c)?;depth.append(&mut c)?;c.nulls(3)?;web.append(&mut c)?;flange.append(&mut c)?;},
 Profile::Custom{..}=>{c.push(Cell::Text("Custom"))?;c.nulls(15)?;}}
 let id=c.emit(p,"bim_profile")?;if let Profile::Custom{outline}=value{list(p,"bim_profile_outline",id,outline)?;}Ok(())}
fn read_profile(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<Profile>{let row=r.required("bim_profile",parent,slot+1,23)?;active(row,1,6,slot+1)?;let outline=r.list::<Vertex>("bim_profile_outline",row.rowid)?;if row.text(7)?!="Custom"&&!outline.is_empty(){return Err(invalid("BIM noncustom profile owns an outline"))}let mut i=8;match row.text(7)?{
 "Rectangle"=>{absent(row,14,9)?;Ok(Profile::Rectangle{width:f64::read(row,&mut i,r.native)?,depth:f64::read(row,&mut i,r.native)?})},
 "Circle"=>{absent(row,8,6)?;absent(row,17,6)?;i=14;Ok(Profile::Circle{diameter:f64::read(row,&mut i,r.native)?})},
 "IShape"=>{absent(row,14,3)?;let width=f64::read(row,&mut i,r.native)?;let depth=f64::read(row,&mut i,r.native)?;i=17;Ok(Profile::IShape{width,depth,web:f64::read(row,&mut i,r.native)?,flange:f64::read(row,&mut i,r.native)?})},
 "Custom"=>{absent(row,8,15)?;Ok(Profile::Custom{outline})},_=>Err(invalid("BIM profile kind is undeclared"))}}
impl Scalar for RoofShape{
 const WIDTH:usize=19;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Flat=>{c.push(Cell::Text("Flat"))?;c.nulls(18)},Self::Shed{pitch,direction}=>{c.push(Cell::Text("Shed"))?;pitch.append(c)?;direction.append(c)?;c.nulls(12)},Self::Gable{pitch,ridge_direction}=>{c.push(Cell::Text("Gable"))?;pitch.append(c)?;c.nulls(3)?;ridge_direction.append(c)?;c.nulls(9)},Self::Hip{pitch}=>{c.push(Cell::Text("Hip"))?;pitch.append(c)?;c.nulls(15)},Self::Mansard{lower_pitch,upper_pitch,break_height}=>{c.push(Cell::Text("Mansard"))?;c.nulls(9)?;lower_pitch.append(c)?;upper_pitch.append(c)?;break_height.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;let name=next(row,i)?;let end=start+19;let output=match name{SqliteValue::Text(name)=>match name.as_str(){"Flat"=>{absent(row,start+1,18)?;Self::Flat},"Shed"=>{absent(row,start+7,12)?;Self::Shed{pitch:f64::read(row,i,n)?,direction:f64::read(row,i,n)?}},"Gable"=>{absent(row,start+4,3)?;absent(row,start+10,9)?;let pitch=f64::read(row,i,n)?;*i=start+7;Self::Gable{pitch,ridge_direction:f64::read(row,i,n)?}},"Hip"=>{absent(row,start+4,15)?;Self::Hip{pitch:f64::read(row,i,n)?}},"Mansard"=>{absent(row,start+1,9)?;*i=start+10;Self::Mansard{lower_pitch:f64::read(row,i,n)?,upper_pitch:f64::read(row,i,n)?,break_height:f64::read(row,i,n)?}},_=>return Err(invalid("BIM roof kind is undeclared"))},_=>return Err(invalid("BIM roof kind requires text"))};*i=end;Ok(output)}
}
impl Scalar for OpeningKind{
 const WIDTH:usize=9;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Window{window_type}=>{c.push(Cell::Text("Window"))?;window_type.append(c)?;c.nulls(7)},Self::Door{door_type}=>{c.push(Cell::Text("Door"))?;c.nulls(1)?;door_type.append(c)?;c.nulls(6)},Self::Void{width,height}=>{c.push(Cell::Text("Void"))?;c.nulls(2)?;width.append(c)?;height.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;let name=next(row,i)?;let output=match name{SqliteValue::Text(name)=>match name.as_str(){"Window"=>{absent(row,start+2,7)?;Self::Window{window_type:String::read(row,i,n)?}},"Door"=>{absent(row,start+1,1)?;absent(row,start+3,6)?;*i=start+2;Self::Door{door_type:String::read(row,i,n)?}},"Void"=>{absent(row,start+1,2)?;*i=start+3;Self::Void{width:f64::read(row,i,n)?,height:f64::read(row,i,n)?}},_=>return Err(invalid("BIM opening kind is undeclared"))},_=>return Err(invalid("BIM opening kind requires text"))};*i=start+9;Ok(output)}
}
impl Scalar for StairFlight{
 const WIDTH:usize=14;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Straight=>{c.push(Cell::Text("Straight"))?;c.nulls(13)},Self::LTurn{split,turn}=>{c.push(Cell::Text("LTurn"))?;split.append(c)?;turn.append(c)?;c.nulls(9)},Self::UTurn{gap}=>{c.push(Cell::Text("UTurn"))?;c.nulls(4)?;gap.append(c)?;c.nulls(6)},Self::Spiral{radius,sweep}=>{c.push(Cell::Text("Spiral"))?;c.nulls(7)?;radius.append(c)?;sweep.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;let name=next(row,i)?;let output=match name{SqliteValue::Text(name)=>match name.as_str(){"Straight"=>{absent(row,start+1,13)?;Self::Straight},"LTurn"=>{absent(row,start+5,9)?;Self::LTurn{split:f64::read(row,i,n)?,turn:Turn::read(row,i,n)?}},"UTurn"=>{absent(row,start+1,4)?;absent(row,start+8,6)?;*i=start+5;Self::UTurn{gap:f64::read(row,i,n)?}},"Spiral"=>{absent(row,start+1,7)?;*i=start+8;Self::Spiral{radius:f64::read(row,i,n)?,sweep:f64::read(row,i,n)?}},_=>return Err(invalid("BIM flight kind is undeclared"))},_=>return Err(invalid("BIM flight kind requires text"))};*i=start+14;Ok(output)}
}
impl Scalar for Infill{
 const WIDTH:usize=4;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::None=>{c.push(Cell::Text("None"))?;c.nulls(3)},Self::Glass{thickness}=>{c.push(Cell::Text("Glass"))?;thickness.append(c)},Self::Panel{thickness}=>{c.push(Cell::Text("Panel"))?;thickness.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;match next(row,i)?{SqliteValue::Text(name)=>match name.as_str(){"None"=>{absent(row,start+1,3)?;*i=start+4;Ok(Self::None)},"Glass"=>Ok(Self::Glass{thickness:f64::read(row,i,n)?}),"Panel"=>Ok(Self::Panel{thickness:f64::read(row,i,n)?}),_=>Err(invalid("BIM infill kind is undeclared"))},_=>Err(invalid("BIM infill requires text"))}}
}
trait Entity:Sized{
 const WIDTH:usize;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>;
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>;
 fn read(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Self>;
}
fn write_list<T:Scalar>(p:&mut RowWriter<'_,'_>,parent:i64,value:&Vec<T>,table:&str)->Result<()>{list(p,table,parent,value)}
fn read_list<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,table:&'static str)->Result<Vec<T>>{r.list(table,parent)}
macro_rules! entity{($t:ty;$($field:ident:$kind:ty),+$(; $($child:ident:$nested:ty=>$write:ident($wa:expr),$read:ident($ra:expr)),+)?)=>{impl Entity for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{$(self.$field.append(c)?;)+Ok(())}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{$( $($write(p,parent,&self.$child,$wa)?;)+ )?let _=(p,parent);Ok(())}
 fn read(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Self>{r.native.charge(std::mem::size_of::<Self>())?;let mut i=4;$(let $field=<$kind as Scalar>::read(row,&mut i,r.native)?;)+if i!=row.values.len(){return Err(invalid("BIM entity has extra scalar columns"))}Ok(Self{$($field,)+$( $($child:$read(r,row.rowid,$ra)?,)+ )?})}
}}}
entity!(Material;name:String,category:MaterialCategory,color:Rgb,density:f64,conductivity:f64,specific_heat:f64);
entity!(WallType;name:String;layers:Vec<Layer>=>write_list("bim_wall_layer"),read_list("bim_wall_layer"));
entity!(SlabType;name:String;layers:Vec<Layer>=>write_list("bim_slab_layer"),read_list("bim_slab_layer"));
entity!(RoofType;name:String;layers:Vec<Layer>=>write_list("bim_roof_layer"),read_list("bim_roof_layer"));
entity!(ColumnType;name:String,material:String;profile:Profile=>write_profile(0),read_profile(0));
entity!(BeamType;name:String,material:String;profile:Profile=>write_profile(1),read_profile(1));
entity!(WindowType;name:String,width:f64,height:f64,sill:f64,frame_width:f64,frame_depth:f64,panes:u32,material:String);
entity!(DoorType;name:String,width:f64,height:f64,frame_width:f64,frame_depth:f64,leaves:DoorLeaves,swing:Swing,material:String);
entity!(Site;name:String,latitude:f64,longitude:f64,elevation:f64,true_north:f64;boundary:Vec<Point2>=>write_list("bim_site_boundary"),read_list("bim_site_boundary"));
entity!(Building;site:String,name:String,origin:Point2,rotation:f64,elevation:f64);
entity!(Storey;building:String,name:String,level:i32,height:f64,cut_height:Option<f64>);
entity!(GridLine;building:String,label:String,start:Point2,end:Point2);
entity!(Wall;storey:String,wall_type:String,location:LocationLine,base_offset:f64,phase:Phase,name:String;axis:Axis=>write_axis(0),read_axis(0),top:TopConstraint=>write_top(0),read_top(0));
entity!(CurtainWall;storey:String,base_offset:f64,u_spacing:f64,v_spacing:f64,panel_material:String,mullion_material:String,name:String;axis:Axis=>write_axis(1),read_axis(1),top:TopConstraint=>write_top(1),read_top(1),mullion:Profile=>write_profile(2),read_profile(2));
entity!(Column;storey:String,column_type:String,position:Point2,rotation:f64,base_offset:f64,name:String;top:TopConstraint=>write_top(2),read_top(2));
entity!(Beam;storey:String,beam_type:String,start:Point2,end:Point2,top_offset:f64,name:String);
fn write_holes(p:&mut RowWriter<'_,'_>,parent:i64,value:&Vec<Vec<Vertex>>,_:())->Result<()>{for(index,hole)in value.iter().enumerate(){let id=p.insert("bim_slab_hole",&[Cell::Integer(parent),Cell::Integer(ordinal(index)?)])?;list(p,"bim_slab_hole_vertex",id,hole)?;}Ok(())}
fn read_holes(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Vec<Vec<Vertex>>>{let rows=r.take("bim_slab_hole",parent,1,3,true)?;let mut values=r.native.allocate_vec(rows.len())?;for row in rows{r.native.step()?;values.push(r.list("bim_slab_hole_vertex",row.rowid)?)}Ok(values)}
entity!(Slab;storey:String,slab_type:String,offset:f64,slope:Option<Slope>,name:String;boundary:Vec<Vertex>=>write_list("bim_slab_boundary"),read_list("bim_slab_boundary"),holes:Vec<Vec<Vertex>>=>write_holes(()),read_holes(()));
entity!(Roof;storey:String,roof_type:String,shape:RoofShape,overhang:f64,base_offset:f64,name:String;footprint:Vec<Vertex>=>write_list("bim_roof_footprint"),read_list("bim_roof_footprint"));
entity!(Opening;host:String,kind:OpeningKind,offset:f64,sill_override:Option<f64>,width:Option<f64>,height:Option<f64>,flip_hand:bool,flip_facing:bool,name:String);
entity!(Stair;storey:String,start:Point2,direction:f64,width:f64,flight:StairFlight,max_riser:f64,min_tread:f64,stringer:StairStringer,nosing:f64,tread_thickness:f64,riser:RiserKind,landing_depth:f64,name:String;top:TopConstraint=>write_top(3),read_top(3));
fn write_baluster(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<Baluster>,_:())->Result<()>{if let Some(value)=value{let mut c=Cells::new();c.push(Cell::Integer(parent))?;value.spacing.append(&mut c)?;let id=c.emit(p,"bim_baluster")?;write_profile(p,id,&value.profile,5)?;}Ok(())}
fn read_baluster(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Option<Baluster>>{let Some(row)=r.optional("bim_baluster",parent,1,5)?else{return Ok(None)};let mut i=2;let spacing=f64::read(row,&mut i,r.native)?;Ok(Some(Baluster{spacing,profile:read_profile(r,row.rowid,5)?}))}
entity!(Railing;storey:String,height:f64,post_spacing:f64,infill:Infill,material:String,base_offset:f64,name:String;path:Vec<Point2>=>write_list("bim_railing_path"),read_list("bim_railing_path"),profile:Profile=>write_profile(3),read_profile(3),post_profile:Profile=>write_profile(4),read_profile(4),baluster:Option<Baluster>=>write_baluster(()),read_baluster(()));
entity!(Classification;system:String,code:String,title:String);
impl Entity for Space{
 const WIDTH:usize=11;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{self.storey.append(c)?;self.number.append(c)?;self.name.append(c)?;match &self.boundary{SpaceBoundary::Bounded{seed}=>{c.push(Cell::Text("Bounded"))?;seed.append(c)?;},SpaceBoundary::Explicit{..}=>{c.push(Cell::Text("Explicit"))?;c.nulls(6)?;}}self.usage.append(c)}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{if let SpaceBoundary::Explicit{outline}=&self.boundary{list(p,"bim_space_outline",parent,outline)?;}Ok(())}
 fn read(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Self>{let mut i=4;let storey=String::read(row,&mut i,r.native)?;let number=String::read(row,&mut i,r.native)?;let name=String::read(row,&mut i,r.native)?;let kind=row.text(i)?;i+=1;let outline=r.list("bim_space_outline",row.rowid)?;let boundary=match kind{"Bounded"=>{if !outline.is_empty(){return Err(invalid("BIM bounded space owns an outline"))}SpaceBoundary::Bounded{seed:Point2::read(row,&mut i,r.native)?}},"Explicit"=>{absent(row,i,6)?;i+=6;SpaceBoundary::Explicit{outline}},_=>return Err(invalid("BIM space boundary kind is undeclared"))};let usage=String::read(row,&mut i,r.native)?;Ok(Self{storey,number,name,boundary,usage})}
}
fn write_map<T:Entity>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&BTreeMap<String,T>)->Result<()>{for(index,(key,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(key))?;value.fields(&mut c)?;let id=c.emit(p,table)?;value.children(p,id)?;}Ok(())}
fn read_map<T:Entity>(r:&mut Reader<'_,'_,'_>,table:&'static str,parent:i64)->Result<BTreeMap<String,T>>{let rows=r.take(table,parent,1,T::WIDTH+4,true)?;let mut result=BTreeMap::new();let mut previous:Option<&str>=None;for row in rows{r.native.step()?;let key=row.text(3)?;if previous.is_some_and(|previous|previous>=key){return Err(invalid("BIM map keys are not strictly sorted"))}previous=Some(key);r.native.charge(std::mem::size_of::<(String,T)>()+96)?;let key=r.native.copy_text(key)?;let value=T::read(r,row)?;if result.insert(key,value).is_some(){return Err(invalid("BIM map key is repeated"))}}Ok(result)}
fn write_property<'a>(value:&'a PropertyValue,c:&mut Cells<'a>)->Result<()>{match value{
 PropertyValue::Text{value}=>{c.push(Cell::Text("Text"))?;value.append(c)?;c.nulls(5)},
 PropertyValue::Integer{value}=>{c.push(Cell::Text("Integer"))?;c.nulls(1)?;value.append(c)?;c.nulls(4)},
 PropertyValue::Boolean{value}=>{c.push(Cell::Text("Boolean"))?;c.nulls(2)?;value.append(c)?;c.nulls(3)},
 PropertyValue::Real{value}=>write_measure(c,"Real",value),PropertyValue::Length{value}=>write_measure(c,"Length",value),PropertyValue::Area{value}=>write_measure(c,"Area",value),PropertyValue::Volume{value}=>write_measure(c,"Volume",value),PropertyValue::Angle{value}=>write_measure(c,"Angle",value)
}}
fn write_measure<'a>(c:&mut Cells<'a>,kind:&'static str,value:&'a f64)->Result<()>{c.push(Cell::Text(kind))?;c.nulls(3)?;value.append(c)}
fn read_property(row:&SqliteRow,n:&mut NativeDecodeControl<'_>)->Result<PropertyValue>{match row.text(4)?{
 "Text"=>{absent(row,6,5)?;let mut i=5;Ok(PropertyValue::Text{value:String::read(row,&mut i,n)?})},
 "Integer"=>{absent(row,5,1)?;absent(row,7,4)?;let mut i=6;Ok(PropertyValue::Integer{value:i32::read(row,&mut i,n)?})},
 "Boolean"=>{absent(row,5,2)?;absent(row,8,3)?;let mut i=7;Ok(PropertyValue::Boolean{value:bool::read(row,&mut i,n)?})},
 kind@("Real"|"Length"|"Area"|"Volume"|"Angle")=>{absent(row,5,3)?;let mut i=8;let value=f64::read(row,&mut i,n)?;Ok(match kind{"Real"=>PropertyValue::Real{value},"Length"=>PropertyValue::Length{value},"Area"=>PropertyValue::Area{value},"Volume"=>PropertyValue::Volume{value},_=>PropertyValue::Angle{value}})},_=>Err(invalid("BIM typed property kind is undeclared"))}}
fn visit_rows(snapshot:&ModelSnapshot,p:&mut RowWriter<'_,'_>)->Result<()>{
 let root=p.insert("bim_document",&[Cell::Text(&snapshot.schema)])?;
 let project=p.insert("bim_project",&[Cell::Integer(root),Cell::Text(&snapshot.project.name),Cell::Text(&snapshot.project.description),Cell::Text(&snapshot.project.author),Cell::Text(&snapshot.project.organization)])?;
 list(p,"bim_phase_name",project,&snapshot.project.phase_names)?;
 write_map(p,"bim_material",root,&snapshot.materials)?;write_map(p,"bim_wall_type",root,&snapshot.wall_types)?;write_map(p,"bim_slab_type",root,&snapshot.slab_types)?;write_map(p,"bim_roof_type",root,&snapshot.roof_types)?;
 write_map(p,"bim_column_type",root,&snapshot.column_types)?;write_map(p,"bim_beam_type",root,&snapshot.beam_types)?;write_map(p,"bim_window_type",root,&snapshot.window_types)?;write_map(p,"bim_door_type",root,&snapshot.door_types)?;
 write_map(p,"bim_site",root,&snapshot.sites)?;write_map(p,"bim_building",root,&snapshot.buildings)?;write_map(p,"bim_storey",root,&snapshot.storeys)?;write_map(p,"bim_grid",root,&snapshot.grids)?;
 write_map(p,"bim_wall",root,&snapshot.walls)?;write_map(p,"bim_curtain_wall",root,&snapshot.curtain_walls)?;write_map(p,"bim_column",root,&snapshot.columns)?;write_map(p,"bim_beam",root,&snapshot.beams)?;
 write_map(p,"bim_slab",root,&snapshot.slabs)?;write_map(p,"bim_roof",root,&snapshot.roofs)?;write_map(p,"bim_opening",root,&snapshot.openings)?;write_map(p,"bim_stair",root,&snapshot.stairs)?;
 write_map(p,"bim_railing",root,&snapshot.railings)?;write_map(p,"bim_space",root,&snapshot.spaces)?;write_map(p,"bim_classification",root,&snapshot.classifications)?;
 for(index,(key,sets))in snapshot.properties.iter().enumerate(){let element=p.insert("bim_property_element",&[Cell::Integer(root),Cell::Integer(ordinal(index)?),Cell::Text(key)])?;for(index,(name,values))in sets.iter().enumerate(){let set=p.insert("bim_property_set",&[Cell::Integer(element),Cell::Integer(ordinal(index)?),Cell::Text(name)])?;for(index,(name,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(set))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(name))?;write_property(value,&mut c)?;c.emit(p,"bim_property_value")?;}}}
 Ok(())
}
fn project(snapshot:&ModelSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::new(SCHEMA,c)?;visit_rows(snapshot,&mut p)?;p.finish()}
fn admit(snapshot:&ModelSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<()>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::borrowed(c,SqliteSnapshotPhase::ProjectSnapshot)?;visit_rows(snapshot,&mut p)?;p.finish_borrowed()}
fn read_properties(r:&mut Reader<'_,'_,'_>,parent:i64)->Result<BTreeMap<String,PropertySet>>{
 let elements=r.take("bim_property_element",parent,1,4,true)?;let mut result=BTreeMap::new();let mut previous=None;for element in elements{let key=element.text(3)?;if previous.is_some_and(|v|v>=key){return Err(invalid("BIM property element keys are not sorted"))}previous=Some(key);let key=r.native.copy_text(key)?;r.native.charge(96+std::mem::size_of::<PropertySet>())?;let sets=r.take("bim_property_set",element.rowid,1,4,true)?;let mut values=BTreeMap::new();let mut previous=None;for set in sets{let name=set.text(3)?;if previous.is_some_and(|v|v>=name){return Err(invalid("BIM property set names are not sorted"))}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<BTreeMap<String,PropertyValue>>())?;let properties=r.take("bim_property_value",set.rowid,1,11,true)?;let mut contents=BTreeMap::new();let mut previous=None;for row in properties{let name=row.text(3)?;if previous.is_some_and(|v|v>=name){return Err(invalid("BIM property names are not sorted"))}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<PropertyValue>())?;contents.insert(name,read_property(row,r.native)?);}values.insert(name,contents);}result.insert(key,values);}Ok(result)
}
fn restore(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<ModelSnapshot>{
 semantic_cells::extent(c.limits())?;c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;store::sqlite_snapshot::validate_sqlite_database_schema(database,SCHEMA,c.limits())?;
 let maximum=c.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(maximum,&mut progress);native.charge(std::mem::size_of::<ModelSnapshot>())?;let mut r=Reader{database,native:&mut native,groups:BTreeMap::new(),used:BTreeSet::new()};let document=r.singleton("bim_document",2)?;let root=document.rowid;let schema=r.native.copy_text(document.text(1)?)?;let row=r.required("bim_project",root,1,6)?;let mut i=2;let project=Project{name:String::read(row,&mut i,r.native)?,description:String::read(row,&mut i,r.native)?,author:String::read(row,&mut i,r.native)?,organization:String::read(row,&mut i,r.native)?,phase_names:r.list("bim_phase_name",row.rowid)?};
 let result=ModelSnapshot{schema,project,
 materials:read_map(&mut r,"bim_material",root)?,wall_types:read_map(&mut r,"bim_wall_type",root)?,slab_types:read_map(&mut r,"bim_slab_type",root)?,roof_types:read_map(&mut r,"bim_roof_type",root)?,
 column_types:read_map(&mut r,"bim_column_type",root)?,beam_types:read_map(&mut r,"bim_beam_type",root)?,window_types:read_map(&mut r,"bim_window_type",root)?,door_types:read_map(&mut r,"bim_door_type",root)?,
 sites:read_map(&mut r,"bim_site",root)?,buildings:read_map(&mut r,"bim_building",root)?,storeys:read_map(&mut r,"bim_storey",root)?,grids:read_map(&mut r,"bim_grid",root)?,
 walls:read_map(&mut r,"bim_wall",root)?,curtain_walls:read_map(&mut r,"bim_curtain_wall",root)?,columns:read_map(&mut r,"bim_column",root)?,beams:read_map(&mut r,"bim_beam",root)?,
 slabs:read_map(&mut r,"bim_slab",root)?,roofs:read_map(&mut r,"bim_roof",root)?,openings:read_map(&mut r,"bim_opening",root)?,stairs:read_map(&mut r,"bim_stair",root)?,
 railings:read_map(&mut r,"bim_railing",root)?,spaces:read_map(&mut r,"bim_space",root)?,properties:read_properties(&mut r,root)?,classifications:read_map(&mut r,"bim_classification",root)?};r.finish()?;Ok(result)
}
/// 🪶️ One authored relational visitor admits every native and public snapshot route.
impl store::ArtifactSqliteSnapshot for ModelSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<()>{admit(self,c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{project(self,c)}
 fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self>{restore(database,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload>{admit(self,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self>{let limits=c.limits();semantic_cells::extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{semantic_cells::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)},c)}
}
