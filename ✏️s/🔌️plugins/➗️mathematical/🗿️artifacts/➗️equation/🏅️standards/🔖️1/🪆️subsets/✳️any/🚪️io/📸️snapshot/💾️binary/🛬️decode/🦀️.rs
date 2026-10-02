//! 🛬️ Bounded literal Equation record binding and iterative owned expression construction.
use super::*;
use protocol::native_decoding::NativeDecodeControl;

struct Nodes(Vec<EquationNode>);
impl Drop for Nodes{fn drop(&mut self){for node in self.0.drain(..){retire_node(node);}}}
fn references(value:&dsl::RecordValue,field:u16)->Result<usize,String>{match value.fields.get(&field){Some(dsl::FieldValue::List(values))=>Ok(values.len()),_=>Err("Equation logical reference list differs".into())}}
fn record(value:&dsl::FieldValue)->Result<&dsl::RecordValue,String>{match value{dsl::FieldValue::Record(value)=>Ok(value),_=>Err("Equation logical record differs".into())}}
fn preflight(source:&dsl::RecordValue,control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<(),String>{
 let expression=record(source.fields.get(&3).ok_or("Equation expression missing")?)?;
 let nodes=match expression.fields.get(&1){Some(dsl::FieldValue::List(nodes))=>nodes,_=>return Err("Equation logical node list differs".into())};
 let mut rows=nodes.len().checked_mul(2).and_then(|rows|rows.checked_add(5)).ok_or("Equation row count overflow")?;
 if rows>maximum_rows{return Err("Equation logical row limit exceeded".into())}
 control.begin_stage(nodes.len())?;
 for node in nodes{let node=record(node)?;rows=rows.checked_add(references(node,6)?).and_then(|rows|references(node,7).ok().and_then(|count|rows.checked_add(count))).ok_or("Equation row count overflow or references differ")?;if rows>maximum_rows{return Err("Equation logical row limit exceeded".into())}control.step()?;}
 Ok(())
}
fn child_controlled(values:&mut[Option<EquationNode>],parent:usize,key:u64,control:&mut NativeDecodeControl<'_>)->Result<EquationNode,String>{control.step()?;owned_child(values,parent,key)}
fn group_controlled(values:&mut[Option<EquationNode>],parent:usize,keys:Vec<u64>,control:&mut NativeDecodeControl<'_>)->Result<Vec<EquationNode>,String>{
 let mut result=Nodes(control.allocate_vec(keys.len())?);
 for key in keys{result.0.push(child_controlled(values,parent,key,control)?);}
 Ok(std::mem::take(&mut result.0))
}
pub(super) fn reconstruct(source:&dsl::RecordValue,control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<EquationSnapshot,String>{
 preflight(source,control,maximum_rows)?;
 let value=EquationPackRecord::__dsl_from_record_controlled(source,control).map_err(|error|error.to_string())?;
 let EquationPackRecord{notation,results,computed,equation}=value;let Expression{next_label,nodes}=equation;
 if nodes.is_empty(){return Err("Equation logical root missing".into())}
 let mut total=nodes.len();
 for node in &nodes{total=total.checked_add(node.terms.len()).and_then(|value|value.checked_add(node.factors.len())).and_then(|value|value.checked_add(usize::from(node.base.is_some()))).and_then(|value|value.checked_add(usize::from(node.exponent.is_some()))).ok_or("Equation construction workload overflow")?;}
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
    let base=dsl::__rt::DecodedFieldOwner::new(child_controlled(&mut values.0,index,base,control)?,retire_node);
    let exponent=child_controlled(&mut values.0,index,exponent,control)?;
    EquationNodeKind::Pow{base:Box::new(base.take()),exponent:Box::new(exponent)}
   },
   _=>return Err("Equation logical variant contains unrelated fields".into())
  };
  values.0[index]=Some(EquationNode{label:EquationNodeLabel(label),kind});
 }
 if values.0.iter().skip(1).any(Option::is_some){return Err("Equation logical record contains unowned nodes".into())}
 control.checkpoint()?;
 let expr=values.0[0].take().ok_or("Equation logical root missing")?;
 let(id,target_ref)=target(notation);let notation=store::ArtifactChild::new(id,target_ref);
 let(id,target_ref)=target(results);let results=store::ArtifactChild::new(id,target_ref);
 let(id,target_ref)=target(computed);let computed=store::ArtifactChild::new(id,target_ref);
 Ok(EquationSnapshot{notation,results,computed,equation:EquationExprSnapshot{expr,next_label}})
}
