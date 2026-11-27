//! 🗺️ gismap → pdf — the map's page drawing (`gis_map_snapshot_to_drawing`: markers, routes and
//! regions, north up) written as a one-page vector PDF 1.4 by `s.stdio.semio/v1/drawing`'s own pdf leaf.
//!
//! 🔖 `IoFidelity::Lossy`: a picture of the map — coordinates are shifted onto the page and feature
//! ids and payloads beyond geometry are not carried.
//!
//! 🧩️ The pdf route is not part of the gismap library: only a composition root that offers pdf depends on this crate
//! and declares the artifact through [`declaration`], so the gismap crates compile without the pdf plugin.
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_plugin::io::{ComposeError, ComposedArtifact, ComposerEntry, ErasedComposeSource, IoPayload};
use semio_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinitionError, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
use semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::io_registry::rebuild_native_snapshot;
use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::drawing_bridge::gis_map_snapshot_to_drawing;
use semio_s_artifact_gis_gismap::{GisMapSnapshot, GISMAP_DIALECT};

const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };

/// 📄️ The map as PDF 1.4 bytes.
pub fn serialize_bytes(snapshot: &GisMapSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    semio_s_artifact_stdio_semio_drawing_pdf_export::encode_drawing_pdf(&gis_map_snapshot_to_drawing(snapshot), "1.4").map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("gismap→pdf: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_plugin::io::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_plugin::io::Confidence::Medium })
    })
}

/// 🧾️ The gismap → pdf composer row, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[GISMAP_DIALECT], compose: compose_export_pdf }]).as_slice()
}

/// 🪪️ The composition-route capability the pdf row claims.
pub fn capability() -> Result<ArtifactCapability, ArtifactDefinitionError> {
    ArtifactCapability::new(ArtifactIdentity::parse("s.gis.gismap.composer.pdf")?, ArtifactCapabilityKind::composer())
        .descriptor(b"s.stdio.pdf@1.4/*")?
        .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::composition_route(), "16:s.gis.gismap@1/*17:s.stdio.pdf@1.4/*")?)
}

/// 🔖️ The gismap declaration with its pdf route, for the composition root that offers pdf.
pub fn declaration() -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
    semio_s_artifact_gis_gismap::declaration_from(semio_s_artifact_gis_gismap::definition()?.capability(capability()?)?, &[entries()])
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
