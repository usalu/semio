#!/usr/bin/env python3
"""🧬️ W3-T2-TEXT (session 2): writes the four relative trinity rewriting leaves the node-graph and rail gestures yield
(design §13.2/§13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING) — `drag-working-nodes`, `patch-working-nodes`,
`drag-rule-nodes` and its exact one-row undo `set-rule-layout-points` — with every per-leaf surface (Rust payload, diff,
inverse, text/binary identity, descriptor, payload schema with full `x-semio-ui`, TS/GraphQL/protobuf mirrors). Registries
(aggregate, codecs, grammars, oracle catalog, harness KINDS, crate mounts) are patched separately with anchored edits.
Run from the repo root: `python3 <ticket>/🧪️w3-t2-text-trinity-leaves.py`. Idempotent (overwrites the leaf files)."""
import json
from pathlib import Path

ROOT = Path("✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
OWNER = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
SCHEMA_ID = "https://json.schemas.assets.semio-tech.com/s/trinity/rewriting/mutation/{kind}/schema.json"
SURFACES = ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"]

TARGETS_UI = lambda en, de, den, dde, kind: {
    "widget": "reference", "role": "target", "label": {"en": en, "de": de}, "description": {"en": den, "de": dde},
    "ref": {"kind": kind, "domain": "graph", "granularity": "node"}, "group": "target", "order": 10,
}
OFFSET_UI = lambda axis, order: {
    "widget": "stepper", "role": "value", "label": {"en": f"Offset {axis.upper()}", "de": f"Versatz {axis.upper()}"},
    "description": {"en": f"{'Horizontal' if axis == 'x' else 'Vertical'} offset every node moves by, in canvas units.", "de": f"{'Horizontaler' if axis == 'x' else 'Vertikaler'} Versatz, um den jeder Knoten verschoben wird, in Leinwandeinheiten."},
    "step": 1, "precision": 2, "unit": "px", "group": "offset", "order": order,
}

LEAVES = [
    {
        "dir": "✋️drag-working", "emoji": "✋️", "kind": "drag-working-nodes", "variant": "DragWorkingNodes", "tag": 7, "display": "Drag Working Nodes",
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "Nodes of the working (before) graph moved by one relative offset in canvas units, read off each node's base position. Nodes the graph lacks are skipped (`mutation.partial`); none present is `mutation.target-missing`; a zero offset is `mutation.no-op`.",
        "required": ["targets", "dx", "dy"],
        "properties": {
            "targets": {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1, "uniqueItems": True, "x-semio-ui": TARGETS_UI("Nodes", "Knoten", "Working-graph nodes to drag; ones the graph lacks are skipped.", "Zu ziehende Knoten des Arbeitsgraphen; im Graphen fehlende werden übersprungen.", "node")},
            "dx": {"type": "number", "x-semio-ui": OFFSET_UI("x", 20)},
            "dy": {"type": "number", "x-semio-ui": OFFSET_UI("y", 30)},
        },
        "ts": "export interface DragWorkingNodes {\n  targets: string[];\n  dx: number;\n  dy: number;\n}\n",
        "graphql": "input DragWorkingNodesInput { targets: [String!]!, dx: Float!, dy: Float! }",
        "proto": "message DragWorkingNodes { repeated string targets = 1; double dx = 2; double dy = 3; }",
    },
    {
        "dir": "🩹️patch-working", "emoji": "🩹️", "kind": "patch-working-nodes", "variant": "PatchWorkingNodes", "tag": 8, "display": "Patch Working Nodes",
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "One field (`name` or `kind`) of nodes of the working (before) graph set to one value. Nodes the graph lacks are skipped (`mutation.partial`); none present is `mutation.target-missing`; a kind the graph's manifest does not declare is `mutation.target-mismatch`; nodes that already hold the value are a `mutation.no-op`.",
        "required": ["targets", "field", "value"],
        "properties": {
            "targets": {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1, "uniqueItems": True, "x-semio-ui": TARGETS_UI("Nodes", "Knoten", "Working-graph nodes to patch; ones the graph lacks are skipped.", "Zu ändernde Knoten des Arbeitsgraphen; im Graphen fehlende werden übersprungen.", "node")},
            "field": {"enum": ["name", "kind"], "x-semio-ui": {"widget": "segmented", "role": "discriminator", "label": {"en": "Field", "de": "Feld"}, "description": {"en": "The node field the value is written to.", "de": "Das Knotenfeld, in das der Wert geschrieben wird."}, "options": {"name": {"en": "Name", "de": "Name"}, "kind": {"en": "Kind", "de": "Art"}}, "group": "value", "order": 20}},
            "value": {"type": "string", "minLength": 1, "pattern": "\\S", "x-semio-ui": {"widget": "text", "role": "value", "label": {"en": "Value", "de": "Wert"}, "description": {"en": "The name or the declared node kind every target receives.", "de": "Der Name oder die deklarierte Knotenart, die jedes Ziel erhält."}, "group": "value", "order": 30}},
        },
        "ts": "export interface PatchWorkingNodes {\n  targets: string[];\n  field: \"name\" | \"kind\";\n  value: string;\n}\n",
        "graphql": "input PatchWorkingNodesInput { targets: [String!]!, field: String!, value: String! }",
        "proto": "message PatchWorkingNodes { repeated string targets = 1; string field = 2; string value = 3; }",
    },
    {
        "dir": "🫳️drag-rule", "emoji": "🫳️", "kind": "drag-rule-nodes", "variant": "DragRuleNodes", "tag": 9, "display": "Drag Rule Nodes",
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "Semantic rule-graph nodes (LHS/RHS clauses) moved by one relative offset in canvas units, read off each node's base position: its layout point, else its default slot. Ids the rule draws no node for are skipped (`mutation.partial`); none present is `mutation.target-missing`; a zero offset is `mutation.no-op`.",
        "required": ["targets", "dx", "dy"],
        "properties": {
            "targets": {"type": "array", "items": {"type": "string", "minLength": 1}, "minItems": 1, "uniqueItems": True, "x-semio-ui": TARGETS_UI("Rule Nodes", "Regelknoten", "Rule-graph clause nodes to drag; ids the rule draws no node for are skipped.", "Zu ziehende Klauselknoten des Regelgraphen; Ids ohne Knoten in der Regel werden übersprungen.", "ruleNode")},
            "dx": {"type": "number", "x-semio-ui": OFFSET_UI("x", 20)},
            "dy": {"type": "number", "x-semio-ui": OFFSET_UI("y", 30)},
        },
        "ts": "export interface DragRuleNodes {\n  targets: string[];\n  dx: number;\n  dy: number;\n}\n",
        "graphql": "input DragRuleNodesInput { targets: [String!]!, dx: Float!, dy: Float! }",
        "proto": "message DragRuleNodes { repeated string targets = 1; double dx = 2; double dy = 3; }",
    },
    {
        "dir": "📍️set-rule-layout", "emoji": "📍️", "kind": "set-rule-layout-points", "variant": "SetRuleLayoutPoints", "tag": 10, "display": "Set Rule Layout Points",
        "outcomes": ["applied", "no-op", "rejected"],
        "description": "Absolute rule-editor positions: every `points` key set to its point, every `cleared` key removed (back to its default slot). The exact undo of a rule-node drag; a key named twice is `mutation.invariant`; nothing to change is `mutation.no-op`.",
        "required": ["points", "cleared"],
        "properties": {
            "points": {
                "type": "array",
                "items": {"type": "object", "additionalProperties": False, "required": ["key", "x", "y"], "properties": {
                    "key": {"type": "string", "minLength": 1, "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Rule Node", "de": "Regelknoten"}, "ref": {"kind": "ruleNode", "domain": "graph", "granularity": "node"}}},
                    "x": {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "X", "de": "X"}, "step": 1, "precision": 2, "unit": "px"}},
                    "y": {"type": "number", "x-semio-ui": {"widget": "stepper", "role": "value", "label": {"en": "Y", "de": "Y"}, "step": 1, "precision": 2, "unit": "px"}},
                }},
                "x-semio-ui": {"role": "value", "label": {"en": "Points", "de": "Punkte"}, "description": {"en": "Rule nodes placed at an explicit position.", "de": "Regelknoten an einer expliziten Position."}, "group": "layout", "order": 10},
            },
            "cleared": {"type": "array", "items": {"type": "string", "minLength": 1}, "uniqueItems": True, "x-semio-ui": {"widget": "reference", "role": "target", "label": {"en": "Cleared Rule Nodes", "de": "Zurückgesetzte Regelknoten"}, "description": {"en": "Rule nodes returned to their default slot.", "de": "Regelknoten, die an ihren Standardplatz zurückkehren."}, "ref": {"kind": "ruleNode", "domain": "graph", "granularity": "node", "many": True}, "group": "layout", "order": 20}},
        },
        "invariants": [{"id": "keys-unique", "description": {"en": "Every rule node is named at most once across points and cleared keys.", "de": "Jeder Regelknoten wird über Punkte und zurückgesetzte Schlüssel höchstens einmal genannt."}}],
        "ts": "export interface RuleLayoutPlacement {\n  key: string;\n  x: number;\n  y: number;\n}\n\nexport interface SetRuleLayoutPoints {\n  points: RuleLayoutPlacement[];\n  cleared: string[];\n}\n",
        "graphql": "input RuleLayoutPlacementInput { key: String!, x: Float!, y: Float! }\ninput SetRuleLayoutPointsInput { points: [RuleLayoutPlacementInput!]!, cleared: [String!]! }",
        "proto": "message RuleLayoutPlacement { string key = 1; double x = 2; double y = 3; }\nmessage SetRuleLayoutPoints { repeated RuleLayoutPlacement points = 1; repeated string cleared = 2; }",
    },
]

