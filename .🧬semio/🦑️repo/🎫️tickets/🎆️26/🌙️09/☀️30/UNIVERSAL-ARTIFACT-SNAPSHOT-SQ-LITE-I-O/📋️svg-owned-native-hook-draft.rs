//! 🎨️ Unmounted actual owner hook additions, pending the selected Native laws.
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl;
impl semio_framework_os_kernel::ArtifactSqliteSnapshot for super::SvgSnapshot {
    fn decode_sqlite_snapshot_native(payload: &semio_framework_os_kernel::io_schema::IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        let (schema, doc) = semio_s_artifact_stdio_xml::schema::snapshot::decode_xml_native_snapshot(payload, "stdio.svg", control)?;
        let snapshot = pack::value::DecodedValue::new(Self { schema, doc }, |snapshot| semio_s_artifact_stdio_xml::schema::snapshot::sqlite::retire_xml_document(snapshot.doc));
        validate_root(&snapshot.doc)?;
        Ok(snapshot.take())
    }
    fn encode_sqlite_snapshot_native(&self, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<semio_framework_os_kernel::io_schema::IoPayload, String> {
        validate_root(&self.doc)?;
        semio_s_artifact_stdio_xml::schema::snapshot::encode_xml_native_snapshot(&self.schema, semio_s_artifact_stdio_xml::schema::snapshot::sqlite::XmlDocumentView::from(&self.doc), "stdio.svg", encoding, control)
    }
    fn retire_sqlite_snapshot(self) { semio_s_artifact_stdio_xml::schema::snapshot::sqlite::retire_xml_document(self.doc); }
}
