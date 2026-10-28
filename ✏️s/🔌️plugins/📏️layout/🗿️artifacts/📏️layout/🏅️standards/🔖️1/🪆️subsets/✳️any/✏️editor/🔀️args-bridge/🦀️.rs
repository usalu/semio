//! 🔀️ Host-action bridge of the layout editor: folds the shell `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into the snake_case `FromValue` payloads of `🎮️commands/*`, one arm per `LayoutCommand` row. See [`super`].

//#region 🔖️ActionBridge
use super::*;
use semio_framework_plugin::{FaultCode, FaultOrigin};

fn snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    entries.retain(|(existing, _)| existing != key);
    entries.push((key.to_string(), value));
}

/// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the `i64`/`u64` codecs
/// decode EXACT integers only, so whole finite floats get their integer variant back (`f64` fields
/// accept any `Number` variant, so nothing else changes).
fn integral(value: DslValue) -> DslValue {
    match value {
        DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
            if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
        }
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
        other => other,
    }
}

/// 📝️ Prints a host control value into the `String` field the patch/engagement verbs carry
/// (`"512"`, `"true"`, or the text itself) — the reducers re-parse per field.
fn stringify(value: DslValue) -> DslValue {
    match value {
        DslValue::String(_) => value,
        DslValue::Null => DslValue::String(String::new()),
        other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
    }
}

/// 🔁️ Snake-cases every key of `args`, applies `aliases` (snake_case source → destination, first
/// present source wins and never overwrites a present destination) and seeds `defaults` for keys
/// still absent.
fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
    let mut entries: Vec<(String, DslValue)> = Vec::new();
    if let Some(DslValue::Object(object)) = args {
        for (key, value) in object {
            put(&mut entries, &snake(key), integral(value.clone()));
        }
    }
    for (from, into) in aliases {
        if entries.iter().any(|(key, _)| key == into) {
            continue;
        }
        if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
            put(&mut entries, into, value);
        }
    }
    for (key, value) in defaults {
        if !entries.iter().any(|(existing, _)| existing == key) {
            entries.push(((*key).to_string(), value.clone()));
        }
    }
    DslValue::Object(entries)
}

/// 📷️ The canvas nests its pose under `camera`; a flat `{x, y, zoom}` payload is admitted as the
/// pose itself.
fn nest_camera(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if !entries.iter().any(|(key, _)| key == "camera") {
            let pose = DslValue::Object(entries.iter().filter(|(key, _)| matches!(key.as_str(), "x" | "y" | "zoom")).cloned().collect());
            entries.push(("camera".into(), pose));
        }
    }
    folded
}

fn with_text_value(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
            slot.1 = stringify(slot.1.clone());
        }
    }
    folded
}

fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("layout action '{action}' arguments do not decode: {error}")))
}