RUST = {
"drag-working-nodes": r'''//! ✋️ Relative rewriting mutation — `DragWorkingNodes`: a set of working-graph nodes moved by one common offset in canvas
//! units. The gesture's own inputs (which nodes, which offset) are the payload, so editing the drag in history re-derives every
//! position from whatever graph it replays on. The node-graph drag tool yields it, one per released drag (design §13.3 of
//! ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-working-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "drag-working-nodes")]
pub struct DragWorkingNodes {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_working_nodes(targets: Vec<String>, dx: f64, dy: f64) -> RewriteRuleMutation {
    RewriteRuleMutation::DragWorkingNodes(DragWorkingNodes { targets, dx, dy })
}

impl DragWorkingNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, a finite offset.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && self.dx.is_finite() && self.dy.is_finite()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DragWorkingNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "working-nodes", kind: "drag-working-nodes", record: "DraggedWorkingNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (super::super::offset_text(self.dx), super::super::offset_text(self.dy));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 node".to_string(), "1 Knoten".to_string()),
            count => (format!("{count} nodes"), format!("{count} Knoten")),
        };
        protocol::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en})"), &format!("{items_de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
''',
"patch-working-nodes": r'''//! 🩹️ Relative rewriting mutation — `PatchWorkingNodes`: one field (`name` or `kind`) of a set of working-graph nodes set to
//! one value. The rail's patch verb yields it; history edits which nodes, which field and the value, and replays them on any
//! graph; its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🩹️ `patch-working-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "patch-working-nodes")]
pub struct PatchWorkingNodes {
    pub targets: Vec<String>,
    pub field: String,
    pub value: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn patch_working_nodes(targets: Vec<String>, field: String, value: String) -> RewriteRuleMutation {
    RewriteRuleMutation::PatchWorkingNodes(PatchWorkingNodes { targets, field, value })
}

impl PatchWorkingNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, the `name` or `kind` field, a non-blank value.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && matches!(self.field.as_str(), "name" | "kind") && !self.value.trim().is_empty()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for PatchWorkingNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "patch", entity: "working-nodes", kind: "patch-working-nodes", record: "PatchedWorkingNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 node".to_string(), "1 Knoten".to_string()),
            count => (format!("{count} nodes"), format!("{count} Knoten")),
        };
        match self.field.as_str() {
            "kind" => protocol::LocalizedLabel::native(&format!("Set the kind of {items_en} to “{}”", self.value), &format!("Art von {items_de} auf „{}“ setzen", self.value)),
            _ => protocol::LocalizedLabel::native(&format!("Rename {items_en} to “{}”", self.value), &format!("{items_de} in „{}“ umbenennen", self.value)),
        }
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
''',
"drag-rule-nodes": r'''//! 🫳️ Relative rewriting mutation — `DragRuleNodes`: a set of semantic rule-graph nodes (the LHS/RHS clause nodes) moved by
//! one common offset in canvas units, from each node's base position — its `rule_layout` point, else its default slot
//! ([`crate::standards::v1::subsets::any::schema::rule_graph_position`]). The node-graph drag tool yields it, one per released
//! drag (design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is ONE `set-rule-layout-points`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🫳️ `drag-rule-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "drag-rule-nodes")]
pub struct DragRuleNodes {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_rule_nodes(targets: Vec<String>, dx: f64, dy: f64) -> RewriteRuleMutation {
    RewriteRuleMutation::DragRuleNodes(DragRuleNodes { targets, dx, dy })
}

impl DragRuleNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, a finite offset.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && self.dx.is_finite() && self.dy.is_finite()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DragRuleNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "rule-nodes", kind: "drag-rule-nodes", record: "DraggedRuleNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (super::super::offset_text(self.dx), super::super::offset_text(self.dy));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 rule node".to_string(), "1 Regelknoten".to_string()),
            count => (format!("{count} rule nodes"), format!("{count} Regelknoten")),
        };
        protocol::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en})"), &format!("{items_de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
''',
"set-rule-layout-points": r'''//! 📍️ Absolute rewriting mutation — `SetRuleLayoutPoints`: many keys of the `rule_layout` map at once — every `points` key set
//! to its point, every `cleared` key removed (its node returns to its default slot). The exact one-row undo of a rule-node drag,
//! and the one row a reorganize publishes.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 📍️ One rule node at an explicit position.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct RuleLayoutPlacement {
    pub key: String,
    pub x: f64,
    pub y: f64,
}

/// 📍️ `set-rule-layout-points` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-rule-layout-points")]
pub struct SetRuleLayoutPoints {
    #[dsl(table)]
    pub points: Vec<RuleLayoutPlacement>,
    pub cleared: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn set_rule_layout_points(points: Vec<RuleLayoutPlacement>, cleared: Vec<String>) -> RewriteRuleMutation {
    RewriteRuleMutation::SetRuleLayoutPoints(SetRuleLayoutPoints { points, cleared })
}

impl SetRuleLayoutPoints {
    /// 🛂️ Whether the payload is well-formed: every key at most once across `points` and `cleared`, every point finite.
    pub fn holds_invariants(&self) -> bool {
        let keys: Vec<&str> = self.points.iter().map(|point| point.key.as_str()).chain(self.cleared.iter().map(String::as_str)).collect();
        !keys.iter().enumerate().any(|(at, key)| key.is_empty() || keys[..at].contains(key)) && self.points.iter().all(|point| point.x.is_finite() && point.y.is_finite())
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for SetRuleLayoutPoints {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "rule-layout-points", kind: "set-rule-layout-points", record: "SetRuleLayoutPoints" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        match self.points.len() + self.cleared.len() {
            1 => protocol::LocalizedLabel::native("Place 1 rule node", "1 Regelknoten platzieren"),
            count => protocol::LocalizedLabel::native(&format!("Place {count} rule nodes"), &format!("{count} Regelknoten platzieren")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.points.iter().map(|point| point.key.clone()).chain(self.cleared.iter().cloned()).collect()
    }
}
//#endregion 🔖️Mutation
''',
}

