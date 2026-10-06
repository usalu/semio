static BORROWED_ARTIFACT_REFERENCE_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"artifact-id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"artifact-kind",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"standard",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"subset",semio_framework_dsl_record::BorrowedShape::Text),
];
impl semio_framework_dsl_record::BorrowedDslRecord for ArtifactRef{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:BORROWED_ARTIFACT_REFERENCE_FIELDS};
}
impl semio_framework_dsl_record::BorrowedDslField for ArtifactRef{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
