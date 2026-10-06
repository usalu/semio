static BORROWED_RUN_STATUS:&[(&'static str,u32)]=&[("pending",0),("running",1),("succeeded",2),("failed",3),("canceled",4)];
static BORROWED_RUN_NODE_STATUS:&[(&'static str,u32)]=&[("computed",0),("cacheHit",1),("failed",2)];
static BORROWED_RUN_TRIGGER_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"kind",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(1,"actor",semio_framework_dsl_record::BorrowedShape::Text)},
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(2,"automation_ref",semio_framework_dsl_record::BorrowedShape::Text)},
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(3,"event_fingerprint",semio_framework_dsl_record::BorrowedShape::Text)},
];
impl semio_framework_dsl_record::BorrowedDslField for RunStatus{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_RUN_STATUS);}
impl semio_framework_dsl_record::BorrowedDslField for RunNodeStatus{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_RUN_NODE_STATUS);}
impl semio_framework_dsl_record::BorrowedDslRecord for RunTrigger{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:BORROWED_RUN_TRIGGER_FIELDS};
}
impl semio_framework_dsl_record::BorrowedDslField for RunTrigger{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
}
