//! 📋️ Flat Forms native records retain literal field ownership independently of SQL.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
#[derive(semio_framework_dsl_record_derive::DslScalar)]pub(super) enum Kind{Null,Boolean,Unsigned,Signed,Float,Text,Bytes,Array,Object}
pub(super) struct Octets(pub Vec<u8>);
impl semio_framework_dsl_record::BorrowedDslField for Octets {
 const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Bytes64;
}
impl DslField for Octets{
 fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Bytes64}
 fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(c:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{c.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Bytes64)}
 fn to_value(&self)->FieldValue{semio_framework_dsl_record::FieldValue::Bytes64(self.0.clone())}
 fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{c.copy_bytes(&self.0).map(semio_framework_dsl_record::FieldValue::Bytes64)}
 fn from_value(v:&FieldValue)->Result<Self,String>{match v{semio_framework_dsl_record::FieldValue::Bytes64(v)=>Ok(Self(v.clone())),_=>Err("Forms octet field differs".into())}}
 fn from_value_controlled(v:&FieldValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match v{semio_framework_dsl_record::FieldValue::Bytes64(v)=>c.copy_bytes(v).map(Self),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Forms octet field differs"))}}
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Member{pub name:String,pub value:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Value{
 pub kind:Kind,pub boolean:Option<bool>,pub unsigned:Option<u64>,pub signed:Option<i64>,pub float_bits:Option<u64>,pub text:Option<String>,pub bytes:Option<Octets>,pub items:Vec<u64>,pub members:Vec<Member>
}
#[derive(semio_framework_dsl_record_derive::DslScalar)]pub(super) enum ConditionKind{Const,Var,Eq,And,Or,Truthy}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Condition{
 pub kind:ConditionKind,pub value:Option<u64>,pub name:Option<String>,pub left:Option<u64>,pub right:Option<u64>,pub expression:Option<u64>,pub items:Vec<u64>
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct OptionRecord{pub value:String,pub label:String}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct VectorRecord{pub key:String,pub label:Option<String>,pub value:Option<f64>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Question{
 pub id:String,pub label:String,pub kind:String,pub description:Option<String>,pub required:Option<bool>,pub placeholder:Option<String>,pub default:Option<u64>,
 pub min:Option<f64>,pub max:Option<f64>,pub step:Option<f64>,pub unit:Option<String>,pub text:Option<String>,
 pub options:Option<Vec<OptionRecord>>,pub fields:Option<Vec<VectorRecord>>,pub schema:Option<String>,pub src:Option<String>,pub accept:Option<String>,pub example_id:Option<String>,pub params:Option<u64>,pub condition:Option<u64>
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Step{pub id:String,pub title:String,pub description:Option<String>,pub questions:Vec<Question>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Answer{pub question_id:String,pub label:String,pub kind:String,pub value:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]pub(super) struct Response{pub id:String,pub submitted_at:u64,pub definition_version:String,pub answers:Vec<Answer>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]#[dsl(extension="forms")]
pub(super) struct Document{
 pub schema:String,pub id:String,pub version:String,pub title:Option<String>,pub structure:crate::FormsStructureChild,pub results:crate::FormsResultsChild,
 pub steps:Vec<Step>,pub responses:Vec<Response>,pub values:Vec<Value>,pub conditions:Vec<Condition>
}
