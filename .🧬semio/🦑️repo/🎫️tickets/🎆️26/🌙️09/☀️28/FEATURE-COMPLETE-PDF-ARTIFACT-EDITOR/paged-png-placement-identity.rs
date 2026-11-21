//! 🖼️ Authenticates the paired straight PDF color and soft-mask planes as one resource.

use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_hash::Sha256;
use semio_framework_value::paged::{ByteReadSource, PAGED_BYTES_CHUNK_BYTES};

const DOMAIN: &[u8] = b"s.stdio.pdf.png-placement.v1\n";
const KEY: &str = "pdf:s.stdio.png-plane:sha256:";
pub const PLANE_CAPACITY: usize = 4 * 1024 * 1024;
pub const FACT_BYTES: usize = 42;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaneRole {
    Color,
    Alpha,
}

impl PlaneRole {
    fn key(self) -> &'static str {
        match self { Self::Color => ":color", Self::Alpha => ":alpha" }
    }
}

/// 📏️ A bounded 8-bit gray or RGB source, with an optional independent opacity plane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlaneShape {
    width: u32,
    height: u32,
    components: u8,
    alpha: bool,
}

impl PlaneShape {
    pub fn new(width: u32, height: u32, components: u8, alpha: bool) -> Option<Self> {
        if !matches!(components, 1 | 3) || width == 0 || height == 0 { return None; }
        let pixels = u64::from(width) * u64::from(height);
        if pixels > (PLANE_CAPACITY / usize::from(components)) as u64 { return None; }
        Some(Self { width, height, components, alpha })
    }

    pub fn dimensions(self) -> (u32, u32) { (self.width, self.height) }
    pub fn components(self) -> u8 { self.components }
    pub fn has_alpha(self) -> bool { self.alpha }

    pub fn len(self, role: PlaneRole) -> usize {
        let pixels = self.width as usize * self.height as usize;
        match role {
            PlaneRole::Color => pixels * usize::from(self.components),
            PlaneRole::Alpha => if self.alpha { pixels } else { 0 },
        }
    }

    fn encode(self) -> [u8; 10] {
        let mut raw = [0; 10];
        raw[0..4].copy_from_slice(&self.width.to_le_bytes());
        raw[4..8].copy_from_slice(&self.height.to_le_bytes());
        raw[8] = self.components;
        raw[9] = u8::from(self.alpha);
        raw
    }
}

/// 🪪️ The paired digest binds each role to dimensions, component count and the other role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlaneFacts {
    pub shape: PlaneShape,
    pub digest: [u8; 32],
}

impl PlaneFacts {
    pub fn encode(self) -> [u8; FACT_BYTES] {
        let mut raw = [0; FACT_BYTES];
        raw[..10].copy_from_slice(&self.shape.encode());
        raw[10..].copy_from_slice(&self.digest);
        raw
    }

    pub fn decode(raw: [u8; FACT_BYTES]) -> Option<Self> {
        let width = u32::from_le_bytes(raw[0..4].try_into().ok()?);
        let height = u32::from_le_bytes(raw[4..8].try_into().ok()?);
        let alpha = match raw[9] { 0 => false, 1 => true, _ => return None };
        let shape = PlaneShape::new(width, height, raw[8], alpha)?;
        Some(Self { shape, digest: raw[10..].try_into().ok()? })
    }

    pub fn reference(self, role: PlaneRole) -> Option<ArtifactRef> {
        if self.shape.len(role) == 0 { return None; }
        let mut id = String::with_capacity(KEY.len() + 64 + role.key().len());
        id.push_str(KEY);
        for b in self.digest {
            const HEX: &[u8] = b"0123456789abcdef";
            id.push(HEX[usize::from(b >> 4)] as char);
            id.push(HEX[usize::from(b & 15)] as char);
        }
        id.push_str(role.key());
        Some(ArtifactRef { artifact_id: id, dialect: ArtifactDialect { artifact_kind: "s.stdio.binary".into(), standard: "raw".into(), subset: "*".into() } })
    }

