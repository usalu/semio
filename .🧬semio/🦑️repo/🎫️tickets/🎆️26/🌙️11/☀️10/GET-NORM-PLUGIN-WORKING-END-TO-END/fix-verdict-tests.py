#!/usr/bin/env python3
"""Replace temporary dump tests with verdict assertions."""
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")

REPLACEMENTS = {
    "seismic-multipart/": (
        '''#[test]
fn dump() {
    let snap = decode_en1998_dsl(crate::seismic_multipart::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] seismic_multipart {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
''',
        '''#[test]
fn primary_asset_has_no_failing_check() {
    let snap = decode_en1998_dsl(crate::seismic_multipart::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    let fails: Vec<_> = report.failing().map(|c| c.id.clone()).collect();
    assert!(!report.checks.is_empty(), "the compliant multi-part example emits checks");
    assert!(fails.is_empty(), "fails={fails:?}");
}
''',
    ),
    "seismic-multipart-fail/": (
        '''#[test]
fn dump() {
    let snap = decode_en1998_dsl(crate::seismic_multipart_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] seismic_multipart_fail {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
''',
        '''#[test]
fn primary_asset_fails_and_every_failure_has_a_remedy() {
    let snap = decode_en1998_dsl(crate::seismic_multipart_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 2, "fail_count={}", fails.len());
    for check in &fails {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
''',
    ),
    "seismic-rc-frame-fail/": (
        '''#[test]
fn dump() {
    let snap = decode_en1998_dsl(crate::seismic_rc_frame_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] seismic_rc_frame_fail {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
''',
        '''#[test]
fn primary_asset_fails_and_every_failure_has_a_remedy() {
    let snap = decode_en1998_dsl(crate::seismic_rc_frame_fail::PRIMARY_TEXT).expect("decodes");
    let report = evaluate(&snap);
    let fails: Vec<_> = report.failing().collect();
    assert!(fails.len() >= 1, "fail_count={}", fails.len());
    for check in &fails {
        assert!(!check.remedies.is_empty(), "{} has no remedy", check.id);
    }
}
''',
    ),
    "seismic-rc-frame/": (
        '''#[test]
fn dump_rc_frame() {
    let snap = semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl(crate::seismic_rc_frame::PRIMARY_TEXT).expect("decodes");
    let report = semio_s_artifact_norm_en1998_inferences::evaluate(&snap);
    for c in &report.checks { eprintln!("[DEBUG] seismic_rc_frame {} {:?} u={} rem={}", c.id, c.status, c.utilization, c.remedies.len()); }
}
''',
        '''#[test]
fn primary_asset_has_no_failing_check() {
    let snap = semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl(crate::seismic_rc_frame::PRIMARY_TEXT).expect("decodes");
    let report = semio_s_artifact_norm_en1998_inferences::evaluate(&snap);
    let fails: Vec<_> = report.failing().map(|c| c.id.clone()).collect();
    assert!(!report.checks.is_empty(), "the compliant RC frame emits checks");
    assert!(fails.is_empty(), "fails={fails:?}");
}
''',
    ),
}

def main() -> None:
    changed = 0
    for path in ROOT.rglob("*.rs"):
        text = path.read_text()
        if "[DEBUG]" not in text:
            continue
        key = next((name for name in REPLACEMENTS if name in str(path)), None)
        if key is None:
            print("SKIP", path)
            continue
        old, new = REPLACEMENTS[key]
        if old not in text:
            print("MISMATCH", path)
            continue
        path.write_text(text.replace(old, new, 1))
        print("WROTE", path)
        changed += 1
    purlin = None
    for path in ROOT.rglob("*.rs"):
        if "aluminium-roof-purlin" in str(path) and path.name.endswith(".rs") and "tests" in str(path):
            purlin = path
            break
    if purlin is not None:
        text = purlin.read_text()
        dump = '''
#[test]
fn debug_dump_checks() {
    let report = evaluate(&decode_en1999_dsl(crate::aluminium_roof_purlin::PRIMARY_TEXT).expect("decodes"));
    for c in &report.checks {
        eprintln!("[DEBUG] {:?} {} u={:.3} computed={:?} limit={:?}", c.status, c.id, c.utilization, c.computed, c.limit);
    }
}
'''
        if dump in text:
            purlin.write_text(text.replace(dump, "\n"))
            print("WROTE", purlin)
            changed += 1
        else:
            print("PURLIN DEBUG STILL" if "[DEBUG]" in text else "PURLIN CLEAN", purlin)
    print("changed", changed)

if __name__ == "__main__":
    main()
