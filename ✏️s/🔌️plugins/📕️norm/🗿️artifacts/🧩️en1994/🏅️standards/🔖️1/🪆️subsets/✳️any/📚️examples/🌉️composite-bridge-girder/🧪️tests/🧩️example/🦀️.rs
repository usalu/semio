use semio_s_artifact_norm_en1994_inferences::evaluate;
use semio_s_artifact_norm_en1994_io::text::snapshot::decode_en1994_dsl;

#[test]
fn dump() {
    let snap = decode_en1994_dsl(crate::composite_bridge_girder::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] composite_bridge_girder {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
