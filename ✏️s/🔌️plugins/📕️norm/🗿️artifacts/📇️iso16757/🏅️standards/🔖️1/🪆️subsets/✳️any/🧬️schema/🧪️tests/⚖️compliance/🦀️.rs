use super::*;
use crate::standards::v1::subsets::any::schema::component::part_5::ScriptRuntime;
use crate::Iso16757Snapshot;
use std::collections::BTreeMap;

#[semio_framework_async_macros::async_test]
async fn reference_fixture_selects_one_product() {
    let doc = Iso16757Snapshot::default();
    let selection = part_1::select_products(&doc.catalogue, &doc.selection);
    assert_eq!(selection.matches.len(), 1);
    assert!(!selection.ambiguity);
}

#[semio_framework_async_macros::async_test]
async fn geometry_bbox_volume_for_box_primitive() {
    let doc = Iso16757Snapshot::default();
    let geom = doc.geometry.objects.get("geom.valve.50").expect("geometry");
    let bbox = part_2::evaluate_bounding_box(geom.shape.as_ref().expect("shape"), &doc.geometry).expect("bbox");
    assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
}

#[semio_framework_async_macros::async_test]
async fn dictionary_controlled_values_filter_by_subject() {
    let doc = Iso16757Snapshot::default();
    let list = doc.dictionary.controlled_lists.first().expect("list");
    let allowed = part_4::filter_controlled_values(list, "subject.valve", &doc.dictionary);
    assert_eq!(allowed, vec!["50", "80", "100"]);
}

#[semio_framework_async_macros::async_test]
async fn part_number_script_is_deterministic() {
    let runtime = part_5::DefaultScriptRuntime;
    let rule = crate::part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() };
    let inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]);
    let part_no = part_5::calculate_part_number(&rule, &inputs, &runtime).expect("part number");
    assert_eq!(part_no, "550");
}

#[semio_framework_async_macros::async_test]
async fn ifc_step_export_contains_data_section() {
    let doc = Iso16757Snapshot::default();
    let ifc = part_5::build_ifc_catalogue(&doc.catalogue);
    let step = part_5::export_ifc_step(&ifc);
    assert!(step.contains("ENDSEC"));
    assert!(step.contains("IFCPRODUCT") || step.contains("IfcProduct"));
}

#[semio_framework_async_macros::async_test]
async fn composition_cycle_detected() {
    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.compositions.insert("product.a".into(), vec![crate::part_1::CompositionRelationship { component_product_id: "product.b".into(), quantity: 1 }]);
    doc.catalogue.compositions.insert("product.b".into(), vec![crate::part_1::CompositionRelationship { component_product_id: "product.a".into(), quantity: 1 }]);
    assert!(part_1::detect_composition_cycle(&doc.catalogue, "product.a"));
}

#[semio_framework_async_macros::async_test]
async fn script_rejects_forbidden_import() {
    let runtime = part_5::DefaultScriptRuntime;
    let err = runtime.execute("import fs", &HashMap::new(), crate::part_5::ScriptLimits::default()).unwrap_err();
    assert!(matches!(err, crate::part_5::ScriptError::InvalidExpression(_)));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_constraint_operators() {
    let dec = |v: f64| CatalogueValue::Decimal { value: v };
    let mk = |op, value| crate::part_1::SelectionConstraint { property_id: "p".into(), operator: op, value };
    assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::NotEqual, dec(6.0))));
    assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::NotEqual, dec(5.0))));
    assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::LessThan, dec(6.0))));
    assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::LessThan, dec(4.0))));
    assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::GreaterThan, dec(4.0))));
    assert!(!part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::GreaterThan, dec(6.0))));
    let range = CatalogueValue::Range { min: 1.0, max: 10.0, unit: None };
    assert!(part_1::evaluate_constraint(&dec(5.0), &mk(crate::part_1::ConstraintOperator::InRange, range.clone())));
    assert!(!part_1::evaluate_constraint(&dec(50.0), &mk(crate::part_1::ConstraintOperator::InRange, range)));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_constraint_type_mismatch_returns_false() {
    let constraint = crate::part_1::SelectionConstraint { property_id: "p".into(), operator: crate::part_1::ConstraintOperator::LessThan, value: CatalogueValue::Text { value: "x".into() } };
    assert!(!part_1::evaluate_constraint(&CatalogueValue::Decimal { value: 1.0 }, &constraint));
}

