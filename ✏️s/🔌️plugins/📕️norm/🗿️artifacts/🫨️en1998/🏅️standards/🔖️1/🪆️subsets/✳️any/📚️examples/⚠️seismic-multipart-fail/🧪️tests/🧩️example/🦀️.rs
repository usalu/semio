use semio_s_artifact_norm_en1998_inferences::evaluate;
use semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl;

#[test]
fn dump() {
    let snap = decode_en1998_dsl(crate::seismic_multipart_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] seismic_multipart_fail {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
