//! 🏢️ Literal BIM fields, exact scalar words and owned authored relationships.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{AnchorEnd,AnnotationAnchor,AreaMeasure,Axis,Baluster,ClashRef,ClassificationItem,CurtainGrid,CurtainPanel,DetailLevel,DimensionUnit,DoorLeaves,ElementSelector,EndJoin,ExprPoint3,FamilyCategory,Infill,IssuePriority,IssueStatus,IssueViewpoint,Layer,LocationLine,MaterialCategory,MepShape,MepSystem,OpeningKind,Orientation,Paper,ParameterKind,ParametricProfile,Phase,Point2,Point3,Profile,Project,PropertyDef,PropertyKind,PropertySet,PropertyValue,RailingHost,Rgb,RiserKind,RoofShape,RuleKind,RuleScope,RuleSeverity,ScheduleCategory,ScheduleColumn,ScheduleFilter,ScheduleGroup,ScheduleKey,ScheduleOp,ScheduleSort,SectionBox,Slope,SolidAxis,SolidShape,SpaceBoundary,StairFlight,StairStringer,Swing,TagCategory,TemplateTarget,Terminator,TopConstraint,Vertex,ViewCamera,ViewCategory,ViewCrop,ViewKind,ViewPlane,WallSide};
use crate::standards::v1::subsets::any::schema::snapshot::{AnnotationStyle,AreaScheme,Beam,BeamType,Building,Ceiling,CeilingType,ClashSet,ClassificationSystem,Column,ColumnType,Component,ComponentOverride,CurtainPanelOverride,CurtainWall,CurtainWallType,DesignOption,Dimension,DoorType,ElementMembership,Family,FamilyParameter,FamilySolid,GridLine,Issue,IssueComment,Leader,LoadCase,Material,MepElement,ModelSnapshot,Opening,OptionGroup,PropertyTemplate,Railing,Ramp,Restraints,Roof,RoofType,Rule,Schedule,Sheet,SheetRevision,Site,Slab,SlabType,Space,SpaceConditions,Stair,Storey,StructuralLoad,StructuralLocation,StructuralSupport,Tag,TextNote,View,Viewport,Wall,WallSweep,WallType,WindowType,Workset,Zone};
use semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap;
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_value::ValueError;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,RowWriter}};
pub use semio_s_artifact_bim_model_sqlite_schema::SCHEMA;
use semio_s_artifact_bim_model_sqlite_schema::scalar::{absent, active, list, next, ordinal, owners, write_property, Cells, Reader, Scalar};
use semio_s_artifact_bim_model_sqlite_schema::{add,census as semantic_cells,compare_text,invalid};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
type Result<T>=std::result::Result<T,ValueError>;
impl Scalar for Restraints {
    const WIDTH: usize = 6;

