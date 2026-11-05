//! 🧪️ Inspector drafts follow the neutral variant and refusal laws through actual typed preparation.
use super::*;
use semio_framework::ActionArgOption;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🪺️inspector-forms/🔣️.json")).unwrap() }
fn fixture_value(value: &serde_json::Value) -> DslValue { DslValue::from(value) }
fn boundary() -> ActionArgDef {
    let selector = ActionArgDef::with_schema("/kind", LocalizedLabel::native("Kind", "Art"), ArgSchema::String {
        options: ["solid", "tube"].into_iter().map(|value| ActionArgOption { value: value.into(), label: LocalizedLabel::native(value, value) }).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None,
    }).required();
    let mut fields = vec![selector, ActionArgDef::text("/label", LocalizedLabel::native("Label", "Bezeichnung")).required()];
    for (name, group) in [("radius", "solid"), ("outerRadius", "tube"), ("wallThickness", "tube")] {
        let mut input = ActionArgDef::with_schema(format!("/{name}"), LocalizedLabel::native(name, name), ArgSchema::number(Some(0.0), None, None, false)).required();
        input.group = Some(group.into()); fields.push(input);
    }
    ActionArgDef::object("/shape", LocalizedLabel::native("Shape", "Form"), fields).required()
}
fn request(draft: Option<DslValue>, locale: Locale) -> StagedFormRequest {
    let mut args = vec![("path".into(), DslValue::String("shape.kind".into())), ("value".into(), DslValue::String("tube".into()))];
    if let Some(draft) = draft { args.push(("replacement".into(), draft)); }
    StagedFormRequest { token: StagedFormToken { action: "setField".into(), generation: "1".into(), revision: "r1".into(), window: "real-inputs".into() }, args: DslValue::Object(args), locale, page: None }
}
#[test]
fn variant_draft_removes_old_fields_without_inventing_required_values() {
    let fixture = fixture();
    let plan = NormInspectorFormPlan::variant(boundary(), &fixture_value(&fixture["variant"]["before"]), "kind", "tube", "shape".into()).unwrap();
    assert_eq!(plan.seed, fixture_value(&fixture["variant"]["expectedDraft"]));
    for locale in [Locale::En, Locale::De] {
        let request = request(None, locale);
        let response = plan.response(&request);
        let StagedFormField::Record { create, .. } = &response.fields[0] else { panic!("typed object form") };
        assert_eq!(create.as_ref(), Some(&plan.seed));
        assert!(!response.ready);
        assert!(plan.admitted_args(&request, |_| panic!("missing draft must not reach native admission")).is_err());
    }
}
#[test]
fn refusal_preserves_entered_values_and_complete_draft_uses_native_authority() {
    let fixture = fixture();
    let plan = NormInspectorFormPlan::variant(boundary(), &fixture_value(&fixture["variant"]["before"]), "kind", "tube", "shape".into()).unwrap();
    let incomplete = request(Some(plan.seed.clone()), Locale::En);
    assert!(plan.admitted_args(&incomplete, |_| panic!("incomplete draft must not reach native admission")).is_err());
    let request = request(Some(fixture_value(&fixture["variant"]["completed"])), Locale::De);
    let original = request.clone();
    assert_eq!(plan.admitted_args(&request, |_| Err("native domain refusal".into())).unwrap_err(), "native domain refusal");
    assert_eq!(request, original);
    let args = plan.admitted_args(&request, |_| Ok(())).unwrap();
    assert_eq!(args.get("path").and_then(DslValue::as_str), Some("shape"));
    assert_eq!(args.get("value"), Some(&fixture_value(&fixture["variant"]["completed"])));
    assert!(args.get("replacement").is_none());
}

