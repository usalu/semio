//! 📤️ Export En1992Snapshot as .csv.

use semio_framework_plugin::{DocumentCodec, IoError, JsonCodec, MediaFormat};

//#region 🔖️Export
pub fn export(snapshot: &crate::artifacts::en1992::En1992Snapshot) -> Result<Vec<u8>, IoError> {
    let value = serde_json::to_value(snapshot).map_err(|e| IoError::Payload(e.to_string()))?;
    JsonCodec.export(&value)
}

pub fn register() {
    let kind = "data.📘️en1992";
    let format = MediaFormat::Csv;
    semio_framework_os::register_os_media_export_handler(kind, format, move |doc| {
        let snapshot: crate::artifacts::en1992::En1992Snapshot =
            serde_json::from_value(doc.clone()).map_err(|e| e.to_string())?;
        let bytes = export(&snapshot).map_err(|e| e.to_string())?;
        let stem = kind.replace('.', "_");
        semio_framework_os::OsMediaExportResult::from_format_bytes(bytes, format, &stem)
    });
}
//#endregion 🔖️Export
