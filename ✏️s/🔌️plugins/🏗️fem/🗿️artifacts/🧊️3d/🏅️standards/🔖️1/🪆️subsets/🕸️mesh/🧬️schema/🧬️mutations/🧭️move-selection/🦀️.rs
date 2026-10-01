//! 🧭️ Fem3d mutation — `MoveSelection` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemSolid};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧭️ One gumball gesture over the selection's geometry, stated RELATIVELY so it replays on any base: every named
/// node and every footprint point of every named solid is scaled by `(sx, sy, sz)`, rotated by `angle` (radians,
/// right-handed) about the axis `(axisX, axisY, axisZ)` through the pivot, then offset by `(dx, dy, dz)` —
/// `p' = c + R(axis, angle)·S(sx, sy, sz)·(p − c) + d`. A drag is the pure offset, a rotation the pure angle, a scaling
/// the pure factors; every part stays editable in history. A solid follows only a map that keeps its footprint
/// plane (it cannot tip over its own extrusion axis); one that cannot is skipped and reported. Targets keep their ids,
/// so every element, support and load that names them travels along.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "move-selection")]
pub struct MoveSelection {
    pub node_ids: Vec<String>,
    pub solid_ids: Vec<String>,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub pivot_z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub axis_x: f64,
    pub axis_y: f64,
    pub axis_z: f64,
    pub angle: f64,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

impl MoveSelection {
    /// 📐️ Where the transform carries the world point `point`.
    pub fn map(&self, point: [f64; 3]) -> [f64; 3] {
        let pivot = [self.pivot_x, self.pivot_y, self.pivot_z];
        let scaled = [(point[0] - pivot[0]) * self.sx, (point[1] - pivot[1]) * self.sy, (point[2] - pivot[2]) * self.sz];
        let turned = if self.angle == 0.0 { scaled } else { rotated(scaled, self.unit_axis().unwrap_or([0.0, 0.0, 1.0]), self.angle) };
        [pivot[0] + turned[0] + self.dx, pivot[1] + turned[1] + self.dy, pivot[2] + turned[2] + self.dz]
    }

    /// 🧭️ The rotation axis at unit length; `None` for the zero axis.
    pub fn unit_axis(&self) -> Option<[f64; 3]> {
        let length = (self.axis_x * self.axis_x + self.axis_y * self.axis_y + self.axis_z * self.axis_z).sqrt();
        (length > 1e-12).then(|| [self.axis_x / length, self.axis_y / length, self.axis_z / length])
    }

