//! 🧬️ Presentation diff schema — sparse field delta over the artifact. `source` is a field patch of the shared figure and `tiles`
//! a positional row delta (`removed`/`inserted`/`moved`/`patched`, see `protocol::list_delta`). The content-addressed `presentation` handle is never carried:
//! `apply` re-derives it from the `(source, tiles)` it leaves behind.

use crate::{FigureTileDraft, FigureTileFrame, FigureTileSource, PresentationSnapshot};
use schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the presentation artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// The derived `presentation` handle and the immutable `animation` handle (see `crate::animation_child_handle`'s doc comment) have
/// no field here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    /// 🖼️ Field patch of the persisted figure.
    #[state(artifact)]
    pub source: Option<PresentationSourcePatch>,
    /// 🧱 Row delta of the tiles.
    #[state(artifact)]
    pub tiles: Option<PresentationTilesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧱️ Carries the optional source aspect as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationOptionalAspect {
    pub value: Option<f64>,
}

/// 🧱️ Carries the optional PDF page as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationOptionalPage {
    pub value: Option<u32>,
}

/// 🩹 Field patch of the shared figure; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationSourcePatch {
    pub src: Option<String>,
    pub kind: Option<String>,
    pub frame: Option<FigureTileFrame>,
    pub source_aspect: Option<PresentationOptionalAspect>,
    pub pdf_page: Option<PresentationOptionalPage>,
}

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
//#endregion 🔖️DeltaHelpers

impl PresentationSourcePatch {
    fn fold(&self, source: &FigureTileSource) -> FigureTileSource {
        FigureTileSource {
            src: self.src.clone().unwrap_or_else(|| source.src.clone()),
            kind: self.kind.clone().unwrap_or_else(|| source.kind.clone()),
            frame: self.frame.clone().unwrap_or_else(|| source.frame.clone()),
            source_aspect: self.source_aspect.as_ref().map_or(source.source_aspect, |aspect| aspect.value),
            pdf_page: self.pdf_page.as_ref().map_or(source.pdf_page, |page| page.value),
        }
    }

    fn compose(&self, later: &Self) -> Self {
        Self {
            src: later.src.clone().or_else(|| self.src.clone()),
            kind: later.kind.clone().or_else(|| self.kind.clone()),
            frame: later.frame.clone().or_else(|| self.frame.clone()),
            source_aspect: later.source_aspect.clone().or_else(|| self.source_aspect.clone()),
            pdf_page: later.pdf_page.clone().or_else(|| self.pdf_page.clone()),
        }
    }

    fn inverse(&self, base: &FigureTileSource) -> Self {
        Self {
            src: self.src.as_ref().map(|_| base.src.clone()),
            kind: self.kind.as_ref().map(|_| base.kind.clone()),
            frame: self.frame.as_ref().map(|_| base.frame.clone()),
            source_aspect: self.source_aspect.as_ref().map(|_| PresentationOptionalAspect { value: base.source_aspect }),
            pdf_page: self.pdf_page.as_ref().map(|_| PresentationOptionalPage { value: base.pdf_page }),
        }
    }

    fn is_empty(&self) -> bool {
        self.src.is_none() && self.kind.is_none() && self.frame.is_none() && self.source_aspect.is_none() && self.pdf_page.is_none()
    }
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

impl MutationDiff<PresentationSnapshot> for PresentationDiff {
    fn apply(&self, snapshot: &PresentationSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PresentationSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(patch) = &self.source {
            next.source = patch.fold(&next.source);
        }
        if let Some(delta) = &self.tiles {
            next.tiles = delta.commit_onto(&next.tiles, capability).map_err(|error| error.under(["tiles"]))?;
        }
        if self.source.is_some() || self.tiles.is_some() {
            next.presentation = crate::presentation_child_handle(&next.source, &next.tiles);
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.source = match (self.source.take(), other.source) {
            (Some(first), Some(later)) => Some(first.compose(&later)),
            (first, later) => later.or(first),
        };
        self.tiles = match (self.tiles.take(), other.tiles) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<PresentationSnapshot> for PresentationDiff {
    fn inverse(&self, base: &PresentationSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            source: self.source.as_ref().map(|patch| patch.inverse(&base.source)),
            tiles: self.tiles.as_ref().map(|delta| delta.inverse(&base.tiles)),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.source.as_ref().is_none_or(PresentationSourcePatch::is_empty) && self.tiles.as_ref().is_none_or(PresentationTilesDelta::is_empty)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
