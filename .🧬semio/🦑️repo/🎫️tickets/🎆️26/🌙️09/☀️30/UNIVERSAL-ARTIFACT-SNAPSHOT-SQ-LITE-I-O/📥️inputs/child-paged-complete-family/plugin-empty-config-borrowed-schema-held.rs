static BORROWED_NO_CONFIG_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[];
impl semio_framework_dsl_record::BorrowedDslRecord for NoConfig{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Lines,fields:BORROWED_NO_CONFIG_FIELDS};
}
impl semio_framework_dsl_record::BorrowedDslField for NoConfig{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
