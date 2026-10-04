# Diagnostic TextError Mandatory Kind Rust Production

2026-10-03T00:24:37.000Z

Genuine Diagnostic compiler RED had10diagnostics/no discovery/no runtime. This actual four-source cut introduces mandatory direct Value-owned kind, explicit constructors and Diagnostic conversion kind, moving from_value_error with caller span, bounded controlled wire kind codec and explicit actual Limits cause sites. Original 39 corpus semantics require authored kind at existing three text syntax cases. Every original body/message/span is retained in complete before captures. No legacy constructor, kind default, implicit span or String classifier is introduced. Copy-only kind is discarded only during owner retirement while original bounded owned fields retain their retirement.

High owns the unfinished actual higher caller cut. No dependent native consumer is launched before its complete coherent selected SourceReady. Historical original Value123/JSON48/Diagnostic9 proofs and current test-only RED are retained; native Diagnostic10 GREEN is not claimed yet. Old unchecked trait methods and fault transport byte APIs remain the declared next contract retirement frontier.

## 🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs

Before SHA-256 a8697bbbfb27b9a8238add77ebfe95430e644a6bd358fdcbfde6edd7b73b8a8c; after e693361a35adc95ea4e32dc4c5818ecb9946cc6eb254496bfb5838aa62d6ffaa

Full immediate original / inverse:

```
//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

pub use crate::span::TextSpan;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
#[path = "🎛️controlled/🦀️.rs"]
pub(crate) mod controlled;

//#region 🔖️Errors
/// 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq)]
pub struct TextError {
    pub message: String,
    pub span: TextSpan,
    pub expected: Option<String>,
}

impl std::fmt::Display for TextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
    }
}

impl std::error::Error for TextError {}

/// 🌉️ Hand-written, not derived — same DAG reason as `FaultCode`/`Severity` above. No
/// `#[serde(rename_all = …)]` on the original, so field names stay as declared.
impl ToValue for TextError {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_text_error(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), DslValue::String(expected.clone())));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for TextError {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_text_error(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TextError, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
        };
        let span = match find("span") {
            Some(slot) => TextSpan::from_value(slot)?,
            None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing TextError.span")),
        };
        let expected = match find("expected") {
            None | Some(DslValue::Null) => None,
            Some(DslValue::String(text)) => Some(text),
            Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.expected, found {other:?}"))),
        };
        Ok(TextError { message, span, expected })
    }
}

impl TextError {
    pub fn new(message: impl Into<String>, span: TextSpan) -> Self {
        Self { message: message.into(), span, expected: None }
    }

    pub fn expected(message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { message: message.into(), span, expected: Some(expected.into()) }
    }

    pub fn from_diagnostic(diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error()
    }
}

/// 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// 🌉️ Hand-written, not derived: `FaultCode` is `#[serde(transparent)]`, a shape
/// `#[derive(ToValue, FromValue)]` does not support (see its own module docstring).
impl ToValue for FaultCode {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_code(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.clone())
    }
}
impl FromValue for FaultCode {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_code(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(FaultCode(s)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCode, found {other:?}"))),
        }
    }
}

/// 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

/// 🚦️ Declaration order is the level order: `#[derive(PartialOrd, Ord)]` makes `Info < Warning <
/// Error < Fatal` a structural fact, not a hand-maintained comparator. `as_u8`/`from_u8` (0..3)
/// give a stable wire-compatible numeric mirror for TS/WIT twins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌉️ Hand-written, not derived: `Severity` is a plain unit-only "string enum" (`#[serde(rename_all
/// = "camelCase")]` with no `tag`, serde's default bare-string representation) —
/// `#[derive(ToValue, FromValue)]`'s enum path only supports internally-tagged representations.
impl ToValue for Severity {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_severity(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for Severity {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_severity(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "info" => Ok(Severity::Info),
                "warning" => Ok(Severity::Warning),
                "error" => Ok(Severity::Error),
                "fatal" => Ok(Severity::Fatal),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Severity variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl Severity {
    /// 🔢️ Stable numeric mirror of declaration order, 0..3.
    pub fn as_u8(self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..3.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Severity::Info),
            1 => Some(Severity::Warning),
            2 => Some(Severity::Error),
            3 => Some(Severity::Fatal),
            _ => None,
        }
    }
}

/// 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for ExpectedSet {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_expected(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("tokens".to_string(), DslValue::Array(self.tokens.iter().cloned().map(DslValue::String).collect())),
            ("keywords".to_string(), DslValue::Array(self.keywords.iter().cloned().map(DslValue::String).collect())),
            ("keys".to_string(), DslValue::Array(self.keys.iter().cloned().map(DslValue::String).collect())),
        ])
    }
}
impl FromValue for ExpectedSet {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_expected(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ExpectedSet, found {other:?}"))),
        };
        let strings = |key: &str| -> Result<Vec<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(Vec::new()),
                Some(DslValue::Array(items)) => items
                    .iter()
                    .map(|item| match item {
                        DslValue::String(text) => Ok(text.clone()),
                        other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string in ExpectedSet.{key}, found {other:?}"))),
                    })
                    .collect(),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for ExpectedSet.{key}, found {other:?}"))),
            }
        };
        Ok(ExpectedSet { tokens: strings("tokens")?, keywords: strings("keywords")?, keys: strings("keys")? })
    }
}

