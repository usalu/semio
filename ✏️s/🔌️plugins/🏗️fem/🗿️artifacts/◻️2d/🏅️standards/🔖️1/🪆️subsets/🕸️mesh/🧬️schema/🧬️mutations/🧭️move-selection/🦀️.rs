//! 🧭️ Fem2d mutation — `MoveSelection` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::Fem2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧭️ One gumball gesture over the selection's geometry, stated RELATIVELY so it replays on any base: every named
/// node and every outline and hole point of every named region is scaled by `(sx, sy)` and rotated by `angle`
/// (radians, counter-clockwise) about the pivot, then offset by `(dx, dy)` — `p' = c + R(angle)·S(sx, sy)·(p − c) + d`.
/// A drag is the pure offset, a rotation the pure angle, a scaling the pure factors; every part stays editable in
/// history. Targets keep their ids, so every element, support and load that names them travels along.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
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

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for MoveSelection {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "selection", kind: "move-selection", record: "MovedSelection" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
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

//#region 🧪️Laws
/// ⚖️ The laws every committed `move-selection` scenario holds, written once beside the leaf and called by each
/// scenario's `🧪️tests/<scenario>/🦀️.rs` with its committed quintet.
#[cfg(test)]
pub mod laws {
    use crate::standards::v1::subsets::any::schema::diff::Fem2dDiff;
    use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation, Fem2dMutation};
    use crate::Fem2dSnapshot;

    fn snapshot(text: &str) -> Fem2dSnapshot {
        dsl::json::from_json_str(text).expect("snapshot decodes")
    }

    fn mutation(text: &str) -> Fem2dMutation {
        dsl::json::from_json_str(text).expect("mutation decodes")
    }

    fn value(text: &str) -> dsl::DslValue {
        dsl::json::from_json_str(text).expect("committed JSON parses")
    }

    fn applied(base: &Fem2dSnapshot, operation: &Fem2dMutation) -> Fem2dSnapshot {
        let mut snapshot = base.clone();
        apply_fem2d_mutation(&mut snapshot, operation).expect("the leaf applies");
        snapshot
    }

    /// ▶️ `before` → exactly `after`, through exactly the committed delta, bound to this kind's descriptor.
    pub fn forward(before: &str, operation: &str, after: &str, diff: &str) {
        let (base, operation) = (snapshot(before), mutation(operation));
        let produced = applied(&base, &operation);
        assert_eq!(produced, snapshot(after), "the applied state is the committed after-snapshot");
        assert_ne!(produced, base, "a forward vector moves the model");
        assert_eq!(dsl::ToValue::to_value(protocol::Mutation::diff(&operation, &base).diff()), value(diff), "the produced delta is the committed one");
        assert_eq!(<Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&operation).kind, "move-selection");
    }

    /// ↩️ The inverse restores `before` exactly, one whole-record replacement per moved node and region.
    pub fn inverse_restores(before: &str, operation: &str) {
        let (base, operation) = (snapshot(before), mutation(operation));
        let inverse = inverse_fem2d_mutation(&base, &operation);
        let moved = applied(&base, &operation);
        let changed = moved.nodes.iter().filter(|node| !base.nodes.contains(node)).count() + moved.regions.iter().filter(|region| !base.regions.contains(region)).count();
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
        assert_eq!(produced.diff(), &Fem2dDiff::default(), "a refusal carries the empty delta");
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
        assert!(inverse_fem2d_mutation(&base, &operation).is_empty(), "nothing moved, nothing to restore");
    }

    /// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
    pub fn canonical(before: &str, after: &str, operation: &str, diff: Option<&str>) {
        for text in [before, after] {
            assert_eq!(dsl::ToValue::to_value(&snapshot(text)), value(text), "a committed snapshot is canonical");
        }
        assert_eq!(dsl::ToValue::to_value(&mutation(operation)), value(operation), "the committed mutation is canonical");
        if let Some(diff) = diff {
            let decoded: Fem2dDiff = dsl::json::from_json_str(diff).expect("the committed diff decodes");
            assert_eq!(dsl::ToValue::to_value(&decoded), value(diff), "the committed diff is canonical");
        }
    }
}
//#endregion 🧪️Laws
