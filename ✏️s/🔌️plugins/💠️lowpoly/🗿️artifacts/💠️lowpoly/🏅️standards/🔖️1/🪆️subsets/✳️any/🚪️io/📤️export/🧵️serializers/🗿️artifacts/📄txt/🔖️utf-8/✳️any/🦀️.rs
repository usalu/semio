//! lowpoly -> txt
//!
//! 📜️ `s.stdio.txt@utf-8/*` is CARRIER_TEXT (its native `Text` payload IS the raw external file
//! text, verbatim -- see `TxtSnapshot`'s own `store::ArtifactDsl` impl doc, "CARRIER LAW"). The
//! honest lowpoly.txt representation is therefore lowpoly's OWN canonical `.lowpoly` DSL text
//! (`store::ArtifactDsl for LowpolySnapshot`, `../../../../../../🧬️schema/📸️snapshot/📝️text/🦀️.rs`)
//! carried verbatim as the txt body -- never a second bespoke grammar.
use crate::artifacts::lowpoly::schema::snapshot::text::print_dsl;
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<TxtSnapshot, store::TextError> {
    Ok(TxtSnapshot::from_body(&print_dsl(snapshot)))
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.to_body().into_bytes())
}