/// 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` at API boundaries that predate diagnostics.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(self.message, self.span, expected),
            None => TextError::new(self.message, self.span),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Diagnostic {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_diagnostic(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("span".to_string(), self.span.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
        ];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), expected.to_value()));
        }
        entries.push(("scope".to_string(), self.scope.to_value()));
        DslValue::Object(entries)
    }
}
impl FromValue for Diagnostic {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_diagnostic(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Diagnostic, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Diagnostic.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
        };
        Ok(Diagnostic {
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            span: TextSpan::from_value(required("span")?)?,
            message,
            expected: match find("expected") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(ExpectedSet::from_value(slot)?),
            },
            scope: match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            },
        })
    }
}

//#region 🔖️Fault
/// 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FaultScope {
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub module: Option<String>,
    pub body_key: Option<String>,
}

/// 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultCause {
    pub message: String,
    pub code: Option<FaultCode>,
}

/// 🧯️ Structured abort report crossing every os boundary.
/// Scope metadata owns a separate allocation to keep error return values compact.
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    pub scope: Box<FaultScope>,
    pub span: Option<TextSpan>,
    pub causes: Vec<FaultCause>,
    pub retryable: bool,
}

/// 🌉️ Owned Value conversion preserves the canonical Diagnostic fault wire.
/// Literal camelCase keys and absent optional fields match the declared fault schema.
impl ToValue for FaultOrigin {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_origin(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            FaultOrigin::Edge => "edge",
            FaultOrigin::Renderer => "renderer",
            FaultOrigin::Os => "os",
            FaultOrigin::Module => "module",
            FaultOrigin::Plugin => "plugin",
            FaultOrigin::App => "app",
            FaultOrigin::Extension => "extension",
            FaultOrigin::Framework => "framework",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FaultOrigin {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_origin(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "edge" => Ok(FaultOrigin::Edge),
                "renderer" => Ok(FaultOrigin::Renderer),
                "os" => Ok(FaultOrigin::Os),
                "module" => Ok(FaultOrigin::Module),
                "plugin" => Ok(FaultOrigin::Plugin),
                "app" => Ok(FaultOrigin::App),
                "extension" => Ok(FaultOrigin::Extension),
                "framework" => Ok(FaultOrigin::Framework),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FaultOrigin variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultScope {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_scope(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        for (key, slot) in [
            ("pluginId", &self.plugin_id),
            ("appId", &self.app_id),
            ("instanceId", &self.instance_id),
            ("module", &self.module),
            ("bodyKey", &self.body_key),
        ] {
            if let Some(text) = slot {
                entries.push((key.to_string(), DslValue::String(text.clone())));
            }
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultScope {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_scope(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultScope, found {other:?}"))),
        };
        let take = |key: &str| -> Result<Option<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(None),
                Some(DslValue::String(text)) => Ok(Some(text.clone())),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultScope.{key}, found {other:?}"))),
            }
        };
        Ok(FaultScope {
            plugin_id: take("pluginId")?,
            app_id: take("appId")?,
            instance_id: take("instanceId")?,
            module: take("module")?,
            body_key: take("bodyKey")?,
        })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultCause {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_cause(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
        if let Some(code) = &self.code {
            entries.push(("code".to_string(), code.to_value()));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultCause {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_cause(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultCause, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
        };
        let code = match find("code") {
            None | Some(DslValue::Null) => None,
            Some(slot) => Some(FaultCode::from_value(slot)?),
        };
        Ok(FaultCause { message, code })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Fault {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_fault(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("origin".to_string(), self.origin.to_value()),
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
            ("scope".to_string(), self.scope.to_value()),
        ];
        if let Some(span) = &self.span {
            entries.push(("span".to_string(), span.to_value()));
        }
        if !self.causes.is_empty() {
            entries.push(("causes".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));
        }
        entries.push(("retryable".to_string(), DslValue::Bool(self.retryable)));
        DslValue::Object(entries)
    }
}
impl FromValue for Fault {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_fault(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Fault, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Fault.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
        };
        Ok(Fault {
            origin: FaultOrigin::from_value(required("origin")?)?,
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            message,
            scope: Box::new(match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            }),
            span: match find("span") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(TextSpan::from_value(slot)?),
            },
            causes: match find("causes") {
                None | Some(DslValue::Null) => Vec::new(),
                Some(DslValue::Array(items)) => items.into_iter().map(FaultCause::from_value).collect::<Result<_, _>>()?,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for Fault.causes, found {other:?}"))),
            },
            retryable: match find("retryable") {
                None | Some(DslValue::Null) => false,
                Some(DslValue::Bool(flag)) => flag,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a bool for Fault.retryable, found {other:?}"))),
            },
        })
    }
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), retryable: false }
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        *self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// 🗣️ Canonical one-line rendering of a fault for `String` error channels.
    ///
    /// `Fault` deliberately has no `Display`: a structured abort must not be silently interpolated
    /// into prose, and `to_string()` would hide the {@link FaultCode} every triage tool keys on.
    /// Every boundary that has to collapse a fault into text calls this instead, so the
    /// `code: message` shape is written once rather than re-derived per crate.
    pub fn describe(&self) -> String {
        format!("{}: {}", self.code.0, self.message)
    }
}

/// 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }

    fn into_fault(self) -> Fault
    where
        Self: Sized,
    {
        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), retryable: self.fault_retryable() }
    }
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

/// 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

/// 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    serde_json::from_slice::<DslValue>(bytes).ok().and_then(|value| Fault::from_value(value).ok()).unwrap_or_else(|| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// 🔁️ Maps an error type into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_error {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️FaultDescribeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fault-describe/🦀️.rs"]
mod fault_describe_tests;
#[cfg(test)]
#[path = "🧪️tests/🎛️controlled/🦀️.rs"]
mod controlled_tests;
//#endregion 🔖️FaultDescribeTests

#[cfg(test)]
#[path = "🚧️text-error/🧪️tests/🦀️.rs"]
mod text_error_refusal_tests;

```

Full authored production source:

```
//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

pub use crate::span::TextSpan;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
#[path = "🎛️controlled/🦀️.rs"]
pub(crate) mod controlled;

//#region 🔖️Errors
/// 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq)]
pub struct TextError {
    pub kind: ValueRefusalKind,
    pub message: String,
    pub span: TextSpan,
    pub expected: Option<String>,
}

impl std::fmt::Display for TextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
    }
}

