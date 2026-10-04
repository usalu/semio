//! 🎛️ Closed Diagnostic projection with caller-owned admission and bounded borrowed field scans.
use super::*;
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, Number};
type Fields = DecodedValue<Vec<(String, DslValue)>>;

fn fields<'a, const N: usize>(value: &'a DslValue, names: [&str; N], c: &mut NativeDecodeControl<'_>) -> Result<[Option<&'a DslValue>; N], ValueError> {
    c.scoped_stage(|c| {
        c.checkpoint()?;
        let DslValue::Object(entries) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected diagnostic object")) };
        if entries.len() > N { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown or duplicate diagnostic field")) }
        c.begin_stage(entries.len())?;
        let mut output = [None; N];
        for (name, value) in entries {
            let index = names.iter().position(|candidate| *candidate == name).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown diagnostic field"))?;
            if output[index].replace(value).is_some() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "duplicate diagnostic field")) }
            c.step()?;
        }
        Ok(output)
    })
}
fn required<'a>(value: Option<&'a DslValue>, message: &'static str) -> Result<&'a DslValue, ValueError> { value.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) }
fn text(value: &DslValue, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<String, ValueError> {
    c.checkpoint()?;
    let DslValue::String(value) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) };
    c.copy_text(value)
}
fn optional_text(value: Option<&DslValue>, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<Option<String>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(None) }, Some(value) => text(value, message, c).map(Some) }
}
fn optional<T: FromValue>(value: Option<&DslValue>, c: &mut NativeDecodeControl<'_>) -> Result<Option<T>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(None) }, Some(value) => T::from_value_controlled(value, c).map(Some) }
}
fn strings(value: Option<&DslValue>, message: &'static str, c: &mut NativeDecodeControl<'_>) -> Result<Vec<String>, ValueError> {
    match value { None | Some(DslValue::Null) => { c.checkpoint()?; Ok(Vec::new()) }, Some(DslValue::Array(_)) => Vec::<String>::from_value_controlled(value.unwrap(), c), Some(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)) }
}
fn scalar(c: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { c.scoped_stage(|c| { c.begin_stage(1)?; c.step() }) }
fn record(c: &mut NativeEncodeControl<'_>, count: usize, build: impl FnOnce(&mut Fields, &mut NativeEncodeControl<'_>) -> Result<(), ValueError>) -> Result<DslValue, ValueError> {
    c.scoped_stage(|c| {
        c.begin_stage(count)?;
        let mut fields = DslValue::object_encoding_controlled(count, c)?;
        build(&mut fields, c)?;
        c.checkpoint()?;
        Ok(DslValue::Object(fields.take()))
    })
}
fn push<T: ToValue + ?Sized>(fields: &mut Fields, name: &str, value: &T, c: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    let value = value.to_value_controlled(c)?;
    DslValue::push_encoding_controlled(fields.get_mut(), name, value, c)?;
    c.step()
}
pub(super) fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let mut cursor = semio_framework_value::retirement::owned_retirement(value);
    while !cursor.terminal_is_empty() { cursor.close_step(256, 65536).expect("diagnostic retirement respects bounded grant"); }
}