fn catalogue_args(action: &str, args: Option<&DslValue>) -> Result<DslValue, Fault> {
    use catalogue_panel::{catalogue_kind, LAYOUT_CATALOGUE_DRAG_MIME, LAYOUT_CATALOGUE_KIND_MIME_PREFIX};
    let invalid = || Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("layout action '{action}' requires one supported catalogue kind"));
    let mut folded = fold(args, &[], &[]);
    let DslValue::Object(entries) = &mut folded else { return Err(invalid()) };
    let field = |name: &str| entries.iter().find_map(|(key, value)| (key == name).then_some(value));
    let kind = if action == "canvasDragOver" {
        let Some(DslValue::Array(types)) = field("types") else { return Err(invalid()) };
        let mut has_base = false;
        let mut kind = None;
        for value in types {
            let DslValue::String(mime) = value else { return Err(invalid()) };
            has_base |= mime == LAYOUT_CATALOGUE_DRAG_MIME;
            if let Some(candidate) = mime.strip_prefix(LAYOUT_CATALOGUE_KIND_MIME_PREFIX) {
                if kind.is_some() { return Err(invalid()); }
                kind = Some(catalogue_kind(candidate).ok_or_else(invalid)?);
            }
        }
        if !has_base { return Err(invalid()); }
        kind.ok_or_else(invalid)?
    } else {
        let Some(DslValue::String(raw)) = field("drag_data") else { return Err(invalid()) };
        let value = semio_framework_pack_json::from_json_str::<DslValue>(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| invalid())?;
        let DslValue::Object(payload) = value else { return Err(invalid()) };
        let mut kind = None;
        let mut artifact_ref = None;
        let mut proxy_data_url = None;
        for (key, item) in payload {
            let DslValue::String(text) = item else { return Err(invalid()) };
            match key.as_str() {
                "kind" => kind = Some(catalogue_kind(&text).ok_or_else(invalid)?),
                "artifactRef" | "artifact_ref" => artifact_ref = Some(text.clone()),
                "proxyDataUrl" | "proxy_data_url" => proxy_data_url = Some(text.clone()),
                _ => return Err(invalid()),
            }
        }
        let kind = kind.ok_or_else(invalid)?;
        entries.retain(|(key, _)| !matches!(key.as_str(), "types" | "drag_data"));
        put(entries, "kind", DslValue::String(kind.into()));
        if let Some(reference) = artifact_ref {
            put(entries, "artifact_ref", DslValue::String(reference));
        }
        if let Some(proxy) = proxy_data_url {
            put(entries, "proxy_data_url", DslValue::String(proxy));
        }
        return Ok(folded);
    };
    entries.retain(|(key, _)| !matches!(key.as_str(), "types" | "drag_data"));
    put(entries, "kind", DslValue::String(kind.into()));
    Ok(folded)
}

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<LayoutCommand, Fault> {
    const PAGE: &[(&str, &str)] = &[("id", "page_id"), ("value", "page_id")];
    const TEXT: &[(&str, &str)] = &[("text", "value"), ("input", "value")];
    let text = |value: &str| DslValue::String(value.into());
    let plain = || fold(args, &[], &[]);
    Ok(match action {
        "setActivePage" => LayoutCommand::SetActivePage(decode(action, fold(args, PAGE, &[]))?),
        "focusPreflightIssue" => LayoutCommand::FocusPreflightIssue(decode(action, fold(args, &[("id", "object_id"), ("frame_id", "object_id")], &[]))?),
        "engagementInput" => LayoutCommand::EngagementInput(decode(action, with_text_value(fold(args, TEXT, &[("value", text(""))])))?),
        "canvasPointerDown" => LayoutCommand::CanvasPointerDown(decode(action, fold(args, &[("shift_key", "extend")], &[("button", DslValue::Number(semio_framework_value::Number::Int(0))), ("extend", DslValue::Bool(false))]))?),
        "canvasPointerMove" => LayoutCommand::CanvasPointerMove(decode(action, fold(args, &[], &[("samples", DslValue::Array(Vec::new()))]))?),
        "canvasPointerUp" => LayoutCommand::CanvasPointerUp(decode(action, fold(args, &[], &[("cancelled", DslValue::Bool(false))]))?),
        "canvasDragOver" => LayoutCommand::CanvasDragOver(decode(action, catalogue_args(action, args)?)?),
        "canvasDragLeave" => LayoutCommand::CanvasDragLeave(decode(action, plain())?),
        "setCamera" => LayoutCommand::SetCamera(decode(action, nest_camera(plain()))?),
        "addFrame" => LayoutCommand::AddFrame(decode(action, fold(args, &[("value", "kind")], &[("kind", text("rect"))]))?),
        "addPage" => LayoutCommand::AddPage(decode(action, plain())?),
        "patchPage" => LayoutCommand::PatchPage(decode(action, with_text_value(fold(args, &[("id", "page_id")], &[("value", text(""))])))?),
        "patchFrame" => LayoutCommand::PatchFrame(decode(action, with_text_value(fold(args, &[("id", "frame_id")], &[("value", text(""))])))?),
        "canvasDrop" => LayoutCommand::CanvasDrop(decode(action, catalogue_args(action, args)?)?),
        "exportPng" => LayoutCommand::ExportPng(decode(action, fold(args, PAGE, &[]))?),
        "exportSvg" => LayoutCommand::ExportSvg(decode(action, fold(args, PAGE, &[]))?),
        "exportPdf" => LayoutCommand::ExportPdf(decode(action, fold(args, PAGE, &[]))?),
        "exportStep" => LayoutCommand::ExportStep(decode(action, fold(args, PAGE, &[]))?),
        "exportIdml" => LayoutCommand::ExportIdml(decode(action, plain())?),
        "exportPackage" => LayoutCommand::ExportPackage(decode(action, plain())?),
        "engagementSubmit" => LayoutCommand::EngagementSubmit(decode(action, with_text_value(fold(args, TEXT, &[("value", text(""))])))?),
        "deleteSelection" => LayoutCommand::DeleteSelection(decode(action, plain())?),
        "translateSelection" => LayoutCommand::TranslateSelection(decode(action, plain())?),
        "rotateSelection" => LayoutCommand::RotateSelection(decode(action, plain())?),
        "scaleSelection" => LayoutCommand::ScaleSelection(decode(action, plain())?),
        "patchDocument" => LayoutCommand::PatchDocument(decode(action, with_text_value(fold(args, &[], &[("value", text(""))])))?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the layout editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
