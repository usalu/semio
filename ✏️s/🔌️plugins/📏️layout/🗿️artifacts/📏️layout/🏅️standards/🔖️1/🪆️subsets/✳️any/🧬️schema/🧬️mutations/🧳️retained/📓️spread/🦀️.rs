//! 📓️ Original-source retained spread membership owners.
#[path="🔺️output.rs"]pub mod output;
use super::target::LayoutTargetSource;
pub(crate) struct SpreadRows<'a>(pub &'a[semio_s_artifact_layout_layout_model::Spread]);
impl LayoutTargetSource for SpreadRows<'_>{fn identity(&self)->usize{751}fn count(&self)->usize{self.0.len()}fn key(&self,i:usize)->&str{&self.0[i].id}}
pub(crate) struct SpreadIds<'a>(pub &'a[String]);
impl LayoutTargetSource for SpreadIds<'_>{fn identity(&self)->usize{752}fn count(&self)->usize{self.0.len()}fn key(&self,i:usize)->&str{&self.0[i]}}
