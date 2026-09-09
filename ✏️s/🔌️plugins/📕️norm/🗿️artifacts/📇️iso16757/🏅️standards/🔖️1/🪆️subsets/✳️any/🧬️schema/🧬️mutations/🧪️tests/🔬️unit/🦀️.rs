use super::*;
use crate::{part_1, part_4, part_5, Cardinality, LocalizedText, Names};
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &Iso16757Snapshot, operation: &Iso16757Mutation) -> Iso16757Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(base).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn change_exchange_process_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let mutation = Iso16757Mutation::ChangeExchangeProcess(change_exchange_process::mutation::ChangeExchangeProcess { new_exchange_process: part_5::ExchangeProcess::ProvideCatalogue });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.exchange_process, part_5::ExchangeProcess::ProvideCatalogue);
}

#[semio_framework_async_macros::async_test]
async fn update_script_limits_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let mutation = Iso16757Mutation::UpdateScriptLimits(update_script_limits::mutation::UpdateScriptLimits { new_max_steps: 1, new_max_recursion: 2, new_timeout_ms: 3 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.script_limits, part_5::ScriptLimits { max_steps: 1, max_recursion: 2, timeout_ms: 3 });
}

#[semio_framework_async_macros::async_test]
async fn change_and_remove_part_number_input_round_trip() {
    let base = Iso16757Snapshot::reference_fixture();
    let change = Iso16757Mutation::ChangePartNumberInput(change_part_number_input::mutation::ChangePartNumberInput { key: "new-key".into(), new_value: crate::CatalogueValue::Decimal { value: 7.0 } });
    let after_change = round_trip(&base, &change);
    assert_eq!(after_change.part_number_inputs.get("new-key"), Some(&crate::CatalogueValue::Decimal { value: 7.0 }));

    let remove = Iso16757Mutation::RemovePartNumberInput(remove_part_number_input::mutation::RemovePartNumberInput { key: "dn".into() });
    let after_remove = round_trip(&base, &remove);
    assert!(!after_remove.part_number_inputs.contains_key("dn"));
}

#[semio_framework_async_macros::async_test]
async fn change_part_number_input_undo_of_a_fresh_key_is_remove() {
    let base = Iso16757Snapshot::reference_fixture();
    let change = Iso16757Mutation::ChangePartNumberInput(change_part_number_input::mutation::ChangePartNumberInput { key: "fresh".into(), new_value: crate::CatalogueValue::Boolean { value: true } });
    let undo = change.inverse(&base);
    assert_eq!(undo, vec![Iso16757Mutation::RemovePartNumberInput(remove_part_number_input::mutation::RemovePartNumberInput { key: "fresh".into() })]);
}

#[semio_framework_async_macros::async_test]
async fn selection_class_and_constraints_round_trip() {
    let base = Iso16757Snapshot::reference_fixture();
    let change_class = Iso16757Mutation::ChangeSelectionClass(change_selection_class::mutation::ChangeSelectionClass { new_class_id: "class.other".into() });
    let after = round_trip(&base, &change_class);
    assert_eq!(after.selection.class_id, "class.other");

    let constraint = part_1::SelectionConstraint { property_id: "prop.other".into(), operator: part_1::ConstraintOperator::NotEqual, value: crate::CatalogueValue::Text { value: "x".into() } };
    let add = Iso16757Mutation::AddSelectionConstraint(add_selection_constraint::mutation::AddSelectionConstraint { constraint });
    let after_add = round_trip(&base, &add);
    assert_eq!(after_add.selection.constraints.len(), base.selection.constraints.len() + 1);

    let remove = Iso16757Mutation::RemoveSelectionConstraint(remove_selection_constraint::mutation::RemoveSelectionConstraint { index: 0 });
    let after_remove = round_trip(&base, &remove);
    assert_eq!(after_remove.selection.constraints.len(), base.selection.constraints.len() - 1);
}

