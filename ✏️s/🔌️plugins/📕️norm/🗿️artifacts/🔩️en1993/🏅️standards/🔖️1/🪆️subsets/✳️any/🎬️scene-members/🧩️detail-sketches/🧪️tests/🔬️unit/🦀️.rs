use super::*;
use crate::model::En1993Snapshot;

fn compliant() -> En1993Snapshot {
    En1993Snapshot::compliant_heb240_frame()
}

#[test]
fn the_effective_width_of_the_compliant_panel_follows_en_1993_1_5_table_4_1() {
    let panel = &compliant().plated_panels[0];
    let (rho, b_eff, edge) = panel_effective_width(panel).expect("effective width");
    assert!((rho - 0.736_328_5).abs() < 1e-6);
    assert!((b_eff - 0.441_797_1).abs() < 1e-6);
    assert!((edge - 0.220_898_6).abs() < 1e-6);
}

#[test]
fn the_whitmore_width_spreads_thirty_degrees_through_the_bolt_rows() {
    let joint = &compliant().joints[0];
    let (first, last) = whitmore(joint).expect("bolted rows");
    assert!((first - 0.06).abs() < 1e-12);
    assert!((last - 0.129_282_032_3).abs() < 1e-9);
    let welded = crate::model::SteelJoint { kind: semio_s_artifact_norm_en1993_model::JointKind::Welded, ..joint.clone() };
    assert!(whitmore(&welded).is_none());
}

#[test]
fn the_haunch_is_a_tenth_of_the_span_long_and_as_deep_as_the_beam() {
    let mut joint = compliant().joints[0].clone();
    assert!(haunch(&joint, 0.24).is_none());
    joint.beam_span = 6.0;
    let (length, depth) = haunch(&joint, 0.24).expect("haunch");
    assert!((length - 0.6).abs() < 1e-12 && (depth - 0.24).abs() < 1e-12);
}
