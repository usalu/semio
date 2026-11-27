//! 🪄️ generation2d → pdf — the page drawing (`generation2d_drawing`) written as a one-page vector PDF 1.4 by `s.stdio.semio/v1/drawing`'s own
//! pdf leaf, the one pdf writer every owner shares.
//!
//! 🔖 `IoFidelity::Lossy`: a page drawing, not the document — there is no pdf import.
//!
//! 🧩️ The pdf route is not part of the generation2d library: only a composition root that offers pdf depends on this
//! crate and declares the artifact through [`declaration`].
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_plugin::io::{ComposeError, ComposedArtifact, ComposerEntry, ErasedComposeSource, IoPayload};
use semio_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinitionError, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
use semio_s_artifact_procedural_2d_io::generation2d_drawing;
use semio_s_artifact_procedural_2d_io::io_registry::{rebuild_native_snapshot, GENERATION2D_DIALECT};
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };

/// 📄️ The page drawing as PDF 1.4 bytes.
pub fn serialize_bytes(snapshot: &Generation2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let refused = |error: String| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("generation2d→pdf: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1));
    semio_s_artifact_stdio_semio_drawing_pdf_export::encode_drawing_pdf(&generation2d_drawing(snapshot).map_err(refused)?, "1.4").map_err(refused)
}

fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources).await?;
        let bytes = serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_plugin::io::Confidence::Medium })
    })
}

/// 🧾️ The generation2d → pdf composer row, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[GENERATION2D_DIALECT], compose: compose_export_pdf }]).as_slice()
}

/// 🪪️ The composition-route capability the pdf row claims.
pub fn capability() -> Result<ArtifactCapability, ArtifactDefinitionError> {
    ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation2d.composer.pdf")?, ArtifactCapabilityKind::composer())
        .descriptor(b"s.stdio.pdf@1.4/*")?
        .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::composition_route(), "29:s.procedural.generation2d@1/*17:s.stdio.pdf@1.4/*")?)
}

/// 🔖️ The generation2d declaration with its pdf route, for the composition root that offers pdf.
pub fn declaration() -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
    semio_s_artifact_procedural_generation2d::declaration_from(semio_s_artifact_procedural_generation2d::definition()?.capability(capability()?)?, &[entries()])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
