struct OwnedSnapshot(Option<MdSnapshot>);
impl Drop for OwnedSnapshot{fn drop(&mut self){if let Some(value)=self.0.take(){super::owned_pack::retire_owned(value)}}}

fn validate_owned(snapshot:&MdSnapshot,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.md"||dialect.standard!="commonmark"||dialect.subset!="*"{return Err(format!("CommonMark does not own semantic subset {}",dialect.to_coordinate()).into())}
 let restored=OwnedSnapshot(Some(<MdSnapshot as ArtifactSqliteSnapshot>::from_sqlite_database(database,control)?));let expected=project(snapshot,control)?;let candidate=project(restored.0.as_ref().ok_or("CommonMark retained candidate is missing")?,control)?;let total=expected.tables.iter().try_fold(0usize,|total,table|total.checked_add(table.rows.len()).ok_or("CommonMark comparison work overflow"))?;let mut completed=0;
 if expected.tables.len()!=candidate.tables.len(){return Err("CommonMark document identity disagrees with its snapshot".to_string().into())}for(wanted,actual)in expected.tables.iter().zip(&candidate.tables){if wanted.name!=actual.name||wanted.rows.len()!=actual.rows.len(){return Err("CommonMark document identity disagrees with its snapshot".to_string().into())}for(row,other)in wanted.rows.iter().zip(&actual.rows){if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)?;}if row.rowid!=other.rowid||row.values!=other.values{return Err("CommonMark document identity disagrees with its snapshot".to_string().into())}completed+=1;}}
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)?;Ok(store::io_schema::IoOutcome::clean(()))
}

fn decode_hook(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<MdSnapshot,String>{super::owned_pack::decode_owned(payload,control)}
fn encode_hook(value:&MdSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{super::owned_pack::encode_owned(value,encoding,control)}
fn retire_hook(value:MdSnapshot){super::owned_pack::retire_owned(value)}
