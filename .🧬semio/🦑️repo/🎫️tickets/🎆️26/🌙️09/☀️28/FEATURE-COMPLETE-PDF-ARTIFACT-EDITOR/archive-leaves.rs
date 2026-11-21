//! 🌿 Receives the ordinary page rotation, document information and language mutations.

use super::{PdfMutation, receiving_invalid, metadata::{
    array, extra, named, optional, optional_string, record, scope_extra, small_record, unsigned,
}};
use crate::standards::v1_7::subsets::base::schema::{
    mutations::{SetInfo, SetLanguage, SetPageRotation},
    snapshot::{PdfDate, PdfDictEntry, PdfInfo},
};
use semio_framework_value::{
    DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError,
    retained_clone::retained_clone_grant_admits,
};

const ROOT: [&str; 4] = ["index", "rotation", "language", "info"];
const INFO: [&str; 10] = [
    "title", "author", "subject", "keywords", "creator", "producer",
    "creationDate", "modificationDate", "trapped", "extra",
];

#[derive(Clone, Copy)]
pub(super) enum Leaf { Rotation, Info, Language }

impl Leaf {
    pub(super) fn for_kind(kind: Option<&str>) -> Option<Self> {
        match kind {
            Some("setPageRotation") => Some(Self::Rotation),
            Some("setInfo") => Some(Self::Info),
            Some("setLanguage") => Some(Self::Language),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Phase { #[default] Find, Info, Scope, Build, Field(usize), Done }

#[derive(Default)]
pub(super) struct LeafReceiving {
    phase: Phase,
    scan: usize,
    root: [Option<usize>; ROOT.len()],
    info: [Option<usize>; INFO.len()],
}

pub(super) enum LeafStep { Pending, Ordinary, Complete }

fn signed(value: Option<&DslValue>) -> Result<i32, ValueError> {
    value.and_then(DslValue::as_i64).and_then(|number| i32::try_from(number).ok())
        .ok_or_else(|| receiving_invalid("PDF date needs an exact signed integer"))
}

fn date(value: Option<&DslValue>) -> Result<Option<PdfDate>, ValueError> {
    let Some(value) = value.filter(|node| !matches!(node, DslValue::Null)) else { return Ok(None); };
    record(value)?;
    let part = |name, default| named(value, name).map_or(Ok(default), |node| unsigned(Some(node)));
    Ok(Some(PdfDate {
        year: signed(named(value, "year"))?,
        month: part("month", 1)?,
        day: part("day", 1)?,
        hour: part("hour", 0)?,
        minute: part("minute", 0)?,
        second: part("second", 0)?,
        offset_minutes: optional(named(value, "offsetMinutes"), signed)?,
    }))
}

fn selected<'a>(value: &'a DslValue, position: Option<usize>) -> Option<&'a DslValue> {
    position.and_then(|pos| value.as_object()?.get(pos).map(|(_, value)| value))
}

fn selected_mut(value: &mut DslValue, position: Option<usize>) -> Option<&mut DslValue> {
    let DslValue::Object(fields) = value else { return None; };
    position.and_then(|pos| fields.get_mut(pos).map(|(_, value)| value))
}

impl LeafReceiving {
    fn info<'a>(&self, root: &'a DslValue) -> Result<&'a [(String, DslValue)], ValueError> {
        let value = selected(root, self.root[3]).ok_or_else(|| receiving_invalid("setInfo needs an info record"))?;
        record(value)
    }

    pub(super) fn demand(&self, leaf: Leaf, root: &DslValue) -> (RetirementDemand, u64) {
        let (count, work) = match self.phase {
            Phase::Find => (0, ROOT.len() as u64 + 1),
            Phase::Info => (0, INFO.len() as u64 + 1),
            Phase::Scope => (0, 8 + 4 * 24 * 16),
            Phase::Field(6 | 7) if matches!(leaf, Leaf::Info) => (0, 8 * 24 + 1),
            Phase::Field(9) if matches!(leaf, Leaf::Info) => {
                let count = selected(root, self.root[3]).and_then(|value| array(selected(value, self.info[9])).ok())
                    .map_or(0, <[DslValue]>::len);
                (count, 8 + 8 * 24 * count as u64)
            }
            _ => (0, 2),
        };
        let bytes = count * size_of::<PdfDictEntry>();
        (RetirementDemand { depth: 4, copy_bytes: size_of::<Self>() + 2 * size_of::<Option<PdfMutation>>() + 2 * bytes,
            capacity_bytes: bytes, ..Default::default() }, work)
    }

