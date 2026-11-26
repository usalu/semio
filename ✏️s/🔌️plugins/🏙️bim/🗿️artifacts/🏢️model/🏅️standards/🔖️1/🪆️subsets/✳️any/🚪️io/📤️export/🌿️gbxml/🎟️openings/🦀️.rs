//! 🪟️ Plans original openings in the retained host frame, including paired-room deduplication.

use super::{Frame, ModelInference, ModelSnapshot, OriginalSurfaces, SurfaceAddress, SurfaceKind, turned};
use super::original_surfaces::surface;
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::gbxml::geometry::OpeningProjection;

const POINTS: usize = 32;

/// 🪟️ An original opening or the synthesized glazing of an opaque curtain host.
#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub struct OriginalOpeningRow {
    pub address: SurfaceAddress,
    pub glazing: bool,
    pub offset: [f64; 2],
    pub width: f64,
    pub height: f64,
}

/// 🔢️ A consecutive range in the original opening output table.
#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub struct OpeningRange { pub first: usize, pub count: usize }

#[derive(Default)]
enum Stage {
    #[default]
    Start,
    Scan { side: usize, next: usize },
    Deduplicate { candidate: SurfaceAddress, next: usize },
    Geometry { candidate: SurfaceAddress, side: usize, geometry: OpeningProjection },
    Publish { next: Option<(usize, usize)>, row: Option<OriginalOpeningRow> },
    Range { range: Option<OpeningRange> },
}

#[derive(Default)]
struct ChildIndex {
    rows: InferenceValues<usize>,
    scanned: usize,
    nearest: Option<usize>,
    pending: Option<usize>,
}

impl ChildIndex {
    fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, sides: &OriginalSurfaces, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        let extent = sides.extent();
        if !self.rows.admit_source_step(cx)? || !self.rows.initialize_step(extent, cx)? { return Ok(false); }
        if self.scanned == extent { return Ok(true); }
        let ordinal = extent - self.scanned - 1;
        if self.pending.is_some() {
            if self.rows.publish_step(ordinal, &mut self.pending, cx)? { self.scanned += 1; }
        } else if cx.charge_work_unit() {
            let side = sides.source_at(ordinal).and_then(|at| surface(model, inferred, at))
                .ok_or_else(|| ValueError::invariant("gbXML child index source disappeared"))?;
            if !side.parent.is_empty() { self.nearest = Some(ordinal); }
            if self.nearest.is_some() { self.pending = self.nearest; }
            else { self.scanned += 1; }
        }
        Ok(false)
    }

    fn next(&self, sides: &OriginalSurfaces, from: SurfaceAddress) -> Option<SurfaceAddress> {
        let candidate = sides.source_at(*self.rows.get(sides.ordinal(from)?)?)?;
        (candidate.room == from.room).then_some(candidate)
    }
}

/// 📇️ Original host-to-opening ranges and the ordered scalar geometry of each opening.
#[derive(Default)]
pub struct OriginalOpenings {
    source: Option<(usize, usize, usize)>,
    ranges: InferenceValues<OpeningRange>,
    rows: InferenceValues<OriginalOpeningRow>,
    children: ChildIndex,
    current: usize,
    count: usize,
    first: usize,
    stage: Stage,
}

