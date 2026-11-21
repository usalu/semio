//! 🧩️ Binds an exact encoded PNG to durable PDF graph parts and two sampled image roles.

use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfIndirectObject, PdfObject};
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_hash::Sha256;
use semio_framework_value::paged::ByteReadSource;

const TYPE: &str = "SemioPngSourceV1";
pub const PNG_GRAPH_CHUNK_BYTES: usize = 32 * 1024;
pub const PNG_GRAPH_MAXIMUM_BYTES: usize = 512 * 1024;
pub const PNG_GRAPH_MAXIMUM_PIXELS: usize = 262_144;
const PARTS: usize = PNG_GRAPH_MAXIMUM_BYTES / PNG_GRAPH_CHUNK_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PngSourceError { Reference, Extent, Missing, Duplicate, Part, Digest }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PngPlane { Rgb, Alpha }

impl PngPlane {
    fn kind(self) -> &'static str {
        match self { Self::Rgb => "s.stdio.png.rgb", Self::Alpha => "s.stdio.png.alpha" }
    }

    /// 🏷️ Addresses one plane by the complete original PNG digest.
    pub fn reference(self, digest: [u8; 32]) -> ArtifactRef {
        super::artifact_reference(self.kind(), digest)
    }

    pub fn match_reference(reference: &ArtifactRef) -> Option<Self> { Self::from_reference(reference).ok().map(|(role, _)| role) }

    pub fn channels(self) -> usize { match self { Self::Rgb => 3, Self::Alpha => 1 } }

    fn from_reference(reference: &ArtifactRef) -> Result<(Self, [u8; 32]), PngSourceError> {
        let id = reference.artifact_id.strip_prefix("pdf:").ok_or(PngSourceError::Reference)?;
        let (kind, hex) = id.split_once(":sha256:").ok_or(PngSourceError::Reference)?;
        let plane = [Self::Rgb, Self::Alpha].into_iter().find(|plane| plane.kind() == kind).ok_or(PngSourceError::Reference)?;
        let mut digest = [0; 32];
        if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) { return Err(PngSourceError::Reference); }
        for (index, value) in digest.iter_mut().enumerate() {
            *value = u8::from_str_radix(&hex[2 * index..2 * index + 2], 16).map_err(|_| PngSourceError::Reference)?;
        }
        if reference.dialect.artifact_kind != "s.stdio.binary" || reference.dialect.standard != "raw" || reference.dialect.subset != "*" {
            return Err(PngSourceError::Reference);
        }
        Ok((plane, digest))
    }
}

/// 🎚️ Binds an expected exact byte extent to the authenticated original digest.
#[derive(Clone, Copy)]
pub struct PngGraphExtent {
    pub digest: [u8; 32],
    pub length: usize,
}

impl PngGraphExtent {
    pub fn chunks(self) -> Result<usize, PngSourceError> {
        if !(1..=PNG_GRAPH_MAXIMUM_BYTES).contains(&self.length) { return Err(PngSourceError::Extent); }
        Ok(self.length.div_ceil(PNG_GRAPH_CHUNK_BYTES))
    }

    pub fn chunk_len(self, ordinal: usize) -> Result<usize, PngSourceError> {
        if ordinal >= self.chunks()? { return Err(PngSourceError::Part); }
        Ok((self.length - ordinal * PNG_GRAPH_CHUNK_BYTES).min(PNG_GRAPH_CHUNK_BYTES))
    }

    /// 📥️ Moves one encoded window into an ordinary retained native graph dictionary.
    pub fn part(self, ordinal: usize, octets: String) -> Result<PdfObject, PngSourceError> {
        if octets.len() != self.chunk_len(ordinal)? * 2 { return Err(PngSourceError::Part); }
        Ok(PdfObject::Dict(vec![
            PdfDictEntry::new("Type", PdfObject::Name(TYPE.into())),
            PdfDictEntry::new("Digest", PdfObject::Str(self.digest.to_vec())),
            PdfDictEntry::new("Extent", PdfObject::Int(self.length as i64)),
            PdfDictEntry::new("Ordinal", PdfObject::Int(ordinal as i64)),
            PdfDictEntry::new("Octets", PdfObject::Text(octets)),
        ]))
    }
}

/// 🔒️ Identifies the private source parts that the physical writer replaces with lowered XObjects.
pub fn is_png_graph_source(value: &PdfObject) -> bool {
    matches!(value, PdfObject::Dict(entries) if entries.first().is_some_and(|entry|
        entry.key == "Type" && entry.value.as_name() == Some(TYPE)))
}

