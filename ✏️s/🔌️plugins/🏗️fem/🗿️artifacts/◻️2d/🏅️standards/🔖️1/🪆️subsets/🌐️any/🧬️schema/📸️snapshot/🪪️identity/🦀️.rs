//! 🛂️ Borrowed record identity admission before native snapshot ownership is constructed.
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "FEM2d borrowed identity is absent, empty, duplicated, or exceeds shared address bounds") }

fn rows<'a>(shape: &Shape, value: &'a FieldValue, visit: &mut impl FnMut(&'a RecordValue, &RecordSpec) -> Result<(), ValueError>) -> Result<(), ValueError> {
    match (shape, value) {
        (Shape::Table(producer), FieldValue::List(items)) => {
            let spec = (producer.ordinary)();
            for item in items { let FieldValue::Record(row) = item else { return Err(invalid()) }; visit(row, &spec)?; }
        }
        (Shape::List(inner), FieldValue::List(items)) => { for item in items { rows(inner, item, visit)?; } }
        (Shape::Record(producer), FieldValue::Record(row)) => visit(row, &(producer.ordinary)())?,
        (Shape::Statements(variants), FieldValue::Statements(items)) => {
            for (key, row) in items {
                let (_, producer) = variants.iter().find(|(variant, _)| variant == key).ok_or_else(invalid)?;
                visit(row, &(producer.ordinary)())?;
            }
        }
        (Shape::Block(inner), FieldValue::Block(value)) => rows(inner, value, visit)?,
        (_, FieldValue::Absent) => {}
        _ => return Err(invalid()),
    }
    Ok(())
}

fn identity<'a>(row: &'a RecordValue, spec: &RecordSpec) -> Result<&'a str, ValueError> {
    let field = spec.fields.iter().find(|field| field.key == "id").ok_or_else(invalid)?;
    let Some(FieldValue::Text(id)) = row.get(field.id) else { return Err(invalid()) };
    Ok(id)
}

/// 🪪️ Meters public typed-snapshot semantic admission without replacing its original decode authority.
pub(super) fn validate_typed(snapshot: &crate::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let rows = snapshot.nodes.iter().map(|row| ("node", "", row.id.as_str()))
            .chain(snapshot.elements.iter().map(|row| ("element", "", crate::standards::v1::subsets::any::schema::structure::element_id(row))))
            .chain(snapshot.regions.iter().map(|row| ("region", "", row.id.as_str())))
            .chain(snapshot.materials.iter().map(|row| ("material", "", row.id.as_str())))
            .chain(snapshot.sections.iter().map(|row| ("section", "", row.id.as_str())))
            .chain(snapshot.supports.iter().map(|row| ("support", "", row.id.as_str())))
            .chain(snapshot.load_cases.iter().map(|row| ("loadCase", "", row.id.as_str())))
            .chain(snapshot.combinations.iter().map(|row| ("combination", "", row.id.as_str())))
            .chain(snapshot.load_cases.iter().flat_map(|case| case.loads.iter().map(move |load| ("load", case.id.as_str(), crate::standards::v1::subsets::any::schema::structure::load_id(load)))));
        let mut count = 0usize;
        for _ in rows.clone() { control.step()?; count = count.checked_add(1).ok_or_else(invalid)?; }
        let mut identities = control.allocate_vec::<(&str, &str, &str)>(count)?;
        for row @ (kind, owner, id) in rows { control.step()?; validate_segments(kind, owner, id)?; identities.push(row); }
        validate_sorted(&mut identities, control)
    })
}

fn validate_segments(kind: &str, owner: &str, id: &str) -> Result<(), ValueError> {
    if kind == "load" { protocol::wire::validate_scoped_identity_segments(&["load", owner, id]).map_err(|_| invalid())?; }
    else { protocol::wire::validate_scoped_identity_segments(&[kind, id]).map_err(|_| invalid())?; }
    Ok(())
}

fn validate_sorted(identities: &mut [(&str, &str, &str)], control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    control.checkpoint()?;
    identities.sort_unstable();
    control.checkpoint()?;
    for pair in identities.windows(2) { control.step()?; if pair[0] == pair[1] { return Err(invalid()); } }
    Ok(())
}


/// 🪪️ Checks exact typed namespaces and case-owned loads with admitted scratch and phase cancellation.
pub(super) fn validate(record: &RecordValue, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(0)?;
        let spec = crate::standards::v1::subsets::any::schema::snapshot::component::reference::Fem2dSnapshotValue::__dsl_spec();
        let mut count = 0usize;
        walk(record, &spec, &mut |_, _, _| {
            control.step()?;
            count = count.checked_add(1).ok_or_else(invalid)?;
            Ok(())
        })?;
        let mut identities = control.allocate_vec::<(&str, &str, &str)>(count)?;
        walk(record, &spec, &mut |kind, owner, id| {
            control.step()?;
            validate_segments(kind, owner, id)?;
            identities.push((kind, owner, id));
            Ok(())
        })?;
        validate_sorted(&mut identities, control)
    })
}

fn walk<'a>(record: &'a RecordValue, spec: &RecordSpec, visit: &mut impl FnMut(&'static str, &'a str, &'a str) -> Result<(), ValueError>) -> Result<(), ValueError> {
    for field in &spec.fields {
        let kind = match field.key.as_str() {
            "nodes" => "node", "elements" => "element", "regions" => "region", "materials" => "material", "sections" => "section", "supports" => "support", "load-cases" | "load_cases" | "loadCases" => "loadCase", "combinations" => "combination", _ => continue,
        };
        let value = record.get(field.id).ok_or_else(invalid)?;
        rows(&field.shape, value, &mut |row, spec| {
            let id = identity(row, spec)?;
            visit(kind, "", id)?;
            if kind == "loadCase" {
                let field = spec.fields.iter().find(|field| field.key == "loads").ok_or_else(invalid)?;
                let value = row.get(field.id).ok_or_else(invalid)?;
                rows(&field.shape, value, &mut |load, spec| visit("load", id, identity(load, spec)?))?;
            }
            Ok(())
        })?;
    }
    Ok(())
}
