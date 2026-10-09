//! 📏️ The annotation tools: the dimension (click the anchors, then the place of the dimension line), the tag (click an element), the text note (click a place) and the leader (click what to point at, then
//! click where the text goes). An anchor is the strongest thing under the pointer within a pick reach, an end of a wall, a column centre, an opening centre, a grid line or a wall face, else the snapped
//! point, so a dimension follows what it measures. Hold Shift to chain more anchors onto a dimension, and while placing the line to force it horizontal or vertical. Every click that completes an
//! annotation writes one history row of `create-*` mutations; the first annotation of a model also creates the standard annotation style in the same row. The preview shows the printed value as it
//! will read.

use super::plane::{angle, dist, point2, same, P};
use super::session::{Mark, Pointer, Preview, Step, Style, Tool, ToolContext, ToolEvent, PICK_PIXELS, REJECTED, STOREY_MISSING};
use super::snap::SnapHit;
use crate::editor::bim::entities::notations::standard_style;
use crate::standards::v1::subsets::any::schema::inferences::annotation_layout::anchors::{reference_point, resolve, Reference};
use crate::standards::v1::subsets::any::schema::inferences::annotation_layout::{dimensions, Inputs};
use crate::{AnchorEnd, AnnotationAnchor, Dimension, Leader, ModelMutation, ModelSnapshot, Point2, Tag, TagCategory, TextNote, WallSide};
use std::f64::consts::{FRAC_PI_2, PI};

/// 📏️ The grid every offset snaps to, in metres.
const OFFSET_STEP: f64 = 0.05;
/// 📐️ How close a measuring direction snaps to horizontal or vertical, in radians.
const AXIS_SNAP: f64 = 0.035;

/// 📏️ Which annotation tool this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Dimension,
    Tag,
    Note,
    Leader,
}

/// 👆️ What the pointer means: the anchor under it and the point to mark.
#[derive(Clone, Debug, PartialEq)]
pub struct Picked {
    pub anchor: AnnotationAnchor,
    pub at: P,
    pub snap: Option<SnapHit>,
}

fn segment_distance(a: P, b: P, p: P) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length_squared = dx * dx + dy * dy;
    let t = if length_squared == 0.0 { 0.0 } else { (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / length_squared).clamp(0.0, 1.0) };
    dist(p, [a[0] + t * dx, a[1] + t * dy])
}

fn p2(p: Point2) -> P {
    [p.x, p.y]
}

/// 👆️ The anchor under a pointer on the plan of the storey: the strongest of the wall ends, column centres, opening centres, grid lines and wall faces within the pick reach (the nearest among equals),
/// else the snapped point.
pub fn pick_anchor(ctx: &ToolContext<'_>, pointer: &Pointer) -> Picked {
    let reach = pointer.tolerance * PICK_PIXELS;
    let at = pointer.at;
    let mut best: Option<(u8, f64, AnnotationAnchor, P)> = None;
    let mut offer = |rank: u8, distance: f64, anchor: AnnotationAnchor, marked: P| {
        if distance <= reach && best.as_ref().is_none_or(|(held, nearest, ..)| rank < *held || (rank == *held && distance < *nearest)) {
            best = Some((rank, distance, anchor, marked));
        }
    };
    if let Some(storey) = ctx.storey() {
        let snapshot = ctx.snapshot;
        for (id, wall) in snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey) {
            let (start, end) = match &wall.axis {
                crate::Axis::Line { start, end } | crate::Axis::Arc { start, end, .. } => (p2(*start), p2(*end)),
            };
            offer(0, dist(at, start), AnnotationAnchor::WallEnd { wall: id.clone(), end: AnchorEnd::Start }, start);
            offer(0, dist(at, end), AnnotationAnchor::WallEnd { wall: id.clone(), end: AnchorEnd::End }, end);
        }
        for (id, column) in snapshot.columns.iter().filter(|(_, column)| column.storey == storey) {
            offer(1, dist(at, p2(column.position)), AnnotationAnchor::ColumnCentre { column: id.clone() }, p2(column.position));
        }
        for (id, frame) in ctx.inference.opening_frames.iter().filter(|(id, frame)| frame.host_length > 0.0 && crate::editor::bim::entities::storey_of(snapshot, id).as_deref() == Some(storey)) {
            offer(2, dist(at, p2(frame.point)), AnnotationAnchor::OpeningCentre { opening: id.clone() }, p2(frame.point));
        }
        let building = snapshot.storeys.get(storey).map(|row| row.building.as_str());
        for (id, grid) in snapshot.grids.iter().filter(|(_, grid)| Some(grid.building.as_str()) == building) {
            offer(3, segment_distance(p2(grid.start), p2(grid.end), at), AnnotationAnchor::Grid { grid: id.clone() }, at);
        }
        for (id, wall) in snapshot.walls.iter().filter(|(_, wall)| wall.storey == storey) {
            let Some(layout) = ctx.inference.wall_layout.get(id).filter(|layout| !layout.footprint.is_empty()) else { continue };
            for (side, face) in [(WallSide::Left, layout.left_face), (WallSide::Right, layout.right_face)] {
                if face.bulge == 0.0 && !matches!(wall.axis, crate::Axis::Arc { .. }) {
                    offer(4, segment_distance(p2(face.start), p2(face.end), at), AnnotationAnchor::WallFace { wall: id.clone(), side }, at);
                }
            }
        }
    }
    match best {
        Some((_, _, anchor, marked)) => Picked { anchor, at: marked, snap: None },
        None => {
            let hit = ctx.snapped(pointer, None, &[], &[]);
            Picked { anchor: AnnotationAnchor::Point { point: point2(hit.point) }, at: hit.point, snap: Some(hit) }
        }
    }
}

