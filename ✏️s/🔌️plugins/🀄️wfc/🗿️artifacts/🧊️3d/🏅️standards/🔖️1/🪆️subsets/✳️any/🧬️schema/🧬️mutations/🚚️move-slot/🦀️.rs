//! 🚚️ `wfc3d` mutation — `MoveSlot`: relocates one slot's box, by moving its MINIMUM corner; the
//! extent is `resize-slot`'s business. A drag gesture coalesces into exactly one of these on release;
//! mid-drag ticks never author a document edit.

use crate::diff::Wfc3dDiff;
use crate::schema::snapshot::Wfc3dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️MoveSlot
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveSlot {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn move_slot<Op>(id: String, x: f64, y: f64, z: f64) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::move_slot::MoveSlot>,
{
    Op::from(MoveSlot { id, x, y, z })
}

impl<Op> MutationKind<Wfc3dSnapshot, Op> for MoveSlot
where
    Op: protocol::Mutation<Wfc3dSnapshot, Diff = Wfc3dDiff> + From<MoveSlot>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "slot", kind: "move-slot", record: "MovedSlot" };

    fn diff(&self, base: &Wfc3dSnapshot) -> protocol::MutationOutcome<Wfc3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move slot \"{}\"", self.id), &format!("Slot \"{}\" verschieben", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️MoveSlot

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{schema::snapshot::Color, schema::snapshot::TileMedia3d};

//#region 🥽️MeshChild
/// 🥽️ Addresses one tile's geometry as this document's child in the `s.stdio.semio@v1/mesh` store —
/// the `TileMedia3d::MeshChild` variant's handle. A tile catalogue small enough to author inline
/// uses `TileMedia3d::Mesh` instead and mints no handle at all.
pub fn mesh_child_handle(tile_id: &str) -> store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot> {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    store::os_store::artifact_child::ArtifactChild::new(tile_id.to_string(), semio_framework_artifact_reference::ArtifactRef { artifact_id: tile_id.to_string(), dialect })
}
/// 📦️ An axis-aligned unit box in tile space (`0..1` on every axis), as inline media. Twelve
/// triangles over eight corners — the honest default body for a tile that fills its slot's box, and
/// what the preview falls back to when a tile's media resolves to nothing.
pub fn unit_box_media(color: Option<Color>) -> TileMedia3d {
    TileMedia3d::Mesh {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0],
        indices: vec![0, 2, 1, 0, 3, 2, 4, 5, 6, 4, 6, 7, 0, 1, 5, 0, 5, 4, 3, 7, 6, 3, 6, 2, 0, 4, 7, 0, 7, 3, 1, 2, 6, 1, 6, 5],
        color,
    }
}

/// 🔺️ A unit wedge in tile space — the box with its `+y`/`+z` edge collapsed, so a roof/ramp tile
/// reads differently from a solid bay at a glance.
pub fn unit_wedge_media(color: Option<Color>) -> TileMedia3d {
    TileMedia3d::Mesh {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0],
        indices: vec![0, 2, 1, 0, 3, 2, 0, 1, 5, 0, 5, 4, 3, 0, 4, 1, 2, 5, 2, 3, 4, 2, 4, 5],
        color,
    }
}
//#endregion 🥽️MeshChild
}
//#endregion 🧩️ArtifactItems3
