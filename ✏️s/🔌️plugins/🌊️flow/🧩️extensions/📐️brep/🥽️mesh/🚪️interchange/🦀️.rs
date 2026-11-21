//! 📼️ Binary mesh interchange inside the Flow mesh job.
use flow_extension_sdk::mesh::decode_base64;
use neural_engine::EvalError;
use semio_framework_mesh_engine::{MeshData, io::binary::{BinaryStlReader, BinaryStlWriter, glb_document::GlbDocumentReader, stream::GlbCursor}};
use super::{LIMIT, invalid};
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneProgress};

/// 🗃️ Binary mesh formats use typed text channels carrying standard base64.
#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
pub enum BinaryMeshFormat { Stl, Glb }

impl BinaryMeshFormat {
    pub fn port(self) -> &'static str {
        match self { Self::Stl => "stl", Self::Glb => "glb" }
    }

}

/// 📤️ Streams binary facets or GLB bytes from the tessellated mesh.
pub enum BinaryMeshWriter { Stl(BinaryStlWriter), Glb(GlbCursor) }

impl BinaryMeshWriter {
    pub fn new(format: BinaryMeshFormat, mesh: &MeshData) -> Result<Self, EvalError> {
        if mesh.triangle_count() > LIMIT { return Err(invalid("binary export exceeds 100000 triangles")); }
        match format {
            BinaryMeshFormat::Stl => Ok(Self::Stl(BinaryStlWriter::default())),
            BinaryMeshFormat::Glb if mesh.normals.len() == mesh.positions.len() => {
                GlbCursor::new(mesh).map(Self::Glb).map_err(invalid)
            }
            BinaryMeshFormat::Glb => Err(invalid("GLB export requires tessellated normals")),
        }
    }

    pub fn step(&mut self, mesh: &MeshData, output: &mut Vec<u8>) -> Result<bool, EvalError> {
        match self {
            Self::Stl(writer) => Ok(writer.step(mesh, output, 64)),
            Self::Glb(writer) => {
                let mut buffer = [0; 1024];
                let mut position = 0;
                let mut complete = false;
                for _ in 0..buffer.len() {
                    let copy = writer.next_copy_demand();
                    let fuel = writer.next_work_demand(mesh);
                    let step = writer.advance(mesh, &mut [], &mut buffer[position..], 1, copy, 1, fuel)
                        .map_err(|error| invalid(error.reason))?;
                    position += step.written;
                    complete = step.complete;
                    if complete { break; }
                }
                output.extend_from_slice(&buffer[..position]);
                Ok(complete)
            }
        }
    }
}

/// 🧵️ Captures a bounded base64 chunk per turn before the format codec reads the binary document.
#[derive(semio_framework_value::RetireOwned)]
pub struct BinaryMeshSource {
    format: BinaryMeshFormat,
    offset: usize,
    carry: [u8; 3],
    carry_len: usize,
    padded: bool,
    bytes: Vec<u8>,
    stl: Option<BinaryStlReader>,
    glb: Option<GlbDocumentReader>,
}

impl BinaryMeshSource {
    pub fn new(format: BinaryMeshFormat) -> Self {
        Self { format, offset: 0, carry: [0; 3], carry_len: 0, padded: false, bytes: Vec::new(), stl: None, glb: None }
    }

    pub fn phase(&self) -> &'static str {
        self.glb.as_ref().map_or("mesh-source-binary", GlbDocumentReader::phase)
    }

    pub fn take_retained_progress(&mut self) -> RetainedCloneProgress {
        self.glb.as_mut().map_or_else(Default::default, GlbDocumentReader::take_progress)
    }

    pub fn needs_glb_effect(&self, text: &str) -> bool {
        matches!(self.format, BinaryMeshFormat::Glb) && self.offset == text.len()
            && self.carry_len == 0
    }

    pub fn step_glb(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<MeshData>, EvalError> {
        let reader = self.glb.get_or_insert_with(GlbDocumentReader::default);
        if reader.document_ready() {
            reader.project_step(&self.bytes, LIMIT, effect).map_err(invalid)
        } else {
            reader.parse_step(&self.bytes, effect).map(|_| None).map_err(invalid)
        }
    }

    pub fn step(&mut self, text: &str) -> Result<Option<MeshData>, EvalError> {
        if text.len() > 16_000_000 { return Err(invalid("binary mesh input exceeds 16 MB of text")); }
        if self.offset == text.len() {
            if self.carry_len != 0 { return Err(invalid("base64 mesh input has an incomplete final group")); }
            return match self.format {
                BinaryMeshFormat::Stl => {
                    if self.stl.is_none() {
                        self.stl = Some(BinaryStlReader::new(&self.bytes, LIMIT).map_err(invalid)?);
                    }
                    self.stl.as_mut().unwrap().step(&self.bytes, 64).map_err(invalid)
                }
                BinaryMeshFormat::Glb => Err(invalid("GLB requires an original decode turn")),
            };
        }
        let mut chunk = [0u8; 4099];
        chunk[..self.carry_len].copy_from_slice(&self.carry[..self.carry_len]);
        let end = self.offset.saturating_add(4096).min(text.len());
        let mut length = self.carry_len;
        for &byte in &text.as_bytes()[self.offset..end] {
            if byte.is_ascii_whitespace() { continue; }
            if self.padded { return Err(invalid("base64 mesh input continues after padding")); }
            chunk[length] = byte;
            length += 1;
        }
        self.offset = end;
        let remainder = length % 4;
        let boundary = length - remainder;
        self.carry[..remainder].copy_from_slice(&chunk[boundary..length]);
        self.carry_len = remainder;
        if boundary != 0 {
            let block = std::str::from_utf8(&chunk[..boundary]).map_err(|_| invalid("base64 mesh input must use ASCII text"))?;
            if let Some(padding) = block.find('=') {
                if remainder != 0 || padding < boundary - 2 || !block[padding..].bytes().all(|byte| byte == b'=') {
                    return Err(invalid("base64 mesh input contains invalid padding"));
                }
                self.padded = true;
            }
            self.bytes.extend(decode_base64(block)?);
        }
        Ok(None)
    }
}
