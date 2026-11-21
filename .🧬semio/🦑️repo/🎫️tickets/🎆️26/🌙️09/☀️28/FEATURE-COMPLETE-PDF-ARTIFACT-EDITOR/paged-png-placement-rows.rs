//! 📌️ Authorizes one native paired-plane source, two graph roles, and one PDF paint operation.

use super::{
    plane_identity::{PlaneFacts, PlaneRole, PngPlanes},
    plane_pack::{self, ArchivedPlaneCursor, MAX_ROWS_BYTES, PngPlacementArchive, invalid},
};
use pack::PackRefusal;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_pack_source::{PackSink, PackSource};
use semio_os_spr::OpBinary;
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{
    mutations::{AppendPageContent, PdfMutation, ReplaceContent, SetImage},
    snapshot::{PdfColorSpace, PdfImage, PdfImageBody, PdfOp, PdfSnapshot},
};

const ROW_CAP: usize = 2048;

#[derive(semio_framework_value::RetireOwned)]
pub struct PngPlacementRows {
    revision: [u8; 32],
    alpha: Option<PdfMutation>,
    color: PdfMutation,
    paint: PdfMutation,
}

impl PngPlacementRows {
    /// 🖱️ Stages bounded image rows and paint at the supplied active page revision.
    pub fn prepare(
        revision: [u8; 32], snapshot: &PdfSnapshot, facts: PlaneFacts,
        color_name: &str, mask_name: Option<&str>, paint: PdfMutation,
    ) -> Result<Self, PackRefusal> {
        let fresh = |name: &str| valid_name(name) && snapshot.image(name).is_none() && snapshot.form(name).is_none();
        if !fresh(color_name) || mask_name.is_some_and(|id| !fresh(id) || id == color_name)
            || facts.shape.has_alpha() != mask_name.is_some()
        { return Err(invalid("PNG placement names already exist or mask name is missing")); }
        let color = image(facts, PlaneRole::Color, color_name, mask_name)?;
        let alpha = mask_name.map(|name| image(facts, PlaneRole::Alpha, name, None)).transpose()?;
        let rows = Self { revision, alpha, color, paint };
        rows.validate(facts)?;
        match &rows.paint {
            PdfMutation::AppendPageContent(AppendPageContent { index, .. })
                if snapshot.pages.get(*index).is_some() => {}
            PdfMutation::ReplaceContent(ReplaceContent { index, at, .. })
                if snapshot.pages.get(*index).and_then(|page| page.content.get(*at))
                    .is_some_and(|op| matches!(op, PdfOp::InlineImage { .. }) || matches!(op, PdfOp::PaintXObject { name } if snapshot.image(name).is_some())) => {}
            _ => return Err(invalid("PNG placement target is no longer an image location")),
        }
        Ok(rows)
    }

    pub fn revision(&self) -> &[u8; 32] { &self.revision }

    pub fn iter(&self) -> impl Iterator<Item = &PdfMutation> {
        self.alpha.iter().chain([&self.color, &self.paint])
    }

    pub fn into_mutations(self) -> impl Iterator<Item = PdfMutation> {
        self.alpha.into_iter().chain([self.color, self.paint])
    }

