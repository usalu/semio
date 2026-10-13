"""🔀️ Moves each pdf conformance subset's owned-snapshot policy onto its registered validator.

Replaces the dead per-subset `VALIDATOR_ENTRY`/`validator_entry`/`register` trio with a
`store::io::SnapshotSubsetValidator` impl and drops the imports only that trio used.
Usage: python3 🔀️snapshot-validators.py <pdf artifact root>
"""
import re
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "🏅️standards"
PROFILE = "crate::standards::v1_7::subsets::base::io::sqlite::snapshot"
SUBSETS = [
    ("7️⃣1.7/🪆️subsets/🗄️a", "PdfAValidator", "check_pdf_a_conformance", (True, "Some(Severity::Error)", False)),
    ("7️⃣1.7/🪆️subsets/🖨️x", "PdfXValidator", "check_x_conformance", (True, "Some(Severity::Warning)", True)),
    ("7️⃣1.7/🪆️subsets/📐️e", "PdfEValidator", "check_e_conformance", (True, "Some(Severity::Error)", False)),
    ("7️⃣1.7/🪆️subsets/♿️ua", "PdfUaValidator", "check_ua_conformance", (False, "None", False)),
    ("7️⃣1.7/🪆️subsets/🧾️vt", "PdfVtValidator", "check_vt_conformance", (True, "Some(Severity::Warning)", True)),
    ("7️⃣1.7/🪆️subsets/⚕️h", "PdfHValidator", "check_h_conformance", (False, "Some(Severity::Warning)", False)),
    ("4️⃣1.4/🪆️subsets/🗄️a", "PdfAValidator", "check_pdf_a_conformance", None),
    ("4️⃣1.4/🪆️subsets/🖨️x", "PdfXValidator", "check_pdf_x_conformance", None),
]


def snapshot_impl(validator: str, check: str, profile) -> str:
    head = f"    impl store::io::SnapshotSubsetValidator for {validator} {{\n        type Snapshot = PdfSnapshot;\n\n        fn validate_snapshot(snapshot: &PdfSnapshot, database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> store::io_schema::IoResult<()> {{\n"
    if profile is None:
        body = f"            control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 1, 1).map_err(store::io_schema::IoError::from_value_error)?;\n            let _ = database;\n            Ok(store::io_schema::IoOutcome {{ value: (), diagnostics: {check}(snapshot) }})\n"
    else:
        encryption, actions, page_box = profile
        body = f"            {PROFILE}::validate_conformance_profile(snapshot, database, control, {check}(snapshot), {PROFILE}::PdfConformanceProfile {{ encryption_forbidden: {str(encryption).lower()}, action_severity: {actions}, page_box_required: {str(page_box).lower()} }})\n"
    return head + body + "        }\n    }\n"


for leaf, validator, check, profile in SUBSETS:
    path = ROOT / leaf / "🚪️io" / "🦀️.rs"
    text = path.read_text(encoding="utf-8")
    start = text.index("    static VALIDATOR_ENTRY:")
    register = text.index("    pub fn register() {", start)
    end = text.index("\n    }\n", register) + len("\n    }\n")
    text = text[:start] + snapshot_impl(validator, check, profile) + text[end:]
    text = text.replace("semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,", "")
    text = text.replace(",semio_framework_plugin::io::SubsetValidatorEntry}", "}")
    if text.count("OnceLock") == 1:
        text = text.replace("    use std::sync::OnceLock;\n", "")
    if re.search(r"register_subset_validator|subset_validator_entry_of|SubsetValidatorEntry|VALIDATOR_ENTRY", text):
        raise SystemExit(f"leftover registration symbol in {path}")
    path.write_text(text, encoding="utf-8")
    print(f"ok {leaf}")
