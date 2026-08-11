//! Scratch harness for the zip diff/absorb/mutation algebra — mirrors the real
//! crate's trait shapes locally so the algorithm can be verified fast while the real
//! workspace crate is blocked by unrelated sibling F1-wave compile errors (txt/xml/csv/deflate).
//! This file's `snapshot`/`diff`/`mutations` modules are byte-for-byte the same LOGIC as the real
//! `🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/{🔺️diff,🧬️mutations}/🦀️component.rs` —
//! only the trait imports/module paths differ (local stub traits instead of `protocol::*`).

use serde::{Deserialize, Serialize};

//#region StubTraits (mirrors protocol::{MutationDiff, command::DiffAlgebra, Mutation})
pub trait MutationDiff<P>: Clone + Default {
    fn apply(&self, base: &P) -> P;
    fn absorb(&mut self, other: Self);
}
pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;
    fn between(base: &P, other: &P) -> Self;
    fn is_empty(&self) -> bool;
}
pub trait Mutation<P>: Clone {
    type Diff: MutationDiff<P>;
    fn diff(&self, base: &P) -> Self::Diff;
    fn inverse(&self, base: &P) -> Vec<Self> where Self: Sized;
}
//#endregion

//#region snapshot (mirrors 📸️snapshot/🦀️component.rs's types exactly)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ZipCompressionMethod { Stored, Deflate }
impl Default for ZipCompressionMethod { fn default() -> Self { Self::Stored } }

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipExtraField { pub id: u16, #[serde(default)] pub payload: Vec<u8> }

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntry {
    pub name: String,
    #[serde(default)] pub data: Vec<u8>,
    #[serde(default)] pub method: ZipCompressionMethod,
    #[serde(default)] pub dos_date: u16,
    #[serde(default)] pub dos_time: u16,
    #[serde(default)] pub unix_mtime: Option<i64>,
    #[serde(default)] pub flags: u16,
    #[serde(default)] pub version_made_by: u16,
    #[serde(default)] pub version_needed: u16,
    #[serde(default)] pub internal_attrs: u16,
    #[serde(default)] pub external_attrs: u32,
    #[serde(default)] pub local_extra: Vec<ZipExtraField>,
    #[serde(default)] pub central_extra: Vec<ZipExtraField>,
    #[serde(default)] pub comment: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipSnapshot {
    pub schema: String,
    #[serde(default)] pub entries: Vec<ZipEntry>,
    #[serde(default)] pub comment: String,
}
impl Default for ZipSnapshot {
    fn default() -> Self { Self { schema: "stdio.zip".into(), entries: Vec::new(), comment: String::new() } }
}
//#endregion

pub mod diff {
    include!("diff_body.rs");
}

pub mod mutations {
    include!("mutations_body.rs");
}