#[semio_framework_async_macros::async_test]
async fn select_products_filters_by_series_id() {
    let mut doc = Iso16757Snapshot::default();
    let other_series = crate::part_1::ProductSeries { id: "series.other".into(), class_id: "class.valve".into(), names: doc.catalogue.product_series[0].names.clone(), shared_property_values: BTreeMap::new(), geometry_id: None };
    let other_product = crate::part_1::Product {
        id: "product.other".into(),
        series_id: "series.other".into(),
        names: other_series.names.clone(),
        parameter_domains: Vec::new(),
        variants: vec![crate::part_1::ProductVariant {
            id: "variant.other".into(),
            parameter_values: BTreeMap::new(),
            property_values: vec![crate::part_1::PropertyValue { definition_id: "prop.dn".into(), value: CatalogueValue::Decimal { value: 50.0 }, function_id: None }],
            article_number: None,
            geometry_id: None,
        }],
        static_properties: Vec::new(),
    };
    doc.catalogue.product_series.push(other_series);
    doc.catalogue.products.push(other_product);
    doc.catalogue.product_indexes.push(crate::part_1::ProductIndex { id: "index.other".into(), product_id: "product.other".into(), variant_id: Some("variant.other".into()), search_tags: Vec::new() });
    let selection = part_1::select_products(&doc.catalogue, &doc.selection);
    assert_eq!(selection.matches.len(), 1);
    assert_eq!(selection.matches[0].id, "index.cv50");
}

#[semio_framework_async_macros::async_test]
async fn select_products_records_missing_property_and_constraint_failures() {
    let mut doc = Iso16757Snapshot::default();
    doc.selection.constraints.push(crate::part_1::SelectionConstraint { property_id: "prop.missing".into(), operator: crate::part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 1.0 } });
    let selection = part_1::select_products(&doc.catalogue, &doc.selection);
    assert!(selection.matches.is_empty());
    assert!(selection.explanations.iter().any(|e| e.contains("missing property")));

    doc.selection.constraints.clear();
    doc.selection.constraints.push(crate::part_1::SelectionConstraint { property_id: "prop.dn".into(), operator: crate::part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 999.0 } });
    let selection = part_1::select_products(&doc.catalogue, &doc.selection);
    assert!(selection.matches.is_empty());
    assert!(selection.explanations.iter().any(|e| e.contains("constraint failed")));
}

#[semio_framework_async_macros::async_test]
async fn select_products_flags_ambiguity_with_multiple_matches() {
    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.product_indexes.push(crate::part_1::ProductIndex { id: "index.cv50.dup".into(), product_id: "product.cv".into(), variant_id: Some("variant.50".into()), search_tags: Vec::new() });
    let selection = part_1::select_products(&doc.catalogue, &doc.selection);
    assert_eq!(selection.matches.len(), 2);
    assert!(selection.ambiguity);
}

#[semio_framework_async_macros::async_test]
async fn resolve_bim_embedding_error_paths() {
    let doc = Iso16757Snapshot::default();
    let unknown_index = part_1::resolve_bim_embedding(&doc.catalogue, "index.unknown", HashMap::new());
    assert!(matches!(unknown_index, Err(NormError::InvalidValue { field, .. }) if field == "index_id"));

    let mut catalogue_no_product = doc.catalogue.clone();
    catalogue_no_product.product_indexes[0].product_id = "product.unknown".into();
    let unknown_product = part_1::resolve_bim_embedding(&catalogue_no_product, "index.cv50", HashMap::new());
    assert!(matches!(unknown_product, Err(NormError::InvalidValue { field, .. }) if field == "product_id"));

    let mut catalogue_no_variant = doc.catalogue.clone();
    catalogue_no_variant.product_indexes[0].variant_id = None;
    let missing_variant = part_1::resolve_bim_embedding(&catalogue_no_variant, "index.cv50", HashMap::new());
    assert!(matches!(missing_variant, Err(NormError::IncompleteInput { field }) if field == "variant_id"));

    let out_of_domain = part_1::resolve_bim_embedding(&doc.catalogue, "index.cv50", HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 12345.0 })]));
    assert!(matches!(out_of_domain, Err(NormError::InvalidValue { field, .. }) if field == "dn"));
}

