//! 🪡️ Builds only the paid COS replacements for an existing page rotation and document Info.

use crate::standards::v1_7::subsets::base::{
    modules::{lexer::CosGrant, lower::lower_info_with, writer::classic_graph_quote::{GraphLoan, GraphOverlay}},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfSnapshot},
};

const MAX_DICTIONARY: usize = 32;
const MAX_NODES: usize = 128;
const MAX_LEAF: usize = 128;
pub const GRAFT_WORK: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraftRefusal { Catalog, Pages, Rotation, Info, Extent, Source }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraftTurn {
    pub work: usize,
    pub copied_bytes: usize,
    pub allocated: usize,
    pub ready: bool,
    pub needs: Option<usize>,
}

#[derive(Default)]
pub struct OrdinaryRows {
    pub page: Option<PdfIndirectObject>,
    pub info: Option<PdfIndirectObject>,
    pub remove: Option<usize>,
    pub info_ref: Option<ObjRef>,
}

impl OrdinaryRows {
    pub fn overlay(&self) -> GraphOverlay<'_> {
        GraphOverlay { page: self.page.as_ref(), info: self.info.as_ref(), remove: self.remove }
    }
}

pub struct OrdinaryGraft {
    root: Option<ObjRef>,
    next_page: usize,
    seen: [bool; 256],
    rows: OrdinaryRows,
    ready: bool,
}

impl Default for OrdinaryGraft {
    fn default() -> Self {
        Self { root: None, next_page: 0, seen: [false; 256], rows: OrdinaryRows::default(), ready: false }
    }
}

impl OrdinaryGraft {
    /// 🛠️ Quotes and funds one change. Unchanged objects always remain borrowed from the original COS.
    pub fn step(
        &mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot, grant: CosGrant, maximum_copy: usize,
    ) -> Result<GraftTurn, GraftRefusal> {
        if self.ready { return Ok(GraftTurn { ready: true, ..Default::default() }); }
        if grant.work < GRAFT_WORK { return Ok(Default::default()); }
        if self.root.is_none() {
            if loan.objects.len() > self.seen.len() || loan.objects.first().is_none_or(|row| row.id.num == 0)
                || loan.objects.windows(2).any(|pair| pair[0].id.num >= pair[1].id.num) { return Err(GraftRefusal::Source); }
            let catalog = dictionary(loan, loan.trailer.root).ok_or(GraftRefusal::Catalog)?;
            let root = dict_get(catalog, "Pages").and_then(reference).ok_or(GraftRefusal::Catalog)?;
            let pages = dictionary(loan, root).ok_or(GraftRefusal::Pages)?;
            let Some(PdfObject::Array(kids)) = dict_get(pages, "Kids") else { return Err(GraftRefusal::Pages); };
            if !name_is(pages, "Type", "Pages") || loan.objects.len() > self.seen.len() || kids.len() != snapshot.pages.len()
                || dict_get(pages, "Count") != Some(&PdfObject::Int(kids.len() as i64))
            { return Err(GraftRefusal::Pages); }
            self.root = Some(root);
            self.rows.info_ref = loan.trailer.info;
            return Ok(work(0, false));
        }
        if let Some(page) = snapshot.pages.get(self.next_page) {
            let root = self.root.ok_or(GraftRefusal::Pages)?;
            let pages = dictionary(loan, root).ok_or(GraftRefusal::Pages)?;
            let Some(PdfObject::Array(kids)) = dict_get(pages, "Kids") else { return Err(GraftRefusal::Pages); };
            let owner = kids.get(self.next_page).and_then(reference).ok_or(GraftRefusal::Pages)?;
            let index = loan.objects.binary_search_by_key(&owner.num, |row| row.id.num).map_err(|_| GraftRefusal::Pages)?;
            if self.seen[index] { return Err(GraftRefusal::Pages); }
            let original = dictionary(loan, owner).ok_or(GraftRefusal::Pages)?;
            if !name_is(original, "Type", "Page") || dict_get(original, "Parent").and_then(reference) != Some(root) {
                return Err(GraftRefusal::Pages);
            }
            let old = match dict_get(original, "Rotate").or_else(|| dict_get(pages, "Rotate")) {
                None => 0,
                Some(PdfObject::Int(n)) => i32::try_from(*n).map_err(|_| GraftRefusal::Rotation)?,
                _ => return Err(GraftRefusal::Rotation),
            };
            let mut allocated = 0;
            if old != page.rotate {
                if self.rows.page.is_some() || !matches!(page.rotate, 0 | 90 | 180 | 270) {
                    return Err(GraftRefusal::Rotation);
                }
                let count = original.len() + usize::from(dict_get(original, "Rotate").is_none());
                let mut nodes = 0;
                let extent = count * size_of::<PdfDictEntry>() + 6 + child_extent(original, &mut nodes, 0)?;
                if let Some(refusal) = missing_grant(extent, grant, maximum_copy) { return Ok(refusal); }
                let mut entries = Vec::with_capacity(count);
                entries.extend(original.iter().cloned());
                let rotation = PdfObject::Int(i64::from(page.rotate));
                if let Some(entry) = entries.iter_mut().find(|entry| entry.key == "Rotate") {
                    entry.value = rotation;
                } else {
                    entries.push(PdfDictEntry::new("Rotate", rotation));
                }
                allocated = dict_capacity(&entries, entries.capacity());
                self.rows.page = Some(PdfIndirectObject { id: owner, value: PdfObject::Dict(entries) });
            }
            self.seen[index] = true;
            self.next_page += 1;
            return Ok(work(allocated, false));
        }
        let result = self.info(loan, &snapshot.info, grant, maximum_copy)?;
        self.ready = result.ready;
        Ok(result)
    }