    fn append<'a>(&'a self, cells: &mut Cells<'a>) -> Result<()> {
        for value in [&self.x, &self.y, &self.z, &self.rx, &self.ry, &self.rz] {
            value.append(cells)?;
        }
        Ok(())
    }

    fn read(row: &SqliteRow, index: &mut usize, native: &mut NativeDecodeControl<'_>) -> Result<Self> {
        Ok(Self {
            x: bool::read(row, index, native)?,
            y: bool::read(row, index, native)?,
            z: bool::read(row, index, native)?,
            rx: bool::read(row, index, native)?,
            ry: bool::read(row, index, native)?,
            rz: bool::read(row, index, native)?,
        })
    }
}
impl Scalar for StructuralLocation{
 const WIDTH:usize=10;
 fn append<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{match self{Self::Point{station}=>{c.push(Cell::Text("Point"))?;station.append(c)?;c.nulls(6)},Self::Line{start,end}=>{c.push(Cell::Text("Line"))?;c.nulls(3)?;start.append(c)?;end.append(c)},Self::Area=>{c.push(Cell::Text("Area"))?;c.nulls(9)}}}
 fn read(row:&SqliteRow,index:&mut usize,n:&mut NativeDecodeControl<'_>)->Result<Self>{let kind=match next(row,index)?{SqliteValue::Text(value)=>value.as_str(),_=>return Err(invalid("BIM structural location requires symbol"))};match kind{"Point"=>{let station=f64::read(row,index,n)?;absent(row,*index,6)?;*index+=6;Ok(Self::Point{station})},"Line"=>{absent(row,*index,3)?;*index+=3;Ok(Self::Line{start:f64::read(row,index,n)?,end:f64::read(row,index,n)?})},"Area"=>{absent(row,*index,9)?;*index+=9;Ok(Self::Area)},_=>Err(invalid("BIM structural location symbol is undeclared"))}}
}
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
entity!(StructuralSupport;name:String,member:String,location:StructuralLocation,offset:Point3,restraints:Restraints);
entity!(LoadCase;name:String,category:String,factor:f64);entity!(StructuralLoad;name:String,load_case:String,member:String,location:StructuralLocation,force:Point3,moment:Point3);
entity!(OptionGroup;name:String);entity!(DesignOption;group:String,name:String,primary:bool);entity!(Workset;name:String,default_visible:bool);entity!(ElementMembership;target:String);
entity!(SpaceConditions;occupancy:Option<String>,occupancy_density:Option<f64>,heating_setpoint:Option<f64>,cooling_setpoint:Option<f64>,ventilation_rate:Option<f64>,lighting_power_density:Option<f64>,equipment_power_density:Option<f64>,schedule:Option<String>);
entity!(ClashSet;name:String,tolerance:f64,clearance:f64;a:ElementSelector=>write_selector(("bim_clash_a",["bim_clash_a_class","bim_clash_a_storey","bim_clash_a_phase","bim_clash_a_element"])),read_selector(("bim_clash_a",["bim_clash_a_class","bim_clash_a_storey","bim_clash_a_phase","bim_clash_a_element"])),b:ElementSelector=>write_selector(("bim_clash_b",["bim_clash_b_class","bim_clash_b_storey","bim_clash_b_phase","bim_clash_b_element"])),read_selector(("bim_clash_b",["bim_clash_b_class","bim_clash_b_storey","bim_clash_b_phase","bim_clash_b_element"])));
entity!(Rule;name:String,kind:RuleKind,limit:f64,severity:RuleSeverity;scope:RuleScope=>write_scope(()),read_scope(()));
entity!(Issue;title:String,description:String,status:IssueStatus,priority:IssuePriority,assignee:String,author:String,created:String,clash:Option<ClashRef>;labels:Vec<String> =>write_list("bim_issue_label"),read_list("bim_issue_label"),elements:Vec<String> =>write_list("bim_issue_element"),read_list("bim_issue_element"),viewpoint:Option<IssueViewpoint> =>write_viewpoint(()),read_viewpoint(()));
entity!(IssueComment;issue:String,author:String,date:String,text:String);
entity!(Component;storey:String,family:String,position:Point2,elevation:f64,rotation:f64,mirrored:bool,host:Option<String>,system:Option<MepSystem>,name:String);
entity!(ComponentOverride;component:String,name:String,value:String);
entity!(MepElement;storey:String,system:MepSystem,shape:MepShape,name:String;path:Vec<Point3> =>write_list("bim_mep_path"),read_list("bim_mep_path"));
fn write_selector(p:&mut RowWriter<'_,'_>,parent:i64,value:&ElementSelector,tables:(&str,[&str;4]))->Result<()>{let id=p.insert(tables.0,&[Cell::Integer(parent)])?;list(p,tables.1[0],id,&value.classes)?;list(p,tables.1[1],id,&value.storeys)?;list(p,tables.1[2],id,&value.phases)?;list(p,tables.1[3],id,&value.ids)}
fn read_selector(r:&mut Reader<'_,'_,'_>,parent:i64,tables:(&'static str,[&'static str;4]))->Result<ElementSelector>{let row=r.required(tables.0,parent,1,2)?;Ok(ElementSelector{classes:r.list(tables.1[0],row.rowid)?,storeys:r.list(tables.1[1],row.rowid)?,phases:r.list(tables.1[2],row.rowid)?,ids:r.list(tables.1[3],row.rowid)?})}
fn write_scope(p:&mut RowWriter<'_,'_>,parent:i64,value:&RuleScope,_:())->Result<()>{let id=p.insert("bim_rule_scope",&[Cell::Integer(parent),Cell::Text(&value.filter)])?;list(p,"bim_rule_scope_storey",id,&value.storeys)?;list(p,"bim_rule_scope_phase",id,&value.phases)?;list(p,"bim_rule_scope_element",id,&value.ids)}
fn read_scope(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<RuleScope>{let row=r.required("bim_rule_scope",parent,1,3)?;let mut index=2;Ok(RuleScope{filter:String::read(row,&mut index,r.native)?,storeys:r.list("bim_rule_scope_storey",row.rowid)?,phases:r.list("bim_rule_scope_phase",row.rowid)?,ids:r.list("bim_rule_scope_element",row.rowid)?})}
fn write_viewpoint(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<IssueViewpoint>,_:())->Result<()>{if let Some(value)=value{let mut c=Cells::new();c.push(Cell::Integer(parent))?;value.camera.append(&mut c)?;value.section.append(&mut c)?;let id=c.emit(p,"bim_issue_viewpoint")?;list(p,"bim_issue_isolate",id,&value.isolate)?;}Ok(())}
fn read_viewpoint(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Option<IssueViewpoint>>{let Some(row)=r.optional("bim_issue_viewpoint",parent,1,38)?else{return Ok(None)};let mut index=2;Ok(Some(IssueViewpoint{camera:ViewCamera::read(row,&mut index,r.native)?,section:Option::<SectionBox>::read(row,&mut index,r.native)?,isolate:r.list("bim_issue_isolate",row.rowid)?}))}
entity!(Material;name:String,category:MaterialCategory,color:Rgb,density:f64,conductivity:f64,specific_heat:f64);
entity!(WallType;name:String;layers:Vec<Layer> =>write_list("bim_wall_layer"),read_list("bim_wall_layer"));
entity!(SlabType;name:String;layers:Vec<Layer> =>write_list("bim_slab_layer"),read_list("bim_slab_layer"));
entity!(RoofType;name:String;layers:Vec<Layer> =>write_list("bim_roof_layer"),read_list("bim_roof_layer"));
entity!(ColumnType;name:String,material:String;profile:Profile=>write_profile(0),read_profile(0));
entity!(BeamType;name:String,material:String;profile:Profile=>write_profile(1),read_profile(1));
entity!(WindowType;name:String,width:f64,height:f64,sill:f64,frame_width:f64,frame_depth:f64,panes:u32,material:String,u_value:Option<f64>,g_value:Option<f64>,frame_fraction:Option<f64>);
entity!(DoorType;name:String,width:f64,height:f64,frame_width:f64,frame_depth:f64,leaves:DoorLeaves,swing:Swing,material:String,u_value:Option<f64>);
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
entity!(Site;name:String,latitude:f64,longitude:f64,elevation:f64,true_north:f64;boundary:Vec<Point2> =>write_list("bim_site_boundary"),read_list("bim_site_boundary"));
entity!(Building;site:String,name:String,origin:Point2,rotation:f64,elevation:f64);
entity!(Storey;building:String,name:String,level:i32,height:f64,cut_height:Option<f64>);
entity!(GridLine;building:String,label:String,start:Point2,end:Point2);
entity!(Wall;storey:String,wall_type:String,location:LocationLine,base_offset:f64,phase:Phase,start_join:Option<EndJoin>,end_join:Option<EndJoin>,name:String,base_slab:Option<String>;axis:Axis=>write_axis(0),read_axis(0),top:TopConstraint=>write_top(0),read_top(0));
entity!(Column;storey:String,column_type:String,position:Point2,rotation:f64,tilt:Option<Slope>,base_offset:f64,phase:Phase,name:String;top:TopConstraint=>write_top(2),read_top(2));
entity!(Beam;storey:String,beam_type:String,top_offset:f64,end_top_offset:Option<f64>,phase:Phase,name:String;axis:Axis=>write_axis(2),read_axis(2));
fn write_holes(p:&mut RowWriter<'_,'_>,parent:i64,value:&Vec<Vec<Vertex>>,tables:(&str,&str))->Result<()>{for(index,hole)in value.iter().enumerate(){let id=p.insert(tables.0,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?)])?;list(p,tables.1,id,hole)?;}Ok(())}
fn read_holes(r:&mut Reader<'_,'_,'_>,parent:i64,tables:(&'static str,&'static str))->Result<Vec<Vec<Vertex>>>{let rows=r.take(tables.0,parent,1,3,Some(2))?;let mut values=r.native.allocate_vec(rows.len())?;for row in rows{r.native.step()?;values.push(r.list(tables.1,row.rowid)?)}Ok(values)}
entity!(Slab;storey:String,slab_type:String,offset:f64,slope:Option<Slope>,phase:Phase,name:String;boundary:Vec<Vertex> =>write_list("bim_slab_boundary"),read_list("bim_slab_boundary"),holes:Vec<Vec<Vertex>> =>write_holes(("bim_slab_hole","bim_slab_hole_vertex")),read_holes(("bim_slab_hole","bim_slab_hole_vertex")));
entity!(Ceiling;storey:String,ceiling_type:String,offset:f64,slope:Option<Slope>,name:String;boundary:Vec<Vertex> =>write_list("bim_ceiling_boundary"),read_list("bim_ceiling_boundary"),holes:Vec<Vec<Vertex>> =>write_holes(("bim_ceiling_hole","bim_ceiling_hole_vertex")),read_holes(("bim_ceiling_hole","bim_ceiling_hole_vertex")));
entity!(Roof;storey:String,roof_type:String,shape:RoofShape,overhang:f64,base_offset:f64,phase:Phase,name:String;footprint:Vec<Vertex> =>write_list("bim_roof_footprint"),read_list("bim_roof_footprint"));
entity!(Opening;host:String,kind:OpeningKind,offset:f64,sill_override:Option<f64>,width:Option<f64>,height:Option<f64>,flip_hand:bool,flip_facing:bool,name:String,reveal_depth:Option<f64>,reveal_material:Option<String>);
entity!(Stair;storey:String,start:Point2,direction:f64,width:f64,flight:StairFlight,max_riser:f64,min_tread:f64,stringer:StairStringer,nosing:f64,tread_thickness:f64,riser:RiserKind,landing_depth:f64,phase:Phase,name:String;top:TopConstraint=>write_top(3),read_top(3));
fn write_baluster(p:&mut RowWriter<'_,'_>,parent:i64,value:&Option<Baluster>,_:())->Result<()>{if let Some(value)=value{let mut c=Cells::new();c.push(Cell::Integer(parent))?;value.spacing.append(&mut c)?;let id=c.emit(p,"bim_baluster")?;write_profile(p,id,&value.profile,6)?;}Ok(())}
fn read_baluster(r:&mut Reader<'_,'_,'_>,parent:i64,_:())->Result<Option<Baluster>>{let Some(row)=r.optional("bim_baluster",parent,1,5)?else{return Ok(None)};let mut i=2;let spacing=f64::read(row,&mut i,r.native)?;Ok(Some(Baluster{spacing,profile:read_profile(r,row.rowid,6)?}))}
entity!(Railing;storey:String,height:f64,post_spacing:f64,infill:Infill,material:String,base_offset:f64,host:Option<RailingHost>,phase:Phase,name:String;path:Vec<Point2> =>write_list("bim_railing_path"),read_list("bim_railing_path"),profile:Profile=>write_profile(4),read_profile(4),post_profile:Profile=>write_profile(5),read_profile(5),baluster:Option<Baluster> =>write_baluster(()),read_baluster(()));
impl Entity for Space{
 const WIDTH:usize=16;
 fn fields<'a>(&'a self,c:&mut Cells<'a>)->Result<()>{self.storey.append(c)?;self.number.append(c)?;self.name.append(c)?;match &self.boundary{SpaceBoundary::Bounded{seed}=>{c.push(Cell::Text("Bounded"))?;seed.append(c)?;},SpaceBoundary::Explicit{..}=>{c.push(Cell::Text("Explicit"))?;c.nulls(6)?;}}self.usage.append(c)?;self.phase.append(c)?;self.zone.append(c)?;self.floor_finish.append(c)?;self.wall_finish.append(c)?;self.ceiling_finish.append(c)}
 fn children(&self,p:&mut RowWriter<'_,'_>,parent:i64)->Result<()>{if let SpaceBoundary::Explicit{outline}=&self.boundary{list(p,"bim_space_outline",parent,outline)?;}Ok(())}
 fn read_at(r:&mut Reader<'_,'_,'_>,row:&SqliteRow,start:usize)->Result<Self>{r.native.charge(std::mem::size_of::<Self>())?;let mut i=start;let storey=String::read(row,&mut i,r.native)?;let number=String::read(row,&mut i,r.native)?;let name=String::read(row,&mut i,r.native)?;let kind=row.text(i)?;i+=1;let outline=r.list("bim_space_outline",row.rowid)?;let boundary=match kind{"Bounded"=>{if !outline.is_empty(){return Err(invalid("BIM bounded space owns an outline"))}SpaceBoundary::Bounded{seed:Point2::read(row,&mut i,r.native)?}},"Explicit"=>{absent(row,i,6)?;i+=6;SpaceBoundary::Explicit{outline}},_=>return Err(invalid("BIM space boundary kind is undeclared"))};let usage=String::read(row,&mut i,r.native)?;let phase=Phase::read(row,&mut i,r.native)?;let zone=Option::<String>::read(row,&mut i,r.native)?;let floor_finish=Option::<String>::read(row,&mut i,r.native)?;let wall_finish=Option::<String>::read(row,&mut i,r.native)?;let ceiling_finish=Option::<String>::read(row,&mut i,r.native)?;if i!=row.values.len(){return Err(invalid("BIM space has extra scalar columns"))}Ok(Self{storey,number,name,boundary,usage,phase,zone,floor_finish,wall_finish,ceiling_finish})}
}
entity!(CeilingType;name:String;layers:Vec<Layer> =>write_list("bim_ceiling_layer"),read_list("bim_ceiling_layer"));
entity!(CurtainWallType;name:String,panel:CurtainPanel,panel_material:String,mullion_material:String,u_value:Option<f64>,g_value:Option<f64>,frame_fraction:Option<f64>;u_grid:CurtainGrid=>write_grid(0),read_grid(0),v_grid:CurtainGrid=>write_grid(1),read_grid(1),interior_mullion:Profile=>write_profile(2),read_profile(2),border_mullion:Profile=>write_profile(3),read_profile(3));
entity!(CurtainWall;storey:String,curtain_wall_type:String,base_offset:f64,phase:Phase,name:String;axis:Axis=>write_axis(1),read_axis(1),top:TopConstraint=>write_top(1),read_top(1),u_grid:Option<CurtainGrid> =>write_optional_grid(2),read_optional_grid(2),v_grid:Option<CurtainGrid> =>write_optional_grid(3),read_optional_grid(3));
entity!(Ramp;storey:String,width:f64,landing_start:f64,landing_end:f64,landing_turn:f64,max_slope:f64,thickness:f64,material:String,base_offset:f64,railing_left:bool,railing_right:bool,name:String;path:Vec<Vertex> =>write_list("bim_ramp_path"),read_list("bim_ramp_path"),top:TopConstraint=>write_top(4),read_top(4));
entity!(AreaScheme;name:String,measure:AreaMeasure;usages:Vec<String> =>write_list("bim_area_usage"),read_list("bim_area_usage"),zones:Vec<String> =>write_list("bim_area_zone"),read_list("bim_area_zone"));
entity!(View;building:String,name:String,kind:ViewKind,storey:Option<String>,plane:Option<ViewPlane>,camera:Option<ViewCamera>,cut_height:Option<f64>,depth:f64,crop:Option<ViewCrop>,phase:Option<Phase>,scale:u32,detail:DetailLevel;hidden:Vec<ViewCategory> =>write_list("bim_view_hidden"),read_list("bim_view_hidden"));
entity!(Sheet;number:String,name:String,paper:Paper,orientation:Orientation,project:String,drawn_by:String,checked_by:String,date:String,revision:String,scale_label:String);
entity!(Dimension;storey:String,angle:f64,offset:f64,style:String,lock:Option<f64>,name:String;anchors:Vec<AnnotationAnchor> =>write_anchors(()),read_anchors(()));
entity!(Leader;storey:String,offset:Point2,text:String,style:String;anchor:AnnotationAnchor=>write_leader_anchor(()),read_leader_anchor(()));
entity!(ScheduleColumn;key:ScheduleKey,heading:Option<String>,total:bool);
entity!(ScheduleSort;key:ScheduleKey,descending:bool);
entity!(ScheduleFilter;key:ScheduleKey,op:ScheduleOp,value:String);
entity!(ScheduleGroup;key:ScheduleKey);
entity!(Schedule;name:String,category:ScheduleCategory,itemize:bool;columns:Vec<ScheduleColumn> =>write_entities("bim_schedule_column"),read_entities("bim_schedule_column"),sort:Vec<ScheduleSort> =>write_entities("bim_schedule_sort"),read_entities("bim_schedule_sort"),filter:Vec<ScheduleFilter> =>write_entities("bim_schedule_filter"),read_entities("bim_schedule_filter"),group:Vec<ScheduleGroup> =>write_entities("bim_schedule_group"),read_entities("bim_schedule_group"),storeys:Vec<String> =>write_list("bim_schedule_storey"),read_list("bim_schedule_storey"),phases:Vec<Phase> =>write_list("bim_schedule_phase"),read_list("bim_schedule_phase"));
entity!(PropertyTemplate;name:String;applies_to:Vec<TemplateTarget> =>write_list("bim_template_target"),read_list("bim_template_target"),properties:Vec<PropertyDef> =>write_entities("bim_property_def"),read_entities("bim_property_def"));
entity!(PropertyDef;name:String,kind:PropertyKind,unit:Option<String>,description:Option<String>,required:bool,minimum:Option<f64>,maximum:Option<f64>;default_value:Option<PropertyValue> =>write_default(()),read_default(()),allowed:Vec<PropertyValue> =>write_list("bim_property_allowed"),read_list("bim_property_allowed"));
entity!(ClassificationSystem;name:String,edition:String,source:Option<String>;entries:Vec<ClassificationItem> =>write_list("bim_classification_item"),read_list("bim_classification_item"));
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
fn write_map<T:Entity>(p:&mut RowWriter<'_,'_>,table:&str,parent:i64,values:&RetainedOrderedMap<String,T>)->Result<()>{for(index,(key,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(parent))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(key))?;value.fields(&mut c)?;let id=c.emit(p,table)?;value.children(p,id)?;}Ok(())}
fn read_map<T:Entity>(r:&mut Reader<'_,'_,'_>,table:&'static str,parent:i64)->Result<RetainedOrderedMap<String,T>>{let rows=r.take(table,parent,1,T::WIDTH+4,Some(2))?;let mut result=RetainedOrderedMap::new();let mut previous:Option<&str>=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM map keys are not strictly sorted"))}}previous=Some(key);r.native.charge(std::mem::size_of::<(String,T)>()+96)?;let key=r.native.copy_text(key)?;let value=T::read(r,row)?;if result.cold_insert(key,value).is_some(){return Err(invalid("BIM map key is repeated"))}}Ok(result)}
fn visit_rows(snapshot:&ModelSnapshot,p:&mut RowWriter<'_,'_>)->Result<()>{
 let root=p.insert("bim_document",&[Cell::Text(&snapshot.schema)])?;
 let project=p.insert("bim_project",&[Cell::Integer(root),Cell::Text(&snapshot.project.name),Cell::Text(&snapshot.project.description),Cell::Text(&snapshot.project.author),Cell::Text(&snapshot.project.organization)])?;
 list(p,"bim_phase_name",project,&snapshot.project.phase_names)?;
 write_map(p,"bim_support",root,&snapshot.supports)?;
 write_map(p,"bim_load_case",root,&snapshot.load_cases)?;
 write_map(p,"bim_structural_load",root,&snapshot.loads)?;
 write_map(p,"bim_option_group",root,&snapshot.option_groups)?;
 write_map(p,"bim_design_option",root,&snapshot.design_options)?;
 write_map(p,"bim_workset",root,&snapshot.worksets)?;
 write_map(p,"bim_element_option",root,&snapshot.element_options)?;
 write_map(p,"bim_element_workset",root,&snapshot.element_worksets)?;
 write_map(p,"bim_space_conditions",root,&snapshot.space_conditions)?;
 write_map(p,"bim_clash_set",root,&snapshot.clash_sets)?;
 write_map(p,"bim_rule",root,&snapshot.rules)?;
 write_map(p,"bim_issue",root,&snapshot.issues)?;
 write_map(p,"bim_issue_comment",root,&snapshot.issue_comments)?;
 write_map(p,"bim_component",root,&snapshot.components)?;
 write_map(p,"bim_component_override",root,&snapshot.component_overrides)?;
 write_map(p,"bim_mep_element",root,&snapshot.mep_elements)?;
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
fn read_properties(r:&mut Reader<'_,'_,'_>,parent:i64)->Result<RetainedOrderedMap<String,PropertySet>>{
 let elements=r.take("bim_property_element",parent,1,4,Some(2))?;let mut result=RetainedOrderedMap::new();let mut previous=None;for element in elements{let key=element.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property element keys are not sorted"))}}previous=Some(key);let key=r.native.copy_text(key)?;r.native.charge(96+std::mem::size_of::<PropertySet>())?;let sets=r.take("bim_property_set",element.rowid,1,4,Some(2))?;let mut values=RetainedOrderedMap::new();let mut previous=None;for set in sets{let name=set.text(3)?;if let Some(previous)=previous{if compare_text(previous,name,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property set names are not sorted"))}}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<RetainedOrderedMap<String,PropertyValue>>())?;let properties=r.take("bim_property_value",set.rowid,1,11,Some(2))?;let mut contents=RetainedOrderedMap::new();let mut previous=None;for row in properties{let name=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,name,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM property names are not sorted"))}}previous=Some(name);let name=r.native.copy_text(name)?;r.native.charge(96+std::mem::size_of::<PropertyValue>())?;let mut index=4;contents.cold_insert(name,PropertyValue::read(row,&mut index,r.native)?);}values.cold_insert(name,contents);}result.cold_insert(key,values);}Ok(result)
}
fn write_maps<T:Scalar>(p:&mut RowWriter<'_,'_>,parent:i64,values:&RetainedOrderedMap<String,RetainedOrderedMap<String,T>>,tables:[&str;2])->Result<()>{for(index,(key,values))in values.iter().enumerate(){let owner=p.insert(tables[0],&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Text(key)])?;for(index,(key,value))in values.iter().enumerate(){let mut c=Cells::new();c.push(Cell::Integer(owner))?;c.push(Cell::Integer(ordinal(index)?))?;c.push(Cell::Text(key))?;value.append(&mut c)?;c.emit(p,tables[1])?;}}Ok(())}
fn read_scalar_map<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,table:&'static str)->Result<RetainedOrderedMap<String,T>>{let rows=r.take(table,parent,1,T::WIDTH+4,Some(2))?;let mut values=RetainedOrderedMap::new();let mut previous=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM scalar map keys are not sorted"))}}previous=Some(key);r.native.charge(96+std::mem::size_of::<(String,T)>())?;let key=r.native.copy_text(key)?;let mut index=4;values.cold_insert(key,T::read(row,&mut index,r.native)?);}Ok(values)}
fn read_maps<T:Scalar>(r:&mut Reader<'_,'_,'_>,parent:i64,tables:[&'static str;2])->Result<RetainedOrderedMap<String,RetainedOrderedMap<String,T>>>{let rows=r.take(tables[0],parent,1,4,Some(2))?;let mut values=RetainedOrderedMap::new();let mut previous=None;for row in rows{r.native.step()?;let key=row.text(3)?;if let Some(previous)=previous{if compare_text(previous,key,r.native)?!=std::cmp::Ordering::Less{return Err(invalid("BIM owned map keys are not sorted"))}}previous=Some(key);r.native.charge(96+std::mem::size_of::<(String,RetainedOrderedMap<String,T>)>())?;let key=r.native.copy_text(key)?;values.cold_insert(key,read_scalar_map(r,row.rowid,tables[1])?);}Ok(values)}
fn restore(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<ModelSnapshot>{
 semantic_cells::extent(c.limits())?;c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;store::sqlite_snapshot::validate_sqlite_database_schema(database,SCHEMA,c.limits())?;
 let maximum=c.limits().max_value_bytes;let mut progress=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,event.completed,event.total).is_ok();let mut native=NativeDecodeControl::new(maximum,&mut progress);native.charge(std::mem::size_of::<ModelSnapshot>())?;let mut r=Reader::new(database,&mut native);let document=r.singleton("bim_document",2)?;let root=document.rowid;let schema=r.native.copy_text(document.text(1)?)?;let row=r.required("bim_project",root,1,6)?;let mut i=2;let project=Project{name:String::read(row,&mut i,r.native)?,description:String::read(row,&mut i,r.native)?,author:String::read(row,&mut i,r.native)?,organization:String::read(row,&mut i,r.native)?,phase_names:r.list("bim_phase_name",row.rowid)?};
 let result=ModelSnapshot{schema,project,
 supports:read_map(&mut r,"bim_support",root)?,
 load_cases:read_map(&mut r,"bim_load_case",root)?,
 loads:read_map(&mut r,"bim_structural_load",root)?,
 option_groups:read_map(&mut r,"bim_option_group",root)?,
 design_options:read_map(&mut r,"bim_design_option",root)?,
 worksets:read_map(&mut r,"bim_workset",root)?,
 element_options:read_map(&mut r,"bim_element_option",root)?,
 element_worksets:read_map(&mut r,"bim_element_workset",root)?,
 space_conditions:read_map(&mut r,"bim_space_conditions",root)?,
 clash_sets:read_map(&mut r,"bim_clash_set",root)?,
 rules:read_map(&mut r,"bim_rule",root)?,
 issues:read_map(&mut r,"bim_issue",root)?,
 issue_comments:read_map(&mut r,"bim_issue_comment",root)?,
 components:read_map(&mut r,"bim_component",root)?,
 component_overrides:read_map(&mut r,"bim_component_override",root)?,
 mep_elements:read_map(&mut r,"bim_mep_element",root)?,

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
impl store::os_store::serialization::ArtifactSqliteSnapshotHost for ModelSnapshot{
 
 fn preflight_sqlite_snapshot_encoding(&self,_encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<()>{admit(self,c)}
 
 
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<store::io_schema::IoPayload>>)->store::os_store::serialization::SnapshotReceivingOutcome {if let Err(error)=admit(self,c){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(), &mut |native|self.__dsl_to_record_controlled(native),c,native_owner,output)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome {let limits=c.limits();if let Err(error)=semantic_cells::extent(limits){return semio_framework_value::RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(error)),work:0}}store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(), &mut |record,snapshot_output,native| { let constructed: Result<_> = (|| {semantic_cells::admit_record(record,limits,native)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },c,native_control,output)}
}
impl pack::snapshot::ArtifactSqliteSnapshot for ModelSnapshot {
const SQLITE_SCHEMA:&'static str=SCHEMA;
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{project(self,c)}
fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self>{restore(database,c)}
}
