mod parsed_value_authority{
    pub trait Sealed{}
    impl Sealed for super::super::Value{}
    impl Sealed for semio_framework_value::DslValue{}
}
/// 🧬️ Closed first-party semantic destinations move each original admitted JSON cell exactly once.
pub trait JsonParsedValue:parsed_value_authority::Sealed+semio_framework_value::retirement::RetireOwned{
    fn json_null()->Self;
    fn json_bool(value:bool)->Self;
    fn json_number(value:super::Number)->Self;
    fn json_string(value:String)->Self;
    fn json_array(values:Vec<Self>)->Self where Self:Sized;
    fn json_object(values:Vec<(String,Self)>)->Self where Self:Sized;
}
impl JsonParsedValue for super::Value{
    fn json_null()->Self{Self::Null}
    fn json_bool(value:bool)->Self{Self::Bool(value)}
    fn json_number(value:super::Number)->Self{Self::Number(value)}
    fn json_string(value:String)->Self{Self::String(value)}
    fn json_array(values:Vec<Self>)->Self{Self::Array(values)}
    fn json_object(values:Vec<(String,Self)>)->Self{Self::Object(super::Object(values))}
}
impl JsonParsedValue for semio_framework_value::DslValue{
    fn json_null()->Self{Self::Null}
    fn json_bool(value:bool)->Self{Self::Bool(value)}
    fn json_number(value:super::Number)->Self{Self::Number(match value{super::Number::UInt(value)=>semio_framework_value::Number::UInt(value),super::Number::Int(value)=>semio_framework_value::Number::Int(value),super::Number::Float(value)=>semio_framework_value::Number::Float(value)})}
    fn json_string(value:String)->Self{Self::String(value)}
    fn json_array(values:Vec<Self>)->Self{Self::Array(values)}
    fn json_object(values:Vec<(String,Self)>)->Self{Self::Object(values)}
}
