static BORROWED_VIEWPORT_PLANAR_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"x",semio_framework_dsl_record::BorrowedShape::Float),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"y",semio_framework_dsl_record::BorrowedShape::Float),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"zoom",semio_framework_dsl_record::BorrowedShape::Float),
];
static BORROWED_VIEWPORT_ORBIT_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"position",semio_framework_dsl_record::BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3))),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"target",semio_framework_dsl_record::BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3))),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"zoom",semio_framework_dsl_record::BorrowedShape::Float),
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(4,"up",semio_framework_dsl_record::BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3)))},
];
impl semio_framework_dsl_record::BorrowedDslRecord for Viewport2d{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:BORROWED_VIEWPORT_PLANAR_FIELDS};
}
impl semio_framework_dsl_record::BorrowedDslField for Viewport2d{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
impl semio_framework_dsl_record::BorrowedDslRecord for Viewport3dOrbit{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:BORROWED_VIEWPORT_ORBIT_FIELDS};
}
impl semio_framework_dsl_record::BorrowedDslField for Viewport3dOrbit{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
