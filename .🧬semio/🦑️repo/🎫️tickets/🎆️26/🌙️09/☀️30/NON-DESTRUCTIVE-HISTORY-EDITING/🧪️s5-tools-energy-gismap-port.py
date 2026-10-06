"""🔌️ Ports energy-model, gis-gismap and (unowned, given to S5-TOOLS) animate presentation to the current peer APIs (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, report
`📓️s4-tools-a-report.md` § S5.10): `dsl::json` → `semio_framework_pack_json` (member policy `Reject`), `TextError::new(kind,
message, span)`, `DslField::from_value -> Result<_, String>`, a retirement step that refuses with a `ValueError`, and the
value traits from `semio_framework_value` (the kernel's `os_dsl` re-export is gone). Explicit file list, every anchor counted,
fails closed before the first write.

Usage (cwd: repo root): python3 🧪️s5-tools-energy-gismap-port.py [--apply]
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
EN = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
GM = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any"
GI = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference"
AN = "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any"
KIND = "semio_framework_value::ValueRefusalKind::InvalidValue"
SPAN = "semio_framework_diagnostic::TextSpan::at(1, 1)"
TO_JSON = ("dsl::json::to_json_string(", "semio_framework_pack_json::to_json_string(", 1)
FROM_JSON = ("dsl::json::from_json_str::<semio_framework_value::DslValue>(camera)", "semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(camera, semio_framework_pack_json::JsonMemberPolicy::Reject)", 1)
TEXT = ("semio_framework_diagnostic::TextError::new(format!(", f"semio_framework_diagnostic::TextError::new({KIND}, format!(", 1)
LOST = 'semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "{}")'
STALE = "Presentation envelope materialize close received a stale operation/generation"
MOVED = "Presentation envelope terminal handle changed before exact registry removal"
FILES = {
    f"{AN}/🚪️io/🧬️mutations/💾️binary/🦀️.rs": [(f'.map_err(|_| "{text}")?', ".map_err(|_| " + LOST.format(text) + ")?", 1) for text in (STALE, MOVED)],
    f"{EN}/✏️editor/🦀️.rs": [TO_JSON, FROM_JSON],
    f"{EN}/👁️viewer/🦀️.rs": [TO_JSON, FROM_JSON],
    f"{EN}/🧬️schema/📸️snapshot/🦀️.rs": [(f"semio_framework_diagnostic::TextError::new(error, {SPAN})", f"semio_framework_diagnostic::TextError::new({KIND}, error, {SPAN})", 1)],
    f"{EN}/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": [TEXT],
    f"{EN}/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs": [TEXT],
    f"{EN}/🧬️schema/📸️snapshot/🪶️sqlite/⚙️systems/🦀️.rs": [('"Energy enum ordinal overflow"))?))}', '"Energy enum ordinal overflow"))?)).map_err(invalid)}', 1)],
    f"{GI}/👷️worker/🦀️.rs": [("use semio_framework_os_kernel::os_dsl::{DslValue,ToValue,FromValue};", "use semio_framework_value::{DslValue,ToValue,FromValue};", 1)],
    f"{GM}/🧬️schema/📸️snapshot/📦️pack/🦀️.rs": [
        (
            ' fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,ValueError>{match value{semio_framework_dsl_record::FieldValue::Bytes64(v)=>Ok(Self(v.clone())),_=>Err(invalid("GIS native octets differ"))}}',
            ' fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Bytes64(v)=>Ok(Self(v.clone())),_=>Err("GIS native octets differ".to_string())}}',
            1,
        )
    ],
    f"{GM}/🧬️schema/🧬️mutations/💾️binary/🦀️.rs": [
        ("    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {", "    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {", 1),
        ('.ok_or_else(|| "GIS value entry lost its retained value".to_string())?;', ".ok_or_else(|| " + LOST.format("GIS value entry lost its retained value") + ")?;", 1),
        ('.ok_or_else(|| "GIS mutation value owner was lost".to_string())?;', ".ok_or_else(|| " + LOST.format("GIS mutation value owner was lost") + ")?;", 1),
    ],
    **{f"{GM}/🚪️io/{path}/🦀️.rs": [TEXT] for path in ("📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any", "📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any", "📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any", "📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any", "📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any", "📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any", "📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any")},
}


def main():
    staged, pending = [], 0
    for relative, edits in FILES.items():
        path = REPO / relative
        if not path.is_file():
            raise SystemExit(f"{relative} is gone")
        text = path.read_text(encoding="utf-8")
        for old, new, count in edits:
            if text.count(old) == count:
                text, pending = text.replace(old, new), pending + count
            elif text.count(old) != 0 or text.count(new) < count:
                raise SystemExit(f"{relative}: {text.count(old)} of {count} site(s) of {old[:70]!r}")
        staged.append((path, text))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'ported' if '--apply' in sys.argv else 'would port'} {pending} site(s) in {len(staged)} listed files")


if __name__ == "__main__":
    main()