#[test]
fn native_form_controls_create_a_draft_and_switch_the_complete_variant_boundary() {
    use semio_framework_plugin_decl_staged_form::*;
    let fixture = fixture();
    let plan = NormInspectorFormPlan::variant(boundary(), &fixture_value(&fixture["variant"]["before"]), "kind", "tube", "shape".into()).unwrap();
    for locale in [Locale::En, Locale::De] {
        let missing = request(None, locale);
        let status = plan.status(&missing, |_| panic!("missing draft cannot reach domain admission"));
        assert!(!status.ready && status.commit_args.is_none());
        let source = NormInspectorBodySource::prepare(&plan, &missing, status, "norm.mock").unwrap();
        assert_eq!(source.controls(), 1);
        let create = source.control(0).unwrap();
        assert!(matches!(create.scalar.kind, StagedScalarKind::Button { page: false }));
        assert!(create.scalar.label.contains(if locale == Locale::En { "Create draft" } else { "Entwurf anlegen" }));
        let Some(StagedBodyBinding::Action(binding)) = create.binding else { panic!("resident draft binding") };
        let StagedBindingEditView::Fixed(StagedBindingFixedView::Set { path, value: StagedBindingValueView::Value(value) }) = binding.edit(0).unwrap() else { panic!("draft creation only") };
        assert_eq!(path.len(), 1);
        assert_eq!(path.segment(0), Some("replacement"));
        assert_eq!(value, &plan.seed);
        let entered = request(Some(fixture_value(&fixture["variant"]["completed"])), locale);
        let status = plan.status(&entered, |_| Ok(()));
        assert!(status.ready && status.commit_args.is_some());
        let source = NormInspectorBodySource::prepare(&plan, &entered, status, "norm.mock").unwrap();
        assert_eq!(source.controls(), 4);
        let selector = source.control(0).unwrap();
        assert_eq!(selector.scalar.label, if locale == Locale::En { "Kind" } else { "Art" });
        let Some(StagedBodyBinding::Action(binding)) = selector.binding else { panic!("variant choice binding") };
        let StagedBindingEditView::Choice { path, options, .. } = binding.edit(0).unwrap() else { panic!("typed choice") };
        assert_eq!(path.len(), 1);
        assert_eq!(path.segment(0), Some("replacement"));
        let solid = (0..options.len()).filter_map(|index| options.option(index)).find(|option| option.key == "solid").unwrap();
        let StagedBindingValueView::Value(solid) = solid.value else { panic!("resident typed variant") };
        assert_eq!(solid.get("kind").and_then(DslValue::as_str), Some("solid"));
        assert!(solid.get("outerRadius").is_none() && solid.get("wallThickness").is_none());
        assert!(solid.get("radius").is_none());
    }
}

#[test]
fn native_long_choice_pages_keep_the_selected_value_and_exact_binding_options() {
    use semio_framework_plugin_decl_staged_form::*;
    let input = ActionArgDef::with_schema("/choice", LocalizedLabel::native("Choice", "Auswahl"), ArgSchema::String {
        options: (0..50).map(|index| ActionArgOption { value: format!("v{index}"), label: LocalizedLabel::native(&format!("Value {index}"), &format!("Wert {index}")) }).collect(), option_source: None, min_len: None, max_len: None, pattern: None, format: None,
    }).required();
    let plan = NormInspectorFormPlan { target: NormInspectorFormTarget::Insert { path: "items".into(), index: 0 }, input, seed: DslValue::String("v0".into()), draft_key: "value".into() };
    let mut request = request(None, Locale::De);
    request.token.action = "insertItem".into();
    request.args = DslValue::object([("value".into(), DslValue::String("v49".into()))]);
    let source = NormInspectorBodySource::prepare(&plan, &request, plan.status(&request, |_| Ok(())), "norm.mock").unwrap();
    assert_eq!(source.controls(), 2);
    let control = source.control(0).unwrap();
    let StagedScalarKind::Choice { value, choices } = control.scalar.kind else { panic!("choice control") };
    assert_eq!(value, "v49");
    assert_eq!(choices.len(), 32);
    assert!((0..choices.len()).any(|index| choices.choice(index).unwrap().key == "v49"));
    let Some(StagedBodyBinding::Action(binding)) = control.binding else { panic!("choice binding") };
    let StagedBindingEditView::Choice { options, .. } = binding.edit(0).unwrap() else { panic!("choice options") };
    assert_eq!(options.len(), choices.len());
    let next = source.control(1).unwrap();
    let Some(StagedBodyBinding::Page(binding)) = next.binding else { panic!("next choice page") };
    request.page = Some(binding.page.clone());
    let source = NormInspectorBodySource::prepare(&plan, &request, plan.status(&request, |_| Ok(())), "norm.mock").unwrap();
    let StagedScalarKind::Choice { choices, .. } = source.control(0).unwrap().scalar.kind else { panic!("second choice page") };
    assert_eq!(choices.len(), 19);
    assert_eq!(choices.choice(0).unwrap().key, "v31");
    assert_eq!(choices.choice(18).unwrap().key, "v49");
}

