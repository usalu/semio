use super::super::{kind_of, kind_holding, ENTITIES};
use super::*;
use crate::standards::v1::subsets::any::editor::bim::terminology::BimLabels;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ZONING: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🏘️zones/🏡️zoning/📸️snapshot/🔣️.json");

fn zoning() -> ModelSnapshot {
    from_json_str(ZONING, JsonMemberPolicy::Reject).expect("the fixture decodes")
}

fn field(kind: &str, key: &str) -> &'static FieldRow {
    kind_of(kind).expect("the kind").fields.iter().find(|row| row.key == key).unwrap_or_else(|| panic!("no {key} row on {kind}"))
}

fn written(snapshot: &ModelSnapshot, kind: &str, key: &str, id: &str, value: &str) -> Option<ModelSnapshot> {
    let mutation = (field(kind, key).write.expect("editable"))(snapshot, id, value)?;
    crate::mutations::apply_model_mutation(snapshot, &mutation).ok()
}

#[semio_framework_async_macros::async_test]
async fn zones_and_area_schemes_are_listed_in_the_project_group_of_the_outliner_and_creatable() {
    let snapshot = zoning();
    for kind in ["zone", "area-scheme"] {
        let row = kind_of(kind).expect("the kind is declared");
        assert!(!row.library && !(row.ids)(&snapshot).is_empty(), "{kind} rows exist in the model");
        assert!(row.create.is_some() && row.delete.is_some() && row.rename.is_some(), "{kind} can be created, renamed and deleted");
    }
    assert!(kind_holding(&snapshot, "as-nsa").is_some_and(|row| row.kind == "area-scheme"));
    assert_eq!(ENTITIES.iter().filter(|row| row.kind == "zone" || row.kind == "area-scheme").count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn the_area_scheme_rows_read_the_rule_and_offer_what_the_model_can_be_counted_by() {
    let snapshot = zoning();
    let read = |key: &str| (field("area-scheme", key).read)(&snapshot, "as-nsa");
    assert_eq!((read("measure").as_deref(), read("usages").as_deref(), read("zones").as_deref()), (Some("Net"), Some("Living, Kitchen"), Some("")));
    assert_eq!(available_usages(&snapshot), "Kitchen, Living, Sleeping");
    assert_eq!(available_zones(&snapshot), "z-day (Day zone), z-night (Night zone)");
    let offered: Vec<&str> = kind_of("area-scheme").expect("the kind").inferred.iter().map(|row| row.key).collect();
    assert_eq!(offered, ["available_usages", "available_zones", "spaces", "resolved", "area", "volume", "occupancy"]);
}

#[semio_framework_async_macros::async_test]
async fn editing_the_usage_list_writes_a_distinct_trimmed_list_and_an_empty_text_counts_every_usage() {
    let snapshot = zoning();
    let edited = written(&snapshot, "area-scheme", "usages", "as-nsa", " Sleeping , Living,Sleeping,, ").expect("applies");
    assert_eq!(edited.area_schemes["as-nsa"].usages, ["Sleeping", "Living"]);
    let cleared = written(&snapshot, "area-scheme", "usages", "as-nsa", "").expect("applies");
    assert!(cleared.area_schemes["as-nsa"].usages.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn editing_the_included_zones_takes_ids_or_names_and_refuses_a_zone_that_does_not_exist() {
    let snapshot = zoning();
    let edited = written(&snapshot, "area-scheme", "zones", "as-gfa", "z-day, night zone, Z-DAY").expect("applies");
    assert_eq!(edited.area_schemes["as-gfa"].zones, ["z-day", "z-night"]);
    assert!(write_scheme_zones(&snapshot, "as-gfa", "z-day, z-ghost").is_none(), "one unknown token refuses the whole edit");
    assert!(written(&snapshot, "area-scheme", "zones", "as-gfa", "").is_some_and(|cleared| cleared.area_schemes["as-gfa"].zones.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn the_measure_is_a_localized_choice_and_a_space_picks_its_zone_and_finishes_from_the_model() {
    let snapshot = zoning();
    let labels = [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE];
    let measure = field("area-scheme", "measure").choices.expect("the measure is a choice");
    assert_eq!(measure(&snapshot, labels[0]).iter().map(|(_, label)| label.as_str()).collect::<Vec<_>>(), ["Gross", "Net"]);
    assert_eq!(measure(&snapshot, labels[1]).iter().map(|(_, label)| label.as_str()).collect::<Vec<_>>(), ["Brutto", "Netto"]);
    assert!(written(&snapshot, "area-scheme", "measure", "as-nsa", "Gross").is_some_and(|edited| edited.area_schemes["as-nsa"].measure == AreaMeasure::Gross));
    let zone = field("space", "zone").choices.expect("the zone is a choice");
    assert_eq!(zone(&snapshot, labels[0]), [("z-day".to_string(), "Day zone".to_string()), ("z-night".to_string(), "Night zone".to_string())]);
    for key in ["floor_finish", "wall_finish", "ceiling_finish"] {
        assert_eq!(field("space", key).choices.expect("a material choice")(&snapshot, labels[0]).len(), snapshot.materials.len(), "{key}");
    }
    assert!(written(&snapshot, "space", "zone", "sp-west", "z-night").is_some_and(|edited| edited.spaces["sp-west"].zone.as_deref() == Some("z-night")));
    assert!(written(&snapshot, "space", "zone", "sp-west", "").is_some_and(|edited| edited.spaces["sp-west"].zone.is_none()), "an empty text leaves the zone");
}

#[semio_framework_async_macros::async_test]
async fn every_zoning_row_has_an_english_and_a_german_label() {
    for kind in ["zone", "area-scheme", "space"] {
        let row = kind_of(kind).expect("the kind");
        for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
            assert!(!(row.label)(labels).as_str().trim().is_empty() && !(row.group)(labels).as_str().trim().is_empty(), "{kind}");
            for field in row.fields {
                assert!(!(field.label)(labels).as_str().trim().is_empty(), "{kind}.{}", field.key);
            }
            for inferred in row.inferred {
                assert!(!(inferred.label)(labels).as_str().trim().is_empty(), "{kind}.{}", inferred.key);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn a_new_area_scheme_counts_every_space_by_net_area_and_a_new_zone_has_no_occupancy() {
    let snapshot = zoning();
    let Ok(ModelMutation::CreateAreaScheme(created)) = create_area_scheme(&snapshot, "as-new", "", "Rentable") else { panic!("create area scheme") };
    assert_eq!((created.area_scheme.measure, created.area_scheme.usages.len(), created.area_scheme.zones.len()), (AreaMeasure::Net, 0, 0));
    let Ok(ModelMutation::CreateZone(created)) = create_zone(&snapshot, "z-new", "", "Annex") else { panic!("create zone") };
    assert_eq!((created.zone.category.as_str(), created.zone.occupancy_density), ("", 0.0));
}
