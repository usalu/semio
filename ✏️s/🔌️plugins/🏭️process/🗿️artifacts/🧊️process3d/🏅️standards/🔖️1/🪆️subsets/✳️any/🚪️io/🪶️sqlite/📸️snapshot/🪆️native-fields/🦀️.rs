/// 🪆️ replacement for actual tagged domain fields, preserving controlled value errors.
macro_rules! controlled_tagged_variant_field{
 ($($name:ty),+)=>{$(
  impl semio_framework_dsl_record::BorrowedDslField for $name{
   const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Statements(<Self as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS);
  }
  impl semio_framework_dsl_record::DslField for $name{
   fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{Ok(semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control)?))}
   fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants())}
   fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)])}
   fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(items)if items.len()==1=><Self as semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0,&items[0].1).map_err(|error|error.message),_=>Err("expected one literal tagged Process3d statement".into())}}
   fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{semio_framework_dsl_record::native_encoding::project_statements(std::slice::from_ref(self),control)}
   fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;match value{semio_framework_dsl_record::FieldValue::Statements(items)if items.len()==1=>{let owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.scoped_stage(|control|{control.begin_stage(0)?;<Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control)})?,<Self as semio_framework_dsl_record::DslVariants>::retire_decoded_variant);control.step()?;Ok(owner.take())},_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"expected one literal tagged Process3d statement"))}})}
   fn retire_decoded(self){<Self as semio_framework_dsl_record::DslVariants>::retire_decoded_variant(self);}
  }
 )+};
}
