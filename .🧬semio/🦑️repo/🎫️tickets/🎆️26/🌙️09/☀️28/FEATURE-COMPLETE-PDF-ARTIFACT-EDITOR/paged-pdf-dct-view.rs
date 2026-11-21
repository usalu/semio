//! 📖️ Lends an already admitted native DCT image and its graph identity to PDF consumers.
use super::PdfDctResourceIdentity;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::paged::ByteReadSource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfDctImageColor {
    Gray { transform: Option<bool> },
    Rgb { transform: Option<bool> },
}

impl PdfDctImageColor {
    /// 🎛️ Gives the exactly preserved typed DCT filter parameter.
    pub fn color_transform(self) -> Option<u32> {
        match self {
            Self::Gray { transform } | Self::Rgb { transform } => transform.map(u32::from),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PdfDctImageView<'a> {
    source: &'a dyn ByteReadSource,
    width: u16,
    height: u16,
    color: PdfDctImageColor,
    digest: [u8; 32],
}

impl<'a> PdfDctImageView<'a> {
    /// 🔑️ Lends exact encoded custody after the caller has admitted the native stream digest.
    pub fn from_admitted(
        source: &'a dyn ByteReadSource,
        dimensions: (u16, u16),
        color: PdfDctImageColor,
        digest: [u8; 32],
    ) -> Option<Self> {
        let (width, height) = dimensions;
        (width != 0 && height != 0 && source.byte_len() >= 4)
            .then_some(Self { source, width, height, color, digest })
    }

    /// 📐️ Gives the admitted nonzero image dimensions.
    pub fn dimensions(self) -> (u16, u16) { (self.width, self.height) }

    /// 🎨️ Gives the verified colorspace and DCT transform.
    pub fn color(self) -> PdfDctImageColor { self.color }

    /// 🪪️ Gives the fixed-size graph key without rehashing the encoded body.
    pub fn digest(self) -> [u8; 32] { self.digest }

    /// 📚️ Borrows the exact native compressed bytes over the resource's lifetime.
    pub fn encoded(self) -> &'a dyn ByteReadSource { self.source }

    /// 🧾️ Projects the existing native DCT artifact reference without reading the stream.
    pub fn reference(self) -> ArtifactRef { PdfDctResourceIdentity::reference_for_digest(self.digest) }
}
