impl semio_framework_dsl_record::BorrowedDslField for WidgetDsl{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Statements(<Self as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS);
}
impl semio_framework_dsl_record::BorrowedDslField for Widget{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=<WidgetDsl as semio_framework_dsl_record::BorrowedDslField>::SHAPE;
}
impl semio_framework_dsl_record::BorrowedDslRecord for SynapseSpec{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=<SynapseDsl as semio_framework_dsl_record::BorrowedDslRecord>::RECORD;
}
impl semio_framework_dsl_record::BorrowedDslField for SynapseSpec{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
impl semio_framework_dsl_record::BorrowedDslRecord for FlowHostSnapshot{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=<FlowHostSnapshotDsl as semio_framework_dsl_record::BorrowedDslRecord>::RECORD;
}
impl semio_framework_dsl_record::BorrowedDslField for FlowHostSnapshot{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
