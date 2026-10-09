//! 🏢️ Literal BIM fields, exact scalar words and owned authored relationships.
use crate::*;
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl};
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
pub const SCHEMA:&str=include_str!("🗄️.sql");
#[path="📏️cells/🦀️.rs"]mod semantic_cells;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"BIM extent overflow"))}
fn compare_text(left:&str,right:&str,n:&mut NativeDecodeControl<'_>)->Result<std::cmp::Ordering>{n.scoped_stage(|n|{let length=left.len().min(right.len());n.begin_stage(length)?;let mut offset=0;while offset<length{let end=add(offset,(length-offset).min(65536))?;let order=left.as_bytes()[offset..end].cmp(&right.as_bytes()[offset..end]);n.advance(end-offset)?;if order!=std::cmp::Ordering::Equal{return Ok(order)}offset=end;}n.checkpoint()?;Ok(left.len().cmp(&right.len()))})}
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
enumeration!(PropertyKind;Text,Real,Integer,Boolean,Length,Area,Volume,Angle);
enumeration!(TemplateTarget;Site,Building,Storey,Wall,CurtainWall,Column,Beam,Slab,Ceiling,Roof,Window,Door,Void,Stair,Ramp,Railing,Space,Zone,WallType,SlabType,CeilingType,RoofType,ColumnType,BeamType,WindowType,DoorType);
enumeration!(AnchorEnd;Start,End);enumeration!(WallSide;Left,Right);enumeration!(TagCategory;Name,Type,Number,Size);
enumeration!(Terminator;Tick,Arrow,Dot);enumeration!(DimensionUnit;Metre,Centimetre,Millimetre);
enumeration!(FamilyCategory;Furniture,Equipment,Casework,Plumbing,Lighting,Mechanical,Electrical,Generic,Profile);
enumeration!(ParameterKind;Length,Angle,Real,Integer,Boolean,Text,Material);enumeration!(SolidAxis;X,Y,Z);
enumeration!(IsoSize;A0,A1,A2,A3,A4);enumeration!(Orientation;Landscape,Portrait);
enumeration!(ScheduleCategory;Wall,CurtainWall,Slab,Roof,Column,Beam,Window,Door,Void,Stair,Railing,Space,Finish,Material);
enumeration!(ScheduleField;Id,Name,Kind,Storey,Level,Type,Phase,Material,Host,Number,Usage,Surface,Swing,Leaves,Panes,Count,Length,Width,Height,Perimeter,GrossSideArea,OpeningArea,NetSideArea,GrossArea,NetArea,SurfaceArea,GrossVolume,NetVolume,Mass,Risers,Thickness,LayerArea,LayerVolume,LayerMass,FinishArea);
enumeration!(ScheduleOp;Equals,NotEquals,Contains,Greater,GreaterOrEqual,Less,LessOrEqual,Empty,NotEmpty);
enumeration!(EndJoin;Miter,Butt,None);enumeration!(ViewKind;Plan,CeilingPlan,Section,Elevation,Orthographic,Perspective);
enumeration!(DetailLevel;Coarse,Medium,Fine);enumeration!(ViewCategory;Walls,CurtainWalls,Columns,Beams,Slabs,Roofs,Openings,Stairs,Railings,Spaces,Grids);
enumeration!(HostSide;Left,Right);enumeration!(AreaMeasure;Gross,Net);
macro_rules! scalar_record{($t:ty;$($field:ident:$kind:ty),+)=>{impl Scalar for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{$(self.$field.append(c)?;)+Ok(())}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{Ok(Self{$($field:<$kind as Scalar>::read(row,index,n)?,)+})}
}}}
scalar_record!(Point2;x:f64,y:f64);scalar_record!(Rgb;r:f64,g:f64,b:f64);
scalar_record!(Vertex;point:Point2,bulge:f64);scalar_record!(Slope;direction:f64,angle:f64);
scalar_record!(StairStringer;kind:StringerKind,width:f64,depth:f64);
scalar_record!(Layer;material:String,thickness:f64,function:LayerFunction);
scalar_record!(ExprPoint;x:String,y:String);scalar_record!(ExprPoint3;x:String,y:String,z:String);
scalar_record!(ViewPlane;start:Point2,end:Point2);scalar_record!(ViewCrop;min:Point2,max:Point2);
scalar_record!(ViewCamera;target:Point2,target_height:f64,azimuth:f64,pitch:f64,distance:f64);
scalar_record!(RailingHost;element:String,side:HostSide,edge:u32,inset:f64);
scalar_record!(ClassificationItem;code:String,title:String,parent:Option<String>);
struct Reader<'d,'n,'p>{database:&'d SqliteDatabase,native:&'n mut NativeDecodeControl<'p>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'d SqliteRow>>>,used:BTreeSet<(&'static str,i64)>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<()>{self.native.charge(96)?;if !self.used.insert((table,row.rowid)){return Err(invalid("BIM row is multiply owned"))}Ok(())}
 fn take(&mut self,table:&'static str,parent:i64,column:usize,width:usize,ordered:Option<usize>)->Result<Vec<&'d SqliteRow>>{
  let key=(table,column);if !self.groups.contains_key(&key){let rows=&self.database.table(table)?.rows;self.native.charge(rows.len().checked_mul(256).ok_or_else(||invalid("BIM ownership workspace overflow"))?)?;let mut groups:BTreeMap<i64,Vec<&SqliteRow>>=BTreeMap::new();let mut ids=BTreeSet::new();for row in rows{self.native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err(invalid("BIM relational row shape differs"))}match row.values.get(column){Some(SqliteValue::Integer(parent))=>groups.entry(*parent).or_default().push(row),Some(SqliteValue::Null)=>{},_=>return Err(invalid("BIM ownership key is invalid"))}}self.groups.insert(key,groups);}
  let rows=self.groups.get_mut(&key).and_then(|groups|groups.remove(&parent)).unwrap_or_default();if let Some(order_column)=ordered{let mut ordered=BTreeMap::new();for row in rows{self.native.step()?;let ordinal=usize::try_from(row.integer(order_column)?).map_err(|e|invalid(e.to_string()))?;if ordered.insert(ordinal,row).is_some(){return Err(invalid("BIM relationship ordinal is repeated"))}}let mut result=self.native.allocate_vec(ordered.len())?;for(index,(ordinal,row))in ordered.into_iter().enumerate(){self.native.step()?;if ordinal!=index{return Err(invalid("BIM relationship ordinals are not dense"))}self.use_row(table,row)?;result.push(row)}Ok(result)}else{for row in &rows{self.native.step()?;self.use_row(table,row)?;}Ok(rows)}
 }
 fn optional(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<Option<&'d SqliteRow>>{let rows=self.take(table,parent,column,width,None)?;if rows.len()>1{return Err(invalid("BIM owned singleton is repeated"))}Ok(rows.into_iter().next())}
 fn required(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<&'d SqliteRow>{self.optional(table,parent,column,width)?.ok_or_else(||invalid("BIM required owned singleton is absent"))}
 fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow>{let rows=&self.database.table(table)?.rows;if rows.len()!=1||rows[0].rowid<=0||rows[0].integer(0)?!=rows[0].rowid||rows[0].values.len()!=width{return Err(invalid("BIM document singleton shape differs"))}self.use_row(table,&rows[0])?;Ok(&rows[0])}
 fn list<T:Scalar>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>>{let rows=self.take(table,parent,1,3+T::WIDTH,Some(2))?;let mut values=self.native.allocate_vec(rows.len())?;for row in rows{self.native.step()?;let mut index=3;values.push(T::read(row,&mut index,self.native)?)}Ok(values)}
 fn finish(self)->Result<()>{let mut total=0;for table in &self.database.tables{self.native.step()?;total=add(total,table.rows.len())?;}if total!=self.used.len(){return Err(invalid("BIM contains unowned rows"))}self.native.checkpoint()}
}
fn ordinal(index:usize)->Result<i64>{i64::try_from(index).map_err(|e|invalid(e.to_string()))}
fn list<T:Scalar>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&[T])->Result<()>{for(index,value)in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;value.append(&mut c)?;c.emit(p,table)?;}Ok(())}
fn active(row:&SqliteRow,start:usize,count:usize,selected:usize)->Result<()>{for index in start..start+count{if index==selected{if !matches!(row.values.get(index),Some(SqliteValue::Integer(_))){return Err(invalid("BIM active owner is absent"))}}else if !matches!(row.values.get(index),Some(SqliteValue::Null)){return Err(invalid("BIM variant has multiple owners"))}}Ok(())}
fn absent(row:&SqliteRow,start:usize,count:usize)->Result<()>{if row.values.get(start..start+count).is_some_and(|cells|cells.iter().all(|v|matches!(v,SqliteValue::Null))){Ok(())}else{Err(invalid("BIM inactive variant columns are populated"))}}
fn owners(c:&mut Cells<'_>,parent:i64,count:usize,slot:usize)->Result<()>{for index in 0..count{c.push(if index==slot{Cell::Integer(parent)}else{Cell::Null})?;}Ok(())}
fn write_axis(p:&mut RowWriter<'_,'_>,parent:i64,value:&Axis,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,3,slot)?;match value{Axis::Line{start,end}=>{c.push(Cell::Text("Line"))?;start.append(&mut c)?;end.append(&mut c)?;c.nulls(3)?;},Axis::Arc{start,end,bulge}=>{c.push(Cell::Text("Arc"))?;start.append(&mut c)?;end.append(&mut c)?;bulge.append(&mut c)?;}}c.emit(p,"bim_axis")?;Ok(())}
fn read_axis(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<Axis>{let row=r.required("bim_axis",parent,slot+1,20)?;active(row,1,3,slot+1)?;let mut i=5;let start=Point2::read(row,&mut i,r.native)?;let end=Point2::read(row,&mut i,r.native)?;match row.text(4)?{"Line"=>{absent(row,17,3)?;Ok(Axis::Line{start,end})},"Arc"=>Ok(Axis::Arc{start,end,bulge:f64::read(row,&mut i,r.native)?}),_=>Err(invalid("BIM axis kind is undeclared"))}}
fn write_top(p:&mut RowWriter<'_,'_>,parent:i64,value:&TopConstraint,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,5,slot)?;let(kind,height,offset,target)=match value{
 TopConstraint::Unconnected{height}=>("Unconnected",Some(height),None,None),
 TopConstraint::StoreyTop{offset}=>("StoreyTop",None,Some(offset),None),
 TopConstraint::Storey{storey,offset}=>("Storey",None,Some(offset),Some((0,storey))),
 TopConstraint::Roof{roof,offset}=>("Roof",None,Some(offset),Some((1,roof))),
 TopConstraint::Slab{slab,offset}=>("Slab",None,Some(offset),Some((2,slab))),
 TopConstraint::Ceiling{ceiling,offset}=>("Ceiling",None,Some(offset),Some((3,ceiling)))};
 c.push(Cell::Text(kind))?;if let Some(value)=height{value.append(&mut c)?}else{c.nulls(3)?}if let Some(value)=offset{value.append(&mut c)?}else{c.nulls(3)?}for index in 0..4{c.push(match target{Some((selected,value))if selected==index=>Cell::Text(value),_=>Cell::Null})?;}c.emit(p,"bim_top")?;Ok(())}