impl std::error::Error for TextError {}

/// 🌉️ Hand-written, not derived — same DAG reason as `FaultCode`/`Severity` above. No
/// `#[serde(rename_all = …)]` on the original, so field names stay as declared.
impl ToValue for TextError {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_text_error(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("kind".to_string(), DslValue::String(self.kind.as_str().into())), ("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), DslValue::String(expected.clone())));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for TextError {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_text_error(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TextError, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let kind = match find("kind") { Some(value) => controlled::refusal_kind(&value)?, None => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "missing TextError.kind")) };
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
        };
        let span = match find("span") {
            Some(slot) => TextSpan::from_value(slot)?,
            None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing TextError.span")),
        };
        let expected = match find("expected") {
            None | Some(DslValue::Null) => None,
            Some(DslValue::String(text)) => Some(text),
            Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.expected, found {other:?}"))),
        };
        Ok(TextError { kind, message, span, expected })
    }
}

impl TextError {
    pub fn new(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan) -> Self {
        Self { kind, message: message.into(), span, expected: None }
    }

    pub fn expected(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { kind, message: message.into(), span, expected: Some(expected.into()) }
    }

    /// 🧭️ Retains owned refusal authority at the caller-authored source position.
    pub fn from_value_error(error: ValueError, span: TextSpan) -> Self { Self::new(error.kind, error.message, span) }

    pub fn from_diagnostic(kind: ValueRefusalKind, diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error(kind)
    }
}

/// 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// 🌉️ Hand-written, not derived: `FaultCode` is `#[serde(transparent)]`, a shape
/// `#[derive(ToValue, FromValue)]` does not support (see its own module docstring).
impl ToValue for FaultCode {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_code(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.clone())
    }
}
impl FromValue for FaultCode {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_code(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(FaultCode(s)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCode, found {other:?}"))),
        }
    }
}

/// 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

/// 🚦️ Declaration order is the level order: `#[derive(PartialOrd, Ord)]` makes `Info < Warning <
/// Error < Fatal` a structural fact, not a hand-maintained comparator. `as_u8`/`from_u8` (0..3)
/// give a stable wire-compatible numeric mirror for TS/WIT twins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌉️ Hand-written, not derived: `Severity` is a plain unit-only "string enum" (`#[serde(rename_all
/// = "camelCase")]` with no `tag`, serde's default bare-string representation) —
/// `#[derive(ToValue, FromValue)]`'s enum path only supports internally-tagged representations.
impl ToValue for Severity {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_severity(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for Severity {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_severity(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "info" => Ok(Severity::Info),
                "warning" => Ok(Severity::Warning),
                "error" => Ok(Severity::Error),
                "fatal" => Ok(Severity::Fatal),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Severity variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl Severity {
    /// 🔢️ Stable numeric mirror of declaration order, 0..3.
    pub fn as_u8(self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..3.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Severity::Info),
            1 => Some(Severity::Warning),
            2 => Some(Severity::Error),
            3 => Some(Severity::Fatal),
            _ => None,
        }
    }
}

/// 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for ExpectedSet {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_expected(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("tokens".to_string(), DslValue::Array(self.tokens.iter().cloned().map(DslValue::String).collect())),
            ("keywords".to_string(), DslValue::Array(self.keywords.iter().cloned().map(DslValue::String).collect())),
            ("keys".to_string(), DslValue::Array(self.keys.iter().cloned().map(DslValue::String).collect())),
        ])
    }
}
impl FromValue for ExpectedSet {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_expected(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ExpectedSet, found {other:?}"))),
        };
        let strings = |key: &str| -> Result<Vec<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(Vec::new()),
                Some(DslValue::Array(items)) => items
                    .iter()
                    .map(|item| match item {
                        DslValue::String(text) => Ok(text.clone()),
                        other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string in ExpectedSet.{key}, found {other:?}"))),
                    })
                    .collect(),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for ExpectedSet.{key}, found {other:?}"))),
            }
        };
        Ok(ExpectedSet { tokens: strings("tokens")?, keywords: strings("keywords")?, keys: strings("keys")? })
    }
}

