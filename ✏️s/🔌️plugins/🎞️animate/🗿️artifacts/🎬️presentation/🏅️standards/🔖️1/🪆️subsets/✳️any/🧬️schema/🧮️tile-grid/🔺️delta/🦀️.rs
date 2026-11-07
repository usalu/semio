//! 🔺️ Tile row delta of the presentation: the list delta and its row patch live with the crate that owns the `FigureTileDraft` row.

use crate::{FigureTileDraft, FigureTileFrame};

protocol::list_delta! {
    pub PresentationTilesDelta { removal: PresentationTilesRemoval, insertion: PresentationTilesInsertion, relocation: PresentationTilesRelocation, modification: PresentationTilesModification, row: FigureTileDraft, patch: PresentationTilePatch, key: id, values_only }
}

/// 🩹 Field patch of one tile; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationTilePatch {
    pub name: Option<String>,
    pub crop: Option<FigureTileFrame>,
}

impl protocol::list_delta::RowPatch<FigureTileDraft> for PresentationTilePatch {
    fn commit_into(&self, row: &mut FigureTileDraft, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(crop) = &self.crop {
            row.crop = crop.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.crop = later.crop.or_else(|| self.crop.take());
    }
    fn inverse(&self, row: &FigureTileDraft) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), crop: self.crop.as_ref().map(|_| row.crop.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.crop.is_none()
    }
}
