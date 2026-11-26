//! 🌡️ Publishes canonical thermal zone ordinals from the committed envelope without owning authored names.

use super::{ModelInference, ModelSnapshot};
use semio_framework_job::StepContext;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_os_kernel::{InferenceValues, InferenceWorkFailure};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Key<'a> {
    owner: &'a str,
    synthetic: bool,
    settings: [Option<u64>; 3],
}

impl Key<'_> {
    fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        std::iter::once(b':').take(usize::from(self.synthetic)).chain(self.owner.bytes())
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes().cmp(other.bytes()).then_with(|| self.settings.cmp(&other.settings))
    }

    fn same_owner(&self, other: &Self) -> bool { self.bytes().eq(other.bytes()) }
}

fn key<'a>(model: &'a ModelSnapshot, inferred: &ModelInference, room: usize) -> Option<Key<'a>> {
    let (id, space) = model.spaces.get_index(room)?;
    inferred.energy_envelopes.get(id)?;
    let zone = space.zone.as_deref().filter(|zone| model.zones.contains_key(*zone))
        .or_else(|| synthetic_zone(model, id));
    let settings = model.space_conditions.get(id).map_or([None; 3], |conditions| {
        [conditions.heating_setpoint, conditions.cooling_setpoint, conditions.ventilation_rate].map(|setting| setting.map(f64::to_bits))
    });
    Some(Key { owner: zone.unwrap_or(id), synthetic: zone.is_none(), settings })
}

fn synthetic_zone<'a>(model: &'a ModelSnapshot, room: &str) -> Option<&'a str> {
    let (mut low, mut high) = (0, model.zones.len());
    while low < high {
        let index = low + (high - low) / 2;
        let (zone, _) = model.zones.get_index(index)?;
        match zone.bytes().cmp(std::iter::once(b':').chain(room.bytes())) {
            Ordering::Equal => return Some(zone.as_str()),
            Ordering::Less => low = index + 1,
            Ordering::Greater => high = index,
        }
    }
    None
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
struct Group { seed: usize, owner_offset: usize }

/// 🎟️ The declared source and thermal conditions for one canonical group.
pub struct OriginalZone<'a> {
    pub owner: &'a str,
    pub synthetic: bool,
    pub first_room: usize,
    pub owner_variant: Option<usize>,
    pub heating: Option<f64>,
    pub cooling: Option<f64>,
    pub outdoor_air: Option<f64>,
}

/// 📚️ Retains sorted groups and room-to-group rows under original indexed custody.
#[derive(Default)]
pub struct OriginalZones {
    source: Option<(usize, usize)>,
    groups: InferenceValues<Group>,
    members: InferenceValues<usize>,
    previous: Option<Group>,
    selected: Option<usize>,
    pending_group: Option<Group>,
    pending_member: Option<usize>,
    group_count: usize,
    scan: usize,
    room: usize,
    grouped: bool,
}

