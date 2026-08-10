//! deser en1996 via txt
//! 🐛️ Pre-migration content here referenced `crate::artifacts::json`/`crate::artifacts::txt`,
//! types that don't exist in this crate (dead code, never mounted by the old glue, never
//! compiled) -- likely a copy-paste of stdio's own internal json<-txt bridge into the wrong
//! plugin's txt target folder. Left as an honest stub producing this artifact's own real
//! snapshot type, pending a real txt import/export implementation.
use crate::artifacts::en1996::En1996Snapshot;
pub fn register() {}
pub fn deserialize(_from: &crate::artifacts::txt::TxtSnapshot) -> Result<En1996Snapshot, String> {
    Err("txt import not yet implemented".into())
}
pub fn deserialize_bytes(_bytes: &[u8]) -> Result<En1996Snapshot, String> {
    Err("txt import not yet implemented".into())
}
