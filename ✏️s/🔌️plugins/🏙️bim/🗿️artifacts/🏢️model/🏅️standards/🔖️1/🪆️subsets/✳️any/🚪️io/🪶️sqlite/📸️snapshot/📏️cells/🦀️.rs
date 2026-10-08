//! 📏️ Borrowed native BIM roles are counted before typed reconstruction.
use super::*;
use semio_framework_dsl_record::{FieldValue as F,RecordValue as R};
use semio_framework_value::{NativeDecodeControl as N,DslValue};
use store::sqlite_snapshot::SqliteDatabaseLimits;
const WIDTHS:&[(&str,usize)]=&[
 ("bim_document",2),("bim_project",6),("bim_phase_name",4),
 ("bim_material",Material::WIDTH+4),("bim_wall_type",WallType::WIDTH+4),("bim_slab_type",SlabType::WIDTH+4),("bim_roof_type",RoofType::WIDTH+4),
 ("bim_column_type",ColumnType::WIDTH+4),("bim_beam_type",BeamType::WIDTH+4),("bim_window_type",WindowType::WIDTH+4),("bim_door_type",DoorType::WIDTH+4),
 ("bim_site",Site::WIDTH+4),("bim_building",Building::WIDTH+4),("bim_storey",Storey::WIDTH+4),("bim_grid",GridLine::WIDTH+4),
 ("bim_wall",Wall::WIDTH+4),("bim_curtain_wall",CurtainWall::WIDTH+4),("bim_column",Column::WIDTH+4),("bim_beam",Beam::WIDTH+4),
 ("bim_slab",Slab::WIDTH+4),("bim_roof",Roof::WIDTH+4),("bim_opening",Opening::WIDTH+4),("bim_stair",Stair::WIDTH+4),
 ("bim_railing",Railing::WIDTH+4),("bim_space",Space::WIDTH+4),("bim_property_element",4),("bim_classification",Classification::WIDTH+4),
 ("bim_wall_layer",8),("bim_slab_layer",8),("bim_roof_layer",8),("bim_site_boundary",9),("bim_axis",19),("bim_top",13),
 ("bim_baluster",5),("bim_profile",23),("bim_profile_outline",12),("bim_slab_boundary",12),("bim_slab_hole",3),("bim_slab_hole_vertex",12),
 ("bim_roof_footprint",12),("bim_railing_path",9),("bim_space_outline",12),("bim_property_set",4),("bim_property_value",11)];
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{if SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<WIDTHS.len()||WIDTHS.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<2{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"BIM authored schema extent exceeds caller limits"))}Ok(())}
fn shape(value:&R,count:u16)->Result<()>{if value.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("BIM native record field map differs"))}}
fn field(value:&R,id:u16)->Result<&F>{value.get(id).ok_or_else(||invalid("BIM native required role is absent"))}
fn record(value:&F,count:u16)->Result<&R>{let F::Record(value)=value else{return Err(invalid("BIM native role requires record"))};shape(value,count)?;Ok(value)}
fn block(value:&F,count:u16)->Result<&R>{let F::Block(value)=value else{return Err(invalid("BIM native role requires authored block"))};record(value,count)}
fn items(value:&F)->Result<&[F]>{let F::List(value)=value else{return Err(invalid("BIM native role requires ordered list"))};Ok(value)}
fn map(value:&F)->Result<&[(String,F)]>{let F::Map(value)=value else{return Err(invalid("BIM native role requires literal map"))};if value.windows(2).any(|pair|pair[0].0>=pair[1].0){return Err(invalid("BIM native map keys are not strictly sorted"))}Ok(value)}
fn variant(value:&F)->Result<(&str,&R)>{let F::Block(value)=value else{return Err(invalid("BIM variant requires authored block"))};let F::Statements(value)=value.as_ref()else{return Err(invalid("BIM variant requires statements"))};let[(name,value)]=value.as_slice()else{return Err(invalid("BIM variant count differs"))};let name=match name.as_str(){"line"=>"Line","arc"=>"Arc","unconnected"=>"Unconnected","storey-top"=>"StoreyTop","storey"=>"Storey","rectangle"=>"Rectangle","circle"=>"Circle","i-shape"=>"IShape","custom"=>"Custom","flat"=>"Flat","shed"=>"Shed","gable"=>"Gable","hip"=>"Hip","mansard"=>"Mansard","window"=>"Window","door"=>"Door","void"=>"Void","straight"=>"Straight","l-turn"=>"LTurn","u-turn"=>"UTurn","spiral"=>"Spiral","none"=>"None","glass"=>"Glass","panel"=>"Panel","bounded"=>"Bounded","explicit"=>"Explicit",_=>return Err(invalid("BIM variant keyword is undeclared"))};Ok((name,value))}
fn enumeration(value:&F,names:&[&str],n:&mut N<'_>)->Result<usize>{n.step()?;let F::Enum(index)=value else{return Err(invalid("BIM scalar enum requires ordinal"))};names.get(*index as usize).map(|name|name.len()).ok_or_else(||invalid("BIM enum ordinal is undeclared"))}
fn floats(value:&R,ids:&[u16],n:&mut N<'_>)->Result<usize>{let mut bytes=0;for id in ids{bytes=add(bytes,cost(field(value,*id)?,"f64",n)?)?;}Ok(bytes)}
fn cost(value:&F,kind:&str,n:&mut N<'_>)->Result<usize>{n.step()?;match kind{
 "text"=>match value{F::Text(value)=>Ok(value.len()),_=>Err(invalid("BIM scalar requires text"))},
 "f64"=>match value{F::Float(value)=>Ok(if value.is_finite(){16+class(*value).len()}else{8+class(*value).len()}),_=>Err(invalid("BIM scalar requires binary64"))},
 "i32"=>match value{F::Int(value)if i32::try_from(*value).is_ok()=>Ok(8),_=>Err(invalid("BIM scalar requires signed32"))},
 "u32"=>match value{F::UInt(value)if u32::try_from(*value).is_ok()=>Ok(8),_=>Err(invalid("BIM scalar requires unsigned32"))},
 "bool"=>match value{F::Bool(_)=>Ok(8),_=>Err(invalid("BIM scalar requires Boolean"))},
 "?f64"=>if matches!(value,F::Absent){Ok(0)}else{cost(value,"f64",n)},
 "point"=>floats(block(value,2)?,&[0,1],n),"rgb"=>floats(block(value,3)?,&[0,1,2],n),
 "?slope"=>if matches!(value,F::Absent){Ok(0)}else{floats(block(value,2)?,&[0,1],n)},
 "stringer"=>{let value=block(value,3)?;add(enumeration(field(value,0)?,&["None","Closed","Open","Mono"],n)?,floats(value,&[1,2],n)?)},
 "material"=>enumeration(value,&["Concrete","Masonry","Wood","Metal","Glass","Insulation","Finish","Membrane","Other"],n),
 "layer"=>enumeration(value,&["Structure","Substrate","Insulation","Finish","Membrane","Core"],n),
 "location"=>enumeration(value,&["Center","Interior","Exterior","CoreCenter"],n),"phase"=>enumeration(value,&["Existing","New","Demolished","Temporary"],n),
 "leaves"=>enumeration(value,&["Single","Double"],n),"swing"|"turn"=>enumeration(value,&["Left","Right"],n),"riser"=>enumeration(value,&["Open","Closed"],n),
 "roof"|"opening"|"flight"|"infill"=>{let(name,value)=variant(value)?;let (count,roles): (u16,&[(u16,&str)])=match(kind,name){
 ("roof","Flat")=>(0,&[]),("roof","Shed")|( "roof","Gable")=>(2,&[(0,"f64"),(1,"f64")]),("roof","Hip")=>(1,&[(0,"f64")]),("roof","Mansard")=>(3,&[(0,"f64"),(1,"f64"),(2,"f64")]),
 ("opening","Window")|("opening","Door")=>(1,&[(0,"text")]),("opening","Void")=>(2,&[(0,"f64"),(1,"f64")]),
 ("flight","Straight")=>(0,&[]),("flight","LTurn")=>(2,&[(0,"f64"),(1,"turn")]),("flight","UTurn")=>(1,&[(0,"f64")]),("flight","Spiral")=>(2,&[(0,"f64"),(1,"f64")]),
 ("infill","None")=>(0,&[]),("infill","Glass")|("infill","Panel")=>(1,&[(0,"f64")]),_=>return Err(invalid("BIM native inline variant is undeclared"))};shape(value,count)?;add(name.len(),scalars(value,roles,n)?)},
 "space"=>{let(name,value)=variant(value)?;match name{"Bounded"=>{shape(value,1)?;add(name.len(),cost(field(value,0)?,"point",n)?)},"Explicit"=>{shape(value,1)?;let _=items(field(value,0)?)?;Ok(name.len())},_=>Err(invalid("BIM space variant is undeclared"))}},
 _=>Err(invalid("BIM scalar role is undeclared"))}}
fn scalars(value:&R,roles:&[(u16,&str)],n:&mut N<'_>)->Result<usize>{let mut bytes=0;for(id,kind)in roles{bytes=add(bytes,cost(field(value,*id)?,kind,n)?)?;}Ok(bytes)}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{n.step()?;if !WIDTHS.iter().any(|(name,_)|*name==table){return Err(invalid("BIM native semantic table is undeclared"))}let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"BIM semantic row limit exceeded"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"BIM semantic value limit exceeded"))}self.rows=rows;self.bytes=bytes;Ok(())}
 fn list(&mut self,value:&F,table:&str,kind:&str,n:&mut N<'_>)->Result<()>{for value in items(value)?{let value=record(value,if kind=="point"{2}else if kind=="layer"{3}else{2})?;let bytes=match kind{"point"=>floats(value,&[0,1],n)?,"layer"=>scalars(value,&[(0,"text"),(1,"f64"),(2,"layer")],n)?,_=>scalars(value,&[(0,"point"),(1,"f64")],n)?};self.row(table,add(24,bytes)?,n)?;}Ok(())}
 fn axis(&mut self,value:&F,n:&mut N<'_>)->Result<()>{let(name,value)=variant(value)?;let roles:&[(u16,&str)]=match name{"Line"=>{shape(value,2)?;&[(0,"point"),(1,"point")]},"Arc"=>{shape(value,3)?;&[(0,"point"),(1,"point"),(2,"f64")]},_=>return Err(invalid("BIM axis kind is undeclared"))};self.row("bim_axis",add(add(16,name.len())?,scalars(value,roles,n)?)?,n)}
 fn top(&mut self,value:&F,n:&mut N<'_>)->Result<()>{let(name,value)=variant(value)?;let roles:&[(u16,&str)]=match name{"Unconnected"|"StoreyTop"=>{shape(value,1)?;&[(0,"f64")]},"Storey"=>{shape(value,2)?;&[(0,"text"),(1,"f64")]},_=>return Err(invalid("BIM top kind is undeclared"))};self.row("bim_top",add(add(16,name.len())?,scalars(value,roles,n)?)?,n)}
 fn profile(&mut self,value:&F,n:&mut N<'_>)->Result<()>{let(name,value)=variant(value)?;let roles:&[(u16,&str)]=match name{"Rectangle"=>{shape(value,2)?;&[(0,"f64"),(1,"f64")]},"Circle"=>{shape(value,1)?;&[(0,"f64")]},"IShape"=>{shape(value,4)?;&[(0,"f64"),(1,"f64"),(2,"f64"),(3,"f64")]},"Custom"=>{shape(value,1)?;&[]},_=>return Err(invalid("BIM profile kind is undeclared"))};self.row("bim_profile",add(add(16,name.len())?,scalars(value,roles,n)?)?,n)?;if name=="Custom"{self.list(field(value,0)?,"bim_profile_outline","vertex",n)?;}Ok(())}
}
fn property(value:&F,n:&mut N<'_>)->Result<usize>{n.step()?;let F::Value(DslValue::Object(values))=value else{return Err(invalid("BIM property requires intrinsic tagged object"))};let[(kind,value)]=values.as_slice()else{return Err(invalid("BIM property tag count differs"))};let DslValue::Object(values)=value else{return Err(invalid("BIM property requires payload object"))};let[(name,value)]=values.as_slice()else{return Err(invalid("BIM property payload count differs"))};if name!="value"{return Err(invalid("BIM property payload role differs"))}let bytes=match(kind.as_str(),value){("Text",DslValue::String(value))=>value.len(),("Integer",value)if value.as_i64().is_some_and(|v|i32::try_from(v).is_ok())=>8,("Boolean",DslValue::Bool(_))=>8,("Real"|"Length"|"Area"|"Volume"|"Angle",value)=>{let value=value.as_f64().ok_or_else(||invalid("BIM property requires binary64"))?;if value.is_finite(){16+class(value).len()}else{8+class(value).len()}},_=>return Err(invalid("BIM property typed payload differs"))};add(kind.len(),bytes)}
pub(super)fn admit_record(source:&R,limits:SqliteDatabaseLimits,n:&mut N<'_>)->Result<()>{extent(limits)?;n.scoped_stage(|n|{n.begin_stage(0)?;shape(source,26)?;let mut c=Census{limits,rows:0,bytes:0};c.row("bim_document",add(8,cost(field(source,0)?,"text",n)?)?,n)?;let project=block(field(source,1)?,5)?;c.row("bim_project",add(16,scalars(project,&[(0,"text"),(1,"text"),(2,"text"),(3,"text")],n)?)?,n)?;for name in items(field(project,4)?)?{c.row("bim_phase_name",add(24,cost(name,"text",n)?)?,n)?;}
 for id in 2..26{let(table,count,roles):(&str,u16,&[(u16,&str)])=match id{
 2=>("bim_material",6,&[(0,"text"),(1,"material"),(2,"rgb"),(3,"f64"),(4,"f64"),(5,"f64")]),
 3=>("bim_wall_type",2,&[(0,"text")]),4=>("bim_slab_type",2,&[(0,"text")]),5=>("bim_roof_type",2,&[(0,"text")]),
 6=>("bim_column_type",3,&[(0,"text"),(2,"text")]),7=>("bim_beam_type",3,&[(0,"text"),(2,"text")]),
 8=>("bim_window_type",8,&[(0,"text"),(1,"f64"),(2,"f64"),(3,"f64"),(4,"f64"),(5,"f64"),(6,"u32"),(7,"text")]),
 9=>("bim_door_type",8,&[(0,"text"),(1,"f64"),(2,"f64"),(3,"f64"),(4,"f64"),(5,"leaves"),(6,"swing"),(7,"text")]),
 10=>("bim_site",6,&[(0,"text"),(1,"f64"),(2,"f64"),(3,"f64"),(4,"f64")]),11=>("bim_building",5,&[(0,"text"),(1,"text"),(2,"point"),(3,"f64"),(4,"f64")]),
 12=>("bim_storey",5,&[(0,"text"),(1,"text"),(2,"i32"),(3,"f64"),(4,"?f64")]),13=>("bim_grid",4,&[(0,"text"),(1,"text"),(2,"point"),(3,"point")]),
 14=>("bim_wall",8,&[(0,"text"),(1,"text"),(3,"location"),(4,"f64"),(6,"phase"),(7,"text")]),
 15=>("bim_curtain_wall",10,&[(0,"text"),(2,"f64"),(4,"f64"),(5,"f64"),(7,"text"),(8,"text"),(9,"text")]),
 16=>("bim_column",7,&[(0,"text"),(1,"text"),(2,"point"),(3,"f64"),(4,"f64"),(6,"text")]),17=>("bim_beam",6,&[(0,"text"),(1,"text"),(2,"point"),(3,"point"),(4,"f64"),(5,"text")]),
 18=>("bim_slab",7,&[(0,"text"),(1,"text"),(4,"f64"),(5,"?slope"),(6,"text")]),19=>("bim_roof",7,&[(0,"text"),(1,"text"),(3,"roof"),(4,"f64"),(5,"f64"),(6,"text")]),
 20=>("bim_opening",9,&[(0,"text"),(1,"opening"),(2,"f64"),(3,"?f64"),(4,"?f64"),(5,"?f64"),(6,"bool"),(7,"bool"),(8,"text")]),
 21=>("bim_stair",14,&[(0,"text"),(1,"point"),(2,"f64"),(3,"f64"),(4,"flight"),(6,"f64"),(7,"f64"),(8,"stringer"),(9,"f64"),(10,"f64"),(11,"riser"),(12,"f64"),(13,"text")]),
 22=>("bim_railing",11,&[(0,"text"),(2,"f64"),(3,"f64"),(7,"infill"),(8,"text"),(9,"f64"),(10,"text")]),23=>("bim_space",5,&[(0,"text"),(1,"text"),(2,"text"),(3,"space"),(4,"text")]),
 24=>("bim_property_element",0,&[]),25=>("bim_classification",3,&[(0,"text"),(1,"text"),(2,"text")]),_=>return Err(invalid("BIM native root role is undeclared"))};
 for(key,value)in map(field(source,id)?)?{n.step()?;if id==24{c.row(table,add(24,key.len())?,n)?;for(name,properties)in map(value)?{c.row("bim_property_set",add(24,name.len())?,n)?;for(name,value)in map(properties)?{c.row("bim_property_value",add(add(24,name.len())?,property(value,n)?)?,n)?;}}continue}
 let value=record(value,count)?;c.row(table,add(add(24,key.len())?,scalars(value,roles,n)?)?,n)?;match id{
 3..=5=>c.list(field(value,1)?,match id{3=>"bim_wall_layer",4=>"bim_slab_layer",_=>"bim_roof_layer"},"layer",n)?,6|7=>c.profile(field(value,1)?,n)?,10=>c.list(field(value,5)?,"bim_site_boundary","point",n)?,
 14=>{c.axis(field(value,2)?,n)?;c.top(field(value,5)?,n)?;},15=>{c.axis(field(value,1)?,n)?;c.top(field(value,3)?,n)?;c.profile(field(value,6)?,n)?;},16=>c.top(field(value,5)?,n)?,
 18=>{c.list(field(value,2)?,"bim_slab_boundary","vertex",n)?;for hole in items(field(value,3)?)?{c.row("bim_slab_hole",24,n)?;c.list(hole,"bim_slab_hole_vertex","vertex",n)?;}},19=>c.list(field(value,2)?,"bim_roof_footprint","vertex",n)?,21=>c.top(field(value,5)?,n)?,
 22=>{c.list(field(value,1)?,"bim_railing_path","point",n)?;c.profile(field(value,4)?,n)?;c.profile(field(value,5)?,n)?;let baluster=field(value,6)?;if !matches!(baluster,F::Absent){let baluster=block(baluster,2)?;c.row("bim_baluster",add(16,cost(field(baluster,1)?,"f64",n)?)?,n)?;c.profile(field(baluster,0)?,n)?;}},
 23=>{let(name,value)=variant(field(value,3)?)?;if name=="Explicit"{c.list(field(value,0)?,"bim_space_outline","vertex",n)?;}},_=>{}
 }}}
 n.checkpoint()})}
