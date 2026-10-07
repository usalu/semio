//! ⚠️ Unmounted canonical Pack producer-authority draft; no native runtime verdict.
use semio_framework_diagnostic::{FaultOrigin,TextError};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};
#[cfg(test)]
#[path = "🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;

#[cfg(test)]
#[path = "🧪️tests/📍️text-refusal/🦀️.rs"]
mod text_refusal_tests;