    pub fn ready(&self) -> bool { self.ready }

    pub fn into_rows(self) -> Result<OrdinaryRows, Self> {
        if self.ready { Ok(self.rows) } else { Err(self) }
    }

    pub fn take_partial(self) -> OrdinaryRows { self.rows }

    fn info(&mut self, loan: GraphLoan<'_>, info: &PdfInfo, grant: CosGrant, maximum_copy: usize) -> Result<GraftTurn, GraftRefusal> {
        let current = loan.trailer.info.map(|reference| dictionary(loan, reference).ok_or(GraftRefusal::Info)).transpose()?.unwrap_or(&[]);
        let fields = info_fields(info);
        if !info.extra.is_empty() || info.creation_date.is_some() || info.modification_date.is_some()
            || fields.iter().filter_map(|(_, text)| *text).any(|text| !plain(text.as_bytes()))
        { return Err(GraftRefusal::Info); }
        let mut seen = [false; 7];
        let mut changed = false;
        for entry in current {
            let index = fields.iter().position(|(key, _)| *key == entry.key).ok_or(GraftRefusal::Info)?;
            if std::mem::replace(&mut seen[index], true) { return Err(GraftRefusal::Info); }
            let raw = match &entry.value {
                PdfObject::Str(bytes) if index != 6 => bytes,
                PdfObject::Text(text) if index != 6 => text.as_bytes(),
                PdfObject::Name(text) if index == 6 => text.as_bytes(),
                _ => return Err(GraftRefusal::Info),
            };
            if !plain(raw) { return Err(GraftRefusal::Info); }
            changed |= fields[index].1.map(str::as_bytes) != Some(raw);
        }
        changed |= fields.iter().enumerate().any(|(index, (_, text))| !seen[index] && text.is_some());
        if !changed { return Ok(work(0, true)); }
        if fields.iter().all(|(_, text)| text.is_none()) {
            if let Some(info) = loan.trailer.info {
                let index = loan.objects.binary_search_by_key(&info.num, |row| row.id.num).map_err(|_| GraftRefusal::Info)?;
                self.rows.remove = Some(index);
                self.rows.info_ref = None;
            }
            return Ok(work(0, true));
        }
        let extent = 16 * size_of::<PdfDictEntry>() + fields.iter().filter_map(|(key, text)| text.map(|value| key.len() + value.len())).sum::<usize>();
        if let Some(refusal) = missing_grant(extent, grant, maximum_copy) { return Ok(refusal); }
        let reference = match loan.trailer.info {
            Some(info) => info,
            None => ObjRef { num: loan.objects.last().ok_or(GraftRefusal::Info)?.id.num.checked_add(1).ok_or(GraftRefusal::Extent)?, gen: 0 },
        };
        let dict = lower_info_with(info, |text| text.as_bytes().to_vec());
        let allocated = dict_capacity(&dict, dict.capacity());
        self.rows.info = Some(PdfIndirectObject { id: reference, value: PdfObject::Dict(dict) });
        self.rows.info_ref = Some(reference);
        Ok(work(allocated, true))
    }
}