/// 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` with the caller's explicit refusal authority.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self, kind: ValueRefusalKind) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(kind, self.message, self.span, expected),
            None => TextError::new(kind, self.message, self.span),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Diagnostic {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_diagnostic(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("span".to_string(), self.span.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
        ];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), expected.to_value()));
        }
        entries.push(("scope".to_string(), self.scope.to_value()));
        DslValue::Object(entries)
    }
}
impl FromValue for Diagnostic {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_diagnostic(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Diagnostic, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Diagnostic.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
        };
        Ok(Diagnostic {
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            span: TextSpan::from_value(required("span")?)?,
            message,
            expected: match find("expected") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(ExpectedSet::from_value(slot)?),
            },
            scope: match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            },
        })
    }
}

//#region 🔖️Fault
/// 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FaultScope {
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub module: Option<String>,
    pub body_key: Option<String>,
}

/// 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultCause {
    pub message: String,
    pub code: Option<FaultCode>,
}

/// 🧯️ Structured abort report crossing every os boundary.
/// Scope metadata owns a separate allocation to keep error return values compact.
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    pub scope: Box<FaultScope>,
    pub span: Option<TextSpan>,
    pub causes: Vec<FaultCause>,
    pub retryable: bool,
}

/// 🌉️ Owned Value conversion preserves the canonical Diagnostic fault wire.
/// Literal camelCase keys and absent optional fields match the declared fault schema.
impl ToValue for FaultOrigin {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_origin(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            FaultOrigin::Edge => "edge",
            FaultOrigin::Renderer => "renderer",
            FaultOrigin::Os => "os",
            FaultOrigin::Module => "module",
            FaultOrigin::Plugin => "plugin",
            FaultOrigin::App => "app",
            FaultOrigin::Extension => "extension",
            FaultOrigin::Framework => "framework",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FaultOrigin {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_origin(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "edge" => Ok(FaultOrigin::Edge),
                "renderer" => Ok(FaultOrigin::Renderer),
                "os" => Ok(FaultOrigin::Os),
                "module" => Ok(FaultOrigin::Module),
                "plugin" => Ok(FaultOrigin::Plugin),
                "app" => Ok(FaultOrigin::App),
                "extension" => Ok(FaultOrigin::Extension),
                "framework" => Ok(FaultOrigin::Framework),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FaultOrigin variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultScope {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_scope(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        for (key, slot) in [
            ("pluginId", &self.plugin_id),
            ("appId", &self.app_id),
            ("instanceId", &self.instance_id),
            ("module", &self.module),
            ("bodyKey", &self.body_key),
        ] {
            if let Some(text) = slot {
                entries.push((key.to_string(), DslValue::String(text.clone())));
            }
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultScope {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_scope(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultScope, found {other:?}"))),
        };
        let take = |key: &str| -> Result<Option<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(None),
                Some(DslValue::String(text)) => Ok(Some(text.clone())),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultScope.{key}, found {other:?}"))),
            }
        };
        Ok(FaultScope {
            plugin_id: take("pluginId")?,
            app_id: take("appId")?,
            instance_id: take("instanceId")?,
            module: take("module")?,
            body_key: take("bodyKey")?,
        })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultCause {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_cause(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
        if let Some(code) = &self.code {
            entries.push(("code".to_string(), code.to_value()));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultCause {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_cause(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultCause, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
        };
        let code = match find("code") {
            None | Some(DslValue::Null) => None,
            Some(slot) => Some(FaultCode::from_value(slot)?),
        };
        Ok(FaultCause { message, code })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Fault {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_fault(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("origin".to_string(), self.origin.to_value()),
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
            ("scope".to_string(), self.scope.to_value()),
        ];
        if let Some(span) = &self.span {
            entries.push(("span".to_string(), span.to_value()));
        }
        if !self.causes.is_empty() {
            entries.push(("causes".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));
        }
        entries.push(("retryable".to_string(), DslValue::Bool(self.retryable)));
        DslValue::Object(entries)
    }
}
impl FromValue for Fault {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_fault(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Fault, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Fault.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
        };
        Ok(Fault {
            origin: FaultOrigin::from_value(required("origin")?)?,
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            message,
            scope: Box::new(match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            }),
            span: match find("span") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(TextSpan::from_value(slot)?),
            },
            causes: match find("causes") {
                None | Some(DslValue::Null) => Vec::new(),
                Some(DslValue::Array(items)) => items.into_iter().map(FaultCause::from_value).collect::<Result<_, _>>()?,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for Fault.causes, found {other:?}"))),
            },
            retryable: match find("retryable") {
                None | Some(DslValue::Null) => false,
                Some(DslValue::Bool(flag)) => flag,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a bool for Fault.retryable, found {other:?}"))),
            },
        })
    }
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self { origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), retryable: false }
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        *self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// 🗣️ Canonical one-line rendering of a fault for `String` error channels.
    ///
    /// `Fault` deliberately has no `Display`: a structured abort must not be silently interpolated
    /// into prose, and `to_string()` would hide the {@link FaultCode} every triage tool keys on.
    /// Every boundary that has to collapse a fault into text calls this instead, so the
    /// `code: message` shape is written once rather than re-derived per crate.
    pub fn describe(&self) -> String {
        format!("{}: {}", self.code.0, self.message)
    }
}

/// 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }

    fn into_fault(self) -> Fault
    where
        Self: Sized,
    {
        Fault { origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), retryable: self.fault_retryable() }
    }
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

/// 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

/// 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    serde_json::from_slice::<DslValue>(bytes).ok().and_then(|value| Fault::from_value(value).ok()).unwrap_or_else(|| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// 🔁️ Maps an error type into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_error {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(ValueRefusalKind::OwnershipLimit, format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(ValueRefusalKind::DepthLimit, format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️FaultDescribeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fault-describe/🦀️.rs"]
mod fault_describe_tests;
#[cfg(test)]
#[path = "🧪️tests/🎛️controlled/🦀️.rs"]
mod controlled_tests;
//#endregion 🔖️FaultDescribeTests

#[cfg(test)]
#[path = "🚧️text-error/🧪️tests/🦀️.rs"]
mod text_error_refusal_tests;

```

## 🧰️framework/🔨️modules/⚠️diagnostic/🎛️controlled/🦀️.rs

Before SHA-256 b238aecd9a8a43c4c3d306c54ab2e983bcbe17e243e24a28c292af079a0d262d; after ecb722e81a8c9ec06c861ce483adbf16e542cd46b2f9b0a84bbd13f9512c9161

Full immediate original / inverse:

```
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
pub(super) fn encode_text_error(value: &TextError, c: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    record(c, 2 + usize::from(value.expected.is_some()), |fields, c| { push(fields, "message", &value.message, c)?; push(fields, "span", &value.span, c)?; if let Some(value) = &value.expected { push(fields, "expected", value, c)?; } Ok(()) })
}
pub(super) fn decode_text_error(value: &DslValue, c: &mut NativeDecodeControl<'_>) -> Result<TextError, ValueError> {
    let [message, span, expected] = fields(value, ["message", "span", "expected"], c)?;
    let message = text(required(message, "missing TextError.message")?, "expected a string for TextError.message", c)?;
    let span = TextSpan::from_value_controlled(required(span, "missing TextError.span")?, c)?;
    let expected = optional_text(expected, "expected a string for TextError.expected", c)?;
    Ok(TextError { message, span, expected })
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

```

Full authored production source:

```
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

```

## 🧰️framework/🔨️modules/⚠️diagnostic/🧬️retirement/🦀️.rs

Before SHA-256 6e0a2ef6db24305e5a2bd8a83c616e46cf57fbade500df9e0660c1cf74dd9522; after bf3965619f671fe3bd8a772f20994e22fdd78a70b29fe90394e2183a9dbd7b94

Full immediate original / inverse:

```
semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
semio_framework_value::artifact_retire_struct!(crate::TextError { message, span, expected });
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
semio_framework_value::artifact_retire_struct!(crate::FaultScope { plugin_id, app_id, instance_id, module, body_key });
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
semio_framework_value::artifact_retire_struct!(crate::Fault { origin, code, severity, message, scope, span, causes, retryable });

```

Full authored production source:

```
semio_framework_value::artifact_retire_leaf!(crate::Severity);
impl semio_framework_value::retirement::RetireOwned for crate::FaultCode {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
}

semio_framework_value::artifact_retire_leaf!(crate::FaultOrigin, crate::TextSpan);
impl semio_framework_value::retirement::RetireOwned for crate::TextError {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { kind: _, message, span, expected } = self;
        semio_framework_value::artifact_retirement_sequence![message, span, expected]
    }
}
semio_framework_value::artifact_retire_struct!(crate::ExpectedSet { tokens, keywords, keys });
semio_framework_value::artifact_retire_struct!(crate::FaultScope { plugin_id, app_id, instance_id, module, body_key });
semio_framework_value::artifact_retire_struct!(crate::FaultCause { message, code });
semio_framework_value::artifact_retire_struct!(crate::Diagnostic { code, severity, span, message, expected, scope });
semio_framework_value::artifact_retire_struct!(crate::Fault { origin, code, severity, message, scope, span, causes, retryable });

```

## 🧰️framework/🔨️modules/⚠️diagnostic/🧫️fixtures/🎛️controlled/🔣️.json

Before SHA-256 8ea059d5ad623923f5d427dea74f40a7869983e9f7c1684ce253881667a8d628; after 64e6cd0adb6153accecb0b822902940996c63ad1cb98a52036a96969d7099828

Full immediate original / inverse:

```
{
  "cases": [
    {
      "id": "TextSpan full",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 3
      },
      "accepted": true,
      "expected": {
        "line": 1,
        "column": 2,
        "length": 3
      }
    },
    {
      "id": "FaultCode full",
      "owner": "FaultCode",
      "input": "module.test",
      "accepted": true,
      "expected": "module.test"
    },
    {
      "id": "TextError full",
      "owner": "TextError",
      "input": {
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      },
      "accepted": true,
      "expected": {
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      }
    },
    {
      "id": "ExpectedSet full",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      },
      "accepted": true,
      "expected": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      }
    },
    {
      "id": "FaultScope full",
      "owner": "FaultScope",
      "input": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      },
      "accepted": true,
      "expected": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      }
    },
    {
      "id": "FaultCause full",
      "owner": "FaultCause",
      "input": {
        "message": "cause",
        "code": "module.cause"
      },
      "accepted": true,
      "expected": {
        "message": "cause",
        "code": "module.cause"
      }
    },
    {
      "id": "Diagnostic full",
      "owner": "Diagnostic",
      "input": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      },
      "accepted": true,
      "expected": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      }
    },
    {
      "id": "Fault full",
      "owner": "Fault",
      "input": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      },
      "accepted": true,
      "expected": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      }
    },
    {
      "id": "Severity info",
      "owner": "Severity",
      "input": "info",
      "accepted": true,
      "expected": "info"
    },
    {
      "id": "Severity warning",
      "owner": "Severity",
      "input": "warning",
      "accepted": true,
      "expected": "warning"
    },
    {
      "id": "Severity error",
      "owner": "Severity",
      "input": "error",
      "accepted": true,
      "expected": "error"
    },
    {
      "id": "Severity fatal",
      "owner": "Severity",
      "input": "fatal",
      "accepted": true,
      "expected": "fatal"
    },
    {
      "id": "FaultOrigin edge",
      "owner": "FaultOrigin",
      "input": "edge",
      "accepted": true,
      "expected": "edge"
    },
    {
      "id": "FaultOrigin renderer",
      "owner": "FaultOrigin",
      "input": "renderer",
      "accepted": true,
      "expected": "renderer"
    },
    {
      "id": "FaultOrigin os",
      "owner": "FaultOrigin",
      "input": "os",
      "accepted": true,
      "expected": "os"
    },
    {
      "id": "FaultOrigin module",
      "owner": "FaultOrigin",
      "input": "module",
      "accepted": true,
      "expected": "module"
    },
    {
      "id": "FaultOrigin plugin",
      "owner": "FaultOrigin",
      "input": "plugin",
      "accepted": true,
      "expected": "plugin"
    },
    {
      "id": "FaultOrigin app",
      "owner": "FaultOrigin",
      "input": "app",
      "accepted": true,
      "expected": "app"
    },
    {
      "id": "FaultOrigin extension",
      "owner": "FaultOrigin",
      "input": "extension",
      "accepted": true,
      "expected": "extension"
    },
    {
      "id": "FaultOrigin framework",
      "owner": "FaultOrigin",
      "input": "framework",
      "accepted": true,
      "expected": "framework"
    },
    {
      "id": "Scope null optional",
      "owner": "FaultScope",
      "input": {
        "pluginId": null
      },
      "accepted": true,
      "expected": {}
    },
    {
      "id": "Expected defaults",
      "owner": "ExpectedSet",
      "input": {
        "tokens": null
      },
      "accepted": true,
      "expected": {
        "tokens": [],
        "keywords": [],
        "keys": []
      }
    },
    {
      "id": "TextError null expected",
      "owner": "TextError",
      "input": {
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": null
      },
      "accepted": true,
      "expected": {
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      }
    },
    {
      "id": "Cause null code",
      "owner": "FaultCause",
      "input": {
        "message": "",
        "code": null
      },
      "accepted": true,
      "expected": {
        "message": ""
      }
    },
    {
      "id": "Diagnostic defaults",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "expected": null,
        "scope": null
      },
      "accepted": true,
      "expected": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "scope": {}
      }
    },
    {
      "id": "Fault defaults",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": null,
        "span": null,
        "causes": null,
        "retryable": null
      },
      "accepted": true,
      "expected": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": {},
        "retryable": false
      }
    },
    {
      "id": "Severity unknown",
      "owner": "Severity",
      "input": "warn",
      "accepted": false
    },
    {
      "id": "Origin unknown",
      "owner": "FaultOrigin",
      "input": "host",
      "accepted": false
    },
    {
      "id": "Code wrong kind",
      "owner": "FaultCode",
      "input": 3,
      "accepted": false
    },
    {
      "id": "Span negative",
      "owner": "TextSpan",
      "input": {
        "line": -1,
        "column": 2,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span fraction",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 1.5,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span overflow",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 4294967296
      },
      "accepted": false
    },
    {
      "id": "Span missing",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2
      },
      "accepted": false
    },
    {
      "id": "Scope unknown",
      "owner": "FaultScope",
      "input": {
        "other": "x"
      },
      "accepted": false
    },
    {
      "id": "Expected wrong item",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          true
        ]
      },
      "accepted": false
    },
    {
      "id": "Cause missing message",
      "owner": "FaultCause",
      "input": {},
      "accepted": false
    },
    {
      "id": "Fault missing code",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "severity": "error",
        "message": "x"
      },
      "accepted": false
    },
    {
      "id": "Diagnostic extra",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "extra": 1
      },
      "accepted": false
    },
    {
      "id": "TextError wrong expected",
      "owner": "TextError",
      "input": {
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": []
      },
      "accepted": false
    }
  ],
  "stress": {
    "listCount": 1024,
    "textRepetitions": 70000,
    "text": "雪",
    "maximumBytes": 8388608,
    "cancelAt": 65536
  },
  "duplicateCases": [
    {
      "id": "duplicate cause message",
      "owner": "FaultCause",
      "entries": [
        [
          "message",
          "first"
        ],
        [
          "message",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate scope plugin",
      "owner": "FaultScope",
      "entries": [
        [
          "pluginId",
          "first"
        ],
        [
          "pluginId",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate span line",
      "owner": "TextSpan",
      "entries": [
        [
          "line",
          1
        ],
        [
          "line",
          2
        ],
        [
          "length",
          3
        ]
      ]
    }
  ],
  "refusalKinds": {
    "malformed": "invalidValue",
    "ownership": "ownershipLimit",
    "cancel": "canceled"
  }
}

```

Full authored production source:

```
{
  "cases": [
    {
      "id": "TextSpan full",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 3
      },
      "accepted": true,
      "expected": {
        "line": 1,
        "column": 2,
        "length": 3
      }
    },
    {
      "id": "FaultCode full",
      "owner": "FaultCode",
      "input": "module.test",
      "accepted": true,
      "expected": "module.test"
    },
    {
      "id": "TextError full",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      },
      "accepted": true,
      "expected": {
        "kind": "invalidValue",
        "message": "bad 雪",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": "name"
      }
    },
    {
      "id": "ExpectedSet full",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      },
      "accepted": true,
      "expected": {
        "tokens": [
          "=",
          "雪"
        ],
        "keywords": [
          "let"
        ],
        "keys": [
          "name"
        ]
      }
    },
    {
      "id": "FaultScope full",
      "owner": "FaultScope",
      "input": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      },
      "accepted": true,
      "expected": {
        "pluginId": "plugin",
        "appId": "app",
        "instanceId": "instance",
        "module": "module",
        "bodyKey": "body"
      }
    },
    {
      "id": "FaultCause full",
      "owner": "FaultCause",
      "input": {
        "message": "cause",
        "code": "module.cause"
      },
      "accepted": true,
      "expected": {
        "message": "cause",
        "code": "module.cause"
      }
    },
    {
      "id": "Diagnostic full",
      "owner": "Diagnostic",
      "input": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      },
      "accepted": true,
      "expected": {
        "code": "module.test",
        "severity": "warning",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "message",
        "expected": {
          "tokens": [
            "=",
            "雪"
          ],
          "keywords": [
            "let"
          ],
          "keys": [
            "name"
          ]
        },
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        }
      }
    },
    {
      "id": "Fault full",
      "owner": "Fault",
      "input": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      },
      "accepted": true,
      "expected": {
        "origin": "framework",
        "code": "framework.test",
        "severity": "fatal",
        "message": "message",
        "scope": {
          "pluginId": "plugin",
          "appId": "app",
          "instanceId": "instance",
          "module": "module",
          "bodyKey": "body"
        },
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "causes": [
          {
            "message": "cause",
            "code": "module.cause"
          },
          {
            "message": "second"
          }
        ],
        "retryable": true
      }
    },
    {
      "id": "Severity info",
      "owner": "Severity",
      "input": "info",
      "accepted": true,
      "expected": "info"
    },
    {
      "id": "Severity warning",
      "owner": "Severity",
      "input": "warning",
      "accepted": true,
      "expected": "warning"
    },
    {
      "id": "Severity error",
      "owner": "Severity",
      "input": "error",
      "accepted": true,
      "expected": "error"
    },
    {
      "id": "Severity fatal",
      "owner": "Severity",
      "input": "fatal",
      "accepted": true,
      "expected": "fatal"
    },
    {
      "id": "FaultOrigin edge",
      "owner": "FaultOrigin",
      "input": "edge",
      "accepted": true,
      "expected": "edge"
    },
    {
      "id": "FaultOrigin renderer",
      "owner": "FaultOrigin",
      "input": "renderer",
      "accepted": true,
      "expected": "renderer"
    },
    {
      "id": "FaultOrigin os",
      "owner": "FaultOrigin",
      "input": "os",
      "accepted": true,
      "expected": "os"
    },
    {
      "id": "FaultOrigin module",
      "owner": "FaultOrigin",
      "input": "module",
      "accepted": true,
      "expected": "module"
    },
    {
      "id": "FaultOrigin plugin",
      "owner": "FaultOrigin",
      "input": "plugin",
      "accepted": true,
      "expected": "plugin"
    },
    {
      "id": "FaultOrigin app",
      "owner": "FaultOrigin",
      "input": "app",
      "accepted": true,
      "expected": "app"
    },
    {
      "id": "FaultOrigin extension",
      "owner": "FaultOrigin",
      "input": "extension",
      "accepted": true,
      "expected": "extension"
    },
    {
      "id": "FaultOrigin framework",
      "owner": "FaultOrigin",
      "input": "framework",
      "accepted": true,
      "expected": "framework"
    },
    {
      "id": "Scope null optional",
      "owner": "FaultScope",
      "input": {
        "pluginId": null
      },
      "accepted": true,
      "expected": {}
    },
    {
      "id": "Expected defaults",
      "owner": "ExpectedSet",
      "input": {
        "tokens": null
      },
      "accepted": true,
      "expected": {
        "tokens": [],
        "keywords": [],
        "keys": []
      }
    },
    {
      "id": "TextError null expected",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": null
      },
      "accepted": true,
      "expected": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        }
      }
    },
    {
      "id": "Cause null code",
      "owner": "FaultCause",
      "input": {
        "message": "",
        "code": null
      },
      "accepted": true,
      "expected": {
        "message": ""
      }
    },
    {
      "id": "Diagnostic defaults",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "expected": null,
        "scope": null
      },
      "accepted": true,
      "expected": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "scope": {}
      }
    },
    {
      "id": "Fault defaults",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": null,
        "span": null,
        "causes": null,
        "retryable": null
      },
      "accepted": true,
      "expected": {
        "origin": "os",
        "code": "os.test",
        "severity": "error",
        "message": "",
        "scope": {},
        "retryable": false
      }
    },
    {
      "id": "Severity unknown",
      "owner": "Severity",
      "input": "warn",
      "accepted": false
    },
    {
      "id": "Origin unknown",
      "owner": "FaultOrigin",
      "input": "host",
      "accepted": false
    },
    {
      "id": "Code wrong kind",
      "owner": "FaultCode",
      "input": 3,
      "accepted": false
    },
    {
      "id": "Span negative",
      "owner": "TextSpan",
      "input": {
        "line": -1,
        "column": 2,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span fraction",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 1.5,
        "length": 3
      },
      "accepted": false
    },
    {
      "id": "Span overflow",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2,
        "length": 4294967296
      },
      "accepted": false
    },
    {
      "id": "Span missing",
      "owner": "TextSpan",
      "input": {
        "line": 1,
        "column": 2
      },
      "accepted": false
    },
    {
      "id": "Scope unknown",
      "owner": "FaultScope",
      "input": {
        "other": "x"
      },
      "accepted": false
    },
    {
      "id": "Expected wrong item",
      "owner": "ExpectedSet",
      "input": {
        "tokens": [
          true
        ]
      },
      "accepted": false
    },
    {
      "id": "Cause missing message",
      "owner": "FaultCause",
      "input": {},
      "accepted": false
    },
    {
      "id": "Fault missing code",
      "owner": "Fault",
      "input": {
        "origin": "os",
        "severity": "error",
        "message": "x"
      },
      "accepted": false
    },
    {
      "id": "Diagnostic extra",
      "owner": "Diagnostic",
      "input": {
        "code": "",
        "severity": "info",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "message": "",
        "extra": 1
      },
      "accepted": false
    },
    {
      "id": "TextError wrong expected",
      "owner": "TextError",
      "input": {
        "kind": "invalidValue",
        "message": "",
        "span": {
          "line": 1,
          "column": 2,
          "length": 3
        },
        "expected": []
      },
      "accepted": false
    }
  ],
  "stress": {
    "listCount": 1024,
    "textRepetitions": 70000,
    "text": "雪",
    "maximumBytes": 8388608,
    "cancelAt": 65536
  },
  "duplicateCases": [
    {
      "id": "duplicate cause message",
      "owner": "FaultCause",
      "entries": [
        [
          "message",
          "first"
        ],
        [
          "message",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate scope plugin",
      "owner": "FaultScope",
      "entries": [
        [
          "pluginId",
          "first"
        ],
        [
          "pluginId",
          "second"
        ]
      ]
    },
    {
      "id": "duplicate span line",
      "owner": "TextSpan",
      "entries": [
        [
          "line",
          1
        ],
        [
          "line",
          2
        ],
        [
          "length",
          3
        ]
      ]
    }
  ],
  "refusalKinds": {
    "malformed": "invalidValue",
    "ownership": "ownershipLimit",
    "cancel": "canceled"
  }
}

```

Actual mounted original39 independent Ajv corpus replay plus strict/noUnchecked TypeScript0diagnostics passed on the registered test-controlled-oracle route, DEBUG39rows, Nx7.7s. This validates authored semantic fixture bindings; it is not native production proof. Owner-only SourceReady1 has216 exact full source/hash rows and refreshed actual workspace/managed lock; High constructor/literal/field/Pack closure remains incomplete, so Native stays held for combined coherent handoff.