/// 📃️ Borrows the exact ordinal and hexadecimal octets of one matching private native declaration.
pub fn matching_png_part<'a>(value: &'a PdfObject, digest: &[u8; 32])
    -> Result<Option<(usize, usize, &'a [u8])>, PngSourceError> {
    let PdfObject::Dict(dict) = value else { return Ok(None); };
    if !is_png_graph_source(value) || !dict.get(1).is_some_and(|entry|
        entry.key == "Digest" && matches!(&entry.value, PdfObject::Str(value) if value == digest)) {
        return Ok(None);
    }
    let [_, _, extent, order, data] = dict.as_slice() else { return Err(PngSourceError::Part); };
    if extent.key != "Extent" || order.key != "Ordinal" || data.key != "Octets" { return Err(PngSourceError::Part); }
    let (PdfObject::Int(total), PdfObject::Int(ordinal), PdfObject::Text(data)) = (&extent.value, &order.value, &data.value)
        else { return Err(PngSourceError::Part); };
    let total = usize::try_from(*total).map_err(|_| PngSourceError::Extent)?;
    let ordinal = usize::try_from(*ordinal).map_err(|_| PngSourceError::Part)?;
    if data.len() != (PngGraphExtent { digest: *digest, length: total }).chunk_len(ordinal)? * 2 { return Err(PngSourceError::Part); }
    Ok(Some((total, ordinal, data.as_bytes())))
}

/// ✍️ Appends the canonical ASCII image of an original byte window to its funded backing.
pub fn append_hex(output: &mut String, bytes: &[u8]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for &byte in bytes {
        output.push(char::from(HEX[(byte >> 4) as usize]));
        output.push(char::from(HEX[(byte & 15) as usize]));
    }
}

pub fn hex_octet(pair: &[u8]) -> Option<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        }
    }
    let [high, low] = pair else { return None; };
    Some((digit(*high)? << 4) | digit(*low)?)
}

/// 📖 Validates and borrows each exact native page without joining its source allocation.
pub struct PngGraphSource<'a> {
    chunks: [Option<&'a [u8]>; PARTS],
    length: usize,
}

impl<'a> PngGraphSource<'a> {
    pub fn resolve(objects: &'a [PdfIndirectObject], reference: &ArtifactRef) -> Result<(PngPlane, Self), PngSourceError> {
        let (plane, digest) = PngPlane::from_reference(reference)?;
        let mut length = None;
        let mut chunks: [Option<&[u8]>; PARTS] = [None; PARTS];
        for item in objects {
            let Some((total, ordinal, data)) = matching_png_part(&item.value, &digest)? else { continue; };
            if length.is_some_and(|old| old != total) { return Err(PngSourceError::Part); }
            if chunks[ordinal].replace(data).is_some() { return Err(PngSourceError::Duplicate); }
            length = Some(total);
        }
        let length = length.ok_or(PngSourceError::Missing)?;
        let mut hash = Sha256::new();
        let mut scratch = [0; 4096];
        for chunk in &chunks[..length.div_ceil(PNG_GRAPH_CHUNK_BYTES)] {
            for window in chunk.ok_or(PngSourceError::Missing)?.chunks(scratch.len() * 2) {
                for (out, pair) in scratch.iter_mut().zip(window.chunks_exact(2)) {
                    *out = hex_octet(pair).ok_or(PngSourceError::Part)?;
                }
                hash.update(&scratch[..window.len() / 2]);
            }
        }
        if hash.finalize() != digest { return Err(PngSourceError::Digest); }
        Ok((plane, Self { chunks, length }))
    }
}

/// 🧾 Excludes only native owners whose exact digest is still bound to a typed image.
pub fn referenced_source_object(
    value: &PdfObject, images: &[crate::standards::v1_7::subsets::base::schema::snapshot::PdfImage],
) -> bool {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;
    if !is_png_graph_source(value) { return false; }
    let Some(PdfObject::Str(digest)) = value.dict_get("Digest") else { return false; };
    images.iter().any(|image| {
        let PdfImageBody::Artifact { reference } = &image.body else { return false; };
        PngPlane::from_reference(reference).is_ok_and(|(_, expected)| digest == &expected)
    })
}

impl ByteReadSource for PngGraphSource<'_> {
    fn byte_len(&self) -> usize { self.length }
    fn byte_at(&self, index: usize) -> Option<u8> {
        let start = (index % PNG_GRAPH_CHUNK_BYTES) * 2;
        hex_octet(self.chunks.get(index / PNG_GRAPH_CHUNK_BYTES)?.as_ref()?.get(start..start + 2)?)
    }

    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        let available = self.length.saturating_sub(offset).min(output.len());
        let mut written = 0;
        while written < available {
            let offset = offset + written;
            let chunk = self.chunks[offset / PNG_GRAPH_CHUNK_BYTES].expect("validated PNG graph source part");
            let start = (offset % PNG_GRAPH_CHUNK_BYTES) * 2;
            let count = (available - written).min((chunk.len() - start) / 2);
            for (out, pair) in output[written..written + count].iter_mut().zip(chunk[start..start + count * 2].chunks_exact(2)) {
                *out = hex_octet(pair).expect("validated original PDF PNG hexadecimal pair");
            }
            written += count;
        }
        written
    }
}
