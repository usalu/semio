//! 📤️ Export Din18599Snapshot as .xlsx.

use semio_framework_plugin::{DocumentCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::din18599::Din18599Snapshot) -> Result<Vec<u8>, IoError> {
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    JsonCodec.export(&value)
}

pub fn register() {
    let kind = "data.📙️din18599";
    let format = MediaFormat::Xlsx;
    semio_framework_os::register_os_media_export_handler(kind, format, move |doc| {
        let snapshot: crate::artifacts::din18599::Din18599Snapshot =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        let stem = kind.replace('.', "_");
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, &stem)
    });
}
//#endregion 🔖️Export
