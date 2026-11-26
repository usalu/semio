//! 🎟️ Plans geometric envelope rows in original caller-owned indexed storage.

use super::{ModelInference, ModelSnapshot, EnvelopeSpace, EnvelopeSurface, PolygonProjection, P3, bearing, round9, turned};
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::io::export::gbxml::pairing::{PairingScore, prefer_side, shared};

const POINTS: usize = 32;

#[derive(Clone, Copy, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct SurfaceAddress { pub room: usize, pub side: usize }

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub struct OriginalSurfaceRow {
    pub surface: SurfaceAddress,
    pub other: Option<SurfaceAddress>,
    pub bearing: f64,
    pub area: f64,
    pub azimuth: f64,
    pub tilt: f64,
    pub corner: P3,
    pub width: f64,
    pub height: f64,
    pub right: P3,
    pub up: P3,
    pub normal: P3,
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
enum Decision { Omit, Publish(OriginalSurfaceRow) }

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct RoomRange { first: usize, count: usize }

enum Stage {
    Host,
    Search { room: usize, next: usize, current: Option<PairingScore>, best: Option<(SurfaceAddress, f64)> },
    Geometry { source: SurfaceAddress, other: Option<SurfaceAddress>, paired: Option<SurfaceAddress>, turn: f64, geometry: PolygonProjection },
    Publish { paired: Option<SurfaceAddress>, partner: Option<Decision>, row: Option<Decision> },
}

impl Default for Stage { fn default() -> Self { Self::Host } }

/// 🗺️ Retains one scalar decision per envelope side and borrows its committed polygon.
#[derive(Default)]
pub struct OriginalSurfaces {
    source: Option<(usize, usize)>,
    ranges: InferenceValues<RoomRange>,
    rows: InferenceValues<Decision>,
    count_room: usize,
    count: usize,
    pending_range: Option<RoomRange>,
    position: SurfaceAddress,
    stage: Stage,
}

fn room_id(model: &ModelSnapshot, ordinal: usize) -> Option<&str> {
    model.spaces.get_index(ordinal).map(|(id, _)| id.as_str())
}

fn room_ordinal(model: &ModelSnapshot, id: &str) -> Option<usize> {
    let (mut low, mut high) = (0, model.spaces.len());
    while low < high {
        let rank = low + (high - low) / 2;
        match room_id(model, rank)?.cmp(id) {
            std::cmp::Ordering::Equal => return Some(rank),
            std::cmp::Ordering::Less => low = rank + 1,
            std::cmp::Ordering::Greater => high = rank,
        }
    }
    None
}

pub(super) fn surface<'a>(model: &ModelSnapshot, inferred: &'a ModelInference, address: SurfaceAddress) -> Option<&'a EnvelopeSurface> {
    envelope(model, inferred, address.room)?.surfaces.get(address.side)
}

pub(super) fn envelope<'a>(model: &ModelSnapshot, inferred: &'a ModelInference, room: usize) -> Option<&'a EnvelopeSpace> {
    inferred.energy_envelopes.get(room_id(model, room)?)
}

