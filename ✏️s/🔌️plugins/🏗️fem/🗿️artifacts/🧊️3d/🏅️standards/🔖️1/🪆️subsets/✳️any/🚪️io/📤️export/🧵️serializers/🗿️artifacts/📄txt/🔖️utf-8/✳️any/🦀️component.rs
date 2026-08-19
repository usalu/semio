//! ser fem3d to txt
//! 🐛️ Pre-migration content here referenced `crate::artifacts::json`/`crate::artifacts::txt`,
//! types that don't exist in this crate (dead code, never mounted by the old glue, never
//! compiled) -- likely a copy-paste of stdio's own internal json<-txt bridge into the wrong
//! plugin's txt target folder. Left as an honest stub producing this artifact's own real
//! snapshot type, pending a real txt import/export implementation.
use crate::artifacts::fem3d::Fem3dSnapshot;
pub async fn register() {}
pub async fn serialize(_from: &Fem3dSnapshot) -> Result<semio_s_plugin_stdio::artifacts::txt::TxtSnapshot, String> {
    Err("txt export not yet implemented".into())
}
pub async fn deserialize_bytes(_bytes: &[u8]) -> Result<Fem3dSnapshot, String> {
    Err("txt import not yet implemented".into())
}
