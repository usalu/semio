use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

type Delta = KeyedDelta<Storey, StoreyPatch>;
type E = Entry<Storey, StoreyPatch>;

fn storey(name: &str, level: i32, height: f64) -> Storey {
    Storey { building: "bldg".into(), name: name.into(), level, height, cut_height: None }
}

fn heightened(height: f64) -> StoreyPatch {
    StoreyPatch { height: Some(height), ..Default::default() }
}

fn renamed(name: &str) -> StoreyPatch {
    StoreyPatch { name: Some(name.into()), ..Default::default() }
}

fn delta(entry: E) -> Delta {
    KeyedDelta::one("st", entry)
}

fn absorbed(earlier: E, later: E) -> Option<E> {
    let mut composed = delta(earlier).then(delta(later));
    composed.0.remove("st")
}

fn base() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.storeys.insert("st".into(), storey("Ground", 0, 3.0));
    snapshot.storeys.insert("up".into(), storey("First", 1, 2.8));
    snapshot
}

//#region 🔖️AbsorbTable
#[semio_framework_async_macros::async_test]
async fn absorb_table_composes_every_pair_of_entries() {
    let (a, b) = (storey("A", 0, 3.0), storey("B", 1, 2.5));
    let merged = StoreyPatch { name: Some("N".into()), height: Some(4.0), ..Default::default() };
    let table: Vec<(E, E, Option<E>)> = vec![
        (E::Created(a.clone()), E::Created(b.clone()), Some(E::Created(b.clone()))),
        (E::Created(a.clone()), E::Deleted, None),
        (E::Created(a.clone()), E::Replaced(b.clone()), Some(E::Created(b.clone()))),
        (E::Created(a.clone()), E::Patched(heightened(4.0)), Some(E::Created(Storey { height: 4.0, ..a.clone() }))),
        (E::Deleted, E::Created(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Deleted, E::Deleted, Some(E::Deleted)),
        (E::Deleted, E::Replaced(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Deleted, E::Patched(heightened(4.0)), Some(E::Deleted)),
        (E::Replaced(a.clone()), E::Created(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Replaced(a.clone()), E::Deleted, Some(E::Deleted)),
        (E::Replaced(a.clone()), E::Replaced(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Replaced(a.clone()), E::Patched(heightened(4.0)), Some(E::Replaced(Storey { height: 4.0, ..a.clone() }))),
        (E::Patched(renamed("N")), E::Created(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Patched(renamed("N")), E::Deleted, Some(E::Deleted)),
        (E::Patched(renamed("N")), E::Replaced(b.clone()), Some(E::Replaced(b.clone()))),
        (E::Patched(renamed("N")), E::Patched(heightened(4.0)), Some(E::Patched(merged))),
    ];
    for (earlier, later, expected) in table {
        assert_eq!(absorbed(earlier.clone(), later.clone()), expected, "{earlier:?} then {later:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn a_later_patch_field_wins_over_an_earlier_one() {
    assert_eq!(absorbed(E::Patched(heightened(3.5)), E::Patched(heightened(4.0))), Some(E::Patched(heightened(4.0))));
}

#[semio_framework_async_macros::async_test]
async fn absorb_is_associative_on_valid_three_chains() {
    let (a, b) = (storey("A", 0, 3.0), storey("B", 1, 2.5));
    let chains: Vec<[E; 3]> = vec![
        [E::Created(a.clone()), E::Patched(heightened(4.0)), E::Patched(renamed("N"))],
        [E::Created(a.clone()), E::Deleted, E::Created(b.clone())],
        [E::Patched(heightened(4.0)), E::Patched(renamed("N")), E::Deleted],
        [E::Deleted, E::Created(b.clone()), E::Patched(heightened(4.0))],
        [E::Replaced(a.clone()), E::Patched(heightened(4.0)), E::Patched(renamed("N"))],
        [E::Patched(heightened(4.0)), E::Deleted, E::Created(b.clone())],
    ];
    for [x, y, z] in chains {
        let left = delta(x.clone()).then(delta(y.clone())).then(delta(z.clone()));
        let grouped = delta(x.clone()).then(delta(y.clone()).then(delta(z.clone())));
        assert_eq!(left, grouped, "{x:?} then {y:?} then {z:?}");
    }
}
//#endregion 🔖️AbsorbTable

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_undoes_every_kind_of_entry() {
    let base = base();
    let diffs = [
        ModelDiff::storeys("st", Entry::Patched(StoreyPatch { height: Some(3.4), name: Some("Lower".into()), ..Default::default() })),
        ModelDiff::storeys("st", Entry::Deleted),
        ModelDiff::storeys("new", Entry::Created(storey("New", 2, 2.7))),
        ModelDiff::storeys("st", Entry::Replaced(storey("Swapped", 5, 3.3))),
        ModelDiff { storeys: Some(KeyedDelta([("st".to_string(), Entry::Deleted), ("up".to_string(), Entry::Patched(heightened(3.0))), ("new".to_string(), Entry::Created(storey("New", 2, 2.7)))].into())), ..ModelDiff::default() },
    ];
    for diff in &diffs {
        assert_diff_algebra_inverse_law::<ModelSnapshot, ModelDiff>(&base, diff).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn an_optional_field_is_restored_to_none_and_to_some() {
    let mut base = ModelSnapshot::default();
    base.openings.insert("o".into(), Opening { host: "w".into(), kind: OpeningKind::Void { width: 1.0, height: 2.0 }, offset: 1.0, sill_override: None, width: None, height: Some(2.1), flip_hand: false, flip_facing: false, reveal_depth: None, reveal_material: None, name: "O".into() });
    let diff = ModelDiff::openings("o", Entry::Patched(OpeningPatch { width: Some(Assigned::new(Some(0.9))), height: Some(Assigned::new(None)), ..Default::default() }));
    let after = protocol::apply_diff(&diff, &base).expect("applies");
    assert_eq!((after.openings["o"].width, after.openings["o"].height), (Some(0.9), None));
    assert_diff_algebra_inverse_law::<ModelSnapshot, ModelDiff>(&base, &diff).await;
}

#[semio_framework_async_macros::async_test]
async fn property_patches_set_replace_and_remove_and_invert() {
    let value = |text: &str| PropertyValue::Text { value: text.into() };
    let mut base = ModelSnapshot::default();
    base.properties.insert("w".into(), PropertySet::from([("Pset".to_string(), [("Fire".to_string(), value("EI60"))].into())]));
    let patch = PropertySetPatch { assigned: [("Pset".to_string(), [("Fire".to_string(), None), ("Acoustic".to_string(), Some(value("45 dB")))].into())].into() };
    let diff = ModelDiff::properties("w", Entry::Patched(patch));
    let after = protocol::apply_diff(&diff, &base).expect("applies");
    assert!(!after.properties["w"]["Pset"].contains_key("Fire") && after.properties["w"]["Pset"].contains_key("Acoustic"));
    assert_diff_algebra_inverse_law::<ModelSnapshot, ModelDiff>(&base, &diff).await;
}

//#endregion 🔖️InverseLaw

//#region 🔖️Apply
#[semio_framework_async_macros::async_test]
async fn apply_refuses_with_a_code_and_the_collection_and_id_as_target() {
    let base = base();
    let duplicate = protocol::apply_diff(&ModelDiff::storeys("st", Entry::Created(storey("Again", 9, 3.0))), &base).expect_err("created id exists");
    assert_eq!((duplicate.code.as_str(), duplicate.target), ("mutation.apply.duplicate-id", vec!["storeys".to_string(), "st".to_string()]));
    for entry in [E::Deleted, E::Replaced(storey("X", 1, 1.0)), E::Patched(heightened(3.0))] {
        let missing = protocol::apply_diff(&ModelDiff::storeys("ghost", entry), &base).expect_err("changed id is absent");
        assert_eq!((missing.code.as_str(), missing.target), ("mutation.apply.missing-target", vec!["storeys".to_string(), "ghost".to_string()]));
    }
}

#[semio_framework_async_macros::async_test]
async fn apply_is_all_or_nothing() {
    let base = base();
    let diff = ModelDiff { storeys: Some(KeyedDelta([("st".to_string(), E::Patched(heightened(3.4))), ("ghost".to_string(), E::Deleted)].into())), ..ModelDiff::default() };
    assert!(protocol::apply_diff(&diff, &base).is_err());
    assert_eq!(base.storeys["st"].height, 3.0, "the base is untouched by a refused diff");
}

#[semio_framework_async_macros::async_test]
async fn an_empty_diff_is_the_identity_and_reports_empty() {
    let base = base();
    let empty = ModelDiff::default();
    assert!(protocol::DiffAlgebra::<ModelSnapshot>::is_empty(&empty));
    assert_eq!(protocol::apply_diff(&empty, &base).expect("applies"), base);
}
//#endregion 🔖️Apply

//#region 🔖️Regions
#[semio_framework_async_macros::async_test]
async fn touches_names_collection_id_and_field() {
    let patched = ModelDiff::storeys("st", Entry::Patched(heightened(3.4))).touches();
    assert_eq!(patched.paths, vec!["storeys/st/height".to_string()]);
    assert!(patched.intersects_any(&["storeys"]) && !patched.intersects_any(&["materials"]));
    assert_eq!(ModelDiff::walls("w", Entry::Deleted).touches().paths, vec!["walls/w".to_string()]);
}
//#endregion 🔖️Regions

//#region 🔖️Wire
#[semio_framework_async_macros::async_test]
async fn the_diff_wire_form_is_sparse_and_round_trips() {
    let diff = ModelDiff::storeys("st", Entry::Patched(heightened(3.4)));
    let text = to_json_string(&diff);
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    assert_eq!(parsed, serde_json::json!({"storeys": {"st": {"entry": "Patched", "height": 3.4}}}));
    assert_eq!(from_json_str::<ModelDiff>(&text, JsonMemberPolicy::Reject).expect("decodes"), diff);
    let deleted = to_json_string(&ModelDiff::storeys("st", Entry::Deleted));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&deleted).expect("JSON"), serde_json::json!({"storeys": {"st": {"entry": "Deleted"}}}));
}
//#endregion 🔖️Wire

#[semio_framework_async_macros::async_test]
async fn a_minimal_patch_drops_exactly_the_fields_that_restate_the_base() {
    let base = storey("Ground", 0, 3.0);
    let both = StoreyPatch { name: Some("Ground".into()), height: Some(3.4), ..Default::default() };
    assert_eq!(both.minimal(&base), heightened(3.4));
    assert!(StoreyPatch { name: Some("Ground".into()), level: Some(0), height: Some(3.0), ..Default::default() }.minimal(&base).is_empty());
    assert!(StoreyPatch::default().minimal(&base).is_empty());
    let slab = Slab { phase: crate::Phase::New, storey: "st".into(), slab_type: "t".into(), boundary: Vec::new(), holes: Vec::new(), offset: 0.0, slope: None, name: "Slab".into() };
    assert!(SlabPatch { slope: Some(Assigned::new(None)), ..Default::default() }.minimal(&slab).is_empty());
    let sloped = SlabPatch { slope: Some(Assigned::new(Some(Slope { direction: 0.0, angle: 0.1 }))), ..Default::default() };
    assert_eq!(sloped.minimal(&slab), sloped);
}

#[semio_framework_async_macros::async_test]
async fn a_minimal_property_patch_drops_restated_and_absent_values() {
    let value = |text: &str| PropertyValue::Text { value: text.into() };
    let base = PropertySet::from([("Pset".to_string(), [("Fire".to_string(), value("EI60"))].into())]);
    let patch = PropertySetPatch { assigned: [("Pset".to_string(), [("Fire".to_string(), Some(value("EI60"))), ("Acoustic".to_string(), Some(value("45 dB"))), ("Gone".to_string(), None)].into())].into() };
    let minimal = patch.minimal(&base);
    assert_eq!(minimal.assigned["Pset"].keys().collect::<Vec<_>>(), ["Acoustic"]);
    assert!(PropertySetPatch { assigned: [("Pset".to_string(), [("Fire".to_string(), Some(value("EI60")))].into())].into() }.minimal(&base).is_empty());
}