impl OriginalSurfaces {
    /// 🧮️ Advances one source observation, index item, score slice or geometry slice.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML surface planning was cancelled").into()); }
        let source = (std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(source); }
            return Ok(false);
        }
        if self.source != Some(source) { return Err(ValueError::invariant("gbXML surface source changed").into()); }
        if !self.ranges.admit_source_step(cx)? || !self.ranges.initialize_step(model.spaces.len(), cx)? { return Ok(false); }
        if self.count_room < model.spaces.len() {
            if self.pending_range.is_some() {
                if self.ranges.publish_step(self.count_room, &mut self.pending_range, cx)? {
                    self.count_room += 1;
                }
            } else if cx.charge_work_unit() {
                let id = room_id(model, self.count_room).ok_or_else(|| ValueError::invariant("gbXML space ordinal is absent"))?;
                let count = inferred.energy_envelopes.get(id).map_or(0, |room| room.surfaces.len());
                self.pending_range = Some(RoomRange { first: self.count, count });
                self.count = self.count.checked_add(count).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "gbXML surface index overflow"))?;
            }
            return Ok(false);
        }
        if !self.rows.admit_source_step(cx)? || !self.rows.initialize_step(self.count, cx)? { return Ok(false); }
        let seed = self.position;
        if seed.room == model.spaces.len() { return Ok(true); }
        if let Stage::Publish { paired, partner, row } = &mut self.stage {
            if let Some(paired) = *paired {
                let range = self.ranges.get(paired.room).ok_or_else(|| ValueError::invariant("gbXML reciprocal range is absent"))?;
                if self.rows.publish_step(range.first + paired.side, partner, cx)? {
                    self.stage = Stage::Publish { paired: None, partner: None, row: row.take() };
                }
            } else {
                let range = self.ranges.get(seed.room).ok_or_else(|| ValueError::invariant("gbXML active range is absent"))?;
                if self.rows.publish_step(range.first + seed.side, row, cx)? {
                    self.stage = Stage::Host;
                    self.position.side += 1;
                }
            }
            return Ok(false);
        }
        if !cx.charge_work_unit() { return Ok(false); }
        let range = self.ranges.get(seed.room).ok_or_else(|| ValueError::invariant("gbXML active range is absent"))?;
        if seed.side == range.count {
            self.position = SurfaceAddress { room: seed.room + 1, side: 0 };
            return Ok(false);
        }
        if self.rows.get(range.first + seed.side).is_some() {
            self.position.side += 1;
            return Ok(false);
        }
        let id = room_id(model, seed.room).ok_or_else(|| ValueError::invariant("gbXML active room is absent"))?;
        let host = surface(model, inferred, seed).ok_or_else(|| ValueError::invariant("gbXML active surface is absent"))?;
        match &mut self.stage {
            Stage::Host => {
                if !host.parent.is_empty() {
                    self.position.side += 1;
                } else if let Some(room) = shared(host).then(|| room_ordinal(model, &host.adjacent)).flatten() {
                    self.stage = Stage::Search { room, next: 0, current: None, best: None };
                } else {
                    self.stage = geometry(model, inferred, seed, None)?;
                }
            }
            Stage::Search { room, next, current, best } => {
                let opposite = self.ranges.get(*room).ok_or_else(|| ValueError::invariant("gbXML opposite room range is absent"))?;
                if *next == opposite.count {
                    self.stage = geometry(model, inferred, seed, best.map(|(address, _)| address))?;
                    return Ok(false);
                }
                let address = SurfaceAddress { room: *room, side: *next };
                let candidate = surface(model, inferred, address).ok_or_else(|| ValueError::invariant("gbXML reciprocal candidate is absent"))?;
                if let Some(score) = current {
                    if let Some(result) = score.advance(host, candidate, POINTS) {
                        if let Some(result) = result {
                            let select = best.is_none_or(|(previous, value)| {
                                result.total_cmp(&value).then_with(|| {
                                    surface(model, inferred, previous).expect("previous candidate exists").id.cmp(&candidate.id)
                                }).is_ge()
                            });
                            if select { *best = Some((address, result)); }
                        }
                        *current = None;
                        *next += 1;
                    }
                } else {
                    let ordinal = opposite.first + *next;
                    if ordinal <= range.first + seed.side || self.rows.get(ordinal).is_some() || !candidate.parent.is_empty() || !shared(candidate) || candidate.adjacent != id {
                        *next += 1;
                    } else if let Some(score) = PairingScore::new(host, candidate) {
                        *current = Some(score);
                    } else {
                        *next += 1;
                    }
                }
            }
            Stage::Geometry { source, other, paired, turn, geometry } => {
                let input = surface(model, inferred, *source).ok_or_else(|| ValueError::invariant("gbXML canonical surface is absent"))?;
                if let Some(projection) = geometry.advance(POINTS, |index| turned(&input.polygon[index], *turn)) {
                    let row = projection.map_or(Decision::Omit, |projection| Decision::Publish(OriginalSurfaceRow {
                        surface: *source, other: *other, bearing: *turn, area: round9(projection.area),
                        azimuth: projection.frame.azimuth(), tilt: projection.frame.tilt(), corner: projection.rect.corner,
                        width: projection.rect.width, height: projection.rect.height,
                        right: projection.frame.right, up: projection.frame.up, normal: projection.frame.normal,
                    }));
                    self.stage = Stage::Publish { paired: *paired, partner: paired.map(|_| Decision::Omit), row: Some(row) };
                }
            }
            Stage::Publish { .. } => unreachable!(),
        }
        Ok(false)
    }

    /// 🔢️ The source ordinal extent once planning has finished, including omitted sides.
    pub fn extent(&self) -> usize { self.count }

    /// 📍️ Finds the original room and side at a flattened source ordinal.
    pub fn source_at(&self, ordinal: usize) -> Option<SurfaceAddress> {
        let (mut low, mut high) = (0, self.count_room);
        while low < high {
            let room = low + (high - low) / 2;
            let range = self.ranges.get(room)?;
            if ordinal < range.first {
                high = room;
            } else if ordinal - range.first >= range.count {
                low = room + 1;
            } else {
                return Some(SurfaceAddress { room, side: ordinal - range.first });
            }
        }
        None
    }

    /// 🔢️ Maps the authored room and side back to their original flattened ordinal.
    pub fn ordinal(&self, address: SurfaceAddress) -> Option<usize> {
        let range = self.ranges.get(address.room)?;
        (address.side < range.count).then(|| range.first + address.side)
    }

    /// 🔎️ Reads the published canonical row of one source ordinal.
    pub fn get(&self, ordinal: usize) -> Option<OriginalSurfaceRow> {
        match self.rows.get(ordinal)? { Decision::Publish(row) => Some(*row), Decision::Omit => None }
    }

    /// ♻️ Quotes the next original range or decision owner retirement.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.rows.terminal_is_empty() { self.rows.retirement_demands(body) } else if !self.ranges.terminal_is_empty() {
            self.ranges.retirement_demands(body)
        } else {
            Ok(Default::default())
        }
    }

    /// 🧺️ Whether both original indexed owners are empty.
    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.ranges.terminal_is_empty() }

    /// 🔚️ Retires an original decision or offset in one funded turn.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.rows.terminal_is_empty() { self.rows.close_step(cx)?; } else if !self.ranges.terminal_is_empty() {
            self.ranges.close_step(cx)?;
        }
        Ok(self.terminal_is_empty())
    }
}

fn geometry(model: &ModelSnapshot, inferred: &ModelInference, seed: SurfaceAddress, paired: Option<SurfaceAddress>) -> Result<Stage, ValueError> {
    let id = room_id(model, seed.room).ok_or_else(|| ValueError::invariant("gbXML seed room is absent"))?;
    let host = surface(model, inferred, seed).ok_or_else(|| ValueError::invariant("gbXML seed surface is absent"))?;
    let (source, other) = if let Some(paired) = paired {
        let other_id = room_id(model, paired.room).ok_or_else(|| ValueError::invariant("gbXML reciprocal room is absent"))?;
        if prefer_side(id, host, other_id) { (seed, Some(paired)) } else { (paired, Some(seed)) }
    } else {
        (seed, None)
    };
    let canonical = room_id(model, source.room).ok_or_else(|| ValueError::invariant("gbXML canonical room is absent"))?;
    let envelope = inferred.energy_envelopes.get(canonical).ok_or_else(|| ValueError::invariant("gbXML canonical envelope is absent"))?;
    let turn = bearing(model, &envelope.storey);
    let geometry = PolygonProjection::new(envelope.surfaces[source.side].polygon.len());
    Ok(Stage::Geometry { source, other, paired, turn, geometry })
}