DIFF = {
"drag-working-nodes": r'''//! 🔺️ Sparse diff builder for `DragWorkingNodes` — every addressed node of the working graph moved by the offset read off its
//! BASE position, the graph written back as compact JSON with sorted keys (`before_fixture_json`).
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DragWorkingNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag names at least one node, never one twice, by a finite offset", payload.targets.clone());
    }
    let moved = super::super::edit_working_graph_nodes(&base.before_fixture_json, &payload.targets, |node| {
        for (axis, offset) in [("x", payload.dx), ("y", payload.dy)] {
            let at = node.get(axis).and_then(serde_json::Value::as_f64).unwrap_or(0.0);
            node.insert(axis.to_string(), serde_json::Value::from(at + offset));
        }
        true
    });
    let Some((json, missing, _)) = moved else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "a zero offset moves nothing").at(payload.targets.clone())]));
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
''',
"patch-working-nodes": r'''//! 🔺️ Sparse diff builder for `PatchWorkingNodes` — every addressed node of the working graph gets the field's value, the graph
//! written back as compact JSON with sorted keys (`before_fixture_json`) once its manifest still declares every kind.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::PatchWorkingNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a patch names at least one node, never one twice, the name or kind field and a non-blank value", payload.targets.clone());
    }
    let value = payload.value.trim().to_string();
    let patched = super::super::edit_working_graph_nodes(&base.before_fixture_json, &payload.targets, |node| {
        let next = serde_json::Value::String(value.clone());
        let changed = node.get(&payload.field) != Some(&next);
        node.insert(payload.field.clone(), next);
        changed
    });
    let Some((json, missing, changed)) = patched else {
        return protocol::MutationOutcome::error("mutation.target-missing", "the working graph does not decode, so it has none of the targets", payload.targets.clone());
    };
    if missing.len() == payload.targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of the working graph", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (not in the working graph): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if !changed {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", format!("every target's {} is already “{value}”", payload.field)).at(payload.targets.clone())]));
    }
    if payload.field == "kind" && !super::super::working_graph_is_valid(&json) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("the working graph's manifest declares no node kind “{value}”"), payload.targets.clone());
    }
    protocol::MutationOutcome::new(RewritingDiff { before_fixture_json: Some(json), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
''',
"drag-rule-nodes": r'''//! 🔺️ Sparse diff builder for `DragRuleNodes` — every addressed rule-graph node's layout point set to its BASE position (layout
//! point, else default slot) plus the offset, one map entry per moved node.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::rule_graph_position;
use crate::{LayoutPoint, RewritingSnapshot};
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::DragRuleNodes, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag names at least one rule node, never one twice, by a finite offset", payload.targets.clone());
    }
    let placed: Vec<(String, LayoutPoint)> = payload.targets.iter().filter_map(|id| rule_graph_position(base, id).map(|point| (id.clone(), point))).collect();
    let missing: Vec<String> = payload.targets.iter().filter(|id| !placed.iter().any(|(placed, _)| placed == *id)).cloned().collect();
    if placed.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("the rule draws none of the {} target node(s)", payload.targets.len()), payload.targets.clone());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} target(s) skipped (no node of this rule): {}", missing.len(), payload.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::new(RewritingDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "a zero offset moves nothing").at(payload.targets.clone())]));
    }
    let mut layout = MapDelta::default();
    for (id, point) in placed {
        layout.absorb(MapDelta::set(id, LayoutPoint { x: point.x + payload.dx, y: point.y + payload.dy }));
    }
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() }).absorb_messages(partial)
}
//#endregion 🔖️Diff
''',
"set-rule-layout-points": r'''//! 🔺️ Sparse diff builder for `SetRuleLayoutPoints` — one map entry per key that changes: a set for a point that differs, a
//! removal for a cleared key the map holds.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::{LayoutPoint, RewritingSnapshot};
use replication::MapDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::SetRuleLayoutPoints, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    if !payload.holds_invariants() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "every rule node is named at most once, every point finite", payload.cleared.clone());
    }
    let mut layout = MapDelta::default();
    for placement in &payload.points {
        let point = LayoutPoint { x: placement.x, y: placement.y };
        if base.rule_layout.get(&placement.key) != Some(&point) {
            layout.absorb(MapDelta::set(placement.key.clone(), point));
        }
    }
    for key in payload.cleared.iter().filter(|key| base.rule_layout.contains_key(*key)) {
        layout.absorb(MapDelta::remove(key.clone()));
    }
    if layout.is_empty() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "every rule node already sits where the payload places it");
    }
    protocol::MutationOutcome::new(RewritingDiff { rule_layout: Some(layout), ..Default::default() })
}
//#endregion 🔖️Diff
''',
}

