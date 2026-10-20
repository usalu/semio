//! 🔌️ protocol every sparse solver job shares (outcome desk, close ladder, checkpoint page writer and restore cursor).
//! 🧮️ Sparse linear algebra core: COO/CSR/CSC storage, reverse-Cuthill-McKee ordering, and the numerical job

use semio_framework_value_persistent::list::PagedList;
use semio_framework_job::{InteractiveJob, OriginalTurnFailure, close_original_retained_step};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeKind;
use semio_framework_job::JobOutcomeView;
use semio_framework_job_payload::JobPayloadAdmissionFault;
use semio_framework_job_payload::JobPayloadStream;
use semio_framework_job::Operation;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job::RetainedCloneStep;
use semio_framework_job_payload::RetainedJobPayload;
use semio_framework_job_payload::RetainedJobPayloadWriter;
use semio_framework_job::StepContext;
use semio_framework_value_edit::sort::HeapSortCursor;
use semio_framework_value_edit::sort::HeapSortWork;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::VecDeque;
use semio_s_fem_algebra::MatD;
use semio_s_fem_algebra::VecD;

#[path = "🤝️numerical-protocol/🦀️.rs"]
mod numerical_protocol;
pub use numerical_protocol::*;

#[path = "🗃️storage-and-ordering/🦀️.rs"]
mod storage_and_ordering;
pub use storage_and_ordering::*;

#[path = "🧮️triplet-csr-build/🦀️.rs"]
mod triplet_csr_build;
pub use triplet_csr_build::*;

#[path = "📄️checkpoint-pages/🦀️.rs"]
mod checkpoint_pages;
pub use checkpoint_pages::*;

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

// #endregion 🔖️Tests