    fn validate(&self, facts: PlaneFacts) -> Result<(), PackRefusal> {
        let color = image_of(&self.color)?;
        let alpha = self.alpha.as_ref().map(image_of).transpose()?;
        validate_graph_pair(facts, color, alpha)?;
        let painted = match &self.paint {
            PdfMutation::AppendPageContent(AppendPageContent { content, .. }) => {
                let [PdfOp::Save, PdfOp::Transform { matrix }, PdfOp::PaintXObject { name }, PdfOp::Restore] = content.as_slice()
                    else { return Err(invalid("PNG insertion needs exactly one qualified placement group")); };
                if !matrix.iter().all(|entry| entry.is_finite()) || matrix[0] <= 0.0 || matrix[3] <= 0.0 {
                    return Err(invalid("PNG placement transform is invalid"));
                }
                name
            }
            PdfMutation::ReplaceContent(ReplaceContent { op: PdfOp::PaintXObject { name }, .. }) => name,
            _ => return Err(invalid("PNG placement paint has a different mutation shape")),
        };
        if painted != &color.id { return Err(invalid("paint names a different color plane")); }
        Ok(())
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, PackRefusal> {
        let mut output = self.revision.to_vec();
        for row in self.iter() {
            let encoded = row.encode_op().map_err(|_| invalid("registered PDF encoder refused a PNG placement row"))?;
            if encoded.is_empty() || encoded.len() > ROW_CAP { return Err(invalid("PNG placement row over cap")); }
            output.extend_from_slice(&(encoded.len() as u16).to_le_bytes());
            output.extend_from_slice(&encoded);
        }
        Ok(output)
    }

    fn decode(mut input: &[u8], facts: PlaneFacts) -> Result<Self, PackRefusal> {
        let (revision, tail) = input.split_at_checked(32)
            .ok_or_else(|| invalid("PNG placement has no canonical base revision"))?;
        input = tail;
        let mut next = || {
            let (prefix, payload) = input.split_at_checked(2).ok_or_else(|| invalid("missing PNG placement row length"))?;
            let count = u16::from_le_bytes(prefix.try_into().expect("exact PNG row size")) as usize;
            if count == 0 || count > ROW_CAP { return Err(invalid("PNG placement operation extent is invalid")); }
            let (bytes, rest) = payload.split_at_checked(count).ok_or_else(|| invalid("PNG placement row is truncated"))?;
            input = rest;
            PdfMutation::decode_op(bytes).map_err(|_| invalid("registered PDF decoder refused a PNG placement row"))
        };
        let alpha = if facts.shape.has_alpha() { Some(next()?) } else { None };
        let color = next()?;
        let paint = next()?;
        if !input.is_empty() { return Err(invalid("PNG placement has trailing operation data")); }
        let rows = Self { revision: revision.try_into().expect("exact PNG revision"), alpha, color, paint };
        rows.validate(facts)?;
        Ok(rows)
    }
}

fn image_of(row: &PdfMutation) -> Result<&PdfImage, PackRefusal> {
    match row {
        PdfMutation::SetImage(SetImage { image, index: None }) => Ok(image),
        _ => Err(invalid("paired roles must be exact SetImage rows")),
    }
}

fn image(facts: PlaneFacts, role: PlaneRole, id: &str, mask: Option<&str>) -> Result<PdfMutation, PackRefusal> {
    let (width, height) = facts.shape.dimensions();
    let reference = facts.reference(role).ok_or_else(|| invalid("missing paired image role"))?;
    let image = PdfImage {
        id: id.into(), width, height, bits_per_component: 8, color_space: Some(space(facts, role)),
        body: PdfImageBody::Artifact { reference }, soft_mask: mask.map(str::to_owned),
        image_mask: false, interpolate: false, decode: Vec::new(), soft_mask_in_data: None,
        mask: None, matte: None, intent: None, optional_content: None, struct_parent: None, extra: Vec::new(),
    };
    Ok(PdfMutation::SetImage(SetImage { image, index: None }))
}

fn space(facts: PlaneFacts, role: PlaneRole) -> PdfColorSpace {
    if facts.shape.components() == 3 && role == PlaneRole::Color { PdfColorSpace::DeviceRgb } else { PdfColorSpace::DeviceGray }
}

fn valid_name(name: &str) -> bool { (1..=128).contains(&name.len()) && name.is_ascii() }

fn validate_graph_pair(facts: PlaneFacts, color: &PdfImage, alpha: Option<&PdfImage>) -> Result<(), PackRefusal> {
    let role_matches = |image: &PdfImage, role| {
        (image.width, image.height) == facts.shape.dimensions()
            && image.bits_per_component == 8 && image.color_space.as_ref() == Some(&space(facts, role))
            && valid_name(&image.id)
            && !image.image_mask && !image.interpolate && image.decode.is_empty() && image.soft_mask_in_data.is_none()
            && image.mask.is_none() && image.matte.is_none() && image.intent.is_none() && image.optional_content.is_none()
            && image.struct_parent.is_none() && image.extra.is_empty()
            && matches!(&image.body, PdfImageBody::Artifact { reference } if facts.matches(role, reference))
    };
    if !role_matches(color, PlaneRole::Color) || facts.shape.has_alpha() != alpha.is_some()
        || color.soft_mask.as_deref() != alpha.map(|mask| mask.id.as_str())
        || alpha.is_some_and(|mask| mask.id == color.id || mask.soft_mask.is_some() || !role_matches(mask, PlaneRole::Alpha))
    { return Err(invalid("PNG color and soft mask do not belong to one admitted plane pair")); }
    Ok(())
}

/// 💾️ Publishes no Pack without checking both exact sample streams and all typed rows.
pub async fn seal_png_placement<S: PackSink>(
    sink: S, facts: PlaneFacts, planes: PngPlanes<'_>, rows: &PngPlacementRows,
    checkpoint: impl std::ops::AsyncFnMut(usize) -> Result<(), S::Error>,
) -> Result<S, S::Error> {
    rows.validate(facts)?;
    let encoded = rows.encode()?;
    debug_assert!(encoded.len() <= MAX_ROWS_BYTES);
    plane_pack::seal(sink, planes, facts, &encoded, checkpoint).await
}

pub struct PngPlacement<S: PackSource> {
    archive: PngPlacementArchive<S>,
    rows: PngPlacementRows,
}

impl<S: PackSource> PngPlacement<S> {
    /// 📂️ Reopens the color resource and verifies exact mask and typed row agreement.
    pub async fn open(source: S, color: &ArtifactRef) -> Result<Self, S::Error> {
        let archive = PngPlacementArchive::open(source, color).await?;
        let rows = PngPlacementRows::decode(&archive.operation().await?, archive.facts())?;
        Ok(Self { archive, rows })
    }

