//! 📥️ Literal Forms relationships reconstruct typed ownership with bounded frontiers.
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::{SQL,FormsSnapshot};
use crate::{FormExpr,FormQuestion,FormStep,FormQuestionOption,FormVectorField};
use crate::schema::{definition::FormsDefinition,response::{FormsAnswer,FormsResponse}};
use semio_framework_value::DslValue;
use semio_framework_value::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteDatabase,SqliteTable,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema_controlled};
use std::ops::Range;
const NAMES:[&str;30]=["forms_document","forms_structure_child","forms_results_child","forms_step","forms_question","forms_option","forms_vector_field","forms_response","forms_answer","forms_condition","forms_condition_const","forms_condition_var","forms_condition_eq","forms_condition_and","forms_condition_or","forms_condition_truthy","forms_condition_item","forms_value","forms_null","forms_boolean","forms_unsigned","forms_signed","forms_float","forms_text","forms_bytes","forms_octet","forms_array","forms_array_element","forms_object","forms_object_member"];
const WIDTHS:[usize;30]=[5,6,6,6,29,5,8,7,7,2,2,2,3,1,1,2,4,2,1,2,3,2,4,2,1,4,1,4,1,5];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Relation{table:usize,parent:i64,ordinal:i64,index:usize}
pub fn retire_value(value:DslValue){<DslValue as semio_framework_value::FromValue>::retire_decoded(value)}
pub fn retire_condition(root:FormExpr){let mut pending=vec![root];while let Some(value)=pending.pop(){match value{FormExpr::Const{value}=>retire_value(value),FormExpr::Var{..}=>{},FormExpr::Eq{left,right}=>{pending.push(*left);pending.push(*right);},FormExpr::And{mut items}|FormExpr::Or{mut items}=>pending.append(&mut items),FormExpr::Truthy{expr}=>pending.push(*expr)}}}
pub fn retire_snapshot(mut snapshot:FormsSnapshot){for step in &mut snapshot.definition.steps{for q in &mut step.blocks{if let Some(v)=q.default.take(){retire_value(v)}if let Some(v)=q.params.take(){retire_value(v)}if let Some(v)=q.condition.take(){retire_condition(v)}}}for r in &mut snapshot.responses{for a in &mut r.answers{retire_value(std::mem::replace(&mut a.value,semio_framework_value::DslValue::Null))}}}
struct SnapshotOwner(Option<FormsSnapshot>);
impl Drop for SnapshotOwner{fn drop(&mut self){if let Some(value)=self.0.take(){retire_snapshot(value)}}}
struct Pool{values:Vec<Option<DslValue>>,conditions:Vec<Option<FormExpr>>}
impl Drop for Pool{fn drop(&mut self){for v in &mut self.values{if let Some(v)=v.take(){retire_value(v)}}for v in &mut self.conditions{if let Some(v)=v.take(){retire_condition(v)}}}}
struct ValuesOwner(Vec<DslValue>);
impl Drop for ValuesOwner{fn drop(&mut self){for v in self.0.drain(..){retire_value(v)}}}
struct MembersOwner(Vec<(String,DslValue)>);
impl Drop for MembersOwner{fn drop(&mut self){for(_,v)in self.0.drain(..){retire_value(v)}}}
struct ConditionsOwner(Vec<FormExpr>);
impl Drop for ConditionsOwner{fn drop(&mut self){for v in self.0.drain(..){retire_condition(v)}}}
struct ValueOwner(Option<DslValue>);
impl Drop for ValueOwner{fn drop(&mut self){if let Some(v)=self.0.take(){retire_value(v)}}}
struct ConditionOwner(Option<FormExpr>);
impl Drop for ConditionOwner{fn drop(&mut self){if let Some(v)=self.0.take(){retire_condition(v)}}}
fn integer(row:&SqliteRow,index:usize)->Result<i64,ValueError>{row.integer(index)}
fn optional_id(row:&SqliteRow,index:usize)->Result<Option<i64>,ValueError>{if row.values[index]==SqliteValue::Null{Ok(None)}else{Ok(Some(integer(row,index)?))}}
fn boolean(row:&SqliteRow,index:usize)->Result<bool,ValueError>{match integer(row,index)?{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms boolean differs"))}}
fn optional_boolean(row:&SqliteRow,index:usize)->Result<Option<bool>,ValueError>{if row.values[index]==SqliteValue::Null{Ok(None)}else{Ok(Some(boolean(row,index)?))}}
fn unsigned(row:&SqliteRow,index:usize)->Result<u64,ValueError>{let high=u32::try_from(integer(row,index)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Forms unsigned high word differs"))?;let low=u32::try_from(integer(row,index+1)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Forms unsigned low word differs"))?;Ok((u64::from(high)<<32)|u64::from(low))}
fn float(row:&SqliteRow,index:usize)->Result<Option<f64>,ValueError>{
 if row.values[index]==SqliteValue::Null&&row.values[index+1]==SqliteValue::Null&&row.values[index+2]==SqliteValue::Null{return Ok(None)}
 let value=f64::from_bits(integer(row,index+1)? as u64);let class=if value.is_nan(){"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};
 if row.text(index+2)?!=class{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms IEEE class disagrees with bits"))}if value.is_nan(){if row.values[index]!=SqliteValue::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms NaN query scalar must be NULL"))}}else if row.real(index)?!=value{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms query REAL disagrees with bits"))}Ok(Some(value))
}
struct Reader<'d,'n,'p>{tables:[&'d SqliteTable;30],used:[Vec<bool>;30],relations:Vec<Relation>,value_pending:Vec<(i64,bool)>,condition_pending:Vec<(i64,bool)>,pool:Pool,native:&'n mut NativeDecodeControl<'p>}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 fn new(database:&'d SqliteDatabase,native:&'n mut NativeDecodeControl<'p>)->Result<Self,ValueError>{
  let mut tables=[database.table(NAMES[0])?;30];let mut used:[Vec<bool>;30]=std::array::from_fn(|_|Vec::new());let total=database.tables.iter().try_fold(0usize,|n,t|n.checked_add(t.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms row count overflow")))?;
  native.begin_stage(total)?;let mut relations=native.allocate_vec(total)?;
  for i in 0..30{tables[i]=database.table(NAMES[i])?;used[i]=native.allocate_vec(tables[i].rows.len())?;used[i].resize(tables[i].rows.len(),false);
   let mut previous=None;for(index,row)in tables[i].rows.iter().enumerate(){native.step()?;if row.values.len()!=WIDTHS[i]||row.integer(0)?!=row.rowid||previous.is_some_and(|id|id>=row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms row identity or column layout differs"))}previous=Some(row.rowid);if matches!(i,3|4|5|6|7|8|16|25|27|29){let ordinal=row.integer(2)?;if ordinal<0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms negative ordinal"))}relations.push(Relation{table:i,parent:row.integer(1)?,ordinal,index});}}
  }
  for(table,column,parent)in[(3,3,false),(4,3,false),(5,3,true),(6,3,true),(7,3,false),(8,3,true)]{let rows=&tables[table].rows;super::admission::identities::unique(rows.len(),rows.iter().map(|row|Ok((if parent{row.integer(1)?}else{0},row.text(column)?))),native)?;}
  native.begin_stage(total)?;for(table,rows)in tables.iter().enumerate(){for row in &rows.rows{native.step()?;match table{
   0=>{if row.text(1)?!="forms.form"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms marker differs"))}},
   1|2=>{if row.text(3)?!="s.stdio.semio"||row.text(4)?!="v1"||row.text(5)?!=if table==1{"value"}else{"table"}{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms child coordinate differs"))}},
   4=>{let min=float(row,10)?;let max=float(row,13)?;let increment=float(row,16)?;if row.text(5)?.trim().is_empty()||[min,max,increment].into_iter().flatten().any(|v|!v.is_finite())||min.zip(max).is_some_and(|(a,b)|a>b)||increment.is_some_and(|v|v<=0.0){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms question range differs"))}},
   6=>{if float(row,5)?.is_some_and(|v|!v.is_finite()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms vector field is not finite"))}},
   7=>{if row.text(6)?.is_empty()||unsigned(row,4)?>9_007_199_254_740_991{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms response metadata differs"))}},
   _=>{}
  }}}
  native.begin_stage(0)?;sort(&mut relations,native)?;
  let mut previous=None;for r in &relations{native.step()?;let expected=if let Some(p)=previous{let p:Relation=p;if p.table==r.table&&p.parent==r.parent{p.ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms ordinal overflow"))?}else{0}}else{0};if r.ordinal!=expected{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms relationship ordinals are not contiguous"))}previous=Some(*r)}
  let value_pending=native.allocate_vec(tables[17].rows.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms construction frontier overflow"))?)?;let condition_pending=native.allocate_vec(tables[9].rows.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms construction frontier overflow"))?)?;
  let mut values=native.allocate_vec(tables[17].rows.len())?;values.resize_with(tables[17].rows.len(),||None);let mut conditions=native.allocate_vec(tables[9].rows.len())?;conditions.resize_with(tables[9].rows.len(),||None);
  let work=total.checked_mul(2).and_then(|n|n.checked_add(tables[16].rows.len())).and_then(|n|n.checked_add(tables[27].rows.len())).and_then(|n|n.checked_add(tables[29].rows.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Forms reconstruction work overflow"))?;native.begin_stage(work)?;
  Ok(Self{tables,used,relations,value_pending,condition_pending,pool:Pool{values,conditions},native})
 }
 fn index(&self,table:usize,id:i64)->Result<usize,ValueError>{self.tables[table].rows.binary_search_by_key(&id,|r|r.rowid).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,format!("missing Forms {} identity {id}",NAMES[table])))}
 fn get(&self,table:usize,id:i64)->Result<&'d SqliteRow,ValueError>{Ok(&self.tables[table].rows[self.index(table,id)?])}
 fn take(&mut self,table:usize,id:i64)->Result<&'d SqliteRow,ValueError>{let index=self.index(table,id)?;if self.used[table][index]{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms repeated or cyclic entity ownership"))}self.used[table][index]=true;self.native.step()?;Ok(&self.tables[table].rows[index])}
 fn range(&self,table:usize,parent:i64)->Range<usize>{let start=self.relations.partition_point(|r|(r.table,r.parent)<(table,parent));let end=self.relations.partition_point(|r|(r.table,r.parent)<=(table,parent));start..end}
 fn relation(&self,index:usize)->&'d SqliteRow{let r=self.relations[index];&self.tables[r.table].rows[r.index]}
 fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,ValueError>{self.native.copy_text(row.text(index)?)}
 fn optional_text(&mut self,row:&SqliteRow,index:usize)->Result<Option<String>,ValueError>{row.optional_text(index)?.map(|s|self.native.copy_text(s)).transpose()}
 fn pop_value(&mut self,id:i64)->Result<DslValue,ValueError>{let i=self.index(17,id)?;self.pool.values[i].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Forms value owner is absent"))}
 fn pop_condition(&mut self,id:i64)->Result<FormExpr,ValueError>{let i=self.index(9,id)?;self.pool.conditions[i].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Forms condition owner is absent"))}
 fn value(&mut self,root:i64)->Result<DslValue,ValueError>{
  self.value_pending.push((root,false));
  while let Some((id,ready))=self.value_pending.pop(){let row=self.get(17,id)?;let kind=row.text(1)?;
   if !ready{self.take(17,id)?;let payload=match kind{"null"=>18,"boolean"=>19,"unsigned"=>20,"signed"=>21,"float"=>22,"text"=>23,"bytes"=>24,"array"=>26,"object"=>28,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms intrinsic kind differs"))};self.take(payload,id)?;self.value_pending.push((id,true));
    if kind=="array"||kind=="object"{let table=if kind=="array"{27}else{29};let value_col=if table==27{3}else{4};for index in self.range(table,id).rev(){let child=self.relation(index);self.take(table,child.rowid)?;self.value_pending.push((child.integer(value_col)?,false));}}
    continue
   }
   let value=match kind{
    "null"=>semio_framework_value::DslValue::Null,"boolean"=>semio_framework_value::DslValue::Bool(boolean(self.get(19,id)?,1)?),"unsigned"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(unsigned(self.get(20,id)?,1)?)),"signed"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(integer(self.get(21,id)?,1)?)),
    "float"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(float(self.get(22,id)?,1)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Forms float is absent"))?)),"text"=>{let row=self.get(23,id)?;semio_framework_value::DslValue::String(self.text(row,1)?)},
    "bytes"=>{let range=self.range(25,id);let mut bytes=self.native.allocate_vec(range.len())?;for index in range{let row=self.relation(index);self.take(25,row.rowid)?;bytes.push(u8::try_from(row.integer(3)?).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Forms octet differs"))?);}semio_framework_value::DslValue::Bytes(bytes)},
    "array"=>{let range=self.range(27,id);let mut owner=ValuesOwner(self.native.allocate_vec(range.len())?);for index in range{let row=self.relation(index);owner.0.push(self.pop_value(row.integer(3)?)?);self.native.step()?;}semio_framework_value::DslValue::Array(std::mem::take(&mut owner.0))},
    "object"=>{let range=self.range(29,id);let mut owner=MembersOwner(self.native.allocate_vec(range.len())?);for index in range{let row=self.relation(index);let name=self.text(row,3)?;let value=self.pop_value(row.integer(4)?)?;owner.0.push((name,value));self.native.step()?;}semio_framework_value::DslValue::Object(std::mem::take(&mut owner.0))},
    _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms intrinsic kind differs"))};
   let index=self.index(17,id)?;self.pool.values[index]=Some(value);
  }self.pop_value(root)
 }
 fn condition(&mut self,root:i64)->Result<FormExpr,ValueError>{
  self.condition_pending.push((root,false));
  while let Some((id,ready))=self.condition_pending.pop(){let row=self.get(9,id)?;let kind=row.text(1)?;
   if !ready{self.take(9,id)?;let payload=match kind{"const"=>10,"var"=>11,"eq"=>12,"and"=>13,"or"=>14,"truthy"=>15,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms condition kind differs"))};let p=self.take(payload,id)?;self.condition_pending.push((id,true));match kind{"eq"=>{self.condition_pending.push((p.integer(2)?,false));self.condition_pending.push((p.integer(1)?,false));},"truthy"=>self.condition_pending.push((p.integer(1)?,false)),"and"|"or"=>{for index in self.range(16,id).rev(){let r=self.relation(index);self.take(16,r.rowid)?;self.condition_pending.push((r.integer(3)?,false));}},_=>{}}continue}
   let value=match kind{
    "const"=>{let value_id=self.get(10,id)?.integer(1)?;FormExpr::Const{value:self.value(value_id)?}},
    "var"=>{let row=self.get(11,id)?;FormExpr::Var{name:self.text(row,1)?}},
    "eq"=>{let row=self.get(12,id)?;self.native.charge(2*std::mem::size_of::<FormExpr>())?;let mut left=ConditionOwner(Some(self.pop_condition(row.integer(1)?)?));let right=self.pop_condition(row.integer(2)?)?;FormExpr::Eq{left:Box::new(left.0.take().unwrap()),right:Box::new(right)}},
    "truthy"=>{let row=self.get(15,id)?;self.native.charge(std::mem::size_of::<FormExpr>())?;FormExpr::Truthy{expr:Box::new(self.pop_condition(row.integer(1)?)?)}},
    "and"|"or"=>{let range=self.range(16,id);let mut owner=ConditionsOwner(self.native.allocate_vec(range.len())?);for index in range{let row=self.relation(index);owner.0.push(self.pop_condition(row.integer(3)?)?);self.native.step()?;}let items=std::mem::take(&mut owner.0);if kind=="and"{FormExpr::And{items}}else{FormExpr::Or{items}}},
    _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms condition kind differs"))};
   let index=self.index(9,id)?;self.pool.conditions[index]=Some(value);
  }self.pop_condition(root)
 }
 fn question(&mut self,row:&'d SqliteRow)->Result<FormQuestion,ValueError>{
  let mut q=FormQuestion{id:self.text(row,3)?,label:self.text(row,4)?,kind:self.text(row,5)?,description:self.optional_text(row,6)?,required:optional_boolean(row,7)?,placeholder:self.optional_text(row,8)?,default:None,min:float(row,10)?,max:float(row,13)?,step:float(row,16)?,unit:self.optional_text(row,19)?,text:self.optional_text(row,20)?,options:None,fields:None,schema:self.optional_text(row,23)?,src:self.optional_text(row,24)?,accept:self.optional_text(row,25)?,example_id:self.optional_text(row,26)?,params:None,condition:None};
  let options=self.range(5,row.rowid);if boolean(row,21)?{let mut values=self.native.allocate_vec(options.len())?;for index in options{let option=self.relation(index);self.take(5,option.rowid)?;values.push(FormQuestionOption{value:self.text(option,3)?,label:self.text(option,4)?});}q.options=Some(values)}else if !options.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms absent options own rows"))}
  let fields=self.range(6,row.rowid);if boolean(row,22)?{let mut values=self.native.allocate_vec(fields.len())?;for index in fields{let field=self.relation(index);self.take(6,field.rowid)?;values.push(FormVectorField{key:self.text(field,3)?,label:self.optional_text(field,4)?,value:float(field,5)?});}q.fields=Some(values)}else if !fields.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms absent vector fields own rows"))}
  let mut default=ValueOwner(optional_id(row,9)?.map(|id|self.value(id)).transpose()?);let mut params=ValueOwner(optional_id(row,27)?.map(|id|self.value(id)).transpose()?);let mut condition=ConditionOwner(optional_id(row,28)?.map(|id|self.condition(id)).transpose()?);
  q.default=default.0.take();q.params=params.0.take();q.condition=condition.0.take();Ok(q)
 }
 fn child<S>(&mut self,table:usize,id:i64)->Result<store::ArtifactChild<S>,ValueError>{let row=self.take(table,id)?;Ok(store::ArtifactChild::new(self.text(row,1)?,semio_framework_artifact_reference::ArtifactRef{artifact_id:self.text(row,2)?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:self.text(row,3)?,standard:self.text(row,4)?,subset:self.text(row,5)?}}))}
 fn snapshot(&mut self)->Result<FormsSnapshot,ValueError>{
  if self.tables[0].rows.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms must own one document"))}let id=self.tables[0].rows[0].rowid;let row=self.take(0,id)?;let mut owner=SnapshotOwner(Some(FormsSnapshot{schema:self.text(row,1)?,id:self.text(row,2)?,version:self.text(row,3)?,title:self.optional_text(row,4)?,definition:FormsDefinition{steps:self.native.allocate_vec(self.range(3,id).len())?},responses:self.native.allocate_vec(self.range(7,id).len())?,structure:self.child(1,id)?,results:self.child(2,id)?}));
  for index in self.range(3,id){let row=self.relation(index);self.take(3,row.rowid)?;let step=FormStep{id:self.text(row,3)?,title:self.text(row,4)?,description:self.optional_text(row,5)?,blocks:self.native.allocate_vec(self.range(4,row.rowid).len())?};owner.0.as_mut().unwrap().definition.steps.push(step);
   for index in self.range(4,row.rowid){let row=self.relation(index);self.take(4,row.rowid)?;let question=self.question(row)?;owner.0.as_mut().unwrap().definition.steps.last_mut().unwrap().blocks.push(question);}
  }
  for index in self.range(7,id){let row=self.relation(index);self.take(7,row.rowid)?;let response=FormsResponse{id:self.text(row,3)?,submitted_at:unsigned(row,4)?,definition_version:self.text(row,6)?,answers:self.native.allocate_vec(self.range(8,row.rowid).len())?};owner.0.as_mut().unwrap().responses.push(response);
   for index in self.range(8,row.rowid){let row=self.relation(index);self.take(8,row.rowid)?;let question_id=self.text(row,3)?;let label=self.text(row,4)?;let kind=self.text(row,5)?;let value=self.value(row.integer(6)?)?;owner.0.as_mut().unwrap().responses.last_mut().unwrap().answers.push(FormsAnswer{question_id,label,kind,value});}
  }
  for table in &self.used{for used in table{self.native.step()?;if !used{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms has unreachable or unused relational rows"))}}}self.native.checkpoint()?;Ok(owner.0.take().unwrap())
 }
}
fn sort(rows:&mut[Relation],control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 fn sift(rows:&mut[Relation],mut root:usize,end:usize,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{loop{let Some(child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{break};control.step()?;let next=if child+1<end&&rows[child]<rows[child+1]{child+1}else{child};if rows[root]>=rows[next]{break}rows.swap(root,next);root=next;}Ok(())}
 for root in(0..rows.len()/2).rev(){sift(rows,root,rows.len(),control)?}for end in(1..rows.len()).rev(){control.step()?;rows.swap(0,end);sift(rows,0,end,control)?}Ok(())
}
pub fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<FormsSnapshot,ValueError>{
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;let maximum=control.limits().max_value_bytes;
 let result=control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress|{let mut callback=|p:semio_framework_value::native_decoding::NativeDecodeProgress|progress(p.completed,p.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining.min(maximum),&mut callback);let result=(||{let mut reader=Reader::new(database,&mut native)?;reader.snapshot()})();(result,native.owned_bytes())})??;
 let mut owner=SnapshotOwner(Some(result));super::admission::forecast(owner.0.as_ref().unwrap(),control,SqliteSnapshotPhase::ReconstructSnapshot)?;Ok(owner.0.take().unwrap())
}
