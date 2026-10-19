//! 🎟️ Original fixed-output operators construct actual values and close inputs on the initiating caller.
use crate::WidgetJob;
use crate::WidgetStep;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::GeometryValue;
use semio_s_artifact_procedural_3d_geometry_value::Outputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetEvaluation;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_model::catalogue::Quality;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneProgress,ordered_map::{BoundedOrdGrant,RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapInsertProgress,RetainedOrderedMapInsertStep}},retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement}};
use semio_framework_3d_brep_engine::{ShapeValue,ShapeRoot};
use {semio_framework_3d_brep_curve::Curve3, semio_framework_3d_brep_surface::Surface, semio_framework_3d_brep_representation::topology::Body, semio_framework_3d_brep_vector::{Pnt3, Vec3, Frame3}};
use semio_framework_3d::mesh::{HalfedgeMesh,MeshPlaneProduction,MeshTranslationProduction};
use std::sync::Arc;
use std::mem::{ManuallyDrop,size_of};

#[path="📐️read/🦀️.rs"]
mod geometry_read;

type Insert=RetainedOrderedMapInsertCursor<String,GeometryValue>;
#[derive(Clone,Copy)]
struct Source{operation:u64,generation:u64,parent:usize,recipient:usize,clock:fn()->Option<u64>,clock_binding:Option<u64>,deadline:u64}
impl PartialEq for Source{fn eq(&self,other:&Self)->bool{self.operation==other.operation&&self.generation==other.generation&&self.parent==other.parent&&self.recipient==other.recipient&&std::ptr::fn_addr_eq(self.clock,other.clock)&&self.clock_binding==other.clock_binding&&self.deadline==other.deadline}}
impl Eq for Source{}
semio_framework_value::artifact_retire_leaf!(Source);
#[derive(Clone,Copy)]
enum ValueKind{Slider,Number,Integer,Boolean,Vector,Point,Plane,MeshVertex,MeshEdge,CurvePoint,MeshPlane,BrepLine,BrepCircle,BrepEllipse,BrepPlane,BrepTranslation,SurfacePoint,SurfaceNormal,MeshTranslation}
semio_framework_value::artifact_retire_leaf!(ValueKind);
#[derive(semio_framework_value::RetireOwned)]
struct State{source:Source,kind:ValueKind,quality:Quality,inputs:Option<ControlledRetirement<WidgetInputs>>,value:Option<GeometryValue>,extra:[Option<GeometryValue>;2],output:usize,key:String,outputs:Option<Outputs>,insertion:Option<Insert>,phase:u8,byte:usize,cancelled:bool,curve:Option<geometry_read::CurveEvaluation>,curve_close:Option<ControlledRetirement<geometry_read::CurveEvaluation>>,plane:Option<MeshPlaneProduction>,mesh:Option<HalfedgeMesh>,shape:Option<ShapeValue>,translation:Option<MeshTranslationProduction>}

