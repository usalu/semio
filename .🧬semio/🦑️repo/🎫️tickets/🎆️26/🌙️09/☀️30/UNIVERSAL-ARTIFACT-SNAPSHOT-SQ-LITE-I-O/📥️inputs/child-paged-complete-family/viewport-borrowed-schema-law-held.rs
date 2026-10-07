#[cfg(test)]
mod borrowed_viewport_schema_tests{
 use super::*;
 use semio_framework_dsl_record::{BorrowedDslRecord,BorrowedShape};
 #[test]
 fn authored_viewport_borrowed_metadata_preserves_original_ids_and_optional_vector(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("@FRAMEWORK_CUSTOM_BORROWED_FIXTURE@")).unwrap();
  for(index,spec)in[<Viewport2d as BorrowedDslRecord>::RECORD,<Viewport3dOrbit as BorrowedDslRecord>::RECORD].into_iter().enumerate(){
   let row=&fixture["records"][index];assert!(spec.keyword.is_none());assert!(matches!(spec.layout,RecordLayout::Inline));assert_eq!(spec.fields.len(),row["fields"].as_array().unwrap().len());
   for(field,expected)in spec.fields.iter().zip(row["fields"].as_array().unwrap()){
    assert_eq!(field.id,expected[0].as_u64().unwrap()as u16);assert_eq!(field.key,expected[1].as_str().unwrap());assert_eq!(field.optional,expected[3].as_bool().unwrap());
    match(expected[2].as_str().unwrap(),field.shape){("Float",BorrowedShape::Float)=>{},("Tuple(Float,3)",BorrowedShape::Tuple(element,Some(3)))=>assert!(matches!(element(),BorrowedShape::Float)),_=>panic!("original viewport shape changed")}
   }
  }
  for _ in 0..256{
   assert!(std::ptr::eq(<Viewport2d as BorrowedDslRecord>::RECORD.fields.as_ptr(),BORROWED_VIEWPORT_PLANAR_FIELDS.as_ptr()));
   assert!(std::ptr::eq(<Viewport3dOrbit as BorrowedDslRecord>::RECORD.fields.as_ptr(),BORROWED_VIEWPORT_ORBIT_FIELDS.as_ptr()));
  }
 }
}
