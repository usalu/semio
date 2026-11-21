//! 🧾 Places canonical SetImage metadata in its concrete owner, one bounded field at a time.

use super::{PdfMutation, receiving_invalid};
use crate::standards::v1_7::subsets::base::schema::{
    mutations::SetImage,
    snapshot::{PdfColorSpace, PdfDictEntry, PdfImage, PdfImageMask, PdfObject},
};
use semio_framework_value::{
    DslValue, RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::retained_clone_grant_admits,
};

const FIELD_COUNT: usize = 17;
const FIELDS: [&str; FIELD_COUNT] = [
    "id", "width", "height", "colorSpace", "bitsPerComponent", "imageMask", "decode", "interpolate",
    "body", "softMask", "softMaskInData", "mask", "matte", "intent", "optionalContent", "structParent", "extra",
];
const SMALL_RECORD: usize = 24;
const SMALL_SEQUENCE: usize = 16;

#[derive(Clone, Copy, Default)]
enum Phase { #[default] Root, Image, Scope, Build, Field(usize), Done }

#[derive(Default)]
pub(super) struct MetadataReceiving {
    phase: Phase,
    scan: usize,
    image: Option<usize>,
    index: Option<usize>,
    positions: [Option<usize>; FIELD_COUNT],
}

pub(super) enum MetadataStep { Pending, Ordinary, Complete }

fn record(value: &DslValue) -> Result<&[(String, DslValue)], ValueError> {
    match value { DslValue::Object(fields) => Ok(fields), _ => Err(receiving_invalid("PDF image metadata expects an object")) }
}

fn record_mut(value: &mut DslValue) -> Result<&mut [(String, DslValue)], ValueError> {
    match value { DslValue::Object(fields) => Ok(fields), _ => Err(receiving_invalid("PDF image metadata expects an object")) }
}

fn named<'a>(value: &'a DslValue, name: &str) -> Option<&'a DslValue> {
    record(value).ok()?.iter().find(|(field, _)| field == name).map(|(_, value)| value)
}

fn named_mut<'a>(value: &'a mut DslValue, name: &str) -> Option<&'a mut DslValue> {
    record_mut(value).ok()?.iter_mut().find(|(field, _)| field == name).map(|(_, value)| value)
}

fn array(value: Option<&DslValue>) -> Result<&[DslValue], ValueError> {
    match value { None => Ok(&[]), Some(DslValue::Array(values)) => Ok(values),
        _ => Err(receiving_invalid("PDF image metadata expects an array")) }
}

fn small_record(value: &DslValue) -> bool {
    record(value).map_or(true, |fields| fields.len() <= SMALL_RECORD)
}

fn small_array(value: Option<&DslValue>) -> bool {
    array(value).map_or(true, |values| values.len() <= SMALL_SEQUENCE)
}

fn scope_color(value: Option<&DslValue>) -> bool {
    let Some(value) = value else { return true; };
    small_record(value) && matches!(named(value, "kind").and_then(DslValue::as_str),
        None | Some("deviceGray" | "deviceRgb" | "deviceCmyk" | "named"))
}

fn scope_mask(value: Option<&DslValue>) -> bool {
    let Some(value) = value else { return true; };
    small_record(value) && small_array(named(value, "ranges"))
}

fn scope_extra(value: Option<&DslValue>) -> bool {
    let Ok(values) = array(value) else { return true; };
    values.len() <= SMALL_SEQUENCE && values.iter().all(|entry| {
        small_record(entry) && named(entry, "value").is_none_or(|value| {
            small_record(value) && matches!(named(value, "kind").and_then(DslValue::as_str),
                None | Some("null" | "bool" | "int" | "text" | "name"))
        })
    })
}

fn unsigned<T: TryFrom<u64>>(value: Option<&DslValue>) -> Result<T, ValueError> {
    value.and_then(DslValue::as_u64).and_then(|value| T::try_from(value).ok())
        .ok_or_else(|| receiving_invalid("PDF image metadata expects an exact unsigned integer"))
}