fn read_top(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<TopConstraint>{let row=r.required("bim_top",parent,slot+1,17)?;active(row,1,5,slot+1)?;let kind=row.text(6)?;if kind=="Unconnected"{absent(row,10,7)?;let mut i=7;return Ok(TopConstraint::Unconnected{height:f64::read(row,&mut i,r.native)?})}absent(row,7,3)?;let mut i=10;let offset=f64::read(row,&mut i,r.native)?;let selected=match kind{"StoreyTop"=>None,"Storey"=>Some(13),"Roof"=>Some(14),"Slab"=>Some(15),"Ceiling"=>Some(16),_=>return Err(invalid("BIM top kind is undeclared"))};for index in 13..17{if Some(index)!=selected{absent(row,index,1)?;}}if let Some(index)=selected{i=index;let value=String::read(row,&mut i,r.native)?;Ok(match kind{"Storey"=>TopConstraint::Storey{storey:value,offset},"Roof"=>TopConstraint::Roof{roof:value,offset},"Slab"=>TopConstraint::Slab{slab:value,offset},_=>TopConstraint::Ceiling{ceiling:value,offset}})}else{Ok(TopConstraint::StoreyTop{offset})}}
fn write_profile(p:&mut RowWriter<'_,'_>,parent:i64,value:&Profile,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,8,slot)?;let(kind,values,family):(&str,[Option<&f64>;5],Option<&String>)=match value{
 Profile::Rectangle{width,depth}=>("Rectangle",[Some(width),Some(depth),None,None,None],None),
 Profile::Circle{diameter}=>("Circle",[None,None,Some(diameter),None,None],None),
 Profile::IShape{width,depth,web,flange}=>("IShape",[Some(width),Some(depth),None,Some(web),Some(flange)],None),
 Profile::Custom{..}=>("Custom",[None;5],None),Profile::Family{family}=>("Family",[None;5],Some(family))};
 c.push(Cell::Text(kind))?;for value in values{if let Some(value)=value{value.append(&mut c)?;}else{c.nulls(3)?;}}if let Some(family)=family{family.append(&mut c)?;}else{c.nulls(1)?;}let id=c.emit(p,"bim_profile")?;if let Profile::Custom{outline}=value{list(p,"bim_profile_outline",id,outline)?;}Ok(())}
