//! 🌊️ Writes a quoted classic PDF graph with one bounded control buffer and borrowed stream bodies.

use super::{PdfObject, encode_stream};
use super::classic_graph_quote::{GraphLoan, GraphQuote, GraphRefusal, MAX_OBJECTS, MAX_PREFIX, SourceIdentity, is_flate, preserved_trailer, stream_extent};
use super::classic_stored_flate::StoredFlate;
use crate::standards::v1_7::subsets::base::modules::lexer::{write_name, write_object, write_string};
use std::io::{self, Write};

#[derive(Debug)]
pub enum WriteRefusal { Graph(GraphRefusal), Sink(io::Error) }
impl From<GraphRefusal> for WriteRefusal { fn from(reason: GraphRefusal) -> Self { Self::Graph(reason) } }
impl From<io::Error> for WriteRefusal { fn from(error: io::Error) -> Self { Self::Sink(error) } }

#[derive(Clone, Copy, Default, Debug)]
pub struct GraphWriteTurn {
    pub prepared: usize,
    pub emitted: usize,
    pub done: bool,
    pub control: bool,
}

#[derive(Clone, Copy)]
enum Phase { Header, Object, Body, ObjectEnd, XrefHeader, XrefRow, Trailer, Done }

pub struct ClassicGraphWriter {
    source: SourceIdentity,
    limit: usize,
    offsets: [u32; MAX_OBJECTS],
    buffer: Vec<u8>,
    buffer_at: usize,
    object: usize,
    body_at: usize,
    flate: Option<StoredFlate>,
    output_at: usize,
    xref_at: usize,
    xref_row: u32,
    xref_object: usize,
    phase: Phase,
}

impl ClassicGraphWriter {
    /// 🌱 The caller donates one empty paid scratch allocation after the complete graph quote.
    pub fn start(quote: &GraphQuote, buffer: Vec<u8>) -> Result<Self, (GraphRefusal, Vec<u8>)> {
        let Some(limit) = quote.maximum() else { return Err((GraphRefusal::NotQuoted, buffer)); };
        if !buffer.is_empty() || buffer.capacity() < MAX_PREFIX {
            return Err((GraphRefusal::ControlExtent, buffer));
        }
        Ok(Self { source: quote.source(), limit, offsets: [0; MAX_OBJECTS], buffer,
            buffer_at: 0, object: 0, body_at: 0, flate: None, output_at: 0, xref_at: 0,
            xref_row: 0, xref_object: 0, phase: Phase::Header })
    }

    /// 🔗 Advances one control action or emits one bounded physical prefix.
    pub fn step(&mut self, graph: GraphLoan<'_>, sink: &mut dyn Write, grant: usize) -> Result<GraphWriteTurn, WriteRefusal> {
        if self.source != graph.identity() { return Err(GraphRefusal::SourceChanged.into()); }
        let mut result = GraphWriteTurn::default();
        if !self.buffer.is_empty() {
            let count = try_write(sink, &self.buffer[self.buffer_at..], grant, self.limit - self.output_at)?;
            result.emitted = count;
            self.buffer_at += count;
            self.output_at += count;
            if self.buffer_at == self.buffer.len() { self.buffer.clear(); self.buffer_at = 0; }
            return Ok(result);
        }
        if matches!(self.phase, Phase::Body) {
            let PdfObject::Stream { data, .. } = &graph.object(self.object).ok_or(GraphRefusal::SourceChanged)?.value else {
                return Err(GraphRefusal::SourceChanged.into());
            };
            if let Some(flate) = self.flate.as_mut() {
                let (emitted, checksummed, done) = flate.step(data, sink, grant, self.limit - self.output_at)?;
                self.output_at += emitted;
                result.emitted = emitted;
                result.prepared = checksummed;
                if done { self.flate = None; self.phase = Phase::ObjectEnd; }
                return Ok(result);
            }
            let written = try_write(sink, &data[self.body_at..], grant, self.limit - self.output_at)?;
            self.output_at += written;
            self.body_at += written;
            result.emitted = written;
            if self.body_at == data.len() { self.phase = Phase::ObjectEnd; }
            return Ok(result);
        }
        if matches!(self.phase, Phase::Done) { result.done = true; return Ok(result); }
        if grant < MAX_PREFIX { return Ok(result); }
        self.prepare(graph)?;
        result.prepared = self.buffer.len();
        result.control = true;
        Ok(result)
    }

