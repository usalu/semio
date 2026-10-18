//! 🔀️ `mirror-elements` payload. Mirrors placed elements about the line through two points: axes, boundaries and paths are reflected, a wall runs the other way so that its interior face stays on its left (its end join preferences trade places), column rotations, slab falls and roof ridges are reflected, a turning stair swaps its hand, and the openings of a mirrored wall take their offset from the other end with the other hand. With a prefix the mirror images are created as copies and the originals stay.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Point2}};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MirrorElements {
    pub ids: Vec<String>,
    pub line_start: Point2,
    pub line_end: Point2,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for MirrorElements
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<MirrorElements> + From<crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements> + From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements> + From<crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "elements", kind: "mirror-elements", record: "MirrorElements" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::mirror_elements::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::mirror_elements::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Mirror {} element(s) about the line through ({}, {}) and ({}, {})", self.ids.len(), self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y), &format!("{} Element(e) an der Linie durch ({}, {}) und ({}, {}) spiegeln", self.ids.len(), self.line_start.x, self.line_start.y, self.line_end.x, self.line_end.y))
    }
    fn target(&self) -> Vec<String> {
        self.ids.clone()
    }
}
