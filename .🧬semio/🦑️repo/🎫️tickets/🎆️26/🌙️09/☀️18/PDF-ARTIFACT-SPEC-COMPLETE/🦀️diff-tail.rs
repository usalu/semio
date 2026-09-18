
//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and is never diffed. `info` is a
/// WEAK value struct (whole-value replaced, never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declared_version: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<PdfKeyedDiff<PdfFont>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<PdfKeyedDiff<PdfImage>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub forms: Option<PdfKeyedDiff<PdfFormXObject>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ext_g_states: Option<PdfKeyedDiff<PdfExtGState>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shadings: Option<PdfKeyedDiff<PdfShading>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub patterns: Option<PdfKeyedDiff<PdfPattern>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_spaces: Option<PdfKeyedDiff<PdfNamedColorSpace>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PdfKeyedDiff<PdfNamedProperties>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub outlines: Option<PdfIndexedDiff<PdfOutlineItem>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub named_destinations: Option<PdfIndexedDiff<PdfNamedDestination>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_labels: Option<PdfIndexedDiff<PdfPageLabelRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub embedded_files: Option<PdfKeyedDiff<PdfEmbeddedFile>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub output_intents: Option<PdfIndexedDiff<PdfOutputIntent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub acro_form: Option<PdfSet<PdfAcroForm>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub optional_content: Option<PdfSet<PdfOptionalContent>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_layout: Option<PdfSet<PdfPageLayout>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub page_mode: Option<PdfSet<PdfPageMode>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewer_preferences: Option<PdfSet<PdfViewerPreferences>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub open_action: Option<PdfSet<PdfOpenAction>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mark_info: Option<PdfSet<PdfMarkInfo>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<PdfSet<[Vec<u8>; 2]>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<PdfSet<PdfEncryption>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<PdfInfo>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub catalog_extra: Option<PdfDictDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<PdfObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "pages")?;
    for modified in &diff.modified {
        validate_page_diff(&modified.diff, &base[modified.index]).map_err(|error| error.under(vec!["pages".to_string(), modified.index.to_string()]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pdf_diff(diff: &PdfDiff, base: &PdfSnapshot) -> MutationApplyResult<()> {
    if let Some(pages) = &diff.pages {
        validate_pages_diff(pages, &base.pages).map_err(|error| error.under(["pages"]))?;
    }
    macro_rules! keyed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(&base.$field, stringify!($field))?;
            })*
        };
    }
    keyed!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files);
    macro_rules! indexed {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                lane.validate(base.$field.len(), stringify!($field))?;
            })*
        };
    }
    indexed!(outlines, named_destinations, page_labels, output_intents);
    if let Some(objects) = &diff.objects {
        validate_objects_diff(objects, &base.objects).map_err(|error| error.under(["objects"]))?;
    }
    if let Some(trailer) = &diff.trailer {
        validate_dict_diff(trailer, &base.trailer).map_err(|error| error.under(["trailer"]))?;
    }
    if let Some(extra) = &diff.catalog_extra {
        validate_dict_diff(extra, &base.catalog_extra).map_err(|error| error.under(["catalogExtra"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pdf_diff_unchecked(diff: &PdfDiff, base: &PdfSnapshot) -> PdfSnapshot {
    let mut next = base.clone();
    if let Some(v) = &diff.declared_version {
        next.declared_version = v.clone();
    }
    if let Some(v) = &diff.info {
        next.info = v.clone();
    }
    if let Some(pd) = &diff.pages {
        next.pages = apply_pages_diff(pd, &base.pages);
    }
    macro_rules! lanes {
        ($($field:ident),*) => {
            $(if let Some(lane) = &diff.$field {
                next.$field = lane.apply(&base.$field);
            })*
        };
    }
    lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents);
    apply_tri(&mut next.acro_form, &diff.acro_form);
    apply_tri(&mut next.optional_content, &diff.optional_content);
    apply_tri(&mut next.page_layout, &diff.page_layout);
    apply_tri(&mut next.page_mode, &diff.page_mode);
    apply_tri(&mut next.viewer_preferences, &diff.viewer_preferences);
    apply_tri(&mut next.open_action, &diff.open_action);
    apply_tri(&mut next.language, &diff.language);
    apply_tri(&mut next.mark_info, &diff.mark_info);
    apply_tri(&mut next.metadata, &diff.metadata);
    apply_tri(&mut next.document_id, &diff.document_id);
    apply_tri(&mut next.encryption, &diff.encryption);
    if let Some(extra) = &diff.catalog_extra {
        next.catalog_extra = apply_dict_diff(extra, &base.catalog_extra);
    }
    if let Some(od) = &diff.objects {
        next.objects = apply_objects_diff(od, &base.objects);
    }
    if let Some(td) = &diff.trailer {
        next.trailer = apply_dict_diff(td, &base.trailer);
    }
    next
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_option<T>(slot: &mut Option<T>, other: Option<T>, merge: impl FnOnce(T, T) -> Option<T>) {
    *slot = match (slot.take(), other) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => merge(a, b),
    };
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot) -> MutationApplyResult<PdfSnapshot> {
        validate_pdf_diff(self, base)?;
        Ok(apply_pdf_diff_unchecked(self, base))
    }

    /// ➕️ Structural, total, base-free sequential-coalesce absorb (`## Absorb` contract).
    /// Scalars and tri-states: LWW. Triples: composed via their own key/index-transported absorb.
    fn absorb(&mut self, other: Self) {
        if other.declared_version.is_some() {
            self.declared_version = other.declared_version;
        }
        if other.info.is_some() {
            self.info = other.info;
        }
        absorb_option(&mut self.pages, other.pages, |a, b| {
            let m = absorb_law_pages_diff(a, &b);
            (!m.is_empty()).then_some(m)
        });
        macro_rules! lanes {
            ($($field:ident),*) => {
                $(absorb_option(&mut self.$field, other.$field, |a, b| {
                    let m = a.absorb(b);
                    (!m.is_empty()).then_some(m)
                });)*
            };
        }
        lanes!(fonts, images, forms, ext_g_states, shadings, patterns, color_spaces, properties, embedded_files, outlines, named_destinations, page_labels, output_intents);
        macro_rules! lww {
            ($($field:ident),*) => {
                $(if other.$field.is_some() {
                    self.$field = other.$field;
                })*
            };
        }
        lww!(acro_form, optional_content, page_layout, page_mode, viewer_preferences, open_action, language, mark_info, metadata, document_id, encryption);
        absorb_option(&mut self.catalog_extra, other.catalog_extra, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.objects, other.objects, |a, b| {
            let m = absorb_law_objects_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
        absorb_option(&mut self.trailer, other.trailer, |a, b| {
            let m = absorb_dict_diff(a, b);
            (!m.is_empty()).then_some(m)
        });
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction): the
    /// state delta from `self.apply(base)` back to `base`.
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = apply_pdf_diff_unchecked(self, base);
        Self::between(&mid, base)
    }

    /// 🧭️ State delta (compose `GetXDiff`): `pages` positionally matched, id collections by key,
    /// `objects` and `trailer` matched by their real keys (`ObjRef`/dict key name).
    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        fn keyed<T: Clone + PartialEq + Keyed>(a: &[T], b: &[T]) -> Option<PdfKeyedDiff<T>> {
            let d = PdfKeyedDiff::between(a, b);
            (!d.is_empty()).then_some(d)
        }
        fn indexed<T: Clone + PartialEq>(a: &[T], b: &[T]) -> Option<PdfIndexedDiff<T>> {
            let d = PdfIndexedDiff::between(a, b);
            (!d.is_empty()).then_some(d)
        }
        fn dict(a: &[PdfDictEntry], b: &[PdfDictEntry]) -> Option<PdfDictDiff> {
            let d = dict_diff_between(a, b);
            (!d.is_empty()).then_some(d)
        }
        let pages = {
            let d = pages_diff_between(&base.pages, &other.pages);
            (!d.is_empty()).then_some(d)
        };
        let objects = {
            let d = objects_diff_between(&base.objects, &other.objects);
            (!d.is_empty()).then_some(d)
        };
        PdfDiff {
            declared_version: (base.declared_version != other.declared_version).then(|| other.declared_version.clone()),
            pages,
            fonts: keyed(&base.fonts, &other.fonts),
            images: keyed(&base.images, &other.images),
            forms: keyed(&base.forms, &other.forms),
            ext_g_states: keyed(&base.ext_g_states, &other.ext_g_states),
            shadings: keyed(&base.shadings, &other.shadings),
            patterns: keyed(&base.patterns, &other.patterns),
            color_spaces: keyed(&base.color_spaces, &other.color_spaces),
            properties: keyed(&base.properties, &other.properties),
            outlines: indexed(&base.outlines, &other.outlines),
            named_destinations: indexed(&base.named_destinations, &other.named_destinations),
            page_labels: indexed(&base.page_labels, &other.page_labels),
            embedded_files: keyed(&base.embedded_files, &other.embedded_files),
            output_intents: indexed(&base.output_intents, &other.output_intents),
            acro_form: tri(&base.acro_form, &other.acro_form),
            optional_content: tri(&base.optional_content, &other.optional_content),
            page_layout: tri(&base.page_layout, &other.page_layout),
            page_mode: tri(&base.page_mode, &other.page_mode),
            viewer_preferences: tri(&base.viewer_preferences, &other.viewer_preferences),
            open_action: tri(&base.open_action, &other.open_action),
            language: tri(&base.language, &other.language),
            mark_info: tri(&base.mark_info, &other.mark_info),
            metadata: tri(&base.metadata, &other.metadata),
            document_id: tri(&base.document_id, &other.document_id),
            encryption: tri(&base.encryption, &other.encryption),
            info: (base.info != other.info).then(|| other.info.clone()),
            catalog_extra: dict(&base.catalog_extra, &other.catalog_extra),
            objects,
            trailer: dict(&base.trailer, &other.trailer),
        }
    }

    fn is_empty(&self) -> bool {
        self == &PdfDiff::default()
    }
}
//#endregion 🔖️Diff

//#region 🔖️MutationDiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_patch(index: usize, diff: PdfPageDiff) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_page(index: usize, page: PdfPage) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index, page }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_page(index: usize) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![index], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_media_box(index: usize, media_box: PdfRect) -> PdfDiff {
    page_patch(index, PdfPageDiff { media_box: Some(media_box), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_crop_box(index: usize, crop_box: Option<PdfRect>) -> PdfDiff {
    page_patch(index, PdfPageDiff { crop_box: Some(PdfSet::from_option(&crop_box)), ..Default::default() })
}
/// 📐 Sets one of the optional page boxes (`bleed`, `trim`, `art`) or clears it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_box(index: usize, kind: PdfPageBox, rect: Option<PdfRect>) -> PdfDiff {
    let set = Some(PdfSet::from_option(&rect));
    page_patch(
        index,
        match kind {
            PdfPageBox::Crop => PdfPageDiff { crop_box: set, ..Default::default() },
            PdfPageBox::Bleed => PdfPageDiff { bleed_box: set, ..Default::default() },
            PdfPageBox::Trim => PdfPageDiff { trim_box: set, ..Default::default() },
            PdfPageBox::Art => PdfPageDiff { art_box: set, ..Default::default() },
        },
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_rotation(index: usize, rotation: i32) -> PdfDiff {
    page_patch(index, PdfPageDiff { rotate: Some(rotation), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_user_unit(index: usize, user_unit: Option<f64>) -> PdfDiff {
    page_patch(index, PdfPageDiff { user_unit: Some(PdfSet::from_option(&user_unit)), ..Default::default() })
}
/// ✏️️ Replaces page `index`'s whole content (computed against `base` so the diff stays sparse
/// where operators agree).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let current: &[PdfOp] = base.pages.get(index).map(|page| page.content.as_slice()).unwrap_or(&[]);
    let content = PdfIndexedDiff::between(current, content);
    if content.is_empty() {
        return PdfDiff::default();
    }
    page_patch(index, PdfPageDiff { content: Some(content), ..Default::default() })
}
/// ➕️ Appends operators to page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_append_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let start = base.pages.get(index).map(|page| page.content.len()).unwrap_or(0);
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: start + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// ➕️ Inserts operators at position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_content(index: usize, at: usize, content: &[PdfOp]) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: at + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// 🗑️ Removes `count` operators from position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_content(index: usize, at: usize, count: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { removed: (at..at + count).collect(), ..Default::default() }), ..Default::default() })
}
/// ✏️️ Replaces the operator at `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_content(index: usize, at: usize, op: PdfOp) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: op }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { added: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_annotation(index: usize, at: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
/// 🔀️ Moves the page at BASE-state index `from` to FINAL-state index `to` -- `removed`/`added`
/// compose the move (no dedicated "moved" slot on `PdfPagesDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_page(base: &PdfSnapshot, from: usize, to: usize) -> PdfDiff {
    let Some(page) = base.pages.get(from) else { return PdfDiff::default() };
    let final_to = to.min(base.pages.len().saturating_sub(1));
    if from == final_to {
        return PdfDiff::default();
    }
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![from], added: vec![PdfPageAdded { index: final_to, page: page.clone() }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_info(info: PdfInfo) -> PdfDiff {
    PdfDiff { info: Some(info), ..Default::default() }
}

/// 🆔 Upsert of one keyed collection item: `modified` when the key exists in `base`, `added`
/// at the end otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_upsert<T: Clone + PartialEq + Keyed>(base: &[T], value: T) -> Option<PdfKeyedDiff<T>> {
    match base.iter().find(|item| item.key() == value.key()) {
        Some(existing) if *existing == value => None,
        Some(_) => Some(PdfKeyedDiff { modified: vec![PdfKeyedItem { key: value.key().to_string(), value }], ..Default::default() }),
        None => Some(PdfKeyedDiff { added: vec![PdfIndexedItem { index: base.len(), value }], ..Default::default() }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_remove<T: Clone + PartialEq + Keyed>(base: &[T], key: &str) -> Option<PdfKeyedDiff<T>> {
    base.iter().any(|item| item.key() == key).then(|| PdfKeyedDiff { removed: vec![key.to_string()], ..Default::default() })
}
macro_rules! keyed_builders {
    ($($set:ident / $remove:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $set(base: &PdfSnapshot, value: $ty) -> PdfDiff {
                PdfDiff { $field: keyed_upsert(&base.$field, value), ..Default::default() }
            }
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $remove(base: &PdfSnapshot, key: &str) -> PdfDiff {
                PdfDiff { $field: keyed_remove(&base.$field, key), ..Default::default() }
            }
        )*
    };
}
keyed_builders!(
    diff_set_font / diff_remove_font => fonts: PdfFont,
    diff_set_image / diff_remove_image => images: PdfImage,
    diff_set_form / diff_remove_form => forms: PdfFormXObject,
    diff_set_ext_g_state / diff_remove_ext_g_state => ext_g_states: PdfExtGState,
    diff_set_shading / diff_remove_shading => shadings: PdfShading,
    diff_set_pattern / diff_remove_pattern => patterns: PdfPattern,
    diff_set_color_space / diff_remove_color_space => color_spaces: PdfNamedColorSpace,
    diff_set_properties / diff_remove_properties => properties: PdfNamedProperties,
    diff_set_embedded_file / diff_remove_embedded_file => embedded_files: PdfEmbeddedFile,
);
/// 📑 Replaces the outline tree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_outlines(base: &PdfSnapshot, outlines: &[PdfOutlineItem]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.outlines, outlines);
    PdfDiff { outlines: (!d.is_empty()).then_some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_named_destination(base: &PdfSnapshot, destination: PdfNamedDestination) -> PdfDiff {
    let d = match base.named_destinations.iter().position(|item| item.name == destination.name) {
        Some(index) if base.named_destinations[index] == destination => return PdfDiff::default(),
        Some(index) => PdfIndexedDiff { modified: vec![PdfIndexedItem { index, value: destination }], ..Default::default() },
        None => PdfIndexedDiff { added: vec![PdfIndexedItem { index: base.named_destinations.len(), value: destination }], ..Default::default() },
    };
    PdfDiff { named_destinations: Some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_named_destination(base: &PdfSnapshot, name: &str) -> PdfDiff {
    match base.named_destinations.iter().position(|item| item.name == name) {
        Some(index) => PdfDiff { named_destinations: Some(PdfIndexedDiff { removed: vec![index], ..Default::default() }), ..Default::default() },
        None => PdfDiff::default(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_labels(base: &PdfSnapshot, labels: &[PdfPageLabelRange]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.page_labels, labels);
    PdfDiff { page_labels: (!d.is_empty()).then_some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_output_intents(base: &PdfSnapshot, intents: &[PdfOutputIntent]) -> PdfDiff {
    let d = PdfIndexedDiff::between(&base.output_intents, intents);
    PdfDiff { output_intents: (!d.is_empty()).then_some(d), ..Default::default() }
}
macro_rules! tri_builders {
    ($($name:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
            pub fn $name(base: &PdfSnapshot, value: Option<$ty>) -> PdfDiff {
                PdfDiff { $field: tri(&base.$field, &value), ..Default::default() }
            }
        )*
    };
}
tri_builders!(
    diff_set_acro_form => acro_form: PdfAcroForm,
    diff_set_optional_content => optional_content: PdfOptionalContent,
    diff_set_page_layout => page_layout: PdfPageLayout,
    diff_set_page_mode => page_mode: PdfPageMode,
    diff_set_viewer_preferences => viewer_preferences: PdfViewerPreferences,
    diff_set_open_action => open_action: PdfOpenAction,
    diff_set_language => language: String,
    diff_set_mark_info => mark_info: PdfMarkInfo,
    diff_set_metadata => metadata: String,
    diff_set_document_id => document_id: [Vec<u8>; 2],
    diff_set_encryption => encryption: PdfEncryption,
);
/// 🔧️ Upserts `key` in the catalog's retained entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_catalog_entry(base: &PdfSnapshot, key: &str, value: PdfObject) -> PdfDiff {
    let leaf = match base.catalog_extra.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&base.catalog_extra[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: base.catalog_extra.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { catalog_extra: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_catalog_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.catalog_extra.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { catalog_extra: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_object(id: ObjRef, index: usize, value: PdfObject) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { added: vec![PdfObjectAdded { index, id, value }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_object(id: ObjRef) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { removed: vec![id], ..Default::default() }), ..Default::default() }
}
/// 🔧️ Upserts object `id`'s value: `modified` against BASE if present, `added` (at the final Vec
/// position) otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_object_value(base: &PdfSnapshot, id: ObjRef, value: PdfObject) -> PdfDiff {
    match base.objects.iter().find(|o| o.id == id) {
        Some(existing) => match value_diff_between(&existing.value, &value) {
            None => PdfDiff::default(),
            Some(d) => PdfDiff { objects: Some(PdfObjectsDiff { modified: vec![PdfObjectModified { id, diff: d }], ..Default::default() }), ..Default::default() },
        },
        None => diff_insert_object(id, base.objects.len(), value),
    }
}
/// 🔧️ Upserts `key` at `path` inside object `id`'s value tree (`modified` if `key` already
/// exists at that container, `added` otherwise). Graceful empty diff if `id`/`path` don't
/// resolve to a real `Dict`/`Stream` container in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str, value: PdfObject) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = match entries.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&entries[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: entries.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Removes `key` at `path` inside object `id`'s value tree. Graceful empty diff if the key
/// isn't actually present in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    if !entries.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = PdfDictDiff { removed: vec![key.to_string()], ..Default::default() };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Upserts `key` in the top-level trailer dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_trailer_entry(base: &PdfSnapshot, key: &str, value: PdfObject) -> PdfDiff {
    let leaf = match base.trailer.iter().position(|e| e.key == key) {
        Some(pos) => match value_diff_between(&base.trailer[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: base.trailer.len(), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { trailer: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_trailer_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.trailer.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { trailer: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}

/// 📐 Which optional page box a mutation addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PdfPageBox {
    Crop,
    Bleed,
    Trim,
    Art,
}
//#endregion 🔖️MutationDiffBuilders

//#region 🔖️DiffCodec
/// 🧾 One codec for every lane: the derive-owned value encoding. Text is the compact JSON line
/// (`📝️text/📖️.grammar.semio`), binary is the `OP_BINARY_FORMAT` byte followed by the
/// container-less pack record body of the same value (`💾️binary/📡️.protocol.semio`). Both are
/// deterministic and decode back to the identical `PdfDiff`.
impl protocol::DiffCodec for PdfDiff {
    fn print_diff(&self) -> String {
        pack::to_json_string(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        pack::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        use pack::value::ToValue;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        out.extend_from_slice(&store::pack_rt::encode_wire_value(&self.to_value()));
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        use pack::value::FromValue;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        match bytes.first() {
            Some(format) if *format == store::pack_rt::OP_BINARY_FORMAT => {}
            Some(format) => return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT))),
            None => return Err(malformed("diff format", 0, "empty diff".into())),
        }
        let value = store::pack_rt::decode_wire_value(&bytes[1..]).map_err(|error| malformed("diff body", 1, error.to_string()))?;
        Self::from_value(value).map_err(|error| malformed("diff value", 1, error.to_string()))
    }
}
//#endregion 🔖️DiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
pub use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;
//#endregion 🔁️Re-exports
