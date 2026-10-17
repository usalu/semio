//! 🏛️ Norm errors and families of the norm compliance core: the error type, the family ids and trait, the report cache and the norm host. See [`super`].

use super::*;

// #region 🔖️Error
/// ⚠️ Norm computation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NormError {
    IncompleteInput { field: String },
    OutOfScope { clause: ClauseId },
    InvalidValue { field: String, reason: String },
}

impl fmt::Display for NormError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteInput { field } => write!(formatter, "incomplete input: {field}"),
            Self::OutOfScope { clause } => write!(formatter, "out of scope: {clause}"),
            Self::InvalidValue { field, reason } => write!(formatter, "invalid {field}: {reason}"),
        }
    }
}

impl std::error::Error for NormError {}
// #endregion 🔖️Error

// #region 🔖️Family
/// 🏷️ Stable identifier for each norm family crate exposed as a ArtifactApp.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
pub enum NormFamilyId {
    Din4108,
    DinEn16798,
    DinV18599,
    En1990,
    En1991,
    En1992,
    En1993,
    En1994,
    En1995,
    En1996,
    En1997,
    En1998,
    En1999,
    Iso16757,
    Vdi3805,
}

impl NormFamilyId {
    pub fn label(self) -> &'static str {
        match self {
            Self::Din4108 => "DIN 4108",
            Self::DinEn16798 => "DIN EN 16798",
            Self::DinV18599 => "DIN V 18599",
            Self::En1990 => "EN 1990",
            Self::En1991 => "EN 1991",
            Self::En1992 => "EN 1992",
            Self::En1993 => "EN 1993",
            Self::En1994 => "EN 1994",
            Self::En1995 => "EN 1995",
            Self::En1996 => "EN 1996",
            Self::En1997 => "EN 1997",
            Self::En1998 => "EN 1998",
            Self::En1999 => "EN 1999",
            Self::Iso16757 => "ISO 16757",
            Self::Vdi3805 => "VDI 3805",
        }
    }
}

/// 🧩️ Headless norm family contract: typed document, undoable operations, and compliance evaluation.
pub trait NormFamily: Send + Sync + 'static {
    type Document: Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send;
    type Mutation: Mutation<Self::Document> + Clone + PartialEq + Send;

    fn family_id() -> NormFamilyId;
    fn evaluate(document: &Self::Document) -> CheckReport;
}

/// 🧾 Snapshot revision fingerprint used as the paint-path report cache key.
pub fn document_revision_key<D: semio_framework_value::ToValue>(document: &D) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let json = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(document));
    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    hasher.finish()
}

fn report_cache() -> &'static std::sync::Mutex<std::collections::HashMap<(NormFamilyId, u64), CheckReport>> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<(NormFamilyId, u64), CheckReport>>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// 📥 Reads a previously evaluated report for `(family, revision)` without running `evaluate`.
pub fn cached_report_for<F: NormFamily>(document: &F::Document) -> Option<CheckReport> {
    let key = (F::family_id(), document_revision_key(document));
    report_cache().lock().ok().and_then(|guard| guard.get(&key).cloned())
}

/// 📤 Stores an evaluated report keyed by `(family, revision)` for paint-path reuse.
pub fn store_cached_report_for<F: NormFamily>(document: &F::Document, report: CheckReport) {
    let key = (F::family_id(), document_revision_key(document));
    if let Ok(mut guard) = report_cache().lock() {
        if guard.len() > 64 {
            guard.clear();
        }
        guard.insert(key, report);
    }
}

/// 🗑️ Drops a cached report so the next evaluate job recomputes it.
pub fn invalidate_cached_report_for<F: NormFamily>(document: &F::Document) {
    let key = (F::family_id(), document_revision_key(document));
    if let Ok(mut guard) = report_cache().lock() {
        guard.remove(&key);
    }
}

/// 🧠️ Retained headless session: document inputs plus the last computed compliance report.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "compliance-testing"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "compliance-testing"), serde(bound(serialize = "F::Document: serde::Serialize", deserialize = "F::Document: serde::de::DeserializeOwned")))]
#[value(bound = "F::Document: semio_framework_value::ToValue, F::Document: semio_framework_value::FromValue")]
pub struct NormHost<F: NormFamily> {
    pub document: F::Document,
    pub report: CheckReport,
}

impl<F: NormFamily> Default for NormHost<F> {
    fn default() -> Self {
        Self::from_artifact(F::Document::default())
    }
}

impl<F: NormFamily> NormHost<F> {
    /// 🖼️ Paint-path constructor: never runs `F::evaluate`. Serves the revision-keyed cache or an empty report.
    pub fn from_artifact(document: F::Document) -> Self {
        let report = cached_report_for::<F>(&document).unwrap_or_default();
        Self { document, report }
    }

    pub fn document(&self) -> &F::Document {
        &self.document
    }

    pub fn report(&self) -> &CheckReport {
        &self.report
    }

    pub fn apply(&mut self, mutation: &F::Mutation) -> protocol::MutationApplyResult<()> {
        let (document, _) = vcs::apply_mutation(&self.document, mutation)?;
        invalidate_cached_report_for::<F>(&self.document);
        self.document = document;
        self.report = CheckReport::default();
        Ok(())
    }

    pub fn replace_document(&mut self, document: F::Document) {
        invalidate_cached_report_for::<F>(&self.document);
        self.document = document;
        self.report = CheckReport::default();
    }

    /// 🧮️ Runs `F::evaluate` off the paint path and stores the revision-keyed cache entry.
    pub fn evaluate(&mut self) {
        self.report = F::evaluate(&self.document);
        store_cached_report_for::<F>(&self.document, self.report.clone());
    }
}
// #endregion 🔖️Family
