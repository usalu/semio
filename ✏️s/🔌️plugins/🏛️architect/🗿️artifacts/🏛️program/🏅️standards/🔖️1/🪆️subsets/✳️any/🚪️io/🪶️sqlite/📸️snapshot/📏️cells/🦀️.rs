//! 📏️ Exact authored Architect storage cells are admitted from the borrowed native record.
use super::*;
use semio_framework_dsl_record::{DslField,FieldValue,RecordSpec,RecordValue};
use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};
pub(super)trait NativeSqlField:Sized{
 const OPTIONAL_PRESENCE:usize=0;
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>;
}
fn required(value:Option<&FieldValue>)->Result<&FieldValue,ValueError>{value.ok_or_else(||invalid("Architect required native SQL field is absent"))}
pub(super)fn scalar<T:DslField>(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;T::from_value_controlled(required(value)?,native)})}
fn sum(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Architect semantic cell extent overflow"))}
pub(super)fn add(bytes:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{let next=sum(*bytes,count)?;if next>maximum{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Architect semantic cells exceed caller value limit"))}*bytes=next;Ok(())}
impl NativeSqlField for String{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.checkpoint()?;match required(value)?{FieldValue::Text(value)=>Ok(value.len()),_=>Err(invalid("Architect native text storage field requires Text"))}}
}
impl NativeSqlField for EntityId{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{String::size(value,native)}
}
impl NativeSqlField for bool{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{let _=scalar::<Self>(value,native)?;Ok(8)}
}
impl NativeSqlField for u32{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{let _=scalar::<Self>(value,native)?;Ok(8)}
}
impl NativeSqlField for u64{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{let word=scalar::<Self>(value,native)?;Ok(8+if word==0{1}else{word.ilog10()as usize+1})}
}
impl NativeSqlField for f64{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{let value=scalar::<Self>(value,native)?;Ok(8+float_class(value).len()+if value.is_nan(){0}else{8})}
}
impl<T:NativeSqlField>NativeSqlField for Option<T>{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.checkpoint()?;match value{None|Some(FieldValue::Absent)=>Ok(T::OPTIONAL_PRESENCE),Some(value)=>sum(T::OPTIONAL_PRESENCE,T::size(Some(value),native)?)}}
}
macro_rules! field_sizes{($row:ident,$spec:ident,$native:ident;$($field:ident:$type:ty),+)=>{{let mut bytes=0usize;$(bytes=sum(bytes,<$type as NativeSqlField>::size(native_field($row,&$spec,stringify!($field))?,$native)?)?;)+bytes}}}
impl NativeSqlField for TextField{
 const OPTIONAL_PRESENCE:usize=8;
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=native_record(required(value)?)?;let spec=Self::__dsl_spec_controlled(native)?;Ok(field_sizes!(row,spec,native;text:String,format:Option<String>))})}
}
impl NativeSqlField for TaggedNote{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=native_record(required(value)?)?;let spec=Self::__dsl_spec_controlled(native)?;Ok(field_sizes!(row,spec,native;tag:String,text:String))})}
}
impl NativeSqlField for TraceLink{
 fn size(value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=native_record(required(value)?)?;let spec=Self::__dsl_spec_controlled(native)?;Ok(field_sizes!(row,spec,native;id:EntityId,from_id:EntityId,to_id:EntityId,kind:TraceKind,label:Option<String>))})}
}
pub(super)fn ordered<T:NativeSqlField>(values:&[FieldValue],native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(values.len())?;for value in values{native.step()?;add(bytes,sum(24,T::size(Some(value),native)?)?,maximum)?;}native.checkpoint()})}
fn strings(row:&RecordValue,spec:&RecordSpec,names:&[&str],native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{for name in names{ordered::<String>(native_list(native_field(row,spec,name)?)?,native,bytes,maximum)?;}Ok(())}
fn timestamp(row:&RecordValue,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let spec=TimestampMeta::__dsl_spec_controlled(native)?;Ok(field_sizes!(row,spec,native;created:String,updated:String,created_by:Option<EntityId>,updated_by:Option<EntityId>))})}
fn ownership(row:&RecordValue,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let spec=Ownership::__dsl_spec_controlled(native)?;add(bytes,field_sizes!(row,spec,native;owner_id:Option<EntityId>,authority_id:Option<EntityId>),maximum)?;strings(row,&spec,&["consultant_ids","participant_ids"],native,bytes,maximum)})}
pub(super)fn header(register:&str,row:&RecordValue,spec:&RecordSpec,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=required_native_record(row,spec,"header")?;let spec=EntityHeader::__dsl_spec_controlled(native)?;add(bytes,sum(24,register.len())?,maximum)?;add(bytes,field_sizes!(row,spec,native;id:EntityId,name:String,description:Option<TextField>,status:LifecycleStatus,priority:Priority),maximum)?;add(bytes,timestamp(required_native_record(row,&spec,"timestamps")?,native)?,maximum)?;ownership(required_native_record(row,&spec,"ownership")?,native,bytes,maximum)?;ordered::<String>(native_list(native_field(row,&spec,"tags")?)?,native,bytes,maximum)?;ordered::<TaggedNote>(native_list(native_field(row,&spec,"notes")?)?,native,bytes,maximum)})}
pub(super)fn quantity(slot:&str,value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=native_record(required(value)?)?;let spec=QuantitySpec::__dsl_spec_controlled(native)?;add(bytes,sum(32,slot.len())?,maximum)?;add(bytes,field_sizes!(row,spec,native;unit:String,min:Option<f64>,max:Option<f64>,target:Option<f64>,current:Option<f64>,forecast:Option<f64>,peak:Option<f64>,average:Option<f64>),maximum)})}
fn child(slot:&str,value:Option<&FieldValue>,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let row=native_record(required(value)?)?;let reference=native_record(required(row.get(1))?)?;add(bytes,sum(16,slot.len())?,maximum)?;add(bytes,String::size(row.get(0),native)?,maximum)?;for id in 0..4{add(bytes,String::size(reference.get(id),native)?,maximum)?;}native.checkpoint()})}
pub(super)fn core(root:&RecordValue,root_spec:&RecordSpec,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{
 native.begin_stage(0)?;
 add(bytes,sum(8,String::size(native_field(root,root_spec,"schema")?,native)?)?,maximum)?;
 let row=required_native_record(root,root_spec,"meta")?;let spec=ProgramMeta::__dsl_spec_controlled(native)?;
 add(bytes,16,maximum)?;add(bytes,field_sizes!(row,spec,native;schema:String,document_id:String,title:String,subtitle:Option<String>,purpose:TextField,industry_sector:String,project_type:String,locale:String,revision:String,source_system:Option<String>,export_profile:Option<String>),maximum)?;
 add(bytes,timestamp(required_native_record(row,&spec,"timestamps")?,native)?,maximum)?;
 strings(row,&spec,&["terminology","classification","author_ids"],native,bytes,maximum)?;
 let row=required_native_record(root,root_spec,"project")?;let spec=ProjectDefinition::__dsl_spec_controlled(native)?;
 add(bytes,16,maximum)?;add(bytes,field_sizes!(row,spec,native;id:EntityId,code:String,client_name:String,owner_organization:String,brief_summary:TextField,problem_statement:TextField,vision:TextField,mission:TextField,geographic_context:TextField,development_context:TextField,operational_context:TextField,funding_model:String),maximum)?;
 add(bytes,timestamp(required_native_record(row,&spec,"timestamps")?,native)?,maximum)?;
 ownership(required_native_record(row,&spec,"ownership")?,native,bytes,maximum)?;
 strings(row,&spec,&["objectives","success_criteria","completion_criteria","decision_criteria","scope_inclusions","scope_exclusions","assumptions","constraints_summary","dependencies","deliverables","phases","regulatory_context"],native,bytes,maximum)?;
 ordered::<Priority>(native_list(native_field(row,&spec,"project_priorities")?)?,native,bytes,maximum)?;
 let row=required_native_record(root,root_spec,"governance")?;let spec=Governance::__dsl_spec_controlled(native)?;
 add(bytes,16,maximum)?;add(bytes,field_sizes!(row,spec,native;id:EntityId,framework:String,quality_policy:TextField,risk_appetite:Option<String>,audit_schedule:Option<String>,owner_id:Option<EntityId>,review_cycle:Option<String>,policy_ownership_id:Option<EntityId>,requirement_ownership_id:Option<EntityId>,risk_ownership_id:Option<EntityId>,reporting_frequency:Option<String>),maximum)?;
 strings(row,&spec,&["roles","responsibilities","approval_matrix","escalation_paths","meeting_cadence","decision_rights","change_control_process","compliance_obligations","document_control","stakeholder_engagement_plan","ethics_policy","data_governance","review_hierarchy","accountability_rules","exception_management","governance_performance"],native,bytes,maximum)?;
 child("knowledge",native_field(root,root_spec,"knowledge")?,native,bytes,maximum)?;
 child("benchmarks",native_field(root,root_spec,"benchmarks")?,native,bytes,maximum)?;
 ordered::<TraceLink>(native_list(native_field(root,root_spec,"traces")?)?,native,bytes,maximum)?;
 native.checkpoint()
})}