fn inputs<'a>(ctx: &'a ToolContext<'_>) -> Inputs<'a> {
    Inputs { layouts: ctx.inference.wall_layout.iter().map(|(id, layout)| (id.as_str(), layout)).collect() }
}

fn reference(ctx: &ToolContext<'_>, anchor: &AnnotationAnchor) -> Option<Reference> {
    resolve(ctx.snapshot, &inputs(ctx), anchor).ok()
}

fn wrap(turn: f64) -> f64 {
    let mut turn = turn % (2.0 * PI);
    if turn > PI {
        turn -= 2.0 * PI;
    }
    if turn <= -PI {
        turn += 2.0 * PI;
    }
    turn
}

/// 📐️ The measuring direction of a dimension: across the first anchor when it is a line (the clear distance to the next), else along the first and the last anchor, snapped to horizontal or
/// vertical within two degrees, and forced to the nearer axis with `square`.
pub fn direction_of(ctx: &ToolContext<'_>, anchors: &[AnnotationAnchor], square: bool) -> f64 {
    let first = anchors.first().and_then(|anchor| reference(ctx, anchor));
    let turn = match first {
        Some(Reference::Segment(a, b)) => wrap((b.y - a.y).atan2(b.x - a.x) + FRAC_PI_2),
        _ => {
            let from = first.map(|reference| p2(reference.pick()));
            let to = anchors.last().and_then(|anchor| reference(ctx, anchor)).map(|reference| p2(reference.pick()));
            match (from, to) {
                (Some(from), Some(to)) if !same(from, to) => angle(from, to),
                _ => 0.0,
            }
        }
    };
    let nearest = (turn / FRAC_PI_2).round() * FRAC_PI_2;
    if square || (turn - nearest).abs() <= AXIS_SNAP { wrap(nearest) } else { turn }
}

/// 📏️ The offset of a dimension line that passes `at`: its distance from the pick point of the first anchor to the left of the measuring direction, on a 5 cm grid.
pub fn offset_for(ctx: &ToolContext<'_>, anchors: &[AnnotationAnchor], turn: f64, at: P) -> f64 {
    let Some(base) = anchors.first().and_then(|anchor| reference(ctx, anchor)).map(|reference| p2(reference.pick())) else { return 0.0 };
    let across = (at[0] - base[0]) * -turn.sin() + (at[1] - base[1]) * turn.cos();
    (across / OFFSET_STEP).round() * OFFSET_STEP + 0.0
}

/// 🎨️ The style an annotation uses: the selected library entry or the first of the model, else the standard style, created in the same row.
fn style_for(ctx: &mut ToolContext<'_>) -> (Option<(String, crate::AnnotationStyle)>, String) {
    match ctx.library_type(&ctx.snapshot.annotation_styles) {
        Some(id) => (None, id),
        None => {
            let id = ctx.mint("annotation-style");
            let name = ctx.name_of(|labels| labels.kind_annotation_style, 0);
            (Some((id.clone(), standard_style(&name))), id)
        }
    }
}

/// 🛡️ The step writing `created` after the style it may need: the model must accept the style, and the annotation on top of it.
fn write(ctx: &ToolContext<'_>, style: Option<(String, crate::AnnotationStyle)>, created: ModelMutation) -> Step {
    use protocol::Mutation;
    let mut base: ModelSnapshot = ctx.snapshot.clone();
    let mut mutations = Vec::new();
    if let Some((id, annotation_style)) = style {
        let make = ModelMutation::CreateAnnotationStyle(crate::mutations::create_annotation_style::CreateAnnotationStyle { id: id.clone(), annotation_style: annotation_style.clone() });
        if !ctx.accepts(&make) {
            return Step::refuse(REJECTED);
        }
        base.annotation_styles.insert(id, annotation_style);
        mutations.push(make);
    }
    let (_diff, messages) = created.diff(&base).into_parts();
    if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Step::refuse(REJECTED);
    }
    mutations.push(created);
    Step::mutate(mutations)
}

