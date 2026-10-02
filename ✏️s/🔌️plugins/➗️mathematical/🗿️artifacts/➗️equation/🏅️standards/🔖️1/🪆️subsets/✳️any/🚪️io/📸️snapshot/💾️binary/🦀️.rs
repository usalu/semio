//! 📦️ Equation artifact — binary document surface + laws (constitutional: pack). The
//! `store::ArtifactPack` impl for `EquationSnapshot` encodes the derived `EquationPackRecord` spec.

use crate::EquationSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::snapshot::{EquationNode,EquationNodeLabel,EquationNodeKind,EquationExprSnapshot};
#[derive(dsl::DslRecord)]
struct Child{child_id:String,artifact_id:String,artifact_kind:String,standard:String,subset:String}
#[derive(dsl::DslScalar)]
enum Kind{Integer,Rational,Symbol,Add,Mul,Pow}
#[derive(dsl::DslRecord)]
struct Node{label:u64,kind:Kind,lexeme:Option<String>,numer:Option<String>,denom:Option<String>,name:Option<String>,terms:Vec<u64>,factors:Vec<u64>,base:Option<u64>,exponent:Option<u64>}
#[derive(dsl::DslRecord)]
struct Expression{next_label:u64,nodes:Vec<Node>}
#[derive(dsl::DslRecord)]
#[dsl(extension="equation")]
struct EquationPackRecord{notation:Child,results:Child,computed:Child,equation:Expression}
fn child(child_id:&str,target:&store::os_io::ArtifactRef)->Child{Child{child_id:child_id.into(),artifact_id:target.artifact_id.clone(),artifact_kind:target.dialect.artifact_kind.clone(),standard:target.dialect.standard.clone(),subset:target.dialect.subset.clone()}}
fn target(value:Child)->(String,store::os_io::ArtifactRef){(value.child_id,store::os_io::ArtifactRef{artifact_id:value.artifact_id,dialect:store::os_io::ArtifactDialect{artifact_kind:value.artifact_kind,standard:value.standard,subset:value.subset}})}
fn retire_node(node:EquationNode){let mut pending=vec![node];while let Some(node)=pending.pop(){match node.kind{EquationNodeKind::Add{terms}=>pending.extend(terms),EquationNodeKind::Mul{factors}=>pending.extend(factors),EquationNodeKind::Pow{base,exponent}=>{pending.push(*base);pending.push(*exponent);},_=>{}}}}
struct Values(Vec<Option<EquationNode>>);
impl Drop for Values{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){retire_node(value);}}}
fn owned_child(values:&mut[Option<EquationNode>],parent:usize,key:u64)->Result<EquationNode,String>{let key=usize::try_from(key).map_err(|_|"Equation child index exceeds native domain")?;if key<=parent||key>=values.len(){return Err("Equation child topology differs".into())}values[key].take().ok_or_else(||"Equation child has repeated owner".into())}
fn group(values:&mut[Option<EquationNode>],parent:usize,keys:Vec<u64>)->Result<Vec<EquationNode>,String>{let mut result=Values(Vec::with_capacity(keys.len()));for key in keys{result.0.push(Some(owned_child(values,parent,key)?));}Ok(result.0.iter_mut().filter_map(Option::take).collect())}
impl EquationPackRecord{
 fn from_snapshot(snapshot:&EquationSnapshot)->Self{let mut pending=std::collections::VecDeque::from([&snapshot.equation.expr]);let mut nodes=Vec::new();let mut next=1u64;while let Some(value)=pending.pop_front(){let mut node=Node{label:value.label.0,kind:Kind::Integer,lexeme:None,numer:None,denom:None,name:None,terms:vec![],factors:vec![],base:None,exponent:None};match&value.kind{EquationNodeKind::Integer{lexeme}=>node.lexeme=Some(lexeme.clone()),EquationNodeKind::Rational{numer,denom}=>{node.kind=Kind::Rational;node.numer=Some(numer.clone());node.denom=Some(denom.clone());},EquationNodeKind::Symbol{name}=>{node.kind=Kind::Symbol;node.name=Some(name.clone());},EquationNodeKind::Add{terms}=>{node.kind=Kind::Add;for child in terms{node.terms.push(next);next+=1;pending.push_back(child);}},EquationNodeKind::Mul{factors}=>{node.kind=Kind::Mul;for child in factors{node.factors.push(next);next+=1;pending.push_back(child);}},EquationNodeKind::Pow{base,exponent}=>{node.kind=Kind::Pow;node.base=Some(next);node.exponent=Some(next+1);next+=2;pending.push_back(base);pending.push_back(exponent);}}nodes.push(node);}Self{notation:child(&snapshot.notation.child_id,&snapshot.notation.target),results:child(&snapshot.results.child_id,&snapshot.results.target),computed:child(&snapshot.computed.child_id,&snapshot.computed.target),equation:Expression{next_label:snapshot.equation.next_label,nodes}}}
 fn into_snapshot(self)->Result<EquationSnapshot,String>{let Self{notation,results,computed,equation}=self;let Expression{next_label,nodes}=equation;if nodes.is_empty(){return Err("Equation logical root missing".into())}let mut values=Values((0..nodes.len()).map(|_|None).collect());for(index,node)in nodes.into_iter().enumerate().rev(){let Node{label,kind,lexeme,numer,denom,name,terms,factors,base,exponent}=node;let kind=match(kind,lexeme,numer,denom,name,terms.is_empty(),factors.is_empty(),base,exponent){(Kind::Integer,Some(lexeme),None,None,None,true,true,None,None)=>EquationNodeKind::Integer{lexeme},(Kind::Rational,None,Some(numer),Some(denom),None,true,true,None,None)=>EquationNodeKind::Rational{numer,denom},(Kind::Symbol,None,None,None,Some(name),true,true,None,None)=>EquationNodeKind::Symbol{name},(Kind::Add,None,None,None,None,_,true,None,None)=>EquationNodeKind::Add{terms:group(&mut values.0,index,terms)?},(Kind::Mul,None,None,None,None,true,_,None,None)=>EquationNodeKind::Mul{factors:group(&mut values.0,index,factors)?},(Kind::Pow,None,None,None,None,true,true,Some(base),Some(exponent))=>{let mut children=Values(vec![Some(owned_child(&mut values.0,index,base)?)]);let exponent=owned_child(&mut values.0,index,exponent)?;EquationNodeKind::Pow{base:Box::new(children.0[0].take().unwrap()),exponent:Box::new(exponent)}},_=>return Err("Equation logical variant contains unrelated fields".into())};values.0[index]=Some(EquationNode{label:EquationNodeLabel(label),kind});}if values.0.iter().skip(1).any(Option::is_some){return Err("Equation logical record contains unowned nodes".into())}let expr=values.0[0].take().ok_or("Equation logical root missing")?;let(id,target_ref)=target(notation);let notation=store::ArtifactChild::new(id,target_ref);let(id,target_ref)=target(results);let results=store::ArtifactChild::new(id,target_ref);let(id,target_ref)=target(computed);let computed=store::ArtifactChild::new(id,target_ref);Ok(EquationSnapshot{notation,results,computed,equation:EquationExprSnapshot{expr,next_label}})}
}
#[path="🛬️decode/🦀️.rs"]
mod controlled;
/// 🛬️ Bind the explicit owned fields and admit reverse construction before every allocation.
pub(crate) fn reconstruct_pack_record_controlled(record:&dsl::RecordValue,control:&mut protocol::native_decoding::NativeDecodeControl<'_>,maximum_rows:usize)->Result<EquationSnapshot,String>{controlled::reconstruct(record,control,maximum_rows)}
/// 🖨️ Print the literal owned equation records without recursively lowering the expression.
pub(crate) fn print_pack_record_text(snapshot:&EquationSnapshot)->String{dsl::print(&EquationPackRecord::from_snapshot(snapshot).__dsl_to_record(),&EquationPackRecord::__dsl_spec(),dsl::JoinMode::Document)}
/// 📖️ Parse every owned field and refuse trailing unrelated native text.
pub(crate) fn parse_pack_record_text(body:&str)->Result<EquationSnapshot,store::TextError>{let record=dsl::parse_exact(body,&EquationPackRecord::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;EquationPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(dsl::__rt::field_error)}
impl store::ArtifactPack for EquationSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,PackError>{let inner=store::pack_rt::encode_document(&EquationPackRecord::__dsl_spec(),&EquationPackRecord::from_snapshot(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|PackError::Schema(e.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,PackError>{let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|e|PackError::Schema(e.to_string()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(PackError::Schema("Equation logical native identity differs".into()))}let(record,_)=store::pack_rt::decode_document(&inner,&EquationPackRecord::__dsl_spec(),options)?;EquationPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(PackError::Schema)}
 fn record_spec()->Option<dsl::RecordSpec>{Some(EquationPackRecord::__dsl_spec())}
}
/// 📦️ Encode the owned logical Equation record.
pub fn encode(snapshot:&EquationSnapshot)->Vec<u8>{store::ArtifactPack::encode_pack(snapshot)}
/// 📖️ Decode the owned logical Equation record.
pub fn decode(bytes:&[u8])->Result<EquationSnapshot,PackError>{<EquationSnapshot as store::ArtifactPack>::decode_pack(bytes)}

//#region 🔖️Store
pub type EquationEnvelope = store::ArtifactEnvelope<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;
pub type EquationStore = store::ArtifactStore<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>;

/// 🔐️ Opens a Equation store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`) — so a bare `EquationStore::new` can be
/// read but never mutated, undone or closed. The app installs the same catalog through
/// `build_document_store_owners`; every standalone store goes through here instead.
pub async fn new_equation_store(envelope: EquationEnvelope) -> Result<OwnedEquationStore, store::VcsError> {
    let mut store = EquationStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<crate::EquationSnapshot, crate::schema::mutations::EquationMutation>());
    Ok(OwnedEquationStore(store))
}

/// 🔚 A standalone Equation store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding, where the
/// original panic is the report worth keeping). Derefs to the bare store for every read and dispatch.
pub struct OwnedEquationStore(EquationStore);

impl OwnedEquationStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Equation document store closes through its exact bounded owners");
        }
    }
}

impl std::ops::Deref for OwnedEquationStore {
    type Target = EquationStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedEquationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedEquationStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