    /// 🫥️ Whether the transform is the identity: no offset, no angle, unit factors.
    pub fn is_identity(&self) -> bool {
        (self.dx, self.dy, self.dz, self.angle, self.sx, self.sy, self.sz) == (0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    }

    /// 🧊️ `solid` re-drawn through [`Self::map`]: every outline and hole point lifted into the world at the solid's
    /// base, mapped and projected back, the extrusion following the base and top. `None` when the map does not keep the
    /// footprint plane or would collapse the height — a solid that cannot follow the transform.
    pub fn map_solid(&self, solid: &FemSolid) -> Option<FemSolid> {
        let lift = |point: &[f64; 2], w: f64| solid.axis.to_world(point[0], point[1], w);
        let mut base: Option<f64> = None;
        let mut project = |point: &[f64; 2]| -> Option<[f64; 2]> {
            let (uv, w) = solid.axis.from_world(self.map(lift(point, solid.base_z)));
            match base {
                Some(known) if (known - w).abs() > 1e-9 => return None,
                None => base = Some(w),
                _ => {}
            }
            Some(uv)
        };
        let outline = solid.outline.iter().map(&mut project).collect::<Option<Vec<_>>>()?;
        let holes = solid.holes.iter().map(|hole| hole.iter().map(&mut project).collect::<Option<Vec<_>>>()).collect::<Option<Vec<_>>>()?;
        let base_z = base?;
        let (_, top) = solid.axis.from_world(self.map(lift(solid.outline.first()?, solid.base_z + solid.height)));
        let height = top - base_z;
        (height > 0.0).then(|| FemSolid { outline, holes, base_z, height, ..solid.clone() })
    }
}

/// 🔄️ Rodrigues' rotation of `point` about the unit `axis` through the origin by `angle` radians.
fn rotated(point: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let (sin, cos) = angle.sin_cos();
    let cross = [axis[1] * point[2] - axis[2] * point[1], axis[2] * point[0] - axis[0] * point[2], axis[0] * point[1] - axis[1] * point[0]];
    let along = (axis[0] * point[0] + axis[1] * point[1] + axis[2] * point[2]) * (1.0 - cos);
    [point[0] * cos + cross[0] * sin + axis[0] * along, point[1] * cos + cross[1] * sin + axis[1] * along, point[2] * cos + cross[2] * sin + axis[2] * along]
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

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for MoveSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "selection", kind: "move-selection", record: "MovedSelection" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let phrase = |nouns: [(usize, &str, &str); 2], join: &str| {
            let parts: Vec<String> = nouns.iter().filter(|(count, ..)| *count > 0).map(|(count, one, many)| format!("{count} {}", if *count == 1 { one } else { many })).collect();
            if parts.is_empty() {
                format!("0 {}", nouns[0].2)
            } else {
                parts.join(join)
            }
        };
        let english = phrase([(self.node_ids.len(), "node", "nodes"), (self.solid_ids.len(), "solid", "solids")], " and ");
        let german = phrase([(self.node_ids.len(), "Knoten", "Knoten"), (self.solid_ids.len(), "Körper", "Körper")], " und ");
        let triple = |values: [f64; 3], german: bool| values.map(|value| number(value, german)).join(if german { "; " } else { ", " });
        let (offset, factors) = ([self.dx, self.dy, self.dz], [self.sx, self.sy, self.sz]);
        match (offset != [0.0; 3], self.angle != 0.0, factors != [1.0; 3]) {
            (true, false, false) => protocol::LocalizedLabel::native(&format!("Move {english} by ({})", triple(offset, false)), &format!("{german} um ({}) verschieben", triple(offset, true))),
            (false, true, false) => protocol::LocalizedLabel::native(&format!("Rotate {english} by {}°", number(self.angle.to_degrees(), false)), &format!("{german} um {}° drehen", number(self.angle.to_degrees(), true))),
            (false, false, true) => protocol::LocalizedLabel::native(&format!("Scale {english} by ({})", triple(factors, false)), &format!("{german} um ({}) skalieren", triple(factors, true))),
            _ => protocol::LocalizedLabel::native(&format!("Transform {english}"), &format!("{german} transformieren")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.node_ids.iter().chain(&self.solid_ids).cloned().collect()
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Laws
/// ⚖️ The laws every committed `move-selection` scenario holds, written once beside the leaf and called by each
/// scenario's `🧪️tests/<scenario>/🦀️.rs` with its committed quintet.
#[cfg(test)]
pub mod laws {
    use crate::standards::v1::subsets::any::schema::diff::Fem3dDiff;
    use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation, Fem3dMutation};
    use crate::Fem3dSnapshot;

    fn snapshot(text: &str) -> Fem3dSnapshot {
        dsl::json::from_json_str(text).expect("snapshot decodes")
    }

    fn mutation(text: &str) -> Fem3dMutation {
        dsl::json::from_json_str(text).expect("mutation decodes")
    }

    fn value(text: &str) -> dsl::DslValue {
        dsl::json::from_json_str(text).expect("committed JSON parses")
    }

    fn applied(base: &Fem3dSnapshot, operation: &Fem3dMutation) -> Fem3dSnapshot {
        let mut snapshot = base.clone();
        apply_fem3d_mutation(&mut snapshot, operation).expect("the leaf applies");
        snapshot
    }

    /// ▶️ `before` → exactly `after`, through exactly the committed delta, bound to this kind's descriptor.
    pub fn forward(before: &str, operation: &str, after: &str, diff: &str) {
        let (base, operation) = (snapshot(before), mutation(operation));
        let produced = applied(&base, &operation);
        assert_eq!(produced, snapshot(after), "the applied state is the committed after-snapshot");
        assert_ne!(produced, base, "a forward vector moves the model");
        assert_eq!(dsl::ToValue::to_value(protocol::Mutation::diff(&operation, &base).diff()), value(diff), "the produced delta is the committed one");
        assert_eq!(<Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&operation).kind, "move-selection");
    }

    /// ↩️ The inverse restores `before` exactly, one whole-record replacement per moved node and solid.
    pub fn inverse_restores(before: &str, operation: &str) {
        let (base, operation) = (snapshot(before), mutation(operation));
        let inverse = inverse_fem3d_mutation(&base, &operation);
        let moved = applied(&base, &operation);
        let changed = moved.nodes.iter().filter(|node| !base.nodes.contains(node)).count() + moved.solids.iter().filter(|solid| !base.solids.contains(solid)).count();
        assert_eq!(inverse.len(), changed, "one inverse step per moved record: {inverse:?}");
        let restored = inverse.iter().fold(moved, |state, step| applied(&state, step));
        assert_eq!(restored, base, "the inverse restores the before-snapshot exactly");
    }

    /// 🎯️ An applied vector's declared diagnostics, in order, are exactly the emitted ones, none an Error or worse.
    pub fn declared_outcome(before: &str, operation: &str, outcome: &str) {
        let declared = value(outcome);
        assert_eq!(declared.get("status").and_then(dsl::DslValue::as_str), Some("applied"));
        let produced = protocol::Mutation::diff(&mutation(operation), &snapshot(before));
        let emitted: Vec<(String, Vec<String>)> = produced.messages().iter().map(|message| (message.code.0.clone(), message.target.clone())).collect();
        let expected: Vec<(String, Vec<String>)> = declared.get("messages").and_then(dsl::DslValue::as_array).expect("declared messages").iter().map(|message| (message.get("code").and_then(dsl::DslValue::as_str).expect("code").to_string(), message.get("target").and_then(dsl::DslValue::as_array).map(|target| target.iter().filter_map(dsl::DslValue::as_str).map(str::to_string).collect()).unwrap_or_default())).collect();
        assert_eq!(emitted, expected, "the emitted diagnostics are the declared ones");
        assert!(produced.messages().iter().all(|message| message.level < protocol::Severity::Error), "an applied vector raises nothing at Error or worse");
    }

    /// ⛔️ A refused or no-op vector leaves the document byte-identical behind an empty delta, raises exactly the
    /// declared diagnostic at its level and address, and inverts to nothing.
    pub fn refusal(before: &str, operation: &str, after: &str, outcome: &str) {
        assert_eq!(before, after, "a refusal changes nothing: the committed snapshots are byte-identical");
        let (base, operation) = (snapshot(before), mutation(operation));
        assert_eq!(applied(&base, &operation), base);
        let produced = protocol::Mutation::diff(&operation, &base);
        assert_eq!(produced.diff(), &Fem3dDiff::default(), "a refusal carries the empty delta");
        let declared = value(outcome);
        let messages = produced.messages();
        assert_eq!(messages.len(), 1, "exactly one diagnostic: {messages:?}");
        match declared.get("status").and_then(dsl::DslValue::as_str) {
            Some("rejected") => {
                assert_eq!(declared.get("code").and_then(dsl::DslValue::as_str), Some(messages[0].code.0.as_str()));
                assert!(messages[0].level >= protocol::Severity::Error, "a rejection is at least an Error");
                let path: Vec<String> = declared.get("path").and_then(dsl::DslValue::as_array).expect("path").iter().filter_map(dsl::DslValue::as_str).map(str::to_string).collect();
                assert_eq!(path, messages[0].target, "the declared address is the emitted one");
            }
            Some("no-op") => {
                assert_eq!(messages[0].code.0, "mutation.no-op");
                assert_eq!(messages[0].level, protocol::Severity::Warning);
            }
            other => panic!("a refusal vector declares rejected or no-op, not {other:?}"),
        }
        assert!(inverse_fem3d_mutation(&base, &operation).is_empty(), "nothing moved, nothing to restore");
    }

    /// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
    pub fn canonical(before: &str, after: &str, operation: &str, diff: Option<&str>) {
        for text in [before, after] {
            assert_eq!(dsl::ToValue::to_value(&snapshot(text)), value(text), "a committed snapshot is canonical");
        }
        assert_eq!(dsl::ToValue::to_value(&mutation(operation)), value(operation), "the committed mutation is canonical");
        if let Some(diff) = diff {
            let decoded: Fem3dDiff = dsl::json::from_json_str(diff).expect("the committed diff decodes");
            assert_eq!(dsl::ToValue::to_value(&decoded), value(diff), "the committed diff is canonical");
        }
    }
}
//#endregion 🧪️Laws
