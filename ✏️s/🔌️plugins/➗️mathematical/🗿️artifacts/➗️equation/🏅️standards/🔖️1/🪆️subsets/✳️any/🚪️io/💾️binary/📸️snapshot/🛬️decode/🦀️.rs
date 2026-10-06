//! 🛬️ Bounded literal Equation record binding and iterative owned expression construction.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::*;
use semio_framework_value::NativeDecodeControl;

struct Nodes(Vec<EquationNode>);
impl Drop for Nodes{fn drop(&mut self){for node in self.0.drain(..){retire_node(node);}}}
fn references(value:&semio_framework_dsl_record::RecordValue,field:u16)->Result<usize,ValueError>{match value.fields.get(&field){Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values.len()),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical reference list differs"))}}
fn record(value:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical record differs"))}}
fn preflight(source:&semio_framework_dsl_record::RecordValue,control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<(),ValueError>{
 let expression=record(source.fields.get(&3).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation expression missing"))?)?;
 let nodes=match expression.fields.get(&1){Some(semio_framework_dsl_record::FieldValue::List(nodes))=>nodes,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical node list differs"))};
 let graph=record(source.fields.get(&4).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation graph missing"))?)?;
 let geometry=record(source.fields.get(&5).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation geometry missing"))?)?;
 let state_rows=[references(graph,1)?,references(graph,2)?,references(geometry,0)?].into_iter().try_fold(1usize,usize::checked_add).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Equation row count overflow"))?;
 let mut rows=nodes.len().checked_mul(2).and_then(|rows|rows.checked_add(5)).and_then(|rows|rows.checked_add(state_rows)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Equation row count overflow"))?;
 if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Equation logical row limit exceeded"))}
 control.begin_stage(nodes.len())?;
 for node in nodes{let node=record(node)?;rows=rows.checked_add(references(node,6)?).and_then(|rows|references(node,7).ok().and_then(|count|rows.checked_add(count))).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Equation row count overflow or references differ"))?;if rows>maximum_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Equation logical row limit exceeded"))}control.step()?;}
 Ok(())
}
fn child_controlled(values:&mut[Option<EquationNode>],parent:usize,key:u64,control:&mut NativeDecodeControl<'_>)->Result<EquationNode,ValueError>{control.step()?;owned_child(values,parent,key)}
fn group_controlled(values:&mut[Option<EquationNode>],parent:usize,keys:Vec<u64>,control:&mut NativeDecodeControl<'_>)->Result<Vec<EquationNode>,ValueError>{
 let mut result=Nodes(control.allocate_vec(keys.len())?);
 for key in keys{result.0.push(child_controlled(values,parent,key,control)?);}
 Ok(std::mem::take(&mut result.0))
}
pub(super) fn reconstruct(source:&semio_framework_dsl_record::RecordValue,control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<EquationSnapshot,ValueError>{
 control.scoped_stage(|control|preflight(source,control,maximum_rows))?;
 let value=EquationPackRecord::__dsl_from_record_controlled(source,control)?;
 let EquationPackRecord{notation,results,computed,equation,graph,geometry}=value;let Expression{next_label,nodes}=equation;
 if nodes.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical root missing"))}
 let mut total=nodes.len();
 for node in &nodes{total=total.checked_add(node.terms.len()).and_then(|value|value.checked_add(node.factors.len())).and_then(|value|value.checked_add(usize::from(node.base.is_some()))).and_then(|value|value.checked_add(usize::from(node.exponent.is_some()))).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Equation construction workload overflow"))?;}
 control.begin_stage(total)?;
 let mut values=Values(control.allocate_vec(nodes.len())?);values.0.resize_with(nodes.len(),||None);
 for(index,node)in nodes.into_iter().enumerate().rev(){
  control.step()?;let Node{label,kind,lexeme,numer,denom,name,terms,factors,base,exponent}=node;
  let kind=match(kind,lexeme,numer,denom,name,terms.is_empty(),factors.is_empty(),base,exponent){
   (Kind::Integer,Some(lexeme),None,None,None,true,true,None,None)=>EquationNodeKind::Integer{lexeme},
   (Kind::Rational,None,Some(numer),Some(denom),None,true,true,None,None)=>EquationNodeKind::Rational{numer,denom},
   (Kind::Symbol,None,None,None,Some(name),true,true,None,None)=>EquationNodeKind::Symbol{name},
   (Kind::Add,None,None,None,None,_,true,None,None)=>EquationNodeKind::Add{terms:group_controlled(&mut values.0,index,terms,control)?},
   (Kind::Mul,None,None,None,None,true,_,None,None)=>EquationNodeKind::Mul{factors:group_controlled(&mut values.0,index,factors,control)?},
   (Kind::Pow,None,None,None,None,true,true,Some(base),Some(exponent))=>{
    control.charge(2*std::mem::size_of::<EquationNode>())?;
    let base=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(child_controlled(&mut values.0,index,base,control)?,retire_node);
    let exponent=child_controlled(&mut values.0,index,exponent,control)?;
    EquationNodeKind::Pow{base:Box::new(base.take()),exponent:Box::new(exponent)}
   },
   _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical variant contains unrelated fields"))
  };
  values.0[index]=Some(EquationNode{label:EquationNodeLabel(label),kind});
 }
 if values.0.iter().skip(1).any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical record contains unowned nodes"))}
 control.checkpoint()?;
 let expr=values.0[0].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical root missing"))?;
 let(id,target_ref)=target(notation);let notation=store::ArtifactChild::new(id,target_ref);
 let(id,target_ref)=target(results);let results=store::ArtifactChild::new(id,target_ref);
 let(id,target_ref)=target(computed);let computed=store::ArtifactChild::new(id,target_ref);
 Ok(EquationSnapshot{graph,geometry,notation,results,computed,equation:EquationExprSnapshot{expr,next_label}})
}