/// 🧵️ Retains original inputs, output backing and insertion ownership through every denied turn.
pub struct OriginalFixedOutputJob{state:ManuallyDrop<State>,transferred:bool}
impl OriginalFixedOutputJob{
    /// 🏗️ Moves actual resolved inputs into an allocation-free operator frame.
    pub fn admit(kind:&Kind,inputs:&mut Option<WidgetInputs>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{Self::admit_with_number(kind,inputs,None,cx)}
    fn admit_with_number(kind:&Kind,inputs:&mut Option<WidgetInputs>,number:Option<f64>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
        let source=source(cx)?;
        if !open(cx)||number.is_some()&&cx.retained_grant().maximum_copy_bytes<8{return Ok(None)}
        if number.is_some_and(|number|!number.is_finite()||kind.id!="math.number"){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original host number policy requires a finite actual number scalar").into())}
        let value_kind=match kind.id.as_str(){"math.number"|"math.angle"|"math.length"=>ValueKind::Number,"math.integer"=>ValueKind::Integer,"math.boolean"=>ValueKind::Boolean,"math.vector"=>ValueKind::Vector,"math.point"=>ValueKind::Point,"math.plane"=>ValueKind::Plane,"brep.curve.line"=>ValueKind::BrepLine,"brep.curve.circle"=>ValueKind::BrepCircle,"brep.curve.ellipse"=>ValueKind::BrepEllipse,"brep.surface.plane"=>ValueKind::BrepPlane,"brep.transform.translate"=>ValueKind::BrepTranslation,"brep.evaluate.surfacePoint"=>ValueKind::SurfacePoint,"brep.evaluate.surfaceNormal"=>ValueKind::SurfaceNormal,"mesh.primitive.plane"=>ValueKind::MeshPlane,"mesh.transform.translate"=>ValueKind::MeshTranslation,"mesh.inspect.vertex"=>ValueKind::MeshVertex,"mesh.inspect.edge"=>ValueKind::MeshEdge,"brep.evaluate.curvePoint"=>ValueKind::CurvePoint,_=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"kind has no original fixed-output producer").into())};
        if inputs.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original fixed-output operator requires its actual resolved inputs").into())}
        post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:if number.is_some(){8}else{0},..Default::default()})?;
        let owner=match ControlledRetirement::new(inputs.take().unwrap()){Ok(owner)=>owner,Err((error,value))=>{*inputs=Some(value);return Err(error.into())}};
        Ok(Some(Self{state:ManuallyDrop::new(State{source,kind:value_kind,quality:kind.quality,inputs:Some(owner),value:number.map(GeometryValue::Number),extra:[None,None],output:0,key:String::new(),outputs:Some(Outputs::default()),insertion:None,phase:u8::from(number.is_some()),byte:0,cancelled:false,curve:number.is_none().then(geometry_read::CurveEvaluation::default),curve_close:None,plane:None,mesh:None,shape:None,translation:None}),transferred:false}))
    }
    /// 🔢️ An explicit host number policy donates its real scalar while preserving and closing actual resolved inputs.
    pub fn admit_number(kind:&Kind,inputs:&mut Option<WidgetInputs>,value:f64,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{Self::admit_with_number(kind,inputs,Some(value),cx)}
    /// 🎚️ Starts the actual source slider value directly without inventing a catalogue kind or inputs.
    pub fn admit_slider(value:f64,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
        let source=source(cx)?;if !open(cx)||cx.retained_grant().maximum_copy_bytes<8{return Ok(None)}if !value.is_finite(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original slider value must be finite").into())}post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()})?;
        Ok(Some(Self{state:ManuallyDrop::new(State{source,kind:ValueKind::Slider,quality:Quality::ExactAnalytic,inputs:None,value:Some(GeometryValue::Number(value)),extra:[None,None],output:0,key:String::new(),outputs:Some(Outputs::default()),insertion:None,phase:2,byte:0,cancelled:false,curve:None,curve_close:None,plane:None,mesh:None,shape:None,translation:None}),transferred:false}))
    }
    /// 📊️ Reads one fixed operator phase without scanning any payload.
    pub fn progress(&self)->f32{if self.state.phase==0{if let Some(translation)=self.state.translation.as_ref(){return 0.1*translation.progress()}if let Some(plane)=self.state.plane.as_ref(){return 0.1*plane.progress()}if matches!(self.state.kind,ValueKind::CurvePoint){return self.state.curve.as_ref().map_or(0.0,|curve|0.1*curve.progress())}return if self.state.shape.is_some(){0.1}else{0.0}}let phase=self.state.phase.min(7)as f32/7.0;if matches!(self.state.kind,ValueKind::MeshEdge){(self.state.output as f32+phase)/3.0}else{phase}}
    pub fn terminal_is_empty(&self)->bool{self.transferred||self.state.inputs.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)&&self.state.value.is_none()&&self.state.extra.iter().all(Option::is_none)&&self.state.key.capacity()==0&&self.state.outputs.is_none()&&self.state.insertion.is_none()&&self.state.curve.is_none()&&self.state.curve_close.is_none()&&self.state.plane.is_none()&&self.state.mesh.is_none()&&self.state.shape.is_none()&&self.state.translation.is_none()}
    /// 🪜️ Advances actual scalar evaluation, recursive input closure, UTF8 output and fixed-page publication.
    pub fn step_original(&mut self,cx:&mut StepContext<'_>)->Result<Option<WidgetStep>,OriginalTurnFailure>{
        if source(cx)?!=self.state.source{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fixed-output job requires its original initiating caller").into())}
        if self.state.phase==7||self.state.cancelled||!open(cx){return Ok(None)}
        let state=&mut*self.state;
        let output_name=match state.kind{ValueKind::Slider=>"number",ValueKind::MeshVertex=>"position",ValueKind::MeshEdge=>["start","end","length"][state.output],ValueKind::CurvePoint|ValueKind::SurfacePoint=>"point",ValueKind::SurfaceNormal=>"normal",ValueKind::MeshPlane|ValueKind::MeshTranslation=>"mesh",ValueKind::BrepLine|ValueKind::BrepCircle|ValueKind::BrepEllipse|ValueKind::BrepPlane|ValueKind::BrepTranslation=>"shape",_=>"value"};
        match state.phase{
            0=>{
                let inputs=state.inputs.as_ref().and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixed query input owner disappeared"))?;
                if matches!(state.kind,ValueKind::BrepLine|ValueKind::BrepCircle|ValueKind::BrepEllipse|ValueKind::BrepPlane|ValueKind::BrepTranslation){
                    if state.shape.is_some(){let bytes=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<ShapeValue>();if cx.retained_grant().maximum_capacity_bytes<bytes{return Ok(None)}state.value=Some(GeometryValue::Shape(Arc::new(state.shape.take().unwrap())));state.curve.take();state.phase=1;post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(None)}
                    if matches!(state.kind,ValueKind::BrepTranslation){let shape=match inputs.get("shape"){Some(GeometryValue::Shape(shape))=>shape,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original BRep translation requires its actual shape").into())};let offset=match inputs.get("offset"){Some(GeometryValue::Vector(offset))=>*offset,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original BRep translation requires its actual vector offset").into())};let(result,receipt)=shape.translate_analytic_original(offset,cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;state.shape=result;if receipt!=Default::default(){post(cx,receipt)?}return Ok(None)}
                    if matches!(state.kind,ValueKind::BrepPlane){let bytes=size_of::<Surface>();if cx.retained_grant().maximum_copy_bytes<bytes{return Ok(None)}let surface=analytic_plane(inputs)?;let mut body=Body::new();let label=body.labels.next_label();state.shape=Some(ShapeValue{root:ShapeRoot::Surface{surface,label},body});post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(None)}
                    if matches!(state.kind,ValueKind::BrepCircle|ValueKind::BrepEllipse){
                        let bytes=size_of::<Curve3>();if cx.retained_grant().maximum_copy_bytes<bytes{return Ok(None)}let curve=analytic_curve(state.kind,inputs)?;let mut body=Body::new();let label=body.labels.next_label();state.shape=Some(ShapeValue{root:ShapeRoot::Curve{curve,label},body});post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(None)
                    }
                    if cx.retained_grant().maximum_copy_bytes<48{return Ok(None)}
                    let point=|port|match inputs.get(port){Some(GeometryValue::Point(point))=>Ok(*point),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original BRep line requires actual point inputs"))};let start=point("start")?;let end=point("end")?;let delta:[f64;3]=std::array::from_fn(|axis|end[axis]-start[axis]);if start.iter().chain(end.iter()).chain(delta.iter()).any(|v|!v.is_finite())||delta[0].hypot(delta[1]).hypot(delta[2])<=1e-9{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original BRep line requires distinct finite endpoints").into())}
                    let mut body=Body::new();let label=body.labels.next_label();let curve=Curve3::Line{origin:Pnt3::new(start[0],start[1],start[2]),dir:semio_framework_3d_brep_vector::Vec3::new(delta[0],delta[1],delta[2])};post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:48,..Default::default()})?;state.shape=Some(ShapeValue{root:ShapeRoot::Curve{curve,label},body});return Ok(None)
                }
                if matches!(state.kind,ValueKind::MeshPlane){
                    if let Some(mesh)=state.mesh.as_ref(){
                        let bytes=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<HalfedgeMesh>();if cx.retained_grant().maximum_capacity_bytes<bytes{return Ok(None)}
                        let _=mesh;state.value=Some(GeometryValue::Mesh(Arc::new(state.mesh.take().unwrap())));state.plane.take();state.curve.take();state.phase=1;post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(None)
                    }
                    if state.plane.is_none(){
                        if cx.retained_grant().maximum_copy_bytes<8{return Ok(None)}
                        let width=geometry_read::number(inputs,"width")?as f32;let depth=geometry_read::number(inputs,"depth")?as f32;let plane=MeshPlaneProduction::new(width,depth)?;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()})?;state.plane=Some(plane);return Ok(None)
                    }
                    let(result,receipt)=state.plane.as_mut().unwrap().step(cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;if receipt!=Default::default(){post(cx,receipt)?}state.mesh=result;return Ok(None)
                }
                if matches!(state.kind,ValueKind::MeshTranslation){
                    if state.mesh.is_some(){let bytes=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<HalfedgeMesh>();if cx.retained_grant().maximum_capacity_bytes<bytes{return Ok(None)}state.value=Some(GeometryValue::Mesh(Arc::new(state.mesh.take().unwrap())));state.translation.take();state.curve.take();state.phase=1;post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?;return Ok(None)}
                    let original=match inputs.get("mesh"){Some(GeometryValue::Mesh(mesh))=>mesh,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translation requires its actual mesh input").into())};
                    if state.translation.is_none(){if cx.retained_grant().maximum_copy_bytes<24{return Ok(None)}let offset=match inputs.get("offset"){Some(GeometryValue::Vector(offset))=>offset.map(|value|value as f32),_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original mesh translation requires its actual vector offset").into())};let translation=MeshTranslationProduction::new(original,offset)?;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:24,..Default::default()})?;state.translation=Some(translation);return Ok(None)}
                    let(result,receipt)=state.translation.as_mut().unwrap().step(original,cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;if receipt!=Default::default(){post(cx,receipt)?}state.mesh=result;return Ok(None)
                }
                if matches!(state.kind,ValueKind::CurvePoint){
                    let(result,receipt)=state.curve.as_mut().unwrap().step(inputs,cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;
                    if receipt!=Default::default(){post(cx,receipt)?}
                    if let Some(value)=result{state.value=Some(value);state.curve_close=Some(ControlledRetirement::new(state.curve.take().unwrap()).ok().unwrap());state.phase=1;}return Ok(None)
                }
                if matches!(state.kind,ValueKind::MeshVertex){
                    if cx.retained_grant().maximum_copy_bytes<24{return Ok(None)}
                    let value=geometry_read::mesh_vertex(inputs)?;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:24,..Default::default()})?;state.value=Some(value);state.curve.take();state.phase=1;return Ok(None)
                }
                if matches!(state.kind,ValueKind::MeshEdge){
                    if cx.retained_grant().maximum_copy_bytes<56{return Ok(None)}let[start,end,length]=geometry_read::mesh_edge(inputs)?;state.value=Some(start);state.extra=[Some(end),Some(length)];state.curve.take();state.phase=1;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:56,..Default::default()})?;return Ok(None)
                }
                if matches!(state.kind,ValueKind::SurfacePoint|ValueKind::SurfaceNormal){if cx.retained_grant().maximum_copy_bytes<24{return Ok(None)}let value=geometry_read::analytic_surface(inputs,matches!(state.kind,ValueKind::SurfaceNormal))?;state.value=Some(value);state.curve.take();state.phase=1;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:24,..Default::default()})?;return Ok(None)}
                let bytes=match state.kind{ValueKind::Number|ValueKind::Integer=>8,ValueKind::Boolean=>1,ValueKind::Vector|ValueKind::Point=>24,ValueKind::Plane=>48,ValueKind::MeshVertex|ValueKind::MeshEdge|ValueKind::CurvePoint|ValueKind::MeshPlane|ValueKind::BrepLine|ValueKind::BrepCircle|ValueKind::BrepEllipse|ValueKind::BrepPlane|ValueKind::BrepTranslation|ValueKind::SurfacePoint|ValueKind::SurfaceNormal|ValueKind::MeshTranslation|ValueKind::Slider=>unreachable!()};
                if cx.retained_grant().maximum_copy_bytes<bytes{return Ok(None)}
                let value=state.inputs.as_ref().and_then(ControlledRetirement::original).and_then(|inputs|inputs.get("value")).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original math value port is absent"))?;
                let value=match(state.kind,value){(ValueKind::Number,GeometryValue::Number(value))=>GeometryValue::Number(*value),(ValueKind::Number,GeometryValue::Integer(value))=>GeometryValue::Number(*value as f64),(ValueKind::Integer,GeometryValue::Integer(value))=>GeometryValue::Integer(*value),(ValueKind::Boolean,GeometryValue::Boolean(value))=>GeometryValue::Boolean(*value),(ValueKind::Vector,GeometryValue::Vector(value))=>GeometryValue::Vector(*value),(ValueKind::Point,GeometryValue::Point(value))=>GeometryValue::Point(*value),(ValueKind::Plane,GeometryValue::Plane(value))=>GeometryValue::Plane(*value),_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original math value port has the wrong type").into())};
                post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;state.value=Some(value);state.curve.take();state.phase=1;
            }
            1=>{
                if let Some(owner)=state.curve_close.as_mut(){
                    let grant=cx.retained_grant();let demand=owner.selected_demand(grant.maximum_copy_bytes)?;
                    if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
                    let receipt=owner.step(grant).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)?;if owner.terminal_is_empty(){state.curve_close.take();}return Ok(None)
                }
                let owner=state.inputs.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value original input owner disappeared"))?;
                let grant=cx.retained_grant();let demand=owner.selected_demand(grant.maximum_copy_bytes)?;
                if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
                let receipt=owner.step(grant).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)?;
                if owner.terminal_is_empty(){state.inputs.take();state.phase=2;}
            }
            2=>{
                if cx.retained_grant().maximum_capacity_bytes<output_name.len(){return Ok(None)}
                state.key.try_reserve_exact(output_name.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"math value output key reserve failed"))?;
                state.phase=3;post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:state.key.capacity(),..Default::default()})?;
            }
            3=>{
                if state.byte==output_name.len(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=4;}
                else if cx.retained_grant().maximum_copy_bytes>0{state.key.push(output_name.as_bytes()[state.byte]as char);state.byte+=1;post(cx,RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;}
            }
            4=>{
                let grant=cx.retained_grant();if grant.maximum_copy_bytes<Insert::constructor_copy_bytes()||grant.maximum_capacity_bytes<Insert::constructor_capacity_bytes(){return Ok(None)}
                let map=state.outputs.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value output dictionary disappeared"))?;let key=std::mem::take(&mut state.key);let value=state.value.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value scalar disappeared"))?;
                match Insert::admit(map,key,value,grant){Ok((insertion,receipt))=>{state.insertion=Some(insertion);state.phase=5;post(cx,receipt)?;},Err((error,map,key,value))=>{state.outputs=Some(map);state.key=key;state.value=Some(value);return Err(OriginalTurnFailure::debit_original(error,cx))}}
            }
            5=>{
                let insertion=state.insertion.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value insertion disappeared"))?;
                if insertion.output_ready(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.outputs=insertion.take();insertion.begin_close();state.phase=6;}
                else{let grant=cx.retained_grant();let step=insertion.advance(RetainedOrderedMapInsertGrant{retirement:grant,comparison:BoundedOrdGrant{maximum_items:1,maximum_bytes:1},maximum_moved_items:grant.maximum_items,maximum_moved_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes}).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;let receipt=match step{RetainedOrderedMapInsertStep::Progress(receipt)|RetainedOrderedMapInsertStep::Complete{progress:receipt,..}=>receipt};post(cx,insert_receipt(receipt)?)?;}
            }
            6=>{
                let insertion=state.insertion.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value insertion closure disappeared"))?;
                if !insertion.terminal_is_empty(){let receipt=insertion.close_step(cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)?;return Ok(None)}
                state.insertion.take();post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;
                if matches!(state.kind,ValueKind::MeshEdge)&&state.output<2{state.value=state.extra[state.output].take();state.output+=1;state.byte=0;state.phase=2;return Ok(None)}state.phase=7;
                let outputs=state.outputs.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"math value completed output disappeared"))?;return Ok(Some(WidgetStep::Done(WidgetEvaluation::ok(outputs,state.quality))))
            }
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"math value original phase is invalid").into()),
        }
        Ok(None)
    }
}
fn source(cx:&mut StepContext<'_>)->Result<Source,OriginalTurnFailure>{Ok(Source{operation:cx.operation().0,generation:cx.generation().0,parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),clock:cx.clock_source(),clock_binding:cx.original_clock_binding(),deadline:cx.original_absolute_deadline_us()})}
fn analytic_curve(kind:ValueKind,inputs:&WidgetInputs)->Result<Curve3,ValueError>{
 let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"original analytic curve requires finite center normal and positive radii");
 let center=match inputs.get("center"){Some(GeometryValue::Point(point))if point.iter().all(|value|value.is_finite())=>*point,_=>return Err(invalid())};let normal=match inputs.get("normal"){Some(GeometryValue::Vector(vector))if vector.iter().all(|value|value.is_finite())=>*vector,_=>return Err(invalid())};let length=normal[0].hypot(normal[1]).hypot(normal[2]);if !length.is_finite()||length<=1e-12{return Err(invalid())}let normal=Vec3::new(normal[0]/length,normal[1]/length,normal[2]/length);let origin=Pnt3::new(center[0],center[1],center[2]);
 let positive=|port|{let radius=geometry_read::number(inputs,port)?;if !radius.is_finite()||radius<=0.0{return Err(invalid())}Ok(radius)};
 if matches!(kind,ValueKind::BrepCircle){return Ok(Curve3::Circle{frame:Frame3::from_normal(origin,normal).ok_or_else(invalid)?,radius:positive("radius")?})}
 let hint=if normal.x.abs()<0.9{Vec3::X}else{Vec3::Y};Ok(Curve3::Ellipse{frame:Frame3::from_x_z(origin,hint,normal).ok_or_else(invalid)?,major_radius:positive("semiMajor")?,minor_radius:positive("semiMinor")?})
}
fn analytic_plane(inputs:&WidgetInputs)->Result<Surface,ValueError>{
 let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"original BRep plane requires finite origin and nonzero normal");let plane=match inputs.get("plane"){Some(GeometryValue::Plane(plane))=>plane,_=>return Err(invalid())};if plane.origin.iter().chain(plane.normal.iter()).any(|value|!value.is_finite()){return Err(invalid())}let length=plane.normal[0].hypot(plane.normal[1]).hypot(plane.normal[2]);if !length.is_finite()||length<=1e-12{return Err(invalid())}let frame=Frame3::from_normal(Pnt3::new(plane.origin[0],plane.origin[1],plane.origin[2]),Vec3::new(plane.normal[0]/length,plane.normal[1]/length,plane.normal[2]/length)).ok_or_else(invalid)?;Ok(Surface::Plane{frame})
}
fn open(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();!cx.should_yield()&&!cx.is_cancelled()&&grant.maximum_items>0&&grant.maximum_depth>0}
fn post(cx:&mut StepContext<'_>,receipt:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{let result=cx.consume_retained(receipt);cx.consume_fuel(1);result.map_err(|error|error.with_retained_progress(receipt).into())}
fn insert_receipt(receipt:RetainedOrderedMapInsertProgress)->Result<RetainedCloneProgress,ValueError>{receipt.retirement.checked_add(RetainedCloneProgress{copied_items:receipt.moved_items,copied_bytes:receipt.moved_bytes,retained_capacity_bytes:receipt.retained_capacity_bytes,..Default::default()})}
impl WidgetJob for OriginalFixedOutputJob{fn step(&mut self,_fuel:usize)->WidgetStep{WidgetStep::Working{progress:self.progress()}}fn cancel(&mut self){self.state.cancelled=true}fn step_on_original(&mut self,cx:&mut StepContext<'_>)->Result<Option<WidgetStep>,OriginalTurnFailure>{self.step_original(cx)}}
impl RetireOwned for OriginalFixedOutputJob{fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl Drop for OriginalFixedOutputJob{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"math value job reached Drop before original paid output or retirement closure");if self.terminal_is_empty()&&!self.transferred{unsafe{ManuallyDrop::drop(&mut self.state)}}}}

#[cfg(any(test, feature = "testkit"))]
#[path="../../🔢️math-values/🧪️tests/🎟️original/🦀️.rs"]
pub mod tests;
