//! 📏️ Quotes the supported classic COS graph before a writer can borrow any stream data.

use super::{DocumentTrailer, ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, WRITER_TRAILER_KEYS};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;

pub const MAX_OBJECTS: usize = 256;
pub const MAX_PREFIX: usize = 4096;
pub const QUOTE_WORK: usize = 4096;
const MAX_NODES: usize = 128;
const MAX_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphRefusal {
    SourceChanged, ObjectRoster, MissingReference, InvalidVersion, UnsupportedStream,
    ControlExtent, OutputExtent, NotQuoted,
}

#[derive(Clone, Copy, Default)]
pub struct GraphOverlay<'a> {
    pub page: Option<&'a PdfIndirectObject>,
    pub info: Option<&'a PdfIndirectObject>,
    pub remove: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct GraphLoan<'a> {
    pub version: &'a str,
    pub objects: &'a [PdfIndirectObject],
    pub trailer: &'a DocumentTrailer,
    pub overlay: GraphOverlay<'a>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceIdentity {
    version: (usize, usize),
    objects: (usize, usize),
    trailer: usize,
    overlay: (usize, usize, Option<usize>),
}

impl<'a> GraphLoan<'a> {
    pub(crate) fn identity(self) -> SourceIdentity {
        SourceIdentity {
            version: (self.version.as_ptr() as usize, self.version.len()),
            objects: (self.objects.as_ptr() as usize, self.objects.len()),
            trailer: std::ptr::from_ref(self.trailer) as usize,
            overlay: (address(self.overlay.page), address(self.overlay.info), self.overlay.remove),
        }
    }

    fn added_info(self) -> Option<&'a PdfIndirectObject> {
        self.overlay.info.filter(|info| self.objects.last().is_none_or(|last| info.id.num > last.id.num))
    }

    pub fn len(self) -> usize {
        self.objects.len() - usize::from(self.overlay.remove.is_some()) + usize::from(self.added_info().is_some())
    }

    pub fn object(self, index: usize) -> Option<&'a PdfIndirectObject> {
        let index = index + usize::from(self.overlay.remove.is_some_and(|removed| index >= removed));
        if index == self.objects.len() { return self.added_info(); }
        let original = self.objects.get(index)?;
        Some([self.overlay.page, self.overlay.info].into_iter().flatten()
            .find(|object| object.id == original.id).unwrap_or(original))
    }

    pub fn lookup(self, reference: ObjRef) -> Option<&'a PdfIndirectObject> {
        if let Some(added) = self.added_info().filter(|info| info.id == reference) { return Some(added); }
        let base = self.objects.binary_search_by_key(&reference.num, |object| object.id.num).ok()?;
        if self.overlay.remove == Some(base) { return None; }
        let index = base - usize::from(self.overlay.remove.is_some_and(|removed| removed < base));
        self.object(index).filter(|object| object.id == reference)
    }

    fn contains(self, reference: ObjRef) -> bool { self.lookup(reference).is_some() }
}

fn address(value: Option<&PdfIndirectObject>) -> usize {
    value.map_or(0, |object| std::ptr::from_ref(object) as usize)
}

pub(crate) fn preserved_trailer(entry: &PdfDictEntry) -> bool {
    !matches!(entry.key.as_str(), "Root" | "Info" | "ID") && !WRITER_TRAILER_KEYS.contains(&entry.key.as_str())
}

pub(crate) fn is_flate(filters: &[PdfStreamFilter]) -> bool {
    matches!(filters, [PdfStreamFilter::Flate { predictor: None }])
}

pub(crate) fn stream_extent(filters: &[PdfStreamFilter], length: usize) -> Result<usize, GraphRefusal> {
    match filters {
        [] | [PdfStreamFilter::Dct { .. }] | [PdfStreamFilter::Jpx] => Ok(length),
        filters if is_flate(filters) => super::classic_stored_flate::encoded_len(length).ok_or(GraphRefusal::OutputExtent),
        _ => Err(GraphRefusal::UnsupportedStream),
    }
}

#[derive(Default)]
struct Structure {
    nodes: usize,
    bytes: usize,
}

impl Structure {
    fn add(&mut self, bytes: usize) -> Result<(), GraphRefusal> {
        self.bytes = self.bytes.checked_add(bytes).filter(|extent| *extent <= MAX_PREFIX)
            .ok_or(GraphRefusal::ControlExtent)?;
        Ok(())
    }

    fn scaled(&mut self, length: usize, factor: usize, extra: usize) -> Result<(), GraphRefusal> {
        self.add(length.checked_mul(factor).and_then(|size| size.checked_add(extra)).ok_or(GraphRefusal::ControlExtent)?)
    }

    fn entry(&mut self, entry: &PdfDictEntry, graph: GraphLoan<'_>, depth: usize) -> Result<(), GraphRefusal> {
        self.scaled(entry.key.len(), 3, 4)?;
        self.value(&entry.value, graph, depth)
    }

