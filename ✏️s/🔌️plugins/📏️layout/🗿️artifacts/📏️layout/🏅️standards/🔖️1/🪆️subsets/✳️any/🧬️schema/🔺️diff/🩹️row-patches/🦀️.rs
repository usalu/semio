//! 🩹️ Row patches of the layout diff: the paragraph, character, parent page, spread, page and grid patches and their row patch implementations. See [`super`].

use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;

//#region 🔖️DeltaHelpers
/// 🩹 Sparse patch for a {@link ParagraphStyle}.
#[derive(Clone, Debug, Default, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParagraphStylePatch {
    pub name: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub leading: Option<f64>,
    pub tracking: Option<f64>,
    pub alignment: Option<String>,
}

/// 🩹 Sparse patch for a {@link CharacterStyle}.
#[derive(Clone, Debug, Default, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CharacterStylePatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub font_family: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub font_size: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub font_weight: Option<Option<u32>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub italic: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub color: Option<Option<[f32; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option_serde"))]
    pub tracking: Option<Option<f64>>,
}

/// 🩹 Sparse patch for a {@link ParentPage}.
#[derive(Clone, Debug, Default, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParentPagePatch {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// 🩹 Sparse patch for a {@link Spread}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct SpreadPatch {
    pub name: Option<String>,
    pub page_ids: Option<Vec<String>>,
}

/// 📄️ Owned-field patch of one page: its dimensions, margins, columns and parent, the positional guides, and the keyed overrides, frames and layers.
#[derive(Clone, Debug, Default, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(default, deny_unknown_fields)]
pub struct PagePatch {
    pub name: Option<String>,
    pub spread_id: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_right: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_left: Option<f64>,
    pub columns_count: Option<u32>,
    pub columns_gutter: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    pub parent_page_id: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::deserialize_double_option", deserialize_controlled_with = "crate::deserialize_double_option_controlled", default_controlled = "crate::default_double_option_controlled", retire_with = "crate::retire_double_option")]
    pub background_drawing: Option<Option<LayoutDrawingMember>>,
    pub guides: Option<PageGuidesDelta>,
    pub overrides: PageOverridesDelta,
    pub frames: PageFramesDelta,
    pub layers: PageLayersDelta,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️RowPatches
macro_rules! field_row_patch {
    ($patch:ty, $row:ty, |$patch_id:ident, $base_id:ident| $inverse:expr) => {
        impl RowPatch<$row> for $patch {
            fn commit_into(&self, row: &mut $row, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
                row.apply_patch(self);
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                let first = std::mem::take(self);
                *self = later.or(first);
            }
            fn inverse(&self, row: &$row) -> Self {
                let ($patch_id, $base_id) = (self, row);
                $inverse
            }
            fn is_empty(&self) -> bool {
                *self == Self::default()
            }
        }
    };
}

impl ParagraphStylePatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self {
            name: self.name.or(earlier.name),
            font_family: self.font_family.or(earlier.font_family),
            font_size: self.font_size.or(earlier.font_size),
            font_weight: self.font_weight.or(earlier.font_weight),
            leading: self.leading.or(earlier.leading),
            tracking: self.tracking.or(earlier.tracking),
            alignment: self.alignment.or(earlier.alignment),
        }
    }
}

impl CharacterStylePatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self {
            name: self.name.or(earlier.name),
            font_family: self.font_family.or(earlier.font_family),
            font_size: self.font_size.or(earlier.font_size),
            font_weight: self.font_weight.or(earlier.font_weight),
            italic: self.italic.or(earlier.italic),
            color: self.color.or(earlier.color),
            tracking: self.tracking.or(earlier.tracking),
        }
    }
}

impl ParentPagePatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self { name: self.name.or(earlier.name), width: self.width.or(earlier.width), height: self.height.or(earlier.height) }
    }
}

impl SpreadPatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self { name: self.name.or(earlier.name), page_ids: self.page_ids.or(earlier.page_ids) }
    }
}

impl ImageLinkPatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self { path: self.path.or(earlier.path), width: self.width.or(earlier.width), height: self.height.or(earlier.height), dpi: self.dpi.or(earlier.dpi), color_profile: self.color_profile.or(earlier.color_profile) }
    }
}

field_row_patch!(ParagraphStylePatch, ParagraphStyle, |patch, base| ParagraphStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    leading: patch.leading.map(|_| base.leading),
    tracking: patch.tracking.map(|_| base.tracking),
    alignment: patch.alignment.as_ref().map(|_| base.alignment.clone()),
});

