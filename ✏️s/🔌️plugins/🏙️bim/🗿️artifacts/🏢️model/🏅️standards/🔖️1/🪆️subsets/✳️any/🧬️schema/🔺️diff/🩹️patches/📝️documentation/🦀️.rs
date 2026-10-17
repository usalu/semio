//! 📝️ Sparse patches of the documentation records: dimensions, tags, text notes, leaders, annotation styles, sheets, viewports, sheet revisions, views and schedules.

use super::*;

/// 🩹 Sparse patch of [`Dimension`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct DimensionPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<AnnotationAnchor>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub lock: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<Dimension> for DimensionPatch {
    fn write(&self, base: &Dimension) -> Dimension {
        Dimension { storey: take(&self.storey, &base.storey), anchors: take(&self.anchors, &base.anchors), angle: take(&self.angle, &base.angle), offset: take(&self.offset, &base.offset), style: take(&self.style, &base.style), lock: take_assigned(&self.lock, &base.lock), name: take(&self.name, &base.name) }
    }
    fn restoring(&self, base: &Dimension) -> Self {
        Self { storey: restore(&self.storey, &base.storey), anchors: restore(&self.anchors, &base.anchors), angle: restore(&self.angle, &base.angle), offset: restore(&self.offset, &base.offset), style: restore(&self.style, &base.style), lock: restore_assigned(&self.lock, &base.lock), name: restore(&self.name, &base.name) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.anchors, later.anchors);
        merge_slot(&mut self.angle, later.angle);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.style, later.style);
        merge_slot(&mut self.lock, later.lock);
        merge_slot(&mut self.name, later.name);
    }
    fn minimal(&self, base: &Dimension) -> Self {
        Self { storey: changed(&self.storey, &base.storey), anchors: changed(&self.anchors, &base.anchors), angle: changed(&self.angle, &base.angle), offset: changed(&self.offset, &base.offset), style: changed(&self.style, &base.style), lock: changed_assigned(&self.lock, &base.lock), name: changed(&self.name, &base.name) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.anchors.is_some() {
            out.push("anchors".to_string());
        }
        if self.angle.is_some() {
            out.push("angle".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.style.is_some() {
            out.push("style".to_string());
        }
        if self.lock.is_some() {
            out.push("lock".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Tag`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct TagPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<TagCategory>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl Patch<Tag> for TagPatch {
    fn write(&self, base: &Tag) -> Tag {
        Tag { storey: take(&self.storey, &base.storey), element: take(&self.element, &base.element), category: take(&self.category, &base.category), offset: take(&self.offset, &base.offset), style: take(&self.style, &base.style) }
    }
    fn restoring(&self, base: &Tag) -> Self {
        Self { storey: restore(&self.storey, &base.storey), element: restore(&self.element, &base.element), category: restore(&self.category, &base.category), offset: restore(&self.offset, &base.offset), style: restore(&self.style, &base.style) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.element, later.element);
        merge_slot(&mut self.category, later.category);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.style, later.style);
    }
    fn minimal(&self, base: &Tag) -> Self {
        Self { storey: changed(&self.storey, &base.storey), element: changed(&self.element, &base.element), category: changed(&self.category, &base.category), offset: changed(&self.offset, &base.offset), style: changed(&self.style, &base.style) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.element.is_some() {
            out.push("element".to_string());
        }
        if self.category.is_some() {
            out.push("category".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.style.is_some() {
            out.push("style".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`TextNote`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct TextNotePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl Patch<TextNote> for TextNotePatch {
    fn write(&self, base: &TextNote) -> TextNote {
        TextNote { storey: take(&self.storey, &base.storey), position: take(&self.position, &base.position), text: take(&self.text, &base.text), rotation: take(&self.rotation, &base.rotation), style: take(&self.style, &base.style) }
    }
    fn restoring(&self, base: &TextNote) -> Self {
        Self { storey: restore(&self.storey, &base.storey), position: restore(&self.position, &base.position), text: restore(&self.text, &base.text), rotation: restore(&self.rotation, &base.rotation), style: restore(&self.style, &base.style) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.position, later.position);
        merge_slot(&mut self.text, later.text);
        merge_slot(&mut self.rotation, later.rotation);
        merge_slot(&mut self.style, later.style);
    }
    fn minimal(&self, base: &TextNote) -> Self {
        Self { storey: changed(&self.storey, &base.storey), position: changed(&self.position, &base.position), text: changed(&self.text, &base.text), rotation: changed(&self.rotation, &base.rotation), style: changed(&self.style, &base.style) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.position.is_some() {
            out.push("position".to_string());
        }
        if self.text.is_some() {
            out.push("text".to_string());
        }
        if self.rotation.is_some() {
            out.push("rotation".to_string());
        }
        if self.style.is_some() {
            out.push("style".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Leader`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct LeaderPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AnnotationAnchor>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl Patch<Leader> for LeaderPatch {
    fn write(&self, base: &Leader) -> Leader {
        Leader { storey: take(&self.storey, &base.storey), anchor: take(&self.anchor, &base.anchor), offset: take(&self.offset, &base.offset), text: take(&self.text, &base.text), style: take(&self.style, &base.style) }
    }
    fn restoring(&self, base: &Leader) -> Self {
        Self { storey: restore(&self.storey, &base.storey), anchor: restore(&self.anchor, &base.anchor), offset: restore(&self.offset, &base.offset), text: restore(&self.text, &base.text), style: restore(&self.style, &base.style) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.anchor, later.anchor);
        merge_slot(&mut self.offset, later.offset);
        merge_slot(&mut self.text, later.text);
        merge_slot(&mut self.style, later.style);
    }
    fn minimal(&self, base: &Leader) -> Self {
        Self { storey: changed(&self.storey, &base.storey), anchor: changed(&self.anchor, &base.anchor), offset: changed(&self.offset, &base.offset), text: changed(&self.text, &base.text), style: changed(&self.style, &base.style) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.anchor.is_some() {
            out.push("anchor".to_string());
        }
        if self.offset.is_some() {
            out.push("offset".to_string());
        }
        if self.text.is_some() {
            out.push("text".to_string());
        }
        if self.style.is_some() {
            out.push("style".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`AnnotationStyle`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct AnnotationStylePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub text_height: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub terminator: Option<Terminator>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub unit: Option<DimensionUnit>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub mark_size: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub overshoot: Option<f64>,
}

impl Patch<AnnotationStyle> for AnnotationStylePatch {
    fn write(&self, base: &AnnotationStyle) -> AnnotationStyle {
        AnnotationStyle { name: take(&self.name, &base.name), text_height: take(&self.text_height, &base.text_height), terminator: take(&self.terminator, &base.terminator), unit: take(&self.unit, &base.unit), precision: take(&self.precision, &base.precision), mark_size: take(&self.mark_size, &base.mark_size), gap: take(&self.gap, &base.gap), overshoot: take(&self.overshoot, &base.overshoot) }
    }
    fn restoring(&self, base: &AnnotationStyle) -> Self {
        Self { name: restore(&self.name, &base.name), text_height: restore(&self.text_height, &base.text_height), terminator: restore(&self.terminator, &base.terminator), unit: restore(&self.unit, &base.unit), precision: restore(&self.precision, &base.precision), mark_size: restore(&self.mark_size, &base.mark_size), gap: restore(&self.gap, &base.gap), overshoot: restore(&self.overshoot, &base.overshoot) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.text_height, later.text_height);
        merge_slot(&mut self.terminator, later.terminator);
        merge_slot(&mut self.unit, later.unit);
        merge_slot(&mut self.precision, later.precision);
        merge_slot(&mut self.mark_size, later.mark_size);
        merge_slot(&mut self.gap, later.gap);
        merge_slot(&mut self.overshoot, later.overshoot);
    }
    fn minimal(&self, base: &AnnotationStyle) -> Self {
        Self { name: changed(&self.name, &base.name), text_height: changed(&self.text_height, &base.text_height), terminator: changed(&self.terminator, &base.terminator), unit: changed(&self.unit, &base.unit), precision: changed(&self.precision, &base.precision), mark_size: changed(&self.mark_size, &base.mark_size), gap: changed(&self.gap, &base.gap), overshoot: changed(&self.overshoot, &base.overshoot) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.text_height.is_some() {
            out.push("text_height".to_string());
        }
        if self.terminator.is_some() {
            out.push("terminator".to_string());
        }
        if self.unit.is_some() {
            out.push("unit".to_string());
        }
        if self.precision.is_some() {
            out.push("precision".to_string());
        }
        if self.mark_size.is_some() {
            out.push("mark_size".to_string());
        }
        if self.gap.is_some() {
            out.push("gap".to_string());
        }
        if self.overshoot.is_some() {
            out.push("overshoot".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Sheet`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SheetPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub paper: Option<Paper>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drawn_by: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub checked_by: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub scale_label: Option<String>,
}

impl Patch<Sheet> for SheetPatch {
    fn write(&self, base: &Sheet) -> Sheet {
        Sheet { number: take(&self.number, &base.number), name: take(&self.name, &base.name), paper: take(&self.paper, &base.paper), orientation: take(&self.orientation, &base.orientation), project: take(&self.project, &base.project), drawn_by: take(&self.drawn_by, &base.drawn_by), checked_by: take(&self.checked_by, &base.checked_by), date: take(&self.date, &base.date), revision: take(&self.revision, &base.revision), scale_label: take(&self.scale_label, &base.scale_label) }
    }
    fn restoring(&self, base: &Sheet) -> Self {
        Self { number: restore(&self.number, &base.number), name: restore(&self.name, &base.name), paper: restore(&self.paper, &base.paper), orientation: restore(&self.orientation, &base.orientation), project: restore(&self.project, &base.project), drawn_by: restore(&self.drawn_by, &base.drawn_by), checked_by: restore(&self.checked_by, &base.checked_by), date: restore(&self.date, &base.date), revision: restore(&self.revision, &base.revision), scale_label: restore(&self.scale_label, &base.scale_label) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.number, later.number);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.paper, later.paper);
        merge_slot(&mut self.orientation, later.orientation);
        merge_slot(&mut self.project, later.project);
        merge_slot(&mut self.drawn_by, later.drawn_by);
        merge_slot(&mut self.checked_by, later.checked_by);
        merge_slot(&mut self.date, later.date);
        merge_slot(&mut self.revision, later.revision);
        merge_slot(&mut self.scale_label, later.scale_label);
    }
    fn minimal(&self, base: &Sheet) -> Self {
        Self { number: changed(&self.number, &base.number), name: changed(&self.name, &base.name), paper: changed(&self.paper, &base.paper), orientation: changed(&self.orientation, &base.orientation), project: changed(&self.project, &base.project), drawn_by: changed(&self.drawn_by, &base.drawn_by), checked_by: changed(&self.checked_by, &base.checked_by), date: changed(&self.date, &base.date), revision: changed(&self.revision, &base.revision), scale_label: changed(&self.scale_label, &base.scale_label) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.number.is_some() {
            out.push("number".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.paper.is_some() {
            out.push("paper".to_string());
        }
        if self.orientation.is_some() {
            out.push("orientation".to_string());
        }
        if self.project.is_some() {
            out.push("project".to_string());
        }
        if self.drawn_by.is_some() {
            out.push("drawn_by".to_string());
        }
        if self.checked_by.is_some() {
            out.push("checked_by".to_string());
        }
        if self.date.is_some() {
            out.push("date".to_string());
        }
        if self.revision.is_some() {
            out.push("revision".to_string());
        }
        if self.scale_label.is_some() {
            out.push("scale_label".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Viewport`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ViewportPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Assigned<Option<ViewCrop>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<Assigned<Option<String>>>,
}

impl Patch<Viewport> for ViewportPatch {
    fn write(&self, base: &Viewport) -> Viewport {
        Viewport { sheet: take(&self.sheet, &base.sheet), view: take(&self.view, &base.view), position: take(&self.position, &base.position), scale: take(&self.scale, &base.scale), crop: take_assigned(&self.crop, &base.crop), label: take_assigned(&self.label, &base.label) }
    }
    fn restoring(&self, base: &Viewport) -> Self {
        Self { sheet: restore(&self.sheet, &base.sheet), view: restore(&self.view, &base.view), position: restore(&self.position, &base.position), scale: restore(&self.scale, &base.scale), crop: restore_assigned(&self.crop, &base.crop), label: restore_assigned(&self.label, &base.label) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.sheet, later.sheet);
        merge_slot(&mut self.view, later.view);
        merge_slot(&mut self.position, later.position);
        merge_slot(&mut self.scale, later.scale);
        merge_slot(&mut self.crop, later.crop);
        merge_slot(&mut self.label, later.label);
    }
    fn minimal(&self, base: &Viewport) -> Self {
        Self { sheet: changed(&self.sheet, &base.sheet), view: changed(&self.view, &base.view), position: changed(&self.position, &base.position), scale: changed(&self.scale, &base.scale), crop: changed_assigned(&self.crop, &base.crop), label: changed_assigned(&self.label, &base.label) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.sheet.is_some() {
            out.push("sheet".to_string());
        }
        if self.view.is_some() {
            out.push("view".to_string());
        }
        if self.position.is_some() {
            out.push("position".to_string());
        }
        if self.scale.is_some() {
            out.push("scale".to_string());
        }
        if self.crop.is_some() {
            out.push("crop".to_string());
        }
        if self.label.is_some() {
            out.push("label".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`SheetRevision`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SheetRevisionPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

impl Patch<SheetRevision> for SheetRevisionPatch {
    fn write(&self, base: &SheetRevision) -> SheetRevision {
        SheetRevision { sheet: take(&self.sheet, &base.sheet), number: take(&self.number, &base.number), date: take(&self.date, &base.date), description: take(&self.description, &base.description), author: take(&self.author, &base.author) }
    }
    fn restoring(&self, base: &SheetRevision) -> Self {
        Self { sheet: restore(&self.sheet, &base.sheet), number: restore(&self.number, &base.number), date: restore(&self.date, &base.date), description: restore(&self.description, &base.description), author: restore(&self.author, &base.author) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.sheet, later.sheet);
        merge_slot(&mut self.number, later.number);
        merge_slot(&mut self.date, later.date);
        merge_slot(&mut self.description, later.description);
        merge_slot(&mut self.author, later.author);
    }
    fn minimal(&self, base: &SheetRevision) -> Self {
        Self { sheet: changed(&self.sheet, &base.sheet), number: changed(&self.number, &base.number), date: changed(&self.date, &base.date), description: changed(&self.description, &base.description), author: changed(&self.author, &base.author) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.sheet.is_some() {
            out.push("sheet".to_string());
        }
        if self.number.is_some() {
            out.push("number".to_string());
        }
        if self.date.is_some() {
            out.push("date".to_string());
        }
        if self.description.is_some() {
            out.push("description".to_string());
        }
        if self.author.is_some() {
            out.push("author".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`View`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ViewPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ViewKind>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<Assigned<Option<String>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub plane: Option<Assigned<Option<ViewPlane>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<Assigned<Option<ViewCamera>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub cut_height: Option<Assigned<Option<f64>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Assigned<Option<ViewCrop>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<Vec<ViewCategory>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Assigned<Option<Phase>>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u32>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub detail: Option<DetailLevel>,
}

impl Patch<View> for ViewPatch {
    fn write(&self, base: &View) -> View {
        View { building: take(&self.building, &base.building), name: take(&self.name, &base.name), kind: take(&self.kind, &base.kind), storey: take_assigned(&self.storey, &base.storey), plane: take_assigned(&self.plane, &base.plane), camera: take_assigned(&self.camera, &base.camera), cut_height: take_assigned(&self.cut_height, &base.cut_height), depth: take(&self.depth, &base.depth), crop: take_assigned(&self.crop, &base.crop), hidden: take(&self.hidden, &base.hidden), phase: take_assigned(&self.phase, &base.phase), scale: take(&self.scale, &base.scale), detail: take(&self.detail, &base.detail) }
    }
    fn restoring(&self, base: &View) -> Self {
        Self { building: restore(&self.building, &base.building), name: restore(&self.name, &base.name), kind: restore(&self.kind, &base.kind), storey: restore_assigned(&self.storey, &base.storey), plane: restore_assigned(&self.plane, &base.plane), camera: restore_assigned(&self.camera, &base.camera), cut_height: restore_assigned(&self.cut_height, &base.cut_height), depth: restore(&self.depth, &base.depth), crop: restore_assigned(&self.crop, &base.crop), hidden: restore(&self.hidden, &base.hidden), phase: restore_assigned(&self.phase, &base.phase), scale: restore(&self.scale, &base.scale), detail: restore(&self.detail, &base.detail) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.building, later.building);
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.kind, later.kind);
        merge_slot(&mut self.storey, later.storey);
        merge_slot(&mut self.plane, later.plane);
        merge_slot(&mut self.camera, later.camera);
        merge_slot(&mut self.cut_height, later.cut_height);
        merge_slot(&mut self.depth, later.depth);
        merge_slot(&mut self.crop, later.crop);
        merge_slot(&mut self.hidden, later.hidden);
        merge_slot(&mut self.phase, later.phase);
        merge_slot(&mut self.scale, later.scale);
        merge_slot(&mut self.detail, later.detail);
    }
    fn minimal(&self, base: &View) -> Self {
        Self { building: changed(&self.building, &base.building), name: changed(&self.name, &base.name), kind: changed(&self.kind, &base.kind), storey: changed_assigned(&self.storey, &base.storey), plane: changed_assigned(&self.plane, &base.plane), camera: changed_assigned(&self.camera, &base.camera), cut_height: changed_assigned(&self.cut_height, &base.cut_height), depth: changed(&self.depth, &base.depth), crop: changed_assigned(&self.crop, &base.crop), hidden: changed(&self.hidden, &base.hidden), phase: changed_assigned(&self.phase, &base.phase), scale: changed(&self.scale, &base.scale), detail: changed(&self.detail, &base.detail) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.building.is_some() {
            out.push("building".to_string());
        }
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.kind.is_some() {
            out.push("kind".to_string());
        }
        if self.storey.is_some() {
            out.push("storey".to_string());
        }
        if self.plane.is_some() {
            out.push("plane".to_string());
        }
        if self.camera.is_some() {
            out.push("camera".to_string());
        }
        if self.cut_height.is_some() {
            out.push("cut_height".to_string());
        }
        if self.depth.is_some() {
            out.push("depth".to_string());
        }
        if self.crop.is_some() {
            out.push("crop".to_string());
        }
        if self.hidden.is_some() {
            out.push("hidden".to_string());
        }
        if self.phase.is_some() {
            out.push("phase".to_string());
        }
        if self.scale.is_some() {
            out.push("scale".to_string());
        }
        if self.detail.is_some() {
            out.push("detail".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// 🩹 Sparse patch of [`Schedule`]: an absent field is untouched.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct SchedulePatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub category: Option<ScheduleCategory>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<ScheduleColumn>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ScheduleSort>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ScheduleFilter>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<ScheduleGroup>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub itemize: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub storeys: Option<Vec<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<Phase>>,
}

impl Patch<Schedule> for SchedulePatch {
    fn write(&self, base: &Schedule) -> Schedule {
        Schedule { name: take(&self.name, &base.name), category: take(&self.category, &base.category), columns: take(&self.columns, &base.columns), sort: take(&self.sort, &base.sort), filter: take(&self.filter, &base.filter), group: take(&self.group, &base.group), itemize: take(&self.itemize, &base.itemize), storeys: take(&self.storeys, &base.storeys), phases: take(&self.phases, &base.phases) }
    }
    fn restoring(&self, base: &Schedule) -> Self {
        Self { name: restore(&self.name, &base.name), category: restore(&self.category, &base.category), columns: restore(&self.columns, &base.columns), sort: restore(&self.sort, &base.sort), filter: restore(&self.filter, &base.filter), group: restore(&self.group, &base.group), itemize: restore(&self.itemize, &base.itemize), storeys: restore(&self.storeys, &base.storeys), phases: restore(&self.phases, &base.phases) }
    }
    fn merge(&mut self, later: Self) {
        merge_slot(&mut self.name, later.name);
        merge_slot(&mut self.category, later.category);
        merge_slot(&mut self.columns, later.columns);
        merge_slot(&mut self.sort, later.sort);
        merge_slot(&mut self.filter, later.filter);
        merge_slot(&mut self.group, later.group);
        merge_slot(&mut self.itemize, later.itemize);
        merge_slot(&mut self.storeys, later.storeys);
        merge_slot(&mut self.phases, later.phases);
    }
    fn minimal(&self, base: &Schedule) -> Self {
        Self { name: changed(&self.name, &base.name), category: changed(&self.category, &base.category), columns: changed(&self.columns, &base.columns), sort: changed(&self.sort, &base.sort), filter: changed(&self.filter, &base.filter), group: changed(&self.group, &base.group), itemize: changed(&self.itemize, &base.itemize), storeys: changed(&self.storeys, &base.storeys), phases: changed(&self.phases, &base.phases) }
    }
    fn touched(&self) -> Vec<String> {
        let mut out = Vec::new();
        if self.name.is_some() {
            out.push("name".to_string());
        }
        if self.category.is_some() {
            out.push("category".to_string());
        }
        if self.columns.is_some() {
            out.push("columns".to_string());
        }
        if self.sort.is_some() {
            out.push("sort".to_string());
        }
        if self.filter.is_some() {
            out.push("filter".to_string());
        }
        if self.group.is_some() {
            out.push("group".to_string());
        }
        if self.itemize.is_some() {
            out.push("itemize".to_string());
        }
        if self.storeys.is_some() {
            out.push("storeys".to_string());
        }
        if self.phases.is_some() {
            out.push("phases".to_string());
        }
        out
    }
    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
