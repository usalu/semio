//! 🧾️ Seals one native JPEG payload with the registered PDF image and paint mutation rows.
use super::{
    dct_view::{PdfDctImageColor, PdfDctImageView},
    install_pack::{write_dct_install_pack, NativeDctInstallPack},
};
use pack::PackRefusal;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_pack_source::{PackSink, PackSource};
use semio_framework_value::ValueRefusalKind;
use semio_os_spr::OpBinary;
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::{AppendPageContent, PdfMutation, ReplaceContent, SetImage},
    snapshot::{PdfColorSpace, PdfImageBody, PdfOp},
};

pub const DCT_INSTALL_PLACEMENT_TAG: u64 = 61;
const MAX_OP_BYTES: usize = 4096;

fn invalid(detail: &str) -> PackRefusal {
    PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "PDF native DCT placement", offset: 0, detail: detail.into() }
}

/// 🔒️ One private transaction contains the immutable source, exact base revision and typed rows.
pub struct PdfDctPlacementRows {
    revision: [u8; 32],
    mutations: [PdfMutation; 2],
}

impl PdfDctPlacementRows {
    /// 📌️ Checks the registered direct image and insert or replace operation shapes together.
    pub fn prepare(revision: [u8; 32], mutations: [PdfMutation; 2], image: PdfDctImageView<'_>) -> Result<Self, PackRefusal> {
        let resource = image.reference();
        let (width, height) = image.dimensions();
        validate(&mutations, &resource, (width, height), image.color())?;
        Ok(Self { revision, mutations })
    }

    /// 🪪️ Supplies the same checked typed rows to the store under the original canonical revision.
    pub fn mutations(&self, revision: [u8; 32]) -> Result<&[PdfMutation; 2], PackRefusal> {
        if self.revision != revision { return Err(invalid("placement base revision changed")); }
        Ok(&self.mutations)
    }

    fn encode(&self) -> Result<Vec<u8>, PackRefusal> {
        let mut output = self.revision.to_vec();
        for mutation in &self.mutations {
            let bytes = mutation.encode_op().map_err(|_| invalid("registered PDF mutation encoder refused"))?;
            if bytes.is_empty() || bytes.len() > MAX_OP_BYTES { return Err(invalid("placement mutation is oversized")); }
            output.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
            output.extend_from_slice(&bytes);
        }
        Ok(output)
    }

    fn decode(mut data: &[u8]) -> Result<Self, PackRefusal> {
        let revision = data.get(..32).ok_or_else(|| invalid("missing placement revision"))?
            .try_into().expect("checked placement revision");
        data = &data[32..];
        let mut next = || {
            let prefix = data.get(..2).ok_or_else(|| invalid("missing mutation length"))?;
            let count = usize::from(u16::from_le_bytes(prefix.try_into().expect("checked mutation length")));
            if count == 0 || count > MAX_OP_BYTES { return Err(invalid("placement mutation extent is invalid")); }
            let (body, tail) = data[2..].split_at_checked(count).ok_or_else(|| invalid("placement mutation is truncated"))?;
            let mutation = PdfMutation::decode_op(body).map_err(|_| invalid("registered PDF mutation decoder refused"))?;
            data = tail;
            Ok(mutation)
        };
        let mutations = [next()?, next()?];
        if !data.is_empty() { return Err(invalid("placement record contains trailing bytes")); }
        Ok(Self { revision, mutations })
    }
}

fn validate(
    mutations: &[PdfMutation; 2],
    reference: &ArtifactRef,
    dimensions: (u16, u16),
    color: PdfDctImageColor,
) -> Result<(), PackRefusal> {
    let [PdfMutation::SetImage(SetImage { image, index: None }), paint] = mutations else {
        return Err(invalid("placement must publish a direct SetImage and one paint change"));
    };
    let space = match color {
        PdfDctImageColor::Gray { .. } => PdfColorSpace::DeviceGray,
        PdfDctImageColor::Rgb { .. } => PdfColorSpace::DeviceRgb,
    };
    if image.width != u32::from(dimensions.0) || image.height != u32::from(dimensions.1)
        || image.color_space.as_ref() != Some(&space) || image.bits_per_component != 8
        || image.id.len() > 128 || image.id.is_empty() || !image.id.is_ascii()
        || image.image_mask || image.interpolate || !image.decode.is_empty()
        || image.soft_mask.is_some() || image.soft_mask_in_data.is_some() || image.mask.is_some()
        || image.matte.is_some() || image.intent.is_some() || image.optional_content.is_some()
        || image.struct_parent.is_some() || !image.extra.is_empty()
        || !matches!(&image.body, PdfImageBody::Artifact { reference: authored } if authored == reference)
    {
        return Err(invalid("placed image differs from the admitted DCT source"));
    }
    let name = match paint {
        PdfMutation::AppendPageContent(AppendPageContent { content, .. }) => {
            let [PdfOp::Save, PdfOp::Transform { matrix }, PdfOp::PaintXObject { name }, PdfOp::Restore] = content.as_slice()
                else { return Err(invalid("insert paint needs its authored transform group")); };
            if !matrix.iter().all(|value| value.is_finite()) || matrix[0] <= 0.0 || matrix[3] <= 0.0 {
                return Err(invalid("insert paint transform is invalid"));
            }
            name
        }
        PdfMutation::ReplaceContent(ReplaceContent { op: PdfOp::PaintXObject { name }, .. }) => name,
        _ => return Err(invalid("unsupported DCT paint operation")),
    };
    if name != &image.id { return Err(invalid("paint and native resource name differ")); }
    Ok(())
}

/// 💾️ Emits one Pack containing both the original JPEG and its exactly paired operations.
pub async fn seal_dct_placement<S: PackSink>(
    sink: S,
    rows: &PdfDctPlacementRows,
    view: PdfDctImageView<'_>,
    checkpoint: impl std::ops::AsyncFnMut(usize) -> Result<(), S::Error>,
) -> Result<S, S::Error> {
    validate(&rows.mutations, &view.reference(), view.dimensions(), view.color())?;
    write_dct_install_pack(sink, view, &rows.encode()?, checkpoint).await
}

/// 🛬️ Reopens the same sealed source and verifies agreement of the typed and native lanes.
pub async fn reopen_dct_placement<S: PackSource>(
    source: S,
    reference: &ArtifactRef,
) -> Result<(NativeDctInstallPack<S>, PdfDctPlacementRows), S::Error> {
    let install = NativeDctInstallPack::open(source, reference).await?;
    let encoded = install.placement_record().await?;
    let rows = PdfDctPlacementRows::decode(&encoded)?;
    let facts = install.facts();
    validate(&rows.mutations, reference, facts.dimensions, facts.color)?;
    Ok((install, rows))
}
