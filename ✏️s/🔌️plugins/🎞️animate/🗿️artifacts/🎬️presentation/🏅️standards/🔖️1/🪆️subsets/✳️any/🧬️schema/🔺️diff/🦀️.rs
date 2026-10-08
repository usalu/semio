//! 🧬️ Presentation diff schema — sparse field delta over the artifact. `source` is a field patch of the shared figure and `tiles`
//! an id-keyed row delta (`added`/`removed`/`patched`/`reordered`); `presentation` is the content-addressed handle minted for the
//! resulting `(source, tiles)`.

use crate::{FigureTileDraft, FigureTileFrame, FigureTileSource, PresentationChild, PresentationSnapshot};
use schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the presentation artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `presentation` carries a whole-handle replacement (content-addressed, so a changed handle IS the change signal), minted from
/// the post-delta `(source, tiles)` payload by [`diff_set_presentation`]. `animation` never changes (see
/// `crate::animation_child_handle`'s doc comment), so this diff carries no field for it.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    /// 🖼️ Field patch of the persisted figure the replacement `presentation` handle was minted from.
    #[state(artifact)]
    pub source: Option<PresentationSourcePatch>,
    /// 🧱 Row delta of the tiles the replacement `presentation` handle was minted from.
    #[state(artifact)]
    pub tiles: Option<PresentationTilesDelta>,
    #[state(artifact)]
    pub presentation: Option<PresentationChild>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧱️ Carries the optional source aspect as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationOptionalAspect {
    pub value: Option<f64>,
}

/// 🧱️ Carries the optional PDF page as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationOptionalPage {
    pub value: Option<u32>,
}

/// 🩹 Field patch of the shared figure; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationSourcePatch {
    pub src: Option<String>,
    pub kind: Option<String>,
    pub frame: Option<FigureTileFrame>,
    pub source_aspect: Option<PresentationOptionalAspect>,
    pub pdf_page: Option<PresentationOptionalPage>,
}

/// 🧩️ Id-keyed row delta for the tiles.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationTilesDelta {
    pub added: Vec<FigureTileDraft>,
    pub removed: Vec<String>,
    pub patched: Vec<PresentationTilePatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 Field patch of one tile; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationTilePatch {
    pub id: String,
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

    /// 🔁️ The patch replacing `base` by `replacement` field by field, `None` when they are equal.
    pub fn replacing(base: &FigureTileSource, other: &FigureTileSource) -> Option<Self> {
        let patch = Self {
            src: (base.src != other.src).then(|| other.src.clone()),
            kind: (base.kind != other.kind).then(|| other.kind.clone()),
            frame: (base.frame != other.frame).then(|| other.frame.clone()),
            source_aspect: (base.source_aspect != other.source_aspect).then(|| PresentationOptionalAspect { value: other.source_aspect }),
            pdf_page: (base.pdf_page != other.pdf_page).then(|| PresentationOptionalPage { value: other.pdf_page }),
        };
        (!patch.is_empty()).then_some(patch)
    }

    fn is_empty(&self) -> bool {
        self.src.is_none() && self.kind.is_none() && self.frame.is_none() && self.source_aspect.is_none() && self.pdf_page.is_none()
    }
}

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for PresentationTilesDelta {
    type Row = FigureTileDraft;
    type Patch = PresentationTilePatch;
    fn added(&self) -> &[FigureTileDraft] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[PresentationTilePatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<FigureTileDraft>, removed: Vec<String>, patched: Vec<PresentationTilePatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &FigureTileDraft) -> &str {
        &row.id
    }
    fn patch_key(patch: &PresentationTilePatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &PresentationTilePatch, row: &mut FigureTileDraft) -> Result<(), protocol::MutationApplyError> {
        if let Some(name) = &patch.name {
            row.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            row.crop = crop.clone();
        }
        Ok(())
    }
    fn patch_compose(first: &PresentationTilePatch, later: &PresentationTilePatch) -> PresentationTilePatch {
        PresentationTilePatch { id: first.id.clone(), name: later.name.clone().or_else(|| first.name.clone()), crop: later.crop.clone().or_else(|| first.crop.clone()) }
    }
    fn patch_inverse(patch: &PresentationTilePatch, base: &FigureTileDraft) -> PresentationTilePatch {
        PresentationTilePatch { id: patch.id.clone(), name: patch.name.as_ref().map(|_| base.name.clone()), crop: patch.crop.as_ref().map(|_| base.crop.clone()) }
    }
    fn patch_between(base: &FigureTileDraft, other: &FigureTileDraft) -> Option<PresentationTilePatch> {
        let patch = PresentationTilePatch { id: other.id.clone(), name: (base.name != other.name).then(|| other.name.clone()), crop: (base.crop != other.crop).then(|| other.crop.clone()) };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &PresentationTilePatch) -> bool {
        patch.name.is_none() && patch.crop.is_none()
    }
}

impl MutationDiff<PresentationSnapshot> for PresentationDiff {
    fn apply(&self, snapshot: &PresentationSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PresentationSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(patch) = &self.source {
            next.source = patch.fold(&next.source);
        }
        if let Some(delta) = &self.tiles {
            next.tiles = keyed_apply(&next.tiles, delta).map_err(|error| error.under(["tiles"]))?;
        }
        if let Some(presentation) = &self.presentation {
            next.presentation = presentation.clone();
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
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        if other.presentation.is_some() {
            self.presentation = other.presentation;
        }
    }
}

impl protocol::DiffAlgebra<PresentationSnapshot> for PresentationDiff {
    fn inverse(&self, base: &PresentationSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            source: self.source.as_ref().map(|patch| patch.inverse(&base.source)),
            tiles: self.tiles.as_ref().map(|delta| keyed_inverse(delta, &base.tiles)),
            presentation: self.presentation.as_ref().map(|_| base.presentation.clone()),
        }
    }
    fn between(base: &PresentationSnapshot, other: &PresentationSnapshot) -> Self {
        let tiles = keyed_between::<PresentationTilesDelta>(&base.tiles, &other.tiles);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            source: PresentationSourcePatch::replacing(&base.source, &other.source),
            tiles: (!keyed_is_empty(&tiles)).then_some(tiles),
            presentation: (base.presentation != other.presentation).then(|| other.presentation.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.source.as_ref().is_none_or(PresentationSourcePatch::is_empty) && self.tiles.as_ref().is_none_or(keyed_is_empty) && self.presentation.is_none()
    }
}

/// 🔁️ The row delta replacing the `base` tiles by `replacement`: what a whole-collection replace kind names.
pub fn tiles_replacing(base: &[FigureTileDraft], replacement: &[FigureTileDraft]) -> PresentationTilesDelta {
    keyed_between::<PresentationTilesDelta>(base, replacement)
}

/// 🔺️ Completes a `(source patch, tiles delta)` pair with the content-addressed `presentation` handle minted from the resulting
/// `(source, tiles)` — the handle is derived from the post-delta payload, so it is computed here from `base` and the two deltas,
/// never captured from an applied snapshot.
pub fn diff_set_presentation(base: &PresentationSnapshot, source: Option<PresentationSourcePatch>, tiles: Option<PresentationTilesDelta>) -> PresentationDiff {
    let after_source = source.as_ref().map_or_else(|| base.source.clone(), |patch| patch.fold(&base.source));
    let after_tiles = tiles.as_ref().map_or_else(|| base.tiles.clone(), |delta| keyed_apply(&base.tiles, delta).unwrap_or_else(|_| base.tiles.clone()));
    PresentationDiff { source, tiles, presentation: Some(crate::presentation_child_handle(&after_source, &after_tiles)), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