#[semio_framework_async_macros::async_test]
async fn resolve_bim_embedding_falls_back_to_series_geometry() {
    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.products[0].variants[0].geometry_id = None;
    let embedding = part_1::resolve_bim_embedding(&doc.catalogue, "index.cv50", HashMap::new()).expect("embedding");
    assert_eq!(embedding.resolved_geometry_id, doc.catalogue.product_series[0].geometry_id);
}

#[semio_framework_async_macros::async_test]
async fn validate_catalogue_structure_flags_issues() {
    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.products.clear();
    assert!(part_1::validate_catalogue_structure(&doc.catalogue).iter().any(|i| i.contains("no products")));

    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.products[0].series_id = "series.unknown".into();
    let issues = part_1::validate_catalogue_structure(&doc.catalogue);
    assert!(issues.iter().any(|i| i.contains("references unknown series")));

    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.property_definitions[0].id = String::new();
    let issues = part_1::validate_catalogue_structure(&doc.catalogue);
    assert!(issues.iter().any(|i| i.contains("empty property definition id")));

    let mut doc = Iso16757Snapshot::default();
    doc.catalogue.compositions.insert("product.cv".into(), vec![crate::part_1::CompositionRelationship { component_product_id: "product.cv".into(), quantity: 1 }]);
    let issues = part_1::validate_catalogue_structure(&doc.catalogue);
    assert!(issues.iter().any(|i| i.contains("composition cycle")));
}

#[semio_framework_async_macros::async_test]
async fn substitute_parameters_recurses_through_node_kinds() {
    let primitive = crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0)]) };
    let transform = crate::part_2::GeometryNode::Transform { translation: [1.0, 0.0, 0.0], rotation_deg: [0.0, 0.0, 0.0], child: Box::new(primitive.clone()) };
    let boolean = crate::part_2::GeometryNode::Boolean { operator: crate::part_2::BooleanOperator::Union, children: vec![primitive.clone(), transform.clone()] };
    let reference = crate::part_2::GeometryNode::Reference { geometry_id: "geom.x".into() };

    let values = HashMap::from([("width".into(), 2.0)]);
    match part_2::substitute_parameters(&primitive, &values) {
        crate::part_2::GeometryNode::Primitive { parameters, .. } => assert_eq!(parameters["width"], 2.0),
        _ => panic!("expected primitive"),
    }
    match part_2::substitute_parameters(&transform, &values) {
        crate::part_2::GeometryNode::Transform { child, .. } => match *child {
            crate::part_2::GeometryNode::Primitive { parameters, .. } => assert_eq!(parameters["width"], 2.0),
            _ => panic!("expected primitive child"),
        },
        _ => panic!("expected transform"),
    }
    match part_2::substitute_parameters(&boolean, &values) {
        crate::part_2::GeometryNode::Boolean { children, .. } => assert_eq!(children.len(), 2),
        _ => panic!("expected boolean"),
    }
    match part_2::substitute_parameters(&reference, &values) {
        crate::part_2::GeometryNode::Reference { geometry_id } => assert_eq!(geometry_id, "geom.x"),
        _ => panic!("expected reference"),
    }
}