field_row_patch!(CharacterStylePatch, CharacterStyle, |patch, base| CharacterStylePatch {
    name: patch.name.as_ref().map(|_| base.name.clone()),
    font_family: patch.font_family.as_ref().map(|_| base.font_family.clone()),
    font_size: patch.font_size.map(|_| base.font_size),
    font_weight: patch.font_weight.map(|_| base.font_weight),
    italic: patch.italic.map(|_| base.italic),
    color: patch.color.map(|_| base.color),
    tracking: patch.tracking.map(|_| base.tracking),
});

field_row_patch!(ParentPagePatch, ParentPage, |patch, base| ParentPagePatch { name: patch.name.as_ref().map(|_| base.name.clone()), width: patch.width.map(|_| base.width), height: patch.height.map(|_| base.height) });

field_row_patch!(SpreadPatch, Spread, |patch, base| SpreadPatch { name: patch.name.as_ref().map(|_| base.name.clone()), page_ids: patch.page_ids.as_ref().map(|_| base.page_ids.clone()) });

field_row_patch!(ImageLinkPatch, ImageLink, |patch, base| ImageLinkPatch {
    path: patch.path.as_ref().map(|_| base.path.clone()),
    width: patch.width.map(|_| base.width),
    height: patch.height.map(|_| base.height),
    dpi: patch.dpi.map(|_| base.dpi),
    color_profile: patch.color_profile.as_ref().map(|_| base.color_profile.clone()),
});