    fn value(&mut self, value: &PdfObject, graph: GraphLoan<'_>, depth: usize) -> Result<(), GraphRefusal> {
        self.nodes += 1;
        if self.nodes > MAX_NODES || depth > MAX_DEPTH { return Err(GraphRefusal::ControlExtent); }
        match value {
            PdfObject::Null | PdfObject::Bool(_) => self.add(5),
            PdfObject::Int(_) | PdfObject::Ref(_) => {
                if let PdfObject::Ref(reference) = value {
                    if !graph.contains(*reference) { return Err(GraphRefusal::MissingReference); }
                }
                self.add(20)
            }
            PdfObject::Real(value) if value.coefficient.len() <= 128 && value.scale <= 384 && value.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => self.add(512),
            PdfObject::Real(_) => Err(GraphRefusal::ControlExtent),
            PdfObject::Date(_) => self.add(64),
            PdfObject::Str(bytes) => self.scaled(bytes.len(), 2, 2),
            PdfObject::Text(text) => self.scaled(text.len(), 4, 6),
            PdfObject::Name(name) => self.scaled(name.len(), 3, 1),
            PdfObject::Array(array) => {
                self.scaled(array.len(), 1, 2)?;
                for value in array { self.value(value, graph, depth + 1)?; }
                Ok(())
            }
            PdfObject::Dict(dict) => {
                self.add(4)?;
                for entry in dict { self.entry(entry, graph, depth + 1)?; }
                Ok(())
            }
            PdfObject::Stream { .. } => Err(GraphRefusal::UnsupportedStream),
        }
    }
}

pub struct GraphQuote {
    source: SourceIdentity,
    limit: usize,
    index: usize,
    maximum: usize,
    ready: bool,
}

impl GraphQuote {
    pub fn start(graph: GraphLoan<'_>, limit: usize) -> Result<Self, GraphRefusal> {
        if graph.objects.is_empty() || graph.objects.len() > MAX_OBJECTS
            || graph.overlay.remove.is_some_and(|index| index >= graph.objects.len())
            || graph.len() == 0 || graph.len() > MAX_OBJECTS || graph.trailer.extra.len() > MAX_NODES {
            return Err(GraphRefusal::ObjectRoster);
        }
        if !graph.version.is_empty() && (graph.version.len() > 16
            || !graph.version.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.'))
        { return Err(GraphRefusal::InvalidVersion); }
        let mut previous = 0;
        for object in graph.objects {
            if object.id.num <= previous { return Err(GraphRefusal::ObjectRoster); }
            previous = object.id.num;
        }
        if !graph.contains(graph.trailer.root) || graph.trailer.info.is_some_and(|info| !graph.contains(info)) {
            return Err(GraphRefusal::MissingReference);
        }
        let last = graph.object(graph.len() - 1).ok_or(GraphRefusal::ObjectRoster)?.id.num;
        let overhead = (last as usize).checked_add(1).and_then(|size| size.checked_mul(20))
            .and_then(|size| size.checked_add(128)).filter(|size| *size <= limit && *size <= u32::MAX as usize)
            .ok_or(GraphRefusal::OutputExtent)?;
        Ok(Self { source: graph.identity(), limit, index: 0, maximum: overhead, ready: false })
    }

    /// 🧮️ Quotes at most one object or the trailer after the caller grants the fixed control turn.
    pub fn next(&mut self, graph: GraphLoan<'_>, work: usize) -> Result<bool, GraphRefusal> {
        if self.source != graph.identity() { return Err(GraphRefusal::SourceChanged); }
        if self.ready || work < QUOTE_WORK { return Ok(self.ready); }
        let mut structure = Structure { nodes: 0, bytes: if self.index == graph.len() { 160 } else { 64 } };
        let raw = if let Some(object) = graph.object(self.index) {
            match &object.value {
                PdfObject::Stream { dict, data, filters } => {
                    if dict.len() > MAX_NODES { return Err(GraphRefusal::UnsupportedStream); }
                    if !filters.is_empty() { structure.add(256)?; }
                    for entry in dict.iter().filter(|entry| entry.key != "Length"
                        && (filters.is_empty() || !matches!(entry.key.as_str(), "Filter" | "DecodeParms")))
                    {
                        structure.entry(entry, graph, 0)?;
                    }
                    stream_extent(filters, data.len())?
                }
                value => { structure.value(value, graph, 0)?; 0 }
            }
        } else {
            for entry in graph.trailer.extra.iter().filter(|entry| preserved_trailer(entry)) {
                structure.entry(entry, graph, 0)?;
            }
            if let Some([first, second]) = &graph.trailer.id {
                structure.scaled(first.len(), 2, 3)?;
                structure.scaled(second.len(), 2, 3)?;
            }
            0
        };
        let maximum = self.maximum.checked_add(structure.bytes).and_then(|size| size.checked_add(raw))
            .filter(|size| *size <= self.limit && *size <= u32::MAX as usize).ok_or(GraphRefusal::OutputExtent)?;
        self.maximum = maximum;
        if self.index == graph.len() { self.ready = true; } else { self.index += 1; }
        Ok(self.ready)
    }

    pub fn maximum(&self) -> Option<usize> { self.ready.then_some(self.maximum) }

    pub(crate) fn source(&self) -> SourceIdentity { self.source }
}
