fn reconstruct_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>, declared_schema: &str) -> Result<Self,ValueError> {

        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?; validate_sqlite_database_schema(database, declared_schema, control.limits())?;
        let document = database.table("semio_value_document")?.single_row()?; identity(document, 3)?; if document.rowid != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio value document requires identifier 1")); }
        let nodes = database.table("semio_value_node")?.ordered_rows(2)?; let mut names = BTreeMap::new(); let mut unique_names = BTreeSet::new(); let mut roots = vec![document.integer(2)?];
        for node in &nodes { identity(node, 5)?; if node.integer(1)? != 1 || !unique_names.insert(node.text(3)?) || names.insert(node.rowid, node.text(3)?).is_some() { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio value graph node")); } roots.push(node.integer(4)?); }
        let mut restored = Owned::new(reconstruct_value_forest(database, VALUE_TABLES, &roots, Some(&names), control)?);
        restored.get_mut().reverse();
        let schema=reconstruct_text(control,document.text(1)?)?;
        let root=restored.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio value document root"))?;
        let mut snapshot=Owned::new(Self{schema,root,nodes:Vec::new()});
        for node in nodes {let id=ValueId{value:reconstruct_text(control,node.text(3)?)?};let value=restored.get_mut().pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio graph node value"))?;snapshot.get_mut().nodes.push(SemioValueNode{id,value});}
        Ok(snapshot.take())
    }