#[semio_framework_async_macros::async_test]
async fn rename_catalogue_and_manufacturer_round_trip() {
    let base = Iso16757Snapshot::reference_fixture();
    let rename_catalogue = Iso16757Mutation::RenameCatalogue(rename_catalogue::mutation::RenameCatalogue { new_name: "Renamed Catalogue".into() });
    let after = round_trip(&base, &rename_catalogue);
    assert_eq!(after.catalogue.metadata.names.preferred.text, "Renamed Catalogue");

    let rename_manufacturer = Iso16757Mutation::RenameManufacturer(rename_manufacturer::mutation::RenameManufacturer { new_name: "Renamed Mfg".into() });
    let after = round_trip(&base, &rename_manufacturer);
    assert_eq!(after.catalogue.manufacturer.names.preferred.text, "Renamed Mfg");
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_product_group_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let product_group = part_1::ProductGroup { id: "group.new".into(), names: Names { preferred: LocalizedText { locale: "en".into(), text: "New Group".into() }, short_name: None, alternatives: Vec::new() }, dictionary_subject_id: None };
    let create = Iso16757Mutation::CreateProductGroup(create_product_group::mutation::CreateProductGroup { product_group: product_group.clone(), index: None });
    let after_create = round_trip(&base, &create);
    assert!(after_create.catalogue.product_groups.iter().any(|group| group.id == "group.new"));

    let undo = create.inverse(&base);
    assert_eq!(undo, vec![Iso16757Mutation::DeleteProductGroup(delete_product_group::mutation::DeleteProductGroup { id: "group.new".into() })]);

    let rename = Iso16757Mutation::RenameProductGroup(rename_product_group::mutation::RenameProductGroup { id: "group.new".into(), new_name: "Renamed".into() });
    let after_rename = round_trip(&after_create, &rename);
    assert_eq!(after_rename.catalogue.product_groups.iter().find(|group| group.id == "group.new").unwrap().names.preferred.text, "Renamed");

    let delete = Iso16757Mutation::DeleteProductGroup(delete_product_group::mutation::DeleteProductGroup { id: "group.valves".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.catalogue.product_groups.iter().any(|group| group.id == "group.valves"));
}

#[semio_framework_async_macros::async_test]
async fn delete_product_group_of_a_missing_id_has_an_empty_inverse() {
    let base = Iso16757Snapshot::reference_fixture();
    let delete = Iso16757Mutation::DeleteProductGroup(delete_product_group::mutation::DeleteProductGroup { id: "nope".into() });
    assert!(delete.inverse(&base).is_empty(), "deleting an absent id has nothing to undo");
}

#[semio_framework_async_macros::async_test]
async fn create_rename_delete_product_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let product = part_1::Product {
        id: "product.new".into(),
        series_id: "series.cv".into(),
        names: Names { preferred: LocalizedText { locale: "en".into(), text: "New Product".into() }, short_name: None, alternatives: Vec::new() },
        parameter_domains: Vec::new(),
        variants: Vec::new(),
        static_properties: Vec::new(),
    };
    let create = Iso16757Mutation::CreateProduct(create_product::mutation::CreateProduct { product: product.clone(), index: None });
    let after_create = round_trip(&base, &create);
    assert!(after_create.catalogue.products.iter().any(|p| p.id == "product.new"));

    let rename = Iso16757Mutation::RenameProduct(rename_product::mutation::RenameProduct { id: "product.new".into(), new_name: "Renamed Product".into() });
    let after_rename = round_trip(&after_create, &rename);
    assert_eq!(after_rename.catalogue.products.iter().find(|p| p.id == "product.new").unwrap().names.preferred.text, "Renamed Product");

    let delete = Iso16757Mutation::DeleteProduct(delete_product::mutation::DeleteProduct { id: "product.cv".into() });
    let after_delete = round_trip(&base, &delete);
    assert!(!after_delete.catalogue.products.iter().any(|p| p.id == "product.cv"));
    let undo = delete.inverse(&base);
    assert_eq!(undo.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_property_definition_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let definition = part_1::PropertyDefinition {
        id: "prop.new".into(),
        names: Names { preferred: LocalizedText { locale: "en".into(), text: "New Prop".into() }, short_name: None, alternatives: Vec::new() },
        data_type: "text".into(),
        unit: None,
        cardinality: Cardinality::optional(),
        kind: part_1::PropertyKind::Static,
        dictionary_property_id: None,
    };
    let create = Iso16757Mutation::CreatePropertyDefinition(create_property_definition::mutation::CreatePropertyDefinition { property_definition: definition, index: None });
    let after_create = round_trip(&base, &create);
    assert!(after_create.catalogue.property_definitions.iter().any(|d| d.id == "prop.new"));

    let delete = Iso16757Mutation::DeletePropertyDefinition(delete_property_definition::mutation::DeletePropertyDefinition { id: "prop.new".into() });
    let after_delete = round_trip(&after_create, &delete);
    assert!(!after_delete.catalogue.property_definitions.iter().any(|d| d.id == "prop.new"));
}

#[semio_framework_async_macros::async_test]
async fn create_then_delete_subject_round_trips() {
    let base = Iso16757Snapshot::reference_fixture();
    let subject = part_4::Subject {
        id: "subject.new".into(),
        kind: part_4::SubjectKind::ProductClass,
        names: Names { preferred: LocalizedText { locale: "en".into(), text: "New Subject".into() }, short_name: None, alternatives: Vec::new() },
        definition: LocalizedText { locale: "en".into(), text: "A new subject".into() },
        parent_id: None,
    };
    let create = Iso16757Mutation::CreateSubject(create_subject::mutation::CreateSubject { subject: subject.clone(), index: None });
    let after_create = round_trip(&base, &create);
    assert!(after_create.dictionary.subjects.iter().any(|s| s.id == "subject.new"));

    let delete = Iso16757Mutation::DeleteSubject(delete_subject::mutation::DeleteSubject { id: "subject.new".into() });
    let after_delete = round_trip(&after_create, &delete);
    assert!(!after_delete.dictionary.subjects.iter().any(|s| s.id == "subject.new"));

    let delete_missing = Iso16757Mutation::DeleteSubject(delete_subject::mutation::DeleteSubject { id: "nope".into() });
    assert!(delete_missing.inverse(&base).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn semantic_kinds_cover_every_variant() {
    assert_eq!(Iso16757Mutation::kinds().len(), 21);
    let mutation = Iso16757Mutation::RenameCatalogue(rename_catalogue::mutation::RenameCatalogue { new_name: "x".into() });
    assert_eq!(mutation.semantics().kind, "rename-catalogue");
    assert_eq!(mutation.semantics().record, "RenamedCatalogue");
}
