#!/usr/bin/env python3
"""🧯️ S4-NORM: the controlled `DslField` methods now return `ValueError` (peer DSL extraction); the hand-written norm bridges
(vdi3805 `SheetId`/`RecordFamilyId`/`SheetAttributes` + their attribute decoders, iso16757 `CatalogueId`/`CatalogueValue`/`PartNumberRule` +
the `🛬️native` decoders) still declared `String`. Rewrites, inside exactly those regions, `Result<_,String>` → `Result<_,ValueError>`,
string error literals → `<prefix>_invalid(…)` (`ValueRefusalKind::InvalidValue`), and drops the `.map_err(|error|error.to_string())`
adapters. `from_value` (the uncontrolled method, still `String`) is left alone. `--check` writes nothing."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ART = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
VALUE_ERROR = "semio_framework_value::ValueError"


def convert(text: str, helper: str) -> str:
    text = re.sub(r"(fn (?:shape_controlled|to_value_controlled|from_value_controlled)\b[^{]*?)Result<([^{]*?),\s*String>", lambda m: f"{m.group(1)}Result<{m.group(2)},{VALUE_ERROR}>", text)
    text = text.replace(".map(semio_framework_dsl_record::FieldValue::Value).map_err(|error|error.to_string())", ".map(semio_framework_dsl_record::FieldValue::Value)")
    return text


def convert_helpers(text: str, helper: str) -> str:
    text = re.sub(r"Result<([^<>]*(?:<[^<>]*(?:<[^<>]*>[^<>]*)*>)?[^<>]*?),\s*String>", lambda m: f"Result<{m.group(1)},{VALUE_ERROR}>", text)
    text = re.sub(r'Err\(("(?:[^"\\]|\\.)*")\.into\(\)\)', lambda m: f"Err({helper}({m.group(1)}))", text)
    text = re.sub(r'Err\((format!\((?:[^()]|\([^()]*\))*\))\)', lambda m: f"Err({helper}({m.group(1)}))", text)
    text = re.sub(r'\.ok_or\(("(?:[^"\\]|\\.)*")\)', lambda m: f".ok_or_else(||{helper}({m.group(1)}))", text)
    text = re.sub(r'\.ok_or_else\(\s*\|\|\s*("(?:[^"\\]|\\.)*")\.into\(\)\s*\)', lambda m: f".ok_or_else(||{helper}({m.group(1)}))", text)
    text = re.sub(r'\.ok_or_else\(\s*\|\|\s*(format!\((?:[^()]|\([^()]*\))*\))\s*\)', lambda m: f".ok_or_else(||{helper}({m.group(1)}))", text)
    text = re.sub(r'\.map_err\(\|_\|\s*("(?:[^"\\]|\\.)*")(?:\.into\(\))?\)', lambda m: f".map_err(|_|{helper}({m.group(1)}))", text)
    text = re.sub(r'\.map_err\(\|_\|("(?:[^"\\]|\\.)*")\.to_string\(\)\)', lambda m: f".map_err(|_|{helper}({m.group(1)}))", text)
    text = text.replace(".map_err(|error|error.to_string())", "")
    return text


def edit_region(text: str, start: str, end: str, fn) -> str:
    a = text.index(start)
    b = text.index(end, a)
    return text[:a] + fn(text[a:b]) + text[b:]


def main(argv: list[str]) -> int:
    pending = []
    vdi = ART / "🏭️vdi3805/🦀️.rs"
    text = vdi.read_text()
    new = convert(text, "vdi3805_invalid")
    new = edit_region(new, "fn sheet_attribute_object<'a>(", "/// ⚙️ Product configuration block.", lambda region: convert_helpers(region, "vdi3805_invalid"))
    new = new.replace('            _ => Err("expected VDI3805 sheet attribute value".into()),', '            _ => Err(vdi3805_invalid("expected VDI3805 sheet attribute value")),')
    if "fn vdi3805_invalid(" not in new:
        new = new.replace("fn sheet_attribute_object<'a>(", '/// 🚫️ An invalid VDI 3805 attribute value.\nfn vdi3805_invalid(message: impl Into<String>) -> semio_framework_value::ValueError {\n    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)\n}\n\nfn sheet_attribute_object<\'a>(', 1)
    if new != text:
        pending.append((vdi, new))
    iso = ART / "📇️iso16757/🦀️.rs"
    text = iso.read_text()
    new = convert(text, "")
    if new != text:
        pending.append((iso, new))
    native = ART / "📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛬️native/🦀️.rs"
    text = native.read_text()
    new = convert_helpers(text, "invalid")
    if "fn invalid(" not in new:
        new = new.replace("use std::collections::BTreeMap;\n", "use std::collections::BTreeMap;\nfn invalid(message:impl Into<String>)->semio_framework_value::ValueError{semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}\n", 1)
    if new != text:
        pending.append((native, new))
    if "--check" not in argv:
        for path, new in pending:
            path.write_text(new)
    print(("pending=" if "--check" in argv else "written=") + str(len(pending)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
