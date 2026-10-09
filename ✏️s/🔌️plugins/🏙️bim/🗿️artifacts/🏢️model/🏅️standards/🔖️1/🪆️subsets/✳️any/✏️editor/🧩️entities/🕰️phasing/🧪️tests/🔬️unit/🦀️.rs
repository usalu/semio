use super::*;
use crate::{Building, Point2, Storey};

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn the_four_phases_are_offered_in_project_order_in_both_languages() {
    let snapshot = demo();
    let english = phase_choices(&snapshot, &BimLabels::NATIVE_EN);
    assert_eq!(english, [("Existing", "Existing"), ("New", "New"), ("Demolished", "Demolished"), ("Temporary", "Temporary")].map(|(value, label)| (value.to_string(), label.to_string())));
    let german = phase_choices(&snapshot, &BimLabels::NATIVE_DE);
    assert_eq!(german.iter().map(|row| row.1.as_str()).collect::<Vec<_>>(), vec!["Bestand", "Neubau", "Abbruch", "Temporär"]);
    assert_eq!(german.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(), english.iter().map(|row| row.0.as_str()).collect::<Vec<_>>(), "the values are language neutral");
}

#[semio_framework_async_macros::async_test]
async fn every_offered_phase_value_writes_the_set_element_phase_mutation() {
    let snapshot = demo();
    for (value, _) in phase_choices(&snapshot, &BimLabels::NATIVE_EN) {
        let phase = Phase::parse(&value).expect("an offered value parses");
        assert_eq!(write_phase(&snapshot, "w-south", &value), Some(ModelMutation::SetElementPhase(SetElementPhase { id: "w-south".into(), phase })));
    }
    assert_eq!(write_phase(&snapshot, "w-south", "demolished"), write_phase(&snapshot, "w-south", " Demolished "), "case and padding do not matter");
    assert_eq!(write_phase(&snapshot, "w-south", "someday"), None);
    assert_eq!(write_phase(&snapshot, "w-south", ""), None);
}

#[semio_framework_async_macros::async_test]
async fn storeys_are_offered_by_building_then_level_and_name_the_building_only_when_there_are_several() {
    let mut snapshot = demo();
    let single = storey_choices(&snapshot, &BimLabels::NATIVE_EN);
    assert_eq!(single.len(), snapshot.storeys.len());
    let levels: Vec<i32> = single.iter().map(|(id, _)| snapshot.storeys[id].level).collect();
    assert!(levels.windows(2).all(|pair| pair[0] <= pair[1]), "by level within the building");
    assert!(single.iter().all(|(id, label)| *label == snapshot.storeys[id].name), "a single building is not named");
    let building = snapshot.buildings.values().next().expect("a building").clone();
    snapshot.buildings.insert("bldg-annex".into(), Building { name: "Annex".into(), origin: Point2 { x: 20.0, y: 0.0 }, ..building });
    snapshot.storeys.insert("st-annex".into(), Storey { building: "bldg-annex".into(), name: "Annex ground".into(), level: 0, height: 3.0 });
    let several = storey_choices(&snapshot, &BimLabels::NATIVE_EN);
    assert_eq!(several.len(), snapshot.storeys.len() );
    assert!(several.iter().all(|(_, label)| label.ends_with(')')), "{several:?}");
    assert!(several.iter().any(|(id, label)| id == "st-annex" && label == "Annex ground (Annex)"));
    assert_eq!(several, storey_choices(&snapshot, &BimLabels::NATIVE_DE), "storey names are authored, not localized");
}

#[semio_framework_async_macros::async_test]
async fn a_picked_storey_writes_the_set_element_storey_mutation_with_the_trimmed_id() {
    let snapshot = demo();
    assert_eq!(write_storey(&snapshot, "w-south", " st-first "), Some(ModelMutation::SetElementStorey(SetElementStorey { id: "w-south".into(), storey: "st-first".into() })));
    assert_eq!(write_storey(&snapshot, "w-south", "st-first"), write_storey(&snapshot, "w-south", "st-first"));
}