    pub fn facts(&self) -> PlaneFacts { self.archive.facts() }

    pub fn planes(&self) -> ArchivedPlaneCursor<'_, S> { self.archive.planes() }

    /// 🔏️ Lends apply rows only after the payment host verifies the same original Pack and revision.
    pub fn authorize_insertion(
        &self, revision: [u8; 32], proof: &ArchivedPlaneCursor<'_, S>,
    ) -> Result<impl Iterator<Item = &PdfMutation>, PackRefusal> {
        if revision != self.rows.revision || !proof.verified_for(&self.archive) {
            return Err(invalid("stale revision or unverified original placement source"));
        }
        Ok(self.rows.iter())
    }

    /// 🧾️ Authorizes current logical Save roles against a verified immutable install source.
    pub fn authorize_save<'a>(
        &'a self, color: &'a PdfImage, mask: Option<&'a PdfImage>, proof: &ArchivedPlaneCursor<'_, S>,
    ) -> Result<AuthorizedPngSave<'a, S>, PackRefusal> {
        if !proof.verified_for(&self.archive) { return Err(invalid("PNG Save source is unverified")); }
        validate_graph_pair(self.facts(), color, mask)?;
        Ok(AuthorizedPngSave { archive: &self.archive, color, mask })
    }
}

/// 🗝️ Binds Save graph roles to the same persistent plane source for the exact sequential handoff.
pub struct AuthorizedPngSave<'a, S: PackSource> {
    archive: &'a PngPlacementArchive<S>,
    color: &'a PdfImage,
    mask: Option<&'a PdfImage>,
}

impl<S: PackSource> AuthorizedPngSave<'_, S> {
    pub fn images(&self) -> (&PdfImage, Option<&PdfImage>) { (self.color, self.mask) }
    pub fn facts(&self) -> PlaneFacts { self.archive.facts() }
    pub fn planes(&self) -> ArchivedPlaneCursor<'_, S> { self.archive.planes() }
}
