//! Serialize layout to stdio.pdf.
//!
//! 🔀️ Imports from stdio's version-pinned `standards::v1_4` module explicitly (not the crate-root
//! `artifacts::pdf::schema::snapshot` re-export) — that top-level alias now canonically points at
//! `standards::v1_7`'s differently-shaped `PdfSnapshot` (no `page`/`PageDoc`), per stdio's own
//! `🗿️artifacts/📄️pdf/🦀️component.rs` doc comment on the two independent 1.4/1.7 declarations. This
//! serializer targets the 1.4 dialect (its own directory name), so it always meant the 1.4 shape —
//! confirmed pre-existing/unrelated to this ticket's composition work (stdio's re-export repoint
//! landed in its own commit, this file untouched since 2026-08-10).
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_s_plugin_stdio::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

pub async fn register() {}

pub async fn serialize(from: &LayoutSnapshot) -> Result<PdfSnapshot, store::PackError> {
    Ok(PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        page: PageDoc { width: 612.0, height: 792.0, text: <LayoutSnapshot as store::ArtifactDsl>::print_dsl(from) },
    })
}
