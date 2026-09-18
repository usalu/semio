//! 📦️ WFC 2D artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::{Wfc2dTileMedia, Wfc2dSnapshot};
use store::{ErasedSnapshotRetirement, PackError, SnapshotRetirementStep};

/// 📦️ Encodes a `Wfc2dSnapshot` to its binary pack form.
pub fn encode(document: &Wfc2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Wfc2dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Wfc2dSnapshot, PackError> {
    <Wfc2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 🧮️ One retirement step's budget unit — the bytes one displaced collection is charged, so a caller
/// with a small `maximum_bytes` makes progress across several turns instead of stalling.
const WFC_2D_RETIREMENT_STEP_BYTES: usize = 4_096;

/// ♻️ The four collections a displaced `Wfc2dSnapshot` releases, in release order. `tiles` goes
/// FIRST: a tile's media can carry an inline bitmap payload or an `ArtifactChild` handle, so it is
/// by far the heaviest row and the only one that can outlive the document that named it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wfc2dRetirementStage {
    Tiles,
    Rules,
    Edges,
    Slots,
}

impl Wfc2dRetirementStage {
    const ORDER: [Self; 4] = [Self::Tiles, Self::Rules, Self::Edges, Self::Slots];
}

/// ♻️ A snapshot displaced by a decode-in-place, released in bounded steps rather than dropped in one
/// unbounded `Drop` — the discipline `store::ArtifactStore` asks of every artifact whose decode can
/// overwrite a live projection. `Drop` asserts the terminal state was actually reached, so a caller
/// that abandons a retirement mid-way fails loudly instead of leaking silently.
pub struct Wfc2dSnapshotRetirement {
    displaced: std::mem::ManuallyDrop<Wfc2dSnapshot>,
    stage: usize,
}

impl Wfc2dSnapshotRetirement {
    fn new(displaced: Wfc2dSnapshot) -> Self {
        Self { displaced: std::mem::ManuallyDrop::new(displaced), stage: 0 }
    }
}

impl ErasedSnapshotRetirement for Wfc2dSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.stage >= Wfc2dRetirementStage::ORDER.len() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes < WFC_2D_RETIREMENT_STEP_BYTES {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let released = match Wfc2dRetirementStage::ORDER[self.stage] {
            Wfc2dRetirementStage::Tiles => std::mem::take(&mut self.displaced.tiles).len(),
            Wfc2dRetirementStage::Rules => std::mem::take(&mut self.displaced.rules).len(),
            Wfc2dRetirementStage::Edges => std::mem::take(&mut self.displaced.edges).len(),
            Wfc2dRetirementStage::Slots => std::mem::take(&mut self.displaced.slots).len(),
        };
        self.stage += 1;
        Ok(SnapshotRetirementStep::Pending { released_items: released.max(1).min(maximum_items), released_bytes: WFC_2D_RETIREMENT_STEP_BYTES })
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage >= Wfc2dRetirementStage::ORDER.len()
    }
}

impl Drop for Wfc2dSnapshotRetirement {
    /// ⚠️ `ManuallyDrop` exists so the collections above are released by `close_step`, never by an
    /// unbounded `Drop`; what is left at this point is the emptied husk, and dropping it is O(1).
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "WFC 2D snapshot displacement reached Drop before terminal-empty close");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.displaced) };
    }
}

/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement —
/// the only decode entry point a mounted store may call, because it never drops the old snapshot
/// inline.
pub fn decode_into(target: &mut Wfc2dSnapshot, bytes: &[u8]) -> Result<Box<dyn ErasedSnapshotRetirement>, PackError> {
    let next = decode(bytes)?;
    Ok(Box::new(Wfc2dSnapshotRetirement::new(std::mem::replace(target, next))))
}
//#endregion ♻️Retirement

//#region 🖼️TileMediaRaster
/// 🖼️ Projects a `Bitmap` tile's palette-indexed pixels into a `data:image/png;base64,…` URL — the
/// exact carrier `Canvas2dHost`'s `kind: "image"` layer reads (`layer.dataUrl`, resolved through its
/// own `imageCache`). Both the editor preview and the viewer board paint through this one function,
/// so a bitmap tile really shows its pixels instead of a labelled rectangle.
///
/// It lives in the BINARY io facet rather than in either window because both surfaces need it and a
/// viewer file may never import through the sibling editor module (`policyViewerPurityBreaches`).
/// Encoding goes through `s.stdio.png`'s own encoder — a hand-rolled PNG writer beside it would be a
/// second, drifting implementation of a format this repo already owns.
///
/// Returns `None` for any media that is not a `Bitmap`, for a zero-sized one, and for a payload whose
/// decoded length is not exactly `width * height` — a malformed tile draws its outline rather than
/// failing the whole surface refresh.
pub fn tile_media_png_data_url(media: &Wfc2dTileMedia) -> Option<String> {
    let Wfc2dTileMedia::Bitmap { width, height, palette, pixels } = media else { return None };
    let (width, height) = (*width, *height);
    if width == 0 || height == 0 || palette.is_empty() {
        return None;
    }
    let count = (width as usize).checked_mul(height as usize)?;
    let indices = base64_codec::base64_standard_decode(pixels).ok()?;
    if indices.len() != count {
        return None;
    }
    let mut rgba = Vec::with_capacity(count.checked_mul(4)?);
    for index in indices {
        let colour = palette.get(usize::from(index)).copied().unwrap_or_default();
        rgba.extend_from_slice(&[colour.r.min(255) as u8, colour.g.min(255) as u8, colour.b.min(255) as u8, colour.a.min(255) as u8]);
    }
    let snapshot = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot { width, height, pixels: rgba, ..Default::default() };
    let bytes = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&snapshot).ok()?;
    Some(format!("data:image/png;base64,{}", base64_codec::base64_standard_encode(bytes)))
}
//#endregion 🖼️TileMediaRaster

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type Wfc2dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier
