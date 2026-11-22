//! 📑️ Shared report facts from published VDI 3805 record corrections.

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CorrectedFields {
    pub check_id: &'static str,
    pub correction: &'static str,
    pub positions: Vec<usize>,
    pub invalid: Vec<usize>,
}
