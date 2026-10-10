use super::*;
use crate::standards::v1::subsets::any::schema::inferences::ModelInference;
use crate::ModelSnapshot;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const ROOM: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🪑️components/🏠️room/📸️snapshot/🔣️.json");

fn inferred() -> ModelInference {
    let snapshot: ModelSnapshot = from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the room decodes");
    ModelInference::infer(&snapshot).expect("infers")
}

fn close(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-9 * want.abs().max(1.0), "{got} vs {want}");
}

#[test]
fn the_solid_of_a_component_has_the_volume_of_its_visible_family_solids() {
    let inference = inferred();
    let table = &inference.element_solids["c-table"];
    assert_eq!(table.family, SolidFamily::Component);
    close(table.volume, 1.6 * 0.8 * 0.74);
    close(table.volume, inference.components["c-table"].volume);
    assert_eq!(table.groups.len(), 1);
    assert_eq!((table.groups[0].part.as_str(), table.groups[0].material.as_str()), ("body", "m-oak"));
    assert_eq!(table.storey, "st-ground");
    assert!(table.mesh().is_watertight());
}

#[test]
fn a_hidden_solid_of_the_family_is_not_part_of_the_instance() {
    let inference = inferred();
    close(inference.element_solids["c-chair-high"].volume, 0.45 * 0.45 * 0.05 + 0.45 * 0.05 * 0.45);
    assert_eq!(inference.element_solids["c-chair-high"].groups.len(), 1);
}

#[test]
fn the_solid_lies_where_the_value_says_the_component_stands() {
    let inference = inferred();
    let basin = &inference.element_solids["c-basin-south"];
    close(basin.bounds.min.x, 3.0);
    close(basin.bounds.max.x, 3.6);
    close(basin.bounds.min.y, 0.1);
    close(basin.bounds.max.y, 0.55);
    close(basin.bounds.min.z, 0.85);
    close(basin.bounds.max.z, 1.05);
    let north = &inference.element_solids["c-basin-north"];
    close(north.bounds.min.y, 3.45);
    close(north.bounds.max.y, 3.9);
    assert!(north.volume > 0.0, "a mirror reverses the winding of the faces");
    close(north.volume, 0.6 * 0.45 * 0.2);
    let value = &inference.components["c-basin-north"];
    close(value.bounds.min.x, north.bounds.min.x);
    close(value.bounds.max.y, north.bounds.max.y);
}

#[test]
fn a_rotated_component_is_tessellated_in_the_building_frame_and_its_value_box_holds_it() {
    let inference = inferred();
    let solid = &inference.element_solids["c-table"];
    let value = &inference.components["c-table"];
    assert!(value.bounds.min.x <= solid.bounds.min.x + 1e-9 && value.bounds.max.x >= solid.bounds.max.x - 1e-9);
    assert!(value.bounds.min.y <= solid.bounds.min.y + 1e-9 && value.bounds.max.y >= solid.bounds.max.y - 1e-9);
}

#[test]
fn a_component_without_geometry_has_an_empty_solid() {
    let inference = inferred();
    for id in ["c-ghost", "c-profile"] {
        assert!(inference.element_solids.get(id).is_none(), "{id}");
    }
}