#[semio_framework_async_macros::async_test]
async fn evaluate_bounding_box_error_paths() {
    let catalogue = crate::part_2::GeometryCatalogue::default();
    let missing_width = crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::new() };
    assert!(matches!(part_2::evaluate_bounding_box(&missing_width, &catalogue), Err(NormError::IncompleteInput { field }) if field == "width"));

    let unknown_kind = crate::part_2::GeometryNode::Primitive { kind: "cone".into(), parameters: BTreeMap::new() };
    assert!(matches!(part_2::evaluate_bounding_box(&unknown_kind, &catalogue), Err(NormError::OutOfScope { .. })));

    let empty_boolean = crate::part_2::GeometryNode::Boolean { operator: crate::part_2::BooleanOperator::Union, children: Vec::new() };
    assert!(matches!(part_2::evaluate_bounding_box(&empty_boolean, &catalogue), Err(NormError::IncompleteInput { field }) if field == "boolean_children"));

    let unresolved_ref = crate::part_2::GeometryNode::Reference { geometry_id: "missing".into() };
    assert!(matches!(part_2::evaluate_bounding_box(&unresolved_ref, &catalogue), Err(NormError::InvalidValue { field, .. }) if field == "geometry_id"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_bounding_box_cylinder_sphere_boolean_transform() {
    let catalogue = crate::part_2::GeometryCatalogue::default();
    let cylinder = crate::part_2::GeometryNode::Primitive { kind: "cylinder".into(), parameters: BTreeMap::from([("radius".into(), 1.0), ("height".into(), 2.0)]) };
    let bbox = part_2::evaluate_bounding_box(&cylinder, &catalogue).expect("cylinder bbox");
    assert_eq!(bbox.max, [2.0, 2.0, 2.0]);

    let sphere = crate::part_2::GeometryNode::Primitive { kind: "sphere".into(), parameters: BTreeMap::from([("radius".into(), 1.5)]) };
    let bbox = part_2::evaluate_bounding_box(&sphere, &catalogue).expect("sphere bbox");
    assert_eq!(bbox.max, [3.0, 3.0, 3.0]);

    let a = crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) };
    let b = crate::part_2::GeometryNode::Transform {
        translation: [5.0, 5.0, 5.0],
        rotation_deg: [0.0, 0.0, 0.0],
        child: Box::new(crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) }),
    };
    let boolean = crate::part_2::GeometryNode::Boolean { operator: crate::part_2::BooleanOperator::Union, children: vec![a, b] };
    let bbox = part_2::evaluate_bounding_box(&boolean, &catalogue).expect("boolean bbox");
    assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
    assert_eq!(bbox.max, [6.0, 6.0, 6.0]);

    let mut objects = BTreeMap::new();
    objects.insert(
        "geom.ref".to_string(),
        crate::part_2::GeometryObject {
            id: "geom.ref".into(),
            shape: Some(crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 1.0), ("height".into(), 1.0), ("depth".into(), 1.0)]) }),
            symbolic: None,
            spaces: Vec::new(),
            surfaces: Vec::new(),
            ports: Vec::new(),
            parameter_bindings: BTreeMap::new(),
        },
    );
    let ref_catalogue = crate::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
    let reference = crate::part_2::GeometryNode::Reference { geometry_id: "geom.ref".into() };
    let bbox = part_2::evaluate_bounding_box(&reference, &ref_catalogue).expect("resolved reference bbox");
    assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
}

#[semio_framework_async_macros::async_test]
async fn validate_geometry_graph_self_reference_and_cycle() {
    let mut objects = BTreeMap::new();
    let self_ref = crate::part_2::GeometryObject {
        id: "geom.self".into(),
        shape: Some(crate::part_2::GeometryNode::Reference { geometry_id: "geom.self".into() }),
        symbolic: None,
        spaces: Vec::new(),
        surfaces: Vec::new(),
        ports: Vec::new(),
        parameter_bindings: BTreeMap::new(),
    };
    objects.insert("geom.self".to_string(), self_ref.clone());
    let catalogue = crate::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
    let mut visited = HashSet::new();
    let issues = part_2::validate_geometry_graph(&self_ref, &catalogue, &mut visited);
    assert!(issues.iter().any(|i| i.contains("self-reference")));
    assert!(visited.is_empty());

    let mut objects = BTreeMap::new();
    let a = crate::part_2::GeometryObject {
        id: "geom.a".into(),
        shape: Some(crate::part_2::GeometryNode::Reference { geometry_id: "geom.b".into() }),
        symbolic: None,
        spaces: Vec::new(),
        surfaces: Vec::new(),
        ports: Vec::new(),
        parameter_bindings: BTreeMap::new(),
    };
    let b = crate::part_2::GeometryObject {
        id: "geom.b".into(),
        shape: Some(crate::part_2::GeometryNode::Reference { geometry_id: "geom.a".into() }),
        symbolic: None,
        spaces: Vec::new(),
        surfaces: Vec::new(),
        ports: Vec::new(),
        parameter_bindings: BTreeMap::new(),
    };
    objects.insert("geom.a".to_string(), a.clone());
    objects.insert("geom.b".to_string(), b);
    let catalogue = crate::part_2::GeometryCatalogue { objects, primitive_registry: Vec::new() };
    let mut visited = HashSet::new();
    let issues = part_2::validate_geometry_graph(&a, &catalogue, &mut visited);
    assert!(issues.iter().any(|i| i.contains("cycle in geometry reference")));
}