INVERSE = {
"drag-working-nodes": r'''//! ↩️ Inverse for `DragWorkingNodes` — ONE `edit-before-fixture` putting the BASE working graph back (never a negated offset,
//! which rounding would not restore exactly); nothing when the drag moves nothing.
use crate::standards::v1::subsets::any::schema::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragWorkingNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match super::diff::diff(payload, base).diff().before_fixture_json.is_some() {
        true => vec![edit_before_fixture(base.before_fixture_json.clone())],
        false => Vec::new(),
    }
}
//#endregion 🔖️Inverse
''',
"patch-working-nodes": r'''//! ↩️ Inverse for `PatchWorkingNodes` — ONE `edit-before-fixture` putting the BASE working graph back; nothing when the patch
//! changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{edit_before_fixture, RewriteRuleMutation};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::PatchWorkingNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    match super::diff::diff(payload, base).diff().before_fixture_json.is_some() {
        true => vec![edit_before_fixture(base.before_fixture_json.clone())],
        false => Vec::new(),
    }
}
//#endregion 🔖️Inverse
''',
"drag-rule-nodes": r'''//! ↩️ Inverse for `DragRuleNodes` — ONE `set-rule-layout-points` putting every moved node back: its BASE layout point when it had
//! one, cleared (back to its default slot) when it had none; nothing when the drag moves nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points, RewriteRuleMutation, RuleLayoutPlacement};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragRuleNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    let Some(layout) = super::diff::diff(payload, base).diff().rule_layout.clone() else { return Vec::new() };
    let moved: Vec<&String> = layout.entries().keys().collect();
    let points = moved.iter().filter_map(|key| base.rule_layout.get(*key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = moved.iter().filter(|key| !base.rule_layout.contains_key(**key)).map(|key| (*key).clone()).collect();
    vec![set_rule_layout_points(points, cleared)]
}
//#endregion 🔖️Inverse
''',
"set-rule-layout-points": r'''//! ↩️ Inverse for `SetRuleLayoutPoints` — ONE `set-rule-layout-points` putting every key the payload changes back: its BASE point
//! when the map held one, cleared otherwise; nothing when the payload changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points, RewriteRuleMutation, RuleLayoutPlacement};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::SetRuleLayoutPoints, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    let Some(layout) = super::diff::diff(payload, base).diff().rule_layout.clone() else { return Vec::new() };
    let changed: Vec<&String> = layout.entries().keys().collect();
    let points = changed.iter().filter_map(|key| base.rule_layout.get(*key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = changed.iter().filter(|key| !base.rule_layout.contains_key(**key)).map(|key| (*key).clone()).collect();
    vec![set_rule_layout_points(points, cleared)]
}
//#endregion 🔖️Inverse
''',
}

