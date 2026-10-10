use super::*;
use crate::{Profile, TopConstraint};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use serde_json::json;

fn square() -> serde_json::Value {
    json!([{ "point": { "x": 0.0, "y": 0.0 }, "bulge": 0.0 }, { "point": { "x": 8.0, "y": 0.0 }, "bulge": 0.0 }, { "point": { "x": 8.0, "y": 6.0 }, "bulge": 0.0 }, { "point": { "x": 0.0, "y": 6.0 }, "bulge": 0.0 }])
}

fn house() -> ModelSnapshot {
    let wall = |name: &str| json!({ "storey": "st", "wall_type": "wt", "axis": { "Line": { "start": { "x": 0.0, "y": 0.0 }, "end": { "x": 8.0, "y": 0.0 } } }, "location": "Center", "base_offset": 0.0, "top": { "StoreyTop": { "offset": 0.0 } }, "phase": "New", "name": name });
    let document = json!({
        "schema": "s.bim.model@1",
        "project": { "name": "T", "description": "", "author": "", "organization": "", "phase_names": [] },
        "materials": { "m": { "name": "M", "category": "Masonry", "color": { "r": 0.5, "g": 0.5, "b": 0.5 }, "density": 1800.0, "conductivity": 0.8, "specific_heat": 900.0 } },
        "wall_types": { "wt": { "name": "W", "layers": [{ "material": "m", "thickness": 0.3, "function": "Structure" }] } },
        "slab_types": { "slt": { "name": "S", "layers": [{ "material": "m", "thickness": 0.2, "function": "Structure" }] } },
        "roof_types": { "rt": { "name": "R", "layers": [{ "material": "m", "thickness": 0.1, "function": "Finish" }] } },
        "sites": { "s": { "name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 0.0, "true_north": 0.0, "boundary": [] } },
        "buildings": { "b": { "site": "s", "name": "House", "origin": { "x": 0.0, "y": 0.0 }, "rotation": 0.0, "elevation": 0.0 }, "b2": { "site": "s", "name": "Annex", "origin": { "x": 20.0, "y": 0.0 }, "rotation": 0.0, "elevation": 0.0 } },
        "storeys": { "st": { "building": "b", "name": "Ground", "level": 0, "height": 3.0 }, "st2": { "building": "b2", "name": "Annex ground", "level": 0, "height": 3.0 } },
        "walls": { "w": wall("W") },
        "slabs": { "sl": { "storey": "st", "slab_type": "slt", "boundary": square(), "holes": [], "offset": 0.0, "phase": "New", "name": "" }, "sl2": { "storey": "st2", "slab_type": "slt", "boundary": square(), "holes": [], "offset": 0.0, "phase": "New", "name": "" } },
        "roofs": { "r": { "storey": "st", "roof_type": "rt", "footprint": square(), "shape": "Flat", "overhang": 0.0, "base_offset": 0.0, "phase": "New", "name": "" } }
    });
    from_json_str(&document.to_string(), JsonMemberPolicy::Reject).expect("the test model decodes")
}

fn sweep(extra: serde_json::Value) -> WallSweep {
    let mut row = json!({ "host": "w", "side": "Left", "profile": { "Rectangle": { "width": 0.02, "depth": 0.1 } }, "height": 0.0, "inset": 0.0, "material": "m", "name": "Baseboard" });
    row.as_object_mut().expect("a sweep object").extend(extra.as_object().expect("extra object").clone());
    from_json_str(&row.to_string(), JsonMemberPolicy::Reject).expect("the sweep decodes")
}

fn code(flaw: Option<Flaw>) -> Option<(OutcomeCode, Vec<String>)> {
    flaw.map(|flaw| (flaw.code, flaw.path))
}

#[test]
fn a_sweep_names_an_existing_host_and_material_and_a_sensible_section() {
    let base = house();
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({})))), None);
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "host": "w-gone" })))), Some((OutcomeCode::TargetMissing, vec!["host".to_string()])));
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "material": "m-gone" })))), Some((OutcomeCode::TargetMissing, vec!["material".to_string()])));
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "profile": { "Rectangle": { "width": 0.0, "depth": 0.1 } } })))), Some((OutcomeCode::Invariant, vec!["profile".to_string()])));
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "height": -0.01 })))), Some((OutcomeCode::Invariant, vec!["height".to_string()])));
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "inset": 0.02 })))), Some((OutcomeCode::Invariant, vec!["inset".to_string()])), "an inset as deep as the profile hides all of it");
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "inset": 0.01 })))), None);
    assert_eq!(code(sweep_flaw(&base, &sweep(json!({ "profile": { "Circle": { "diameter": 0.04 } }, "inset": 0.01 })))), None);
}