/// 🔢️ Exact decimal query text for an authored unsigned64 scalar stays on the stack.
#[derive(Clone,Copy)]
pub(super)struct Decimal64{bytes:[u8;20],start:usize}
impl Decimal64{
 pub(super)fn new(mut value:u64)->Self{let mut bytes=[b'0';20];let mut start=20;loop{start-=1;bytes[start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}Self{bytes,start}}
 pub(super)fn text(&self)->&str{std::str::from_utf8(&self.bytes[self.start..]).expect("authored unsigned64 decimal is ASCII")}
}
/// 🧮️ The independent authored schema has at most32columns, including its surrogate identity.
pub(super)struct FieldCells<'a>{values:[FieldCell<'a>;31],length:usize}
impl<'a>FieldCells<'a>{
 pub(super)fn new()->Self{Self{values:[FieldCell::Borrowed(Cell::Null);31],length:0}}
 pub(super)fn push(&mut self,value:FieldCell<'a>){assert!(self.length<self.values.len(),"Architect authored scalar width exceeds its declared schema");self.values[self.length]=value;self.length+=1;}
 pub(super)fn extend(&mut self,values:impl IntoIterator<Item=FieldCell<'a>>){for value in values{self.push(value);}}
}
impl<'a>std::ops::Deref for FieldCells<'a>{type Target=[FieldCell<'a>];fn deref(&self)->&Self::Target{&self.values[..self.length]}}

pub(super)fn audit(row:&RecordValue,spec:&RecordSpec,native:&mut NativeDecodeControl<'_>,bytes:&mut usize,maximum:usize)->Result<(),ValueError>{native.scoped_stage(|native|{
 native.begin_stage(0)?;add(bytes,8,maximum)?;
 add(bytes,field_sizes!(row,spec,native;action:AuditAction,actor_id:Option<EntityId>,subject_id:EntityId,subject_kind:String,timestamp:String,details:TextField,before_state:Option<String>,after_state:Option<String>,ip_address:Option<String>,client:Option<String>,session_id:Option<String>,change_record_id:Option<EntityId>,success:bool,error_message:Option<String>,correlation_id:Option<String>,retention_until:Option<String>),maximum)?;
 ordered::<String>(native_list(native_field(row,spec,"compliance_tags")?)?,native,bytes,maximum)?;
 if let Some(value)=native_field(row,spec,"trace_link")?.filter(|value|!matches!(value,FieldValue::Absent)){add(bytes,sum(8,TraceLink::size(Some(value),native)?)?,maximum)?;}
 native.checkpoint()
})}