TS_DIFF = {
"drag-working-nodes": ("export function diff(payload: DragWorkingNodes, base: readonly { id: string; x: number; y: number }[]): { id: string; x: number; y: number }[] {\n  return base.map((node) => (payload.targets.includes(node.id) ? { ...node, x: node.x + payload.dx, y: node.y + payload.dy } : node));\n}\n", "DragWorkingNodes", "mirror of the node moves of the working graph"),
"patch-working-nodes": ("export function diff(payload: PatchWorkingNodes, base: readonly { id: string; name: string; kind: string }[]): { id: string; name: string; kind: string }[] {\n  return base.map((node) => (payload.targets.includes(node.id) ? { ...node, [payload.field]: payload.value.trim() } : node));\n}\n", "PatchWorkingNodes", "mirror of the field patch of the working graph"),
"drag-rule-nodes": ("export function diff(payload: DragRuleNodes, basePositions: Readonly<Record<string, { x: number; y: number }>>): { ruleLayout: Record<string, { x: number; y: number }> } {\n  const ruleLayout: Record<string, { x: number; y: number }> = {};\n  for (const id of payload.targets) {\n    const base = basePositions[id];\n    if (base !== undefined) ruleLayout[id] = { x: base.x + payload.dx, y: base.y + payload.dy };\n  }\n  return { ruleLayout };\n}\n", "DragRuleNodes", "mirror of the per-key layout-point upserts"),
"set-rule-layout-points": ("export function diff(payload: SetRuleLayoutPoints, base: Readonly<Record<string, { x: number; y: number }>>): { ruleLayout: Record<string, { x: number; y: number } | null> } {\n  const ruleLayout: Record<string, { x: number; y: number } | null> = {};\n  for (const point of payload.points) if (base[point.key]?.x !== point.x || base[point.key]?.y !== point.y) ruleLayout[point.key] = { x: point.x, y: point.y };\n  for (const key of payload.cleared) if (base[key] !== undefined) ruleLayout[key] = null;\n  return { ruleLayout };\n}\n", "SetRuleLayoutPoints", "mirror of the per-key set/clear delta"),
}