    pub(super) fn advance(
        &mut self, leaf: Leaf, value: &mut DslValue, output: &mut Option<PdfMutation>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(LeafStep, RetainedCloneProgress), ValueError> {
        let (demand, work) = self.demand(leaf, value);
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
            return Ok((LeafStep::Pending, Default::default()));
        }
        let action = self.perform(leaf, value, output, effect);
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
            retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
        action.map(|step| (step, receipt)).map_err(|error| error.with_retained_progress(receipt))
    }

    fn perform(
        &mut self, leaf: Leaf, value: &mut DslValue, output: &mut Option<PdfMutation>,
        effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<LeafStep, ValueError> {
        self.phase = match self.phase {
            Phase::Find => {
                if let Some((name, _)) = record(value)?.get(self.scan) {
                    if let Some(index) = ROOT.iter().position(|key| key == name) { self.root[index] = Some(self.scan); }
                    self.scan += 1;
                    Phase::Find
                } else if matches!(leaf, Leaf::Info) {
                    self.scan = 0;
                    Phase::Info
                } else { Phase::Build }
            }
            Phase::Info => {
                if let Some((name, _)) = self.info(value)?.get(self.scan) {
                    if let Some(index) = INFO.iter().position(|key| key == name) { self.info[index] = Some(self.scan); }
                    self.scan += 1;
                    Phase::Info
                } else { Phase::Scope }
            }
            Phase::Scope => {
                let info = selected(value, self.root[3]).expect("validated info field");
                if !selected(info, self.info[6]).is_none_or(small_record)
                    || !selected(info, self.info[7]).is_none_or(small_record)
                    || !scope_extra(selected(info, self.info[9]))
                { return Ok(LeafStep::Ordinary); }
                Phase::Build
            }
            Phase::Build => {
                if output.is_some() { return Err(ValueError::invariant("ordinary PDF mutation target is occupied")); }
                *output = Some(match leaf {
                    Leaf::Rotation => PdfMutation::SetPageRotation(SetPageRotation { index: 0, rotation: 0 }),
                    Leaf::Language => PdfMutation::SetLanguage(SetLanguage { language: None }),
                    Leaf::Info => PdfMutation::SetInfo(SetInfo { info: PdfInfo::default() }),
                });
                Phase::Field(0)
            }
            Phase::Field(field) => match output.as_mut().expect("admitted ordinary PDF mutation") {
                PdfMutation::SetPageRotation(target) => {
                    if field == 0 {
                        target.index = unsigned(selected(value, self.root[0]))?;
                        Phase::Field(1)
                    } else {
                        target.rotation = unsigned(selected(value, self.root[1]))?;
                        Phase::Done
                    }
                }
                PdfMutation::SetLanguage(target) => {
                    target.language = optional_string(selected_mut(value, self.root[2]))?;
                    Phase::Done
                }
                PdfMutation::SetInfo(target) => {
                    let raw = selected_mut(value, self.root[3]).expect("admitted info metadata");
                    let field_value = selected_mut(raw, self.info[field]);
                    let info = &mut target.info;
                    match field {
                        0 => info.title = optional_string(field_value)?,
                        1 => info.author = optional_string(field_value)?,
                        2 => info.subject = optional_string(field_value)?,
                        3 => info.keywords = optional_string(field_value)?,
                        4 => info.creator = optional_string(field_value)?,
                        5 => info.producer = optional_string(field_value)?,
                        6 => info.creation_date = date(field_value.as_deref())?,
                        7 => info.modification_date = date(field_value.as_deref())?,
                        8 => info.trapped = optional_string(field_value)?,
                        9 => info.extra = extra(field_value, effect)?,
                        _ => unreachable!(),
                    }
                    if field + 1 == INFO.len() { Phase::Done } else { Phase::Field(field + 1) }
                }
                _ => unreachable!("ordinary PDF mutation family changed"),
            },
            Phase::Done => return Ok(LeafStep::Complete),
        };
        Ok(LeafStep::Pending)
    }
}