impl OriginalZones {
    /// 🛠️ Advances one original group search, row publication or member lookup.
    pub fn advance(&mut self, model: &ModelSnapshot, inferred: &ModelInference, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if cx.is_cancelled() { return Err(ValueError::literal(ValueRefusalKind::Canceled, "gbXML zone planning was cancelled").into()); }
        let source = (std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize);
        if self.source.is_none() {
            cx.original_source_parent()?.validate_borrowed_context(cx)?;
            if cx.charge_work_unit() { self.source = Some(source); }
            return Ok(false);
        }
        if self.source != Some(source) { return Err(ValueError::invariant("gbXML zone source changed").into()); }
        let count = model.spaces.len();
        if !self.groups.admit_source_step(cx)? || !self.groups.initialize_step(count, cx)? { return Ok(false); }
        if !self.grouped {
            if let Some(group) = self.pending_group {
                if self.groups.publish_step(self.group_count, &mut self.pending_group, cx)? {
                    self.group_count += 1;
                    self.previous = Some(group);
                    self.selected = None;
                    self.scan = 0;
                }
            } else if cx.charge_work_unit() {
                if self.scan < count {
                    let candidate = self.scan;
                    self.scan += 1;
                    if let Some(candidate_key) = key(model, inferred, candidate) {
                        if self.previous.and_then(|previous| key(model, inferred, previous.seed)).is_none_or(|previous| previous.cmp(&candidate_key).is_lt())
                            && self.selected.and_then(|selected| key(model, inferred, selected)).is_none_or(|selected| candidate_key.cmp(&selected).is_lt())
                        {
                            self.selected = Some(candidate);
                        }
                    }
                } else if let Some(seed) = self.selected {
                    let group_key = key(model, inferred, seed).ok_or_else(|| ValueError::invariant("gbXML zone key disappeared"))?;
                    let owner_offset = self.previous.filter(|previous| key(model, inferred, previous.seed).is_some_and(|previous| previous.same_owner(&group_key)))
                        .map_or(0, |previous| previous.owner_offset + 1);
                    self.pending_group = Some(Group { seed, owner_offset });
                } else {
                    self.grouped = true;
                }
            }
            return Ok(false);
        }
        if !self.members.admit_source_step(cx)? || !self.members.initialize_step(count, cx)? { return Ok(false); }
        if self.room == count { return Ok(true); }
        if self.pending_member.is_some() {
            if self.members.publish_step(self.room, &mut self.pending_member, cx)? { self.room += 1; }
        } else if cx.charge_work_unit() {
            let Some(target) = key(model, inferred, self.room) else { self.room += 1; return Ok(false); };
            self.pending_member = Some(self.find(model, inferred, target).ok_or_else(|| ValueError::invariant("gbXML member has no published thermal zone"))?);
        }
        Ok(false)
    }

    fn find(&self, model: &ModelSnapshot, inferred: &ModelInference, target: Key<'_>) -> Option<usize> {
        let (mut low, mut high) = (0, self.group_count);
        while low < high {
            let index = low + (high - low) / 2;
            let issued = key(model, inferred, self.groups.get(index)?.seed)?;
            match issued.cmp(&target) {
                Ordering::Equal => return Some(index),
                Ordering::Greater => high = index,
                Ordering::Less => low = index + 1,
            }
        }
        None
    }

    /// 🧾️ The number of uniquely grouped thermal zones.
    pub fn len(&self) -> usize { self.group_count }

    /// 🧮️ Whether planning published any zone.
    pub fn is_empty(&self) -> bool { self.group_count == 0 }

    /// 🏠️ Reads the original published group ordinal for a room.
    pub fn member(&self, room: usize) -> Option<usize> { self.members.get(room).copied() }

    /// 🏷️ Borrows the original owner and the sorted group's published conditions.
    pub fn get<'a>(&self, model: &'a ModelSnapshot, inferred: &ModelInference, index: usize) -> Option<OriginalZone<'a>> {
        if self.source != Some((std::ptr::from_ref(model) as usize, std::ptr::from_ref(inferred) as usize)) { return None; }
        let group = self.groups.get(index)?;
        let group_key = key(model, inferred, group.seed)?;
        let has_next = self.groups.get(index + 1).and_then(|next| key(model, inferred, next.seed)).is_some_and(|next| next.same_owner(&group_key));
        let [heating, cooling, outdoor_air] = group_key.settings.map(|setting| setting.map(f64::from_bits));
        Some(OriginalZone { owner: group_key.owner, synthetic: group_key.synthetic, first_room: group.seed,
            owner_variant: (!group_key.synthetic && (group.owner_offset != 0 || has_next)).then_some(group.owner_offset + 1),
            heating, cooling, outdoor_air })
    }

    /// ♻️ Quotes the selected original zone owner retirement.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.members.terminal_is_empty() { self.members.retirement_demands(body) } else if !self.groups.terminal_is_empty() {
            self.groups.retirement_demands(body)
        } else { Ok(Default::default()) }
    }

    /// 🧺️ Whether the original group and membership sources have both retired.
    pub fn terminal_is_empty(&self) -> bool { self.members.terminal_is_empty() && self.groups.terminal_is_empty() }

    /// 🔚️ Retires one original zone or membership row or source.
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, InferenceWorkFailure> {
        if !self.members.terminal_is_empty() { self.members.close_step(cx)?; } else if !self.groups.terminal_is_empty() {
            self.groups.close_step(cx)?;
        }
        Ok(self.terminal_is_empty())
    }
}