TS_INVERSE = {
"drag-working-nodes": ("export function inverse(_payload: DragWorkingNodes, baseBeforeFixtureJson: string): EditBeforeFixture[] {\n  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];\n}\n", "DragWorkingNodes", "import type { EditBeforeFixture } from \"../../🖼️edit-before-fixture/🟦️.ts\";\n"),
"patch-working-nodes": ("export function inverse(_payload: PatchWorkingNodes, baseBeforeFixtureJson: string): EditBeforeFixture[] {\n  return [{ newBeforeFixtureJson: baseBeforeFixtureJson }];\n}\n", "PatchWorkingNodes", "import type { EditBeforeFixture } from \"../../🖼️edit-before-fixture/🟦️.ts\";\n"),
"drag-rule-nodes": ("export function inverse(payload: DragRuleNodes, baseLayout: Readonly<Record<string, { x: number; y: number }>>): SetRuleLayoutPoints[] {\n  return [{ points: payload.targets.filter((key) => baseLayout[key] !== undefined).map((key) => ({ key, ...baseLayout[key]! })), cleared: payload.targets.filter((key) => baseLayout[key] === undefined) }];\n}\n", "DragRuleNodes", "import type { SetRuleLayoutPoints } from \"../../📍️set-rule-layout/🟦️.ts\";\n"),
"set-rule-layout-points": ("export function inverse(payload: SetRuleLayoutPoints, baseLayout: Readonly<Record<string, { x: number; y: number }>>): SetRuleLayoutPoints[] {\n  const keys = [...payload.points.map((point) => point.key), ...payload.cleared];\n  return [{ points: keys.filter((key) => baseLayout[key] !== undefined).map((key) => ({ key, ...baseLayout[key]! })), cleared: keys.filter((key) => baseLayout[key] === undefined) }];\n}\n", "SetRuleLayoutPoints", ""),
}


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


