//! 🧫️ Every committed `En1991Mutation` specification vector — one canonical case per mutation leaf — held to one law.
//!
//! @see ../../../../🧫️fixtures/🧬️mutations — the committed `(before, mutation, after, diff, outcome)` bundles.
//! @see ../../../../🔮️oracles/🔣️.json — the `en1991-1-any` catalog that registers each bundle.
//! @see ../../../../🧪️tests/🏋️mutate-en1991-1/🥒️.feature — the independent Python reference reading the same bundles.

use crate::{En1991Diff, En1991Mutation, En1991Snapshot};
use semio_framework_value::ToValue;
use protocol::Mutation;

//#region 🧾️Vector
/// 🧾️ One committed vector: the semantic kind it witnesses and its five committed files.
pub(crate) struct Vector {
    pub(crate) kind: &'static str,
    pub(crate) before: &'static str,
    pub(crate) mutation: &'static str,
    pub(crate) after: &'static str,
    pub(crate) diff: &'static str,
    pub(crate) outcome: &'static str,
}

/// 🧾️ The committed op and before-snapshot of one vector, for the inverse-sum law.
pub(crate) fn committed_op_and_before(vector: &Vector) -> (En1991Mutation, En1991Snapshot) {
    let op: En1991Mutation = store::os_store::test_support::assert_wire_witness(vector.mutation);
    let before: En1991Snapshot = semio_framework_pack_json::from_json_str(vector.before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed before-snapshot decodes");
    (op, before)
}

/// 🔣️ A committed file as the independent `serde_json` oracle reads it.
fn committed(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("a committed vector file is JSON")
}

/// 🪞️ A production value as the independent `serde_json` oracle reads its Rust wire.
fn wire<T: ToValue>(value: &T) -> serde_json::Value {
    serde_json::Value::from(value.to_value())
}

/// 🎯️ The same op addressing a position no collection holds, for an op whose payload carries an `index`.
fn out_of_range(op: &En1991Mutation) -> Option<En1991Mutation> {
    let mut payload = serde_json::Value::from(op.payload_value());
    *payload.get_mut("index")? = serde_json::Value::from(u64::from(u32::MAX));
    op.with_payload_value(semio_framework_value::DslValue::from(&payload)).ok()
}

/// ⚖️ The law every committed vector obeys: the mutation file is the canonical Rust wire of one `kind` op whose binary frame
/// round-trips; both snapshots and the diff are canonical; production dispatch turns BEFORE into exactly the committed diff
/// under the committed outcome and lands on AFTER; the committed diff alone carries BEFORE to AFTER; the op's own inverse
/// restores BEFORE; and the leaf descriptor declares exactly the outcome classes dispatch reaches from the vector — its own
/// status, `no-op` when re-applying the op to AFTER changes nothing, `rejected` when the op addressing an index no collection
/// holds is refused.
pub(crate) fn assert_vector(vector: Vector) {
    let kind = vector.kind;
    let op: En1991Mutation = store::os_store::test_support::assert_wire_witness(vector.mutation);
    assert_eq!(op.descriptor().semantic_kind, kind, "{kind}: the committed mutation is another kind's op");
    let framed = protocol::OpBinary::encode_op(&op).expect("the op encodes to its binary frame");
    assert_eq!(<En1991Mutation as protocol::OpBinary>::decode_op(&framed).expect("its binary frame decodes"), op, "{kind}: the binary frame does not round-trip");
    let before: En1991Snapshot = semio_framework_pack_json::from_json_str(vector.before, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed before-snapshot decodes");
    let after: En1991Snapshot = semio_framework_pack_json::from_json_str(vector.after, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed after-snapshot decodes");
    let delta: En1991Diff = semio_framework_pack_json::from_json_str(vector.diff, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed diff decodes");
    assert_eq!(wire(&before), committed(vector.before), "{kind}: the committed before-snapshot is not the canonical wire");
    assert_eq!(wire(&after), committed(vector.after), "{kind}: the committed after-snapshot is not the canonical wire");
    assert_eq!(wire(&delta), committed(vector.diff), "{kind}: the committed diff is not the canonical wire");
    let outcome = op.diff(&before);
    assert_eq!(wire(outcome.diff()), committed(vector.diff), "{kind}: production dispatch produces another diff than the committed one");
    let status = committed(vector.outcome)["status"].as_str().expect("the committed outcome names its status").to_string();
    match status.as_str() {
        "applied" => assert!(outcome.messages().is_empty(), "{kind}: an applied vector raises {:?}", outcome.messages()),
        "no-op" => assert_ne!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal), "{kind}: a no-op vector is refused"),
        "rejected" => assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal), "{kind}: a rejected vector is accepted"),
        other => panic!("{kind}: unknown committed outcome status {other:?}"),
    }
    let applied = protocol::apply_diff(outcome.diff(), &before).expect("the produced diff applies to the committed before-snapshot");
    assert_eq!(applied, after, "{kind}: production dispatch does not land on the committed after-snapshot");
    assert_eq!(protocol::apply_diff(&delta, &before).expect("the committed diff applies to the committed before-snapshot"), after, "{kind}: the committed diff does not carry before to after");
    assert_eq!(status == "applied", applied != before, "{kind}: an applied vector must move the document and only an applied one may");
    let inverse = op.inverse(&before).expect("valid retained mutation inverse fixture");
    assert_eq!(status == "applied", !inverse.is_empty(), "{kind}: an applied vector computes a non-empty inverse and only an applied one does");
    let restored = inverse.iter().fold(applied, |current, step| protocol::apply_diff(step.diff(&current).diff(), &current).expect("an inverse step applies"));
    assert_eq!(restored, before, "{kind}: replaying the inverse does not restore the committed before-snapshot");
    let no_op = op.diff(&after).messages().iter().any(|message| message.code.0 == "mutation.no-op");
    let rejected = out_of_range(&op).is_some_and(|stray| stray.diff(&before).worst_level() >= Some(semio_framework_diagnostic::Severity::Error));
    let reached: Vec<&str> = [(true, status.as_str()), (no_op, "no-op"), (rejected, "rejected")].into_iter().filter_map(|(reached, class)| reached.then_some(class)).collect();
    let declared: Vec<&str> = op.descriptor().outcome_classes.iter().map(|class| class.as_str()).collect();
    assert_eq!(declared, reached, "{kind}: the descriptor's outcome classes are not the ones production dispatch reaches");
}
//#endregion 🧾️Vector

