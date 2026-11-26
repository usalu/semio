//! 🔢️ Authoring SQLite scalar words, row cells and owned relational traversal.
use super::{add, class, invalid};
use semio_framework_io_sqlite_model::{SqliteDatabase, SqliteRow, SqliteValue};
use semio_framework_io_sqlite_snapshot::artifact::{Cell, RowWriter};
use semio_framework_value::ValueError;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use std::collections::{BTreeMap, BTreeSet};
type Result<T> = std::result::Result<T, ValueError>;

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{AnchorEnd,AreaMeasure,ClashRef,ClassificationItem,CurtainPanel,DetailLevel,DimensionUnit,DoorLeaves,ElementClass,EndJoin,ExprPoint,ExprPoint3,FamilyCategory,HostSide,Infill,IsoSize,IssuePriority,IssueStatus,Layer,LayerFunction,LocationLine,MaterialCategory,MepShape,MepSystem,OpeningKind,Orientation,Paper,ParameterKind,Phase,Point2,Point3,Profile,PropertyKind,PropertyValue,RailingHost,Rgb,RiserKind,RoofShape,RuleKind,RuleSeverity,ScheduleCategory,ScheduleField,ScheduleKey,ScheduleOp,SectionBox,Slope,SolidAxis,StairFlight,StairStringer,StringerKind,Swing,TagCategory,TemplateTarget,Terminator,Turn,Vertex,ViewCamera,ViewCategory,ViewCrop,ViewKind,ViewPlane,WallSide};
pub struct Cells<'a>{values:[Cell<'a>;64],len:usize}
impl<'a>Cells<'a>{
 pub fn new()->Self{Self{values:[Cell::Null;64],len:0}}
 pub fn push(&mut self,value:Cell<'a>)->Result<()>{if self.len==self.values.len(){return Err(invalid("BIM row exceeds authored stack width"))}self.values[self.len]=value;self.len+=1;Ok(())}
 pub fn nulls(&mut self,count:usize)->Result<()>{for _ in 0..count{self.push(Cell::Null)?;}Ok(())}
 pub fn emit(&self,p:&mut RowWriter<'_,'_>,table:&str)->Result<i64>{p.insert(table,&self.values[..self.len])}
}
pub fn next<'a>(row:&'a SqliteRow,index:&mut usize)->Result<&'a SqliteValue>{let value=row.values.get(*index).ok_or_else(||invalid("BIM scalar column is absent"))?;*index+=1;Ok(value)}
pub trait Scalar:Sized{
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
enumeration!(ScheduleField;Id,Name,Kind,Storey,Level,Type,Phase,Material,Host,Number,Usage,Surface,Swing,Leaves,Panes,Count,Length,Width,Height,Perimeter,GrossSideArea,OpeningArea,NetSideArea,GrossArea,NetArea,SurfaceArea,GrossVolume,NetVolume,Mass,Risers,Thickness,LayerArea,LayerVolume,LayerMass,FinishArea,UValue,GValue);
enumeration!(ScheduleOp;Equals,NotEquals,Contains,Greater,GreaterOrEqual,Less,LessOrEqual,Empty,NotEmpty);
enumeration!(EndJoin;Miter,Butt,None);enumeration!(ViewKind;Plan,CeilingPlan,Section,Elevation,Orthographic,Perspective);
enumeration!(DetailLevel;Coarse,Medium,Fine);enumeration!(ViewCategory;Walls,CurtainWalls,Columns,Beams,Slabs,Roofs,Openings,Stairs,Railings,Spaces,Grids);
enumeration!(HostSide;Left,Right);enumeration!(AreaMeasure;Gross,Net);
enumeration!(ElementClass;Wall,CurtainWall,Window,Door,Column,Beam,Slab,Ceiling,Roof,Stair,Ramp,Railing,WallSweep,Component,Mep);
enumeration!(MepSystem;Supply,Return,Exhaust,DomesticWater,Waste,Gas,Power,Data,Lighting);
enumeration!(RuleKind;MinClearHeight,MaxRiser,MinTread,MinStairWidth,MinDoorWidth,MaxRampSlope,MinCorridorWidth,MaxCompartmentArea);
enumeration!(RuleSeverity;Error,Warning,Note);enumeration!(IssueStatus;Open,InProgress,Resolved,Closed);enumeration!(IssuePriority;Low,Normal,High,Critical);
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
scalar_record!(Point3;x:f64,y:f64,z:f64);
scalar_record!(ClashRef;set:String,first:String,second:String);scalar_record!(SectionBox;min:Point3,max:Point3);
impl Scalar for MepShape{
 const WIDTH:usize=10;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Duct{width,height}=>{c.push(Cell::Text("Duct"))?;width.append(c)?;height.append(c)?;c.nulls(3)},Self::Tray{width,height}=>{c.push(Cell::Text("Tray"))?;width.append(c)?;height.append(c)?;c.nulls(3)},Self::Pipe{diameter}=>{c.push(Cell::Text("Pipe"))?;c.nulls(6)?;diameter.append(c)}}}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let kind=match next(row,index)?{SqliteValue::Text(value)=>value.as_str(),_=>return Err(invalid("BIM MEP shape requires symbol"))};match kind{"Duct"|"Tray"=>{let width=f64::read(row,index,n)?;let height=f64::read(row,index,n)?;absent(row,*index,3)?;*index+=3;Ok(if kind=="Duct"{Self::Duct{width,height}}else{Self::Tray{width,height}})},"Pipe"=>{absent(row,*index,6)?;*index+=6;Ok(Self::Pipe{diameter:f64::read(row,index,n)?})},_=>Err(invalid("BIM MEP shape symbol is undeclared"))}}
}
pub struct Reader<'d,'n,'p>{database:&'d SqliteDatabase,pub native:&'n mut NativeDecodeControl<'p>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'d SqliteRow>>>,used:BTreeSet<(&'static str,i64)>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 pub fn new(database:&'d SqliteDatabase,native:&'n mut NativeDecodeControl<'p>)->Self{
   Self{database,native,groups:BTreeMap::new(),used:BTreeSet::new()}
 }

 fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<()>{self.native.charge(96)?;if !self.used.insert((table,row.rowid)){return Err(invalid("BIM row is multiply owned"))}Ok(())}
 pub fn take(&mut self,table:&'static str,parent:i64,column:usize,width:usize,ordered:Option<usize>)->Result<Vec<&'d SqliteRow>>{
  let key=(table,column);if !self.groups.contains_key(&key){let rows=&self.database.table(table)?.rows;self.native.charge(rows.len().checked_mul(256).ok_or_else(||invalid("BIM ownership workspace overflow"))?)?;let mut groups:BTreeMap<i64,Vec<&SqliteRow>>=BTreeMap::new();let mut ids=BTreeSet::new();for row in rows{self.native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||!ids.insert(row.rowid){return Err(invalid("BIM relational row shape differs"))}match row.values.get(column){Some(SqliteValue::Integer(parent))=>groups.entry(*parent).or_default().push(row),Some(SqliteValue::Null)=>{},_=>return Err(invalid("BIM ownership key is invalid"))}}self.groups.insert(key,groups);}
  let rows=self.groups.get_mut(&key).and_then(|groups|groups.remove(&parent)).unwrap_or_default();if let Some(order_column)=ordered{let mut ordered=BTreeMap::new();for row in rows{self.native.step()?;let ordinal=usize::try_from(row.integer(order_column)?).map_err(|e|invalid(e.to_string()))?;if ordered.insert(ordinal,row).is_some(){return Err(invalid("BIM relationship ordinal is repeated"))}}let mut result=self.native.allocate_vec(ordered.len())?;for(index,(ordinal,row))in ordered.into_iter().enumerate(){self.native.step()?;if ordinal!=index{return Err(invalid("BIM relationship ordinals are not dense"))}self.use_row(table,row)?;result.push(row)}Ok(result)}else{for row in &rows{self.native.step()?;self.use_row(table,row)?;}Ok(rows)}
 }
 pub fn optional(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<Option<&'d SqliteRow>>{let rows=self.take(table,parent,column,width,None)?;if rows.len()>1{return Err(invalid("BIM owned singleton is repeated"))}Ok(rows.into_iter().next())}
 pub fn required(&mut self,table:&'static str,parent:i64,column:usize,width:usize)->Result<&'d SqliteRow>{self.optional(table,parent,column,width)?.ok_or_else(||invalid("BIM required owned singleton is absent"))}
 pub fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow>{let rows=&self.database.table(table)?.rows;if rows.len()!=1||rows[0].rowid<=0||rows[0].integer(0)?!=rows[0].rowid||rows[0].values.len()!=width{return Err(invalid("BIM document singleton shape differs"))}self.use_row(table,&rows[0])?;Ok(&rows[0])}
 pub fn list<T:Scalar>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>>{let rows=self.take(table,parent,1,3+T::WIDTH,Some(2))?;let mut values=self.native.allocate_vec(rows.len())?;for row in rows{self.native.step()?;let mut index=3;values.push(T::read(row,&mut index,self.native)?)}Ok(values)}
 pub fn finish(self)->Result<()>{let mut total=0;for table in &self.database.tables{self.native.step()?;total=add(total,table.rows.len())?;}if total!=self.used.len(){return Err(invalid("BIM contains unowned rows"))}self.native.checkpoint()}
}
pub fn ordinal(index:usize)->Result<i64>{i64::try_from(index).map_err(|e|invalid(e.to_string()))}
pub fn list<T:Scalar>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&[T])->Result<()>{for(index,value)in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;value.append(&mut c)?;c.emit(p,table)?;}Ok(())}
pub fn active(row:&SqliteRow,start:usize,count:usize,selected:usize)->Result<()>{for index in start..start+count{if index==selected{if !matches!(row.values.get(index),Some(SqliteValue::Integer(_))){return Err(invalid("BIM active owner is absent"))}}else if !matches!(row.values.get(index),Some(SqliteValue::Null)){return Err(invalid("BIM variant has multiple owners"))}}Ok(())}
pub fn absent(row:&SqliteRow,start:usize,count:usize)->Result<()>{if row.values.get(start..start+count).is_some_and(|cells|cells.iter().all(|v|matches!(v,SqliteValue::Null))){Ok(())}else{Err(invalid("BIM inactive variant columns are populated"))}}
pub fn owners(c:&mut Cells<'_>,parent:i64,count:usize,slot:usize)->Result<()>{for index in 0..count{c.push(if index==slot{Cell::Integer(parent)}else{Cell::Null})?;}Ok(())}
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
pub fn write_property<'a>(value:&'a PropertyValue,c:&mut Cells<'a>)->Result<()>{match value{
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