for leaf in LEAVES:
    kind, variant, emoji, folder = leaf["kind"], leaf["variant"], leaf["emoji"], ROOT / leaf["dir"]
    snake = kind.replace("-", "_")
    write(folder / "🦀️.rs", RUST[kind])
    write(folder / "🔺️diff" / "🦀️.rs", DIFF[kind])
    write(folder / "↩️inverse" / "🦀️.rs", INVERSE[kind])
    write(folder / "📝️text" / "🦀️.rs", f"//! 📝️ Direct text-codec identity for {kind} / {variant}.\n\npub const TEXT_OPCODE: &str = \"{kind}\";\n")
    write(folder / "💾️binary" / "🦀️.rs", f"//! 💾️ Direct binary-codec identity for {kind} / {variant}.\n\npub const BINARY_TAG: u8 = dsl::protocol_record::tag_u8(include_str!(\"../../💾️binary/📡️.protocol.semio\"), \"{kind}\");\n")
    descriptor = {
        "schemaVersion": 1, "owner": f"{OWNER}/{leaf['dir']}", "semanticKind": kind, "displayName": leaf["display"], "emoji": emoji, "aggregateVariant": variant,
        "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": kind, "binaryTag": leaf["tag"], "invertibility": "explicit-mutation", "diffParticipation": "detect",
        "outcomeClasses": leaf["outcomes"], "composition": "atomic", "requiredLanguageSurfaces": SURFACES,
    }
    write(folder / "🔣️.json", json.dumps(descriptor, ensure_ascii=False, indent=2) + "\n")
    schema = {
        "$schema": "http://json-schema.org/draft-07/schema#", "$id": SCHEMA_ID.format(kind=kind), "title": variant, "description": leaf["description"],
        "type": "object", "additionalProperties": False, "required": ["mutation", *leaf["required"]],
        "properties": {"mutation": {"const": variant[0].lower() + variant[1:]}, **leaf["properties"]},
    }
    if "invariants" in leaf:
        schema["x-semio-invariant"] = leaf["invariants"]
    write(folder / "🧬️schema" / "🔣️.json", json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
    write(folder / "🟦️.ts", f"/** {emoji} Relative rewriting `{kind}` payload mirror of `{variant}`. */\n" + leaf["ts"])
    body, name, note = TS_DIFF[kind]
    write(folder / "🔺️diff" / "🟦️.ts", f"/** 🔺️ rewriting {kind}/🔺️diff — {note}. */\nimport type {{ {name} }} from \"../🟦️.ts\";\n\n" + body)
    body, name, extra = TS_INVERSE[kind]
    write(folder / "↩️inverse" / "🟦️.ts", f"/** ↩️ rewriting {kind}/↩️inverse — mirror of the one-row exact undo. */\nimport type {{ {name} }} from \"../🟦️.ts\";\n" + extra + "\n" + body)
    write(folder / "🔗️.graphql", f"# {emoji} Relative {kind} / {variant} payload.\n" + leaf["graphql"] + "\n")
    write(folder / "🛰️.proto", f"syntax = \"proto3\";\npackage semio.s.trinity.rewriting.mutation.{snake};\n// {kind} / {variant}\n" + leaf["proto"] + "\n")
    print(f"{leaf['dir']}: {kind} written")
