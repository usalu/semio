//! 📥️ Import .gltf into Process3dSnapshot.

use semio_framework_plugin::{DocumentCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Import
pub fn import(bytes: &[u8]) -> Result<crate::artifacts::process3d::Process3dSnapshot, IoError> {
    match JsonCodec.import(bytes) {
        Ok(value) => serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string())),
        Err(primary) => {
            // Textual formats may arrive as raw UTF-8; attempt JSON parse of the string body first.
            if let Ok(text) = std::str::from_utf8(bytes) {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(text) {
                    return serde_json::from_value(value).map_err(|e| IoError::Payload(e.to_string()));
                }
            }
            let _ = MediaFormat::Gltf;
            Err(primary)
        }
    }
}

pub fn register() {
    let kind = "saw";
    let format = MediaFormat::Gltf;
    semio_framework_os::register_os_media_import_handler(kind, format, move |bytes| {
        let snapshot = import(bytes).map_err(|e| e.to_string())?;
        serde_json::to_value(snapshot).map_err(|e| e.to_string())
    });
}
//#endregion 🔖️Import