#[test]
fn an_attach_names_a_target_of_the_same_building_with_a_finite_offset() {
    let base = house();
    let roof = |id: &str, offset: f64| TopConstraint::Roof { roof: id.to_string(), offset };
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &roof("r", 0.0), None)), None);
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &roof("r-gone", 0.0), None)), Some((OutcomeCode::TargetMissing, vec!["top".to_string(), "roof".to_string()])));
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &roof("r", f64::NAN), None)), Some((OutcomeCode::Invariant, vec!["top".to_string(), "offset".to_string()])));
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &TopConstraint::Slab { slab: "sl2".into(), offset: 0.0 }, None)), Some((OutcomeCode::Invariant, vec!["top".to_string(), "slab".to_string()])), "a slab of the annex");
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &TopConstraint::Ceiling { ceiling: "ce".into(), offset: 0.0 }, None)), Some((OutcomeCode::TargetMissing, vec!["top".to_string(), "ceiling".to_string()])));
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &TopConstraint::StoreyTop { offset: 0.0 }, Some("sl"))), None);
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &TopConstraint::StoreyTop { offset: 0.0 }, Some("sl2"))), Some((OutcomeCode::Invariant, vec!["base_slab".to_string()])));
    assert_eq!(code(attach_flaw(&base, Some("w"), "st", &TopConstraint::StoreyTop { offset: 0.0 }, Some("sl-gone"))), Some((OutcomeCode::TargetMissing, vec!["base_slab".to_string()])));
    assert_eq!(code(attach_flaw(&base, None, "st", &TopConstraint::Unconnected { height: 2.4 }, None)), None, "no attach, nothing to judge");
}

#[test]
fn only_a_wall_attaches_the_top_of_an_element_to_a_surface() {
    let base = house();
    let flaw = crate::mutations::wall_geometry::top_flaw(&base, "st", &TopConstraint::Roof { roof: "r".into(), offset: 0.0 }).expect("a refusal");
    assert_eq!((flaw.code, flaw.path), (OutcomeCode::Invariant, vec!["top".to_string()]));
}

#[test]
fn a_reveal_has_a_non_negative_depth_and_an_existing_material() {
    let base = house();
    assert_eq!(code(reveal_flaw(&base, Some(0.1), Some("m"))), None);
    assert_eq!(code(reveal_flaw(&base, None, None)), None);
    assert_eq!(code(reveal_flaw(&base, Some(0.0), None)), None, "a reveal as shallow as the face is allowed");
    assert_eq!(code(reveal_flaw(&base, Some(-0.1), None)), Some((OutcomeCode::Invariant, vec!["reveal_depth".to_string()])));
    assert_eq!(code(reveal_flaw(&base, Some(f64::INFINITY), None)), Some((OutcomeCode::Invariant, vec!["reveal_depth".to_string()])));
    assert_eq!(code(reveal_flaw(&base, None, Some("m-gone"))), Some((OutcomeCode::TargetMissing, vec!["reveal_material".to_string()])));
    let _ = Profile::Circle { diameter: 1.0 };
}