fn read_profile(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<Profile>{let row=r.required("bim_profile",parent,slot+1,26)?;active(row,1,8,slot+1)?;let kind=row.text(9)?;let outline=r.list::<Vertex>("bim_profile_outline",row.rowid)?;if kind!="Custom"&&!outline.is_empty(){return Err(invalid("BIM noncustom profile owns an outline"))}let selected:&[usize]=match kind{"Rectangle"=>&[10,13],"Circle"=>&[16],"IShape"=>&[10,13,19,22],"Custom"|"Family"=>&[],_=>return Err(invalid("BIM profile kind is undeclared"))};for index in [10,13,16,19,22]{if !selected.contains(&index){absent(row,index,3)?;}}if kind!="Family"{absent(row,25,1)?;}let mut read=|index:usize|{let mut i=index;f64::read(row,&mut i,r.native)};Ok(match kind{"Rectangle"=>Profile::Rectangle{width:read(10)?,depth:read(13)?},"Circle"=>Profile::Circle{diameter:read(16)?},"IShape"=>Profile::IShape{width:read(10)?,depth:read(13)?,web:read(19)?,flange:read(22)?},"Custom"=>Profile::Custom{outline},_=>{let mut i=25;Profile::Family{family:String::read(row,&mut i,r.native)?}}})}
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
impl Scalar for CurtainPanel{
 const WIDTH:usize=4;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Glass=>{c.push(Cell::Text("Glass"))?;c.nulls(3)},Self::Empty=>{c.push(Cell::Text("Empty"))?;c.nulls(3)},Self::Solid{material}=>{c.push(Cell::Text("Solid"))?;material.append(c)?;c.nulls(2)},Self::Door{door_type}=>{c.push(Cell::Text("Door"))?;c.nulls(1)?;door_type.append(c)?;c.nulls(1)},Self::Window{window_type}=>{c.push(Cell::Text("Window"))?;c.nulls(2)?;window_type.append(c)}}}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*index;let value=match row.text(start)?{"Glass"=>{absent(row,start+1,3)?;Self::Glass},"Empty"=>{absent(row,start+1,3)?;Self::Empty},"Solid"=>{absent(row,start+2,2)?;*index=start+1;Self::Solid{material:String::read(row,index,n)?}},"Door"=>{absent(row,start+1,1)?;absent(row,start+3,1)?;*index=start+2;Self::Door{door_type:String::read(row,index,n)?}},"Window"=>{absent(row,start+1,2)?;*index=start+3;Self::Window{window_type:String::read(row,index,n)?}},_=>return Err(invalid("BIM curtain panel kind is undeclared"))};*index=start+Self::WIDTH;Ok(value)}
}
impl Scalar for Paper{
 const WIDTH:usize=8;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Iso{size}=>{c.push(Cell::Text("Iso"))?;size.append(c)?;c.nulls(6)},Self::Custom{width,height}=>{c.push(Cell::Text("Custom"))?;c.nulls(1)?;width.append(c)?;height.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;let value=match row.text(start)?{"Iso"=>{absent(row,start+2,6)?;*i=start+1;Self::Iso{size:IsoSize::read(row,i,n)?}},"Custom"=>{absent(row,start+1,1)?;*i=start+2;Self::Custom{width:f64::read(row,i,n)?,height:f64::read(row,i,n)?}},_=>return Err(invalid("BIM paper kind is undeclared"))};*i=start+8;Ok(value)}
}
impl Scalar for ScheduleKey{
 const WIDTH:usize=4;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Field{field}=>{c.push(Cell::Text("Field"))?;field.append(c)?;c.nulls(2)},Self::Property{set,name}=>{c.push(Cell::Text("Property"))?;c.nulls(1)?;set.append(c)?;name.append(c)}}}
 fn read(row:&SqliteRow,i:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*i;let value=match row.text(start)?{"Field"=>{absent(row,start+2,2)?;*i=start+1;Self::Field{field:ScheduleField::read(row,i,n)?}},"Property"=>{absent(row,start+1,1)?;*i=start+2;Self::Property{set:String::read(row,i,n)?,name:String::read(row,i,n)?}},_=>return Err(invalid("BIM schedule key kind is undeclared"))};*i=start+4;Ok(value)}
}
fn write_grid(p:&mut RowWriter<'_,'_>,parent:i64,value:&CurtainGrid,slot:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,4,slot)?;match value{CurtainGrid::Spacing{spacing}=>{c.push(Cell::Text("Spacing"))?;spacing.append(&mut c)?;},CurtainGrid::Lines{..}=>{c.push(Cell::Text("Lines"))?;c.nulls(3)?;}}let id=c.emit(p,"bim_curtain_grid")?;if let CurtainGrid::Lines{positions}=value{list(p,"bim_curtain_grid_position",id,positions)?;}Ok(())}
fn read_grid_row(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,slot:usize)->Result<CurtainGrid>{active(row,1,4,slot+1)?;let positions=r.list("bim_curtain_grid_position",row.rowid)?;match row.text(5)?{"Spacing"=>{if !positions.is_empty(){return Err(invalid("BIM spacing grid owns positions"))}let mut i=6;Ok(CurtainGrid::Spacing{spacing:f64::read(row,&mut i,r.native)?})},"Lines"=>{absent(row,6,3)?;Ok(CurtainGrid::Lines{positions})},_=>Err(invalid("BIM curtain grid kind is undeclared"))}}
fn read_grid(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<CurtainGrid>{let row=r.required("bim_curtain_grid",parent,slot+1,9)?;read_grid_row(r,row,slot)}
fn write_optional_grid(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<CurtainGrid>,slot:usize)->Result<()>{if let Some(value)=value{write_grid(p,parent,value,slot)?;}Ok(())}
fn read_optional_grid(r:&mut Reader<'_,'_,'_>,parent:i64,slot:usize)->Result<Option<CurtainGrid>>{let row=r.optional("bim_curtain_grid",parent,slot+1,9)?;row.map(|row|read_grid_row(r,row,slot)).transpose()}
fn write_anchor(p:&mut RowWriter<'_,'_>,parent:i64,value:&AnnotationAnchor,slot:usize,index:usize)->Result<()>{let mut c=Cells::new();owners(&mut c,parent,2,slot)?;c.push(Cell::Integer(ordinal(index)?))?;let(kind,point,wall,side,end,opening,grid,column)=match value{
 AnnotationAnchor::Point{point}=>("Point",Some(point),None,None,None,None,None,None),
 AnnotationAnchor::WallFace{wall,side}=>("WallFace",None,Some(wall),Some(side),None,None,None,None),
 AnnotationAnchor::WallAxis{wall}=>("WallAxis",None,Some(wall),None,None,None,None,None),
 AnnotationAnchor::WallEnd{wall,end}=>("WallEnd",None,Some(wall),None,Some(end),None,None,None),
 AnnotationAnchor::OpeningCentre{opening}=>("OpeningCentre",None,None,None,None,Some(opening),None,None),
 AnnotationAnchor::Grid{grid}=>("Grid",None,None,None,None,None,Some(grid),None),
 AnnotationAnchor::ColumnCentre{column}=>("ColumnCentre",None,None,None,None,None,None,Some(column))};
 c.push(Cell::Text(kind))?;if let Some(value)=point{value.append(&mut c)?;}else{c.nulls(6)?;}for value in [wall]{if let Some(value)=value{value.append(&mut c)?;}else{c.nulls(1)?;}}if let Some(value)=side{value.append(&mut c)?;}else{c.nulls(1)?;}if let Some(value)=end{value.append(&mut c)?;}else{c.nulls(1)?;}for value in [opening,grid,column]{if let Some(value)=value{value.append(&mut c)?;}else{c.nulls(1)?;}}c.emit(p,"bim_annotation_anchor")?;Ok(())}
fn read_anchor(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,slot:usize)->Result<AnnotationAnchor>{active(row,1,2,slot+1)?;let kind=row.text(4)?;let selected:&[usize]=match kind{"Point"=>&[5,8],"WallFace"=>&[11,12],"WallAxis"=>&[11],"WallEnd"=>&[11,13],"OpeningCentre"=>&[14],"Grid"=>&[15],"ColumnCentre"=>&[16],_=>return Err(invalid("BIM annotation anchor kind is undeclared"))};for index in [5,8,11,12,13,14,15,16]{if !selected.contains(&index){absent(row,index,if index<11{3}else{1})?;}}let mut i=5;Ok(match kind{"Point"=>AnnotationAnchor::Point{point:Point2::read(row,&mut i,r.native)?},"WallFace"=>{i=11;AnnotationAnchor::WallFace{wall:String::read(row,&mut i,r.native)?,side:WallSide::read(row,&mut i,r.native)?}},"WallAxis"=>{i=11;AnnotationAnchor::WallAxis{wall:String::read(row,&mut i,r.native)?}},"WallEnd"=>{i=11;let wall=String::read(row,&mut i,r.native)?;i=13;AnnotationAnchor::WallEnd{wall,end:AnchorEnd::read(row,&mut i,r.native)?}},"OpeningCentre"=>{i=14;AnnotationAnchor::OpeningCentre{opening:String::read(row,&mut i,r.native)?}},"Grid"=>{i=15;AnnotationAnchor::Grid{grid:String::read(row,&mut i,r.native)?}},_=>{i=16;AnnotationAnchor::ColumnCentre{column:String::read(row,&mut i,r.native)?}}})}
fn write_anchors(p:&mut RowWriter<'_,'_>,parent:i64,values:&Vec<AnnotationAnchor>,_:())->Result<()>{for(index,value)in values.iter().enumerate(){write_anchor(p,parent,value,0,index)?;}Ok(())}
fn read_anchors(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Vec<AnnotationAnchor>>{let rows=r.take("bim_annotation_anchor",parent,1,17,Some(3))?;let mut values=r.native.allocate_vec(rows.len())?;for row in rows{values.push(read_anchor(r,row,0)?);}Ok(values)}
fn write_leader_anchor(p:&mut RowWriter<'_,'_>,parent:i64,value:&AnnotationAnchor,_:())->Result<()>{write_anchor(p,parent,value,1,0)}
fn read_leader_anchor(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<AnnotationAnchor>{let rows=r.take("bim_annotation_anchor",parent,2,17,Some(3))?;if rows.len()!=1{return Err(invalid("BIM leader must own exactly one anchor"))}read_anchor(r,rows[0],1)}
fn write_default(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<PropertyValue>,_:())->Result<()>{if let Some(value)=value{let mut c=Cells::new();c.push(Cell::Integer(parent))?;value.append(&mut c)?;c.emit(p,"bim_property_default")?;}Ok(())}
fn read_default(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Option<PropertyValue>>{let Some(row)=r.optional("bim_property_default",parent,1,9)?else{return Ok(None)};let mut i=2;PropertyValue::read(row,&mut i,r.native).map(Some)}
fn write_parametric(p:&mut RowWriter<'_,'_>,parent:i64,value:&ParametricProfile)->Result<()>{let mut c=Cells::new();c.push(Cell::Integer(parent))?;let(kind,values)=match value{ParametricProfile::Rectangle{width,depth}=>("Rectangle",[Some(width),Some(depth),None,None,None]),ParametricProfile::Circle{diameter}=>("Circle",[None,None,Some(diameter),None,None]),ParametricProfile::IShape{width,depth,web,flange}=>("IShape",[Some(width),Some(depth),None,Some(web),Some(flange)]),ParametricProfile::Polygon{..}=>("Polygon",[None;5])};c.push(Cell::Text(kind))?;for value in values{c.push(match value{Some(value)=>Cell::Text(value),None=>Cell::Null})?;}let id=c.emit(p,"bim_parametric_profile")?;if let ParametricProfile::Polygon{points}=value{list(p,"bim_parametric_polygon_point",id,points)?;}Ok(())}
fn read_parametric(r:&mut Reader<'_,'_,'_>,parent:i64)->Result<ParametricProfile>{let row=r.required("bim_parametric_profile",parent,1,8)?;let kind=row.text(2)?;let points=r.list("bim_parametric_polygon_point",row.rowid)?;if kind!="Polygon"&&!points.is_empty(){return Err(invalid("BIM nonpolygon parametric profile owns points"))}let selected:&[usize]=match kind{"Rectangle"=>&[3,4],"Circle"=>&[5],"IShape"=>&[3,4,6,7],"Polygon"=>&[],_=>return Err(invalid("BIM parametric profile kind is undeclared"))};for index in 3..8{if !selected.contains(&index){absent(row,index,1)?;}}let mut read=|mut i|String::read(row,&mut i,r.native);Ok(match kind{"Rectangle"=>ParametricProfile::Rectangle{width:read(3)?,depth:read(4)?},"Circle"=>ParametricProfile::Circle{diameter:read(5)?},"IShape"=>ParametricProfile::IShape{width:read(3)?,depth:read(4)?,web:read(6)?,flange:read(7)?},_=>ParametricProfile::Polygon{points}})}
trait Entity:Sized{
 const WIDTH:usize;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>;
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>;
 fn read_at(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,start:usize)->Result<Self>;
 fn read(r:&mut Reader<'_,'_,'_>,row:&SqliteRow)->Result<Self>{Self::read_at(r,row,4)}
}
fn write_list<T:Scalar>(p:&mut RowWriter<'_,'_>,parent:i64,value:&Vec<T>,table:&str)->Result<()>{list(p,table,parent,value)}
fn read_list<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,table:&'static str)->Result<Vec<T>>{r.list(table,parent)}
macro_rules! entity{($t:ty;$($field:ident:$kind:ty),+$(; $($child:ident:$nested:ty=>$write:ident($wa:expr),$read:ident($ra:expr)),+)?)=>{impl Entity for $t{
 const WIDTH:usize=0$(+<$kind as Scalar>::WIDTH)+;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{$(self.$field.append(c)?;)+Ok(())}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{$( $($write(p,parent,&self.$child,$wa)?;)+ )?let _=(p,parent);Ok(())}
 fn read_at(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,start:usize)->Result<Self>{r.native.charge(std::mem::size_of::<Self>())?;let mut i=start;$(let $field=<$kind as Scalar>::read(row,&mut i,r.native)?;)+if i!=row.values.len(){return Err(invalid("BIM entity has extra scalar columns"))}Ok(Self{$($field,)+$( $($child:$read(r,row.rowid,$ra)?,)+ )?})}
}}}
entity!(Material;name:String,category:MaterialCategory,color:Rgb,density:f64,conductivity:f64,specific_heat:f64);
entity!(WallType;name:String;layers:Vec<Layer>=>write_list("bim_wall_layer"),read_list("bim_wall_layer"));
entity!(SlabType;name:String;layers:Vec<Layer>=>write_list("bim_slab_layer"),read_list("bim_slab_layer"));
entity!(RoofType;name:String;layers:Vec<Layer>=>write_list("bim_roof_layer"),read_list("bim_roof_layer"));
entity!(ColumnType;name:String,material:String;profile:Profile=>write_profile(0),read_profile(0));
entity!(BeamType;name:String,material:String;profile:Profile=>write_profile(1),read_profile(1));
entity!(WindowType;name:String,width:f64,height:f64,sill:f64,frame_width:f64,frame_depth:f64,panes:u32,material:String);
entity!(DoorType;name:String,width:f64,height:f64,frame_width:f64,frame_depth:f64,leaves:DoorLeaves,swing:Swing,material:String);
entity!(CurtainPanelOverride;curtain:String,u:u32,v:u32,panel:CurtainPanel);
entity!(Family;name:String,category:FamilyCategory);
entity!(FamilyParameter;family:String,name:String,kind:ParameterKind,value:String);
entity!(Zone;name:String,category:String,occupancy_density:f64);
entity!(Tag;storey:String,element:String,category:TagCategory,offset:Point2,style:String);
entity!(TextNote;storey:String,position:Point2,text:String,rotation:f64,style:String);
entity!(AnnotationStyle;name:String,text_height:f64,terminator:Terminator,unit:DimensionUnit,precision:u32,mark_size:f64,gap:f64,overshoot:f64);
entity!(Viewport;sheet:String,view:String,position:Point2,scale:u32,crop:Option<ViewCrop>,label:Option<String>);
entity!(SheetRevision;sheet:String,number:String,date:String,description:String,author:String);
entity!(WallSweep;host:String,side:WallSide,height:f64,inset:f64,material:String,name:String;profile:Profile=>write_profile(7),read_profile(7));
entity!(Site;name:String,latitude:f64,longitude:f64,elevation:f64,true_north:f64;boundary:Vec<Point2>=>write_list("bim_site_boundary"),read_list("bim_site_boundary"));
entity!(Building;site:String,name:String,origin:Point2,rotation:f64,elevation:f64);
entity!(Storey;building:String,name:String,level:i32,height:f64,cut_height:Option<f64>);
entity!(GridLine;building:String,label:String,start:Point2,end:Point2);
entity!(Wall;storey:String,wall_type:String,location:LocationLine,base_offset:f64,phase:Phase,start_join:Option<EndJoin>,end_join:Option<EndJoin>,name:String,base_slab:Option<String>;axis:Axis=>write_axis(0),read_axis(0),top:TopConstraint=>write_top(0),read_top(0));
entity!(Column;storey:String,column_type:String,position:Point2,rotation:f64,tilt:Option<Slope>,base_offset:f64,phase:Phase,name:String;top:TopConstraint=>write_top(2),read_top(2));
entity!(Beam;storey:String,beam_type:String,top_offset:f64,end_top_offset:Option<f64>,phase:Phase,name:String;axis:Axis=>write_axis(2),read_axis(2));
fn write_holes(p:&mut RowWriter<'_,'_>,parent:i64,value:&Vec<Vec<Vertex>>,tables:(&str,&str))->Result<()>{for(index,hole)in value.iter().enumerate(){let id=p.insert(tables.0,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?)])?;list(p,tables.1,id,hole)?;}Ok(())}
fn read_holes(r:&mut Reader<'_,'_,'_>,parent:i64,tables:(&'static str,&'static str))->Result<Vec<Vec<Vertex>>>{let rows=r.take(tables.0,parent,1,3,Some(2))?;let mut values=r.native.allocate_vec(rows.len())?;for row in rows{r.native.step()?;values.push(r.list(tables.1,row.rowid)?)}Ok(values)}
entity!(Slab;storey:String,slab_type:String,offset:f64,slope:Option<Slope>,phase:Phase,name:String;boundary:Vec<Vertex>=>write_list("bim_slab_boundary"),read_list("bim_slab_boundary"),holes:Vec<Vec<Vertex>>=>write_holes(("bim_slab_hole","bim_slab_hole_vertex")),read_holes(("bim_slab_hole","bim_slab_hole_vertex")));
entity!(Ceiling;storey:String,ceiling_type:String,offset:f64,slope:Option<Slope>,name:String;boundary:Vec<Vertex>=>write_list("bim_ceiling_boundary"),read_list("bim_ceiling_boundary"),holes:Vec<Vec<Vertex>>=>write_holes(("bim_ceiling_hole","bim_ceiling_hole_vertex")),read_holes(("bim_ceiling_hole","bim_ceiling_hole_vertex")));
entity!(Roof;storey:String,roof_type:String,shape:RoofShape,overhang:f64,base_offset:f64,phase:Phase,name:String;footprint:Vec<Vertex>=>write_list("bim_roof_footprint"),read_list("bim_roof_footprint"));
entity!(Opening;host:String,kind:OpeningKind,offset:f64,sill_override:Option<f64>,width:Option<f64>,height:Option<f64>,flip_hand:bool,flip_facing:bool,name:String,reveal_depth:Option<f64>,reveal_material:Option<String>);
entity!(Stair;storey:String,start:Point2,direction:f64,width:f64,flight:StairFlight,max_riser:f64,min_tread:f64,stringer:StairStringer,nosing:f64,tread_thickness:f64,riser:RiserKind,landing_depth:f64,phase:Phase,name:String;top:TopConstraint=>write_top(3),read_top(3));
fn write_baluster(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<Baluster>,_:())->Result<()>{if let Some(value)=value{let mut c=Cells::new();c.push(Cell::Integer(parent))?;value.spacing.append(&mut c)?;let id=c.emit(p,"bim_baluster")?;write_profile(p,id,&value.profile,6)?;}Ok(())}
fn read_baluster(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Option<Baluster>>{let Some(row)=r.optional("bim_baluster",parent,1,5)?else{return Ok(None)};let mut i=2;let spacing=f64::read(row,&mut i,r.native)?;Ok(Some(Baluster{spacing,profile:read_profile(r,row.rowid,6)?}))}
entity!(Railing;storey:String,height:f64,post_spacing:f64,infill:Infill,material:String,base_offset:f64,host:Option<RailingHost>,phase:Phase,name:String;path:Vec<Point2>=>write_list("bim_railing_path"),read_list("bim_railing_path"),profile:Profile=>write_profile(4),read_profile(4),post_profile:Profile=>write_profile(5),read_profile(5),baluster:Option<Baluster>=>write_baluster(()),read_baluster(()));
impl Entity for Space{
 const WIDTH:usize=16;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{self.storey.append(c)?;self.number.append(c)?;self.name.append(c)?;match &self.boundary{SpaceBoundary::Bounded{seed}=>{c.push(Cell::Text("Bounded"))?;seed.append(c)?;},SpaceBoundary::Explicit{..}=>{c.push(Cell::Text("Explicit"))?;c.nulls(6)?;}}self.usage.append(c)?;self.phase.append(c)?;self.zone.append(c)?;self.floor_finish.append(c)?;self.wall_finish.append(c)?;self.ceiling_finish.append(c)}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{if let SpaceBoundary::Explicit{outline}=&self.boundary{list(p,"bim_space_outline",parent,outline)?;}Ok(())}
 fn read_at(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,start:usize)->Result<Self>{r.native.charge(std::mem::size_of::<Self>())?;let mut i=start;let storey=String::read(row,&mut i,r.native)?;let number=String::read(row,&mut i,r.native)?;let name=String::read(row,&mut i,r.native)?;let kind=row.text(i)?;i+=1;let outline=r.list("bim_space_outline",row.rowid)?;let boundary=match kind{"Bounded"=>{if !outline.is_empty(){return Err(invalid("BIM bounded space owns an outline"))}SpaceBoundary::Bounded{seed:Point2::read(row,&mut i,r.native)?}},"Explicit"=>{absent(row,i,6)?;i+=6;SpaceBoundary::Explicit{outline}},_=>return Err(invalid("BIM space boundary kind is undeclared"))};let usage=String::read(row,&mut i,r.native)?;let phase=Phase::read(row,&mut i,r.native)?;let zone=Option::<String>::read(row,&mut i,r.native)?;let floor_finish=Option::<String>::read(row,&mut i,r.native)?;let wall_finish=Option::<String>::read(row,&mut i,r.native)?;let ceiling_finish=Option::<String>::read(row,&mut i,r.native)?;if i!=row.values.len(){return Err(invalid("BIM space has extra scalar columns"))}Ok(Self{storey,number,name,boundary,usage,phase,zone,floor_finish,wall_finish,ceiling_finish})}
}
entity!(CeilingType;name:String;layers:Vec<Layer>=>write_list("bim_ceiling_layer"),read_list("bim_ceiling_layer"));
entity!(CurtainWallType;name:String,panel:CurtainPanel,panel_material:String,mullion_material:String;u_grid:CurtainGrid=>write_grid(0),read_grid(0),v_grid:CurtainGrid=>write_grid(1),read_grid(1),interior_mullion:Profile=>write_profile(2),read_profile(2),border_mullion:Profile=>write_profile(3),read_profile(3));
entity!(CurtainWall;storey:String,curtain_wall_type:String,base_offset:f64,phase:Phase,name:String;axis:Axis=>write_axis(1),read_axis(1),top:TopConstraint=>write_top(1),read_top(1),u_grid:Option<CurtainGrid>=>write_optional_grid(2),read_optional_grid(2),v_grid:Option<CurtainGrid>=>write_optional_grid(3),read_optional_grid(3));
entity!(Ramp;storey:String,width:f64,landing_start:f64,landing_end:f64,landing_turn:f64,max_slope:f64,thickness:f64,material:String,base_offset:f64,railing_left:bool,railing_right:bool,name:String;path:Vec<Vertex>=>write_list("bim_ramp_path"),read_list("bim_ramp_path"),top:TopConstraint=>write_top(4),read_top(4));
entity!(AreaScheme;name:String,measure:AreaMeasure;usages:Vec<String>=>write_list("bim_area_usage"),read_list("bim_area_usage"),zones:Vec<String>=>write_list("bim_area_zone"),read_list("bim_area_zone"));
entity!(View;building:String,name:String,kind:ViewKind,storey:Option<String>,plane:Option<ViewPlane>,camera:Option<ViewCamera>,cut_height:Option<f64>,depth:f64,crop:Option<ViewCrop>,phase:Option<Phase>,scale:u32,detail:DetailLevel;hidden:Vec<ViewCategory>=>write_list("bim_view_hidden"),read_list("bim_view_hidden"));
entity!(Sheet;number:String,name:String,paper:Paper,orientation:Orientation,project:String,drawn_by:String,checked_by:String,date:String,revision:String,scale_label:String);
entity!(Dimension;storey:String,angle:f64,offset:f64,style:String,lock:Option<f64>,name:String;anchors:Vec<AnnotationAnchor>=>write_anchors(()),read_anchors(()));
entity!(Leader;storey:String,offset:Point2,text:String,style:String;anchor:AnnotationAnchor=>write_leader_anchor(()),read_leader_anchor(()));
entity!(ScheduleColumn;key:ScheduleKey,heading:Option<String>,total:bool);
entity!(ScheduleSort;key:ScheduleKey,descending:bool);
entity!(ScheduleFilter;key:ScheduleKey,op:ScheduleOp,value:String);
entity!(ScheduleGroup;key:ScheduleKey);
entity!(Schedule;name:String,category:ScheduleCategory,itemize:bool;columns:Vec<ScheduleColumn>=>write_entities("bim_schedule_column"),read_entities("bim_schedule_column"),sort:Vec<ScheduleSort>=>write_entities("bim_schedule_sort"),read_entities("bim_schedule_sort"),filter:Vec<ScheduleFilter>=>write_entities("bim_schedule_filter"),read_entities("bim_schedule_filter"),group:Vec<ScheduleGroup>=>write_entities("bim_schedule_group"),read_entities("bim_schedule_group"),storeys:Vec<String>=>write_list("bim_schedule_storey"),read_list("bim_schedule_storey"),phases:Vec<Phase>=>write_list("bim_schedule_phase"),read_list("bim_schedule_phase"));
entity!(PropertyTemplate;name:String;applies_to:Vec<TemplateTarget>=>write_list("bim_template_target"),read_list("bim_template_target"),properties:Vec<PropertyDef>=>write_entities("bim_property_def"),read_entities("bim_property_def"));
entity!(PropertyDef;name:String,kind:PropertyKind,unit:Option<String>,description:Option<String>,required:bool,minimum:Option<f64>,maximum:Option<f64>;default_value:Option<PropertyValue>=>write_default(()),read_default(()),allowed:Vec<PropertyValue>=>write_list("bim_property_allowed"),read_list("bim_property_allowed"));
entity!(ClassificationSystem;name:String,edition:String,source:Option<String>;entries:Vec<ClassificationItem>=>write_list("bim_classification_item"),read_list("bim_classification_item"));
impl Entity for FamilySolid{
 const WIDTH:usize=17;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{self.family.append(c)?;self.name.append(c)?;let(kind,values,axis):(&str,[Option<&String>;8],Option<&SolidAxis>)=match &self.shape{
 SolidShape::Extrusion{base,height,..}=>("Extrusion",[Some(base),Some(height),None,None,None,None,None,None],None),
 SolidShape::Revolution{axis,angle,..}=>("Revolution",[None,None,Some(angle),None,None,None,None,None],Some(axis)),
 SolidShape::Sweep{..}=>("Sweep",[None;8],None),
 SolidShape::Cuboid{x,y,z,width,depth,height}=>("Cuboid",[None,Some(height),None,Some(x),Some(y),Some(z),Some(width),Some(depth)],None)};
 c.push(Cell::Text(kind))?;for value in &values[..2]{c.push(value.map_or(Cell::Null,|value|Cell::Text(value)))?;}if let Some(axis)=axis{axis.append(c)?;}else{c.nulls(1)?;}for value in &values[2..]{c.push(value.map_or(Cell::Null,|value|Cell::Text(value)))?;}self.material.append(c)?;self.visible.append(c)?;self.offset.append(c)}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{match &self.shape{SolidShape::Extrusion{profile,..}|SolidShape::Revolution{profile,..}=>write_parametric(p,parent,profile),SolidShape::Sweep{profile,path}=>{write_parametric(p,parent,profile)?;list(p,"bim_solid_sweep_point",parent,path)},SolidShape::Cuboid{..}=>Ok(())}}
 fn read_at(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,start:usize)->Result<Self>{r.native.charge(std::mem::size_of::<Self>())?;let mut i=start;let family=String::read(row,&mut i,r.native)?;let name=String::read(row,&mut i,r.native)?;let kind=row.text(i)?;let base=i+1;let selected:&[usize]=match kind{"Extrusion"=>&[0,1],"Revolution"=>&[2,3],"Sweep"=>&[],"Cuboid"=>&[1,4,5,6,7,8],_=>return Err(invalid("BIM solid shape kind is undeclared"))};for index in 0..9{if !selected.contains(&index){absent(row,base+index,1)?;}}let path=r.list("bim_solid_sweep_point",row.rowid)?;if kind!="Sweep"&&!path.is_empty(){return Err(invalid("BIM nonsweep solid owns a path"))}let shape=match kind{
 "Extrusion"=>{i=base;let base=String::read(row,&mut i,r.native)?;let height=String::read(row,&mut i,r.native)?;SolidShape::Extrusion{profile:read_parametric(r,row.rowid)?,base,height}},
 "Revolution"=>{i=base+2;let axis=SolidAxis::read(row,&mut i,r.native)?;let angle=String::read(row,&mut i,r.native)?;SolidShape::Revolution{profile:read_parametric(r,row.rowid)?,axis,angle}},
 "Sweep"=>SolidShape::Sweep{profile:read_parametric(r,row.rowid)?,path},
 _=>{if r.optional("bim_parametric_profile",row.rowid,1,8)?.is_some(){return Err(invalid("BIM cuboid owns a parametric profile"))}i=base+1;let height=String::read(row,&mut i,r.native)?;i=base+4;SolidShape::Cuboid{x:String::read(row,&mut i,r.native)?,y:String::read(row,&mut i,r.native)?,z:String::read(row,&mut i,r.native)?,width:String::read(row,&mut i,r.native)?,depth:String::read(row,&mut i,r.native)?,height}}};
 i=base+9;let material=String::read(row,&mut i,r.native)?;let visible=String::read(row,&mut i,r.native)?;let offset=ExprPoint3::read(row,&mut i,r.native)?;if i!=row.values.len(){return Err(invalid("BIM solid has extra scalar columns"))}Ok(Self{family,name,shape,material,visible,offset})}
}
fn write_entities<T:Entity>(p:&mut RowWriter<'_,'_>,parent:i64,values:&Vec<T>,table:&str)->Result<()>{for(index,value)in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;value.fields(&mut c)?;let id=c.emit(p,table)?;value.children(p,id)?;}Ok(())}
fn read_entities<T:Entity>(r:&mut Reader<'_,'_,'_>,parent:i64,table:&'static str)->Result<Vec<T>>{let rows=r.take(table,parent,1,T::WIDTH+3,Some(2))?;let mut values=r.native.allocate_vec(rows.len())?;for row in rows{r.native.step()?;values.push(T::read_at(r,row,3)?)}Ok(values)}
fn write_map<T:Entity>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&BTreeMap<String,T>)->Result<()>{for(index,(key,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(key))?;value.fields(&mut c)?;let id=c.emit(p,table)?;value.children(p,id)?;}Ok(())}
fn read_map<T:Entity>(r:&mut Reader<'_,'_,'_>,table:&'static str,parent:i64)->Result<BTreeMap<String,T>>{let rows=r.take(table,parent,1,T::WIDTH+4,Some(2))?;let mut result=BTreeMap::new();let mut previous:Option<&str>=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM map keys are not strictly sorted"))}}previous=Some(key);r.native.charge(std::mem::size_of::<(String,T)>()+96)?;let key=r.native.copy_text(key)?;let value=T::read(r,row)?;if result.insert(key,value).is_some(){return Err(invalid("BIM map key is repeated"))}}Ok(result)}
fn write_property<'a>(value:&'a PropertyValue,c:&mut Cells<'a>)->Result<()>{match value{
 PropertyValue::Text{value}=>{c.push(Cell::Text("Text"))?;value.append(c)?;c.nulls(5)},
 PropertyValue::Integer{value}=>{c.push(Cell::Text("Integer"))?;c.nulls(1)?;value.append(c)?;c.nulls(4)},
 PropertyValue::Boolean{value}=>{c.push(Cell::Text("Boolean"))?;c.nulls(2)?;value.append(c)?;c.nulls(3)},
 PropertyValue::Real{value}=>write_measure(c,"Real",value),PropertyValue::Length{value}=>write_measure(c,"Length",value),PropertyValue::Area{value}=>write_measure(c,"Area",value),PropertyValue::Volume{value}=>write_measure(c,"Volume",value),PropertyValue::Angle{value}=>write_measure(c,"Angle",value)
}}
fn write_measure<'a>(c:&mut Cells<'a>,kind:&'static str,value:&'a f64)->Result<()>{c.push(Cell::Text(kind))?;c.nulls(3)?;value.append(c)}
impl Scalar for PropertyValue{
 const WIDTH:usize=7;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{write_property(self,c)}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let start=*index;let value=match row.text(start)?{
 "Text"=>{absent(row,start+2,5)?;*index=start+1;Self::Text{value:String::read(row,index,n)?}},
 "Integer"=>{absent(row,start+1,1)?;absent(row,start+3,4)?;*index=start+2;Self::Integer{value:i32::read(row,index,n)?}},
 "Boolean"=>{absent(row,start+1,2)?;absent(row,start+4,3)?;*index=start+3;Self::Boolean{value:bool::read(row,index,n)?}},
 kind@("Real"|"Length"|"Area"|"Volume"|"Angle")=>{absent(row,start+1,3)?;*index=start+4;let value=f64::read(row,index,n)?;match kind{"Real"=>Self::Real{value},"Length"=>Self::Length{value},"Area"=>Self::Area{value},"Volume"=>Self::Volume{value},_=>Self::Angle{value}}},_=>return Err(invalid("BIM typed property kind is undeclared"))};*index=start+Self::WIDTH;Ok(value)}
}
fn visit_rows(snapshot:&ModelSnapshot,p:&mut RowWriter<'_,'_>)->Result<()>{
 let root=p.insert("bim_document",&[Cell::Text(&snapshot.schema)])?;
 let project=p.insert("bim_project",&[Cell::Integer(root),Cell::Text(&snapshot.project.name),Cell::Text(&snapshot.project.description),Cell::Text(&snapshot.project.author),Cell::Text(&snapshot.project.organization)])?;
 list(p,"bim_phase_name",project,&snapshot.project.phase_names)?;
 write_map(p,"bim_material",root,&snapshot.materials)?;
 write_map(p,"bim_wall_type",root,&snapshot.wall_types)?;
 write_map(p,"bim_slab_type",root,&snapshot.slab_types)?;
 write_map(p,"bim_roof_type",root,&snapshot.roof_types)?;
 write_map(p,"bim_column_type",root,&snapshot.column_types)?;
 write_map(p,"bim_beam_type",root,&snapshot.beam_types)?;
 write_map(p,"bim_window_type",root,&snapshot.window_types)?;
 write_map(p,"bim_door_type",root,&snapshot.door_types)?;
 write_map(p,"bim_curtain_wall_type",root,&snapshot.curtain_wall_types)?;
 write_map(p,"bim_site",root,&snapshot.sites)?;
 write_map(p,"bim_building",root,&snapshot.buildings)?;
 write_map(p,"bim_storey",root,&snapshot.storeys)?;
 write_map(p,"bim_grid",root,&snapshot.grids)?;
 write_map(p,"bim_wall",root,&snapshot.walls)?;
 write_map(p,"bim_curtain_wall",root,&snapshot.curtain_walls)?;
 write_map(p,"bim_curtain_panel_override",root,&snapshot.curtain_panel_overrides)?;
 write_map(p,"bim_column",root,&snapshot.columns)?;
 write_map(p,"bim_beam",root,&snapshot.beams)?;
 write_map(p,"bim_slab",root,&snapshot.slabs)?;
 write_map(p,"bim_roof",root,&snapshot.roofs)?;
 write_map(p,"bim_opening",root,&snapshot.openings)?;
 write_map(p,"bim_stair",root,&snapshot.stairs)?;
 write_map(p,"bim_railing",root,&snapshot.railings)?;
 write_map(p,"bim_ramp",root,&snapshot.ramps)?;
 write_map(p,"bim_ceiling_type",root,&snapshot.ceiling_types)?;
 write_map(p,"bim_ceiling",root,&snapshot.ceilings)?;
 write_map(p,"bim_space",root,&snapshot.spaces)?;
 write_map(p,"bim_zone",root,&snapshot.zones)?;
 write_map(p,"bim_area_scheme",root,&snapshot.area_schemes)?;
 write_map(p,"bim_view",root,&snapshot.views)?;
 write_map(p,"bim_sheet",root,&snapshot.sheets)?;
 write_map(p,"bim_viewport",root,&snapshot.viewports)?;
 write_map(p,"bim_sheet_revision",root,&snapshot.sheet_revisions)?;
 write_map(p,"bim_dimension",root,&snapshot.dimensions)?;
 write_map(p,"bim_tag",root,&snapshot.tags)?;
 write_map(p,"bim_text_note",root,&snapshot.text_notes)?;
 write_map(p,"bim_leader",root,&snapshot.leaders)?;
 write_map(p,"bim_annotation_style",root,&snapshot.annotation_styles)?;
 write_map(p,"bim_wall_sweep",root,&snapshot.wall_sweeps)?;
 write_map(p,"bim_schedule",root,&snapshot.schedules)?;
 write_map(p,"bim_family",root,&snapshot.families)?;
 write_map(p,"bim_family_parameter",root,&snapshot.family_parameters)?;
 write_map(p,"bim_family_solid",root,&snapshot.family_solids)?;
 write_map(p,"bim_property_template",root,&snapshot.property_templates)?;
 write_map(p,"bim_classification_system",root,&snapshot.classification_systems)?;
 for(index,(key,sets))in snapshot.properties.iter().enumerate(){let element=p.insert("bim_property_element",&[Cell::Integer(root),Cell::Integer(ordinal(index)?),Cell::Text(key)])?;for(index,(name,values))in sets.iter().enumerate(){let set=p.insert("bim_property_set",&[Cell::Integer(element),Cell::Integer(ordinal(index)?),Cell::Text(name)])?;for(index,(name,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(set))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(name))?;write_property(value,&mut c)?;c.emit(p,"bim_property_value")?;}}}
 write_maps(p,root,&snapshot.classifications,["bim_classification_element","bim_classification_assignment"])?;
 Ok(())
}
fn project(snapshot:&ModelSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::new(SCHEMA,c)?;visit_rows(snapshot,&mut p)?;p.finish()}
fn admit(snapshot:&ModelSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<()>{semantic_cells::extent(c.limits())?;let mut p=RowWriter::borrowed(c,SqliteSnapshotPhase::ProjectSnapshot)?;visit_rows(snapshot,&mut p)?;p.finish_borrowed()}
fn read_properties(r:&mut Reader<'_,'_,'_>,parent:i64)->Result<BTreeMap<String,PropertySet>>{
 let elements=r.take("bim_property_element",parent,1,4,Some(2))?;let mut result=BTreeMap::new();let mut previous=None;for element in elements{let key=element.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property element keys are not sorted"))}}previous=Some(key);let key=r.native.copy_text(key)?;r.native.charge(96+std::mem::size_of::<PropertySet>())?;let sets=r.take("bim_property_set",element.rowid,1,4,Some(2))?;let mut values=BTreeMap::new();let mut previous=None;for set in sets{let name=set.text(3)?;if let Some(previous)=previous{if compare_text(previous,name,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property set names are not sorted"))}}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<BTreeMap<String,PropertyValue>>())?;let properties=r.take("bim_property_value",set.rowid,1,11,Some(2))?;let mut contents=BTreeMap::new();let mut previous=None;for row in properties{let name=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,name,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property names are not sorted"))}}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<PropertyValue>())?;let mut index=4;contents.insert(name,PropertyValue::read(row,&mut index,r.native)?);}values.insert(name,contents);}result.insert(key,values);}Ok(result)
}
fn write_maps<T:Scalar>(p:&mut RowWriter<'_,'_>,parent:i64,values:&BTreeMap<String,BTreeMap<String,T>>,tables:[&str;2])->Result<()>{for(index,(key,values))in values.iter().enumerate(){let owner=p.insert(tables[0],&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Text(key)])?;for(index,(key,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(owner))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(key))?;value.append(&mut c)?;c.emit(p,tables[1])?;}}Ok(())}
fn read_scalar_map<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,table:&'static str)->Result<BTreeMap<String,T>>{let rows=r.take(table,parent,1,T::WIDTH+4,Some(2))?;let mut values=BTreeMap::new();let mut previous=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM scalar map keys are not sorted"))}}previous=Some(key);r.native.charge(96+std::mem::size_of::<(String,T)>())?;let key=r.native.copy_text(key)?;let mut index=4;values.insert(key,T::read(row,&mut index,r.native)?);}Ok(values)}
fn read_maps<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,tables:[&'static str;2])->Result<BTreeMap<String,BTreeMap<String,T>>>{let rows=r.take(tables[0],parent,1,4,Some(2))?;let mut values=BTreeMap::new();let mut previous=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM owned map keys are not sorted"))}}previous=Some(key);r.native.charge(96+std::mem::size_of::<(String,BTreeMap<String,T>)>())?;let key=r.native.copy_text(key)?;values.insert(key,read_scalar_map(r,row.rowid,tables[1])?);}Ok(values)}
fn restore(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<ModelSnapshot>{
 semantic_cells::extent(c.limits())?;c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;store::sqlite_snapshot::validate_sqlite_database_schema(database,SCHEMA,c.limits())?;
 let maximum=c.limits().max_value_bytes;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(maximum,&mut progress);native.charge(std::mem::size_of::<ModelSnapshot>())?;let mut r=Reader{database,native:&mut native,groups:BTreeMap::new(),used:BTreeSet::new()};let document=r.singleton("bim_document",2)?;let root=document.rowid;let schema=r.native.copy_text(document.text(1)?)?;let row=r.required("bim_project",root,1,6)?;let mut i=2;let project=Project{name:String::read(row,&mut i,r.native)?,description:String::read(row,&mut i,r.native)?,author:String::read(row,&mut i,r.native)?,organization:String::read(row,&mut i,r.native)?,phase_names:r.list("bim_phase_name",row.rowid)?};
 let result=ModelSnapshot{schema,project,
 materials:read_map(&mut r,"bim_material",root)?,
 wall_types:read_map(&mut r,"bim_wall_type",root)?,
 slab_types:read_map(&mut r,"bim_slab_type",root)?,
 roof_types:read_map(&mut r,"bim_roof_type",root)?,
 column_types:read_map(&mut r,"bim_column_type",root)?,
 beam_types:read_map(&mut r,"bim_beam_type",root)?,
 window_types:read_map(&mut r,"bim_window_type",root)?,
 door_types:read_map(&mut r,"bim_door_type",root)?,
 curtain_wall_types:read_map(&mut r,"bim_curtain_wall_type",root)?,
 sites:read_map(&mut r,"bim_site",root)?,
 buildings:read_map(&mut r,"bim_building",root)?,
 storeys:read_map(&mut r,"bim_storey",root)?,
 grids:read_map(&mut r,"bim_grid",root)?,
 walls:read_map(&mut r,"bim_wall",root)?,
 curtain_walls:read_map(&mut r,"bim_curtain_wall",root)?,
 curtain_panel_overrides:read_map(&mut r,"bim_curtain_panel_override",root)?,
 columns:read_map(&mut r,"bim_column",root)?,
 beams:read_map(&mut r,"bim_beam",root)?,
 slabs:read_map(&mut r,"bim_slab",root)?,
 roofs:read_map(&mut r,"bim_roof",root)?,
 openings:read_map(&mut r,"bim_opening",root)?,
 stairs:read_map(&mut r,"bim_stair",root)?,
 railings:read_map(&mut r,"bim_railing",root)?,
 ramps:read_map(&mut r,"bim_ramp",root)?,
 ceiling_types:read_map(&mut r,"bim_ceiling_type",root)?,
 ceilings:read_map(&mut r,"bim_ceiling",root)?,
 spaces:read_map(&mut r,"bim_space",root)?,
 zones:read_map(&mut r,"bim_zone",root)?,
 area_schemes:read_map(&mut r,"bim_area_scheme",root)?,
 views:read_map(&mut r,"bim_view",root)?,
 sheets:read_map(&mut r,"bim_sheet",root)?,
 viewports:read_map(&mut r,"bim_viewport",root)?,
 sheet_revisions:read_map(&mut r,"bim_sheet_revision",root)?,
 dimensions:read_map(&mut r,"bim_dimension",root)?,
 tags:read_map(&mut r,"bim_tag",root)?,
 text_notes:read_map(&mut r,"bim_text_note",root)?,
 leaders:read_map(&mut r,"bim_leader",root)?,
 annotation_styles:read_map(&mut r,"bim_annotation_style",root)?,
 wall_sweeps:read_map(&mut r,"bim_wall_sweep",root)?,
 schedules:read_map(&mut r,"bim_schedule",root)?,
 families:read_map(&mut r,"bim_family",root)?,
 family_parameters:read_map(&mut r,"bim_family_parameter",root)?,
 family_solids:read_map(&mut r,"bim_family_solid",root)?,
 property_templates:read_map(&mut r,"bim_property_template",root)?,
 classification_systems:read_map(&mut r,"bim_classification_system",root)?,
 properties:read_properties(&mut r,root)?,classifications:read_maps(&mut r,root,["bim_classification_element","bim_classification_assignment"])?};r.finish()?;Ok(result)
}
/// 🪶️ One authored relational visitor admits every native and public snapshot route.
impl store::ArtifactSqliteSnapshot for ModelSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<()>{admit(self,c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{project(self,c)}
 fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self>{restore(database,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload>{admit(self,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c,native_owner)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self>{let limits=c.limits();semantic_cells::extent(limits)?;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {semantic_cells::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },c,native_control)}
}