/// 📏️ The state of one annotation tool: the anchors picked so far, whether a dimension is waiting for the place of its line, and the last pointer.
pub struct Annotate {
    kind: Kind,
    picks: Vec<AnnotationAnchor>,
    placing: bool,
    hover: Option<Picked>,
    cursor: Option<(P, bool)>,
}

impl Annotate {
    pub fn new(kind: Kind) -> Self {
        Self { kind, picks: Vec::new(), placing: false, hover: None, cursor: None }
    }

    fn reset(&mut self) {
        let hover = self.hover.take();
        *self = Self { hover, ..Self::new(self.kind) };
    }

    fn dimension(&mut self, ctx: &mut ToolContext<'_>, at: P, square: bool) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let turn = direction_of(ctx, &self.picks, square);
        let offset = offset_for(ctx, &self.picks, turn, at);
        let (style, style_id) = style_for(ctx);
        let count = ctx.snapshot.dimensions.values().filter(|row| row.storey == storey).count();
        let name = ctx.name_of(|labels| labels.kind_dimension, count);
        let id = ctx.mint("dimension");
        let dimension = Dimension { storey, anchors: self.picks.clone(), angle: turn, offset, style: style_id, lock: None, name };
        let step = write(ctx, style, ModelMutation::CreateDimension(crate::mutations::create_dimension::CreateDimension { id, dimension }));
        if step.refused.is_none() {
            self.reset();
        }
        step
    }

    fn tag(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(plan) = ctx.inference.plan_linework.get(&storey) else { return Step::default() };
        let reach = pointer.tolerance * PICK_PIXELS;
        let Some((element, origin)) = crate::editor::bim::modes::edit::windows::plan::pick(plan, (pointer.at[0], pointer.at[1]), reach).and_then(|element| reference_point(ctx.snapshot, &element).map(|origin| (element, origin))) else { return Step::default() };
        let category = if ctx.snapshot.spaces.contains_key(&element) {
            TagCategory::Number
        } else if ctx.snapshot.openings.contains_key(&element) {
            TagCategory::Type
        } else {
            TagCategory::Name
        };
        let offset = Point2 { x: ((pointer.at[0] - origin.x) / OFFSET_STEP).round() * OFFSET_STEP + 0.0, y: ((pointer.at[1] - origin.y) / OFFSET_STEP).round() * OFFSET_STEP + 0.0 };
        let (style, style_id) = style_for(ctx);
        let id = ctx.mint("tag");
        let tag = Tag { storey, element, category, offset, style: style_id };
        write(ctx, style, ModelMutation::CreateTag(crate::mutations::create_tag::CreateTag { id, tag }))
    }

    fn note(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let at = ctx.snapped(pointer, None, &[], &[]).point;
        let count = ctx.snapshot.text_notes.values().filter(|row| row.storey == storey).count();
        let text = ctx.name_of(|labels| labels.kind_text_note, count);
        let (style, style_id) = style_for(ctx);
        let id = ctx.mint("text-note");
        let note = TextNote { storey, position: point2(at), text, rotation: 0.0, style: style_id };
        write(ctx, style, ModelMutation::CreateTextNote(crate::mutations::create_text_note::CreateTextNote { id, text_note: note }))
    }

    fn leader(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let Some(storey) = ctx.storey().map(str::to_string) else { return Step::refuse(STOREY_MISSING) };
        let Some(anchor) = self.picks.first().cloned() else {
            let picked = pick_anchor(ctx, pointer);
            self.picks.push(picked.anchor);
            return Step::default();
        };
        let Some(tip) = reference(ctx, &anchor).map(|reference| p2(reference.pick())) else {
            self.reset();
            return Step::refuse(REJECTED);
        };
        let at = ctx.snapped(pointer, Some(tip), &[], &[]).point;
        let offset = Point2 { x: ((at[0] - tip[0]) / OFFSET_STEP).round() * OFFSET_STEP + 0.0, y: ((at[1] - tip[1]) / OFFSET_STEP).round() * OFFSET_STEP + 0.0 };
        let count = ctx.snapshot.leaders.values().filter(|row| row.storey == storey).count();
        let text = ctx.name_of(|labels| labels.kind_leader, count);
        let (style, style_id) = style_for(ctx);
        let id = ctx.mint("leader");
        let leader = Leader { storey, anchor, offset, text, style: style_id };
        let step = write(ctx, style, ModelMutation::CreateLeader(crate::mutations::create_leader::CreateLeader { id, leader }));
        if step.refused.is_none() {
            self.reset();
        }
        step
    }