fn work(allocated: usize, ready: bool) -> GraftTurn {
    GraftTurn { work: GRAFT_WORK, copied_bytes: allocated + if allocated == 0 { 0 } else { size_of::<PdfIndirectObject>() },
        allocated, ready, needs: None }
}

fn missing_grant(extent: usize, grant: CosGrant, maximum_copy: usize) -> Option<GraftTurn> {
    (extent > grant.allocation_bytes || extent + size_of::<PdfIndirectObject>() > maximum_copy)
        .then_some(GraftTurn { work: GRAFT_WORK, needs: Some(extent), ..Default::default() })
}

fn plain(bytes: &[u8]) -> bool { bytes.len() <= MAX_LEAF && bytes.iter().all(|byte| (0x20..=0x7e).contains(byte)) }

fn info_fields(info: &PdfInfo) -> [(&str, Option<&str>); 7] {
    [("Title", info.title.as_deref()), ("Author", info.author.as_deref()), ("Subject", info.subject.as_deref()),
     ("Keywords", info.keywords.as_deref()), ("Creator", info.creator.as_deref()), ("Producer", info.producer.as_deref()),
     ("Trapped", info.trapped.as_deref())]
}

fn reference(value: &PdfObject) -> Option<ObjRef> {
    match value { PdfObject::Ref(reference) => Some(*reference), _ => None }
}

fn dictionary(loan: GraphLoan<'_>, owner: ObjRef) -> Option<&[PdfDictEntry]> {
    match &loan.lookup(owner)?.value {
        PdfObject::Dict(entries) if entries.len() <= MAX_DICTIONARY => Some(entries),
        _ => None,
    }
}

fn dict_get<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> {
    dict.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
}

fn name_is(dict: &[PdfDictEntry], key: &str, wanted: &str) -> bool {
    matches!(dict_get(dict, key), Some(PdfObject::Name(name)) if name == wanted)
}

fn child_extent(dict: &[PdfDictEntry], budget: &mut usize, depth: usize) -> Result<usize, GraftRefusal> {
    if depth > 6 || dict.len() > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
    let mut bytes = 0;
    for (index, entry) in dict.iter().enumerate() {
        if entry.key.len() > MAX_LEAF || dict[..index].iter().any(|earlier| earlier.key == entry.key) {
            return Err(GraftRefusal::Extent);
        }
        bytes += entry.key.len() + value_extent(&entry.value, budget, depth + 1)?;
    }
    Ok(bytes)
}

fn value_extent(value: &PdfObject, budget: &mut usize, depth: usize) -> Result<usize, GraftRefusal> {
    *budget += 1;
    if *budget > MAX_NODES || depth > 6 { return Err(GraftRefusal::Extent); }
    match value {
        PdfObject::Dict(dict) => Ok(dict.len() * size_of::<PdfDictEntry>() + child_extent(dict, budget, depth)?),
        PdfObject::Array(values) if values.len() <= MAX_DICTIONARY => {
            let mut bytes = values.len() * size_of::<PdfObject>();
            for value in values { bytes += value_extent(value, budget, depth + 1)?; }
            Ok(bytes)
        }
        PdfObject::Name(text) | PdfObject::Text(text) if text.len() <= MAX_LEAF => Ok(text.len()),
        PdfObject::Str(bytes) if bytes.len() <= MAX_LEAF => Ok(bytes.len()),
        PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Ref(_) | PdfObject::Date(_) => Ok(0),
        PdfObject::Real(decimal) if decimal.coefficient.len() <= MAX_LEAF && decimal.scale <= 384 && decimal.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => Ok(decimal.coefficient.len()),
        _ => Err(GraftRefusal::Extent),
    }
}

fn dict_capacity(dict: &[PdfDictEntry], capacity: usize) -> usize {
    capacity * size_of::<PdfDictEntry>() + dict.iter().map(|entry| entry.key.capacity() + value_capacity(&entry.value)).sum::<usize>()
}

fn value_capacity(value: &PdfObject) -> usize {
    match value {
        PdfObject::Dict(dict) => dict_capacity(dict, dict.capacity()),
        PdfObject::Array(values) => values.capacity() * size_of::<PdfObject>() + values.iter().map(value_capacity).sum::<usize>(),
        PdfObject::Name(text) | PdfObject::Text(text) => text.capacity(),
        PdfObject::Str(bytes) => bytes.capacity(),
        PdfObject::Real(decimal) => decimal.coefficient.capacity(),
        _ => 0,
    }
}
