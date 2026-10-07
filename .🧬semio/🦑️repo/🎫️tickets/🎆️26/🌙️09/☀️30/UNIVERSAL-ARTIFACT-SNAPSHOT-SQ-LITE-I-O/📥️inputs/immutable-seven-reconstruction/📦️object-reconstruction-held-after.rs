fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::artifact::RowIndex;
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=single_float_row(database,"semio_object_document")?;identity(document,12)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio object document identifier"))}
 let mut references=RowIndex::new(database,"semio_object_reference",5,float_columns("semio_object_reference"),control,"duplicate Semio object reference identity")?;
 for(count,&index)in references.indices().iter().enumerate(){let row=references.row(index)?;for column in 1..5{row.text(column)?;}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,references.len())?;}
 let transform=SemioTransform{translation:SemioPoint3{x:document.real(2)?,y:document.real(3)?,z:document.real(4)?},rotation:SemioQuaternion{x:document.real(5)?,y:document.real(6)?,z:document.real(7)?,w:document.real(8)?},scale:SemioPoint3{x:document.real(9)?,y:document.real(10)?,z:document.real(11)?}};
 let mut snapshot=Owned::new(Self{schema:String::new(),transform,brep:None,mesh:None,properties:None});
 snapshot.get_mut().brep=child::<SemioBrepSnapshot>(database,"semio_object_brep_child","brep",&mut references,control)?;
 snapshot.get_mut().mesh=child::<SemioMeshSnapshot>(database,"semio_object_mesh_child","mesh",&mut references,control)?;
 snapshot.get_mut().properties=child::<SemioValueSnapshot>(database,"semio_object_value_child","value",&mut references,control)?;
 if references.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unowned Semio object reference"))}
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;snapshot.get_mut().validate().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;Ok(snapshot.take())
}