fn optional<T>(value: Option<&DslValue>, decode: impl FnOnce(Option<&DslValue>) -> Result<T, ValueError>) -> Result<Option<T>, ValueError> {
    match value { None | Some(DslValue::Null) => Ok(None), value => decode(value).map(Some) }
}

fn boolean(value: Option<&DslValue>) -> Result<bool, ValueError> {
    match value { Some(DslValue::Bool(value)) => Ok(*value), _ => Err(receiving_invalid("PDF image metadata expects a boolean")) }
}

fn float(value: &DslValue) -> Result<f64, ValueError> {
    value.as_f64().ok_or_else(|| receiving_invalid("PDF image metadata expects a number"))
}

fn string(value: Option<&mut DslValue>) -> Result<String, ValueError> {
    match value { Some(DslValue::String(value)) => Ok(std::mem::take(value)),
        _ => Err(receiving_invalid("PDF image metadata expects text")) }
}

fn optional_string(value: Option<&mut DslValue>) -> Result<Option<String>, ValueError> {
    match value { None | Some(DslValue::Null) => Ok(None), value => string(value).map(Some) }
}

fn color(value: Option<&mut DslValue>) -> Result<Option<PdfColorSpace>, ValueError> {
    let Some(value) = value.filter(|value| !matches!(value, DslValue::Null)) else { return Ok(None); };
    Ok(Some(match named(value, "kind").and_then(DslValue::as_str) {
        Some("deviceGray") => PdfColorSpace::DeviceGray,
        Some("deviceRgb") => PdfColorSpace::DeviceRgb,
        Some("deviceCmyk") => PdfColorSpace::DeviceCmyk,
        Some("named") => PdfColorSpace::Named { name: string(named_mut(value, "name"))? },
        _ => return Err(receiving_invalid("invalid PDF image color-space discriminator")),
    }))
}

