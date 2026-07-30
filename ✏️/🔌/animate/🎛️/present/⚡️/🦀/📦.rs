//! 🎞️ Animate present app — document entities (constitutional: general).

use protocol::{Identified, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖Domain
/// 📐 Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
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

/// 📜 `.present` textual document: `schema=... \n source { ... } \n tiles [ ... ]` (see
/// {@link store::DocumentDsl}).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[dsl(extension = "present", layout = "lines")]
#[serde(rename_all = "camelCase")]
pub struct PresentDeck {
    pub schema: String,
    #[dsl(block)]
    pub source: FigureTileSource,
    #[dsl(table)]
    pub tiles: Vec<FigureTileDraft>,
}

pub const PRESENT_DECK_SCHEMA: &str = "animate.present.deck";

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: "/bauteilbörse.png".into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(1222.0 / 896.0), pdf_page: None }
}

pub fn default_present_deck() -> PresentDeck {
    PresentDeck { schema: PRESENT_DECK_SCHEMA.into(), source: default_figure_tile_source(), tiles: Vec::new() }
}
//#endregion 🔖Domain

//#region 🔖CollectionSupport
/// 🪪 Orphan-rule anchor: `Identified`/`Patchable` (from `protocol`) can only be implemented for
/// `FigureTileDraft` in the crate that defines it — this is that crate.
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
        Some(FigureTileDraftPatch { name: (self.name != other.name).then(|| other.name.clone()), crop: (self.crop != other.crop).then(|| other.crop.clone()) })
    }
}
//#endregion 🔖CollectionSupport

//#region 🔖Dsl
/// 📜 `PresentDeck`'s `.present` DSL grammar (`schema=... source { ... } tiles [ ... ]`) is declared
/// directly on the struct definitions above via `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslRecord)]`
/// (see {@link store::DocumentDsl}).
//#endregion 🔖Dsl

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_deck_schema_is_animate_present() {
        assert_eq!(default_present_deck().schema, PRESENT_DECK_SCHEMA);
    }
}
//#endregion 🧪Tests
