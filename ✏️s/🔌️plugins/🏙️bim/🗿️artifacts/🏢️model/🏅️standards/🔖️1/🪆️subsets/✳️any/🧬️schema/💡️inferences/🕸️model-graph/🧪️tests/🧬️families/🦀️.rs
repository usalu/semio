use super::compute::take_hashed;
use super::*;
use protocol::Inference;
use crate::{Entry, FamilyParameterPatch, ModelDiff, Profile, ProjectPatch};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const FRAME: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧬️families/🏛️frame/📸️snapshot/🔣️.json");
const TABLE: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

fn frame() -> ModelSnapshot {
    from_json_str(FRAME, JsonMemberPolicy::Reject).expect("the frame decodes")
}

fn table() -> ModelSnapshot {
    from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes")
}

fn settle(session: &mut ModelInferenceSession, snapshot: &ModelSnapshot) {
    session.update(snapshot, &ModelDiff::default());
}

fn volume(inference: &ModelInference, id: &str) -> f64 {
    inference.element_solids[id].volume
}

fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-9 * want.abs().max(1.0), "{got} vs {want}");
}

const HEA_AREA: f64 = 2.0 * 0.2 * 0.01 + (0.19 - 2.0 * 0.01) * 0.0065;

#[test]
fn families_are_cache_transparent_warm_equals_cold_equals_uncached() {
    let snapshot = frame();
    let uncached = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(uncached.families.keys().collect::<Vec<_>>(), ["fam-hea", "fam-rhs", "fam-table"]);
    take_hashed();
    let mut session = ModelInferenceSession::new();
    let cold = session.refresh(&snapshot).clone();
    assert_eq!(session.report().computed_by_kind.get("family"), Some(&3), "one node per family: {:?}", session.report());
    assert_eq!(cold.families, uncached.families);
    let warm = session.refresh(&snapshot).clone();
    assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
    assert_eq!(warm, uncached);
    assert_eq!(cold, uncached);
}

#[test]
fn a_family_profile_is_what_the_solid_of_a_column_a_beam_and_a_railing_is_built_from() {
    let inference = ModelInference::infer(&frame()).expect("infers");
    close(volume(&inference, "col-hea"), HEA_AREA * 3.0);
    close(volume(&inference, "col-plain"), 0.2 * 0.2 * 3.0);
    let mut plain = frame();
    plain.beam_types.get_mut("bt-rhs").expect("type").profile = Profile::Rectangle { width: 0.1, depth: 0.06 };
    plain.railings.get_mut("rail-rhs").expect("railing").profile = Profile::Rectangle { width: 0.1, depth: 0.06 };
    let reference = ModelInference::infer(&plain).expect("infers");
    close(volume(&inference, "beam-rhs"), volume(&reference, "beam-rhs"));
    close(volume(&inference, "rail-rhs"), volume(&reference, "rail-rhs"));
    assert!(volume(&inference, "beam-rhs") > 0.0 && volume(&inference, "rail-rhs") > 0.0);
    let mut by_shape = frame();
    by_shape.column_types.get_mut("ct-hea").expect("type").profile = Profile::IShape { width: 0.2, depth: 0.19, web: 0.0065, flange: 0.01 };
    close(volume(&inference, "col-hea"), volume(&ModelInference::infer(&by_shape).expect("infers"), "col-hea"));
    close(inference.quantities.elements["col-hea"].gross_area, HEA_AREA);
}

#[test]
fn the_solid_follows_the_parameters_of_its_family_and_only_the_users_of_that_family_recompute() {
    let snapshot = frame();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let edit = ModelDiff::family_parameters("fam-hea.b", Entry::Patched(FamilyParameterPatch { value: Some("300 mm".into()), ..Default::default() }));
    let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
    let incremental = session.update(&after, &edit).clone();
    let report = session.report().clone();
    assert!(!report.gated, "{report:?}");
    assert_eq!(report.computed_by_kind.get("family"), Some(&1), "{report:?}");
    assert_eq!(report.computed_by_kind.get("solid"), Some(&1), "only the column that uses fam-hea: {report:?}");
    assert_eq!(report.computed_by_kind.get("wall-layout"), None);
    close(volume(&incremental, "col-hea"), (2.0 * 0.3 * 0.01 + (0.19 - 2.0 * 0.01) * 0.0065) * 3.0);
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
}

#[test]
fn an_edit_of_a_family_nothing_uses_recomputes_that_family_and_no_solid() {
    let snapshot = frame();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let edit = ModelDiff::family_parameters("fam-table.width", Entry::Patched(FamilyParameterPatch { value: Some("2 m".into()), ..Default::default() }));
    let after = protocol::apply_diff(&edit, &snapshot).expect("applies");
    let incremental = session.update(&after, &edit).clone();
    let report = session.report().clone();
    assert_eq!(report.computed_by_kind.get("family"), Some(&1), "{report:?}");
    assert_eq!(report.computed_by_kind.get("solid"), None, "{report:?}");
    assert_eq!(incremental, ModelInference::infer(&after).expect("infers"));
}

