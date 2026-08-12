//! 🎞️ Animate present artifact — document entities + `ArtifactKindSpec` (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub use crate::artifacts::present::schema::mutations::PresentMutation;

pub use crate::artifacts::present::schema::diff::PresentDiff;

pub const PRESENT_DOCUMENT_SCHEMA: &str = "animate.present";
pub use crate::artifacts::present::snapshot::schema::{default_snapshot, PresentSnapshot};

//#region 🔖️Domain
/// 📐️ Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
/// {@link FigureTileSource}/{@link FigureTileDraft}), so it declares no `#[dsl(keyword)]` of its own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    #[dsl(block)]
    pub frame: FigureTileFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub crop: FigureTileFrame,
}
//#endregion 🔖️Domain

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource {
        src: "/🖼️bauteilbörse.png".into(),
        kind: "figure".into(),
        frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 },
        source_aspect: Some(1222.0 / 896.0),
        pdf_page: None,
    }
}

pub fn default_present_snapshot() -> PresentSnapshot {
    default_snapshot()
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::present::create_animate_present_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: PRESENT_DOCUMENT_SCHEMA.into(),
        name: "Animate Present".into(),
        source_format: PRESENT_DOCUMENT_SCHEMA.into(),
        component_kind: "panel".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
        schema: PRESENT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec!["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<FigureTileDraftPatch> {
        Some(FigureTileDraftPatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            crop: (self.crop != other.crop).then(|| other.crop.clone()),
        })
    }
}
//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_snapshot_schema_is_animate_present() {
        assert_eq!(default_present_snapshot().schema, PRESENT_DOCUMENT_SCHEMA);
    }

    #[test]
    fn artifact_kind_matches_the_store_schema() {
        assert_eq!(artifact_kind().schema, PRESENT_DOCUMENT_SCHEMA);
        assert_eq!(artifact_kind().id, PRESENT_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::present::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("PresentComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