//#region 🧪️Cases
#[path = "../../🌍change-annex/🧪️tests/✅apply/🦀️.rs"]
mod change_annex;
#[path = "../../🗺️change-snow-zone/🧪️tests/✅apply/🦀️.rs"]
mod change_snow_zone;
#[path = "../../❄️change-altitude/🧪️tests/✅apply/🦀️.rs"]
mod change_altitude;
#[path = "../../❄️change-en-sk/🧪️tests/✅apply/🦀️.rs"]
mod change_en_sk;
#[path = "../../🏔️change-north-german-lowland-snow/🧪️tests/✅apply/🦀️.rs"]
mod change_north_german_lowland_snow;
#[path = "../../🪁change-wind-zone/🧪️tests/✅apply/🦀️.rs"]
mod change_wind_zone;
#[path = "../../🌬️change-en-vb/🧪️tests/✅apply/🦀️.rs"]
mod change_en_vb;
#[path = "../../🏞️change-terrain-category/🧪️tests/✅apply/🦀️.rs"]
mod change_terrain_category;
#[path = "../../🧭change-mixed-terrain-upwind/🧪️tests/✅apply/🦀️.rs"]
mod change_mixed_terrain_upwind;
#[path = "../../📏change-mixed-terrain-distance/🧪️tests/✅apply/🦀️.rs"]
mod change_mixed_terrain_distance;
#[path = "../../📐change-orography-factor/🧪️tests/✅apply/🦀️.rs"]
mod change_orography_factor;
#[path = "../../🏝️change-coast-or-island/🧪️tests/✅apply/🦀️.rs"]
mod change_coast_or_island;
#[path = "../../🏢change-air-density/🧪️tests/✅apply/🦀️.rs"]
mod change_air_density;
#[path = "../../🧱change-height/🧪️tests/✅apply/🦀️.rs"]
mod change_height;
#[path = "../../🏠change-width/🧪️tests/✅apply/🦀️.rs"]
mod change_width;
#[path = "../../💨change-depth/🧪️tests/✅apply/🦀️.rs"]
mod change_depth;
#[path = "../../🌡️change-assumed-delta-t/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_delta_t;
#[path = "../../🔥change-construction-activity/🧪️tests/✅apply/🦀️.rs"]
mod change_construction_activity;
#[path = "../../⚙️change-assumed-construction-qk/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_construction_qk;
#[path = "../../🌉change-structure-kind/🧪️tests/✅apply/🦀️.rs"]
mod change_structure_kind;
#[path = "../../🌉change-bridge-lane/🧪️tests/✅apply/🦀️.rs"]
mod change_bridge_lane;
#[path = "../../🏗️change-bridge-span/🧪️tests/✅apply/🦀️.rs"]
mod change_bridge_span;
#[path = "../../↔️change-bridge-lane-width/🧪️tests/✅apply/🦀️.rs"]
mod change_bridge_lane_width;
#[path = "../../🌾change-assumed-bridge-tandem/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_tandem;
#[path = "../../🛣️change-assumed-bridge-udl/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_udl;
#[path = "../../🚛change-assumed-bridge-lm2/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_lm2;
#[path = "../../🚶change-assumed-bridge-footway/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_footway;
#[path = "../../🏙️change-storey-count/🧪️tests/✅apply/🦀️.rs"]
mod change_storey_count;
#[path = "../../🌡️change-t-max/🧪️tests/✅apply/🦀️.rs"]
mod change_t_max;
#[path = "../../🧊change-t-min/🧪️tests/✅apply/🦀️.rs"]
mod change_t_min;
#[path = "../../🕰️change-initial-temperature/🧪️tests/✅apply/🦀️.rs"]
mod change_initial_temperature;
#[path = "../../🏗️change-thermal-element-type/🧪️tests/✅apply/🦀️.rs"]
mod change_thermal_element_type;
#[path = "../../🌉change-thermal-bridge-type/🧪️tests/✅apply/🦀️.rs"]
mod change_thermal_bridge_type;
#[path = "../../📏change-linear-temperature-gradient/🧪️tests/✅apply/🦀️.rs"]
mod change_linear_temperature_gradient;
#[path = "../../🔥change-fire-mode/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_mode;
#[path = "../../📉change-fire-curve/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_curve;
#[path = "../../⏱️change-fire-duration/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_duration;
#[path = "../../♨️change-assumed-gas-temperature/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_gas_temperature;
#[path = "../../🔆change-assumed-h-net/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_h_net;
#[path = "../../🗺️change-fire-compartment-area/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_compartment_area;
#[path = "../../📐change-fire-compartment-height/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_compartment_height;
#[path = "../../🪟change-fire-opening-factor/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_opening_factor;
#[path = "../../🧱change-fire-thermal-inertia/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_thermal_inertia;
#[path = "../../🏢change-fire-occupancy/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_occupancy;
#[path = "../../⛽change-fire-load-density-qf/🧪️tests/✅apply/🦀️.rs"]
mod change_fire_load_density_qf;
#[path = "../../🔋change-assumed-qf-d/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_qf_d;
#[path = "../../🚛change-assumed-bridge-lm3/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_lm3;
#[path = "../../👥change-assumed-bridge-lm4/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_bridge_lm4;
#[path = "../../📦change-bridge-load-group/🧪️tests/✅apply/🦀️.rs"]
mod change_bridge_load_group;
#[path = "../../🏗️change-crane-claimed/🧪️tests/✅apply/🦀️.rs"]
mod change_crane_claimed;
#[path = "../../💥change-crane-class/🧪️tests/✅apply/🦀️.rs"]
mod change_crane_class;
#[path = "../../➕change-hoist-class/🧪️tests/✅apply/🦀️.rs"]
mod change_hoist_class;
#[path = "../../⏫change-hoisting-speed/🧪️tests/✅apply/🦀️.rs"]
mod change_hoisting_speed;
#[path = "../../➖change-assumed-crane-wheel/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_crane_wheel;
#[path = "../../↔️change-assumed-crane-horizontal/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_crane_horizontal;
#[path = "../../🏭change-silo-claimed/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_claimed;
#[path = "../../⚖️change-silo-kind/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_kind;
#[path = "../../🌾change-silo-bulk-density/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_bulk_density;
#[path = "../../🏷️change-silo-height/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_height;
#[path = "../../⭕change-silo-hydraulic-radius/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_hydraulic_radius;
#[path = "../../🔎change-silo-mu/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_mu;
#[path = "../../⚙️change-silo-k/🧪️tests/✅apply/🦀️.rs"]
mod change_silo_k;
#[path = "../../🌀change-assumed-silo-pressure/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_silo_pressure;
#[path = "../../📦change-assumed-silo-patch/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_silo_patch;
#[path = "../../🧱change-assumed-silo-wall-friction/🧪️tests/✅apply/🦀️.rs"]
mod change_assumed_silo_wall_friction;
#[path = "../../🏢change-floor-assumed-qk/🧪️tests/✅apply/🦀️.rs"]
mod change_floor_assumed_qk;
#[path = "../../🚧change-self-weight-assumed-gk/🧪️tests/✅apply/🦀️.rs"]
mod change_self_weight_assumed_gk;
#[path = "../../🌨️change-roof-assumed-sk/🧪️tests/✅apply/🦀️.rs"]
mod change_roof_assumed_sk;
#[path = "../../🛡️change-wind-face-assumed-wp/🧪️tests/✅apply/🦀️.rs"]
mod change_wind_face_assumed_wp;
#[path = "../../🚗change-accidental-assumed-force/🧪️tests/✅apply/🦀️.rs"]
mod change_accidental_assumed_force;
#[path = "../../➕️insert-floors/🧪️tests/✅apply/🦀️.rs"]
mod insert_floors;
#[path = "../../➖️remove-floors/🧪️tests/✅apply/🦀️.rs"]
mod remove_floors;
#[path = "../../➕️insert-self-weight-elements/🧪️tests/✅apply/🦀️.rs"]
mod insert_self_weight_elements;
#[path = "../../➖️remove-self-weight-elements/🧪️tests/✅apply/🦀️.rs"]
mod remove_self_weight_elements;
#[path = "../../➕️insert-roofs/🧪️tests/✅apply/🦀️.rs"]
mod insert_roofs;
#[path = "../../➖️remove-roofs/🧪️tests/✅apply/🦀️.rs"]
mod remove_roofs;
#[path = "../../➕️insert-wind-faces/🧪️tests/✅apply/🦀️.rs"]
mod insert_wind_faces;
#[path = "../../➖️remove-wind-faces/🧪️tests/✅apply/🦀️.rs"]
mod remove_wind_faces;
#[path = "../../➕️insert-accidental-cases/🧪️tests/✅apply/🦀️.rs"]
mod insert_accidental_cases;
#[path = "../../➖️remove-accidental-cases/🧪️tests/✅apply/🦀️.rs"]
mod remove_accidental_cases;
//#endregion 🧪️Cases
