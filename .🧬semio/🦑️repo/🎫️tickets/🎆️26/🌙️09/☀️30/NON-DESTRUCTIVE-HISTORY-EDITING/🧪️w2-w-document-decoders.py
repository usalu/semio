"""📨️ W2-W-document: adds the generic `decode_<aggregate>_mutation_payload(kind, payload_json)` bridge (design §11, F10) to
every in-scope aggregate's Delegation region, next to its `apply_*`/`inverse_*`. Idempotent."""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
TARGETS = {
    "📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🦀️.rs": ("decode_x_conformance_mutation_payload", "PdfX1Mutation"),
    "📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🦀️.rs": ("decode_a_conformance_mutation_payload", "PdfA1Mutation"),
    "📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs": ("decode_pdf_mutation_payload", "PdfMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/🦀️.rs": ("decode_ua_conformance_mutation_payload", "PdfUaMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🧬️schema/🧬️mutations/🦀️.rs": ("decode_h_conformance_mutation_payload", "PdfHMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/🦀️.rs": ("decode_e_conformance_mutation_payload", "PdfEMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/🦀️.rs": ("decode_x_conformance_mutation_payload", "PdfXMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🦀️.rs": ("decode_a_conformance_mutation_payload", "PdfAMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/🦀️.rs": ("decode_vt_conformance_mutation_payload", "PdfVtMutation"),
    "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs": ("decode_pdf_mutation_payload", "PdfMutation"),
}

TEMPLATE = """

/// 📨️ Builds the operation of semantic kind `kind` from its editable payload JSON — the leaf wire (`payload_value()`) a
/// `🥒️.feature` row carries — through the derive's generic `from_payload_value`.
pub fn {name}(kind: &str, payload: &str) -> Result<{ty}, String> {{
    use protocol::Mutation;
    pack::from_json_str(payload).and_then(|value| {ty}::from_payload_value(kind, value)).map_err(|error| error.to_string())
}}
"""

def apply(targets):
    for relative, (name, ty) in targets.items():
        path = ROOT / relative
        text = path.read_text()
        if f"pub fn {name}(" in text:
            print("unchanged", relative)
            continue
        anchor = "//#endregion 🔖️Delegation"
        assert text.count(anchor) == 1, relative
        text = text.replace(anchor, TEMPLATE.format(name=name, ty=ty)[1:] + anchor, 1)
        path.write_text(text)
        print("patched", relative)

if __name__ == "__main__":
    apply(TARGETS)
