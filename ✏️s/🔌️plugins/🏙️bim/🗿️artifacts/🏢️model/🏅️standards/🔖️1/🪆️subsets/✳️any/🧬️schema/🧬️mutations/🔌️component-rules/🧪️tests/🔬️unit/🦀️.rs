use super::*;
use crate::mutations::{
    align_elements::AlignElements, copy_elements::CopyElements, delete_elements::DeleteElements, delete_family::DeleteFamily, delete_storey::DeleteStorey, delete_wall::DeleteWall, mirror_elements::MirrorElements, move_elements::MoveElements, place_elements::PlaceElements,
    rename_element::RenameElement, rotate_elements::RotateElements, set_element_storey::SetElementStorey, set_family::SetFamily, split_wall::SplitWall,
};
use crate::mutations::elements::Placement;
use crate::{ComponentOverride, Family, FamilyParameter, MepSystem, ModelMutation, ParameterKind, Point2, Storey, Wall};
use protocol::{Mutation, OutcomeCode};
use semio_framework_diagnostic::Severity;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    from_json_str(text, JsonMemberPolicy::Reject).expect("a record")
}

fn wall(storey: &str) -> Wall {
    decode(&format!(r#"{{"storey":"{storey}","wall_type":"wt","axis":{{"Line":{{"start":{{"x":0,"y":0}},"end":{{"x":8,"y":0}}}}}},"location":"Center","base_offset":0,"top":{{"StoreyTop":{{"offset":0}}}},"phase":"New","name":"Wall"}}"#))
}

fn component(family: &str, x: f64, y: f64, name: &str) -> Component {
    Component { storey: "st".into(), family: family.into(), position: Point2 { x, y }, elevation: 0.0, rotation: 0.0, mirrored: false, host: None, system: None, name: name.into() }
}

fn run(storey: &str, system: MepSystem, shape: MepShape, path: &[(f64, f64, f64)], name: &str) -> MepElement {
    MepElement { storey: storey.into(), system, shape, path: path.iter().map(|(x, y, z)| Point3 { x: *x, y: *y, z: *z }).collect(), name: name.into() }
}

fn parameter(base: &mut ModelSnapshot, owner: &str, name: &str, value: &str) {
    base.family_parameters.insert(family_rules::parameter_key(owner, name), FamilyParameter { family: owner.into(), name: name.into(), kind: ParameterKind::Length, value: value.into() });
}

fn over(base: &mut ModelSnapshot, owner: &str, name: &str, value: &str) {
    base.component_overrides.insert(family_rules::parameter_key(owner, name), ComponentOverride { component: owner.into(), name: name.into(), value: value.into() });
}

fn base() -> ModelSnapshot {
    let mut base = ModelSnapshot::default();
    for (id, level) in [("st", 0), ("up", 1)] {
        base.storeys.insert(id.into(), Storey { building: "b".into(), name: id.into(), level, height: 3.0, cut_height: None });
    }
    base.buildings.insert("b".into(), decode(r#"{"site":"s","name":"House","origin":{"x":0,"y":0},"rotation":0,"elevation":0}"#));
    base.wall_types.insert("wt".into(), decode(r#"{"name":"Brick","layers":[]}"#));
    base.walls.insert("w".into(), wall("st"));
    base.walls.insert("w2".into(), wall("st"));
    base.walls.insert("wup".into(), wall("up"));
    base.families.insert("table".into(), Family { name: "Table".into(), category: FamilyCategory::Furniture });
    base.families.insert("basin".into(), Family { name: "Basin".into(), category: FamilyCategory::Plumbing });
    base.families.insert("hea".into(), Family { name: "HEA".into(), category: FamilyCategory::Profile });
    parameter(&mut base, "table", "width", "1.6 m");
    parameter(&mut base, "table", "depth", "0.8 m");
    parameter(&mut base, "table", "double", "2 * width");
    parameter(&mut base, "basin", "width", "0.6 m");
    let mut basin = component("basin", 3.0, 0.4, "Basin");
    basin.host = Some("w".into());
    basin.elevation = 0.85;
    base.components.insert("c-table".into(), component("table", 2.0, 3.0, "Table"));
    base.components.insert("c-basin".into(), basin);
    over(&mut base, "c-table", "width", "2 m");
    over(&mut base, "c-basin", "width", "0.5 m");
    base.mep_elements.insert("m-duct".into(), run("st", MepSystem::Supply, MepShape::Duct { width: 0.3, height: 0.2 }, &[(1.0, 1.0, 2.5), (5.0, 1.0, 2.5)], "Duct"));
    base
}

fn messages(mutation: &ModelMutation, base: &ModelSnapshot) -> (crate::ModelDiff, Vec<protocol::MutationMessage>) {
    mutation.diff(base).into_parts()
}

fn refusal(mutation: &ModelMutation, base: &ModelSnapshot) -> (String, Vec<String>) {
    let (diff, messages) = messages(mutation, base);
    assert_eq!(diff, crate::ModelDiff::default(), "a refusal writes nothing");
    let hit = messages.iter().find(|message| matches!(message.level, Severity::Warning | Severity::Error | Severity::Fatal)).expect("a refusal");
    (hit.code.0.to_string(), hit.target.clone())
}

fn applied(mutation: &ModelMutation, base: &ModelSnapshot) -> ModelSnapshot {
    let (diff, messages) = messages(mutation, base);
    assert!(!messages.iter().any(|message| matches!(message.level, Severity::Error | Severity::Fatal)), "{messages:?}");
    protocol::apply_diff(&diff, base).expect("the diff applies")
}

fn undone(mutation: &ModelMutation, base: &ModelSnapshot) -> ModelSnapshot {
    let mut state = applied(mutation, base);
    for undo in mutation.inverse(base).expect("an inverse").iter().rev() {
        state = applied(undo, &state);
    }
    state
}

fn roundtrip(mutation: ModelMutation, base: &ModelSnapshot) -> ModelSnapshot {
    let after = applied(&mutation, base);
    assert_eq!(&undone(&mutation, base), base, "the inverse restores the base");
    after
}

#[test]
fn a_component_needs_its_storey_a_family_that_is_no_profile_finite_numbers_and_a_wall_of_its_storey() {
    let base = base();
    assert!(component_fault(&base, &component("table", 1.0, 1.0, "T")).is_none());
    let fault = |component: Component| component_fault(&base, &component).map(|fault| (fault.code, fault.field));
    assert_eq!(fault(Component { storey: "gone".into(), ..component("table", 1.0, 1.0, "T") }), Some((OutcomeCode::TargetMissing, "storey".into())));
    assert_eq!(fault(component("gone", 1.0, 1.0, "T")), Some((OutcomeCode::TargetMissing, "family".into())));
    assert_eq!(fault(component("hea", 1.0, 1.0, "T")), Some((OutcomeCode::Invariant, "family".into())));
    assert_eq!(fault(component("table", f64::NAN, 1.0, "T")), Some((OutcomeCode::Invariant, "position".into())));
    assert_eq!(fault(Component { elevation: f64::INFINITY, ..component("table", 1.0, 1.0, "T") }), Some((OutcomeCode::Invariant, "elevation".into())));
    assert_eq!(fault(Component { rotation: f64::NAN, ..component("table", 1.0, 1.0, "T") }), Some((OutcomeCode::Invariant, "rotation".into())));
    assert_eq!(fault(Component { host: Some("w".into()), ..component("basin", 1.0, 1.0, "T") }), None);
    assert_eq!(fault(Component { host: Some("ghost".into()), ..component("basin", 1.0, 1.0, "T") }), Some((OutcomeCode::TargetMissing, "host".into())));
    assert_eq!(fault(Component { host: Some("st".into()), ..component("basin", 1.0, 1.0, "T") }), Some((OutcomeCode::Invariant, "host".into())));
    assert_eq!(fault(Component { host: Some("wup".into()), ..component("basin", 1.0, 1.0, "T") }), Some((OutcomeCode::Invariant, "host".into())));
}

#[test]
fn an_mep_element_needs_a_storey_positive_sections_and_a_path_of_distinct_finite_points() {
    let base = base();
    let duct = MepShape::Duct { width: 0.3, height: 0.2 };
    let good = run("st", MepSystem::Supply, duct.clone(), &[(0.0, 0.0, 2.0), (1.0, 0.0, 2.0)], "R");
    assert!(mep_fault(&base, &good).is_none());
    let field = |mep: MepElement| mep_fault(&base, &mep).map(|fault| fault.field);
    assert_eq!(field(MepElement { storey: "gone".into(), ..good.clone() }), Some("storey".into()));
    for shape in [MepShape::Duct { width: 0.0, height: 0.2 }, MepShape::Tray { width: 0.3, height: -1.0 }, MepShape::Pipe { diameter: 0.0 }, MepShape::Pipe { diameter: f64::NAN }] {
        assert_eq!(field(MepElement { shape, ..good.clone() }), Some("shape".into()));
    }
    for path in [vec![(0.0, 0.0, 0.0)], vec![(0.0, 0.0, 0.0), (0.0, 0.0, 0.0)], vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 0.0)], vec![(0.0, 0.0, 0.0), (f64::NAN, 0.0, 0.0)]] {
        assert_eq!(field(run("st", MepSystem::Gas, duct.clone(), &path, "R")), Some("path".into()), "{path:?}");
    }
    assert!(mep_fault(&base, &run("st", MepSystem::Gas, duct, &[(0.0, 0.0, 0.0), (0.0, 0.0, 2.0)], "Riser")).is_none(), "a vertical run is a run");
}

#[test]
fn an_override_needs_a_parameter_a_formula_of_the_family_and_no_circle_under_the_overrides() {
    let mut base = base();
    let fault = |base: &ModelSnapshot, name: &str, text: &str| override_fault(base, "c-table", name, text).map(|fault| (fault.code, fault.field));
    assert_eq!(fault(&base, "depth", "0.9 m"), None);
    assert_eq!(override_fault(&base, "c-ghost", "depth", "1 m").map(|fault| fault.field), Some("component".into()));
    assert_eq!(fault(&base, "height", "1 m"), Some((OutcomeCode::TargetMissing, "name".into())));
    assert_eq!(fault(&base, "depth", "2 *"), Some((OutcomeCode::Invariant, "value".into())));
    assert_eq!(fault(&base, "depth", "ghost"), Some((OutcomeCode::TargetMissing, "value".into())));
    assert_eq!(fault(&base, "width", "double"), Some((OutcomeCode::Invariant, "value".into())), "double is two times width");
    over(&mut base, "c-table", "double", "depth + 1 m");
    assert_eq!(fault(&base, "width", "double"), None, "the override of double no longer refers to width");
    assert_eq!(fault(&base, "depth", "double"), Some((OutcomeCode::Invariant, "value".into())), "depth would depend on double, which depends on depth");
}

#[test]
fn a_component_changes_its_family_only_with_overrides_the_new_family_can_evaluate() {
    let base = base();
    assert!(family_swap_fault(&base, "c-basin", "table").is_none(), "the basin override of width exists in the table family");
    let fault = family_swap_fault(&base, "c-table", "basin");
    assert!(fault.is_none(), "basin has a width too");
    let mut lone = base.clone();
    lone.families.insert("lamp".into(), Family { name: "Lamp".into(), category: FamilyCategory::Lighting });
    parameter(&mut lone, "lamp", "size", "0.3 m");
    assert_eq!(family_swap_fault(&lone, "c-table", "lamp").map(|fault| fault.field), Some("family".into()));
    over(&mut lone, "c-table", "depth", "double");
    parameter(&mut lone, "basin", "depth", "double");
    parameter(&mut lone, "basin", "double", "depth");
    assert_eq!(family_swap_fault(&lone, "c-table", "basin").map(|fault| fault.field), Some("family".into()), "depth and double would depend on each other in a circle");
}

#[test]
fn walls_know_the_components_mounted_on_them() {
    let base = base();
    assert_eq!(mounted_on(&base, "w").cloned().collect::<Vec<_>>(), vec!["c-basin".to_string()]);
    assert_eq!(mounted_on(&base, "w2").count(), 0);
}

#[test]
fn deleting_a_wall_takes_its_mounted_components_and_their_overrides_and_the_inverse_restores_them() {
    let base = base();
    let after = roundtrip(ModelMutation::DeleteWall(DeleteWall { id: "w".into() }), &base);
    assert!(!after.components.contains_key("c-basin") && after.components.contains_key("c-table"));
    assert!(!after.component_overrides.contains_key("c-basin.width") && after.component_overrides.contains_key("c-table.width"));
}

#[test]
fn deleting_a_storey_takes_its_components_mep_elements_and_overrides() {
    let base = base();
    let after = roundtrip(ModelMutation::DeleteStorey(DeleteStorey { id: "st".into() }), &base);
    assert!(after.components.is_empty() && after.mep_elements.is_empty() && after.component_overrides.is_empty());
}

#[test]
fn deleting_elements_takes_components_and_mep_elements_with_their_data() {
    let base = base();
    let after = roundtrip(ModelMutation::DeleteElements(DeleteElements { ids: vec!["c-table".into(), "m-duct".into()] }), &base);
    assert!(!after.components.contains_key("c-table") && after.mep_elements.is_empty() && !after.component_overrides.contains_key("c-table.width"));
}

#[test]
fn a_family_stays_while_a_component_places_it_and_cannot_become_a_profile() {
    let base = base();
    assert_eq!(refusal(&ModelMutation::DeleteFamily(DeleteFamily { id: "table".into() }), &base), ("mutation.target-referenced".into(), vec!["table".into()]));
    let mut free = base.clone();
    free.components.retain(|_, row| row.family != "table");
    free.component_overrides.retain(|_, row| row.component != "c-table");
    assert!(!free.components.is_empty());
    applied(&ModelMutation::DeleteFamily(DeleteFamily { id: "table".into() }), &free);
    let to_profile = ModelMutation::SetFamily(SetFamily { id: "table".into(), name: None, category: Some(FamilyCategory::Profile) });
    assert_eq!(refusal(&to_profile, &base), ("mutation.target-referenced".into(), vec!["table".into()]));
}

#[test]
fn a_wall_that_carries_a_component_is_not_split() {
    let base = base();
    let split = |id: &str| ModelMutation::SplitWall(SplitWall { id: id.into(), t: 0.5, new_id: "w-new".into() });
    assert_eq!(refusal(&split("w"), &base), ("mutation.target-referenced".into(), vec!["w".into()]));
    roundtrip(split("w2"), &base);
}

#[test]
fn moving_a_wall_to_another_storey_takes_its_mounted_components_along_and_a_mounted_component_stays() {
    let base = base();
    let after = roundtrip(ModelMutation::SetElementStorey(SetElementStorey { id: "w".into(), storey: "up".into() }), &base);
    assert_eq!(after.components["c-basin"].storey, "up");
    assert_eq!(refusal(&ModelMutation::SetElementStorey(SetElementStorey { id: "c-basin".into(), storey: "up".into() }), &base).0, "mutation.invariant");
    let after = roundtrip(ModelMutation::SetElementStorey(SetElementStorey { id: "c-table".into(), storey: "up".into() }), &base);
    assert_eq!((after.components["c-table"].storey.as_str(), after.components["c-table"].elevation), ("up", 0.0));
    let after = roundtrip(ModelMutation::SetElementStorey(SetElementStorey { id: "m-duct".into(), storey: "up".into() }), &base);
    assert_eq!(after.mep_elements["m-duct"].storey, "up");
}

#[test]
fn elements_are_named_and_found_by_the_shared_vocabulary() {
    let base = base();
    assert!(elements::exists(&base, "c-table") && elements::exists(&base, "m-duct") && !elements::exists(&base, "c-table.width"));
    assert!(elements::holds_data(&base, "c-basin") && elements::holds_data(&base, "m-duct"));
    assert_eq!(elements::taken(&base, "c-table"), Some("Component"));
    assert_eq!(elements::taken(&base, "c-table.width"), Some("Component override"));
    assert_eq!(elements::taken(&base, "m-duct"), Some("MEP element"));
    assert_eq!(elements::storey_of(&base, "c-table").as_deref(), Some("st"));
    let after = roundtrip(ModelMutation::RenameElement(RenameElement { id: "m-duct".into(), name: "Main".into() }), &base);
    assert_eq!(after.mep_elements["m-duct"].name, "Main");
    let after = roundtrip(ModelMutation::RenameElement(RenameElement { id: "c-table".into(), name: "Desk".into() }), &base);
    assert_eq!(after.components["c-table"].name, "Desk");
}

#[test]
fn moving_a_component_or_an_mep_element_moves_x_and_y_and_keeps_z() {
    let mut base = base();
    base.components.get_mut("c-table").expect("table").elevation = 0.4;
    let after = roundtrip(ModelMutation::MoveElements(MoveElements { ids: vec!["c-table".into(), "m-duct".into()], vector: Point2 { x: 1.0, y: -2.0 } }), &base);
    let table = &after.components["c-table"];
    assert_eq!((table.position, table.elevation), (Point2 { x: 3.0, y: 1.0 }, 0.4));
    assert_eq!(after.mep_elements["m-duct"].path, vec![Point3 { x: 2.0, y: -1.0, z: 2.5 }, Point3 { x: 6.0, y: -1.0, z: 2.5 }]);
}

#[test]
fn rotating_turns_a_free_component_and_leaves_the_rotation_of_a_mounted_one_to_its_wall() {
    let base = base();
    let quarter = std::f64::consts::FRAC_PI_2;
    let after = roundtrip(ModelMutation::RotateElements(RotateElements { ids: vec!["c-table".into(), "c-basin".into()], pivot: Point2 { x: 0.0, y: 0.0 }, angle: quarter }), &base);
    assert!((after.components["c-table"].rotation - quarter).abs() < 1e-12);
    assert_eq!(after.components["c-table"].position, Point2 { x: -3.0, y: 2.0 });
    assert_eq!(after.components["c-basin"].rotation, 0.0);
    assert_ne!(after.components["c-basin"].position, base.components["c-basin"].position);
}

#[test]
fn mirroring_reflects_the_family_frame_and_flips_the_mirror_flag() {
    let mut base = base();
    base.components.get_mut("c-table").expect("table").rotation = 0.3;
    let across_y = MirrorElements { ids: vec!["c-table".into(), "c-basin".into()], line_start: Point2 { x: 0.0, y: 0.0 }, line_end: Point2 { x: 0.0, y: 1.0 }, prefix: None };
    let after = roundtrip(ModelMutation::MirrorElements(across_y.clone()), &base);
    let table = &after.components["c-table"];
    assert!(table.mirrored && (table.rotation - (std::f64::consts::PI - 0.3 - std::f64::consts::PI)).abs() < 1e-9, "{table:?}");
    assert_eq!(table.position, Point2 { x: -2.0, y: 3.0 });
    let basin = &after.components["c-basin"];
    assert!(basin.mirrored && basin.rotation == 0.0 && basin.host.as_deref() == Some("w"));
    let again = applied(&ModelMutation::MirrorElements(MirrorElements { ids: vec!["c-table".into()], ..across_y }), &after);
    assert!(!again.components["c-table"].mirrored);
    assert!((again.components["c-table"].rotation - 0.3).abs() < 1e-9, "mirroring twice returns the frame");
}

#[test]
fn copies_carry_the_overrides_and_remount_on_a_copied_wall() {
    let base = base();
    let copy = |ids: &[&str]| ModelMutation::CopyElements(CopyElements { ids: ids.iter().map(|id| id.to_string()).collect(), vector: Point2 { x: 0.0, y: 5.0 }, prefix: "cp".into() });
    let alone = roundtrip(copy(&["c-table", "m-duct"]), &base);
    assert_eq!(alone.components.len(), base.components.len() + 1);
    assert_eq!(alone.mep_elements.len(), 2);
    let minted: Vec<&String> = alone.components.keys().filter(|id| id.starts_with("cp-")).collect();
    assert_eq!(minted.len(), 1);
    assert_eq!(alone.component_overrides[&format!("{}.width", minted[0])].value, "2 m");
    assert_eq!(alone.components[minted[0]].position, Point2 { x: 2.0, y: 8.0 });
    let with_wall = roundtrip(copy(&["c-basin", "w"]), &base);
    let basin = with_wall.components.iter().find(|(id, _)| id.starts_with("cp-")).map(|(_, row)| row).expect("copied basin");
    let wall = with_wall.walls.keys().find(|id| id.starts_with("cp-")).expect("copied wall");
    assert_eq!(basin.host.as_ref(), Some(wall));
    let beside = roundtrip(copy(&["c-basin"]), &base);
    assert_eq!(beside.components.iter().find(|(id, _)| id.starts_with("cp-")).and_then(|(_, row)| row.host.clone()).as_deref(), Some("w"), "a basin copied without its wall stays on the wall it hung on");
}

#[test]
fn the_mounting_flag_of_a_placement_is_read_from_the_record() {
    let base = base();
    let given = Placement::Component { position: Point2 { x: 2.0, y: 3.0 }, rotation: 0.0, mirrored: false, hosted: true };
    let place = ModelMutation::PlaceElements(PlaceElements { placements: [("c-table".to_string(), given)].into() });
    assert_eq!(refusal(&place, &base).0, "mutation.no-op");
    let moved = Placement::Component { position: Point2 { x: 9.0, y: 3.0 }, rotation: 0.0, mirrored: true, hosted: true };
    let after = roundtrip(ModelMutation::PlaceElements(PlaceElements { placements: [("c-table".to_string(), moved)].into() }), &base);
    assert_eq!((after.components["c-table"].position.x, after.components["c-table"].mirrored), (9.0, true));
}

#[test]
fn aligning_uses_the_origin_of_a_component_and_the_extent_of_a_path() {
    use crate::mutations::modify::{AlignAxis, AlignEdge};
    let base = base();
    let after = roundtrip(ModelMutation::AlignElements(AlignElements { ids: vec!["c-table".into(), "m-duct".into()], axis: AlignAxis::X, edge: AlignEdge::Min, target: 0.0 }), &base);
    assert_eq!(after.components["c-table"].position.x, 0.0);
    assert_eq!(after.mep_elements["m-duct"].path[0].x, 0.0);
}
