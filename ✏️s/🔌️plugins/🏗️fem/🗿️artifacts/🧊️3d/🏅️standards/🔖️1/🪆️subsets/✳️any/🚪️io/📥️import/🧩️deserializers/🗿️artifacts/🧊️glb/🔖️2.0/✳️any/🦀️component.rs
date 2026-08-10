//! 📥️ Import .glb into Fem3dSnapshot.

use semio_framework_plugin::{ArtifactCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::fem3d::Fem3dSnapshot, IoError> {
    match JsonCodec.import(bytes) {
        Ok(value) => serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string())),
        Err(primary) => {
            // Textual formats may arrive as raw UTF-8; attempt JSON parse of the string body first.
            if let Ok(text) = std::str::from_utf8(bytes) {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
                    return serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()));
                }
            }
            let _ = MediaFormat::Glb;
            Err(primary)
        }
    }
}

pub fn register() {}
//#endregion 🔖️Import
