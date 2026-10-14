//! 📐️ Borrowed mesh vertex and stepped rational BRep curve evaluation preserve actual input geometry.
use super::*;
use semio_s_artifact_procedural_3d_geometry_value::SelectionKind;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_framework_3d_brep_engine::{ShapeRoot,ShapeValue};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d::mesh::{VertexId,EdgeId};

/// 🪢️ One original rational de Boor frontier; backing is reserved and copied separately.
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct CurveEvaluation{phase:u8,validated:usize,low:usize,high:usize,span:usize,degree:usize,initialized:usize,round:usize,blend:usize,scratch:Vec<[f64;4]>,done:usize,total:usize}
impl CurveEvaluation{
    pub(super) fn scratch_pending(&self)->bool{self.phase==3}
    pub(super) fn scratch_copy_pending(&self)->bool{matches!(self.phase,4|5)}
    pub fn progress(&self)->f32{if self.phase==7{return 1.0}if self.total==0{return 0.0}(self.done as f32/self.total as f32).min(0.99)}
    /// ⏱️ Reads or produces one genuine control, knot, span, backing or homogeneous blend event.
    pub fn step(&mut self,inputs:&WidgetInputs,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Option<GeometryValue>,RetainedCloneProgress),ValueError>{let result=self.advance(inputs,grant)?;if result.1.copied_items>0{self.done=self.done.checked_add(1).ok_or_else(||invalid("original rational curve progress overflow").with_retained_progress(result.1))?}Ok(result)}
    fn advance(&mut self,inputs:&WidgetInputs,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Option<GeometryValue>,RetainedCloneProgress),ValueError>{
        let source=match inputs.get("curve"){Some(GeometryValue::Shape(shape))=>shape,_=>return Err(invalid("original curve point requires its actual curve shape"))};
        let parameter=number(inputs,"parameter")?;let(curve,t)=curve_at(source,parameter)?;
        let one=RetainedCloneProgress{copied_items:1,..Default::default()};
        let Curve3::Nurbs{knots,controls,weights}=curve else{
            if grant.maximum_copy_bytes<24{return Ok((None,Default::default()))}let point=curve.eval(t);return Ok((Some(GeometryValue::Point([point.x,point.y,point.z])),RetainedCloneProgress{copied_bytes:24,..one}))
        };
        let p=knots.degree;let n=controls.len();
        if n<=p||weights.len()!=n||n.checked_add(p).and_then(|v|v.checked_add(1))!=Some(knots.knots.len()){return Err(invalid("original rational curve has invalid knot/control/weight extent"))}
        match self.phase{
            0=>{self.total=knots.knots.len().checked_add(n).and_then(|v|v.checked_add(p.checked_mul(p.checked_add(1)?)?/2)).and_then(|v|v.checked_add(p)).and_then(|v|v.checked_add(usize::BITS as usize+10)).ok_or_else(||invalid("original rational curve work extent overflow"))?;self.degree=p;self.low=p;self.high=n;self.phase=1;Ok((None,one))}
            1=>{
                if self.validated<knots.knots.len(){let i=self.validated;let knot=knots.knots[i];if !knot.is_finite()||i>0&&knots.knots[i-1]>knot{return Err(invalid("original rational curve knots must be finite and ordered"))}self.validated+=1;return Ok((None,one))}
                let index=self.validated-knots.knots.len();if index<n{let point=controls[index];if !point.x.is_finite()||!point.y.is_finite()||!point.z.is_finite()||!weights[index].is_finite(){return Err(invalid("original rational curve control and weight must be finite"))}self.validated+=1;return Ok((None,one))}
                self.phase=2;Ok((None,one))
            }
            2=>{
                if t>=knots.knots[n]{self.span=n-1;self.phase=3;}
                else if t<=knots.knots[p]{self.span=p;self.phase=3;}
                else if self.high-self.low>1{let mid=self.low+(self.high-self.low)/2;if t<knots.knots[mid]{self.high=mid}else{self.low=mid}}
                else{self.span=self.low;self.phase=3;}
                Ok((None,one))
            }
            3=>{
                let count=p.checked_add(1).ok_or_else(||invalid("original rational curve degree overflow"))?;let bytes=count.checked_mul(size_of::<[f64;4]>()).ok_or_else(||invalid("original rational scratch extent overflow"))?;
                if grant.maximum_capacity_bytes<bytes{return Ok((None,Default::default()))}
                self.scratch.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original rational scratch backing reserve failed"))?;self.phase=4;
                let receipt=RetainedCloneProgress{retained_capacity_bytes:self.scratch.capacity()*size_of::<[f64;4]>(),..one};if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original rational scratch allocator exceeded admitted backing").with_retained_progress(receipt))}Ok((None,receipt))
            }
            4=>{
                if self.initialized>p{self.round=1;self.blend=p;self.phase=5;return Ok((None,one))}
                if grant.maximum_copy_bytes<size_of::<[f64;4]>(){return Ok((None,Default::default()))}
                let index=self.span-p+self.initialized;let point=controls[index];let weight=weights[index];let atom=[point.x*weight,point.y*weight,point.z*weight,weight];if atom.iter().any(|v|!v.is_finite()){return Err(invalid("original rational curve control must be finite"))}self.scratch.push(atom);self.initialized+=1;Ok((None,RetainedCloneProgress{copied_bytes:size_of::<[f64;4]>(),..one}))
            }
            5=>{
                if self.round>p{self.phase=6;return Ok((None,one))}
                if grant.maximum_copy_bytes<size_of::<[f64;4]>(){return Ok((None,Default::default()))}
                let j=self.blend;let i=self.span-p+j;let denominator=knots.knots[i+p-self.round+1]-knots.knots[i];let alpha=if denominator==0.0{0.0}else{(t-knots.knots[i])/denominator};let left=self.scratch[j-1];let right=self.scratch[j];self.scratch[j]=std::array::from_fn(|axis|(1.0-alpha)*left[axis]+alpha*right[axis]);
                if j==self.round{self.round+=1;self.blend=p}else{self.blend-=1}Ok((None,RetainedCloneProgress{copied_bytes:size_of::<[f64;4]>(),..one}))
            }
            6=>{
                if grant.maximum_copy_bytes<24{return Ok((None,Default::default()))}
                let homogeneous=self.scratch[p];if homogeneous[3]==0.0{return Err(invalid("original rational curve point has zero homogeneous weight"))}let point=std::array::from_fn(|axis|homogeneous[axis]/homogeneous[3]);if point.iter().any(|v|!v.is_finite()){return Err(invalid("original rational curve point is not finite"))}self.phase=7;Ok((Some(GeometryValue::Point(point)),RetainedCloneProgress{copied_bytes:24,..one}))
            }
            _=>Ok((None,Default::default())),
        }
    }
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
pub(super) fn number(inputs:&WidgetInputs,port:&str)->Result<f64,ValueError>{match inputs.get(port){Some(GeometryValue::Number(value))=>Ok(*value),Some(GeometryValue::Integer(value))=>Ok(*value as f64),_=>Err(invalid("original geometric query parameter has the wrong type"))}}
fn curve_domain(curve:&Curve3)->Result<(f64,f64),ValueError>{
    if let Curve3::Nurbs{knots,controls,weights}=curve{let p=knots.degree;let n=controls.len();if n<=p||weights.len()!=n||n.checked_add(p).and_then(|v|v.checked_add(1))!=Some(knots.knots.len()){return Err(invalid("original rational curve has invalid source extent"))}let domain=(knots.knots[p],knots.knots[n]);if !domain.0.is_finite()||!domain.1.is_finite()||domain.0>domain.1{return Err(invalid("original rational curve domain is invalid"))}return Ok(domain)}Ok(curve.domain())
}
fn curve_at(shape:&ShapeValue,s:f64)->Result<(&Curve3,f64),ValueError>{
    if !s.is_finite(){return Err(invalid("original curve parameter must be finite"))}
    let slack=1e-9*(1.0+s.abs());
    match &shape.root{
        ShapeRoot::Curve{curve,..}=>{let domain=curve_domain(curve)?;within(domain,s,slack)?;Ok((curve,s.clamp(domain.0,domain.1)))}
        ShapeRoot::Edge(id)=>{let edge=shape.body.edges.get(*id).ok_or_else(||invalid("original curve edge is absent"))?;let curve=shape.body.curves3.get(edge.curve).ok_or_else(||invalid("original edge supporting curve is absent"))?;within(edge.range,s,slack)?;Ok((curve,s.clamp(edge.range.0,edge.range.1)))}
        ShapeRoot::Wire(wire)=>{
            let domain=(0.0,wire.members.len()as f64);if wire.members.is_empty(){return Err(invalid("original wire parameter source is empty"))}within(domain,s,slack)?;let s=s.clamp(domain.0,domain.1);let index=(s.ceil()as usize).saturating_sub(1).min(wire.members.len()-1);let(id,forward)=wire.members[index];let edge=shape.body.edges.get(id).ok_or_else(||invalid("original wire edge is absent"))?;let curve=shape.body.curves3.get(edge.curve).ok_or_else(||invalid("original wire supporting curve is absent"))?;let range=if forward{edge.range}else{(edge.range.1,edge.range.0)};Ok((curve,range.0+(s-index as f64)*(range.1-range.0)))
        }
        _=>Err(invalid("original curve point requires a curve edge or wire")),
    }
}
fn within(domain:(f64,f64),s:f64,slack:f64)->Result<(),ValueError>{if domain.0.is_nan()||domain.1.is_nan()||domain.0>domain.1||s<domain.0-slack||s>domain.1+slack{return Err(invalid("original curve parameter is outside its domain"))}Ok(())}
/// 🔍️ Reads the genuine selected mesh vertex without producing any cold selection roster.
pub fn mesh_vertex(inputs:&WidgetInputs)->Result<GeometryValue,ValueError>{
    let mesh=match inputs.get("mesh"){Some(GeometryValue::Mesh(mesh))=>mesh,_=>return Err(invalid("original mesh vertex query requires its actual mesh"))};
    let selection=match inputs.get("vertex"){Some(GeometryValue::Selection(selection))if selection.component==SelectionKind::Vertex&&selection.ids.len()==1=>selection,_=>return Err(invalid("original mesh vertex query requires one vertex selection"))};
    let index=u32::try_from(selection.ids[0]).map_err(|_|invalid("original mesh selected vertex exceeds its handle range"))?;let point=mesh.vertex_position(VertexId(index)).map_err(|_|invalid("original mesh selected vertex is absent"))?;Ok(GeometryValue::Point(point.0.map(f64::from)))
}

/// 📏️ Reads one genuine selected edge and its fixed two endpoint handles without a cold roster.
pub fn mesh_edge(inputs:&WidgetInputs)->Result<[GeometryValue;3],ValueError>{
 let mesh=match inputs.get("mesh"){Some(GeometryValue::Mesh(mesh))=>mesh,_=>return Err(invalid("original mesh edge query requires its actual mesh"))};let selection=match inputs.get("edge"){Some(GeometryValue::Selection(selection))if selection.component==SelectionKind::Edge&&selection.ids.len()==1=>selection,_=>return Err(invalid("original mesh edge query requires one edge selection"))};let index=u32::try_from(selection.ids[0]).map_err(|_|invalid("original mesh selected edge exceeds its handle range"))?;let(from,to)=mesh.edge_endpoints(EdgeId(index)).map_err(|_|invalid("original mesh selected edge is absent"))?;let start=mesh.vertex_position(from).map_err(|_|invalid("original mesh edge start vertex is absent"))?.0.map(f64::from);let end=mesh.vertex_position(to).map_err(|_|invalid("original mesh edge end vertex is absent"))?.0.map(f64::from);let delta:[f64;3]=std::array::from_fn(|axis|end[axis]-start[axis]);let length=delta[0].hypot(delta[1]).hypot(delta[2]);if start.iter().chain(end.iter()).any(|value|!value.is_finite())||!length.is_finite(){return Err(invalid("original mesh edge coordinates must be finite"))}Ok([GeometryValue::Point(start),GeometryValue::Point(end),GeometryValue::Number(length)])
}

/// 🏳️ Reads a fixed analytic surface atom after excluding every free-form and trimmed owner.
pub fn analytic_surface(inputs:&WidgetInputs,normal:bool)->Result<GeometryValue,ValueError>{
 let shape=match inputs.get("surface"){Some(GeometryValue::Shape(shape))=>shape,_=>return Err(invalid("original surface query requires its actual shape"))};let ShapeRoot::Surface{surface,..}=&shape.root else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original trimmed surface query has no funded producer"))};if matches!(surface,Surface::Nurbs{..}){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original rational surface query has no funded producer"))}let u=number(inputs,"u")?;let v=number(inputs,"v")?;if !u.is_finite()||!v.is_finite(){return Err(invalid("original surface parameters must be finite"))}let(ud,vd)=surface.domain();within(ud,u,1e-9)?;within(vd,v,1e-9)?;let u=u.clamp(ud.0,ud.1);let v=v.clamp(vd.0,vd.1);
 let result=if normal{let vector=surface.normal(u,v).ok_or_else(||invalid("original analytic surface normal is singular"))?;GeometryValue::Vector([vector.x,vector.y,vector.z])}else{let point=surface.eval(u,v);GeometryValue::Point([point.x,point.y,point.z])};let coordinates=match &result{GeometryValue::Point(coordinates)|GeometryValue::Vector(coordinates)=>coordinates,_=>unreachable!()};if coordinates.iter().any(|value|!value.is_finite()){return Err(invalid("original analytic surface output must be finite"))}Ok(result)
}
