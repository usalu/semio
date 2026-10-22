//! 🎚️ `update-link` — replaces the print size, resolution, and color profile of one image link.

#[cfg(test)] use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutLinksDelta;
use semio_s_artifact_layout_layout_diff::LayoutLinksModification;
use semio_s_artifact_layout_layout_diff::{ImageLinkPatch, LayoutDiff};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

const MAX_LINK_DIMENSION: u32 = 2_048;
const MAX_LINK_PIXELS: u64 = 4_194_304;
const MAX_LINK_DPI: u32 = 9_600;

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct UpdateLink {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    pub color_profile: Option<String>,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for UpdateLink where Op: protocol::Mutation<LayoutSnapshot, Diff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_link::UpdateLink> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "update", entity: "link", kind: "update-link", record: "UpdatedLink" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native(&format!("Update link \"{}\"", self.id), &format!("Verknüpfung \"{}\" aktualisieren", self.id)) }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for UpdateLink where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + From<crate::update_link::UpdateLink> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_update_link(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({ inverse_update_link(self, base)? 
    })
}
}

fn link_ok(payload: &UpdateLink) -> bool {
    payload.width > 0
        && payload.height > 0
        && payload.width <= MAX_LINK_DIMENSION
        && payload.height <= MAX_LINK_DIMENSION
        && u64::from(payload.width) * u64::from(payload.height) <= MAX_LINK_PIXELS
        && payload.dpi > 0
        && payload.dpi <= MAX_LINK_DPI
        && payload.color_profile.as_ref().map(|value| !value.is_empty() && value.len() <= 64).unwrap_or(true)
}

pub fn diff_update_link(payload: &UpdateLink, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let Some(link) = base.links.iter().find(|link| link.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Link \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !link_ok(payload) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A link keeps a positive size within the print envelope, a resolution from 1 to 9600 dpi, and a short color profile.", std::iter::empty::<String>());
    }
    if link.width == payload.width && link.height == payload.height && link.dpi == payload.dpi && link.color_profile == payload.color_profile {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Link print details are already set to that value.");
    }
    protocol::MutationOutcome::new(LayoutDiff {
        links: Some(LayoutLinksDelta { modified: vec![LayoutLinksModification { id: payload.id.clone(), patch: ImageLinkPatch { width: Some(payload.width), height: Some(payload.height), dpi: Some(payload.dpi), color_profile: Some(payload.color_profile.clone().unwrap_or_default()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}

pub fn inverse_update_link<Op: From<crate::update_link::UpdateLink>>(payload: &UpdateLink, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(link) = base.links.iter().find(|link| link.id == payload.id) else { return Vec::new() };
    vec![Op::from(UpdateLink { id: link.id.clone(), width: link.width, height: link.height, dpi: link.dpi, color_profile: link.color_profile.clone() })]

    })())
}