    fn prepare(&mut self, graph: GraphLoan<'_>) -> Result<(), WriteRefusal> {
        let max_number = graph.object(graph.len() - 1).ok_or(GraphRefusal::ObjectRoster)?.id.num;
        let size = max_number + 1;
        let out = &mut self.buffer;
        self.phase = match self.phase {
            Phase::Header => {
                let version = if graph.version.is_empty() { "1.7" } else { graph.version };
                write!(out, "%PDF-{version}\n%")?;
                out.extend_from_slice(&[0xE2, 0xE3, 0xCF, 0xD3, b'\n']);
                Phase::Object
            }
            Phase::Object => {
                let object = graph.object(self.object).ok_or(GraphRefusal::SourceChanged)?;
                self.offsets[self.object] = self.output_at as u32;
                writeln!(out, "{} {} obj", object.id.num, object.id.gen)?;
                match &object.value {
                    PdfObject::Stream { dict, data, filters } => {
                        out.extend_from_slice(b"<<");
                        let mut first = true;
                        for entry in dict.iter().filter(|entry| entry.key != "Length"
                            && (filters.is_empty() || !matches!(entry.key.as_str(), "Filter" | "DecodeParms")))
                        {
                            append_entry(out, &mut first, &entry.key, &entry.value);
                        }
                        self.flate = is_flate(filters).then(StoredFlate::new);
                        if self.flate.is_some() {
                            append_entry(out, &mut first, "Filter", &PdfObject::name("FlateDecode"));
                        } else if !filters.is_empty() {
                            for entry in encode_stream(&[], filters).1 {
                                append_entry(out, &mut first, &entry.key, &entry.value);
                            }
                        }
                        append_entry(out, &mut first, "Length", &PdfObject::Int(stream_extent(filters, data.len())? as i64));
                        out.extend_from_slice(b">>\nstream\n");
                        self.body_at = 0;
                        Phase::Body
                    }
                    value => {
                        write_object(out, value);
                        out.extend_from_slice(b"\nendobj\n");
                        next_object(&mut self.object, graph.len())
                    }
                }
            }
            Phase::ObjectEnd => {
                out.extend_from_slice(b"\nendstream\nendobj\n");
                next_object(&mut self.object, graph.len())
            }
            Phase::XrefHeader => {
                self.xref_at = self.output_at;
                write!(out, "xref\n0 {size}\n{:010} 65535 f \n", next_free(graph, 1, 0))?;
                self.xref_row = 1;
                Phase::XrefRow
            }
            Phase::XrefRow => {
                let number = self.xref_row;
                if let Some(object) = graph.object(self.xref_object).filter(|object| object.id.num == number) {
                    let offset = self.offsets[self.xref_object];
                    writeln!(out, "{offset:010} {:05} n ", object.id.gen)?;
                    self.xref_object += 1;
                } else {
                    writeln!(out, "{:010} 00000 f ", next_free(graph, number + 1, self.xref_object))?;
                }
                self.xref_row += 1;
                if self.xref_row == size { Phase::Trailer } else { Phase::XrefRow }
            }
            Phase::Trailer => {
                let trailer = graph.trailer;
                out.extend_from_slice(b"trailer\n<</Size ");
                write!(out, "{size}")?;
                let mut first = false;
                append_entry(out, &mut first, "Root", &PdfObject::Ref(trailer.root));
                if let Some(info) = trailer.info { append_entry(out, &mut first, "Info", &PdfObject::Ref(info)); }
                if let Some([first_id, second_id]) = &trailer.id {
                    out.extend_from_slice(b" /ID[");
                    write_string(out, first_id);
                    out.push(b' ');
                    write_string(out, second_id);
                    out.push(b']');
                }
                for entry in trailer.extra.iter().filter(|entry| preserved_trailer(entry)) {
                    append_entry(out, &mut first, &entry.key, &entry.value);
                }
                write!(out, ">>\nstartxref\n{}\n%%EOF\n", self.xref_at)?;
                Phase::Done
            }
            Phase::Body | Phase::Done => unreachable!("prepared graph advancing outside a control turn"),
        };
        if self.buffer.len() > MAX_PREFIX { return Err(GraphRefusal::ControlExtent.into()); }
        Ok(())
    }

    /// ♻️ Transfers the exact same paid scratch owner for the job's original retirement.
    pub fn into_buffer(self) -> Vec<u8> { self.buffer }
}

fn next_object(index: &mut usize, count: usize) -> Phase {
    *index += 1;
    if *index < count { Phase::Object } else { Phase::XrefHeader }
}

fn next_free(graph: GraphLoan<'_>, mut number: u32, first: usize) -> u32 {
    for index in first..graph.len() {
        let Some(object) = graph.object(index) else { return 0; };
        if object.id.num < number { continue; }
        if object.id.num != number { return number; }
        number += 1;
    }
    0
}

pub(crate) fn try_write(sink: &mut dyn Write, bytes: &[u8], grant: usize, remaining: usize) -> Result<usize, WriteRefusal> {
    let count = bytes.len().min(grant).min(4096);
    if count == 0 { return Ok(0); }
    if count > remaining { return Err(GraphRefusal::OutputExtent.into()); }
    let done = sink.write(&bytes[..count])?;
    if done > count { return Err(io::Error::from(io::ErrorKind::InvalidData).into()); }
    Ok(done)
}

fn append_entry(out: &mut Vec<u8>, first: &mut bool, key: &str, value: &PdfObject) {
    if !std::mem::replace(first, false) { out.push(b' '); }
    write_name(out, key);
    if !matches!(value, PdfObject::Name(_) | PdfObject::Array(_) | PdfObject::Dict(_) | PdfObject::Str(_) | PdfObject::Stream { .. }) {
        out.push(b' ');
    }
    if let ("URI", PdfObject::Text(text)) = (key, value) { write_string(out, text.as_bytes()); }
    else { write_object(out, value); }
}