impl OriginalOpenings {
    /// 🛠️ Advances one original child scan, duplicate comparison, vertex slice or row.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, sides: &OriginalSurfaces, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML opening planning was cancelled").into()); }
        let source = (std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize, std::ptr::from_ref(sides) as usize);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(source); }
            return Ok(false);
        }
        if self.source != Some(source) { return Err(ValueError::invariant("gbXML opening source changed").into()); }
        let extent = sides.extent();
        if !self.children.advance(model, inferred, sides, cx)? { return Ok(false); }
        if !self.ranges.admit_source_step(cx)? || !self.ranges.initialize_step(extent, cx)? { return Ok(false); }
        if !self.rows.admit_source_step(cx)? || !self.rows.initialize_step(extent, cx)? { return Ok(false); }
        if self.current == extent { return Ok(true); }
        match &mut self.stage {
            Stage::Publish { next, row } => {
                if self.count == extent { return Err(ValueError::invariant("gbXML opening extent exceeded its envelope").into()); }
                if self.rows.publish_step(self.count, row, cx)? {
                    self.count += 1;
                    self.stage = next.map_or_else(|| Stage::Range { range: Some(OpeningRange { first: self.first, count: self.count - self.first }) },
                        |(side, next)| Stage::Scan { side, next });
                }
                return Ok(false);
            }
            Stage::Range { range } => {
                if self.ranges.publish_step(self.current, range, cx)? {
                    self.current += 1;
                    self.stage = Stage::Start;
                }
                return Ok(false);
            }
            _ => (),
        }
        if !cx.charge_work_unit() { return Ok(false); }
        let Some(host) = sides.get(self.current) else {
            self.current += 1;
            return Ok(false);
        };
        let first = surface(model, inferred, host.surface).ok_or_else(|| ValueError::invariant("gbXML opening host disappeared"))?;
        match &mut self.stage {
            Stage::Start => {
                self.first = self.count;
                self.stage = Stage::Scan { side: 0, next: 0 };
            }
            Stage::Scan { side, next } => {
                let address = if *side == 0 { Some(host.surface) } else { host.other };
                let candidate = address.and_then(|parent| self.children.next(sides, SurfaceAddress { room: parent.room, side: *next }));
                if let Some((address, child)) = candidate.and_then(|address| surface(model, inferred, address).map(|child| (address, child))) {
                    let parent = if *side == 0 { first } else {
                        host.other.and_then(|other| surface(model, inferred, other)).ok_or_else(|| ValueError::invariant("gbXML opposite host disappeared"))?
                    };
                    if child.parent != parent.id || child.polygon.len() < 3 {
                        *next = address.side + 1;
                    } else if *side == 1 {
                        self.stage = Stage::Deduplicate { candidate: address, next: 0 };
                    } else {
                        let frame = Frame { normal: host.normal, right: host.right, up: host.up };
                        let geometry = OpeningProjection::new(frame, host.corner, turned(&child.polygon[0], host.bearing), child.polygon.len());
                        self.stage = Stage::Geometry { candidate: address, side: 0, geometry };
                    }
                } else if *side == 0 && host.other.is_some() {
                    self.stage = Stage::Scan { side: 1, next: 0 };
                } else if first.kind == SurfaceKind::CurtainWall && model.curtain_wall_types.contains_key(&first.construction) {
                    let row = OriginalOpeningRow { address: host.surface, glazing: true, offset: [0.0; 2], width: host.width, height: host.height };
                    self.stage = Stage::Publish { next: None, row: Some(row) };
                } else {
                    self.stage = Stage::Range { range: Some(OpeningRange { first: self.first, count: self.count - self.first }) };
                }
            }
            Stage::Deduplicate { candidate, next } => {
                let child = surface(model, inferred, *candidate).ok_or_else(|| ValueError::invariant("gbXML paired child disappeared"))?;
                let next_child = self.children.next(sides, SurfaceAddress { room: host.surface.room, side: *next });
                if let Some((address, other)) = next_child.and_then(|address| surface(model, inferred, address).map(|child| (address, child))) {
                    if other.parent == first.id && other.element == child.element {
                        self.stage = Stage::Scan { side: 1, next: candidate.side + 1 };
                    } else {
                        *next = address.side + 1;
                    }
                } else {
                    let frame = Frame { normal: host.normal, right: host.right, up: host.up };
                    let geometry = OpeningProjection::new(frame, host.corner, turned(&child.polygon[0], host.bearing), child.polygon.len());
                    self.stage = Stage::Geometry { candidate: *candidate, side: 1, geometry };
                }
            }
            Stage::Geometry { candidate, side, geometry } => {
                let child = surface(model, inferred, *candidate).ok_or_else(|| ValueError::invariant("gbXML opening geometry disappeared"))?;
                if let Some((offset, rect)) = geometry.advance(POINTS, |index| turned(&child.polygon[index], host.bearing)) {
                    let row = OriginalOpeningRow { address: *candidate, glazing: false, offset, width: rect.width, height: rect.height };
                    self.stage = Stage::Publish { next: Some((*side, candidate.side + 1)), row: Some(row) };
                }
            }
            Stage::Publish { .. } | Stage::Range { .. } => unreachable!(),
        }
        Ok(false)
    }

    /// 📏️ The number of original opening rows.
    pub fn len(&self) -> usize { self.count }

    /// 🧮️ Whether no valid openings were planned.
    pub fn is_empty(&self) -> bool { self.count == 0 }

    /// 🔢️ Reads the ordered openings of one canonical host's source ordinal.
    pub fn range(&self, host: usize) -> Option<OpeningRange> { self.ranges.get(host).copied() }

    /// 📖️ Reads one opening's original address and rectangle.
    pub fn row(&self, index: usize) -> Option<OriginalOpeningRow> { self.rows.get(index).copied() }

    /// ♻️ Quotes the next original opening row or range owner.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.children.rows.terminal_is_empty() { self.children.rows.retirement_demands(body) }
        else if !self.rows.terminal_is_empty() { self.rows.retirement_demands(body) } else if !self.ranges.terminal_is_empty() {
            self.ranges.retirement_demands(body)
        } else {
            Ok(Default::default())
        }
    }

    /// 🧺️ Whether both original opening owners have retired.
    pub fn terminal_is_empty(&self) -> bool { self.children.rows.terminal_is_empty() && self.rows.terminal_is_empty() && self.ranges.terminal_is_empty() }

    /// 🔚️ Retires one original opening or source loan.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.children.rows.terminal_is_empty() { self.children.rows.close_step(cx)?; }
        else if !self.rows.terminal_is_empty() { self.rows.close_step(cx)?; } else if !self.ranges.terminal_is_empty() {
            self.ranges.close_step(cx)?;
        }
        Ok(self.terminal_is_empty())
    }
}
