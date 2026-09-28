
//#region 📚️ExampleRefusal
/// 🙅️ Why one example switch seats nothing — the typed refusal every example loader answers instead of opening the
/// genesis document in silence (schema `ExampleRefusalCodeV1`, `🧬️schema/🔣️.json`). The shells show
/// [`ExampleRefusalKind::label`] in the viewer's locale, keyed by the frozen [`ExampleRefusalKind::code`]; the
/// TypeScript twin `exampleRefusalNoticeTextV1` (`🛂️manifest/🟦️.ts`) and this enum are pinned by the shared
/// `🧫️fixtures/📚️example-refusal.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExampleRefusalKind {
    Unknown,
    Undecodable,
    Empty,
}

impl ExampleRefusalKind {
    /// 📋️ Every kind, in schema order.
    pub const ALL: [Self; 3] = [Self::Unknown, Self::Undecodable, Self::Empty];

    /// 🏷️ The frozen fault code a shell keys its notice on.
    pub fn code(self) -> &'static str {
        match self {
            Self::Unknown => "example.unknown",
            Self::Undecodable => "example.undecodable",
            Self::Empty => "example.empty",
        }
    }

    /// 🔎️ The kind a fault code names, if it is an example refusal at all.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.code() == code)
    }

    /// 🗣️ What a person is told (`ExampleRefusalMessagesV1`).
    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Unknown => LocalizedLabel::native("This app has no example with that name.", "Diese App hat kein Beispiel mit diesem Namen."),
            Self::Undecodable => LocalizedLabel::native("The example could not be read: its file is outdated or damaged.", "Das Beispiel konnte nicht gelesen werden: Seine Datei ist veraltet oder beschädigt."),
            Self::Empty => LocalizedLabel::native("The example file is empty, so nothing was loaded.", "Die Beispieldatei ist leer, daher wurde nichts geladen."),
        }
    }
}

/// 🙅️ One refused example switch: its kind, the example id the switch named, and the decoder's own detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExampleRefusal {
    pub kind: ExampleRefusalKind,
    pub example_id: String,
    pub detail: String,
}

impl ExampleRefusal {
    /// 🔎️ The id names no example this surface declares.
    pub fn unknown(example_id: &str) -> Self {
        Self { kind: ExampleRefusalKind::Unknown, example_id: example_id.to_string(), detail: String::new() }
    }

    /// 🧩️ The declared example's asset does not decode into the artifact's document.
    pub fn undecodable(example_id: &str, detail: impl Into<String>) -> Self {
        Self { kind: ExampleRefusalKind::Undecodable, example_id: example_id.to_string(), detail: detail.into() }
    }

    /// 🕳️ The declared example's asset decodes into the genesis document.
    pub fn empty(example_id: &str) -> Self {
        Self { kind: ExampleRefusalKind::Empty, example_id: example_id.to_string(), detail: String::new() }
    }
}

/// 🔁️ The refusal crosses every boundary as a [`crate::Fault`]: the frozen code, a warning (the switch changed
/// nothing), never retryable (the same asset refuses again), the decoder's detail as its one cause.
impl crate::FaultFrom for ExampleRefusal {
    fn fault_origin(&self) -> crate::FaultOrigin {
        crate::FaultOrigin::App
    }

    fn fault_code(&self) -> crate::FaultCode {
        crate::FaultCode::new(self.kind.code())
    }

    fn fault_severity(&self) -> crate::Severity {
        crate::Severity::Warning
    }

    fn fault_message(&self) -> String {
        format!("{} ({})", self.kind.label().resolve(Terminology::Native, Locale::En), self.example_id)
    }

    fn fault_causes(&self) -> Vec<crate::FaultCause> {
        if self.detail.is_empty() {
            return Vec::new();
        }
        vec![crate::FaultCause { message: self.detail.clone(), code: Some(crate::FaultCode::new(self.kind.code())) }]
    }
}

impl From<ExampleRefusal> for crate::Fault {
    fn from(refusal: ExampleRefusal) -> Self {
        crate::FaultFrom::into_fault(refusal)
    }
}

/// 🎯️ THE example-seat predicate, first half: the empty id seats the app's own genesis document (`None`), a declared
/// id seats that example's authored body, and any other id is refused before anything decodes. `examples` are the
/// `(id, authored body)` pairs the surface declares; the TypeScript twin is `exampleSeatOutcomeV1`.
pub fn example_seat<'a>(example_id: &str, examples: &[(&str, &'a str)]) -> Result<Option<&'a str>, ExampleRefusal> {
    if example_id.is_empty() {
        return Ok(None);
    }
    examples.iter().find(|(id, _)| *id == example_id).map(|(_, body)| Some(*body)).ok_or_else(|| ExampleRefusal::unknown(example_id))
}

/// 🎯️ THE example-seat predicate, second half: a declared example's decode outcome — a decode error is
/// `example.undecodable`, a decoded genesis document is `example.empty`, anything else seats.
pub fn example_decoded<S: Default + PartialEq, E: std::fmt::Display>(example_id: &str, decoded: Result<S, E>) -> Result<S, ExampleRefusal> {
    match decoded {
        Err(error) => Err(ExampleRefusal::undecodable(example_id, error.to_string())),
        Ok(document) if document == S::default() => Err(ExampleRefusal::empty(example_id)),
        Ok(document) => Ok(document),
    }
}
//#endregion 📚️ExampleRefusal
