//! Serialize writer to stdio.txt.
use crate::artifacts::writer::WriterSnapshot;
use semio_s_plugin_stdio::artifacts::txt::TxtSnapshot;

pub fn register() {}

pub fn serialize(from: &WriterSnapshot) -> Result<TxtSnapshot, store::PackError> {
    Ok(TxtSnapshot::from_body(&<WriterSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub fn serialize_text(from: &WriterSnapshot) -> Result<String, store::PackError> {
    Ok(<WriterSnapshot as store::ArtifactDsl>::print_dsl(from))
}