#[semio_framework_async_macros::async_test]
async fn validate_geometry_graph_empty_parameter_binding() {
    let object = crate::part_2::GeometryObject { id: "geom.bind".into(), shape: None, symbolic: None, spaces: Vec::new(), surfaces: Vec::new(), ports: Vec::new(), parameter_bindings: BTreeMap::from([("width".into(), String::new())]) };
    let catalogue = crate::part_2::GeometryCatalogue::default();
    let mut visited = HashSet::new();
    let issues = part_2::validate_geometry_graph(&object, &catalogue, &mut visited);
    assert!(issues.iter().any(|i| i.contains("empty parameter binding")));
}

#[semio_framework_async_macros::async_test]
async fn subtype_closure_is_transitive() {
    let dictionary = crate::part_4::Dictionary {
        reference: crate::DictionaryRef { id: "d".into(), version: "1".into() },
        subjects: Vec::new(),
        relationships: vec![
            crate::part_4::Relationship { id: "r1".into(), kind: crate::part_4::RelationshipKind::IsSubtypeOf, source_id: "a".into(), target_id: "b".into(), cardinality: crate::Cardinality::optional() },
            crate::part_4::Relationship { id: "r2".into(), kind: crate::part_4::RelationshipKind::IsSubtypeOf, source_id: "b".into(), target_id: "c".into(), cardinality: crate::Cardinality::optional() },
        ],
        properties: Vec::new(),
        controlled_lists: Vec::new(),
        meta_subjects: Vec::new(),
    };
    let closure = part_4::subtype_closure(&dictionary, "a");
    assert!(closure.contains("a") && closure.contains("b") && closure.contains("c"));
}

#[semio_framework_async_macros::async_test]
async fn detect_subtype_cycle_true() {
    let subject = |id: &str| crate::part_4::Subject {
        id: id.into(),
        kind: crate::part_4::SubjectKind::ProductClass,
        names: crate::Names { preferred: crate::LocalizedText { locale: "en".into(), text: id.into() }, short_name: None, alternatives: Vec::new() },
        definition: crate::LocalizedText { locale: "en".into(), text: String::new() },
        parent_id: None,
    };
    let dictionary = crate::part_4::Dictionary {
        reference: crate::DictionaryRef { id: "d".into(), version: "1".into() },
        subjects: vec![subject("a"), subject("b")],
        relationships: vec![
            crate::part_4::Relationship { id: "r1".into(), kind: crate::part_4::RelationshipKind::IsSubtypeOf, source_id: "a".into(), target_id: "b".into(), cardinality: crate::Cardinality::optional() },
            crate::part_4::Relationship { id: "r2".into(), kind: crate::part_4::RelationshipKind::IsSubtypeOf, source_id: "b".into(), target_id: "a".into(), cardinality: crate::Cardinality::optional() },
        ],
        properties: Vec::new(),
        controlled_lists: Vec::new(),
        meta_subjects: Vec::new(),
    };
    assert!(part_4::detect_subtype_cycle(&dictionary));
}

#[semio_framework_async_macros::async_test]
async fn resolve_property_found_and_missing() {
    let doc = Iso16757Snapshot::default();
    assert!(part_4::resolve_property(&doc.dictionary, "prop.dn").is_some());
    assert!(part_4::resolve_property(&doc.dictionary, "prop.unknown").is_none());
}

#[semio_framework_async_macros::async_test]
async fn validate_dictionary_flags_dangling_and_cardinality_review() {
    let mut doc = Iso16757Snapshot::default();
    doc.dictionary.relationships.push(crate::part_4::Relationship {
        id: "r.dangling".into(),
        kind: crate::part_4::RelationshipKind::IsDependentOn,
        source_id: "subject.valve".into(),
        target_id: "subject.unknown".into(),
        cardinality: crate::Cardinality::optional(),
    });
    doc.dictionary.relationships.push(crate::part_4::Relationship {
        id: "r.cardinality".into(),
        kind: crate::part_4::RelationshipKind::HasPart,
        source_id: "subject.valve".into(),
        target_id: "subject.valve".into(),
        cardinality: crate::Cardinality { min: 2, max: Some(3) },
    });
    let issues = part_4::validate_dictionary(&doc.dictionary);
    assert!(issues.iter().any(|i| i.contains("dangling endpoints")));
    assert!(issues.iter().any(|i| i.contains("cardinality review")));
}