#[test]
fn insertion_seed_contains_only_one_declared_variant() {
    let seed = time_travel_default_value(&boundary().schema);
    assert_eq!(seed.get("kind").and_then(DslValue::as_str), Some("solid"));
    assert!(seed.get("radius").is_some());
    assert!(seed.get("outerRadius").is_none() && seed.get("wallThickness").is_none());
}

#[test]
fn layout_groups_preserve_required_inputs_without_a_variant_selector() {
    let mut field = ActionArgDef::text("/label", LocalizedLabel::native("Label", "Bezeichnung")).required();
    field.group = Some("presentation".into());
    let input = ActionArgDef::object("/item", LocalizedLabel::native("Item", "Eintrag"), vec![field]);
    let seed = norm_inspector_seed(&input.schema);
    assert!(seed.get("label").is_some());
    assert_eq!(coerce(&input, &seed), Some(seed));
}


#[test]
fn compiled_shape_seeds_match_the_existing_cold_oracle_for_every_neutral_input_schema(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json")).unwrap();
 fn compare_schema(schema:&ArgSchema){
  assert_eq!(semio_framework::arg_schema_shape_seed(schema),norm_inspector_seed(schema));
  match schema{ArgSchema::Object{fields}=>for input in fields{assert_eq!(semio_framework::action_arg_shape_seed(input),input.default.clone().unwrap_or_else(||norm_inspector_seed(&input.schema)));compare_schema(&input.schema)},ArgSchema::Array{items,..}=>compare_schema(items),_=>{}}
 }
 for case in corpus["cases"].as_array().unwrap(){
  let Some(inputs)=case.get("expectedInputs")else{continue};
  let inputs:Vec<ActionArgDef>=serde_json::from_value(inputs.clone()).unwrap();
  for input in inputs{compare_schema(&input.schema)}
 }
}

#[test]
fn compiled_singletons_match_the_independent_cold_add_oracle_without_replacing_array_defaults(){
 let law:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/🌱️shape-seeds/singleton-cases.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let schema:ArgSchema=serde_json::from_value(row["schema"].clone()).unwrap();
  let mut input=ActionArgDef::with_schema("/items",LocalizedLabel::native("Items","Einträge"),schema.clone());input.default=row.get("default").map(|value|serde_json::from_value(value.clone()).unwrap());
  let expected=match &schema{ArgSchema::Array{items,..}=>Some(DslValue::Array(vec![norm_inspector_seed(items)])),ArgSchema::Reference{many:true,..}=>{let mut item=schema.clone();if let ArgSchema::Reference{many,..}=&mut item{*many=false}Some(DslValue::Array(vec![norm_inspector_seed(&item)]))},_=>None};
  let rows=semio_framework::action_arg_shape_seeds(&[input.clone()]);
  assert_eq!(rows[0].value,input.default.clone().unwrap_or_else(||norm_inspector_seed(&input.schema)));assert_eq!(rows[0].singleton,expected);
  assert_eq!(serde_json::to_value(&rows[0].singleton).unwrap(),row["singleton"]);
 }
}
