#!/usr/bin/env python3
"""🔮️ The `patch-snapshot` arms of the stdio third-party oracles and case adapters (D4 of ticket
26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), staged as exact anchor replacements: each oracle reads the input with its own
library into its own snapshot wire, applies the row's one pointer operation (`semio_repo_test_host::law::patched_snapshot`)
and re-encodes; the inverse restores the reference's own reading of the original. Idempotent (an edit whose result is
already present is skipped); `--check` lists pending edits and exits 1 when any is pending; a missing anchor is an error.

@see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs
"""
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
A = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
PATCHED = "semio_repo_test_host::law::patched_snapshot"

EDITS = [
    # ifc 4: the Exchange's own set-snapshot payload, patched, replaces the document.
    (A + "🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🦀️.rs",
     '            "set-snapshot" => replace_with_snapshot(exchange, params.get("snapshot").ok_or("set-snapshot carries `snapshot`")?),\n',
     '            "set-snapshot" => replace_with_snapshot(exchange, params.get("snapshot").ok_or("set-snapshot carries `snapshot`")?),\n'
     '            "patch-snapshot" => {\n'
     '                let payload = snapshot_payload_of(exchange)?;\n'
     f'                let patched = {PATCHED}(payload.get("snapshot").ok_or("the reading carries no snapshot")?, params.get("patch").ok_or("patch-snapshot carries `patch`")?)?;\n'
     '                replace_with_snapshot(exchange, &patched)\n'
     '            }\n'),
    (A + "🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🏗️mutate-ifc-4/🦀️.rs",
     '        "set-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),\n',
     '        "set-snapshot" | "patch-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String("set-snapshot".to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),\n'),
    # ifc 2x3
    (A + "🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '            "set-snapshot" => replace_with_snapshot(exchange, params.get("snapshot").ok_or("set-snapshot carries `snapshot`")?),\n',
     '            "set-snapshot" => replace_with_snapshot(exchange, params.get("snapshot").ok_or("set-snapshot carries `snapshot`")?),\n'
     '            "patch-snapshot" => {\n'
     '                let payload = document_snapshot_payload(exchange);\n'
     f'                let patched = {PATCHED}(payload.get("snapshot").ok_or("the reading carries no snapshot")?, params.get("patch").ok_or("patch-snapshot carries `patch`")?)?;\n'
     '                replace_with_snapshot(exchange, &patched)\n'
     '            }\n'),
    (A + "🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧪️tests/🧱️mutate-ifc-2x3/🦀️.rs",
     '        "set-snapshot" => wire_spec("set-snapshot", oracle_snapshot_payload(input)?),\n',
     '        "set-snapshot" | "patch-snapshot" => wire_spec("set-snapshot", oracle_snapshot_payload(input)?),\n'),
    # step ap214
    (A + "📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '            "set-snapshot" => replace_with_snapshot(exchange, member("snapshot")?),\n',
     '            "set-snapshot" => replace_with_snapshot(exchange, member("snapshot")?),\n'
     '            "patch-snapshot" => {\n'
     '                let payload = document_snapshot_payload(exchange)?;\n'
     f'                let patched = {PATCHED}(payload.get("snapshot").ok_or("the reading carries no snapshot")?, member("patch")?)?;\n'
     '                replace_with_snapshot(exchange, &patched)\n'
     '            }\n'),
    (A + "📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧪️tests/📐️mutate-step-ap214/🦀️.rs",
     '        "set-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),\n',
     '        "set-snapshot" | "patch-snapshot" => return Ok(Json::Object(vec![("kind".to_string(), Json::String("set-snapshot".to_string())), ("params".to_string(), oracle_snapshot_payload(input)?)])),\n'),
    # pptx: the slide list read back as the oracle's own set-snapshot wire, patched, re-applied.
    (A + "📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '    fn apply(mut slides: Vec<PSlide>, kind: &str, params: &Json) -> Result<Vec<PSlide>, String> {\n        match kind {\n',
     '    fn apply(mut slides: Vec<PSlide>, kind: &str, params: &Json) -> Result<Vec<PSlide>, String> {\n        match kind {\n'
     '            "patch-snapshot" => {\n'
     '                let reading = inverse_spec(&slides, "set-snapshot", params)?;\n'
     '                let snapshot = reading.get("params").and_then(|params| params.get("snapshot")).ok_or("patch-snapshot: the reading carries no snapshot")?;\n'
     f'                let patched = {PATCHED}(snapshot, params.get("patch").ok_or("patch-snapshot: missing patch")?)?;\n'
     '                apply(slides, "set-snapshot", &Json::Object(vec![("snapshot".into(), patched)]))\n'
     '            }\n'),
    (A + "📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '        Ok(match kind {\n            "set-snapshot" => {\n                let slides = base.iter().map(slide_wire)',
     '        Ok(match kind {\n            "set-snapshot" | "patch-snapshot" => {\n                let slides = base.iter().map(slide_wire)'),
    # dwg: a preamble field pointer is the set-version-info it amounts to (the oracle's whole reading of the container).
    (A + "🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🔮️oracles/🦀️.rs",
     '            stub_document(&Preamble { version, maintenance_version: number(snapshot, "maintenanceVersion").unwrap_or(0.0) as u8, codepage: number(snapshot, "codepage").unwrap_or(0.0) as u16, ..current })\n        }\n',
     '            stub_document(&Preamble { version, maintenance_version: number(snapshot, "maintenanceVersion").unwrap_or(0.0) as u8, codepage: number(snapshot, "codepage").unwrap_or(0.0) as u16, ..current })\n        }\n'
     '        "patch-snapshot" => {\n'
     '            let patch = params.get("patch").ok_or("patch-snapshot: missing `patch`")?;\n'
     '            let field = match (patch.str("operation").as_str(), patch.str("path").as_str()) {\n'
     '                ("set", "/version") => "version",\n'
     '                ("set", "/maintenanceVersion") => "maintenanceVersion",\n'
     '                ("set", "/codepage") => "codepage",\n'
     '                (operation, path) => return Err(format!("patch-snapshot {operation} {path} reaches past the preamble this oracle reads")),\n'
     '            };\n'
     '            let mut fields = vec![\n'
     '                ("version".to_string(), Json::String(current.version.clone())),\n'
     '                ("maintenanceVersion".to_string(), Json::Number(f64::from(current.maintenance_version))),\n'
     '                ("codepage".to_string(), Json::Number(f64::from(current.codepage))),\n'
     '            ];\n'
     '            fields.iter_mut().filter(|(name, _)| name == field).for_each(|(_, value)| *value = patch.get("value").cloned().unwrap_or(Json::Null));\n'
     '            oracle_apply_mutation(input, &Json::Object(vec![("kind".to_string(), Json::String("set-version-info".to_string())), ("params".to_string(), Json::Object(fields))]))\n'
     '        }\n'),
    (A + "🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🔮️oracles/🦀️.rs",
     '    match spec.str("kind").as_str() {\n        "set-version-info" => {\n            let params = Json::Object(vec![\n',
     '    match spec.str("kind").as_str() {\n        "set-version-info" | "patch-snapshot" => {\n            let params = Json::Object(vec![\n'),
    # json: json-rust's own tagged-wire reading, patched; `restore-snapshot` is the oracle-internal vehicle of the inverse.
    (A + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '/// 🌳 The `value` member of a leaf payload, read through [`library_from_wire`].\n',
     '/// 🌲️ A `json::JsonValue` as the tagged `JsonValue` wire [`library_from_wire`] reads (a number as the lexeme json-rust prints).\n'
     '#[cfg(feature = "oracles")]\n'
     'fn library_to_wire(value: &json::JsonValue) -> Json {\n'
     '    let tagged = |kind: &str, member: Option<(&str, Json)>| Json::Object(std::iter::once(("kind".to_string(), Json::String(kind.to_string()))).chain(member.map(|(key, value)| (key.to_string(), value))).collect());\n'
     '    match value {\n'
     '        json::JsonValue::Null => tagged("null", None),\n'
     '        json::JsonValue::Boolean(flag) => tagged("bool", Some(("value", Json::Bool(*flag)))),\n'
     '        json::JsonValue::Number(_) => tagged("number", Some(("lexeme", Json::String(value.dump())))),\n'
     '        json::JsonValue::Short(_) | json::JsonValue::String(_) => tagged("string", Some(("value", Json::String(value.as_str().unwrap_or_default().to_string())))),\n'
     '        json::JsonValue::Array(items) => tagged("array", Some(("items", Json::Array(items.iter().map(library_to_wire).collect())))),\n'
     '        json::JsonValue::Object(object) => tagged("object", Some(("members", Json::Array(object.iter().map(|(key, member)| Json::Object(vec![("key".to_string(), Json::String(key.to_string())), ("value".to_string(), library_to_wire(member))])).collect())))),\n'
     '    }\n'
     '}\n'
     '\n'
     '/// 🩹️ The reference\'s own `JsonSnapshot` reading of `input` (`{schema, value}`) that a `patch-snapshot` row\'s pointer\n'
     '/// operation addresses, and that the case adapter restores the original through (`restore-snapshot`).\n'
     '#[cfg(feature = "oracles")]\n'
     'pub fn snapshot_wire(input: &[u8]) -> Result<Json, String> {\n'
     '    Ok(Json::Object(vec![("schema".to_string(), Json::String("stdio.json".to_string())), ("value".to_string(), library_to_wire(&read_json(input)?))]))\n'
     '}\n'
     '\n'
     '/// 🌳 The `value` member of a leaf payload, read through [`library_from_wire`].\n'),
    (A + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),\n    }\n}\n\n/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.\n#[cfg(not(feature = "oracles"))]\npub fn oracle_apply_mutation(',
     '        "patch-snapshot" => {\n'
     f'            let patched = {PATCHED}(&snapshot_wire(input)?, params.get("patch").ok_or("patch-snapshot: missing `patch`")?)?;\n'
     '            write_json(&library_from_wire(patched.get("value").unwrap_or(&Json::Null))?)\n'
     '        }\n'
     '        "restore-snapshot" => write_json(&library_from_wire(params.get("snapshot").and_then(|snapshot| snapshot.get("value")).unwrap_or(&Json::Null))?),\n'
     '        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),\n    }\n}\n\n'
     '/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.\n'
     '#[cfg(not(feature = "oracles"))]\n'
     'pub fn snapshot_wire(_input: &[u8]) -> Result<Json, String> {\n'
     '    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())\n'
     '}\n\n'
     '/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.\n#[cfg(not(feature = "oracles"))]\npub fn oracle_apply_mutation('),
    (A + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧪️tests/🔀️mutate-json-rfc8259/🦀️.rs",
     'use semio_s_artifact_stdio_json_test_oracle::standards::v_rfc8259::subsets::base::{oracle_apply_mutation, project_json_value, read_at, round_trip, PathSeg};\n',
     'use semio_s_artifact_stdio_json_test_oracle::standards::v_rfc8259::subsets::base::{oracle_apply_mutation, project_json_value, read_at, round_trip, snapshot_wire, PathSeg};\n'),
    (A + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧪️tests/🔀️mutate-json-rfc8259/🦀️.rs",
     '        other => Err(format!("no inverse rule for kind {other:?}")),\n',
     '        "patch-snapshot" => Ok(kind_spec("restore-snapshot", json_object(vec![("snapshot", snapshot_wire(original)?)]))),\n'
     '        other => Err(format!("no inverse rule for kind {other:?}")),\n'),
    # pdf 1.7: a page box pointer is the declared box kind it amounts to in lopdf's reading.
    (A + "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '    fn apply_kind(document: &mut Document, kind: &str, params: &Json) -> Result<(), String> {\n        match kind {\n',
     '    /// 🩹️ A `patch-snapshot` row as the declared kind its one pointer operation is in this oracle\'s reading: setting a\n'
     '    /// page\'s `mediaBox` or `cropBox` is `set-page-media-box` / `set-page-crop-box`; any other pointer has no reading here.\n'
     '    fn patch_as_kind(params: &Json) -> Result<(String, Json), String> {\n'
     '        let patch = params.get("patch").ok_or("patch-snapshot: missing `patch`")?;\n'
     '        let path = patch.str("path");\n'
     '        let segments: Vec<&str> = path.split(\'/\').skip(1).collect();\n'
     '        match (patch.str("operation").as_str(), segments.as_slice()) {\n'
     '            ("set", ["pages", index, field @ ("mediaBox" | "cropBox")]) if index.parse::<usize>().is_ok() => {\n'
     '                let kind = if *field == "mediaBox" { "set-page-media-box" } else { "set-page-crop-box" };\n'
     '                Ok((kind.to_string(), object(vec![("index", Json::Number(index.parse::<usize>().unwrap_or(0) as f64)), (*field, patch.get("value").cloned().unwrap_or(Json::Null))])))\n'
     '            }\n'
     '            (operation, _) => Err(format!("patch-snapshot {operation} {path} has no reading in this oracle")),\n'
     '        }\n'
     '    }\n'
     '\n'
     '    fn apply_kind(document: &mut Document, kind: &str, params: &Json) -> Result<(), String> {\n        match kind {\n'
     '            "patch-snapshot" => {\n'
     '                let (kind, params) = patch_as_kind(params)?;\n'
     '                return apply_kind(document, &kind, &params);\n'
     '            }\n'),
    (A + "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs",
     '        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);\n        Ok(match kind {\n',
     '        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);\n        Ok(match kind {\n'
     '            "patch-snapshot" => {\n'
     '                let (kind, params) = patch_as_kind(params)?;\n'
     '                return inverse_spec(document, &kind, &params);\n'
     '            }\n'),
]


def main() -> int:
    check = "--check" in sys.argv
    pending = 0
    for relative, anchor, replacement in EDITS:
        path = ROOT / relative
        text = path.read_text(encoding="utf-8")
        if replacement in text:
            continue
        if text.count(anchor) != 1:
            raise SystemExit(f"anchor not unique in {relative}: {anchor[:80]!r}")
        pending += 1
        if not check:
            path.write_text(text.replace(anchor, replacement), encoding="utf-8")
        print(f"{'pending' if check else 'applied'}: {relative.removeprefix(A)}")
    print(f"{pending} edit(s) {'pending' if check else 'applied'} of {len(EDITS)}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
