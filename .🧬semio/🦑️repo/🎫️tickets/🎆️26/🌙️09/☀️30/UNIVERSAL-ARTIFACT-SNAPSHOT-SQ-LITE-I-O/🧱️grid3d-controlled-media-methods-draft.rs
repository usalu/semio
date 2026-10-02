fn shape_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::Shape,String>{Ok(dsl::Shape::Statements(<Self as dsl::DslVariants>::variants_controlled(control)?))}
fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match value{dsl::FieldValue::Statements(items)if items.len()==1=><Self as dsl::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control).map_err(|e|e.message),_=>Err("grid3d expected exactly one tagged media record".into())}}
fn to_value_controlled(&self,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{dsl::native_encoding::project_statements(std::slice::from_ref(self),control)}
fn retire_decoded(self){<Self as dsl::DslVariants>::retire_decoded_variant(self)}
