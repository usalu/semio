use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::text::snapshot as text;
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

const HOUSE: &str = include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/💡️inferences/🏠️house/📸️snapshot/🔣️.json");

fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("house decodes")
}

#[semio_framework_async_macros::async_test]
async fn the_artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, BIM_MODEL_DOCUMENT_SCHEMA);
    assert_eq!(ModelSnapshot::default().schema, BIM_MODEL_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn the_definition_assembles() {
    definition().expect("the capability rows are well formed");
}

#[semio_framework_async_macros::async_test]
async fn the_committed_house_is_canonical_json() {
    let reencoded: serde_json::Value = serde_json::from_str(&to_json_string(&house())).expect("JSON");
    let committed: serde_json::Value = serde_json::from_str(HOUSE).expect("JSON");
    let numbers = |value: serde_json::Value| value.to_string().replace(".0,", ",").replace(".0}", "}").replace(".0]", "]");
    assert_eq!(reencoded.as_object().expect("object").keys().count(), 26, "schema, project and the 24 collections");
    for (key, value) in committed.as_object().expect("object") {
        assert_eq!(numbers(reencoded[key].clone()), numbers(value.clone()), "{key}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_pack_and_the_dsl_text_round_trip_the_house() {
    let model = house();
    assert_eq!(pack::decode(&pack::encode(&model)).expect("pack decodes"), model);
    let printed = text::print_dsl(&model);
    assert_eq!(text::parse_dsl(&printed).expect("dsl parses"), model);
    assert_eq!(text::print_dsl(&text::parse_dsl(&printed).expect("dsl parses")), printed, "printing is a fixed point");
}

#[semio_framework_async_macros::async_test]
async fn every_data_enum_and_optional_field_survives_the_codecs() {
    let mut model = house();
    model.openings.insert("o-1".into(), Opening { host: "w-ground-south".into(), kind: OpeningKind::Door { door_type: "d-1".into() }, offset: 1.0, sill_override: None, width: Some(0.9), height: None, flip_hand: true, flip_facing: false, name: "Door".into() });
    model.roofs.insert("r-1".into(), Roof { storey: "st-roof".into(), roof_type: "rt".into(), footprint: vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 4.0, y: 0.0 }, bulge: 0.25 }, Vertex { point: Point2 { x: 4.0, y: 3.0 }, bulge: 0.0 }], shape: RoofShape::Mansard { lower_pitch: 1.2, upper_pitch: 0.4, break_height: 1.0 }, overhang: 0.3, base_offset: 0.0, name: "Roof".into() });
    model.stairs.insert("s-1".into(), Stair { storey: "st-ground".into(), start: Point2 { x: 1.0, y: 1.0 }, direction: 0.5, width: 1.1, flight: StairFlight::LTurn { split: 0.5, turn: Turn::Left }, top: TopConstraint::StoreyTop { offset: 0.0 }, max_riser: 0.18, min_tread: 0.27, stringer: crate::STANDARD_STRINGER, nosing: 0.0, tread_thickness: crate::STANDARD_TREAD_THICKNESS, riser: crate::STANDARD_RISER, landing_depth: 1.1, name: "Stair".into() });
    model.columns.insert("c-1".into(), Column { storey: "st-ground".into(), column_type: "ct".into(), position: Point2 { x: 2.0, y: 2.0 }, rotation: 0.0, base_offset: 0.0, top: TopConstraint::Unconnected { height: 2.5 }, name: "Column".into() });
    model.column_types.insert("ct".into(), ColumnType { name: "Round".into(), profile: Profile::Circle { diameter: 0.3 }, material: "m-brick".into() });
    model.slabs.insert("sl-1".into(), Slab { storey: "st-ground".into(), slab_type: "st".into(), boundary: vec![Vertex { point: Point2 { x: 0.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 5.0, y: 0.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 5.0, y: 5.0 }, bulge: 0.0 }], holes: vec![vec![Vertex { point: Point2 { x: 1.0, y: 1.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 2.0, y: 1.0 }, bulge: 0.0 }, Vertex { point: Point2 { x: 2.0, y: 2.0 }, bulge: 0.0 }]], offset: -0.2, slope: Some(Slope { direction: 0.0, angle: 0.05 }), name: "Slab".into() });
    model.spaces.insert("sp-1".into(), Space { storey: "st-ground".into(), number: "0.01".into(), name: "Hall".into(), boundary: SpaceBoundary::Bounded { seed: Point2 { x: 1.0, y: 1.0 } }, usage: "circulation".into() });
    model.properties.insert("w-ground-south".into(), PropertySet::from([("Pset_WallCommon".to_string(), [("FireRating".to_string(), PropertyValue::Text { value: "EI60".into() })].into())]));
    assert_eq!(pack::decode(&pack::encode(&model)).expect("pack decodes"), model);
    assert_eq!(text::parse_dsl(&text::print_dsl(&model)).expect("dsl parses"), model);
    assert_eq!(from_json_str::<ModelSnapshot>(&to_json_string(&model), JsonMemberPolicy::Reject).expect("json decodes"), model);
}
