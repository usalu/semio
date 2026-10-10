use super::*;
use protocol::Inference;
use crate::editor::bim::entities::{kind_of, kind_holding};
use crate::ModelInference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

fn model() -> ModelSnapshot {
    from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes")
}

fn write(kind: &str, key: &str, id: &str, value: &str) -> Option<ModelMutation> {
    let field = kind_of(kind).expect("kind").fields.iter().find(|field| field.key == key).expect("field");
    (field.write.expect("editable"))(&model(), id, value)
}

#[test]
fn a_family_is_a_library_entry_and_its_solids_are_its_children() {
    let (family, solid) = (kind_of("family").expect("family"), kind_of("family-solid").expect("family solid"));
    assert!(family.library && !solid.library);
    let snapshot = model();
    assert_eq!((family.name)(&snapshot, "fam-table").as_deref(), Some("Dining table"));
    assert_eq!((solid.parent)(&snapshot, "s-top").as_deref(), Some("fam-table"));
    assert!(kind_holding(&snapshot, "s-top").is_some_and(|row| row.kind == "family-solid"));
    assert_eq!((family.parent)(&snapshot, "fam-table"), None);
}

#[test]
fn the_family_fields_read_the_authored_values_and_write_exactly_one_sparse_leaf() {
    let snapshot = model();
    let family = kind_of("family").expect("family");
    let read = |key: &str| (family.fields.iter().find(|field| field.key == key).expect("field").read)(&snapshot, "fam-table");
    assert_eq!((read("name").as_deref(), read("category").as_deref()), (Some("Dining table"), Some("Furniture")));
    assert!(matches!(write("family", "category", "fam-table", "casework"), Some(ModelMutation::SetFamily(leaf)) if leaf.category == Some(FamilyCategory::Casework) && leaf.name.is_none()));
    assert!(matches!(write("family", "name", "fam-table", "  Desk "), Some(ModelMutation::SetFamily(leaf)) if leaf.name.as_deref() == Some("Desk")));
    assert!(write("family", "name", "fam-table", "  ").is_none() && write("family", "category", "fam-table", "kitchen").is_none());
}

#[test]
fn the_solid_formulas_are_written_canonical_and_text_that_does_not_parse_is_no_write() {
    let leaf = write("family-solid", "visible", "s-top", "width>1.4m");
    assert!(matches!(leaf, Some(ModelMutation::SetFamilySolid(leaf)) if leaf.visible.as_deref() == Some("width > 1.4 m") && leaf.name.is_none() && leaf.material.is_none()));
    assert!(write("family-solid", "material", "s-top", "\"m-steel\"").is_some());
    assert!(write("family-solid", "material", "s-top", "1 m +").is_none());
}

#[test]
fn what_a_family_shows_is_inferred_and_a_broken_family_counts_its_issues() {
    let snapshot = model();
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let read = |rows: &[InferredRow], key: &str, id: &str| (rows.iter().find(|row| row.key == key).expect("row").read)(&snapshot, &inference, id);
    assert!(read(FAMILY_INFERRED, "volume", "fam-table").and_then(|text| text.parse::<f64>().ok()).is_some_and(|volume| volume > 0.0));
    assert_eq!(read(FAMILY_INFERRED, "issues", "fam-table").as_deref(), Some("0"));
    assert!(read(FAMILY_INFERRED, "issues", "fam-broken").and_then(|text| text.parse::<usize>().ok()).is_some_and(|count| count > 0));
    assert!(read(FAMILY_SOLID_INFERRED, "volume", "s-top").is_some());
    assert_eq!(read(FAMILY_INFERRED, "volume", "fam-ghost"), None);
}

#[test]
fn a_new_family_takes_the_category_of_its_parent_and_a_new_solid_needs_its_family() {
    let snapshot = model();
    assert!(matches!(create_family(&snapshot, "fam-new", "Casework", "Desk"), Ok(ModelMutation::CreateFamily(leaf)) if leaf.family.category == FamilyCategory::Casework && leaf.family.name == "Desk"));
    assert!(matches!(create_family(&snapshot, "fam-new", "", "Desk"), Ok(ModelMutation::CreateFamily(leaf)) if leaf.family.category == FamilyCategory::Generic));
    assert_eq!(create_family_solid(&snapshot, "s-new", "fam-ghost", "Bench").err(), Some("bim.create.family-missing"));
    let created = create_family_solid(&snapshot, "s-new", "fam-table", "Bench").expect("creates");
    assert!(matches!(&created, ModelMutation::CreateFamilySolid(leaf) if leaf.solid.name == "Bench" && leaf.solid.family == "fam-table"));
    let applied = crate::mutations::apply_model_mutation(&snapshot, &created).expect("applies");
    assert!(applied.family_solids.contains_key("s-new"));
}
