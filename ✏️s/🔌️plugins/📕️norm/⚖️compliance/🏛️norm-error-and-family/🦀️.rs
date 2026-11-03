//! 🏛️ Norm errors and families of the norm compliance core: the error type, the family ids and trait, and the norm host. See [`super`].

use super::*;
use std::sync::Arc;

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

    /// 📅️ The designation with the print edition of the principal part the family implements, `None` for families that cite no single print. The editions are the commonly cited ones and stay unverified until the registry records the print edition at verification (see `NdpSource::edition`).
    pub fn edition(self) -> Option<&'static str> {
        match self {
            Self::En1990 => Some("EN 1990:2002+A1:2005"),
            Self::En1991 => Some("EN 1991-1-1:2002"),
            Self::En1992 => Some("EN 1992-1-1:2004+A1:2014"),
            Self::En1993 => Some("EN 1993-1-1:2005+A1:2014"),
            Self::En1994 => Some("EN 1994-1-1:2004"),
            Self::En1995 => Some("EN 1995-1-1:2004+A2:2014"),
            Self::En1996 => Some("EN 1996-1-1:2005+A1:2012"),
            Self::En1997 => Some("EN 1997-1:2004+A1:2013"),
            Self::En1998 => Some("EN 1998-1:2004+A1:2013"),
            Self::En1999 => Some("EN 1999-1-1:2007+A2:2013"),
            Self::Din4108 | Self::DinEn16798 | Self::DinV18599 | Self::Iso16757 | Self::Vdi3805 => None,
        }
    }
}

/// 🧩️ Headless norm family contract: typed document, undoable operations, and compliance evaluation.
pub trait NormFamily: Send + Sync + 'static {
    type Document: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static;
    type Mutation: Mutation<Self::Document> + Clone + PartialEq + Send;

    fn family_id() -> NormFamilyId;
    fn evaluate(document: &Self::Document) -> CheckReport;

    /// 🛰️ Settled report of exactly `document` for the paint path, as one shared handle so no reader copies it; families with a retained `SharedEvaluation` (`live_report!`) override it so edits re-run only the affected units, the default is the full fold.
    fn live_report(document: &Self::Document) -> Arc<CheckReport> {
        Arc::new(Self::evaluate(document))
    }

    /// 🖼️ Paint path that never freezes: the report as it stands after waiting at most `budget`, plus whether it is the full evaluation (`live_report!` modules override it with `responsive`; the default settles synchronously).
    fn live_responsive(document: &Self::Document, _budget: std::time::Duration) -> (Arc<CheckReport>, bool) {
        (Self::live_report(document), true)
    }

    /// 📊️ Progress of the latest live run, for the transient lane; idle for families without a retained session.
    fn live_progress() -> incremental_evaluation::EvalProgress {
        incremental_evaluation::EvalProgress::default()
    }

    /// 🛑️ Supersedes the live run in flight; a no-op for families without a retained session.
    fn cancel_live() {}
}

/// ⚡️ Norm family whose evaluation is a fold of independently re-runnable check units; adopting it enables retained incremental sessions.
pub trait IncrementalNormFamily: NormFamily + incremental_evaluation::UnitSource<Document = <Self as NormFamily>::Document> {
    fn evaluate_incrementally(document: &<Self as NormFamily>::Document) -> CheckReport
    where
        Self: Sized,
    {
        incremental_evaluation::evaluate_units::<Self>(document)
    }
}

/// 🧠️ Retained headless session: document inputs plus the last computed compliance report, shared by handle with the live session and the lane clock (a host never owns a copy).
#[derive(Clone, Debug, PartialEq)]
pub struct NormHost<F: NormFamily> {
    pub document: F::Document,
    pub report: Arc<CheckReport>,
}

impl<F: NormFamily> Default for NormHost<F>
where
    F::Document: Default,
{
    fn default() -> Self {
        Self { document: F::Document::default(), report: Arc::new(CheckReport::default()) }
    }
}

impl<F: NormFamily> NormHost<F> {
    /// 🎐 Host over `document` with its settled live report: every read is current after any edit, no evaluate command in between.
    pub fn live(document: F::Document) -> Self {
        let report = F::live_report(&document);
        Self { document, report }
    }

    /// 🖼️ Like [`Self::live`] but bounded by `budget`: slow units keep running on the session worker and the flag tells whether the report is already complete.
    pub fn live_within(document: F::Document, budget: std::time::Duration) -> (Self, bool) {
        let (report, settled) = F::live_responsive(&document, budget);
        (Self { document, report }, settled)
    }

    pub fn document(&self) -> &F::Document {
        &self.document
    }

    pub fn report(&self) -> &CheckReport {
        &self.report
    }

    /// 🔗️ The report as its shared handle, for readers that keep it beyond the host (the lane clock caches it without a copy).
    pub fn shared_report(&self) -> Arc<CheckReport> {
        self.report.clone()
    }

    pub fn apply(&mut self, mutation: &F::Mutation) -> protocol::MutationApplyResult<()>
    where
        F::Mutation: protocol::ColdMutation<F::Document>,
    {
        let (document, _) = vcs::apply_mutation(&self.document, mutation)?;
        self.document = document;
        self.report = F::live_report(&self.document);
        Ok(())
    }

    pub fn replace_document(&mut self, document: F::Document) {
        self.report = F::live_report(&document);
        self.document = document;
    }

    /// 🧮️ Re-evaluates the host's document through the live session.
    pub fn evaluate(&mut self) {
        self.report = F::live_report(&self.document);
    }
}
// #endregion 🔖️Family
