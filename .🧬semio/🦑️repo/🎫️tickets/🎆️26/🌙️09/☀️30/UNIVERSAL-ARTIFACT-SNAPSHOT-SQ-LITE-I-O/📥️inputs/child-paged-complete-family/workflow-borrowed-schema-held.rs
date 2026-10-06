static BORROWED_MEDIA_CLASS:&[(&'static str,u32)]=&[("twoD",0),("threeD",1),("text",2),("data",3),("graph",4),("kit",5),("computation",6),("presentation",7)];
static BORROWED_MEDIA_FORM:&[(&'static str,u32)]=&[("any",0),("vector",1),("raster",2),("brep",3),("mesh",4),("document",5),("value",6),("dag",7),("trinity",8),("type",9),("design",10),("kit",11),("flow",12),("sequence",13),("procedure",14),("deck",15)];
static BORROWED_MEDIA_DIRECTION:&[(&'static str,u32)]=&[("in",0),("out",1)];
static BORROWED_PORT_MULTIPLICITY:&[(&'static str,u32)]=&[("one",0),("many",1)];
static BORROWED_MEDIA_CONTRACT_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"kind_id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"class",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_CLASS)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"form",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_FORM)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"wire_kind",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(4,"wire_format",semio_framework_dsl_record::BorrowedShape::Text)},
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(5,"wire_schema",semio_framework_dsl_record::BorrowedShape::Text)},
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(6,"conversion_from",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_FORM))},
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(7,"conversion_to",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_FORM))},
];
static BORROWED_WORKFLOW_MEDIA_PORT_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"port_id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"label",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"direction",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_DIRECTION)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(4,"class",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_CLASS)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(5,"form",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_MEDIA_FORM)),
 semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(6,"kind_id",semio_framework_dsl_record::BorrowedShape::Text)},
 semio_framework_dsl_record::BorrowedFieldSpec::new(7,"required",semio_framework_dsl_record::BorrowedShape::Bool),
 semio_framework_dsl_record::BorrowedFieldSpec::new(8,"multiplicity",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_PORT_MULTIPLICITY)),
];
static BORROWED_WORKFLOW_INPUT_FIELDS:&[semio_framework_dsl_record::BorrowedFieldSpec]=&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"kind_id",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"selector",semio_framework_dsl_record::BorrowedShape::Text),
 semio_framework_dsl_record::BorrowedFieldSpec::new(3,"required",semio_framework_dsl_record::BorrowedShape::Bool),
 semio_framework_dsl_record::BorrowedFieldSpec::new(4,"multiplicity",semio_framework_dsl_record::BorrowedShape::Enum(BORROWED_PORT_MULTIPLICITY)),
];
impl semio_framework_dsl_record::BorrowedDslRecord for MediaContract{const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:BORROWED_MEDIA_CONTRACT_FIELDS};}
impl semio_framework_dsl_record::BorrowedDslField for MediaContract{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}
impl semio_framework_dsl_record::BorrowedDslRecord for WorkflowMediaPort{const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:BORROWED_WORKFLOW_MEDIA_PORT_FIELDS};}
impl semio_framework_dsl_record::BorrowedDslField for WorkflowMediaPort{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}
impl semio_framework_dsl_record::BorrowedDslRecord for WorkflowInput{const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:BORROWED_WORKFLOW_INPUT_FIELDS};}
impl semio_framework_dsl_record::BorrowedDslField for WorkflowInput{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}
