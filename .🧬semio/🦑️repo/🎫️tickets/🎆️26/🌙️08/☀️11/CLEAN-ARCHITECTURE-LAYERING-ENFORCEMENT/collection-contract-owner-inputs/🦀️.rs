//! 🧺️ Domain-neutral identity and caller-defined patch contracts.
/// 🏷️ Identifies an item within a collection by its stable id.
pub trait Identified<TId> {fn id(&self)->&TId;}
/// 🩹️ Applies a caller-defined patch and compares another value.
pub trait Patchable<TPatch>:Sized {fn apply_patch(&mut self,patch:&TPatch);fn diff_patch(&self,other:&Self)->Option<TPatch>;}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