    fn down(&mut self, ctx: &mut ToolContext<'_>, pointer: &Pointer) -> Step {
        let picked = pick_anchor(ctx, pointer);
        self.cursor = Some((picked.at, pointer.modifiers.shift));
        self.hover = Some(picked.clone());
        match self.kind {
            Kind::Dimension if self.placing => self.dimension(ctx, pointer.at, pointer.modifiers.shift),
            Kind::Dimension => {
                self.picks.push(picked.anchor);
                self.placing = self.picks.len() >= 2 && !pointer.modifiers.shift;
                Step::default()
            }
            Kind::Tag => self.tag(ctx, pointer),
            Kind::Note => self.note(ctx, pointer),
            Kind::Leader => self.leader(ctx, pointer),
        }
    }

    fn draft(&self, ctx: &ToolContext<'_>, hover: &Picked) -> Option<(f64, f64, Dimension)> {
        let storey = ctx.storey()?.to_string();
        let mut anchors = self.picks.clone();
        if !self.placing {
            anchors.push(hover.anchor.clone());
        }
        let turn = direction_of(ctx, &anchors, self.cursor.is_some_and(|(_, square)| square));
        let offset = if self.placing { offset_for(ctx, &anchors, turn, hover.at) } else { 0.0 };
        let style = ctx.library_type(&ctx.snapshot.annotation_styles).unwrap_or_default();
        Some((turn, offset, Dimension { storey, anchors, angle: turn, offset, style, lock: None, name: String::new() }))
    }
}

impl Tool for Annotate {
    fn anchor(&self) -> Option<P> {
        self.cursor.map(|(at, _)| at).filter(|_| !self.picks.is_empty())
    }

    fn event(&mut self, ctx: &mut ToolContext<'_>, event: &ToolEvent) -> Step {
        match event {
            ToolEvent::Move(pointer) => {
                let picked = pick_anchor(ctx, pointer);
                self.cursor = Some((if self.placing { pointer.at } else { picked.at }, pointer.modifiers.shift));
                self.hover = Some(Picked { at: if self.placing { pointer.at } else { picked.at }, ..picked });
                Step::default()
            }
            ToolEvent::Down(pointer) => self.down(ctx, pointer),
            ToolEvent::Up(_) | ToolEvent::Double(_) => Step::default(),
            ToolEvent::Finish => {
                if self.kind == Kind::Dimension && !self.placing && self.picks.len() >= 2 {
                    self.placing = true;
                }
                Step::default()
            }
            ToolEvent::Escape | ToolEvent::Lost => {
                self.reset();
                Step::default()
            }
        }
    }

    fn preview(&self, ctx: &ToolContext<'_>) -> Preview {
        let mut marks = Vec::new();
        for anchor in &self.picks {
            if let Some(reference) = reference(ctx, anchor) {
                marks.push(Mark::dot(p2(reference.pick()), Style::Handle));
            }
        }
        let Some(hover) = &self.hover else { return Preview::of(marks) };
        match self.kind {
            Kind::Dimension if !self.picks.is_empty() => {
                if let Some((_, _, draft)) = self.draft(ctx, hover) {
                    let layout = dimensions::layout_of(ctx.snapshot, &inputs(ctx), &draft);
                    if layout.complete && (self.placing || draft.anchors.len() >= 2) {
                        for segment in &layout.segments {
                            marks.push(Mark::path(&[p2(segment.from), p2(segment.to)], false, Style::Ghost));
                            marks.push(Mark::label(p2(segment.text_at.at), segment.text.clone()));
                        }
                        marks.extend(layout.anchors.iter().filter_map(|anchor| anchor.extension).map(|line| Mark::path(&[p2(line.start), p2(line.end)], false, Style::Guide)));
                    }
                }
            }
            Kind::Leader if !self.picks.is_empty() => {
                if let Some(tip) = reference(ctx, &self.picks[0]).map(|reference| p2(reference.pick())) {
                    marks.push(Mark::path(&[tip, hover.at], false, Style::Ghost));
                }
            }
            _ => {}
        }
        marks.extend(hover.snap.as_ref().map(Mark::snap));
        if hover.snap.is_none() {
            marks.push(Mark::dot(hover.at, Style::Snap));
        }
        Preview::of(marks)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
