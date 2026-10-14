//! 🧭️ Fem2d mutation — `MoveSelection` payload + `MutationKind` impl.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode;
use crate::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧭️ One gumball gesture over the selection's geometry, stated RELATIVELY so it replays on any base: every named
/// node and every outline and hole point of every named region is scaled by `(sx, sy)` and rotated by `angle`
/// (radians, counter-clockwise) about the pivot, then offset by `(dx, dy)` — `p' = c + R(angle)·S(sx, sy)·(p − c) + d`.
/// A drag is the pure offset, a rotation the pure angle, a scaling the pure factors; every part stays editable in
/// history. Targets keep their ids, so every element, support and load that names them travels along.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-selection")]
pub struct MoveSelection {
    pub node_ids: Vec<String>,
    pub region_ids: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub dx: f64,
    pub dy: f64,
    pub angle: f64,
    pub sx: f64,
    pub sy: f64,
}

impl MoveSelection {
    /// 📐️ Where the transform carries the model point `(x, y)`.
    pub fn map(&self, x: f64, y: f64) -> (f64, f64) {
        let (sin, cos) = self.angle.sin_cos();
        let (u, v) = ((x - self.pivot_x) * self.sx, (y - self.pivot_y) * self.sy);
        (self.pivot_x + u * cos - v * sin + self.dx, self.pivot_y + u * sin + v * cos + self.dy)
    }

    /// 🫥️ Whether the transform is the identity: no offset, no angle, unit factors.
    pub fn is_identity(&self) -> bool {
        (self.dx, self.dy, self.angle, self.sx, self.sy) == (0.0, 0.0, 0.0, 1.0, 1.0)
    }

    /// 🛡️ The hard-bound breach of this payload, as its `mutation.invariant` message and address: a non-finite number, a factor that is not positive, or a target named twice.
    pub fn breach(&self) -> Option<(String, Vec<String>)> {
        let targets: Vec<String> = self.node_ids.iter().chain(&self.region_ids).cloned().collect();
        if [self.pivot_x, self.pivot_y, self.dx, self.dy, self.angle, self.sx, self.sy].iter().any(|value| !value.is_finite()) {
            return Some(("A move-selection carries a non-finite number.".to_string(), targets));
        }
        if self.sx <= 0.0 || self.sy <= 0.0 {
            return Some((format!("A move-selection needs positive scale factors, got ({}, {}).", self.sx, self.sy), targets));
        }
        let repeated = |ids: &[String]| ids.iter().enumerate().find(|(at, id)| ids[..*at].contains(id)).map(|(_, id)| id.clone());
        repeated(&self.node_ids).or_else(|| repeated(&self.region_ids)).map(|twice| (format!("A move-selection names \"{twice}\" twice."), vec![twice]))
    }

    /// 📍️ `node` carried through the transform, or `None` when it does not move.
    pub fn moved_node(&self, node: &FemNode) -> Option<FemNode> {
        let (x, y) = self.map(node.x, node.y);
        let moved = FemNode { x, y, ..node.clone() };
        (moved != *node).then_some(moved)
    }

    /// 🟩️ `region` carried through the transform, or `None` when it does not move.
    pub fn moved_region(&self, region: &FemRegion) -> Option<FemRegion> {
        let point = |point: &[f64; 2]| {
            let (x, y) = self.map(point[0], point[1]);
            [x, y]
        };
        let moved = FemRegion { outline: region.outline.iter().map(point).collect(), holes: region.holes.iter().map(|hole| hole.iter().map(point).collect()).collect(), ..region.clone() };
        (moved != *region).then_some(moved)
    }
}

/// 🔢️ `value` as history prints it: shortest form, a decimal comma in German.
fn number(value: f64, german: bool) -> String {
    let text = format!("{}", (value * 1_000.0).round() / 1_000.0);
    if german {
        text.replace('.', ",")
    } else {
        text
    }
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for MoveSelection
where
    Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<ReplaceNode> + From<ReplaceRegion>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "selection", kind: "move-selection", record: "MovedSelection" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let (nodes, regions) = (self.node_ids.len(), self.region_ids.len());
        let phrase = |nouns: [(usize, &str, &str); 2], join: &str| {
            let parts: Vec<String> = nouns.iter().filter(|(count, ..)| *count > 0).map(|(count, one, many)| format!("{count} {}", if *count == 1 { one } else { many })).collect();
            if parts.is_empty() {
                format!("0 {}", nouns[0].2)
            } else {
                parts.join(join)
            }
        };
        let english = phrase([(nodes, "node", "nodes"), (regions, "region", "regions")], " and ");
        let german = phrase([(nodes, "Knoten", "Knoten"), (regions, "Bereich", "Bereiche")], " und ");
        let degrees = self.angle.to_degrees();
        match ((self.dx, self.dy) != (0.0, 0.0), self.angle != 0.0, (self.sx, self.sy) != (1.0, 1.0)) {
            (true, false, false) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Move {english} by ({}, {})", number(self.dx, false), number(self.dy, false)), &format!("{german} um ({}; {}) verschieben", number(self.dx, true), number(self.dy, true))),
            (false, true, false) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Rotate {english} by {}°", number(degrees, false)), &format!("{german} um {}° drehen", number(degrees, true))),
            (false, false, true) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Scale {english} by ({}, {})", number(self.sx, false), number(self.sy, false)), &format!("{german} um ({}; {}) skalieren", number(self.sx, true), number(self.sy, true))),
            _ => semio_framework_ui_locale::LocalizedLabel::native(&format!("Transform {english}"), &format!("{german} transformieren")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.node_ids.iter().chain(&self.region_ids).cloned().collect()
    }
}
//#endregion 🔖️Mutation
