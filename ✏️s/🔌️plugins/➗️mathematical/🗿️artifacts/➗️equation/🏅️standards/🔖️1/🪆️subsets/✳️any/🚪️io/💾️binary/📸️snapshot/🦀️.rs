//! 📦️ Equation artifact — binary document surface + laws (constitutional: pack). The
//! `store::ArtifactPack` impl for `EquationSnapshot` encodes the derived `EquationPackRecord` spec.

use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::EquationSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::snapshot::{EquationNode,EquationNodeLabel,EquationNodeKind,EquationExprSnapshot};
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Child{child_id:String,artifact_id:String,artifact_kind:String,standard:String,subset:String}
#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum Kind{Integer,Rational,Symbol,Add,Mul,Pow}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Node{label:u64,kind:Kind,lexeme:Option<String>,numer:Option<String>,denom:Option<String>,name:Option<String>,terms:Vec<u64>,factors:Vec<u64>,base:Option<u64>,exponent:Option<u64>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct Expression{next_label:u64,nodes:Vec<Node>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension="equation")]
struct EquationPackRecord{notation:Child,results:Child,computed:Child,equation:Expression,graph:crate::EquationGraph,geometry:crate::EquationGeometry}
fn child(child_id:&str,target:&semio_framework_artifact_reference::ArtifactRef)->Child{Child{child_id:child_id.into(),artifact_id:target.artifact_id.clone(),artifact_kind:target.dialect.artifact_kind.clone(),standard:target.dialect.standard.clone(),subset:target.dialect.subset.clone()}}
fn target(value:Child)->(String,semio_framework_artifact_reference::ArtifactRef){(value.child_id,semio_framework_artifact_reference::ArtifactRef{artifact_id:value.artifact_id,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value.artifact_kind,standard:value.standard,subset:value.subset}})}
fn retire_node(node:EquationNode){let mut pending=vec![node];while let Some(node)=pending.pop(){match node.kind{EquationNodeKind::Add{terms}=>pending.extend(terms),EquationNodeKind::Mul{factors}=>pending.extend(factors),EquationNodeKind::Pow{base,exponent}=>{pending.push(*base);pending.push(*exponent);},_=>{}}}}
struct Values(Vec<Option<EquationNode>>);
impl Drop for Values{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){retire_node(value);}}}
fn owned_child(values:&mut[Option<EquationNode>],parent:usize,key:u64)->Result<EquationNode,ValueError>{let key=usize::try_from(key).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Equation child index exceeds native domain"))?;if key<=parent||key>=values.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation child topology differs"))}values[key].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation child has repeated owner"))}
fn group(values:&mut[Option<EquationNode>],parent:usize,keys:Vec<u64>)->Result<Vec<EquationNode>,ValueError>{let mut result=Values(Vec::with_capacity(keys.len()));for key in keys{result.0.push(Some(owned_child(values,parent,key)?));}Ok(result.0.iter_mut().filter_map(Option::take).collect())}
impl EquationPackRecord{
 fn from_snapshot(snapshot:&EquationSnapshot)->Self{let mut pending=std::collections::VecDeque::from([&snapshot.equation.expr]);let mut nodes=Vec::new();let mut next=1u64;while let Some(value)=pending.pop_front(){let mut node=Node{label:value.label.0,kind:Kind::Integer,lexeme:None,numer:None,denom:None,name:None,terms:vec![],factors:vec![],base:None,exponent:None};match&value.kind{EquationNodeKind::Integer{lexeme}=>node.lexeme=Some(lexeme.clone()),EquationNodeKind::Rational{numer,denom}=>{node.kind=Kind::Rational;node.numer=Some(numer.clone());node.denom=Some(denom.clone());},EquationNodeKind::Symbol{name}=>{node.kind=Kind::Symbol;node.name=Some(name.clone());},EquationNodeKind::Add{terms}=>{node.kind=Kind::Add;for child in terms{node.terms.push(next);next+=1;pending.push_back(child);}},EquationNodeKind::Mul{factors}=>{node.kind=Kind::Mul;for child in factors{node.factors.push(next);next+=1;pending.push_back(child);}},EquationNodeKind::Pow{base,exponent}=>{node.kind=Kind::Pow;node.base=Some(next);node.exponent=Some(next+1);next+=2;pending.push_back(base);pending.push_back(exponent);}}nodes.push(node);}Self{notation:child(&snapshot.notation.child_id,&snapshot.notation.target),results:child(&snapshot.results.child_id,&snapshot.results.target),computed:child(&snapshot.computed.child_id,&snapshot.computed.target),equation:Expression{next_label:snapshot.equation.next_label,nodes},graph:snapshot.graph.clone(),geometry:snapshot.geometry.clone()}}
 fn into_snapshot(self)->Result<EquationSnapshot,ValueError>{let Self{notation,results,computed,equation,graph,geometry}=self;let Expression{next_label,nodes}=equation;if nodes.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical root missing"))}let mut values=Values((0..nodes.len()).map(|_|None).collect());for(index,node)in nodes.into_iter().enumerate().rev(){let Node{label,kind,lexeme,numer,denom,name,terms,factors,base,exponent}=node;let kind=match(kind,lexeme,numer,denom,name,terms.is_empty(),factors.is_empty(),base,exponent){(Kind::Integer,Some(lexeme),None,None,None,true,true,None,None)=>EquationNodeKind::Integer{lexeme},(Kind::Rational,None,Some(numer),Some(denom),None,true,true,None,None)=>EquationNodeKind::Rational{numer,denom},(Kind::Symbol,None,None,None,Some(name),true,true,None,None)=>EquationNodeKind::Symbol{name},(Kind::Add,None,None,None,None,_,true,None,None)=>EquationNodeKind::Add{terms:group(&mut values.0,index,terms)?},(Kind::Mul,None,None,None,None,true,_,None,None)=>EquationNodeKind::Mul{factors:group(&mut values.0,index,factors)?},(Kind::Pow,None,None,None,None,true,true,Some(base),Some(exponent))=>{let mut children=Values(vec![Some(owned_child(&mut values.0,index,base)?)]);let exponent=owned_child(&mut values.0,index,exponent)?;EquationNodeKind::Pow{base:Box::new(children.0[0].take().unwrap()),exponent:Box::new(exponent)}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical variant contains unrelated fields"))};values.0[index]=Some(EquationNode{label:EquationNodeLabel(label),kind});}if values.0.iter().skip(1).any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical record contains unowned nodes"))}let expr=values.0[0].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Equation logical root missing"))?;let(id,target_ref)=target(notation);let notation=store::ArtifactChild::new(id,target_ref);let(id,target_ref)=target(results);let results=store::ArtifactChild::new(id,target_ref);let(id,target_ref)=target(computed);let computed=store::ArtifactChild::new(id,target_ref);Ok(EquationSnapshot{graph,geometry,notation,results,computed,equation:EquationExprSnapshot{expr,next_label}})}
}
#[path="🛬️decode/🦀️.rs"]
mod controlled;
#[path="🛫️encode/🦀️.rs"]mod controlled_output;
pub(crate) fn decode_sqlite_native(payload:&store::os_io::IoPayload,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<EquationSnapshot,ValueError>{let maximum_rows=control.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,"mathematical.equation",EquationPackRecord::__dsl_spec_producer(),|record,native|controlled::reconstruct(record,native,maximum_rows),control)}
pub(crate) fn encode_sqlite_native(snapshot:&EquationSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{let maximum_rows=control.limits().max_rows;store::encode_sqlite_snapshot_record_native(encoding,"mathematical.equation",EquationPackRecord::__dsl_spec_producer(),|native|controlled_output::project(snapshot,maximum_rows,native),control)}
/// 🛬️ Bind the explicit owned fields and admit reverse construction before every allocation.
pub(crate) fn reconstruct_pack_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>,maximum_rows:usize)->Result<EquationSnapshot,ValueError>{controlled::reconstruct(record,control,maximum_rows)}
/// 🖨️ Print the literal owned equation records without recursively lowering the expression.
pub(crate) fn print_pack_record_text(snapshot:&EquationSnapshot)->String{semio_framework_dsl_record::print(&EquationPackRecord::from_snapshot(snapshot).__dsl_to_record(),&EquationPackRecord::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document)}
/// 📖️ Parse every owned field and refuse trailing unrelated native text.
pub(crate) fn parse_pack_record_text(body:&str)->Result<EquationSnapshot,semio_framework_diagnostic::TextError>{let record=semio_framework_dsl_record::parse_exact(body,&EquationPackRecord::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;EquationPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))}
impl store::ArtifactPack for EquationSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,PackError>{let inner=store::pack_rt::encode_document(&EquationPackRecord::__dsl_spec(),&EquationPackRecord::from_snapshot(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e| PackError::from(e.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Equation logical native identity differs")))}let(record,_)=store::pack_rt::decode_document(&inner,&EquationPackRecord::__dsl_spec(),options)?;EquationPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(|error|PackError::from(error))}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(EquationPackRecord::__dsl_spec())}
}
/// 📦️ Encode the owned logical Equation record.
pub fn encode(snapshot:&EquationSnapshot)->Vec<u8>{store::ArtifactPack::encode_pack(snapshot)}
/// 📖️ Decode the owned logical Equation record.
pub fn decode(bytes:&[u8])->Result<EquationSnapshot,PackError>{<EquationSnapshot as store::ArtifactPack>::decode_pack(bytes)}

//#region 🔖️Store














//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