#[semio_framework_async_macros::async_test]
async fn filter_controlled_values_context_rules() {
    let doc = Iso16757Snapshot::default();
    let mut empty_context_list = doc.dictionary.controlled_lists[0].clone();
    empty_context_list.context_subject_ids.clear();
    assert_eq!(part_4::filter_controlled_values(&empty_context_list, "anything", &doc.dictionary), empty_context_list.values);

    let mut unrelated_list = doc.dictionary.controlled_lists[0].clone();
    unrelated_list.context_subject_ids = vec!["subject.other".into()];
    assert!(part_4::filter_controlled_values(&unrelated_list, "subject.valve", &doc.dictionary).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn to_iso12006_mappings_basic() {
    let doc = Iso16757Snapshot::default();
    let mappings = part_4::to_iso12006_mappings(&doc.dictionary);
    assert_eq!(mappings.len(), doc.dictionary.subjects.len());
    assert_eq!(mappings[0].iso12006_uri, "iso12006://subject/subject.valve");
    assert_eq!(mappings[0].object_kind, "ProductClass");
}

#[semio_framework_async_macros::async_test]
async fn calculate_part_number_table_rule_paths() {
    let runtime = part_5::DefaultScriptRuntime;
    let rows = vec![BTreeMap::from([("dn".to_string(), "50".to_string()), ("code".to_string(), "CV50".to_string())])];
    let rule = crate::part_5::PartNumberRule::Table { rows, output_column: "code".into() };
    let inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]);
    assert_eq!(part_5::calculate_part_number(&rule, &inputs, &runtime).expect("match"), "CV50");

    let no_match_inputs = BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 999.0 })]);
    let err = part_5::calculate_part_number(&rule, &no_match_inputs, &runtime).unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "part_number_table"));

    let missing_output_rule = crate::part_5::PartNumberRule::Table { rows: vec![BTreeMap::from([("dn".to_string(), "50".to_string())])], output_column: "code".into() };
    let err = part_5::calculate_part_number(&missing_output_rule, &inputs, &runtime).unwrap_err();
    assert!(matches!(err, NormError::IncompleteInput { field } if field == "code"));

    let literal_rule = crate::part_5::PartNumberRule::Literal { value: "LIT-1".into() };
    assert_eq!(part_5::calculate_part_number(&literal_rule, &inputs, &runtime).expect("literal"), "LIT-1");
}

#[semio_framework_async_macros::async_test]
async fn script_runtime_timeout() {
    let runtime = part_5::DefaultScriptRuntime;
    let limits = crate::part_5::ScriptLimits { max_steps: 100, max_recursion: 10, timeout_ms: 0 };
    let err = runtime.execute("1 + 1", &HashMap::new(), limits).unwrap_err();
    assert!(matches!(err, crate::part_5::ScriptError::Timeout(0)));
}

#[semio_framework_async_macros::async_test]
async fn script_runtime_limit_errors() {
    let runtime = part_5::DefaultScriptRuntime;
    let recursion_limits = crate::part_5::ScriptLimits { max_steps: 1000, max_recursion: 2, timeout_ms: 5_000 };
    let err = runtime.execute("(((1)))", &HashMap::new(), recursion_limits).unwrap_err();
    assert!(matches!(err, crate::part_5::ScriptError::RecursionLimit(2)));

    let step_limits = crate::part_5::ScriptLimits { max_steps: 1, max_recursion: 100, timeout_ms: 5_000 };
    let err = runtime.execute("1+1", &HashMap::new(), step_limits).unwrap_err();
    assert!(matches!(err, crate::part_5::ScriptError::StepLimit(0)));

    let err = runtime.execute("unknownVar", &HashMap::new(), crate::part_5::ScriptLimits::default()).unwrap_err();
    assert!(matches!(err, crate::part_5::ScriptError::InvalidExpression(ref e) if e == "unknownVar"));
}

#[semio_framework_async_macros::async_test]
async fn script_runtime_arithmetic_operators() {
    let runtime = part_5::DefaultScriptRuntime;
    let inputs = HashMap::new();
    let result = runtime.execute("(10 - 4) / 2 * 3", &inputs, crate::part_5::ScriptLimits::default()).expect("arithmetic");
    assert!((result.value - 9.0).abs() < 1e-9);
}
