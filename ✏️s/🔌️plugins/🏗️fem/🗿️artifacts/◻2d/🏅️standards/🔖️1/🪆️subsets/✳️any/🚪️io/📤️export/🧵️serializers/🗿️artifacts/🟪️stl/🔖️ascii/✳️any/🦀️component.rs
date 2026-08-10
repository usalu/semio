//! 📤️ Export Fem2dSnapshot as .stl.

use semio_framework_plugin::{ArtifactCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::fem2d::Fem2dSnapshot) -> Result<Vec<u8>, IoError> {
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    JsonCodec.export(&value)
}

pub fn register() {}
//#endregion 🔖️Export
