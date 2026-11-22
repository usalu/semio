use semio_s_artifact_norm_en1993_inferences::evaluate;
use semio_s_artifact_norm_en1993_io::text::snapshot::decode_en1993_dsl;

#[test]
fn dump() {
    let snap = decode_en1993_dsl(crate::heb240_compliant::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] heb240_compliant {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