pub(crate) fn encode_span(value: &TextSpan, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 3, |fields, c| { push(fields, "line", &value.line, c)?; push(fields, "column", &value.column, c)?; push(fields, "length", &value.length, c) })
}
pub(crate) fn decode_span(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<TextSpan, ValueError> {
    let [line, column, length] = fields(value, ["line", "column", "length"], c)?;
    let number = |value: Option<&DslValue>, message, c: &mut NativeDecodeControl<'_>| -> Result<u32, ValueError> {
        scalar(c)?;
        match required(value, message)? {
            DslValue::Number(Number::Float(value)) if value.is_finite() && value.fract() == 0.0 && *value >= 0.0 && *value <= u32::MAX as f64 => Ok(*value as u32),
            DslValue::Number(value) => value.as_u64().and_then(|value| u32::try_from(value).ok()).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an exact u32 for TextSpan")),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a number for TextSpan")),
        }
    };
    Ok(TextSpan { line: number(line, "missing TextSpan.line", c)?, column: number(column, "missing TextSpan.column", c)?, length: number(length, "missing TextSpan.length", c)? })
}
pub(super) fn refusal_kind(value: &DslValue) -> Result<ValueRefusalKind, ValueError> {
    match value { DslValue::String(value) => match value.as_str() { "invalidValue" => Ok(ValueRefusalKind::InvalidValue), "canceled" => Ok(ValueRefusalKind::Canceled), "ownershipLimit" => Ok(ValueRefusalKind::OwnershipLimit), "allocationFailed" => Ok(ValueRefusalKind::AllocationFailed), "workLimit" => Ok(ValueRefusalKind::WorkLimit), "depthLimit" => Ok(ValueRefusalKind::DepthLimit), "unsupportedOwner" => Ok(ValueRefusalKind::UnsupportedOwner), "invariantViolated" => Ok(ValueRefusalKind::InvariantViolated), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown TextError.kind")) }, _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected a string for TextError.kind")) }
}
pub(super) fn encode_text_error(value: &TextError, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 3 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "kind", value.kind.as_str(), c)?; push(fields, "message", &value.message, c)?; push(fields, "span", &value.span, c)?; if let Some(value) = &value.expected { push(fields, "expected", value, c)?; } Ok(()) })
}
pub(super) fn decode_text_error(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<TextError, ValueError> {
    let [kind, message, span, expected] = fields(value, ["kind", "message", "span", "expected"], c)?;
    scalar(c)?;
    let kind = refusal_kind(required(kind, "missing TextError.kind")?)?;
    let message = text(required(message, "missing TextError.message")?, "expected a string for TextError.message", c)?;
    let span = TextSpan::from_value_controlled(required(span, "missing TextError.span")?, c)?;
    let expected = optional_text(expected, "expected a string for TextError.expected", c)?;
    Ok(TextError { kind, message, span, expected })
}
pub(super) fn encode_code(value: &FaultCode, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { value.0.to_value_controlled(c) }
pub(super) fn decode_code(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultCode, ValueError> { text(value, "expected a string for FaultCode", c).map(FaultCode) }
pub(super) fn encode_severity(value: &Severity, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { match value { Severity::Info => "info", Severity::Warning => "warning", Severity::Error => "error", Severity::Fatal => "fatal" }.to_value_controlled(c) }
pub(super) fn decode_severity(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Severity, ValueError> {
    scalar(c)?;
    match value { DslValue::String(value) => match value.as_str() { "info" => Ok(Severity::Info), "warning" => Ok(Severity::Warning), "error" => Ok(Severity::Error), "fatal" => Ok(Severity::Fatal), _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown Severity variant")) }, _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for Severity")) }
}
pub(super) fn encode_origin(value: &FaultOrigin, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { match value { FaultOrigin::Edge => "edge", FaultOrigin::Renderer => "renderer", FaultOrigin::Os => "os", FaultOrigin::Module => "module", FaultOrigin::Plugin => "plugin", FaultOrigin::App => "app", FaultOrigin::Extension => "extension", FaultOrigin::Framework => "framework" }.to_value_controlled(c) }
pub(super) fn decode_origin(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultOrigin, ValueError> {
    scalar(c)?;
    match value { DslValue::String(value) => match value.as_str() { "edge" => Ok(FaultOrigin::Edge), "renderer" => Ok(FaultOrigin::Renderer), "os" => Ok(FaultOrigin::Os), "module" => Ok(FaultOrigin::Module), "plugin" => Ok(FaultOrigin::Plugin), "app" => Ok(FaultOrigin::App), "extension" => Ok(FaultOrigin::Extension), "framework" => Ok(FaultOrigin::Framework), _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown FaultOrigin variant")) }, _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string for FaultOrigin")) }
}
pub(super) fn encode_expected(value: &ExpectedSet, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { record(c, 3, |fields, c| { push(fields, "tokens", &value.tokens, c)?; push(fields, "keywords", &value.keywords, c)?; push(fields, "keys", &value.keys, c) }) }
pub(super) fn decode_expected(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<ExpectedSet, ValueError> {
    let [tokens, keywords, keys] = fields(value, ["tokens", "keywords", "keys"], c)?;
    let tokens = strings(tokens, "expected an array for ExpectedSet.tokens", c)?.guard_decoded();
    let keywords = strings(keywords, "expected an array for ExpectedSet.keywords", c)?.guard_decoded();
    let keys = strings(keys, "expected an array for ExpectedSet.keys", c)?.guard_decoded();
    Ok(ExpectedSet { tokens: tokens.take(), keywords: keywords.take(), keys: keys.take() })
}
pub(super) fn encode_scope(value: &FaultScope, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    let values = [("pluginId", &value.plugin_id), ("appId", &value.app_id), ("instanceId", &value.instance_id), ("module", &value.module), ("bodyKey", &value.body_key)];
    record(c, values.iter().filter(|(_, value)| value.is_some()).count(), |fields, c| { for (key, value) in values { if let Some(value) = value { push(fields, key, value, c)?; } } Ok(()) })
}
pub(super) fn decode_scope(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultScope, ValueError> {
    let [plugin_id, app_id, instance_id, module, body_key] = fields(value, ["pluginId", "appId", "instanceId", "module", "bodyKey"], c)?;
    Ok(FaultScope { plugin_id: optional_text(plugin_id, "expected a string for FaultScope.pluginId", c)?, app_id: optional_text(app_id, "expected a string for FaultScope.appId", c)?, instance_id: optional_text(instance_id, "expected a string for FaultScope.instanceId", c)?, module: optional_text(module, "expected a string for FaultScope.module", c)?, body_key: optional_text(body_key, "expected a string for FaultScope.bodyKey", c)? })
}
pub(super) fn encode_cause(value: &FaultCause, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { record(c, 1 + usize::from(value.code.is_some()), |fields, c| { push(fields, "message", &value.message, c)?; if let Some(value) = &value.code { push(fields, "code", value, c)?; } Ok(()) }) }
pub(super) fn decode_cause(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<FaultCause, ValueError> {
    let [message, code] = fields(value, ["message", "code"], c)?;
    Ok(FaultCause { message: text(required(message, "missing FaultCause.message")?, "expected a string for FaultCause.message", c)?, code: optional(code, c)? })
}
pub(super) fn encode_diagnostic(value: &Diagnostic, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 5 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "code", &value.code, c)?; push(fields, "severity", &value.severity, c)?; push(fields, "span", &value.span, c)?; push(fields, "message", &value.message, c)?; if let Some(value) = &value.expected { push(fields, "expected", value, c)?; } push(fields, "scope", &value.scope, c) })
}
pub(super) fn decode_diagnostic(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Diagnostic, ValueError> {
    let [code, severity, span, message, expected, scope] = fields(value, ["code", "severity", "span", "message", "expected", "scope"], c)?;
    let code = FaultCode::from_value_controlled(required(code, "missing Diagnostic.code")?, c)?.guard_decoded();
    let severity = Severity::from_value_controlled(required(severity, "missing Diagnostic.severity")?, c)?;
    let span = TextSpan::from_value_controlled(required(span, "missing Diagnostic.span")?, c)?;
    let message = text(required(message, "missing Diagnostic.message")?, "expected a string for Diagnostic.message", c)?.guard_decoded();
    let expected = optional::<ExpectedSet>(expected, c)?.guard_decoded();
    let scope = optional::<FaultScope>(scope, c)?.unwrap_or_default().guard_decoded();
    Ok(Diagnostic { code: code.take(), severity, span, message: message.take(), expected: expected.take(), scope: scope.take() })
}
pub(super) fn encode_fault(value: &Fault, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 6 + usize::from(value.span.is_some()) + usize::from(!value.causes.is_empty()), |fields, c| { push(fields, "origin", &value.origin, c)?; push(fields, "code", &value.code, c)?; push(fields, "severity", &value.severity, c)?; push(fields, "message", &value.message, c)?; push(fields, "scope", &*value.scope, c)?; if let Some(value) = &value.span { push(fields, "span", value, c)?; } if !value.causes.is_empty() { push(fields, "causes", &value.causes, c)?; } push(fields, "retryable", &value.retryable, c) })
}
pub(super) fn decode_fault(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<Fault, ValueError> {
    let [origin, code, severity, message, scope, span, causes, retryable] = fields(value, ["origin", "code", "severity", "message", "scope", "span", "causes", "retryable"], c)?;
    let origin = FaultOrigin::from_value_controlled(required(origin, "missing Fault.origin")?, c)?;
    let code = FaultCode::from_value_controlled(required(code, "missing Fault.code")?, c)?.guard_decoded();
    let severity = Severity::from_value_controlled(required(severity, "missing Fault.severity")?, c)?;
    let message = text(required(message, "missing Fault.message")?, "expected a string for Fault.message", c)?.guard_decoded();
    c.charge(size_of::<FaultScope>())?;
    let scope = optional::<FaultScope>(scope, c)?.unwrap_or_default().guard_decoded();
    let span = optional::<TextSpan>(span, c)?;
    let causes = optional::<Vec<FaultCause>>(causes, c)?.unwrap_or_default().guard_decoded();
    let retryable = optional::<bool>(retryable, c)?.unwrap_or(false);
    c.checkpoint()?;
    Ok(Fault { origin, code: code.take(), severity, message: message.take(), scope: Box::new(scope.take()), span, causes: causes.take(), retryable })
}