fn reserve<T>(count: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<T>, ValueError> {
    let mut output = Vec::new();
    if count == 0 { return Ok(output); }
    if effect.actual().is_some() { return Err(ValueError::invariant("PDF metadata allocation post is occupied")); }
    *effect.actual() = Some(0);
    output.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF image metadata reserve failed"))?;
    *effect.actual() = Some(output.capacity() * size_of::<T>());
    Ok(output)
}

fn sequence<T>(values: &[DslValue], decode: impl Fn(&DslValue) -> Result<T, ValueError>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<T>, ValueError> {
    for value in values { decode(value)?; }
    let mut output = reserve(values.len(), effect)?;
    output.extend(values.iter().map(|value| decode(value).ok().expect("validated PDF image metadata sample")));
    Ok(output)
}

fn mask(value: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<PdfImageMask>, ValueError> {
    let Some(value) = value.filter(|value| !matches!(value, DslValue::Null)) else { return Ok(None); };
    Ok(Some(match named(value, "kind").and_then(DslValue::as_str) {
        Some("stencil") => PdfImageMask::Stencil { image: string(named_mut(value, "image"))? },
        Some("colorKey") => {
            let ranges = named(value, "ranges").ok_or_else(|| receiving_invalid("PDF color-key mask requires ranges"))?;
            PdfImageMask::ColorKey { ranges: sequence(array(Some(ranges))?, |value| unsigned(Some(value)), effect)? }
        }
        _ => return Err(receiving_invalid("invalid PDF image mask discriminator")),
    }))
}

fn scalar_cos(value: &DslValue) -> Result<PdfObject, ValueError> {
    let field = named(value, "value");
    match named(value, "kind").and_then(DslValue::as_str) {
        Some("null") => Ok(PdfObject::Null),
        Some("bool") => boolean(field).map(PdfObject::Bool),
        Some("int") => field.and_then(DslValue::as_i64).map(PdfObject::Int)
            .ok_or_else(|| receiving_invalid("PDF COS integer expects an exact signed integer")),
        Some("text" | "name") if field.and_then(DslValue::as_str).is_some() => Ok(PdfObject::Null),
        _ => Err(receiving_invalid("invalid PDF image extra COS value")),
    }
}

fn extra(value: Option<&mut DslValue>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Vec<PdfDictEntry>, ValueError> {
    let Some(value) = value else { return Ok(Vec::new()); };
    for entry in array(Some(value))? {
        if named(entry, "key").and_then(DslValue::as_str).is_none() {
            return Err(receiving_invalid("PDF image extra entry requires a text key"));
        }
        scalar_cos(named(entry, "value").ok_or_else(|| receiving_invalid("PDF image extra entry requires a value"))?)?;
    }
    let DslValue::Array(entries) = value else { unreachable!() };
    let mut output = reserve(entries.len(), effect)?;
    for entry in entries {
        let key = string(named_mut(entry, "key")).expect("validated extra key");
        let raw = named_mut(entry, "value").expect("validated COS value");
        let value = match named(raw, "kind").and_then(DslValue::as_str) {
            Some("text") => PdfObject::Text(string(named_mut(raw, "value")).expect("validated COS text")),
            Some("name") => PdfObject::Name(string(named_mut(raw, "value")).expect("validated COS name")),
            _ => scalar_cos(raw).expect("validated COS scalar"),
        };
        output.push(PdfDictEntry { key, value });
    }
    Ok(output)
}

impl MetadataReceiving {
    fn image<'a>(&self, value: &'a DslValue) -> Result<&'a [(String, DslValue)], ValueError> {
        let ordinal = self.image.ok_or_else(|| receiving_invalid("SetImage requires image"))?;
        record(&record(value)?[ordinal].1)
    }

    fn image_mut<'a>(&self, value: &'a mut DslValue) -> Result<&'a mut [(String, DslValue)], ValueError> {
        let ordinal = self.image.ok_or_else(|| receiving_invalid("SetImage requires image"))?;
        record_mut(&mut record_mut(value)?[ordinal].1)
    }

    fn field<'a>(&self, value: &'a DslValue, field: usize) -> Result<Option<&'a DslValue>, ValueError> {
        Ok(self.positions[field].map(|index| &self.image(value).expect("validated image record")[index].1))
    }

    pub(super) fn demand(&self, value: &DslValue) -> Result<(RetirementDemand, u64), ValueError> {
        let base_copy = size_of::<Self>() + size_of::<PdfImage>() + size_of::<Option<PdfMutation>>();
        let (count, element, work) = match self.phase {
            Phase::Scope => (0, 0, 8 + 4 * SMALL_RECORD as u64 * SMALL_SEQUENCE as u64),
            Phase::Field(field @ (6 | 11 | 12 | 16)) => {
                let value = self.field(value, field)?;
                let count = if field == 11 {
                    value.filter(|value| named(value, "kind").and_then(DslValue::as_str) == Some("colorKey"))
                        .and_then(|value| array(named(value, "ranges")).ok()).map_or(0, <[DslValue]>::len)
                } else { array(value).map_or(0, <[DslValue]>::len) };
                let element = match field { 11 => size_of::<u32>(), 16 => size_of::<PdfDictEntry>(), _ => size_of::<f64>() };
                let work = if field == 16 { 8 + 8 * SMALL_RECORD as u64 * count as u64 }
                    else { 2 + 4 * count as u64 + 2 * SMALL_RECORD as u64 };
                (count, element, work)
            }
            Phase::Field(3 | 8) => (0, 0, 2 + 2 * SMALL_RECORD as u64),
            Phase::Image => (0, 0, FIELDS.len() as u64 + 1),
            _ => (0, 0, 1),
        };
        Ok((RetirementDemand { copy_bytes: base_copy + count * element * 2,
            capacity_bytes: count * element, depth: 4, ..Default::default() }, work))
    }

    pub(super) fn advance(
        &mut self, value: &mut DslValue, output: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<(MetadataStep, RetainedCloneProgress), ValueError> {
        let (demand, work) = self.demand(value)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work || !effect.performed(work)? {
            return Ok((MetadataStep::Pending, Default::default()));
        }
        let action = self.perform(value, output, effect);
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes,
            retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() };
        action.map(|state| (state, receipt)).map_err(|error| error.with_retained_progress(receipt))
    }

    fn perform(
        &mut self, value: &mut DslValue, output: &mut Option<PdfMutation>, effect: &mut RetainedAllocationEffect<'_>,
    ) -> Result<MetadataStep, ValueError> {
        match self.phase {
            Phase::Root => {
                if let Some((key, _)) = record(value)?.get(self.scan) {
                    match key.as_str() { "image" => self.image = Some(self.scan), "index" => self.index = Some(self.scan), _ => {} }
                    self.scan += 1;
                } else { self.scan = 0; self.phase = Phase::Image; }
            }
            Phase::Image => {
                if let Some((key, _)) = self.image(value)?.get(self.scan) {
                    if let Some(field) = FIELDS.iter().position(|field| field == key) { self.positions[field] = Some(self.scan); }
                    self.scan += 1;
                } else { self.phase = Phase::Scope; }
            }
            Phase::Scope => {
                if !(scope_color(self.field(value, 3)?) && small_array(self.field(value, 6)?)
                    && self.field(value, 8)?.is_none_or(small_record)
                    && scope_mask(self.field(value, 11)?) && small_array(self.field(value, 12)?)
                    && scope_extra(self.field(value, 16)?))
                {
                    return Ok(MetadataStep::Ordinary);
                }
                self.phase = Phase::Build;
            }
            Phase::Build => {
                if output.is_some() { return Err(ValueError::invariant("PDF metadata target occupied")); }
                let index = self.index.map(|index| &record(value).expect("validated SetImage record")[index].1);
                let index = optional(index, unsigned::<usize>)?;
                let mut image = PdfImage::samples(String::new(), 0, 0, PdfColorSpace::DeviceRgb, 0, Vec::new());
                image.color_space = None;
                *output = Some(PdfMutation::SetImage(SetImage { image, index }));
                self.phase = Phase::Field(0);
            }
            Phase::Field(field) => {
                let Some(PdfMutation::SetImage(target)) = output.as_mut() else {
                    return Err(ValueError::invariant("PDF metadata target disappeared"));
                };
                let value = self.positions[field].map(|index| &mut self.image_mut(value).expect("validated image record")[index].1);
                let shared = value.as_deref();
                let image = &mut target.image;
                match field {
                    0 => image.id = string(value)?,
                    1 => image.width = unsigned(shared)?,
                    2 => image.height = unsigned(shared)?,
                    3 => image.color_space = color(value)?,
                    4 => image.bits_per_component = shared.map_or(Ok(0), |value| unsigned(Some(value)))?,
                    5 => image.image_mask = shared.map_or(Ok(false), |value| boolean(Some(value)))?,
                    6 => image.decode = sequence(array(shared)?, float, effect)?,
                    7 => image.interpolate = shared.map_or(Ok(false), |value| boolean(Some(value)))?,
                    8 => {
                        if shared.and_then(|body| named(body, "kind")).and_then(DslValue::as_str) != Some("samples")
                            || !matches!(shared.and_then(|body| named(body, "values")), Some(DslValue::Array(values)) if values.is_empty())
                        {
                            return Err(ValueError::invariant("PDF image metadata lost its sample donation marker"));
                        }
                    }
                    9 => image.soft_mask = optional_string(value)?,
                    10 => image.soft_mask_in_data = optional(shared, unsigned::<u32>)?,
                    11 => image.mask = mask(value, effect)?,
                    12 => image.matte = match shared { None | Some(DslValue::Null) => None,
                        _ => Some(sequence(array(shared)?, float, effect)?) },
                    13 => image.intent = optional_string(value)?,
                    14 => image.optional_content = optional_string(value)?,
                    15 => image.struct_parent = optional(shared, unsigned::<u32>)?,
                    16 => image.extra = extra(value, effect)?,
                    _ => unreachable!(),
                }
                if field + 1 == FIELD_COUNT { self.phase = Phase::Done; } else { self.phase = Phase::Field(field + 1); }
            }
            Phase::Done => return Ok(MetadataStep::Complete),
        }
        Ok(MetadataStep::Pending)
    }
}
