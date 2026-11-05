//! 🔎️ Both evaluation and resolution use the same first-party original origin lookup.
use super::*;
pub(crate) use flow::{FlowOriginLookupWork,FlowOriginLookupOutcome};

#[cfg(test)]
#[path="🧪️origin.rs"]
mod tests;
