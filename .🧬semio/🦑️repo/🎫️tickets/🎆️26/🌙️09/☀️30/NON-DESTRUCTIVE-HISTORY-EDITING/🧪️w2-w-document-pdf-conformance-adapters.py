"""📨️ W2-W-document: converts the six PDF 1.7 conformance-class case adapters (ua, h, e, x, a, vt) to the generic wire
decoder (design §11, F10) — the per-kind params→op match, its number/box/ordinal helpers and the `programOrdinal` lookup
are deleted; the forward row and the oracle's computed undo spec both decode through
`decode_<subset>_conformance_mutation_payload`."""
import pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets")
SUBSETS = {"♿️ua": ("ua", "PdfUaMutation"), "⚕️h": ("h", "PdfHMutation"), "📐️e": ("e", "PdfEMutation"), "🖨️x": ("x", "PdfXMutation"), "🗄️a": ("a", "PdfAMutation"), "🧾️vt": ("vt", "PdfVtMutation")}

for folder, (subset, ty) in SUBSETS.items():
    path = next((ROOT / folder / "🧪️tests").glob("*/🦀️.rs"))
    text = path.read_text()
    if f"decode_{subset}_conformance_mutation_payload" in text:
        print("unchanged", folder)
        continue
    text = text.replace("    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::conformance_support as support;\n", "", 1)
    text = text.replace("    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfSnapshot};\n", "    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;\n", 1)
    text, count = re.subn(rf"    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::{subset}::schema::mutations::(\{{[^}}]*\}}|\*);\n", f"    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::{subset}::schema::mutations::{{apply_{subset}_conformance_mutation, decode_{subset}_conformance_mutation_payload, {ty}}};\n", text, count=1)
    assert count == 1, (folder, "mutations use")
    start = text.index("    fn number(value: &Json) -> Option<f64> {")
    end_marker = f"    fn mutation_from_spec(base: &PdfSnapshot, spec: &Json) -> Result<{ty}, String> {{"
    body = text.index(end_marker)
    end = text.index("\n    }\n", body) + len("\n    }\n")
    replacement = f"""    /// 📨️ The scenario's `{{kind, params}}` row — or the oracle's computed undo spec — decoded generically: `params` is the
    /// leaf wire payload, the only channel between the feature and the subject's typed `{ty}`.
    fn mutation_from_spec(spec: &Json) -> Result<{ty}, String> {{
        decode_{subset}_conformance_mutation_payload(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }}
"""
    text = text[:start] + replacement + text[end:]
    text = text.replace("mutation_from_spec(&snapshot, &spec)", "mutation_from_spec(&spec)").replace("mutation_from_spec(&snapshot, &undo)", "mutation_from_spec(&undo)")
    assert "program_reference" not in text and "four_numbers" not in text and "support::" not in text, folder
    path.write_text(text)
    print("converted", folder)