impl RowPatch<TextStory> for TextStoryPatch {
    fn commit_into(&self, row: &mut TextStory, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(content) = &self.content {
            row.content = content.clone();
        }
        if let Some(delta) = &self.style_runs {
            row.style_runs = apply_positional(&row.style_runs, delta).map_err(|error| error.under(["styleRuns"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.content = later.content.or_else(|| self.content.take());
        self.style_runs = match (self.style_runs.take(), later.style_runs) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
    }
    fn inverse(&self, row: &TextStory) -> Self {
        Self { content: self.content.as_ref().map(|_| row.content.clone()), style_runs: self.style_runs.as_ref().map(|delta| inverse_positional(delta, &row.style_runs)) }
    }
    fn is_empty(&self) -> bool {
        self.content.is_none() && self.style_runs.as_ref().is_none_or(is_empty_positional)
    }
}

impl FramePatch {
    pub(super) fn or(self, earlier: Self) -> Self {
        Self {
            x: self.x.or(earlier.x),
            y: self.y.or(earlier.y),
            width: self.width.or(earlier.width),
            height: self.height.or(earlier.height),
            rotation: self.rotation.or(earlier.rotation),
            fill: self.fill.or(earlier.fill),
            stroke: self.stroke.or(earlier.stroke),
            wrap_mode: self.wrap_mode.or(earlier.wrap_mode),
            columns: self.columns.or(earlier.columns),
            locked: self.locked.or(earlier.locked),
            visible: self.visible.or(earlier.visible),
            story_id: self.story_id.or(earlier.story_id),
            thread_next: self.thread_next.or(earlier.thread_next),
            inset_x: self.inset_x.or(earlier.inset_x),
            inset_y: self.inset_y.or(earlier.inset_y),
            inset_width: self.inset_width.or(earlier.inset_width),
            inset_height: self.inset_height.or(earlier.inset_height),
            layer_id: self.layer_id.or(earlier.layer_id),
        }
    }

    /// ↩️ The absolute setters restoring the fields this patch sets to the values `frame` holds.
    fn restoring(&self, frame: &Frame) -> Self {
        let bounds = frame.bounds();
        let (fill, stroke) = match frame {
            Frame::Rect { fill, stroke, .. } => (Some(*fill), Some(*stroke)),
            _ => (None, None),
        };
        let text = match frame {
            Frame::Text { wrap_mode, columns, story_id, thread_next, inset, .. } => Some((wrap_mode, *columns, story_id, thread_next, inset)),
            _ => None,
        };
        Self {
            x: self.x.map(|_| bounds.x),
            y: self.y.map(|_| bounds.y),
            width: self.width.map(|_| bounds.width),
            height: self.height.map(|_| bounds.height),
            rotation: self.rotation.map(|_| bounds.rotation),
            fill: self.fill.and(fill),
            stroke: self.stroke.and(stroke),
            wrap_mode: self.wrap_mode.as_ref().and(text.map(|text| text.0.clone())),
            columns: self.columns.and(text.map(|text| text.1)),
            locked: self.locked.map(|_| frame.locked()),
            visible: self.visible.map(|_| frame.visible()),
            story_id: self.story_id.as_ref().and(text.map(|text| text.2.clone())),
            thread_next: self.thread_next.as_ref().and(text.map(|text| text.3.clone())),
            inset_x: self.inset_x.and(text.map(|text| text.4.x)),
            inset_y: self.inset_y.and(text.map(|text| text.4.y)),
            inset_width: self.inset_width.and(text.map(|text| text.4.width)),
            inset_height: self.inset_height.and(text.map(|text| text.4.height)),
            layer_id: self.layer_id.as_ref().map(|_| frame.layer_id().to_string()),
        }
    }
}

impl RowPatch<Frame> for FramePatch {
    fn commit_into(&self, row: &mut Frame, _capability: ApplyCapability) -> Result<(), MutationApplyError> {
        crate::apply_frame_field_patch(row, self);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let first = std::mem::take(self);
        *self = later.or(first);
    }
    fn inverse(&self, row: &Frame) -> Self {
        self.restoring(row)
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl RowPatch<Page> for PagePatch {
    fn commit_into(&self, row: &mut Page, capability: ApplyCapability) -> Result<(), MutationApplyError> {
        if let Some(spread) = &self.spread_id { row.spread_id = spread.clone(); }
        if let Some(name) = &self.name {
            row.name = name.clone();
        }
        if let Some(value) = self.width {
            row.width = value;
        }
        if let Some(value) = self.height {
            row.height = value;
        }
        if let Some(value) = self.margin_top {
            row.margins.top = value;
        }
        if let Some(value) = self.margin_right {
            row.margins.right = value;
        }
        if let Some(value) = self.margin_bottom {
            row.margins.bottom = value;
        }
        if let Some(value) = self.margin_left {
            row.margins.left = value;
        }
        if let Some(value) = self.columns_count {
            row.columns.count = value;
        }
        if let Some(value) = self.columns_gutter {
            row.columns.gutter = value;
        }
        if let Some(parent) = &self.parent_page_id {
            row.parent_page_id = parent.clone();
        }
        if let Some(child) = &self.background_drawing { row.background_drawing = child.clone(); }
        if let Some(delta) = &self.guides {
            row.guides = apply_positional(&row.guides, delta).map_err(|error| error.under(["guides"]))?;
        }
        row.overrides = self.overrides.commit_onto(&row.overrides, capability).map_err(|error| error.under(["overrides"]))?;
        row.frames = self.frames.commit_onto(&row.frames, capability).map_err(|error| error.under(["frames"]))?;
        row.layers = self.layers.commit_onto(&row.layers, capability).map_err(|error| error.under(["layers"]))?;
        semio_s_artifact_layout_layout_model::derive_page_layers(row);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.name = later.name.or_else(|| self.name.take());
        self.width = later.width.or(self.width);
        self.height = later.height.or(self.height);
        self.margin_top = later.margin_top.or(self.margin_top);
        self.margin_right = later.margin_right.or(self.margin_right);
        self.margin_bottom = later.margin_bottom.or(self.margin_bottom);
        self.margin_left = later.margin_left.or(self.margin_left);
        self.columns_count = later.columns_count.or(self.columns_count);
        self.columns_gutter = later.columns_gutter.or(self.columns_gutter);
        self.spread_id = later.spread_id.or_else(|| self.spread_id.take());
        self.parent_page_id = later.parent_page_id.or_else(|| self.parent_page_id.take());
        self.background_drawing = later.background_drawing.or_else(|| self.background_drawing.take());
        self.guides = match (self.guides.take(), later.guides) {
            (Some(first), Some(second)) => Some(absorb_positional(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        self.overrides.absorb(later.overrides);
        self.frames.absorb(later.frames);
        self.layers.absorb(later.layers);
    }
    fn inverse(&self, row: &Page) -> Self {
        Self {
            name: self.name.as_ref().map(|_| row.name.clone()),
            width: self.width.map(|_| row.width),
            height: self.height.map(|_| row.height),
            margin_top: self.margin_top.map(|_| row.margins.top),
            margin_right: self.margin_right.map(|_| row.margins.right),
            margin_bottom: self.margin_bottom.map(|_| row.margins.bottom),
            margin_left: self.margin_left.map(|_| row.margins.left),
            columns_count: self.columns_count.map(|_| row.columns.count),
            columns_gutter: self.columns_gutter.map(|_| row.columns.gutter),
            spread_id: self.spread_id.as_ref().map(|_| row.spread_id.clone()),
            parent_page_id: self.parent_page_id.as_ref().map(|_| row.parent_page_id.clone()),
            background_drawing: self.background_drawing.as_ref().map(|_| row.background_drawing.clone()),
            guides: self.guides.as_ref().map(|delta| inverse_positional(delta, &row.guides)),
            overrides: self.overrides.inverse(&row.overrides),
            frames: self.frames.inverse(&row.frames),
            layers: self.layers.inverse(&row.layers),
        }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.margin_top.is_none()
            && self.margin_right.is_none()
            && self.margin_bottom.is_none()
            && self.margin_left.is_none()
            && self.columns_count.is_none()
            && self.columns_gutter.is_none()
            && self.spread_id.is_none()
            && self.parent_page_id.is_none()
            && self.background_drawing.is_none()
            && self.guides.as_ref().is_none_or(is_empty_positional)
            && self.overrides.is_empty()
            && self.frames.is_empty()
            && self.layers.is_empty()
    }
}

impl GridPatch {
    pub(super) fn applied(&self, base: &GridSettings) -> GridSettings {
        GridSettings {
            baseline_grid: self.baseline_grid.unwrap_or(base.baseline_grid),
            baseline_offset: self.baseline_offset.unwrap_or(base.baseline_offset),
            snap_to_baseline: self.snap_to_baseline.unwrap_or(base.snap_to_baseline),
        }
    }
    pub(super) fn inverse_against(&self, base: &GridSettings) -> Self {
        Self { baseline_grid: self.baseline_grid.map(|_| base.baseline_grid), baseline_offset: self.baseline_offset.map(|_| base.baseline_offset), snap_to_baseline: self.snap_to_baseline.map(|_| base.snap_to_baseline) }
    }
    pub(super) fn composed(&mut self, later: Self) {
        self.baseline_grid = later.baseline_grid.or(self.baseline_grid);
        self.baseline_offset = later.baseline_offset.or(self.baseline_offset);
        self.snap_to_baseline = later.snap_to_baseline.or(self.snap_to_baseline);
    }
}
//#endregion 🔖️RowPatches

impl PartialEq for ParagraphStylePatch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.font_family == other.font_family
            && self.font_size.map(f64::to_bits) == other.font_size.map(f64::to_bits)
            && self.font_weight == other.font_weight
            && self.leading.map(f64::to_bits) == other.leading.map(f64::to_bits)
            && self.tracking.map(f64::to_bits) == other.tracking.map(f64::to_bits)
            && self.alignment == other.alignment
    }
}

impl PartialEq for CharacterStylePatch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.font_family == other.font_family
            && self.font_size.map(|value| value.map(f64::to_bits)) == other.font_size.map(|value| value.map(f64::to_bits))
            && self.font_weight == other.font_weight
            && self.italic == other.italic
            && self.color.as_ref().map(|value| value.as_ref().map(|color| color.map(f32::to_bits))) == other.color.as_ref().map(|value| value.as_ref().map(|color| color.map(f32::to_bits)))
            && self.tracking.map(|value| value.map(f64::to_bits)) == other.tracking.map(|value| value.map(f64::to_bits))
    }
}

impl PartialEq for ParentPagePatch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.width.map(f64::to_bits) == other.width.map(f64::to_bits)
            && self.height.map(f64::to_bits) == other.height.map(f64::to_bits)
    }
}

impl PartialEq for PagePatch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.spread_id == other.spread_id
            && self.width.map(f64::to_bits) == other.width.map(f64::to_bits)
            && self.height.map(f64::to_bits) == other.height.map(f64::to_bits)
            && self.margin_top.map(f64::to_bits) == other.margin_top.map(f64::to_bits)
            && self.margin_right.map(f64::to_bits) == other.margin_right.map(f64::to_bits)
            && self.margin_bottom.map(f64::to_bits) == other.margin_bottom.map(f64::to_bits)
            && self.margin_left.map(f64::to_bits) == other.margin_left.map(f64::to_bits)
            && self.columns_count == other.columns_count
            && self.columns_gutter.map(f64::to_bits) == other.columns_gutter.map(f64::to_bits)
            && self.parent_page_id == other.parent_page_id
            && self.background_drawing == other.background_drawing
            && self.guides == other.guides
            && self.overrides == other.overrides
            && self.frames == other.frames
            && self.layers == other.layers
    }
}