#[test]
fn a_diff_the_graph_does_not_read_is_gated_and_computes_no_family() {
    let snapshot = frame();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let before = session.inference().clone();
    let renamed = ModelDiff { project: Some(ProjectPatch { name: Some("Renamed".into()), ..Default::default() }), ..Default::default() };
    let after = protocol::apply_diff(&renamed, &snapshot).expect("applies");
    let served = session.update(&after, &renamed).clone();
    assert!(session.report().gated && session.report().computed == 0 && session.report().computed_by_kind.get("family").is_none(), "{:?}", session.report());
    assert_eq!(served, before);
}

#[test]
fn every_authored_input_of_a_family_recomputes_it_and_equals_a_fresh_inference() {
    let snapshot = frame();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    let mut current = snapshot;
    let edits: Vec<Box<dyn Fn(&mut ModelSnapshot)>> = vec![
        Box::new(|model| model.family_parameters.get_mut("fam-rhs.depth").expect("depth").value = "80 mm".into()),
        Box::new(|model| model.family_solids.get_mut("s-rhs").expect("solid").visible = "false".into()),
        Box::new(|model| model.family_solids.get_mut("s-hea").expect("solid").material = "\"m-brick\"".into()),
        Box::new(|model| model.families.get_mut("fam-rhs").expect("family").name = "Renamed".into()),
        Box::new(|model| {
            model.materials.remove("m-oak");
        }),
        Box::new(|model| {
            model.family_parameters.remove("fam-table.width");
            model.family_solids.remove("s-top");
        }),
    ];
    for (index, edit) in edits.iter().enumerate() {
        edit(&mut current);
        let incremental = session.refresh(&current).clone();
        assert_eq!(incremental, ModelInference::infer(&current).expect("infers"), "edit {index}");
    }
    assert!(session.inference().families["fam-table"].parameters.is_empty());
}

#[test]
fn deleting_the_family_a_type_names_empties_the_solids_and_raises_a_dangling_reference() {
    let mut snapshot = frame();
    let mut session = ModelInferenceSession::new();
    settle(&mut session, &snapshot);
    snapshot.families.remove("fam-hea");
    snapshot.family_parameters.retain(|_, row| row.family != "fam-hea");
    snapshot.family_solids.retain(|_, row| row.family != "fam-hea");
    let after = session.refresh(&snapshot).clone();
    assert!(!after.families.contains_key("fam-hea"));
    assert!(after.element_solids.get("col-hea").is_none_or(|solid| solid.is_empty()), "no outline, no solid");
    assert!(after.diagnostics.iter().any(|found| found.code.slug() == "reference.profile-family" && found.elements == ["ct-hea"] && found.missing == ["fam-hea"]), "{:?}", after.diagnostics);
    assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
}

#[test]
fn a_family_that_is_no_profile_is_a_dangling_profile_too() {
    let mut snapshot = frame();
    snapshot.column_types.get_mut("ct-hea").expect("type").profile = Profile::Family { family: "fam-table".into() };
    let inference = ModelInference::infer(&snapshot).expect("infers");
    assert!(inference.diagnostics.iter().any(|found| found.code.slug() == "reference.profile-family" && found.missing == ["fam-table"]));
    assert!(inference.element_solids.get("col-hea").is_none_or(|solid| solid.is_empty()), "a furniture family has no outline");
}

#[test]
fn every_kind_of_fault_of_a_family_is_a_diagnostic_naming_the_family_and_the_part() {
    let inference = ModelInference::infer(&table()).expect("infers");
    let slugs: std::collections::BTreeSet<&str> = inference.diagnostics.iter().map(|found| found.code.slug()).filter(|slug| slug.starts_with("family.")).collect();
    for want in ["family.syntax", "family.kind", "family.cycle", "family.unknown", "family.division-by-zero", "family.negative", "family.dependency"] {
        assert!(slugs.contains(want), "{want} missing in {slugs:?}");
    }
    let cycle = inference.diagnostics.iter().find(|found| found.code.slug() == "family.cycle" && found.elements == ["fam-broken", "loop_a"]).expect("the cycle names the family and the parameter");
    assert_eq!(cycle.missing, ["loop_a", "loop_b"]);
    assert!(inference.diagnostics.iter().all(|found| !found.elements.iter().any(|id| id == "fam-table")), "a sound family raises nothing");
    assert!(inference.families["fam-table"].issues.is_empty());
}

#[test]
fn the_text_projection_of_the_families_is_the_oracle_table() {
    let inference = ModelInference::infer(&table()).expect("infers");
    let text = crate::standards::v1::subsets::any::schema::inferences::families::metrics::table_json(&inference.families);
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    assert_eq!(parsed.as_object().expect("object").len(), 5);
    close(parsed["fam-hea"]["outline_area"].as_f64().expect("area"), HEA_AREA);
    assert_eq!(parsed["fam-table"]["solids"]["s-shelf"]["visible"], false);
    let projected = crate::standards::v1::subsets::any::io::text::snapshot::encode_inference_projection_json(&table(), "families").expect("the families projection exists");
    assert_eq!(projected, text);
}
