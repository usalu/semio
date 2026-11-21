//! 🌊️ Emits verified packed 8-bit planes through the existing classic PDF writer.

use super::{ObjRef, PdfDictEntry, PdfObject, PdfWriter, write_dict};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::plane_identity::{PlaneRole, PlaneShape};
use std::io::{self, Write};

const SUFFIX: &[u8] = b"\nendstream\nendobj\n";

pub struct PlaneObjectWriter {
    writer: PdfWriter,
    object: ObjRef,
    prefix: Vec<u8>,
    prefix_offset: usize,
    body_remaining: usize,
    suffix_offset: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaneObjectRefusal {
    Writer, Object, Mask, Extent,
}

impl PdfWriter {
    /// 📐 Begins a clear raw image. The Save planner has already authorized the paired graph roles.
    pub fn begin_plane_object(
        self, object: ObjRef, shape: PlaneShape, role: PlaneRole, mask: Option<ObjRef>,
    ) -> Result<PlaneObjectWriter, (PlaneObjectRefusal, Self)> {
        if self.cipher.is_some() || self.xref_stream { return Err((PlaneObjectRefusal::Writer, self)); }
        if object.num == 0 || object.num == u32::MAX || self.offsets.contains_key(&object.num) {
            return Err((PlaneObjectRefusal::Object, self));
        }
        if mask.is_some() != (role == PlaneRole::Color && shape.has_alpha())
            || mask.is_some_and(|mask| mask.num == 0 || mask.num == u32::MAX || mask.num == object.num)
        {
            return Err((PlaneObjectRefusal::Mask, self));
        }
        let length = shape.len(role);
        if length == 0 { return Err((PlaneObjectRefusal::Extent, self)); }
        let (width, height) = shape.dimensions();
        let space = if role == PlaneRole::Color && shape.components() == 3 { "DeviceRGB" } else { "DeviceGray" };
        let mut entries = vec![
            PdfDictEntry::new("Type", PdfObject::name("XObject")),
            PdfDictEntry::new("Subtype", PdfObject::name("Image")),
            PdfDictEntry::new("Width", PdfObject::Int(i64::from(width))),
            PdfDictEntry::new("Height", PdfObject::Int(i64::from(height))),
            PdfDictEntry::new("BitsPerComponent", PdfObject::Int(8)),
            PdfDictEntry::new("ColorSpace", PdfObject::name(space)),
            PdfDictEntry::new("Length", PdfObject::Int(length as i64)),
        ];
        if let Some(mask) = mask { entries.push(PdfDictEntry::new("SMask", PdfObject::Ref(mask))); }
        let mut prefix = Vec::with_capacity(256);
        writeln!(prefix, "{} {} obj", object.num, object.gen).expect("writing a PDF header into a byte vector");
        write_dict(&mut prefix, &entries);
        prefix.extend_from_slice(b"\nstream\n");
        let end = self.offset.checked_add(prefix.len())
            .and_then(|offset| offset.checked_add(length))
            .and_then(|offset| offset.checked_add(SUFFIX.len()));
        let Some(end) = end.filter(|offset| u32::try_from(*offset).is_ok()) else {
            return Err((PlaneObjectRefusal::Extent, self));
        };
        Ok(PlaneObjectWriter { writer: self, object, prefix, prefix_offset: 0,
            body_remaining: length, suffix_offset: 0, end })
    }
}

impl PlaneObjectWriter {
    pub fn complete(&self) -> bool { self.suffix_offset == SUFFIX.len() }

    /// 🪜️ Emits one bounded slice, returning the actual count taken from the caller's plane loan.
    pub fn step(&mut self, fragment: &[u8], sink: &mut dyn Write, grant: usize) -> io::Result<usize> {
        if self.prefix_offset < self.prefix.len() {
            self.prefix_offset += write(sink, &self.prefix[self.prefix_offset..], grant)?;
            return Ok(0);
        }
        if self.body_remaining != 0 {
            if fragment.len() > self.body_remaining {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "plane exceeds its admitted length"));
            }
            let written = write(sink, fragment, grant)?;
            self.body_remaining -= written;
            return Ok(written);
        }
        if !fragment.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "plane already reached its admitted length"));
        }
        self.suffix_offset += write(sink, &SUFFIX[self.suffix_offset..], grant)?;
        Ok(0)
    }

    /// 🧾️ Records the original generation and exact byte offset only after the entire body and suffix.
    pub fn finish(mut self) -> Result<PdfWriter, Self> {
        if !self.complete() { return Err(self); }
        self.writer.offsets.insert(self.object.num, (self.writer.offset, self.object.gen));
        self.writer.max_number = self.writer.max_number.max(self.object.num);
        self.writer.offset = self.end;
        Ok(self.writer)
    }
}

fn write(sink: &mut dyn Write, data: &[u8], grant: usize) -> io::Result<usize> {
    let count = data.len().min(grant).min(4096);
    if count == 0 { return Ok(0); }
    let written = sink.write(&data[..count])?;
    if written > count {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid PDF plane sink count"));
    }
    Ok(written)
}
