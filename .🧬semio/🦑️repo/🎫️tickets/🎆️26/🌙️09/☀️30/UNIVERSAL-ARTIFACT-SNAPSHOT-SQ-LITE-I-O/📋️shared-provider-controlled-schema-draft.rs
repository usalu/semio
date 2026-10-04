/// 🏛️ Constructs authored declarations through the existing controlled schema authority once.
pub(super) fn schema_controlled(sql:&str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase>{
 control.checkpoint(phase,0,0)?;let limits=control.limits();bound(sql.len(),limits.max_schema_bytes,ValueRefusalKind::OwnershipLimit,"SQLite declared schema bytes")?;
 let tokens=lex(sql,phase,control)?;let mut tables:Vec<SqliteTable>=Vec::new();let mut start=0usize;let mut schema_bytes=0usize;
 for end in 0..=tokens.len(){
  if end!=tokens.len()&&(tokens[end].quoted||tokens[end].literal||tokens[end].text!=";"){continue}
  if start<end{
   let statement=sql[tokens[start].start..tokens[end-1].end].trim();let definition=parse_table(statement,phase,control)?;
   bound(definition.columns.len(),limits.max_columns,ValueRefusalKind::WorkLimit,"SQLite columns")?;
   bound(sum(tables.len(),1)?,limits.max_tables,ValueRefusalKind::WorkLimit,"SQLite tables")?;
   for table in &tables{if ascii_compare(&table.name,&definition.name,phase,control)?.is_eq(){return Err(invalid("SQLite duplicate declared table"))}}
   schema_bytes=sum(sum(schema_bytes,statement.len())?,definition.name.len())?;bound(schema_bytes,limits.max_schema_bytes,ValueRefusalKind::OwnershipLimit,"SQLite schema bytes")?;
   let sql=copy_text(statement,phase,control)?;grow(&mut tables,phase,control)?;tables.push(SqliteTable{name:definition.name,sql,rows:Vec::new()});
  }
  start=end+1;
 }
 control.checkpoint(phase,tokens.len(),tokens.len())?;Ok(SqliteDatabase{tables})
}