    pub fn matches(self, role: PlaneRole, reference: &ArtifactRef) -> bool {
        if self.shape.len(role) == 0
            || reference.dialect.artifact_kind != "s.stdio.binary"
            || reference.dialect.standard != "raw" || reference.dialect.subset != "*"
        { return false; }
        let Some(encoded) = reference.artifact_id.strip_prefix(KEY).and_then(|id| id.strip_suffix(role.key())) else { return false; };
        let nibble = |byte| match byte { b'0'..=b'9' => Some(byte - b'0'), b'a'..=b'f' => Some(byte - b'a' + 10), _ => None };
        encoded.len() == 64 && encoded.as_bytes().chunks_exact(2).zip(self.digest).all(|(digits, byte)|
            nibble(digits[0]).zip(nibble(digits[1])).is_some_and(|(hi, lo)| hi << 4 | lo == byte))
    }
}

/// 🔗️ Borrows both immutable output planes through the same admitted decoder owner.
#[derive(Clone, Copy)]
pub struct PngPlanes<'a> {
    shape: PlaneShape,
    color: &'a dyn ByteReadSource,
    alpha: &'a dyn ByteReadSource,
}

impl<'a> PngPlanes<'a> {
    pub fn new(shape: PlaneShape, color: &'a dyn ByteReadSource, alpha: &'a dyn ByteReadSource) -> Option<Self> {
        (color.byte_len() == shape.len(PlaneRole::Color) && alpha.byte_len() == shape.len(PlaneRole::Alpha))
            .then_some(Self { shape, color, alpha })
    }

    pub fn shape(self) -> PlaneShape { self.shape }

    pub fn loan(self, role: PlaneRole) -> &'a dyn ByteReadSource {
        match role { PlaneRole::Color => self.color, PlaneRole::Alpha => self.alpha }
    }
}

/// 🔐️ Hashes a canonical pair from page prefixes and refuses jointly inconsistent extents.
pub struct PlaneIdentity {
    shape: PlaneShape,
    offset: usize,
    hash: Sha256,
}

impl PlaneIdentity {
    pub fn new(shape: PlaneShape) -> Self {
        let mut hash = Sha256::new();
        hash.update(DOMAIN);
        hash.update(&shape.encode());
        Self { shape, offset: 0, hash }
    }

    pub fn shape(&self) -> PlaneShape { self.shape }

    pub fn position(&self) -> Option<(PlaneRole, usize)> {
        let color = self.shape.len(PlaneRole::Color);
        if self.offset < color { return Some((PlaneRole::Color, self.offset)); }
        let alpha_offset = self.offset - color;
        (alpha_offset < self.shape.len(PlaneRole::Alpha)).then_some((PlaneRole::Alpha, alpha_offset))
    }

    pub fn push(&mut self, role: PlaneRole, bytes: &[u8]) -> Option<()> {
        let (expected, offset) = self.position()?;
        if expected != role || bytes.is_empty() || bytes.len() > PAGED_BYTES_CHUNK_BYTES
            || bytes.len() > self.shape.len(role) - offset { return None; }
        self.hash.update(bytes);
        self.offset += bytes.len();
        Some(())
    }

    pub fn read_one(&mut self, planes: PngPlanes<'_>, scratch: &mut [u8], allowance: usize) -> Option<usize> {
        if planes.shape() != self.shape { return None; }
        let Some((role, offset)) = self.position() else { return Some(0); };
        let count = scratch.len().min(allowance).min(PAGED_BYTES_CHUNK_BYTES).min(self.shape.len(role) - offset);
        if count == 0 { return Some(0); }
        if planes.loan(role).read_bytes(offset, &mut scratch[..count]) != count { return None; }
        self.push(role, &scratch[..count])?;
        Some(count)
    }

    pub fn finish(self) -> Option<PlaneFacts> {
        self.position().is_none().then(|| PlaneFacts { shape: self.shape, digest: self.hash.finalize() })
    }
}
